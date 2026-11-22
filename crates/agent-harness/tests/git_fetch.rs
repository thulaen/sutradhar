//! IT-0155-CLONE (spec 0155, "Push", step 1): the harness fetches the
//! one named commit from the agent's clone into its own repository with
//! nothing from the clone executed, on real temporary repositories.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]
#![allow(unused_results, reason = "git calls are made for their effect")]

mod common;

use agent_harness::agent::AgentName;
use agent_harness::git_fetch::{FetchLimits, FetchRefusal, fetch_commit, tidy};
use common::repo::{Repo, tripwire};

fn me() -> AgentName {
    AgentName::new("codex").unwrap()
}

const LIMITS: FetchLimits = FetchLimits {
    max_repo_bytes: 10 * 1024 * 1024,
};

/// The harness's own empty repository.
fn harness(name: &str) -> Repo {
    let repo = Repo::new(name);
    repo.git(&["checkout", "-q", "--detach"]);
    repo.git(&["branch", "-q", "-D", "main"]);
    repo
}

fn has(repo: &Repo, commit: &str) -> bool {
    std::process::Command::new("git")
        .arg("-C")
        .arg(&repo.dir)
        .args(["cat-file", "-e", &format!("{commit}^{{commit}}")])
        .status()
        .unwrap()
        .success()
}

#[test]
fn the_named_commit_and_its_history_come_into_the_harness_repository() {
    let clone = Repo::new("agent-clone");
    let tip = clone.commit_file("a.md", "a\n", "a", "Codex Agent");
    let own = harness("own-a");
    assert!(!has(&own, &tip));
    let got = fetch_commit(&own.dir, &clone.dir, &tip, &me(), LIMITS);
    assert_eq!(got, Ok(()));
    assert!(has(&own, &tip));
}

#[test]
fn a_commit_on_no_branch_the_agent_offers_is_refused() {
    let clone = Repo::new("agent-dangling");
    let _tip = clone.commit_file("a.md", "a\n", "a", "Codex Agent");
    let first = clone.git(&["rev-parse", "HEAD~1"]);
    // Move the branch back so the newest commit hangs off no branch.
    let hidden = clone.head();
    clone.git(&["reset", "-q", "--hard", &first]);
    let own = harness("own-b");
    let got = fetch_commit(&own.dir, &clone.dir, &hidden, &me(), LIMITS);
    assert_eq!(got, Err(FetchRefusal::CommitNotOffered));
}

#[test]
fn bad_commit_ids_and_clone_paths_are_refused_before_git_runs() {
    let clone = Repo::new("agent-bad");
    let tip = clone.head();
    let own = harness("own-c");
    for bad in ["", "main", "--upload-pack=x", "abc"] {
        let got = fetch_commit(&own.dir, &clone.dir, bad, &me(), LIMITS);
        assert_eq!(got, Err(FetchRefusal::BadCommitId), "{bad}");
    }
    let relative = std::path::Path::new("relative/clone");
    let got = fetch_commit(&own.dir, relative, &tip, &me(), LIMITS);
    assert_eq!(got, Err(FetchRefusal::BadClonePath));
    let odd = std::path::Path::new("/tmp/a b;rm -rf x");
    let got = fetch_commit(&own.dir, odd, &tip, &me(), LIMITS);
    assert_eq!(got, Err(FetchRefusal::BadClonePath));
    let missing = std::path::Path::new("/nonexistent/clone");
    let got = fetch_commit(&own.dir, missing, &tip, &me(), LIMITS);
    assert_eq!(got, Err(FetchRefusal::BadClonePath));
}

