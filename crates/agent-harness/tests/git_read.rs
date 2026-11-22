//! IT-0155-CLONE and IT-0155-AUTHOR (spec 0155, "Push", steps 1 and 2):
//! the commits of a push are read from the harness's own repository with
//! git turned down to what cannot run anything, on real temporary
//! repositories.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]
#![allow(unused_results, reason = "git calls are made for their effect")]

mod common;

use agent_harness::git_read::{GitLimits, GitRefusal, read_new_commits};
use common::repo::{Repo, tripwire};

const LIMITS: GitLimits = GitLimits {
    max_commits: 20,
    max_object_bytes: 1024,
};

fn read(repo: &Repo, main: &str, tip: &str) -> Result<usize, GitRefusal> {
    read_new_commits(repo.path(), main, tip, &LIMITS).map(|c| c.len())
}

#[test]
fn the_new_commits_come_out_oldest_first_with_who_what_and_where() {
    let repo = Repo::new("basic");
    let main = repo.head();
    repo.commit_file(
        "docs/a.md",
        "a\n",
        "docs: a\n\nTask: #640",
        "Codex Agent",
    );
    let tip = repo.commit_file("src/b.rs", "b\n", "feat: b", "Codex Agent");
    let commits = read_new_commits(repo.path(), &main, &tip, &LIMITS).unwrap();
    assert_eq!(commits.len(), 2);
    let first = &commits[0];
    assert_eq!(first.author, "Codex Agent");
    assert_eq!(first.committer, "Codex Agent");
    assert_eq!(first.parents, 1);
    assert_eq!(first.paths, vec!["docs/a.md".to_owned()]);
    assert!(first.message.contains("Task: #640"));
    assert_eq!(commits[1].paths, vec!["src/b.rs".to_owned()]);
}

#[test]
fn another_author_or_committer_is_read_as_it_is() {
    let repo = Repo::new("names");
    let main = repo.head();
    std::fs::write(repo.dir.join("x"), "x\n").unwrap();
    repo.git(&["add", "-A"]);
    repo.git_as(&["commit", "-q", "-m", "x"], "Claude Agent", "Codex Agent");
    let tip = repo.head();
    let c = read_new_commits(repo.path(), &main, &tip, &LIMITS).unwrap();
    assert_eq!(c[0].author, "Claude Agent");
    assert_eq!(c[0].committer, "Codex Agent");
}

#[test]
fn a_rename_lists_both_paths_and_a_delete_lists_its_path() {
    let repo = Repo::new("rename");
    let _ = repo.commit_file(
        "old.md",
        "some text that is long enough\n",
        "a",
        "Codex Agent",
    );
    let main = repo.head();
    repo.git(&["mv", "old.md", "new.md"]);
    repo.git(&["commit", "-q", "-m", "rename"]);
    repo.git(&["rm", "-q", "README"]);
    repo.git(&["commit", "-q", "-m", "delete"]);
    let tip = repo.head();
    let c = read_new_commits(repo.path(), &main, &tip, &LIMITS).unwrap();
    let mut renamed = c[0].paths.clone();
    renamed.sort();
    assert_eq!(renamed, vec!["new.md".to_owned(), "old.md".to_owned()]);
    assert_eq!(c[1].paths, vec!["README".to_owned()]);
}

#[test]
fn a_merge_is_read_with_its_two_parents() {
    let repo = Repo::new("merge");
    let main = repo.head();
    repo.git(&["checkout", "-q", "-b", "side"]);
    repo.commit_file("side.md", "s\n", "side", "Codex Agent");
    repo.git(&["checkout", "-q", "main"]);
    repo.commit_file("main.md", "m\n", "main", "Codex Agent");
    repo.git(&["merge", "-q", "--no-ff", "-m", "merge", "side"]);
    let tip = repo.head();
    let c = read_new_commits(repo.path(), &main, &tip, &LIMITS).unwrap();
    assert!(c.iter().any(|commit| commit.parents == 2));
}

#[test]
fn a_tip_that_does_not_descend_from_main_is_refused() {
    let repo = Repo::new("fork");
    let main = repo.head();
    repo.commit_file("a.md", "a\n", "a", "Codex Agent");
    repo.git(&["checkout", "-q", "-b", "other", &main]);
    let other = repo.commit_file("b.md", "b\n", "b", "Codex Agent");
    let ahead = repo.git(&["rev-parse", "main"]);
    assert_eq!(read(&repo, &ahead, &other), Err(GitRefusal::NotDescendant));
}

#[test]
fn too_many_commits_are_refused() {
    let repo = Repo::new("many");
    let main = repo.head();
    let mut tip = main.clone();
    for i in 0..21 {
        tip = repo.commit_file("f.md", &format!("{i}\n"), "c", "Codex Agent");
    }
    assert_eq!(read(&repo, &main, &tip), Err(GitRefusal::TooManyCommits));
}

