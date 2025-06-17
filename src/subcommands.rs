use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum Commands {
    Stats { owner: String, repo_name: String },
    ReadMe { owner: String, repo_name: String},
}
