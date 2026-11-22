# 0142: Local Performance Lab

## What it does

Performance Lab helps the owner find out where time goes on a page,
compare like-for-like captures and decide what to investigate next. It
shows local load measurements, the browser's recorded requests and, when
admitted, sampled images of the load. It keeps those findings separate
from supplied measurements and real-visitor field summaries. It does not
feed ranking, publish, change a site, or decide what a search engine saw.

The [audit suite](0140-unified-site-audit-suite.md) owns the common
entry and [Audit Grid](0143-audit-grid-findings-and-local-preview.md)
owns the shared findings view. This spec owns the Lab run, calibration,
capture, comparison and Lab-specific views. Spec 0044 owns the
registered site and Audit source. Spec 0047 owns template budgets,
paired extension-cost checks and the existing speed notice. Spec 0115
owns the browser cage, broker and render levels. Spec 0116 owns
protected capture custody and offline replay. Spec 0134 owns
the release load gate. The 0082 legal wall applies before each request.
The optional Lab method needs the exact 0115 EX-02 component, platform
cage, rendering method and calibration evidence before it can run.

## Choosing what to measure

1. **Local replay.** Use a synthetic fixture or an eligible, permitted
   and scrubbed recording in 0116 custody. Resolve only against the
   admitted recording. A missing resource never starts a network call.
   Replay timings describe the local replay unless a separate row
   explicitly presents the original recording's observed timing.
2. **One owned public page.** The owner gives an exact current 0020
   grant for one public, guest-readable canonical address on the active
   registered site, named run/profile, purpose, methods and bounds.
   Dispatch uses the existing Audit runner and the admitted broker.
   Each redirect target and resource must pass the current guest rules,
   registered site and exact grant, host scope, robots, resource wall,
   request allowance and pace. Recheck all of those at dispatch. Block
   any undisclosed target or resource and mark affected facts partial.
   No cookie, account, credential, browser session, bypass, staging
   visit or raw payload import follows from that grant.
3. **Supplied measurements.** An authorised site owner imports a bounded
   typed summary or trace for the same site. Protected staging remains
   supplied-only. Others' regional measurements remain supplied-only,
   even when a local profile names a network condition. Record who
   supplied it, known collection method, claimed region, original
   capture date and source precision. Report a claimed region and any
   alternate viewport dimensions as supplied, and show unknown method
   or coverage explicitly. Alternate sizes create no runnable presets.
   Do not fill a gap by reaching the origin or a remote helper. A timing
   import can authorise inspection only. Replay needs separate 0116
   admission. The v1 timing file carries
   neither pixel data nor raw HTTP traffic.
4. **Field data.** Read a compatible authorised supplied own-site summary. A
separate
   0044 CrUX or PageSpeed profile may contribute only already
   admitted records from its exact producer. Spec 0040 data supplies no
   field speed observation by itself.
   Lab selection never starts a provider pull. Retain producer,
   requested and returned URL or origin, form factor, dates, metric,
   percentile and coverage. Keep an origin fallback visibly separate.
   Overlapping rolling windows are not independent samples. Do not
   infer coverage, browser populations or raw visitor data.

The local runs use the admitted 0115 mobile and desktop profiles only.
Mobile is the default and desktop consumes its own run and allowance.
A user-visible profile names the admitted versions, device geometry and
network mechanism. A network label alone cannot prove a throughput or
latency profile. Regional labels never select compute. No cloud helper,
regional endpoint, third-party integration or expanded device catalogue
can be started here.

## Evidence records

All records have typed values and closed enumerations. The service is
the authority for generated identities and evidence references. Time
units are integer microseconds unless stated otherwise. Sizes are bytes.
An interval is half-open. The end of observation and its reason are
recorded separately. Each value has one state: measured, simulated,
supplied, partial, unavailable or not applicable. Store reason,
coverage, and any lower bound separately from the value. Unavailable
and not applicable have no numeric value. Do not store URLs, headers,
body bytes, page text or secret values in trace rows.

