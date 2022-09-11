use std::{collections::HashMap, hash::Hash, ops::Range, vec::IntoIter};

use crate::natural::Natural;

#[derive(Debug, Clone, Eq, Default)]
pub struct Scope {
    exprs: Box<[Expr]>,
    expr_from_label: HashMap<Label, usize>,
}

impl Scope {
    pub fn try_from_exprs(iter: impl IntoIterator<Item = Expr>) -> ScopeResult {
        let mut errors = Vec::new();
        let scope = Self::try_from_exprs_with_emit(iter, &mut |err| errors.push(err));
        ScopeResult {
            scope,
            errors: errors.into_boxed_slice(),
        }
    }

    pub fn try_from_exprs_with_emit(
        iter: impl IntoIterator<Item = Expr>,
        emit: &mut dyn FnMut(ScopeError),
    ) -> Self {
        let mut expr_from_label = HashMap::<Label, usize>::new();
        let mut expr_from_implicit_label = HashMap::<Label, Option<usize>>::new();

        let exprs = iter
            .into_iter()
            .enumerate()
            .map(|(expr_ref, expr)| {
                for label in expr.labels.iter() {
                    if let Some((other_label, _)) = expr_from_label.get_key_value(label) {
                        emit(ScopeError::DuplicateLabel(
                            label.span.clone(),
                            other_label.span.clone(),
                        ));
                    } else {
                        expr_from_label.insert(label.clone(), expr_ref);
                    }
                }

                // If there are no explicit labels, try to use the
                // expression's implicit label
                if expr_ref > 0 && expr.labels.is_empty() {
                    if let Some(label) = expr.implicit_label() {
                        if !expr_from_label.contains_key(&label) {
                            if let Some(other_expr_ref) = expr_from_implicit_label.get_mut(&label) {
                                *other_expr_ref = None;
                            } else {
                                expr_from_implicit_label.insert(label, Some(expr_ref));
                            }
                        }
                    }
                }

                expr
            })
            .collect();

        // Apply implicit labels that are unique within the scope
        for (label, expr_ref) in expr_from_implicit_label {
            if let Some(expr_ref) = expr_ref {
                expr_from_label.entry(label).or_insert(expr_ref);
            }
        }

        Self {
            exprs, // TODO: Try to resolve all the symbols we can in this scope
            expr_from_label,
        }
    }
}

impl IntoIterator for Scope {
    type Item = Expr;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        // into_vec: https://github.com/rust-lang/rust/issues/59878
        self.exprs.into_vec().into_iter()
    }
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.exprs.eq(&other.exprs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "this `ScopeResult` may have errors, which should be handled"]
pub struct ScopeResult {
    pub scope: Scope,
    pub errors: Box<[ScopeError]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeError {
    DuplicateLabel(Range<usize>, Range<usize>),
}

#[derive(Debug, Clone, Eq)]
pub struct Expr<Labels = Box<[Label]>> {
    pub labels: Labels,
    pub span: Range<usize>,
    pub variant: ExprVariant,
}

impl Expr<()> {
    pub(crate) fn variant(span: Range<usize>, variant: ExprVariant) -> Self {
        Self {
            labels: (),
            span,
            variant,
        }
    }

    pub(crate) fn with_path(self, path: Box<Path>) -> Result<Self, Range<usize>> {
        self.with_path_rec(path, 0).map_err(|(path, depth)| {
            let start = path.get(depth).unwrap().span.start;
            let end = path.last().unwrap().span.end;
            start..end
        })
    }

    pub(crate) fn with_labels(self, labels: Box<[Label]>) -> Expr {
        Expr {
            labels,
            span: self.span,
            variant: self.variant,
        }
    }
}

impl Expr {
    pub fn variant(
        labels: impl Into<Box<[Label]>>,
        span: Range<usize>,
        variant: ExprVariant,
    ) -> Self {
        Self {
            labels: labels.into(),
            span,
            variant,
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
        Self::variant(
            labels,
            span,
            ExprVariant::Symbol(Symbol::unresolved(symbol)),
        )
    }

    pub fn path(
        labels: impl Into<Box<[Label]>>,
        span: Range<usize>,
        path: impl Into<Box<Path>>,
    ) -> Self {
        Self::variant(
            labels,
            span,
            ExprVariant::Symbol(Symbol::unresolved_path(path)),
        )
    }

    pub fn sexpr(
        labels: impl Into<Box<[Label]>>,
        span: Range<usize>,
        scope: impl Into<Scope>,
    ) -> Self {
        Self::variant(labels, span, ExprVariant::SExpr(scope.into()))
    }

    pub fn span_with_labels(&self) -> Range<usize> {
        let start = self
            .labels
            .first()
            .map(|Label { span, .. }| span.start)
            .unwrap_or(self.span.start);

        start..self.span.end
    }

    fn implicit_label(&self) -> Option<Label> {
        match &self.variant {
            ExprVariant::Symbol(symbol) => Some(match symbol {
                Symbol::Unresolved(symbol) => Label {
                    span: self.span.clone(),
                    symbol: symbol.clone(),
                },
                Symbol::UnresolvedPath(path) => {
                    path.last().expect("at least one symbol in path").clone()
                }
                Symbol::Resolved(_) => unreachable!(),
            }),
            ExprVariant::Natural(_) | ExprVariant::SExpr(_) => None,
        }
    }
}

impl<Labels> Expr<Labels> {
    fn with_path_rec(self, path: Box<Path>, depth: usize) -> Result<Self, (Box<Path>, usize)> {
        Ok(match path.get(depth) {
            Some(label) => Self {
                labels: self.labels,
                span: self.span.start..path.last().unwrap().span.end,
                variant: match self.variant {
                    ExprVariant::Natural(_) => return Err((path, depth)),
                    ExprVariant::Symbol(symbol) => {
                        ExprVariant::Symbol(Symbol::UnresolvedPath(match symbol {
                            // into_vec: https://github.com/rust-lang/rust/issues/59878
                            Symbol::Unresolved(symbol) => [Label::new(self.span, symbol)]
                                .into_iter()
                                .chain(path.into_vec().into_iter().skip(depth))
                                .collect(),
                            Symbol::UnresolvedPath(orig_path) => orig_path
                                .into_vec()
                                .into_iter()
                                .chain(path.into_vec().into_iter().skip(depth))
                                .collect(),
                            Symbol::Resolved(_) => unreachable!(),
                        }))
                    }
                    ExprVariant::SExpr(scope) => match scope.expr_from_label.get(label).copied() {
                        Some(index) => {
                            scope
                                .into_iter()
                                .nth(index)
                                .unwrap()
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

impl<Labels: PartialEq> PartialEq for Expr<Labels> {
    fn eq(&self, other: &Self) -> bool {
        self.labels.eq(&other.labels) && self.variant.eq(&other.variant)
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
pub enum ExprVariant {
    Natural(Natural),
    Symbol(Symbol),
    SExpr(Scope),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    /// References an [Expr] by its label.
    Unresolved(String),

    /// References a label path to an [Expr].
    UnresolvedPath(Box<Path>),

    /// A resolved symbol to a parent [Expr] + a path to a child
    /// [Expr]. An empty path would be a self-reference.
    Resolved(Box<ResolvedPath>),
}

impl Symbol {
    pub fn unresolved(symbol: impl Into<String>) -> Self {
        Self::Unresolved(symbol.into())
    }

    pub fn unresolved_path(path: impl Into<Box<Path>>) -> Self {
        Self::UnresolvedPath(path.into())
    }
}

pub type Path = [Label];

pub type ResolvedPath = [usize];
