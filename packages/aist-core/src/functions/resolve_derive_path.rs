use crate::{ParsePathError, parse_path};
use errgonomic::{handle, handle_bool, handle_opt};
use ra_ap_hir::{Macro, ModPath, Module};
use ra_ap_hir_def::{ModuleId, nameres::MacroSubNs, resolver::HasResolver};
use ra_ap_ide_db::RootDatabase;
use ra_ap_span::SyntaxContext;
use ra_ap_syntax::{Edition, ast::HasGenericArgs};
use thiserror::Error;

pub fn resolve_derive_path(module: Module, db: &RootDatabase, path: &str, edition: Edition) -> Result<Macro, ResolveDerivePathError> {
    use ResolveDerivePathError::*;
    let syntax = handle!(parse_path(path, edition), ParsePathFailed);
    handle_bool!(syntax.segments().any(|segment| segment.generic_arg_list().is_some() || segment.type_anchor().is_some()), PathInvalid, path: path.to_owned());
    let mod_path = handle_opt!(ModPath::from_src(db, syntax, &mut |_| SyntaxContext::root(edition)), ModPathInvalid, path: path.to_owned());
    let module = ModuleId::from(module);
    let resolved = handle_opt!(module.resolver(db).resolve_path_as_macro(db, &mod_path, Some(MacroSubNs::Attr)), MacroNotFound, path: path.to_owned());
    let macro_ = Macro::from(resolved.def);
    handle_bool!(!macro_.is_derive(db), MacroKindInvalid, path: path.to_owned());
    Ok(macro_)
}

#[derive(Error, Debug)]
pub enum ResolveDerivePathError {
    #[error("failed to parse required derive path")]
    ParsePathFailed { source: ParsePathError },
    #[error("derive path '{path}' must not contain type arguments or qualified types", path = path.escape_debug())]
    PathInvalid { path: String },
    #[error("'{path}' is not a module path", path = path.escape_debug())]
    ModPathInvalid { path: String },
    #[error("macro '{path}' could not be resolved in the type's scope", path = path.escape_debug())]
    MacroNotFound { path: String },
    #[error("macro '{path}' is not a derive macro", path = path.escape_debug())]
    MacroKindInvalid { path: String },
}
