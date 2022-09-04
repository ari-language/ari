use std::ops::Range;

use chumsky::prelude::*;
use num_bigint::BigUint;
use num_traits::Num;

use crate::{
    ast::{path_span, Expr, ExprVariant, Label, Path, Scope, Symbol},
    natural::Natural,
};

// TODO: Ref, Deref, extended labels & text expressions
pub fn parser() -> impl Parser<char, Scope, Error = Error> {
    let expr = recursive(|expr| {
        let base_expr = choice((
            sexpr(expr)
                .map(ExprVariant::SExpr)
                .map_with_span(Expr::with_span)
                .labelled(ErrorLabel::SExpr),
            natural()
                .map(ExprVariant::Natural)
                .map_with_span(Expr::with_span)
                .labelled(ErrorLabel::Natural),
            symbol()
                .map(Symbol::Unresolved)
                .map(ExprVariant::Symbol)
                .map_with_span(Expr::with_span)
                .labelled(ErrorLabel::Symbol),
        ));

        let expr_with_path = base_expr
            .then(path().or_not().labelled(ErrorLabel::Path))
            .validate(|(expr, path), _span, emit| match path {
                Some(path) => path.and_then(|path| match expr.with_path(path, 0) {
                    Ok(expr) => Some(expr),
                    Err((path, depth)) => {
                        emit(Error::invalid_path(path_span(&path[depth..])));
                        None
                    }
                }),
                None => Some(expr),
            })
            .labelled(ErrorLabel::ExprWithPath);

        let labels_with_expr = label()
            .labelled(ErrorLabel::Label)
            .separated_by(text::whitespace())
            .at_least(1)
            .collect::<Option<Box<[Label]>>>()
            .then_ignore(required_whitespace().or(end()))
            .then(expr_with_path.clone().map(Some).or(end().to(None)))
            .validate(|(labels, expr), span, emit| match labels {
                Some(labels) => match expr {
                    Some(expr) => expr.map(|expr| {
                        debug_assert!(expr.labels.is_empty());
                        Expr::with_labels(expr, labels)
                    }),
                    None => {
                        emit(Error::unexpected_end(span.end));
                        None
                    }
                },
                None => expr.flatten(),
            })
            .labelled(ErrorLabel::LabelsWithExpr);

        choice((labels_with_expr, expr_with_path))
    });

    let trailing_right_parens = just(')')
        .validate(|c, span, emit| emit(Error::unexpected_char(span, c)))
        .repeated();

    expr.separated_by(required_whitespace())
        .at_least(1)
        .flatten()
        .collect()
        .padded()
        .then_ignore(trailing_right_parens)
        .then_ignore(end())
}

fn sexpr(
    expr: impl Parser<char, Option<Expr>, Error = Error> + Clone,
) -> impl Parser<char, Scope, Error = Error> + Clone {
    expr.separated_by(required_whitespace())
        .flatten()
        .collect()
        .padded()
        .delimited_by(
            just('('),
            just(')')
                .map(|_| Ok(()))
                .or_else(|err| Ok(Err(err)))
                .validate(|result, _span, emit| {
                    if let Err(err) = result {
                        emit(err)
                    }
                }),
        )
}

fn path() -> impl Parser<char, Option<Box<Path>>, Error = Error> + Copy + Clone {
    label().repeated().at_least(1).collect()
}

fn label() -> impl Parser<char, Option<Label>, Error = Error> + Copy + Clone {
    just(':')
        .ignore_then(symbol().map(Ok).or_else(|err| Ok(Err(err))))
        .validate(|symbol, span, emit| match symbol {
            Ok(symbol) => Some(Label::new(span, symbol)),
            Err(err) => {
                emit(err);
                None
            }
        })
}

fn natural() -> impl Parser<char, Natural, Error = Error> + Copy + Clone {
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
}

fn symbol() -> impl Parser<char, String, Error = Error> + Copy + Clone {
    filter(symbol_char).repeated().at_least(1).collect()
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
