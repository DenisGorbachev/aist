use crate::{CompareDeclarationError, CompareEnumDeclarationError, CompareMethodError, ParseDeclarationError, ParseDeriveAttributeError, ParsePathError, WorkspaceInfo, WorkspaceInfoFindCrateError, compare_declaration, compare_enum_declaration, compare_method, definition_source, item_name, parse_declaration, parse_derive_attribute, parse_path, syntax_equal};
use errgonomic::{ErrVec, handle, handle_bool, handle_iter, handle_opt};
use ra_ap_hir::{AssocItem, Crate, HasSource, Impl, ModuleDef, attach_db};
use ra_ap_syntax::{
    AstNode,
    ast::{self, HasName, Item},
};
use thiserror::Error;

pub struct RustContract<'a> {
    pub workspace: &'a WorkspaceInfo,
    pub krate: Crate,
}

impl<'a> RustContract<'a> {
    pub fn new(workspace: &'a WorkspaceInfo, name: &str) -> Result<Self, RustContractNewError> {
        use RustContractNewError::*;
        let krate = handle!(workspace.find_crate(name, "lib.rs"), FindCrateFailed, name: name.to_owned());
        Ok(Self {
            workspace,
            krate,
        })
    }

    pub fn definition(&self, name: &str, mut accepts: impl FnMut(ModuleDef) -> bool) -> Result<ModuleDef, RustContractDefinitionError> {
        use RustContractDefinitionError::*;
        let db = &self.workspace.db;
        let mut definitions = self
            .krate
            .modules(db)
            .into_iter()
            .flat_map(|module| module.declarations(db))
            .filter(|definition| {
                accepts(*definition)
                    && definition
                        .name(db)
                        .is_some_and(|actual| actual.as_str() == name)
            });
        let definition = handle_opt!(definitions.next(), DefinitionNotFound, name: name.to_owned());
        handle_bool!(definitions.next().is_some(), DefinitionNotUniqueInvalid, name: name.to_owned());
        Ok(definition)
    }

    pub fn item(&self, expected: &Item) -> Result<Item, RustContractItemError> {
        use RustContractItemError::*;
        let name = handle_opt!(item_name(expected), ItemKindInvalid, expected: expected.syntax().to_string());
        let definition = handle!(self.definition(&name, |definition| matches!((expected, definition), (Item::Struct(_) | Item::Enum(_), ModuleDef::Adt(_)) | (Item::TypeAlias(_), ModuleDef::TypeAlias(_)) | (Item::Trait(_), ModuleDef::Trait(_)) | (Item::MacroRules(_) | Item::MacroDef(_), ModuleDef::Macro(_)))), DefinitionFailed, name);
        Ok(handle_opt!(definition_source(definition, &self.workspace.db), SourceNotFound, name))
    }

    pub fn declaration(&self, expected: &str) -> Result<(), RustContractDeclarationError> {
        use RustContractDeclarationError::*;
        let expected = handle!(parse_declaration(expected, self.krate.edition(&self.workspace.db)), ParseDeclarationFailed);
        let actual = handle!(self.item(&expected), ItemFailed, expected: expected.syntax().to_string());
        handle!(compare_declaration(&actual, &expected), CompareDeclarationFailed);
        Ok(())
    }

    pub fn require_derive(&self, owner: &str, path: &str) -> Result<(), RustContractRequireDeriveError> {
        use RustContractRequireDeriveError::*;
        let edition = self.krate.edition(&self.workspace.db);
        let required = handle!(parse_path(path, edition), ParsePathFailed, owner: owner.to_owned());
        let definition = handle!(self.definition(owner, |item| matches!(item, ModuleDef::Adt(_))), DefinitionFailed, path: path.to_owned());
        let item = handle_opt!(definition_source(definition, &self.workspace.db), SourceNotFound, owner: owner.to_owned(), path: path.to_owned());
        let attributes = item
            .syntax()
            .children()
            .filter_map(ast::Attr::cast)
            .filter(|attr| attr.simple_name().is_some_and(|name| name == "derive"));
        let paths = handle_iter!(attributes.map(|attr| parse_derive_attribute(&attr, edition)), ParseDeriveAttributeFailed, owner: owner.to_owned(), path: path.to_owned());
        let found = paths
            .iter()
            .flatten()
            .any(|actual| syntax_equal(actual.syntax(), required.syntax(), &|element| element.kind().is_trivia(), &|element| element.kind().is_trivia()));
        handle_bool!(!found, DeriveNotFound, owner: owner.to_owned(), path: path.to_owned(), actual: item.syntax().to_string());
        Ok(())
    }

