use crate::{RustTypeDefinition, RustTypeDefinitionFromSymbol, impl_list_types_aist_command, impl_symbol_rust_type_definition_collector, rust_type_definition, rust_type_kind_from_module_def};
use clap::Parser;
use ra_ap_hir::symbols::FileSymbol;
use serde::{Deserialize, Serialize};

/// Lists types by dispatching macro-expanded HIR definitions through a reusable matcher macro.
#[derive(Parser, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[command(flatten_help = true)]
pub struct ListTypesAistCommandV4;

impl RustTypeDefinitionFromSymbol for ListTypesAistCommandV4 {
    /// PRUNING: omits semantic symbols that do not define data types or type aliases because they are irrelevant to the requested listing.
    fn rust_type_definition(symbol: &FileSymbol<'_>) -> Option<RustTypeDefinition> {
        rust_type_kind_from_module_def!(symbol.def).map(|kind| rust_type_definition(symbol, kind))
    }
}

impl_symbol_rust_type_definition_collector!(ListTypesAistCommandV4);
impl_list_types_aist_command!(ListTypesAistCommandV4, ListTypesAistCommandV4RunError);
