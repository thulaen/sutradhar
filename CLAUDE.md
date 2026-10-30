# Continue work on Sutradhar

Read [AGENTS.md](AGENTS.md) first. Its rules apply to Claude in every
environment. Speak to the owner in plain English and explain unfamiliar terms.

The owner replaced the failed project drive with a dedicated protected directory
on Dell's existing data SSD. Read [storage
preservation](docs/dev/storage-preservation.md) and the [incident
record](docs/dev/storage-recovery.md). Preserve locked foundation versions; do
not delete or weaken their protection without the owner's explicit instruction.
Runtime services and the private ledger still need migration and verification
before normal execution resumes.

Then read the [handoff](docs/dev/handoff.md), which records the current
checkpoint, the remaining setup work and the order for continuing safely.
Preserve the full scope in [current
decisions](docs/requirements/current-decisions.md).

The repository contains a documentation foundation and a research catalogue.
It does not yet contain an installable product or qualified enforcement tooling.
The [three implementation specifications](docs/specifications/README.md) are
explicitly approved for their bounded implementation. Read the exact
[approval record](docs/specifications/approvals.md); do not ask again for those
unchanged designs or infer approval of other features.

Inspect current Git changes and the private task ledger before writing. Work
requires the authorised Dell host to be available. The editing computer is for
source editing and Git synchronisation only. Cloud access to GitHub does not
give Claude access to uncommitted local files or private Dell services.
