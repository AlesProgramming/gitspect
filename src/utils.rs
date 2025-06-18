use std::collections::HashMap;

use crate::github_api::{Branch, GithubFile, RepoStats};
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
        _ => file.content.clone(),
    }
}

pub fn get_percentages_from_lang_hashmap(langs: &HashMap<String, u32>) -> String {
    let keys: Vec<&String> = langs.keys().collect();
    let values: Vec<&u32> = langs.values().collect();
    let total_value: u32 = langs.values().sum();

    let mut updated_hashmap: HashMap<String, f32> = HashMap::new();

    for k in 0..keys.len() {
        updated_hashmap.insert(
            keys[k].to_string(),
            values[k].clone() as f32 / total_value as f32,
        );
    }

    let mut sorted_order: Vec<(&String, &f32)> = updated_hashmap.iter().collect();
    sorted_order.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    let mut output = "\n".to_owned();

    for (lang, percent) in &sorted_order {
        output.push_str(&(format!("{}: {:.5}%", lang.cyan(), *percent * 100.0) + &format!(" [{} bytes] \n", langs[*lang])));
    }

    output
}

pub fn get_info_from_branches(branches: &Vec<Branch>) -> String {
    let mut output = "\n".to_owned();

    output.push_str(&format!("Branches ({}) \n", branches.len()));
    for branch in branches {
        output.push_str(&format!(" -> {:15} (commit: {}, protected: {}) \n", branch.name.white(), branch.commit.sha.bright_green(), (branch.protected.to_string()).red()));
    }
    
    output
}
