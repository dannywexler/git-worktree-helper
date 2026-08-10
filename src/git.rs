use std::{path::PathBuf, process::Command};

use git2::Repository;

use crate::logging::{LogFatalOption, LogFatalResult, log_fatal};

pub fn clone_repo(code_dir: PathBuf, host: &str, project: &str, repo: &str) -> Repository {
    let repo_url = format!("https://{}/{}/{}", host, project, repo);
    let repo_path = code_dir.join(project).join(repo).join(".bare");
    if repo_path.exists() {
        println!("Repo {repo_url} already cloned at {repo_path:?}");
    } else {
        println!("Cloning {}", repo_url);
        let clone_exit_code = Command::new("git")
            .args([
                "clone",
                "--bare",
                &repo_url,
                repo_path
                    .to_str()
                    .fatal(&format!("Repo path {repo_path:?} was not UTF8")),
            ])
            .status()
            .fatal("cloning repo")
            .code()
            .fatal("cloning was terminated");
        if clone_exit_code == 0 {
            println!("Successfully cloned {repo_url} into {repo_path:?}");
        } else {
            log_fatal(&format!("cloning. Got exit code: {clone_exit_code}"));
        }
    }
    Repository::open_bare(repo_path)
        .fatal_err(|err| format!("opening bare repo: {}", err.message()))
}
