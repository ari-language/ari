use pretty_assertions::assert_eq;

use ari::{
    ast::{Expr, Label, Scope},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn applied_to_symbol() {
    assert_eq!(
        parser().parse_recovery("symbol:path"),
        (
            Some(
                Scope::try_from_exprs([Expr::path(
                    [],
                    0..11,
                    [Label::new(0..6, "symbol"), Label::new(6..11, "path")],
                )])
                .scope
            ),
            vec![],
        )
    );
}

#[test]
fn applied_to_symbol_chained() {
    assert_eq!(
        parser().parse_recovery("symbol:x:y:z"),
        (
            Some(
                Scope::try_from_exprs([Expr::path(
                    [],
                    0..12,
                    [
                        Label::new(0..6, "symbol"),
                        Label::new(6..8, "x"),
                        Label::new(8..10, "y"),
                        Label::new(10..12, "z"),
                    ],
                )])
                .scope
            ),
            vec![],
        )
    );
}

#[test]
fn applied_to_sexpr() {
    assert_eq!(
        parser().parse_recovery("(* :r 256 :g 256 :b 256):r"),
        (
            Some(Scope::try_from_exprs([Expr::natural([], 0..26, 256u16)]).scope),
            vec![],
        )
    );
}

#[test]
fn applied_to_sexpr_symbol() {
    assert_eq!(
        parser().parse_recovery("(* r g b):r"),
        (
            Some(Scope::try_from_exprs([Expr::symbol([], 0..11, "r")]).scope),
            vec![]
        )
    );
}

#[test]
fn applied_to_sexpr_symbol_chained() {
    assert_eq!(
        parser().parse_recovery("(* x):x:a:b:c"),
        (
            Some(
                Scope::try_from_exprs([Expr::path(
                    [],
                    0..13,
                    [
                        Label::new(3..4, "x"),
                        Label::new(7..9, "a"),
                        Label::new(9..11, "b"),
                        Label::new(11..13, "c"),
                    ],
                )])
                .scope
            ),
            vec![],
        )
    );
}

#[test]
fn applied_to_sexpr_path_chained() {
    assert_eq!(
        parser().parse_recovery("(* x:y:z):z:a:b:c"),
        (
            Some(
                Scope::try_from_exprs([Expr::path(
                    [],
                    0..17,
                    [
                        Label::new(3..4, "x"),
                        Label::new(4..6, "y"),
                        Label::new(6..8, "z"),
                        Label::new(11..13, "a"),
                        Label::new(13..15, "b"),
                        Label::new(15..17, "c"),
                    ],
                )])
                .scope
            ),
            vec![],
        )
    );
}

#[test]
fn must_be_complete() {
    assert_eq!(
        parser().parse_recovery("symbol:"),
        (
            Some(Scope::try_from_exprs([]).scope),
            vec![Error::unexpected_end(7)
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::Path)
                .with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn multiple_must_be_chained() {
    assert_eq!(
        parser().parse_recovery("symbol:x:y :z"),
        (
            Some(
                Scope::try_from_exprs([Expr::path(
                    [],
                    0..10,
                    [
                        Label::new(0..6, "symbol"),
                        Label::new(6..8, "x"),
                        Label::new(8..10, "y"),
                    ],
                )])
                .scope
            ),
            vec![Error::unexpected_end(13).with_label(ErrorLabel::LabelsWithExpr)],
        )
    );
}

#[test]
fn cant_have_left_paren() {
    assert_eq!(
        parser().parse_recovery("symbol:("),
        (
            Some(Scope::try_from_exprs([]).scope),
            vec![Error::unexpected_char(7..8, '(')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::Path)
                .with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn cant_have_right_paren() {
    assert_eq!(
        parser().parse_recovery("symbol:)"),
        (
            Some(Scope::try_from_exprs([]).scope),
            vec![Error::unexpected_char(7..8, ')')
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::Path)
                .with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn cant_apply_to_natural() {
    assert_eq!(
        parser().parse_recovery("256:x"),
        (
            Some(Scope::try_from_exprs([]).scope),
            vec![Error::invalid_path(3..5).with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn cant_apply_to_sexpr_natural() {
    assert_eq!(
        parser().parse_recovery("(* :x 256):x:y:b"),
        (
            Some(Scope::try_from_exprs([]).scope),
            vec![Error::invalid_path(12..16).with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn cant_apply_to_sexpr_missing_path_label() {
    assert_eq!(
        parser().parse_recovery("(* :x (* :y (* :z 256))):x:y:b"),
        (
            Some(Scope::try_from_exprs([]).scope),
            vec![Error::invalid_path(28..30).with_label(ErrorLabel::ExprWithPath)],
        )
    );
}
