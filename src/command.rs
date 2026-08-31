use Subcommand::*;
use clap::Parser;
use errgonomic::map_err;
use save_load::format::Format;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Serialize, Deserialize, Clone, Debug)]
#[command(author, version, about, propagate_version = true, flatten_help = true, disable_help_subcommand = true)]
pub struct Command {
    #[arg(short = 'p', long)]
    pub project_root: PathBuf,
    #[arg(long, value_enum, default_value_t = Format::Yaml)]
    pub output_format: Format,
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(clap::Subcommand, Serialize, Deserialize, Clone, Debug)]
pub enum Subcommand {
    ListTypes(ListTypesCommand),
}

impl Command {
    pub async fn run(self) -> Result<ExitCode, CommandRunError> {
        use CommandRunError::*;
        let Self {
            project_root,
            output_format,
            subcommand,
        } = self;
        match subcommand {
            ListTypes(command) => map_err!(command.run(project_root, output_format).await, ListTypesCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to run list-types command")]
    ListTypesCommandRunFailed { source: ListTypesCommandRunError },
}

mod list_types_command;
pub use list_types_command::*;
