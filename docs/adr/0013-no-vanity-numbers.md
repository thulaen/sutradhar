# ADR-0013: No vanity numbers

Decided by the owner on 6 November 2026, carried into this repository
on 9 November 2026. The owner's words: "ban vanity metrics and create a
hard blocking gate for it upon commit, push and release". The numbers
must come from Search Console, GA4 and our own clean-room analytics.

## Decision

A vanity number looks good but changes no decision. Such numbers are
banned. A number may be reported only when
[`gate/metrics.toml`](../../gate/metrics.toml) lists it with:

- **one approved source**, such as Search Console, Bing, GA4, darshak
  or our own analytics. Sources are never added together;
- **a real decision** it helps someone make, of at least four words;
- **what it is compared with**, such as the same pages before an update.

The check `metrics.no-vanity` runs at commit and push:

- A listed number missing any of these fails.
- A known vanity name in code, settings or scripts fails, with its file
  and line, in any spelling: `total_visits`, `totalVisits`,
  `total-visits`, or as part of a longer name. The names live in the
  list, so they can change.
- Only test files may name them, so the check itself can be tested.
  No note in a file switches the check off.
- A missing or broken list blocks. It never passes.

## What this cannot promise

- **A machine cannot judge meaning.** It sees that a decision is
  written, not that it is honest. The reviews judge that.
- **Names can dodge it.** A vanity number under a new name is not
  caught. When screens show numbers, a later check must confirm each
  shown number is on the list.
- **Raw counts inside sums.** Our analytics needs raw counts to work
  out rates. Until the screens check exists, such a count needs a name
  that is not on the vanity list, and the reviews judge it.

## Example

An agent adds `total_visits` to a report. The check fails and names
the line. The agent uses `clicks_change_after_update` instead, which
says where it comes from, what to do about it and what it is compared
with.
