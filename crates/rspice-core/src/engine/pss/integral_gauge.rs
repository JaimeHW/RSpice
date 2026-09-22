//! Preserve free expression-integral constants in a consistent shooting step.
use super::*;

impl PssCircuit {
    /// Remove finite-difference noise only where circuit structure proves an
    /// integral increment independent of a shooting coordinate. Fixed voltage
    /// differences may float: an exact common-offset cancellation is enough.
    pub(super) fn certify_integral_sensitivities(
        &self,
        jacobian: &mut [Vec<Value>],
        subtract_identity: bool,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if self.physical_state_dimension() == self.state_dimension() {
            return Ok(());
        }
        let mut parent = (0..=self.num_nodes()).collect::<Vec<_>>();
        fn root(parent: &mut [usize], mut node: usize) -> usize {
            let mut representative = node;
            while parent[representative] != representative {
                representative = parent[representative];
            }
            while parent[node] != node {
                let next = parent[node];
                parent[node] = representative;
                node = next;
            }
            representative
        }
        for (&pos, &neg) in self
            .voltage_sources
            .node_pos
            .iter()
            .zip(&self.voltage_sources.node_neg)
        {
            let left = root(&mut parent, pos);
            let right = root(&mut parent, neg);
            parent[left] = right;
        }
        loop {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut changed = false;
            for index in 0..self.vcvs.len() {
                if self.vcvs.gains[index] == 0.0
                    || root(&mut parent, self.vcvs.ctrl_pos[index])
                        == root(&mut parent, self.vcvs.ctrl_neg[index])
                {
                    let left = root(&mut parent, self.vcvs.node_pos[index]);
                    let right = root(&mut parent, self.vcvs.node_neg[index]);
                    if left != right {
                        parent[left] = right;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
        for node in 0..parent.len() {
            root(&mut parent, node);
        }
        let ground = parent[0];
        let mut plans = self
            .behavioral_sources
            .prescribed_integral_rates()
            .map_err(SimulationError::Circuit)?;
        for expression in self.capacitors.value_expressions.iter().flatten() {
            expression
                .append_prescribed_integral_rates(&mut plans)
                .map_err(SimulationError::Circuit)?;
        }
        let physical = self.physical_state_dimension();
        let mut ancestors: Vec<Option<std::collections::BTreeSet<usize>>> = Vec::new();
        for (index, plan) in plans.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let dependencies = plan.dependencies_with_offsets(|coordinate| {
                if coordinate >= self.num_nodes() {
                    return Some(coordinate + 1);
                }
                let group = parent[coordinate + 1];
                (group != ground).then_some(group)
            });
            let known = dependencies.and_then(|dependencies| {
                let mut all = std::collections::BTreeSet::new();
                for dependency in dependencies {
                    all.insert(dependency);
                    all.extend(ancestors.get(dependency)?.as_ref()?.iter().copied());
                }
                Some(all)
            });
            if let Some(dependencies) = &known {
                for col in 0..self.state_dimension() {
                    if col < physical || !dependencies.contains(&(col - physical)) {
                        jacobian[physical + index][col] =
                            Value::from(!subtract_identity && col == physical + index);
                    }
                }
            }
            ancestors.push(known);
        }
        Ok(())
    }
}

impl Engine {
    pub(super) fn pss_solve_with_integral_constants(
        &self,
        circuit: &PssCircuit,
        state: &ShootingState,
        jacobian: &[Vec<Value>],
        rhs: &[Value],
        config: &PssConfig,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        match self.pss_solve_linear_system(jacobian, rhs) {
            Err(SimulationError::Solver(SolverError::SingularMatrix))
                if circuit.physical_state_dimension() < state.dimension() => {}
            result => return result,
        }
        let mut scales = state
            .x0
            .iter()
            .enumerate()
            .map(|(index, &value)| {
                circuit.perturbation_scale(
                    index,
                    value,
                    self.current_abstol() / self.voltage_abstol(),
                )
            })
            .collect::<Vec<_>>();
        // The autonomous period, when present, is a physical unknown and
        // must get a pivot. It is never eligible for an integral gauge.
        if rhs.len() == state.dimension() + 1 {
            scales.push(state.period);
        }
        solve_preserving_integral_constants(
            jacobian,
            rhs,
            &scales,
            circuit.physical_state_dimension()..state.dimension(),
            circuit.grid_steps(config),
            abort,
        )
    }
}

fn solve_preserving_integral_constants(
    a: &[Vec<Value>],
    b: &[Value],
    scales: &[Value],
    integrals: std::ops::Range<usize>,
    steps: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<Value>, SimulationError> {
    let n = b.len();
    let singular = || SimulationError::Solver(SolverError::SingularMatrix);
    if a.len() != n
        || scales.len() != n
        || integrals.end > n
        || integrals.start > integrals.end
        || a.iter()
            .any(|row| row.len() != n || row.iter().any(|v| !v.is_finite()))
        || b.iter().any(|v| !v.is_finite())
        || scales.iter().any(|v| !v.is_finite() || *v <= 0.)
    {
        return Err(singular());
    }
    let mut matrix = a
        .iter()
        .enumerate()
        .map(|(row, entries)| {
            entries
                .iter()
                .enumerate()
                .map(|(col, value)| value * scales[col] / scales[row])
                .chain(std::iter::once(-b[row] / scales[row]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    if matrix.iter().flatten().any(|value| !value.is_finite()) {
        return Err(SolverError::Overflow.into());
    }
    let column_scale = (0..n)
        .map(|col| {
            matrix
                .iter()
                .fold(0.0_f64, |value, row| value.max(row[col].abs()))
        })
        .collect::<Vec<_>>();
    let order = (0..n)
        .filter(|col| !integrals.contains(col))
        .chain(integrals.clone());
    let mut pivots = Vec::new();
    for col in order {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let rank = pivots.len();
        let pivot = (rank..n)
            .max_by(|&left, &right| matrix[left][col].abs().total_cmp(&matrix[right][col].abs()));
        let Some(pivot) = pivot else {
            break;
        };
        if matrix[pivot][col].abs() <= 8.0 * n as Value * Value::EPSILON * column_scale[col] {
            if !integrals.contains(&col) {
                return Err(singular());
            }
            continue; // Keep this free integral's initial value: its update is zero.
        }
        matrix.swap(rank, pivot);
        let divisor = matrix[rank][col];
        for value in &mut matrix[rank] {
            *value /= divisor;
        }
        for row in rank + 1..n {
            let factor = matrix[row][col];
            for column in 0..=n {
                matrix[row][column] -= factor * matrix[rank][column];
            }
            matrix[row][col] = 0.0;
        }
        pivots.push(col);
    }
    let mut scaled = vec![0.0; n];
    for (row, &col) in pivots.iter().enumerate().rev() {
        scaled[col] = matrix[row][n]
            - (0..n)
                .filter(|&index| index != col)
                .map(|index| matrix[row][index] * scaled[index])
                .sum::<Value>();
    }
    let solution = scaled
        .iter()
        .zip(scales)
        .map(|(value, scale)| value * scale)
        .collect::<Vec<_>>();
    if solution.iter().any(|value| !value.is_finite()) {
        return Err(SolverError::Overflow.into());
    }
    // Certify ALL original equations, including redundant rows. Allow only
    // roundoff from the accepted period integration and linear arithmetic,
    // never the user's nonlinear tolerance or a regularizing conductance.
    let roundoff = 8.0 * steps.max(n).max(1) as Value * Value::EPSILON;
    for (row, entries) in a.iter().enumerate() {
        let mut residual = b[row] / scales[row];
        let mut magnitude = residual.abs();
        for (&entry, &value) in entries.iter().zip(&solution) {
            let term = entry * value / scales[row];
            residual += term;
            magnitude += term.abs();
        }
        let integration_roundoff = if integrals.contains(&row) { 1.0 } else { 0.0 };
        if !residual.is_finite()
            || !magnitude.is_finite()
            || residual.abs() > roundoff * (magnitude + integration_roundoff)
        {
            return Err(singular());
        }
    }
    Ok(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integral_gauge_preserves_constants_without_regularizing_physical_state_or_drift() {
        // x is physical, u integrates a prescribed zero-mean input, z integrates
        // u. Closure fixes u, leaves z free, and still has to correct x.
        let matrix = vec![vec![-1., 0., 2.], vec![0., 0., 0.], vec![0., 1., 0.]];
        let solve = |rhs: &[Value], integrals| {
            solve_preserving_integral_constants(
                &matrix,
                rhs,
                &[1., 1., 1.],
                integrals,
                256,
                &NoAbort,
            )
        };
        assert_eq!(solve(&[3., 0., 4.], 1..3).unwrap(), vec![3., -4., 0.]);
        assert!(solve(&[3., 1e-10, 4.], 1..3).is_err());
        assert!(solve(&[3., 0., 4.], 3..3).is_err());
        let tiny = vec![vec![1e-18, 1.], vec![0., 0.]];
        let step =
            solve_preserving_integral_constants(&tiny, &[-1., 0.], &[1., 1.], 1..2, 1, &NoAbort)
                .unwrap();
        assert!((step[0] - 1e18).abs() < 256.);
        assert_eq!(step[1], 0.);
    }
}
