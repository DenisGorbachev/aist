use crate::{CanonicalJsonError, Fingerprint, canonical_json};
use errgonomic::handle;
use serde::Serialize;
use thiserror::Error;

pub fn hash_json(value: &impl Serialize) -> Result<Fingerprint, HashJsonError> {
    use HashJsonError::*;
    let bytes = handle!(canonical_json(value), CanonicalJsonFailed);
    Ok(Fingerprint::from(bytes.as_slice()))
}

#[derive(Error, Debug)]
pub enum HashJsonError {
    #[error("failed to encode cache key input")]
    CanonicalJsonFailed { source: CanonicalJsonError },
}
