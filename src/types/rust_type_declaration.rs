use crate::{RustSourceByteOffset, RustTypeKind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug)]
pub struct RustTypeDeclaration {
    pub name: String,
    pub kind: RustTypeKind,
    pub byte_offset: RustSourceByteOffset,
}
