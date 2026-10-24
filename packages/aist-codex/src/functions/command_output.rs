use errgonomic::{handle, handle_bool};
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitStatus;
use std::time::Duration;
use thiserror::Error;
use tokio::process::Command;
use tokio::time::{error::Elapsed, timeout};

pub async fn command_output(program: &OsStr, args: &[OsString], directory: &Path, duration: Duration) -> Result<Vec<u8>, CommandOutputError> {
    use CommandOutputError::*;
    let mut command = Command::new(program);
    command.args(args).current_dir(directory).kill_on_drop(true);
    let result = handle!(timeout(duration, command.output()).await, TimeoutFailed, program: program.to_os_string(), args: args.to_vec(), directory: directory.to_path_buf());
    let output = handle!(result, OutputFailed, program: program.to_os_string(), args: args.to_vec(), directory: directory.to_path_buf());
    handle_bool!(!output.status.success(), CommandFailed, program: program.to_os_string(), args: args.to_vec(), directory: directory.to_path_buf(), status: output.status, stderr: String::from_utf8_lossy(&output.stderr).into_owned());
    Ok(output.stdout)
}

#[derive(Error, Debug)]
pub enum CommandOutputError {
    #[error("command timed out in '{directory}'", directory = directory.display())]
    TimeoutFailed { source: Elapsed, program: OsString, args: Vec<OsString>, directory: PathBuf },
    #[error("failed to execute command in '{directory}'", directory = directory.display())]
    OutputFailed { source: io::Error, program: OsString, args: Vec<OsString>, directory: PathBuf },
    #[error("command exited with {status}: {stderr}")]
    CommandFailed { program: OsString, args: Vec<OsString>, directory: PathBuf, status: ExitStatus, stderr: String },
}
