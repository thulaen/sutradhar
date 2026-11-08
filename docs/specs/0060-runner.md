# 0060: Forgejo's runner on Dell

Forgejo Actions runs jobs that a push asks for, such as checks. The
runner is the program on Dell that takes those jobs and runs them. It is
roadmap Phase 1 item 2. The rules it follows are in
[AGENTS.md section 11](../../AGENTS.md#11-the-shared-computer-dell):
runner jobs are heavy jobs, and they run in containers that cannot reach
the release key, the GitHub tokens or the private notes on Dell.

## What it does

- The runner is Forgejo's own, pinned by version and hash in
  `gate/tools.toml`, signed up to this repository only, with the label
  `dell`.
- It takes one job at a time. A small loop asks Forgejo for a job every
  30 seconds, inside a heavy-job place (spec 0059). With no job waiting,
  the runner says so and ends at once, so the place is held only for a
  moment. With a job, the place is held until the job ends.
- Each job runs in a fresh container made from the official Ubuntu 26.04
  image, pinned by its digest. The container:
  - gets no folder from Dell (`valid_volumes` is empty);
  - cannot reach the container service (`docker_host` is `-`);
  - is not privileged;
  - is capped at 4 GB of memory, no swap, 8 CPU threads and 4,096
    processes;
  - joins only `runner-jobs`, a closed network (no way out) that only
    Forgejo's container also joins, so it reaches Forgejo and nothing
    else: not Dell, not the rest of the home network, not the internet.
- The runner's cache server is off, since it would need a folder on
  Dell.
- The container service belongs to the `sutradhar-agent` account, as do
  the runner's private folder and its sign-up file (mode 0600). The loop
  runs as a user with sudo, which pankti needs, and starts the runner as
  `sutradhar-agent`.

## Workflows

Workflows live in `.forgejo/workflows/`. Every step is a plain shell
command. The gate check `workflows.sealed` reads each workflow with a
full YAML reader and refuses `container`, `services`, `options` and
`uses` as a key anywhere, however the key is written, since each would
get past the sealed, capped container or bring a language runtime into
the job. It refuses a Forgejo workflow that runs anywhere but `dell`. So
that our reader and Forgejo's cannot disagree, a workflow may hold only
plain printable ASCII with `\n` line breaks, as one document, with no
YAML tags (a tag such as `!!binary` could make a key mean something
other than its text).

`runner-check.yml` runs on every push to `main` and proves the points
above: the job is in the pinned Ubuntu image; it has no route out and
cannot reach the internet; it reaches Forgejo; it sees no socket, so no
container service; and it has at most 4 GB of memory. It also lists its
mounts, so a review can see that no folder from Dell is among them.

The full gate in CI, with the pinned Rust toolchain in its own image, is
part of Phase 2 (GitHub CI matches it).

## Install

```sh
deploy/runner/install.sh RUNNER_DIR RUNNER_BIN QUEUE_DIR RUN_AS SOCKET \
  HEADER_FILE
```

It first checks that the runner program matches its hash in
`gate/tools.toml`. It writes the settings from `deploy/runner/config.yml`
and, the first time only, signs the runner up with a sign-up token
fetched using `HEADER_FILE`, Forgejo's private sign-in header. The token
goes to the runner on standard input, so it never appears on a command
line or in a log. Forgejo's sign-up token can be used again until it is
reset, so after a sign-up it is reset inside Forgejo's container with
`forgejo forgejo-cli actions generate-runner-token`, its output thrown
away. Then the script installs the loop and starts
`sutradhar-runner.service`.

## Limits

- The loop's place is held for a moment on each check, even with no job
  waiting, so another heavy job may wait about a second longer.
- A job can reach Forgejo's web and API, with the job's own short-lived
  token only.
- A job's container memory is counted by the container service, not
  by the heavy-job slice. The container's own 4 GB cap and the place it
  holds keep it inside the Dell budget.
- The runner's own messages are filed in the system log under each
  pass's capped scope (`run-*.scope`), not under the loop's service.

## Tests

- Live: `runner-check.yml` passes on Dell on the push that adds it.
- Each check above is a step of that workflow, so a changed setting that
  breaks one fails the run.
