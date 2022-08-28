use std::{collections::HashMap, ops::Range, vec::IntoIter};

use crate::natural::Natural;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Scope {
    exprs: Box<[Expr]>,
    expr_from_label: HashMap<String, usize>,
}

impl FromIterator<Expr> for Scope {
    fn from_iter<T: IntoIterator<Item = Expr>>(iter: T) -> Self {
        let mut expr_from_label = HashMap::<String, usize>::new();
        let mut expr_from_implicit_label = HashMap::<String, Vec<usize>>::new();

        let exprs = iter
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
            match exprs[..] {
                [i] => {
                    expr_from_label.entry(symbol).or_insert(i);
                }
                _ => (),
            }
        }

        Scope {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expr {
    pub span: Range<usize>,
    pub variant: ExprVariant,
    pub labels: Box<[Label]>,
}

impl Expr {
    pub fn new(span: Range<usize>, variant: ExprVariant) -> Self {
        Self {
            span,
            variant,
            labels: Box::new([]),
        }
    }

    // Convenience for chumsky::map_with_span
    pub(crate) fn with_span(variant: ExprVariant, span: Range<usize>) -> Self {
        Self::new(span, variant)
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

    fn implicit_label(&self) -> Option<&str> {
        match &self.variant {
            ExprVariant::Symbol(symbol) => Some(symbol.implicit_label()),
            ExprVariant::Natural(_) | ExprVariant::SExpr(_) => None,
        }
    }

    pub(crate) fn with_path(
        self,
        path: Box<Path>,
        depth: usize,
    ) -> Result<Expr, (Box<Path>, usize)> {
        let end = path
            .last()
            .map(|Label { span, .. }| span.end)
            .unwrap_or(self.span.end);

        Ok(Expr {
            span: self.span.start..end,
            variant: match self.variant {
                ExprVariant::Natural(natural) => ExprVariant::Natural(match depth < path.len() {
                    true => return Err((path, depth)),
                    false => natural,
                }),
                ExprVariant::Symbol(symbol) => ExprVariant::Symbol(match depth < path.len() {
                    // into_vec: https://github.com/rust-lang/rust/issues/59878
                    true => Symbol::UnresolvedPath(match symbol {
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
                    }),
                    false => symbol,
                }),
                ExprVariant::SExpr(scope) => match path.get(depth) {
                    Some(Label { symbol, .. }) => {
                        match scope.expr_from_label.get(symbol).map(|&index| index) {
                            Some(index) => {
                                scope
                                    .into_iter()
                                    .nth(index)
                                    .unwrap()
                                    .with_path(path, depth + 1)?
                                    .variant
                            }
                            None => return Err((path, depth)),
                        }
                    }
                    None => ExprVariant::SExpr(scope),
                },
            },
            labels: self.labels,
        })
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

pub fn path_span(path: &Path) -> Range<usize> {
    let start = path
        .first()
        .expect("at least one symbol in path")
        .span
        .start;

    let end = path.last().expect("at least one symbol in path").span.end;

    start..end
}

pub type ResolvedPath = [usize];