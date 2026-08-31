use crate::{ListRustProjectTypesError, RustTypeDefinitionCollector, WriteRustProjectTypesError, list_rust_project_types, write_rust_project_types};
use errgonomic::handle;
use std::io::stdout;
use std::path::Path;
use std::process::ExitCode;
use thiserror::Error;

pub fn run_list_types<C: RustTypeDefinitionCollector>(project_dir: &Path) -> Result<ExitCode, RunListTypesError> {
    use RunListTypesError::*;
    let project_types = handle!(list_rust_project_types::<C>(project_dir), ListRustProjectTypesFailed);
    let stdout = stdout();
    let mut stdout = stdout.lock();
    handle!(write_rust_project_types(&mut stdout, project_types), WriteRustProjectTypesFailed);
    Ok(ExitCode::SUCCESS)
}

#[derive(Error, Debug)]
pub enum RunListTypesError {
    #[error("failed to list Rust types")]
    ListRustProjectTypesFailed { source: ListRustProjectTypesError },
    #[error("failed to write the Rust project type listing")]
    WriteRustProjectTypesFailed { source: WriteRustProjectTypesError },
}
