# id: H42
# title: Property test with too few cases
# why: A property tried on a handful of inputs rarely meets the one that breaks.
# fix: Keep the default of 256 cases, or set more; never fewer than ten.
package nirikshak.h42

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": c.text} if {
	some c in input.call_text
	endswith(c.callee, "with_cases")
	c.text in {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}
}
