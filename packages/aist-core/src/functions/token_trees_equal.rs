use crate::{ComparisonSpan, iter_eq_by};
use ra_ap_syntax::SyntaxNode;
use ra_ap_syntax_bridge::{DocCommentDesugarMode, syntax_node_to_token_tree};
use ra_ap_tt::{Leaf, Spacing, TokenTree};

pub fn token_trees_equal(actual: &SyntaxNode, expected: &SyntaxNode) -> bool {
    let span = ComparisonSpan::default();
    let actual = syntax_node_to_token_tree(actual, span, span.0, DocCommentDesugarMode::ProcMacro);
    let expected = syntax_node_to_token_tree(expected, span, span.0, DocCommentDesugarMode::ProcMacro);
    iter_eq_by(actual.view().as_token_trees().iter_flat_tokens(), expected.view().as_token_trees().iter_flat_tokens(), |actual, expected| match (&actual, &expected) {
        (TokenTree::Leaf(Leaf::Punct(actual)), TokenTree::Leaf(Leaf::Punct(expected))) => actual.char == expected.char && (actual.spacing == Spacing::Joint) == (expected.spacing == Spacing::Joint),
        _ => actual == expected,
    })
}
