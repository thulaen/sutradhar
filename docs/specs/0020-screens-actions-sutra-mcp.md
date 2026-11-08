# 0020: Screens, actions, sutra and MCP

Everything a person or an AI tool can do with Sutradhar is written down
once, as an "action": its name, who may use it, what it needs, what it
changes and its help text. The desktop app, the `sutra` command and the
MCP server are doors into the same list. Each door checks the same
Cedar permissions, writes the same audit entry and shows the same help.
No door can do what the others cannot, and none gives hidden power to
publish or moderate. It is roadmap Phase 5 item 5 and Phase 3 item 5.

Example: a moderator accepts a suggestion in the forum's admin pages;
the owner runs `sutra setu suggestion approve 81234`; Claude, connected
through MCP, asks to do the same. All three reach
`setu.suggestion.approve`. The first two succeed if the person is
permitted. The third is refused with SUT-805, "AI agents may recommend
but not do this".

## What it does

### Words used here

- **Action**: one named operation, such as `vinyas.setting.change`.
- **Action layer**: the Rust code every door calls to run an action.
- **Door**: a way in: the desktop app, `sutra` and MCP. The management
  API the app uses is the same layer, not a fourth door. Moderator
  decisions made in a site's admin pages arrive through the site link
  and run as the same actions, with the moderator's account as caller.
- **MCP** (Model Context Protocol): an open standard by which AI tools
  find and call another program's tools.
- **Plan and apply**: an action that changes something first returns a
  plan (a preview with an id); applying the plan makes the change.
- **Capability id**: the CAP-NNN entry an action serves.

### Scope

In scope: the action registry and runner; the `sutra` command tree,
made from the registry; discovery and built-in help; the MCP server;
the desktop app's shell (navigation, command palette, help panel,
notices, engine-down states), its link to the management API and event
stream, and the screens listed under capability ids; SDS rules on every
screen. Out of scope: what each action does; the review page's content;
health and diagnostics screens; settings history; charts; progress.
Those parts declare their actions here.

### Crates

- `sutradhar-actions`: the `Action` trait, the registry and the runner.
- `sutradhar-dwarpal` ("gatekeeper"): who the caller is and what they
  may do, through the Cedar rules. Trait `Authoriser`.
- `sutradhar-lekha`: audit. Trait `AuditSink`.
- `sutradhar-mcp`: MCP messages and the mapping from actions to MCP
  tools and resources, on the official Rust MCP SDK (rmcp), pinned.
- The `sutradhar` program mounts the actions as Axum routes on the
  management socket, with an OpenAPI description; `sutra` uses clap;
  `sutra mcp` serves MCP.

```rust
#[async_trait::async_trait]
pub trait Action: Send + Sync + 'static {
    type Input: serde::de::DeserializeOwned + utoipa::ToSchema
        + garde::Validate;
    type Output: serde::Serialize + utoipa::ToSchema;
    fn spec(&self) -> &'static ActionSpec;
    async fn plan(&self, cx: &Ctx, input: &Self::Input)
        -> Result<Plan, ActionError>;
    async fn apply(&self, cx: &Ctx, plan: &Plan)
        -> Result<Self::Output, ActionError>;
}
```

`ActionSpec` holds: id, capability ids, permission, kind (read, change,
dangerous or long-running), agent rule (allowed, propose only or
refused), idempotent or not, summary, help text, examples, manual page,
error codes and the progress job it starts, if any. Read actions skip
`plan`.

### The runner

1. For every door, in this order: identify the caller, check the agent
   rule, authorise with Cedar, validate input (garde), rate-limit
   (governor), plan, ask for confirmation where the kind needs it,
   apply, write the audit entry and the change-journal entry, publish
   the event (sanchar) and return.
2. A door only turns its own format into an `Input` and the result back.
3. One job per part: doors translate, the runner sequences, each action
   does one thing, dwarpal decides permission.
