use crate::{CanonicalJsonError, CopyReviewSourceError, Fingerprint, VerifyFingerprintError, canonical_json, copy_review_source, verify_fingerprint};
use errgonomic::{ErrVec, handle, handle_iter};
use futures::{StreamExt, stream};
use serde::Serialize;
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};
use tempfile::{TempDir, tempdir};
use thiserror::Error;
use tokio::fs::write;

#[derive(Debug)]
pub struct ReviewSnapshot {
    directory: TempDir,
    files: BTreeMap<PathBuf, Fingerprint>,
    fingerprint: Fingerprint,
}

impl ReviewSnapshot {
    pub async fn capture(sources: &BTreeMap<PathBuf, PathBuf>, context: &impl Serialize) -> Result<Self, ReviewSnapshotCaptureError> {
        use ReviewSnapshotCaptureError::*;
        let directory = handle!(tempdir(), TempdirFailed);
        let results = stream::iter(sources.iter())
            .then(|(name, source)| async {
                copy_review_source(directory.path(), name, source)
                    .await
                    .map(|fingerprint| (name.to_owned(), fingerprint))
            })
            .collect::<Vec<_>>()
            .await;
        let files = handle_iter!(results.into_iter(), CopyReviewSourceFailed)
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        let bytes = handle!(canonical_json(&(context, &files)), CanonicalJsonFailed);
        let fingerprint = Fingerprint::from(bytes.as_slice());
        let path = directory.path().join("context.json");
        handle!(write(&path, &bytes).await, WriteFailed, path);
        Ok(Self {
            directory,
            files,
            fingerprint,
        })
    }

    pub fn directory(&self) -> &Path {
        self.directory.path()
    }

    pub fn files(&self) -> &BTreeMap<PathBuf, Fingerprint> {
        &self.files
    }

    pub fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    pub async fn verify(&self) -> Result<(), ReviewSnapshotVerifyError> {
        use ReviewSnapshotVerifyError::*;
        let results = stream::iter(&self.files)
            .then(|(name, fingerprint)| async move { verify_fingerprint(&self.directory.path().join("sources").join(name), *fingerprint).await })
            .collect::<Vec<_>>()
            .await;
        handle_iter!(results.into_iter(), VerifyFingerprintFailed);
        handle!(verify_fingerprint(&self.directory.path().join("context.json"), self.fingerprint).await, VerifyContextFailed);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ReviewSnapshotCaptureError {
    #[error("failed to create isolated review snapshot")]
    TempdirFailed { source: io::Error },
    #[error("failed to capture review sources")]
    CopyReviewSourceFailed { source: ErrVec<CopyReviewSourceError> },
    #[error("failed to encode snapshot context")]
    CanonicalJsonFailed { source: CanonicalJsonError },
    #[error("failed to write snapshot context '{path}'", path = path.display())]
    WriteFailed { source: io::Error, path: PathBuf },
}

#[derive(Error, Debug)]
pub enum ReviewSnapshotVerifyError {
    #[error("review sources changed after capture")]
    VerifyFingerprintFailed { source: ErrVec<VerifyFingerprintError> },
    #[error("review context changed after capture")]
    VerifyContextFailed { source: VerifyFingerprintError },
}
