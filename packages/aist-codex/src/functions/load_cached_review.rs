use crate::{CacheEntry, Fingerprint, ReviewSnapshot, ReviewStatus, ReviewVerdictValidateError};
use errgonomic::{handle, handle_bool};
use serde_json::{Error as JsonError, from_slice};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, SystemTimeError};
use thiserror::Error;
use tokio::fs::read;

pub async fn load_cached_review(path: &Path, key: Fingerprint, snapshot: &ReviewSnapshot, max_age: Duration) -> Result<Option<CacheEntry>, LoadCachedReviewError> {
    use LoadCachedReviewError::*;
    let bytes = match read(path).await {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        result => handle!(result, ReadFailed, path: path.to_path_buf()),
    };
    let entry: CacheEntry = handle!(from_slice(&bytes), FromSliceFailed, path: path.to_path_buf(), bytes);
    handle_bool!(entry.key != key || entry.verdict.status == ReviewStatus::Inconclusive, EntryInvalid, entry);
    handle!(entry.verdict.validate(snapshot), ValidateFailed, entry);
    let age = handle!(SystemTime::now().duration_since(entry.created), DurationSinceFailed, entry);
    Ok((age <= max_age).then_some(entry))
}

#[derive(Error, Debug)]
pub enum LoadCachedReviewError {
    #[error("failed to read cached review '{path}'", path = path.display())]
    ReadFailed { source: io::Error, path: PathBuf },
    #[error("cached review is not valid JSON: '{path}'", path = path.display())]
    FromSliceFailed { source: JsonError, path: PathBuf, bytes: Vec<u8> },
    #[error("cached review has an incorrect key or an inconclusive verdict")]
    EntryInvalid { entry: Box<CacheEntry> },
    #[error("cached review failed validation")]
    ValidateFailed { source: ReviewVerdictValidateError, entry: Box<CacheEntry> },
    #[error("cached review timestamp is in the future")]
    DurationSinceFailed { source: SystemTimeError, entry: Box<CacheEntry> },
}
