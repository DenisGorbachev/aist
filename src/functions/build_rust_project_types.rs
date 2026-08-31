use crate::{LocatedRustTypeDeclaration, RustFileTypes, RustProjectTypes};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// PRUNING: coalesces duplicate declarations emitted for multiple Cargo targets because each source declaration represents one project type in the output.
pub fn build_rust_project_types(declarations: impl IntoIterator<Item = LocatedRustTypeDeclaration>) -> RustProjectTypes {
    let mut types_by_path = declarations
        .into_iter()
        .fold(BTreeMap::<PathBuf, Vec<_>>::new(), |mut types_by_path, located| {
            types_by_path
                .entry(located.path)
                .or_default()
                .push(located.declaration);
            types_by_path
        });
    types_by_path.values_mut().for_each(|types| {
        types.sort_unstable_by(|left, right| {
            left.byte_offset
                .cmp(&right.byte_offset)
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.name.cmp(&right.name))
        });
        types.dedup();
    });
    let files = types_by_path
        .into_iter()
        .map(|(path, types)| RustFileTypes {
            path,
            types,
        })
        .collect();
    RustProjectTypes {
        files,
    }
}
