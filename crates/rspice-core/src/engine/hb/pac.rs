//! Periodic AC (PAC) analysis: small-signal conversion analysis around a
//! harmonic-balance operating point.
//!
//! The large-signal periodic solution comes from the HB solver; the
//! small-signal stimulus is applied at `f = offset + m*f0` and the linearized
//! time-varying network maps it into responses at every sideband
//! `offset + k*f0`. See `harmonic_balance::solver::periodic_ac` for the
//! conversion-matrix formulation.

use super::*;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::harmonic_balance::{PeriodicAcExcitation, PeriodicSidebandWindow};
use crate::analysis::pac::{PacConfig, PacResult};
use crate::analysis::{HbConfig, HbError as AnalysisHbError, HbSolverState};

/// PAC analysis result with convergence info
#[derive(Debug)]
pub struct PacAnalysisResult {
    /// The PAC solution: per-node sideband spectra and the conversion matrix
    /// for the configured output node.
    pub result: PacResult,
    /// Large-signal fundamental frequency (Hz)
    pub fundamental_freq: Value,
    /// Whether the operating-point solve converged
    pub converged: bool,
}

enum PacOperatingPoint<'a> {
    Shooting(&'a super::super::PssOperatingPoint),
    HarmonicBalance(&'a HbOperatingPoint),
}

pub(super) struct PacInputPort {
    pub(super) node_injections: Vec<(usize, Complex64)>,
    pub(super) voltage_source_index: Option<usize>,
}

impl Engine {
    /// Run Periodic AC analysis.
    ///
    /// Solves the large-signal periodic operating point with harmonic
    /// balance, samples the periodically time-varying small-signal
    /// linearization, and solves the sideband-coupled system at every sweep
    /// offset. The per-node spectra answer "what does the circuit do to the
    /// input source's signal"; when an output node is configured, the full
    /// conversion matrix (every input sideband to every output sideband) is
    /// filled for that node.
    pub fn run_pac(
        &self,
        netlist: &Netlist,
        config: PacConfig,
    ) -> Result<PacAnalysisResult, SimulationError> {
        self.run_pac_with_abort(netlist, config, &NoAbort)
    }

    /// Run periodic AC with cooperative cancellation between operating-point
    /// iterations and frequency/sideband solves.
    pub fn run_pac_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pac_impl(netlist, config, None, abort)
    }

    /// Run periodic AC from an exact previously converged shooting-PSS
    /// operating point. The large-signal state is projected into the PAC
    /// spectral basis; it is never re-solved.
    pub fn run_pac_from_pss_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        operating_point: &super::super::PssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pac_impl(
            netlist,
            config,
            Some(PacOperatingPoint::Shooting(operating_point)),
            abort,
        )
    }

    /// Run periodic AC from an exact previously converged harmonic-balance
    /// operating point. The large-signal state is consumed directly and is
    /// never re-solved.
    pub fn run_pac_from_hb_with_abort(
        &self,
        netlist: &Netlist,
        config: PacConfig,
        operating_point: &HbOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        engine.run_pac_impl(
            netlist,
            config,
            Some(PacOperatingPoint::HarmonicBalance(operating_point)),
            abort,
        )
    }

    fn run_pac_impl(
        &self,
        netlist: &Netlist,
        mut config: PacConfig,
        operating_point: Option<PacOperatingPoint<'_>>,
        abort: &dyn AbortSignal,
    ) -> Result<PacAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let Some(operating_point) = &operating_point {
            config.fundamental_freq = match operating_point {
                PacOperatingPoint::Shooting(point) => point.analysis().result.frequency,
                PacOperatingPoint::HarmonicBalance(point) => point.config().fundamental_freq,
            };
        }
        if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
            return Err(SimulationError::Circuit(
                "PAC requires a positive fundamental frequency".to_string(),
            ));
        }
        config
            .validate()
            .map_err(|e| SimulationError::Circuit(format!("Invalid PAC config: {e}")))?;
        let frequency_count = config.frequency_point_count().map_err(|error| {
            SimulationError::Circuit(format!("Invalid PAC frequency sweep: {error}"))
        })?;
        self.ensure_analysis_points(frequency_count)?;
        let sideband_count = config.num_sidebands();
        self.ensure_analysis_points(sideband_count)?;
        let result_record_count = frequency_count.checked_mul(sideband_count).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "PAC result grid {frequency_count} frequencies x {sideband_count} sidebands overflows this platform"
            ))
        })?;
        self.ensure_analysis_points(result_record_count)?;

        // The operating point needs enough harmonics that every conversion
        // coupling G[k-m] over the sideband span exists, with headroom for
        // the drive itself. Compute in i64 so extreme public i32 bounds
        // cannot overflow before the resource policy rejects them.
        let span = usize::try_from(
            (i64::from(config.sideband_max) - i64::from(config.sideband_min)).unsigned_abs(),
        )
        .unwrap_or(usize::MAX);
        let extreme = usize::try_from(
            i64::from(config.sideband_min)
                .unsigned_abs()
                .max(i64::from(config.sideband_max).unsigned_abs()),
        )
        .unwrap_or(usize::MAX);
        let op_harmonics = span.max(extreme).max(8);
        self.ensure_analysis_points(op_harmonics.saturating_add(1))?;
        if let Some(operating_point) = &operating_point
            && op_harmonics
                > match operating_point {
                    PacOperatingPoint::Shooting(point) => point.spectral_harmonic_capacity(),
                    PacOperatingPoint::HarmonicBalance(point) => point.spectral_harmonic_capacity(),
                }
        {
            let capacity = match operating_point {
                PacOperatingPoint::Shooting(point) => point.spectral_harmonic_capacity(),
                PacOperatingPoint::HarmonicBalance(point) => point.spectral_harmonic_capacity(),
            };
            return Err(SimulationError::Circuit(format!(
                "PAC requires {op_harmonics} periodic harmonics for its sideband span, but the retained periodic state has capacity {}",
                capacity
            )));
        }

        let mut hb_config = match &operating_point {
            Some(PacOperatingPoint::HarmonicBalance(point)) => point.config().clone(),
            _ => HbConfig::new(config.fundamental_freq)
                .with_harmonics(op_harmonics)
                .with_oversample(4),
        };
        // PAC's tolerances govern the nonlinear periodic operating point.
        // The subsequent sideband systems use deterministic direct solves and
        // therefore have no iterative tolerance of their own.
        if !matches!(operating_point, Some(PacOperatingPoint::HarmonicBalance(_))) {
            hb_config.tolerance = config.reltol;
            hb_config.abstol = config.abstol;
        }
        let hb_config = self.hb_config_for_netlist(netlist, hb_config)?;
        self.hb_validate_config(&hb_config)?;
        if let Some(PacOperatingPoint::HarmonicBalance(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, &hb_config)?;
        }
        if let Some(PacOperatingPoint::Shooting(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, point.config())?;
        }

        let input_name = config
            .input_source
            .clone()
            .ok_or_else(|| SimulationError::Circuit("PAC requires an input source".to_string()))?;

        let circuit = self.build_circuit_with_abort(netlist, abort)?;
        let num_nodes = circuit.num_nodes();
        if num_nodes == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        let periodic_branches = circuit
            .num_branches()
            .checked_add(Self::hb_periodic_extra_branch_count(&circuit)?)
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "PAC canonical and distributed-network branch count overflows this platform"
                        .to_string(),
                )
            })?;
        let periodic_unknowns = num_nodes.checked_add(periodic_branches).ok_or_else(|| {
            SimulationError::Circuit(
                "PAC periodic node and branch count overflows this platform".to_string(),
            )
        })?;
        let lifted_unknowns = periodic_unknowns.checked_mul(sideband_count).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "PAC lifted dimension {periodic_unknowns} MNA unknowns x {sideband_count} sidebands overflows this platform"
            ))
        })?;
        self.ensure_matrix_unknowns(lifted_unknowns)?;
        let spectra_complex_values = result_record_count
            .checked_mul(periodic_unknowns)
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "PAC retained MNA grid {result_record_count} records x {periodic_unknowns} node/branch unknowns overflows this platform"
                ))
            })?;
        let conversion_values = if config.output_node.is_some() {
            frequency_count
                .checked_mul(sideband_count)
                .and_then(|value| value.checked_mul(sideband_count))
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "PAC conversion grid {frequency_count} x {sideband_count} x {sideband_count} overflows this platform"
                    ))
                })?
        } else {
            0
        };
        let retained_complex_values = spectra_complex_values
            .checked_add(conversion_values)
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "PAC retained complex-value count overflows usize".to_string(),
                )
            })?;
        let retained_scalar_values = retained_complex_values.checked_mul(2).ok_or_else(|| {
            SimulationError::Circuit("PAC retained scalar-value count overflows usize".to_string())
        })?;
        self.ensure_result_values(retained_scalar_values)?;
        if let Some(summary) =
            periodic_capability::summarize(&periodic_capability::periodic_residual_gaps(&circuit))
        {
            return Err(HbError::UnsupportedNonlinearDevices(summary).into());
        }
        if let Some(summary) =
            periodic_capability::summarize(&periodic_capability::periodic_descriptor_gaps(&circuit))
        {
            return Err(SimulationError::unsupported_capability(
                "analysis.pac.periodic_mna",
                format!(
                    "PAC exact periodic MNA is unavailable because the circuit contains {summary}"
                ),
            ));
        }

        let drive_tones = Self::hb_collect_drive_tones(&hb_config)?;

        let mut solver = HbSolver::try_new(hb_config.clone(), num_nodes).map_err(|error| {
            SimulationError::Circuit(format!("PAC solver construction failed: {error}"))
        })?;
        let node_names = self.hb_build_node_names(&circuit, num_nodes);
        solver.set_node_names(node_names.clone());

        // Use one canonical exact-MNA solver for both the large-signal
        // operating point and its periodic small-signal linearization. The
        // authored source spectra must be registered before the canonical
        // V/L/R branch map so its voltage-source descriptors retain the same
        // large-signal constraints Newton solves. Keeping one registry also
        // makes branch identity drift between the producer and consumer
        // structurally impossible.
        self.hb_stamp_resistors(&circuit, &mut solver);
        self.hb_stamp_capacitors(&circuit, &mut solver);
        self.hb_stamp_voltage_sources(&circuit, &mut solver, &hb_config, &drive_tones)?;
        self.hb_stamp_periodic_mna_branches(&circuit, &mut solver)?;
        self.hb_stamp_current_sources(&circuit, &mut solver, &hb_config, &drive_tones)?;

        let has_nonlinear = periodic_capability::has_exact_periodic_nonlinear_devices(&circuit);
        if has_nonlinear {
            self.hb_stamp_supported_nonlinear_devices(&circuit, &mut solver, num_nodes);
        }
        let branch_names = solver.try_periodic_mna_branch_names().map_err(|error| {
            SimulationError::Circuit(format!(
                "PAC branch-result metadata construction failed: {error}"
            ))
        })?;

        if let Some(PacOperatingPoint::HarmonicBalance(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, &hb_config)?;
        }
        if let Some(PacOperatingPoint::Shooting(point)) = &operating_point {
            point.authenticate_for_reuse(netlist, &self.config, point.config())?;
        }

        let solve_operating_point = operating_point.is_none();
        let mut state = if let Some(operating_point) = operating_point {
            match operating_point {
                PacOperatingPoint::Shooting(point) => {
                    self.hb_state_from_pss_operating_point(point, &hb_config, &node_names)?
                }
                PacOperatingPoint::HarmonicBalance(point) => {
                    point.to_solver_state(&node_names, &branch_names)?
                }
            }
        } else {
            HbSolverState::new(num_nodes, op_harmonics)
        };
        let branch_count = branch_names.len();
        state
            .try_prepare_mna_branches(branch_count, hb_config.num_harmonics)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "PAC operating-point MNA state construction failed: {error}"
                ))
            })?;
        if solve_operating_point {
            if has_nonlinear {
                solver
                    .solve_newton_with_abort(&mut state, abort)
                    .map_err(|e| match e {
                        crate::analysis::HbError::Aborted => SimulationError::Aborted,
                        _ => SimulationError::Circuit(format!(
                            "PAC operating-point solve failed: {e}"
                        )),
                    })?;
            } else {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                solver.solve_linear(&mut state).map_err(|e| {
                    SimulationError::Circuit(format!("PAC operating-point solve failed: {e}"))
                })?;
            }
        }
        if num_nodes.checked_add(branch_count) != Some(periodic_unknowns) {
            return Err(SimulationError::Circuit(format!(
                "PAC periodic solver exposes {num_nodes} nodes and {branch_count} branches, but resource qualification used {periodic_unknowns} MNA unknowns"
            )));
        }

        // Resolve the named source to an exact unit small-signal excitation.
        let input_port = Self::pac_input_port(&circuit, &input_name, num_nodes)?;
        let branch_voltage = input_port
            .voltage_source_index
            .map(|source_index| {
                solver
                    .periodic_voltage_source_branch(source_index)
                    .map(|branch| vec![(branch, Complex64::new(1.0, 0.0))])
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "PAC input voltage source '{input_name}' has no periodic MNA branch"
                        ))
                    })
            })
            .transpose()?
            .unwrap_or_default();

        let mut sweep_config = config.clone();
        sweep_config.fundamental_freq = config.fundamental_freq;
        let frequencies = sweep_config
            .try_frequency_points_with_abort(abort)
            .map_err(|error| match error {
                crate::analysis::FrequencyGridError::Aborted => SimulationError::Aborted,
                _ => SimulationError::Circuit(format!("Invalid PAC frequency sweep: {error}")),
            })?;
        if frequencies.is_empty() {
            return Err(SimulationError::Circuit(
                "PAC frequency sweep produced no points".to_string(),
            ));
        }

        let output_idx = config
            .output_node
            .as_deref()
            .map(|name| {
                if netlist.ground_policy().is_ground(name) {
                    return Err(SimulationError::Circuit(
                        "PAC output node cannot be ground".to_owned(),
                    ));
                }
                node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "PAC output node '{name}' not found in circuit nodes"
                        ))
                    })
            })
            .transpose()?;
        let output_ref_idx = config
            .output_ref
            .as_deref()
            .filter(|name| !name.trim().is_empty())
            .map(|name| {
                if netlist.ground_policy().is_ground(name) {
                    return Ok(None);
                }
                node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name))
                    .map(Some)
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "PAC output reference node '{name}' not found in circuit nodes"
                        ))
                    })
            })
            .transpose()?
            .flatten();
        if output_idx.is_some() && output_idx == output_ref_idx {
            return Err(SimulationError::Circuit(
                "PAC output node and reference node must be different".to_owned(),
            ));
        }

        let mut result = if output_idx.is_some() {
            PacResult::new(
                config.fundamental_freq,
                frequencies,
                config.sideband_min,
                config.sideband_max,
                node_names.clone(),
                branch_names,
            )
        } else {
            PacResult::new_without_conversion_matrix(
                config.fundamental_freq,
                frequencies,
                config.sideband_min,
                config.sideband_max,
                node_names.clone(),
                branch_names,
            )
        }
        .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        result.set_input_source(&input_name);
        if let Some(ref out) = config.output_node {
            result.set_output_node(out);
        }
        // The solve below drives a unit excitation so the conversion matrix
        // stays a transfer function. The authored drive amplitude and the
        // sideband-zero selection travel on the result for whoever publishes
        // it; see `PacResult::pac_magnitude` and `PacResult::include_dc`.
        result.pac_magnitude = config.pac_magnitude;
        result.include_dc = config.include_dc;

        // Excitation columns: the input source's own frequency (m = 0)
        // always; every input sideband when a conversion matrix is wanted.
        let excitation_count = if output_idx.is_some() {
            sideband_count
        } else {
            1
        };
        let mut excitation_sidebands = Vec::new();
        excitation_sidebands
            .try_reserve_exact(excitation_count)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "PAC excitation-sideband allocation failed: {error}"
                ))
            })?;
        if output_idx.is_some() {
            excitation_sidebands.extend(config.sideband_min..=config.sideband_max);
        } else {
            excitation_sidebands.push(0);
        }

        for freq_idx in 0..result.frequencies.len() {
            let offset = result.frequencies[freq_idx];
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut excitations = Vec::new();
            let mut branch_excitations: Vec<&[(usize, Complex64)]> = Vec::new();
            excitations
                .try_reserve_exact(excitation_sidebands.len())
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "PAC excitation-column allocation failed: {error}"
                    ))
                })?;
            branch_excitations
                .try_reserve_exact(excitation_sidebands.len())
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "PAC branch-excitation-column allocation failed: {error}"
                    ))
                })?;
            for &m in &excitation_sidebands {
                let mut column_injections = Vec::new();
                column_injections
                    .try_reserve_exact(input_port.node_injections.len())
                    .map_err(|error| {
                        SimulationError::Circuit(format!(
                            "PAC input-port allocation failed: {error}"
                        ))
                    })?;
                column_injections.extend_from_slice(&input_port.node_injections);
                excitations.push(PeriodicAcExcitation {
                    sideband: m,
                    injections: column_injections,
                });
                branch_excitations.push(branch_voltage.as_slice());
            }

            let sideband_count = result.num_sidebands();
            solver
                .solve_periodic_ac_each_with_branch_voltages(
                    &state,
                    PeriodicSidebandWindow {
                        offset_hz: offset,
                        sideband_min: config.sideband_min,
                        sideband_max: config.sideband_max,
                    },
                    &excitations,
                    &branch_excitations,
                    |col, solution| {
                        if abort.is_aborted() {
                            return Err(AnalysisHbError::Aborted);
                        }
                        if solution.len() != lifted_unknowns {
                            return Err(AnalysisHbError::InvalidCircuit(format!(
                                "PAC solver returned {} values for a {lifted_unknowns}-value lifted MNA system",
                                solution.len()
                            )));
                        }
                        if let Some((index, value)) = solution
                            .iter()
                            .copied()
                            .enumerate()
                            .find(|(_, value)| !value.re.is_finite() || !value.im.is_finite())
                        {
                            return Err(AnalysisHbError::InvalidCircuit(format!(
                                "PAC solver returned a non-finite value at lifted MNA index {index} ({:+.6e}{:+.6e}j)",
                                value.re, value.im
                            )));
                        }
                        let m = *excitation_sidebands.get(col).ok_or_else(|| {
                            AnalysisHbError::InvalidCircuit(format!(
                                "PAC returned unexpected excitation column {col}"
                            ))
                        })?;
                        if m == 0 {
                            for k_idx in 0..sideband_count {
                                let k = i64::from(config.sideband_min)
                                    .checked_add(i64::try_from(k_idx).map_err(|_| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC sideband offset exceeds i64".to_string(),
                                        )
                                    })?)
                                    .and_then(|value| i32::try_from(value).ok())
                                    .ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC sideband index exceeds i32".to_string(),
                                        )
                                    })?;
                                let data = result
                                    .get_sideband_data_mut(freq_idx, k)
                                    .ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC result is missing frequency {freq_idx}, sideband {k}"
                                        ))
                                    })?;
                                if data.node_voltages.len() != num_nodes
                                    || data.branch_currents.len() != branch_count
                                {
                                    return Err(AnalysisHbError::InvalidCircuit(format!(
                                        "PAC sideband result cardinality differs at frequency {freq_idx}, sideband {k}: {} nodes/{} branches; expected {num_nodes}/{branch_count}",
                                        data.node_voltages.len(),
                                        data.branch_currents.len()
                                    )));
                                }
                                for node in 0..num_nodes {
                                    let index = node
                                        .checked_mul(sideband_count)
                                        .and_then(|row| row.checked_add(k_idx))
                                        .ok_or_else(|| {
                                            AnalysisHbError::InvalidCircuit(
                                                "PAC node-spectrum index overflows usize"
                                                    .to_string(),
                                            )
                                        })?;
                                    let value = *solution.get(index).ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC solution is missing node {node}, sideband {k}"
                                        ))
                                    })?;
                                    data.set_voltage(node, value).map_err(|error| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC node-spectrum publication failed: {error}"
                                        ))
                                    })?;
                                }
                                for branch in 0..branch_count {
                                    let row = num_nodes.checked_add(branch).ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC branch-row index overflows usize".to_string(),
                                        )
                                    })?;
                                    let index = row
                                        .checked_mul(sideband_count)
                                        .and_then(|row| row.checked_add(k_idx))
                                        .ok_or_else(|| {
                                            AnalysisHbError::InvalidCircuit(
                                                "PAC branch-spectrum index overflows usize"
                                                    .to_string(),
                                            )
                                        })?;
                                    let value = *solution.get(index).ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC solution is missing branch {branch}, sideband {k}"
                                        ))
                                    })?;
                                    data.set_current(branch, value).map_err(|error| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC branch-spectrum publication failed: {error}"
                                        ))
                                    })?;
                                }
                            }
                        }

                        if let Some(out) = output_idx {
                            for k_idx in 0..sideband_count {
                                let k = i64::from(config.sideband_min)
                                    .checked_add(i64::try_from(k_idx).map_err(|_| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC sideband offset exceeds i64".to_string(),
                                        )
                                    })?)
                                    .and_then(|value| i32::try_from(value).ok())
                                    .ok_or_else(|| {
                                        AnalysisHbError::InvalidCircuit(
                                            "PAC sideband index exceeds i32".to_string(),
                                        )
                                    })?;
                                let output = solution[out * sideband_count + k_idx];
                                let output_voltage = if let Some(reference) = output_ref_idx {
                                    output - solution[reference * sideband_count + k_idx]
                                } else {
                                    output
                                };
                                if !output_voltage.re.is_finite()
                                    || !output_voltage.im.is_finite()
                                {
                                    return Err(AnalysisHbError::InvalidCircuit(format!(
                                        "PAC differential output is non-representable at offset {offset:.6e} Hz, input sideband {m}, output sideband {k}"
                                    )));
                                }
                                result
                                    .conversion_matrix
                                    .set(freq_idx, k, m, output_voltage)
                                    .map_err(|error| {
                                        AnalysisHbError::InvalidCircuit(format!(
                                            "PAC conversion publication failed: {error}"
                                        ))
                                    })?;
                            }
                        }
                        if abort.is_aborted() {
                            return Err(AnalysisHbError::Aborted);
                        }
                        Ok(())
                    },
                )
                .map_err(|error| match error {
                    AnalysisHbError::Aborted => SimulationError::Aborted,
                    error => SimulationError::Circuit(format!(
                        "PAC solve failed at offset {offset:.6e} Hz: {error}"
                    )),
                })?;

            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
        }

        result.set_convergence_info(state.iteration.max(1), state.residual_norm);

        Ok(PacAnalysisResult {
            result,
            fundamental_freq: config.fundamental_freq,
            converged: state.converged,
        })
    }

    /// Map the named small-signal source to an exact MNA excitation.
    pub(in crate::engine::hb) fn pac_input_port(
        circuit: &CircuitData,
        input_name: &str,
        num_nodes: usize,
    ) -> Result<PacInputPort, SimulationError> {
        let trimmed = input_name.trim();

        let validate_terminals = |kind: &str, np: usize, nn: usize| {
            if np > num_nodes || nn > num_nodes {
                return Err(SimulationError::Circuit(format!(
                    "PAC input {kind} source '{trimmed}' references node pair ({np}, {nn}), outside the circuit's 0..={num_nodes} node range"
                )));
            }
            if np == nn {
                return Err(SimulationError::Circuit(format!(
                    "PAC input {kind} source '{trimmed}' has identical terminals and no effective port"
                )));
            }
            Ok(())
        };

        if let Some(idx) = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(trimmed))
        {
            let np = circuit.voltage_sources.node_pos[idx];
            let nn = circuit.voltage_sources.node_neg[idx];
            validate_terminals("voltage", np, nn)?;
            return Ok(PacInputPort {
                node_injections: Vec::new(),
                voltage_source_index: Some(idx),
            });
        }

        if let Some(idx) = circuit
            .current_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(trimmed))
        {
            let np = circuit.current_sources.node_pos[idx];
            let nn = circuit.current_sources.node_neg[idx];
            validate_terminals("current", np, nn)?;
            let mut injections = Vec::new();
            if np > 0 {
                injections.push((np - 1, Complex64::new(-1.0, 0.0)));
            }
            if nn > 0 {
                injections.push((nn - 1, Complex64::new(1.0, 0.0)));
            }
            return Ok(PacInputPort {
                node_injections: injections,
                voltage_source_index: None,
            });
        }

        Err(SimulationError::Circuit(format!(
            "PAC input source '{trimmed}' not found among independent sources"
        )))
    }
}
