# 0072: RankMedulla and SpamGuard

RankMedulla judges whether a link would help a reader and orders the
choices a moderator sees. SpamGuard looks for attempts to distort that
choice and explains its cautions. Both use Sutradhar's own designs and the
owner's own site evidence. Public descriptions of search systems give
research questions. Our charts and scores describe the site and our model.

## Where each part belongs

| Part | Owner and contract |
|---|---|
| Reading and storage | Sampark reads signed guest-visible revisions (0021). Shodhan supplies readable pieces, their primary ranges and the active index generation (0022, 0026). |
| Text models and matching | Specs 0024 and 0023 own the pinned embedder, reranker, candidate search, safe anchors and matching's hard filters. |
| RankMedulla | The existing `shodhan.rank` path owns pair scoring, calibration, the relevance gate, ordering and explanations (0027). Its full-body method uses the existing method registry (0055). |
| SpamGuard | Kavach owns detection and the existing `SpamSignal` contract. The review view uses the existing moderation hint route (0051), with no member sanctions. |
| Graph and activity | Specs 0049, 0034 and 0050 own graph facts, human rates and votes. Each producer supplies its existing typed signal or an admitted method input. |
| Learning | Ganit owns maths, the lab owns comparisons and the tuner owns activation and rollback (0053, 0056). |
| Search changes | QuarryView owns event bands and separate metric series (0043). GanitEvolve owns research trials and evidence receipts (0012, 0013). |

## RankMedulla

### Full-body evidence

Build one body summary for each complete guest-visible post, thread or
resource revision. Reuse spec 0022's stored piece vectors and stream them
in stable piece order. Give each piece the weight of its body tokens,
leaving out the leading overlap that spec 0022 copies from the piece
before it, so each body token counts once. The thread or resource title
that spec 0022 repeats at the start of every piece adds no weight; its
tokens stay inside each piece vector, so the mean still covers the title.
The replies together carry at most the opening post's total weight, or a
resource's description's: when their total is above it, every reply's
pieces are scaled by the same factor down to it. When the opening post or
description has no body weight, the replies keep their own weights. So
when the page's own subject has body weight, replies, however many or
long, cannot outweigh it. A total weight of zero is unavailable evidence.
Normalise each piece vector and keep the weighted mean in 64-bit
precision. Titles and extras also keep their existing retrieval and
ranking roles. Quotes, masks and private zones follow the parser's
existing rules.

For a source and destination, compute each mean's dot product with the
other mean normalised to unit length. Each direction measures how well its
readable ranges agree with the other body's topic. Clamp the smaller
result to the method score range in spec 0027; a value above 1 by more
than rounding error is unavailable evidence, like a non-finite number.
Treat an empty body, missing range, mixed generation, zero norm or
non-finite number as unavailable evidence. Keep the two directional scores
and the method's reason code for review. A weak direction remains visible
even when the other direction is strong.

Bind the summary to the site, canonical content ID, revision, reading
rules and active model generation. Bind both summaries to the ranking
result. An edit, a visibility change, or a new, removed or moved reply in
a thread invalidates the matching summary and affected proposals. Build
summaries through the existing resumable reading job. Check completion
against the versioned piece inventory before replacing a summary. Each
pair uses fixed-size vectors, so long bodies need no new model calls per
pair. The bounded-window reranker still checks the anchor in context.

### Decision and display

1. Run the existing hard filters before scoring. Read spec 0028's
   content-link count for the whole source thread or resource, including
   links in all its readable posts or updates. At or above
   `linking.thread.max_links` (spec 0014), skip new link proposals for
   that source. Count link occurrences through the canonical parser and
   existing link inventory. Missing or incomplete inventory keeps the
   proposal waiting. Use the cap again at apply time in the same
   revision-checked write as the edit (0028). A competing edit must leave
   the cap intact.
2. Run the admitted body and other relevance methods. Require the reranker
   and calibrated acceptance to clear their existing thresholds (0027).
   Partial body evidence can appear as provisional work with a reason.
   Only complete, current evidence can yield an approvable proposal.
3. Order the passing candidates using the existing typed need, quality,
   freshness and trust signals. Use bounded human rates and the allowed
   structural counts. Apply existing freeze rules on bot traffic. None of
   these signals can admit a refused candidate.
4. Return the result through the existing `Ranker` and `Calibrator`
   contracts. The explanation records both content revisions, method and
   calibration versions, coverage, gate outcome and stable reason codes.
   Break score ties by canonical destination ID. Keep existing suggestion,
   signed review, acceptance and undo routes (0028, 0029).

## SpamGuard

Kavach reads the same guest-visible revision and its site-local facts.
SpamGuard checks posts and the links inside them, on both site types, with
all three evidence families below. WordPress comments are left to Akismet
(owner, 12 November 2026): kavach reads whether Akismet is installed and
active, a signed plugin fact of spec 0021, through sampark's public API
once at the start of every run, treating an absent or stale answer as not
active, and where it is missing or switched off, the explanation surfaces
say that comments are unchecked.

