use crate::{GetAdtError, get_adt};
use errgonomic::handle;
use ra_ap_hir::{Adt, Crate, Struct};
use ra_ap_ide_db::RootDatabase;
use serde::Serialize;
use thiserror::Error;

/// PRUNING: returns no ADT when the unique declaration is not a struct because the caller requested a struct.
pub fn get_struct(name: &str, krate: &Crate, db: &RootDatabase) -> Result<Struct, GetStructError> {
    use Adt::*;
    use GetStructError::*;
    let adt = handle!(get_adt(name, krate, db), GetAdtFailed, name: name.to_owned());
    match adt {
        Struct(item) => Ok(item),
        _ => Err(AdtNotStructInvalid {
            name: name.to_owned(),
        }),
    }
}

#[derive(Serialize, Error, Debug)]
pub enum GetStructError {
    #[error("failed to get ADT '{name}'")]
    GetAdtFailed { source: GetAdtError, name: String },
    #[error("ADT '{name}' is not a struct")]
    AdtNotStructInvalid { name: String },
}
