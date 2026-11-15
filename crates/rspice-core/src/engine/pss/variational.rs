//! Native shooting's exact period-map Jacobian, by the variational recurrence
//! of one traversal's accepted steps.
//!
//! A step of length `h` to `t_k` on the companion coefficients
//! `(g, v₁, v₂, i₁)` solves `f(x_k) + Q̇_k + s(t_k) = 0` with
//! `Q̇_k = (g·Q_k − v₁·Q_{k−1} − v₂·Q_{k−2})/h − i₁·Q̇_{k−1}`. Its derivative at
//! the accepted solution is the recurrence
//!
//! - `(g/h·C_k + G_k)·δx_k = (v₁·δQ_{k−1} + v₂·δQ_{k−2})/h + i₁·δQ̇_{k−1}`,
//! - `δQ_k = C_k·δx_k` and `δQ̇_k = g/h·δQ_k − (the right-hand side)`,
//!
//! with `G_k` and `C_k` the small-signal operator's two parts at the accepted
//! point, `G_k` with the GMIN floor every step stamps on a node only GMIN
//! holds. A period starts as shooting starts it: every charge is taken from
//! the start's solution with no companion current, `δQ_0 = δQ_{−1} = C_0·δx_0`
//! and `δQ̇_0 = 0`, where `δx_0` is a coordinate's perturbation through the
//! spanning forest.
//!
//! Finite differences of the period map cannot resolve a Floquet multiplier
//! within the map's noise of 1: a floating body that charges through a
//! junction's leakage gives `1 − λ` of 1e-5 and less, and every column of
//! `Φ − I` along it is noise. The recurrence is that derivative to rounding.
//!
//! **Scope.** It is the map's exact derivative where every charge the circuit
//! stores is a function of the solution the step integrates on its own
//! companion coefficients: capacitors and generated compact models, whose
//! `ddt` operands the forest's coordinates fix, and junction diodes, whose
//! bias and charge the period's start resets from its solution, beside
//! resistive devices and sources. A circuit with a native device whose charge
//! history the start is not shown to re-take (a BJT, JFET, MOSFET or
//! BSIM/EKV/SOI/VDMOS model, a magnetic core, a memristor, an XSPICE model),
//! an inductor, a capacitor expression, a switch, a line, a
//! behavioral source or a runtime Verilog-A module, or a basis with a winding
//! current, an integral, a delayed wave, a descriptor or amplifier
//! constraints, keeps the central-difference columns, as does one whose
//! recurrence a seeded direction of the map does not confirm or whose step
//! matrix does not factor.

use super::{PssCircuit, PssJacobianProbe, PssStateTrace, PssTraversal};
use crate::abort_signal::AbortSignal;
use crate::engine::ac::SmallSignalAnalysisKind;
use crate::engine::{Engine, SimulationError};
use crate::solver::{
    ComplexMatrix, FactorizationRequest, NumericFactorizationPolicy, StaticMatrix,
};
use crate::{CircuitData, Value};

/// How many columns go through the steps together.
const COLUMNS_PER_BLOCK: usize = 32;

/// How far a seeded direction of the map may differ from the recurrence's
/// image of it, against the direction's size: the finite difference's own
/// error is far below it, and a device the recurrence misreads far above.
const CONFIRMATION_TOLERANCE: Value = 1e-3;

/// One step of the recurrence.
struct Step {
    a: StaticMatrix,
    /// `C_k` on the structure's stored positions.
    c: Vec<Value>,
    /// `g/h`, `v₁/h`, `v₂/h` and `i₁`.
    weights: (Value, Value, Value, Value),
}

/// Whether the recurrence can state this circuit's period map: no device
/// keeps a charge between steps except as a function of the solution the
/// step integrates.
fn admits_variational(circuit: &CircuitData) -> bool {
    let capacitors = (0..circuit.capacitors.stamps.len())
        .all(|index| circuit.capacitors.value_expression(index).is_none());
    #[cfg(feature = "veriloga")]
    let runtime = !circuit.has_veriloga_devices() && !circuit.has_mixed_signal_hosts();
    #[cfg(not(feature = "veriloga"))]
    let runtime = true;
    capacitors
        && runtime
        && circuit.shooting_takes_every_native_charge_from_its_start()
        && circuit.inductors.is_empty()
        && circuit.tlines.is_empty()
        && circuit.vswitches.is_empty()
        && circuit.iswitches.is_empty()
        && circuit.behavioral_sources.is_empty()
}

impl Engine {
    /// Whether the recurrence states this driven circuit's period map.
    pub(super) fn pss_admits_variational(
        &self,
        circuit: &PssCircuit,
        config: &crate::analysis::PssConfig,
    ) -> bool {
        !config.is_autonomous()
            && self.config.spice_dialect != crate::SpiceDialect::Xyce
            && admits_variational(circuit)
            && circuit.charge_coordinate_maps().is_some()
    }