SpamGuard examines three evidence families. Content evidence covers
repetition and duplication, found by kavach's own near-duplicate method
over hashed body token ranges outside links. Under the short-post rule, a
post with fewer than `kavach.duplicate.min_tokens` (default 20) body
tokens outside links and quotes, such as "Thanks!" or an image-only post,
takes part in no near-duplicate pair, and a pair in which either post's
first delivered revision was below that size gives no duplicate evidence
against either post, whoever wrote them, an exception to the same-author
rule below, so a short original that is copied and later expanded never
carries the evidence. It needs no vectors, takes candidate pairs from hash
buckets rather than comparing every pair, and does not reuse ganit job
286, whose piece vectors include link text (spec 0022). Any record derived
from input facts carries the facts it was built from. Link evidence covers
concentration on a destination, anchor repetition and reciprocity using
canonical link facts. Traffic evidence covers bot or anomaly findings
already issued by darshak. Quoted text is in no family, since spec 0022
skips it, and a link inside a quote at any depth, which spec 0022 flags,
is in no family either; on WordPress, spec 0026 lists each `blockquote`
element as a `wp_quote` zone, and kavach itself leaves its text and links
out of every family the same way, whoever wrote the quote or the post it
names; any evidence stays with the quoted post's own revisions, under the
usual revision, closing and erasure rules. A link that Sutradhar itself
inserted is likewise in no family against any post while its suggestion is
in the applied, undoing or undo_conflict state (spec 0028). Kavach learns
of these links only through sujhav's public API, never from sujhav's
store. Before it counts a post's link evidence it reads the states of that
post's suggestions there; while any is approving, or the read fails, that
post's link evidence is unavailable and kavach waits. A failed read is
retried with backoff under spec 0019's job retries, and repeated failure
raises a health alert. Kavach reads again, recomputes the post and
withdraws any hint that loses its support on each of sujhav's
`suggestion.approved`, `.applied`, `.apply_failed`, `.stale`,
`.undo_conflict`, `.removed` and `.gone` events, which carry ids, states
and codes only. It keeps its own copy of each exempt link's post id,
suggestion id and destination canonical id, with no text, so the exemption
holds while the link stays, even if sujhav's records are lost, as on an
uninstall, and spec 0021's purge leaves it in place like the delivery
number, epoch and author id record below. These quote and own-link rules
can let a spammer escape by quoting a link, but never make a quoter or an
approved suggestion carry evidence. Keep legitimate templates, technical
identifiers and repeated navigation separate from readable body evidence.

Traffic to a page is open to outsiders, so traffic findings only freeze
that page's traffic evidence; they never lower its place and never count
as a family towards a content hint against it. Content and link evidence
count only against the post or resource update whose own text carries
them, never against a thread for its replies. A thread's `SpamSignal`
reads only its opening post, and a resource's reads only its description
(Resource Manager keeps it as the resource's first update row, which
kavach treats as the description and never as a separate resource update),
so an outsider cannot frame a rival page. When spec 0023 picks a reply as
a link's target, the spam risk method reads that reply's own `SpamSignal`,
and the opening post's signal never stands in for it.

Kavach keeps, set once in its own store, the delivery sequence number and
counter epoch (spec 0021 item 6) with which sampark first delivers each
revision guest-visible, kept through every loss of guest visibility (soft
deletion, unapproval, a move or a password) until the content is
hard-deleted or the revision bound below drops it, and the author id at
that delivery; the author id is used only for the author rules below
(same-author, cross-author, own-author pattern members and reciprocity)
and is removed when the member is erased. For each delivered revision that
differs from the last one kept, kavach also keeps one fixed-size
near-duplicate signature and its link keys, each a keyed hash under a
per-site secret key that tijori holds, with internal links kept as
canonical ids, and never text, a URL or an anchor. Its bucket index holds
one entry per content item and key, with the earliest kept revision that
holds it, so a post's place is judged from its first matching revision
after later edits. At most `kavach.revisions.max` (default 50, at least 2)
revisions are kept per content item, always the first and the latest, and
the bound covers their order rows, signatures, link keys and index
entries. Signatures, link keys and index entries are purged when the
content stops being guest-visible, as spec 0021 purges content, removed on
hard deletion or the author's erasure, and dropped when tijori's per-site
key changes. Once any kept data of an item is dropped, by the bound, a
purge or a key change, the item keeps one fixed-size marker, and only a
match at its first revision, with that revision's data still present, can
have a known order; every other match of that item, and every pair whose
matching revision was dropped or purged, has none. The delivery number,
epoch and author id record holds no text, so spec 0021's purge leaves it
in place. Each kavach revision maps to one sampark content state of its
content type and id, as spec 0021 item 6 defines it. On WordPress, a post
is saved before the plugin's hook numbers its change, so another post can
go public in between; two WordPress revisions by different authors
therefore never have a known order. Otherwise, order between two revisions
is certain only when both carry their own numbers in the same epoch, the
lower being earlier, and each post's or resource update's first
guest-visible delivery came from a note of the kind created, published or
approved (spec 0021 outbox items 1 and 6 and its WordPress change events,
which number a creation, approval or publication only while the content's
shown-before flag was clear); a post first seen guest-visible on any other
delivery, such as an edit after its section was opened to guests, has no
known order. Every other pair, including numbers from different epochs and
any delivery with no number, has no known order. Kavach never guesses an
order. If the counter it reads from sampark's public API is below the
highest own number it had recorded in that epoch before the read, or a new
epoch starts (`sampark.epoch_started`), kavach treats every number of the
old epoch as having no known order, recomputes that epoch's evidence,
emits `spam.content_hint_withdrawn` for any hint that loses its support
and raises a health alert. This can let a copy escape evidence, but never
makes an original carry another author's. Never order by outbox id, event
id, handling order, the time of a current revision or a date the author
sets. Keeping order through restores, late notes and lost tables is
outside this spec. Order is judged pair by pair. For two posts that
directly match as near-duplicates, each side's place is the number of its
first delivered revision that directly matches the other; duplicate
evidence between one author's posts counts only against the side with the
later place, under the same-author and cross-author rules below, never
through shared neighbours. A post joins a link pattern at its first
delivered revision that holds the link. So a later edit to the original
does not make it a copy, and an old post edited to copy a newer one is the
copy.

