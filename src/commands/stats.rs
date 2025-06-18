use crate::{github_api, utils};

pub async fn get_stats(owner: &str, repo_name: &str, github_token: &str) {
    println!("Fetching stats for repository {}/{}...", owner, repo_name);

    let client = github_api::GitHubClient::new(Some(github_token.to_string()));

    match client.fetch_stats(&owner, &repo_name).await {
        Ok(stats) => {
            let response = utils::format_stats_struct(&stats);
            print!("{}", response);
        }
        Err(e) => {
            eprintln!("Failed to fetch stats: {}", e);
        }
    }
}
