# id: R18
# title: Queue with no limit
# why: A queue with no limit can grow until the computer runs out of memory.
# fix: Use a bounded channel and decide what happens when it is full.
#
# Rule 18 of the approved list. Calls are named through the file's
# imports. Test code is left out.
package nirikshak.r18

import rego.v1

unbounded(callee) if callee == "std::sync::mpsc::channel"

unbounded(callee) if endswith(callee, "::unbounded_channel")

unbounded(callee) if endswith(callee, "::unbounded")

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": c.path, "line": c.line, "what": c.callee} if {
	some c in input.call
	unbounded(c.callee)
	in_code(c.path, c.function)
}
