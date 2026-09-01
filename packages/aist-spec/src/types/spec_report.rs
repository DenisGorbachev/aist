use crate::{AistLibCrate, AistLibCrateNewError};
use aist_core::WorkspaceInfo;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SpecReport {
    pub aist: Result<AistLibCrate, AistLibCrateNewError>,
}

impl SpecReport {
    pub fn new(ws: &WorkspaceInfo) -> Self {
        let aist = AistLibCrate::new(ws);
        Self {
            aist,
        }
    }
}
