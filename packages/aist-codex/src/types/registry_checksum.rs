use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct RegistryChecksum {
    pub files: BTreeMap<PathBuf, String>,
}
