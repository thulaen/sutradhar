use super::{
    Action, COPIED, FROM_GITHUB, MARKER, Rules, Sides, URGENT_TITLE, copy_of,
    is_private, outgoing, plan,
};
use crate::model::{Comment, Issue, Milestone, Release};
use crate::record::Record;

const ME: &str = "owner";

fn milestone(number: u64, title: &str, state: &str) -> Milestone {
    Milestone {
        number,
        title: title.to_owned(),
        description: "d".to_owned(),
        state: state.to_owned(),
    }
}

fn issue(number: u64, title: &str, body: &str) -> Issue {
    Issue {
        number,
        title: title.to_owned(),
        body: body.to_owned(),
        state: "open".to_owned(),
        labels: Vec::new(),
        milestone: Some("Later".to_owned()),
        author: "someone".to_owned(),
    }
}

fn copy_on_github(i: &Issue, number: u64) -> Issue {
    Issue {
        number,
        author: ME.to_owned(),
        body: format!("{}\n\n{MARKER}{} -->\n", i.body, i.number),
        ..i.clone()
    }
}

fn comment(id: u64, on: u64) -> Comment {
    Comment {
        id,
        issue: on,
        author: "pat".to_owned(),
        body: "hello".to_owned(),
    }
}

fn run(
    sides: &Sides,
    record: &Record,
    words: &[String],
) -> Result<Vec<Action>, Vec<String>> {
    plan(
        sides,
        Rules {
            github_user: ME,
            private: words,
            record,
        },
    )
}

#[test]
fn creates_what_github_lacks() {
    let sides = Sides {
        forgejo_milestones: vec![milestone(9, "Phase 1", "open")],
        forgejo_issues: vec![issue(3, "Task", "body")],
        forgejo_releases: vec![Release {
            tag: "v0.1.0".to_owned(),
            name: "v0.1.0".to_owned(),
            body: "notes".to_owned(),
            prerelease: false,
        }],
        ..Sides::default()
    };
    let got = run(&sides, &Record::default(), &[]).unwrap();
    assert!(matches!(&got[0], Action::CreateMilestone(_)));
    assert!(matches!(&got[1], Action::CreateIssue(i)
        if i.body == format!("body\n\n{MARKER}3 -->")));
    assert!(matches!(&got[2], Action::CreateRelease(_)));
}

#[test]
fn updates_only_what_changed() {
    let task = issue(3, "Task", "body");
    let mut stale = copy_on_github(&task, 40);
    stale.state = "closed".to_owned();
    let same = issue(4, "Same", "x");
    let sides = Sides {
        forgejo_milestones: vec![
            milestone(9, "P1", "closed"),
            milestone(10, "P2", "open"),
        ],
        github_milestones: vec![
            milestone(1, "P1", "open"),
            milestone(2, "P2", "open"),
        ],
        forgejo_issues: vec![task, same.clone()],
        github_issues: vec![stale, copy_on_github(&same, 41)],
        ..Sides::default()
    };
    let got = run(&sides, &Record::default(), &[]).unwrap();
    assert_eq!(got.len(), 2, "{got:?}");
    assert!(matches!(&got[0], Action::UpdateMilestone(1, _)));
    assert!(matches!(&got[1], Action::UpdateIssue(40, i) if i.state == "open"));
}

#[test]
fn only_the_syncs_own_issues_are_copies() {
    let task = issue(3, "Task", "body");
    let mut fake = copy_on_github(&task, 40);
    fake.author = "stranger".to_owned();
    assert_eq!(copy_of(&fake, ME), None);
    let sides = Sides {
        forgejo_issues: vec![task],
        github_issues: vec![fake],
        ..Sides::default()
    };
    let got = run(&sides, &Record::default(), &[]).unwrap();
    assert!(matches!(&got[0], Action::CreateIssue(_)));
    assert!(matches!(&got[1], Action::Import(i) if i.number == 40));
}

#[test]
fn bad_markers_are_not_copies() {
    let mut odd = copy_on_github(&issue(3, "T", "b"), 40);
    odd.body = format!("{MARKER}x -->");
    assert_eq!(copy_of(&odd, ME), None);
    odd.body = format!("{MARKER}3");
    assert_eq!(copy_of(&odd, ME), None);
    odd.body = "none".to_owned();
    assert_eq!(copy_of(&odd, ME), None);
}

