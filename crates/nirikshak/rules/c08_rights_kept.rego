# id: C08
# title: Container keeps all its rights
# why: A container keeps a set of root rights unless they are dropped.
# fix: Add cap_drop: [ALL] and add back only what it needs.
#
# Task #112: mistakes in setup files.
package nirikshak.c08

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": c.name} if {
	some c in input.container
	not c.cap_drop_all
}