#[test]
fn a_clone_that_asks_git_to_run_a_program_runs_nothing() {
    let clone = Repo::new("agent-hostile");
    let tip = clone.commit_file("a.md", "a\n", "a", "Codex Agent");
    let (script, marker) = tripwire("fetch");
    let program = script.display().to_string();
    for (key, value) in [
        ("uploadpack.packObjectsHook", program.as_str()),
        ("core.fsmonitor", program.as_str()),
        ("core.hooksPath", "/nonexistent"),
        ("core.alternateRefsCommand", program.as_str()),
        ("core.sshCommand", program.as_str()),
        ("core.gitProxy", program.as_str()),
        ("pack.threads", "1"),
    ] {
        clone.git(&["config", key, value]);
    }
    let _ = std::fs::remove_file(&marker);
    let own = harness("own-d");
    let got = fetch_commit(&own.dir, &clone.dir, &tip, &me(), LIMITS);
    assert_eq!(got, Ok(()));
    assert!(!marker.exists(), "git ran the program");
    let _ = std::fs::remove_file(script);
}

/// The raw bytes of a hex object id.
fn raw_id(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).unwrap();
            u8::from_str_radix(text, 16).unwrap()
        })
        .collect()
}

#[test]
fn a_malformed_object_is_refused_when_it_comes_in() {
    let clone = Repo::new("agent-bad-tree");
    // A tree with an entry named `.git` whose blob exists: every object is
    // present, so only the check of each object in full can refuse it.
    let readme = clone.git(&["rev-parse", "HEAD:README"]);
    let mut bytes = b"100644 .git\0".to_vec();
    bytes.extend_from_slice(&raw_id(&readme));
    let pid = std::process::id();
    let raw = std::env::temp_dir().join(format!("bad-tree-{pid}"));
    std::fs::write(&raw, bytes).unwrap();
    let path = raw.to_str().unwrap();
    let tree =
        clone.git(&["hash-object", "-w", "-t", "tree", "--literally", path]);
    let commit = clone.git(&["commit-tree", &tree, "-m", "bad"]);
    clone.git(&["update-ref", "refs/heads/main", &commit]);
    let own = harness("own-e");
    let got = fetch_commit(&own.dir, &clone.dir, &commit, &me(), LIMITS);
    assert_eq!(got, Err(FetchRefusal::FetchFailed));
    assert!(!has(&own, &commit));
    let _ = std::fs::remove_file(raw);
}

#[test]
fn a_repository_over_the_size_limit_is_refused() {
    let clone = Repo::new("agent-big");
    let tip = clone.commit_file("big.bin", &"x".repeat(200_000), "big", "A");
    let own = harness("own-f");
    let small = FetchLimits {
        max_repo_bytes: 1024,
    };
    let got = fetch_commit(&own.dir, &clone.dir, &tip, &me(), small);
    assert_eq!(got, Err(FetchRefusal::RepoTooBig));
}

/// How many bytes of objects the repository holds.
fn size(repo: &Repo) -> u64 {
    let out = repo.git(&["count-objects", "-v"]);
    out.lines()
        .filter_map(|l| l.split_once(": "))
        .filter(|(k, _)| matches!(*k, "size" | "size-pack"))
        .filter_map(|(_, v)| v.trim().parse::<u64>().ok())
        .sum()
}

#[test]
fn each_agent_has_its_own_refs_and_cannot_remove_another_agents() {
    let a = Repo::new("agent-a");
    let b = Repo::new("agent-b");
    let tip_a = a.commit_file("a.md", "a\n", "a", "A");
    let tip_b = b.commit_file("b.md", "b\n", "b", "B");
    let own = harness("own-g");
    let (one, two) = (me(), AgentName::new("claude").unwrap());
    assert_eq!(fetch_commit(&own.dir, &a.dir, &tip_a, &one, LIMITS), Ok(()));
    assert_eq!(fetch_commit(&own.dir, &b.dir, &tip_b, &two, LIMITS), Ok(()));
    assert!(has(&own, &tip_a) && has(&own, &tip_b));
    // The second agent cannot be offered the first agent's commit.
    let got = fetch_commit(&own.dir, &b.dir, &tip_a, &two, LIMITS);
    assert_eq!(got, Err(FetchRefusal::CommitNotOffered));
    // The first agent's refs survive the second agent's refused fetch.
    let again = fetch_commit(&own.dir, &a.dir, &tip_a, &one, LIMITS);
    assert_eq!(again, Ok(()));
}

