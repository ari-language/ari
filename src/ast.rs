use std::{collections::HashMap, hash::Hash, ops::Range, vec::IntoIter};

use crate::natural::Natural;

/// Ari's "abstract syntax tree"
#[derive(Debug, Clone, Default)]
pub struct Ast {
    exprs: Box<[Expr]>,
    expr_from_label: HashMap<String, (usize, usize)>,
}

impl Ast {
    pub fn try_from_exprs(
        iter: impl IntoIterator<Item = Expr>,
    ) -> Result<Ast, (Box<[AstError]>, Ast)> {
        let mut errors = Vec::new();
        let ast = Self::try_from_exprs_with_emit(iter, &mut |err| errors.push(err));
        if errors.is_empty() {
            Ok(ast)
        } else {
            Err((errors.into_boxed_slice(), ast))
        }
    }

    pub fn try_from_exprs_with_emit(
        iter: impl IntoIterator<Item = Expr>,
        emit: &mut dyn FnMut(AstError),
    ) -> Self {
        let iter = iter.into_iter();
        let mut exprs: Vec<Expr> = Vec::with_capacity(iter.size_hint().0);
        let mut expr_from_label = HashMap::<String, (usize, usize)>::new();
        for (index, expr) in iter.enumerate() {
            exprs.push(expr);
            let expr = &exprs[index];
            for (label_index, label) in expr.labels.iter().enumerate() {
                if let Some((other_index, other_label_index)) = expr_from_label.get(&label.name) {
                    emit(AstError::DuplicateLabel(
                        label.span.clone(),
                        exprs[*other_index].labels[*other_label_index].span.clone(),
                    ));
                } else {
                    expr_from_label.insert(label.name.clone(), (index, label_index));
                }
            }
        }

        Self {
            // TODO: Try to resolve all the references we can in this scope
            exprs: exprs.into_boxed_slice(),
            expr_from_label,
        }
    }
}

impl IntoIterator for Ast {
    type Item = Expr;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // into_vec: https://github.com/rust-lang/rust/issues/59878
        self.exprs.into_vec().into_iter()
    }
}

impl PartialEq for Ast {
    fn eq(&self, other: &Self) -> bool {
        self.exprs.eq(&other.exprs)
    }
}

impl Eq for Ast {}

impl Hash for Ast {
    #[no_coverage]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.exprs.hash(state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstError {
    DuplicateLabel(Range<usize>, Range<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub labels: Box<Labels>,
    pub base: BaseExpr,
}

impl Expr {
    pub fn variant(
        labels: impl Into<Box<Labels>>,
        span: Range<usize>,
        variant: ExprVariant,
    ) -> Self {
        Self {
            labels: labels.into(),
            base: BaseExpr::variant(span, variant),
        }
    }

    pub fn natural(
        labels: impl Into<Box<Labels>>,
        span: Range<usize>,
        natural: impl Into<Natural>,
    ) -> Self {
        Self::variant(labels, span, ExprVariant::Natural(natural.into()))
    }

    pub fn unresolved_symbol(
        labels: impl Into<Box<Labels>>,
        span: Range<usize>,
        name: impl Into<String>,
    ) -> Self {
        Self::unresolved_reference(labels, span.clone(), Symbol::new(span, name), [])
    }

    pub fn unresolved_reference(
        labels: impl Into<Box<Labels>>,
        span: Range<usize>,
        root: Symbol,
        path: impl Into<Box<UnresolvedPath>>,
    ) -> Self {
        Self::variant(
            labels,
            span,
            ExprVariant::Reference(Reference::unresolved(root, path)),
        )
    }

    pub fn sexpr(labels: impl Into<Box<Labels>>, span: Range<usize>, ast: impl Into<Ast>) -> Self {
        Self::variant(labels, span, ExprVariant::SExpr(ast.into()))
    }

    pub fn span(&self) -> Range<usize> {
        let start = self
            .labels
            .first()
            .map(|Label { span, .. }| span.start)
            .unwrap_or(self.base.span.start);

        start..self.base.span.end
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseExpr {
    pub span: Range<usize>,
    pub variant: ExprVariant,
}

impl BaseExpr {
    pub(crate) fn variant(span: Range<usize>, variant: ExprVariant) -> Self {
        Self { span, variant }
    }

    pub(crate) fn with_labels(self, labels: Box<Labels>) -> Expr {
        Expr { labels, base: self }
    }

    pub(crate) fn with_path(self, path: Box<UnresolvedPath>) -> Result<Self, Range<usize>> {
        self.with_path_rec(path, 0).map_err(|(path, depth)| {
            let start = path.get(depth).unwrap().span.start;
            let end = path.last().unwrap().span.end;
            start..end
        })
    }

    fn with_path_rec(
        self,
        path: Box<UnresolvedPath>,
        depth: usize,
    ) -> Result<Self, (Box<UnresolvedPath>, usize)> {
        Ok(match path.get(depth) {
            Some(symbol) => Self {
                span: self.span.start..path.last().unwrap().span.end,
                variant: match self.variant {
                    ExprVariant::Natural(_) => return Err((path, depth)),
                    ExprVariant::Reference(reference) => ExprVariant::Reference(match reference {
                        Reference::Unresolved(unresolved) => {
                            // into_vec: https://github.com/rust-lang/rust/issues/59878
                            Reference::Unresolved(unresolved.join(path.into_vec()))
                        }
                        Reference::Resolved(_) => unreachable!(),
                    }),
                    ExprVariant::SExpr(ast) => {
                        match ast.expr_from_label.get(&symbol.name).copied() {
                            Some((index, _)) => {
                                ast.into_iter()
                                    .nth(index)
                                    .unwrap()
                                    .base
                                    .with_path_rec(path, depth + 1)?
                                    .variant
                            }
                            None => return Err((path, depth)),
                        }
                    }
                },
            },
            None => self,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprVariant {
    Natural(Natural),
    Reference(Reference),
    SExpr(Ast),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Reference {
    Unresolved(UnresolvedReference),
    Resolved(ResolvedReference),
}

impl Reference {
    pub fn unresolved(root: Symbol, path: impl Into<Box<UnresolvedPath>>) -> Self {
        Self::Unresolved(UnresolvedReference {
            root,
            path: path.into(),
        })
    }
}

/// A chain of symbols pointing to an [Expr] within the current scope.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnresolvedReference {
    pub root: Symbol,
    pub path: Box<UnresolvedPath>,
}

impl UnresolvedReference {
    fn join(self, other: impl IntoIterator<Item = Label>) -> Self {
        // into_vec: https://github.com/rust-lang/rust/issues/59878
        Self {
            root: self.root,
            path: self
                .path
                .into_vec()
                .into_iter()
                .chain(other.into_iter())
                .collect(),
        }
    }
}

pub type UnresolvedPath = [Label];

/// A resolved relative path to a parent [Expr].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedReference {
    pub root: usize,
    pub path: Box<ResolvedPath>,
}

pub type ResolvedPath = [usize];

pub type Labels = [Label];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    pub span: Range<usize>,
    pub name: String,
}

impl Label {
    pub fn new(span: Range<usize>, name: impl Into<String>) -> Self {
        Self {
            span,
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub span: Range<usize>,
    pub name: String,
}

impl Symbol {
    pub fn new(span: Range<usize>, name: impl Into<String>) -> Self {
        Self {
            span,
            name: name.into(),
        }
    }
}
