use std::fmt::Debug;
use std::path::PathBuf;
use std::process::Command;

use git2::Repository;
use git2::string_array::StringArray;

use crate::logging::StringResult;
use crate::safe_fs::MapIOErrorExtension;

pub trait StringArrayExtension {
    fn to_vec<S1: AsRef<str> + Debug, S2: AsRef<str> + Debug>(
        self,
        repo_path: S1,
        label: S2,
    ) -> StringResult<Vec<String>>;
}

impl StringArrayExtension for StringArray {
    fn to_vec<S1: AsRef<str> + Debug, S2: AsRef<str> + Debug>(
        self,
        repo_path: S1,
        label: S2,
    ) -> StringResult<Vec<String>> {
        let repo_path_str = repo_path.as_ref();
        let label_str = label.as_ref();
        self.iter()
            .collect::<Result<Vec<_>, _>>()
            .map_git_err(format!(
                "Repository at {:?} could not access {:?}",
                repo_path_str, label_str
            ))?
            .iter()
            .map(|item| {
                item.ok_or_else(|| {
                    format!(
                        "Repository at {:?} {:?} item was not UTF8",
                        repo_path_str, label_str
                    )
                })
                .map(|item_str| item_str.to_owned())
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

pub trait MapGitErrorExtension<T> {
    fn map_git_err(self, msg: impl AsRef<str> + Debug) -> StringResult<T>;
}

impl<T> MapGitErrorExtension<T> for Result<T, git2::Error> {
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
    repository: Repository,
}

impl BareRepo {
    fn try_open_or_clone(
        code_dir: PathBuf,
        host: String,
        project_name: String,
        repo_name: String,
    ) -> StringResult<BareRepo> {
        let repo_path = code_dir.join(&project_name).join(&repo_name).join(".bare");
        let repo_already_exists = repo_path.exists();
        let repo_path_str = repo_path
            .to_str()
            .ok_or_else(|| format!("Repo path {:?} was not valid UTF8", repo_path))?;
        let repo_url = format!("https://{}/{}/{}", host, project_name, repo_name);
        if !repo_already_exists {
            println!("Cloning {repo_url} into {repo_path_str}");
            let clone_exit_code = Command::new("git")
                .args(["clone", "--bare", &repo_url, repo_path_str])
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
        let repository = Repository::open_bare(repo_path_str)
            .map_git_err(format!("Could not open {repo_path_str} as bare repo."))?;

        if repo_already_exists {
            println!("Repo already present at {repo_path_str}");
        }

        Ok(BareRepo {
            code_dir: code_dir.clone(),
            host: host.clone(),
            project_name: project_name.clone(),
            repo_name: repo_name.clone(),
            repo_path: repo_path.clone(),
            repository,
        })
    }

    pub fn get_remote_name(&self) -> StringResult<String> {
        let all_remotes = self
            .repository
            .remotes()
            .map_git_err(format!(
                "Could not access remotes for repo at {:?}",
                self.repo_path
            ))?
            .to_vec(format!("{:?}", self.repo_path), "remotes")?;

        let all_remotes_length = all_remotes.len();

        match all_remotes_length {
            0 => Err(format!(
                "Repository at {:?} had no remotes!",
                self.repo_path
            )),
            1 => all_remotes
                .first()
                .ok_or_else(|| {
                    format!(
                        "Repository at {:?} was missing the first remote!",
                        self.repo_path
                    )
                })
                .map(|first_item| first_item.to_owned()),
            _ => Err(format!(
                "Repository at {:?} had {all_remotes_length} remotes: {all_remotes:?}!",
                self.repo_path
            )),
        }
    }

    pub fn get_default_branch_name(&self) -> StringResult<String> {
        let remote_name = self.get_remote_name()?;
        let mut connected_remote = self.repository.find_remote(&remote_name).map_git_err(format!(
            "Repository at {:?} could not access remote named {remote_name}",
            self.repo_path
        ))?;

        connected_remote
            .connect(git2::Direction::Fetch)
            .map_git_err(format!(
                "Repository at {:?} could not connect to remote named {remote_name}", self.repo_path
            ))?;

        let default_branch_buf = connected_remote
            .default_branch()
            .map_git_err(format!(
                "Repository at {:?} could not access the default branch name", self.repo_path
            ))?;

        let default_branch_str = default_branch_buf
            .as_str()
            .map_git_err(format!("Could not convert default branch name to string"))?;

        let default = default_branch_str
            .strip_prefix("refs/heads/")
            .ok_or_else(|| format!("Default branch name {default_branch_str} did not start with refs/heads/"))?
            .to_owned();
        Ok(default)
    }
}

pub fn handle_clone_repo(
    code_dir: PathBuf,
    host: &str,
    project_name: &str,
    repo_name: &str,
) -> StringResult {
    let br =
        BareRepo::try_open_or_clone(code_dir, host.into(), project_name.into(), repo_name.into())?;
    let remote_name = br.get_remote_name()?;
    println!("Got remote name: {remote_name}");
    let default_branch = br.get_default_branch_name()?;
    println!("Got default branch name: {default_branch}");
    Ok(())
}

pub fn handle_clone_branch(
    _code_dir: PathBuf,
    _host: &str,
    _project: Option<String>,
    _repo: Option<String>,
    _branch: Option<String>,
) -> StringResult {
    todo!()
}