#[test]
fn a_symlink_a_submodule_and_a_modules_file_are_refused() {
    let repo = Repo::new("plain");
    let main = repo.head();
    std::os::unix::fs::symlink("/etc/passwd", repo.dir.join("link")).unwrap();
    repo.git(&["add", "-A"]);
    repo.git(&["commit", "-q", "-m", "link"]);
    let link = repo.head();
    assert_eq!(read(&repo, &main, &link), Err(GitRefusal::NotPlainFile));

    let repo = Repo::new("gitlink");
    let main = repo.head();
    let sha = "1111111111111111111111111111111111111111";
    repo.git(&[
        "update-index",
        "--add",
        "--cacheinfo",
        &format!("160000,{sha},sub"),
    ]);
    repo.git(&["commit", "-q", "-m", "sub"]);
    let sub = repo.head();
    assert_eq!(read(&repo, &main, &sub), Err(GitRefusal::NotPlainFile));

    let repo = Repo::new("modules");
    let main = repo.head();
    let tip = repo.commit_file("deep/.gitmodules", "[x]\n", "m", "Codex Agent");
    assert_eq!(read(&repo, &main, &tip), Err(GitRefusal::ModulesFile));
}

#[test]
fn an_object_over_the_size_limit_is_refused() {
    let repo = Repo::new("big");
    let main = repo.head();
    let tip =
        repo.commit_file("big.bin", &"x".repeat(2000), "big", "Codex Agent");
    assert_eq!(read(&repo, &main, &tip), Err(GitRefusal::ObjectTooBig));
    let ok = LIMITS.max_object_bytes + 1;
    let wide = GitLimits {
        max_object_bytes: ok * 4,
        ..LIMITS
    };
    assert!(read_new_commits(repo.path(), &main, &tip, &wide).is_ok());
}

#[test]
fn a_commit_id_that_is_not_one_is_refused_before_git_runs() {
    let repo = Repo::new("ids");
    let good = repo.head();
    for bad in ["--output=/tmp/x", "-h", "main", "HEAD", "", "abc"] {
        assert_eq!(
            read(&repo, &good, bad),
            Err(GitRefusal::BadCommitId),
            "{bad}"
        );
        assert_eq!(
            read(&repo, bad, &good),
            Err(GitRefusal::BadCommitId),
            "{bad}"
        );
    }
}

#[test]
fn a_repository_that_asks_git_to_run_a_program_runs_nothing() {
    let repo = Repo::new("hostile");
    let main = repo.head();
    let (script, marker) = tripwire("ran");
    std::fs::write(
        repo.dir.join(".gitattributes"),
        "* diff=evil filter=evil\n",
    )
    .unwrap();
    let tip = repo.commit_file("a.md", "a\n", "a", "Codex Agent");
    // Only now does the repository ask git to run the program.
    let program = script.display().to_string();
    for (key, value) in [
        ("core.fsmonitor", program.as_str()),
        ("core.hooksPath", "/nonexistent"),
        ("diff.external", program.as_str()),
        ("core.pager", program.as_str()),
        ("core.sshCommand", program.as_str()),
        ("core.editor", program.as_str()),
        ("diff.evil.textconv", program.as_str()),
        ("filter.evil.clean", program.as_str()),
    ] {
        repo.git(&["config", key, value]);
    }
    let _ = std::fs::remove_file(&marker);
    let got = read(&repo, &main, &tip);
    assert_eq!(got, Ok(1));
    assert!(!marker.exists(), "git ran the program");
    let _ = std::fs::remove_file(script);
}

#[test]
fn a_replace_ref_does_not_change_what_is_read() {
    let repo = Repo::new("replace");
    let main = repo.head();
    let b = repo.commit_file("b.md", "b\n", "b", "Codex Agent");
    let tip = repo.commit_file("c.md", "c\n", "c", "Codex Agent");
    repo.git(&["checkout", "-q", "-b", "other", &main]);
    let x = repo.commit_file("x.md", "x\n", "x", "Other Person");
    repo.git(&["replace", "-f", &b, &x]);
    let c = read_new_commits(repo.path(), &main, &tip, &LIMITS).unwrap();
    assert_eq!(c.len(), 2);
    assert_eq!(c[0].paths, vec!["b.md".to_owned()]);
    assert_eq!(c[0].author, "Codex Agent");
}

#[test]
fn a_missing_object_is_never_fetched_from_a_remote_the_repository_names() {
    let repo = Repo::new("lazy");
    let main = repo.head();
    let (script, marker) = tripwire("lazy");
    let tip = repo.commit_file("a.md", "a\n", "a", "Codex Agent");
    let blob = repo.git(&["rev-parse", &format!("{tip}:a.md")]);
    let object = repo
        .dir
        .join(".git/objects")
        .join(&blob[..2])
        .join(&blob[2..]);
    std::fs::remove_file(object).unwrap();
    let url = format!("ext::{}", script.display());
    for (key, value) in [
        ("extensions.partialClone", "evil"),
        ("remote.evil.url", url.as_str()),
        ("remote.evil.promisor", "true"),
        ("protocol.ext.allow", "always"),
    ] {
        repo.git(&["config", key, value]);
    }
    let _ = std::fs::remove_file(&marker);
    assert!(read(&repo, &main, &tip).is_err());
    assert!(!marker.exists(), "git fetched from the remote");
    let _ = std::fs::remove_file(script);
}

#[test]
fn a_commit_with_thousands_of_files_is_read_without_stalling() {
    let repo = Repo::new("thousands");
    let main = repo.head();
    for i in 0..7000 {
        std::fs::write(repo.dir.join(format!("f{i}.txt")), format!("{i}\n"))
            .unwrap();
    }
    repo.git(&["add", "-A"]);
    repo.git(&["commit", "-q", "-m", "many"]);
    let tip = repo.head();
    let c = read_new_commits(repo.path(), &main, &tip, &LIMITS).unwrap();
    assert_eq!(c[0].paths.len(), 7000);
}
