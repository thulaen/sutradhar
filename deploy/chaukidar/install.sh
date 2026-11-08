#!/bin/sh
# Installs or updates the watchman, chaukidar (see its specification),
# and records fresh fingerprints. Run it from the repository's top folder
# on the commit just accepted on main, and again after any other install:
#   deploy/chaukidar/install.sh WATCH_DIR HEADER_FILE
# WATCH_DIR is a private folder holding `list`, the watch list. HEADER_FILE
# holds Forgejo's sign-in header for raising the alarm.
set -eu
forgejo=http://127.0.0.1:3000/api/v1/repos/sutradhar-owner/sutradhar
dir=$(realpath -m "$1")
header=$(realpath -m "$2")
umask 077
sudo systemctl stop chaukidar.timer 2>/dev/null || true
mkdir -p "$dir/bin"
chmod 0700 "$dir"
[ -f "$dir/list" ] || {
  echo "chaukidar: write the watch list $dir/list first" >&2
  exit 1
}
cargo build --quiet --release --package chaukidar
install -m 0755 target/release/chaukidar "$dir/bin/chaukidar"
cat >"$dir/chaukidar.service" <<UNIT
[Unit]
Description=Sutradhar chaukidar: watches what guards the project

[Service]
Type=oneshot
UMask=0077
User=$(id -un)
Environment=HOME=$HOME
Environment=CHAUKIDAR_FORGEJO=$forgejo
Environment=CHAUKIDAR_HEADER=$header
Environment=CHAUKIDAR_GITHUB=thulaen/sutradhar
ExecStart=$dir/bin/chaukidar check $dir/list $dir/sums $dir/state
UNIT
cat >"$dir/chaukidar.timer" <<UNIT
[Unit]
Description=Run chaukidar every five minutes

[Timer]
OnBootSec=2min
OnUnitActiveSec=5min

[Install]
WantedBy=timers.target
UNIT
for unit in chaukidar.service chaukidar.timer; do
  sudo install -m 0644 "$dir/$unit" "/etc/systemd/system/$unit"
done
sudo systemctl daemon-reload
"$dir/bin/chaukidar" record "$dir/list" "$dir/sums"
: >"$dir/state"
sudo systemctl enable --quiet --now chaukidar.timer
echo "chaukidar: installed; it checks every five minutes"
