use crate::{FindCrateError, ReadCargoMetadataError, find_crate, read_cargo_metadata};
use cargo_metadata::{Metadata, camino::Utf8Path};
use errgonomic::{PathBufDisplay, handle, handle_discard, handle_opt, map_err};
use ra_ap_hir::Crate;
use ra_ap_ide_db::RootDatabase;
use ra_ap_load_cargo::{LoadCargoConfig, ProcMacroServerChoice, load_workspace_at};
use ra_ap_proc_macro_api::ProcMacroClient;
use ra_ap_project_model::{CargoConfig, CargoFeatures, ProjectManifest, RustLibSource};
use ra_ap_vfs::{AbsPath, Vfs};
use serde::Serialize;
use std::io;
use std::path::{self, Path};
use std::sync::OnceLock;
use thiserror::Error;

pub struct WorkspaceInfo {
    pub db: RootDatabase,
    pub vfs: Vfs,
    pub proc_macro_client: ProcMacroClient,
    manifest: ProjectManifest,
    metadata: OnceLock<Metadata>,
}

impl TryFrom<&Path> for WorkspaceInfo {
    type Error = TryFromPathForWorkspaceInfoError;

    fn try_from(project_root: &Path) -> Result<Self, Self::Error> {
        use TryFromPathForWorkspaceInfoError::*;
        let cargo_config = CargoConfig {
            all_targets: true,
            features: CargoFeatures::All,
            set_test: true,
            sysroot: Some(RustLibSource::Discover),
            ..CargoConfig::default()
        };
        let load_config = LoadCargoConfig {
            load_out_dirs_from_check: true,
            with_proc_macro_server: ProcMacroServerChoice::Sysroot,
            prefill_caches: false,
            num_worker_threads: 1,
            proc_macro_processes: 1,
        };
        let absolute = handle!(path::absolute(project_root), AbsoluteFailed, project_root: project_root.to_path_buf());
        let utf8 = handle_opt!(Utf8Path::from_path(&absolute), ProjectPathInvalid, path: absolute);
        let root = handle_discard!(<&AbsPath>::try_from(utf8), ProjectPathInvalid, path: absolute);
        let manifest = handle!(ProjectManifest::discover_single(root), DiscoverSingleFailed, path: absolute);
        let (db, vfs, proc_macro_client) = handle!(
            load_workspace_at(manifest.manifest_path().as_ref(), &cargo_config, &load_config, &|_| {}),
            LoadWorkspaceAtFailed,
            project_root: project_root.to_path_buf()
        );
        let proc_macro_client = handle_opt!(proc_macro_client, ProcMacroClientNotFound, project_root: project_root.to_path_buf());
        Ok(Self {
            db,
            vfs,
            proc_macro_client,
            manifest,
            metadata: OnceLock::new(),
        })
    }
}

impl WorkspaceInfo {
    pub fn cargo_metadata(&self) -> Result<&Metadata, WorkspaceInfoCargoMetadataError> {
        use WorkspaceInfoCargoMetadataError::*;
        if let Some(metadata) = self.metadata.get() {
            return Ok(metadata);
        }
        let ProjectManifest::CargoToml(manifest) = &self.manifest else {
            return Err(CargoManifestNotFound {
                manifest: self.manifest.to_string(),
            });
        };
        let metadata = handle!(read_cargo_metadata(manifest), ReadCargoMetadataFailed, manifest: manifest.to_string());
        Ok(self.metadata.get_or_init(|| metadata))
    }

    pub fn find_crate(&self, name: &str, file_name: &str) -> Result<Crate, WorkspaceInfoFindCrateError> {
        use WorkspaceInfoFindCrateError::*;
        map_err!(find_crate(name, file_name, &self.db, &self.vfs), FindCrateFailed, name: name.to_owned(), file_name: file_name.to_owned())
    }
}

#[derive(Error, Debug)]
pub enum TryFromPathForWorkspaceInfoError {
    #[error("failed to make project root '{project_root}' absolute")]
    AbsoluteFailed { source: io::Error, project_root: PathBufDisplay },
    #[error("project path '{path}' must be absolute UTF-8")]
    ProjectPathInvalid { path: PathBufDisplay },
    #[error("failed to discover project manifest at '{path}'")]
    DiscoverSingleFailed { source: anyhow::Error, path: PathBufDisplay },
    #[error("failed to load Rust project '{project_root}' into rust-analyzer")]
    LoadWorkspaceAtFailed { source: anyhow::Error, project_root: PathBufDisplay },
    #[error("rust-analyzer's proc-macro server was not found while loading project '{project_root}'")]
    ProcMacroClientNotFound { project_root: PathBufDisplay },
}

#[derive(Serialize, Error, Debug)]
pub enum WorkspaceInfoFindCrateError {
    #[error("failed to find local crate named '{name}' with root file name '{file_name}'")]
    FindCrateFailed { source: FindCrateError, name: String, file_name: String },
}

#[derive(Error, Debug)]
pub enum WorkspaceInfoCargoMetadataError {
    #[error("project '{manifest}' has no Cargo manifest")]
    CargoManifestNotFound { manifest: String },
    #[error("failed to read resolved Cargo metadata for '{manifest}'")]
    ReadCargoMetadataFailed { source: Box<ReadCargoMetadataError>, manifest: String },
}
