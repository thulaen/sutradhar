#!/bin/sh
# Runs Forgejo's runner one job at a time, each inside a heavy-job place
# (see the runner specification):
#   deploy/runner/loop.sh RUNNER_DIR RUNNER_BIN QUEUE_DIR RUN_AS SOCKET
# RUN_AS is the account that owns the container service at SOCKET.
set -eu
dir=$1
bin=$2
queue=$3/bin/pankti
user=$4
socket=$5
while :; do
  # With no job waiting, the runner says so and ends at once, so the
  # place is held only for a moment.
  # shellcheck disable=SC2016 # the inner shell expands these
  "$queue" run -- sudo -n -u "$user" env DOCKER_HOST="unix://$socket" \
    sh -c 'cd "$1" && exec "$2" one-job --config config.yml' \
    runner "$dir" "$bin" || true
  sleep 30
done
