use pretty_assertions::assert_eq;

use ari::{
    ast::{Expr, ExprVariant, Label, Scope},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn single() {
    assert_eq!(
        parser().parse_recovery(":label 256"),
        (
            Some(Scope::from_iter([Expr::new(
                7..10,
                ExprVariant::Natural(256u16.into())
            )
            .with_labels([Label::new(0..6, "label")])])),
            vec![],
        )
    );
}

#[test]
fn multiple() {
    assert_eq!(
        parser().parse_recovery(":label1 :label2 256"),
        (
            Some(Scope::from_iter([Expr::new(
                16..19,
                ExprVariant::Natural(256u16.into())
            )
            .with_labels([
                Label::new(0..7, "label1"),
                Label::new(8..15, "label2"),
            ])])),
            vec![],
        )
    );
}

#[test]
fn multiple_chained() {
    assert_eq!(
        parser().parse_recovery(":x:y:z 256"),
        (
            Some(Scope::from_iter([Expr::new(
                7..10,
                ExprVariant::Natural(256u16.into())
            )
            .with_labels([
                Label::new(0..2, "x"),
                Label::new(2..4, "y"),
                Label::new(4..6, "z"),
            ])])),
            vec![],
        )
    );
}

#[test]
fn must_have_name() {
    assert_eq!(
        parser().parse_recovery(":"),
        (
            Some(Scope::from_iter([])),
            vec![Error::unexpected_end(1)
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::LabelsWithExpr)],
        )
    );
}

#[test]
fn must_have_name_in_sexpr() {
    // FIXME: Trailing garbage is a terrible error here, it should be
    // consistent with must_have_name
    assert_eq!(
        parser().parse_recovery("(: )"),
        (
            Some(Scope::from_iter([Expr::new(
                0..2,
                ExprVariant::SExpr(Scope::from_iter([]))
            )])),
            vec![
                Error::unexpected_char(1..2, ':')
                    .with_label(ErrorLabel::SExpr)
                    .with_label(ErrorLabel::ExprWithPath),
                Error::trailing_garbage(3..4)
            ],
        )
    );
}

#[test]
fn must_have_associated_expr() {
    assert_eq!(
        parser().parse_recovery(":label "),
        (
            Some(Scope::from_iter([])),
            vec![Error::unexpected_end(7).with_label(ErrorLabel::LabelsWithExpr)]
        )
    );
}

#[test]
fn must_have_associated_expr_in_sexpr() {
    // FIXME: Trailing garbage is a terrible error here, it should be
    // consistent with must_have_associated_expr
    assert_eq!(
        parser().parse_recovery("(:label )"),
        (
            Some(Scope::from_iter([Expr::new(
                0..2,
                ExprVariant::SExpr(Scope::from_iter([]))
            )])),
            vec![
                Error::unexpected_char(1..2, ':')
                    .with_label(ErrorLabel::SExpr)
                    .with_label(ErrorLabel::ExprWithPath),
                Error::trailing_garbage(2..9)
            ]
        )
    );
}

#[test]
fn names_cant_have_colon() {
    assert_eq!(
        parser().parse_recovery(":: 256"),
        (
            Some(Scope::from_iter([Expr::new(
                3..6,
                ExprVariant::Natural(256u16.into())
            )])),
            vec![Error::unexpected_char(1..2, ':')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::LabelsWithExpr)],
        )
    );
}

#[test]
fn names_cant_have_left_paren() {
    assert_eq!(
        parser().parse_recovery(":( 256"),
        (
            Some(Scope::from_iter([Expr::new(
                3..6,
                ExprVariant::Natural(256u16.into())
            )])),
            vec![Error::unexpected_char(1..2, '(')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::LabelsWithExpr)],
        )
    );
}

#[test]
fn names_cant_have_right_paren() {
    assert_eq!(
        parser().parse_recovery(":) 256"),
        (
            Some(Scope::from_iter([Expr::new(
                3..6,
                ExprVariant::Natural(256u16.into())
            )])),
            vec![Error::unexpected_char(1..2, ')')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::LabelsWithExpr)],
        )
    );
}
