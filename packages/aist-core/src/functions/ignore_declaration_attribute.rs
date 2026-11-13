use crate::declaration_owner;
use ra_ap_syntax::{AstNode, SyntaxElement, SyntaxKind, SyntaxNode, ast::Attr};

pub fn ignore_declaration_attribute(element: &SyntaxElement, root: &SyntaxNode) -> bool {
    use SyntaxKind::*;
    if !matches!(element.kind(), ATTR | DOC_COMMENT) {
        return false;
    }
    let Some(parent) = element.parent() else {
        return false;
    };
    if parent == *root {
        return element.kind() == DOC_COMMENT
            || !matches!(root.kind(), MACRO_RULES | MACRO_DEF)
            || element
                .as_node()
                .and_then(|node| Attr::cast(node.clone()))
                .and_then(|attr| attr.simple_name())
                .is_some_and(|name| name == "doc");
    }
    let allowed = match parent.kind() {
        RECORD_FIELD | TUPLE_FIELD | VARIANT => matches!(root.kind(), STRUCT | ENUM),
        FN | TYPE_ALIAS => root.kind() == TRAIT,
        _ => false,
    };
    allowed && declaration_owner(&parent).is_some_and(|owner| owner.syntax() == root)
}