| Record | Required identity and fields |
| --- | --- |
| Profile | Profile ID and revision, route, source kind, site, target scope, allowed method set, operating system and architecture, engine and cage digests, clock profile, viewport in CSS pixels, rational pixel ratio, font set digest, CPU schedule, connectivity mechanism, up/down rates and latency when admissible, protocol policy, cache start state, locale/time zone, guest consent state, fixture/manifest or native revision condition, run deadline, stop condition, metric-definition versions, bounds and calibration reference. Supplied-only fields can explicitly be unknown. |
| Calibration | ID, exact profile digest, synthetic fixture and reference digests, seed, host attestation, measurement clock and precision, five raw sample pairs, pair order, metric expected interval, observed interval and decision, dated admissions and invalidation causes. |
| Capture | Stable capture ID, site, target or redacted fixture reference, owning grant and epoch, source kind and source digest, versioned profile and calibration references, plan digest, material revision, clock domain, UTC start or supplied capture range, origin tick, completion/stop reason, limits used, coverage, and immutable evidence digest. A claim or unknown in a supplied file is not a trusted local clock or grant. |
| Clock | Capture-scoped clock ID, source and domain (real monotonic, virtual or supplied), origin event, calibrated tick resolution and maximum error. Only a recorded bounded conversion can align two clock domains. UTC dates are for provenance. |
| Request attempt | Capture ID, unique attempt sequence, allowlisted resource-kind and safe resource reference, parent/redirect chain references, broker decision, method, sent/unsent/replay status, observed protocol and cache disposition, response status class, first and last byte ticks if eligible, transferred byte count or reason, terminal outcome and coverage. Revalidation, cache-only reuse and refused attempts have separate types. |
| Request phase | Attempt ID, named interval (queued, lookup, connect, handshake, request, wait or receive), clock ID, start/end tick or reason, and observed, unavailable, not applicable or coarsened state. Reused connections and coarsened third-party intervals have an explicit reason. Never infer omitted phases by subtraction. |
| Sampled frame | Capture ID, frame sequence, presentation tick and clock, viewport and physical dimensions, admitted privacy state, immutable authorised vault object reference, and sample interval/stop reason. Missing, redacted or dropped frames have no image payload. No inline image or encoded bytes in the record. |
| Visual event | Capture ID, event sequence, algorithm version, source event tick/clock, event kind, safe affected-region reference if admitted, unit or score and coverage. A frame link requires one exact presented-frame identity and clock correlation; otherwise keep the event and sampled imagery separate. Store no selector text, private image or page excerpt. |
| Metric | Capture ID, metric key and version, source kind, value or null, unit, statistic, clock domain and observation bounds, contributing evidence IDs, coverage, precision, reason and comparability profile. |
| Baseline | Site, named comparison ID, prior capture set and digests, exact profile and calibration lineage, metric definition, approved treatment and matching fields, source type and intervals, budget revision if using 0047, lifecycle epoch and expiry. Advancing it is a local owner action retaining prior immutable records. |

A measured known zero is distinct from unavailable. Counts state their
population. Capture and projection use the 0118 encrypted store and
forced site scope. Apply the 0082 scrub before digest, storage, log,
spill or presentation. Pixels follow 0115 N-1669 and RN-11. Verify
authorised scrubbed pixels and safe event markers independently before
any durable pixel write, digest or presentation. If the approved
profile cannot prove this, withhold the image and its protected
regions. Clock values are compared only within the same
admitted domain or a proved conversion. Dates and inherited headers
cannot align timers. Removing a site, revoking rights or expiry closes
all managed reads, derived comparisons, frame access and replay first.
Then erase through 0030 and 0116. Keep only the permitted content-free
audit record. Restore cannot revive a stale grant, expired calibration
or revoked evidence reference.

## Measurement methods

