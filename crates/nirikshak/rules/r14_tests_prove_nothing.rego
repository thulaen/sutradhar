# id: R14
# title: Test that proves nothing
# why: A test with no check that can fail passes whatever the code does.
# fix: Add an assertion that fails when the behaviour is wrong.
#
# Rule 14 of the approved list. A test checks something when it, or a
# function it calls, uses an assert macro or panic, or uses `?`, which
# fails the test on an error. Functions are matched by name alone, so a
# helper of the same name elsewhere can hide a finding.
package nirikshak.r14

import rego.v1

checks_itself contains m.function if {
	some m in input.macro
	contains(m.name, "assert")
}

checks_itself contains m.function if {
	some m in input.macro
	m.name == "panic"
}

checks_itself contains t.function if {
	some t in input.uses_try
}

last_part(callee) := name if {
	parts := split(trim_prefix(callee, "."), "::")
	name := parts[count(parts) - 1]
}

calls[name] := targets if {
	some f in input.function
	name := f.name
	targets := {t |
		some c in input.call
		c.function == name
		t := last_part(c.callee)
	}
}

findings contains {"path": f.path, "line": f.first, "what": f.name} if {
	some f in input.function
	f.kind == "test"
	reached := graph.reachable(calls, {f.name})
	count(reached & checks_itself) == 0
}
