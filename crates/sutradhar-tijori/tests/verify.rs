//! Tests for the strict Ed25519 check, with the test vectors of RFC 8032
//! section 7.1.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use sutradhar_tijori::verify::{VerifyError, verify_strict};

fn hex<const N: usize>(text: &str) -> [u8; N] {
    let bytes: Vec<u8> = text
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
        })
        .collect();
    bytes.try_into().unwrap()
}

struct Vector {
    public_half: &'static str,
    message: &'static [u8],
    signature: &'static str,
}

const RFC8032: [Vector; 3] = [
    Vector {
        public_half: "d75a980182b10ab7d54bfed3c964073a\
                      0ee172f3daa62325af021a68f707511a",
        message: b"",
        signature: "e5564300c360ac729086e2cc806e828a\
                    84877f1eb8e5d974d873e06522490155\
                    5fb8821590a33bacc61e39701cf9b46b\
                    d25bf5f0595bbe24655141438e7a100b",
    },
    Vector {
        public_half: "3d4017c3e843895a92b70aa74d1b7ebc\
                      9c982ccf2ec4968cc0cd55f12af4660c",
        message: b"\x72",
        signature: "92a009a9f0d4cab8720e820b5f642540\
                    a2b27b5416503f8fb3762223ebdb69da\
                    085ac1e43e15996e458f3613d0f11d8c\
                    387b2eaeb4302aeeb00d291612bb0c00",
    },
    Vector {
        public_half: "fc51cd8e6218a1a38da47ed00230f058\
                      0816ed13ba3303ac5deb911548908025",
        message: b"\xaf\x82",
        signature: "6291d657deec24024827e69c3abe01a3\
                    0ce548a284743a445e3680d7db5ac3ac\
                    18ff9b538d16f290ae67f760984dc659\
                    4a7c15e9716ed28dc027beceea1ec40a",
    },
];

fn signature(text: &str) -> [u8; 64] {
    hex(&text.replace(' ', ""))
}

#[test]
fn accepts_every_rfc_8032_vector() {
    for v in &RFC8032 {
        assert_eq!(
            verify_strict(
                &hex(v.public_half),
                v.message,
                &signature(v.signature)
            ),
            Ok(())
        );
    }
}

#[test]
fn refuses_altered_bytes() {
    let v = &RFC8032[1];
    assert_eq!(
        verify_strict(&hex(v.public_half), b"\x73", &signature(v.signature)),
        Err(VerifyError::BadSignature)
    );
}

#[test]
fn refuses_a_signature_from_another_key() {
    let (a, b) = (&RFC8032[1], &RFC8032[2]);
    assert_eq!(
        verify_strict(&hex(b.public_half), a.message, &signature(a.signature)),
        Err(VerifyError::BadSignature)
    );
}

#[test]
fn refuses_a_non_canonical_signature() {
    // Adding the group order L to S gives a second encoding of the same
    // signature; strict checking refuses it.
    const L: [u8; 32] = [
        0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2,
        0xde, 0xf9, 0xde, 0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0x10,
    ];
    let v = &RFC8032[0];
    let mut sig = signature(v.signature);
    let mut carry = 0u16;
    for (i, l) in L.iter().enumerate() {
        let byte = sig.get_mut(i.checked_add(32).unwrap()).unwrap();
        let sum = u16::from(*byte)
            .checked_add(u16::from(*l))
            .and_then(|s| s.checked_add(carry))
            .unwrap();
        *byte = u8::try_from(sum & 0xff).unwrap();
        carry = sum >> 8;
    }
    assert_eq!(
        verify_strict(&hex(v.public_half), v.message, &sig),
        Err(VerifyError::BadSignature)
    );
}

#[test]
fn refuses_a_small_order_public_key() {
    // The neutral point (encoded as 1 then zeros) has order 1.
    let mut weak = [0u8; 32];
    weak[0] = 1;
    assert_eq!(
        verify_strict(&weak, b"", &[0u8; 64]),
        Err(VerifyError::WeakKey)
    );
}

#[test]
fn refuses_bytes_that_are_not_a_curve_point() {
    // y = 2 is not on the curve for either sign of x.
    let mut bad = [0u8; 32];
    bad[0] = 2;
    assert_eq!(
        verify_strict(&bad, b"", &[0u8; 64]),
        Err(VerifyError::BadKey)
    );
}

#[test]
fn every_error_explains_itself_in_plain_english() {
    for error in [
        VerifyError::BadKey,
        VerifyError::WeakKey,
        VerifyError::BadSignature,
    ] {
        assert_ne!(error.to_string(), "");
    }
}
