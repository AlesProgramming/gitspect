mod github_api;
mod utils;
mod subcommands;
mod commands;

use clap::{Parser};
use dotenv::dotenv;
use std::{env, io::{self, Write}};
use crate::subcommands::Commands;

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

    println!("{}", BANNER);

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
                        commands::stats::get_stats(&owner, &repo_name, &github_token).await;
                    },
                    Commands::ReadMe { owner, repo_name } => {
                        commands::readme::get_readme(&owner, &repo_name, &github_token).await;
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
