# id: M04
# title: Very large struct or enum
# why: A struct with many fields, or an enum with many variants, usually holds several ideas that change for different reasons.
# fix: Split it into smaller types, each holding one idea.
package nirikshak.m04

import rego.v1

limit := {"struct": 12, "enum": 30}

findings contains {"path": t.path, "line": t.line, "what": t.name} if {
	some t in input.shape
	t.kind == "code"
	t.size > limit[t.what]
}
