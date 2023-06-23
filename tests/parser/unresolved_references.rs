use pretty_assertions::assert_eq;

use ari::{
    ast::{Expr, Label, Scope, Symbol},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn apply_path_to_symbol() {
    assert_eq!(
        parser().parse_recovery("symbol:path"),
        (
            Some(
                Scope::try_from_exprs([Expr::unresolved_reference(
                    [],
                    0..11,
                    Symbol::new(0..6, "symbol"),
                    [Label::new(6..11, "path")],
                )])
                .unwrap()
            ),
            vec![],
        )
    );
}

#[test]
fn apply_deep_path_to_symbol() {
    assert_eq!(
        parser().parse_recovery("symbol:x:y:z"),
        (
            Some(
                Scope::try_from_exprs([Expr::unresolved_reference(
                    [],
                    0..12,
                    Symbol::new(0..6, "symbol"),
                    [
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
fn apply_path_to_sexpr() {
    assert_eq!(
        parser().parse_recovery("(* :r 256 :g 256 :b 256):r"),
        (
            Some(Scope::try_from_exprs([Expr::natural([], 0..26, 256u16)]).unwrap()),
            vec![],
        )
    );
}

#[test]
fn path_must_be_complete() {
    assert_eq!(
        parser().parse_recovery("symbol:"),
        (
            Some(Scope::try_from_exprs([]).unwrap()),
            vec![Error::unexpected_end(7)
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::Label)
                .with_label(ErrorLabel::Path)
                .with_label(ErrorLabel::Reference)],
        )
    );
}

#[test]
fn deep_path_must_be_chained() {
    assert_eq!(
        parser().parse_recovery("symbol:x:y :z"),
        (
            Some(
                Scope::try_from_exprs([Expr::unresolved_reference(
                    [],
                    0..10,
                    Symbol::new(0..6, "symbol"),
                    [Label::new(6..8, "x"), Label::new(8..10, "y")],
                )])
                .unwrap()
            ),
            vec![Error::unexpected_end(13).with_label(ErrorLabel::LabelledExpr)],
        )
    );
}

#[test]
fn path_cant_have_left_paren() {
    assert_eq!(
        parser().parse_recovery("symbol:("),
        (
            Some(Scope::try_from_exprs([]).unwrap()),
            vec![
                Error::unexpected_char(7..8, '(')
                    .with_label(ErrorLabel::Symbol)
                    .with_label(ErrorLabel::Label)
                    .with_label(ErrorLabel::Path)
                    .with_label(ErrorLabel::Reference),
                Error::trailing_garbage(7..8),
            ],
        )
    );
}

#[test]
fn path_cant_have_right_paren() {
    assert_eq!(
        parser().parse_recovery("symbol:)"),
        (
            Some(Scope::try_from_exprs([]).unwrap()),
            vec![
                Error::unexpected_char(7..8, ')')
                    .with_label(ErrorLabel::Symbol)
                    .with_label(ErrorLabel::Label)
                    .with_label(ErrorLabel::Path)
                    .with_label(ErrorLabel::Reference),
                Error::trailing_garbage(7..8),
            ],
        )
    );
}

#[test]
fn cant_apply_path_to_natural() {
    assert_eq!(
        parser().parse_recovery("256:x"),
        (
            Some(Scope::try_from_exprs([]).unwrap()),
            vec![Error::invalid_path(3..5).with_label(ErrorLabel::Reference)],
        )
    );
}

#[test]
fn cant_apply_path_to_natural_in_sexpr() {
    assert_eq!(
        parser().parse_recovery("(* :x 256):x:y:b"),
        (
            Some(Scope::try_from_exprs([]).unwrap()),
            vec![Error::invalid_path(12..16).with_label(ErrorLabel::Reference)],
        )
    );
}

#[test]
fn cant_apply_path_to_missing_path_label_in_sexpr() {
    assert_eq!(
        parser().parse_recovery("(* :x (* :y (* :z 256))):x:y:b"),
        (
            Some(Scope::try_from_exprs([]).unwrap()),
            vec![Error::invalid_path(28..30).with_label(ErrorLabel::Reference)],
        )
    );
}
