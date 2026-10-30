# SPEC-010 part 9: files on the forum host

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-010](models.md); moved from part 1 to keep that page
within the file limits. Requirements: R106, R109, R025, R052, R129;
ADR-0002 and ADR-0003.

## The per-site overlay lock

- Place: `<data directory>/models/site-models.lock` on the forum host.
  It is runtime data, not a repository file.
- Same schema as `models.lock` (part 1), plus: `tier` (`community` or
  `site-trained`), `added_at`, `check_id`, `licence_accept_event`, and
  each large file's Hub-published SHA-256 next to Sutradhar's own.
- Written only by the service. Its own SHA-256 is kept in the
  service's store (bhandar); a mismatch means it was edited outside the
  service, and its models are refused (MF-E11).
- It may only add entries. An entry with a Verified entry's `id` or
  `repo` is refused, so the overlay cannot replace a Verified model.
- It holds no personal data: people are referenced only through audit
  event identifiers. Uninstall always deletes it (part 4).

## Active models

For the two roles a community model can fill, the active model is a
setting (`models.active.embedder`, `models.active.reranker`) that
defaults to the Verified entry. For every role, only activation, update
and rollback change the active model or revision, as one atomic
settings version between jobs, with history kept (R025).

## File storage on the forum host

- `<data directory>/models/<entry id>/<revision>/`: verified files,
  read-only. `staging/`: downloads in progress. `trials/`: trial
  results (numbers and ids) and trial vectors.
- The data directory is mounted into the service's container (R129)
  and belongs to the service account the container runs as: folders
  `0700`, files `0400` once verified. PHP and other accounts cannot
  read them. Removing the containers does not remove it (part 4).
- Per role, the active model and one previous model (for rollback) are
  kept. Inactive community models stay until an administrator removes
  them. Disk use is shown on the Models page.
