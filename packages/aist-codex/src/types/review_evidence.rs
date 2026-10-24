use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReviewEvidence {
    pub path: PathBuf,
    pub line: NonZeroUsize,
    pub explanation: String,
}
