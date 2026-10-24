use crate::{ignore_declaration_element, syntax_equal};
use errgonomic::handle_bool;
use ra_ap_syntax::{AstNode, ast::Item};
use thiserror::Error;

pub fn compare_declaration(actual: &Item, expected: &Item) -> Result<(), CompareDeclarationError> {
    use CompareDeclarationError::*;
    handle_bool!(!syntax_equal(actual.syntax(), expected.syntax(), &|element| ignore_declaration_element(element, actual.syntax(), &[]), &|element| ignore_declaration_element(element, expected.syntax(), &[])), DeclarationInvalid, actual: actual.syntax().to_string(), expected: expected.syntax().to_string());
    Ok(())
}

#[derive(Error, Debug)]
pub enum CompareDeclarationError {
    #[error("declaration differs from its required syntax: expected '{expected}', found '{actual}'", expected = expected.escape_debug(), actual = actual.escape_debug())]
    DeclarationInvalid { actual: String, expected: String },
}
