# 0026: shodhan, the text engine

shodhan ("refining") is the Rust library that turns a post's stored
BBCode into the words a reader sees, finds phrases that could become
links, and places an accepted link by wrapping existing words without
changing any other character. It also owns the typed language-feature
contract (tokens, word types, lemmas, noun phrases, names, parsing,
coreference where supported and tone) and the content reports built on
it. Approved spaCy packages provide the broad NLP implementations through
the admitted Rust-owned bridge; no first-party Python production component
is permitted. shodhan serves reading (0022), matching and anchors (0023)
and staff suggestions (0028). It is roadmap Phase 4 item 3.

## What it does

### Where it runs

1. The crate `sutradhar-shodhan` (Rust name `shodhan`) owns deterministic
   BBCode parsing, readable text, placement, insertion, undo and typed
   feature orchestration. Its pure parsing core has no network, database,
   clock, globals or files except caller-supplied data.
2. Parsing, readable text, placement, insertion and undo are light and may
   run in any admitted service job. spaCy pipelines and BGE tokenisation
   run only in resource-admitted model jobs. The desktop preview uses the
   pure parsing core and never loads model weights.
3. Long Rust calls run on a rayon pool or through `spawn_blocking`, never
   on Tokio's worker threads. Packaged Python calls run through the
   admitted Rust bridge or packaged local service with bounded inputs,
   outputs, time and memory.
4. shodhan holds no model weights directly. Model, tokenizer and spaCy
   artefacts are pinned in `models.lock` under spec 0024 and selected by
   immutable compatibility keys. BGE and spaCy packages execute their own
   algorithms; no pure-Rust model reimplementation or first-party Python
   production adapter is allowed.
5. shodhan adds no capability of its own; staff suggestions (0028) and
   the post X-ray screen register the ones it serves.
6. Out of scope: reading order, piece storage, vectors and the index
   (0022); scoring, reranking and choosing suggestions (0023); guest
   visibility (sampark applies XenForo's rules, and shodhan is given
   only allowed text); the review flow and the XenForo edit itself
   (0028 and the PHP add-on); rendering BBCode to HTML (XenForo does
   that); keyword search with tantivy.
7. Every Sutradhar edit needs two passes for the same operation: the
   main Rust validator and an independently written second Rust
   validator (prahari's veto). XenForo's own permission, visibility,
   pause, version and undo checks still apply.

### spaCy language-feature contract

`Parse(request_id, tenant_id, pipeline_key, language, source_digest, text)`
returns a versioned `SpacyFeatureSet`. The caller supplies only complete,
authorised readable text and its immutable source digest. The result
contains the pipeline and knowledge-base digests, enabled packaged
components, schema version and zero or more of:

- tokens with exact UTF-8 source offsets, whitespace and sentence ids;
- sentences with exact source offsets;
- lemmas, parts of speech, morphological features and dependencies;
- noun chunks where the declared pipeline supports them;
- named entities, entity links and their confidence and knowledge-base
  provenance;
- text classifications and rule-based matches with their taxonomy version;
- coreference, sentiment and relation outputs only where an admitted
  packaged component and feature contract explicitly support them; and
- component warnings and unsupported-capability outcomes.

Offsets always refer to the immutable source bytes. Normalised or lemmatised
text never replaces the original offsets used for anchor insertion. A
pipeline or component-order change creates a new feature-schema identity
and invalidates dependent features until rebuilt.

The language matrix in `models.lock` declares admitted capabilities.
A diagnostic language entry adds no product support. Under DR-081
only British-English content can pass spec 0018 in version 1.
For each language used by the pipeline the matrix names the
tokeniser, trained pipeline, entity linker, knowledge base, classifier,
packaged component, quality fixture and resource profile. Tokeniser support
is not trained-model support. A missing trained capability returns
`ERR-0026-UNSUPPORTED-LANGUAGE-CAPABILITY`; it never silently substitutes
an English model or labels tokeniser-only output as NER, parsing or
classification.

Only spaCy's built-in or approved third-party packaged components are
allowed. Custom-trained pipelines use declarative spaCy training
configuration, immutable training data and the packaged training command.
No first-party Python factory, callback, component or training loop is
permitted. A required capability without an approved packaged route is
blocked and explicitly unsupported.

BGE tokenisation is a separate contract owned by specs 0022 and 0024. Its
tokenizer digest, maximum input tokens, overflow strategy and chunking
receipt travel with every embedding request. spaCy tokens are not assumed
to equal BGE tokens.

### The forum profile

Every call takes a versioned forum profile, read by sampark from the
forum's database: the standard tags of the XenForo version and every
registered custom tag (whether it takes an option and its pattern,
whether its content is parsed, whether smilies and automatic links stop
inside it); the smilie codes; the censored words, their replacements
and the censor character; the board address, friendly URLs and titles
in URLs on or off, route prefixes, old domains, mirrors and CDN hosts;
and the largest message XenForo accepts. A faulty profile, such as a
tag registered twice, is refused with SUT-404, never guessed at.

### Grammar and tags

XenForo 2.3's own parser is the authority; where this spec and XenForo
differ, XenForo wins and this spec is corrected. Facts below were
checked on XenForo 2.3.13 on Dell's test forum.

```text
message   = { text | open-tag | close-tag } ;
open-tag  = "[" name "]" | "[" name "=" option "]"
          | "[" name attribute { attribute } "]" ;
close-tag = "[/" name "]" ;
name      = word-char { word-char } ;  (* ASCII letter, digit, "_" *)
option    = "'" { any } "'" | '"' { any } '"' | { any but "]" } ;
attribute = space { space } key "=" value ;
value     = "'" { any } "'" | '"' { any } '"'
          | { any but quote mark, space or "]" } ;
```

- Tag names compare without case, ASCII letters only; XenForo keeps
  the case as typed.
- A quoted option ends at the first matching quote mark followed by
  `]`, so it may hold line breaks and `]`. A bare option ends at the
  first `]`; a line break does not end it. Each attribute must be
  followed by a space or `]`; `[B ]` is text.
- A `[` that does not start a well-formed token for a registered tag
  is text: so is `[*]` outside a list, an unclosed literal `[url]`,
  Markdown-like `**bold**` and backticks. HTML entities are text.
  Stored text is read as stored, whatever XenForo's Markdown option.

| Tag | Kind | What shodhan does |
| --- | --- | --- |
| B, I, U, S | inline style | read; a link may sit inside |
| COLOR, FONT, SIZE | inline style | option ignored; as above |
| LEFT, CENTER, RIGHT, JUSTIFY, INDENT | block style | paragraph; link allowed |
| HEADING | block | one paragraph; a link may sit inside |
| HR | block | a rule, `[HR][/HR]`; ends a paragraph |
| LIST, `[*]` | block, item | each item is a paragraph |
| TABLE, TR, TH, TD | block | each cell is a paragraph |
| QUOTE | quote | skipped, attribution included |
| CODE, ICODE, PHP, HTML | not parsed | skipped |
| PLAIN | not parsed | read; never-anchor |
| SPOILER, ISPOILER | spoiler | skipped, title included |
| URL, EMAIL | link | existing link; text read, never-anchor |
| USER | mention | read; never-anchor |
| IMG, ATTACH | data | address or id not read; `alt` is an extra |
| MEDIA, EMBED | data | site, id and attributes not read |
| custom tags | custom | read; never-anchor |

- These are all of XenForo 2.3.13's standard tags (no subscript or
  superscript). IMG, PHP, HTML, PLAIN, MEDIA, ATTACH and USER are not
  parsed; nor are CODE and ICODE unless the option is `rich`, nor URL
  and EMAIL without an option. `[CODE=rich]` parses but is still code,
  so it is skipped.
- `[*]` is not a tag to the parser: LIST splits its text at each `[*]`.
  Text between `[HR]` and `[/HR]` is a new paragraph after the rule.
- COLOR, FONT and SIZE need an option matching XenForo's pattern;
  INDENT and HEADING take only digits; otherwise the tag is text. TR
  is a tag only directly in TABLE, TH and TD only directly in TR. A
  link tag inside a link is text.
- Options are never readable, except `alt` on IMG and ATTACH (IMG
  falls back to `title`), which is an extra. A quote's attribution,
  such as `"Name, post: 123, member: 45"`, is never content.

### Parsing rules

1. One pass, start to end, jumping to the next `[` or line break with a
   fast byte search. Tags sit on an explicit stack: no recursion and no
   backtracking patterns, so time and memory grow with the message.
   winnow reads tag tokens; a hand-written pass builds the tree.
2. Inside a not-parsed tag, every opening tag and every other closing
   tag is text; its first own closing tag ends it.
   `[CODE][CODE]x[/CODE][/CODE]` is code holding `[CODE]x`, then text.
3. A closing tag that matches a tag further out closes every inner tag
   and reopens it at once with the same text: in `[B]a [I]b[/B] c[/I]`,
   "b" is bold italic and " c" italic. Placement across such tangled
   tags is always refused.
4. A closing tag with no open match is text.
5. A tag still open at the end of the message, or of its enclosing tag,
   runs to that end: `a [QUOTE]b` shows "b" as a quote.
