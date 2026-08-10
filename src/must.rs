use crate::logging::LogFatalResult;
use resolve_path::PathResolveExt;
use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

pub fn must_resolve_path<P: AsRef<Path> + Debug>(string_path: P) -> PathBuf {
    string_path
        .as_ref()
        .try_resolve()
        .map(|path| path.into())
        .fatal(&format!("resolving path {:?}", string_path))
}

pub fn must_create_dir<P: AsRef<Path> + Debug>(string_path: P) -> PathBuf {
    let path = must_resolve_path(string_path);
    if path.exists() {
        return path;
    }
    create_dir_all(path.clone())
        .map(|_| {
            println!("Created dir: {path:?}");
            path
        })
        .fatal("creating directory")
}
