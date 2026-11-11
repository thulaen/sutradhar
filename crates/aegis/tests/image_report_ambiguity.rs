//! Scanner input cannot erase a hole or borrow another hole's allowance.

use aegis::images::{allowed, findings, holes};

fn report(entry: &str) -> String {
    format!(r#"{{"bomFormat":"CycloneDX","vulnerabilities":[{entry}]}}"#)
}

#[test]
fn duplicate_decision_fields_are_refused() {
    let entries = [
        r#"{"id":"NEW","id":"CVE-2026-4738"}"#,
        r#"{"ratings":[{"severity":"high"}],"ratings":[]}"#,
        r#"{"affects":[{"ref":"new"}],"affects":[]}"#,
        r#"{"ratings":[{"severity":"high","severity":"low"}]}"#,
        r#"{"ratings":[{"severity":"high","sever\u0069ty":"low"}]}"#,
        r#"{"affects":[{"ref":"new","ref":"old"}]}"#,
    ];
    for entry in entries {
        assert!(holes(&report(entry)).is_err(), "accepted {entry}");
    }
    for input in [
        r#"{"bomFormat":"other","bomFormat":"CycloneDX"}"#,
        concat!(
            r#"{"bomFormat":"CycloneDX","vulnerabilities":["#,
            r#"{"ratings":[{"severity":"high"}]}],"vulnerabilities":[]}"#,
        ),
    ] {
        assert!(holes(input).is_err(), "accepted {input}");
    }
}

#[test]
fn invalid_severity_names_are_refused() {
    for severity in ["high ", " critical", "CRIT1CAL", "", "hіgh"] {
        let entry = serde_json::json!({"ratings": [{"severity": severity}]});
        assert!(holes(&report(&entry.to_string())).is_err());
    }
}

#[test]
fn supported_severities_and_missing_ratings_remain_valid() {
    for severity in [
        "none", "info", "low", "medium", "high", "critical", "unknown", "High",
        "CRITICAL",
    ] {
        let entry = serde_json::json!({"ratings": [{"severity": severity}]});
        assert!(holes(&report(&entry.to_string())).is_ok());
    }
    assert!(holes(&report(r#"{"ratings":[{"score":0.1}]}"#)).is_ok());
}

#[test]
fn arrays_cannot_replace_consumed_objects() {
    for input in [
        r#"["CycloneDX",[]]"#.to_owned(),
        report("[]"),
        report(r#"{"ratings":[[]]}"#),
        report(r#"{"affects":[[]]}"#),
    ] {
        assert!(holes(&input).is_err(), "accepted array object: {input}");
    }
}

#[test]
fn expanded_diagnostics_are_bounded_before_formatting() {
    let entry = serde_json::json!({
        "id": "x".repeat(262_144),
        "ratings": [{"severity": "high"}],
        "affects": vec![serde_json::json!({"ref": "x"}); 8],
    });
    assert!(holes(&report(&entry.to_string())).is_err());
    let entry = serde_json::json!({
        "ratings": [{"severity": "critical"}],
        "affects": vec![serde_json::json!({"ref": "x"}); 4097],
    });
    assert!(holes(&report(&entry.to_string())).is_err());
}

#[test]
fn diagnostic_byte_and_count_boundaries_are_exact() {
    for extra in [0, 1] {
        let entry = serde_json::json!({
            "id": "x".repeat(1_048_576 - 12 + extra),
            "ratings": [{"severity": "high"}], "affects": [{"ref": "x"}],
        });
        let result = holes(&report(&entry.to_string()));
        if extra == 0 {
            assert_eq!(result.map(|v| v[0].len()), Ok(1_048_576));
        } else {
            assert!(result.is_err());
        }
        let entry = serde_json::json!({
            "ratings": [{"severity": "critical"}],
            "affects": vec![serde_json::json!({"ref": "x"}); 4096 + extra],
        });
        assert_eq!(holes(&report(&entry.to_string())).is_ok(), extra == 0);
    }
}

#[test]
fn controls_in_consumed_text_are_refused() {
    for control in [
        '\n', '\r', '\t', '\u{1b}', '\u{85}', '\u{61c}', '\u{200e}',
        '\u{200f}', '\u{2028}', '\u{2029}', '\u{202a}', '\u{202b}', '\u{202c}',
        '\u{202d}', '\u{202e}', '\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}',
    ] {
        for entry in [
            serde_json::json!({
                "id": format!("CVE-NEW{control}"),
                "ratings": [{"severity":"high"}],
            }),
            serde_json::json!({
                "affects": [{"ref": format!("pkg:generic/name{control}")}],
            }),
        ] {
            assert!(holes(&report(&entry.to_string())).is_err());
        }
    }
}

#[test]
fn an_allowance_requires_the_whole_vulnerability_id() {
    let allowed = [allowed::Entry {
        image: "forgejo".into(),
        id: "CVE-2026-4738".into(),
        task: 130,
        until: allowed::LAST_DAY.into(),
    }];
    for id in [
        "CVE-2026-4738X",
        "XCVE-2026-4738",
        "CVE-2026-4738%20",
        "CVE-2026-4738:NEW",
    ] {
        let entry = serde_json::json!({
            "id": id, "ratings": [{"severity": "high"}],
        });
        let found = findings(&report(&entry.to_string())).unwrap_or_default();
        assert_eq!(found.len(), 1);
        let mut seen = Vec::new();
        let kept = allowed::not_allowed("forgejo", found, &allowed, &mut seen);
        assert_eq!(kept.len(), 1);
        assert_ne!(seen[0].1, allowed[0].id);
    }
    let entry = serde_json::json!({
        "id": "CVE-2026-4738 (unrelated)", "ratings": [{"severity":"high"}],
    });
    assert!(holes(&report(&entry.to_string())).is_err());
}

#[test]
fn reference_text_does_not_change_an_allowance_id() {
    let permitted = [allowed::Entry {
        image: "forgejo".into(),
        id: "CVE-2026-4738".into(),
        task: 130,
        until: allowed::LAST_DAY.into(),
    }];
    for reference in [
        "pkg:generic/a",
        "pkg:generic/a (critical) in b",
        "pkg:generic/a (high) in b",
        "pkg:generic/a (extra)",
    ] {
        let entry = serde_json::json!({
            "id": "CVE-2026-4738", "ratings": [{"severity": "high"}],
            "affects": [{"ref": reference}],
        });
        let found = findings(&report(&entry.to_string())).unwrap_or_default();
        assert_eq!(found.len(), 1);
        let mut seen = Vec::new();
        assert_eq!(
            allowed::not_allowed("forgejo", found, &permitted, &mut seen,),
            Vec::<String>::new()
        );
        assert_eq!(seen, [("forgejo".into(), "CVE-2026-4738".into())]);
    }
}
