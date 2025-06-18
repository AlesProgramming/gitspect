use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum Commands {
    #[clap(aliases = &["statistics", "stat"])]
    Stats { owner: String, repo_name: String },

    #[clap(aliases = &["readme", "rm"])]
    ReadMe { owner: String, repo_name: String },

    #[clap(aliases = &["language"])]
    Lang { owner: String, repo_name: String },

    #[clap(aliases = &["branch", "br"])]
    Branches { owner: String, repo_name: String },
}

