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
            vec![
                Error::unexpected_end(1)
                    .with_label(ErrorLabel::Label)
                    .with_label(ErrorLabel::LabelsWithExpr),
                Error::unexpected_end(1).with_label(ErrorLabel::LabelsWithExpr)
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
fn names_cant_have_colon() {
    assert_eq!(
        parser().parse_recovery(":: 256"),
        (
            Some(Scope::from_iter([Expr::new(
                3..6,
                ExprVariant::Natural(256u16.into())
            )])),
            vec![
                Error::unexpected_end(1)
                    .with_label(ErrorLabel::Label)
                    .with_label(ErrorLabel::LabelsWithExpr),
                Error::unexpected_end(2)
                    .with_label(ErrorLabel::Label)
                    .with_label(ErrorLabel::LabelsWithExpr),
            ],
        )
    );
}

#[test]
fn names_cant_have_left_paren() {
    // TODO: Error recovery for unbalanced parens
    assert_eq!(
        parser().parse_recovery(":( 256"),
        (
            None,
            vec![
                Error::unexpected_end(1)
                    .with_label(ErrorLabel::Label)
                    .with_label(ErrorLabel::LabelsWithExpr),
                Error::unexpected_end(1).with_label(ErrorLabel::LabelsWithExpr),
                Error::unexpected_end(6)
                    .with_label(ErrorLabel::SExpr)
                    .with_label(ErrorLabel::ExprWithPath)
            ],
        )
    );
}

#[test]
fn names_cant_have_right_paren() {
    // TODO: Error recovery for unbalanced parens
    assert_eq!(
        parser().parse_recovery(":) 256"),
        (
            None,
            vec![
                Error::unexpected_end(1)
                    .with_label(ErrorLabel::Label)
                    .with_label(ErrorLabel::LabelsWithExpr),
                Error::unexpected_end(1).with_label(ErrorLabel::LabelsWithExpr),
                Error::unexpected_char(1..2, ')')
            ],
        )
    );
}