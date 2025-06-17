use crate::github_api::RepoStats;
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
