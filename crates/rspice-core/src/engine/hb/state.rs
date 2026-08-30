//! Retention and phase projection support for converged HB periodic state.

use super::*;
use crate::abort_signal::{AbortSignal, NoAbort};
#[cfg(feature = "veriloga")]
use crate::analysis::harmonic_balance::HbContinuationLimitation;
use crate::analysis::harmonic_balance::{HbReactiveKind, HbReactiveSpectrum};
use crate::circuit::CircuitData;
use crate::engine::transient::{
    netlist_checkpoint_identity, netlist_fingerprint, simulation_checkpoint_identity,
};
use crate::engine::{TransientCheckpoint, TransientResult};
use crate::numerics::integration::LteEstimator;
use std::collections::BTreeSet;
use std::f64::consts::TAU;

/// Explicit state-completeness contract for an HB-derived Envelope warm start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbEnvelopeStateGuarantee {
    /// Exact phase projection for ordinary linear resistors/capacitors,
    /// independent current sources, and ideal voltage-source MNA branches.
    ExactLinearRcMnaV1,
}

/// Authenticated HB carrier state that can restart transient integration at
/// slow-time origin zero for the deliberately narrow complete circuit subset.
#[derive(Debug, Clone, PartialEq)]
pub struct HbEnvelopeContinuationState {
    guarantee: HbEnvelopeStateGuarantee,
    fundamental_freq: Value,
    num_harmonics: usize,
    hb_config_identity: String,
    canonical_frozen_sources: Vec<String>,
    original_netlist_identity: String,
    resolved_simulation_identity: String,
    history_step: Value,
    checkpoint: TransientCheckpoint,
}

impl HbEnvelopeContinuationState {
    pub fn guarantee(&self) -> HbEnvelopeStateGuarantee {
        self.guarantee
    }

    pub fn fundamental_freq(&self) -> Value {
        self.fundamental_freq
    }

    pub fn num_harmonics(&self) -> usize {
        self.num_harmonics
    }

    pub fn canonical_frozen_sources(&self) -> &[String] {
        &self.canonical_frozen_sources
    }

    pub fn original_netlist_identity(&self) -> &str {
        &self.original_netlist_identity
    }

    pub fn resolved_simulation_identity(&self) -> &str {
        &self.resolved_simulation_identity
    }

    pub fn hb_config_identity(&self) -> &str {
        &self.hb_config_identity
    }

    pub fn history_step(&self) -> Value {
        self.history_step
    }

    pub fn time_origin(&self) -> Value {
        self.checkpoint.time
    }
}