6. An unregistered name is text, and so is a registered tag whose
   option breaks its rule. Bracketed tokens of unregistered tag-like
   text are never-anchor. A registered custom tag's content is read
   and never-anchor.
7. At most 20 tags are open at once; a deeper opening tag and its
   closing tag are text. There is no limit on the number of tags.
8. Smilie codes match exactly, case included, anywhere, even inside a
   word, longest first, outside tags that stop smilies. A smilie is its
   own segment, never a word or anchor; stored text keeps the code.
9. Bare web and e-mail addresses are found by the rule of XenForo's
   automatic linker, outside link tags and tags that stop it. They
   exist in stored text only after import or a save with formatting
   off, and XenForo then shows them as plain text. shodhan records them
   as bare addresses.
10. Line breaks may be LF, CRLF or lone CR, kept as stored. XenForo's
    forms turn CRLF into LF and delete lone CR, zero-width spaces,
    byte-order marks, soft hyphens and other controls; imports may
    still hold them.

Parsing never fails because of markup. It fails only for the limits
(size, invalid UTF-8, budget), and then returns an error, never a
partial result.

### What XenForo's save rewrites

Even with every filter off, XenForo's save adds missing closing tags and
writes tangled tags closed and reopened: `[B][I]text[/B][/I]` is stored
as `[B][I]text[/I][/B][I][/I]`. Case, options, smilie codes, accents and
line breaks are kept. `resave_form` computes that text. If it differs
from the stored message, a link edit would change more than two tags,
so no link is placed: the post is shown as skipped with SUT-424.

### Segments, offsets and the position map

- `parse` returns a `Document` whose segments tile the stored message:
  each byte in one segment, in order; joined, they give the message
  byte for byte. Each segment has its stored span; its kind (text, tag
  token, line break, smilie, bare address); its context (quote, not
  parsed, spoiler, link, mention, custom tag, data, heading, list item,
  table cell, style); and its tag, in an arena of tag records with
  name, option, attributes, open and close spans, parent and a
  reopened flag.
- Inside shodhan positions are byte offsets. At every boundary (stores,
  sampark, 0022, 0023, the add-on) they are code points, so PHP's `mb_`
  functions agree. The preview also gets UTF-16 positions for the
  page's scripts. Every span is half-open.
- An offset index keeps a checkpoint every 1,024 code points, so byte
  and code-point conversion takes logarithmic time. Offsets fit in
  32 bits.
- The position map records runs of readable text: identity; normalised
  (a short run whose NFC form differs, one unit); mask (one unit); and
  separator (added characters, mapped between stored characters).
- `to_stored(span)` gives the stored span holding exactly those
  characters, and refuses a span that cuts a unit or crosses a
  separator or skipped zone. `to_readable(pos)` gives the readable
  position at or after a stored one.
- Invariants: positions rise together; every readable character but
  separators maps to stored characters; a unit-aligned span maps to
  stored and back unchanged; every mapped boundary is a grapheme
  cluster boundary of both texts.
- A map is keyed by the SHA-256 of the stored bytes and never reused:
  any re-save that changes one byte means a fresh parse.

### Readable text, masks and zones

- Read: ordinary text, styles, headings, list items, table cells,
  `[PLAIN]`, visible text of links and bare addresses, mentions and
  custom tag content (the last four never-anchor). Not read: tag
  tokens, quotes at any depth with attributions, code, spoilers with
  titles, and data tag content.
- A block tag or skipped zone ends a paragraph (a blank line). A smilie
  becomes one space ("great:cool:idea" is two words). A line break
  stays a line break. A post with no readable text (all quote, only
  images) gets "no readable text", with its extras still gathered.
- Masks: censored words match by XenForo's rules: case-insensitive,
  Unicode letters, a non-word character or text edge on each side,
  and a `*` at either end lifts that side's condition (`*cat` matches
  "bobcat"). The mask is the word's own replacement, or the censor
  character once per code point of the listed word ("café" gives
  four). shodhan also matches the NFC form, so a decomposed spelling is
  masked though XenForo misses it, and masks extras such as `alt` too.
  One implementation serves reading, placement and preview. A caller
  may pass censor spans; one off character boundaries is SUT-405.
- Masked words never enter readable text, pieces, candidates,
  snapshots, logs or errors.
- `zones` lists in stored code points every quote, attribution, code
  block, spoiler, link, bare address, mention, custom tag, smilie,
  mask, data tag and extra, for the post X-ray and 0022.
- In WordPress HTML, `zones` also lists each `blockquote` element, as
  a zone of its own kind, `wp_quote`, for kavach (spec 0072). The
  parser counts open blockquote elements, without limit, and lists
  only the outermost 20 levels as zones: a deeper blockquote and its
  matching closing tag lie inside the innermost listed zone. A closing
  tag when none is open is ignored, and an unclosed blockquote's zone
  runs to the end of the post. It is not skipped: it stays readable
  text for pieces, and its links keep their normal flag and count, so
  indexing (0022) and the link allowance (0028) are unchanged.
- Never-anchor: link text, bare addresses and any text that is an
  address or domain; mentions, custom tags, `[PLAIN]`; masks and
  smilies; extras. Skipped zones are not in readable text at all.

### Normalisation and segmentation

- The stored message is never changed; normalisation makes views.
  XenForo stores text as typed. Readable text is NFC
  (`unicode-normalization`); anchors are placed at stored positions,
  so decomposed bytes stay as they were.
- The match key is NFC, then locale-free lower case, then without soft
  hyphens, zero-width spaces, word joiners and byte-order marks. The
  zero-width joiner and non-joiner stay. Accents stay ("resume" never
  matches "résumé"); "ß" is not "ss"; "I" lowers to "i" in any
  language, so Turkish "ı" stays distinct. The key has its own map
  back to readable text.
- Grapheme clusters (`unicode-segmentation`, extended): no cut anywhere
  falls inside a cluster.
- Words: Unicode's default rules, and `icu_segmenter`'s dictionary and
  model breaker for runs of Chinese, Japanese, Thai, Lao, Khmer and
  Burmese. `words` joins both and every word boundary is a cluster
  boundary.
- Sentences: Unicode's default rules plus a reviewed abbreviation list
  ("e.g.", "Dr.") after which no sentence ends; never across a
  paragraph. Paragraphs are as 0022 defines them.
- Right-to-left text is segmented like any other. A span with a
  bidirectional control, or a word mixing scripts (by `icu_properties`'
  script property), is never an anchor.

### Language

- `lingua` names each paragraph's language, all languages in
  high-accuracy mode if the measured memory fits, otherwise low
  accuracy with the accuracy change reported. Letters are counted by
  Unicode's alphabetic property.
- A paragraph under `reading.language.min_letters` (20) does not vote.
  It may inherit the post's current eligible language evidence if
  it belongs to the same site and has no contrary evidence. An
  uncertain paragraph has no vote or anchor.
- The main language holds the most letters among voting paragraphs; a
  tie goes to the earlier paragraph. With no voter, use current
  eligible thread evidence (from its title and first post) only
  within the same site authority and without contrary item evidence.
  Without a valid claim the post is skipped with its reason.
- The result records the main language, each paragraph's language and
  letters, and the count of paragraphs in other languages.
- No link goes in a paragraph whose language differs from the main
  one, though the paragraph may still be read for pieces. Apply
  spec 0018 before indexing or linking. Detection of other languages
  is a diagnostic, not support for those languages.
- An identifier that cannot load or fails gives SUT-407 and the post
  waits.

### Streaming read

Reading (0022) never builds a whole `Document`. It uses `ReadStream`,
which gives the same readable text, zones, links and pieces as
`parse` with `readable`, with memory that does not grow with the post.

- The message: held as one copy, the bytes as received from the 1 MiB
  slices, in one buffer sized once from the stated length. sampark
  hands this buffer over; it is not copied. It is kept only so offsets
  point into it. Its checksum (each part's, for a WordPress post read
  in parts) is checked before it is parsed and before anything from
  it is stored. It is freed as soon as reading of the message ends.
- The parser streams over that buffer. BBCode uses our own parser with
  an explicit stack of at most 20 open tags. WordPress HTML uses
  lol_html, a streaming rewriter, with its memory limit
  (`max_allowed_memory_usage`) set to 4 MiB. Our BBCode parser keeps
  the same 4 MiB limit for its own state. Going over it is SUT-433,
  with no partial result, reported as a defect; no fixture meets it,
  a tag bomb included.
- Readable text flows straight into the piece builder. The builder
  holds only the piece being filled, the text waiting for the next
  boundary and the overlap: at most three pieces of text.
- Piece cap in bytes: a piece holds at most 64 KiB of UTF-8 text, a
  hard cap beside the token size. Whichever is reached first ends the
  piece, split by 0022's rules: paragraph, then sentence, then word.
  A single word over 64 KiB is split at grapheme boundaries, counted
  and shown. A piece over the cap after splitting is SUT-409 and a
  defect, never cut.
- Each finished piece goes to the embedder queue one at a time. The
  queue holds at most 4 pieces; the builder waits when it is full.
