//! IT-0155-AUTHOR, IT-0155-LOCKS and the task and `main` checks of the
//! push (spec 0155, "Push"), on plain data with no git and no network.
#![allow(clippy::unwrap_used, reason = "tests stop at the first failure")]

use agent_harness::push_check::{
    Agent, Lock, NewCommit, PushFacts, PushRefusal, check_main_push,
};

const LIMIT_MS: u64 = 360_000;
const HEAD: &str = "1111111111111111111111111111111111111111";

fn agent() -> Agent {
    Agent {
        author: "Codex Agent".to_owned(),
        committer: "Codex Agent".to_owned(),
        taken_tasks: vec![640],
    }
}

fn commit(paths: &[&str]) -> NewCommit {
    NewCommit {
        author: "Codex Agent".to_owned(),
        committer: "Codex Agent".to_owned(),
        parents: 1,
        message: "docs: a change\n\nTask: #640\n".to_owned(),
        paths: paths.iter().map(|p| (*p).to_owned()).collect(),
    }
}

fn lock(path: &str, millis_left: u64) -> Lock {
    Lock {
        path: path.to_owned(),
        millis_left,
    }
}

fn run(commits: &[NewCommit], locks: &[Lock]) -> Result<(), PushRefusal> {
    let agent = agent();
    check_main_push(&PushFacts {
        agent: &agent,
        commits,
        locks,
        push_limit_ms: LIMIT_MS,
        main_head: HEAD,
        expected_main: HEAD,
    })
}

#[test]
fn a_clean_push_of_the_agents_own_locked_commit_passes() {
    let locks = [lock("docs/a.md", LIMIT_MS)];
    assert_eq!(run(&[commit(&["docs/a.md"])], &locks), Ok(()));
}

#[test]
fn an_empty_push_and_a_merge_are_refused() {
    let locks = [lock("docs/a.md", LIMIT_MS)];
    assert_eq!(run(&[], &locks), Err(PushRefusal::NoCommits));
    let mut merge = commit(&["docs/a.md"]);
    merge.parents = 2;
    assert_eq!(run(&[merge], &locks), Err(PushRefusal::NotOneParent));
    let mut root = commit(&["docs/a.md"]);
    root.parents = 0;
    assert_eq!(run(&[root], &locks), Err(PushRefusal::NotOneParent));
}

#[test]
fn another_agents_name_or_none_is_refused() {
    let locks = [lock("docs/a.md", LIMIT_MS)];
    for name in ["Claude Agent", "", "codex agent", "Codex Agent "] {
        let mut by = commit(&["docs/a.md"]);
        by.author = name.to_owned();
        assert_eq!(run(&[by], &locks), Err(PushRefusal::WrongAuthor), "{name}");
        let mut via = commit(&["docs/a.md"]);
        via.committer = name.to_owned();
        assert_eq!(
            run(&[via], &locks),
            Err(PushRefusal::WrongCommitter),
            "{name}"
        );
    }
}

#[test]
fn the_task_line_must_be_exactly_one_and_the_agents_own() {
    let locks = [lock("docs/a.md", LIMIT_MS)];
    let bad = [
        ("docs: x\n\nsee #640 for more\n", PushRefusal::NoTaskLine),
        ("docs: x\n\nTask: 640\n", PushRefusal::NoTaskLine),
        ("docs: x\n\nTask: #64a\n", PushRefusal::NoTaskLine),
        (
            "docs: x\n\nTask: #640\nTask: #640\n",
            PushRefusal::NoTaskLine,
        ),
        ("docs: x\n\nTask: #641\n", PushRefusal::TaskNotTaken),
        ("docs: x\n\nTask: #0\n", PushRefusal::NoTaskLine),
    ];
    for (message, want) in bad {
        let mut c = commit(&["docs/a.md"]);
        message.clone_into(&mut c.message);
        assert_eq!(run(&[c], &locks), Err(want), "{message:?}");
    }
}

#[test]
fn every_changed_path_needs_a_live_lock() {
    let locks = [lock("docs/a.md", LIMIT_MS)];
    let two = commit(&["docs/a.md", "docs/b.md"]);
    assert_eq!(run(&[two], &locks), Err(PushRefusal::PathNotLocked));
    // A rename lists both the old and the new path.
    let rename = commit(&["docs/a.md", "docs/new.md"]);
    assert_eq!(run(&[rename], &locks), Err(PushRefusal::PathNotLocked));
    let none = commit(&[]);
    assert_eq!(run(&[none], &locks), Err(PushRefusal::NoPaths));
}

