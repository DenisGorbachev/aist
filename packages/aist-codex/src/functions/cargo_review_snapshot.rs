use crate::{CargoConfigFingerprintsError, CargoReviewContext, CargoReviewOptions, CommandOutputError, PackageReviewSourcesError, ReviewPackagesError, ReviewSnapshot, ReviewSnapshotCaptureError, cargo_config_fingerprints, command_output, package_review_sources, review_packages};
use cargo_metadata::Metadata;
use errgonomic::{ErrVec, handle, handle_bool, handle_iter};
use futures::{StreamExt, stream};
use serde_json::{Error as JsonError, from_slice};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::string::FromUtf8Error;
use std::time::Duration;
use thiserror::Error;
use tokio::fs::canonicalize;

pub async fn cargo_review_snapshot(directory: &Path, package: &str, options: &CargoReviewOptions, duration: Duration) -> Result<ReviewSnapshot, CargoReviewSnapshotError> {
    use CargoReviewSnapshotError::*;
    let directory = handle!(canonicalize(directory).await, CanonicalizeFailed, path: directory.to_path_buf());
    let metadata_bytes = handle!(command_output(OsStr::new("cargo"), &options.metadata_args(), &directory, duration).await, MetadataCommandOutputFailed);
    let metadata: Metadata = handle!(from_slice(&metadata_bytes), FromSliceFailed, bytes: metadata_bytes);
    let packages = handle!(review_packages(&metadata, package), ReviewPackagesFailed);
    let results = stream::iter(&packages)
        .then(|package| package_review_sources(package, duration))
        .collect::<Vec<_>>()
        .await;
    let mut sources = handle_iter!(results.into_iter(), PackageReviewSourcesFailed)
        .into_iter()
        .flatten()
        .collect::<BTreeMap<_, _>>();
    handle_bool!(options.extra_inputs.keys().any(|name| sources.contains_key(name)), ExtraInputsInvalid, paths: options.extra_inputs.keys().cloned().collect::<Vec<_>>());
    sources.extend(
        options
            .extra_inputs
            .iter()
            .map(|(name, path)| (name.clone(), path.clone())),
    );
    let rustc = handle!(command_output(OsStr::new("rustc"), &["-vV".into()], &directory, duration).await, RustcCommandOutputFailed);
    let cargo = handle!(command_output(OsStr::new("cargo"), &["-vV".into()], &directory, duration).await, CargoCommandOutputFailed);
    let configuration = handle!(cargo_config_fingerprints(&directory).await, CargoConfigFingerprintsFailed);
    let environment = [
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_BUILD_TARGET",
        "RUSTC_BOOTSTRAP",
        "RUSTC",
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
        "RUSTUP_TOOLCHAIN",
    ]
    .into_iter()
    .map(|name| (name, env::var_os(name)))
    .collect::<BTreeMap<_, _>>();
    let ids = packages
        .iter()
        .map(|package| &package.id)
        .collect::<BTreeSet<_>>();
    let nodes = metadata
        .resolve
        .iter()
        .flat_map(|resolve| &resolve.nodes)
        .filter(|node| ids.contains(&node.id))
        .map(|node| (&node.id, node))
        .collect::<BTreeMap<_, _>>();
    let rustc = handle!(String::from_utf8(rustc), RustcFromUtf8Failed);
    let cargo = handle!(String::from_utf8(cargo), CargoFromUtf8Failed);
    let context = CargoReviewContext {
        package,
        packages,
        resolve: nodes,
        options,
        rustc,
        cargo,
        configuration,
        environment,
    };
    Ok(handle!(ReviewSnapshot::capture(&sources, &context).await, CaptureFailed))
}

#[derive(Error, Debug)]
pub enum CargoReviewSnapshotError {
    #[error("failed to resolve review directory '{path}'", path = path.display())]
    CanonicalizeFailed { source: io::Error, path: PathBuf },
    #[error("failed to resolve the reviewed Cargo package")]
    MetadataCommandOutputFailed { source: Box<CommandOutputError> },
    #[error("failed to decode Cargo metadata")]
    FromSliceFailed { source: JsonError, bytes: Vec<u8> },
    #[error("failed to select the reviewed dependency closure")]
    ReviewPackagesFailed { source: ReviewPackagesError },
    #[error("failed to enumerate review sources")]
    PackageReviewSourcesFailed { source: ErrVec<PackageReviewSourcesError> },
    #[error("extra inputs overwrite packaged review sources")]
    ExtraInputsInvalid { paths: Vec<PathBuf> },
    #[error("failed to identify the Rust compiler")]
    RustcCommandOutputFailed { source: Box<CommandOutputError> },
    #[error("failed to identify Cargo")]
    CargoCommandOutputFailed { source: Box<CommandOutputError> },
    #[error("Rust compiler identity is not UTF-8")]
    RustcFromUtf8Failed { source: FromUtf8Error },
    #[error("Cargo identity is not UTF-8")]
    CargoFromUtf8Failed { source: FromUtf8Error },
    #[error("failed to fingerprint Cargo configuration")]
    CargoConfigFingerprintsFailed { source: CargoConfigFingerprintsError },
    #[error("failed to capture the review snapshot")]
    CaptureFailed { source: ReviewSnapshotCaptureError },
}
