use crate::{github_api, utils};

pub async fn get_contributors(owner: &str, repo_name: &str, github_token: &str, per_page: &i32, page: &i32) {
    println!(
        "Fetching {} contributors for repository {}/{} on page {}...",
        per_page, owner, repo_name, page
    );

    let client = github_api::GitHubClient::new(Some(github_token.to_string()));

    match client.get_contributors(owner, repo_name, per_page, page).await {
        Ok(contributors_list) => {
            let text = utils::info_display::get_info_from_contributors(&contributors_list);
            println!("{}", text);
        }
        Err(e) => {
            eprintln!("Failed to fetch languages: {}", e);
        }
    }
}
