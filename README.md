<pre>
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
</pre>

# gitspect

A command-line tool to fetch and display GitHub repository data like branches, stats, and languages.

## Features
- **setkey**: Edit GitHub API key
- **viewkey**: View GitHub API key
- **stats**: Show repository statistics  
- **read-me**: Display repository's README file  
- **lang**: List programming languages used in the repository  
- **branches**: List branches with optional pagination and filter options  
- **commits**: Show commit history with pagination, branch, and author filters  
- **contributors**: Show contributors to the repository with pagination options  
- **open**: Open GitHub user or repository page in the browser  
- **clear**: Clear the terminal screen  
- **help**: Show usage instructions and available commands  

## Configuration

Use the **`setkey`** command to add your classic GitHub Personal Access Token (with access to public repositories).  
&nbsp;&nbsp;&nbsp;&nbsp;**or**  
Add your token manually to the `.env` file in the project root:

```env
GITHUB_TOKEN=your_personal_access_token_here
```

> **Note:** If you clone the project and manually edit the `.env` file, you must rebuild the project using Cargo.
> Make sure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed:
>
> ```bash
> cargo build
> ```

## Preview

https://github.com/user-attachments/assets/af831b4b-3634-41ed-ae4b-16409a723f89

## AI Use

- Generated README.md
- Generated help command
- Helped with converting JSON to Rust structs
