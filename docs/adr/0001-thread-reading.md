# ADR-0001: Read every post; summarise a thread from a selected subset

Status: accepted by the owner, 30 October 2026. Requirements: R101, R102.
The owner's exact words are in the
[model decisions source](../requirements/sources/model-decisions-2026-10-30.md).

## Context

The owner asked for threads to be read in chunks but in full, so that there
are no truncations or short summaries. Some threads hold thousands of
posts, and matching needs every post's own content.

## Decision

There is no cap on reading and a cap on summarising.

1. Every eligible (guest-visible, R013) post in every thread is read for
   existing links, language, state and its per-post index entries, with no
   size limit and no staff warning. Reading is incremental: only new and
   edited posts are read again.
2. The thread-level topic representation (the summary vector) uses a
   selected subset of up to 20 posts:
   - the thread title, weighted;
   - the first post in full, up to the model's limit;
   - the accepted answer, if any;
   - the 5 highest-reacted replies;
   - the first 5 replies;
   - the 3 most recent posts;
   - after duplicates are removed, the next most-reacted replies until the
     subset reaches 20 posts.
3. Every selected post except the first is cut to its first 400
   characters, excluding quotes, code, spoilers and signatures.
4. The counts (5, 5, 3 and the total of 20) and the 400-character
   length are settings with these defaults.

## Consequences

- Per-post pieces cover every post (R101); a post outside the subset still
  counts through its own pieces.
- Recorder's note: changing a setting means the affected summary vectors
  are rebuilt; the feature specification confirms this.