- Links, zones, spans and readable text are written to the store in
  batches of at most 2 MiB. The offset index keeps one checkpoint per
  1,024 code points (128 KiB for 16 MiB).
- A WordPress post over 16 MiB is fed part by part into the same
  stream; the stream's state is the carry-over between parts.

### Pieces, extras and the rules version

- shodhan implements 0022's piece rules: title, blank line, then whole
  paragraphs, within the piece size (`reading.pieces.tokens`, 1,024 by
  default, never fewer); an overlap of whole sentences within
  `reading.pieces.overlap_tokens` (64 by default); long paragraphs
  split at sentences, long sentences at words; every readable
  character in a piece; nothing cut silently.
- `TokenCounter::count` counts tokens, markers included. In service it
  loads the active embedder's tokenizer with `tokenizers` from the
  pinned files; tests use a fake counter with a small window. Each
  piece is recounted; one over the piece size or over a model's window
  is SUT-409, never cut. Missing tokenizer files are SUT-408.
- Extras: `alt` text from the message; attachment names, link-preview
  and media titles from sampark. Each is read in full, split like body
  text when longer than one input, and is never-anchor.
- `RULES_VERSION` changes whenever a rule changes any output. 0022
  includes it in its reading-rules version, so a change re-reads the
  forum in the background.

### Existing links and addresses

- `links` lists each existing link: its form (`[URL=address]text[/URL]`
  with any quoting, `[URL]address[/URL]`, `[URL unfurl="true"]`,
  `[EMAIL]` with or without option, or bare address), stored span,
  address, visible text, and an inside-a-quote flag. Quoted links count
  as existing connections but not against the link allowance.
- `classify_url` reads one address with the board settings, offline:
  internal or external, accepting `www` or not, `http` or `https`, a
  trailing slash, a subdirectory, `index.php?threads/...`, and listed
  old domains, mirrors and CDN hosts. Hosts compare in any case; route
  prefixes do not (`/THREADS/1/` is "unreadable", never a thread).
- Internal kinds: thread, post, resource, resource update, attachment,
  member or other, from `/threads/123/`, `/threads/title.123/`,
  `.../page-3`, `/posts/456/`, `/goto/post?id=456`,
  `/threads/123/post-456` and `#post-456`. Percent-encoded slugs,
  tracking parameters and trailing punctuation are handled; relative
  addresses resolve against the board. The number decides; the slug
  is ignored. Following moved-thread redirects stays with sampark.

### Phrase sources

- Keyphrases: `yake-rust` scores one- to four-word phrases over the
  whole readable body (frequency, first position, spread, capitals)
  with a reviewed English stop list. Occurrences are found in the
  match key with Aho–Corasick, whole words only. The best 20 per post
  are kept, ties by first position.
- Stemmed n-grams: runs of one to four words not starting or ending
  with a stop word and not crossing punctuation, sentence or paragraph
  ends. Words are stemmed with `rust-stemmers` for the paragraph's
  language only; no stemming across languages. "Power supplies" and
  "power supply" share "power suppli".
- Name-like phrases: one to four capitalised words taken whole; a
  sub-part is never offered alone ("Apple" in "Apple Pie Recipe"). A
  sentence-initial capital counts only if capitalised elsewhere in the
  post or a title phrase. Flagged `name_like`.
- Noun phrases from the admitted packaged spaCy noun-chunk output where
  supported, without leading words such as "the" or "my"; unsupported
  noun chunks produce no improvised phrase source.
- Title phrases: n-grams, keyphrases and name-like phrases of each
  destination's title (thread, resource, hub page), in its own
  language. Generic titles and stop-list phrases ("Help!", "Question")
  give none.

### Names

Name recognition is mandatory and always on for every declared Version 1
language; no setting turns it off. The admitted packaged spaCy pipeline
by the selection rule below is kept. A language without a passing route
remains an explicit readiness blocker.

- `NameFinder::find` is a typed view over `SpacyFeatureSet`, running in
  admitted inference jobs behind `names`. It does not load or execute a
  separate entity model. `FakeNameFinder` gives fixed labels in tests.
- Input is complete, authorised readable text in the declared language.
  Resource-bound splitting follows the spaCy pipeline's admitted request
  contract and preserves exact source offsets; no part is discarded or
  represented as a complete parse.
- Allowed labels: person; organisation; product, brand or object;
  place; event. Refused labels: political party or movement; religion
  or religious group; nationality or ethnic group; film; music; book;
  other title of a work; language; law; date; time; money; percentage;
  quantity; number. The pipeline-specific mapping must account for every
  source label; an unmapped or unknown label is refused.
- A stretch counts only on word boundaries with a calibrated score at
  least `anchors.names.min_score` (0.5); it is never widened or trimmed.
  Where the admitted pipeline exposes no comparable confidence, admission
  must define and validate a separate abstention rule rather than pretend
  a binary entity label has score 1.0. The stretch gives its label to a
  name-like phrase with the same span; an unlabelled name-like phrase
  stays an ordinary phrase.
- A person goes to 0023's people safeguards: offered only when the
  destination's title or first post has the same name, never a
  member's user name, and marked "person".
- Refused kinds are hard rules: a candidate overlapping a refused
  stretch is dropped whatever its source; an unlabelled occurrence with
  the same match key in the post is dropped; a destination title phrase
  with a refused label is left out of the index; 0023 sees only that a
  span was refused, by kind.
- If the pipeline cannot load or fails, SUT-425 and the post's candidates
  wait; names are never offered unlabelled instead. Each result records
  the complete pipeline and component identities from `models.lock`, so a
  new artefact re-reads names.

### Word types

- Each main-language word eligible for an anchor gets a Universal
  Dependencies tag from the same `SpacyFeatureSet`. No anchor starts or
  ends on a pronoun, verb, auxiliary, determiner, adposition or
  conjunction (`Upos::blocks_anchor_edge`), so "click here" never links;
  the stop list and name-like rule may check types.
- `WordTagger::tag` returns the 17 UD tags for the requested source span,
  or SUT-426 when tagging is unsupported, malformed or inconsistent with
  the validated offsets. Noun phrases use packaged noun-chunk output only
  where the pipeline declares it. `FakeWordTagger` serves tests.
- Pipelines or components whose licences forbid the intended use stay
  out. Missing trained support returns
  `ERR-0026-UNSUPPORTED-LANGUAGE-CAPABILITY`; tokeniser-only output never
  becomes an improvised tag or noun phrase.

### The phrase index and matching

- `PhraseIndex::build` takes eligible destinations' titles and a memory
  budget, once per serving generation; `update` replaces one
  destination's phrases. Over budget is SUT-410. It holds two
  Aho–Corasick automatons (`aho-corasick`), over match keys and over
  stemmed keys, each pattern listing its destinations.
- `find` scans a post's match key and stems once each, in time linear
  in post length plus matches. Matches start and end on word
  boundaries ("cat" never in "category"). Each title hit holds the
  readable span, destination, phrase and kind (exact or stemmed). The
  anchor is the post's own words in the post's own case.
- Close spellings: an n-gram is compared only with title phrases of
  the same word count, found through an index of title words by first
  two letters and length. Only words of at least
  `anchors.fuzzy.min_letters` (5, never lowered without the owner) may
  differ; shorter words must equal. `strsim`'s normalised
  Damerau–Levenshtein on the whole phrase must reach
  `anchors.fuzzy.min_similarity` (0.90): "raspbery pi" scores about
  0.92 against "raspberry pi". A person never matches by close
  spelling. A close match is flagged `close_spelling`; nothing is
  corrected.

### Anchor candidates

`anchor_candidates` joins keyphrases, title-matching n-grams, noun
phrases, name-like phrases, names and title hits. One span found by
several sources is one candidate with all source labels. Each passes,
here and again at application time (one implementation):

- not in a skipped zone, link, bare address, mention, custom tag,
  `[PLAIN]`, mask, smilie or extra;
- on word and cluster boundaries; inside one paragraph, never across a
  line break, list item, table cell or heading;
- balanced: every tag opened inside closes inside, none opened before
  closes inside;
- no bidirectional control or mixed-script word;
- not a number, date, amount or percentage alone ("2018", "5.1");
- not a stop-list phrase ("this", "here", "the thread", "click here",
  "Hi all");
- 3 to 80 characters and at most 8 words;
- not overlapping a refused name, nor edged by a blocked word type;
- not in a non-English post or a paragraph outside the main language
  or of unknown language.

Each candidate records stored and readable spans, text equal to the
stored characters there, source labels, name label, `name_like` and
`close_spelling`. All occurrences are listed in order, the first
eligible one marked. The member-name check, scoring and choice among
overlapping candidates belong to 0023.

### The placement check

`check_placement` takes a document and stored span and returns the
place for two tags, or one refusal. Refused when the span:

- is empty or off word or cluster boundaries;
- touches a quote, attribution, code, `[PLAIN]`, spoiler, link, bare
  address, mention, custom tag, data tag, smilie, mask or tag token;
