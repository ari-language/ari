use std::ops::Range;

use chumsky::prelude::*;
use num_bigint::BigUint;

use crate::{
    ast::{path_span, Expr, ExprVariant, Label, Path, Scope, Symbol},
    natural::Natural,
};

// TODO: Ref, Deref, extended labels & text expressions
pub fn parser() -> impl Parser<char, Scope, Error = Error> {
    let expr = recursive(|expr| {
        let base_expr = choice((
            // TODO: Better recovery of unbalanced parens
            scope(expr)
                .or_not()
                .map(|exprs| exprs.unwrap_or_default())
                .padded()
                .delimited_by(just('('), just(')'))
                .map(ExprVariant::SExpr)
                .map_with_span(Expr::with_span)
                .labelled(ErrorLabel::SExpr),
            natural()
                .map(Natural::from)
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
            .then(path().labelled(ErrorLabel::Path))
            .validate(|(expr, path), _span, emit| match expr.with_path(path, 0) {
                Ok(expr) => Some(expr),
                Err((path, depth)) => {
                    emit(Error::invalid_path(path_span(&path[depth..])));
                    return None;
                }
            })
            .labelled(ErrorLabel::ExprWithPath);

        let labels_with_expr = label()
            .labelled(ErrorLabel::Label)
            .then_ignore(text::whitespace())
            .repeated()
            .at_least(1)
            .flatten()
            .collect::<Box<[Label]>>()
            .then(expr_with_path.clone().or_not().map(Option::flatten))
            .validate(|(labels, expr), span, emit| match expr {
                Some(expr) => {
                    debug_assert!(expr.labels.is_empty());
                    Some(Expr::with_labels(expr, labels))
                }
                None => {
                    emit(Error::unexpected_end(span.end));
                    None
                }
            })
            .labelled(ErrorLabel::LabelsWithExpr);

        choice((labels_with_expr, expr_with_path))
    });

    scope(expr).padded().then_ignore(end())
}

fn scope(
    expr: impl Parser<char, Option<Expr>, Error = Error> + Clone,
) -> impl Parser<char, Scope, Error = Error> + Clone {
    expr.separated_by(
        filter(|c: &char| c.is_whitespace())
            .ignored()
            .repeated()
            .at_least(1),
    )
    .at_least(1)
    .flatten()
    .collect()
}

fn path() -> impl Parser<char, Box<Path>, Error = Error> + Copy + Clone {
    label().repeated().flatten().collect()
}

fn label() -> impl Parser<char, Option<Label>, Error = Error> + Copy + Clone {
    just(':')
        .ignore_then(symbol().or_not())
        .validate(|symbol, span, emit| match symbol {
            Some(symbol) => Some(Label::new(span, symbol)),
            None => {
                emit(Error::unexpected_end(span.end));
                None
            }
        })
}

fn natural() -> impl Parser<char, BigUint, Error = Error> + Copy + Clone {
    // TODO: Support custom bases: binary, hex, octal, ...
    // TODO: Support underscore for separator
    // TODO: Manually build natural from digits to avoid overhead of
    // converting to string + parse
    text::int(10).map(|s: String| s.parse().unwrap())
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
