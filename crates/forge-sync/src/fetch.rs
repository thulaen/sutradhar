//! Reading both sides, and copying the code.

use serde_json::Value;

use crate::model;
use crate::plan::Sides;
use crate::run::Run;

/// Where things are.
#[derive(Debug, Clone)]
pub struct Config {
    /// Forgejo's API address for the repository, such as
    /// `http://127.0.0.1:3000/api/v1/repos/OWNER/NAME`.
    pub forgejo: String,
    /// A private file holding the `Authorization:` header for Forgejo.
    pub header_file: String,
    /// The GitHub repository, such as `OWNER/NAME`.
    pub github: String,
    /// A bare git repository with remotes `forgejo` and `github`.
    pub mirror: String,
    /// The sync's record (see [`crate::record`]). It is made if
    /// missing.
    pub record: String,
    /// The sync's own GitHub account.
    pub github_user: String,
    /// The pinned gitleaks program.
    pub gitleaks: String,
    /// Words that must never leave Dell.
    pub private: Vec<String>,
}

const PAGES: u32 = 100;

/// Every item of a paged Forgejo list, as one JSON list.
///
/// # Errors
/// When curl fails or a page is not a JSON list.
pub fn forgejo_list(
    run: &dyn Run,
    config: &Config,
    path: &str,
) -> Result<String, String> {
    let header = format!("@{}", config.header_file);
    collect(|page| {
        let url = format!("{}/{path}&limit=50&page={page}", config.forgejo);
        run.run("curl", &["-sS", "--fail", "-H", &header, &url], &[])
    })
}

/// Every item of a paged GitHub list, as one JSON list.
///
/// # Errors
/// When gh fails or a page is not a JSON list.
pub fn github_list(
    run: &dyn Run,
    config: &Config,
    path: &str,
) -> Result<String, String> {
    collect(|page| {
        let url =
            format!("repos/{}/{path}&per_page=100&page={page}", config.github);
        run.run("gh", &["api", &url], &[])
    })
}

fn collect(
    mut page: impl FnMut(u32) -> Result<String, String>,
) -> Result<String, String> {
    let mut all = Vec::new();
    for n in 1..=PAGES {
        let text = page(n)?;
        match serde_json::from_str::<Value>(&text) {
            Ok(Value::Array(items)) if items.is_empty() => {
                return Ok(Value::Array(all).to_string());
            }
            Ok(Value::Array(items)) => all.extend(items),
            _ => return Err(format!("page {n} is not a JSON list")),
        }
    }
    Err(format!("more than {PAGES} pages"))
}

/// What each side holds.
///
/// # Errors
/// When either side cannot be read.
pub fn sides(run: &dyn Run, config: &Config) -> Result<Sides, String> {
    let f = |path: &str| forgejo_list(run, config, path);
    let g = |path: &str| github_list(run, config, path);
    Ok(Sides {
        forgejo_milestones: model::milestones(
            &f("milestones?state=all")?,
            "id",
        )?,
        github_milestones: model::milestones(
            &g("milestones?state=all")?,
            "number",
        )?,
        forgejo_issues: model::issues(&f("issues?state=all&type=issues")?)?,
        github_issues: model::issues(&g("issues?state=all")?)?,
        forgejo_releases: model::releases(&f("releases?draft=false")?)?,
        github_releases: model::releases(&g("releases?")?)?,
        github_comments: model::comments(&g("issues/comments?")?)?,
    })
}

/// Brings `main` and every tag from Forgejo into the mirror.
///
/// # Errors
/// When git fails.
pub fn fetch_code(run: &dyn Run, config: &Config) -> Result<(), String> {
    let fetch = [
        "-C",
        &config.mirror,
        "fetch",
        "--quiet",
        "--prune",
        "forgejo",
        "+refs/heads/main:refs/heads/main",
        "+refs/tags/*:refs/tags/*",
    ];
    run.run("git", &fetch, &[]).map(|_| ())
}

/// Sends `main`, and only the tags on `main`, from the mirror to GitHub.
/// Call it only after [`crate::guard::code`] has passed. Returns the
/// tags sent.
///
/// # Errors
/// When git fails.
pub fn push_code(
    run: &dyn Run,
    config: &Config,
) -> Result<Vec<String>, String> {
    let m = config.mirror.as_str();
    let tags = run.run("git", &["-C", m, "tag", "--merged", "main"], &[])?;
    let refs: Vec<String> = tags
        .lines()
        .filter(|t| !t.trim().is_empty())
        .map(|t| format!("refs/tags/{}", t.trim()))
        .collect();
    let mut push = vec!["-C", m, "push", "--quiet", "github", "main"];
    push.extend(refs.iter().map(String::as_str));
    let _out = run.run("git", &push, &[])?;
    Ok(tags
        .lines()
        .map(|t| t.trim().to_owned())
        .filter(|t| !t.is_empty())
        .collect())
}

/// Sends a JSON body to Forgejo's API with POST.
///
/// # Errors
/// When curl fails.
pub fn forgejo_post(
    run: &dyn Run,
    config: &Config,
    path: &str,
    body: &serde_json::Value,
) -> Result<(), String> {
    let header = format!("@{}", config.header_file);
    let url = format!("{}/{path}", config.forgejo);
    let args = [
        "-sS",
        "--fail",
        "-X",
        "POST",
        "-H",
        &header,
        "-H",
        "Content-Type: application/json",
        "--data-binary",
        "@-",
        &url,
    ];
    run.run("curl", &args, body.to_string().as_bytes())
        .map(|_| ())
}
