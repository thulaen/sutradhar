# id: H44
# title: Local clock time
# why: Local time moves with time zone and daylight saving; stored times lie.
# fix: Use UTC, such as SystemTime::now; convert only for display.
package nirikshak.h44

import rego.v1

local := {"chrono::Local::now", "chrono::offset::Local::now", "Local::now", "time::OffsetDateTime::now_local", "time::UtcOffset::current_local_offset", "time::UtcOffset::local_offset_at"}

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": c.path, "line": c.line, "what": c.callee} if {
	some c in input.call
	c.callee in local
	in_code(c.path, c.function)
}
