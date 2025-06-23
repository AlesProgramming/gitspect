use base64::{engine::general_purpose::STANDARD, Engine as _};

use crate::schemas::GithubFile;

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