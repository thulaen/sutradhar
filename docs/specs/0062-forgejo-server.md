# 0062: Forgejo on Dell and its settings

Forgejo is the source of truth for code, tasks, milestones and releases
([AGENTS.md sections 2, 8 and 9](../../AGENTS.md)). It runs on Dell
with a rootless build whose exact version and digest are pinned in
[the tool list](../../gate/tools.toml) and used by
[the compose file](../../docker/forgejo/compose.yaml). It is reached
from Dell only on `127.0.0.1:3000`. It may use 1 GB of memory and one
processor, has a read-only file system with a small temporary folder,
and holds no special rights.

## Settings

Forgejo reads settings named `FORGEJO__section__KEY`. In a section
name, `_0X2E_` stands for a dot.

| Setting | Value | Why |
|---|---|---|
| `SUTRADHAR_DOOR_DIR` | the door's folder on Dell | Mounted as `/door`, so Forgejo's hook reaches the door's socket. It is set in the private file `docker/forgejo/.env`, which is never committed. |
| `FORGEJO__database__DB_TYPE` | `sqlite3` | One small file; nothing else to run. |
| `FORGEJO__log__MODE` | `console` | Logs go to the container's log, which is capped in size. |
| `FORGEJO__server__DOMAIN` | `127.0.0.1` | Dell only. |
| `FORGEJO__server__ROOT_URL` | `http://127.0.0.1:3000/` | The address links are built from. |
| `FORGEJO__server__DISABLE_SSH` | `true` | Git goes over HTTP on Dell only. |
| `FORGEJO__server__START_SSH_SERVER` | `false` | No SSH server inside. |
| `FORGEJO__server__LFS_START_SERVER` | `false` | Large-file storage is not used. |
| `FORGEJO__server__OFFLINE_MODE` | `true` | Nothing is fetched from outside, such as avatars. |
| `FORGEJO__security__INSTALL_LOCK` | `true` | The set-up page can never run again. |
| `FORGEJO__service__DISABLE_REGISTRATION` | `true` | No one can make an account. |
| `FORGEJO__service__REQUIRE_SIGNIN_VIEW` | `false` | The owner chose to let the old work list be read without signing in. This repository is private, so reading it always needs sign-in. |
| `FORGEJO__openid__ENABLE_OPENID_SIGNIN` | `false` | No outside sign-in. |
| `FORGEJO__openid__ENABLE_OPENID_SIGNUP` | `false` | No outside sign-up. |
| `FORGEJO__mailer__ENABLED` | `false` | No mail leaves Dell. |
| `FORGEJO__repository__DISABLE_HTTP_GIT` | `false` | Git over HTTP is how pushes arrive. |
| `FORGEJO__repository__FORCE_PRIVATE` | `false` | Left to each repository. |
| `FORGEJO__repository__DEFAULT_PRIVATE` | `private` | New repositories start private. |
| `FORGEJO__repository__DEFAULT_BRANCH` | `main` | All work is on main. |
| `FORGEJO__repository__DEFAULT_REPO_UNITS` | code, tasks, projects, releases, Actions | What a new repository has. |
| `FORGEJO__repository__DISABLED_REPO_UNITS` | pull requests, wiki, outside tasks and wiki, packages | No pull requests: all work is on main. The others are not used. |
| `FORGEJO__repository_0X2E_upload__ENABLED` | `false` | Files come in by git only, through the door. |
| `FORGEJO__packages__ENABLED` | `false` | No package store. |
| `FORGEJO__actions__ENABLED` | `true` | Forgejo Actions runs the runner's jobs ([0060](0060-runner.md)). |
| `FORGEJO__actions__DEFAULT_ACTIONS_URL` | `self` | Actions are looked up on this Forgejo only, never outside. |
| `FORGEJO__mirror__ENABLED` | `false` | forge-sync copies to GitHub instead ([0002](0002-forge-sync.md)). |
| `FORGEJO__federation__ENABLED` | `false` | Talks to no other server. |
| `FORGEJO__cron_0X2E_update_checker__ENABLED` | `false` | Never calls out to check for updates. |

The check `analysis.nirikshak` (rule R05, [0061](0061-nirikshak.md))
refuses a setting in this file that this page does not name.

## Networks

Forgejo joins two networks. Runner jobs join only `runner-jobs`, which
has no way out, so a job reaches Forgejo and nothing else
([0060](0060-runner.md)).
