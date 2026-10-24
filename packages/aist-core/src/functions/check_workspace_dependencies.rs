use crate::{CheckPackageDependenciesError, ReadTomlError, check_package_dependencies, read_toml};
use cargo_metadata::{Error as MetadataError, MetadataCommand};
use errgonomic::{ErrVec, handle, handle_bool, handle_iter};
use std::path::{Path, PathBuf};
use thiserror::Error;
use toml::Value;

pub fn check_workspace_dependencies(root: &Path, required: &[&str]) -> Result<(), CheckWorkspaceDependenciesError> {
    use CheckWorkspaceDependenciesError::*;
    let metadata = handle!(MetadataCommand::new().current_dir(root).no_deps().exec(), ExecFailed, root: root.to_path_buf());
    let path = metadata
        .workspace_root
        .join("Cargo.toml")
        .into_std_path_buf();
    let manifest = handle!(read_toml(&path), ReadTomlFailed);
    let dependencies = manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(Value::as_table);
    let missing: Vec<_> = required
        .iter()
        .filter(|name| !dependencies.is_some_and(|dependencies| dependencies.contains_key(**name)))
        .map(|name| (*name).to_owned())
        .collect();
    let results = metadata
        .workspace_packages()
        .into_iter()
        .map(|package| check_package_dependencies(package.manifest_path.as_std_path(), dependencies));
    handle_iter!(results, CheckPackageDependenciesFailed, path, missing);
    handle_bool!(!missing.is_empty(), DependenciesNotFound, path, missing);
    Ok(())
}

#[derive(Error, Debug)]
pub enum CheckWorkspaceDependenciesError {
    #[error("failed to inspect Cargo workspace '{root}'", root = root.display())]
    ExecFailed { source: MetadataError, root: PathBuf },
    #[error("failed to read workspace manifest")]
    ReadTomlFailed { source: ReadTomlError },
    #[error("invalid package dependency declarations; missing workspace dependencies: {missing:?}")]
    CheckPackageDependenciesFailed { source: ErrVec<CheckPackageDependenciesError>, path: PathBuf, missing: Vec<String> },
    #[error("required workspace dependencies are missing: {missing:?}")]
    DependenciesNotFound { path: PathBuf, missing: Vec<String> },
}
