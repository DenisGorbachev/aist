use errgonomic::map_err;
use std::env::current_dir;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub fn unwrap_or_current_dir(project_root: Option<PathBuf>) -> Result<PathBuf, UnwrapOrCurrentDirError> {
    use UnwrapOrCurrentDirError::*;
    project_root.map_or_else(|| map_err!(current_dir(), CurrentDirFailed), Ok)
}

#[derive(Error, Debug)]
pub enum UnwrapOrCurrentDirError {
    #[error("failed to get the current directory")]
    CurrentDirFailed { source: io::Error },
}
