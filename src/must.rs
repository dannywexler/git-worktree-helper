use git2::Repository;
use resolve_path::PathResolveExt;
use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
    process,
};

pub fn must_resolve_path<P: AsRef<Path> + Debug>(string_path: P) -> PathBuf {
    match string_path.as_ref().try_resolve() {
        Ok(path) => path.into(),
        Err(path_err) => {
            eprintln!(
                "Error resolving path {:?} Got message: {}",
                string_path, path_err
            );
            process::exit(1);
        }
    }
}

pub fn must_create_dir<P: AsRef<Path> + Debug>(string_path: P) -> PathBuf {
    let path = must_resolve_path(string_path);
    if path.exists() {
        return path;
    }
    if let Err(create_dir_error) = create_dir_all(path.clone()) {
        eprintln!("Error creating dir: {:?}", path);
        eprintln!("{}", create_dir_error);
        process::exit(1);
    } else {
        println!("Created dir: {:?}", path);
        path
    }
}

pub fn must_open_bare_repo<P: AsRef<Path>>(path: P) -> Repository {
    match Repository::open_bare(path) {
        Ok(rep) => rep,
        Err(open_bare_error) => {
            eprintln!("Error opening bare_repo: {}", open_bare_error);
            process::exit(1);
        }
    }
}
