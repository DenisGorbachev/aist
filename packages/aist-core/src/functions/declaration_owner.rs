use ra_ap_syntax::{AstNode, SyntaxNode, ast::Item};

pub fn declaration_owner(node: &SyntaxNode) -> Option<Item> {
    node.ancestors().skip(1).find_map(Item::cast)
}
