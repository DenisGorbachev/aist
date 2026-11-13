use errgonomic::handle;
use serde::de::DeserializeOwned;
use std::fs::read_to_string;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use toml::{de, from_str};

pub fn read_toml<T: DeserializeOwned>(path: &Path) -> Result<T, ReadTomlError> {
    use ReadTomlError::*;
    let text = handle!(read_to_string(path), ReadToStringFailed, path: path.to_path_buf());
    Ok(handle!(from_str(&text), FromStrFailed, path: path.to_path_buf(), text))
}

#[derive(Error, Debug)]
pub enum ReadTomlError {
    #[error("failed to read '{path}'", path = path.display())]
    ReadToStringFailed { source: io::Error, path: PathBuf },
    #[error("failed to parse '{path}'", path = path.display())]
    FromStrFailed { source: Box<de::Error>, path: PathBuf, text: String },
}
