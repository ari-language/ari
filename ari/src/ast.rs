use std::{collections::HashMap, ops::Range, vec::IntoIter};

use crate::natural::Natural;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scope {
    exprs: Box<[Expr]>,
    expr_from_label: HashMap<String, usize>,
}

impl Scope {
    pub fn from_exprs<E: IntoIterator<Item = Expr>>(exprs: E) -> Self {
        let mut expr_from_label = HashMap::<String, usize>::new();
        let mut expr_from_implicit_label = HashMap::<String, Vec<usize>>::new();

        let exprs = exprs
            .into_iter()
            .enumerate()
            .map(|(i, expr)| {
                for Label { symbol, .. } in expr.labels.iter() {
                    // TODO: Figure out how to emit duplicate error in here
                    if !expr_from_label.contains_key(symbol) {
                        expr_from_label.insert(symbol.clone(), i);
                    }
                }

                // If there are no explicit labels, try to use the
                // expression's implicit label
                if i > 0 && expr.labels.is_empty() {
                    if let Some(symbol) = expr.implicit_label() {
                        if !expr_from_label.contains_key(symbol) {
                            if let Some(exprs) = expr_from_implicit_label.get_mut(symbol) {
                                exprs.push(i);
                            } else {
                                expr_from_implicit_label.insert(symbol.to_owned(), vec![i]);
                            }
                        }
                    }
                }

                expr
            })
            .collect();

        // Apply implicit labels that are unique within the scope
        for (symbol, exprs) in expr_from_implicit_label {
            if let [i] = exprs[..] {
                expr_from_label.entry(symbol).or_insert(i);
            }
        }

        Scope {
            exprs, // TODO: Try to resolve all the symbols we can in this scope
            expr_from_label,
        }
    }
}

impl<const N: usize> From<[Expr; N]> for Scope {
    fn from(exprs: [Expr; N]) -> Self {
        Self::from_exprs(exprs.into_iter())
    }
}

impl FromIterator<Expr> for Scope {
    fn from_iter<T: IntoIterator<Item = Expr>>(iter: T) -> Self {
        Self::from_exprs(iter)
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub span: Range<usize>,
    pub variant: ExprVariant,
    pub labels: Box<[Label]>,
}

impl Expr {
    pub fn variant(span: Range<usize>, variant: ExprVariant) -> Self {
        Self {
            span,
            variant,
            labels: Box::new([]),
        }
    }

    pub fn natural<N: Into<Natural>>(span: Range<usize>, natural: N) -> Self {
        Self::variant(span, ExprVariant::Natural(natural.into()))
    }

    pub fn symbol<S: Into<String>>(span: Range<usize>, symbol: S) -> Self {
        Self::variant(span, ExprVariant::Symbol(Symbol::unresolved(symbol)))
    }

    pub fn path<P: Into<Box<Path>>>(span: Range<usize>, path: P) -> Self {
        Self::variant(span, ExprVariant::Symbol(Symbol::unresolved_path(path)))
    }

    pub fn sexpr<S: Into<Scope>>(span: Range<usize>, scope: S) -> Self {
        Self::variant(span, ExprVariant::SExpr(scope.into()))
    }

    pub fn with_labels<L: Into<Box<[Label]>>>(mut self, labels: L) -> Self {
        self.labels = labels.into();
        self
    }

    pub fn span_with_labels(&self) -> Range<usize> {
        let start = self
            .labels
            .first()
            .map(|Label { span, .. }| span.start)
            .unwrap_or(self.span.start);

        start..self.span.end
    }

    pub fn with_path(self, path: Box<Path>) -> Result<Expr, Range<usize>> {
        self.with_path_rec(path, 0).map_err(|(path, depth)| {
            let start = path.get(depth).unwrap().span.start;
            let end = path.last().unwrap().span.end;
            start..end
        })
    }

    fn with_path_rec(self, path: Box<Path>, depth: usize) -> Result<Expr, (Box<Path>, usize)> {
        Ok(match path.get(depth) {
            Some(Label { symbol, .. }) => Expr {
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
                    ExprVariant::SExpr(scope) => match scope.expr_from_label.get(symbol).copied() {
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
                labels: self.labels,
            },
            None => self,
        })
    }

    fn implicit_label(&self) -> Option<&str> {
        match &self.variant {
            ExprVariant::Symbol(symbol) => Some(symbol.implicit_label()),
            ExprVariant::Natural(_) | ExprVariant::SExpr(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub span: Range<usize>,
    pub symbol: String,
}

impl Label {
    pub fn new<S: Into<String>>(span: Range<usize>, symbol: S) -> Self {
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
    pub fn unresolved<S: Into<String>>(symbol: S) -> Self {
        Self::Unresolved(symbol.into())
    }

    pub fn unresolved_path<P: Into<Box<Path>>>(path: P) -> Self {
        Self::UnresolvedPath(path.into())
    }

    fn implicit_label(&self) -> &str {
        match self {
            Symbol::Unresolved(symbol) => symbol,
            Symbol::UnresolvedPath(path) => path
                .last()
                .expect("at least one symbol in path")
                .symbol
                .as_str(),

            Symbol::Resolved(_) => unreachable!(),
        }
    }
}

pub type Path = [Label];

pub type ResolvedPath = [usize];
