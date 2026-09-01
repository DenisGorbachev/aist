use crate::{SpecReport, UnwrapOrCurrentDirError, unwrap_or_current_dir};
use aist_core::{TryFromPathForWorkspaceInfoError, WorkspaceInfo};
use clap::Parser;
use errgonomic::{PathBufDisplay, handle};
use save_load::errors::save_one_error::SaveOneError;
use save_load::format::Format;
use serde::{Deserialize, Serialize};
use std::io::stdout;
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

#[derive(Parser, Serialize, Deserialize, Clone, Debug)]
#[command(author, version, about, propagate_version = true, flatten_help = true, disable_help_subcommand = true)]
pub struct SpecCommand {
    #[arg(short = 'p', long)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_root: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = Format::Yaml)]
    pub output_format: Format,
}

impl SpecCommand {
    pub async fn run(self) -> Result<ExitCode, SpecCommandRunError> {
        use SpecCommandRunError::*;
        let Self {
            project_root,
            output_format,
        } = self;
        let project_root = handle!(unwrap_or_current_dir(project_root), UnwrapOrCurrentDirFailed);
        let workspace_info = handle!(WorkspaceInfo::try_from(project_root.as_path()), TryFromFailed, project_root);
        let report = SpecReport::new(&workspace_info);
        handle!(output_format.writeln_one(&mut stdout().lock(), &report), WritelnOneFailed, output_format, report);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum SpecCommandRunError {
    #[error("failed to resolve the project root")]
    UnwrapOrCurrentDirFailed { source: UnwrapOrCurrentDirError },
    #[error("failed to load project root '{project_root}'")]
    TryFromFailed { source: TryFromPathForWorkspaceInfoError, project_root: PathBufDisplay },
    #[error("failed to write the specification report as {output_format}")]
    WritelnOneFailed { source: SaveOneError, output_format: Format, report: SpecReport },
}