    pub fn method(&self, owner: &str, expected: &str) -> Result<(), RustContractMethodError> {
        use RustContractMethodError::*;
        let item = handle!(parse_declaration(expected, self.krate.edition(&self.workspace.db)), ParseDeclarationFailed, owner: owner.to_owned());
        let Item::Fn(expected_fn) = item else {
            return Err(MethodInvalid {
                owner: owner.to_owned(),
                expected: expected.to_owned(),
            });
        };
        let name = handle_opt!(expected_fn.name(), NameNotFound, owner: owner.to_owned(), expected: expected.to_owned());
        let definition = handle!(self.definition(owner, |item| matches!(item, ModuleDef::Adt(_))), DefinitionFailed, expected: expected.to_owned());
        let ModuleDef::Adt(adt) = definition else {
            return Err(AdtInvalid {
                owner: owner.to_owned(),
                expected: expected.to_owned(),
            });
        };
        let db = &self.workspace.db;
        let methods = attach_db(db, || {
            Impl::all_for_type(db, adt.ty(db))
                .into_iter()
                .filter(|item| item.trait_(db).is_none())
                .flat_map(|item| item.items(db))
                .filter_map(|item| match item {
                    AssocItem::Function(function) if name.text() == function.name(db).as_str() => Some(function),
                    _ => None,
                })
                .collect::<Vec<_>>()
        });
        let [method] = methods.as_slice() else {
            return Err(MethodNotUniqueInvalid {
                owner: owner.to_owned(),
                expected: expected.to_owned(),
                found: methods.len(),
            });
        };
        let source = handle_opt!(method.source(db), SourceNotFound, owner: owner.to_owned(), expected: expected.to_owned());
        handle!(compare_method(&source.value, &expected_fn), CompareMethodFailed, owner: owner.to_owned());
        Ok(())
    }

    pub fn forbids_traits(&self, owner: &str, forbidden: &[&str]) -> Result<(), RustContractForbidsTraitsError> {
        use RustContractForbidsTraitsError::*;
        let forbidden: Vec<_> = forbidden.iter().map(|name| (*name).to_owned()).collect();
        let definition = handle!(self.definition(owner, |item| matches!(item, ModuleDef::Adt(_))), DefinitionFailed, forbidden);
        let ModuleDef::Adt(adt) = definition else {
            return Err(AdtInvalid {
                owner: owner.to_owned(),
                forbidden,
            });
        };
        let db = &self.workspace.db;
        let found = attach_db(db, || {
            Impl::all_for_type(db, adt.ty(db))
                .into_iter()
                .filter_map(|item| {
                    item.trait_(db)
                        .map(|trait_| trait_.name(db).as_str().to_owned())
                })
                .filter(|name| forbidden.contains(name))
                .collect::<Vec<_>>()
        });
        handle_bool!(!found.is_empty(), TraitsInvalid, owner: owner.to_owned(), forbidden, found);
        Ok(())
    }

    pub fn test_fn(&self, name: &str) -> Result<(), RustContractTestFnError> {
        use RustContractTestFnError::*;
        let definition = handle!(self.definition(name, |item| matches!(item, ModuleDef::Function(_))), DefinitionFailed);
        handle_bool!(!matches!(definition, ModuleDef::Function(function) if function.is_test(&self.workspace.db)), TestInvalid, name: name.to_owned());
        Ok(())
    }

