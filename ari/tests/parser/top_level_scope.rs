use ari::parser::{parser, Error, ErrorLabel};
use chumsky::Parser;

#[test]
fn cant_be_empty() {
    assert_eq!(
        parser().parse_recovery(""),
        (
            None,
            vec![Error::unexpected_end(0)
                .with_label(ErrorLabel::Symbol)
                .with_label(ErrorLabel::ExprWithPath)],
        )
    );
}
