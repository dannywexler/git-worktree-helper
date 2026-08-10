use std::{
    path::Path,
    process::{self, Command},
};

use git2::Repository;

pub fn clone_repo(code_dir: &str, host: &str, project: &str, repo: &str) -> Repository {
    let repo_url = format!("https://{}/{}/{}", host, project, repo);
    let repo_path = Path::new(code_dir).join(project).join(repo).join(".bare");
    println!("Cloning {} into {}", repo_url, repo_path.display());
    match Command::new("git")
        .args([
            "clone",
            "--bare",
            &repo_url,
            repo_path.to_str().expect("UTF8 path"),
        ])
        .status()
    {
        Ok(exit_status) => match exit_status.code() {
            Some(exit_code) => match exit_code {
                0 => println!("Cloned successfully"),
                _ => {
                    eprintln!("Error cloning. Got exit code: {}", exit_code);
                    process::exit(1);
                }
            },
            None => {
                eprintln!("Error cloning. Termintated by signal");
                process::exit(1);
            }
        },
        Err(clone_err) => {
            eprintln!("Error cloning. {}", clone_err);
            process::exit(1)
        }
    };
    match Repository::open_bare(repo_path) {
        Ok(rep) => rep,
        Err(open_bare_error) => {
            eprintln!("Error opening bare_repo: {}", open_bare_error);
            process::exit(1);
        }
    }
}
