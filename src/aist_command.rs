use crate::{ListTypesAistCommandV1, ListTypesAistCommandV1RunError, ListTypesAistCommandV2, ListTypesAistCommandV2RunError, ListTypesAistCommandV3, ListTypesAistCommandV3RunError, ListTypesAistCommandV4, ListTypesAistCommandV4RunError, ListTypesAistCommandV5, ListTypesAistCommandV5RunError, ListTypesAistCommandV6, ListTypesAistCommandV6RunError};
use AistSubcommand::*;
use clap::{Parser, Subcommand};
use errgonomic::map_err;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Serialize, Deserialize, Clone, Debug)]
#[command(author, version, about, propagate_version = true, flatten_help = true, disable_help_subcommand = true)]
pub struct AistCommand {
    #[arg(short = 'p', long)]
    pub project_dir: PathBuf,
    #[command(subcommand)]
    pub subcommand: AistSubcommand,
}

#[derive(Subcommand, Serialize, Deserialize, Clone, Debug)]
pub enum AistSubcommand {
    ListTypesV1(ListTypesAistCommandV1),
    ListTypesV2(ListTypesAistCommandV2),
    ListTypesV3(ListTypesAistCommandV3),
    ListTypesV4(ListTypesAistCommandV4),
    ListTypesV5(ListTypesAistCommandV5),
    ListTypesV6(ListTypesAistCommandV6),
}

impl AistCommand {
    pub async fn run(self) -> Result<ExitCode, AistCommandRunError> {
        use AistCommandRunError::*;
        let Self {
            project_dir,
            subcommand,
        } = self;
        match subcommand {
            ListTypesV1(command) => map_err!(command.run(project_dir).await, ListTypesAistCommandV1RunFailed),
            ListTypesV2(command) => map_err!(command.run(project_dir).await, ListTypesAistCommandV2RunFailed),
            ListTypesV3(command) => map_err!(command.run(project_dir).await, ListTypesAistCommandV3RunFailed),
            ListTypesV4(command) => map_err!(command.run(project_dir).await, ListTypesAistCommandV4RunFailed),
            ListTypesV5(command) => map_err!(command.run(project_dir).await, ListTypesAistCommandV5RunFailed),
            ListTypesV6(command) => map_err!(command.run(project_dir).await, ListTypesAistCommandV6RunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum AistCommandRunError {
    #[error("failed to run list-types implementation V1")]
    ListTypesAistCommandV1RunFailed { source: ListTypesAistCommandV1RunError },
    #[error("failed to run list-types implementation V2")]
    ListTypesAistCommandV2RunFailed { source: ListTypesAistCommandV2RunError },
    #[error("failed to run list-types implementation V3")]
    ListTypesAistCommandV3RunFailed { source: ListTypesAistCommandV3RunError },
    #[error("failed to run list-types implementation V4")]
    ListTypesAistCommandV4RunFailed { source: ListTypesAistCommandV4RunError },
    #[error("failed to run list-types implementation V5")]
    ListTypesAistCommandV5RunFailed { source: ListTypesAistCommandV5RunError },
    #[error("failed to run list-types implementation V6")]
    ListTypesAistCommandV6RunFailed { source: ListTypesAistCommandV6RunError },
}
