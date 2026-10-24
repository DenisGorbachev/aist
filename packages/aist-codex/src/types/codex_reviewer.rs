use crate::{AcquireCacheLockError, CacheEntry, CachedReview, CommandOutputError, HashJsonError, LoadCachedReviewError, ReviewRequest, ReviewSnapshot, ReviewSnapshotVerifyError, ReviewStatus, RunCodexReviewError, StoreCachedReviewError, acquire_cache_lock, command_output, hash_json, load_cached_review, run_codex_review, store_cached_review};
use errgonomic::{handle, handle_bool};
use futures::{Stream, StreamExt, stream};
use std::collections::BTreeMap;
use std::io;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use tokio::fs::create_dir_all;
use toml::Value;

#[derive(Debug)]
pub struct CodexReviewer {
    pub executable: PathBuf,
    pub model: String,
    pub reasoning_effort: String,
    pub cache_directory: PathBuf,
    pub cache_epoch: String,
    pub max_age: Duration,
    pub timeout: Duration,
}

impl CodexReviewer {
    pub fn settings(&self) -> BTreeMap<String, Value> {
        [
            ("approval_policy", Value::from("never")),
            ("default_permissions", Value::from("aist-review")),
            (
                "permissions",
                Value::Table(
                    [(
                        "aist-review".into(),
                        Value::Table(
                            [
                                (
                                    "filesystem".into(),
                                    Value::Table(
                                        [
                                            (":root".into(), Value::from("deny")),
                                            (":minimal".into(), Value::from("read")),
                                            (":workspace_roots".into(), Value::from("read")),
                                        ]
                                        .into_iter()
                                        .collect(),
                                    ),
                                ),
                                (
                                    "network".into(),
                                    Value::Table(
                                        [("enabled".into(), Value::from(false))]
                                            .into_iter()
                                            .collect(),
                                    ),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    )]
                    .into_iter()
                    .collect(),
                ),
            ),
            ("model_reasoning_effort", Value::from(self.reasoning_effort.as_str())),
            ("project_doc_max_bytes", Value::from(0)),
            ("web_search", Value::from("disabled")),
            ("features.multi_agent", Value::from(false)),
            ("features.multi_agent_v2", Value::from(false)),
            ("features.skill_search", Value::from(false)),
            ("features.skip_host_skill_discovery", Value::from(true)),
            ("features.shell_snapshot", Value::from(false)),
            ("shell_environment_policy.inherit", Value::from("none")),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect()
    }

    pub async fn check(&self, snapshot: &ReviewSnapshot, request: &ReviewRequest) -> Result<CachedReview, CodexReviewerCheckError> {
        use CodexReviewerCheckError::*;
        handle_bool!(request.id.trim().is_empty() || request.question.trim().is_empty() || self.model.trim().is_empty() || self.reasoning_effort.trim().is_empty(), RequestInvalid, request: request.to_owned());
        handle!(snapshot.verify().await, VerifyFailed);
        let version = handle!(command_output(self.executable.as_os_str(), &["--version".into()], snapshot.directory(), self.timeout).await, CommandOutputFailed);
        let settings = self.settings();
        let schema = include_str!("../review_schema.json");
        let prompt = format!("{}\n\nReview ID: {}\nRequirement:\n{}\n", include_str!("../review_prompt.md"), request.id, request.question);
        let key = handle!(hash_json(&("aist-codex-review-v1", snapshot.fingerprint(), &self.executable, &version, &self.model, &self.cache_epoch, &settings, schema, &prompt)), HashJsonFailed);
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
        let verdict = handle!(run_codex_review(self, snapshot, &settings, &prompt, schema).await, RunCodexReviewFailed, request: request.to_owned());
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

    pub fn check_many<'a, I>(&'a self, snapshot: &'a ReviewSnapshot, requests: I, concurrency: NonZeroUsize) -> impl Stream<Item = (ReviewRequest, Result<CachedReview, CodexReviewerCheckError>)> + 'a
    where
        I: IntoIterator<Item = ReviewRequest>,
        I::IntoIter: 'a,
    {
        stream::iter(requests)
            .map(move |request| async move {
                let result = self.check(snapshot, &request).await;
                (request, result)
            })
            .buffer_unordered(concurrency.get())
    }
}

#[derive(Error, Debug)]
pub enum CodexReviewerCheckError {
    #[error("review ID, question, model, and reasoning effort must be nonempty")]
    RequestInvalid { request: ReviewRequest },
    #[error("captured inputs are no longer valid")]
    VerifyFailed { source: ReviewSnapshotVerifyError },
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
