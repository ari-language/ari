#![allow(clippy::unit_arg)]

use std::ops::Range;

use chumsky::prelude::*;
use num_bigint::BigUint;
use num_traits::Num;

use crate::{
    ast::{
        path_span, BaseExpr, Expr, ExprVariant, Label, Labels, Reference, Scope, ScopeError,
        Symbol, UnresolvedPath,
    },
    natural::Natural,
};

// TODO: Ref, Deref, extended labels & text expressions
pub fn parser() -> impl Parser<char, Scope, Error = Error> {
    scope(recursive(|expr| {
        labelled(reference(choice((
            sexpr(expr)
                .map_with_span(|scope, span| BaseExpr::variant(span, ExprVariant::SExpr(scope))),
            natural().map_with_span(|natural, span| {
                BaseExpr::variant(span, ExprVariant::Natural(natural))
            }),
            symbol().map_with_span(|symbol, span| {
                BaseExpr::variant(
                    span.clone(),
                    ExprVariant::Reference(Reference::unresolved(Symbol::new(span, symbol), [])),
                )
            }),
        ))))
    }))
    .then_ignore(
        any()
            .ignored()
            .repeated()
            .validate(|trailing_garbage, span, emit| {
                if !trailing_garbage.is_empty() {
                    emit(Error::trailing_garbage(span))
                }
            }),
    )
}

fn labelled(
    expr: impl Parser<char, Result<BaseExpr, ()>, Error = Error> + Clone,
) -> impl Parser<char, Result<Expr, ()>, Error = Error> + Clone {
    let labels_with_expr = label()
        .separated_by(text::whitespace())
        .at_least(1)
        .collect::<Result<Box<Labels>, ()>>()
        .then_ignore(text::whitespace().at_least(1).or_not())
        .then(expr.clone().or_not())
        .validate(|(labels, expr), span, emit| match labels {
            Ok(labels) => match expr {
                Some(expr) => expr.map(|expr| expr.with_labels(labels)),
                None => Err(emit(Error::unexpected_end(span.end))),
            },
            Err(()) => match expr {
                Some(expr) => expr.map(|expr| expr.with_labels(Box::new([]))),
                None => Err(()),
            },
        })
        .labelled(ErrorLabel::LabelledExpr);

    choice((
        labels_with_expr,
        expr.map(|expr| expr.map(|expr| expr.with_labels(Box::new([])))),
    ))
}

fn label() -> impl Parser<char, Result<Label, ()>, Error = Error> + Clone {
    just(':')
        .ignore_then(symbol().map(Ok).or_else(|err| Ok(Err(err))))
        .validate(|symbol, span, emit| match symbol {
            Ok(symbol) => Ok(Label::new(span, symbol)),
            Err(err) => Err(emit(err)),
        })
        .labelled(ErrorLabel::Label)
}

fn reference(
    expr: impl Parser<char, BaseExpr, Error = Error> + Clone,
) -> impl Parser<char, Result<BaseExpr, ()>, Error = Error> + Clone {
    expr.then(path())
        .validate(|(expr, path), _span, emit| {
            path.and_then(|path| {
                expr.take_from_path(path)
                    .map_err(|(path, depth)| emit(Error::invalid_path(path_span(&path[depth..]))))
            })
        })
        .labelled(ErrorLabel::Reference)
}

fn path() -> impl Parser<char, Result<Box<UnresolvedPath>, ()>, Error = Error> + Clone {
    label().repeated().collect().labelled(ErrorLabel::Path)
}

fn sexpr(
    expr: impl Parser<char, Result<Expr, ()>, Error = Error> + Clone,
) -> impl Parser<char, Scope, Error = Error> + Clone {
    scope(expr)
        .delimited_by(
            just('('),
            just(')')
                .map(|_| Ok(()))
                .or_else(|err| Ok(Err(err)))
                .validate(|result, _span, emit| result.map_err(emit)),
        )
        .labelled(ErrorLabel::SExpr)
}

