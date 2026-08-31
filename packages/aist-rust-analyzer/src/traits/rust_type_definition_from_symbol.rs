use crate::RustTypeDefinition;
use ra_ap_hir::symbols::FileSymbol;

pub trait RustTypeDefinitionFromSymbol {
    fn rust_type_definition(symbol: &FileSymbol<'_>) -> Option<RustTypeDefinition>;
}
