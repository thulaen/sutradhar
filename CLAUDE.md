# Continue work on Sutradhar

Read [AGENTS.md](AGENTS.md) first. Its rules apply to Claude in every
environment. Speak to the owner in plain English and explain unfamiliar terms.

For the latest saved work, read the
[Claude continuation record](docs/dev/claude-continuation-2026-10-30c.md),
then the
[30 October requirements update](docs/requirements/updates-2026-10-30/README.md)
and the [completion ledger](docs/dev/completion-ledger.md). The earlier
[30 October continuation handoff](docs/dev/claude-handoff-2026-10-30.md)
remains history; its draft pull request step is superseded.

The owner replaced the failed project drive with a dedicated protected directory
on Dell's existing data SSD.
Read [storage preservation](docs/dev/storage-preservation.md) and the
[incident record](docs/dev/storage-recovery.md). Preserve locked foundation
versions; do not delete or weaken their protection without the owner's explicit
instruction. Runtime services and the private ledger still need migration and
verification before normal execution resumes.

Then read the [handoff](docs/dev/handoff.md), which records the current
checkpoint, the remaining setup work and the order for continuing safely.
Preserve the full scope in
[current decisions](docs/requirements/current-decisions.md).

The repository contains a documentation foundation and a research catalogue.
It does not yet contain an installable product or qualified enforcement tooling.
The [three implementation specifications](docs/specifications/README.md) are
explicitly approved for their bounded implementation. Read the exact
[approval record](docs/specifications/approvals.md); do not ask again for those
unchanged designs or infer approval of other features.

Inspect current Git changes and the private task ledger before writing. Work
requires the authorised Dell host. All work happens in a Claude session on
Dell, in the Dell working copy; nothing is edited on or copied from the
Windows computer, which only runs the app (R119). If a step fails
unexpectedly, tell the owner first, before doing anything else.
Publication follows
[ADR-0005](docs/adr/0005-local-publication.md): commits are made on Dell and,
once the route is built, reach GitHub only through the gated Dell repository,
which pushes straight on to GitHub. Cloud access
to GitHub does not give Claude access to uncommitted local files or private
Dell services.
