use crate::{CacheEntry, CanonicalJsonError, canonical_json};
use errgonomic::{handle, handle_opt};
use std::io;
use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, PersistError};
use thiserror::Error;
use tokio::fs::write;

pub async fn store_cached_review(path: &Path, entry: &CacheEntry) -> Result<(), StoreCachedReviewError> {
    use StoreCachedReviewError::*;
    let parent = handle_opt!(path.parent(), ParentNotFound, path: path.to_path_buf());
    let temporary = handle!(NamedTempFile::new_in(parent), NewInFailed, path: path.to_path_buf());
    let bytes = handle!(canonical_json(entry), CanonicalJsonFailed);
    handle!(write(temporary.path(), &bytes).await, WriteFailed, path: temporary.path().to_path_buf(), bytes);
    handle!(temporary.as_file().sync_all(), SyncAllFailed, path: temporary.path().to_path_buf());
    handle!(temporary.persist(path), PersistFailed, path: path.to_path_buf());
    Ok(())
}

#[derive(Error, Debug)]
pub enum StoreCachedReviewError {
    #[error("cache path has no parent: '{path}'", path = path.display())]
    ParentNotFound { path: PathBuf },
    #[error("failed to create temporary cache entry for '{path}'", path = path.display())]
    NewInFailed { source: io::Error, path: PathBuf },
    #[error("failed to encode cached review")]
    CanonicalJsonFailed { source: CanonicalJsonError },
    #[error("failed to write temporary cache entry '{path}'", path = path.display())]
    WriteFailed { source: io::Error, path: PathBuf, bytes: Vec<u8> },
    #[error("failed to sync temporary cache entry '{path}'", path = path.display())]
    SyncAllFailed { source: io::Error, path: PathBuf },
    #[error("failed to atomically publish cache entry '{path}'", path = path.display())]
    PersistFailed { source: PersistError, path: PathBuf },
}
