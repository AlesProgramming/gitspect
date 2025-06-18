use colored::*;

pub fn print_custom_help() {
    println!("\n");
    println!("{}", "Commands:".underline());

    println!("  {:<12}{}", "stats".bold(), "Show repository statistics");
    println!("  {:<12}  {}", "", "<owner> <repo_name>".italic().dimmed());

    println!(
        "  {:<12}{}",
        "read-me".bold(),
        "Display repository's README"
    );
    println!("  {:<12}  {}", "", "<owner> <repo_name>".italic().dimmed());

    println!(
        "  {:<12}{}",
        "lang".bold(),
        "List languages used in the repository"
    );
    println!("  {:<12}  {}", "", "<owner> <repo_name>".italic().dimmed());

    println!(
        "  {:<12}{}",
        "branches".bold(),
        "List branches with optional filters"
    );
    println!("  {:<12}  {}", "", "<owner> <repo_name>".italic().dimmed());
    println!(
        "  {:<12}  {}",
        "",
        "--per-page <AMOUNT> (default: 30)".italic().dimmed()
    );
    println!(
        "  {:<12}  {}",
        "",
        "--page <PAGE #> (default: 1)".italic().dimmed()
    );
    println!(
        "  {:<12}  {}",
        "",
        "--open <BRANCH NAME> (optional)".italic().dimmed()
    );

    println!("  {:<12}{}", "clear".bold(), "Clear the screen");

    println!("  {:<12}{}", "help".bold(), "Show this help message");

    println!("\n{}", "Options:".underline());
    println!("  {:<12}{}", "-h, --help".bold(), "Print help information");

    println!(
        "\n{}",
        "Type 'quit' or 'exit' to leave the CLI.".italic().dimmed()
    );

    println!("\n");
}
