//! Facts about how containers are set up (task #112): each service of a
//! compose file and each stage of a Dockerfile, so rules can find a
//! container that runs as root, has no limits, or takes an image that is
//! not pinned. Workflows are not read here: `workflows.sealed` already
//! refuses every outside action and container option in them.

use yaml_rust2::parser::{MarkedEventReceiver, Parser};
use yaml_rust2::scanner::Marker;
use yaml_rust2::{Event, Yaml, YamlLoader};

use crate::facts::{Facts, number, text};

#[path = "setup_layout.rs"]
mod layout;
#[path = "setup_values.rs"]
mod values;

use layout::{Layout, is_services};
use values::{flag, has, limit, plain, written};

/// Counts YAML aliases (`*name`): compose files may not use them, since
/// a few lines of them can grow into a huge document when read.
struct Aliases(usize);

impl MarkedEventReceiver for Aliases {
    fn on_event(&mut self, event: Event, _mark: Marker) {
        if matches!(event, Event::Alias(_)) {
            self.0 = self.0.saturating_add(1);
        }
    }
}

fn failure(path: &str, why: &str, facts: &mut Facts) {
    facts.add(
        "parse_failure",
        &[
            ("path", text(path)),
            ("line", number(1)),
            ("reason", text(why)),
        ],
    );
}

/// The `services` map of a compose file, read safely: a file with no
/// top-level `services:` is not a compose file; one that cannot be read,
/// or uses aliases, is a `parse_failure` and blocks.
fn load(path: &str, source: &str, facts: &mut Facts) -> Option<Yaml> {
    // A byte-order mark is not part of the first key.
    let source = source.trim_start_matches('\u{feff}');
    if !source.lines().any(is_services) {
        return None;
    }
    let mut aliases = Aliases(0);
    if let Err(e) = Parser::new_from_str(source).load(&mut aliases, true) {
        failure(path, &format!("not valid YAML: {e}"), facts);
        return None;
    }
    if aliases.0 > 0 {
        failure(path, "compose files may not use YAML aliases", facts);
        return None;
    }
    let Ok(docs) = YamlLoader::load_from_str(source) else {
        failure(path, "not valid YAML", facts);
        return None;
    };
    let services = docs.first().map(|d| d["services"].clone());
    if services.as_ref().is_none_or(|s| s.as_hash().is_none()) {
        failure(path, "`services` is not a map", facts);
        return None;
    }
    services
}

/// Adds the facts of one compose file; other YAML files have none.
pub fn read_compose(path: &str, source: &str, facts: &mut Facts) {
    let Some(services) = load(path, source, facts) else {
        return;
    };
    let Some(services) = services.as_hash() else {
        return;
    };
    let layout = Layout::new(source);
    for (name, service) in services {
        let name = plain(name);
        let at = layout.service(&name);
        let line = number(at);
        let opts = &service["security_opt"];
        let limits = &service["deploy"]["resources"]["limits"];
        let fields = [
            ("path", text(path)),
            ("name", text(name.clone())),
            ("line", line.clone()),
            ("image", text(plain(&service["image"]))),
            ("user", text(plain(&service["user"]))),
            (
                "privileged",
                serde_json::Value::Bool(flag(&service["privileged"])),
            ),
            ("network_mode", text(plain(&service["network_mode"]))),
            (
                "read_only",
                serde_json::Value::Bool(flag(&service["read_only"])),
            ),
            (
                "mem_limit",
                serde_json::Value::Bool(
                    limit(&service["mem_limit"]) || limit(&limits["memory"]),
                ),
            ),
            (
                "pids_limit",
                serde_json::Value::Bool(
                    limit(&service["pids_limit"]) || limit(&limits["pids"]),
                ),
            ),
            (
                "no_new_privileges",
                serde_json::Value::Bool(
                    has(opts, "no-new-privileges:true")
                        || has(opts, "no-new-privileges"),
                ),
            ),
            (
                "cap_drop_all",
                serde_json::Value::Bool(has(&service["cap_drop"], "ALL")),
            ),
        ];
        facts.add("container", &fields);
        read_env(path, &layout, &name, &service["environment"], facts);
    }
}

/// The settings one service sets, in an `environment` map or a list of
/// `NAME=value` items.
fn read_env(
    path: &str,
    layout: &Layout,
    name: &str,
    env: &Yaml,
    facts: &mut Facts,
) {
    let pairs: Vec<(String, String)> = match env {
        Yaml::Hash(map) => {
            map.iter().map(|(k, v)| (plain(k), plain(v))).collect()
        }
        Yaml::Array(items) => items
            .iter()
            .map(|i| {
                let item = plain(i);
                match item.split_once('=') {
                    Some((k, v)) => (k.to_owned(), v.to_owned()),
                    None => (item, String::new()),
                }
            })
            .collect(),
        _ => Vec::new(),
    };
    for (key, value) in pairs {
        let at = layout.setting(name, &key);
        facts.add(
            "container_env",
            &[
                ("path", text(path)),
                ("service", text(name)),
                ("name", text(key)),
                ("written", serde_json::Value::Bool(written(&value))),
                ("line", number(at)),
            ],
        );
    }
}

/// Adds the facts of one Dockerfile: each `FROM` stage, with the last
/// `USER` set in that stage (empty when none is).
pub fn read_dockerfile(path: &str, source: &str, facts: &mut Facts) {
    let mut stages: Vec<(usize, String, String, String)> = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let mut words = line.split_whitespace();
        let first = words.next().unwrap_or_default().to_ascii_uppercase();
        let rest: Vec<&str> = words.collect();
        if first == "FROM" {
            let words: Vec<&str> = rest
                .iter()
                .copied()
                .filter(|w| !w.starts_with("--"))
                .collect();
            let image = words.first().copied().unwrap_or_default();
            // `FROM image AS name` names the stage for later `FROM name`.
            let alias = match words.as_slice() {
                [_, as_word, name, ..]
                    if as_word.eq_ignore_ascii_case("as") =>
                {
                    name.to_ascii_lowercase()
                }
                _ => String::new(),
            };
            stages.push((
                index.saturating_add(1),
                image.to_owned(),
                String::new(),
                alias,
            ));
        } else if first == "USER"
            && let Some(stage) = stages.last_mut()
        {
            rest.first().map_or("", |w| *w).clone_into(&mut stage.2);
        }
    }
    let last = stages.len();
    for (n, (line, image, user, alias)) in stages.into_iter().enumerate() {
        facts.add(
            "docker_stage",
            &[
                ("path", text(path)),
                ("line", number(line)),
                ("image", text(image)),
                ("user", text(user)),
                ("alias", text(alias)),
                ("last", serde_json::Value::Bool(n.saturating_add(1) == last)),
            ],
        );
    }
}

#[cfg(test)]
#[path = "setup_facts_tests.rs"]
mod tests;
