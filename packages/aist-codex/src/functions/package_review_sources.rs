use crate::{CommandOutputError, Fingerprint, RegistryChecksum, command_output};
use cargo_metadata::Package;
use errgonomic::{handle, handle_opt};
use serde_json::{Error as JsonError, from_slice};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;
use std::string::FromUtf8Error;
use std::time::Duration;
use thiserror::Error;
use tokio::fs::read;
use tokio::process::Command;

pub async fn package_review_sources(package: &Package, duration: Duration) -> Result<BTreeMap<PathBuf, PathBuf>, PackageReviewSourcesError> {
    use PackageReviewSourcesError::*;
    let manifest = package.manifest_path.as_std_path();
    let directory = handle_opt!(manifest.parent(), ParentNotFound, manifest: manifest.to_path_buf());
    let paths = if package
        .source
        .as_ref()
        .is_some_and(|source| source.repr.starts_with("registry+") || source.repr.starts_with("sparse+"))
    {
        let path = directory.join(".cargo-checksum.json");
        let bytes = handle!(read(&path).await, ReadFailed, path: path.to_owned());
        let checksum: RegistryChecksum = handle!(from_slice(&bytes), FromSliceFailed, path, bytes);
        checksum.files.into_keys().collect::<Vec<_>>()
    } else {
        let args: Vec<OsString> = [
            "package",
            "--list",
            "--allow-dirty",
            "--locked",
            "--manifest-path",
        ]
        .into_iter()
        .map(OsString::from)
        .chain([manifest.into()])
        .collect();
        let mut command = Command::new("cargo");
        command.args(&args).current_dir(directory);
        let bytes = handle!(command_output(command, duration).await, CommandOutputFailed, manifest: manifest.to_path_buf());
        let list = handle!(String::from_utf8(bytes), FromUtf8Failed, manifest: manifest.to_path_buf());
        list.lines()
            .filter(|name| !matches!(*name, ".cargo_vcs_info.json" | "Cargo.lock" | "Cargo.toml.orig"))
            .map(PathBuf::from)
            .collect()
    };
    let id = Fingerprint::from(package.id.repr.as_bytes());
    let prefix = PathBuf::from("packages").join(format!("{}-{id}", package.name));
    Ok(paths
        .into_iter()
        .filter(|name| !matches!(name.to_str(), Some("mise.toml" | "mise.lock")) && !name.starts_with(".mise"))
        .map(|name| (prefix.join(&name), directory.join(name)))
        .collect())
}

#[derive(Error, Debug)]
pub enum PackageReviewSourcesError {
    #[error("failed to read registry source inventory '{path}'", path = path.display())]
    ReadFailed { source: io::Error, path: PathBuf },
    #[error("invalid registry source inventory '{path}'", path = path.display())]
    FromSliceFailed { source: JsonError, path: PathBuf, bytes: Vec<u8> },
    #[error("package manifest has no parent: '{manifest}'", manifest = manifest.display())]
    ParentNotFound { manifest: PathBuf },
    #[error("failed to enumerate packaged sources for '{manifest}'", manifest = manifest.display())]
    CommandOutputFailed { source: Box<CommandOutputError>, manifest: PathBuf },
    #[error("package source paths are not UTF-8 for '{manifest}'", manifest = manifest.display())]
    FromUtf8Failed { source: FromUtf8Error, manifest: PathBuf },
}
