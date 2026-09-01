use crate::{StructCommand, StructCommandNewError};
use aist_core::WorkspaceInfo;
use errgonomic::{handle_bool, handle_opt};
use itertools::Itertools;
use ra_ap_hir::Crate;
use serde::Serialize;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Serialize, Debug)]
pub struct AistPackage {
    pub command: Result<StructCommand, StructCommandNewError>,
}

impl AistPackage {
    /// PRUNING: retains only the conformance result for `Command`; rust-analyzer crate handles are execution state and are irrelevant to the report.
    pub fn new(ws: &WorkspaceInfo) -> Result<Self, AistPackageNewError> {
        use AistPackageNewError::*;
        let db = &ws.db;
        let (package_crates, mut manifest_paths): (Vec<_>, Vec<_>) = Crate::all(db)
            .into_iter()
            .filter(|krate| krate.origin(db).is_local())
            .filter_map(|krate| {
                let env = krate.base().env(db);
                env.get("CARGO_PKG_NAME")
                    .filter(|name| name == "aist")
                    .and_then(|_| env.get("CARGO_MANIFEST_PATH"))
                    .map(|path| (krate, PathBuf::from(path)))
            })
            .unzip();
        manifest_paths.sort_unstable();
        manifest_paths.dedup();
        handle_bool!(manifest_paths.len() > 1, LocalPackageInvalid, manifest_paths);
        let manifest_path = handle_opt!(manifest_paths.into_iter().next(), LocalPackageNotFound);
        let referenced_package_crates = package_crates
            .iter()
            .flat_map(|krate| krate.dependencies(db))
            .map(|dependency| dependency.krate)
            .filter(|dependency| package_crates.contains(dependency))
            .unique()
            .collect::<Vec<_>>();
        let library_crates = if referenced_package_crates.is_empty() {
            package_crates
                .iter()
                .copied()
                .filter(|krate| {
                    krate
                        .display_name(db)
                        .is_some_and(|display_name| display_name.canonical_name().as_str() == "aist")
                })
                .collect::<Vec<_>>()
        } else {
            referenced_package_crates
        };
        let library_crate_count = library_crates.len();
        handle_bool!(library_crate_count > 1, LibraryCrateInvalid, manifest_path, library_crate_count);
        let aist_lib = handle_opt!(library_crates.into_iter().next(), LibraryCrateNotFound, manifest_path);
        let command = StructCommand::new(&aist_lib, db);
        Ok(Self {
            command,
        })
    }
}

#[derive(Serialize, Error, Debug)]
pub enum AistPackageNewError {
    #[error("local package 'aist' was not found")]
    LocalPackageNotFound {},
    #[error("multiple local packages named 'aist' were found at {len} manifest paths", len = manifest_paths.len())]
    LocalPackageInvalid { manifest_paths: Vec<PathBuf> },
    #[error("library crate for package manifest '{manifest_path}' was not found", manifest_path = manifest_path.display())]
    LibraryCrateNotFound { manifest_path: PathBuf },
    #[error("found {library_crate_count} possible library crates for package manifest '{manifest_path}'", manifest_path = manifest_path.display())]
    LibraryCrateInvalid { manifest_path: PathBuf, library_crate_count: usize },
}
