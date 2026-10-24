use crate::{ParseDeclarationError, parse_declaration};
use errgonomic::{handle, handle_opt};
use ra_ap_syntax::{
    Edition,
    ast::{Item, Path, Type},
};
use thiserror::Error;

pub fn parse_path(text: &str, edition: Edition) -> Result<Path, ParsePathError> {
    use ParsePathError::*;
    let source = format!("type AistPath = {text};");
    let item = handle!(parse_declaration(&source, edition), ParseDeclarationFailed, text: text.to_owned());
    let path = match item {
        Item::TypeAlias(item) => match item.ty() {
            Some(Type::PathType(typ)) => typ.path(),
            _ => None,
        },
        _ => None,
    };
    Ok(handle_opt!(path, PathInvalid, text: text.to_owned()))
}

#[derive(Error, Debug)]
pub enum ParsePathError {
    #[error("failed to parse Rust path '{text}'")]
    ParseDeclarationFailed { source: ParseDeclarationError, text: String },
    #[error("expected a Rust path, found '{text}'")]
    PathInvalid { text: String },
}
