use crate::{github_api, utils};

pub async fn get_langs(owner: &str, repo_name: &str, github_token: &str) {
    println!(
        "Fetching language breakdown for repository {}/{}...",
        owner, repo_name
    );

    let client = github_api::GitHubClient::new(Some(github_token.to_string()));

    match client.get_langs(owner, repo_name).await {
        Ok(lang_hash) => {
            let text = utils::get_percentages_from_lang_hashmap(&lang_hash);
            println!("{}", text);
        }
        Err(e) => {
            eprintln!("Failed to fetch languages: {}", e);
        }
    }
}
