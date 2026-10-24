use ra_ap_syntax::algo::non_trivia_sibling;
use ra_ap_syntax::{
    AstNode, Direction, SyntaxElement, SyntaxKind, SyntaxNode, T,
    ast::{Attr, HasName, Variant},
};

pub fn ignore_declaration_element(element: &SyntaxElement, root: &SyntaxNode, unconstrained_fields: &[&str]) -> bool {
    use SyntaxKind::*;
    let Some(parent) = element.parent() else {
        return false;
    };
    if element.kind().is_trivia() {
        return true;
    }
    match element.kind() {
        ATTR => {
            if parent == *root {
                return !matches!(root.kind(), MACRO_RULES | MACRO_DEF)
                    || element
                        .as_node()
                        .and_then(|node| Attr::cast(node.clone()))
                        .and_then(|attr| attr.simple_name())
                        .is_some_and(|name| name == "doc");
            }
            match parent.kind() {
                RECORD_FIELD | TUPLE_FIELD => parent.ancestors().any(|node| node == *root) && matches!(root.kind(), STRUCT | ENUM),
                VARIANT => parent
                    .parent()
                    .and_then(|list| list.parent())
                    .is_some_and(|owner| owner == *root),
                FN | TYPE_ALIAS => {
                    root.kind() == TRAIT
                        && parent
                            .parent()
                            .and_then(|list| list.parent())
                            .is_some_and(|owner| owner == *root)
                }
                _ => false,
            }
        }
        RECORD_FIELD_LIST | TUPLE_FIELD_LIST => {
            parent.kind() == VARIANT
                && Variant::cast(parent)
                    .and_then(|variant| variant.name())
                    .is_some_and(|name| {
                        unconstrained_fields
                            .iter()
                            .any(|ignored| name.text() == *ignored)
                    })
        }
        T![,] => {
            let list_is_normalized = match parent.kind() {
                GENERIC_PARAM_LIST => parent.parent().is_some_and(|owner| owner == *root) && matches!(root.kind(), STRUCT | ENUM | TYPE_ALIAS),
                RECORD_FIELD_LIST | TUPLE_FIELD_LIST => matches!(root.kind(), STRUCT | ENUM),
                VARIANT_LIST => parent.parent().is_some_and(|owner| owner == *root),
                _ => false,
            };
            list_is_normalized && non_trivia_sibling(element.clone(), Direction::Next).is_some_and(|next| matches!(next.kind(), T!['}'] | T![')'] | T![>]))
        }
        _ => false,
    }
}
