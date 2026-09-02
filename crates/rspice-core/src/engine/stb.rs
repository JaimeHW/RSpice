//! STB (loop stability) analysis: true loop gain via Tian's method.
//!
//! The probe is a designated 0 V voltage source placed in the feedback loop
//! (the Spectre `stb` probe convention — its MNA branch already carries the
//! loop current). At every sweep frequency two small-signal solves run
//! against the same linearized matrix, with every other independent source
//! AC-dead:
//!
//! 1. **Voltage injection**: the probe branch drives a unit series voltage.
//! 2. **Current injection**: a unit current is injected into the probe's
//!    sense terminal.
//!
//! With `v` the sense-terminal voltage and `i` the probe branch current
//! (positive from the + terminal through the source), the loop gain follows
//! Tian, Visvanathan, Hantgan, Kundert, "Striving for Small-Signal
//! Stability", IEEE Circuits & Devices Magazine 17(1), Jan 2001:
//!
//! ```text
//! T = -1 / (1 - 1/(2*(i1*v2 - v1*i2) + v1 + i2))
//! ```
//!
//! implemented in the algebraically equivalent, singularity-free form
//! `T = D/(1 - D)` with `D = 2*(i1*v2 - v1*i2) + v1 + i2`. Unlike single
//! voltage injection, the combination is exact for arbitrary loading and
//! bidirectional transmission at the break, and is independent of the probe
//! orientation. The convention yields phase 0 at DC for a stable inverting
//! loop, so phase margin is `180 deg + arg T` at unity crossover — exactly
//! what `StbAnalyzer` extracts.

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::stb::{StbAnalysisError, StbAnalyzer, StbConfig, StbResult};
use crate::{Complex64, Netlist, Value};
use std::f64::consts::PI;

// A completed STB result deliberately retains both its primary sweep and the
// derived Bode/Nyquist records.  Resource accounting must charge the copies
// that remain reachable from `StbAnalysisResult`, not just the three primary
// frequency/complex-loop-gain scalars.
const STB_PRIMARY_VALUES_PER_POINT: usize = 3;
const STB_BODE_VALUES_PER_POINT: usize = 6;
const STB_NYQUIST_VALUES_PER_POINT: usize = 3;
const STB_MARGIN_VALUES: usize = 6;

fn stb_retained_result_value_count(
    point_count: usize,
    compute_nyquist: bool,
) -> Result<usize, StbAnalysisError> {
    let values_per_point = STB_PRIMARY_VALUES_PER_POINT
        + STB_BODE_VALUES_PER_POINT
        + if compute_nyquist {
            STB_NYQUIST_VALUES_PER_POINT
        } else {
            0
        };
    point_count
        .checked_mul(values_per_point)
        .and_then(|values| values.checked_add(STB_MARGIN_VALUES))
        .ok_or(StbAnalysisError::CapacityOverflow {
            object: "STB retained-result value count",
        })
}

fn map_stb_analysis_error(error: StbAnalysisError) -> SimulationError {
    match error {
        StbAnalysisError::Aborted => SimulationError::Aborted,
        StbAnalysisError::InvalidConfiguration(error) => {
            SimulationError::Circuit(format!("Invalid STB config: {error}"))
        }
        StbAnalysisError::CapacityOverflow { .. } | StbAnalysisError::Allocation { .. } => {
            SimulationError::Circuit(error.to_string())
        }
    }
}

fn try_reserve_stb_values<T>(
    values: &mut Vec<T>,
    requested: usize,
    object: &'static str,
) -> Result<(), SimulationError> {
    values
        .try_reserve_exact(requested)
        .map_err(|_| map_stb_analysis_error(StbAnalysisError::Allocation { object, requested }))
}

fn try_owned_probe_name(probe: &str) -> Result<String, SimulationError> {
    let mut owned = String::new();
    owned.try_reserve_exact(probe.len()).map_err(|_| {
        map_stb_analysis_error(StbAnalysisError::Allocation {
            object: "STB probe name",
            requested: probe.len(),
        })
    })?;
    owned.push_str(probe);
    Ok(owned)
}

