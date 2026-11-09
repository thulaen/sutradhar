use super::check;
use crate::file_kinds::Kind;

fn found(text: &str) -> usize {
    check("a.rs", Kind::Rust, text).len()
}

/// Joins parts, so this file holds no trick whole.
fn t(a: &str, b: &str) -> String {
    [a, b].concat()
}

fn planted() -> Vec<String> {
    vec![
        t("kani::ass", "ume(false);"),
        t("ass", "ume ( false );"),
        t("ass", "ume(\n    false\n);"),
        t("ass", "ume(/* x */ false);"),
        t("ass", "ume!(false);"),
        t("#[kani::requ", "ires(false)]"),
        t("requ", "ires false,"),
        t("use kani::ass", "ume as a;"),
        t("adm", "it();"),
        t("#[verifier::exter", "nal_body]"),
        t("#[verifier(exter", "nal_body)]"),
        t("#[verifier::exter", "nal]"),
        t("#![verifier::exter", "nal]"),
        t("#[verifier(exter", "nal)]"),
        t("#[cfg_attr(verus_keep_ghost, verifier::exter", "nal)]"),
        t("#[verus_verify(exter", "nal_body)]"),
        t("#[trust", "ed]"),
        t("#![trust", "ed]"),
        t("#[creusot_contracts::trust", "ed]"),
        t("#[cfg_attr(creusot, trust", "ed)]"),
        t("#[kani::should_", "panic]"),
        t("#[verifier::exec_allows_no_", "decreases_clause]"),
        t("#[kani::unw", "ind(0)]"),
        t("#[kani::st", "ub(a, b)]"),
        t("#[st", "ub(a, b)]"),
        t("pub ass", "ume_specification fn f();"),
        t("external_fn_", "specification! { fn f(); }"),
        t("external_type_", "specification! { struct S; }"),
        t("external_trait_", "specification! { trait T {} }"),
        t("extern_", "spec! { fn f(); }"),
    ]
}

#[test]
fn every_known_trick_is_refused() {
    for trick in planted() {
        let text = format!("fn f() {{\n    {trick}\n}}\n");
        assert!(found(&text) >= 1, "{trick}");
    }
}

#[test]
fn a_trick_is_found_on_its_own_line() {
    let text = format!("fn f() {{}}\n\n{}\n", t("adm", "it();"));
    let lines: Vec<usize> = check("a.rs", Kind::Rust, &text)
        .iter()
        .map(|f| f.line)
        .collect();
    assert_eq!(lines, [3]);
}

#[test]
fn honest_code_and_things_that_only_look_like_tricks_pass() {
    let honest = [
        "#[kani::proof]\n#[kani::unwind(8)]\nfn check() {\n    \
         let x: u8 = kani::any();\n    kani::assume(x < 10);\n}\n"
            .to_owned(),
        "fn f(q: Q) { q.admit(); }\n".to_owned(),
        "#[kani::stub_verified(f)]\nfn g() {}\n".to_owned(),
        format!(
            "// {}\n/* {} */\n",
            t("ass", "ume(false)"),
            t("#[trust", "ed]")
        ),
        format!(
            "let s = \"{} {}\";\n",
            t("ass", "ume(false)"),
            t("adm", "it()")
        ),
        "let unwind = loop_unwind(0);\n".to_owned(),
        "#[derive(Debug)]\nstruct Stubborn;\n".to_owned(),
    ];
    for text in honest {
        assert_eq!(found(&text), 0, "{text}");
    }
}

#[test]
fn only_rust_files_are_read() {
    let trick = planted().remove(0);
    assert_eq!(check("a.md", Kind::Markdown, &trick).len(), 0);
}

#[test]
fn literals_cannot_hide_a_trick() {
    let tricks = [
        format!("let q = '\"';\n{}\nlet r = '\"';\n", t("adm", "it();")),
        format!("let q = b'\"';\n{}\nlet r = b'\"';\n", t("adm", "it();")),
        format!("let q = r\"\\\";\n{}\nlet r = \"\";\n", t("adm", "it();")),
        format!("let q = r#\"\"#;\n{}\n", t("adm", "it();")),
        format!("let q = br\"\\\";\n{}\nlet r = \"\";\n", t("adm", "it();")),
    ];
    for text in tricks {
        assert_eq!(found(&text), 1, "{text}");
    }
}

#[test]
fn zero_in_any_spelling_and_a_bound_assume_are_refused() {
    let tricks = [
        t("#[kani::unw", "ind(00)]"),
        t("#[kani::unw", "ind(0x0)]"),
        t("#[kani::unw", "ind(0_u32)]"),
        t("#[kani::unw", "ind(0b0usize)]"),
        t("let f = kani::ass", "ume;"),
        t("g(kani::ass", "ume, x);"),
    ];
    for trick in tricks {
        assert!(found(&trick) >= 1, "{trick}");
    }
    let honest = [
        t("use kani::{any, ass", "ume};"),
        t("#[kani::unw", "ind(10)]"),
        t("#[kani::unw", "ind(0x10)]"),
    ];
    for text in honest {
        assert_eq!(found(&text), 0, "{text}");
    }
}
