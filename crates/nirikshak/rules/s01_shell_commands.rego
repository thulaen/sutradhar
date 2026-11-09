# id: S01
# title: Command run through a shell
# why: A shell reads its whole line, so outside text can run other programs.
# fix: Start the program itself, with its arguments as a list and no shell.
#
# Security rule 1. Test code is left out.
package nirikshak.s01

import rego.v1

shells := {"sh", "bash", "/bin/sh", "/bin/bash", "zsh", "dash", "cmd", "powershell"}

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": c.path, "line": c.line, "what": c.text} if {
	some c in input.call_text
	c.callee == "std::process::Command::new"
	c.text in shells
	in_code(c.path, c.function)
}
