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

pub struct GitHubClient {
    client: reqwest::Client,
    token: Option<String>,
}

impl GitHubClient {
    pub fn new(token: Option<String>) -> Self {
        GitHubClient {
            client: reqwest::Client::new(),
            token,
        }
    }

    fn build_headers(&self) -> Result<HeaderMap, Box<dyn Error>> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Accept",
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert("User-Agent", HeaderValue::from_static("gitspect-cli"));

        if let Some(token) = &self.token {
            if !token.is_empty() {
                headers.insert(
                    "Authorization",
                    HeaderValue::from_str(&format!("Bearer {}", token))?,
                );
            }
        }

        Ok(headers)
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T, Box<dyn Error>> {
        let headers = self.build_headers()?;
        let resp = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        let json = resp.json::<T>().await?;
        Ok(json)
    }

    pub async fn fetch_stats(&self, owner: &str, repo: &str) -> Result<RepoStats, Box<dyn Error>> {
        let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
        self.get_json(&url).await
    }

    pub async fn fetch_readme(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<GithubFile, Box<dyn Error>> {
        let url = format!("https://api.github.com/repos/{}/{}/readme", owner, repo);
        self.get_json(&url).await
    }

    pub async fn get_langs(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<HashMap<String, u32>, Box<dyn Error>> {
        let url = format!("https://api.github.com/repos/{}/{}/languages", owner, repo);
        self.get_json(&url).await
    }

    pub async fn get_branches(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Branch>, Box<dyn Error>> {
        let url = format!("https://api.github.com/repos/{}/{}/branches", owner, repo);
        self.get_json(&url).await
    }
}