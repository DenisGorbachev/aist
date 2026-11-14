use errgonomic::{handle_bool, handle_opt};
use ra_ap_hir::{Macro, Semantics};
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::{AstNode, ast::Meta};
use thiserror::Error;

pub fn resolve_derive_attribute(semantics: &Semantics<'_, RootDatabase>, attribute: &Meta) -> Result<impl Iterator<Item = Macro>, ResolveDeriveAttributeError> {
    use ResolveDeriveAttributeError::*;
    let macros = handle_opt!(semantics.resolve_derive_macro(attribute), ResolveDeriveMacroFailed, attribute: attribute.syntax().to_string());
    let unresolved: Vec<_> = macros
        .iter()
        .enumerate()
        .filter_map(|(index, macro_)| macro_.is_none().then_some(index))
        .collect();
    handle_bool!(!unresolved.is_empty(), DeriveMacroNotFound, attribute: attribute.syntax().to_string(), unresolved);
    Ok(macros.into_iter().flatten())
}

#[derive(Error, Debug)]
pub enum ResolveDeriveAttributeError {
    #[error("rust-analyzer could not resolve derive attribute '{attribute}'", attribute = attribute.escape_debug())]
    ResolveDeriveMacroFailed { attribute: String },
    #[error("unresolved derive macros at zero-based indices {unresolved:?} in '{attribute}'", attribute = attribute.escape_debug())]
    DeriveMacroNotFound { attribute: String, unresolved: Vec<usize> },
}
