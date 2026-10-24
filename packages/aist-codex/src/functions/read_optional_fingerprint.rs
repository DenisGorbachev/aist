use crate::Fingerprint;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs::read;

pub async fn read_optional_fingerprint(path: &Path) -> Result<Option<Fingerprint>, ReadOptionalFingerprintError> {
    use ReadOptionalFingerprintError::*;
    match read(path).await {
        Ok(bytes) => Ok(Some(Fingerprint::from(bytes.as_slice()))),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(source) => Err(ReadFailed {
            source,
            path: path.to_path_buf(),
        }),
    }
}

#[derive(Error, Debug)]
pub enum ReadOptionalFingerprintError {
    #[error("failed to read fingerprint input '{path}'", path = path.display())]
    ReadFailed { source: io::Error, path: PathBuf },
}
