use crate::{declaration_owner, is_trailing_comma};
use ra_ap_syntax::{AstNode, SyntaxElement, SyntaxKind, SyntaxNode};

pub fn ignore_declaration_trailing_comma(element: &SyntaxElement, root: &SyntaxNode) -> bool {
    use SyntaxKind::*;
    let Some(parent) = element.parent() else {
        return false;
    };
    let allowed = match parent.kind() {
        GENERIC_PARAM_LIST => matches!(root.kind(), STRUCT | ENUM | TYPE_ALIAS) && parent.parent().is_some_and(|owner| owner == *root),
        RECORD_FIELD_LIST | TUPLE_FIELD_LIST => matches!(root.kind(), STRUCT | ENUM) && declaration_owner(&parent).is_some_and(|owner| owner.syntax() == root),
        VARIANT_LIST => root.kind() == ENUM && parent.parent().is_some_and(|owner| owner == *root),
        _ => false,
    };
    allowed && is_trailing_comma(element)
}
