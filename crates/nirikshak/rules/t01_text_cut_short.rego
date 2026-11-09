# id: T01
# title: Text cut short
# why: The owner's rule is no truncation anywhere: every post and page is read in full, so a cut hides text from the reading and the matching.
# fix: Read the whole text and split it into pieces; or refuse the whole message with an error.
#
# Task #77. In code (not tests), a cut on anything named like content
# (text, body, content, post, page, message, title, chunk, token, word,
# passage, snippet, excerpt, html or bbcode): truncate, split_off,
# take or skip on its characters, bytes, words or lines, or a slice or
# get from the start. Refusing a whole message is allowed; it cuts
# nothing.
package nirikshak.t01

import rego.v1

content_words := ["text", "body", "content", "post", "page", "message", "title", "chunk", "token", "word", "passage", "snippet", "excerpt", "html", "bbcode"]

# The name a receiver is about: its first part, as in `post.body` or
# `text.chars()`, and every field after it.
names_content(receiver) if {
	parts := split(replace(replace(lower(receiver), "()", ""), "::", "."), ".")
	some part in parts
	some w in content_words
	contains(part, w)
}

cuts := {".truncate", ".split_off", ".take", ".drain"}

in_code(path, name) if {
	some f in input.function
	f.path == path
	f.name == name
	f.kind == "code"
}

findings contains {"path": m.path, "line": m.line, "what": concat("", [m.receiver, m.method])} if {
	some m in input.method_receiver
	m.method in cuts
	names_content(m.receiver)
	in_code(m.path, m.function)
}

findings contains {"path": r.path, "line": r.line, "what": concat("", [r.receiver, "[..]"])} if {
	some r in input.range_cut
	names_content(r.receiver)
	in_code(r.path, r.function)
}
