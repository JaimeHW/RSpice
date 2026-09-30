//! Captured automatic export settings authenticated with prepared execution.

use super::{PreparationError, PreparationStage};
use crate::netlist_preparation::dependencies::execution_current_directory;
use rspice_app_types::canonical::{CanonicalWriter, content_digest};
use rspice_app_types::product::ContentDigest;
use rspice_simulation_contract::sp_draft::{SpDialogState, SpPlacedPort};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Immutable automatic Touchstone-export policy authenticated by preflight.
///
/// The output prefix is captured before execution so edits to the live
/// schematic path or S-parameter dialog cannot redirect a completed run. The
/// digest uses the platform-exact path identity; display conversion is never
/// used as persistence authority.
#[derive(Clone, PartialEq, Eq)]
pub struct TouchstoneExportPolicy(TouchstoneExport);

#[derive(Debug, Clone, PartialEq, Eq)]
enum TouchstoneExport {
    Disabled,
    Enabled {
        version: u32,
        output_directory: PathBuf,
        output_stem: OsString,
        output_identity: ContentDigest,
    },
}

impl std::fmt::Debug for TouchstoneExportPolicy {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, formatter)
    }
}

impl TouchstoneExportPolicy {
    pub const fn disabled() -> Self {
        Self(TouchstoneExport::Disabled)
    }

    pub fn enabled(
        version: u32,
        output_directory: PathBuf,
        output_stem: OsString,
    ) -> Result<Self, PreparationError> {
        if !(1..=2).contains(&version) {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Touchstone export version must be 1 or 2, got {version}"),
            ));
        }
        if output_directory.as_os_str().is_empty() || output_stem.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Touchstone export requires a non-empty captured output prefix",
            ));
        }
        let stem_path = Path::new(&output_stem);
        if stem_path.file_name() != Some(output_stem.as_os_str())
            || stem_path.components().count() != 1
        {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Touchstone export stem must be one path component",
            ));
        }
        let output_identity = exact_path_digest(&output_directory.join(&output_stem));
        Ok(Self(TouchstoneExport::Enabled {
            version,
            output_directory,
            output_stem,
            output_identity,
        }))
    }

    pub const fn version(&self) -> Option<u32> {
        match &self.0 {
            TouchstoneExport::Disabled => None,
            TouchstoneExport::Enabled { version, .. } => Some(*version),
        }
    }

    pub fn output_path(
        &self,
        run_id: u64,
        analysis_idx: usize,
        num_ports: usize,
    ) -> Option<PathBuf> {
        let TouchstoneExport::Enabled {
            output_directory,
            output_stem,
            ..
        } = &self.0
        else {
            return None;
        };
        let mut file_name = output_stem.clone();
        file_name.push(format!(
            "_run{run_id:04}_sp{:02}.s{}p",
            analysis_idx.max(1),
            num_ports.max(2)
        ));
        Some(output_directory.join(file_name))
    }

    pub fn encode(&self, writer: &mut CanonicalWriter) {
        writer.domain("touchstone-export-policy");
        match &self.0 {
            TouchstoneExport::Disabled => writer.u8(0),
            TouchstoneExport::Enabled {
                version,
                output_identity,
                ..
            } => {
                writer.u8(1);
                writer.u64(u64::from(*version));
                writer.digest(*output_identity);
            }
        }
    }
}

fn exact_path_digest(path: &Path) -> ContentDigest {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;

        let mut bytes = Vec::new();
        for unit in path.as_os_str().encode_wide() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        content_digest("rspice.touchstone-output-prefix/windows-utf16be/v1", &bytes)
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;

        content_digest(
            "rspice.touchstone-output-prefix/unix-bytes/v1",
            path.as_os_str().as_bytes(),
        )
    }
    #[cfg(not(any(windows, unix)))]
    {
        content_digest(
            "rspice.touchstone-output-prefix/utf8/v1",
            path.as_os_str().to_string_lossy().as_bytes(),
        )
    }
}

pub fn touchstone_export_policy_for_dialog(
    dialog: &SpDialogState,
    schematic: &impl AsRef<rspice_design::schematic::document::SchematicDocument>,
    source_path: Option<&Path>,
) -> Result<TouchstoneExportPolicy, PreparationError> {
    let ports = rspice_design::rf_ports::rf_ports(schematic);
    let placed = ports
        .iter()
        .map(|port| SpPlacedPort {
            reference: &port.reference,
            port_number: port.port_number,
            z0: &port.z0,
            nets: &port.nets,
        })
        .collect::<Vec<_>>();
    let mut dialog = dialog.clone();
    dialog.ensure_initialized();
    let config = dialog.to_config(Some(&placed)).map_err(|error| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!("Invalid Touchstone export settings: {error}"),
        )
    })?;
    if !config.touchstone_export {
        return Ok(TouchstoneExportPolicy::disabled());
    }

    let (directory, stem) = touchstone_output_prefix(source_path)?;
    TouchstoneExportPolicy::enabled(config.touchstone_version, directory, stem)
}

fn touchstone_output_prefix(
    source_path: Option<&Path>,
) -> Result<(PathBuf, OsString), PreparationError> {
    let current = execution_current_directory()?;
    let Some(source) = source_path else {
        return Ok((current, OsString::from("untitled")));
    };

    let absolute = if source.is_absolute() {
        source.to_path_buf()
    } else {
        current.join(source)
    };
    let directory = absolute
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| current.clone());
    let directory = directory.canonicalize().unwrap_or(directory);
    let stem = absolute
        .file_stem()
        .filter(|stem| !stem.is_empty())
        .map(OsString::from)
        .unwrap_or_else(|| OsString::from("untitled"));
    Ok((directory, stem))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automatic_export_identity_is_derived_from_the_exact_output_prefix() {
        let first = TouchstoneExportPolicy::enabled(
            2,
            PathBuf::from("sealed-output-a"),
            OsString::from("amp"),
        )
        .expect("first output policy");
        let second = TouchstoneExportPolicy::enabled(
            2,
            PathBuf::from("sealed-output-b"),
            OsString::from("amp"),
        )
        .expect("second output policy");
        assert_ne!(first, second);

        assert!(
            TouchstoneExportPolicy::enabled(
                2,
                PathBuf::from("sealed-output"),
                OsString::from("../redirect"),
            )
            .is_err(),
            "a stem must not be able to redirect the captured directory"
        );
    }
}
