use errgonomic::{handle_bool, handle_opt};
use ra_ap_hir::{ItemInNs, Module, ModuleDef, Name, Trait};
use ra_ap_ide_db::RootDatabase;
use thiserror::Error;

pub fn resolve_trait(module: &Module, db: &RootDatabase, path: &str) -> Result<Trait, ResolveTraitError> {
    use ResolveTraitError::*;
    let mut traits = module
        .resolve_mod_path(db, path.split("::").map(Name::new_root))
        .into_iter()
        .flatten()
        .filter_map(|item| match item {
            ItemInNs::Types(ModuleDef::Trait(trait_)) => Some(trait_),
            _ => None,
        });
    let trait_ = handle_opt!(traits.next(), TraitNotFound, path: path.to_owned());
    handle_bool!(traits.next().is_some(), TraitNotUniqueInvalid, path: path.to_owned());
    Ok(trait_)
}

#[derive(Error, Debug)]
pub enum ResolveTraitError {
    #[error("trait path '{path}' could not be resolved")]
    TraitNotFound { path: String },
    #[error("trait path '{path}' is ambiguous")]
    TraitNotUniqueInvalid { path: String },
}
