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
    Branches {
        owner: String,
        repo_name: String,
        #[clap(long, default_value_t = 30, value_name = "AMOUNT PER PAGE")]
        per_page: i32,
        #[clap(long, default_value_t = 1, value_name = "PAGE #")]
        page: i32,
        #[clap(long, default_value_t = String::new(), value_name = "NAME OF BRANCH")]
        open: String,
    },

    #[clap(aliases = &["cls"])]
    Clear {},
}