/// STB analysis result: the Tian loop gain sweep plus extracted margins.
#[derive(Debug, Clone)]
pub struct StbAnalysisResult {
    /// Sweep frequencies (Hz)
    pub frequencies: Vec<Value>,
    /// Complex loop gain at each frequency
    pub loop_gains: Vec<Complex64>,
    /// Bode/Nyquist data and stability margins
    pub result: StbResult,
    /// Name of the probe voltage source
    pub probe_name: String,
}

impl Engine {
    /// Run loop-stability analysis at the probe source named in
    /// `config.probe_node`.
    ///
    /// The probe must be an existing voltage source with zero DC value: a
    /// nonzero value would mean the element biases the circuit and was
    /// almost certainly not meant as a probe. Any AC specification on the
    /// probe or on other independent sources is ignored — loop gain is a
    /// property of the linearized network, not of the stimuli.
    pub fn run_stb(
        &self,
        netlist: &Netlist,
        config: StbConfig,
    ) -> Result<StbAnalysisResult, SimulationError> {
        self.run_stb_with_abort(netlist, config, &NoAbort)
    }

    /// Cancellable form of [`Self::run_stb`].
    pub fn run_stb_with_abort(
        &self,
        netlist: &Netlist,
        config: StbConfig,
        abort: &dyn AbortSignal,
    ) -> Result<StbAnalysisResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let frequency_count = config
            .frequency_point_count()
            .map_err(|err| SimulationError::Circuit(format!("Invalid STB config: {err}")))?;
        let retained_result_values =
            stb_retained_result_value_count(frequency_count, config.compute_nyquist)
                .map_err(map_stb_analysis_error)?;
        self.ensure_analysis_points(frequency_count)?;
        self.ensure_result_values(retained_result_values)?;

        let probe_name = config
            .probe_node
            .as_deref()
            .ok_or_else(|| {
                SimulationError::Circuit(
                    "STB requires a probe: name a 0 V voltage source placed in the loop"
                        .to_string(),
                )
            })?
            .trim();
        if probe_name.is_empty() {
            return Err(SimulationError::Circuit(
                "STB probe name is empty".to_string(),
            ));
        }
        let probe_name = try_owned_probe_name(probe_name)?;

        // Reserve every user-sized retained STB vector before circuit
        // construction. A hostile or unallocatable request therefore cannot
        // consume operating-point/frequency-solve work before it fails.
        let frequencies = config
            .try_frequency_points_with_abort(abort)
            .map_err(map_stb_analysis_error)?;
        if frequencies.len() != frequency_count {
            return Err(map_stb_analysis_error(StbAnalysisError::CapacityOverflow {
                object: "STB frequency grid",
            }));
        }
        let mut loop_gains = Vec::new();
        try_reserve_stb_values(&mut loop_gains, frequency_count, "STB loop-gain result")?;
        let prepared_result = StbResult::try_with_capacity(frequency_count, config.compute_nyquist)
            .map_err(map_stb_analysis_error)?;

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;
        if circuit.num_nodes() == 0 {
            return Err(SimulationError::Circuit("Circuit has no nodes".to_string()));
        }
        Self::ensure_no_mixed_signal_analysis(&circuit, "STB analysis")?;
        Self::ensure_supported_dynamic_charges(&circuit, "STB")?;

