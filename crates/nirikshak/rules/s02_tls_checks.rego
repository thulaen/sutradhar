# id: S02
# title: Certificate checks switched off
# why: Without certificate checks anyone in between can read or change traffic.
# fix: Keep certificate and host-name checks on; trust the certificate instead.
#
# Security rule 2, everywhere, tests included.
package nirikshak.s02

import rego.v1

off := {".danger_accept_invalid_certs", ".danger_accept_invalid_hostnames"}

findings contains {"path": c.path, "line": c.line, "what": c.callee} if {
	some c in input.call
	c.callee in off
}
