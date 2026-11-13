use ra_ap_syntax::algo::non_trivia_sibling;
use ra_ap_syntax::{Direction, SyntaxElement, T};

pub fn is_trailing_comma(element: &SyntaxElement) -> bool {
    element.kind() == T![,] && non_trivia_sibling(element.clone(), Direction::Next).is_some_and(|next| matches!(next.kind(), T!['}'] | T![')'] | T![>]))
}