Some posts have no known order. A no-order change is a backfill or
reconciliation run, a section newly made guest-visible (the permission
watch), or a delivery whose note is any kind other than one created,
edited, published or approved (spec 0021 outbox items 1 and 6), such as a
move, merge, split, restore or a section deletion that moves threads, or a
WordPress post made public (a password removed, a private post made public
or a post published again), whoever made it, since others may have read
the content while it was hidden. A post first delivered by a no-order
change has no known order against any other post. Duplicate and link
evidence of every kind between two posts with no known order, reciprocity
included, counts against neither, except between posts by the same author
(below). This can let a copy escape, but never frames an original that
kavach saw first. So content first delivered by a backfill, which has no
note and so carries author id 0, carries no duplicate or link evidence
against any other post. A post that was never guest-visible, such as a
draft, a scheduled post or one awaiting approval, takes the number with
which sampark first delivers it when it is published or approved. Under
the same-author rule, two posts by the same author are never exempt for
lacking an order, except under the short-post rule above: duplicate and
link evidence between them counts against the later one when their order
is known, and against both when it is not. The same author means the same
registered member id (the WordPress user id on WordPress), as kavach
stored it at each revision's first delivery; a guest or deleted member's
id of 0 never matches. Kavach takes the author id from sampark's delivery
event, which carries a non-zero id only for the content author's own save,
under spec 0021 item 6 and its WordPress paragraph; a save by the feed
reader, an import, cron, a form, an application password, the admin
control panel or another user carries 0. Posts that WordPress users
holding the publish capability (spec 0021) publish under one shared
account count as one author, and this spec's guarantee against outsiders
does not cover what those users publish.

Link evidence counts against the content whose revision holds the links;
links pointing at a page never count against that page. A link pattern is
keyed by its canonical destination: an internal link's canonical id (spec
0022), or an external link's URL as spec 0026's parser reads it from the
stored message, with scheme and host in lower case and any default port
and fragment removed. Its anchor is the link's visible text from the same
parser. A pattern is keyed by that destination, or by the destination with
a normalised anchor, never by an anchor alone. A post's concentration and
anchor repetition support counts only its own author's members of the
pattern ordered before it or with no known order to it, and each further
occurrence of the pattern's link in its own text; other authors' members
count only against themselves. So the earliest joiner gets none from other
authors' posts, and no flood by others ever counts against a post.
Reciprocity is a pair of posts, each linking to the other's page. It
counts only when both posts have the same author, and then against the
post whose link joined last and completed the pair, or against both when
their order is not known; a pair by different authors counts against
neither, so an outsider cannot bait it.

Count qualifying support by family. Each detector counts in the one family
its catalogue sheet (spec 0055) declares, and every one of its inputs must
belong to that family; a detector with inputs from more than one family is
refused. A detector in the traffic family reads only traffic inputs and
never enters `SpamSignal`. An input fact is one canonical fact record: a
link inventory entry with its anchor, a body token range outside links, or
a darshak finding. Identity keys such as content ID and revision are not
input facts. Content detectors read body token ranges outside links; link
detectors read link inventory entries. A repeated detector, or detectors
joined by shared input facts, directly or through other detectors, give
one family's support whatever families they declare, so one input can
never supply two families. Kavach's detection and the spam-view estimate
below run as the existing resumable, bounded jobs.

