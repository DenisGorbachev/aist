use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct CargoReviewOptions {
    pub target: Option<String>,
    pub features: BTreeSet<String>,
    pub all_features: bool,
    pub no_default_features: bool,
    pub extra_inputs: BTreeMap<PathBuf, PathBuf>,
    pub extra_context: BTreeMap<String, String>,
}

impl CargoReviewOptions {
    pub fn metadata_args(&self) -> Vec<OsString> {
        let mut args = vec![
            "metadata".into(),
            "--locked".into(),
            "--format-version".into(),
            "1".into(),
        ];
        if self.all_features {
            args.push("--all-features".into());
        }
        if self.no_default_features {
            args.push("--no-default-features".into());
        }
        if let Some(target) = &self.target {
            args.extend(["--filter-platform".into(), target.into()]);
        }
        self.features
            .iter()
            .for_each(|feature| args.extend(["--features".into(), feature.into()]));
        args
    }
}