- crosses a line break, paragraph, list item, table cell or heading;
- is unbalanced, such as half inside `[B]`;
- has an enclosing tag off the allowed list: B, I, U, S, COLOR, FONT,
  SIZE, LEFT, CENTER, RIGHT, JUSTIFY, INDENT, HEADING, list item, table
  cell (inside `[B]` the link goes inside the bold);
- already has 20 enclosing tags (SUT-421);
- sits in a reopened tag; or holds a bidirectional control or
  mixed-script word.

A span is never moved, widened or trimmed to fit.

### Building and verifying a link

`insert_link` takes the stored message, profile, budget and a request
(expected SHA-256, stored span in code points, expected anchor text,
target: thread, post, resource or hub page by number):

1. Hash differs: SUT-411, no change; the post is analysed afresh.
2. Characters at the span differ from the anchor: SUT-412.
3. Placement check refuses: SUT-413 with the reason.
4. The post already links to the target, quotes and bare addresses
   counted: SUT-423.
5. `resave_form` differs from the message: SUT-424, checked before
   anything is built.
6. The address is the short form from board settings and number only:
   board address plus `/threads/123/` or `/posts/456/`, or
   `index.php?threads/123/` with friendly URLs off. No post or title
   text enters it. A quote mark, square bracket, space or control in it
   is SUT-414.
7. Tags are `[URL='address']` and `[/URL]`, as XenForo's editor writes.
8. The new message is the old one with the two tags inserted by byte
   offset; nothing else changes.
9. Longer than the forum's largest message, in code points: SUT-422.
10. `verify_insertion` runs; a failure is a defect, SUT-415, and
    nothing is returned.

The insertion record holds old and new hashes, address, exact tag
strings, anchor text, tag positions, target and rules version; 0028
keeps it for undo. `verify_insertion` passes only if deleting the two
tags gives the old message byte for byte; the new message has exactly
one more link, at the anchor, with the anchor as text; every other
segment keeps kind, context and text; readable text is unchanged; and
no link is nested.

### Checks at application time

The sequence 0028 confirms: on approval a job reads the current message
and sampark rechecks guest visibility of source and target; it calls
`insert_link` with the recorded hash and anchor (a refusal ends it,
shown in plain English with a manual link); it sends the add-on post
number, old hash, new message and new hash. In one transaction the
add-on stops if the stored hash differs, saves through XenForo's post
editor as a silent system-user edit (formatting off, no "last edited"
note, automated mode), then compares the stored hash with the new one
and rolls back with SUT-416 on any difference. Such an edit does not
bump the thread or alert anyone and raises the edit count; whether
edit history keeps the old text is 0028's choice. XenForo nests
transactions as savepoints.

### Undo that keeps later edits

`plan_undo` takes the current message, profile, record and budget:

- remove: exactly one link outside quotes has the record's opening tag
  and wraps exactly the anchor text; delete its two tags only, keeping
  every other byte (a later bold wrap stays);
- nothing to undo: no such link and none wraps the anchor; no edit,
  SUT-418 as information;
- conflict: the tag remains with changed text, another address wraps
  the anchor, or two or more match; no change, SUT-417 for review.

`verify_undo` checks the result is the current message less exactly
the two tags, with readable text unchanged. Applying uses the same hash
checks. If XenForo's save would rewrite the current message, SUT-424
sends it to review. A link inside a quote is never removed.

### The member-text option

The option is on from the start: a moderator approves each change (a
spelling fix, a link wrapped round a misspelt word of five or more
letters, a reworded link text). By default shodhan only wraps existing
words; undo and recheck apply as for links. shodhan writes nothing:
spelling candidates come only from the forum's own words (titles,
tags, glossary, abbreviations) or a moderator's typing, never from a
language model, agent, paid service or local model. A person's name is
never offered for fixing. With the option off, a link on a misspelt
word is refused with the reason.

### The review preview

- The preview runs the same crate (default features) as the service,
  with no second implementation. It shows the rules version, the
  X-ray, placement, the insertion preview and the undo preview, with
  spans in code points and UTF-16 units.
- It shows the stored message with the two tags highlighted and the
  readable text with the anchor highlighted, always as text, never
  markup, so post text cannot run code.
- Each preview returns the SHA-256 of what it would save; the approval
  carries it, and a different result at save time saves nothing and
  shows the preview again (SUT-419). A rules-version mismatch with the
  service turns the preview off (SUT-420) until reload.
- No network calls; nothing kept between calls. Raw stored text goes
  only to a viewer who could open the post in XenForo's editor; anyone
  else gets a service-made preview with masks.
- The budget stops a preview after 1 second: "preview unavailable for
  this post; the server will still check it".

### The developer-only conformance command

A PHP command in the add-on's `Cli/Command` folder runs a corpus of
synthetic posts through XenForo's own parser, renderer and preparer and
writes each post's tag tree, shown text, tag per character and resave
text, committed as fixtures shodhan must match. It refuses to run
unless development mode is on and a test-forum marker is set, runs only
on Dell's test forum, and is left out of every release build.

### Errors

`TextError` is one enumeration; every fallible function returns
`Result<_, TextError>`. Each variant has a SUT code (block SUT-400 to
SUT-449), a plain-English message and a manual page. Messages hold
numbers, offsets, counts, codes and setting keys only, never post text,
anchors or names.

| Code | Meaning |
|---|---|
| SUT-401 | one message or part over 16 MiB; a defect |
| SUT-402 | invalid UTF-8; never decoded with substitutes |
| SUT-403 | budget used up; retried later, no partial result |
| SUT-404 | invalid forum profile |
| SUT-405 | censor span off character boundaries |
| SUT-406 | position map inconsistent; a defect, issue filed |
| SUT-407 | language identifier unavailable |
| SUT-408 | tokenizer files missing or fail to load |
| SUT-409 | input over a model window, or piece over its token or byte cap |
| SUT-410 | phrase index over its memory budget |
| SUT-411 | post changed since the suggestion |
| SUT-412 | anchor text no longer at its offsets |
| SUT-413 | placement refused, with reason |
| SUT-414 | link address cannot be formed safely |
| SUT-415 | insertion or undo verification failed; a defect |
| SUT-416 | XenForo stored other text; edit rolled back |
| SUT-417 | undo needs review |
| SUT-418 | nothing to undo |
| SUT-419 | preview and service result differ |
| SUT-420 | preview rules version differs from the service |
| SUT-421 | 20 tags already enclose the span |
| SUT-422 | new message longer than the forum allows |
| SUT-423 | post already links to the target |
| SUT-424 | XenForo's save would rewrite the post; skipped, undo to review |
| SUT-425 | name model unavailable or failed; candidates wait |
| SUT-426 | word-type tagger unavailable or failed; candidates wait |
| SUT-427 | language-layer model file malformed |
| SUT-428 | model file differs from its `models.lock` digest |
| SUT-429 | model tokenising conventions differ from settings |
| SUT-430 | a setting out of range, naming its key |
| SUT-431 | malformed CoNLL-U line, naming its line number |
| SUT-432 | readable text handed to the layer over 16 MiB in one part |
| SUT-433 | streaming parser over its 4 MiB limit; a defect |

### Public API conventions

Inputs are borrowed; outputs are owned plain data, all serde
serialisable. Types: `CodePoint` (32-bit), `Span`, `Utf16Span`,
`MessageHash`, `ForumProfile` (with `TagRegistry`, `CustomTag`,
`CensorRules`, `BoardSettings`, checked by `validate`), `Budget` (work
units plus an optional caller "stop now" check asked every 64 KiB),
`Document`, `Segment`, `TagNode`, `Readable`, `Zone`, `MatchKey`,
`PostLanguage`, `ExistingLink`, `UrlTarget`, phrase and candidate
types, `NameLabel`, `Name`, `Piece`, `Extra`, `Placement`, `Refusal`,
`LinkTarget`, `InsertionRequest`, `Insertion`, `InsertionRecord` and
`UndoPlan`, `ReadStream`. Functions: `parse`, `readable`, `match_key`,
`words`, `sentences`, `paragraphs`, `graphemes`, `zones`, `resave_form`,
`LanguageDetector::new`, `post_language`, `pieces`, `extras`, `links`,
`classify_url`, `keyphrases`, `ngrams`, `name_like`, `PhraseIndex`,
`similarity`, `anchor_candidates`, `names`, `check_placement`,
`insert_link`, `verify_insertion`, `plan_undo`, `verify_undo`.
`unwrap`, `expect`, `panic`, unchecked indexing and unchecked
arithmetic are denied; overflow checks stay on in release.

### Packaged spaCy language execution

The `LanguageFeatureClient` Rust trait is the only production route to
language features. Its admitted implementation calls `Parse` through the
Rust-owned packaged-library bridge. `FixedLanguageFeatures` supplies
fixed results in tests. shodhan does not implement a second tokeniser,
tagger, lemmatiser, entity recogniser, parser or neural network.

- The Rust adapter validates the response schema, pipeline and component
  identities, package versions, artefact digests and language before it
  accepts a `SpacyFeatureSet`. A mismatch is SUT-428 or SUT-429 as
  applicable; malformed features are SUT-427.
