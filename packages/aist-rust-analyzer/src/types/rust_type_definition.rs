use crate::RustTypeKind;
use ra_ap_hir::symbols::DeclarationLocation;

#[derive(Clone, Debug)]
pub struct RustTypeDefinition {
    pub name: String,
    pub kind: RustTypeKind,
    pub location: DeclarationLocation,
}
