use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{self, Display, Formatter};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Fingerprint(pub [u8; 32]);

impl From<&[u8]> for Fingerprint {
    fn from(bytes: &[u8]) -> Self {
        Self(Sha256::digest(bytes).into())
    }
}

impl Display for Fingerprint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .try_for_each(|byte| write!(formatter, "{byte:02x}"))
    }
}
