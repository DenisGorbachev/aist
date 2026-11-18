use crate::{FindCratesIoCrateError, ResolveTraitError, RustContract, find_crates_io_crate, resolve_trait};
use errgonomic::{handle, handle_bool, handle_opt};
use ra_ap_hir::{Crate, Trait};
use ra_ap_ide_db::base_db::{CrateOrigin, LangCrateOrigin};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use thiserror::Error;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TraitOrigin<'a> {
    Core,
    CratesIo(&'a str),
}

impl Display for TraitOrigin<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core => f.write_str("sysroot core"),
            Self::CratesIo(package) => write!(f, "crates.io package '{package}'"),
        }
    }
}

impl TraitOrigin<'_> {
    pub fn resolve(self, contract: &RustContract<'_>, path: &str) -> Result<Trait, TraitOriginResolveError> {
        use TraitOriginResolveError::*;
        let db = &contract.workspace.db;
        let mut crates = contract
            .krate
            .base()
            .transitive_deps(db)
            .into_iter()
            .map(Crate::from);
        let krate = match self {
            Self::Core => handle_opt!(crates.find(|krate| matches!(krate.origin(db), CrateOrigin::Lang(LangCrateOrigin::Core))), CoreNotFound),
            Self::CratesIo(package) => handle!(find_crates_io_crate(contract.workspace, crates, package), FindCratesIoCrateFailed),
        };
        let required = handle!(resolve_trait(&krate.root_module(db), db, path), ResolveTraitFailed, origin: self.to_string());
        handle_bool!(required.module(db).krate(db) != krate, DefiningCrateInvalid, origin: self.to_string(), path: path.to_owned());
        Ok(required)
    }
}

#[derive(Debug, Error)]
pub enum TraitOriginResolveError {
    #[error("sysroot core crate not found in dependencies")]
    CoreNotFound {},
    #[error("failed to identify trait's defining package")]
    FindCratesIoCrateFailed { source: Box<FindCratesIoCrateError> },
    #[error("failed to resolve trait in {origin}")]
    ResolveTraitFailed { source: ResolveTraitError, origin: String },
    #[error("'{path}' resolves to a trait defined outside {origin}")]
    DefiningCrateInvalid { origin: String, path: String },
}
