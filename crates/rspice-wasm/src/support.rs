//! Shared preflight helpers used by every runner.

use rspice_core::{AbortSignal, Engine, Netlist, ResourceLimits, SimulationConfig};

use crate::DetailedWasmResult;
use crate::errors::WasmError;

pub(crate) fn parse_netlist_detailed(
    source: &str,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> DetailedWasmResult<Netlist> {
    Netlist::parse_validated_with_options_and_abort(
        source,
        rspice_core::netlist::NetlistParseOptions {
            resource_limits,
            ..rspice_core::netlist::NetlistParseOptions::default()
        },
        abort,
    )
    .map_err(|error| match error {
        rspice_core::netlist::ParseWithAbortError::Aborted => Box::new(
            WasmError::from_simulation_error(rspice_core::engine::SimulationError::Aborted),
        ),
        rspice_core::netlist::ParseWithAbortError::Parse(error) => {
            Box::new(WasmError::from_parse_error(error))
        }
    })
}

pub(crate) fn engine_with_resource_limits(
    resource_limits: ResourceLimits,
) -> DetailedWasmResult<Engine> {
    let config = SimulationConfig {
        resource_limits,
        ..SimulationConfig::default()
    };
    Engine::try_new(config).map_err(|error| {
        Box::new(WasmError::from_simulation_error(
            rspice_core::engine::SimulationError::Configuration(error),
        ))
    })
}
