use crate::{ContractReport, observe, read_toml};
use cargo_metadata::MetadataCommand;
use std::iter::once;
use std::path::Path;
use toml::Value;

pub fn check_workspace_dependencies(report: &mut ContractReport, root: &Path, required: &[&str]) {
    let metadata = observe!(report, MetadataCommand::new().current_dir(root).no_deps().exec());
    let manifest = observe!(
        report,
        read_toml(
            &metadata
                .workspace_root
                .join("Cargo.toml")
                .into_std_path_buf()
        )
    );
    let dependencies = manifest
        .get("workspace")
        .and_then(|workspace| workspace.get("dependencies"))
        .and_then(Value::as_table);
    required
        .iter()
        .for_each(|name| report.require(format!("workspace.dependencies/{name}"), dependencies.is_some_and(|dependencies| dependencies.contains_key(*name))));
    metadata
        .workspace_packages()
        .into_iter()
        .for_each(|package| {
            let manifest = observe!(report, read_toml(package.manifest_path.as_std_path()));
            let scopes = once(&manifest).chain(
                manifest
                    .get("target")
                    .and_then(Value::as_table)
                    .into_iter()
                    .flat_map(|targets| targets.values()),
            );
            scopes.for_each(|scope| {
                ["dependencies", "dev-dependencies", "build-dependencies"]
                    .into_iter()
                    .for_each(|kind| {
                        scope
                            .get(kind)
                            .and_then(Value::as_table)
                            .into_iter()
                            .flat_map(|dependencies| dependencies.iter())
                            .for_each(|(name, dependency)| {
                                let inherited = dependency.get("workspace").and_then(Value::as_bool) == Some(true);
                                let source_local = [
                                    "version", "path", "git", "branch", "tag", "rev", "registry", "package",
                                ]
                                .into_iter()
                                .any(|key| dependency.get(key).is_some());
                                report.require(format!("{}/{kind}/{name}/workspace dependency", package.name), inherited && !source_local && dependencies.is_some_and(|dependencies| dependencies.contains_key(name)));
                            });
                    });
            });
        });
}
