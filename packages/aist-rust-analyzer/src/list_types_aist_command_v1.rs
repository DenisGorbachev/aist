use crate::{RustTypeDefinition, RustTypeDefinitionFromSymbol, RustTypeKind, impl_list_types_aist_command, impl_symbol_rust_type_definition_collector, rust_type_definition};
use clap::Parser;
use ra_ap_hir::{Adt, ModuleDef, symbols::FileSymbol};
use serde::{Deserialize, Serialize};

/// Lists types by exhaustively matching each macro-expanded HIR definition.
#[derive(Parser, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[command(flatten_help = true)]
pub struct ListTypesAistCommandV1;

impl RustTypeDefinitionFromSymbol for ListTypesAistCommandV1 {
    /// PRUNING: omits semantic symbols that do not define data types or type aliases because they are irrelevant to the requested listing.
    fn rust_type_definition(symbol: &FileSymbol<'_>) -> Option<RustTypeDefinition> {
        use Adt::{Enum as EnumAdt, Struct as StructAdt, Union as UnionAdt};
        use RustTypeKind::*;
        let kind = match symbol.def {
            ModuleDef::Adt(EnumAdt(_)) => Some(Enum),
            ModuleDef::Adt(StructAdt(_)) => Some(Struct),
            ModuleDef::Adt(UnionAdt(_)) => Some(Union),
            ModuleDef::TypeAlias(_) => Some(TypeAlias),
            _ => None,
        };
        kind.map(|kind| rust_type_definition(symbol, kind))
    }
}

impl_symbol_rust_type_definition_collector!(ListTypesAistCommandV1);
impl_list_types_aist_command!(ListTypesAistCommandV1, ListTypesAistCommandV1RunError);
