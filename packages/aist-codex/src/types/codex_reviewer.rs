use crate::{AcquireCacheLockError, CacheEntry, CachedReview, CommandOutputError, HashJsonError, LoadCachedReviewError, ReviewRequest, ReviewSnapshot, ReviewSnapshotVerifyError, ReviewStatus, RunCodexReviewError, StoreCachedReviewError, acquire_cache_lock, command_output, hash_json, load_cached_review, run_codex_review, store_cached_review};
use errgonomic::{handle, handle_bool};
use std::collections::BTreeMap;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use tokio::fs::{canonicalize, create_dir_all};
use tokio::process::Command;
use toml::Value;

#[derive(Debug)]
pub struct CodexReviewer {
    pub executable: PathBuf,
    pub policy_root: PathBuf,
    pub model: String,
    pub reasoning_effort: String,
    pub cache_directory: PathBuf,
    pub cache_epoch: String,
    pub max_age: Duration,
    pub timeout: Duration,
}

impl CodexReviewer {
    pub fn settings(&self) -> BTreeMap<String, Value> {
        [("model_reasoning_effort".into(), Value::from(self.reasoning_effort.as_str()))]
            .into_iter()
            .collect()
    }

    pub async fn check(&self, snapshot: &ReviewSnapshot, request: &ReviewRequest) -> Result<CachedReview, CodexReviewerCheckError> {
        use CodexReviewerCheckError::*;
        handle_bool!(request.id.trim().is_empty() || request.question.trim().is_empty() || self.model.trim().is_empty() || self.reasoning_effort.trim().is_empty(), RequestInvalid, request: request.to_owned());
        handle!(snapshot.verify().await, VerifyFailed);
        let policy_root = handle!(canonicalize(&self.policy_root).await, CanonicalizeFailed, path: self.policy_root.to_owned());
        let mut command = Command::new(&self.executable);
        command.arg("--version").current_dir(&policy_root);
        let version = handle!(command_output(command, self.timeout).await, CommandOutputFailed);
        let settings = self.settings();
        let schema = include_str!("../review_schema.json");
        let prompt = format!("{}\n\nReview ID: {}\nRequirement:\n{}\n", include_str!("../review_prompt.md"), request.id, request.question);
        let key = handle!(hash_json(&("aist-codex-review-v2", snapshot.fingerprint(), &policy_root, &self.executable, &version, &self.model, &self.cache_epoch, &settings, schema, &prompt)), HashJsonFailed);
        handle!(create_dir_all(&self.cache_directory).await, CreateDirAllFailed, path: self.cache_directory.to_owned());
        let path = self.cache_directory.join(format!("{key}.json"));
        let lock_path = self.cache_directory.join(format!("{key}.lock"));
        let _lock = handle!(acquire_cache_lock(&lock_path, self.timeout).await, AcquireCacheLockFailed);
        if let Some(entry) = handle!(load_cached_review(&path, key, snapshot, self.max_age).await, LoadCachedReviewFailed) {
            return Ok(CachedReview {
                key,
                cached: true,
                verdict: entry.verdict,
            });
        }
        let verdict = handle!(run_codex_review(self, &policy_root, snapshot, &settings, &prompt, schema).await, RunCodexReviewFailed, request: request.to_owned());
        handle!(snapshot.verify().await, VerifyAfterReviewFailed);
        let entry = CacheEntry {
            key,
            created: SystemTime::now(),
            verdict,
        };
        if entry.verdict.status != ReviewStatus::Inconclusive {
            handle!(store_cached_review(&path, &entry).await, StoreCachedReviewFailed);
        }
        Ok(CachedReview {
            key,
            cached: false,
            verdict: entry.verdict,
        })
    }
}

#[derive(Error, Debug)]
pub enum CodexReviewerCheckError {
    #[error("review ID, question, model, and reasoning effort must be nonempty")]
    RequestInvalid { request: ReviewRequest },
    #[error("captured inputs are no longer valid")]
    VerifyFailed { source: ReviewSnapshotVerifyError },
    #[error("failed to resolve policy directory '{path}'", path = path.display())]
    CanonicalizeFailed { source: io::Error, path: PathBuf },
    #[error("failed to identify Codex harness")]
    CommandOutputFailed { source: Box<CommandOutputError> },
    #[error("failed to compute review cache key")]
    HashJsonFailed { source: HashJsonError },
    #[error("failed to create cache directory '{path}'", path = path.display())]
    CreateDirAllFailed { source: io::Error, path: PathBuf },
    #[error("failed to acquire review cache lock")]
    AcquireCacheLockFailed { source: AcquireCacheLockError },
    #[error("failed to load cached review")]
    LoadCachedReviewFailed { source: LoadCachedReviewError },
    #[error("Codex review failed for '{id}'", id = request.id)]
    RunCodexReviewFailed { source: RunCodexReviewError, request: ReviewRequest },
    #[error("captured inputs changed during review")]
    VerifyAfterReviewFailed { source: ReviewSnapshotVerifyError },
    #[error("failed to cache review")]
    StoreCachedReviewFailed { source: StoreCachedReviewError },
}
