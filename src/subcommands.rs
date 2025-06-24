use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum Commands {
    #[clap(aliases = &["statistics", "stat"])]
    Stats {
        owner: String,
        repo_name: String,
    },

    #[clap(aliases = &["readme", "rm"])]
    ReadMe {
        owner: String,
        repo_name: String,
    },

    #[clap(aliases = &["language", "languages"])]
    Lang {
        owner: String,
        repo_name: String,
    },

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

    #[clap(aliases = &["commit", "cm"])]
    Commits {
        owner: String,
        repo_name: String,
        branch: String,
        #[clap(long, default_value_t = 30, value_name = "AMOUNT PER PAGE")]
        per_page: i32,
        #[clap(long, default_value_t = 1, value_name = "PAGE #")]
        page: i32,
        #[clap(long, default_value_t = String::new(), value_name = "COMMIT AUTHOR'S EMAIL")]
        author: String,
    },

    #[clap(aliases = &["contrib", "authors"])]
    Contributors {
        owner: String,
        repo_name: String,
        #[clap(long, default_value_t = 30, value_name = "AMOUNT PER PAGE")]
        per_page: i32,
        #[clap(long, default_value_t = 1, value_name = "PAGE #")]
        page: i32,
    },

    Open {
        name: String,
    },

    #[clap(aliases = &["cls"])]
    Clear {},

    Help {},
}
