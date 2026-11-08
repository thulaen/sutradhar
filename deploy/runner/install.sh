#!/bin/sh
# Installs or updates Forgejo's runner on Dell (see the runner
# specification). Run it from the repository's top folder:
#   deploy/runner/install.sh RUNNER_DIR RUNNER_BIN QUEUE_DIR RUN_AS SOCKET \
#     HEADER_FILE
# RUNNER_DIR is a private folder owned by RUN_AS, the account that owns
# the container service at SOCKET. RUNNER_BIN is the pinned runner
# program (gate/tools.toml). HEADER_FILE holds Forgejo's sign-in header,
# used only to sign the runner up the first time.
set -eu
forgejo=http://127.0.0.1:3000
repo=$forgejo/api/v1/repos/sutradhar-owner/sutradhar
dir=$(realpath -m "$1")
bin=$(realpath "$2")
queue=$(realpath "$3")
user=$4
socket=$5
header=$(realpath "$6")
# The runner program must match its pinned hash (AGENTS.md section 4).
want=$(awk '/^name = "forgejo-runner"/ { f = 1 }
  f && /^sha256/ { gsub(/"/, "", $3); print $3; exit }' gate/tools.toml)
echo "$want  $bin" | sha256sum --check --quiet - || {
  echo "runner: $bin does not match gate/tools.toml" >&2
  exit 1
}
sudo systemctl stop sutradhar-runner.service 2>/dev/null || true
sudo -n -u "$user" sh -c 'umask 077; mkdir -p "$1"' sh "$dir"
# shellcheck disable=SC2024 # the settings file is read as this user
sudo -n -u "$user" sh -c 'umask 077; cat >"$1/config.yml"' sh "$dir" \
  <deploy/runner/config.yml
if ! sudo -n -u "$user" test -f "$dir/.runner"; then
  url=$repo/actions/runners/registration-token
  token=$(curl -sf -H @"$header" "$url" |
    sed -n 's/.*"token":"\([^"]*\)".*/\1/p')
  [ -n "$token" ] || {
    echo "runner: could not get a sign-up token from Forgejo" >&2
    exit 1
  }
  # The runner asks for its address, token, name and labels in turn. The
  # token goes in on standard input, so it is never on a command line.
  signed=0
  printf '%s\n' "$forgejo" "$token" dell-1 "" |
    sudo -n -u "$user" sh -c 'umask 077; cd "$1" &&
      "$2" register --config config.yml >/dev/null' sh "$dir" "$bin" ||
    signed=$?
  unset token
  # The sign-up token works again until it is reset, so reset it now and
  # throw the new one away.
  sudo -n -u "$user" env DOCKER_HOST="unix://$socket" docker exec -u git \
    sutradhar-forgejo forgejo forgejo-cli actions generate-runner-token \
    --scope sutradhar-owner/sutradhar >/dev/null || {
    echo "runner: could not reset the sign-up token; reset it now" >&2
    exit 1
  }
  [ "$signed" -eq 0 ] || {
    echo "runner: the sign-up failed; the token was reset anyway" >&2
    exit 1
  }
fi
sudo -n -u "$user" chmod 0600 "$dir/.runner"
install -m 0755 deploy/runner/loop.sh "$queue/bin/runner-loop"
unit=$(mktemp)
cat >"$unit" <<UNIT
[Unit]
Description=Sutradhar: Forgejo's runner, one job at a time in the queue
After=network-online.target

[Service]
User=$(id -un)
Environment=HOME=$HOME
ExecStart="$queue/bin/runner-loop" "$dir" "$bin" "$queue" "$user" "$socket"
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
UNIT
sudo install -m 0644 "$unit" /etc/systemd/system/sutradhar-runner.service
rm -f "$unit"
sudo systemctl daemon-reload
sudo systemctl enable --quiet --now sutradhar-runner.service
echo "runner: installed; jobs run one at a time through the queue"
