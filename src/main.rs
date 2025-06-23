mod commands;
mod github_api;
mod schemas;
mod subcommands;
mod utils;

use crate::subcommands::Commands;
use clap::Parser;
use clearscreen;
use colored::*;
use dotenv::dotenv;
use std::{
    env,
    io::{self, Write},
};

const BANNER: &str = r#"
           ███   █████                                         █████   
          ░░░   ░░███                                         ░░███    
  ███████ ████  ███████    █████  ████████   ██████   ██████  ███████  
 ███░░███░░███ ░░░███░    ███░░  ░░███░░███ ███░░███ ███░░███░░░███░   
░███ ░███ ░███   ░███    ░░█████  ░███ ░███░███████ ░███ ░░░   ░███    
░███ ░███ ░███   ░███ ███ ░░░░███ ░███ ░███░███░░░  ░███  ███  ░███ ███
░░███████ █████  ░░█████  ██████  ░███████ ░░██████ ░░██████   ░░█████ 
 ░░░░░███░░░░░    ░░░░░  ░░░░░░   ░███░░░   ░░░░░░   ░░░░░░     ░░░░░  
 ███ ░███                         ░███                                 
░░██████                          █████                                
 ░░░░░░                          ░░░░░                                 



"#;

#[derive(Parser)]
#[command(name = "gitspect", disable_help_subcommand = true)]
#[command(about = "CLI tool to analyze GitHub repository stats", long_about = None)]
#[command(after_help = "Type 'quit' or 'exit' to leave the CLI.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").unwrap_or_default();

    print_banner();

    loop {
        let prompt = "gitspect> ".bold().white();
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" || input == "quit" {
            break;
        }

        let mut args = vec!["gitspect"];
        args.extend(input.split_whitespace());

        match Cli::try_parse_from(args) {
            Ok(cli) => match cli.command {
                Commands::Stats { owner, repo_name } => {
                    commands::stats::get_stats(&owner, &repo_name, &github_token).await;
                }
                Commands::ReadMe { owner, repo_name } => {
                    commands::readme::get_readme(&owner, &repo_name, &github_token).await;
                }
                Commands::Lang { owner, repo_name } => {
                    commands::lang::get_langs(&owner, &repo_name, &github_token).await;
                }
                Commands::Branches {
                    owner,
                    repo_name,
                    per_page,
                    page,
                    open,
                } => {
                    commands::branches::get_branches(
                        &owner,
                        &repo_name,
                        &github_token,
                        &per_page,
                        &page,
                        &open,
                    )
                    .await;
                }
                Commands::Commits {
                    owner,
                    repo_name,
                    per_page,
                    page,
                    branch,
                    author,
                } => {
                    commands::commits::get_commits(
                        &owner,
                        &repo_name,
                        &github_token,
                        &per_page,
                        &page,
                        &branch,
                        &author,
                    )
                    .await;
                }
                Commands::Contributors {owner,repo_name, per_page, page } => {
                    commands::contributors::get_contributors(&owner, &repo_name, &github_token, &per_page, &page)
                        .await;
                }
                Commands::Open { name } => {
                    commands::open::open_github(&name);
                }
                Commands::Clear {} => {
                    clearscreen::clear().unwrap();
                    print_banner();
                }
                Commands::Help {} => {
                    commands::help::print_custom_help();
                }
            },
            Err(_e) => {
                println!("\tAin't a command, partner!");
            }
        }
    }

    let farewell_text = "'Till next time.".bold().white();
    println!("\n {} \n", farewell_text);
}

fn print_banner() {
    println!("{}", BANNER);
}
