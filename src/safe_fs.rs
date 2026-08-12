use std::{
    fmt::Debug,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use resolve_path::PathResolveExt;

use crate::logging::StringResult;

pub trait MapIOError<T> {
    fn map_io_err(self, msg: impl AsRef<str> + Debug) -> StringResult<T>;
}

impl<T> MapIOError<T> for std::io::Result<T> {
    fn map_io_err(self, msg: impl AsRef<str> + Debug) -> StringResult<T> {
        self.map_err(|io_err| {
            let mut err_msg = String::from("Fatal IO Error! ");
            err_msg.push_str(msg.as_ref());
            err_msg.push_str(&format!("\n  Cause: {:?}", io_err.to_string()));
            err_msg
        })
    }
}

pub fn must_resolve_path<P: AsRef<Path> + Debug>(string_path: P) -> StringResult<PathBuf> {
    string_path
        .as_ref()
        .try_resolve()
        .map(|path| path.into())
        .map_io_err(format!("Could not resolve path {:?}", string_path))
}

pub fn must_create_dir<P: AsRef<Path> + Debug>(string_path: P) -> StringResult<PathBuf> {
    let path = must_resolve_path(string_path)?;
    if path.exists() {
        return Ok(path);
    }
    create_dir_all(&path)
        .map(|_| {
            println!("Created dir: {path:?}");
            path.clone()
        })
        .map_io_err(format!("Could not create dir {path:?}"))
}
