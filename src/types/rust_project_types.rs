use crate::RustFileTypes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct RustProjectTypes {
    pub files: Vec<RustFileTypes>,
}
