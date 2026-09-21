//! Netlist execution and retained independent-phase operating points.
mod bindings;
mod card;
mod frequency_sweep;
#[cfg(test)]
mod integral_tests;
mod noise_sources;
mod qpac;
mod qpnoise;
mod qpxf;
mod sources;
mod state;
#[cfg(test)]
mod tests;

use super::*;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicError, QuasiPeriodicGrid, QuasiPeriodicGridConfig, QuasiPeriodicSolution,
    QuasiPeriodicSolveConfig,
};
pub use qpac::{QpacAnalysisResult, QpacInputQuantity, QpacRequest, QpacResultMetadata};
pub use qpnoise::{
    QpnoiseAnalysisResult, QpnoiseContributorRank, QpnoiseFrequencyAxis, QpnoiseInput,
    QpnoiseIntegrated, QpnoiseIntegration, QpnoiseIntegrationMethod, QpnoiseLattices,
    QpnoiseNoiseFigure, QpnoiseObservation, QpnoiseOutput, QpnoiseOutputSpectrum, QpnoiseQuantity,
    QpnoiseReference, QpnoiseRequest, QpnoiseResultMetadata, QpnoiseSourceLayout, QpnoiseSources,
    QpnoiseSpectrumLayout, QpnoiseTransferMetadata, QpnoiseUnavailable, QpnoiseValue,
};
pub use qpxf::{
    QpxfAnalysisResult, QpxfFrequencyAxis, QpxfGroupDelay, QpxfInputLattices, QpxfInputSource,
    QpxfOutput, QpxfQuantity, QpxfRequest, QpxfResultMetadata, QpxfSources, QpxfTransfer,
};
pub use state::{QpssOperatingPoint, QpssOperatingPointMetadata};
use std::sync::Arc;

