use crate::{github_api, utils};

pub async fn get_branches(owner: &str, repo_name: &str, github_token: &str) {
    println!(
        "Fetching branches for repository {}/{}...",
        owner, repo_name
    );

    match github_api::get_branches(owner, repo_name, github_token).await {
        Ok(branches) => {
            let text = utils::get_info_from_branches(&branches);
            println!("{}", text);
        }
        Err(e) => {
            eprintln!("Failed to fetch languages: {}", e);
        }
    }
}
