use crate::filter_adt;
use errgonomic::{handle_bool, handle_opt};
use ra_ap_hir::{Adt, Crate};
use ra_ap_ide_db::RootDatabase;
use serde::Serialize;
use thiserror::Error;

/// PRUNING: returns no matching ADT when the name is absent or non-unique because this function can only return a unique declaration.
pub fn get_adt(name: &str, krate: &Crate, db: &RootDatabase) -> Result<Adt, GetAdtError> {
    use GetAdtError::*;
    let mut adts = filter_adt(name, krate, db);
    let adt = handle_opt!(adts.next(), AdtNotFound, name: name.to_owned());
    handle_bool!(adts.next().is_some(), AdtNotUniqueInvalid, name: name.to_owned());
    Ok(adt)
}

#[derive(Serialize, Error, Debug)]
pub enum GetAdtError {
    #[error("ADT '{name}' was not found")]
    AdtNotFound { name: String },
    #[error("multiple ADTs named '{name}' were found")]
    AdtNotUniqueInvalid { name: String },
}
