# id: H11
# title: File written without a buffer
# why: Each write to a bare file is its own system call, many times slower.
# fix: Wrap the file in std::io::BufWriter, and flush it at the end.
#
# A function that opens a file and writes to it more than once with
# write! or writeln!, with no buffer.
package nirikshak.h11

import rego.v1

opens := {"std::fs::File::create", "std::fs::File::options", "std::fs::OpenOptions::new"}

buffers(path, name) if {
	some c in input.call
	c.path == path
	c.function == name
	some end in ["BufWriter::new", "LineWriter::new", "BufWriter::with_capacity"]
	endswith(c.callee, end)
}

writes(path, name) := {m.line |
	some m in input.macro
	m.path == path
	m.function == name
	m.name in {"write", "writeln"}
}

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": c.path, "line": c.line, "what": c.callee} if {
	some c in input.call
	c.callee in opens
	count(writes(c.path, c.function)) > 1
	not buffers(c.path, c.function)
	in_code(c.path, c.function)
}
