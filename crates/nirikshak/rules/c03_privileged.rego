# id: C03
# title: Privileged container
# why: A privileged container can reach every device and change the computer itself.
# fix: Remove privileged: true; give only the one right it needs, if any.
#
# Task #112: mistakes in setup files.
package nirikshak.c03

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": c.name} if {
	some c in input.container
	c.privileged
}
