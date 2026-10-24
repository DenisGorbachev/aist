use crate::Fingerprint;
use errgonomic::{handle, handle_bool};
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs::read;

pub async fn verify_fingerprint(path: &Path, expected: Fingerprint) -> Result<(), VerifyFingerprintError> {
    use VerifyFingerprintError::*;
    let bytes = handle!(read(path).await, ReadFailed, path: path.to_path_buf());
    let actual = Fingerprint::from(bytes.as_slice());
    handle_bool!(actual != expected, FingerprintInvalid, path: path.to_path_buf(), actual);
    Ok(())
}

#[derive(Error, Debug)]
pub enum VerifyFingerprintError {
    #[error("failed to read captured review input '{path}'", path = path.display())]
    ReadFailed { source: io::Error, path: PathBuf },
    #[error("captured review input changed: '{path}'", path = path.display())]
    FingerprintInvalid { path: PathBuf, actual: Fingerprint },
}
