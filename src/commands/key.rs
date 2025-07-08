use std::env;
use std::io::stdin;
use colored::Colorize;
use tokio::fs;

pub async fn update_key(key: &str) {
    let new_line = format!("GITHUB_TOKEN={}", key);
    let res = fs::write(".env", new_line).await;

    match res {
        Ok(_) => {
            env::set_var("GITHUB_TOKEN", key);
            let update_text = "Key Updated".bold();
            println!("{}", update_text);
        }
        Err(e) => {
            let error_text = "Failed to update key: ".bright_red().bold();
            println!("{} {}", error_text, e);
        }
    }
}

pub async fn view_key() {
    println!("Are you sure you want to view your github token? [y/n]: ");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    if input == "yes" || input == "y" {
        let key = env::var("GITHUB_TOKEN");
        match key {
            Ok(k) => {
                println!("Your key: {}", k);
            }
            Err(e) => {
                println!("Failed to grab token: {}", e);
            }
        }
    } else {
        return;
    }
}


