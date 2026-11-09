#!/bin/bash
# The nightly disk sweep (see the pankti specification). Builds and
# caches only grow, so each night:
#   deploy/pankti/sweep.sh QUEUE_DIR CAP_BYTES FLOOR_BYTES FOLDER...
# - it holds both of the queue's places, so no queued job runs meanwhile,
#   and skips a folder whose cargo build lock is held by any other build;
# - leftover work copies in FOLDER/aegis-work older than a day go;
# - a FOLDER bigger than CAP_BYTES is emptied (it only holds build
#   output, which the next build makes again);
# - scratch folders our tests leave in the temp folder (aegis-* and
#   forge-sync-*, this account's own, over an hour old) go, since on Dell
#   the temp folder lives in memory;
# - it fails, so the alarm is raised, when the disk still has less than
#   FLOOR_BYTES free, or when anything goes wrong.
# A FOLDER is touched only when it is a full path, not / or a home
# folder, and holds cargo's own files: a real CACHEDIR.TAG (not a link)
# with the standard mark, and .rustc_info.json.
set -eu
queue=$1
cap=$2
floor=$3
shift 3
first=${1:-.}
mark="Signature: 8a477f597d28d172789f06886806bc55"

build_folder() {
  home=${HOME:-/root}
  homes=${home%/*}
  case "$1" in
  / | "$homes" | "$homes"/* | /root | /root/* | "$home"/*) return 1 ;;
  /*) ;;
  *) return 1 ;;
  esac
  [ -d "$1" ] && [ ! -L "$1" ] || return 1
  [ -f "$1/CACHEDIR.TAG" ] && [ ! -L "$1/CACHEDIR.TAG" ] || return 1
  [ "$(head -n 1 "$1/CACHEDIR.TAG")" = "$mark" ] || return 1
  [ -f "$1/.rustc_info.json" ] && [ ! -L "$1/.rustc_info.json" ]
}

# Takes every cargo build lock in the folder and keeps it until the
# folder is done, so no build can start in it meanwhile. Fails when a
# build holds one now.
held=()
hold_locks() {
  held=()
  for lock in "$1"/*/.cargo-lock "$1"/*/*/.cargo-lock; do
    [ -e "$lock" ] || continue
    exec {fd}<"$lock"
    held+=("$fd")
    flock -n "$fd" || return 1
  done
}

let_go() {
  for fd in "${held[@]}"; do
    exec {fd}<&-
  done
  held=()
}

sweep() {
  dir=$1
  if ! build_folder "$dir"; then
    echo "sweep: $dir is not a cargo build folder; left alone" >&2
    return 0
  fi
  if ! hold_locks "$dir"; then
    let_go
    echo "sweep: $dir is being built; left for tomorrow"
    return 0
  fi
  if [ -d "$dir/aegis-work" ]; then
    find "$dir/aegis-work" -mindepth 1 -maxdepth 1 -mtime +0 \
      -exec rm -rf --one-file-system -- {} +
  fi
  size=$(du -sb -- "$dir" | cut -f1)
  if [ "$size" -gt "$cap" ]; then
    find "$dir" -mindepth 1 -maxdepth 1 ! -name CACHEDIR.TAG \
      -exec rm -rf --one-file-system -- {} +
    echo "sweep: emptied $dir ($size bytes)"
  fi
  let_go
}

# Hold the line, then both places, as a benchmark does.
exec 7<"$queue/line" 8<"$queue/place-1" 9<"$queue/place-2"
flock 7
flock 8
flock 9
for dir in "$@"; do
  sweep "$dir"
done
tmp=${TMPDIR:-/tmp}
find "$tmp" -mindepth 1 -maxdepth 1 -user "$(id -un)" -mmin +60 \
  \( -name 'aegis-*' -o -name 'forge-sync-*' \) \
  -exec rm -rf --one-file-system -- {} +
free=$(df -B1 --output=avail "$first" | tail -n 1 | tr -d ' ')
case "$free" in
'' | *[!0-9]*)
  echo "sweep: cannot read the free space" >&2
  exit 1
  ;;
esac
if [ "$free" -lt "$floor" ]; then
  echo "sweep: only $free bytes free, under the floor of $floor" >&2
  exit 1
fi
echo "sweep: $free bytes free"
