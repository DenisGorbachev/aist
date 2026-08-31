use ra_ap_syntax::TextSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[serde(transparent)]
pub struct RustSourceByteOffset(pub u32);

impl From<TextSize> for RustSourceByteOffset {
    fn from(value: TextSize) -> Self {
        Self(u32::from(value))
    }
}
