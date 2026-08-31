use crate::RustTypeDeclaration;
use std::path::PathBuf;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct LocatedRustTypeDeclaration {
    pub path: PathBuf,
    pub declaration: RustTypeDeclaration,
}
