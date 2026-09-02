use ra_ap_hir::{Adt, Crate, ModuleDef};
use ra_ap_ide_db::RootDatabase;

/// PRUNING: ignores non-ADT declarations and ADTs with other names because the caller requested only ADTs named `name`.
pub fn filter_adt<'a>(name: &'a str, krate: &Crate, db: &'a RootDatabase) -> impl Iterator<Item = Adt> + 'a {
    use ModuleDef::*;
    krate
        .modules(db)
        .into_iter()
        .flat_map(|module| module.declarations(db))
        .filter_map(move |definition| match definition {
            Adt(item) if item.name(db).as_str() == name => Some(item),
            _ => None,
        })
}
