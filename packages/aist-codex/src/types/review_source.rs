use crate::Fingerprint;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ReviewSource {
    pub fingerprint: Fingerprint,
    pub lines: usize,
}

impl From<&[u8]> for ReviewSource {
    fn from(bytes: &[u8]) -> Self {
        Self {
            fingerprint: Fingerprint::from(bytes),
            lines: bytes.split_inclusive(|byte| *byte == b'\n').count(),
        }
    }
}
