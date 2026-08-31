use crate::{RustTypeDefinition, RustTypeKind, declared_rust_symbols, rust_type_definition};
use ra_ap_hir::{Crate, symbols::SymbolCollector};
use ra_ap_ide_db::RootDatabase;

/// PRUNING: omits dependency crates and semantic symbols that do not define data types or type aliases because the requested listing is limited to project type declarations.
pub fn rust_type_definitions_from_symbols(db: &RootDatabase) -> impl Iterator<Item = RustTypeDefinition> + '_ {
    let symbols = Crate::all(db)
        .into_iter()
        .filter(|krate| krate.origin(db).is_local())
        .flat_map(|krate| krate.modules(db))
        .flat_map(|module| SymbolCollector::new_module(db, module, false).into_vec());
    declared_rust_symbols(symbols).filter_map(|symbol| RustTypeKind::from_module_def(symbol.def).map(|kind| rust_type_definition(&symbol, kind)))
}
