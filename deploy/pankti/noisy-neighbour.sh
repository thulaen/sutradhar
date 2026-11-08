#!/bin/sh
# The noisy-neighbour test (see the pankti specification). It runs one
# heavy job that takes its whole place, about 3.8 GB and 8 CPU threads,
# for a minute, and checks that every probe still answers within 2 s:
#   deploy/pankti/noisy-neighbour.sh QUEUE_DIR PROBE...
# Each PROBE is a shell command, such as a curl of Forgejo's health page.
set -eu
queue=$1/bin/pankti
shift
[ "$#" -gt 0 ] || {
  echo "noisy-neighbour: name at least one probe" >&2
  exit 2
}
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
mark=$work/started
# The job marks that it has begun, keeps 8 threads busy, and holds about
# 3.8 GB until the minute is up. It fails if the memory part is stopped.
# shellcheck disable=SC2016 # the job's own shell expands these
hog=': >"$1"
for _ in 1 2 3 4 5 6 7 8; do timeout 60 sh -c "while :; do :; done" & done
{ head -c 3800000000 /dev/zero; sleep 60; } | tail >/dev/null
held=$?
wait
exit "$held"'
"$queue" run -- sh -c "$hog" hog "$mark" &
job=$!
# The job may wait in line for a place first; probing starts only once
# it runs, so the test can never pass without the load.
for _ in $(seq 1800); do
  [ -e "$mark" ] && break
  sleep 1
done
[ -e "$mark" ] || {
  echo "noisy-neighbour: the heavy job did not start within 30 minutes" >&2
  exit 1
}
sleep 10
failed=0
for _ in $(seq 20); do
  for probe in "$@"; do
    if ! timeout 2 sh -c "$probe" >/dev/null 2>&1; then
      echo "noisy-neighbour: a probe did not answer in 2 s: $probe" >&2
      failed=1
    fi
  done
  sleep 2
done
if ! wait "$job"; then
  echo "noisy-neighbour: the heavy job failed or was stopped" >&2
  failed=1
fi
if [ "$failed" -ne 0 ]; then
  exit 1
fi
echo "noisy-neighbour: every probe answered while the heavy job ran"
