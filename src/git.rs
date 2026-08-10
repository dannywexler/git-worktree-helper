use std::{
    path::PathBuf,
    process::{self, Command},
};

use git2::Repository;

use crate::must::must_open_bare_repo;

pub fn clone_repo(code_dir: PathBuf, host: &str, project: &str, repo: &str) -> Repository {
    let repo_url = format!("https://{}/{}/{}", host, project, repo);
    let repo_path = code_dir.join(project).join(repo).join(".bare");
    if repo_path.exists() {
        println!("Repo {repo_url} already cloned at {repo_path:?}");
    } else {
        println!("Cloning {}", repo_url);
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
                    0 => println!(
                        "Successfully cloned {} into {}",
                        repo_url,
                        repo_path.display()
                    ),
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
    }
    must_open_bare_repo(repo_path)
}
