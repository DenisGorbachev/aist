use crate::{AistPackage, AistPackageNewError};
use aist_core::WorkspaceInfo;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SpecReport {
    pub aist: Result<AistPackage, AistPackageNewError>,
}

impl SpecReport {
    pub fn new(ws: &WorkspaceInfo) -> Self {
        let aist = AistPackage::new(ws);
        Self {
            aist,
        }
    }
}