        let probe_idx = circuit
            .voltage_sources
            .names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(&probe_name))
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "STB probe '{probe_name}' is not a voltage source in the circuit; \
                     insert a 0 V source in the loop and name it as the probe"
                ))
            })?;

        let dc_value = circuit.voltage_sources.dc_values[probe_idx];
        if dc_value != 0.0 {
            return Err(SimulationError::Circuit(format!(
                "STB probe '{probe_name}' has DC value {dc_value} V; the probe must be \
                 a 0 V source so it only senses the loop"
            )));
        }

        let node_pos = circuit.voltage_sources.node_pos[probe_idx];
        let node_neg = circuit.voltage_sources.node_neg[probe_idx];
        if node_pos == 0 || node_neg == 0 {
            // With one terminal on the reference node the method is blind,
            // not merely inaccurate: the zero-volt probe shorts the current
            // injection straight to ground, so the second experiment carries
            // no loop information and the formula degenerates to T = 0 for
            // any circuit. Ground is the nodal reference, not a branch of
            // the loop — there is nothing to break there.
            return Err(SimulationError::Circuit(format!(
                "STB probe '{probe_name}' has a grounded terminal; place the probe in \
                 series with the loop's signal path (both terminals on circuit nodes), \
                 e.g. between the output and the feedback network"
            )));
        }

        // Loop gain is independent of the probe orientation (Tian's method
        // is symmetric); measure at the + terminal by convention.
        let sense_node = node_pos;

        let br_ordinal = circuit.voltage_sources.branch_indices[probe_idx];

        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let has_nonlinear = circuit.has_nonlinear_devices();
        let dc_solution = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            abort,
        )?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if has_nonlinear {
            circuit.update_nonlinear(&dc_solution);
        }
        circuit
            .prepare_behavioral_small_signal(&dc_solution)
            .map_err(SimulationError::Circuit)?;

        let size = circuit.matrix_size();
        let br = circuit.get_branch_matrix_index(br_ordinal);

        let mut ac_matrix = rspice_matrix::ComplexMatrix::from_real_structure(&matrix);
        let batched_value_count = size.checked_mul(2).ok_or_else(|| {
            map_stb_analysis_error(StbAnalysisError::CapacityOverflow {
                object: "STB batched solve workspace",
            })
        })?;
        let mut batched_rhs = Vec::new();
        try_reserve_stb_values(
            &mut batched_rhs,
            batched_value_count,
            "STB batched right-hand side",
        )?;
        batched_rhs.resize(batched_value_count, Complex64::new(0.0, 0.0));
        batched_rhs[br - 1] = Complex64::new(1.0, 0.0);
        batched_rhs[size + sense_node - 1] = Complex64::new(1.0, 0.0);
        let mut batched_solution = Vec::new();
        try_reserve_stb_values(
            &mut batched_solution,
            batched_value_count,
            "STB batched solution",
        )?;

        for (frequency_index, &freq) in frequencies.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let omega = 2.0 * PI * freq;
            circuit
                .prepare_behavioral_small_signal_at_frequency(&dc_solution, freq)
                .map_err(SimulationError::Circuit)?;
            Self::try_fill_small_signal_matrix_with_vbic_delay_mode(
                &circuit,
                &mut ac_matrix,
                &dc_solution,
                omega,
                super::ac::SmallSignalAnalysisKind::Ac,
                true,
                true,
            )?;

            // Both Tian experiments share the same factorization and run as
            // one batched triangular solve.
            ac_matrix
                .solve_many_into(&batched_rhs, 2, &mut batched_solution)
                .map_err(SimulationError::Solver)?;
            let sol_v = &batched_solution[..size];
            let v1 = sol_v[sense_node - 1];
            let i1 = sol_v[br - 1];

            let sol_i = &batched_solution[size..];
            let v2 = sol_i[sense_node - 1];
            let i2 = sol_i[br - 1];

            // Tian: T = -1/(1 - 1/D) = D/(1 - D); D -> 0 (no loop) gives
            // T -> 0 without the intermediate division blowing up.
            let d = 2.0 * (i1 * v2 - v1 * i2) + v1 + i2;
            let denom = Complex64::new(1.0, 0.0) - d;
            let t = if denom.norm() < 1e-30 {
                Complex64::new(f64::INFINITY, 0.0)
            } else {
                d / denom
            };
            loop_gains.push(t);
            abort.observe_progress((frequency_index + 1) as f64 / frequencies.len() as f64);
        }

        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let analyzer = StbAnalyzer::new(config);
        let result = analyzer
            .analyze_preallocated_with_abort(&frequencies, &loop_gains, prepared_result, abort)
            .map_err(map_stb_analysis_error)?;

        Ok(StbAnalysisResult {
            frequencies,
            loop_gains,
            result,
            probe_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::stb::StbSweepType;

    const RESOURCE_LIMIT_DECK: &str = "STB retained-result resource limit\n\
         EAMP out 0 in 0 10\n\
         VPROBE out fb 0\n\
         RF fb in 10k\n\
         RIN in 0 1k\n\
         .END\n";

    fn resource_limit_config(compute_nyquist: bool) -> StbConfig {
        StbConfig::new()
            .with_sweep(10.0, 1.0e3, 4)
            .with_sweep_type(StbSweepType::Linear)
            .with_probe("VPROBE")
            .with_nyquist(compute_nyquist)
    }

    fn engine_with_result_limit(limit: usize) -> Engine {
        let mut config = crate::engine::SimulationConfig::default();
        config.resource_limits.max_result_values = limit;
        Engine::new(config)
    }

    fn actual_retained_result_values(result: &StbAnalysisResult) -> usize {
        result
            .frequencies
            .len()
            .checked_add(
                result
                    .loop_gains
                    .len()
                    .checked_mul(2)
                    .expect("loop-gain shape"),
            )
            .and_then(|count| {
                count.checked_add(
                    result
                        .result
                        .bode_points
                        .len()
                        .checked_mul(6)
                        .expect("Bode shape"),
                )
            })
            .and_then(|count| {
                count.checked_add(
                    result
                        .result
                        .nyquist_points
                        .len()
                        .checked_mul(3)
                        .expect("Nyquist shape"),
                )
            })
            .and_then(|count| count.checked_add(6))
            .expect("retained STB result shape")
    }

    #[test]
    fn stb_result_limit_accepts_the_exact_retained_value_count() {
        let netlist = Netlist::parse(RESOURCE_LIMIT_DECK).expect("STB resource deck parses");

        for compute_nyquist in [false, true] {
            let config = resource_limit_config(compute_nyquist);
            let point_count = config
                .frequency_point_count()
                .expect("valid STB point count");
            let exact_limit = stb_retained_result_value_count(point_count, compute_nyquist)
                .expect("small STB result shape is addressable");
            let result = engine_with_result_limit(exact_limit)
                .run_stb(&netlist, config)
                .expect("the exact retained-value limit must admit STB");

            assert_eq!(result.frequencies.len(), point_count);
            assert_eq!(result.loop_gains.len(), point_count);
            assert_eq!(result.result.bode_points.len(), point_count);
            assert_eq!(
                result.result.nyquist_points.len(),
                if compute_nyquist { point_count } else { 0 }
            );
            assert_eq!(actual_retained_result_values(&result), exact_limit);
        }
    }

    #[test]
    fn stb_result_limit_rejects_one_value_below_before_circuit_work() {
        let netlist = Netlist::parse(RESOURCE_LIMIT_DECK).expect("STB resource deck parses");

        for compute_nyquist in [false, true] {
            let config = resource_limit_config(compute_nyquist);
            let point_count = config
                .frequency_point_count()
                .expect("valid STB point count");
            let requested = stb_retained_result_value_count(point_count, compute_nyquist)
                .expect("small STB result shape is addressable");
            let limit = requested - 1;
            let abort = crate::abort_signal::CountingAbort::new(usize::MAX);

            let error = engine_with_result_limit(limit)
                .run_stb_with_abort(&netlist, config, &abort)
                .expect_err("one value below the exact requirement must fail");

            assert!(matches!(
                error,
                SimulationError::ResourceLimit(resource)
                    if resource.resource == crate::resource::ResourceKind::ResultValues
                        && resource.requested == requested
                        && resource.limit == limit
            ));
            assert_eq!(
                abort.count(),
                1,
                "the result budget must fail after the entry abort check and before circuit construction"
            );
        }
    }

    #[test]
    fn stb_retained_result_count_is_exact_at_boundary_and_rejects_overflow() {
        for compute_nyquist in [false, true] {
            let values_per_point = if compute_nyquist { 12 } else { 9 };
            let largest = (usize::MAX - STB_MARGIN_VALUES) / values_per_point;
            assert_eq!(
                stb_retained_result_value_count(largest, compute_nyquist),
                Ok(largest * values_per_point + STB_MARGIN_VALUES)
            );
            assert!(matches!(
                stb_retained_result_value_count(largest + 1, compute_nyquist),
                Err(StbAnalysisError::CapacityOverflow {
                    object: "STB retained-result value count"
                })
            ));
        }
    }

    #[test]
    fn stb_retained_result_overflow_fails_before_circuit_work() {
        let netlist = Netlist::parse(RESOURCE_LIMIT_DECK).expect("STB resource deck parses");
        let mut limits = crate::ResourceLimits::unlimited();
        limits.max_analysis_points = usize::MAX;
        limits.max_result_values = usize::MAX;
        let mut simulation_config = crate::engine::SimulationConfig::default();
        simulation_config.resource_limits = limits;
        let engine = Engine::new(simulation_config);
        let abort = crate::abort_signal::CountingAbort::new(usize::MAX);
        let error = engine
            .run_stb_with_abort(
                &netlist,
                StbConfig::new()
                    .with_sweep(1.0, 2.0, usize::MAX)
                    .with_sweep_type(StbSweepType::Linear)
                    .with_probe("VPROBE"),
                &abort,
            )
            .expect_err("overflowing retained STB shape must fail");

        assert!(matches!(
            error,
            SimulationError::Circuit(message)
                if message == "STB retained-result value count exceeds addressable capacity"
        ));
        assert_eq!(
            abort.count(),
            1,
            "retained shape must fail after entry cancellation and before circuit construction"
        );
    }

    #[test]
    fn stb_frequency_allocation_failure_precedes_circuit_work() {
        let netlist = Netlist::parse(RESOURCE_LIMIT_DECK).expect("STB resource deck parses");
        let mut simulation_config = crate::engine::SimulationConfig::default();
        simulation_config.resource_limits = crate::ResourceLimits::unlimited();
        let engine = Engine::new(simulation_config);
        let point_count = isize::MAX as usize / std::mem::size_of::<Value>() + 1;
        let abort = crate::abort_signal::CountingAbort::new(usize::MAX);
        let error = engine
            .run_stb_with_abort(
                &netlist,
                StbConfig::new()
                    .with_sweep(1.0, 2.0, point_count)
                    .with_sweep_type(StbSweepType::Linear)
                    .with_probe("VPROBE")
                    .with_nyquist(false),
                &abort,
            )
            .expect_err("unallocatable STB frequency grid must fail");

        assert!(matches!(
            error,
            SimulationError::Circuit(message)
                if message == format!(
                    "unable to allocate {point_count} elements for STB frequency grid"
                )
        ));
        assert_eq!(
            abort.count(),
            2,
            "allocation must fail during the pre-circuit frequency projection"
        );
    }

    #[test]
    fn stb_refreshes_frequency_dependent_behavioral_conductance() {
        let netlist = Netlist::parse_with_options(
            "live FREQ STB operator\n\
             .PARAM RUNTIME_R={FREQ}\n\
             EAMP out 0 in 0 10\n\
             VPROBE out fb 0\n\
             RF fb in {RUNTIME_R}\n\
             RIN in 0 1k\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect("frequency-dependent STB deck parses");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let config = StbConfig::new()
            .with_sweep(10.0, 100.0, 2)
            .with_sweep_type(StbSweepType::Linear)
            .with_probe("VPROBE");
        let result = engine
            .run_stb(&netlist, config)
            .expect("frequency-dependent STB operators solve");
        assert_eq!(result.loop_gains.len(), 2);
        assert!(
            (result.loop_gains[0] - result.loop_gains[1]).norm() > 1.0e-3,
            "STB operator retained a stale FREQ conductance: {:?}",
            result.loop_gains
        );
    }
}
