use crate::{RustTypeDefinition, RustTypeDefinitionCollector, impl_list_types_aist_command, rust_type_definitions_from_hir_modules};
use clap::Parser;
use ra_ap_ide_db::RootDatabase;
use serde::{Deserialize, Serialize};

/// Lists types by traversing macro-expanded HIR module declarations, associated items, and block DefMaps directly.
#[derive(Parser, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[command(flatten_help = true)]
pub struct ListTypesAistCommandV6;

impl RustTypeDefinitionCollector for ListTypesAistCommandV6 {
    fn rust_type_definitions(db: &RootDatabase) -> impl Iterator<Item = RustTypeDefinition> + '_ {
        rust_type_definitions_from_hir_modules(db)
    }
}

impl_list_types_aist_command!(ListTypesAistCommandV6, ListTypesAistCommandV6RunError);