impl Engine {
    fn hb_envelope_config_identity(config: &HbConfig) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"rspice-hb-envelope-config-v1\0");
        let mut field = |name: &str, bytes: &[u8]| {
            hasher.update(&(name.len() as u64).to_le_bytes());
            hasher.update(name.as_bytes());
            hasher.update(&(bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        };
        field(
            "fundamental_freq",
            &config.fundamental_freq.to_bits().to_le_bytes(),
        );
        field(
            "num_harmonics",
            &(config.num_harmonics as u64).to_le_bytes(),
        );
        field("tone_count", &(config.tones.len() as u64).to_le_bytes());
        for (index, tone) in config.tones.iter().enumerate() {
            field(
                &format!("tone[{index}].frequency"),
                &tone.frequency.to_bits().to_le_bytes(),
            );
            field(
                &format!("tone[{index}].num_harmonics"),
                &(tone.num_harmonics as u64).to_le_bytes(),
            );
            field(&format!("tone[{index}].name"), tone.name.as_bytes());
            if let Some(source_name) = &tone.source_name {
                field(
                    &format!("tone[{index}].source_name"),
                    source_name.as_bytes(),
                );
            } else {
                field(&format!("tone[{index}].source_name"), b"<none>");
            }
        }
        for (name, value) in [
            ("tolerance", config.tolerance),
            ("abstol", config.abstol),
            ("damping", config.damping),
            ("min_damping", config.min_damping),
        ] {
            field(name, &value.to_bits().to_le_bytes());
        }
        for (name, value) in [
            ("max_iterations", config.max_iterations),
            ("oversample_factor", config.oversample_factor),
            ("max_mixing_order", config.max_mixing_order),
            ("gmres_restart", config.gmres_restart),
        ] {
            field(name, &(value as u64).to_le_bytes());
        }
        field(
            "collocation_points",
            &(config
                .collocation_points
                .map(|value| value as u64)
                .unwrap_or(u64::MAX))
            .to_le_bytes(),
        );
        for (name, value) in [
            ("use_krylov", config.use_krylov),
            ("source_stepping", config.source_stepping),
            ("use_exact_jacobian", config.use_exact_jacobian),
            ("verbose", config.verbose),
        ] {
            field(name, &[u8::from(value)]);
        }
        hasher.finalize().to_hex().to_string()
    }

    fn hb_envelope_requested_source_set(
        frozen_source_names: &[String],
    ) -> Result<BTreeSet<String>, SimulationError> {
        let mut requested = BTreeSet::new();
        for name in frozen_source_names {
            let canonical = name.trim().to_ascii_lowercase();
            if canonical.is_empty() {
                return Err(SimulationError::Circuit(
                    "HB Envelope frozen-source selection contains an empty source name".to_string(),
                ));
            }
            if !requested.insert(canonical.clone()) {
                return Err(SimulationError::Circuit(format!(
                    "HB Envelope frozen-source selection contains duplicate source '{canonical}'"
                )));
            }
        }
        Ok(requested)
    }

    fn hb_envelope_freeze_selected_sources(
        circuit: &mut CircuitData,
        requested: &BTreeSet<String>,
    ) -> Result<Vec<String>, SimulationError> {
        let mut canonical_names = Vec::with_capacity(requested.len());
        for requested_name in requested {
            let voltage_matches = circuit
                .voltage_sources
                .names
                .iter()
                .enumerate()
                .filter(|(_, name)| name.eq_ignore_ascii_case(requested_name))
                .collect::<Vec<_>>();
            let current_matches = circuit
                .current_sources
                .names
                .iter()
                .enumerate()
                .filter(|(_, name)| name.eq_ignore_ascii_case(requested_name))
                .collect::<Vec<_>>();
            if voltage_matches.len() + current_matches.len() != 1 {
                return Err(SimulationError::Circuit(format!(
                    "HB Envelope frozen-source selection references {} independent source '{requested_name}'",
                    if voltage_matches.is_empty() && current_matches.is_empty() {
                        "unknown"
                    } else {
                        "ambiguous"
                    }
                )));
            }

            let (canonical_name, has_waveform) =
                if let Some((index, name)) = voltage_matches.first() {
                    (
                        (*name).clone(),
                        circuit.voltage_sources.source_specs[*index].is_some(),
                    )
                } else {
                    let (index, name) = current_matches[0];
                    (
                        name.clone(),
                        circuit.current_sources.source_specs[index].is_some(),
                    )
                };
            if !has_waveform {
                return Err(SimulationError::Circuit(format!(
                    "HB Envelope frozen source '{canonical_name}' does not define a transient waveform"
                )));
            }
            let value = circuit
                .voltage_sources
                .freeze_transient_source_at_time(&canonical_name, 0.0)
                .or_else(|| {
                    circuit
                        .current_sources
                        .freeze_transient_source_at_time(&canonical_name, 0.0)
                })
                .expect("the uniquely matched independent source remains present");
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "HB Envelope frozen source '{canonical_name}' evaluates to non-finite value {value:e} at time zero"
                )));
            }
            canonical_names.push(canonical_name);
        }
        canonical_names.sort_by_key(|name| name.to_ascii_lowercase());
        Ok(canonical_names)
    }

    fn ensure_hb_envelope_linear_subset(circuit: &CircuitData) -> Result<(), SimulationError> {
        let mut blockers = Vec::new();
        if !circuit.resistor_branches.is_empty() {
            blockers.push("zero-resistance MNA branches");
        }
        if !circuit.inductors.is_empty() {
            blockers.push("inductors");
        }
        if circuit.has_nonlinear_devices() {
            blockers.push("nonlinear devices");
        }
        if !circuit.vcvs.is_empty()
            || !circuit.vccs.is_empty()
            || !circuit.cccs.is_empty()
            || !circuit.ccvs.is_empty()
        {
            blockers.push("controlled sources");
        }
        if !circuit.vswitches.is_empty()
            || !circuit.iswitches.is_empty()
            || !circuit.generic_switches.is_empty()
        {
            blockers.push("switches");
        }
        if !circuit.tlines.is_empty()
            || !circuit.coupled_tlines.is_empty()
            || !circuit.couplings.is_empty()
            || !circuit.coupled_inductor_pairs.is_empty()
            || !circuit.multi_winding_transformers.is_empty()
            || !circuit.jiles_atherton_inductors.is_empty()
            || !circuit.xyce_core_groups.is_empty()
        {
            blockers.push("distributed or magnetic state");
        }
        if !circuit.behavioral_sources.is_empty() {
            blockers.push("behavioral sources");
        }
        if !circuit.xspice_instances.is_empty() {
            blockers.push("XSPICE devices");
        }
        #[cfg(feature = "veriloga")]
        if circuit.has_veriloga_devices() {
            blockers.push("Verilog-A devices");
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if circuit.has_generated_veriloga_devices() {
            blockers.push("generated Verilog-A devices");
        }
        if circuit.capacitors.internal.iter().any(|internal| *internal)
            || circuit
                .capacitors
                .ic_branch_indices
                .iter()
                .any(Option::is_some)
        {
            blockers.push("internal or IC-constrained capacitor branches");
        }
        if circuit.num_branches() != circuit.voltage_sources.len() {
            blockers.push("unrecognized MNA branch families");
        }

        if blockers.is_empty() {
            Ok(())
        } else {
            blockers.sort_unstable();
            blockers.dedup();
            Err(SimulationError::Circuit(format!(
                "HB Envelope continuation is unavailable because the circuit contains {}; the exact initializer currently supports only ordinary R/C elements and independent voltage/current sources",
                blockers.join(", ")
            )))
        }
    }

    fn hb_terminal_voltage_spectrum(
        result: &HbResult,
        node_pos: usize,
        node_neg: usize,
    ) -> Vec<Complex64> {
        (0..=result.num_harmonics)
            .map(|harmonic| {
                let node_value = |node: usize| {
                    if node == 0 {
                        Complex64::new(0.0, 0.0)
                    } else {
                        result
                            .spectral_voltages
                            .get(node - 1)
                            .and_then(|spectrum| spectrum.coefficients.get(harmonic))
                            .copied()
                            .unwrap_or_default()
                    }
                };
                node_value(node_pos) - node_value(node_neg)
            })
            .collect()
    }

    fn hb_envelope_solution_at_phase(
        circuit: &CircuitData,
        phase: &crate::analysis::HbPhaseState,
    ) -> Result<Vec<Value>, SimulationError> {
        let mut solution = vec![0.0; circuit.matrix_size()];
        let mut seen_nodes = BTreeSet::new();
        for (name, voltage) in &phase.node_voltages {
            let node = circuit.get_node_by_name(name).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "HB Envelope phase projection references unknown node '{name}'"
                ))
            })?;
            if node > 0 {
                solution[node - 1] = *voltage;
                seen_nodes.insert(node);
            }
        }
        if seen_nodes.len() != circuit.num_nodes() {
            return Err(SimulationError::Circuit(format!(
                "HB Envelope phase projection retained {} of {} circuit nodes",
                seen_nodes.len(),
                circuit.num_nodes()
            )));
        }

        let mut seen_branches = BTreeSet::new();
        for (name, current) in &phase.mna_branch_currents {
            let branch = circuit.get_branch_by_name(name).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "HB Envelope phase projection references unknown MNA branch '{name}'"
                ))
            })?;
            let index = circuit.num_nodes() + branch - 1;
            let slot = solution.get_mut(index).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "HB Envelope MNA branch '{name}' has out-of-range ordinal {branch}"
                ))
            })?;
            *slot = *current;
            seen_branches.insert(branch);
        }
        if seen_branches.len() != circuit.voltage_sources.len() {
            return Err(SimulationError::Circuit(format!(
                "HB Envelope phase projection retained {} of {} ideal voltage-source branches",
                seen_branches.len(),
                circuit.voltage_sources.len()
            )));
        }
        if solution.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "HB Envelope phase projection contains non-finite MNA state".to_string(),
            ));
        }
        Ok(solution)
    }

    fn hb_envelope_checkpoint(
        &self,
        netlist: &Netlist,
        authenticated_netlist_identity: &str,
        mut circuit: CircuitData,
        config: &HbConfig,
        result: &HbResult,
    ) -> Result<(TransientCheckpoint, Value), SimulationError> {
        if !result.is_valid() || !result.continuation_limitations.is_empty() {
            return Err(SimulationError::Circuit(
                "HB Envelope continuation requires a converged, complete periodic state"
                    .to_string(),
            ));
        }
        let history_points = config.fft_size().max(4);
        let period = 1.0 / config.fundamental_freq;
        let history_step = period / history_points as Value;
        if !history_step.is_finite() || history_step <= 0.0 {
            return Err(SimulationError::Circuit(
                "HB Envelope continuation history interval is invalid".to_string(),
            ));
        }

        let phase_step = TAU / history_points as Value;
        let mut projected = Vec::with_capacity(4);
        let mut solutions = Vec::with_capacity(4);
        for offset in (0..4).rev() {
            let phase = -(offset as Value) * phase_step;
            let state = result
                .project_phase(phase)
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
            let solution = Self::hb_envelope_solution_at_phase(&circuit, &state)?;
            projected.push(state);
            solutions.push(solution);
        }

        let latest = projected
            .last()
            .expect("four HB history projections were constructed");
        let previous = &projected[2];
        let older = &projected[1];
        let oldest = &projected[0];
        for index in 0..circuit.capacitors.len() {
            let name = &circuit.capacitors.names[index];
            let reactive_at = |state: &crate::analysis::HbPhaseState| {
                state
                    .reactive_states
                    .iter()
                    .find(|reactive| {
                        reactive.kind == HbReactiveKind::Capacitor
                            && reactive.device_name.eq_ignore_ascii_case(name)
                    })
                    .map(|reactive| (reactive.voltage, reactive.current))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "HB Envelope phase projection omitted capacitor '{name}'"
                        ))
                    })
            };
            let latest_reactive = reactive_at(latest)?;
            circuit.capacitors.v_prev[index] = latest_reactive.0;
            circuit.capacitors.v_prev_prev[index] = reactive_at(previous)?.0;
            circuit.capacitors.v_prev_prev_prev[index] = reactive_at(older)?.0;
            // The oldest sample is deliberately evaluated as part of the
            // accepted history proof even though the checkpoint format retains
            // three capacitor voltages.
            let _ = reactive_at(oldest)?;
            circuit.capacitors.i_prev[index] = latest_reactive.1;
            let coefficients = crate::numerics::integration::CompanionCoefficients::for_method(
                self.config.integration_method,
            );
            circuit.capacitors.i_eq[index] = coefficients.capacitor_ieq(
                circuit.capacitors.capacitances[index],
                history_step,
                circuit.capacitors.v_prev[index],
                circuit.capacitors.v_prev_prev[index],
                circuit.capacitors.i_prev[index],
            );
        }

        let lte_reference = self
            .config
            .transient_lte_reference
            .unwrap_or_else(|| self.config.spice_dialect.default_transient_lte_reference());
        let mut lte_estimator = LteEstimator::with_tolerances_and_reference(
            self.transient_lte_reltol(),
            self.transient_lte_abstol(),
            lte_reference,
        );
        for solution in &solutions {
            lte_estimator.record(solution, history_step);
        }
        let checkpoint = TransientCheckpoint::capture(
            netlist_fingerprint(netlist),
            Some(authenticated_netlist_identity.to_string()),
            simulation_checkpoint_identity(&self.config),
            0.0,
            solutions.last().expect("latest HB solution exists"),
            &circuit,
            crate::engine::TransientStartupMode::OperatingPoint,
            Some(&lte_estimator),
        )
        .map_err(SimulationError::Circuit)?;
        Ok((checkpoint, history_step))
    }

    /// Solve an authenticated carrier-periodic HB state with selected slow
    /// source waveforms frozen at their exact time-zero values, then create an
    /// exact transient continuation state for the supported linear R/C subset.
    pub fn run_hb_envelope_continuation_state(
        &self,
        netlist: &Netlist,
        config: HbConfig,
        frozen_source_names: &[String],
    ) -> Result<(HbAnalysisResult, HbEnvelopeContinuationState), SimulationError> {
        self.run_hb_envelope_continuation_state_with_abort(
            netlist,
            config,
            frozen_source_names,
            &NoAbort,
        )
    }

    /// Cancellable form of [`Self::run_hb_envelope_continuation_state`].
    pub fn run_hb_envelope_continuation_state_with_abort(
        &self,
        netlist: &Netlist,
        config: HbConfig,
        frozen_source_names: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<(HbAnalysisResult, HbEnvelopeContinuationState), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        let config = engine.hb_config_for_netlist(netlist, config)?;
        engine.hb_validate_config(&config)?;
        let requested_sources = Self::hb_envelope_requested_source_set(frozen_source_names)?;
        let authenticated_netlist_identity = netlist_checkpoint_identity(netlist)
            .expect("every elaborated netlist has a semantic identity");
        let original_circuit = engine.build_circuit_with_abort(netlist, abort)?;
        if netlist_checkpoint_identity(netlist).as_deref()
            != Some(authenticated_netlist_identity.as_str())
        {
            return Err(SimulationError::Circuit(
                "HB Envelope input dependencies changed during circuit elaboration; refusing to authenticate a stale source snapshot"
                    .to_string(),
            ));
        }
        Self::ensure_hb_envelope_linear_subset(&original_circuit)?;
        let mut frozen_circuit = original_circuit.clone();
        let canonical_frozen_sources =
            Self::hb_envelope_freeze_selected_sources(&mut frozen_circuit, &requested_sources)?;
        let analysis = engine.run_hb_with_prebuilt_circuit_abort(
            netlist,
            frozen_circuit,
            config.clone(),
            None,
            abort,
        )?;
        if netlist_checkpoint_identity(netlist).as_deref()
            != Some(authenticated_netlist_identity.as_str())
        {
            return Err(SimulationError::Circuit(
                "HB Envelope input dependencies changed during the carrier solve; refusing to publish a mismatched continuation artifact"
                    .to_string(),
            ));
        }
        let (checkpoint, history_step) = engine.hb_envelope_checkpoint(
            netlist,
            &authenticated_netlist_identity,
            original_circuit,
            &config,
            &analysis.result,
        )?;
        let resolved_simulation_identity = simulation_checkpoint_identity(&engine.config);
        Ok((
            analysis,
            HbEnvelopeContinuationState {
                guarantee: HbEnvelopeStateGuarantee::ExactLinearRcMnaV1,
                fundamental_freq: config.fundamental_freq,
                num_harmonics: config.num_harmonics,
                hb_config_identity: Self::hb_envelope_config_identity(&config),
                canonical_frozen_sources,
                original_netlist_identity: authenticated_netlist_identity,
                resolved_simulation_identity,
                history_step,
                checkpoint,
            },
        ))
    }

    /// Reactivate the original selected source waveforms at slow-time origin
    /// zero and continue transient integration from an authenticated HB state.
    pub fn run_tran_from_hb_envelope_state(
        &self,
        netlist: &Netlist,
        expected_hb_config: &HbConfig,
        expected_frozen_source_names: &[String],
        state: &HbEnvelopeContinuationState,
        duration: Value,
        max_step: Value,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        self.run_tran_from_hb_envelope_state_with_abort(
            netlist,
            expected_hb_config,
            expected_frozen_source_names,
            state,
            duration,
            max_step,
            &NoAbort,
        )
    }

    /// Cancellable form of [`Self::run_tran_from_hb_envelope_state`].
    #[allow(clippy::too_many_arguments)]
    pub fn run_tran_from_hb_envelope_state_with_abort(
        &self,
        netlist: &Netlist,
        expected_hb_config: &HbConfig,
        expected_frozen_source_names: &[String],
        state: &HbEnvelopeContinuationState,
        duration: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let engine = self.resolved_for_netlist(netlist);
        let expected_hb_config =
            engine.hb_config_for_netlist(netlist, expected_hb_config.clone())?;
        if state.guarantee != HbEnvelopeStateGuarantee::ExactLinearRcMnaV1 {
            return Err(SimulationError::Circuit(
                "HB Envelope continuation artifact has an unsupported completeness guarantee"
                    .to_string(),
            ));
        }
        if Self::hb_envelope_config_identity(&expected_hb_config) != state.hb_config_identity {
            return Err(SimulationError::Circuit(
                "HB Envelope continuation artifact belongs to a different HB configuration"
                    .to_string(),
            ));
        }
        let expected_sources =
            Self::hb_envelope_requested_source_set(expected_frozen_source_names)?;
        let retained_sources = state
            .canonical_frozen_sources
            .iter()
            .map(|source| source.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        if expected_sources != retained_sources {
            return Err(SimulationError::Circuit(
                "HB Envelope continuation artifact belongs to a different frozen-source set"
                    .to_string(),
            ));
        }
        if !duration.is_finite() || duration <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "HB Envelope continuation duration must be finite and positive, got {duration:e}"
            )));
        }
        let netlist_identity = netlist_checkpoint_identity(netlist)
            .expect("every elaborated netlist has a semantic identity");
        if netlist_identity != state.original_netlist_identity {
            return Err(SimulationError::Circuit(
                "HB Envelope continuation artifact belongs to a different original netlist"
                    .to_string(),
            ));
        }
        if simulation_checkpoint_identity(&engine.config) != state.resolved_simulation_identity {
            return Err(SimulationError::Circuit(
                "HB Envelope continuation artifact belongs to a different resolved simulation configuration"
                    .to_string(),
            ));
        }
        state
            .checkpoint
            .validate_for_with_config(netlist, &engine.config)
            .map_err(SimulationError::Circuit)?;
        let checkpoint = state
            .checkpoint
            .bind_authenticated_synthetic_origin_max_step(max_step)
            .map_err(SimulationError::Circuit)?;
        engine.run_tran_resume_with_abort(
            netlist,
            &checkpoint,
            state.time_origin() + duration,
            max_step,
            abort,
        )
    }

    pub(in crate::engine::hb) fn hb_attach_periodic_state(
        &self,
        circuit: &CircuitData,
        result: &mut HbResult,
    ) -> Result<(), SimulationError> {
        let omega0 = TAU * result.fundamental_freq;

        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            let voltage_coefficients =
                Self::hb_terminal_voltage_spectrum(result, stamp.pp.row, stamp.nn.row);
            let capacitance = circuit.capacitors.capacitances[index];
            let current_coefficients = voltage_coefficients
                .iter()
                .enumerate()
                .map(|(harmonic, &voltage)| {
                    let omega = harmonic as Value * omega0;
                    Complex64::new(0.0, omega * capacitance) * voltage
                })
                .collect();
            result.reactive_spectra.push(HbReactiveSpectrum {
                device_name: circuit.capacitors.names[index].clone(),
                kind: HbReactiveKind::Capacitor,
                voltage_coefficients,
                current_coefficients,
                dc_current_is_exact: true,
            });
        }

        for index in 0..circuit.inductors.len() {
            let voltage_coefficients = Self::hb_terminal_voltage_spectrum(
                result,
                circuit.inductors.node_pos[index],
                circuit.inductors.node_neg[index],
            );
            let exact_current = result
                .mna_branch_currents
                .iter()
                .find(|branch| {
                    branch
                        .device_name
                        .eq_ignore_ascii_case(&circuit.inductors.names[index])
                })
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "HB result lost exact MNA current spectrum for inductor '{}'",
                        circuit.inductors.names[index]
                    ))
                })?;
            if exact_current.coefficients.len() != voltage_coefficients.len()
                || exact_current
                    .coefficients
                    .iter()
                    .any(|value| !value.re.is_finite() || !value.im.is_finite())
            {
                return Err(SimulationError::Circuit(format!(
                    "HB exact MNA current spectrum for inductor '{}' is malformed",
                    circuit.inductors.names[index]
                )));
            }
            result.reactive_spectra.push(HbReactiveSpectrum {
                device_name: circuit.inductors.names[index].clone(),
                kind: HbReactiveKind::Inductor,
                voltage_coefficients,
                current_coefficients: exact_current.coefficients.clone(),
                dc_current_is_exact: true,
            });
        }

        #[cfg(feature = "veriloga")]
        if circuit.veriloga_devices().iter().next().is_some() {
            result
                .continuation_limitations
                .push(HbContinuationLimitation::VerilogAInternalStateNotRetained);
        }
        Ok(())
    }
}
