use colored::Colorize;

use crate::schemas::{Branch, CommitInfo, Contributor};

pub fn get_info_from_branches(branches: &Vec<Branch>) -> String {
    let mut output = "\n".to_owned();

    output.push_str(&format!("Branches ({}) \n", branches.len()));
    for branch in branches {
        output.push_str(&format!(
            " -> {:15} (commit: {}, protected: {}) \n\t url: {} \n",
            branch.name.white(),
            branch.commit.sha.bright_green(),
            (branch.protected.to_string()).red(),
            branch.commit.url
        ));
    }

    output
}

pub fn get_info_from_commits(commits: &Vec<CommitInfo>) -> String {
    let mut output = "\n".to_owned();

    output.push_str(&format!("Commits ({}) \n", commits.len()));
    for commit in commits {
        let indented_message = commit
            .commit
            .message
            .lines()
            .map(|line| format!("\t {}", line.white()))
            .collect::<Vec<_>>()
            .join("\n");

        output.push_str(&format!(
            " \n -> author: {}, date: {} \n\t message: {} \n\t sha: {} \n",
            commit.commit.author.email.blue(),
            commit.commit.author.date.yellow(),
            indented_message,
            commit.sha.bright_green(),
        ));
    }

    output
}

pub fn get_info_from_contributors(contributors: &Vec<Contributor>) -> String {
    let mut output = "\n".to_owned();

    output.push_str(&format!("Contributors ({}) \n", contributors.len()));
    for contributor in contributors {
        let contributor_info = format!(
            "Login: {}\n\tID: {}\n\tContributions: {}\n\tProfile: {} \n",
            contributor.login.blue(),
            contributor.id.to_string().cyan(),
            contributor.contributions.to_string().green(),
            contributor.html_url.underline().yellow(),
        );

        output.push_str(&contributor_info);
    }

    output
}
