# Changes to earlier decisions

Status: reconciliation of the recorded interview. This is not proof that the
replacement behaviour has been implemented.

Read [current decisions](current-decisions.md) for the active rules and the
[source index](sources/README.md) for exact wording. Preserve the historical
passages, but do not implement an earlier alternative after it has been replaced.
When a question adds a long list, only the conflicting portions are superseded;
the remaining list stays in scope.

## Earlier clarifications

| Later decision | Replaces or qualifies | Effect |
| --- | --- | --- |
| Q033 | Q029 patent suggestion | Exclude unresolved patent concerns. Independent coding does not clear a patent. |
| Q038 | Optional-install reading of Q032 | Install all approved packs; use lazy loading and resource qualification. |
| Q052–Q053 | A per-post or destination-count reading of the five-link cap | Count content-link placements across the whole thread/resource; prevent repeated source-to-target thread connections. |
| Q061 | Literal Spring Boot compliance in Q060 | XenForo's requirements win at the integration boundary. |
| Q064–Q066 | Unmapped standards, unexamined equivalent mutants, single noisy timing comparisons | Approve a language-appropriate rulebook; resolve equivalent cases; use controlled repeated measurements. |
| Q069, Q071 | An assumed mandatory independent human reviewer | Honest, evidenced self-review is allowed for engineering reviews; later GitHub AI-review checks are additional. |
| Q079 | Unrestricted detailed tracking in Q044/Q077 | Detailed histories need consent; withdrawal stops collection. |
| Q085 | A count limited to original named algorithms | Distinct reasoning procedures and meaningful documented research variants may qualify. |
| Q090–Q091 | Approving a preview while analysis continues or accepting a stale suggestion | Complete applicable available-evidence analysis first; changed source needs fresh approval. |
| Q094, Q104 | Python reading XenForo tables in the architecture proposal | PHP supplies approved data. Python cannot directly access XenForo's database. |
| Q103, Q105, Q108 | Automatic publication/invitations in earlier module examples | Human approval is required for published changes, invitations and each hub link. |
| Q109 | Threads/resources as the only possible destination types in Q097 | Approved optional hubs may also be destinations. Tags remain excluded. |
| Q115 | Conflicting reaction mappings in Q112 | Sad and Angry are negative even if the platform assigns neutral scores. |
| Q116 | Old Zen G name and older provisional module names | Sutradhar and the approved glossary apply, including kavach for spam protection. |
| Q118 | Paid backlink-provider dependencies in Q115 | Remove paid backlink services; focus external analytics on GSC, Bing and GA4. |
| Q119, Q121 | Title editing and automatic site-SEO remedies | No title changes; other SEO remedies are reports/proposals only. |
| Q125 | A single search/storage backend | Support both Elasticsearch/OpenSearch and optional AWS/GCP storage routes; qualify actual services rather than inventing a hosted offering. |
| Q130 | Indian names alone in the UI | English labels lead, with glossary names alongside. |
| Q140 | Interpreting 6,000 methods as a ban on incremental commits | Development commits are allowed; a complete product release remains blocked until all required scope is qualified. |
| Q145 | Q019's linking-only method scope | The catalogue also covers the specified relevance, spam, quality, statistical and learning work; no duplicate counting. |
| Q146–Q149 | A single favourable aggregate or tuning on final data | Important groups and opportunity coverage matter; fixed and fresh assessments remain distinct. |

## Final decisions Q151–Q209

