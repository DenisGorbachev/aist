use crate::{Fingerprint, ReviewVerdict};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CacheEntry {
    pub key: Fingerprint,
    pub created: SystemTime,
    pub verdict: ReviewVerdict,
}
