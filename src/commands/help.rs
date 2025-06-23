use colored::*;

pub fn print_custom_help() {
    println!("\n");
    println!("{}", "Commands:".underline());

    println!("  {:<14}{}", "stats".bold(), "Show repository statistics");
    println!("  {:<14}  {}", "", "<owner> <repo_name>".italic().dimmed());

    println!("  {:<14}{}", "read-me".bold(), "Display repository's README");
    println!("  {:<14}  {}", "", "<owner> <repo_name>".italic().dimmed());

    println!("  {:<14}{}", "lang".bold(), "List languages used in the repository");
    println!("  {:<14}  {}", "", "<owner> <repo_name>".italic().dimmed());

    println!("  {:<14}{}", "branches".bold(), "List branches with optional filters");
    println!("  {:<14}  {}", "", "<owner> <repo_name>".italic().dimmed());
    println!("  {:<14}  {}", "", "--per-page <AMOUNT> (default: 30)".italic().dimmed());
    println!("  {:<14}  {}", "", "--page <PAGE #> (default: 1)".italic().dimmed());
    println!("  {:<14}  {}", "", "--open <BRANCH NAME> (optional)".italic().dimmed());
    
    println!("  {:<14}{}", "commits".bold(), "Show commit history with filters");
    println!("  {:<14}  {}", "", "<owner> <repo_name>".italic().dimmed());
    println!("  {:<14}  {}", "", "--per-page <AMOUNT PER PAGE> (default: 30)".italic().dimmed());
    println!("  {:<14}  {}", "", "--page <PAGE #> (default: 1)".italic().dimmed());
    println!("  {:<14}  {}", "", "--branch <BRANCH NAME> (default: main)".italic().dimmed());
    println!("  {:<14}  {}", "", "--author <COMMIT AUTHOR> (optional)".italic().dimmed());

    println!("  {:<14}{}", "contributors".bold(), "Show contributors to the repository");
    println!("  {:<14}  {}", "", "<owner> <repo_name>".italic().dimmed());
    println!("  {:<14}  {}", "", "--per-page <AMOUNT> (default: 30)".italic().dimmed());
    println!("  {:<14}  {}", "", "--page <PAGE #> (default: 1)".italic().dimmed());

    println!("  {:<14}{}", "open".bold(), "Open GitHub repo or user page in the browser");
    println!("  {:<14}  {}", "", "<username> or <username/repo>".italic().dimmed());

    println!("  {:<14}{}", "clear".bold(), "Clear the screen");
    println!("  {:<14}{}", "help".bold(), "Show this help message");

    println!(
        "\n{}",
        "Type 'quit' or 'exit' to leave the CLI.".italic().dimmed()
    );

    println!("\n");
}