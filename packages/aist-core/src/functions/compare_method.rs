use crate::{ignore_method_element, syntax_equal};
use errgonomic::handle_bool;
use ra_ap_syntax::{
    AstNode,
    ast::{Fn, HasVisibility},
};
use thiserror::Error;

pub fn compare_method(actual: &Fn, expected: &Fn) -> Result<(), CompareMethodError> {
    use CompareMethodError::*;
    let public = actual
        .visibility()
        .is_some_and(|visibility| visibility.pub_token().is_some() && visibility.visibility_inner().is_none());
    handle_bool!(!public, VisibilityInvalid, actual: actual.syntax().to_string(), expected: expected.syntax().to_string());
    handle_bool!(!syntax_equal(actual.syntax(), expected.syntax(), &|element| ignore_method_element(element, actual.syntax()), &|element| ignore_method_element(element, expected.syntax())), SignatureInvalid, actual: actual.syntax().to_string(), expected: expected.syntax().to_string());
    Ok(())
}

#[derive(Error, Debug)]
pub enum CompareMethodError {
    #[error("required method is not public")]
    VisibilityInvalid { actual: String, expected: String },
    #[error("method signature differs from its required syntax: expected '{expected}', found '{actual}'", expected = expected.escape_debug(), actual = actual.escape_debug())]
    SignatureInvalid { actual: String, expected: String },
}