| Decision | Current effect | Earlier statement affected |
| --- | --- | --- |
| Q151 | Only wrap existing words; no rewriting in the linking flow. | Q028 and rewrite suggestions in Q093/Q148. |
| Q152 | Apply settings between jobs; jobs retain immutable versions. | Restart-only configuration examples in Q148. |
| Q153 | Removed connections stay suppressed until staff reset. | Cooldown wording in Q148. |
| Q154 | Automatic lab promotion is limited to settings within approved bounds. | Broad model/code promotion in Q145. Contradictory UI paragraphs are settled by Q157. |
| Q155 | Use the synthetic/development/nightly data tiers. | Synthetic-only Q142. Cloud judging is later amended by Q173. |
| Q156 | Release quality judging needs multiple providers. | Any proposed one-provider judge arrangement. |
| Q157 | Moderator queue only; no author-editor suggestions or automatic insertion in v1. | Contradictory flow descriptions in Q154 and earlier automatic-mode examples. |
| Q158 | Full applicable checks before commit; fast recheck and unchanged-evidence verification at push. | A fast-suite-only interpretation of Q155. Confirmed by Q202 with Q204's bot exception. |
| Q159 | Full evaluation text initially stays on the validation host. | Earlier cloud-judge assumptions; Q173 later creates a narrow redacted-pack exception. |
| Q160 | The lab has an 8 GB combined budget. | Applying the production 2 GB ceiling to all lab services. |
| Q161 | Long evaluations checkpoint and resume in later allowed windows. | Treating an unfinished run as passed or silently restarting it. |
| Q162 | At most one minute of unfinished work is repeated after a qualified failure. | Unbounded lost work on recovery. |
| Q163 | Approved non-executable formats are allowed, including reviewed alternatives to safetensors. | A safetensors-only reading of Q148. |
| Q164 | Use XenForo UI foundations and approved chart libraries. | Replacing the forum's frontend foundation by inference. |
| Q165 | Resource features require a compatible Resource Manager installation. | Assuming every XenForo installation contains resources. |
| Q166 | Admit reviewed permissive terms, including model/data terms separately. | Assuming named public packages are cleared by availability. |
| Q167 | A continuous tuning step changes at most 5% of the current weight. | Unspecified microtuning size. |
| Q168 | Compare tuning in the background before activation. | Live queue experiments by default. |
| Q169 | No author opt-out from moderator-approved linking. | The opt-out scenario in Q148. Analytics consent remains separate. |
| Q170 | Accepted guest votes have equal influence after checks. | Implicitly discounting all guest feedback. |
| Q171 | Staff choose an authorised publishing account in the GUI. | An assumed fixed author for generated drafts. |
| Q172 | A restricted staff review page serves moderators without ACP access. | Queue placement below posts in Q102. |
| Q173 | Restricted extension workers; bounded redacted judging packs may go to the two named providers. | Q159's local-only judging rule. Full corpus and credentials remain private. |
| Q174 | PHP has an incremental 8 MB request budget; the service has a combined 2 GB production cap. | Q003's original whole-add-on memory wording. Shared services must be accounted for separately. |
| Q175 | The lower end of the 95% confidence interval must reach 95%. | The observed 475-of-500 rule in Q145. |
| Q176 | Three-round consensus, invalid-output repair and reliability checks; unresolved cases count as failures. | Agreement-only filtering. Also accepts proactive memory refusal plus crash recovery, no-tools/read-only judging and verified account controls. |
| Q177 | Include the actual linked reply in a reply-target judging pack. | Opening-post-only target excerpts. |
| Q178 | Later moderator decisions affect future learning, not past gate scores. | Retroactively changing a completed release assessment. |
| Q179 | First-round agreement below 95% blocks release. | Merely reporting ambiguous-rubric disagreement without blocking. |
| Q180 | Recheck a bounded cache sample for judge drift. | Blind permanent reuse of unchanged verdicts; Q183 determines when judges run. |
| Q181 | Approve an outside candidate before its code executes on the validation host. | Running untrusted public submissions with private-host access. |
| Q182 | Pending suggestions expire after 30 days, or earlier on invalidation. | Unlimited queue lifetime. |
| Q183 | Rejected links return only after meaningful content changes; full judging is release-only. | Nightly judge calls in Q155/Q180. Non-judge nightly checks remain; optional merge warnings are budgeted. |
| Q184 | WCAG 2.2 AA is a release requirement for owned UI. | An unspecified accessibility target. |
| Q185 | Publish the actual tested mainstream browser environments. | Claiming browser support without matching evidence. |
| Q186 | Run a supervised 72-hour soak independently of the editing computer. | A chat/session-bound soak. |
| Q187 | Publish automatically after all release conditions pass. | A presumed final manual publication step; later governance is settled in Q196/Q199. |
| Q188 | Rebuild private evaluation data from an authorised export after disk loss and requalify it. | Assuming additional backups on unapproved machines or disks. |
| Q189 | Record safe handover in a private rolling SQLite run ledger. | Implicit ownership transfer when an agent disappears. |
| Q190 | A tightly limited documentation-only bootstrap may mark code-only checks not applicable with proof. | Fake test passes for software that does not exist. |
| Q191 | The first internal implementation milestone is the method research catalogue. | A smaller end-to-end linking product as the first milestone. |
| Q192 | One home per explanation; add the requested toolchain and separate public manual. | Duplicated method prose. Auto-mode manual text is subject to Q157; installation text is subject to Q207. |
| Q193 | Agents may commit/push proposal branches; use protected PR workflows, bots and manual publication. | Dell-only commit/push rule in Q063 and local-Claude restriction in Q070. Its internal merge-approval conflict is resolved by Q196. |
| Q194 | Candidate tag starts checks; a final tag/publication follows success on the same qualified code. | A final release tag/publication before judging. |
| Q195 | Preserve a frozen manual for every full version, including patch versions. | Reusing only the X.Y manual directory for every patch. |
| Q196 | Trusted ordinary PR merging and releases are unattended after checks. | Q193's owner-click-only paragraph. Q199 retains protected decisions. |
| Q197 | Public-code/synthetic-data cloud checks are allowed; never reveal the owner's forum in public materials. | All-checks-on-Dell wording in Q045/Q155. Private-data restrictions remain. |
| Q198 | All project work pauses when the validation host is unavailable. | Continuing cloud proposals while that host is offline. |
| Q199 | The owner still approves specifications and changes to protected rules. | Interpreting unattended operation as permission to rewrite governance. |
| Q200 | Reviewed anonymous aggregate statistics may be public. | A public-results policy restricted to synthetic numbers/pass-fail only. |
| Q201 | Automation uses a separate GitHub App identity. | Sharing the owner's approval identity with agents. |
| Q202 | Keep full applicable checks before ordinary agent commits. | Fast-only branch commit checks. |
| Q203 | Pause and report publication deadlines during required-service outages. | An unconditional five-minute promise while all work must be paused. |
| Q204 | Recognised Dependabot/Release Please proposal commits alone may precede checks. | Applying ordinary-agent pre-commit timing to standard proposal bots. Merge checks remain mandatory. |
| Q205 | Judge at least 500 fixed and at least 500 fresh suggestions separately. | Treating 500 as the total across both sets. |
| Q206 | Count initial invalid judge outputs even when repaired; repairs are reported separately. | Measuring reliability only after successful repairs. |
| Q207 | Point-and-click installation is the primary path; commands are optional advanced reference. | Q192's copy-and-paste-command installation path. |
| Q208 | Use the named providers' reviewed service locations; do not promise a fixed region. | The region-restricted judge wording in Q148. Terms, redaction and training controls still require verification. |
| Q209 | Keep minimal permanent events separately from erasable identifying information. | An audit design that cannot honour erasure without rewriting event history. |

## Lists that remain proposals rather than facts

The owner's direct storage instruction on 30 October 2026 replaces the earlier
dedicated-drive choice: use a dedicated `sutradhar` directory on the existing
data SSD, excluding the operating-system disk. Preserve its contents unless the
owner explicitly instructs their deletion. Notify the separately identified
related Codex session to leave that project directory untouched. Exact hardware,
host paths and session identifiers stay in private operator records.

The package names in Q093 are an admission backlog, not a verified bill of
materials. The SEO claims in Q115 require evidence and capability checks. The
edge cases in Q148 are acceptance requirements, not passing tests. The manual
brief in Q192 is a delivery requirement, not evidence of a deployed site. Old
names inside those sources are historical; the Q116 glossary applies now.