| Method | Meaning and conditions |
| --- | --- |
| First contentful paint | First eligible presented content event after the declared navigation origin under the admitted Paint Timing profile. Test a blank, background-only and eligible first-paint sequence. No milestone is inferred from a frame or completed request alone. |
| Largest visible paint | Ordered eligible largest-content candidates with strictly specified visibility and observation cutoff. Use the latest eligible winner under the admitted algorithm. Keep the 0115 layout-based approximation labelled simulated when there is no admitted observer. |
| Visual progress time | A separately named versioned local visual progress measure. Integrate the fraction of viewport still different from the qualified final image over complete timestamped sample intervals. Use the last admitted sample as final only when the pinned settle rule holds. Compare exact RGB channels after admitted sRGB conversion over the same complete viewport. A pixel differs if any channel differs. Weight each interval by its preceding sample’s fraction of differing pixels. Store the sum in microseconds and show the 250 ms sampling interval beside it. Missing frame, mutable baseline, time gap or incomplete settle makes it unavailable. Use the name Speed Index only after an independent standards comparison proves the selected method. |
| Total blocking time | Initial local load-window measure uses eligible FCP through the end of the qualified 500 ms quiet settle interval, on one real monotonic clock. For each task [s,e), sum the intersection of [s+50 ms,e) with that window, with zero for an empty intersection. Never charge the first 50 ms again after clipping. Mark this local blocking measure unless a separate reference qualification establishes equivalent TBT semantics. Both endpoints and complete eligible tasks are required. No virtual-clock substitution. |
| Layout stability | Keep the 0115 simulated movement total separate. A separately admitted observer may report a CLS result only from eligible events and the versioned session-window rules, with input exclusions and the observed lifecycle. A clipped observation cannot be called complete field CLS. |
| Time to first byte | Initial exact route measures first byte of the final non-1xx top-level response minus navigation start, on the admitted real clock for an uncached direct network response. Provisional 1xx events never set the endpoint. A redirect, unproved proxy timing, cache-only or revalidated response, or service-worker handling is unavailable for this profile. Reused connection retains the same formula when both events are observed, and lookup or handshake may be not applicable. Header completion and final-body time never substitute. |
| Scripted step to paint | For a separately admitted offline fixture and fixed safe step, measure the monotonic interval from admitted input dispatch to the first presented frame proved to contain the expected fixture update. Capture step and frame IDs, clock error, and fixture revision. An out-of-window response is censored. Unrelated paint, missing presentation proof, virtual time or no update gives no value. Show as a local scripted step, with step name and defined scope. Never report it as INP. |
| Field summary | Present only the source's own metric, percentile, device selection, URL or origin and coverage. INP appears solely when supplied by the authorised field source. Never substitute TBT, local scripted results or a load-time approximation for it. |

The scripted step uses the registered synthetic harness or admitted
offline fixture replay. The owner preapproves the exact harmless
read-only step, such as opening a synthetic disclosure. Its immutable
fixture records the expected visual update and step deadline.
The owner cannot supply an executable script or open-ended selector.
No form submission, login, navigation, production action or input to
the live owned page is permitted. The step does not change the existing
0115 safe inspection rules.

## Calibration and bounds

The candidate must earn independent review under QR-003 and EX-02.
An approved component alone grants no metric, phase, replay or paint
method. For each enabled combination of platform, renderer, clock and
profile, prove the complete recorded chain with a sealed synthetic lab
and independent observation of the expected event. Compare five valid
alternating paired runs against the fixture's known plan and an admitted
reference. Publish the seed, immutable inputs, five pairs, observed
error and reviewer receipt. Both platforms need their own result.
A separate reference browser remains subject to QR-049.

Use a monotonic clock with measured resolution at most 1 ms and
synthetic timestamp error at most 5 ms. For opt-in frame sampling, at
least one image is due each 250 ms of the recognition window. A missing
interval ends any complete visual calculation. Proof of frame-to-event
attribution needs engine presentation identities and a measured clock
bound of 5 ms. When network shaping is proposed, prove down and up
rates within 10% and controlled round-trip latency within the greater
of 5 ms or 10%, measured on isolated synthetic traffic. Report applied
rather than nominal values. The existing 0047 mobile CPU profile keeps
its own five-pair ratio test. Lack of reference or failed calibration
keeps the affected metric or profile unavailable. Changed engine,
cage, toolchain, clock, CPU policy, firmware or method requires fresh
attestation and calibration before scheduling.

These are proposed Lab maxima, each narrowed by the active 0078
promise, 0115 engine caps, 0044 site allowance and 0082 wall. No cap is
activated without measured admission at the supported memory floor.

| Bound | Proposed maximum and behaviour |
| --- | --- |
| Concurrency | One active Lab run on the PC and one queued run per site. One 0019 lease and one worker CPU slot; never pre-empt live rank or review. |
| Memory | 768 MiB aggregate for worker, broker overhead, scratch and projection together; render process at most 512 MiB and also within the active 0115 cap. Hold admission if the 0078 promise is smaller. |
| Capture time | 20 s real time per capture and 10 s virtual when the virtual clock is selected. No more than three captures or 60 s actual capture time per local job. One public capture only. Settle requires the final 500 ms without eligible new load or approved fixture paint, otherwise the deadline marks partial. |
| Network and trace | Public route at most one top-level navigation and 80 sent resource/redirect attempts, all inside the predeclared grant. Log at most 160 attempts including denied and local ones, with overflow making trace incomplete. At most 8 MiB accepted bytes per capture and at most the admitted site's lower cap. No extra setup or probe request. |
| Images and storage | Optional opt-in paint only. First 12 samples, at most 2,000 pixels along either edge and 512 KiB each, under the lower 0115 cap. Stop sampling at three seconds, the last image bound or any earlier stop. A longer capture cannot yield complete visual progress from those images alone. At most 16 MiB encrypted durable report and assets per capture, 128 MiB scratch per run, and 30 days or the lower source term. |
| Imports and views | At most one 8 MiB typed source file, 5,000 supplied rows per import and 200 rows per protected page; no active or compressed file format in this profile. A role or rights mismatch, parser gap or excess bound commits no new evidence. |
| Public page footprint | Zero new on-page scripts, queries, added bytes, instrumentation or native effects. Existing guest Audit pace, slowing and pause remain mandatory. |

