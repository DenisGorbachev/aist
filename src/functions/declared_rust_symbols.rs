use ra_ap_hir::symbols::FileSymbol;

/// PRUNING: omits imports and documentation aliases because they refer to declarations instead of defining additional project types.
pub fn declared_rust_symbols<'db>(symbols: impl IntoIterator<Item = FileSymbol<'db>>) -> impl Iterator<Item = FileSymbol<'db>> {
    symbols
        .into_iter()
        .filter(|symbol| !symbol.is_import && !symbol.is_alias)
}
