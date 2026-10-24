use crate::{ParsePathError, parse_path};
use errgonomic::{handle, handle_opt};
use ra_ap_syntax::{
    AstNode, Edition,
    ast::{Attr, GenericArg, HasGenericArgs, Path, Type},
};
use thiserror::Error;

pub fn parse_derive_attribute(attr: &Attr, edition: Edition) -> Result<Vec<Path>, ParseDeriveAttributeError> {
    use ParseDeriveAttributeError::*;
    let (_, tokens) = handle_opt!(attr.as_simple_call(), AttributeInvalid, attribute: attr.syntax().to_string());
    let text = tokens.syntax().text().to_string();
    let inner = handle_opt!(text.strip_prefix('(').and_then(|text| text.strip_suffix(')')), DelimitersInvalid, attribute: attr.syntax().to_string());
    let source = format!("AistDerives<{inner}>");
    let path = handle!(parse_path(&source, edition), ParsePathFailed, attribute: attr.syntax().to_string());
    let arguments = handle_opt!(path.segment().and_then(|segment| segment.generic_arg_list()), ArgumentsNotFound, attribute: attr.syntax().to_string());
    let paths = arguments
        .generic_args()
        .map(|argument| match argument {
            GenericArg::TypeArg(argument) => match argument.ty() {
                Some(Type::PathType(typ)) => typ.path(),
                _ => None,
            },
            _ => None,
        })
        .collect::<Option<Vec<_>>>();
    Ok(handle_opt!(paths, PathInvalid, attribute: attr.syntax().to_string()))
}

#[derive(Error, Debug)]
pub enum ParseDeriveAttributeError {
    #[error("expected a derive attribute with arguments")]
    AttributeInvalid { attribute: String },
    #[error("expected parentheses around derive paths")]
    DelimitersInvalid { attribute: String },
    #[error("failed to parse derive paths")]
    ParsePathFailed { source: ParsePathError, attribute: String },
    #[error("derive arguments are missing")]
    ArgumentsNotFound { attribute: String },
    #[error("derive arguments must be paths")]
    PathInvalid { attribute: String },
}
