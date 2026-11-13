use crate::{ReadTomlError, inherits_workspace_dependency, package_dependency_tables, read_toml};
use cargo_util_schemas::manifest::{PackageName, TomlDependency, TomlManifest};
use errgonomic::{handle, handle_bool};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub fn check_package_dependencies(path: &Path, workspace_dependencies: Option<&BTreeMap<PackageName, TomlDependency>>) -> Result<(), CheckPackageDependenciesError> {
    use CheckPackageDependenciesError::*;
    let manifest: TomlManifest = handle!(read_toml(path), ReadTomlFailed);
    let invalid: Vec<_> = package_dependency_tables(&manifest)
        .flat_map(|(target, kind, dependencies)| {
            dependencies.iter().filter_map(move |(name, dependency)| {
                let declared = workspace_dependencies.is_some_and(|dependencies| dependencies.contains_key(name));
                (!declared || !inherits_workspace_dependency(dependency)).then(|| format!("{target}/{kind}/{name}"))
            })
        })
        .collect();
    handle_bool!(!invalid.is_empty(), DependenciesInvalid, path: path.to_path_buf(), invalid, manifest, workspace_dependencies: workspace_dependencies.cloned());
    Ok(())
}

#[derive(Error, Debug)]
pub enum CheckPackageDependenciesError {
    #[error("failed to read package manifest")]
    ReadTomlFailed { source: ReadTomlError },
    #[error("dependencies must inherit workspace versions and sources in '{path}': {invalid:?}", path = path.display())]
    DependenciesInvalid { path: PathBuf, invalid: Vec<String>, manifest: Box<TomlManifest>, workspace_dependencies: Option<BTreeMap<PackageName, TomlDependency>> },
}
