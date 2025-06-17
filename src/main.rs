mod github_api;
mod utils;
mod commands;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::{env, io::{self, Write}};
use crate::commands::Commands;

#[derive(Parser)]
#[command(name = "gitspect")]
#[command(about = "CLI tool to analyze GitHub repository stats", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").unwrap_or_default();

    loop {
        print!("gitspect> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" || input == "quit" {
            break;
        }

        let mut args = vec!["gitspect"];
        args.extend(input.split_whitespace());

        match Cli::try_parse_from(args){
            Ok(cli) => {
                match cli.command {
                    Commands::Stats { owner, repo_name } => {
                        println!("Fetching stats for repository {}/{}...", owner, repo_name);

                        let stats = github_api::fetch_stats(&owner, &repo_name, &github_token).await.unwrap();

                        let response = utils::format_stats_struct(&stats);
                        print!("{}", response);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    println!("'Till next time.");
}
