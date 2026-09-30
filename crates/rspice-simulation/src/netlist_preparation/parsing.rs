//! Deck parsing and executable hierarchy checks with typed preparation failures.

use crate::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted};
use crate::preparation::{PreparationError, PreparationStage};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::StatisticalParamMode;
use std::path::Path;

pub fn parse_runner_netlist_with_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    parse_runner_netlist_with_resource_limits_and_abort(
        netlist_text,
        source_path,
        rspice_core::ResourceLimits::default(),
        abort,
    )
}

pub fn parse_runner_netlist_with_resource_limits_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    resource_limits: rspice_core::ResourceLimits,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    parse_runner_netlist_with_options_and_abort(
        netlist_text,
        source_path,
        rspice_core::netlist::NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            statistical_seed: None,
            resource_limits,
            ..Default::default()
        },
        abort,
    )
}

pub fn parse_runner_netlist_with_options_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    options: rspice_core::netlist::NetlistParseOptions,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    ensure_not_aborted(abort)?;
    let parsed = match source_path {
        Some(path) => rspice_core::Netlist::parse_with_path_and_options_and_abort(
            netlist_text,
            path,
            options,
            abort,
        ),
        None => rspice_core::Netlist::parse_with_options_and_abort(netlist_text, options, abort),
    }
    .map_err(|error| match error {
        rspice_core::netlist::ParseWithAbortError::Aborted => ServiceRunError::Aborted,
        rspice_core::netlist::ParseWithAbortError::Parse(
            rspice_core::netlist::ParseError::ResourceLimit(error),
        ) => ServiceRunError::ResourceLimit(error),
        rspice_core::netlist::ParseWithAbortError::Parse(error) => {
            ServiceRunError::Failure(format!("Parse error: {error}"))
        }
    });
    ensure_not_aborted(abort)?;
    parsed
}

/// Resolve the same waveform grammar and instance parameters the engine uses
/// before any dispatch proof is built. File-backed waveforms require sealing;
/// their node or parameter names cannot identify a dependency by themselves.
pub fn validated_executable_hierarchy(
    executable_netlist: &str,
) -> Result<(rspice_core::Netlist, rspice_core::netlist::FlattenedNetlist), PreparationError> {
    let parsed = rspice_core::netlist::parse_netlist(executable_netlist).map_err(|error| {
        PreparationError::new(
            PreparationStage::ModelBindings,
            format!("Executable source cannot authenticate project model use: {error}"),
        )
    })?;
    let flattened =
        rspice_core::netlist::flatten_netlist_with_models(&parsed).map_err(|error| {
            PreparationError::new(
                PreparationStage::ModelBindings,
                format!("Executable hierarchy cannot authenticate project model use: {error}"),
            )
        })?;
    for element in &flattened.elements {
        let dependency = rspice_core::netlist::independent_source_file_dependency(&element.kind)
            .map_err(|error| {
                PreparationError::new(PreparationStage::SourceChecks, error.to_string())
            })?;
        if let Some(path) = dependency {
            return Err(PreparationError::new(
                PreparationStage::SourceChecks,
                format!(
                    "Executable netlist contains an unsealed external dependency (file-backed PWL source) on element '{}': {path}",
                    element.name
                ),
            ));
        }
    }
    Ok((parsed, flattened))
}