Kavach emits one versioned `SpamSignal` with the site, content revision,
producer version, freshness and reason codes. The admitted spam risk
method, one of spec 0027's caution methods, keeps its existing evidence
contract. `SpamSignal` holds the counted qualifying support of the content
and link families only. Under the cross-author rule, duplicate evidence
between posts by different authors is a diagnostic only: it never counts
in `SpamSignal` or towards a content hint, and only duplicates between
posts by one author count. So content first published where kavach cannot
see it, such as a members-only section, a conversation or another site,
and copied publicly first by someone else, is never lowered and never
hinted for that copy. Content and link evidence reach ranking only through
`SpamSignal`, never through the trust door. The spam risk method reads
only `SpamSignal` and keeps spec 0027's caution contract and spec 0069's
fusion rules; SpamGuard adds no other route into the fused score or the
gate. These, and only these, read `SpamSignal` or the spam risk column:
the spam risk method, with its own lab trials, tuner runs and variants in
its caution role; every fuse step, in the live scoring model and in chains
(specs 0068 and 0069), with its job 45 weight training, the tuner's weight
steps and job 46's feature importance, each reading the spam risk column
only as written and turned by its direction; and the explanation surfaces,
where the spam risk method's reason codes by family are shown only to
holders of "View content hints" and other reviewers see its score, weight
and importance and that a spam caution applied, as spec 0027's explanation
says. After the fuse step, these and only these read the fused score or
the calibrated probability, as specs 0027, 0028, 0053, 0056, 0066 and 0069
already set: calibration (job 33), the relevance gate, the order key, job
2's replacement margin, job 3's display threshold, job 4's review-queue
order, job 34's conformal count, jobs 95 and 142 (spec 0027), job 192 and
the tuner's replay of current and proposed scores with jobs 128, 129 and
147 (spec 0053), the confidence filter on spec 0028's review queue, the
explanation's confidence, spec 0069's probe-set selection, lab comparisons
and release evidence. Lab comparisons, release evidence and the tuner's
replay may measure them, and a guard that compares them with moderators'
choices, as spec 0053's step 3 does, is allowed. Job 147's offline policy
scoring (spec 0053, step 3) may weight logged decisions by ratios from
them; but no tuner, lab trial or GanitEvolve variant may otherwise weight
or filter its cases by them or use their own values, such as the mean
calibrated probability of accepted links, as its objective, fitness or
guard. No other method, job, detector, tuner, lab trial or GanitEvolve
variant may declare `SpamSignal`, the spam risk column or any value
computed from either as an input, objective, guard, fitness, label, case
filter, sample weight, split or stopping rule. Outside the spam risk
method's own trials, tuner runs and variants, no objective, fitness,
guard, label or stopping rule may be built from spam risk itself, such as
the spam risk of accepted links. Outside those trials, tuner runs and
variants, the false cautions on legitimate content that the release
evidence below records may serve only as a comparison's predeclared
permitted harm. Kavach's own SpamGuard findings on guest-visible content
and its links outside `SpamSignal`, such as its cross-author diagnostic,
single-family support, duplicate clusters and traffic findings, and any
value computed from them, are read only by kavach itself, its detectors
included, to build `SpamSignal` and its hints, reading the cross-author
diagnostic only to leave cross-author pairs out and traffic findings only
in its traffic detectors, by the hints module for content hints only,
never account hints, by the explanation surfaces, which show them only to
holders of "View content hints", by the spam-view estimate and
GanitEvolve's own search over its versions, and, for traffic findings
only, by the trust door of spec 0027's order stage; no other method, job,
detector, tuner, lab trial or GanitEvolve variant may declare them in any
of the uses above. Kavach builds only these values from these findings for
other readers, each under its own rule in this spec: `SpamSignal` and the
spam risk column, under the list above of the only readers of
`SpamSignal`; content hints, under the hint rules of this section and spec
0051; and the spam-view estimate and its bands, under the section "Our
estimate of a search engine's spam view". The trust door's output from
kavach's traffic findings, the frozen traffic rate and the door's trust signal,
is not limited by this reader rule either: it is read like any other
order-stage signal under spec 0027, it only freezes traffic evidence and
never lowers a place, and its "traffic frozen" label stays visible to
every reviewer, as spec 0027 says. Spec 0050's guest vote checks read
kavach's spam check on the vote itself (item 4 there), which is not
SpamGuard evidence. Outside the hints module's own handling of content
hints, no method, job, detector, tuner, lab trial or GanitEvolve variant
at all may declare, in any of those uses, content-hint rows,
`content_hint_evidence`, `hint.raised` or `hint.decided` of the content
kind, the `spam.content_hint_*` events, or any value computed from them,
so a moderator's decision on a hint never trains or steers anything. Spec
0027's gate check and spec 0055's catalogue check (`sutra methods check`)
each refuse every declaration outside these rules. The traffic family
reaches ranking only through the trust door of spec 0027's order stage,
since rule 4 there keeps traffic out of relevance. The spam-view estimate
below never enters `SpamSignal`, the trust door or any method input.
Through that door, SpamGuard's traffic findings only freeze traffic
evidence, as above. The trust door can never improve relevance or promote
a target. A single family is a diagnostic for hints, though its support in
`SpamSignal` can still lower relevance through the spam risk method. A
content hint needs current support from at least two separate families
counted against that content, the rule spec 0051 applies to account hints,
here applied to SpamGuard's families. A current `SpamSignal` with no
qualifying support gives the spam risk method a current "no finding"
value, so a clean page does not wait. Missing or stale output is
unavailable, not zero, and the proposal waits under spec 0027's rule for
missing inputs; it is never read as an accusation or as clean.

