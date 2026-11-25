use errgonomic::{handle, map_err};
use std::fs::{File, TryLockError};
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;
use tokio::fs::OpenOptions;
use tokio::time::{error::Elapsed, sleep, timeout};

pub async fn acquire_cache_lock(path: &Path, duration: Duration) -> Result<File, AcquireCacheLockError> {
    use AcquireCacheLockError::*;
    let file = handle!(OpenOptions::new().create(true).truncate(false).read(true).write(true).open(path).await, OpenFailed, path: path.to_path_buf())
        .into_std()
        .await;
    let wait = async {
        loop {
            match file.try_lock() {
                Ok(()) => break Ok(file),
                Err(TryLockError::WouldBlock) => sleep(Duration::from_millis(50)).await,
                Err(TryLockError::Error(source)) => break Err(source),
            }
        }
    };
    let result = handle!(timeout(duration, wait).await, TimeoutFailed, path: path.to_path_buf());
    map_err!(result, TryLockFailed, path: path.to_path_buf())
}

#[derive(Error, Debug)]
pub enum AcquireCacheLockError {
    #[error("failed to open cache lock '{path}'", path = path.display())]
    OpenFailed { source: io::Error, path: PathBuf },
    #[error("failed to lock cache entry '{path}'", path = path.display())]
    TryLockFailed { source: io::Error, path: PathBuf },
    #[error("timed out waiting for cache lock '{path}'", path = path.display())]
    TimeoutFailed { source: Elapsed, path: PathBuf },
}
