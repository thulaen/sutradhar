# 0041: Editorial Coach

The Editorial Coach answers one question for an editor: what is the
smallest edit worth testing on this page? The editor opens an exact
passage, sees the evidence, compares the wording and can keep the
original. It is advice only. It is roadmap Phase 6 item 1. Its later
additions are in [0042](0042-coach-additions.md).

## What it does

### One thin layer

The Coach reads and presents what Sutradhar already holds. It adds no
second drafting route, overlap check, connector or test system. Feature
numbers below are those of the archive's feature register.

| Coach task | Reuses |
| --- | --- |
| Write or update | Content overlap check ([0052](0052-hubs-summaries-drafts.md)); features 16, 36, 91 |
| Where is the answer | Features 7, 12, 17, 27, 246 |
| What needs a refresh | Feature 252; content draft checks |
| Write from our expertise | Content drafting route and evidence pack; feature 245 |
| What to preserve | Content draft checks; draft diff |
| Did the change help | Features 20, 21 |
| Search and analytics facts | Connectors ([0040](0040-search-connectors.md)); feature 483; darshak |

New here: the edit card, the keep-the-original action, the rules below
and the five tools.

### The edit card

1. Each tip shows the exact page version, the passage, the proposed
   wording, live evidence, dates, data freshness, sample size, and the
   explanations it kept or ruled out.
2. It shows the deciding reasons, never one score.
3. A forecast the data cannot support is hidden. Example: 60 clicks from
   3,000 views, where similar pages get 3%, is a gap of 30 clicks to
   look at, not a promise of 30 more.
4. "Leave it alone" and "not enough evidence" are normal answers. A draft
   can pass with no suggestions.

### Order of checks

1. Permission, freshness, sample size, and whether the action is safe.
2. Confirm demand and fetch the passage.
3. Rank by size of the gap, how directly the text supports it, the
   author's effort and the risk to pages doing well.
4. Only ranked tips reach the editor.

GA4 may rank Coach tips. This holds for the Coach only: link ranking
stays on darshak rates. GA4 figures are never added to other counts.

### What each source can show

- Search Console: demand for a task on this site. It may hide rows, so
  page totals stay apart from known-query totals. Compare like device
  and position.
- GA4: landing-page results. Clicks and sessions count differently.
  Never claim one query or paragraph caused a sign-up.
- darshak: approved cookie-free totals only. A click count without the
  number of views gives no click rate. Missing values stay unknown.
- The content index: exact passages and versions. A passage is fetched
  before it is cited. A word match alone does not prove a paragraph
  confused readers.
- A traffic fall alone never proves a statement false; demand is checked
  first. A date change alone is not freshness.

### What the Coach may change

- Staff guides: a proposed revision delivered to WordPress or XenForo's
  own editor, with a keep-the-original action. Staff publish it. It is
  never applied by itself.
- Titles: a suggestion only, and only where the page text shows a
  mismatch. This is a narrow exception to the SEO rule against title
  proposals ([0044](0044-seo-signals-crawler.md)).
- Member posts: only through the owner's separate member-text option,
  with a moderator approving each change. That option has its own design.
- A later human edit cancels a stale proposal. Undo keeps later edits.
- Findings are advice only. The approved hard checks (such as private
  data or an unsupported quote) stay the only blockers. Automatic checks
  are read-only, and a check still waiting never counts as passed.

### Using a member's answer

The site's users decide, through a bounded setting.

| `coach.member_answer_use` | Effect |
| --- | --- |
| `link_or_short_quote` (default) | A link, or a credited quote up to 50 words |
| `credited_reuse` | Longer reuse, always credited |
| `with_member_consent` | Only with that member's agreement |

Credit and permission always stay visible. One person's report stays
separate from a repeatable result.

### Where wording comes from

Sutradhar computes every number, permission and reference; a model
never supplies them.

1. Writing tips are written by pure-Rust models on the owner's PC
   (candle or tract). They always work, with no outside service.
