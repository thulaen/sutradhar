//! The checks on a push of `main` (spec 0155, "Push", step 2), on plain
//! data. The harness fills [`PushFacts`] from its own copy of the commits
//! and from the mailbox; nothing here runs git or touches the network.
//! A refusal names the rule, never a path or a message.

use crate::request_check::{NUMBER_MAX, is_path};

/// The agent the socket named, from the owner's account file and the
/// harness's own record of the tasks it took.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Agent {
    /// The author name every new commit must carry.
    pub author: String,
    /// The committer name every new commit must carry.
    pub committer: String,
    /// The tasks the agent took.
    pub taken_tasks: Vec<u64>,
}

/// One new commit in the push, read from the harness's own copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewCommit {
    /// The author name.
    pub author: String,
    /// The committer name.
    pub committer: String,
    /// How many parents it has.
    pub parents: usize,
    /// The whole commit message.
    pub message: String,
    /// Every path the commit changes, a rename listing both paths.
    pub paths: Vec<String>,
}

/// One live mailbox lock the agent holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lock {
    /// The file or folder locked.
    pub path: String,
    /// How long the lock has left, in milliseconds.
    pub millis_left: u64,
}

/// Everything the checks need.
#[derive(Debug, Clone, Copy)]
pub struct PushFacts<'a> {
    /// The agent that asked.
    pub agent: &'a Agent,
    /// The new commits, oldest first.
    pub commits: &'a [NewCommit],
    /// The agent's live locks.
    pub locks: &'a [Lock],
    /// The most time one push may take, in milliseconds.
    pub push_limit_ms: u64,
    /// The head of `main` as Forgejo holds it now.
    pub main_head: &'a str,
    /// The head of `main` the agent expected.
    pub expected_main: &'a str,
}

refusals! {
    /// Why a push was refused.
    PushRefusal {
        /// `main` is not where the agent expected it.
        MainMoved => "main is not where the push expected it",
        /// There is no new commit.
        NoCommits => "the push has no new commit",
        /// A commit is a merge or a root commit.
        NotOneParent => "a commit is a merge or a root commit",
        /// A commit has another author.
        WrongAuthor => "a commit has another author",
        /// A commit has another committer.
        WrongCommitter => "a commit has another committer",
        /// A commit lacks one well-formed `Task: #N` line.
        NoTaskLine => "a commit lacks one well-formed task line",
        /// The named task is not one the agent took.
        TaskNotTaken => "the task is not one the agent took",
        /// A commit changes no path.
        NoPaths => "a commit changes no path",
        /// The agent's record has a blank author or committer name.
        BlankAgentName => "the agent's record has a blank name",
        /// A changed path is not a plain relative path.
        BadPath => "a changed path is not a plain relative path",
        /// A changed path is under no live lock the agent holds.
        PathNotLocked => "a changed path is not under the agent's lock",
        /// A lock over a changed path ends before the push could finish.
        LockTooShort => "a lock ends before the push could finish",
    }
}

/// Checks a push of `main`.
///
/// # Errors
/// The first rule the push breaks.
pub fn check_main_push(facts: &PushFacts<'_>) -> Result<(), PushRefusal> {
    if facts.main_head != facts.expected_main {
        return Err(PushRefusal::MainMoved);
    }
    if facts.commits.is_empty() {
        return Err(PushRefusal::NoCommits);
    }
    facts
        .commits
        .iter()
        .try_for_each(|commit| check_commit(facts, commit))
}

fn check_commit(
    facts: &PushFacts<'_>,
    commit: &NewCommit,
) -> Result<(), PushRefusal> {
    if facts.agent.author.trim().is_empty()
        || facts.agent.committer.trim().is_empty()
    {
        return Err(PushRefusal::BlankAgentName);
    }
    if commit.parents != 1 {
        return Err(PushRefusal::NotOneParent);
    }
    if commit.author != facts.agent.author {
        return Err(PushRefusal::WrongAuthor);
    }
    if commit.committer != facts.agent.committer {
        return Err(PushRefusal::WrongCommitter);
    }
    let task = task_number(&commit.message).ok_or(PushRefusal::NoTaskLine)?;
    if !facts.agent.taken_tasks.contains(&task) {
        return Err(PushRefusal::TaskNotTaken);
    }
    if commit.paths.is_empty() {
        return Err(PushRefusal::NoPaths);
    }
    commit
        .paths
        .iter()
        .try_for_each(|path| check_path(facts, path))
}

/// The task number in the one `Task: #N` line of a message. A line that
/// starts `Task:` but is not well formed, or a second such line, gives
/// none; a mention elsewhere does not count.
fn task_number(message: &str) -> Option<u64> {
    let mut lines = message.lines().filter(|l| l.starts_with("Task:"));
    let line = lines.next()?;
    if lines.next().is_some() {
        return None;
    }
    let digits = line.strip_prefix("Task: #")?;
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let number: u64 = digits.parse().ok()?;
    (1..=NUMBER_MAX).contains(&number).then_some(number)
}

/// Whether a lock on `lock` covers `path`: the same file, or a folder
/// that holds it. A longer name that only starts the same is not covered,
/// and a lock that is not a plain relative path covers nothing.
fn covers(lock: &str, path: &str) -> bool {
    let lock = lock.trim_end_matches('/');
    is_path(lock)
        && (path == lock
            || path
                .strip_prefix(lock)
                .is_some_and(|rest| rest.starts_with('/')))
}

fn check_path(facts: &PushFacts<'_>, path: &str) -> Result<(), PushRefusal> {
    if !is_path(path) {
        return Err(PushRefusal::BadPath);
    }
    let mut covered = false;
    for lock in facts.locks.iter().filter(|l| covers(&l.path, path)) {
        if lock.millis_left > 0 && lock.millis_left >= facts.push_limit_ms {
            return Ok(());
        }
        covered = true;
    }
    if covered {
        Err(PushRefusal::LockTooShort)
    } else {
        Err(PushRefusal::PathNotLocked)
    }
}
