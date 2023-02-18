use pretty_assertions::assert_eq;

use ari::{
    ast::{Ast, Expr, Label},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn applied_to_symbol() {
    assert_eq!(
        parser().parse_recovery("symbol:path"),
        (
            Some(
                Ast::try_from_exprs([Expr::path(
                    [],
                    0..11,
                    [Label::new(0..6, "symbol"), Label::new(6..11, "path")],
                )])
                .unwrap()
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
                Ast::try_from_exprs([Expr::path(
                    [],
                    0..12,
                    [
                        Label::new(0..6, "symbol"),
                        Label::new(6..8, "x"),
                        Label::new(8..10, "y"),
                        Label::new(10..12, "z"),
                    ],
                )])
                .unwrap()
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
            Some(Ast::try_from_exprs([Expr::natural([], 0..26, 256u16)]).unwrap()),
            vec![],
        )
    );
}

#[test]
fn must_be_complete() {
    assert_eq!(
        parser().parse_recovery("symbol:"),
        (
            Some(Ast::try_from_exprs([]).unwrap()),
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
                Ast::try_from_exprs([Expr::path(
                    [],
                    0..10,
                    [
                        Label::new(0..6, "symbol"),
                        Label::new(6..8, "x"),
                        Label::new(8..10, "y"),
                    ],
                )])
                .unwrap()
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
            Some(Ast::try_from_exprs([]).unwrap()),
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
            Some(Ast::try_from_exprs([]).unwrap()),
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
            Some(Ast::try_from_exprs([]).unwrap()),
            vec![Error::invalid_path(3..5).with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn cant_apply_to_sexpr_natural() {
    assert_eq!(
        parser().parse_recovery("(* :x 256):x:y:b"),
        (
            Some(Ast::try_from_exprs([]).unwrap()),
            vec![Error::invalid_path(12..16).with_label(ErrorLabel::ExprWithPath)],
        )
    );
}

#[test]
fn cant_apply_to_sexpr_missing_path_label() {
    assert_eq!(
        parser().parse_recovery("(* :x (* :y (* :z 256))):x:y:b"),
        (
            Some(Ast::try_from_exprs([]).unwrap()),
            vec![Error::invalid_path(28..30).with_label(ErrorLabel::ExprWithPath)],
        )
    );
}