A content hint holds one content ID and revision, never accounts, and
closes when its revision is superseded, its content is deleted or it stops
being guest-visible. Kavach emits each content hint as a typed outbox
event, `spam.content_hint_raised`, for one flagged content revision. The
event carries that content ID and revision, kavach's finding ID, the
families, strengths and stable reason codes only, never post text, names,
anchors or a list of other content (spec 0019). The hints module reads the
related content, such as a duplicate cluster, through kavach's public API
by finding ID, and builds each evidence summary from codes and counts.
Panchayat's hints module (spec 0051) consumes that event safely when it is
delivered again and stores it in `content_hint` for both site types. It
closes the hint on sampark's existing revision, deletion and visibility
events (spec 0021), and on kavach's typed outbox event
`spam.content_hint_withdrawn`, which carries only the finding ID, content
ID and revision. On XenForo, content hints appear in the review queue as
one hint kind; on WordPress, where that queue is not available, they
appear only in the explanation surfaces of spec 0027, which read them
through a typed content-hints trait that shodhan declares, with a test
fake, and panchayat's hints module implements, wired in the composition
root like `SpamSignal`. Only holders of the "View content hints"
permission see them: a XenForo permission and a WordPress capability that
no role holds until an administrator grants it, separate from "View
account hints". A member's address, name, private message or email does
not enter this content signal. Use a site-local human-rate source and its
existing privacy floor. A bot anomaly can freeze the last good rate. Where
none exists, leave that rate unavailable. No scorer or hint can delete a
link, alter a post, remove a vote or sanction a member. Edits go through
the existing moderator route. Hold page and evidence data apart by site,
including caches, checkpoints and tuning snapshots. Body vectors stay in
the site's content store. Trials use the lab's existing approved data
route and export numeric results. The two sites stay wholly separate: no
graph, activity or training data is pooled across them.

## Our estimate of a search engine's spam view

SpamGuard also keeps our own estimate of how a search engine's spam system
might judge each site, so a moderator can see which pages look at risk
before a search change does. It is our model, built from public spam
policies and the site's own measured data, and every screen, report,
command, MCP tool, API field, event and export that carries it calls it
our estimate, never a search engine's verdict or a copy of its system
(owner, 12 November 2026).

The estimate reads the same three evidence families per page and per site,
counted under SpamGuard's order, same-author, cross-author, quote and
own-link rules, so a copy's evidence never raises the original's band,
with the site's own Search Console and GA4 series (spec 0043). It reports
a risk band with its support and uncertainty, and the reasons, by family.
With a missing or stale Search Console or GA4 series, the band is unknown,
never low. Like SpamGuard, the estimate never counts links or traffic
aimed at a page against that page: traffic findings only make its band
less certain, never higher. Per-page bands and their reasons need the
"View content hints" permission; site-level bands are shown only to the
staff who may view that site's charts under spec 0043; estimate events
carry IDs, bands and reason codes only. Neither it nor any value computed
from it is ever an input, objective, guard, fitness, label, case filter,
sample weight, split or stopping rule for any ranking, relevance,
detection, tuner, lab or GanitEvolve method or job, except GanitEvolve's
own search over versions of the estimate, whose fitness is its agreement
with the site's measured series. It is a report only: it never enters the
relevance gate, never orders candidates and never acts on a page or a
member. GanitEvolve may search versions of it under the release evidence
rules below; each counts only as specs 0055 and 0071 allow.

## Research and release evidence

GanitEvolve runs frozen, seeded comparisons on synthetic cases and
approved snapshots of the owner's sites. Use the lab's thread and
time splits and held-out evaluation. Record support by topic and site
type, useful relevance and ranking change, false cautions on legitimate
content, calibration, latency, memory and work spent. Predeclare the
comparison, required gain, permitted harm and stopping rule. Shadow
results are reports. The existing tuner and approval path govern any
activation. Keep the last admitted bundle on a failed check and prove
rollback on both site types. The live inference and batch code follow
the Rust rules in AGENTS.md.

Study published core and spam update dates as event bands in 0043. Use
the existing approved notice source and the separate Search Console, GA4
and darshak series. Read closed, measured days. Show support and
uncertainty with the comparison. Use the dates as observations, so the
report establishes no cause or private search system decision.
Search-update chart observations from this study are never an input,
objective, guard, fitness, label, case filter, sample weight, split or
stopping rule for any ranking, relevance, detection, tuner, lab or
GanitEvolve method, and never enter the relevance gate. The site's own
Search Console and GA4 series are likewise never an input, objective,
guard, fitness, label, case filter, sample weight, split or stopping
rule for any detection method, kavach detector, or the spam risk method
or any of its lab trials, tuner runs and variants; within SpamGuard,
only GanitEvolve's own search over versions of the estimate may use
agreement with them. Spec 0027's gate check and spec 0055's catalogue
check refuse these uses too.

