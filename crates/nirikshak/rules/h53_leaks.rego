# id: H53
# title: Memory leaked on purpose
# why: Leaked memory is never given back, so a long-running service grows.
# fix: Keep the value owned, or borrow it for as long as it is needed.
package nirikshak.h53

import rego.v1

leaks := {"std::mem::forget", "core::mem::forget", "Box::leak", "std::boxed::Box::leak", "ManuallyDrop::new", "std::mem::ManuallyDrop::new", ".leak"}

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": c.path, "line": c.line, "what": c.callee} if {
	some c in input.call
	c.callee in leaks
	in_code(c.path, c.function)
}
