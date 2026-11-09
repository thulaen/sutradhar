# id: R05
# title: Setting not documented
# why: An environment setting that no page explains cannot be set safely.
# fix: Explain the setting in a page, in a code span such as `NAME`.
#
# Rule 5 of the approved list, and the first that crosses languages: a
# setting read in Rust must be named in a Markdown page. Cargo's own
# settings (CARGO_*) and test code are left out.
package nirikshak.r05

import rego.v1

reads_setting contains "std::env::var"

reads_setting contains "std::env::var_os"

reads_setting contains "std::env::set_var"

setting_macro contains "env"

setting_macro contains "option_env"

not_test(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

not_test(path, "<top>") if {
	some f in input.file
	f.path == path
	f.language == "rust"
	not contains(path, "/tests/")
}

used contains {"path": c.path, "line": c.line, "what": c.text} if {
	some c in input.call_text
	c.callee in reads_setting
	not_test(c.path, c.function)
}

used contains {"path": m.path, "line": m.line, "what": m.text} if {
	some m in input.macro_text
	m.name in setting_macro
	not_test(m.path, m.function)
}

documented contains d.text if {
	some d in input.doc_code
}

findings contains u if {
	some u in used
	not startswith(u.what, "CARGO_")
	not u.what in documented
}