#[test]
fn a_locked_folder_covers_what_is_inside_it_and_nothing_beside_it() {
    let locks = [lock("docs/specs", LIMIT_MS)];
    assert_eq!(run(&[commit(&["docs/specs/0155.md"])], &locks), Ok(()));
    assert_eq!(
        run(&[commit(&["docs/specs-old/x.md"])], &locks),
        Err(PushRefusal::PathNotLocked)
    );
    assert_eq!(
        run(&[commit(&["docs/specsx"])], &locks),
        Err(PushRefusal::PathNotLocked)
    );
}

#[test]
fn a_lock_must_outlast_the_push() {
    let enough = [lock("docs/a.md", LIMIT_MS)];
    let short = [lock("docs/a.md", LIMIT_MS - 1)];
    assert_eq!(run(&[commit(&["docs/a.md"])], &enough), Ok(()));
    assert_eq!(
        run(&[commit(&["docs/a.md"])], &short),
        Err(PushRefusal::LockTooShort)
    );
}

#[test]
fn a_second_live_lock_does_not_rescue_a_short_one() {
    let locks = [lock("docs/a.md", 1), lock("docs", LIMIT_MS)];
    assert_eq!(run(&[commit(&["docs/a.md"])], &locks), Ok(()));
    let locks = [lock("docs/a.md", 1), lock("docs", 2)];
    assert_eq!(
        run(&[commit(&["docs/a.md"])], &locks),
        Err(PushRefusal::LockTooShort)
    );
}

#[test]
fn a_main_that_has_moved_is_refused() {
    let agent = agent();
    let commits = [commit(&["docs/a.md"])];
    let locks = [lock("docs/a.md", LIMIT_MS)];
    let moved = check_main_push(&PushFacts {
        agent: &agent,
        commits: &commits,
        locks: &locks,
        push_limit_ms: LIMIT_MS,
        main_head: HEAD,
        expected_main: "2222222222222222222222222222222222222222",
    });
    assert_eq!(moved, Err(PushRefusal::MainMoved));
}

#[test]
fn a_refusal_never_repeats_a_path_or_a_message() {
    let locks = [lock("docs/a.md", LIMIT_MS)];
    let mut c = commit(&["secret/path.md"]);
    let hidden = ["ghp", "_hidden"].concat();
    c.message = format!("docs: x\n\nTask: #640\n{hidden}\n");
    let refusal = run(&[c], &locks).unwrap_err();
    let text = format!("{refusal:?} {refusal}");
    assert!(
        !text.contains("secret") && !text.contains(&hidden),
        "{text}"
    );
}

#[test]
fn an_expired_lock_never_counts_even_with_no_time_limit() {
    let agent = agent();
    let commits = [commit(&["docs/a.md"])];
    let locks = [lock("docs/a.md", 0)];
    let facts = PushFacts {
        agent: &agent,
        commits: &commits,
        locks: &locks,
        push_limit_ms: 0,
        main_head: HEAD,
        expected_main: HEAD,
    };
    assert_eq!(check_main_push(&facts), Err(PushRefusal::LockTooShort));
}

#[test]
fn a_blank_name_in_the_agents_record_refuses_everything() {
    let locks = [lock("docs/a.md", LIMIT_MS)];
    let mut blank = agent();
    blank.author = "  ".to_owned();
    let mut nameless = commit(&["docs/a.md"]);
    nameless.author = "  ".to_owned();
    let commits = [nameless];
    let facts = PushFacts {
        agent: &blank,
        commits: &commits,
        locks: &locks,
        push_limit_ms: LIMIT_MS,
        main_head: HEAD,
        expected_main: HEAD,
    };
    assert_eq!(check_main_push(&facts), Err(PushRefusal::BlankAgentName));
}

#[test]
fn paths_are_plain_relative_paths_and_locks_are_too() {
    let locks = [lock("docs/a.md", LIMIT_MS)];
    for path in ["docs/a.md/", "/docs/a.md", "docs//a.md", "docs/./a.md"] {
        assert_eq!(
            run(&[commit(&[path])], &locks),
            Err(PushRefusal::BadPath),
            "{path}"
        );
    }
    for bad in ["docs/../src", "", "/", "//", "../src", "docs/./src"] {
        let locks = [lock(bad, LIMIT_MS)];
        let file = commit(&["src/x.rs"]);
        assert_eq!(
            run(&[file], &locks),
            Err(PushRefusal::PathNotLocked),
            "{bad:?}"
        );
    }
    let folder = [lock("docs/", LIMIT_MS)];
    assert_eq!(run(&[commit(&["docs/a.md"])], &folder), Ok(()));
}