/// Explicit assignment of an AC-only source to an independent tone. A
/// waveform with authored clocks obtains its tuple from those frequencies.
/// Multiple entries can deliberately drive several tones from one AC source.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpssSourceTone {
    pub source: String,
    pub tone: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QpssInitialState {
    Zero,
    DcOperatingPoint,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpssConfig {
    pub grid: QuasiPeriodicGridConfig,
    pub solver: QuasiPeriodicSolveConfig,
    pub source_tones: Vec<QpssSourceTone>,
    pub initial_state: QpssInitialState,
}

impl QpssConfig {
    pub fn new(frequencies_hz: Vec<Value>, harmonics: Vec<usize>) -> Self {
        Self {
            grid: QuasiPeriodicGridConfig::new(frequencies_hz, harmonics),
            solver: QuasiPeriodicSolveConfig::default(),
            source_tones: Vec::new(),
            initial_state: QpssInitialState::Zero,
        }
    }
}

fn numerical_error(error: QuasiPeriodicError) -> SimulationError {
    match error {
        QuasiPeriodicError::Aborted => SimulationError::Aborted,
        QuasiPeriodicError::ResourceLimit(error) => SimulationError::ResourceLimit(error),
        QuasiPeriodicError::ConvergenceFailed { iterations, .. } => {
            SimulationError::ConvergenceFailed(iterations)
        }
        other => SimulationError::Circuit(other.to_string()),
    }
}

fn invalid(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("QPSS: {}", message.into()))
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

impl Engine {
    /// Solve a driven quasiperiodic circuit with independent source phases.
    /// The returned state retains signed tone tuples, never a fabricated
    /// common fundamental. Authored transient clocks take precedence over AC
    /// annotations. An AC-only large-signal drive requires an explicit tone
    /// assignment so an AC probe is not silently applied to every clock.
    pub fn run_qpss(
        &self,
        netlist: &Netlist,
        config: QpssConfig,
    ) -> Result<QpssOperatingPoint, SimulationError> {
        self.run_qpss_with_abort(netlist, config, &NoAbort)
    }

    pub fn run_qpss_with_abort(
        &self,
        netlist: &Netlist,
        config: QpssConfig,
        abort: &dyn AbortSignal,
    ) -> Result<QpssOperatingPoint, SimulationError> {
        self.run_qpss_with_optional_dc_seed_and_abort(netlist, config, None, abort)
    }

    /// Initialize QPSS from the caller's exact configured OP, without solving
    /// another default OP. The caller must reproduce the OP's physical
    /// temperature and supply environment in this netlist/engine.
    pub fn run_qpss_with_dc_seed_and_abort(
        &self,
        netlist: &Netlist,
        config: QpssConfig,
        seed: &crate::engine::PeriodicDcOperatingPointSeed,
        abort: &dyn AbortSignal,
    ) -> Result<QpssOperatingPoint, SimulationError> {
        check_abort(abort)?;
        if config.initial_state != QpssInitialState::DcOperatingPoint {
            return Err(invalid(
                "an explicit DC seed requires DC operating-point initialization",
            ));
        }
        self.run_qpss_with_optional_dc_seed_and_abort(netlist, config, Some(seed), abort)
    }

    fn run_qpss_with_optional_dc_seed_and_abort(
        &self,
        netlist: &Netlist,
        config: QpssConfig,
        dc_seed: Option<&crate::engine::PeriodicDcOperatingPointSeed>,
        abort: &dyn AbortSignal,
    ) -> Result<QpssOperatingPoint, SimulationError> {
        check_abort(abort)?;
        let engine = self.resolved_for_netlist(netlist);
        config.validate_configuration()?;
        let grid = Arc::new(
            QuasiPeriodicGrid::new_with_abort(
                config.grid.clone(),
                &engine.config.resource_limits,
                abort,
            )
            .map_err(numerical_error)?,
        );
        let producer = state::Producer::capture(netlist, &engine.config, &config)?;
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        if let Some(seed) = dc_seed {
            seed.validate_for_circuit(&circuit)?;
        }
        Self::ensure_no_mixed_signal_analysis(&circuit, "quasiperiodic steady-state analysis")?;
        let required_unknowns = circuit
            .num_nodes()
            .saturating_add(circuit.num_branches())
            .saturating_add(Self::hb_periodic_extra_branch_count(&circuit)?)
            .saturating_add(circuit.behavioral_sources.integral_count());
        crate::analysis::quasi_periodic::solve::check_workload(
            required_unknowns,
            &grid,
            &config.solver.linear,
            &engine.config.resource_limits,
        )
        .map_err(numerical_error)?;
        let mut solver = engine.qpss_circuit_solver(&circuit, &grid)?;
        let node_names = engine.hb_build_node_names(&circuit, circuit.num_nodes());
        let mut branch_names = solver
            .try_periodic_mna_branch_names()
            .map_err(|error| invalid(error.to_string()))?;
        let unknowns = node_names.len().saturating_add(branch_names.len());
        let integral_names = branch_names.split_off(solver.physical_branch_count());
        engine.ensure_matrix_unknowns(unknowns.saturating_mul(grid.len()))?;
        engine.ensure_result_values(unknowns.saturating_mul(grid.len()).saturating_mul(6))?;
        let sources = sources::build(&engine, &circuit, &config, grid.clone(), abort)?;
        let mut seed = match config.initial_state {
            QpssInitialState::Zero => None,
            QpssInitialState::DcOperatingPoint => Some(engine.qpss_dc_seed(
                netlist,
                &grid,
                &node_names,
                &branch_names,
                circuit.num_branches(),
                dc_seed,
                abort,
            )?),
        };
        if let Some(seed) = &mut seed {
            // An OP contains no integral history. Zero is only the initial
            // guess for these coordinates; the full torus equations decide it.
            seed.resize(unknowns, vec![Complex64::ZERO; grid.len()]);
        }
        let solution = solver
            .solve_quasi_periodic_with_abort(
                grid,
                &config.solver,
                &sources,
                seed.as_deref(),
                &engine.config.resource_limits,
                abort,
            )
            .map_err(numerical_error)?;
        check_abort(abort)?;
        if producer != state::Producer::capture(netlist, &engine.config, &config)? {
            return Err(invalid("semantic producer inputs changed during the solve"));
        }
        QpssOperatingPoint::bind(
            producer,
            config,
            node_names,
            branch_names,
            integral_names,
            solution,
        )
    }

    fn qpss_circuit_solver(
        &self,
        circuit: &CircuitData,
        grid: &QuasiPeriodicGrid,
    ) -> Result<HbSolver, SimulationError> {
        for gaps in [
            periodic_capability::periodic_residual_gaps(circuit),
            periodic_capability::periodic_descriptor_gaps(circuit),
        ] {
            if let Some(summary) = periodic_capability::summarize(&gaps) {
                return Err(SimulationError::unsupported_capability(
                    "analysis.qpss.device",
                    format!("QPSS exact F/Q and MNA are unavailable for {summary}"),
                ));
            }
        }
        if circuit.num_nodes() == 0 {
            return Err(invalid("circuit has no nodes"));
        }
        // This object is only a physical device/MNA registry here. Its HB
        // basis and source arrays never enter the independent-phase solver.
        let mut solver =
            self.new_hb_solver(HbConfig::new(1.0).with_harmonics(1), circuit.num_nodes())?;
        self.hb_stamp_resistors(circuit, &mut solver);
        self.hb_stamp_capacitors(circuit, &mut solver);
        self.hb_stamp_periodic_mna_branches(circuit, &mut solver)?;
        self.hb_stamp_supported_nonlinear_devices(
            circuit,
            &mut solver,
            circuit.num_nodes(),
            BehavioralBasis::QuasiPeriodic(grid),
        )?;
        Ok(solver)
    }

    fn qpss_dc_seed(
        &self,
        netlist: &Netlist,
        grid: &QuasiPeriodicGrid,
        nodes: &[String],
        branches: &[String],
        canonical_branches: usize,
        dc_seed: Option<&crate::engine::PeriodicDcOperatingPointSeed>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Vec<Complex64>>, SimulationError> {
        let internal_dc;
        let (dc_nodes, voltages, dc_branches, currents) = if let Some(seed) = dc_seed {
            let (voltages, currents) = seed.solution().split_at(seed.node_names().len());
            (seed.node_names(), voltages, seed.branch_names(), currents)
        } else {
            internal_dc = self.run_dc_op_with_abort(netlist, abort)?;
            (
                internal_dc.node_names.as_slice(),
                internal_dc.node_voltages.as_slice(),
                internal_dc.branch_names.as_slice(),
                internal_dc.branch_currents.as_slice(),
            )
        };
        let mut seed = vec![vec![Complex64::ZERO; grid.len()]; nodes.len() + branches.len()];
        for (names, available, values, offset) in [
            (nodes, dc_nodes, voltages, 0),
            (branches, dc_branches, currents, nodes.len()),
        ] {
            for (row, name) in names.iter().enumerate() {
                check_abort(abort)?;
                let Some(index) = available
                    .iter()
                    .position(|candidate| candidate.eq_ignore_ascii_case(name))
                else {
                    if offset > 0 && row >= canonical_branches {
                        // Distributed-wave auxiliary currents need no DC
                        // history. Zero is an explicit initial guess only;
                        // the full QP equations must still converge.
                        continue;
                    }
                    return Err(invalid(format!(
                        "DC initial state has no coordinate '{name}'"
                    )));
                };
                let value = *values
                    .get(index)
                    .ok_or_else(|| invalid("DC initial-state values are incomplete"))?;
                if !value.is_finite() {
                    return Err(invalid("DC initial state is non-finite"));
                }
                seed[offset + row][grid.dc_index()] = Complex64::new(value, 0.0);
            }
        }
        Ok(seed)
    }
}
