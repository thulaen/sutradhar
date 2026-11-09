# id: H25
# title: Cache filled but never cleared
# why: A long-lived cache with no way out serves stale or revoked data.
# fix: Remove or expire entries when their source changes, or bound it.
#
# A cache kept on self or in a static, filled in a file where nothing
# ever takes entries out of it.
package nirikshak.h25

import rego.v1

fills := {".insert", ".entry", ".push", ".push_back", ".put", ".extend", ".get_or_insert_with"}

empties := {".remove", ".clear", ".retain", ".invalidate", ".invalidate_all", ".pop", ".pop_front", ".pop_back", ".drain", ".truncate", ".swap_remove", ".shift_remove", ".expire", ".evict"}

cache(r) if contains(upper(r), "CACHE")

long_lived(r) if {
	cache(r)
	startswith(r, "self.")
}

long_lived(r) if contains(r, "CACHE")

emptied(path) if {
	some m in input.method_receiver
	m.path == path
	m.method in empties
	cache(m.receiver)
}

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": m.path, "line": m.line, "what": m.receiver} if {
	some m in input.method_receiver
	m.method in fills
	long_lived(m.receiver)
	not emptied(m.path)
	in_code(m.path, m.function)
}
