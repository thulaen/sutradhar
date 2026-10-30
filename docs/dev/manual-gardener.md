# Weekly manual gardener

Status: task instructions; no scheduled job is installed yet.
Audience: a documentation agent working on an authorised project task.

Use this task after the manual toolchain and protected GitHub identity have been
approved and qualified. Follow [AGENTS.md](../../AGENTS.md) and the
[repository setup](repository-setup.md) rules throughout.

## Task

Update the user manual to match the product changes since the last successful
gardening checkpoint. Use plain English and British spelling. Work only while
the authorised Dell host is available. Preserve other agents' work and reserve
the manual files you will edit.

1. Find the last merged `manual: weekly update` checkpoint and its recorded Git
   revision. If none exists, use the last approved manual baseline and state
   that this is the first gardening run.
2. Read the changes to settings, commands, error codes, modules, review pages
   and dashboard behaviours. Confirm the actual behaviour of the target
   revision; do not turn a plan into a claim that a feature exists.
3. Update affected pages under `manual/`. Retain each page's audience and
   outcome. Keep framework internals out of the manual. Link related
   explanations instead of copying them.
4. Regenerate references and synthetic screenshots with the approved tools on an
   allowed host. Do not hand-edit generated pages or use real forum screenshots.
5. Set each reviewed page's `reviewed` value to the product revision actually
   inspected. Keep the build warning for pages not reviewed in 60 days; do not
   change dates to silence it without reviewing the page.
6. Run the full applicable checks before creating the agent commit, including
   the documentation, accessibility, privacy and changed-interface checks.
   Record honest results for the exact content. A missing check is not a pass.
7. Open a pull request titled `manual: weekly update`, using the separate App
   identity. Enable auto-merge only when the change is ordinary documentation
   within approved rules. Request owner approval for any protected decision.
8. Inspect the built artifact and preview, fix failures, and record the merged
   revision as the next gardening checkpoint only after a successful merge.

Report affected pages, evidence, gaps in the product, unresolved blockers and
the pull request link. Never publish identifying forum details in the report.
