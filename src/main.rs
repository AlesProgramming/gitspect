use clap::{Parser, Subcommand};
use dotenv::dotenv;
use std::env;

#[derive(Parser)]
#[command(name = "gitspect")]
#[command(about = "CLI tool to analyze GitHub repository stats", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Stats { owner: String, repo_name: String },
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let github_token = env::var("GITHUB_TOKEN").unwrap_or_default();

    match &cli.command {
        Commands::Stats { owner, repo_name } => {
            println!("Fetching stats for repository {}/{}...", owner, repo_name);
        }
    }
}