## Execution and reports

The 0019 scheduler records an immutable plan and rechecks current grant,
source, quota, calibration and 0078 promise at claim, dispatch, commit,
restore and read. A paused or withdrawn grant fences each outstanding
attempt and further capture. Stop work with the owning scheduler and
delete uncommitted scratch. Commit exactly one sealed capture per
admitted effect key (site, plan identity, capture slot and source
revision). Reconcile known durable state after a crash. A proposed
public run that may already have dispatched is held with its partial
status; restart and view never redispatch it. A new attempt requires a
fresh exact grant. A released capture with an unavailable method may
still show its eligible measured facts.

The desktop can show authorised pixels. Local tools expose only the
permitted typed summaries and safe evidence references under 0140.
They never return or dereference an image or private visual region.
Present source,
population, revision, profile, stop and reason beside each measurement.
A request table is the keyboard equivalent of the waterfall. The
filmstrip is a sequence of admitted samples with time, available textual
event markers and an accessible list; playback uses actual sample
spacing and never fills unseen frames. Preview uses only authorised
immutable pixels. Plain read-only report opening and navigation never
start a capture, change a baseline, grant scope or send a notice.
Existing 0094/0100 local exports can receive eligible summaries from
their own exact profiles; no new format, PDF writer or developer archive
comes from this spec.

Compare same-source captures with the same profile, statistic, metric,
units, method, coverage and admitted clock. Keep deliberate treatment
changes named and pin all other inputs. Show before, after, signed
difference, raw eligible count, excluded count and scope. Compare
regional sets only when provenance, population, method and intervals
are proved compatible; a region label is insufficient. Use existing
0047 matched-budget and notice rules when applicable. Structure
findings come from typed admitted 0044/0047/0115 observations. Show
measured changes separately from estimates, with uncertainty, bounds
and reasons. A waterfall correlation is not the cause of a delay.

### Local tool projections

Use 0020 same-site static actions and dedicated
`audit.performance.read` plus the exact projection grant.

| Action | Input | Output |
|---|---|---|
| audit_performance_summary | Scoped capture ID and up to 16 admitted metric IDs | Eligible values, source, unit, method, coverage, bounded fact IDs and safe unavailable reasons. |
| audit_waterfall_rows | Capture ID, optional admitted resource class, cursor | Up to 50 eligible attempt and phase summaries in capture order. Include timing state and safe reason. Redact raw URLs, initiator text, content and pixels before release. |

Reads open stored results only. Failure, pause and scope denial have
no acquisition effect. Every cursor uses the current 0143 site,
capture, source, session, field, grant and erasure fence.

### Evidence and category priorities

Show individual evidence and per-kind comparisons. The Audit Grid may
pass the existing compatible findings to 0048 for its explained,
within-kind priorities. Show each reason, evidence limits and what
would make the priority change. The public GTmetrix report guide below
documents a combined grade. The Lab has no grade, combined
score or score history. A changed profile leaves prior evidence intact.

## Requirements and planned verification

These are proposed named cases. Runtime admission uses the owning
checks in 0044, 0047, 0082, 0115 and 0116; no Lab case substitutes for
those proofs.