#[test]
fn what_came_from_github_and_the_urgent_task_stay_home() {
    let note = format!("x {FROM_GITHUB}8 -->");
    let sides = Sides {
        forgejo_issues: vec![
            issue(5, URGENT_TITLE, "problems"),
            issue(6, "[From GitHub #8] Idea", &note),
        ],
        ..Sides::default()
    };
    assert_eq!(run(&sides, &Record::default(), &[]).unwrap().len(), 0);
}

#[test]
fn the_record_stops_repeats() {
    let mut copied = issue(7, "Old", "b");
    copied.labels = vec![COPIED.to_owned()];
    let sides = Sides {
        forgejo_issues: vec![issue(3, "Task", "body")],
        github_issues: vec![
            issue(6, "New", "x"),
            copied,
            issue(9, "Seen", "y"),
        ],
        github_comments: vec![comment(2, 6)],
        ..Sides::default()
    };
    let record = Record {
        copied: vec![3],
        imported: vec![9],
        comments: vec![2],
        ..Record::default()
    };
    let got = run(&sides, &record, &[]).unwrap();
    assert_eq!(got.len(), 1, "{got:?}");
    assert!(matches!(&got[0], Action::Import(i) if i.number == 6));
}

#[test]
fn comments_go_to_the_right_task() {
    let task = issue(3, "Task", "body");
    let brought =
        issue(12, "[From GitHub #6] Idea", &format!("{FROM_GITHUB}6 -->"));
    let mine = Comment {
        author: ME.to_owned(),
        ..comment(3, 40)
    };
    let idea = Issue {
        labels: vec![COPIED.to_owned()],
        ..issue(6, "Idea", "x")
    };
    let sides = Sides {
        forgejo_issues: vec![task.clone(), brought],
        github_issues: vec![copy_on_github(&task, 40), idea],
        github_comments: vec![
            comment(1, 40),
            comment(2, 6),
            mine,
            comment(4, 99),
        ],
        ..Sides::default()
    };
    let got = run(&sides, &Record::default(), &[]).unwrap();
    assert_eq!(got.len(), 2, "{got:?}");
    assert!(matches!(&got[0], Action::ImportComment(3, c) if c.id == 1));
    assert!(matches!(&got[1], Action::ImportComment(12, c) if c.id == 2));
}

#[test]
fn anything_private_stops_the_whole_run() {
    let words = vec!["MyForum".to_owned()];
    let mut labelled = issue(4, "Fine", "fine");
    labelled.labels = vec!["about-myforum".to_owned()];
    let sides = Sides {
        forgejo_milestones: vec![milestone(9, "Phase 1", "open")],
        forgejo_issues: vec![issue(3, "Fine too", "ok"), labelled],
        ..Sides::default()
    };
    let blocked = run(&sides, &Record::default(), &words).unwrap_err();
    assert_eq!(blocked, ["task #4 looks private, so nothing was sent"]);
}

#[test]
fn names_every_kind_of_change() {
    let i = issue(3, "T", "b");
    let m = milestone(1, "M", "open");
    let r = Release {
        tag: "v1".to_owned(),
        name: "n".to_owned(),
        body: "b".to_owned(),
        prerelease: false,
    };
    let all = [
        Action::CreateMilestone(m.clone()),
        Action::UpdateMilestone(1, m),
        Action::CreateIssue(i.clone()),
        Action::UpdateIssue(2, i.clone()),
        Action::CreateRelease(r),
        Action::Import(i),
        Action::ImportComment(3, comment(1, 2)),
    ];
    let names: Vec<String> = all.iter().map(super::name).collect();
    assert_eq!(names[4], "release v1");
    assert_eq!(names[6], "GitHub comment 1");
    assert_eq!(outgoing(&all[5]).len(), 0);
    assert_eq!(outgoing(&all[3]).len(), 3);
}

#[test]
fn spots_private_text() {
    let words = vec!["secretword".to_owned(), " ".to_owned()];
    assert!(is_private(&["has SecretWord".to_owned()], &words));
    assert!(is_private(&[["/srv", "/data"].concat()], &[]));
    assert!(!is_private(&["plain".to_owned()], &words));
}

#[test]
fn alarms_are_never_copied_either_way() {
    let sides = Sides {
        forgejo_issues: vec![issue(70, super::ALARM_TITLE, "changed")],
        github_issues: vec![issue(71, super::ALARM_TITLE, "changed")],
        ..Sides::default()
    };
    assert_eq!(run(&sides, &Record::default(), &[]).unwrap().len(), 0);
}
