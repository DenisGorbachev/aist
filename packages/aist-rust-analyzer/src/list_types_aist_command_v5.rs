use crate::{RustTypeDefinition, RustTypeDefinitionFromSymbol, RustTypeKind, impl_list_types_aist_command, impl_symbol_rust_type_definition_collector, rust_type_definition};
use clap::Parser;
use ra_ap_hir::symbols::FileSymbol;
use serde::{Deserialize, Serialize};

/// Lists types by delegating macro-expanded HIR classification to the shared semantic type-kind projector.
#[derive(Parser, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[command(flatten_help = true)]
pub struct ListTypesAistCommandV5;

impl RustTypeDefinitionFromSymbol for ListTypesAistCommandV5 {
    /// PRUNING: omits semantic symbols that do not define data types or type aliases because they are irrelevant to the requested listing.
    fn rust_type_definition(symbol: &FileSymbol<'_>) -> Option<RustTypeDefinition> {
        RustTypeKind::from_module_def(symbol.def).map(|kind| rust_type_definition(symbol, kind))
    }
}

impl_symbol_rust_type_definition_collector!(ListTypesAistCommandV5);
impl_list_types_aist_command!(ListTypesAistCommandV5, ListTypesAistCommandV5RunError);
