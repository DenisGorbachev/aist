use crate::{RustTypeDefinition, RustTypeDefinitionFromSymbol, RustTypeKind, impl_list_types_aist_command, impl_symbol_rust_type_definition_collector, rust_type_definition};
use clap::Parser;
use ra_ap_hir::{ModuleDef, symbols::FileSymbol};
use serde::{Deserialize, Serialize};

/// Lists types by projecting macro-expanded HIR definitions through rust-analyzer's ADT sum type.
#[derive(Parser, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[command(flatten_help = true)]
pub struct ListTypesAistCommandV2;

impl RustTypeDefinitionFromSymbol for ListTypesAistCommandV2 {
    /// PRUNING: omits semantic symbols that do not define data types or type aliases because they are irrelevant to the requested listing.
    fn rust_type_definition(symbol: &FileSymbol<'_>) -> Option<RustTypeDefinition> {
        match symbol.def {
            ModuleDef::Adt(adt) => Some(rust_type_definition(symbol, RustTypeKind::from(adt))),
            ModuleDef::TypeAlias(_) => Some(rust_type_definition(symbol, RustTypeKind::TypeAlias)),
            _ => None,
        }
    }
}

impl_symbol_rust_type_definition_collector!(ListTypesAistCommandV2);
impl_list_types_aist_command!(ListTypesAistCommandV2, ListTypesAistCommandV2RunError);
