use crate::{CollectRustTypeDeclarationsError, RustProjectTypes, build_rust_project_types, collect_rust_type_declarations, rust_source_paths};
use aist_core::WorkspaceInfo;
use errgonomic::handle;
use ra_ap_hir::Semantics;
use thiserror::Error;

pub fn list_rust_project_types(workspace_info: &WorkspaceInfo) -> Result<RustProjectTypes, ListRustProjectTypesError> {
    use ListRustProjectTypesError::*;
    let semantics = Semantics::new(&workspace_info.db);
    let source_paths = rust_source_paths(&workspace_info.vfs);
    let declarations = handle!(collect_rust_type_declarations(&workspace_info.db, &semantics, &source_paths), CollectRustTypeDeclarationsFailed);
    Ok(build_rust_project_types(declarations))
}

#[derive(Error, Debug)]
pub enum ListRustProjectTypesError {
    #[error("failed to collect macro-expanded Rust type declarations")]
    CollectRustTypeDeclarationsFailed { source: CollectRustTypeDeclarationsError },
}
