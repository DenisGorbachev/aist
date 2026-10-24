use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(ValueEnum, Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Debug)]
#[value(rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum ReviewStatus {
    Satisfied,
    Violated,
    Inconclusive,
}
