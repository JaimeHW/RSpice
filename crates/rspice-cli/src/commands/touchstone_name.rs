//! One filename contract for network publication and automatic import.

use crate::cli::CliError;
use rspice_core::analysis::s_param::TouchstoneVersion;
use std::path::Path;

enum Extension<'a> {
    Numbered(&'a str),
    Generic,
}

fn extension(path: &Path) -> Option<Extension<'_>> {
    let extension = path.extension()?.to_str()?;
    if extension.eq_ignore_ascii_case("snp") || extension.eq_ignore_ascii_case("ts") {
        return Some(Extension::Generic);
    }
    let ports = extension
        .strip_prefix(['s', 'S'])?
        .strip_suffix(['p', 'P'])?;
    (!ports.is_empty() && ports.bytes().all(|byte| byte.is_ascii_digit()))
        .then_some(Extension::Numbered(ports))
}

pub(super) fn is_touchstone(path: &Path) -> bool {
    extension(path).is_some()
}

pub(super) fn output_version(
    path: &Path,
    num_ports: usize,
) -> Result<Option<TouchstoneVersion>, CliError> {
    match extension(path) {
        None => Ok(None),
        Some(Extension::Generic) => Ok(Some(TouchstoneVersion::V2)),
        Some(Extension::Numbered(ports)) => {
            if ports.parse::<usize>().ok() == Some(num_ports) && num_ports > 0 {
                return Ok(Some(TouchstoneVersion::V1));
            }
            Err(CliError::InvalidArgument {
                message: format!(
                    "Touchstone output '{}' declares {ports} ports, but the result has {num_ports}",
                    path.display()
                ),
                suggestion: Some(format!("use an .s{num_ports}p extension for this network")),
            })
        }
    }
}
