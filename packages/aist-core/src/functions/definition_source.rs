use ra_ap_hir::{HasSource, ModuleDef};
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::AstNode;

pub fn definition_source(definition: ModuleDef, db: &RootDatabase) -> Option<String> {
    use ModuleDef::*;
    match definition {
        Adt(item) => item
            .source(db)
            .map(|source| source.value.syntax().to_string()),
        Trait(item) => item
            .source(db)
            .map(|source| source.value.syntax().to_string()),
        TypeAlias(item) => item
            .source(db)
            .map(|source| source.value.syntax().to_string()),
        Macro(item) => item
            .source(db)
            .map(|source| source.value.syntax().to_string()),
        _ => None,
    }
}
