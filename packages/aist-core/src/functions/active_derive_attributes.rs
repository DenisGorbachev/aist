use ra_ap_hir::{Crate, Semantics};
use ra_ap_hir_expand::attrs::expand_cfg_attr;
use ra_ap_ide_db::RootDatabase;
use ra_ap_syntax::ast::{HasAttrs, Meta};
use std::convert::Infallible;
use std::ops::ControlFlow;

pub fn active_derive_attributes(item: &impl HasAttrs, krate: &Crate, semantics: &Semantics<'_, RootDatabase>) -> Vec<Meta> {
    let mut attributes = Vec::new();
    let _ = expand_cfg_attr(
        item.attrs(),
        || krate.cfg(semantics.db),
        |meta, _| {
            if meta.simple_name().is_some_and(|name| name == "derive") || semantics.resolve_derive_macro(&meta).is_some() {
                attributes.push(meta);
            }
            ControlFlow::<Infallible>::Continue(())
        },
    );
    attributes
}