#[test]
fn a_refused_fetch_leaves_nothing_behind() {
    let clone = Repo::new("agent-too-big");
    let tip = clone.commit_file("big.bin", &"x".repeat(200_000), "big", "A");
    let own = harness("own-h");
    let before = size(&own);
    let small = FetchLimits {
        max_repo_bytes: 1024,
    };
    let got = fetch_commit(&own.dir, &clone.dir, &tip, &me(), small);
    assert_eq!(got, Err(FetchRefusal::RepoTooBig));
    assert!(!has(&own, &tip));
    assert!(size(&own) <= before + 8, "objects were left behind");
}

#[test]
fn tidy_removes_the_objects_of_a_deleted_branch() {
    let clone = Repo::new("agent-tidy");
    let tip = clone.commit_file("t.bin", &"y".repeat(100_000), "t", "A");
    let own = harness("own-i");
    assert_eq!(
        fetch_commit(&own.dir, &clone.dir, &tip, &me(), LIMITS),
        Ok(())
    );
    own.git(&["update-ref", "-d", "refs/agent/codex/heads/main"]);
    assert!(has(&own, &tip));
    tidy(&own.dir);
    assert!(!has(&own, &tip));
}

#[test]
fn a_link_in_the_path_a_git_file_and_borrowed_objects_are_refused() {
    let real = Repo::new("agent-real");
    let tip = real.head();
    let own = harness("own-j");
    // A link to a real clone.
    let link = std::env::temp_dir()
        .join(format!("harness-link-{}", std::process::id()));
    let _ = std::fs::remove_file(&link);
    std::os::unix::fs::symlink(&real.dir, &link).unwrap();
    let got = fetch_commit(&own.dir, &link, &tip, &me(), LIMITS);
    assert_eq!(got, Err(FetchRefusal::BadClonePath));
    let _ = std::fs::remove_file(link);
    // A `.git` file that points somewhere else.
    let filed = Repo::new("agent-gitfile");
    std::fs::remove_dir_all(filed.dir.join(".git")).unwrap();
    std::fs::write(
        filed.dir.join(".git"),
        format!("gitdir: {}\n", real.dir.join(".git").display()),
    )
    .unwrap();
    let got = fetch_commit(&own.dir, &filed.dir, &tip, &me(), LIMITS);
    assert_eq!(got, Err(FetchRefusal::UnsafeClone));
    // A clone that borrows objects from another repository.
    let borrower = Repo::new("agent-borrower");
    let info = borrower.dir.join(".git/objects/info");
    std::fs::create_dir_all(&info).unwrap();
    let objects = real.dir.join(".git/objects");
    std::fs::write(info.join("alternates"), format!("{}\n", objects.display()))
        .unwrap();
    let got = fetch_commit(&own.dir, &borrower.dir, &tip, &me(), LIMITS);
    assert_eq!(got, Err(FetchRefusal::UnsafeClone));
}

#[test]
fn a_shallow_clone_does_not_bring_in_half_a_history() {
    let source = Repo::new("agent-source");
    source.commit_file("a.md", "a\n", "a", "A");
    source.commit_file("b.md", "b\n", "b", "A");
    let shallow = std::env::temp_dir()
        .join(format!("harness-shallow-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&shallow);
    let url = format!("file://{}", source.dir.display());
    let status = std::process::Command::new("git")
        .args(["clone", "-q", "--depth", "1", &url])
        .arg(&shallow)
        .status()
        .unwrap();
    assert!(status.success());
    let tip = source.head();
    let own = harness("own-k");
    let got = fetch_commit(&own.dir, &shallow, &tip, &me(), LIMITS);
    let _ = std::fs::remove_dir_all(&shallow);
    assert!(got.is_err(), "a shallow clone was accepted: {got:?}");
}
