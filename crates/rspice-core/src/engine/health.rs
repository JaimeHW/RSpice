//! Deterministic engine readiness probe.

use std::time::{Duration, Instant};

use super::{Engine, SimulationError};
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::netlist::{Netlist, NetlistParseOptions, ParseError, ParseWithAbortError};

const HEALTH_NETLIST: &str =
    "RSpice readiness probe\nVHEALTH out 0 1\nRHEALTH out 0 1k\n.OP\n.END\n";
const EXPECTED_OUTPUT_VOLTAGE: Value = 1.0;
const OUTPUT_TOLERANCE: Value = 1.0e-12;

/// Successful parser-to-solver readiness probe measurements.
#[derive(Debug, Clone, PartialEq)]
pub struct EngineHealthReport {
    /// End-to-end probe latency.
    pub elapsed: Duration,
    /// Elements accepted by the parser.
    pub element_count: usize,
    /// Non-ground nodes returned by the solver.
    pub node_count: usize,
    /// Branch unknowns returned by the solver.
    pub branch_count: usize,
    /// Voltage observed at the probe's output node.
    pub output_voltage: Value,
}

impl Engine {
    /// Exercise configuration validation, bounded parsing, circuit
    /// construction, matrix assembly, and a deterministic linear DC solve.
    ///
    /// This is a readiness probe, not a comprehensive numerical test suite. It
    /// performs no filesystem or network I/O and honors this engine's resource
    /// limits. Services can call it during startup or from a readiness endpoint.
    pub fn health_check(&self) -> Result<EngineHealthReport, SimulationError> {
        self.health_check_with_abort(&NoAbort)
    }

    /// Run the readiness probe with cooperative cancellation.
    pub fn health_check_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<EngineHealthReport, SimulationError> {
        let started = Instant::now();
        self.ensure_valid_configuration()?;
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let parse_options = NetlistParseOptions {
            resource_limits: self.config.resource_limits,
            ..NetlistParseOptions::default()
        };
        let netlist =
            Netlist::parse_validated_with_options_and_abort(HEALTH_NETLIST, parse_options, abort)
                .map_err(map_health_parse_error)?;
        let element_count = netlist.elements.len();
        let result = self.run_dc_op_with_abort(&netlist, abort)?;
        let output_voltage = result.try_voltage_named("out").ok_or_else(|| {
            SimulationError::Circuit(
                "readiness probe result did not contain its output node".to_string(),
            )
        })?;
        let error = (output_voltage - EXPECTED_OUTPUT_VOLTAGE).abs();
        if !output_voltage.is_finite() || error > OUTPUT_TOLERANCE {
            return Err(SimulationError::Circuit(format!(
                "readiness probe expected V(out)={EXPECTED_OUTPUT_VOLTAGE}, observed {output_voltage}"
            )));
        }

        Ok(EngineHealthReport {
            elapsed: started.elapsed(),
            element_count,
            node_count: result.node_voltages.len().saturating_sub(1),
            branch_count: result.branch_currents.len(),
            output_voltage,
        })
    }
}

fn map_health_parse_error(error: ParseWithAbortError) -> SimulationError {
    match error {
        ParseWithAbortError::Aborted => SimulationError::Aborted,
        ParseWithAbortError::Parse(ParseError::ResourceLimit(limit)) => limit.into(),
        ParseWithAbortError::Parse(error) => {
            SimulationError::Netlist(format!("readiness probe could not be parsed: {error}"))
        }
    }
}
