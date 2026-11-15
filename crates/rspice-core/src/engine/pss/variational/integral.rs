//! The recurrence with generated `idt` sites carried as coordinates.
//!
//! Each carried integral `y` is integrated by the step's own companion rule
//! from `ẏ = u(x, y)`, as a charge is from its current, so it enters the
//! recurrence as one more stored quantity: the step's matrix gains, per
//! integral, a column of the model rows' partials `∂F/∂y` and a row
//! `(g/h)·δy − ∂u/∂x·δx − ∂u/∂y·δy` equal to the integral's history, and the
//! integral's rate in the history is its integrand's perturbation.
//!
//! A period starts with every integral held at its coordinate (see
//! `PssGeneratedIntegrals`), so the start's solution moves with an integral's
//! coordinate as well as with a charge's: each start perturbation solves the
//! consistency system, the static operator with every charge coordinate
//! pinned, for a unit move of one coordinate, and an integral's history
//! starts at its coordinate with its integrand's perturbation as the previous
//! input, as the model's own history does.

use super::{Step, propagate};
use crate::Value;
use crate::abort_signal::AbortSignal;
use crate::engine::pss::{PssCircuit, PssJacobianProbe, PssStateTrace};
use crate::engine::{Engine, SimulationError};
use crate::solver::{FactorizationRequest, SolverOptions, StaticMatrix};
use std::collections::HashMap;

/// The recorded traversal the recurrence runs on: its trace, each point's
/// static and reactive operator on the circuit's stored positions, those
/// positions, and the step matrices' solver options.
pub(super) type Traversal<'a> = (
    &'a PssStateTrace,
    &'a [(Vec<Value>, Vec<Value>)],
    &'a [(usize, usize)],
    SolverOptions,
);

