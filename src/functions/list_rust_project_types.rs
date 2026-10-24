use crate::{LocatedRustTypeDeclaration, LocatedRustTypeDeclarationFromSymbolError, RustProjectTypes, RustTypeKind, build_rust_project_types, rust_source_paths};
use aist_core::WorkspaceInfo;
use errgonomic::{ErrVec, handle_iter};
use ra_ap_hir::{Crate, Semantics, symbols::SymbolCollector};
use thiserror::Error;

pub fn list_rust_project_types(workspace_info: &WorkspaceInfo) -> Result<RustProjectTypes, ListRustProjectTypesError> {
    use ListRustProjectTypesError::*;
    let db = &workspace_info.db;
    let semantics = Semantics::new(db);
    let source_paths = rust_source_paths(&workspace_info.vfs);
    let results = Crate::all(db)
        .into_iter()
        .filter(|krate| krate.origin(db).is_local())
        .flat_map(|krate| krate.modules(db))
        .flat_map(|module| SymbolCollector::new_module(db, module, false).into_vec())
        .filter(|symbol| !symbol.is_import && !symbol.is_alias)
        .filter_map(|symbol| RustTypeKind::from_module_def(symbol.def).map(|kind| LocatedRustTypeDeclaration::from_symbol(&symbol, kind, &semantics, &source_paths)));
    Ok(build_rust_project_types(handle_iter!(results, FromSymbolFailed)))
}

#[derive(Error, Debug)]
pub enum ListRustProjectTypesError {
    #[error("failed to locate Rust type declarations")]
    FromSymbolFailed { source: ErrVec<LocatedRustTypeDeclarationFromSymbolError> },
}
