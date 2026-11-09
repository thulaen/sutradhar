# id: C04
# title: Container on the host network
# why: On the host network a container can reach every port on the computer and is not walled off.
# fix: Use a named network and publish only the ports it needs.
#
# Task #112: mistakes in setup files.
package nirikshak.c04

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": c.name} if {
	some c in input.container
	c.network_mode == "host"
}
