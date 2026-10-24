use ra_ap_hir::{HasSource, ModuleDef};
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::{AstNode, ast};

pub fn definition_source(definition: ModuleDef, db: &RootDatabase) -> Option<ast::Item> {
    use ModuleDef::*;
    match definition {
        Adt(item) => item
            .source(db)
            .and_then(|source| ast::Item::cast(source.value.syntax().clone())),
        Trait(item) => item
            .source(db)
            .and_then(|source| ast::Item::cast(source.value.syntax().clone())),
        TypeAlias(item) => item
            .source(db)
            .and_then(|source| ast::Item::cast(source.value.syntax().clone())),
        Macro(item) => item
            .source(db)
            .and_then(|source| ast::Item::cast(source.value.syntax().clone())),
        _ => None,
    }
}
