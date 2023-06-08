use std::{
    cell::Cell,
    collections::{hash_map, HashMap},
    fmt,
    hash::{Hash, Hasher},
    ops::Range,
    vec::IntoIter,
};

use crate::natural::Natural;

/// Ari's "abstract syntax tree"
#[derive(Default)]
pub struct Ast {
    exprs: Box<[Expr]>,
    expr_from_label: HashMap<String, (usize, usize)>,

    // TODO: Redesign `unresolved_map` as a radix tree
    #[allow(clippy::type_complexity)]
    unresolved_map: Cell<Option<HashMap<String, Vec<(usize, *mut Reference)>>>>,
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
        let mut exprs = Vec::with_capacity(iter.size_hint().0);
        let mut expr_from_label: HashMap<String, (usize, usize)> = HashMap::new();
        for (index, expr) in iter.enumerate() {
            exprs.push(expr);
            let expr = &exprs[index];
            for (label_index, label) in expr.labels.iter().enumerate() {
                if let Some((other_index, other_label_index)) =
                    expr_from_label.get(&label.name).copied()
                {
                    emit(AstError::DuplicateLabel(
                        label.span.clone(),
                        exprs[other_index].labels[other_label_index].span.clone(),
                    ));
                } else {
                    expr_from_label.insert(label.name.clone(), (index, label_index));
                }
            }
        }

        let mut unresolved_map: HashMap<String, Vec<(usize, *mut Reference)>> = HashMap::new();
        for (index, expr) in exprs.iter().enumerate() {
            match &expr.base.variant {
                ExprVariant::Natural(_natural) => (),
                ExprVariant::Reference(reference) => match reference {
                    Reference::Unresolved(unresolved) => {
                        let reference = reference as *const Reference as *mut Reference;
                        let symbol = &unresolved.symbol.name;
                        if let Some((other_index, _)) = expr_from_label.get(symbol).copied() {
                            let other_expr = &exprs[other_index];
                            match other_expr.base.resolve_path(unresolved.path.as_ref()) {
                                Ok(path) => {
                                    // NOTE: Could wrap Reference with RefCell to avoid unsafe, but would be more
                                    // complicated and less efficient
                                    unsafe {
                                        *reference = Reference::Resolved(ResolvedReference {
                                            scope: 0,
                                            offset: other_index as isize - index as isize,
                                            path,
                                        });
                                    }
                                }
                                Err(path) => emit(AstError::InvalidPath(path_span(path))),
                            };
                        } else {
                            // Don't need to handle when symbol is already in unresolved_map, since
                            // we can't have duplicate labels in the same scope
                            unresolved_map.insert(symbol.clone(), vec![(1, reference)]);
                        }
                    }
                    Reference::Resolved(_path) => (),
                },
                ExprVariant::SExpr(ast) => {
                    for (symbol, mut references) in ast.unresolved_map.take().into_iter().flatten()
                    {
                        if let Some((other_index, _)) = expr_from_label.get(&symbol).copied() {
                            let expr = &exprs[other_index];
                            for (scope, reference) in references {
                                // NOTE: Could use paths instead of pointers to avoid unsafe, but
                                // would be more complicated and less efficient
                                unsafe {
                                    let Reference::Unresolved(unresolved) = &*reference else { unreachable!() };
                                    match expr.base.resolve_path(&unresolved.path) {
                                        Ok(path) => {
                                            *reference = Reference::Resolved(ResolvedReference {
                                                scope,
                                                offset: other_index as isize - index as isize,
                                                path,
                                            });
                                        }
                                        Err(path) => emit(AstError::InvalidPath(path_span(path))),
                                    }
                                }
                            }
                        } else {
                            for unresolved in references.iter_mut() {
                                unresolved.0 += 1;
                            }

                            match unresolved_map.entry(symbol) {
                                hash_map::Entry::Occupied(mut entry) => {
                                    entry.get_mut().extend(references);
                                }
                                hash_map::Entry::Vacant(entry) => {
                                    entry.insert(references);
                                }
                            };
                        }
                    }
                }
            }
        }

