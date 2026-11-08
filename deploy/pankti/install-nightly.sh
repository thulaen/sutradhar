#!/bin/sh
# Installs the nightly noisy-neighbour test (see the pankti
# specification). Run it from the repository's top folder, after
# deploy/pankti/install.sh and deploy/chaukidar/install.sh:
#   deploy/pankti/install-nightly.sh QUEUE_DIR WATCH_DIR HEADER_FILE
# A failed night raises the watchman's alarm through WATCH_DIR's
# chaukidar, using HEADER_FILE, Forgejo's private sign-in header.
set -eu
forgejo=http://127.0.0.1:3000
dir=$(realpath -m "$1")
watch=$(realpath -m "$2")
header=$(realpath -m "$3")
install -m 0755 deploy/pankti/noisy-neighbour.sh "$dir/bin/noisy-neighbour"
cat >"$dir/sutradhar-noisy-neighbour.service" <<UNIT
[Unit]
Description=Sutradhar noisy-neighbour test: a heavy job must not starve Forgejo
OnFailure=sutradhar-noisy-neighbour-alarm.service

[Service]
Type=oneshot
User=$(id -un)
Environment=HOME=$HOME
ExecStart="$dir/bin/noisy-neighbour" "$dir" "curl -sf $forgejo/api/healthz" "curl -sf -o /dev/null $forgejo/explore/repos"
UNIT
cat >"$dir/sutradhar-noisy-neighbour-alarm.service" <<UNIT
[Unit]
Description=Raise the alarm when the noisy-neighbour test fails

[Service]
Type=oneshot
UMask=0077
User=$(id -un)
Environment=HOME=$HOME
Environment=CHAUKIDAR_FORGEJO=$forgejo/api/v1/repos/sutradhar-owner/sutradhar
Environment="CHAUKIDAR_HEADER=$header"
Environment=CHAUKIDAR_GITHUB=thulaen/sutradhar
ExecStart="$watch/bin/chaukidar" raise "the nightly noisy-neighbour test failed"
UNIT
cat >"$dir/sutradhar-noisy-neighbour.timer" <<UNIT
[Unit]
Description=Run the noisy-neighbour test every night

[Timer]
OnCalendar=*-*-* 03:30
Persistent=true

[Install]
WantedBy=timers.target
UNIT
for unit in sutradhar-noisy-neighbour.service \
  sutradhar-noisy-neighbour-alarm.service sutradhar-noisy-neighbour.timer; do
  sudo install -m 0644 "$dir/$unit" "/etc/systemd/system/$unit"
done
sudo systemctl daemon-reload
sudo systemctl enable --quiet --now sutradhar-noisy-neighbour.timer
echo "pankti: the noisy-neighbour test runs every night at 03:30"
