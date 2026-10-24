use crate::{ignore_declaration_element, syntax_equal};
use errgonomic::handle_bool;
use ra_ap_syntax::{
    AstNode,
    ast::{Enum, HasName},
};
use thiserror::Error;

pub fn compare_enum_declaration(actual: &Enum, expected: &Enum, unconstrained_fields: &[&str]) -> Result<(), CompareEnumDeclarationError> {
    use CompareEnumDeclarationError::*;
    let valid_names = unconstrained_fields.iter().all(|name| {
        expected.variant_list().is_some_and(|variants| {
            variants
                .variants()
                .any(|variant| variant.name().is_some_and(|actual| actual.text() == *name))
        })
    });
    handle_bool!(!valid_names, VariantNotFound, expected: expected.syntax().to_string(), unconstrained_fields: unconstrained_fields.iter().map(|name| (*name).to_owned()).collect::<Vec<_>>());
    handle_bool!(!syntax_equal(actual.syntax(), expected.syntax(), &|element| ignore_declaration_element(element, actual.syntax(), unconstrained_fields), &|element| ignore_declaration_element(element, expected.syntax(), unconstrained_fields)), DeclarationInvalid, actual: actual.syntax().to_string(), expected: expected.syntax().to_string(), unconstrained_fields: unconstrained_fields.iter().map(|name| (*name).to_owned()).collect::<Vec<_>>());
    Ok(())
}

#[derive(Error, Debug)]
pub enum CompareEnumDeclarationError {
    #[error("unconstrained variant does not occur in the expected enum")]
    VariantNotFound { expected: String, unconstrained_fields: Vec<String> },
    #[error("enum differs from its required syntax: expected '{expected}', found '{actual}'", expected = expected.escape_debug(), actual = actual.escape_debug())]
    DeclarationInvalid { actual: String, expected: String, unconstrained_fields: Vec<String> },
}
