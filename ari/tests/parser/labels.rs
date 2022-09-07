use pretty_assertions::assert_eq;

use ari::{
    ast::{Expr, Label, Scope},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn single() {
    assert_eq!(
        parser().parse_recovery(":label 256"),
        (
            Some(Scope::from_exprs([Expr::natural(
                7..10,
                [Label::new(0..6, "label")],
                256u16,
            )])),
            vec![],
        )
    );
}

#[test]
fn multiple() {
    assert_eq!(
        parser().parse_recovery(":label1 :label2 256"),
        (
            Some(Scope::from_exprs([Expr::natural(
                16..19,
                [Label::new(0..7, "label1"), Label::new(8..15, "label2")],
                256u16,
            )])),
            vec![],
        )
    );
}

#[test]
fn multiple_chained() {
    assert_eq!(
        parser().parse_recovery(":x:y:z 256"),
        (
            Some(Scope::from_exprs([Expr::natural(
                7..10,
                [
                    Label::new(0..2, "x"),
                    Label::new(2..4, "y"),
                    Label::new(4..6, "z"),
                ],
                256u16,
            )])),
            vec![],
        )
    );
}

#[test]
fn must_have_name() {
    assert_eq!(
        parser().parse_recovery(": "),
        (
            Some(Scope::from_exprs([])),
            vec![Error::unexpected_char(1..2, ' ')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::LabelsWithExpr)],
        )
    );
}

#[test]
fn must_have_name_in_sexpr() {
    assert_eq!(
        parser().parse_recovery("(: )"),
        (
            Some(Scope::from_exprs([Expr::sexpr(0..4, [], [])])),
            vec![Error::unexpected_char(2..3, ' ')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::LabelsWithExpr)
                .with_label(ErrorLabel::SExpr)
                .with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn must_have_associated_expr() {
    assert_eq!(
        parser().parse_recovery(":label "),
        (
            Some(Scope::from_exprs([])),
            vec![Error::unexpected_end(7).with_label(ErrorLabel::LabelsWithExpr)]
        )
    );
}

#[test]
fn must_have_associated_expr_in_sexpr() {
    assert_eq!(
        parser().parse_recovery("(:label )"),
        (
            Some(Scope::from_exprs([Expr::sexpr(0..9, [], [])])),
            vec![Error::unexpected_end(8)
                .with_label(ErrorLabel::LabelsWithExpr)
                .with_label(ErrorLabel::SExpr)
                .with_label(ErrorLabel::ExprWithPath)]
        )
    );
}

#[test]
fn names_cant_have_colon() {
    assert_eq!(
        parser().parse_recovery(":: 256"),
        (
            Some(Scope::from_exprs([Expr::natural(3..6, [], 256u16)])),
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
            Some(Scope::from_exprs([Expr::natural(3..6, [], 256u16)])),
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
            Some(Scope::from_exprs([Expr::natural(3..6, [], 256u16)])),
            vec![Error::unexpected_char(1..2, ')')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::LabelsWithExpr)],
        )
    );
}