    pub fn enum_declaration(&self, expected: &str, unconstrained_fields: &[&str]) -> Result<(), RustContractEnumDeclarationError> {
        use RustContractEnumDeclarationError::*;
        let expected = handle!(parse_declaration(expected, self.krate.edition(&self.workspace.db)), ParseDeclarationFailed);
        let actual = handle!(self.item(&expected), ItemFailed, expected: expected.syntax().to_string());
        let (Item::Enum(actual), Item::Enum(expected)) = (&actual, &expected) else {
            return Err(EnumInvalid {
                actual: actual.syntax().to_string(),
                expected: expected.syntax().to_string(),
            });
        };
        handle!(compare_enum_declaration(actual, expected, unconstrained_fields), CompareEnumDeclarationFailed);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum RustContractNewError {
    #[error("failed to find library '{name}'")]
    FindCrateFailed { source: Box<WorkspaceInfoFindCrateError>, name: String },
}

#[derive(Error, Debug)]
pub enum RustContractDefinitionError {
    #[error("declaration '{name}' not found")]
    DefinitionNotFound { name: String },
    #[error("declaration '{name}' is not unique")]
    DefinitionNotUniqueInvalid { name: String },
}

#[derive(Error, Debug)]
pub enum RustContractItemError {
    #[error("unsupported declaration kind")]
    ItemKindInvalid { expected: String },
    #[error("failed to find declaration '{name}'")]
    DefinitionFailed { source: RustContractDefinitionError, name: String },
    #[error("source for '{name}' not found")]
    SourceNotFound { name: String },
}

#[derive(Error, Debug)]
pub enum RustContractDeclarationError {
    #[error("failed to parse expected declaration")]
    ParseDeclarationFailed { source: ParseDeclarationError },
    #[error("failed to get actual declaration")]
    ItemFailed { source: RustContractItemError, expected: String },
    #[error("declaration check failed")]
    CompareDeclarationFailed { source: CompareDeclarationError },
}

#[derive(Error, Debug)]
pub enum RustContractRequireDeriveError {
    #[error("failed to parse required derive path for '{owner}'")]
    ParsePathFailed { source: ParsePathError, owner: String },
    #[error("failed to locate type requiring derive '{path}'")]
    DefinitionFailed { source: RustContractDefinitionError, path: String },
    #[error("source for '{owner}' not found")]
    SourceNotFound { owner: String, path: String },
    #[error("failed to read derives for '{owner}'")]
    ParseDeriveAttributeFailed { source: ErrVec<ParseDeriveAttributeError>, owner: String, path: String },
    #[error("'{owner}' must derive '{path}'")]
    DeriveNotFound { owner: String, path: String, actual: String },
}

#[derive(Error, Debug)]
pub enum RustContractMethodError {
    #[error("failed to parse required method for '{owner}'")]
    ParseDeclarationFailed { source: ParseDeclarationError, owner: String },
    #[error("expected a method signature for '{owner}'")]
    MethodInvalid { owner: String, expected: String },
    #[error("required method for '{owner}' has no name")]
    NameNotFound { owner: String, expected: String },
    #[error("failed to find method owner")]
    DefinitionFailed { source: RustContractDefinitionError, expected: String },
    #[error("'{owner}' is not an ADT")]
    AdtInvalid { owner: String, expected: String },
    #[error("expected one matching inherent method on '{owner}', found {found}")]
    MethodNotUniqueInvalid { owner: String, expected: String, found: usize },
    #[error("source for method on '{owner}' not found")]
    SourceNotFound { owner: String, expected: String },
    #[error("method check failed for '{owner}'")]
    CompareMethodFailed { source: CompareMethodError, owner: String },
}

#[derive(Error, Debug)]
pub enum RustContractForbidsTraitsError {
    #[error("failed to find type")]
    DefinitionFailed { source: RustContractDefinitionError, forbidden: Vec<String> },
    #[error("'{owner}' is not an ADT")]
    AdtInvalid { owner: String, forbidden: Vec<String> },
    #[error("'{owner}' implements forbidden traits: {found:?}")]
    TraitsInvalid { owner: String, forbidden: Vec<String>, found: Vec<String> },
}

#[derive(Error, Debug)]
pub enum RustContractTestFnError {
    #[error("failed to find test function")]
    DefinitionFailed { source: RustContractDefinitionError },
    #[error("'{name}' is not a test function")]
    TestInvalid { name: String },
}

#[derive(Error, Debug)]
pub enum RustContractEnumDeclarationError {
    #[error("failed to parse expected enum")]
    ParseDeclarationFailed { source: ParseDeclarationError },
    #[error("failed to get actual enum")]
    ItemFailed { source: RustContractItemError, expected: String },
    #[error("expected two enum declarations")]
    EnumInvalid { actual: String, expected: String },
    #[error("enum declaration check failed")]
    CompareEnumDeclarationFailed { source: CompareEnumDeclarationError },
}