2. A paid route (AWS Bedrock with the owner's own key) is used only when
   the owner turns it on and sets a spending cap. It is off by default.
3. An AI tool the owner connects through Sutradhar's MCP server may read
   the evidence and draft wording. Every such draft is only a suggestion
   that a moderator must accept, labelled with where it came from.
4. Wording follows the ordered list and shown fallback in
   [0042](0042-coach-additions.md).
5. With no wording route, the Coach still shows evidence and the
   original passage, and invents no wording.

- A staff member's own unpublished draft may go to a route that leaves
  the owner's PC (the paid route or an MCP tool) only with a clear "this
  leaves your computer" notice, shown first every time.
- Other people's drafts, private member text and staff-only text never
  leave the PC. Sending nothing is always possible.
- The content drafting rule that Sutradhar calls no language model is
  loosened for the Coach, in these ways only.

### Paired sites

Sites stay separate, with one exception. For two registered sites with
the same owner, the repeat check may read both, only if the owner turns
it on for that pair. It reports overlap and suggests a link. It never
copies text across and never merges their data. No pair exists until
the owner names one.

### Switching on

Unlike most features, the Coach stays off until it passes a blind test.
Editors compare evidence-backed tips with generic tips on staff pages of
a test site. The test records whether the source supports the edit,
whether it answers the reader's question, how long checking takes, and
how often leaving the text alone was right. Editor acceptance is kept
apart from search results. The owner approves the pass mark before the
test. The blind test also covers the tips added in
[0042](0042-coach-additions.md). After passing, the Coach starts on with
researched settings.

### Tools

The desktop app and the MCP server show the same evidence. Each tool is
limited to the authorised site, user, visibility and version, is checked
against the Cedar rules, and wraps an existing feature.

| Tool | Returns | Wraps |
| --- | --- | --- |
| `find_editorial_opportunities` | Shortlist, and items held back | 16, 12, 17 |
| `get_editor_hints` | Passages and tips for one version | 7, 27, 246 |
| `prepare_writing_brief` | Overlap, home, outline, gaps | Content checks, 36, 91 |
| `check_draft` | Read-only warnings and a preview | Content checks, 252 |
| `compare_edit_outcomes` | Plan or results, with uncertainty | 20, 21 |

Later additions, keeping the same five tools:

- `check_draft` takes pasted and uploaded input and the new checks.
- `prepare_writing_brief` gains related questions.
- `get_editor_hints` shows "tested" lines from the test room
  ([0039](0039-test-room.md)).
- `compare_edit_outcomes` shows chatbot-check results, read only.

### Left out on purpose

Typing-time checks; comparing colleagues' unsaved drafts; a separate
content workspace; cross-owner reports; rival pages; automatic
publishing; co-editing; branches; AI-text detectors; organisation fact
registers. A site's typed business details
([0047](0047-site-structure-checks.md)) are not a fact register: they
are one record per site, compared with its pages, never treated as the
truth and never used to settle a contradiction.

### Logic and maths

Decision logic is Rust, proved with Kani where it decides. Gap and
uncertainty maths are in ganit. The site add-ons make no outgoing
requests and do no work per keystroke.

## Limits

- Models and every Coach job run inside the service slice, within the memory
  limits in AGENTS.md section 5.6. The Coach screens run in the desktop
  window's cap.
- The paid route is never called without a key and a spending cap.
- Member quotes are capped at 50 words by default.
- Sample-size floors and the spending cap are set by research before the
  build.

## Footprint budget

This part adds nothing to public pages. Proposed revisions reach the
site's own editor for staff and change nothing until staff publish.

## How it is tested

Rust tests for each point:

- EC01: every tip has a page version, a source and a date, or it is not
  shown.
- EC02: not enough data gives "not enough evidence", with no forecast.
- EC03: keep-the-original leaves the page unchanged.
- EC04: nothing is applied or published without a person.
- EC05: a human edit after a preview cancels that proposal.
- EC06: another person's draft, or private text, never reaches a route
  that leaves the PC.
- EC07: sending a draft shows the notice first, every time.
- EC08: a paid route with no key or cap is never called.
- EC09: with no wording route, evidence still shows and no wording is
  invented.
- EC10: a denied page, another owner's site, missing GA4 and suppressed
  counts each give a clear, correct result.
- EC11: a plausible claim with no source is flagged.
- EC12: the paired-site check is off by default and refuses different
  owners.
- EC13: the Coach stays off until the recorded blind test passes.
- EC14: a Coach warning never blocks a draft; a hard check still does.
- An MCP draft is stored only as a suggestion and is never applied
  without a moderator's acceptance.

## Changes from the source

- Writing tips come from pure-Rust models; the owner's MCP tool may
  draft wording only as a suggestion a moderator accepts (AGENTS.md 5.5).
- The paid route is AWS Bedrock with the owner's own key.
- Outside local model runtimes are replaced by pure-Rust models.
- Decision logic and adapters are rewritten as Rust, proved with Kani;
  maths uses ganit.
- The admin app becomes the Dioxus desktop app built from SDS.

Source: `docs/specifications/signals/editorial-coach.md` in the archive
`archive-2026-11-08-old-repo`.
