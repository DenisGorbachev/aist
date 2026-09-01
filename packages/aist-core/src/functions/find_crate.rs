use errgonomic::{handle_bool, handle_opt};
use ra_ap_hir::Crate;
use ra_ap_ide_db::RootDatabase;
use ra_ap_vfs::Vfs;
use serde::Serialize;
use thiserror::Error;

pub fn find_crate(name: &str, file_name: &str, db: &RootDatabase, vfs: &Vfs) -> Result<Crate, FindCrateError> {
    use FindCrateError::*;
    let mut crates = Crate::all(db).into_iter().filter(|krate| {
        krate.origin(db).is_local()
            && krate
                .display_name(db)
                .is_some_and(|display_name| display_name.canonical_name().as_str() == name)
            && vfs
                .file_path(krate.root_file(db))
                .as_path()
                .is_some_and(|path| path.file_name() == Some(file_name))
    });
    let krate = handle_opt!(crates.next(), CrateNotFound, name: name.to_owned(), file_name: file_name.to_owned());
    handle_bool!(crates.next().is_some(), CrateNotUnique, name: name.to_owned(), file_name: file_name.to_owned());
    Ok(krate)
}

#[derive(Serialize, Error, Debug)]
pub enum FindCrateError {
    #[error("local crate named '{name}' with root file name '{file_name}' was not found")]
    CrateNotFound { name: String, file_name: String },
    #[error("multiple local crates named '{name}' with root file name '{file_name}' were found")]
    CrateNotUnique { name: String, file_name: String },
}
