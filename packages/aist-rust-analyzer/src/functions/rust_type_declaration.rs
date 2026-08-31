use crate::{LocatedRustTypeDeclaration, RustSourceByteOffset, RustTypeDeclaration, RustTypeDefinition};
use errgonomic::handle_opt;
use ra_ap_hir::Semantics;
use ra_ap_ide_db::{FileId, RootDatabase};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

pub fn rust_type_declaration(db: &RootDatabase, semantics: &Semantics<'_, RootDatabase>, source_paths: &HashMap<FileId, PathBuf>, definition: RustTypeDefinition) -> Result<LocatedRustTypeDeclaration, RustTypeDeclarationError> {
    use RustTypeDeclarationError::*;
    let syntax = definition.location.syntax(semantics);
    let source_range = semantics.original_range(&syntax).into_file_id(db);
    let path = handle_opt!(source_paths.get(&source_range.file_id), SourcePathNotFound, definition, file_id: source_range.file_id).to_path_buf();
    Ok(LocatedRustTypeDeclaration {
        path,
        declaration: RustTypeDeclaration {
            name: definition.name,
            kind: definition.kind,
            byte_offset: RustSourceByteOffset::from(source_range.range.start()),
        },
    })
}

#[derive(Error, Debug)]
pub enum RustTypeDeclarationError {
    #[error("source path for macro-expanded Rust type '{name}' was not found for file {file_id:?}", name = definition.name)]
    SourcePathNotFound { definition: RustTypeDefinition, file_id: FileId },
}
