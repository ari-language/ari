use pretty_assertions::assert_eq;

use ari::{
    ast::{Ast, Expr, Label, Symbol},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn back_reference() {
    assert_eq!(
        parser().parse_recovery(":a 256 :b a"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::natural([Label::new(0..2, "a")], 3..6, 256u16),
                    Expr::resolved_reference([Label::new(7..9, "b")], 10..11, 0, -1, []),
                ])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn forward_reference() {
    assert_eq!(
        parser().parse_recovery(":a b :b 256"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::resolved_reference([Label::new(0..2, "a")], 3..4, 0, 1, []),
                    Expr::natural([Label::new(5..7, "b")], 8..11, 256u16),
                ])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn direct_self_reference() {
    assert_eq!(
        parser().parse_recovery(":a a"),
        (
            Some(
                Ast::try_from_exprs([Expr::resolved_reference(
                    [Label::new(0..2, "a")],
                    3..4,
                    0,
                    0,
                    []
                )])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn nested_self_reference() {
    assert_eq!(
        parser().parse_recovery(":a (+ a 256)"),
        (
            Some(
                Ast::try_from_exprs([Expr::sexpr(
                    [Label::new(0..2, "a")],
                    3..12,
                    Ast::try_from_exprs([
                        Expr::unresolved_symbol([], 4..5, "+"),
                        Expr::resolved_reference([], 6..7, 1, 0, []),
                        Expr::natural([], 8..11, 256u16),
                    ])
                    .unwrap()
                )])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn indirect_self_reference() {
    assert_eq!(
        parser().parse_recovery(":a b :b a"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::resolved_reference([Label::new(0..2, "a")], 3..4, 0, 1, []),
                    Expr::resolved_reference([Label::new(5..7, "b")], 8..9, 0, -1, []),
                ])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn indirect_unresolved_reference() {
    assert_eq!(
        parser().parse_recovery(":a c :b a"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::unresolved_symbol([Label::new(0..2, "a")], 3..4, "c"),
                    Expr::resolved_reference([Label::new(5..7, "b")], 8..9, 0, -1, [])
                ])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn back_reference_in_parent_scope() {
    assert_eq!(
        parser().parse_recovery(":a 256 (* :b a)"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::natural([Label::new(0..2, "a")], 3..6, 256u16),
                    Expr::sexpr(
                        [],
                        7..15,
                        Ast::try_from_exprs([
                            Expr::unresolved_symbol([], 8..9, "*"),
                            Expr::resolved_reference([Label::new(10..12, "b")], 13..14, 1, -1, []),
                        ])
                        .unwrap()
                    )
                ])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn forward_reference_in_parent_scope() {
    assert_eq!(
        parser().parse_recovery("(* :a b) :b 256"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::sexpr(
                        [],
                        0..8,
                        Ast::try_from_exprs([
                            Expr::unresolved_symbol([], 1..2, "*"),
                            Expr::resolved_reference([Label::new(3..5, "a")], 6..7, 1, 1, []),
                        ])
                        .unwrap()
                    ),
                    Expr::natural([Label::new(9..11, "b")], 12..15, 256u16),
                ])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn back_reference_in_sibling_scope() {
    assert_eq!(
        parser().parse_recovery(":a (* :b 256) :b a:b"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::sexpr(
                        [Label::new(0..2, "a")],
                        3..13,
                        Ast::try_from_exprs([
                            Expr::unresolved_symbol([], 4..5, "*"),
                            Expr::natural([Label::new(6..8, "b")], 9..12, 256u16),
                        ])
                        .unwrap()
                    ),
                    Expr::resolved_reference([Label::new(14..16, "b")], 17..20, 0, -1, [1]),
                ])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn forward_reference_in_sibling_scope() {
    assert_eq!(
        parser().parse_recovery(":a b:a :b (* :a 256)"),
        (
            Some(
                Ast::try_from_exprs([
                    Expr::resolved_reference([Label::new(0..2, "a")], 3..6, 0, 1, [1]),
                    Expr::sexpr(
                        [Label::new(7..9, "b")],
                        10..20,
                        Ast::try_from_exprs([
                            Expr::unresolved_symbol([], 11..12, "*"),
                            Expr::natural([Label::new(13..15, "a")], 16..19, 256u16),
                        ])
                        .unwrap()
                    ),
                ])
                .unwrap()
            ),
            vec![],
        ),
    );
}

#[test]
fn invalid_reference_path() {
    assert_eq!(
        parser().parse_recovery("(* :a 256 :b a:b)"),
        (
            Some(
                Ast::try_from_exprs([Expr::sexpr(
                    [],
                    0..17,
                    Ast::try_from_exprs([
                        Expr::unresolved_symbol([], 1..2, "*"),
                        Expr::natural([Label::new(3..5, "a")], 6..9, 256u16),
                        Expr::unresolved_reference(
                            [Label::new(10..12, "b")],
                            13..16,
                            Symbol::new(13..14, "a"),
                            [Label::new(14..16, "b")]
                        )
                    ])
                    .unwrap_or_else(|(_err, ast)| ast)
                )])
                .unwrap()
            ),
            vec![Error::invalid_path(14..16)
                .with_label(ErrorLabel::SExpr)
                .with_label(ErrorLabel::Reference)],
        ),
    );
}

#[test]
fn partially_invalid_reference_path() {
    assert_eq!(
        parser().parse_recovery(":a (* :x (* :y (* :z 256)) :b a:x:y:b)"),
        (
            Some(
                Ast::try_from_exprs([Expr::sexpr(
                    [Label::new(0..2, "a")],
                    3..38,
                    Ast::try_from_exprs([
                        Expr::unresolved_symbol([], 4..5, "*"),
                        Expr::sexpr(
                            [Label::new(6..8, "x")],
                            9..26,
                            Ast::try_from_exprs([
                                Expr::unresolved_symbol([], 10..11, "*"),
                                Expr::sexpr(
                                    [Label::new(12..14, "y")],
                                    15..25,
                                    Ast::try_from_exprs([
                                        Expr::unresolved_symbol([], 16..17, "*"),
                                        Expr::natural([Label::new(18..20, "z")], 21..24, 256u16),
                                    ])
                                    .unwrap()
                                ),
                            ])
                            .unwrap()
                        ),
                        Expr::unresolved_reference(
                            [Label::new(27..29, "b")],
                            30..37,
                            Symbol::new(30..31, "a"),
                            [
                                Label::new(31..33, "x"),
                                Label::new(33..35, "y"),
                                Label::new(35..37, "b")
                            ]
                        )
                    ])
                    .unwrap()
                )])
                .unwrap_or_else(|(_err, ast)| ast)
            ),
            vec![Error::invalid_path(35..37)],
        ),
    );
}
