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

Add your classic GitHub Personal Access Token with public repo access to the `.env`  file in the project root

```text
GITHUB_TOKEN=your_personal_access_token_here
```

## AI Use

- Generated README.md
- Generated help command
- Helped with converting JSON to Rust structs