fn scope(
    expr: impl Parser<char, Result<Expr, ()>, Error = Error> + Clone,
) -> impl Parser<char, Scope, Error = Error> + Clone {
    expr.separated_by(text::whitespace().at_least(1))
        .flatten()
        .validate(|exprs, _span, emit| {
            Scope::try_from_exprs_with_emit(exprs, &mut |err| {
                emit(match err {
                    ScopeError::DuplicateLabel(span, other_span) => {
                        Error::duplicate_label(span, other_span)
                    }
                    ScopeError::InvalidPath(span) => Error::invalid_path(span),
                })
            })
        })
        .padded()
}

fn natural() -> impl Parser<char, Natural, Error = Error> + Clone {
    // TODO: Support underscore for separator
    // TODO: Manually build natural from digits to avoid overhead of
    // converting to string + parse
    choice((
        just('0').ignore_then(choice((
            just("b")
                .ignore_then(text::int(2).map(|s: String| BigUint::from_str_radix(&s, 2).unwrap())),
            just("o")
                .ignore_then(text::int(8).map(|s: String| BigUint::from_str_radix(&s, 8).unwrap())),
            just("x").ignore_then(
                text::int(16).map(|s: String| BigUint::from_str_radix(&s, 16).unwrap()),
            ),
        ))),
        text::int(10).map(|s: String| BigUint::from_str_radix(&s, 10).unwrap()),
    ))
    .map(Natural::from)
    .labelled(ErrorLabel::Natural)
}

fn symbol() -> impl Parser<char, String, Error = Error> + Clone {
    filter(symbol_char)
        .repeated()
        .at_least(1)
        .collect()
        .labelled(ErrorLabel::Symbol)
}

fn symbol_char(c: &char) -> bool {
    match c {
        ':' | '(' | ')' => false,
        c => !c.is_whitespace(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {
    pub span: Range<usize>,
    pub variant: ErrorVariant,
    pub trace: Vec<ErrorLabel>,
}

impl Error {
    pub fn unexpected_char(span: Range<usize>, found: char) -> Self {
        Self {
            variant: ErrorVariant::UnexpectedChar(Some(found)),
            span,
            trace: Vec::new(),
        }
    }

    pub fn unexpected_end(pos: usize) -> Self {
        Self {
            variant: ErrorVariant::UnexpectedChar(None),
            span: pos..pos,
            trace: Vec::new(),
        }
    }

    pub fn duplicate_label(span: Range<usize>, other_span: Range<usize>) -> Self {
        Self {
            variant: ErrorVariant::DuplicateLabel(other_span),
            span,
            trace: Vec::new(),
        }
    }

    pub fn invalid_path(span: Range<usize>) -> Self {
        Self {
            variant: ErrorVariant::InvalidPath,
            span,
            trace: Vec::new(),
        }
    }

    pub fn trailing_garbage(span: Range<usize>) -> Self {
        Self {
            variant: ErrorVariant::TrailingGarbage,
            span,
            trace: Vec::new(),
        }
    }

    pub fn with_label(mut self, label: ErrorLabel) -> Self {
        self.trace.push(label);
        self
    }
}

impl chumsky::Error<char> for Error {
    type Span = Range<usize>;

    type Label = ErrorLabel;

    fn expected_input_found<Iter: IntoIterator<Item = Option<char>>>(
        span: Self::Span,
        _expected: Iter,
        found: Option<char>,
    ) -> Self {
        if let Some(found) = found {
            Self::unexpected_char(span, found)
        } else {
            debug_assert_eq!(span.start, span.end);
            Self::unexpected_end(span.end)
        }
    }

    fn with_label(self, label: Self::Label) -> Self {
        Error::with_label(self, label)
    }

    fn merge(self, _other: Self) -> Self {
        self
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorVariant {
    UnexpectedChar(Option<char>),
    DuplicateLabel(Range<usize>),
    InvalidPath,
    TrailingGarbage,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorLabel {
    Natural,
    Symbol,
    Label,
    LabelledExpr,
    Path,
    Reference,
    SExpr,
}
