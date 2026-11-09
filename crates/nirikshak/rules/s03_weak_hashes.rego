# id: S03
# title: Weak hash function
# why: MD5 and SHA-1 can be forged, so they cannot protect or identify anything.
# fix: Use SHA-256, which the project already uses for fingerprints.
#
# Security rule 3, everywhere, tests included.
package nirikshak.s03

import rego.v1

weak(callee) if startswith(callee, "md5::")

weak(callee) if startswith(callee, "sha1::")

weak(callee) if startswith(callee, "sha1_smol::")

findings contains {"path": c.path, "line": c.line, "what": c.callee} if {
	some c in input.call
	weak(c.callee)
}
