#!/bin/sh
# Puts the door's hook into Forgejo's repository and restarts Forgejo
# with the door's folder mounted (docs/specs/0001-door.md):
#   deploy/door/install-hook.sh DOCKER_COMMAND...
# DOCKER_COMMAND is how to run docker for Forgejo, such as `docker` or
# `sudo -u NAME env DOCKER_HOST=... docker`.
set -eu
hooks=/var/lib/gitea/git/repositories/sutradhar-owner/sutradhar.git/hooks
"$@" compose -f docker/forgejo/compose.yaml up --detach --quiet-pull
"$@" exec -i -u git sutradhar-forgejo sh -c \
  "cat >$hooks/pre-receive.d/aegis-door &&
   chmod 0755 $hooks/pre-receive.d/aegis-door" \
  <docker/forgejo/door-pre-receive.sh
echo "door: the hook is in place"
