#!/bin/sh
# Installs or updates pankti, the heavy-job queue (see its
# specification). Run it from the repository's top folder on the commit
# just accepted on main:
#   deploy/pankti/install.sh QUEUE_DIR GROUP
# QUEUE_DIR holds the lock files and the program. Every user in GROUP
# may queue jobs. Heavy jobs then run as
#   QUEUE_DIR/bin/pankti run [--both] -- COMMAND...
# It also names the queue in this checkout's setting `sutradhar.queue`,
# which the hooks and the door read.
set -eu
dir=$(realpath -m "$1")
group=$2
mkdir -p "$dir/bin"
sudo chgrp "$group" "$dir"
chmod 2770 "$dir"
for name in line place-1 place-2; do
  [ -f "$dir/$name" ] || : >"$dir/$name"
  chmod 0660 "$dir/$name"
done
cargo build --quiet --release --package pankti
install -m 0755 target/release/pankti "$dir/bin/pankti.new"
mv "$dir/bin/pankti.new" "$dir/bin/pankti"
cat >"$dir/sutradhar-heavy.slice" <<UNIT
[Unit]
Description=Sutradhar heavy jobs: two places of 4 GB and 8 CPU threads

[Slice]
MemoryMax=8G
MemorySwapMax=0
CPUQuota=1600%
UNIT
sudo install -m 0644 "$dir/sutradhar-heavy.slice" \
  /etc/systemd/system/sutradhar-heavy.slice
sudo systemctl daemon-reload
git config sutradhar.queue "$dir/bin/pankti"
echo "pankti: installed; heavy jobs run through $dir/bin/pankti"