- spaCy token and sentence boundaries describe language features. The
  deterministic word, sentence and paragraph boundaries above remain the
  authority for anchor safety. The adapter converts exact UTF-8 offsets
  to code-point spans and the stored-text position map, and rejects an
  offset outside the source, inside a UTF-8 sequence or across an
  excluded zone. It never changes source text to make offsets agree.
- `Annotation` contains source spans, whitespace, sentence ids, UD part-
  of-speech tags, lemmas, morphology, dependencies and noun chunks only
  when the declared pipeline supplies them. Missing capabilities remain
  absent and produce the unsupported-capability outcome; no English or
  rule-based fallback silently fills them.
- `NameFinder::find` and `WordTagger::tag` are typed views over the same
  `SpacyFeatureSet`, not separately executing models. One parse supplies
  all enabled features for a source digest. Repeated calls with the same
  source and immutable compatibility key return the same features.
- Annotations of real posts are private: in memory or the site's own
  store only, never in logs, errors, snapshots or the repository.
- Long calls and resource admission follow "Where it runs". Cancellation
  stops waiting for new output and discards late output whose request,
  source or pipeline identity no longer matches.

### CoNLL-U, evaluation and pipeline artefacts

- CoNLL-U (ten tab-separated columns, `#` comments and a blank line after
  each sentence) is an interchange format for admitted training and
  evaluation data, not a production model format. `conllu::read` keeps
  comments, multiword ranges (`1-2`) and empty words (`1.1`), and refuses
  a word line without ten columns or a numeric ID (SUT-431). Rust owns
  validated conversion to and from the packaged training command's
  declared input format.
- Annotations of real posts never become training data. The forum gold
  evaluation path exchanges only the ids, positions and labels permitted
  by spec 0066; it does not send post text to an external evaluation or
  training service.
- A served pipeline is an immutable directory artefact with a manifest
  covering every file, its SHA-256, the spaCy and component package
  versions, language and capability declarations, feature-schema version,
  source-data and declarative-configuration hashes, licences and resource
  profile. `models.lock` pins the complete artefact digest and its
  transitive package lock. Partial, unknown or mutable artefacts are
  refused.
- Packaged spaCy training commands produce custom-trained pipelines from
  declarative configuration. Rust supplies validated paths, immutable
  configuration and resource limits, and records the command receipt,
  random seed where the package supports one, logs without private text,
  output hashes and evaluation results. No first-party Python training
  loop, factory, callback or component is generated or loaded.
- Evaluation compares the packaged pipeline's frozen output with the
  locked gold labels through the Rust adapter. Package-reported scores
  are supporting evidence, not a substitute for Sutradhar's offset,
  refusal, privacy, reconstruction and subgroup tests.

### Language-layer settings

Each setting is governed (0014), has a staff tooltip, shows the old value
with one-step undo and is validated with SUT-430 on failure. No setting
overrides the load governor or source-integrity rules.

- `text.nlp.pipeline` selects a language-compatible immutable key from
  `models.lock`, not an arbitrary path, package name or URL. Changing it
  creates a new serving generation and rebuilds dependent features.
- `text.sentences.abbreviations` and
  `text.nlp.sentences.line_break_ends` govern only deterministic anchor
  segmentation. They do not alter a packaged pipeline or claim to retrain
  its sentence component.
- `text.nlp.budget.max_tokens` bounds work per mode. A request exceeding
  the admitted bound waits or fails explicitly; it is never truncated and
  treated as complete evidence.
- Light, medium and ultra modes (`NlpSettings::for_mode`) change resource
  admission only. Within one serving generation they use the same
  pipeline and feature semantics. Balanced and aggressive presets cannot
  weaken privacy, refusal or quality floors.

### Capability admission and stages

Every capability needs feature-specific research, rights and provenance
evidence, a lab run and an admission record under specs 0013 and 0055
before implementation. Ordinary use of spaCy and approved packaged
components follows package provenance and licence review. ADR-0020's
strict researcher/implementer separation applies only when Sutradhar
actually reimplements a method; using a package is not represented as a
clean-room reimplementation.

Each admission protocol freezes its datasets, label mapping, metrics,
subgroups, resource budget and link-reconstruction tests before seeing
results. Numerical floors are blockers until research supplies and
independent review approves them; this specification does not invent
them. An inconclusive or failing candidate stays unavailable. A patent
or licence block stops the affected route and is recorded separately
from experimental evidence.

- Stage 1 supplies tokens, sentences, parts of speech, lemmas,
  morphology and noun chunks from an admitted spaCy pipeline. Quality is
  measured on the declared language's locked gold data and the forum
  gold set, including exact offsets, unsupported languages and mixed
  scripts. Tokeniser-only pipelines cannot satisfy trained-feature
  admission.
- Names use the same pipeline's packaged NER and, where separately
  admitted, entity-linking output. The allowed/refused-label mapping is
  versioned and complete: an unknown source label is refused rather than
  guessed into an allowed class. Exact-span F1 by allowed label, refused-
  label leakage, unknown-label behaviour and forum anchor recall are
  measured together. Name recognition is mandatory for Version 1's
  declared languages, so a language without a passing packaged NER route
  is a readiness blocker, not a silently degraded release.
- Stage 2b supplies dependency heads and relations from the same admitted
  pipeline. LAS, UAS, relation-specific errors, offset integrity and the
  effect on reports are measured on locked language and forum fixtures.
  There is no bespoke arc-eager, perceptron or biaffine implementation.
- Stage 3a coreference remains optional and report-only. It ships only
  where an approved packaged spaCy component, its model and its licence
  support the declared language and pass the frozen coreference and
  privacy protocol. Otherwise it returns unsupported and report ordering
  does not use it. Pronouns resolved to a person are counted, never
  listed.
- Stage 3b tone remains optional, aggregate-only and report-only. It
  requires an approved packaged classifier or rule-based spaCy component
  with a declarative, admitted configuration and a versioned taxonomy.
  A route needing a first-party Python component is blocked. Evaluation
  includes staff agreement and subgroup errors; tone never produces
  member-text candidates or a per-member score.
- Order: stage 1; names; stage 2b; reports that need only those features;
  optional stage 3a; optional stage 3b. No stage hides an earlier open
  blocker, and no optional capability becomes implied evidence when it is
  unsupported.

### Content reports (stage 4)

Rules for every report:

- Report only: evidence for staff that never changes a post, link,
  score or ranking or feeds a suggestion. Earlier rulings that make
  readability, numeric facts or quote counts optional ranking inputs
  stand for their own signals; reports add no second path.
- On by default, each turnable off per site by a governed setting.
- Privacy: guest-visible content only; no member name, user name or
  `[USER=id]` target; people found as names are counted, never named;
  every cell under `analytics.min_count` shows "fewer than N".
  DR-137 and [spec 0144](0144-site-corpus-and-text-mining.md) allow
  a separate exact-span private owner view. It cannot be a report
  source or share a name with any 0026 stage, graph or ranking.
- Each site's summaries stay in its own store; no report joins sites;
  no figure leaves Dell without a privacy review.
