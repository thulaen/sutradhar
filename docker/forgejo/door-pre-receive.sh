#!/bin/sh
# Forgejo pre-receive hook: hands every pushed ref to aegis-door, which
# makes every decision (AGENTS.md sections 7 to 9). The push goes in
# only if the door answers yes for every ref. The ref name goes first in
# the address, so a crafted name can only drop the rest and be refused.
set -eu
socket="${DOOR_SOCKET:-/door/door.sock}"
zero=0000000000000000000000000000000000000000
main=$(git rev-parse --verify --quiet refs/heads/main || echo "$zero")
pack=$(mktemp -p "$GIT_DIR" door-pack.XXXXXX)
answer=$(mktemp -p "$GIT_DIR" door-answer.XXXXXX)
trap 'rm -f "$pack" "$answer"' EXIT
while read -r old new ref; do
  if [ "$new" = "$zero" ]; then
    : >"$pack"
  elif [ "$old" = "$zero" ] && [ "$main" != "$zero" ]; then
    printf '%s\n--not\n%s\n' "$new" "$main" |
      git pack-objects --revs --stdout -q >"$pack"
  elif [ "$old" = "$zero" ]; then
    printf '%s\n' "$new" | git pack-objects --revs --stdout -q >"$pack"
  else
    printf '%s\n--not\n%s\n' "$new" "$old" |
      git pack-objects --revs --stdout -q >"$pack"
  fi
  code=$(curl -sS --globoff --unix-socket "$socket" -X POST -H 'Expect:' \
    -H 'Content-Type: application/octet-stream' -T "$pack" \
    -o "$answer" -w '%{http_code}' \
    "http://door/check?ref=$ref&old=$old&new=$new&main=$main") || code=000
  cat "$answer"
  if [ "$code" != 200 ]; then
    echo "door: the push is refused"
    exit 1
  fi
done
