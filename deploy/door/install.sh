#!/bin/sh
# Installs or updates the door (docs/specs/0001-door.md). Run it from the
# repository's top folder, on the commit just accepted on main:
#   deploy/door/install.sh DOOR_DIR FORGEJO_UID
# DOOR_DIR is a private folder on Dell; FORGEJO_UID is the Dell user id
# that Forgejo's container runs as. It uses sudo for the system service.
set -eu
door="$1"
forgejo_uid="$2"
here=$(pwd)
mkdir -p "$door/bin" "$door/run"
cargo build --quiet --release --package aegis-door
install -m 0755 target/release/aegis-door "$door/bin/aegis-door"
if [ ! -d "$door/work/.git" ]; then
  git clone --quiet "$here" "$door/work"
fi
git -C "$door/work" config gc.auto 0
# Only the door's own account and Forgejo's may reach the socket.
chmod 0700 "$door/run"
setfacl -m "u:$forgejo_uid:rwx" "$door/run"
toolchain=$(sed -n 's/^channel = "\(.*\)"$/\1/p' rust-toolchain.toml)
cat >"$door/sutradhar-door.service" <<UNIT
[Unit]
Description=Sutradhar door: checks every push to Forgejo's main

[Service]
User=$(id -un)
Group=$(id -gn)
SupplementaryGroups=$(stat -c %G "$door/..")
Environment=RUSTUP_HOME=${RUSTUP_HOME:?}
# A pushed rust-toolchain.toml must not choose the tools that judge it.
Environment=RUSTUP_TOOLCHAIN=$toolchain
Environment=CARGO_HOME=${CARGO_HOME:?}
Environment=PATH=$(dirname "$(command -v cargo)"):/usr/local/bin:/usr/bin:/bin
ExecStart=$door/bin/aegis-door $door/run/door.sock $door/work
Restart=always
RestartSec=2
MemoryMax=4G
CPUQuota=800%

[Install]
WantedBy=multi-user.target
UNIT
sudo install -m 0644 "$door/sutradhar-door.service" \
  /etc/systemd/system/sutradhar-door.service
sudo systemctl daemon-reload
sudo systemctl enable --quiet sutradhar-door.service
sudo systemctl restart sutradhar-door.service
printf 'SUTRADHAR_DOOR_DIR=%s\n' "$door/run" >docker/forgejo/.env
echo "door: installed; now run deploy/door/install-hook.sh"
