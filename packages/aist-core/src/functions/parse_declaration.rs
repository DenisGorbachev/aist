use errgonomic::{handle_bool, handle_opt};
use ra_ap_syntax::{
    Edition, SourceFile, SyntaxError,
    ast::{HasModuleItem, Item},
};
use thiserror::Error;

pub fn parse_declaration(text: &str, edition: Edition) -> Result<Item, ParseDeclarationError> {
    use ParseDeclarationError::*;
    let parsed = SourceFile::parse(text, edition);
    let errors = parsed.errors();
    handle_bool!(!errors.is_empty(), SyntaxInvalid, text: text.to_owned(), errors);
    let mut items = parsed.tree().items();
    let item = handle_opt!(items.next(), ItemNotFound, text: text.to_owned());
    handle_bool!(items.next().is_some(), ItemNotUniqueInvalid, text: text.to_owned());
    Ok(item)
}

#[derive(Error, Debug)]
pub enum ParseDeclarationError {
    #[error("declaration contains invalid Rust syntax: {errors:?}")]
    SyntaxInvalid { text: String, errors: Vec<SyntaxError> },
    #[error("expected one Rust declaration")]
    ItemNotFound { text: String },
    #[error("expected one Rust declaration, found multiple items")]
    ItemNotUniqueInvalid { text: String },
}
