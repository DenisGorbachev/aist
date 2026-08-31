use crate::{RustTypeDefinition, rust_type_definition_from_module_def};
use ra_ap_hir::{Module, ModuleDef};
use ra_ap_ide_db::RootDatabase;

pub fn rust_type_definitions_from_module(db: &RootDatabase, module: Module) -> impl Iterator<Item = RustTypeDefinition> + '_ {
    let declarations = module.declarations(db);
    let trait_aliases = declarations
        .iter()
        .filter_map(|definition| match definition {
            ModuleDef::Trait(definition) => Some(*definition),
            _ => None,
        })
        .flat_map(|definition| definition.items(db))
        .filter_map(|item| item.as_type_alias())
        .map(ModuleDef::TypeAlias)
        .collect::<Vec<_>>();
    let impl_aliases = module
        .impl_defs(db)
        .into_iter()
        .flat_map(|definition| definition.items(db))
        .filter_map(|item| item.as_type_alias())
        .map(ModuleDef::TypeAlias);
    declarations
        .into_iter()
        .chain(trait_aliases)
        .chain(impl_aliases)
        .filter_map(move |definition| rust_type_definition_from_module_def(db, definition))
}
