# id: C07
# title: Container may gain new rights
# why: Without no-new-privileges a program inside can raise its own rights.
# fix: Add no-new-privileges:true under security_opt.
#
# Task #112: mistakes in setup files.
package nirikshak.c07

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": c.name} if {
	some c in input.container
	not c.no_new_privileges
}
