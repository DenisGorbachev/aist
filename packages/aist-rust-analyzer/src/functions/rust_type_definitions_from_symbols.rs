use crate::{RustTypeDefinition, RustTypeDefinitionFromSymbol, declared_rust_symbols};
use ra_ap_hir::{Crate, symbols::SymbolCollector};
use ra_ap_ide_db::RootDatabase;

/// PRUNING: omits dependency crates because the requested listing is limited to types declared by workspace members.
pub fn rust_type_definitions_from_symbols<C: RustTypeDefinitionFromSymbol>(db: &RootDatabase) -> impl Iterator<Item = RustTypeDefinition> + '_ {
    let symbols = Crate::all(db)
        .into_iter()
        .filter(|krate| krate.origin(db).is_local())
        .flat_map(|krate| krate.modules(db))
        .flat_map(|module| SymbolCollector::new_module(db, module, false).into_vec());
    declared_rust_symbols(symbols).filter_map(|symbol| C::rust_type_definition(&symbol))
}