| Requirement | Planned proof and expected result |
| --- | --- |
| PLAB-01 Exact acquisition | BDD-PLAB-01: give one own public page a current grant, then try excluded redirect, revoked resource and protected staging. The exact public dispatch alone can proceed; missing resources mark coverage. A supplied staging summary is inspectable only. Stand-in broker records no denied dispatch. Reuse RN-03/04/33. |
| PLAB-02 Independent calibration | IT-PLAB-02: record five alternating synthetic pairs on each candidate platform; include precise boundary, one over, changed clock identity and no matching presentation event. Only calibrated matching variants produce results. Require reviewed EX-02/cage evidence, independent of fixture correctness. |
| PLAB-03 Metric semantics | IT-PLAB-03: exercise blank and eligible paints, late larger candidate, exactly 50 ms task and boundary clipping, a 100 ms task from 20 ms before FCP yielding 50 ms blocking, a provisional response followed by a proved final first byte, cache-refused variant, timed-out response and admitted scripted update with an unrelated frame. Record per-metric results, source and clock. No lab INP; an absent event leaves unavailable. |
| PLAB-04 Censored trace | IT-PLAB-04: replay recorded offline attempts with cache reuse, 304, denied request and coarsened phase, a dropped frame, and a lost settle interval. Distinguish recorded original from replay; mark incomplete values and never fill phases or frames. Reuse IT-LS-11. |
| PLAB-05 Scoped comparisons | BDD-PLAB-05: compare compatible complete local captures, changed metric version, mixed field p75/local median, URL/origin fallback and incomparable supplied regions. Only compatible evidence yields a delta. Unknown method survives safe reading. Incompatible or partial evidence cannot trigger an alert. |
| PLAB-06 Budget and recovery | FI-PLAB-06: stop at each time, count, memory or disk fence and crash before dispatch, after uncertain dispatch, after durable commit and before acknowledgements. Recorded outcomes show ceiling and phase; no double capture or duplicate public request. A revoked epoch blocks recovery and reading. |
| PLAB-07 Views, custody and erasure | IT-PLAB-07: test keyboard table/player, stale view, wrong-site role, dropped frame, denied pixel lookup through MCP, revoked local export, tampered object, pending erase and stale restored backup. No unproved bytes or data leak; history reflects limited coverage; erase follows 0030/0116 and existing recipient scope. |
| PLAB-08 Evidence and priority | BDD-PLAB-08: open a fully measured report alongside sparse and mixed input and imported weights. Eligible measures remain readable and priorities keep their stated reasons in each category. No grade appears and imported weights change no priority or prior result. |
| PLAB-09 Read-only effect boundary | IT-PLAB-09: open snapshots, compare, import timing rows and scrub an expired report. Verify zero site effect, outside connections, ranking inputs and extra page work. Catch denied cloud/profile actions in both screen and local tools. |

### Bounded state model plan

Use the 0114 Quint model lane with two sites, up to two grants per
site, two grant epochs, two source revisions, two captures and two
attempts per capture. Model claim, current grant check, consume,
dispatch, revoke, timeout, crash, compare, commit, read, remove and
restore. Check bounded invariants: every dispatched attempt has its
current exact authority at the broker fence; an uncertain public
attempt cannot replay; commit at most once per effect key; no
projection crosses site or reads after withdrawal; a restored older
epoch cannot release evidence; an incompatible or partial input
cannot yield a complete comparison. Include a counterexample for
removing the dispatch fence and one for reusing a stale capture on
restore. Link model transitions to PLAB-01/05/06/07/09, INV-0140-AU-02
through INV-0140-AU-06 and the owning integration proof. A bounded model,
synthetic sample, or research
candidate is never runtime, policy or release admission.

## Public measurement references

| Source | Evidence scope |
| --- | --- |
| [W3C Paint Timing](https://www.w3.org/TR/paint-timing/) | Browser paint events |
| [W3C Largest Contentful Paint](https://www.w3.org/TR/largest-contentful-paint/) | Candidate selection and timing |
| [W3C Resource Timing](https://www.w3.org/TR/resource-timing-1/) | Network phases and timing exposure |
| [Chrome Lighthouse Speed Index](https://developer.chrome.com/docs/lighthouse/performance/speed-index) | Visual progression |
| [Chrome Lighthouse total blocking time](https://developer.chrome.com/docs/lighthouse/performance/lighthouse-total-blocking-time) | Long-task measurement |
| [Chrome guidance on time to first byte](https://web.dev/articles/ttfb/) | Navigation and response boundary |
| [Chrome guidance on layout stability](https://web.dev/articles/cls/) | Eligible shift events and windows |
| [Chrome guidance on INP](https://web.dev/articles/inp) | Real interactions and their latency |
| [Chrome UX Report API](https://developer.chrome.com/docs/crux/api) | Rolling field coverage and URL or origin |
| [GTmetrix report guide](https://gtmetrix.com/blog/everything-you-need-to-know-about-the-new-gtmetrix-report-powered-by-lighthouse/) | Public comparison of product behaviour |
