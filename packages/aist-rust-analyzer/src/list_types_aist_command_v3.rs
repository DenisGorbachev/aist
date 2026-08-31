use crate::{RustTypeDefinition, RustTypeDefinitionFromSymbol, RustTypeKind, impl_list_types_aist_command, impl_symbol_rust_type_definition_collector, rust_type_definition};
use clap::Parser;
use ra_ap_hir::{ModuleDef, symbols::FileSymbol};
use serde::{Deserialize, Serialize};

/// Lists types by chaining separate macro-expanded ADT and type-alias projections.
#[derive(Parser, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[command(flatten_help = true)]
pub struct ListTypesAistCommandV3;

impl RustTypeDefinitionFromSymbol for ListTypesAistCommandV3 {
    /// PRUNING: omits semantic symbols that do not define data types or type aliases because they are irrelevant to the requested listing.
    fn rust_type_definition(symbol: &FileSymbol<'_>) -> Option<RustTypeDefinition> {
        let adt = match symbol.def {
            ModuleDef::Adt(adt) => Some(rust_type_definition(symbol, RustTypeKind::from(adt))),
            _ => None,
        };
        adt.or_else(|| match symbol.def {
            ModuleDef::TypeAlias(_) => Some(rust_type_definition(symbol, RustTypeKind::TypeAlias)),
            _ => None,
        })
    }
}

impl_symbol_rust_type_definition_collector!(ListTypesAistCommandV3);
impl_list_types_aist_command!(ListTypesAistCommandV3, ListTypesAistCommandV3RunError);
