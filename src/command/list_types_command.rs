use crate::{ListRustProjectTypesError, RustProjectTypes, list_rust_project_types};
use clap::Parser;
use errgonomic::handle;
use save_load::errors::save_one_error::SaveOneError;
use save_load::format::Format;
use serde::{Deserialize, Serialize};
use std::io::stdout;
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

/// Lists macro-expanded Rust type declarations in a project.
#[derive(Parser, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[command(flatten_help = true)]
pub struct ListTypesCommand;

impl ListTypesCommand {
    pub async fn run(self, project_root: PathBuf, output_format: Format) -> Result<ExitCode, ListTypesCommandRunError> {
        use ListTypesCommandRunError::*;
        let Self = self;
        let project_types = handle!(list_rust_project_types(&project_root), ListRustProjectTypesFailed);
        let stdout = stdout();
        let mut stdout = stdout.lock();
        handle!(output_format.writeln_one(&mut stdout, &project_types), WritelnOneFailed, output_format, project_types);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum ListTypesCommandRunError {
    #[error("failed to list Rust types")]
    ListRustProjectTypesFailed { source: ListRustProjectTypesError },
    #[error("failed to write the Rust project type listing as {output_format}")]
    WritelnOneFailed { source: SaveOneError, output_format: Format, project_types: RustProjectTypes },
}
