use crate::{ListRustProjectTypesError, WriteRustProjectTypesError, list_rust_project_types, write_rust_project_types};
use clap::Parser;
use errgonomic::handle;
use serde::{Deserialize, Serialize};
use std::io::stdout;
use std::path::PathBuf;
use std::process::ExitCode;
use thiserror::Error;

/// Lists macro-expanded Rust type declarations in a project.
#[derive(Parser, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[command(flatten_help = true)]
pub struct ListTypesAistCommand;

impl ListTypesAistCommand {
    pub async fn run(self, project_dir: PathBuf) -> Result<ExitCode, ListTypesAistCommandRunError> {
        use ListTypesAistCommandRunError::*;
        let Self = self;
        let project_types = handle!(list_rust_project_types(&project_dir), ListRustProjectTypesFailed);
        let stdout = stdout();
        let mut stdout = stdout.lock();
        handle!(write_rust_project_types(&mut stdout, project_types), WriteRustProjectTypesFailed);
        Ok(ExitCode::SUCCESS)
    }
}

#[derive(Error, Debug)]
pub enum ListTypesAistCommandRunError {
    #[error("failed to list Rust types")]
    ListRustProjectTypesFailed { source: ListRustProjectTypesError },
    #[error("failed to write the Rust project type listing")]
    WriteRustProjectTypesFailed { source: WriteRustProjectTypesError },
}
