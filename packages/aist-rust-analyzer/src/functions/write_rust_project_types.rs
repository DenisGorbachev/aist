use crate::RustProjectTypes;
use errgonomic::handle;
use save_load::errors::save_one_error::SaveOneError;
use save_load::format::Format;
use std::io::Write;
use thiserror::Error;

/// PRUNING: drops the project type listing after serializing it because the caller only needs the emitted output.
pub fn write_rust_project_types(writer: &mut impl Write, project_types: RustProjectTypes, output_format: Format) -> Result<(), WriteRustProjectTypesError> {
    use WriteRustProjectTypesError::*;
    handle!(output_format.writeln_one(writer, &project_types), WritelnOneFailed, output_format, project_types);
    Ok(())
}

#[derive(Error, Debug)]
pub enum WriteRustProjectTypesError {
    #[error("failed to write the Rust project type listing as {output_format}")]
    WritelnOneFailed { source: SaveOneError, output_format: Format, project_types: RustProjectTypes },
}
