use errgonomic::handle;
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
                Ok(()) => return Ok(file),
                Err(TryLockError::WouldBlock) => sleep(Duration::from_millis(50)).await,
                Err(TryLockError::Error(source)) => {
                    return Err(TryLockFailed {
                        source,
                        path: path.to_path_buf(),
                    });
                }
            }
        }
    };
    handle!(timeout(duration, wait).await, TimeoutFailed, path: path.to_path_buf())
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
