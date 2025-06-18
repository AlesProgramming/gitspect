use crate::{github_api, utils};

pub async fn get_readme(owner: &str, repo_name: &str, github_token: &str) {
    println!(
        "Fetching README.md for repository {}/{}...",
        owner, repo_name
    );

    let client = github_api::GitHubClient::new(Some(github_token.to_string()));

    match client.fetch_readme(owner, repo_name).await {
        Ok(file) => {
            let text = utils::parse_github_file_to_readme_text(&file);
            println!("{}", text);
        },
        Err(e) => {
            eprintln!("Failed to fetch stats: {}", e);
        }
    }
}
