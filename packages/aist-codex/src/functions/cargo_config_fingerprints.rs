use crate::{Fingerprint, ReadOptionalFingerprintError, read_optional_fingerprint};
use errgonomic::{ErrVec, handle_iter};
use futures::{StreamExt, stream};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub async fn cargo_config_fingerprints(directory: &Path) -> Result<BTreeMap<PathBuf, Fingerprint>, CargoConfigFingerprintsError> {
    use CargoConfigFingerprintsError::*;
    let cargo_home = env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cargo")));
    let directories = directory
        .ancestors()
        .map(|directory| directory.join(".cargo"))
        .chain(cargo_home)
        .collect::<BTreeSet<_>>();
    let paths = directories
        .into_iter()
        .flat_map(|directory| [directory.join("config"), directory.join("config.toml")]);
    let results = stream::iter(paths)
        .then(|path| async move {
            read_optional_fingerprint(&path)
                .await
                .map(|fingerprint| fingerprint.map(|fingerprint| (path, fingerprint)))
        })
        .collect::<Vec<_>>()
        .await;
    Ok(handle_iter!(results.into_iter(), ReadOptionalFingerprintFailed)
        .into_iter()
        .flatten()
        .collect())
}

#[derive(Error, Debug)]
pub enum CargoConfigFingerprintsError {
    #[error("failed to fingerprint Cargo configuration")]
    ReadOptionalFingerprintFailed { source: ErrVec<ReadOptionalFingerprintError> },
}
