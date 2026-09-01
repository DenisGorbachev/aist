use crate::{StructCommand, StructCommandNewError};
use aist_core::{WorkspaceInfo, WorkspaceInfoFindCrateError};
use errgonomic::handle;
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
        let aist_lib = handle!(ws.find_crate("aist", "lib.rs"), FindCrateFailed);
        let command = StructCommand::new(&aist_lib, db);
        Ok(Self {
            command,
        })
    }
}

#[derive(Serialize, Error, Debug)]
pub enum AistLibCrateNewError {
    #[error("failed to find the local 'aist' library crate")]
    FindCrateFailed { source: Box<WorkspaceInfoFindCrateError> },
}