    /// The monodromy `Φ` over the shooting coordinates at `x0`, by the
    /// variational recurrence of one traversal, or `None` where it is not
    /// the period map's exact derivative.
    pub(super) fn pss_variational_monodromy(
        &self,
        circuit: &PssCircuit,
        x0: &[Value],
        probe: PssJacobianProbe<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Vec<Value>>>, SimulationError> {
        let PssJacobianProbe { period, config, .. } = probe;
        if !self.pss_admits_variational(circuit, config) {
            return Ok(None);
        }
        let Some((starts, readout)) = circuit.charge_coordinate_maps() else {
            return Ok(None);
        };
        let mut traversed = circuit.clone();
        let structure = self.build_matrix(&traversed)?;
        let mut matrix = self.build_matrix(&traversed)?;
        traversed.link_indices(&matrix);
        self.pss_set_reactive_state(&mut traversed, x0)?;
        let seed = self.pss_initial_node_solution(&mut traversed, abort)?;
        let mut trace = PssStateTrace::default();
        self.pss_run_tran_internal(
            &mut traversed,
            &mut matrix,
            seed,
            PssTraversal {
                tstop: period,
                max_step: period / circuit.grid_steps(config) as Value,
                fixed_grid: true,
                integration_method: config.integration_method,
                retain_waveform: false,
            },
            Some(&mut trace),
            abort,
        )?;
        if trace.solutions.len() < 2 || trace.steps.len() + 1 != trace.solutions.len() {
            return Ok(None);
        }
        let n = circuit.matrix_size();
        let positions: Vec<(usize, usize)> = structure.stored_positions().collect();
        let mut linearized = circuit.circuit.clone();
        linearized.link_indices(&structure);
        let mut operator = ComplexMatrix::from_real_structure(&structure);
        let mut parts = Vec::with_capacity(trace.solutions.len());
        for solution in &trace.solutions {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            Self::prepare_small_signal_state(&mut linearized, solution)?;
            Self::try_fill_small_signal_matrix_with_vbic_delay_mode(
                &linearized,
                &mut operator,
                solution,
                1.0,
                SmallSignalAnalysisKind::Ac,
                true,
                true,
            )?;
            let mut g = Vec::with_capacity(positions.len());
            let mut c = Vec::with_capacity(positions.len());
            operator.for_each_stored(|_, _, value| {
                g.push(value.re);
                c.push(value.im);
            });
            if g.len() != positions.len() {
                return Ok(None);
            }
            parts.push((g, c));
        }
        let mut options = structure.solver_options();
        options.numeric_factorization = NumericFactorizationPolicy::ReusePivotSequence;
        let pattern: Vec<(usize, usize, Value)> = positions
            .iter()
            .map(|&(row, col)| (row, col, 0.0))
            .collect();
        let template =
            StaticMatrix::from_triplets_with_options(n, n, &pattern, options).map_err(|error| {
                SimulationError::Circuit(format!(
                    "the period map's recurrence has no step matrix: {error}"
                ))
            })?;
        if !template.stored_positions().eq(positions.iter().copied()) {
            return Ok(None);
        }
        let mut steps = Vec::with_capacity(trace.steps.len());
        for (&(h, coefficients), (g, c)) in trace.steps.iter().zip(&parts[1..]) {
            if !h.is_finite() || h <= 0.0 {
                return Ok(None);
            }
            let weights = (
                coefficients.coeff_g / h,
                coefficients.coeff_v_n / h,
                if coefficients.needs_two_history {
                    coefficients.coeff_v_n_minus_1 / h
                } else {
                    0.0
                },
                coefficients.coeff_i_n,
            );
            let mut a = template.clone_structure();
            for ((value, g), c) in a.values_mut().iter_mut().zip(g).zip(c) {
                *value = weights.0 * c + g;
            }
            // The floor every step's assembly stamps on a node only GMIN
            // holds, which the small-signal operator leaves out.
            Self::stamp_nodal_gmin(&circuit.circuit, &mut a, 0.0);
            steps.push(Step {
                a,
                c: c.clone(),
                weights,
            });
        }
        let starts: Vec<Vec<Value>> = starts
            .into_iter()
            .map(|start| start[1..].iter().copied().take(n).collect())
            .collect();
        let Some(images) = propagate(n, &positions, &parts[0].1, &starts, &mut steps, abort)?
        else {
            return Ok(None);
        };
        let read = |image: &[Value], (pos, neg): (usize, usize)| {
            let at = |row: usize| row.checked_sub(1).map_or(0.0, |index| image[index]);
            at(pos) - at(neg)
        };
        let dimension = readout.len();
        let mut phi = vec![vec![0.0; dimension]; dimension];
        for (j, image) in images.iter().enumerate() {
            for (i, &pair) in readout.iter().enumerate() {
                phi[i][j] = read(image, pair);
            }
        }
        if phi.iter().flatten().any(|value| !value.is_finite()) {
            return Ok(None);
        }
        Ok(self
            .pss_variational_confirmed(circuit, x0, probe, &phi, abort)?
            .then_some(phi))
    }

    /// Whether the period map's central difference along one seeded
    /// direction agrees with the recurrence's image of it.
    fn pss_variational_confirmed(
        &self,
        circuit: &PssCircuit,
        x0: &[Value],
        probe: PssJacobianProbe<'_>,
        phi: &[Vec<Value>],
        abort: &dyn AbortSignal,
    ) -> Result<bool, SimulationError> {
        let n = x0.len();
        // Unequal weights keep a direction off any symmetry of the circuit.
        let direction: Vec<Value> = (0..n)
            .map(|index| {
                let weight = 1.0 + (index % 7) as Value / 7.0;
                weight
                    * circuit.perturbation_scale(
                        index,
                        x0[index],
                        self.current_abstol() / self.voltage_abstol(),
                        self.config.convergence_config.charge_abstol / self.voltage_abstol(),
                    )
            })
            .collect();
        let mut worker = circuit.clone();
        let mut matrix = self.build_matrix(&worker)?;
        worker.link_indices(&matrix);
        let measured = self.pss_directional_jacobian_product(
            &mut worker,
            &mut matrix,
            x0,
            probe,
            &direction,
            abort,
        )?;
        let size = direction
            .iter()
            .fold(0.0, |norm: Value, value| norm.hypot(*value));
        let difference = phi
            .iter()
            .zip(&measured)
            .zip(&direction)
            .map(|((row, &measured), &own)| {
                let image: Value = row.iter().zip(&direction).map(|(a, b)| a * b).sum();
                image - own - measured
            })
            .fold(0.0, |norm: Value, value| norm.hypot(value));
        Ok(difference <= CONFIRMATION_TOLERANCE * size)
    }
}

/// `y += M·x` for the operator `M` on `positions`.
fn multiply(positions: &[(usize, usize)], values: &[Value], x: &[Value], y: &mut [Value]) {
    for (&(row, col), &value) in positions.iter().zip(values) {
        if value != 0.0 {
            y[row] += value * x[col];
        }
    }
}

/// The end solution's perturbation for each start perturbation, through
/// every step, a block of columns at a time; `None` where a step's matrix
/// does not factor, which the step's own Newton matrix did.
fn propagate(
    n: usize,
    positions: &[(usize, usize)],
    c0: &[Value],
    starts: &[Vec<Value>],
    steps: &mut [Step],
    abort: &dyn AbortSignal,
) -> Result<Option<Vec<Vec<Value>>>, SimulationError> {
    let mut images = Vec::with_capacity(starts.len());
    let mut charge = Vec::new();
    let mut earlier = Vec::new();
    let mut rate = Vec::new();
    let mut rhs = Vec::new();
    let mut solved = Vec::new();
    for chunk in starts.chunks(COLUMNS_PER_BLOCK) {
        let count = chunk.len();
        charge.clear();
        charge.resize(n * count, 0.0);
        for (j, start) in chunk.iter().enumerate() {
            multiply(positions, c0, start, &mut charge[j * n..(j + 1) * n]);
        }
        earlier.clone_from(&charge);
        rate.clear();
        rate.resize(n * count, 0.0);
        solved.clear();
        solved.resize(n * count, 0.0);
        if steps.is_empty() {
            for start in chunk {
                images.push(start.clone());
            }
            continue;
        }
        for step in steps.iter_mut() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let (g, v1, v2, i1) = step.weights;
            rhs.clear();
            rhs.extend(
                charge
                    .iter()
                    .zip(&earlier)
                    .zip(&rate)
                    .map(|((q1, q2), qdot)| v1 * q1 + v2 * q2 + i1 * qdot),
            );
            if step
                .a
                .solve_many_into_with_factorization(
                    &rhs,
                    count,
                    &mut solved,
                    FactorizationRequest::Automatic,
                )
                .is_err()
            {
                return Ok(None);
            }
            std::mem::swap(&mut earlier, &mut charge);
            charge.fill(0.0);
            for j in 0..count {
                let range = j * n..(j + 1) * n;
                multiply(
                    positions,
                    &step.c,
                    &solved[range.clone()],
                    &mut charge[range.clone()],
                );
                for ((qdot, q), b) in rate[range.clone()]
                    .iter_mut()
                    .zip(&charge[range.clone()])
                    .zip(&rhs[range])
                {
                    *qdot = g * q - b;
                }
            }
        }
        for j in 0..count {
            images.push(solved[j * n..(j + 1) * n].to_vec());
        }
    }
    Ok(Some(images))
}
