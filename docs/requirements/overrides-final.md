# Changes to earlier decisions: Q151–Q209

Part of [changes to earlier decisions](overrides.md); read its status and
introduction first.

## Final decisions Q151–Q209

- Q151
  - Current effect: Only wrap existing words; no rewriting in the linking flow.
  - Earlier statement affected: Q028 and rewrite suggestions in Q093/Q148.
- Q152
  - Current effect: Apply settings between jobs; jobs retain immutable
    versions.
  - Earlier statement affected: Restart-only configuration examples in Q148.
- Q153
  - Current effect: Removed connections stay suppressed until staff reset.
  - Earlier statement affected: Cooldown wording in Q148.
- Q154
  - Current effect: Automatic lab promotion is limited to settings within
    approved bounds.
  - Earlier statement affected: Broad model/code promotion in Q145.
    Contradictory UI paragraphs are settled by Q157.
- Q155
  - Current effect: Use the synthetic/development/nightly data tiers.
  - Earlier statement affected: Synthetic-only Q142. Cloud judging is later
    amended by Q173.
- Q156
  - Current effect: Release quality judging needs multiple providers.
  - Earlier statement affected: Any proposed one-provider judge arrangement.
- Q157
  - Current effect: Moderator queue only; no author-editor suggestions or
    automatic insertion in v1.
  - Earlier statement affected: Contradictory flow descriptions in Q154 and
    earlier automatic-mode examples.
- Q158
  - Current effect: Full applicable checks before commit; fast recheck and
    unchanged-evidence verification at push.
  - Earlier statement affected: A fast-suite-only interpretation of Q155.
    Confirmed by Q202 with Q204's bot exception.
- Q159
  - Current effect: Full evaluation text initially stays on the validation
    host.
  - Earlier statement affected: Earlier cloud-judge assumptions; Q173 later
    creates a narrow redacted-pack exception.
- Q160
  - Current effect: The lab has an 8 GB combined budget.
  - Earlier statement affected: Applying the production 2 GB ceiling to all lab
    services.
- Q161
  - Current effect: Long evaluations checkpoint and resume in later allowed
    windows.
  - Earlier statement affected: Treating an unfinished run as passed or
    silently restarting it.
- Q162
  - Current effect: At most one minute of unfinished work is repeated after
    a qualified failure.
  - Earlier statement affected: Unbounded lost work on recovery.
- Q163
  - Current effect: Approved non-executable formats are allowed, including
    reviewed alternatives to safetensors.
  - Earlier statement affected: A safetensors-only reading of Q148.
- Q164
  - Current effect: Use XenForo UI foundations and approved chart libraries.
  - Earlier statement affected: Replacing the forum's frontend foundation by
    inference.
- Q165
  - Current effect: Resource features require a compatible Resource Manager
    installation.
  - Earlier statement affected: Assuming every XenForo installation contains
    resources.
- Q166
  - Current effect: Admit reviewed permissive terms, including model/data
    terms separately.
  - Earlier statement affected: Assuming named public packages are cleared by
    availability.
- Q167
  - Current effect: A continuous tuning step changes at most 5% of the
    current weight.
  - Earlier statement affected: Unspecified microtuning size.
- Q168
  - Current effect: Compare tuning in the background before activation.
  - Earlier statement affected: Live queue experiments by default.
- Q169
  - Current effect: No author opt-out from moderator-approved linking.
  - Earlier statement affected: The opt-out scenario in Q148. Analytics
    consent remains separate.
- Q170
  - Current effect: Accepted guest votes have equal influence after checks.
  - Earlier statement affected: Implicitly discounting all guest feedback.
- Q171
  - Current effect: Staff choose an authorised publishing account in the GUI.
  - Earlier statement affected: An assumed fixed author for generated drafts.
- Q172
  - Current effect: A restricted staff review page serves moderators without
    ACP access.
  - Earlier statement affected: Queue placement below posts in Q102.
- Q173
  - Current effect: Restricted extension workers; bounded redacted judging
    packs may go to the two named providers.
  - Earlier statement affected: Q159's local-only judging rule. Full corpus
    and credentials remain private.
- Q174
  - Current effect: PHP has an incremental 8 MB request budget; the service
    has a combined 2 GB production cap.
  - Earlier statement affected: Q003's original whole-add-on memory wording.
    Shared services must be accounted for separately.
- Q175
  - Current effect: The lower end of the 95% confidence interval must reach
    95%.
  - Earlier statement affected: The observed 475-of-500 rule in Q145.
- Q176
  - Current effect: Three-round consensus, invalid-output repair and
    reliability checks; unresolved cases count as failures.
  - Earlier statement affected: Agreement-only filtering. Also accepts
    proactive memory refusal plus crash recovery, no-tools/read-only judging
    and verified account controls.
- Q177
  - Current effect: Include the actual linked reply in a reply-target
    judging pack.
  - Earlier statement affected: Opening-post-only target excerpts.
- Q178
  - Current effect: Later moderator decisions affect future learning, not
    past gate scores.
  - Earlier statement affected: Retroactively changing a completed release
    assessment.
- Q179
  - Current effect: First-round agreement below 95% blocks release.
  - Earlier statement affected: Merely reporting ambiguous-rubric disagreement
    without blocking.
- Q180
  - Current effect: Recheck a bounded cache sample for judge drift.
  - Earlier statement affected: Blind permanent reuse of unchanged verdicts;
    Q183 determines when judges run.