        Self {
            exprs: exprs.into_boxed_slice(),
            expr_from_label,
            unresolved_map: Cell::new(Some(unresolved_map)),
        }
    }
}

impl fmt::Debug for Ast {
    #[no_coverage]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.exprs.fmt(f)
    }
}

impl Clone for Ast {
    #[no_coverage]
    fn clone(&self) -> Self {
        Self::try_from_exprs(self.exprs.iter().cloned()).unwrap()
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
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.exprs.hash(state)
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstError {
    DuplicateLabel(Range<usize>, Range<usize>),
    InvalidPath(Range<usize>),
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
        symbol: Symbol,
        path: impl Into<Box<UnresolvedPath>>,
    ) -> Self {
        Self::variant(
            labels,
            span,
            ExprVariant::Reference(Reference::unresolved(symbol, path)),
        )
    }

    pub fn resolved_reference(
        labels: impl Into<Box<Labels>>,
        span: Range<usize>,
        scope: usize,
        offset: isize,
        path: impl Into<Box<ResolvedPath>>,
    ) -> Self {
        Self::variant(
            labels,
            span,
            ExprVariant::Reference(Reference::resolved(scope, offset, path)),
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

    fn resolve_path<'p>(
        &self,
        mut unresolved: &'p UnresolvedPath,
    ) -> Result<Box<ResolvedPath>, &'p UnresolvedPath> {
        let mut expr = self;
        let mut resolved = Vec::with_capacity(unresolved.len());
        while let Some((label, remainder)) = unresolved.split_first() {
            let ExprVariant::SExpr(ast) = &expr.variant else { return Err(unresolved) };
            let Some((index, _)) = ast.expr_from_label.get(&label.name).copied() else { return Err(unresolved) };
            expr = &ast.exprs[index].base;
            unresolved = remainder;
            resolved.push(index);
        }

        Ok(resolved.into_boxed_slice())
    }

    pub(crate) fn take_from_path(
        self,
        path: Box<UnresolvedPath>,
    ) -> Result<Self, (Box<UnresolvedPath>, usize)> {
        self.take_from_path_rec(path, 0)
    }

    fn take_from_path_rec(
        self,
        path: Box<UnresolvedPath>,
        depth: usize,
    ) -> Result<Self, (Box<UnresolvedPath>, usize)> {
        Ok(match path.get(depth) {
            Some(label) => Self {
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
                        match ast.expr_from_label.get(&label.name).copied() {
                            Some((index, _)) => {
                                ast.into_iter()
                                    .nth(index)
                                    .unwrap()
                                    .base
                                    .take_from_path_rec(path, depth + 1)?
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
    pub fn unresolved(symbol: Symbol, path: impl Into<Box<UnresolvedPath>>) -> Self {
        Self::Unresolved(UnresolvedReference {
            symbol,
            path: path.into(),
        })
    }

    pub fn resolved(scope: usize, offset: isize, path: impl Into<Box<[usize]>>) -> Self {
        Self::Resolved(ResolvedReference {
            scope,
            offset,
            path: path.into(),
        })
    }
}

/// A chain of symbols pointing to an [Expr] within the current scope.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnresolvedReference {
    pub symbol: Symbol,
    pub path: Box<UnresolvedPath>,
}

impl UnresolvedReference {
    fn join(self, other: impl IntoIterator<Item = Label>) -> Self {
        // into_vec: https://github.com/rust-lang/rust/issues/59878
        Self {
            symbol: self.symbol,
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

pub fn path_span(path: &UnresolvedPath) -> Range<usize> {
    let start = path.first().expect("at least one label in path").span.start;
    let end = path.last().expect("at least one label in path").span.end;
    start..end
}

/// A resolved relative path to a parent [Expr].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResolvedReference {
    pub scope: usize,
    pub offset: isize,
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
