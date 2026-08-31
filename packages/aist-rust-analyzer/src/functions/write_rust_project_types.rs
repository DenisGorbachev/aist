use crate::RustProjectTypes;
use errgonomic::handle;
use std::io::{self, Write};
use thiserror::Error;

/// PRUNING: drops the project type listing after serializing it because the caller only needs the emitted output.
pub fn write_rust_project_types(writer: &mut impl Write, project_types: RustProjectTypes) -> Result<(), WriteRustProjectTypesError> {
    use WriteRustProjectTypesError::*;
    handle!(serde_json::to_writer_pretty(&mut *writer, &project_types), ToWriterPrettyFailed, project_types);
    handle!(writer.write_all(b"\n"), WriteAllFailed, project_types);
    Ok(())
}

#[derive(Error, Debug)]
pub enum WriteRustProjectTypesError {
    #[error("failed to serialize the Rust project type listing")]
    ToWriterPrettyFailed { source: serde_json::Error, project_types: RustProjectTypes },
    #[error("failed to terminate the Rust project type listing with a newline")]
    WriteAllFailed { source: io::Error, project_types: RustProjectTypes },
}
