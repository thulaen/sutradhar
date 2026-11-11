//! Malformed scanner data must never stand for a clean image.

use aegis::images::holes;

#[test]
fn malformed_vulnerability_lists_are_refused() {
    for value in [
        "null",
        "{}",
        "true",
        "0",
        "\"hidden\"",
        "[null]",
        "[1]",
        "[\"hidden\"]",
    ] {
        let report =
            format!(r#"{{"bomFormat":"CycloneDX","vulnerabilities":{value}}}"#);
        assert!(holes(&report).is_err(), "accepted malformed list: {value}");
    }
}
#[test]
fn absent_and_empty_lists_remain_clean() {
    for report in [
        r#"{"bomFormat":"CycloneDX"}"#,
        r#"{"bomFormat":"CycloneDX","vulnerabilities":[]}"#,
    ] {
        assert!(holes(report).is_ok_and(|found| found.is_empty()));
    }
}

#[test]
fn malformed_consumed_vulnerability_fields_are_refused() {
    for entry in [
        r#"{"ratings":null}"#,
        r#"{"ratings":{}}"#,
        r#"{"ratings":[null]}"#,
        r#"{"ratings":[true]}"#,
        r#"{"ratings":[{"severity":null}]}"#,
        r#"{"ratings":[{"severity":7}]}"#,
        r#"{"id":null}"#,
        r#"{"affects":null}"#,
        r#"{"affects":{}}"#,
        r#"{"affects":[null]}"#,
        r#"{"affects":[{"ref":7}]}"#,
    ] {
        let report = format!(
            r#"{{"bomFormat":"CycloneDX","vulnerabilities":[{entry}]}}"#
        );
        assert!(holes(&report).is_err(), "accepted malformed entry: {entry}");
    }
}
