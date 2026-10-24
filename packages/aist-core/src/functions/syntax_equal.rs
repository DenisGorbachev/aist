use crate::{iter_eq_by, token_trees_equal};
use ra_ap_syntax::{NodeOrToken, SyntaxElement, SyntaxKind, SyntaxNode};

pub fn syntax_equal(actual: &SyntaxNode, expected: &SyntaxNode, ignore_actual: &impl Fn(&SyntaxElement) -> bool, ignore_expected: &impl Fn(&SyntaxElement) -> bool) -> bool {
    if actual.kind() != expected.kind() {
        return false;
    }
    if actual.kind() == SyntaxKind::TOKEN_TREE {
        return token_trees_equal(actual, expected);
    }
    iter_eq_by(
        actual
            .children_with_tokens()
            .filter(|element| !ignore_actual(element)),
        expected
            .children_with_tokens()
            .filter(|element| !ignore_expected(element)),
        |actual, expected| match (actual, expected) {
            (NodeOrToken::Node(actual), NodeOrToken::Node(expected)) => syntax_equal(&actual, &expected, ignore_actual, ignore_expected),
            (NodeOrToken::Token(actual), NodeOrToken::Token(expected)) => actual.kind() == expected.kind() && actual.text() == expected.text(),
            _ => false,
        },
    )
}
