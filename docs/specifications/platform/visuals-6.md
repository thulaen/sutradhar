# SPEC-012 part 6: items 46 to 58

Status: APPROVED by the owner on 30 October 2026, with the answers
recorded in the
[specification decisions](../../requirements/sources/specification-decisions-2026-10-30.md).
Part of [SPEC-012](visuals.md). The item format is explained in
[part 3](visuals-3.md).

## Operations, continued

### 46. Retry wave

- Operations page and as an overlay on item 1; D3. Data: dead-letter
  replay events.
- Shows: when staff replay dead letters, items flow back into the
  pipeline at the real replay rate, with successes and repeat failures
  counted.
- Still: counters only. Text: "replaying 40 items: 32 succeeded, 2
  failed again".
- AT-VL146: Each dot is a replay event; counts equal the outcomes;
  without a replay nothing moves.

### 47. Audit timeline

- Operations page; a scrolling HTML list. Data: the audit log with
  diffs (F022, R052).
- Shows: every change with who (system or the moderator), when and a
  diff preview. When private details were erased, the minimal event
  stays and says "details erased".
- Still and Text: the list is text.
- AT-VL147: Entries equal the audit log; an erased record shows its
  minimal event and "details erased", with no text.

### 48. Promotion subway map

- Operations page; D3. Data: module state history (F002).
- Shows: one line per module through shadow, review and auto, with the
  current station lit. As the owner decided (D-VL06), the auto station
  is drawn locked and labelled "not available in version 1" (R009).
- Still: static. Text: each module's state and the date of each change.
- AT-VL148: Stations and dates equal the history; no module can be
  shown at auto; the text says "not available in version 1".

### 49. Setup systems check

- The install wizard; HTML. Data: installer preflight results (F021,
  R049), reported as steps through SPEC-011, using its PHP reporter
  while the service is not yet running.
- Shows: each check lights up when it really completes, passed or
  failed, in the order run; a failed check shows its reason and manual
  link. The checks include SPEC-011's forwarded address and HTTP/2,
  item 3's server-based job runner, and the default docker compose
  project's containers, with room for the service's cap and the
  monitoring services' declared allowance (R129, R121).
- Still: status words change without a light effect. Text: a checklist
  with status words.
- AT-VL149: Each light matches a recorded result; no check shows passed
  before its result exists; a failing fixture check shows its reason.

### 50. Doctor circuit board

- Operations page; D3. Data: `sutra doctor` results (F022) and the
  component map from the registry.
- Shows: components wired together; a failing check breaks the wire at
  the component that failed.
- Still: static. Text: checks with component, status, explanation and
  error code.
- AT-VL150: The break sits at the failing check's component; when all
  pass the circuit is unbroken; the text equals the doctor report.

### 51. Command palette (first ten)

- Sutradhar's own admin and review pages (P-VL03); HTML. Data: search
  over threads (titles of guest-visible threads in the index),
  suggestions, jobs and settings, through the action layer (R044) with
  the person's permissions.
- Shows: Ctrl+K (Cmd+K on Apple computers) opens it; typing searches;
  arrow keys and Enter choose; results within 200 ms at p95 on the
  50,000-thread tier; only what the person may see.
- Still: opens without animation. Text: an accessible combobox and list
  with the result count announced.
- AT-VL151: Opening, searching and choosing work by keyboard alone; a
  moderator without settings permission sees no settings; p95 measured
  on Dell is within budget.

### 52. Mission-control console

- Docked at the bottom of Sutradhar's own admin pages (P-VL03); a
  scrolling HTML list. Data: structured logs with trace IDs from the
  service, scrubbed of post text.
- Shows: folded by default; opened, it shows live logs coloured and
  worded by level, with filters for level, module and trace, and a
  grouped mode using the feed rule. Every entry carries its trace ID
  (R127), which opens the trace in item 42, or says "not sampled" or
  "no longer kept". It never hides the focused element (WCAG 2.4.11).
- Still: entries appear without sliding. Text: the log is text.
- AT-VL152: Entries equal the log records; while folded no log data is
  fetched; moving focus under the dock scrolls the element into view.

## Ambient and notifications

### 53. Favicon badge

- Sutradhar's own admin pages; a small drawn icon (P-VL04). Data: the
  pending suggestion count and the readiness status.
- Shows: the count and a status dot in the browser tab's icon; the page
  title carries the same as words, such as "(12) Review".
- Still and Text: the page title.
- AT-VL153: Badge and title equal the pending count; a readiness change
  changes the dot and the title.

### 54. Rich push notifications

- Outgoing, by ntfy (a push-message service) or email; off by default.
  Content follows D-VL04, totals only with no thread titles, and
  sending follows P-VL10. Data: job completion events (SPEC-011) with a
  chart image drawn from aggregates.
