use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RepoStats {
    pub full_name: String,
    pub description: String,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
    pub watchers_count: u64,
    pub subscribers_count: u64,
    pub language: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GithubFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub url: String,
    pub content: String,
    pub encoding: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct BranchCommit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: BranchCommit,
    pub protected: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CommitInfo {
    pub sha: String,
    pub commit: Commit,
    pub author: Option<User>,
    pub committer: Option<User>,
    pub verification: Option<Verification>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Commit {
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CommitAuthor {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    pub signature: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Contributor {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub site_admin: bool,
    pub contributions: u32,
}
