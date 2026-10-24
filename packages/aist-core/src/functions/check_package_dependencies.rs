use crate::{ReadTomlError, read_toml};
use errgonomic::{handle, handle_bool};
use std::iter::once;
use std::path::{Path, PathBuf};
use thiserror::Error;
use toml::{Table, Value};

pub fn check_package_dependencies(path: &Path, workspace_dependencies: Option<&Table>) -> Result<(), CheckPackageDependenciesError> {
    use CheckPackageDependenciesError::*;
    let manifest = handle!(read_toml(path), ReadTomlFailed);
    let scopes = once(("", &manifest)).chain(
        manifest
            .get("target")
            .and_then(Value::as_table)
            .into_iter()
            .flat_map(|targets| targets.iter())
            .map(|(target, scope)| (target.as_str(), scope)),
    );
    let invalid: Vec<_> = scopes
        .flat_map(|(target, scope)| {
            ["dependencies", "dev-dependencies", "build-dependencies"]
                .into_iter()
                .flat_map(move |kind| {
                    scope
                        .get(kind)
                        .and_then(Value::as_table)
                        .into_iter()
                        .flat_map(|dependencies| dependencies.iter())
                        .filter_map(move |(name, dependency)| {
                            let inherited = dependency.get("workspace").and_then(Value::as_bool) == Some(true);
                            let local_source = [
                                "version", "path", "git", "branch", "tag", "rev", "registry", "package",
                            ]
                            .into_iter()
                            .any(|key| dependency.get(key).is_some());
                            let declared = workspace_dependencies.is_some_and(|dependencies| dependencies.contains_key(name));
                            (!(inherited && !local_source && declared)).then(|| format!("{target}/{kind}/{name}"))
                        })
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
    DependenciesInvalid { path: PathBuf, invalid: Vec<String>, manifest: Value, workspace_dependencies: Option<Table> },
}