- Shows: a message such as "nightly done: 1,204 new suggestions,
  coverage 61% → 63%" with the chart image and a link to the admin
  page, which needs signing in.
- Still: not applicable. Text: the message is text; in email the image
  has a text description.
- AT-VL154: Messages contain only allow-listed fields and no thread
  titles, even with hostile fixture titles; with the default setting
  nothing is sent; a
  failed send is retried and listed on the health page.

### 55. Ops-wall mode

- A full-screen wall view; uses items 1, 3 and 58. Data: headline
  figures, the heartbeat, the pipeline flow and the grouped feed.
- Shows: large figures that keep updating, read-only, with no member
  text, because a wall screen is seen by people who are not signed in.
  It opens with its own read-only token, which an administrator creates
  for one screen and can revoke at any time, rather than a staff
  sign-in left open (D-VL08).
- Still: the reduced-motion forms of items 1 and 3. Text: the figures
  are text.
- AT-VL155: The wall shows no titles, post text or member names even
  when they exist; figures equal their sources; a revoked wall token
  stops updates within one minute; the wall token opens nothing else.

## Forum-facing and trust

### 56. Transparency chips

- Public thread pages; HTML and CSS, with a D3 mini-graph loaded on
  click. Off by default; content follows D-VL02. Before it can be
  switched on, a Claude agent that did not build it reviews it against
  a written privacy checklist and decides; no separate owner question
  is asked (D-VL03, recorder's interpretation). Data: link provenance
  (suggested and approved) and per-thread counts and lists of
  guest-visible connections.
- Prepared data (P-VL11): the service sends each thread's figures to
  PHP, which stores them in the add-on's own XenForo table. Public
  pages read only that table, with one indexed read, adding at most
  5 ms at p95 to the page's generation time; they never call the
  service. The table is a rebuildable cache, deleted at uninstall
  without export (part 2).
- Shows: a subtle marker beside links Sutradhar inserted ("suggested by
  Sutradhar, approved by a moderator"), drawn when the page is shown
  and never stored in the post, so the post's words are untouched
  (R010); and a "connected to 12 discussions" chip with a mini-graph
  pop-up listing the connected threads as ordinary links.
- Only guest-visible connections count (R013); a connection whose
  target stops being public leaves the count and pop-up within 60
  seconds of the change reaching the service.
- No script loads until the chip is used; the pop-up's code is at most
  15 KB.
- Still: the pop-up opens without animation. Text: the marker and chip
  have text labels; the pop-up is a list.
- AT-VL156: With chips on, the stored post text is unchanged; counts
  include only guest-visible connections and drop within 60 seconds of
  a target becoming private; a public page makes no call to the
  service and stays within its 5 ms budget; a guest sees nothing beyond
  the allowed content; with chips off, the page has no chip markup.

## Added by the owner's follow-up

These two items follow the recorder's reading that the owner's "plus an
error log that is deduped, plus a live feed that is deduped" adds two
visuals, which the owner confirmed: two items plus a rule (D-VL01). The
owner also asked for OpenTelemetry "for traces and showing error logs
on the gui" (R127), which item 57 does.

### 57. Deduplicated error log

- Operations page, errors section; HTML table. Data: errors from the
  service and from the add-on, received through the bundled
  OpenTelemetry collector (spans marked as errors with their exception
  events, and error log records carrying trace and span IDs) and stored
  in Sutradhar's own database (R127, ADR-0006). PHP reports only the
  add-on's own errors, with their error codes; XenForo's general error
  log is not read (R043).
- Shows: one row per fingerprint (part 1): code, plain explanation,
  count, first and last seen, affected jobs, latest trace ID and the
  manual link (R051). A repeat raises the count in place. The trace ID
  opens item 42; tail sampling keeps every error trace, so the link
  works for 7 days, after which it says "trace no longer kept" while
  the row stays for `errors.keep_days`.
- Still: counts change without animation. Text: it is a table.
- AT-VL157: 1,000 fixture errors with three fingerprints give three rows
  with correct counts and times; no row holds post text or a secret; a
  new fingerprint adds a row; each row's latest trace opens in item 42,
  and on a fake clock eight days later says "trace no longer kept".

### 58. Deduplicated live feed

- Live page (the grouped view of item 2's stream, D-VL01), the ops wall
  and the console's grouped mode; HTML. Data: the live event feed.
- Shows: repeats within the 60-second window as one row with a count
  and the latest time, for example "post saved, no suggestions ×14 in
  the last minute"; expanding lists the entries still held, each with
  its trace ID opening item 42 when that trace was sampled (R127).
- Still: rows update without sliding. Text: it is a list.
- AT-VL158: Fourteen identical fixture events within 60 seconds give one
  row marked ×14; an event outside the window starts a new row;
  expanding lists the fourteen.
