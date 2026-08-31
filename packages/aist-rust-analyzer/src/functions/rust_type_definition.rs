use crate::{RustTypeDefinition, RustTypeKind};
use ra_ap_hir::symbols::FileSymbol;

pub fn rust_type_definition(symbol: &FileSymbol<'_>, kind: RustTypeKind) -> RustTypeDefinition {
    RustTypeDefinition {
        name: symbol.name.to_string(),
        kind,
        location: symbol.loc,
    }
}
