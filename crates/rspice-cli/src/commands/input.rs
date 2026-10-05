//! One input-resolution contract for run, check and info.

use crate::cli::{CliError, Config, NetlistOptions};
use rspice_core::netlist::{NetlistParseOptions, ParseWithAbortError};
use rspice_core::{Netlist, ResourceLimits};
use std::path::Path;

pub(crate) fn parse_options(
    args: &NetlistOptions,
    limits: ResourceLimits,
    retain_control_script: bool,
) -> NetlistParseOptions {
    let mut options = NetlistParseOptions {
        resource_limits: limits,
        retain_control_script,
        ..Default::default()
    };
    if let Some(mode) = args.redefined_params {
        (
            options.parameter_redefinition_policy,
            options.parameter_redefinition_diagnostic_policy,
        ) = mode.parse_policies();
    }
    if let Some(dialect) = args.spice_dialect {
        options.expression_dialect = dialect.expression_dialect();
    }
    options
}

pub(crate) fn map_error(error: ParseWithAbortError, path: &Path, timeout: Option<f64>) -> CliError {
    match error {
        ParseWithAbortError::Parse(rspice_core::netlist::ParseError::Io(source)) => {
            if source.kind() == std::io::ErrorKind::NotFound {
                CliError::InputNotFound {
                    path: path.to_path_buf(),
                    source,
                }
            } else {
                CliError::InputReadError {
                    path: path.to_path_buf(),
                    source,
                }
            }
        }
        ParseWithAbortError::Aborted => super::run::cancellation_cli_error(timeout),
        ParseWithAbortError::Parse(error) => super::map_parse_error(error),
    }
}

pub(crate) fn parse_source(
    source: &str,
    path: &Path,
    args: &NetlistOptions,
    config: &Config,
    options: NetlistParseOptions,
    timeout: Option<f64>,
) -> Result<Netlist, CliError> {
    let mut paths = args.includes.clone();
    paths.extend(config.paths.include_paths.iter().cloned());
    paths.extend(config.paths.library_paths.iter().cloned());
    let overrides = args
        .defines
        .iter()
        .map(|definition| parse_define(definition))
        .collect::<Result<Vec<_>, _>>()?;
    Netlist::parse_with_root_parameter_overrides_and_abort(
        source,
        path,
        &paths,
        options,
        &overrides,
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| map_error(error, path, timeout))
}

fn parse_define(definition: &str) -> Result<(String, f64), CliError> {
    let invalid = |message| CliError::InvalidArgument {
        message,
        suggestion: Some("use -D NAME=VALUE, e.g. -D RLOAD=4.7k".to_string()),
    };
    let (name, value) = definition
        .split_once('=')
        .ok_or_else(|| invalid(format!("malformed --define '{definition}'")))?;
    let name = name.trim();
    if name.is_empty() {
        return Err(invalid(format!(
            "missing parameter name in --define '{definition}'"
        )));
    }
    let value = rspice_core::netlist::lexer::parse_spice_value(value.trim())
        .map_err(|error| invalid(format!("invalid value in --define '{definition}': {error}")))?;
    if !value.is_finite() {
        return Err(invalid(format!(
            "--define '{definition}' must have a finite value"
        )));
    }
    Ok((name.to_string(), value))
}
