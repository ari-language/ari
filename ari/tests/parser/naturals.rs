use pretty_assertions::assert_eq;

use std::str::FromStr;

use ari::{
    ast::{Expr, Scope},
    parser::{parser, Error},
};

use chumsky::Parser;
use num_bigint::BigUint;

#[test]
fn bottom() {
    assert_eq!(
        parser().parse_recovery("0"),
        (
            Some(Scope::try_from_exprs([Expr::natural(0..1, [], 0u8)]).scope),
            vec![]
        )
    );
}

#[test]
fn unit() {
    assert_eq!(
        parser().parse_recovery("1"),
        (
            Some(Scope::try_from_exprs([Expr::natural(0..1, [], 1u8)]).scope),
            vec![]
        )
    );
}

#[test]
fn decimal() {
    assert_eq!(
        parser().parse_recovery("256"),
        (
            Some(Scope::try_from_exprs([Expr::natural(0..3, [], 256u16)]).scope),
            vec![],
        )
    );
}

#[test]
fn binary() {
    assert_eq!(
        parser().parse_recovery("0b100000000"),
        (
            Some(Scope::try_from_exprs([Expr::natural(0..11, [], 256u16)]).scope),
            vec![],
        )
    );
}

#[test]
fn octal() {
    assert_eq!(
        parser().parse_recovery("0o400"),
        (
            Some(Scope::try_from_exprs([Expr::natural(0..5, [], 256u16)]).scope),
            vec![],
        )
    );
}

#[test]
fn hexidecimal() {
    assert_eq!(
        parser().parse_recovery("0x100"),
        (
            Some(Scope::try_from_exprs([Expr::natural(0..5, [], 256u16)]).scope),
            vec![],
        )
    );
}

#[test]
fn supports_big_naturals_that_fit_in_memory() {
    assert_eq!(
        parser().parse_recovery(
            "1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890"
        ), (
            Some(Scope::try_from_exprs([Expr::natural(
                0..100,
                [],
                BigUint::from_str("1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890")
                    .unwrap()
            )]).scope),
            vec![],
        )
    );
}

#[test]
fn cant_have_zero_prefix() {
    // TODO: Give better error than just trailing garbage
    assert_eq!(
        parser().parse_recovery("0123456789"),
        (
            Some(Scope::try_from_exprs([Expr::natural(0..1, [], 0u8)]).scope),
            vec![Error::trailing_garbage(1..10)]
        )
    );
}
