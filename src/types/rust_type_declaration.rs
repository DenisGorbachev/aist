use crate::{RustSourceByteOffset, RustTypeKind};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct RustTypeDeclaration {
    pub byte_offset: RustSourceByteOffset,
    pub kind: RustTypeKind,
    pub name: String,
}
