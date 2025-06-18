use crate::{
    github_api::{self},
    utils,
};
use open;

pub async fn get_branches(
    owner: &str,
    repo_name: &str,
    github_token: &str,
    per_page: &i32,
    page: &i32,
    branch_to_open: &String,
) {
    let client = github_api::GitHubClient::new(Some(github_token.to_string()));

    match client.get_branches(owner, repo_name, per_page, page).await {
        Ok(branches) => match branch_to_open.is_empty() {
            false => {
                println!(
                    "branch_to_opening branch {} in repository {}/{}...",
                    branch_to_open, owner, repo_name
                );

                let branch_to_branch_to_open = branches.iter().find(|branch| branch.name == *branch_to_open);
                match branch_to_branch_to_open {
                    Some(branch) => {
                        let _ = open::that(format!(
                            "https://github.com/{}/{}/tree/{}",
                            owner, repo_name, branch.name
                        ));
                    }
                    None => {
                        println!("Branch not found.")
                    }
                }
            }
            true => {
                println!(
                    "Fetching {} branches on page {} for repository {}/{}...",
                    per_page, page, owner, repo_name
                );

                let text = utils::get_info_from_branches(&branches);
                println!("{}", text);
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch languages: {}", e);
        }
    }
}
