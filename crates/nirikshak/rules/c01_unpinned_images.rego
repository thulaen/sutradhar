# id: C01
# title: Container image not pinned
# why: A tag such as :latest can change under us, so a later build may run code nobody checked.
# fix: Pin the image by its digest, as name:tag@sha256:...
#
# Task #112: mistakes in setup files.
package nirikshak.c01

import rego.v1

findings contains {"path": c.path, "line": c.line, "what": c.image} if {
	some c in input.container
	c.image != ""
	not contains(c.image, "@sha256:")
}

findings contains {"path": d.path, "line": d.line, "what": d.image} if {
	some d in input.docker_stage
	d.image != "scratch"
	not contains(d.image, "@sha256:")
	not earlier_stage(d)
}

# FROM can name an earlier stage of the same file (its `AS` name), which
# is not an image.
earlier_stage(d) if {
	some e in input.docker_stage
	e.path == d.path
	e.line < d.line
	e.alias != ""
	e.alias == lower(d.image)
}
