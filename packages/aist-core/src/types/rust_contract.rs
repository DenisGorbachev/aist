use crate::{ContractReport, WorkspaceInfo, WorkspaceInfoFindCrateError, definition_source, item_name, item_shape, observe};
use errgonomic::{handle, handle_bool, handle_opt};
use quote::{ToTokens, format_ident, quote};
use ra_ap_hir::{AssocItem, Crate, HasSource, Impl, ModuleDef, attach_db};
use ra_ap_syntax::AstNode;
use syn::{ImplItemFn, Item, ItemStruct, Meta, TraitItemFn, Visibility, parse_str};
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

    pub fn definition(&self, name: &str, accepts: impl Fn(ModuleDef) -> bool) -> Result<ModuleDef, RustContractDefinitionError> {
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
        let name = handle_opt!(item_name(expected), ItemKindInvalid, expected: expected.to_token_stream().to_string());
        let definition = handle!(self.definition(&name, |definition| matches!((expected, definition), (Item::Struct(_) | Item::Enum(_), ModuleDef::Adt(_)) | (Item::Type(_), ModuleDef::TypeAlias(_)) | (Item::Trait(_), ModuleDef::Trait(_)) | (Item::Macro(_), ModuleDef::Macro(_)))), DefinitionFailed, name);
        let source = handle_opt!(definition_source(definition, &self.workspace.db), SourceNotFound, name);
        Ok(handle!(parse_str(&source), ParseStrFailed, text: source))
    }

    pub fn declaration(&self, report: &mut ContractReport, expected: &str) {
        let expected: Item = observe!(report, parse_str(expected));
        let Some(name) = item_name(&expected) else {
            report.require("supported declaration kind", false);
            return;
        };
        let actual = observe!(report, self.item(&expected));
        report.equal(format!("{name}/declaration"), item_shape(actual), item_shape(expected));
    }

    pub fn compiler_struct(&self, report: &mut ContractReport, expected: &str) {
        let expected: ItemStruct = observe!(report, parse_str(expected));
        let name = &expected.ident;
        let generics = &expected.generics;
        let (_, arguments, _) = generics.split_for_impl();
        let alias = format_ident!("{name}Rc");
        self.declaration(report, &quote!(pub type #alias #generics = Rc<#name #arguments>;).to_string());
        let name = name.to_string();
        let expected = Item::Struct(expected);
        let actual = observe!(report, self.item(&expected));
        let has_getters = matches!(&actual, Item::Struct(item) if item.attrs.iter().any(|attr| match &attr.meta {
            Meta::List(list) if list.path.is_ident("derive") => list.tokens.clone().into_iter().any(|token| token.to_string() == "Getters"),
            _ => false,
        }));
        report.equal(format!("{name}/declaration"), item_shape(actual), item_shape(expected));
        report.require(format!("{name}/derive Getters"), has_getters);
    }

    pub fn method(&self, report: &mut ContractReport, owner: &str, expected: &str) {
        let expected: TraitItemFn = observe!(report, parse_str(expected));
        let definition = observe!(report, self.definition(owner, |item| matches!(item, ModuleDef::Adt(_))));
        let ModuleDef::Adt(adt) = definition else {
            report.require(format!("{owner}/ADT"), false);
            return;
        };
        let db = &self.workspace.db;
        let methods = attach_db(db, || {
            Impl::all_for_type(db, adt.ty(db))
                .into_iter()
                .filter(|item| item.trait_(db).is_none())
                .flat_map(|item| item.items(db))
                .filter_map(|item| match item {
                    AssocItem::Function(function) if expected.sig.ident == function.name(db).as_str() => Some(function),
                    _ => None,
                })
                .collect::<Vec<_>>()
        });
        let [method] = methods.as_slice() else {
            report.equal(format!("{owner}::{}/unique method", expected.sig.ident), methods.len(), 1);
            return;
        };
        let Some(source) = method.source(db) else {
            report.require(format!("{owner}::{}/source", expected.sig.ident), false);
            return;
        };
        let mut actual: ImplItemFn = observe!(report, parse_str(&source.value.syntax().to_string()));
        actual.sig.inputs.pop_punct();
        report.require(format!("{owner}::{}/public", expected.sig.ident), matches!(actual.vis, Visibility::Public(_)));
        report.equal(format!("{owner}::{}/signature", expected.sig.ident), actual.sig.to_token_stream().to_string(), expected.sig.to_token_stream().to_string());
    }

    pub fn forbids_traits(&self, report: &mut ContractReport, owner: &str, forbidden: &[&str]) {
        let definition = observe!(report, self.definition(owner, |item| matches!(item, ModuleDef::Adt(_))));
        let ModuleDef::Adt(adt) = definition else {
            report.require(format!("{owner}/ADT"), false);
            return;
        };
        let db = &self.workspace.db;
        let traits = attach_db(db, || {
            Impl::all_for_type(db, adt.ty(db))
                .into_iter()
                .filter_map(|item| {
                    item.trait_(db)
                        .map(|trait_| trait_.name(db).as_str().to_owned())
                })
                .collect::<Vec<_>>()
        });
        forbidden
            .iter()
            .for_each(|name| report.require(format!("{owner}/does not implement {name}"), !traits.iter().any(|actual| actual == name)));
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
    #[error("failed to parse declaration")]
    ParseStrFailed { source: syn::Error, text: String },
}
