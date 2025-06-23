use crate::{github_api, utils};

pub async fn get_commits(
    owner: &str,
    repo_name: &str,
    github_token: &str,
    per_page: &i32,
    page: &i32,
    branch: &str,
    author: &str,
) {
    let client = github_api::GitHubClient::new(Some(github_token.to_string()));
    let mut load_msg: String = format!(
        "Fetching {} commits on page {} in branch '{}' for repository {}/{}",
        per_page, page, branch, owner, repo_name
    );

    if !author.is_empty() {
        load_msg.push_str(&format!(" from author {}", author));
    }

    println!("{}...", load_msg);

    match client
        .get_commits(owner, repo_name, per_page, page, branch, author)
        .await
    {
        Ok(commits) => {
            let response = utils::info_display::get_info_from_commits(&commits);
            print!("{}", response);
        }
        Err(e) => {
            eprintln!("Failed to fetch stats: {}", e);
        }
    }
}
