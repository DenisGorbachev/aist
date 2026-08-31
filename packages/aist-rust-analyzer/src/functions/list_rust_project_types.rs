use crate::{CollectRustTypeDeclarationsError, RustProjectTypes, build_rust_project_types, collect_rust_type_declarations, rust_source_paths};
use errgonomic::{PathBufDisplay, handle, handle_opt};
use ra_ap_hir::Semantics;
use ra_ap_load_cargo::{LoadCargoConfig, ProcMacroServerChoice, load_workspace_at};
use ra_ap_project_model::{CargoConfig, CargoFeatures, RustLibSource};
use std::path::Path;
use thiserror::Error;

pub fn list_rust_project_types(project_dir: &Path) -> Result<RustProjectTypes, ListRustProjectTypesError> {
    use ListRustProjectTypesError::*;
    let load_config = LoadCargoConfig {
        load_out_dirs_from_check: true,
        with_proc_macro_server: ProcMacroServerChoice::Sysroot,
        prefill_caches: false,
        num_worker_threads: 1,
        proc_macro_processes: 1,
    };
    let cargo_config = CargoConfig {
        all_targets: true,
        features: CargoFeatures::All,
        set_test: true,
        sysroot: Some(RustLibSource::Discover),
        ..CargoConfig::default()
    };
    let (db, vfs, proc_macro_client) = handle!(
        load_workspace_at(project_dir, &cargo_config, &load_config, &|_| {}),
        LoadWorkspaceAtFailed,
        project_dir: project_dir.to_path_buf()
    );
    let proc_macro_client = handle_opt!(proc_macro_client, ProcMacroClientNotFound, project_dir: project_dir.to_path_buf());
    let _proc_macro_client = proc_macro_client;
    let semantics = Semantics::new(&db);
    let source_paths = rust_source_paths(&vfs);
    let declarations = handle!(collect_rust_type_declarations(&db, &semantics, &source_paths), CollectRustTypeDeclarationsFailed, project_dir: project_dir.to_path_buf());
    Ok(build_rust_project_types(declarations))
}

#[derive(Error, Debug)]
pub enum ListRustProjectTypesError {
    #[error("failed to load Rust project '{project_dir}' into rust-analyzer")]
    LoadWorkspaceAtFailed { source: anyhow::Error, project_dir: PathBufDisplay },
    #[error("rust-analyzer's proc-macro server was not found while loading project '{project_dir}'")]
    ProcMacroClientNotFound { project_dir: PathBufDisplay },
    #[error("failed to collect macro-expanded Rust type declarations from project '{project_dir}'")]
    CollectRustTypeDeclarationsFailed { source: CollectRustTypeDeclarationsError, project_dir: PathBufDisplay },
}
