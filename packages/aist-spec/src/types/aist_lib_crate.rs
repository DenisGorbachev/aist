use crate::{StructCommand, StructCommandNewError};
use aist_core::WorkspaceInfo;
use errgonomic::{handle_bool, handle_opt};
use ra_ap_hir::Crate;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug)]
pub struct AistLibCrate {
    pub command: Result<StructCommand, StructCommandNewError>,
}

impl AistLibCrate {
    pub fn new(ws: &WorkspaceInfo) -> Result<Self, AistLibCrateNewError> {
        use AistLibCrateNewError::*;
        let db = &ws.db;
        let mut named_crates = Crate::all(db);
        named_crates.retain(|krate| {
            krate.origin(db).is_local()
                && krate
                    .display_name(db)
                    .is_some_and(|display_name| display_name.canonical_name().as_str() == "aist")
        });
        let mut referenced_named_crates = named_crates
            .iter()
            .flat_map(|krate| krate.dependencies(db))
            .map(|dependency| dependency.krate)
            .filter(|dependency| named_crates.contains(dependency));
        let aist_lib = match referenced_named_crates.next() {
            Some(aist_lib) => {
                handle_bool!(referenced_named_crates.any(|krate| krate != aist_lib), LocalLibCrateInvalid);
                aist_lib
            }
            None => {
                let mut named_crates = named_crates.iter().copied();
                let aist_lib = handle_opt!(named_crates.next(), LocalLibCrateNotFound);
                handle_bool!(named_crates.next().is_some(), LocalLibCrateInvalid);
                aist_lib
            }
        };
        let command = StructCommand::new(&aist_lib, db);
        Ok(Self {
            command,
        })
    }
}

#[derive(Serialize, Error, Debug)]
pub enum AistLibCrateNewError {
    #[error("local library crate 'aist' was not found")]
    LocalLibCrateNotFound {},
    #[error("multiple possible local library crates named 'aist' were found")]
    LocalLibCrateInvalid {},
}
