#![allow(clippy::unit_arg)]

use std::ops::Range;

use chumsky::prelude::*;
use num_bigint::BigUint;
use num_traits::Num;

use crate::{
    ast::{path_span, Expr, Label, Path, Scope},
    natural::Natural,
};

// TODO: Ref, Deref, extended labels & text expressions
pub fn parser() -> impl Parser<char, Scope, Error = Error> {
    let expr = recursive(|expr| {
        labels_with_expr(expr_with_path(choice((
            sexpr(expr).map_with_span(|sexpr, span| Expr::sexpr(span, sexpr)),
            natural().map_with_span(|natural, span| Expr::natural(span, natural)),
            symbol().map_with_span(|symbol, span| Expr::symbol(span, symbol)),
        ))))
    });

    let trailing_garbage = any()
        .ignored()
        .repeated()
        .validate(|trailing_garbage, span, emit| {
            if !trailing_garbage.is_empty() {
                emit(Error::trailing_garbage(span))
            }
        });

    scope(expr).then_ignore(trailing_garbage)
}

fn labels_with_expr(
    expr: impl Parser<char, Result<Expr, ()>, Error = Error> + Clone,
) -> impl Parser<char, Result<Expr, ()>, Error = Error> + Clone {
    let labels_with_expr = label()
        .separated_by(text::whitespace())
        .at_least(1)
        .collect::<Result<Box<[Label]>, ()>>()
        .then_ignore(required_whitespace().or_not())
        .then(expr.clone().or_not())
        .validate(|(labels, expr), span, emit| match labels {
            Ok(labels) => match expr {
                Some(expr) => expr.map(|expr| {
                    debug_assert!(expr.labels.is_empty());
                    expr.with_labels(labels)
                }),
                None => Err(emit(Error::unexpected_end(span.end))),
            },
            Err(()) => match expr {
                Some(expr) => expr,
                None => Err(()),
            },
        })
        .labelled(ErrorLabel::LabelsWithExpr);

    choice((labels_with_expr, expr))
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

fn expr_with_path(
    expr: impl Parser<char, Expr, Error = Error> + Clone,
) -> impl Parser<char, Result<Expr, ()>, Error = Error> + Clone {
    expr.then(path())
        .validate(|(expr, path), _span, emit| {
            path.and_then(|path| {
                expr.with_path(path, 0)
                    .map_err(|(path, depth)| emit(Error::invalid_path(path_span(&path[depth..]))))
            })
        })
        .labelled(ErrorLabel::ExprWithPath)
}

fn path() -> impl Parser<char, Result<Box<Path>, ()>, Error = Error> + Clone {
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
    expr.separated_by(required_whitespace())
        .flatten()
        .collect()
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

fn required_whitespace() -> impl Parser<char, (), Error = Error> + Copy + Clone {
    filter(|c: &char| c.is_whitespace())
        .ignored()
        .repeated()
        .at_least(1)
        .ignored()
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
    LabelsWithExpr,
    Path,
    ExprWithPath,
    SExpr,
}
