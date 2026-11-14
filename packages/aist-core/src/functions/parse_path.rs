use errgonomic::{handle_bool, handle_opt};
use ra_ap_syntax::{
    AstNode, Edition, SyntaxError,
    ast::{Expr, Path, PathExpr},
};
use thiserror::Error;

pub fn parse_path(text: &str, edition: Edition) -> Result<Path, ParsePathError> {
    use ParsePathError::*;
    let parsed = Expr::parse(text, edition);
    let errors = parsed.errors();
    handle_bool!(!errors.is_empty(), SyntaxInvalid, text: text.to_owned(), errors);
    let path = PathExpr::cast(parsed.syntax_node()).and_then(|expr| expr.path());
    Ok(handle_opt!(path, PathInvalid, text: text.to_owned()))
}

#[derive(Error, Debug)]
pub enum ParsePathError {
    #[error("invalid Rust path: {errors:?}")]
    SyntaxInvalid { text: String, errors: Vec<SyntaxError> },
    #[error("expected a Rust path, found '{text}'", text = text.escape_debug())]
    PathInvalid { text: String },
}
