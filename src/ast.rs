use std::{collections::HashMap, hash::Hash, ops::Range, vec::IntoIter};

use crate::natural::Natural;

/// Ari's "abstract syntax tree"
#[derive(Debug, Clone, Eq, Default)]
pub struct Ast {
    exprs: Box<[Expr]>,
    expr_from_label: HashMap<Label, usize>,
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
        let mut expr_from_label = HashMap::<Label, usize>::new();

        let exprs = iter
            .into_iter()
            .enumerate()
            .map(|(expr_ref, expr)| {
                for label in expr.labels.iter() {
                    if let Some((other_label, _)) = expr_from_label.get_key_value(label) {
                        emit(AstError::DuplicateLabel(
                            label.span.clone(),
                            other_label.span.clone(),
                        ));
                    } else {
                        expr_from_label.insert(label.clone(), expr_ref);
                    }
                }

                expr
            })
            .collect();

        Self {
            exprs, // TODO: Try to resolve all the symbols we can in this scope
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstError {
    DuplicateLabel(Range<usize>, Range<usize>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub labels: Box<[Label]>,
    pub base: BaseExpr,
}

impl Expr {
    pub fn variant(
        labels: impl Into<Box<[Label]>>,
        span: Range<usize>,
        variant: ExprVariant,
    ) -> Self {
        Self {
            labels: labels.into(),
            base: BaseExpr::variant(span, variant),
        }
    }

    pub fn natural(
        labels: impl Into<Box<[Label]>>,
        span: Range<usize>,
        natural: impl Into<Natural>,
    ) -> Self {
        Self::variant(labels, span, ExprVariant::Natural(natural.into()))
    }

    pub fn symbol(
        labels: impl Into<Box<[Label]>>,
        span: Range<usize>,
        symbol: impl Into<String>,
    ) -> Self {
        Self::path(labels, span.clone(), [Label::new(span, symbol)])
    }

    pub fn path(
        labels: impl Into<Box<[Label]>>,
        span: Range<usize>,
        path: impl Into<Box<Path>>,
    ) -> Self {
        Self::variant(labels, span, ExprVariant::Symbol(Symbol::unresolved(path)))
    }

    pub fn sexpr(labels: impl Into<Box<[Label]>>, span: Range<usize>, ast: impl Into<Ast>) -> Self {
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

#[derive(Debug, Clone, Eq)]
pub struct Label {
    pub span: Range<usize>,
    pub symbol: String,
}

impl PartialEq for Label {
    fn eq(&self, other: &Self) -> bool {
        self.symbol.eq(&other.symbol)
    }
}

impl Hash for Label {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state)
    }
}

impl Label {
    pub fn new(span: Range<usize>, symbol: impl Into<String>) -> Self {
        Self {
            span,
            symbol: symbol.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseExpr {
    pub span: Range<usize>,
    pub variant: ExprVariant,
}

impl BaseExpr {
    pub(crate) fn variant(span: Range<usize>, variant: ExprVariant) -> Self {
        Self { span, variant }
    }

    pub(crate) fn with_path(self, path: Box<Path>) -> Result<Self, Range<usize>> {
        self.with_path_rec(path, 0).map_err(|(path, depth)| {
            let start = path.get(depth).unwrap().span.start;
            let end = path.last().unwrap().span.end;
            start..end
        })
    }

    pub(crate) fn with_labels(self, labels: Box<[Label]>) -> Expr {
        Expr { labels, base: self }
    }
}

impl BaseExpr {
    fn with_path_rec(self, path: Box<Path>, depth: usize) -> Result<Self, (Box<Path>, usize)> {
        Ok(match path.get(depth) {
            Some(label) => Self {
                span: self.span.start..path.last().unwrap().span.end,
                variant: match self.variant {
                    ExprVariant::Natural(_) => return Err((path, depth)),
                    ExprVariant::Symbol(symbol) => ExprVariant::Symbol(match symbol {
                        Symbol::Unresolved(orig_path) => Symbol::Unresolved(
                            orig_path.join(path.into_vec().into_iter().skip(depth)),
                        ),
                        Symbol::Resolved(_) => unreachable!(),
                    }),
                    ExprVariant::SExpr(ast) => match ast.expr_from_label.get(label).copied() {
                        Some(index) => {
                            ast.into_iter()
                                .nth(index)
                                .unwrap()
                                .base
                                .with_path_rec(path, depth + 1)?
                                .variant
                        }
                        None => return Err((path, depth)),
                    },
                },
            },
            None => self,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprVariant {
    Natural(Natural),
    Symbol(Symbol),
    SExpr(Ast),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    /// References a label path to an [Expr].
    Unresolved(UnresolvedSymbol),

    /// A resolved symbol to a parent [Expr] + a path to a child
    /// [Expr]. An empty path would be a self-reference.
    Resolved(ResolvedSymbol),
}

impl Symbol {
    pub fn unresolved(path: impl Into<Box<Path>>) -> Self {
        Self::Unresolved(UnresolvedSymbol { path: path.into() })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedSymbol {
    pub path: Box<Path>,
}

impl UnresolvedSymbol {
    fn join(self, path: impl IntoIterator<Item = Label>) -> Self {
        Self {
            // into_vec: https://github.com/rust-lang/rust/issues/59878
            path: self.path.into_vec().into_iter().chain(path).collect(),
        }
    }
}

pub type Path = [Label];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSymbol {
    pub path: Box<ResolvedPath>,
}

pub type ResolvedPath = [usize];
