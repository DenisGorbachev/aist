use crate::{RustSourceByteOffset, RustTypeDeclaration, RustTypeKind};
use errgonomic::handle_opt;
use ra_ap_hir::{Semantics, symbols::FileSymbol};
use ra_ap_ide_db::{FileId, RootDatabase};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct LocatedRustTypeDeclaration {
    pub path: PathBuf,
    pub declaration: RustTypeDeclaration,
}

impl LocatedRustTypeDeclaration {
    pub fn from_symbol(symbol: &FileSymbol<'_>, kind: RustTypeKind, semantics: &Semantics<'_, RootDatabase>, source_paths: &HashMap<FileId, &Path>) -> Result<Self, LocatedRustTypeDeclarationFromSymbolError> {
        use LocatedRustTypeDeclarationFromSymbolError::*;
        let syntax = symbol.loc.syntax(semantics);
        let source_range = semantics.original_range(&syntax).into_file_id(semantics.db);
        let path = handle_opt!(source_paths.get(&source_range.file_id), SourcePathNotFound, name: symbol.name.to_string(), kind, file_id: source_range.file_id).to_path_buf();
        Ok(Self {
            path,
            declaration: RustTypeDeclaration {
                byte_offset: RustSourceByteOffset::from(source_range.range.start()),
                kind,
                name: symbol.name.to_string(),
            },
        })
    }
}

#[derive(Error, Debug)]
pub enum LocatedRustTypeDeclarationFromSymbolError {
    #[error("source path for Rust type '{name}' was not found for file {file_id:?}")]
    SourcePathNotFound { name: String, kind: RustTypeKind, file_id: FileId },
}