4. A module adds actions by registering them; the runner and doors do
   not change. Registration is explicit code
   (`registry.add(ApproveSuggestion)`), checked at start-up. No plug-in
   loading and no reflection.
5. `Authoriser`, `AuditSink`, `EventBus`, `Clock` and `RateLimiter` are
   separate traits, each with a test fake. Actions depend on core
   traits; the core never imports a feature module.
6. One declaration gives the route, the API description entry, the
   `sutra` subcommand and its `--help`, the MCP tool and its JSON
   Schema, the palette entry, the help panel and the manual's reference
   page.
7. Every preview is made by the service, carries `preview: true` and is
   shown as "Preview, not a decision" (0018, item B).

### Discovery and help

- `sutra help` lists modules; `sutra <module> --help` lists actions;
  `sutra describe <action>` prints the full help with examples,
  permission, capability ids and error codes.
- `sutra find <words>` searches names, summaries and help with tantivy,
  so "how do I stop the backlog" finds `karya.job.pause`.
- `sutra docs <topic>` prints the built-in manual page for the running
  version.
- `sutra error SUT-805` explains an error code.
- Every command takes `--json` and returns the action's output schema;
  `--plan` stops after the plan; `--yes` confirms a plan without asking
  only where the action's kind allows it.
- The gate fails any action without a summary, an example, a manual
  page, a capability id or documented error codes.

### MCP

1. Reached only on the owner's PC: through `sutra mcp` (standard input
   and output, a program the owner starts) or the owner-only socket.
   Never a network port.
2. Tools: one per action the token may use, named by its action id, with
   the action's JSON Schema as input and its help as description. Read
   actions are marked read-only.
3. Resources: the built-in manual pages, the action catalogue and
   read-only records.
4. Tokens: the owner makes an agent token on the Agents screen,
   choosing the one site it sees, the site account it acts for, its
   scopes, an expiry (default 30 days) and an optional action limit. A
   token never has more permission than its account. Tokens are shown
   once, stored only as SHA-256 hashes, scoped, expiring and revocable
   at once. A token whose account loses staff rights is refused at its
   next call.
5. Agent rules: a tool may read and may draft. Settings changes are
   plans the owner applies, with the effect shown, checks run and
   history kept. Ranking changes are proposed and tested only.
   Moderation and link approval are recommendations only. Every draft
   is a suggestion a moderator must accept.
6. Content: guest-visible by default; a private section only with an
   explicit scope; private conversations, credentials, keys, audit
   logs, passwords, tokens and email addresses never. Post text is
   data, never instructions: results put it in clearly marked fields
   and tool descriptions say so.
7. A tool list larger than the client's window is paged.
8. Only MCP is offered; another agent protocol is added only when the
   owner names it.

### The desktop app's shell

The Dioxus app holds no engine state and writes only through the
management API, over the owner-only socket.

1. **Engine client**: one typed client for every API call, with a test
   fake. Every call carries a trace. A test keeps the client in step
   with the API description on every commit.
2. **Events**: one task holds the event stream from the service and
   fans it out to screens. It reconnects with back-off from 1 to 30
   seconds and replays from the last event id the service still holds
   (at most 10 minutes or 10,000 events, whichever is smaller). Beyond
   that the screen reloads its data and says so.
3. **Breaker**: after 5 failures in 30 seconds it opens for 15 seconds
   and screens show the engine-down state.
4. **Shell**: layout, navigation, skip link, palette, help panel,
   notices and the engine-down banner.
5. Times show in the owner's time zone. Permissions come from the
   service, never from the app.
6. Ganit's settings what-ifs are read actions that call ganit with a
   strict time limit. If ganit is slow or down, the screen says
   "estimate unavailable" rather than waiting.
7. **Source offer** (AGPL-3.0-or-later): every screen's footer has a
   "Source code" link to the exact source of the running version and
   its third-party notices; `sutra source` prints the same. If the
   running build is not a published one, the link points to the address
   the owner enters, with a reminder that a changed version offered to
   others must offer its own source.

