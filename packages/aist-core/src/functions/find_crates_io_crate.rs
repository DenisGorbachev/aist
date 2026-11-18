use crate::{WorkspaceInfo, WorkspaceInfoCargoMetadataError};
use cargo_metadata::Source;
use cargo_util_schemas::core::{PackageIdSpec, PackageIdSpecError};
use errgonomic::{handle, handle_bool, handle_opt};
use ra_ap_hir::Crate;
use ra_ap_ide_db::base_db::CrateOrigin;
use std::collections::HashSet;
use thiserror::Error;

pub fn find_crates_io_crate(ws: &WorkspaceInfo, crates: impl Iterator<Item = Crate>, package: &str) -> Result<Crate, FindCratesIoCrateError> {
    use FindCratesIoCrateError::*;
    let db = &ws.db;
    let spec = handle!(PackageIdSpec::parse(package), ParseFailed, package: package.to_owned());
    handle_bool!(spec.url().is_some(), PackageSourceInvalid, package: package.to_owned());
    let metadata = handle!(ws.cargo_metadata(), CargoMetadataFailed, package: package.to_owned());
    let roots: HashSet<_> = metadata
        .packages
        .iter()
        .filter(|pkg| {
            pkg.name.as_str() == spec.name()
                && spec
                    .partial_version()
                    .is_none_or(|v| v.matches(&pkg.version))
                && pkg.source.as_ref().is_some_and(Source::is_crates_io)
        })
        .flat_map(|pkg| {
            pkg.targets
                .iter()
                .filter(|target| target.is_lib() || target.is_rlib() || target.is_dylib())
                .filter_map(|target| Some((pkg.manifest_path.parent()?.as_str(), target.src_path.as_str())))
        })
        .collect();
    let mut matches = crates.filter(|krate| {
        let data = krate.base().data(db);
        if !matches!(&data.origin, CrateOrigin::Library { name, .. } if name.as_str() == spec.name()) {
            return false;
        }
        ws.vfs
            .file_path(krate.root_file(db))
            .as_path()
            .is_some_and(|root| roots.contains(&(data.proc_macro_cwd.as_str(), root.as_str())))
    });
    let krate = handle_opt!(matches.next(), CrateNotFound, package: package.to_owned());
    handle_bool!(matches.next().is_some(), CrateNotUniqueInvalid, package: package.to_owned());
    Ok(krate)
}

#[derive(Debug, Error)]
pub enum FindCratesIoCrateError {
    #[error("invalid Cargo package specification '{package}'")]
    ParseFailed { source: PackageIdSpecError, package: String },
    #[error("crates.io package '{package}' must be specified by name and optional version")]
    PackageSourceInvalid { package: String },
    #[error("failed to identify crates.io package '{package}'")]
    CargoMetadataFailed { source: WorkspaceInfoCargoMetadataError, package: String },
    #[error("no reachable library from crates.io package '{package}'")]
    CrateNotFound { package: String },
    #[error("multiple reachable libraries match crates.io package '{package}'; specify name@version")]
    CrateNotUniqueInvalid { package: String },
}