A method counts only through spec 0055. Give each genuinely different
computation a stable identity, behaviour digest, nearest-rival test
and independent signed admission against the exact candidate. A
changed weight, seed, label or surface keeps its owning identity.
Each shipped operation gets one capability entry even when it has
desktop, CLI, MCP and both site surfaces. Record its actual
integration and end-to-end proof. Spec 0064 owns the release floors,
variant identity, scope shares and signed group verification. A
trial or group size supplies no credit without its required proof.

## Acceptance tests

Run the same fixed fixtures through the existing test entry points
for XenForo and WordPress. Stub model outputs only where the test
isolates a gate. Also run recorded model inference and moderator
acceptance and undo in the signed end-to-end route.

| Case | Setup and expected result |
|---|---|
| RM01 Full body | Equal-weight unit vectors P = (1, 0) and Q = (0, 1). Source and destination each hold P, P. The body score is 1. Change only the destination tail to Q and the score is 0.5. Repeat with the changed tail on the source side. The two revisions and corresponding evidence change. One reply ten times longer than the opening post, or fifty short replies, together carry no more weight than the opening post; in a thread whose opening post is image-only the replies keep their own weights, so the body score stays available. |
| RM02 Different method | Use the same title and first passage in both destinations from RM01. Swap the body method for title-only or best-passage scoring. The expected score difference disappears and the distinguishing test fails. |
| RM03 Relevance first | Freeze a fixture gate with thresholds 0.70 for both inputs. Pair A returns 0.80 for each. Pair B returns 0.69 for reranker relevance and 0.80 for acceptance. Give B the best possible order signals. Only A passes. A low acceptance input or an input equal to its threshold is also refused with SUT-550. |
| RM04 Incomplete | Remove the tail range or mix two generations. The result names unavailable body evidence, with no approvable proposal. Restore it and use the same test input to reproduce the complete result. |
| RM05 Link cap | On XenForo, where a quote link does not count, set the cap to five and provide four content links, as spec 0028 counts them, in one thread, plus a quote link that does not count. One valid addition can be approved. With five, the proposal is skipped. Start two approvals at four on different posts of that thread, let the first apply, and require spec 0028's thread allowance guard to refuse the other. Repeat at a different configured cap and with a missing inventory. |
| SG01 Corroboration | On WordPress, each case below that orders posts by different authors instead gives no duplicate evidence against either post. Duplicate evidence between different authors below is a diagnostic only, never counted in `SpamSignal` or towards a hint; the synthetic spam fixture is one author's. Repeat a legitimate template on two pages without suspicious links. The content family alone yields no content hint. Add corroborating link evidence for a synthetic spam fixture. The hint names both families, and its content and link detectors share no input fact. Two records in the content family alone still yield no hint. A rival post that others copy into their threads and link to many times with one repeated anchor gets no content hint, and its spam risk is unchanged; the copies and the linking posts carry the evidence. The same holds after the rival edits its post once the copies exist, and when an outsider links back to a rival that linked to it first, or baits the rival into linking back: a pair by different authors gives no reciprocity evidence against either, while one author's two posts in different threads linking each other carry it against the post whose link completed the pair. An old post edited to copy a newer post carries the duplicate evidence; the newer post carries none. An original numbered first, copied, then given a typo fix before kavach runs keeps its first place and carries none. When the original's first matching revision was dropped past `kavach.revisions.max` or purged while the original was hidden, the pair gives no duplicate evidence against either post, and a post edited 1,000 times keeps no more than that many order rows, signatures and index entries; a copy matching any revision of that post other than its first gives no evidence. A member's post first made in a members-only section, copied publicly by an outsider first, then posted publicly by the member with the member's usual repeated link, gets no content hint from the copy and keeps its spam risk and estimate band unchanged by it. After a purge or a change of tijori's per-site key, an original numbered before its copy and edited after it carries no duplicate evidence. A flood of identical spam posts delivered in one batch carries duplicate evidence, ordered by delivery number, and link evidence when one author posts it. Two near-duplicates by different authors, one first read by a backfill run, give no duplicate evidence against either. A rival's members-only post copied into a public thread, then moved public by a moderator, gets no duplicate evidence, and the same holds when the moderator splits the rival's posts into a new public thread, or when the rival fixes a typo just after its section is opened to guests. A WordPress post copied from an older post by the same author, then moved from draft to published by that author in the editor, carries the duplicate evidence; a XenForo copy by the same member published on a moderator's approval gives none, since the approval is not the member's own save. A post made public by removing its password, or a private post made public, gives no duplicate evidence either way, even when the same save also edits it, and the same holds when the private original is first moved to draft and then published, when a published post is given a password, edited, copied by a password holder and published again, and when a post that was password-protected before the plugin was installed is published through a draft. A post published before install, copied after it, moved to draft and published again gives no duplicate evidence against the original, and neither does a XenForo post that is unapproved, copied and approved again, even when it sat in a members-only section that was opened to guests while it was unapproved. A member who quotes a revision that was edited again, or deleted, before sampark delivered it carries no evidence from the quote. A new resource update copied from an older one carries the duplicate evidence; the older one carries none. A delivery that coalesces an edit with a move carries no number and gives no evidence either way. An outsider's post that sits close to both an earlier post and a rival's later post gives the rival no duplicate evidence. A delivery whose handler reads a newer state than its note's, or whose note was found only on asking again after a later note was numbered, carries no number and gives no evidence either way. When the counter read from sampark falls below kavach's highest recorded number, or a new epoch starts, every number of the old epoch has no known order, hints resting on them are withdrawn and a health alert is raised. Numbers from different epochs never order a pair. Two guest posts with user id 0 are never treated as one author. A 15-token post copied with five tokens added, then expanded by its author, carries no duplicate evidence, whether the copy is by its author or another member. Posts under one shared WordPress account count as one author. A new resource with a link in its description gives no duplicate evidence between its description and its first update row. Posts sent through a front-end form, a "fetch now" feed import from wp-admin or a REST post with the admin's application password, all under the admin account, flooding a destination and anchor before an admin article, give that article no duplicate or link support, and so do XenForo feed-reader posts made under a member's account before that member's own post. On WordPress, a near-copy by another author published while the original's save hook is still running gives no duplicate evidence against the original. Short stock replies such as "Thanks!" or "+1" by different members, and image-only posts, give no duplicate evidence. On WordPress, a post that quotes an earlier post in a blockquote gives no duplicate or link evidence from the quote. Part 2 linking to Part 1 by the same author, then Part 1 edited to link back, carries reciprocity against Part 1, whose link joined last. A flood of one author's copies with no known order between them still carries duplicate and link evidence against every copy, even after the author deletes their account. A member who quotes an outsider's spam link in a reply joins no link pattern, at any quote depth, and neither does a fake quote naming no post. A rival whose thread links to an outsider's page and who quotes that page's link back with the Reply button gives no reciprocity against the rival. When Sutradhar inserts a link from A to B and later from B to A, neither post carries link evidence for them, even when sampark delivers the edit before `suggestion.approved` or `suggestion.applied` arrives, or an undo leaves the link in place after a conflict. A post whose suggestion goes stale while approving stops waiting. When a quoted source is edited to drop its link, or deleted, the quoting posts carry none, and a member who quotes a backfilled post holding a link carries no evidence for it. Outsiders who flood a destination with an anchor before or after a rival's single ordinary link to it give the rival no link support from the flood. A flood of one common anchor such as "click here" forms no pattern on its own. A repeated link gives link evidence only, never content evidence. A spam reply in a rival's thread counts against the reply, not the thread, and a link whose target spec 0023 picks as that reply reads the reply's own signal. A detector declared in the content family that reads link facts is refused, so it adds no family. Two detectors whose inputs only partly overlap also give one family. |
| SG02 Traffic freeze | Supply a last good human rate and then a bot anomaly with a fabricated high rate. The retained rate stays unchanged. With no last good rate, the feature is unavailable. Neither state improves order or passes the relevance gate. Traffic findings never enter `SpamSignal`; they act only through the trust door. Bots sent to a rival page freeze its traffic evidence but do not lower its place and add no family towards a content hint. A detector that reads traffic findings but declares the content or link family is refused and adds no family. |
| SG03 Access and ageing | Feed another site's signal, an old revision, and a deleted target in turn. None affects the current result. Verify evidence erasure and an event containing only IDs and codes. |
| SG04 Action boundary | Spec 0027's gate check admits the spam risk method with its own trial, tuner run and variant, including one whose fitness is its false-caution rate, the live scoring model and a chain fuse step reading the spam risk column as written, job 45's weight training and job 46's feature importance. It admits jobs 2, 3, 4, 33 and 34, the gate, the order key, a probe set chosen by calibrated probability and a lab comparison of chains reading the fused score or calibrated probability. The gate check refuses a candidate generator, a hard filter and each relevance, order-stage or anchor-quality method, tuner, lab trial and GanitEvolve variant that declares `SpamSignal`, the spam risk column or another value computed from them as an input, objective, guard, fitness, label, case filter, sample weight, split or stopping rule; a tuner whose fitness is the spam risk, or the calibrated probability, of accepted links; a GanitEvolve variant relevance method that uses the stored fused score as a feature; and job 2, 3 or 4, or a calibration fit, that declares the spam risk column directly. The gate check refuses each of them that declares content-hint rows, `content_hint_evidence`, content-kind `hint.raised` or `hint.decided`, `spam.content_hint_*` events or a value computed from them in any of those uses, such as a fitness built from upheld hint counts, and each that so declares our estimate or a search-update chart observation. Spec 0055's catalogue check refuses a kavach detector, ganit job 74, an account-hint job of spec 0051 or a darshak detector trained on content-hint decisions. It also refuses a kavach or darshak detector, or the spam risk method or any of its lab trials, tuner runs and variants, that declares our estimate or a search-update chart observation, and a darshak detector that uses the site's Search Console or GA4 series, in any of those uses. In a fixture chain, the gate check refuses an anchor step that declares the spam risk column. Both checks refuse a relevance method, an order-stage method other than the trust door, and a hard filter that declare kavach's cross-author diagnostic or any other SpamGuard finding outside `SpamSignal`, and admit the spam-view estimate and its GanitEvolve search reading them and the trust door reading kavach's traffic findings, and admit the quality door and the order key reading the trust door's frozen traffic rate. They refuse the trust door reading a finding other than traffic, such as the cross-author diagnostic; an account-hint job of spec 0051, or the hints module, building an account hint from kavach's findings; and a tuner, lab trial or GanitEvolve variant other than the estimate's own search that declares them. The gate check admits GanitEvolve's search over versions of the estimate whose fitness is agreement with the site's measured Search Console and GA4 series, and job 147's policy scoring weighted by ratios from the fused score. Spec 0055's catalogue check refuses a kavach detector, and the spam risk method or any of its lab trials, tuner runs and variants, that uses the site's Search Console or GA4 series as its input, objective, guard, fitness, label, case filter, sample weight, split or stopping rule. The gate check admits the tuner's replay of current and proposed scores in spec 0053. Attempt apply without moderator approval, with an expired proposal, and after a newly failed hard filter. No site write occurs. An approved current proposal applies once. Undo restores the exact change and preserves later edits. |
| SG05 Akismet | On a WordPress fixture with Akismet installed and active, a spam comment produces no kavach check, while a spam post and the links inside it still produce all three families' evidence. With Akismet missing, or installed but switched off, the explanation says comments are unchecked. An absent or stale Akismet answer counts as not active. |
| SG06 Our estimate | A synthetic site with link-concentration spam yields a raised risk band naming the link family, labelled as our estimate. The band changes no relevance score, order or site content. Bot traffic sent to a rival page never raises that page's band, and an honest post copied into twenty threads by other members keeps its band, while the copies carry only the cross-author diagnostic. With no Search Console or GA4 series, or a stale one, the band is unknown, never low. A user without "View content hints" sees no per-page band or reason, a member sees no site-level band, and estimate events hold only IDs, bands and reason codes. |
| SG07 Missing kavach | With no current kavach output for a destination, the spam risk method is unavailable, the proposal waits, and no score of zero is used. A current signal with no finding lets a clean page through without waiting. A pair just below the gate with "no finding" is still refused with SUT-550. |
| SG08 Hint permission | A reviewer without "View content hints" sees in the explanation the spam risk method's score, weight and importance and that a spam caution applied, never its reason codes or families, nor any of kavach's other findings such as duplicate clusters or the cross-author diagnostic. On XenForo, a moderator without "View content hints" sees no content hint, and one granted it sees the hint with both families named. On WordPress, a user without the capability sees the explanation with no hint, and one with it sees the hint in the explanation. Superseding the revision, deleting the content or making it no longer guest-visible closes the hint. The same `spam.content_hint_raised` event delivered twice stores one hint. The event schema has no free-text field, and the hint view shows no anchor or post text. |
| RP01 Trial replay | Repeat the fixed seed and held-out protocol on the same candidate. Get the same group member identities, decisions and reasons. A stale bundle, alias, changed setting or repeated check cannot add a count. |
| RP02 Update bands | Join an update notice to a measured day and a missing day in each metric source. Keep the event date and separate series. Missing input stays unknown and the display gives no causal verdict. |

