use pretty_assertions::assert_eq;

use ari::{
    ast::{Ast, Expr, Label},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn single() {
    assert_eq!(
        parser().parse_recovery(":label 256"),
        (
            Some(
                Ast::try_from_exprs([Expr::natural([Label::new(0..6, "label")], 7..10, 256u16)])
                    .ast
            ),
            vec![],
        )
    );
}

#[test]
fn multiple() {
    assert_eq!(
        parser().parse_recovery(":label1 :label2 256"),
        (
            Some(
                Ast::try_from_exprs([Expr::natural(
                    [Label::new(0..7, "label1"), Label::new(8..15, "label2")],
                    16..19,
                    256u16,
                )])
                .ast
            ),
            vec![],
        )
    );
}

#[test]
fn multiple_chained() {
    assert_eq!(
        parser().parse_recovery(":x:y:z 256"),
        (
            Some(
                Ast::try_from_exprs([Expr::natural(
                    [
                        Label::new(0..2, "x"),
                        Label::new(2..4, "y"),
                        Label::new(4..6, "z"),
                    ],
                    7..10,
                    256u16,
                )])
                .ast
            ),
            vec![],
        )
    );
}

#[test]
fn names_cant_have_colon() {
    assert_eq!(
        parser().parse_recovery(":: 256"),
        (
            Some(Ast::try_from_exprs([Expr::natural([], 3..6, 256u16)]).ast),
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
            Some(Ast::try_from_exprs([Expr::natural([], 3..6, 256u16)]).ast),
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
            Some(Ast::try_from_exprs([Expr::natural([], 3..6, 256u16)]).ast),
            vec![Error::unexpected_char(1..2, ')')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::LabelsWithExpr)],
        )
    );
}

#[test]
fn must_have_name() {
    assert_eq!(
        parser().parse_recovery(": "),
        (
            Some(Ast::try_from_exprs([]).ast),
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
            Some(Ast::try_from_exprs([Expr::sexpr([], 0..4, Ast::try_from_exprs([]).ast)]).ast),
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
            Some(Ast::try_from_exprs([]).ast),
            vec![Error::unexpected_end(7).with_label(ErrorLabel::LabelsWithExpr)]
        )
    );
}

#[test]
fn must_have_associated_expr_in_sexpr() {
    assert_eq!(
        parser().parse_recovery("(:label )"),
        (
            Some(Ast::try_from_exprs([Expr::sexpr([], 0..9, Ast::try_from_exprs([]).ast)]).ast),
            vec![Error::unexpected_end(8)
                .with_label(ErrorLabel::LabelsWithExpr)
                .with_label(ErrorLabel::SExpr)
                .with_label(ErrorLabel::ExprWithPath)]
        )
    );
}

#[test]
fn must_be_unique_same_expr() {
    assert_eq!(
        parser().parse_recovery(":label1 :label1 256"),
        (
            Some(
                Ast::try_from_exprs([Expr::natural(
                    [Label::new(0..7, "label1"), Label::new(8..15, "label1")],
                    16..19,
                    256u16
                )])
                .ast
            ),
            vec![Error::duplicate_label(8..15, 0..7)],
        )
    );
}

#[test]
fn must_be_unique_different_expr() {
    assert_eq!(
        parser().parse_recovery(":label1 256 :label1 256"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::natural([Label::new(0..7, "label1")], 8..11, 256u16),
                    Expr::natural([Label::new(12..19, "label1")], 20..23, 256u16)
                ])
                .ast
            ),
            vec![Error::duplicate_label(12..19, 0..7)],
        )
    );
}
