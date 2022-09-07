use pretty_assertions::assert_eq;

use ari::{
    ast::{Expr, Label, Scope},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn sexpr() {
    assert_eq!(
        parser().parse_recovery("(* :r 256 :g 256 :b 256)"),
        (
            Some(Scope::from_exprs([Expr::sexpr(
                0..24,
                [],
                [
                    Expr::symbol(1..2, [], "*"),
                    Expr::natural(6..9, [Label::new(3..5, "r")], 256u16),
                    Expr::natural(13..16, [Label::new(10..12, "g")], 256u16),
                    Expr::natural(20..23, [Label::new(17..19, "b")], 256u16),
                ],
            )])),
            vec![],
        )
    );
}

#[test]
fn empty() {
    assert_eq!(
        parser().parse_recovery("()"),
        (Some(Scope::from_exprs([Expr::sexpr(0..2, [], [])])), vec![])
    );
}

#[test]
fn empty_with_padding() {
    assert_eq!(
        parser().parse_recovery("( )"),
        (Some(Scope::from_exprs([Expr::sexpr(0..3, [], [])])), vec![])
    );
}

#[test]
fn cant_have_left_paren() {
    assert_eq!(
        parser().parse_recovery("("),
        (
            Some(Scope::from_exprs([Expr::sexpr(0..1, [], [])])),
            vec![Error::unexpected_end(1)
                .with_label(ErrorLabel::SExpr)
                .with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn cant_have_right_paren() {
    assert_eq!(
        parser().parse_recovery(")"),
        (
            Some(Scope::from_exprs([])),
            vec![Error::trailing_garbage(0..1)],
        )
    );
}
