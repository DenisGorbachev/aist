use crate::{CodexReviewer, CommandOutputError, ReviewSnapshot, ReviewVerdict, ReviewVerdictValidateError, command_output};
use errgonomic::handle;
use serde_json::{Error as JsonError, from_slice};
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tempfile::tempdir;
use thiserror::Error;
use tokio::fs::{read, write};
use tokio::process::Command;
use toml::Value;

pub async fn run_codex_review(reviewer: &CodexReviewer, policy_root: &Path, snapshot: &ReviewSnapshot, settings: &BTreeMap<String, Value>, prompt: &str, schema: &str) -> Result<ReviewVerdict, RunCodexReviewError> {
    use RunCodexReviewError::*;
    let directory = handle!(tempdir(), TempdirFailed);
    let prompt_path = directory.path().join("prompt.txt");
    let schema_path = directory.path().join("schema.json");
    let output_path = directory.path().join("verdict.json");
    let prompt = format!("{prompt}\nCaptured snapshot directory: {}\n", snapshot.directory().display());
    handle!(write(&prompt_path, prompt).await, WritePromptFailed, path: prompt_path.to_owned());
    handle!(write(&schema_path, schema).await, WriteSchemaFailed, path: schema_path.to_owned());
    let input = handle!(File::open(&prompt_path), OpenFailed, path: prompt_path);
    let mut command = Command::new(&reviewer.executable);
    command
        .args(["exec", "--ephemeral", "--color", "never", "--model"])
        .arg(&reviewer.model)
        .current_dir(policy_root)
        .arg("--output-schema")
        .arg(&schema_path)
        .arg("--output-last-message")
        .arg(&output_path)
        .stdin(Stdio::from(input))
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    settings.iter().for_each(|(key, value)| {
        command.arg("--config").arg(format!("{key}={value}"));
    });
    command.arg("-");
    handle!(command_output(command, reviewer.timeout).await, CommandOutputFailed);
    let bytes = handle!(read(&output_path).await, ReadFailed, path: output_path);
    let verdict: ReviewVerdict = handle!(from_slice(&bytes), FromSliceFailed, bytes);
    handle!(verdict.validate(snapshot), ValidateFailed, verdict);
    Ok(verdict)
}

#[derive(Error, Debug)]
pub enum RunCodexReviewError {
    #[error("failed to create Codex invocation directory")]
    TempdirFailed { source: io::Error },
    #[error("failed to write review prompt '{path}'", path = path.display())]
    WritePromptFailed { source: io::Error, path: PathBuf },
    #[error("failed to write review schema '{path}'", path = path.display())]
    WriteSchemaFailed { source: io::Error, path: PathBuf },
    #[error("failed to open review prompt '{path}'", path = path.display())]
    OpenFailed { source: io::Error, path: PathBuf },
    #[error("Codex review command failed")]
    CommandOutputFailed { source: Box<CommandOutputError> },
    #[error("failed to read Codex verdict '{path}'", path = path.display())]
    ReadFailed { source: io::Error, path: PathBuf },
    #[error("Codex verdict does not match the response schema")]
    FromSliceFailed { source: JsonError, bytes: Vec<u8> },
    #[error("Codex verdict is incomplete or cites uncaptured inputs")]
    ValidateFailed { source: ReviewVerdictValidateError, verdict: Box<ReviewVerdict> },
}
