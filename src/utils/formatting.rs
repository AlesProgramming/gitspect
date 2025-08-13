use std::collections::HashMap;

use colored::Colorize;

use crate::schemas::RepoStats;

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
        stats.description.as_deref().unwrap_or("N/A"),
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
        output.push_str(
            &(format!(
                "\t {}: {:.5}% [{} bytes] \n",
                lang.cyan(),
                *percent * 100.0,
                langs[*lang]
            )),
        );
    }

    output
}
