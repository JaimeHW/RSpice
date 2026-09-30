//! Abort-aware deck parsing with exact resource limits and typed failures.

use crate::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted};
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
