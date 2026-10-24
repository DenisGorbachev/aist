use ra_ap_syntax::algo::non_trivia_sibling;
use ra_ap_syntax::{Direction, SyntaxElement, SyntaxKind, SyntaxNode, T};

pub fn ignore_method_element(element: &SyntaxElement, root: &SyntaxNode) -> bool {
    use SyntaxKind::*;
    if element.kind().is_trivia() {
        return true;
    }
    let Some(parent) = element.parent() else {
        return false;
    };
    if parent == *root && matches!(element.kind(), ATTR | DOC_COMMENT | BLOCK_EXPR | T![;]) {
        return true;
    }
    element.kind() == T![,] && parent.kind() == PARAM_LIST && parent.parent().is_some_and(|owner| owner == *root) && non_trivia_sibling(element.clone(), Direction::Next).is_some_and(|next| next.kind() == T![')'])
}
