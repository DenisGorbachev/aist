use crate::{RustTypeDefinition, rust_type_definitions_from_module};
use ra_ap_hir::{Crate, Module};
use ra_ap_hir_def::{DefWithBodyId, expr_store::Body};
use ra_ap_ide_db::RootDatabase;
use std::collections::HashSet;
use std::iter::from_fn;

/// PRUNING: omits dependency crates because the requested listing is limited to types declared by workspace members.
pub fn rust_type_definitions_from_hir_modules(db: &RootDatabase) -> impl Iterator<Item = RustTypeDefinition> + '_ {
    let mut pending = Crate::all(db)
        .into_iter()
        .filter(|krate| krate.origin(db).is_local())
        .flat_map(|krate| krate.modules(db))
        .collect::<Vec<_>>();
    let mut visited = HashSet::<Module>::new();
    from_fn(move || {
        while let Some(module) = pending.pop() {
            if visited.insert(module) {
                let block_modules = module
                    .declarations(db)
                    .into_iter()
                    .filter_map(|definition| definition.as_def_with_body())
                    .flat_map(|definition| {
                        let body_id = DefWithBodyId::try_from(definition).expect("always succeeds because module declarations cannot contain synthesized builtin-derive methods");
                        let body = Body::of(db, body_id);
                        body.blocks(db)
                            .flat_map(|(_, def_map)| {
                                def_map
                                    .modules()
                                    .map(|(module_id, _)| Module::from(module_id))
                            })
                            .collect::<Vec<_>>()
                    });
                pending.extend(block_modules);
                return Some(module);
            }
        }
        None
    })
    .flat_map(move |module| rust_type_definitions_from_module(db, module))
}
