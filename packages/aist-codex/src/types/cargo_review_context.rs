use crate::{CargoReviewOptions, Fingerprint};
use cargo_metadata::{Node, Package, PackageId};
use serde::Serialize;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct CargoReviewContext<'a> {
    pub package: &'a str,
    pub packages: Vec<&'a Package>,
    pub resolve: BTreeMap<&'a PackageId, &'a Node>,
    pub options: &'a CargoReviewOptions,
    pub rustc: String,
    pub cargo: String,
    pub configuration: BTreeMap<PathBuf, Fingerprint>,
    pub environment: BTreeMap<OsString, Fingerprint>,
}
