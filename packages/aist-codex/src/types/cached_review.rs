use crate::{Fingerprint, ReviewVerdict};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CachedReview {
    pub key: Fingerprint,
    pub cached: bool,
    pub verdict: ReviewVerdict,
}
