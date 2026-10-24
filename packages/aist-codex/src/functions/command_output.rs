use errgonomic::{handle, handle_bool, render_command};
use std::io;
use std::process::{Command, ExitStatus};
use std::time::Duration;
use thiserror::Error;
use tokio::process::Command as TokioCommand;
use tokio::time::{error::Elapsed, timeout};

pub async fn command_output(mut command: TokioCommand, duration: Duration) -> Result<Vec<u8>, CommandOutputError> {
    use CommandOutputError::*;
    command.kill_on_drop(true);
    let result = handle!(timeout(duration, command.output()).await, TimeoutFailed, command: command.into_std());
    let output = handle!(result, OutputFailed, command: command.into_std());
    handle_bool!(!output.status.success(), CommandFailed, command: command.into_std(), status: output.status, stderr: String::from_utf8_lossy(&output.stderr).into_owned());
    Ok(output.stdout)
}

#[derive(Error, Debug)]
pub enum CommandOutputError {
    #[error("command timed out: `{command}`", command = render_command(command))]
    TimeoutFailed { source: Elapsed, command: Box<Command> },
    #[error("failed to execute `{command}`", command = render_command(command))]
    OutputFailed { source: io::Error, command: Box<Command> },
    #[error("`{command}` exited with {status}: {stderr}", command = render_command(command))]
    CommandFailed { command: Box<Command>, status: ExitStatus, stderr: String },
}
