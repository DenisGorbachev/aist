use crate::{LocatedRustTypeDeclaration, RustTypeDeclarationError, RustTypeDefinitionCollector, rust_type_declaration};
use errgonomic::{ErrVec, handle_iter};
use ra_ap_hir::Semantics;
use ra_ap_ide_db::{FileId, RootDatabase};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

/// PRUNING: omits dependency crates and non-type symbols because the requested listing is limited to types declared by workspace members.
pub fn collect_rust_type_declarations<C: RustTypeDefinitionCollector>(db: &RootDatabase, semantics: &Semantics<'_, RootDatabase>, source_paths: &HashMap<FileId, PathBuf>) -> Result<Vec<LocatedRustTypeDeclaration>, CollectRustTypeDeclarationsError> {
    use CollectRustTypeDeclarationsError::*;
    let results = C::rust_type_definitions(db).map(|definition| rust_type_declaration(db, semantics, source_paths, definition));
    Ok(handle_iter!(results, RustTypeDeclarationFailed))
}

#[derive(Error, Debug)]
pub enum CollectRustTypeDeclarationsError {
    #[error("failed to create {len} Rust type declarations", len = source.len())]
    RustTypeDeclarationFailed { source: ErrVec<RustTypeDeclarationError> },
}
