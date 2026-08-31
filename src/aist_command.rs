use crate::{ListTypesAistCommand, ListTypesAistCommandRunError};
use AistSubcommand::*;
use clap::{Parser, Subcommand};
use errgonomic::map_err;
use save_load::format::Format;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Serialize, Deserialize, Clone, Debug)]
#[command(author, version, about, propagate_version = true, flatten_help = true, disable_help_subcommand = true)]
pub struct AistCommand {
    #[arg(short = 'p', long)]
    pub project_dir: PathBuf,
    #[arg(long, value_enum, default_value_t = Format::Yaml)]
    pub output_format: Format,
    #[command(subcommand)]
    pub subcommand: AistSubcommand,
}

#[derive(Subcommand, Serialize, Deserialize, Clone, Debug)]
pub enum AistSubcommand {
    ListTypes(ListTypesAistCommand),
}

impl AistCommand {
    pub async fn run(self) -> Result<ExitCode, AistCommandRunError> {
        use AistCommandRunError::*;
        let Self {
            project_dir,
            output_format,
            subcommand,
        } = self;
        match subcommand {
            ListTypes(command) => map_err!(command.run(project_dir, output_format).await, ListTypesAistCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum AistCommandRunError {
    #[error("failed to run list-types command")]
    ListTypesAistCommandRunFailed { source: ListTypesAistCommandRunError },
}