### SDS on every screen

Plain British English from translation files. Type and spacing on an
8px grid; error summaries, plain warnings and underlined links; the
yellow focus ring: a 2px gap, a 3px `focus` (#FFD23F) band and a 2px
`focus-outline` (#0B0A14) band on `:focus-visible`, never removed.
Skeletons, never bare spinners; every empty state says what happens
next; reduced motion respected; no layout shift.

### Data

Owned by the service in PostgreSQL, with RLS on and forced on every
table and a policy per role; no role has `BYPASSRLS`. For example the
service role may insert and read `action_runs` but not update or delete
them, and reads only its own site's rows.

- `action_runs`: run id, action id, door, caller id, token id (or
  null), plan id, outcome, started, finished, error code. Kept 90 days.
- `action_plans`: plan id, action id, caller, input hash (blake3),
  preview, expiry (15 minutes), applied flag. Applying an expired,
  changed or already applied plan is refused. Expiry uses the
  database's monotonic time.
- `agent_tokens`: id, name, site, account id, scopes, action allow list,
  expiry, created by, last used, revoked, token hash.
- Audit entries go to lekha.

### API and events

- List the catalogue the caller may use, with schemas and help; show
  one action.
- Plan a change and apply a plan; run a read. Every request may carry
  an `Idempotency-Key`; a repeat returns the first result.
- The event stream: topics per module, each event with id, topic, time
  and ids only.
- Events published: `action.applied`, `action.refused`,
  `agent_token.created`, `agent_token.revoked`.

### Failures

Every failure is plain English with an error code and a manual link:

| Code | What the person sees |
| --- | --- |
| SUT-800 | "Sutradhar's engine is not answering." |
| SUT-801 | "Your account cannot do this." Names the permission |
| SUT-802 | Error summary, each field named |
| SUT-803 | "Things changed since you previewed this. Preview again." |
| SUT-804 | "Too many attempts. Try again in N seconds." |
| SUT-805 | "AI agents may recommend but not do this." |
| SUT-806 | "This agent's token no longer works." |
| SUT-807 | "Someone already did this", with who and when |
| SUT-808 | Banner "Live updates paused, reconnecting" |

- A failure inside `apply` rolls back the action's own transaction; the
  audit entry records the refusal or failure, never a success.
- The plan id ties preview to change. A lost connection after apply
  returns the stored result on retry with the same idempotency key.
- If the window is closed or crashes, the service, `sutra` and MCP keep
  working, and so do suggestions. If the service is down, every screen
  shows SUT-800 in place of its data, never stale numbers shown as live;
  cached figures are labelled with their time.
- Long actions report through the progress system and are cancelled only
  through `karya.cancel`.
- CPU-heavy work never runs on Tokio's threads; the layer only waits.
- Two people approving the same thing: one succeeds, the other gets
  SUT-807 naming the first.
- A renamed action keeps its old id as an alias for one release, then
  it is retired in the registry and the release notes.
- A renamed site account keeps its account id in the audit.

### Security and privacy

- One permission check, in dwarpal with Cedar, for every door. The app
  may hide controls a person cannot use, as a courtesy; the API refuses
  regardless.
- Publication and moderation actions are refused or propose-only for
  agents, tested for every action.
- Rate limits per token and per account.
- Tokens never appear in logs, spans or audit entries (secret types).
- No member text in metrics or spans: ids, counts and durations only.
- The management API is fuzzed from its API description.

### Accessibility (WCAG 2.2 AA)

Every screen, in light and dark, at wide and narrow window widths:

- Every control reachable by keyboard in a logical order; a skip link;
  the focus ring on `:focus-visible`; focus moves to the main heading
  after navigation and back to the trigger when a dialog closes; focus
  never hidden under sticky bars (2.4.11).
- New events are announced in a polite live region, at most one
  announcement every 5 seconds, never moving focus. A "pause live
  updates" switch stops the screen changing.
- The palette follows the ARIA combobox pattern; its shortcut can be
  turned off or changed (2.1.4).
- Targets at least 24 by 24 CSS pixels (2.5.8); SDS controls are 48px
  (40px compact). No action needs dragging (2.5.7).
- Help is in the same place on every screen (3.2.6); forms do not ask
  again for what was entered (3.3.7).
- Errors use the error summary, linked to each field; colour never
  carries meaning alone.
- Reduced motion turns off animation; content reflows at 320 CSS pixels
  without sideways scrolling.

### Capability ids

Every screen, command and MCP tool carries a capability id; an action
without one fails the gate. The doors are routes, not capabilities.

| Where | Capability |
| --- | --- |
| Palette, `sutra find` | CAP-006 search from any page |
| Help panel, catalogue, `describe`, `docs`, `actions`, MCP resources | CAP-008 look up what something does |
| Error panel, `sutra error` | CAP-009 look up an error code |
| Manual link on every screen | CAP-007 |
| Access screen | CAP-071 choose which staff groups may act |
| Security log screen | CAP-072 see refused attempts and limits hit |
| Agents screen, `sutra agent token create\|list\|revoke` | NEW-031-A connect an AI agent with a limited, revocable token |
| "Source code" link, `sutra source` | NEW-031-C get the source of the exact version you run |
| Saved layouts | NEW-031-D save my screen layout |
| Settings what-ifs | CAP-012, CAP-015 |

Each feature's actions (including template, rollout and report actions
of the agency edition) carry that feature's ids.

### Observability

- Metrics: `sutradhar_action_runs_total{action,door,outcome}`,
  `sutradhar_action_duration_seconds{action,door}`,
  `sutradhar_mcp_sessions`, `sutradhar_event_stream_clients`,
  `sutradhar_app_engine_breaker_state`; spans per action with action id
  and run id only.
- Alerts: breaker open over 2 minutes; event stream down over 1 minute;
  agent refusal rate above its usual level (a fault record, not a
  notice).
- Every metric and attribute is on the "what we collect" page.

## Limits

- Speed targets on Dell, each also under the 5% median rule:

| Need | Target |
| --- | --- |
| A screen's largest content appears | under 1.5 s, p95 |
| Layout shift on any screen | 0 |
| A click on an action gives visible feedback | under 100 ms |
| A read action returns, layer only | p95 under 50 ms |
| A plan or apply returns, layer only | p95 under 200 ms |
| Another screen shows an approval | p95 under 1 s |
| `sutra --help` or `sutra describe` prints | under 150 ms |
| MCP tool list for a token | p95 under 300 ms |
| `sutra find` answers | p95 under 100 ms |

  "Layer only" leaves out the action's own work. With 10,000 actions,
  the palette and `sutra find` stay within these targets.
- Memory, inside the service slice: registry, runner and MCP 24
  MiB; help and manual text 8 MiB; the help index 4 MiB, memory-mapped.
  The desktop window, web view included, stays within its cap with
  the event fan-out. The nightly peak-memory check writes a fault record
  if a share is passed. Event replay holds at most 10 minutes or 10,000
  events.
- Plans expire after 15 minutes. Tokens expire after 30 days by
  default. Action runs are kept 90 days.

## Footprint budget

This part adds nothing to public pages.

## How it is tested

Rust tests in the crates named above, screen tests on the Dioxus app,
SQL tests for policies and Cedar rule tests. Coverage is 100% of lines,
branches and MC/DC. The runner's step order, `Authoriser`, the agent
rules and plan expiry need every mutant killed, property tests and Kani
proofs.

1. One action, three doors: an approval through the API, `sutra` and
   the app reaches `setu.suggestion.approve`, writes one audit entry
   with its door and returns the same output shape.
2. Every publication and moderation action is "refused" or "propose
   only" for agent tokens.
3. MCP calling `setu.suggestion.approve` with the owner's agent token is
   refused with SUT-805 and nothing changes.
4. An agent with the settings scope plans a settings change; the plan
   shows the effect and checks; it applies only when the owner applies
   it, and history records the agent and the account.
5. An agent's ranking change is stored as a proposal with its measured
   effect; nothing is activated.
6. Property and Kani proof: a token can never do what its account
   cannot.
7. Without the private scope, MCP lists and searches show no post that
   is not guest-visible, no private conversation and no email address;
   a session sees one site only and never credentials, keys or audit
   logs (unit and property).
8. A plan applied twice, after 15 minutes, or after its input changed is
   refused with SUT-807 or SUT-803 (unit and property); Kani proof that
   no apply runs without a matching, unexpired, unapplied plan.
9. Two approvals within 10 ms: one succeeds, the other gets SUT-807
   naming the first, and the first person's screen updates within 1 s
   (loom and stress).
10. The gate walk finds every action with summary, example, manual page,
    capability id and documented errors; `sutra describe` prints them.
11. `sutra find "stop the backlog"` puts `karya.job.pause` in the first
    three results.
12. Service stopped: within 5 seconds an open screen shows SUT-800 in
    place of its data, no number shown as live, and it recovers by
    itself when the service returns (failure injection).
13. Window killed: suggestions keep being made and `sutra` and MCP keep
    working (failure injection).
14. Keyboard walk of every screen and state: each control reachable; the
    focus ring shows both bands and is never hidden; an accessibility
    checker passes every screen and state.
15. Each screen loads with fixture data with its largest content under
    1.5 s and zero layout shift.
16. Load: 50 app sessions and 20 agents run mixed actions for 2 minutes;
    the layer-only targets hold and no request fails.
17. A token in a request appears in no log, span or audit entry.
18. Every SDS component shown loading, empty, in error, with long
    content, dark and narrow matches its stored screenshot.
19. The footer link and `sutra source` give the source and notices of
    the exact version; for a build that is not published, the owner's
    address and the reminder.
20. SQL test: the service role cannot update an action run or read
    another site's rows, even with no filter in the query.
21. Mutation run on the rule-critical code: no mutant survives.
22. MCP is reachable on no network port.

Test-first order: action trait and registry checks (10); runner with
fakes (1, 8); dwarpal, agent rules, proofs and mutants (2, 5, 6, 21)
and policies (20); routes and idempotency keys; `sutra` tree, help,
find and error (10, 11); MCP tools, resources and tokens (3, 4, 7, 17,
22); event stream and replay; app client, events and breaker (12);
shell, palette, help panel, source link, screenshots and accessibility
(14, 15, 18, 19); the race, load and failure tests (9, 13, 16).

Facts to check while building: the current MCP specification's version
and authorisation rules, from its official site; SUT-800 to SUT-809
against the central code list; the window's real memory with the event
fan-out.

## Changes from the source

- The owner's screens are the Dioxus desktop app built from SDS, on the
  owner-only socket, instead of a served web admin app.
- Moderators accept or refuse in the sites' own admin pages, with the
  sites' own sign-in; those decisions arrive through the site link.
- Web sign-in with the forum account, the owner's emergency login
  (SUT-809, NEW-031-B, its table and event), web sessions, cookies,
  form tokens and page presence are not brought, since the app is
  reached only from the owner's own account.
- MCP runs only through `sutra mcp` or the owner-only socket, never a
  network route; each session sees one site only.
- MCP tools may read and draft only; agent settings changes become plans
  the owner applies.
- Permissions are Cedar rules evaluated by dwarpal.
- Colours come from SDS tokens; no other firm's colour values, marks or
  type are copied, and no theme add-on is used.
- The old served-app memory share is replaced by the window's cap.
- The old speed ratchet is the 5% median rule.
- Old web-stack checks and the no-script edge case are dropped; screen
  coverage follows the 100% rule.

Source: `docs/specifications/operations/actions.md`,
`operations/actions-2.md` and `operations/actions-3.md` in the archive
`archive-2026-11-08-old-repo`.
