use crate::RustTypeDeclaration;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct RustFileTypes {
    pub path: PathBuf,
    pub types: Vec<RustTypeDeclaration>,
}
