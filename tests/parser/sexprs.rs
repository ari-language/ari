use pretty_assertions::assert_eq;

use ari::{
    ast::{Ast, Expr, Label},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn sexpr() {
    assert_eq!(
        parser().parse_recovery("(* :r 256 :g 256 :b 256)"),
        (
            Some(
                Ast::try_from_exprs([Expr::sexpr(
                    [],
                    0..24,
                    Ast::try_from_exprs([
                        Expr::symbol([], 1..2, "*"),
                        Expr::natural([Label::new(3..5, "r")], 6..9, 256u16),
                        Expr::natural([Label::new(10..12, "g")], 13..16, 256u16),
                        Expr::natural([Label::new(17..19, "b")], 20..23, 256u16),
                    ])
                    .unwrap(),
                )])
                .unwrap()
            ),
            vec![],
        )
    );
}

#[test]
fn empty() {
    assert_eq!(
        parser().parse_recovery("()"),
        (
            Some(
                Ast::try_from_exprs([Expr::sexpr([], 0..2, Ast::try_from_exprs([]).unwrap())])
                    .unwrap()
            ),
            vec![]
        )
    );
}

#[test]
fn empty_with_padding() {
    assert_eq!(
        parser().parse_recovery("( )"),
        (
            Some(
                Ast::try_from_exprs([Expr::sexpr([], 0..3, Ast::try_from_exprs([]).unwrap())])
                    .unwrap()
            ),
            vec![]
        )
    );
}

#[test]
fn cant_have_left_paren() {
    assert_eq!(
        parser().parse_recovery("("),
        (
            Some(
                Ast::try_from_exprs([Expr::sexpr([], 0..1, Ast::try_from_exprs([]).unwrap())])
                    .unwrap()
            ),
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
            Some(Ast::try_from_exprs([]).unwrap()),
            vec![Error::trailing_garbage(0..1)],
        )
    );
}
