//! Tests for the purpose field (spec 0065 "Signing purposes").
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use sutradhar_tijori::purpose::{Purpose, PurposeError};

const MAX: usize = 32;

#[test]
fn accepts_lower_case_letters_digits_and_hyphens() {
    let purpose = Purpose::new("runner-result", MAX);
    assert_eq!(
        purpose.map(|p| p.as_str().to_owned()),
        Ok("runner-result".to_owned())
    );
}

#[test]
fn refuses_an_empty_name() {
    assert_eq!(
        Purpose::new("", MAX),
        Err(PurposeError::BadLength { len: 0, max: MAX })
    );
}

#[test]
fn refuses_a_name_over_the_policy_length() {
    let long = "a".repeat(MAX + 1);
    assert_eq!(
        Purpose::new(&long, MAX),
        Err(PurposeError::BadLength {
            len: MAX + 1,
            max: MAX
        })
    );
}

#[test]
fn accepts_a_name_at_the_policy_length() {
    assert!(Purpose::new(&"a".repeat(MAX), MAX).is_ok());
}

#[test]
fn refuses_capitals_spaces_and_look_alike_letters() {
    assert_eq!(
        Purpose::new("Review", MAX),
        Err(PurposeError::BadCharacter { at: 0 })
    );
    assert_eq!(
        Purpose::new("re view", MAX),
        Err(PurposeError::BadCharacter { at: 2 })
    );
    // A Cyrillic "е" looks like a Latin "e".
    assert_eq!(
        Purpose::new("r\u{0435}view", MAX),
        Err(PurposeError::BadCharacter { at: 1 })
    );
    assert_eq!(
        Purpose::new("review_record", MAX),
        Err(PurposeError::BadCharacter { at: 6 })
    );
}

#[test]
fn prefix_writes_a_big_endian_length_then_the_name_then_the_caller_bytes() {
    let purpose = Purpose::new("waiver", MAX).ok();
    let signed = purpose.map(|p| p.prefix(b"BODY"));
    let mut expected = vec![0, 0, 0, 0, 0, 0, 0, 6];
    expected.extend_from_slice(b"waiver");
    expected.extend_from_slice(b"BODY");
    assert_eq!(signed, Some(expected));
}

#[test]
fn strip_returns_the_caller_bytes_unchanged() {
    let purpose = Purpose::new("gate-run", MAX).unwrap();
    let signed = purpose.prefix(b"\x00\x01rest");
    assert_eq!(purpose.strip(&signed), Ok(&b"\x00\x01rest"[..]));
}

#[test]
fn strip_refuses_bytes_signed_for_another_purpose() {
    let waiver = Purpose::new("waiver", MAX).unwrap();
    let review = Purpose::new("review-record", MAX).unwrap();
    let signed = waiver.prefix(b"BODY");
    assert_eq!(
        review.strip(&signed),
        Err(PurposeError::WrongPurpose {
            expected: "review-record".to_owned()
        })
    );
}

#[test]
fn strip_refuses_a_purpose_that_is_only_a_prefix_of_the_field() {
    // "gate" must not match a field that says "gate-run".
    let gate_run = Purpose::new("gate-run", MAX).unwrap();
    let gate = Purpose::new("gate", MAX).unwrap();
    let signed = gate_run.prefix(b"BODY");
    assert!(matches!(
        gate.strip(&signed),
        Err(PurposeError::WrongPurpose { .. })
    ));
}

#[test]
fn strip_refuses_truncated_bytes() {
    let purpose = Purpose::new("waiver", MAX).unwrap();
    assert_eq!(purpose.strip(&[0, 0, 0]), Err(PurposeError::Truncated));
    assert_eq!(
        purpose.strip(&[0, 0, 0, 0, 0, 0, 0, 6, b'w']),
        Err(PurposeError::Truncated)
    );
}

#[test]
fn strip_refuses_a_length_larger_than_memory_can_hold() {
    let purpose = Purpose::new("waiver", MAX).unwrap();
    let mut signed = vec![0xff; 8];
    signed.extend_from_slice(b"waiver");
    assert_eq!(purpose.strip(&signed), Err(PurposeError::Truncated));
}

#[test]
fn every_error_explains_itself_in_plain_english() {
    let errors = [
        PurposeError::BadLength { len: 0, max: MAX },
        PurposeError::BadCharacter { at: 3 },
        PurposeError::Truncated,
        PurposeError::WrongPurpose {
            expected: "waiver".to_owned(),
        },
    ];
    for error in errors {
        assert_ne!(error.to_string(), "");
    }
}
