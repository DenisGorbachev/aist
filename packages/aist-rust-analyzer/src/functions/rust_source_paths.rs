use ra_ap_ide_db::FileId;
use ra_ap_vfs::Vfs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// PRUNING: omits virtual rust-analyzer files because macro-expanded declarations are mapped back to their real source files before lookup.
pub fn rust_source_paths(vfs: &Vfs) -> HashMap<FileId, PathBuf> {
    vfs.iter()
        .filter_map(|(file_id, vfs_path)| {
            vfs_path.as_path().map(|path| {
                let path: &Path = path.as_ref();
                (file_id, path.to_path_buf())
            })
        })
        .collect()
}
