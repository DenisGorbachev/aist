use cargo_util_schemas::manifest::{InheritableDependency, PackageName, TomlManifest};
use std::collections::BTreeMap;
use std::iter::once;

pub fn package_dependency_tables(manifest: &TomlManifest) -> impl Iterator<Item = (&str, &'static str, &BTreeMap<PackageName, InheritableDependency>)> {
    let root = (
        "",
        [
            manifest.dependencies.as_ref(),
            manifest.dev_dependencies(),
            manifest.build_dependencies(),
        ],
    );
    let targets = manifest.target.iter().flatten().map(|(target, platform)| {
        (
            target.as_str(),
            [
                platform.dependencies.as_ref(),
                platform.dev_dependencies(),
                platform.build_dependencies(),
            ],
        )
    });
    once(root).chain(targets).flat_map(|(target, tables)| {
        ["dependencies", "dev-dependencies", "build-dependencies"]
            .into_iter()
            .zip(tables)
            .filter_map(move |(kind, dependencies)| dependencies.map(|dependencies| (target, kind, dependencies)))
    })
}
