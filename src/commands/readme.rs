use crate::{github_api, utils};

pub async fn get_readme(owner: &str, repo_name: &str, github_token: &str) {
    println!(
        "Fetching README.md for repository {}/{}...",
        owner, repo_name
    );

    match github_api::fetch_readme(owner, repo_name, github_token).await {
        Ok(file) => {
            let text = utils::parse_github_file_to_readme_text(&file);
            println!("{}", text);
        },
        Err(e) => {
            eprintln!("Failed to fetch stats: {}", e);
        }
    }
}