- Q181
  - Current effect: Approve an outside candidate before its code executes on
    the validation host.
  - Earlier statement affected: Running untrusted public submissions with
    private-host access.
- Q182
  - Current effect: Pending suggestions expire after 30 days, or earlier on
    invalidation.
  - Earlier statement affected: Unlimited queue lifetime.
- Q183
  - Current effect: Rejected links return only after meaningful content
    changes; full judging is release-only.
  - Earlier statement affected: Nightly judge calls in Q155/Q180. Non-judge
    nightly checks remain; optional merge warnings are budgeted.
- Q184
  - Current effect: WCAG 2.2 AA is a release requirement for owned UI.
  - Earlier statement affected: An unspecified accessibility target.
- Q185
  - Current effect: Publish the actual tested mainstream browser environments.
  - Earlier statement affected: Claiming browser support without matching
    evidence.
- Q186
  - Current effect: Run a supervised 72-hour soak independently of the
    editing computer.
  - Earlier statement affected: A chat/session-bound soak.
- Q187
  - Current effect: Publish automatically after all release conditions pass.
  - Earlier statement affected: A presumed final manual publication step;
    later governance is settled in Q196/Q199.
- Q188
  - Current effect: Rebuild private evaluation data from an authorised
    export after disk loss and requalify it.
  - Earlier statement affected: Assuming additional backups on unapproved
    machines or disks.
- Q189
  - Current effect: Record safe handover in a private rolling SQLite run
    ledger.
  - Earlier statement affected: Implicit ownership transfer when an agent
    disappears.
- Q190
  - Current effect: A tightly limited documentation-only bootstrap may
    mark code-only checks not applicable with proof.
  - Earlier statement affected: Fake test passes for software that does not
    exist.
- Q191
  - Current effect: The first internal implementation milestone is the
    method research catalogue.
  - Earlier statement affected: A smaller end-to-end linking product as the
    first milestone.
- Q192
  - Current effect: One home per explanation; add the requested toolchain
    and separate public manual.
  - Earlier statement affected: Duplicated method prose. Auto-mode manual text
    is subject to Q157; installation text is subject to Q207.
- Q193
  - Current effect: Agents may commit/push proposal branches; use protected
    PR workflows, bots and manual publication.
  - Earlier statement affected: Dell-only commit/push rule in Q063 and
    local-Claude restriction in Q070. Its internal merge-approval conflict is
    resolved by Q196.
- Q194
  - Current effect: Candidate tag starts checks; a final tag/publication
    follows success on the same qualified code.
  - Earlier statement affected: A final release tag/publication before
    judging.
- Q195
  - Current effect: Preserve a frozen manual for every full version, including
    patch versions.
  - Earlier statement affected: Reusing only the X.Y manual directory for
    every patch.
- Q196
  - Current effect: Trusted ordinary PR merging and releases are unattended
    after checks.
  - Earlier statement affected: Q193's owner-click-only paragraph. Q199
    retains protected decisions.
- Q197
  - Current effect: Public-code/synthetic-data cloud checks are allowed;
    never reveal the owner's forum in public materials.
  - Earlier statement affected: All-checks-on-Dell wording in Q045/Q155.
    Private-data restrictions remain.
- Q198
  - Current effect: All project work pauses when the validation host is
    unavailable.
  - Earlier statement affected: Continuing cloud proposals while that host is
    offline.
- Q199
  - Current effect: The owner still approves specifications and changes to
    protected rules.
  - Earlier statement affected: Interpreting unattended operation as
    permission to rewrite governance.
- Q200
  - Current effect: Reviewed anonymous aggregate statistics may be public.
  - Earlier statement affected: A public-results policy restricted to
    synthetic numbers/pass-fail only.
- Q201
  - Current effect: Automation uses a separate GitHub App identity.
  - Earlier statement affected: Sharing the owner's approval identity with
    agents.
- Q202
  - Current effect: Keep full applicable checks before ordinary agent commits.
  - Earlier statement affected: Fast-only branch commit checks.
- Q203
  - Current effect: Pause and report publication deadlines during
    required-service outages.
  - Earlier statement affected: An unconditional five-minute promise while all
    work must be paused.
- Q204
  - Current effect: Recognised Dependabot/Release Please proposal commits
    alone may precede checks.
  - Earlier statement affected: Applying ordinary-agent pre-commit timing to
    standard proposal bots. Merge checks remain mandatory.
- Q205
  - Current effect: Judge at least 500 fixed and at least 500 fresh
    suggestions separately.
  - Earlier statement affected: Treating 500 as the total across both sets.
- Q206
  - Current effect: Count initial invalid judge outputs even when repaired;
    repairs are reported separately.
  - Earlier statement affected: Measuring reliability only after successful
    repairs.
- Q207
  - Current effect: Point-and-click installation is the primary path;
    commands are optional advanced reference.
  - Earlier statement affected: Q192's copy-and-paste-command installation
    path.
- Q208
  - Current effect: Use the named providers' reviewed service locations; do
    not promise a fixed region.
  - Earlier statement affected: The region-restricted judge wording in Q148.
    Terms, redaction and training controls still require verification.
- Q209
  - Current effect: Keep minimal permanent events separately from erasable
    identifying information.
  - Earlier statement affected: An audit design that cannot honour erasure
    without rewriting event history.
