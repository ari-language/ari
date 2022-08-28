use std::str::FromStr;

use ari::{
    ast::{Expr, ExprVariant, Scope},
    parser::{parser, Error},
};

use chumsky::Parser;
use num_bigint::BigUint;

#[test]
fn bottom() {
    assert_eq!(
        parser().parse_recovery("0"),
        (
            Some(Scope::from_iter([Expr::new(
                0..1,
                ExprVariant::Natural(0u8.into())
            )])),
            vec![],
        )
    );
}

#[test]
fn unit() {
    assert_eq!(
        parser().parse_recovery("1"),
        (
            Some(Scope::from_iter([Expr::new(
                0..1,
                ExprVariant::Natural(1u8.into())
            )])),
            vec![],
        )
    );
}

#[test]
fn decimal() {
    assert_eq!(
        parser().parse_recovery("256"),
        (
            Some(Scope::from_iter([Expr::new(
                0..3,
                ExprVariant::Natural(256u16.into())
            )])),
            vec![],
        )
    );
}

#[test]
#[ignore = "unimplemented"]
fn binary() {
    assert_eq!(
        parser().parse_recovery("0b11111111"),
        (
            Some(Scope::from_iter([Expr::new(
                0..10,
                ExprVariant::Natural(256u16.into())
            )])),
            vec![],
        )
    );
}

#[test]
#[ignore = "unimplemented"]
fn octal() {
    assert_eq!(
        parser().parse_recovery("0o400"),
        (
            Some(Scope::from_iter([Expr::new(
                0..5,
                ExprVariant::Natural(256u16.into())
            )])),
            vec![],
        )
    );
}

#[test]
#[ignore = "unimplemented"]
fn hexidecimal() {
    assert_eq!(
        parser().parse_recovery("0xFF"),
        (
            Some(Scope::from_iter([Expr::new(
                0..4,
                ExprVariant::Natural(256u16.into())
            )])),
            vec![],
        )
    );
}

#[test]
fn supports_big_naturals_that_fit_in_memory() {
    assert_eq!(parser().parse_recovery(
        "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"
    ), (
        Some(Scope::from_iter([
            Expr::new(0..100, ExprVariant::Natural(
                BigUint::from_str("1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890")
                    .unwrap()
                    .into()
            )),
        ])),
        vec![],
    ));
}

#[test]
fn cant_have_zero_prefix() {
    // TODO: Error recovery by skipping over to end of next whitespace
    assert_eq!(
        parser().parse_recovery("0123456789"),
        (None, vec![Error::unexpected_char(1..2, '1')],)
    );
}