- Honest wording: fixed tooltip caveats ("a reading grade does not
  measure quality or formality"; "demand seen in your forum, never a
  prediction"); no rewrite advice; no single composite index.
- Annotation runs as a background job in bounded chunks with durable
  checkpoints; only per-post summaries (counts, entity keys, sentence
  figures) are stored, never per-token records. The X-ray recomputes
  tokens on demand.
- Every figure shown has an entry in `catalogue/metrics.toml` with an
  approved source, a decision of four words or more and a comparison.

Reports:

1. Entity coverage per thread and section: entities (names, noun
   phrases merged by lemma), how many posts mention each and where
   (first post, replies). String match is not the same referent, and
   the tooltip says so.
2. Gaps: (a) entities many threads mention with no destination of
   their own, "content ideas"; (b) entities with a destination whose
   mentions are rarely linked, "link targets", each a set of posts to
   review. Synonyms, plurals by lemma and redirects merge first.
   Labels: improve, link, consolidate, create, watch.
3. Sentence structure: sentence count, words per sentence (median and
   spread), syllables per word, share of long and complex words, and
   with stage 2b passive clauses, subordination and relative clauses.
   Components, not one grade; the readability formulas are computed
   from these counts by ranking (0027); Dale-Chall waits for its word
   list's licence.
4. Claim, quote and number density per 1,000 words: NUM tokens and
   digit-bearing words (a number marker must directly precede the
   digit-bearing word); quote blocks and inline quotation marks;
   claim-like sentences (declarative, holding a number or name, with
   stage 2b a reporting or copular main verb), with hedging and
   evidential markers. "Claim-like" is a pattern count, never a
   judgement of truth.
5. Connected threads: other threads sharing a thread's entities,
   weighted by rarity, and whether a guest-visible link joins them;
   "connected but not linked" pairs go to moderators as review items,
   never placed. One edge per thread pair, guest-visible links only.

Report settings use the ranges shown; none is opt-in and no preset or mode
lowers privacy floors. Every balanced or aggressive value not explicitly
listed here is a readiness blocker owned by the NLP-report evaluation record.
The affected report and preset cannot build or activate until that record
supplies the value, fixtures, privacy evidence and approval:
`analytics.nlp.enabled` (on); `analytics.nlp.reports` (1 to 5, all by
default); `analytics.nlp.sections` (at least one; private sections
never); `analytics.nlp.person_names` (counts only); `analytics.min_count`
(never below its privacy minimum); `analytics.nlp.gap_min_threads`
(2 to 100); `analytics.nlp.connect_min_shared` (1 to 50);
`analytics.nlp.list_length` (10 to 1,000);
`analytics.nlp.refresh_hours` (6 to 168, per mode);
`analytics.nlp.batch_posts` (50 to 20,000, per mode). Each report
needs its own manual page.

### Provenance, clean-room scope, rights and data

- Packaged spaCy components and models are dependencies, not Sutradhar
  clean-room implementations. Their admission records name the exact
  package and model releases, distribution and artefact hashes, direct
  and transitive dependency locks, licences, notices, upstream sources
  and the reviewed compatibility evidence. Calling a package does not
  remove these obligations.
- The deterministic Rust BBCode, offset and placement implementations
  cite the open specifications and research cards they actually use. A
  strict clean-room record is required only for a component explicitly
  designated for clean-room reimplementation by spec 0076. Such a record
  identifies the permitted sources, separated researcher and implementer,
  independent reviewer, reviewed versions and similarity evidence. No
  document claims that separation for ordinary package use.
- UD guidelines, CoNLL-U, UAX #29 and cited research papers are candidate
  specification or evaluation sources, not evidence that every method in
  a package has been cleared. Each feature-specific admission record
  identifies the sources and versions actually reviewed; an unreviewed
  citation does not satisfy implementation readiness.
- Patent research and legal review are separate evidence gates. The old
  patent numbers and CPC ranges in the source specification are
  unverified search leads, not a completed search or legal opinion.
  Before implementation, the affected feature needs a dated, reproducible
  search with jurisdiction, claims and status analysis; before release,
  flagged risks need the qualified professional review required by spec
  0076. Clean-room authorship alone is not patent clearance.
- Candidate datasets include UD English-EWT, PUD, Atis, CTeTex and
  Pronouns; eligible GUM genres; WikiNER; LitBank; Few-NERD; Open English
  WordNet or Princeton WordNet 3.0; UniMorph English; and VADER's
  lexicon. Inclusion in this list is not admission. Each exact dataset,
  genre and version remains blocked until its upstream licence, notices,
  hash, intended-use compatibility and derivative-artefact obligations
  are recorded and reviewed.
- Excluded unless a later owner decision and rights review change the
  entry: UD GUM as released and its other genres, ParTUT, LinES, GENTLE;
  Penn Treebank, OntoNotes 5.0, CoNLL-2012, ARRAU, CoNLL-2003;
  MultiNERD, the NRC emotion lexicon, MPQA, the Stanford Sentiment
  Treebank, WikiANN, PreCo, the Bing Liu lexicon and SentiWordNet.
- Share-alike sets may teach models only after their exact terms and
  resulting artefact obligations are cleared. Each admitted pipeline
  records its data lineage and required credits and is published apart
  from code when its data terms require that treatment (owner,
  6 November 2026, Q-SH12). `models.lock` and the public notices contain
  the exact dataset versions, hashes, licences and notices.
- Whether share-alike terms reach trained weights is not decided by this
  specification or by reading a paper. Where the answer matters, written
  legal review is a release blocker. Dataset licences are checked again
  at their upstream source immediately before download and admission.
- No real post is training data. Staff may mark guest-visible posts on
  Dell only after privacy review, solely for held-out evaluation reported
  as aggregates. The proposed roughly 3,000-word evaluation sample is not
  a licence to train. Moderator acceptance data for the member-text
  option is separate and aggregate.

## Limits

- One message: at most 16 MiB (16,777,215 bytes), the size of
  XenForo's `xf_post.message` column and so XenForo's largest post.
  16 MiB is the size of one message to every call.
- Longer text is read in full, never cut or refused. A WordPress post
  over 16 MiB comes in parts of at most 16 MiB (0022), fed into one
  `ReadStream`. Its state (open tags, positions, the unfinished
  paragraph and piece) carries from part to part, so the result for a
  post read in parts equals the result for the same post read whole.
- A single message or part over 16 MiB is SUT-401 (SUT-432 for text
  handed to the language layer). It cannot come from a working fetch,
  so it is reported as a defect. No other size limit: 0022 reads every
  post in full.
- Nesting: 20 open tags, as XenForo; an explicit stack, so a tag bomb
  never grows the call stack.
- Work: time and memory grow linearly with input (plus output for
  matching). Doubling a hostile input at most multiplies time by 2.5.
- Memory for reading one message, the worked peak for 16 MiB at the
  hard cap of 64 KiB of text per piece, which bounds a piece whatever
  the piece size (no fixed top in tokens; the owner, 10 November 2026:
  "No fixed top"):

  | Part | Most |
  |---|---|
  | The message, one copy | 16 MiB |
  | Parser state, the lol_html or BBCode limit | 4 MiB |
  | Piece builder, three pieces of text | 192 KiB |
  | Embedder queue, 4 pieces | 256 KiB |
  | Offset index | 128 KiB |
  | Tokenizer working memory | 1 MiB |
  | Store batches | 2 MiB |
  | The `ReadRange` reply, while it is decoded | 1.5 MiB |
  | Total | about 25.1 MiB (26.3 MB) |

  Only one message over 1 MiB is read at a time in the service; others
  wait. Beside it, one ordinary batch (1 MiB of text, its parser state
  of 4 MiB and its piece buffers of 0.5 MiB, about 5.8 MB) and the
  running sums of the threads being read (8 KiB each, at most 64 by
  0022's limit, about 0.5 MB) bring reading's peak to about 33 MB.
  That is under its row in spec 0024's memory table (48 MB under
  16 GB, 128 MB at 16 GB or more), with no change to it.
  Each figure is checked with the counting allocator, in a benchmark
  program (AGENTS.md section 6 rule 8).
- Memory for `parse` (the whole `Document`) is used only to place or
  undo a link in one post, never for reading. It is measured with the
  counting allocator in a benchmark program and counts in the staff
  suggestions budget (0028).
- Budgets: 5 seconds per call in the service, 1 second in the preview.
- Speed targets on the 4-core reference machine, each a hard check:
  - preview of a 150,000-character post within 1 second, the preview
    build at most 3 times the service build's time;
  - `check_placement` at most 50 microseconds; `insert_link` with
    checks at most 300 milliseconds on a 5 MB post;
  - `parse` with `readable`: median 20 microseconds for a 1 KB post,
    5 ms for 150,000 characters, 150 ms for 5 MB, 500 ms for 16 MiB;
  - a tag bomb of one million tags, flat or 100,000 deep: 200 ms;
  - `PhraseIndex::find` at most 1 ms per typical post with 1,000,000
    destinations; `build` at most 60 seconds;
  - names and word types together at most 250 ms per typical post;
    language identification and YAKE measured per paragraph and post.
- Phrase index memory is measured at 50,000 and 1,000,000 destinations. It, the
  language identifier, the name model, the tagger and the language layer count
  in the inference jobs' budget, inside the service slice, within the memory
  limits in AGENTS.md section 5.6, with no swap and 15% headroom. Language-layer
  memory budgets per mode are readiness blockers owned by the language-resource
  admission record. A mode without an admitted measured budget cannot build or
  load. No mode, setting or preset may pass the total; only the chosen method
  loads, a switch that does not fit is refused with the reason, and over budget
  refuses start-up.
- Fixed elsewhere and used here: pieces of the piece size, 1,024
  tokens by default and never fewer, with the overlap setting; extras
  read in full; parts of at most 16 MiB (0022); anchors of 3 to 80
  characters and 8 words, 20 keyphrases per post (0023).
- Private data: link reconstruction samples, the forum gold set and
  their results stay on Dell; only reviewed aggregates leave it.

## Footprint budget

This part adds nothing to public pages. It runs on the owner's PC; the
only change a page shows is a link or text change a moderator accepted,
whose footprint the staff suggestions spec (0028) owns.

## How it is tested

### Unit, property and snapshot tests

- Unit tests are written test-first, and every fixed bug gets a
  regression test that failed before the fix.
- proptest over generated posts: segments rejoin to the input; map
  invariants (monotonic, unit-aligned round trip, every readable
  character mapped, cluster boundaries); decomposed and composed posts
  give the same readable text with stored text unchanged; no masked
  character in readable text, match key, pieces, candidates or errors;
  no candidate overlaps a mask, a smilie or any zone except
  `wp_quote`, or cuts a word or cluster; insertion removes cleanly,
  falls on boundaries, nests no link and adds exactly one; insert,
  edit elsewhere, undo gives the edited input; word boundaries are
  cluster boundaries and sentences cover paragraphs; 0022's piece
  properties with the fake counter; the same input always gives the
  same output.
- Generators weight every tag and custom tags with random options,
  random nesting, malformed forms (unclosed, tangled, stray, mixed
  case, unbalanced quotes), smilies, every address form, and text
  pools of English, decomposed accents, emoji sequences, Chinese,
  Japanese, Thai, right-to-left with bidirectional controls,
  zero-width characters, soft hyphens and all three line endings. A
  failure records its seed; minimised failures are committed.
- Fixtures: every item of the edge-case list on BBCode, extraction,
  placement, addresses and Unicode has one, with a traceability list,
  joined to the synthetic fixture of about 300 threads and the
  conformance corpus. All text is synthetic. Fixtures stay within the
  300-line and 80-character limits, write invisible characters as
  escapes (`\u{200B}`), join long lines with a trailing backslash, and
  build large posts from seeds.
- insta snapshots record segments, zones, readable text, the map at
  every unit, words, sentences, language, links, candidates, insertion
  and undo. A first-party formatter wraps them within limits; an
  over-large output is split or kept as its SHA-256 with first and
  last lines. A changed snapshot needs a reviewer's acceptance and
  reason; a change to readable text, zones or candidates must change
  `RULES_VERSION`.

### Fuzzing, races, chaos and load

- cargo-fuzz targets, 15 seconds each at the push tier and 10 minutes
  each nightly: `parse` (any bytes; no panic; tiles UTF-8 exactly or
  SUT-402); `readable_map` (random masks; map invariants hold);
  `insert` (refusal, or an insertion passing `verify_insertion`);
  `undo` (insert, random edits outside, undo equals the edited message
  without the tags); `classify_url` (no panic); `resave` (no panic;
  reparsing gives the same tags and readable text); `structured`
  (malformed documents from the `arbitrary` crate through all of
  these). A crash files an issue through autoissues and becomes a
  committed regression case. Corpora are synthetic.
- loom: `PhraseIndex::update` during `find` never shows a half-updated
  index; the name finder, tagger and identifier are safe to share
  across threads.
- Chaos, nightly: kill the inference job mid-batch, remove or corrupt
  model files, exhaust budgets at random; each post ends either
  finished or waiting with SUT-403, SUT-407, SUT-425 or SUT-426, never
  half finished.
- A quick load test of at most 2 minutes at the push tier: parse,
  readable text, candidates and insertion over a fixed-seed mix,
  checked against the stored baseline.
- A scanner checks that no fixture text reaches logs, metrics, traces
  or error messages.

### Benchmarks and coverage

- criterion benchmarks on Dell, inputs from fixed seeds: parse of a
  typical, 150,000-character and 5 MB post and two tag bombs;
  `readable` with masks; match key of decomposed text; words for
  English, Thai and Japanese; sentences; language per paragraph;
  keyphrases and n-grams; index build and find at 50,000 and 1,000,000
  destinations; fuzzy similarity; placement; insertion; the resave
  check; undo; pieces with the fake counter; names and word tagging;
  the language layer's stages. Each speed target in Limits is checked
  against stored baselines by the 5% median rule (AGENTS.md 6.8).
- Hard-rule code (zone classification and never-anchor marking, masks,
  the map and offsets, placement and `resave_form`, the address
  builder, insertion and its check, undo and its check, limits and
  budgets) lives in `src/hard/`. Only components entered in ADR-0020's
  strict-core registry require 100% line, branch and MC/DC coverage and
  100% killable-mutant detection; equivalent mutants need a written
  equivalence proof and independent review. Other code follows the
  ordinary feature test and coverage gates. `dead_code` and `unused`
  lints are errors; unused dependencies are refused.
- Critical hard-rule code is proved with Kani, Verus or Creusot where
  the proof obligation and tool support are recorded; for example, an
  accepted insertion never crosses an excluded zone or changes text
  outside the inserted tags. Package internals are qualified through
  their admitted artefacts and contract tests, not represented as proved
  first-party Rust.

### Acceptance cases

Starred (★) cases block any release they touch.

- Parsing: SH01 segments tile every input; SH02 tag case is ignored and
  unknown names are text; SH03 ★ quotes nested 3 and 5 deep are fully
  skipped, text between read; SH04 attribution never readable; SH05
  CODE, `CODE=php`, ICODE, PHP and HTML are single skipped zones; SH06
  `[PLAIN]` read as typed, never placeable; SH07 unclosed QUOTE runs to
  the end, tangled tags reopen, 21-deep tags are text, matching the
  corpus; SH08 custom tags read, never-anchor; SH09 Markdown-like text,
  `&amp;` and literal `[url]` read as shown; SH10 ★ all URL forms,
  EMAIL and bare addresses recorded with form, quote flag kept; SH11 ★
  `[USER=123]@name[/USER]` never an anchor; SH12 IMG, ATTACH and MEDIA
  give only `alt` extras, each read in full; SH13 spoilers
  and titles skipped; SH14 items, cells and headings are paragraphs;
  SH15 `:cool:` is a space even in "great:cool:idea", `:COOL:` is
  text, code inside CODE; SH16 LF, CRLF and CR kept, CRLF two code
  points; SH17 "no readable text" cases; SH18 tag bombs within
  targets and memory; SH19 SUT-402, and 16 MiB plus one byte in one
  message or part SUT-401;
  SH20 (Dell) readable text, zone kinds and `resave_form` equal
  XenForo's on the conformance corpus.
- Text: SH21 ★ decomposed posts give NFC with stored bytes unchanged
  and code points equal the PHP fixture's offsets; SH22 ★ no boundary
  inside emoji, ZWJ sequences or accents, offsets equal `mb_` ones;
  SH23 ★ censor masks as XenForo shows, decomposed too, letters nowhere
  else, no placement touching a mask; SH24 `to_stored` refuses
  crossings; SH25 soft-hyphened "power" shares a key and the anchor
  covers the stored hyphen; SH26 accent, case and "ß" rules.
- Segmentation and language: SH27 abbreviations and "5.1"; SH28 Thai,
  Japanese and Chinese words never cut; SH29 bidi and mixed-script
  refusal; SH30 main-language rules both ways, no candidate in the
  French paragraph; SH31 a 12-letter reply takes the thread's language,
  unknown gives "skipped: language unknown"; SH32 SUT-407; SH77 a
  non-voting short paragraph may hold a link, an unnamed voting one
  may not.
- Pieces: SH33 0022's piece cases pass with the fake counter and one
  token over the piece size is SUT-409; SH34 (Dell) counts, overflow and
  chunk receipts equal the pinned `BAAI/bge-m3` tokenizer contract at
  revision `5617a9f61b028005a4858fdac845db406aefb181`; spaCy token counts
  are never substituted.
- Links and matching: SH35 every address form gives the right kind and
  number, `/THREADS/1/` is unreadable; SH36 keyphrases equal the
  snapshot, at most 20; SH37 stemmed match keeps "power supplies" as
  typed; SH38 "cat"/"category" and "Apple" cases; SH39 the index finds
  exactly what a plain search finds; SH40 close-spelling cases
  ("Raspbery Pi" yes, "Raspberry Po" no, "Jon Smith" no); SH41 no
  candidate from any excluded place or shape; SH42 candidate text
  equals stored characters; SH74 a bare address is never-anchor and
  an existing connection.
- Names and types: SH66 fake finder labels route correctly, refused
  kinds never become candidates, word-cutting labels are dropped; SH67
  (Dell) the pinned spaCy pipeline's mapped labels, confidences or
  abstentions and exact spans equal the frozen snapshot within the
  admitted memory budget, and every unknown source label is refused;
  SH68 SUT-425 with no unlabelled names; SH75 no setting disables
  recognition, "Labour", "Catholic", "Welsh" and work titles give no
  candidate or title phrase, unlabelled repeats are dropped, no anchor is
  edged by a pronoun or verb, and SUT-426 waits; SH78 "the new power
  supply" gives "new power supply" only when the admitted packaged noun-
  chunk route supports it, and hard rules still apply.
- Placement and insertion: SH43 ★ half-in-bold refused, wholly-in-bold
  nested inside; SH44 ★ no link in a link, bare address or EMAIL; SH45
  block crossings refused, within one heading or item accepted; SH46 ★
  removing the tags gives the input and readable text is unchanged;
  SH47 ★ short-form addresses only, unsafe board address SUT-414; SH48
  ★ SUT-411, SUT-412, SUT-423; SH49 decomposed bytes untouched; SH50 ★
  (Dell, with the add-on) a racing edit stops the save, a stored
  difference rolls back with SUT-416, the edit is silent with no bump,
  alert or note; SH69 resave cases give SUT-424 for insert and undo;
  SH70 SUT-421.
- Undo: SH52 later edits around the link survive; SH53 SUT-418 and
  SUT-417 cases; SH54 quoted links never removed.
- Preview: SH51 the preview build and service build give byte-identical
  serialised X-ray, placement, insertion and undo for the corpus and
  10,000 generated posts; SH55 SUT-419; SH56 SUT-420; SH57 UTF-16
  spans equal converted code-point spans; SH58 the preview build holds
  no identifier, tokenizer, index, name model or tagger, makes no
  request, and gives no raw text to a viewer who could not edit.
- Limits and checks: SH59 SUT-403 with no partial result; SH60 every
  `TextError` variant has a code, catalogue entry and manual page and
  no message holds input text; SH61 the 2.5 growth bound; SH62 every
  speed target checked by the median rule; SH63 fuzz runs; SH64
  coverage and mutants; SH65 snapshot and rules-version rules; SH72
  the load test; SH73 loom; SH76 (Dell) the conformance command
  refuses without development mode or marker and is absent from
  release builds.
- Measure and models: SH71 (Dell) link reconstruction follows 0023's
  frozen protocol with no regression; SH79 (Dell) the first model
  choice names every candidate and its figures.
- Language layer: SH81 packaged token offsets round-trip to exact source
  bytes and code-point spans, including empty text, Unicode and excluded
  zones; SH82 packaged sentence ids preserve order and never redefine
  deterministic anchor boundaries; SH83 UD tags, lemmas, morphology and
  dependencies appear only from declared components; SH84 malformed,
  stale or wrong-language responses fail closed without input text in
  errors; SH85 missing trained capabilities return the explicit
  unsupported outcome and never invoke an English fallback; SH86 noun
  chunks remain within their declared sentence and absent support stays
  absent; SH87 pipeline manifests, package locks, artefact hashes,
  settings and mode resource bounds are validated with distinct error
  codes; SH88 CoNLL-U conversion preserves admitted fields and conversion
  or inference failure makes the post wait; SH89 CoNLL-U round-trips and
  bad lines name their line number; SH90 a `SpacyFeatureSet` covers only
  the declared source spans and duplicate, crossing or out-of-range
  offsets are refused; SH91 frozen held-out quality floors are met by the
  admitted packaged pipeline; SH92 a declaratively trained pipeline
  reproduces its locked data, configuration, command and artefact hashes,
  with no first-party Python production source; SH93 speed and SH94
  memory targets include the bridge and packaged runtime; SH95 anchor
  recall is not lowered.
- Reports: SH96 no member or person name and no cell under
  `analytics.min_count`; SH97 no post a guest cannot see; SH98 no
  report changes a post, link, score or ranking; SH99 no report joins
  two sites.
- Member text: SH100 candidates only from forum words or a moderator;
  SH101 no person's name offered; SH102 no model produces a candidate,
  and with the option off a link on a misspelt word is refused.
- No text cut: SH103 a piece size under 1,024 is SUT-430 naming
  `reading.pieces.tokens`; at 1,024 and at 2,048 no piece passes the
  size; SH104 overlaps are whole sentences within
  `reading.pieces.overlap_tokens` at 0, 64 and 128, whole words when one
  sentence is longer, and a value over a quarter of the piece size is
  SUT-430; SH105 a 40 MiB WordPress post read by `ReadStream` in three
  parts gives the same segments, readable text and pieces as a test
  build reading it whole; no character is dropped, no part is refused,
  tags open across a part edge close correctly, and peak memory stays
  within one 16 MiB part's bound; SH106 a 5,000-character `alt` text is
  one never-anchor extra, read in full; SH107 reading a 16 MiB post,
  BBCode and HTML, peaks under 48 MB measured with the counting
  allocator in a benchmark program, each part under its line in the
  worked peak, and the message buffer is freed when reading ends; SH108
  `ReadStream` gives the same readable text, zones, links and pieces as
  `parse` with `readable` on the conformance corpus; SH109 no piece
  holds over 64 KiB of text, a 70 KiB word splits at grapheme boundaries
  and is counted; SH110 a part whose checksum fails is not parsed and
  nothing from it is stored; SH111 a WordPress post with blockquotes
  nested two deep lists each as a `wp_quote` zone, its text stays in
  pieces, its links keep their flag and count, and an anchor candidate
  may sit inside it; a 16 MiB post of nested blockquotes lists at most
  20 open `wp_quote` zones at once and stays within the 4 MiB parser
  limit; 40 nested blockquotes list 20 zones, each closed by its own
  matching tag, and after 21 openings and 20 closings a link still sits
  in a `wp_quote` zone; a stray closing tag is ignored, and an unclosed
  blockquote's zone runs to the end of the post.

### Link reconstruction and pipeline choice

- On Dell, 0023's frozen protocol hides at least 2,000 member-written
  internal links, sampled with a fixed seed from the 50,000-thread tier,
  keeping the words. shodhan's figure is anchor recall: the share of
  hidden links whose words, or an overlapping span, are among its
  candidates. The first frozen run sets the baseline; later changes to
  phrase sources, stop list, fuzzy rule, spaCy pipeline, label mapping or
  feature adapter may not lower it beyond the protocol's interval.
- Initial pipeline choice compares every admitted candidate on one frozen
  sample and selects the highest anchor recall among those meeting all
  feature-specific quality, safety and resource gates. A later challenger
  is evaluated and promoted only through spec 0077's single controlled
  route; this specification supplies feature gates, not a separate model
  activation path. Those gates include:
  1. anchor recall gains 2 points or more at the low end of the paired
     95% interval (10,000 re-samples, fixed seed);
  2. 0023's reconstruction recall and anchor agreement do not get worse,
     each interval's low end no lower than −1 point;
  3. no subgroup (thread type, post length, destination popularity or
     section) is worse with 95% confidence;
  4. no more refused names leak on the labelled refusal set and no
     unknown source label is silently accepted; and
  5. memory and speed, including the packaged runtime and bridge, stay
     within budget on the reference machine.
- The protocol file and its SHA-256 are fixed before any score. The
  comparison re-runs on any change to a pipeline or package artefact,
  label mapping, abstention rule, minimum score, phrase sources, stop
  list, fuzzy rule or rules version, and for every release candidate.
  Each result goes to the owner in plain English; the sample is never
  redrawn to get a pass.
- Rust-controlled lab runs record inputs and outputs:
  `shodhan-spacy-names-*` (pipelines, label mappings and the refusal set),
  `shodhan-spacy-tags-*` (token and tagging features) and
  `shodhan-spacy-parse-*` (dependencies and related features). These are
  evaluation names, not bespoke model implementations.
- Held-out measures use paired bootstrap intervals and feature-specific
  floors frozen in a hashed protocol: UPOS and lemma accuracy given
  tokens, token F1 and sentence F1 on raw text, exact-span entity F1 and
  refused-label leakage, LAS/UAS where supplied, noun-chunk F1 where
  supported, and corresponding forum-gold measures. Freeze and hash the
  test first; qualify the evaluator by its six self-checks; split by
  thread and time; keep statistics in mulyankan; report a result kind,
  not a bare pass; require an independent challenger and reviewer; and
  issue one signed receipt. The test split never trains or tunes.
- Training, when required, invokes the pinned packaged spaCy training
  command with declarative configuration and admitted data. Rust records
  configuration and data hashes, package lock, command receipt and output
  manifest. Evaluation reads the forum gold set only as the ids, spans,
  positions and labels allowed by spec 0066, never as post text. The
  reconstruction comparison runs before the winner is pinned in
  `models.lock`. Passing rivals remain evaluation candidates but cannot
  be activated outside spec 0077.

## Changes from the source

- The browser build of the crate is replaced: the desktop app's review
  preview links the same deterministic Rust parsing crate with default
  features, and preview parity is tested between the two builds. The
  preview does not load spaCy or BGE artefacts.
- Parity and timing tests for first-party integration run as Rust tests.
  Packaged commands are invoked only through their admitted interfaces;
  no first-party Python production test double becomes a production path.
- Bespoke Rust name, tagger, lemmatiser, dependency and neural-network
  candidates are removed. Approved spaCy packages and pipelines supply
  those algorithms through the single typed feature contract.
- First-party Rust denies unsafe code, including the preview binding.
  Native code inside admitted third-party packages remains a supply-chain
  and contract-qualification concern and is not misrepresented as proved
  first-party Rust.
- The deterministic Rust hard rules retain recorded proof obligations.
  The independent second edit validator remains Rust with proofs.
- Packaged spaCy training replaces ganit's bespoke NLP training route.
  mulyankan continues to own statistical evaluation; ganit does not train
  a competing tagger or entity model.
- The 5% best-result ratchet becomes the 5% median rule of AGENTS.md
  6.8; absolute speed targets stay; the 20x goal is kept (spec 0007).
- ADR-0020's strict-core coverage, mutation and independent-review rules
  replace universal whole-crate MC/DC and mutant requirements.
- The old gate names and toolchain-only runs are replaced by the push
  tier and nightly runs.
- Ordinary packaged dependency provenance replaces the source document's
  blanket prohibition on reading or naming outside NLP toolkits. Strict
  clean-room separation applies only to components explicitly designated
  for clean-room reimplementation.
- Old role names (serve, worker, inference) become the service's admitted
  Rust jobs and packaged model jobs inside the governed resource slice.

Source: `docs/specifications/text/README.md`, `shodhan.md`,
`shodhan-2.md`, `shodhan-3.md`, `shodhan-4.md`, `shodhan-5.md`,
`shodhan-6.md`, `shodhan-7.md`, `shodhan-8.md`, `shodhan-9.md`,
`shodhan-10.md`, `shodhan-11.md`, `shodhan-12.md`, `shodhan-13.md`,
`shodhan-14.md`, `shodhan-15.md` and `shodhan-16.md` in the archive
`archive-2026-11-08-old-repo`.
