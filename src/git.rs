use std::{fs::read_dir, path::PathBuf, process::Command};

use git2::{Repository, WorktreeAddOptions};

use crate::logging::{LogFatalOption, LogFatalResult, log_fatal};

pub fn clone_repo(code_dir: &PathBuf, host: &str, project: &str, repo: &str) -> Repository {
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
    let repository = Repository::open_bare(repo_path)
        .fatal_err(|err| format!("opening bare repo: {}", err.code()));
    let default_branch = must_get_default_branch(&repository);
    clone_branch(
        &code_dir,
        &host,
        Some(project.into()),
        Some(repo.into()),
        Some(default_branch),
    );
    repository
}

pub fn clone_branch(
    code_dir: &PathBuf,
    host: &str,
    project: Option<String>,
    repo: Option<String>,
    branch: Option<String>,
) {
    let resolved_project = match project {
        Some(project_name) => {
            println!("Initial project: {project_name}");
            project_name
        }
        None => {
            println!("No initial project.");
            let all_project_dirs: Vec<String> = read_dir(&code_dir)
                .fatal(&format!("reading code_dir {code_dir:?}"))
                .filter_map(|entry| {
                    let dir_entry = entry.fatal("Could not access entry");
                    let dir_name = dir_entry.file_name();
                    let dir_type = dir_entry.file_type().fatal("Could not access file_type");
                    if dir_type.is_dir() && dir_name != ".bare" {
                        Some(dir_name.into_string().fatal("was not UTF8"))
                    } else {
                        None
                    }
                })
                .collect();
            all_project_dirs
                .first()
                .fatal("Must be at least one project")
                .clone()
        }
    };
    println!("Resolved project: {resolved_project:?}");
    let resolved_repo = match repo {
        Some(repo_name) => {
            println!("Iniital repo name: {repo_name}");
            repo_name
        }
        None => {
            println!("No initial project.");
            let all_project_dirs = read_dir(&code_dir.join(&resolved_project))
                .fatal(&format!("reading code_dir {code_dir:?}"))
                .filter_map(|entry| {
                    let dir_entry = entry.fatal("Could not access entry");
                    let dir_name = dir_entry.file_name();
                    let dir_type = dir_entry.file_type().fatal("Could not access file_type");
                    if dir_type.is_dir() {
                        Some(dir_name.into_string().fatal("was not UTF8"))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            all_project_dirs
                .first()
                .fatal("Must be at least one repo")
                .clone()
        }
    };
    let repo_dir = code_dir
        .join(&resolved_project)
        .join(&resolved_repo)
        .join(".bare");
    println!("Resolved repo: {resolved_repo:?}");
    let repository = Repository::open_bare(&repo_dir)
        .fatal_err(|err| format!("opening bare repo: {}", err.message()));
    let default_branch = must_get_default_branch(&repository);
    let all_worktree_names = must_get_all_worktree_names(&repository);
    let available_branches = must_get_all_branches(&repository)
        .into_iter()
        .filter(|branch_name| !all_worktree_names.contains(branch_name))
        .collect::<Vec<_>>();
    println!("all_worktree_names: {all_worktree_names:?}");
    println!("available_branches: {available_branches:?}");
    println!("default_branch: {default_branch}");

    let resolved_branch = match branch {
        Some(branch) => {
            if available_branches.contains(&branch) {
                branch
            } else {
                log_fatal(&format!(
                    "Provided branch {branch} is not an available branch: {available_branches:?}"
                ))
            }
        }
        None => {
            println!("No branch selected");
            if available_branches.contains(&default_branch) {
                println!("Cloning default branch {default_branch}");
                default_branch
            } else {
                println!("TODO: pick among available_branches: {available_branches:?}");
                todo!("TODO: pick among available_branches")
            }
        }
    };
    println!(
        "Cloning branch: {resolved_branch} into new worktree of {resolved_project}/{resolved_repo}"
    );
    let branch_dir = code_dir
        .join(resolved_project)
        .join(resolved_repo)
        .join(&resolved_branch);

    let mut worktree_add_opts = WorktreeAddOptions::new();
    worktree_add_opts.checkout_existing(true);

    repository
        .worktree(&resolved_branch, &branch_dir, Some(&worktree_add_opts))
        .fatal_err(|err| format!("Could not create worktree: {}", err.message()));
}

pub fn must_get_all_branches(repository: &Repository) -> Vec<String> {
    repository
        .branches(None)
        .fatal("Could not list branches")
        .into_iter()
        .map(|branch| {
            branch
                .fatal("Could not access branch")
                .0
                .name()
                .fatal("Could not access branch name")
                .fatal("branch name was not UTF8")
                .to_owned()
        })
        .collect::<Vec<_>>()
}

pub fn must_get_all_worktree_names(repository: &Repository) -> Vec<String> {
    repository
        .worktrees()
        .fatal("Could not list worktrees")
        .into_iter()
        .map(|wt| {
            wt.fatal("Could not access worktree name")
                .fatal("worktree name was not UTF8")
                .to_owned()
        })
        .collect::<Vec<_>>()
}

pub fn must_get_default_branch(repository: &Repository) -> String {
    let mut remote = repository
        .find_remote(
            repository
                .remotes()
                .fatal("listing remotes")
                .into_iter()
                .map(|remote_name| {
                    remote_name
                        .fatal("Could not access remote name")
                        .fatal("remote name was not UTF8")
                })
                .collect::<Vec<_>>()
                .first()
                .fatal("Must have a remote"),
        )
        .fatal(&format!("Could not find remote"));

    remote
        .connect(git2::Direction::Fetch)
        .fatal("Could not connect");

    remote
        .default_branch()
        .fatal_err(|err| format!("Could not read default branch: {}", err.message()))
        .as_str()
        .fatal("Could not parse Buf to str")
        .strip_prefix("refs/heads/")
        .fatal("Default refspec did not start with refs/heads/")
        .to_owned()
}
