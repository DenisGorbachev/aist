use cargo_metadata::{
    CargoOpt, Error as MetadataError, Metadata, MetadataCommand,
    semver::{self, Version},
};
use errgonomic::{handle, handle_bool, handle_opt};
use ra_ap_project_model::ManifestPath;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::tempdir;
use thiserror::Error;

pub fn read_cargo_metadata(manifest: &ManifestPath) -> Result<Metadata, ReadCargoMetadataError> {
    use ReadCargoMetadataError::*;
    let mut cmd = MetadataCommand::new();
    cmd.manifest_path(manifest.as_str())
        .current_dir(manifest.parent().as_str())
        .features(CargoOpt::AllFeatures);
    let locked_error = match cmd
        .clone()
        .other_options(vec!["--locked".to_owned()])
        .exec()
    {
        Ok(metadata) => return Ok(metadata),
        Err(error) => error,
    };
    // Stable Cargo reports its effective lockfile path only in the --locked diagnostic.
    let original = match &locked_error {
        MetadataError::CargoMetadata {
            stderr,
        } => stderr.lines().find_map(|line| {
            let path = line
                .strip_prefix("error: cannot update the lock file ")
                .or_else(|| line.strip_prefix("error: cannot create the lock file "))?
                .strip_suffix(" because --locked was passed to prevent this")?;
            let path = Path::new(path);
            (path.is_absolute() && path.ends_with("Cargo.lock")).then_some(path.to_owned())
        }),
        _ => None,
    };
    let original = handle_opt!(original, LockedMetadataFailed, source: locked_error);
    let output = handle!(
        Command::new(cmd.cargo_command().get_program())
            .current_dir(manifest.parent().as_str())
            .arg("--version")
            .output(),
        CargoVersionFailed
    );
    let version = String::from_utf8_lossy(&output.stdout);
    let version = handle_opt!(version.split_whitespace().nth(1), VersionNotFound, output);
    let version = handle!(version.parse::<Version>(), ParseFailed, version: version.to_owned());
    handle_bool!(!output.status.success() || version < Version::new(1, 97, 0), LockfileOverrideUnavailable, source: locked_error, version);
    let dir = handle!(tempdir(), TempdirFailed);
    let lockfile = dir.path().join("Cargo.lock");
    match fs::read(&original) {
        Ok(contents) => handle!(fs::write(&lockfile, contents), WriteFailed, lockfile),
        Err(source) if source.kind() == ErrorKind::NotFound => {}
        Err(source) => {
            return Err(ReadFailed {
                source,
                original,
            });
        }
    }
    Ok(handle!(cmd.env("CARGO_RESOLVER_LOCKFILE_PATH", &lockfile).exec(), ExecFailed, locked_error: Box::new(locked_error)))
}

#[derive(Debug, Error)]
pub enum ReadCargoMetadataError {
    #[error("failed to query Cargo version")]
    CargoVersionFailed { source: io::Error },
    #[error("Cargo version missing in {output:?}")]
    VersionNotFound { output: Output },
    #[error("invalid Cargo version '{version}'")]
    ParseFailed { source: semver::Error, version: String },
    #[error("failed to read locked Cargo metadata")]
    LockedMetadataFailed { source: MetadataError },
    #[error("Cargo {version} needs an up-to-date lockfile for read-only metadata")]
    LockfileOverrideUnavailable { source: MetadataError, version: Version },
    #[error("failed to create a temporary lockfile directory")]
    TempdirFailed { source: io::Error },
    #[error("failed to read lockfile '{}'", original.display())]
    ReadFailed { source: io::Error, original: PathBuf },
    #[error("failed to write temporary lockfile '{}'", lockfile.display())]
    WriteFailed { source: io::Error, lockfile: PathBuf },
    #[error("failed to resolve Cargo metadata using a temporary lockfile")]
    ExecFailed { source: MetadataError, locked_error: Box<MetadataError> },
}
