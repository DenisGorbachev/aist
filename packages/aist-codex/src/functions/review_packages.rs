use cargo_metadata::{Metadata, Package, PackageId};
use errgonomic::{handle_bool, handle_opt};
use std::collections::BTreeSet;
use thiserror::Error;

pub fn review_packages<'a>(metadata: &'a Metadata, name: &str) -> Result<Vec<&'a Package>, ReviewPackagesError> {
    use ReviewPackagesError::*;
    let mut matches = metadata
        .workspace_packages()
        .into_iter()
        .filter(|package| package.name.as_str() == name);
    let root = handle_opt!(matches.next(), PackageNotFound, name: name.to_owned());
    handle_bool!(matches.next().is_some(), PackageNotUniqueInvalid, name: name.to_owned());
    let resolve = handle_opt!(metadata.resolve.as_ref(), ResolveNotFound);
    let mut pending = vec![root.id.clone()];
    let mut ids = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if ids.insert(id.clone()) {
            let node = handle_opt!(resolve.nodes.iter().find(|node| node.id == id), NodeNotFound, id);
            pending.extend(node.dependencies.iter().cloned());
        }
    }
    let mut packages = metadata
        .packages
        .iter()
        .filter(|package| ids.contains(&package.id))
        .collect::<Vec<_>>();
    handle_bool!(packages.len() != ids.len(), PackagesInvalid, ids);
    packages.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(packages)
}

#[derive(Error, Debug)]
pub enum ReviewPackagesError {
    #[error("workspace package '{name}' not found")]
    PackageNotFound { name: String },
    #[error("workspace package '{name}' is not unique")]
    PackageNotUniqueInvalid { name: String },
    #[error("Cargo metadata has no resolved dependency graph")]
    ResolveNotFound {},
    #[error("resolved dependency node '{id}' not found")]
    NodeNotFound { id: PackageId },
    #[error("resolved dependency packages are incomplete")]
    PackagesInvalid { ids: BTreeSet<PackageId> },
}
