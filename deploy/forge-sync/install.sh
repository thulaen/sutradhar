#!/bin/sh
# Installs or updates forge-sync (docs/specs/0002-forge-sync.md). Run it
# from the repository's top folder, on the commit just accepted on main:
#   deploy/forge-sync/install.sh SYNC_DIR HELPER TOKEN_FILE GITLEAKS WORDS...
# SYNC_DIR is a private folder on Dell. HELPER is a git credential helper
# for Forgejo, and TOKEN_FILE holds a Forgejo token that may write tasks.
# GITLEAKS is the pinned gitleaks program. WORDS are private files of
# words that must never reach GitHub; at least one is needed.
set -eu
dir=$(realpath -m "$1")
helper="$2"
token="$3"
gitleaks="$4"
shift 4
[ "$#" -gt 0 ] || {
  echo "forge-sync: give at least one private word file" >&2
  exit 1
}
forgejo=http://127.0.0.1:3000
repo=sutradhar-owner/sutradhar
github=thulaen/sutradhar
umask 077
mkdir -p "$dir/bin"
chmod 0700 "$dir"
cargo build --quiet --release --package forge-sync
install -m 0755 target/release/forge-sync "$dir/bin/forge-sync"
printf 'Authorization: token %s\n' "$(cat "$token")" >"$dir/header"
chmod 0600 "$dir/header"
cat /dev/null "$@" >"$dir/words"
chmod 0600 "$dir/words"
if [ ! -d "$dir/mirror" ]; then
  git -c credential.helper="$helper" clone --quiet --bare \
    "$forgejo/$repo.git" "$dir/mirror"
  git -C "$dir/mirror" remote rename origin forgejo
  git -C "$dir/mirror" config "credential.$forgejo.helper" "$helper"
  git -C "$dir/mirror" remote add github "https://github.com/$github.git"
fi
cat >"$dir/forge-sync.service" <<UNIT
[Unit]
Description=Sutradhar forge-sync: copies Forgejo to GitHub

[Service]
Type=oneshot
UMask=0077
User=$(id -un)
SupplementaryGroups=$(stat -c %G "$dir/..")
Environment=HOME=$HOME
Environment=FORGE_SYNC_FORGEJO=$forgejo/api/v1/repos/$repo
Environment=FORGE_SYNC_HEADER=$dir/header
Environment=FORGE_SYNC_GITHUB=$github
Environment=FORGE_SYNC_MIRROR=$dir/mirror
Environment=FORGE_SYNC_PRIVATE=$dir/words
Environment=FORGE_SYNC_RECORD=$dir/record
Environment=FORGE_SYNC_GITHUB_USER=$(gh api user --jq .login)
Environment=FORGE_SYNC_GITLEAKS=$gitleaks
TimeoutStartSec=30min
ExecStart=$dir/bin/forge-sync once
UNIT
cat >"$dir/forge-sync.timer" <<UNIT
[Unit]
Description=Run forge-sync every minute

[Timer]
OnBootSec=1min
OnUnitActiveSec=1min

[Install]
WantedBy=timers.target
UNIT
for unit in forge-sync.service forge-sync.timer; do
  sudo install -m 0644 "$dir/$unit" "/etc/systemd/system/$unit"
done
sudo systemctl daemon-reload
sudo systemctl enable --quiet --now forge-sync.timer
echo "forge-sync: installed; it runs every minute"