impl Engine {
    /// The monodromy over physical and integral coordinates, or `None`
    /// where the recurrence cannot state it.
    pub(super) fn pss_integral_monodromy(
        &self,
        circuit: &PssCircuit,
        x0: &[Value],
        probe: PssJacobianProbe<'_>,
        (trace, parts, positions, options): Traversal<'_>,
        readout: &[(usize, usize)],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Vec<Value>>>, SimulationError> {
        let n = circuit.matrix_size();
        let physical = readout.len();
        let carried = circuit.generated_integrals.dimension();
        let size = n + carried;
        if trace.states.len() != trace.solutions.len()
            || trace
                .states
                .iter()
                .any(|state| state.len() != physical + carried)
        {
            return Ok(None);
        }
        let points = trace
            .solutions
            .iter()
            .zip(&trace.states)
            .map(|(solution, state)| (solution.as_slice(), &state[physical..]));
        let Some(couplings) = circuit
            .generated_integrals
            .couplings(&circuit.circuit, points)?
        else {
            return Ok(None);
        };

        // The step matrices' pattern: the circuit's, each integral's column
        // and row, and the integrals' block.
        let mut pattern: Vec<(usize, usize, Value)> = positions
            .iter()
            .map(|&(row, col)| (row, col, 0.0))
            .collect();
        for coupling in &couplings {
            pattern.extend(coupling.input.iter().map(|&(i, col, _)| (n + i, col, 0.0)));
            pattern.extend(coupling.output.iter().map(|&(row, i, _)| (row, n + i, 0.0)));
        }
        for i in 0..carried {
            for j in 0..carried {
                pattern.push((n + i, n + j, 0.0));
            }
        }
        let template = StaticMatrix::from_triplets_with_options(size, size, &pattern, options)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "the period map's recurrence has no step matrix: {error}"
                ))
            })?;
        let augmented: Vec<(usize, usize)> = template.stored_positions().collect();
        let place: HashMap<(usize, usize), usize> = augmented
            .iter()
            .enumerate()
            .map(|(index, &position)| (position, index))
            .collect();
        let original: Vec<usize> = positions.iter().map(|position| place[position]).collect();
        let diagonal: Vec<usize> = (0..carried).map(|i| place[&(n + i, n + i)]).collect();
        let reactive = |c: &[Value]| {
            let mut values = vec![0.0; augmented.len()];
            for (&index, &value) in original.iter().zip(c) {
                values[index] = value;
            }
            for &index in &diagonal {
                values[index] = 1.0;
            }
            values
        };

        let mut steps = Vec::with_capacity(trace.steps.len());
        for ((&(h, coefficients), (g, c)), coupling) in
            trace.steps.iter().zip(&parts[1..]).zip(&couplings[1..])
        {
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
            let values = a.values_mut();
            values.fill(0.0);
            for ((&index, g), c) in original.iter().zip(g).zip(c) {
                values[index] = weights.0 * c + g;
            }
            for &(i, col, partial) in &coupling.input {
                values[place[&(n + i, col)]] -= partial;
            }
            for &(row, i, partial) in &coupling.output {
                values[place[&(row, n + i)]] += partial;
            }
            for &(i, j, partial) in &coupling.cross {
                values[place[&(n + i, n + j)]] -= partial;
            }
            for &index in &diagonal {
                values[index] += weights.0;
            }
            Self::stamp_nodal_gmin(&circuit.circuit, &mut a, 0.0);
            steps.push(Step {
                a,
                c: reactive(c),
                weights,
            });
        }

        // Each coordinate's start: the consistency system, the static
        // operator with every charge coordinate pinned, moved by one unit of
        // the coordinate, and each integral's integrand there.
        let start = &couplings[0];
        let mut consistency: Vec<(usize, usize, Value)> = positions
            .iter()
            .zip(&parts[0].0)
            .map(|(&(row, col), &g)| (row, col, g))
            .collect();
        for (index, &(pos, neg)) in readout.iter().enumerate() {
            for (node, sign) in [(pos, 1.0), (neg, -1.0)] {
                if let Some(row) = node.checked_sub(1) {
                    consistency.push((row, n + index, sign));
                    consistency.push((n + index, row, sign));
                }
            }
        }
        let mut pinned = StaticMatrix::from_triplets(n + physical, n + physical, &consistency)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "the period map's recurrence has no consistency matrix: {error}"
                ))
            })?;
        Self::stamp_nodal_gmin(&circuit.circuit, &mut pinned, 0.0);
        let columns = physical + carried;
        let mut rhs = vec![0.0; (n + physical) * columns];
        for j in 0..physical {
            rhs[j * (n + physical) + n + j] = 1.0;
        }
        for &(row, i, partial) in &start.output {
            rhs[(physical + i) * (n + physical) + row] -= partial;
        }
        let mut moved = vec![0.0; rhs.len()];
        if pinned
            .solve_many_into_with_factorization(
                &rhs,
                columns,
                &mut moved,
                FactorizationRequest::Automatic,
            )
            .is_err()
        {
            return Ok(None);
        }
        let mut starts = Vec::with_capacity(columns);
        let mut rates = Vec::with_capacity(columns);
        for j in 0..columns {
            let mut perturbation = vec![0.0; size];
            perturbation[..n].copy_from_slice(&moved[j * (n + physical)..j * (n + physical) + n]);
            if j >= physical {
                perturbation[n + j - physical] = 1.0;
            }
            let mut rate = vec![0.0; size];
            for &(i, col, partial) in &start.input {
                rate[n + i] += partial * perturbation[col];
            }
            for &(i, k, partial) in &start.cross {
                rate[n + i] += partial * perturbation[n + k];
            }
            starts.push(perturbation);
            rates.push(rate);
        }
        let c0 = reactive(&parts[0].1);
        let Some(images) = propagate(size, &augmented, &c0, &starts, &rates, &mut steps, abort)?
        else {
            return Ok(None);
        };
        let read = |image: &[Value], (pos, neg): (usize, usize)| {
            let at = |row: usize| row.checked_sub(1).map_or(0.0, |index| image[index]);
            at(pos) - at(neg)
        };
        let mut phi = vec![vec![0.0; columns]; columns];
        for (j, image) in images.iter().enumerate() {
            for (i, &pair) in readout.iter().enumerate() {
                phi[i][j] = read(image, pair);
            }
            for i in 0..carried {
                phi[physical + i][j] = image[n + i];
            }
        }
        if phi.iter().flatten().any(|value| !value.is_finite()) {
            return Ok(None);
        }
        Ok(self
            .pss_variational_confirmed(circuit, x0, probe, &phi, abort)?
            .then_some(phi))
    }
}
