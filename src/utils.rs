use crate::github_api::RepoStats;

pub fn format_stats_struct(stats: &RepoStats) -> String {
    format!(
        "Repository: {}\n\
         Description: {}\n\
         Stars: {}\n\
         Forks: {}\n\
         Open Issues: {}\n",
        stats.full_name,
        stats.description,
        stats.stargazers_count,
        stats.forks_count,
        stats.open_issues_count,
    )
}
