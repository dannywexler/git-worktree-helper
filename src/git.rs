use std::fmt::Debug;
use std::path::PathBuf;
use std::process::Command;

use git2::Repository;

use crate::logging::StringResult;
use crate::safe_fs::MapIOError;

trait MapGitError<T> {
    fn map_git_err(self, msg: impl AsRef<str> + Debug) -> StringResult<T>;
}

impl<T> MapGitError<T> for Result<T, git2::Error> {
    fn map_git_err(self, msg: impl AsRef<str> + Debug) -> StringResult<T> {
        self.map_err(|git_err| {
            let mut err_msg = String::from("Fatal Git Error! ");
            err_msg.push_str(msg.as_ref());
            err_msg.push_str(&format!(
                "\n  Cause: {:?}::{:?} {}",
                git_err.class(),
                git_err.code(),
                git_err.message()
            ));
            err_msg
        })
    }
}

struct BareRepo {
    code_dir: PathBuf,
    host: String,
    project_name: String,
    repo_name: String,
    repo_path: PathBuf,
}

impl BareRepo {
    fn new(code_dir: PathBuf, host: String, project_name: String, repo_name: String) -> Self {
        Self {
            code_dir: code_dir.clone(),
            host: host.clone(),
            project_name: project_name.clone(),
            repo_name: repo_name.clone(),
            repo_path: code_dir.join(project_name).join(repo_name).join(".bare"),
        }
    }

    fn clone(&self) -> StringResult<ClonedBareRepo> {
        let repo_already_exists = self.repo_path.exists();
        if !repo_already_exists {
            let repo_url = format!(
                "https://{}/{}/{}",
                self.host, self.project_name, self.repo_name
            );
            let repo_path_str = self
                .repo_path
                .to_str()
                .ok_or_else(|| format!("Repo path {:?} was not valid UTF8", self.repo_path))?;
            println!("Cloning {repo_url} into {repo_path_str}");
            let clone_exit_code = Command::new("git")
                .args(["clone", "--bare", &repo_url, &repo_path_str])
                .status()
                .map_io_err("Could not clone repo")?
                .code()
                .ok_or("Cloning was terminated")?;
            if clone_exit_code == 0 {
                println!("Successfully cloned {repo_url} into {repo_path_str}");
            } else {
                return Err(format!(
                    "Could not clone {repo_url}. Got exit code: {clone_exit_code}"
                ));
            }
        }
        let repository = Repository::open_bare(&self.repo_path)
            .map_git_err(format!("Could not open {:?} as bare repo.", self.repo_path))?;
        if repo_already_exists {
            println!("Repo already present at {:?}", self.repo_path);
        }

        Ok(ClonedBareRepo {
            code_dir: self.code_dir.clone(),
            host: self.host.clone(),
            project_name: self.project_name.clone(),
            repo_name: self.repo_name.clone(),
            repo_path: self.repo_path.clone(),
            repository,
        })
    }
}

struct ClonedBareRepo {
    code_dir: PathBuf,
    host: String,
    project_name: String,
    repo_name: String,
    repo_path: PathBuf,
    repository: Repository,
}

pub fn clone_repo(
    code_dir: PathBuf,
    host: &str,
    project_name: &str,
    repo_name: &str,
) -> StringResult {
    BareRepo::new(code_dir, host.into(), project_name.into(), repo_name.into()).clone()?;
    Ok(())
}

pub fn clone_branch(
    code_dir: PathBuf,
    host: &str,
    project: Option<String>,
    repo: Option<String>,
    branch: Option<String>,
) -> StringResult {
    todo!()
}
