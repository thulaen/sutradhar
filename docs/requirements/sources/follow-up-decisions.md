# Follow-up decisions outside the numbered answers

Status: historical source, subject to
[current decisions](../current-decisions.md) and [overrides](../overrides.md).

## Python arrangement after Q031

The initial Q031 answer asked for an explanation instead of selecting a runtime.
The following direct user message then selected option 1:

> option 1 then, now continue with the questions

The explained option was titled **Option 1: PHP plus a Python helper on your
server**. It described PHP handling XenForo integration and approved writes,
with a packaged background Python helper for heavier analysis. This follow-up
settles the Q031 selection; it is not a fourth runtime option or a new question.

Later decisions still control the details: Q034 removes a control-panel
dependency, Q038 installs approved packs, Q094 establishes the PHP-supplied-data
boundary, Q174 changes memory accounting, and Q207 confirms point-and-click
setup.

## Interview continuation

The later direct message was:

> let's finish the questions

It continued the interview. It did not itself change product requirements or
approve unwritten feature designs. The numbered interview then ended at Q209.

## Foundation specification approval on 30 October 2026

After the three concrete designs were written, reviewed and presented, the owner
answered their bounded implementation approval question:

> Approve all three foundation designs

This authorises SPEC-001, SPEC-002 and SPEC-003 at the exact revisions in the
[approval record](../../specifications/approvals.md). It does not change the
numbered interview count or authorise a product release/private-data
transmission.

## Storage replacement and preservation on 30 October 2026

After an unexpected disconnect, the owner instructed the coordinator to use the
existing data SSD, not the operating-system disk, and create a dedicated folder
named `sutradhar`. The owner required protection against deletion unless they
explicitly instruct it and asked that a specifically identified related Codex
session be told to leave the directory untouched. This is a privacy-scrubbed
summary of that direct instruction; exact hardware, private paths and the target
session identifier remain outside public source.

This replaces the failed drive as the active project location. It does not
authorise formatting either disk or deleting existing project material. See
[storage preservation](../../dev/storage-preservation.md).
