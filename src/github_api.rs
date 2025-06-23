use std::{collections::HashMap, error::Error};

use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

use crate::schemas::*;

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
        per_page: &i32,
        page: &i32,
    ) -> Result<Vec<Branch>, Box<dyn Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/branches?per_page={}&page={}",
            owner, repo, per_page, page
        );
        self.get_json(&url).await
    }

    pub async fn get_commits(
        &self,
        owner: &str,
        repo: &str,
        per_page: &i32,
        page: &i32,
        branch: &str,
        author: &str,
    ) -> Result<Vec<CommitInfo>, Box<dyn Error>> {
        let url: String = if author.is_empty() {
            format!(
                "https://api.github.com/repos/{}/{}/commits?sha={}&per_page={}&page={}",
                owner, repo, branch, per_page, page
            )
        } else {
            format!(
                "https://api.github.com/repos/{}/{}/commits?sha={}&author={}&per_page={}&page={}
",
                owner, repo, branch, author, per_page, page
            )
        };

        self.get_json(&url).await
    }

    pub async fn get_contributors(
        &self,
        owner: &str,
        repo: &str,
        per_page: &i32,
        page: &i32
    ) -> Result<Vec<Contributor>, Box<dyn Error>> {
        let url: String = format!(
            "https://api.github.com/repos/{}/{}/contributors?page={}&per_page={}",
            owner, repo, page, per_page
        );
        self.get_json(&url).await
    }
}
