use crate::{RustTypeDefinition, RustTypeKind};
use ra_ap_hir::{Adt, HasSource, ModuleDef, symbols::DeclarationLocation};
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::{AstNode, SyntaxNodePtr};

/// PRUNING: returns `None` for semantic definitions that do not declare a Rust data type or type alias.
pub fn rust_type_definition_from_module_def(db: &RootDatabase, module_def: ModuleDef) -> Option<RustTypeDefinition> {
    let kind = RustTypeKind::from_module_def(module_def);
    let name = module_def.name(db).map(|name| name.as_str().to_owned());
    let location = match module_def {
        ModuleDef::Adt(Adt::Enum(definition)) => definition.source(db).map(|source| DeclarationLocation {
            hir_file_id: source.file_id,
            ptr: SyntaxNodePtr::new(source.value.syntax()),
            name_ptr: None,
        }),
        ModuleDef::Adt(Adt::Struct(definition)) => definition.source(db).map(|source| DeclarationLocation {
            hir_file_id: source.file_id,
            ptr: SyntaxNodePtr::new(source.value.syntax()),
            name_ptr: None,
        }),
        ModuleDef::Adt(Adt::Union(definition)) => definition.source(db).map(|source| DeclarationLocation {
            hir_file_id: source.file_id,
            ptr: SyntaxNodePtr::new(source.value.syntax()),
            name_ptr: None,
        }),
        ModuleDef::TypeAlias(definition) => definition.source(db).map(|source| DeclarationLocation {
            hir_file_id: source.file_id,
            ptr: SyntaxNodePtr::new(source.value.syntax()),
            name_ptr: None,
        }),
        _ => None,
    };
    kind.zip(name)
        .zip(location)
        .map(|((kind, name), location)| RustTypeDefinition {
            name,
            kind,
            location,
        })
}
