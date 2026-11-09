# id: M08
# title: Function no test reaches
# why: Code that no test calls, even through other functions, can break without any test noticing.
# fix: Add a test that calls it, or calls code that calls it.
#
# A function is reached when a test calls it, names it as a value (in
# code or anywhere in a macro's input), or reaches a function that
# does. Names are matched alone, as in R14, so a test that uses another
# function of the same name counts too. A method of an impl of a trait
# from outside the project is called through that trait, and a
# program's `main` when a test runs the program, so both count as
# reached, with all they call. Limits are in spec 0061.
package nirikshak.m08

import rego.v1

last_part(callee) := name if {
	parts := split(trim_prefix(callee, "."), "::")
	name := parts[count(parts) - 1]
}

uses contains [c.function, last_part(c.callee)] if {
	some c in input.call
}

uses contains [r.function, r.name] if {
	some r in input.fn_ref
}

by_caller[u[0]] contains u[1] if {
	some u in uses
}

names := {f.name | some f in input.function}

# Every function stays a key, so the walk keeps the ones that call
# nothing.
edges := {n: object.get(by_caller, n, set()) | some n in names}

own_traits := {t.name | some t in input.shape; t.what == "trait"}

# The walk starts from what tests use, not from the tests' own names,
# so a test named like a function does not count it as reached.
starts contains n if {
	some f in input.function
	f.kind in {"test", "test_panics"}
	some n in object.get(by_caller, f.name, set())
}

starts contains i.function if {
	some i in input.impl_fn
	not i.trait in own_traits
}

starts contains "main"

# The starting points are reached themselves, even with no calls at all.
reached := starts | graph.reachable(edges, starts)

findings contains {"path": f.path, "line": f.first, "what": f.name} if {
	some f in input.function
	f.kind == "code"
	not f.name in reached
}
