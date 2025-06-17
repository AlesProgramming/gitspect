use crate::github_api::{GithubFile, RepoStats};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use colored::*;

pub fn format_stats_struct(stats: &RepoStats) -> String {
    format!(
        "\n\
     {} {}\n\
     {} {}\n\n\
     {} {}\n\
     {} {}\n\n\
     {} {}\n\n\
     {} {}\n\
     {} {}\n\n\
     {} {}\n\n\
     {} {}\n\
     {} {}\n\
     {} {}\n\n",
        "Repository:".bold().blue(),
        stats.full_name,
        "Description:".bold().blue(),
        stats.description,
        "Stars:".bold().green(),
        stats.stargazers_count,
        "Forks:".bold().green(),
        stats.forks_count,
        "Open Issues:".bold().red(),
        stats.open_issues_count,
        "Watchers:".bold().cyan(),
        stats.watchers_count,
        "Subscribers:".bold().cyan(),
        stats.subscribers_count,
        "Language:".bold().magenta(),
        stats.language.as_deref().unwrap_or("N/A"),
        "Created At:".bold().white(),
        stats.created_at,
        "Updated At:".bold().white(),
        stats.updated_at,
        "Last Push:".bold().white(),
        stats.pushed_at,
    )
}

pub fn parse_github_file_to_readme_text(file: &GithubFile) -> String {
    match file.encoding.as_str() {
        "base64" => {
            let cleaned_content = file.content.replace('\n', "").replace('\r', "");
            match STANDARD.decode(&cleaned_content) {
                Ok(bytes) => {
                    String::from_utf8(bytes).unwrap_or_else(|_| "Invalid UTF-8".to_string())
                }
                Err(e) => format!("Failed to decode base64: {}", e),
            }
        }
        _ => file.content.clone()
    }
}
