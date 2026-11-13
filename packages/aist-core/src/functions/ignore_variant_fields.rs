use crate::declaration_owner;
use ra_ap_syntax::{
    AstNode, SyntaxElement, SyntaxKind, SyntaxNode,
    ast::{HasName, Variant},
};

pub fn ignore_variant_fields(element: &SyntaxElement, root: &SyntaxNode, unconstrained_variants: &[&str]) -> bool {
    use SyntaxKind::*;
    if !matches!(element.kind(), RECORD_FIELD_LIST | TUPLE_FIELD_LIST) {
        return false;
    }
    let Some(variant) = element.parent().and_then(Variant::cast) else {
        return false;
    };
    declaration_owner(variant.syntax()).is_some_and(|owner| owner.syntax() == root)
        && variant.name().is_some_and(|name| {
            unconstrained_variants
                .iter()
                .any(|ignored| name.text() == *ignored)
        })
}
