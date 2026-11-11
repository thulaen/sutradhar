//! Directly check callbacks whose Serde wiring is a string attribute.

use serde::de::value::{Error, StrDeserializer};

use super::{diagnostic_text, identifier};

#[test]
fn decoded_diagnostics_refuse_controls_without_losing_plain_text() {
    assert_eq!(
        diagnostic_text(StrDeserializer::<Error>::new("pkg:generic/name")),
        Ok("pkg:generic/name".to_owned()),
    );
    for text in ["line\nfeed", "escape\u{1b}", "direction\u{200f}"] {
        assert!(diagnostic_text(StrDeserializer::<Error>::new(text)).is_err());
    }
}

#[test]
fn decoded_identifiers_keep_the_entire_allowance_identity() {
    assert_eq!(
        identifier(StrDeserializer::<Error>::new("CVE-2026-4738")),
        Ok("CVE-2026-4738".to_owned()),
    );
    for text in ["CVE-2026-4738 (other)", "two ids", "id\u{61c}"] {
        assert!(identifier(StrDeserializer::<Error>::new(text)).is_err());
    }
}
