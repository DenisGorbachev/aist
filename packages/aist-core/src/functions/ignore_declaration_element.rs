use crate::{ignore_declaration_attribute, ignore_declaration_trailing_comma, ignore_variant_fields};
use ra_ap_syntax::{SyntaxElement, SyntaxKind, SyntaxNode, T};

pub fn ignore_declaration_element(element: &SyntaxElement, root: &SyntaxNode, unconstrained_variants: &[&str]) -> bool {
    use SyntaxKind::*;
    if element.kind().is_trivia() {
        return true;
    }
    match element.kind() {
        ATTR | DOC_COMMENT => ignore_declaration_attribute(element, root),
        RECORD_FIELD_LIST | TUPLE_FIELD_LIST => ignore_variant_fields(element, root, unconstrained_variants),
        T![,] => ignore_declaration_trailing_comma(element, root),
        _ => false,
    }
}
