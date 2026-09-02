use errgonomic::{handle_bool, handle_opt};
use ra_ap_hir::{Adt, Crate, ModuleDef, Struct};
use ra_ap_ide_db::RootDatabase;
use serde::Serialize;
use thiserror::Error;

pub fn find_struct(name: &str, krate: &Crate, db: &RootDatabase) -> Result<Struct, FindStructError> {
    use FindStructError::*;
    let mut structs = krate
        .modules(db)
        .into_iter()
        .flat_map(|module| module.declarations(db))
        .filter_map(|definition| match definition {
            ModuleDef::Adt(Adt::Struct(item)) if item.name(db).as_str() == name => Some(item),
            _ => None,
        });
    let item = handle_opt!(structs.next(), StructNotFound, name: name.to_owned());
    handle_bool!(structs.next().is_some(), StructInvalid, name: name.to_owned());
    Ok(item)
}

#[derive(Serialize, Error, Debug)]
pub enum FindStructError {
    #[error("struct '{name}' was not found")]
    StructNotFound { name: String },
    #[error("multiple structs named '{name}' were found")]
    StructInvalid { name: String },
}
