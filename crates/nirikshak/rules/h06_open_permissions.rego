# id: H06
# title: File anyone may write
# why: Anyone on the computer can change the data written to that file.
# fix: Give the owner (and its group) only what they need, such as 0o600.
#
# A mode set in code whose last digit lets others write (2, 3, 6 or 7).
package nirikshak.h06

import rego.v1

sets_mode(callee) if endswith(callee, "Permissions::from_mode")

sets_mode(callee) if callee in {".mode", ".set_mode"}

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": c.path, "line": c.line, "what": c.text} if {
	some c in input.call_text
	sets_mode(c.callee)
	startswith(c.text, "0o")
	some digit in ["2", "3", "6", "7"]
	endswith(c.text, digit)
	in_code(c.path, c.function)
}
