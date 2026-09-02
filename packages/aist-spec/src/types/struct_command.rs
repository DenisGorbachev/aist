use aist_core::{FindStructError, find_struct};
use errgonomic::map_err;
use ra_ap_hir::Crate;
use ra_ap_ide_db::RootDatabase;
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug)]
pub struct StructCommand;

impl StructCommand {
    pub fn new(lib: &Crate, db: &RootDatabase) -> Result<Self, StructCommandNewError> {
        use StructCommandNewError::*;
        map_err!(find_struct("Command", lib, db), FindStructFailed).map(|_| Self)
    }
}

#[derive(Serialize, Error, Debug)]
pub enum StructCommandNewError {
    #[error("failed to find struct 'Command'")]
    FindStructFailed { source: FindStructError },
}
