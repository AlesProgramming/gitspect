use std::{collections::HashMap, error::Error};

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RepoStats {
    pub full_name: String,
    pub description: String,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
    pub watchers_count: u64,
    pub subscribers_count: u64,
    pub language: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GithubFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub url: String,
    pub content: String,
    pub encoding: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct BranchCommit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: BranchCommit,
    pub protected: bool,
}

pub async fn fetch_stats(
    owner: &str,
    repo: &str,
    token: &str,
) -> Result<RepoStats, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);

    let mut headers = HeaderMap::new();

    headers.insert(
        "Accept",
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert("User-Agent", HeaderValue::from_static("gitspect-cli"));

    if !token.is_empty() {
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    let repo_stats = resp.json::<RepoStats>().await?;

    Ok(repo_stats)
}

pub async fn fetch_readme(
    owner: &str,
    repo: &str,
    token: &str,
) -> Result<GithubFile, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/readme", owner, repo);

    let mut headers = HeaderMap::new();

    headers.insert(
        "Accept",
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert("User-Agent", HeaderValue::from_static("gitspect-cli"));

    if !token.is_empty() {
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    let repo_file = resp.json::<GithubFile>().await?;

    Ok(repo_file)
}

pub async fn get_langs(
    owner: &str,
    repo: &str,
    token: &str,
) -> Result<HashMap<String, u32>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/languages", owner, repo);

    let mut headers = HeaderMap::new();

    headers.insert(
        "Accept",
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert("User-Agent", HeaderValue::from_static("gitspect-cli"));

    if !token.is_empty() {
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    let langs: HashMap<String, u32> = resp.json().await?;

    Ok(langs)
}

pub async fn get_branches(
    owner: &str,
    repo: &str,
    token: &str,
) -> Result<Vec<Branch>, Box<dyn Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/branches", owner, repo);

    let mut headers = HeaderMap::new();

    headers.insert(
        "Accept",
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert("User-Agent", HeaderValue::from_static("gitspect-cli"));

    if !token.is_empty() {
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
    }

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .error_for_status()?;

    let branches: Vec<Branch> = resp.json().await?;

    Ok(branches)
}
