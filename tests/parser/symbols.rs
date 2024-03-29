use pretty_assertions::assert_eq;

use ari::{
    ast::{Expr, Scope},
    parser::{parser, Error, ErrorLabel},
};

use chumsky::Parser;

#[test]
fn symbol() {
    assert_eq!(
        parser().parse_recovery("symbol"),
        (
            Some(Scope::try_from_exprs([Expr::unresolved_symbol([], 0..6, "symbol")]).unwrap()),
            vec![],
        )
    );
}

#[test]
fn supports_builtin_fn_names() {
    #[rustfmt::skip]
    let builtin_fn_names = [
        "=",
        "+", "-",
        "*", "/",
        "^", "log", "root",
        ".",
        "|", "~",
        "&", "!",
        "..",
    ];

    let (left, right): (Vec<_>, Vec<_>) = builtin_fn_names
        .into_iter()
        .map(|symbol| {
            (
                parser().parse_recovery(symbol),
                (
                    Some(
                        Scope::try_from_exprs([Expr::unresolved_symbol(
                            [],
                            0..symbol.len(),
                            symbol,
                        )])
                        .unwrap(),
                    ),
                    vec![],
                ),
            )
        })
        .unzip();

    assert_eq!(left, right);
}

#[test]
fn supports_almost_all_of_unicode_with_exceptions() {
    assert_eq!(
        parser().parse_recovery("🙃"),
        (
            Some(Scope::try_from_exprs([Expr::unresolved_symbol([], 0..1, "🙃")]).unwrap()),
            vec![]
        )
    );
}

#[test]
fn cant_have_whitespace() {
    assert_eq!(
        parser().parse_recovery("symbol "),
        (
            Some(Scope::try_from_exprs([Expr::unresolved_symbol([], 0..6, "symbol")]).unwrap()),
            vec![],
        )
    );
}

#[test]
fn cant_have_colon() {
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
fn cant_have_left_paren() {
    assert_eq!(
        parser().parse_recovery("symbol("),
        (
            Some(Scope::try_from_exprs([Expr::unresolved_symbol([], 0..6, "symbol")]).unwrap()),
            vec![Error::trailing_garbage(6..7)],
        )
    );
}

#[test]
fn cant_have_right_paren() {
    assert_eq!(
        parser().parse_recovery("symbol)"),
        (
            Some(Scope::try_from_exprs([Expr::unresolved_symbol([], 0..6, "symbol")]).unwrap()),
            vec![Error::trailing_garbage(6..7)],
        ),
    );
}