## Limits and footprint

Use the latency, concurrency and memory budgets already owned by specs
0023, 0024 and 0027. Measure the complete path as well as the fixed-size
pair method. On Dell, benchmark identical pinned inputs and machine
conditions under AGENTS.md section 6 rule 8. Include kavach's detection
and the spam-view estimate at the 50,000-thread tier, cold and warm index
reads, long source and destination bodies, a changed tail, concurrent live
and backfill work, and a restored checkpoint. Record wall time,
instruction counts, peak memory and allocations. Compare with the accepted
baseline, and refuse the change on a regression outside the applicable
gate limits. Measure the 50,000-thread tier and both owner PC sizes in the
release test. Use an explicit bound on the active working set. Queue or
resume work when that bound would be exceeded.

This design adds no requests, bytes or queries to public pages.
It uses the existing signed reads and moderator operations. Changes
to those budgets belong to the existing footprint specifications.

## Sources

- Owner answer Q049, archive `docs/requirements/sources/interview-q049-q057.md`.
- Owner answers of 12 November 2026: the estimate of a search engine's
  spam view belongs here, and WordPress comment spam is left to Akismet
  where it is installed.
- Archive ADR-0063, item 3, and the public update study notes in
  `docs/requirements/sources/harness-statistics-seo-ideas-2026-11-06.md`.
- Public descriptions of ranking and spam provide research context.
  [Google Search ranking systems](https://developers.google.com/search/docs/appearance/ranking-systems-guide)
- Public examples describe abuse patterns for synthetic cases.
  [Google Search spam policies](https://developers.google.com/search/docs/essentials/spam-policies)
- The original published account supports the existing comparative
  learning-to-rank research in spec 0027.
  [Burges, From RankNet to LambdaRank to LambdaMART](https://www.microsoft.com/en-us/research/publication/from-ranknet-to-lambdarank-to-lambdamart-an-overview/)
