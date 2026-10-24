use crate::ReviewSource;
use errgonomic::{handle, handle_bool, handle_opt};
use std::io;
use std::path::{Component, Path, PathBuf};
use thiserror::Error;
use tokio::fs::{create_dir_all, read, write};

pub async fn copy_review_source(directory: &Path, name: &Path, source: &Path) -> Result<ReviewSource, CopyReviewSourceError> {
    use CopyReviewSourceError::*;
    handle_bool!(name.file_name().is_none() || !name.components().all(|part| matches!(part, Component::Normal(_))), NameInvalid, name: name.to_path_buf());
    let destination = directory.join("sources").join(name);
    let parent = handle_opt!(destination.parent(), ParentNotFound, destination);
    handle!(create_dir_all(parent).await, CreateDirAllFailed, path: parent.to_path_buf());
    let bytes = handle!(read(source).await, ReadFailed, path: source.to_path_buf());
    handle!(write(&destination, &bytes).await, WriteFailed, path: destination);
    Ok(ReviewSource::from(bytes.as_slice()))
}

#[derive(Error, Debug)]
pub enum CopyReviewSourceError {
    #[error("snapshot name must be a nonempty relative path without parent components: '{name}'", name = name.display())]
    NameInvalid { name: PathBuf },
    #[error("parent of snapshot destination not found: '{destination}'", destination = destination.display())]
    ParentNotFound { destination: PathBuf },
    #[error("failed to create snapshot directory '{path}'", path = path.display())]
    CreateDirAllFailed { source: io::Error, path: PathBuf },
    #[error("failed to read snapshot input '{path}'", path = path.display())]
    ReadFailed { source: io::Error, path: PathBuf },
    #[error("failed to write snapshot input '{path}'", path = path.display())]
    WriteFailed { source: io::Error, path: PathBuf },
}
