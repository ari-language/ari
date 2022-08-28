use ari::{
    ast::{Expr, ExprVariant, Label, Scope, Symbol},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn sexpr() {
    assert_eq!(
        parser().parse_recovery("()"),
        (
            Some(Scope::from_iter([Expr::new(
                0..2,
                ExprVariant::SExpr(Scope::from_iter([]))
            )])),
            vec![],
        )
    );

    assert_eq!(
        parser().parse_recovery("( )"),
        (
            Some(Scope::from_iter([Expr::new(
                0..3,
                ExprVariant::SExpr(Scope::from_iter([]))
            )])),
            vec![],
        )
    );

    assert_eq!(
        parser().parse_recovery("(* :r 256 :g 256 :b 256)"),
        (
            Some(Scope::from_iter([Expr::new(
                0..24,
                ExprVariant::SExpr(Scope::from_iter([
                    Expr::new(1..2, ExprVariant::Symbol(Symbol::unresolved("*"))),
                    Expr::new(6..9, ExprVariant::Natural(256u16.into()))
                        .with_labels([Label::new(3..5, "r")]),
                    Expr::new(13..16, ExprVariant::Natural(256u16.into()))
                        .with_labels([Label::new(10..12, "g")]),
                    Expr::new(20..23, ExprVariant::Natural(256u16.into()))
                        .with_labels([Label::new(17..19, "b")]),
                ]))
            )])),
            vec![],
        )
    );

    // TODO: Error recovery for unbalanced parens
    assert_eq!(
        parser().parse_recovery("("),
        (
            None,
            vec![Error::unexpected_end(1)
                .with_label(ErrorLabel::SExpr)
                .with_label(ErrorLabel::ExprWithPath)],
        )
    );

    // TODO: Error recovery for unbalanced parens
    assert_eq!(
        parser().parse_recovery(")"),
        (
            None,
            vec![Error::unexpected_char(0..1, ')')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::ExprWithPath)],
        )
    );
}
