use errgonomic::handle;
use serde::Serialize;
use serde_json::{Error as JsonError, Value, to_value, to_vec};
use thiserror::Error;

pub fn canonical_json(value: &impl Serialize) -> Result<Vec<u8>, CanonicalJsonError> {
    use CanonicalJsonError::*;
    let mut value = handle!(to_value(value), ToValueFailed);
    value.sort_all_objects();
    Ok(handle!(to_vec(&value), ToVecFailed, value))
}

#[derive(Error, Debug)]
pub enum CanonicalJsonError {
    #[error("failed to serialize review input")]
    ToValueFailed { source: JsonError },
    #[error("failed to encode review input")]
    ToVecFailed { source: JsonError, value: Value },
}
