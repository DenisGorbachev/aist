use clap::ValueEnum;
use core::fmt::{self, Display, Formatter};
use ra_ap_hir::{Adt, ModuleDef};
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum RustTypeKind {
    Enum,
    Struct,
    TypeAlias,
    Union,
}

impl Display for RustTypeKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        use RustTypeKind::*;
        let value = match self {
            Enum => "enum",
            Struct => "struct",
            TypeAlias => "type alias",
            Union => "union",
        };
        formatter.write_str(value)
    }
}

impl RustTypeKind {
    /// PRUNING: returns `None` for semantic definitions that do not declare a Rust data type or type alias.
    pub fn from_module_def(module_def: ModuleDef) -> Option<Self> {
        use ModuleDef::*;
        match module_def {
            Adt(adt) => Some(Self::from(adt)),
            TypeAlias(_) => Some(Self::TypeAlias),
            _ => None,
        }
    }
}

impl From<Adt> for RustTypeKind {
    fn from(value: Adt) -> Self {
        use Adt::*;
        match value {
            Enum(_) => Self::Enum,
            Struct(_) => Self::Struct,
            Union(_) => Self::Union,
        }
    }
}
