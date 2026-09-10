//! Implicit, memoryless constitutive islands in the exact descriptor.
//!
//! Eliminating the linear equations gives v = b(t) + Q*j, j = f(v).
//! Only ports independent of shooting states and of j' reach this module.
//! Each feedback component must have a negative-semidefinite symmetric part
//! of Q. Together with the model's global monotone C1 certificate this proves
//! uniqueness and excludes folds; a sampled Newton Jacobian cannot do that.

use super::*;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
struct Plan {
    ports: Vec<Vec<(ForestValue, Value)>>,
    response: Vec<Value>,
    rates: Vec<bool>,
}

#[derive(Debug, Clone, Default)]
struct Sample {
    voltage: Vec<Value>,
    voltage_rate: Vec<Value>,
    current: Vec<[Value; 2]>,
}

#[derive(Debug, Clone)]
pub(super) struct NonlinearForcing {
    plan: Arc<Plan>,
    initial: Arc<Sample>,
    samples: [(Value, Arc<Sample>); 3],
}

fn sum(terms: impl Iterator<Item = (Value, Value)> + Clone) -> Result<Value, SimulationError> {
    rspice_veriloga_runtime::arithmetic::sum_products(terms).map_err(|_| precision_error())
}

// Compare nonnegative residual/scale pairs without overflowing a quotient
// or rounding a subnormal normalized residual to zero.
fn relative_less(left: (Value, Value), right: (Value, Value)) -> bool {
    use rspice_veriloga_runtime::arithmetic::ScaledValue as S;
    let difference = S::product_sum(
        S::new(left.0),
        S::new(right.1),
        S::new(-right.0),
        S::new(left.1),
    );
    !difference.is_zero() && difference.binary64().is_sign_negative()
}

fn poll(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

impl NonlinearForcing {
    pub(super) fn new(
        mut ports: Vec<Vec<(ForestValue, Value)>>,
        orders: &mut BTreeMap<(ConstraintSource, usize), usize>,
        retained: &mut usize,
        maximum: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>, SimulationError> {
        let n = ports.len();
        // First constitutive derivatives are C0 at the model's C1 joins.
        // Higher descriptor indices need model Taylor jets and branch proofs.
        if orders
            .iter()
            .any(|(&(kind, _), &order)| kind.is_nonlinear() && order > 1)
        {
            return Ok(None);
        }
        *retained = retained
            .saturating_add(n.saturating_mul(n))
            .saturating_add(n.saturating_mul(32))
            .saturating_add(128);
        PssVoltageConstraintBuilder::ensure_words(*retained, maximum)?;
        let available = maximum.saturating_sub(*retained);
        let mut response = vec![0.0; n * n];
        let mut rates = vec![false; n];
        for (row, port) in ports.iter_mut().enumerate() {
            poll(abort)?;
            rates[row] = [ConstraintSource::Diode, ConstraintSource::DiodeVoltage]
                .into_iter()
                .any(|kind| orders.get(&(kind, row)).is_some_and(|&order| order != 0));
            port.retain(|&(value, weight)| {
                if let Some((ConstraintSource::Diode, col, _)) = value.source() {
                    response[row * n + col] = weight;
                    false
                } else {
                    true
                }
            });
        }
        // A requested current rate needs all of its upstream input rates.
        let mut pending: Vec<_> = (0..n).filter(|&index| rates[index]).collect();
        while let Some(row) = pending.pop() {
            poll(abort)?;
            for col in 0..n {
                if response[row * n + col] != 0.0 && !rates[col] {
                    rates[col] = true;
                    pending.push(col);
                }
            }
        }
        if !passive_feedback(&response, n, available, abort)? {
            return Ok(None);
        }
        let initial = Arc::new(Sample::default());
        let result = Self {
            plan: Arc::new(Plan {
                ports,
                response,
                rates,
            }),
            samples: std::array::from_fn(|_| (0.0, initial.clone())),
            initial,
        };
        result.ensure_work(available)?;
        result.extend_orders(orders, abort)?;
        Ok(Some(result))
    }

    pub(super) fn extend_orders(
        &self,
        orders: &mut BTreeMap<(ConstraintSource, usize), usize>,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        for (row, port) in self.plan.ports.iter().enumerate() {
            poll(abort)?;
            for &(value, _) in port {
                let (kind, index, order) = value.source().ok_or_else(precision_error)?;
                let order = order
                    .checked_add(usize::from(self.plan.rates[row]))
                    .ok_or_else(precision_error)?;
                let maximum = orders.entry((kind, index)).or_default();
                *maximum = (*maximum).max(order);
            }
        }
        Ok(())
    }

    fn ensure_work(&self, available: usize) -> Result<(), SimulationError> {
        let n = self.plan.ports.len();
        if n == 0 {
            return Ok(());
        }
        // Triplets, sparse factors, extended recovery, trial vectors, sample
        // replacement, and the shared bounded exact dot-product workspace.
        let terms = self.plan.ports.iter().map(Vec::len).max().unwrap_or(0);
        PssVoltageConstraintBuilder::ensure_words(
            n.saturating_mul(n)
                .saturating_mul(96)
                .saturating_add(n.saturating_mul(128))
                .saturating_add(terms.saturating_mul(2))
                .saturating_add(32_768),
            available,
        )
    }

    pub(super) fn initialize(
        &mut self,
        circuit: &CircuitData,
        behavioral: &BehavioralForcing,
        available: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if self.plan.ports.is_empty() {
            return Ok(());
        }
        self.initial = Arc::new(self.evaluate(circuit, behavioral, 0.0, &[], available, abort)?);
        self.samples = std::array::from_fn(|_| (0.0, self.initial.clone()));
        Ok(())
    }

    pub(super) fn prepare(
        &mut self,
        circuit: &CircuitData,
        behavioral: &BehavioralForcing,
        times: [Value; 3],
        available: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if self.plan.ports.is_empty() {
            return Ok(());
        }
        let mut samples = std::array::from_fn(|_| (0.0, self.initial.clone()));
        for (index, &time) in times.iter().enumerate() {
            poll(abort)?;
            let values = if time == 0.0 {
                self.initial.clone()
            } else if let Some((_, values)) = self
                .samples
                .iter()
                .chain(&samples[..index])
                .find(|(sample_time, _)| *sample_time == time)
            {
                values.clone()
            } else {
                Arc::new(self.evaluate(
                    circuit,
                    behavioral,
                    time,
                    &self.samples[0].1.voltage,
                    available,
                    abort,
                )?)
            };
            samples[index] = (time, values);
        }
        self.samples = samples;
        Ok(())
    }

    pub(super) fn value(
        &self,
        value: ForestValue,
        time: Value,
        extra: usize,
    ) -> Result<Value, SimulationError> {
        let Some((kind, index, order)) = value.source().filter(|(kind, _, _)| kind.is_nonlinear())
        else {
            return Err(precision_error());
        };
        let sample = if time == 0.0 {
            Some(&self.initial)
        } else {
            self.samples
                .iter()
                .find(|(sample_time, _)| *sample_time == time)
                .map(|(_, sample)| sample)
        };
        order
            .checked_add(extra)
            .and_then(|order| {
                sample.and_then(|sample| match (kind, order) {
                    (ConstraintSource::DiodeVoltage, 0) => sample.voltage.get(index),
                    (ConstraintSource::DiodeVoltage, 1) => sample.voltage_rate.get(index),
                    (ConstraintSource::Diode, _) => sample
                        .current
                        .get(index)
                        .and_then(|current| current.get(order)),
                    _ => None,
                })
            })
            .copied()
            .ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "PSS nonlinear constraint value was not prepared at t={time:e}"
                ))
            })
    }

    fn evaluate(
        &self,
        circuit: &CircuitData,
        behavioral: &BehavioralForcing,
        time: Value,
        guess: &[Value],
        available: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Sample, SimulationError> {
        self.ensure_work(available)?;
        let n = self.plan.ports.len();
        let mut terms = Vec::with_capacity(self.plan.ports.iter().map(Vec::len).max().unwrap_or(0));
        let mut input = vec![0.0; n];
        let mut input_rate = vec![0.0; n];
        for (row, port) in self.plan.ports.iter().enumerate() {
            poll(abort)?;
            let mut forcing = |extra| {
                evaluate_form(port, &mut terms, |value| {
                    if value
                        .source()
                        .is_some_and(|(kind, _, _)| kind.is_behavioral())
                    {
                        behavioral.value(value, time, extra)
                    } else {
                        value.forcing(circuit, time, extra)
                    }
                })
            };
            input[row] = forcing(0)?;
            if self.plan.rates[row] {
                input_rate[row] = forcing(1)?;
            }
        }
        // Every admitted constitutive law has f(0)=0. With zero forcing,
        // passivity/uniqueness makes zero the exact solution, even when the
        // previous sample was far from zero.
        let mut voltage = if guess.len() == n && input.iter().any(|&value| value != 0.0) {
            guess.to_vec()
        } else {
            vec![0.0; n]
        };
        let mut current = vec![[0.0; 2]; n];
        let mut conductance = vec![0.0; n];
        let mut residual = vec![0.0; n];
        let mut line_scale = vec![0.0; n];
        let evaluate = |voltage: &[Value],
                        current: &mut [[Value; 2]],
                        conductance: &mut [Value],
                        residual: &mut [Value]| {
            for (index, diode) in circuit.diodes.devices.iter().enumerate() {
                poll(abort)?;
                let (value, slope) = diode.stamped_current_and_conductance(voltage[index]);
                if !value.is_finite() || !slope.is_finite() || slope < 0.0 {
                    return Err(precision_error());
                }
                current[index][0] = value;
                conductance[index] = slope;
            }
            let mut norm = (0.0, 1.0);
            for row in 0..n {
                poll(abort)?;
                residual[row] = sum([(voltage[row], 1.0), (input[row], -1.0)].into_iter().chain(
                    (0..n).map(|col| (-self.plan.response[row * n + col], current[col][0])),
                ))?;
                // A volt-scale absolute floor is unsafe: a dependent source
                // may amplify a tiny control voltage into a full-scale output.
                // Zero equations are exact; every nonzero row is relative to
                // its own voltage scale, including subnormal voltages.
                if residual[row] != 0.0 {
                    let row_norm = (
                        residual[row].abs(),
                        input[row].abs().max(voltage[row].abs()),
                    );
                    if relative_less(norm, row_norm) {
                        norm = row_norm;
                    }
                }
            }
            Ok::<_, SimulationError>(norm)
        };
        let mut converged = false;
        for _ in 0..100 {
            let norm = evaluate(&voltage, &mut current, &mut conductance, &mut residual)?;
            if !relative_less((64.0 * Value::EPSILON, 1.0), norm) {
                converged = true;
                break;
            }
            // Descent must use one fixed set of row scales across the line
            // search. Rescaling by each trial voltage can report no progress
            // as both a voltage and its residual approach a zero crossing.
            // Include the old residual for rows driven through other ports.
            let mut line_norm = (0.0, 1.0);
            for row in 0..n {
                line_scale[row] = input[row]
                    .abs()
                    .max(voltage[row].abs())
                    .max(residual[row].abs());
                let row_norm = (residual[row].abs(), line_scale[row]);
                if relative_less(line_norm, row_norm) {
                    line_norm = row_norm;
                }
            }
            // A currently zero row may acquire a nonlinear trial residual
            // through another port. Give it a positive, fixed merit scale;
            // the final convergence test still uses its own voltage scale.
            let active_scale = line_scale.iter().copied().fold(0.0, Value::max);
            for scale in &mut line_scale {
                if *scale == 0.0 {
                    *scale = active_scale;
                }
            }
            let correction = self.solve(&conductance, &residual, abort)?;
            let mut damping = 1.0;
            let mut improved = false;
            let mut trial = vec![0.0; n];
            for _ in 0..48 {
                poll(abort)?;
                for index in 0..n {
                    trial[index] =
                        sum([(voltage[index], 1.0), (correction[index], -damping)].into_iter())?;
                }
                evaluate(&trial, &mut current, &mut conductance, &mut residual)?;
                let mut trial_norm = (0.0, 1.0);
                for row in 0..n {
                    let row_norm = (residual[row].abs(), line_scale[row]);
                    if relative_less(trial_norm, row_norm) {
                        trial_norm = row_norm;
                    }
                }
                if relative_less(trial_norm, line_norm) {
                    voltage.copy_from_slice(&trial);
                    improved = true;
                    break;
                }
                damping *= 0.5;
            }
            if !improved {
                break;
            }
        }
        if !converged {
            return Err(SimulationError::Circuit(format!(
                "PSS nonlinear algebraic constraint did not converge at t={time:e}"
            )));
        }
        let voltage_rate = if self.plan.rates.iter().any(|&rate| rate) {
            let rate = self.solve(&conductance, &input_rate, abort)?;
            for index in 0..n {
                poll(abort)?;
                if self.plan.rates[index] {
                    current[index][1] = sum([(conductance[index], rate[index])].into_iter())?;
                }
            }
            rate
        } else {
            Vec::new()
        };
        Ok(Sample {
            voltage,
            voltage_rate,
            current,
        })
    }

    fn solve(
        &self,
        conductance: &[Value],
        rhs: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let n = conductance.len();
        let mut triplets = Vec::with_capacity(n * n);
        let mut scaled_rhs = None;
        for row in 0..n {
            poll(abort)?;
            let start = triplets.len();
            for (col, &slope) in conductance.iter().enumerate() {
                let value = sum([
                    ((if row == col { 1.0 } else { 0.0 }), 1.0),
                    (-self.plan.response[row * n + col], slope),
                ]
                .into_iter());
                let Ok(value) = value else {
                    // The row can exceed binary64 even when its solution and
                    // current derivative are finite. Scale the complete row
                    // and RHS together, retaining the coefficient exponent.
                    triplets.truncate(start);
                    let rhs = scaled_rhs.get_or_insert_with(|| rhs.to_vec());
                    self.scaled_jacobian_row(
                        row,
                        conductance,
                        &mut triplets,
                        &mut rhs[row],
                        abort,
                    )?;
                    break;
                };
                if value != 0.0 {
                    triplets.push((row, col, value));
                }
            }
        }
        let mut matrix = StaticMatrix::from_triplets(n, n, &triplets)?;
        poll(abort)?;
        let rhs = scaled_rhs.as_deref().unwrap_or(rhs);
        let solution = match matrix.solve(rhs) {
            Err(SolverError::InaccurateSolution(_)) if n <= 64 => matrix.solve_dense_extended(rhs),
            result => result,
        }?;
        poll(abort)?;
        Ok(solution)
    }

    fn scaled_jacobian_row(
        &self,
        row: usize,
        conductance: &[Value],
        triplets: &mut Vec<(usize, usize, Value)>,
        rhs: &mut Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        use rspice_veriloga_runtime::arithmetic::ScaledValue as S;
        let n = conductance.len();
        let one = S::new(1.0);
        let mut scale = one;
        let mut coefficients = Vec::with_capacity(n);
        for (col, &slope) in conductance.iter().enumerate() {
            poll(abort)?;
            let coefficient = S::sum_products_div(
                [
                    [S::new(if row == col { 1.0 } else { 0.0 }), one],
                    [S::new(-self.plan.response[row * n + col]), S::new(slope)],
                ]
                .into_iter(),
                one,
            )
            .map_err(|_| precision_error())?;
            let magnitude = if coefficient.binary64().is_sign_negative() {
                coefficient.negated()
            } else {
                coefficient
            };
            if magnitude.divide(scale).binary64() > 1.0 {
                scale = magnitude;
            }
            coefficients.push(coefficient);
        }
        let convert = |value: S| {
            let result = value.divide(scale).binary64();
            if !result.is_finite() || (result == 0.0 && !value.is_zero()) {
                Err(precision_error())
            } else {
                Ok(result)
            }
        };
        for (col, value) in coefficients.into_iter().enumerate() {
            poll(abort)?;
            let value = convert(value)?;
            if value != 0.0 {
                triplets.push((row, col, value));
            }
        }
        *rhs = convert(S::new(*rhs))?;
        Ok(())
    }
}

/// Find strongly connected components without recursive stack growth. Acyclic
/// amplification between islands does not invalidate a passive feedback block.
fn passive_feedback(
    q: &[Value],
    n: usize,
    available: usize,
    abort: &dyn AbortSignal,
) -> Result<bool, SimulationError> {
    PssVoltageConstraintBuilder::ensure_words(n.saturating_mul(16), available)?;
    let mut seen = vec![false; n];
    let mut order = Vec::with_capacity(n);
    let mut stack = Vec::with_capacity(n);
    for start in 0..n {
        if seen[start] {
            continue;
        }
        seen[start] = true;
        stack.push((start, 0));
        while let Some((row, next)) = stack.last_mut() {
            poll(abort)?;
            if *next == n {
                order.push(*row);
                stack.pop();
                continue;
            }
            let col = *next;
            *next += 1;
            if q[*row * n + col] != 0.0 && !seen[col] {
                seen[col] = true;
                stack.push((col, 0));
            }
        }
    }
    seen.fill(false);
    let mut component = Vec::with_capacity(n);
    while let Some(start) = order.pop() {
        if seen[start] {
            continue;
        }
        component.clear();
        component.push(start);
        seen[start] = true;
        let mut cursor = 0;
        while cursor < component.len() {
            poll(abort)?;
            let col = component[cursor];
            cursor += 1;
            for row in 0..n {
                if q[row * n + col] != 0.0 && !seen[row] {
                    seen[row] = true;
                    component.push(row);
                }
            }
        }
        if !negative_semidefinite(
            q,
            n,
            &component,
            available.saturating_sub(n.saturating_mul(16)),
            abort,
        )? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Exact Bareiss elimination on -(Q+Q^T). No tolerance can promote a small
/// positive-feedback eigenvalue to zero. Zero pivots of a PSD matrix have an
/// identically zero row; otherwise the certificate fails.
fn negative_semidefinite(
    q: &[Value],
    n: usize,
    component: &[usize],
    available: usize,
    abort: &dyn AbortSignal,
) -> Result<bool, SimulationError> {
    let size = component.len();
    let word_count = |value: &BigInt| (value.bits() as usize).div_ceil(64).saturating_add(4);
    let mut words = size
        .saturating_mul(size)
        .saturating_mul(40)
        .saturating_add(128);
    PssVoltageConstraintBuilder::ensure_words(words, available)?;
    let mut matrix = Vec::with_capacity(size * size);
    for &row in component {
        poll(abort)?;
        for &col in component {
            matrix.push(
                -integer_coefficient(q[row * n + col])? - integer_coefficient(q[col * n + row])?,
            );
        }
    }
    words = matrix.iter().map(word_count).sum();
    let mut previous = BigInt::from(1);
    for pivot_index in 0..size {
        poll(abort)?;
        PssVoltageConstraintBuilder::ensure_words(
            words
                .saturating_add(3 * word_count(&matrix[pivot_index * size + pivot_index]))
                .saturating_add(word_count(&previous)),
            available,
        )?;
        let pivot = matrix[pivot_index * size + pivot_index].clone();
        if pivot.sign() == Sign::Minus {
            return Ok(false);
        }
        if pivot.sign() == Sign::NoSign {
            if (pivot_index + 1..size)
                .any(|col| matrix[pivot_index * size + col].sign() != Sign::NoSign)
            {
                return Ok(false);
            }
            continue;
        }
        for row in pivot_index + 1..size {
            for col in row..size {
                poll(abort)?;
                let index = row * size + col;
                let bits = (pivot.bits() + matrix[index].bits()).max(
                    matrix[row * size + pivot_index].bits()
                        + matrix[pivot_index * size + col].bits(),
                );
                let temporary = (bits as usize)
                    .div_ceil(64)
                    .saturating_add(8)
                    .saturating_mul(12);
                PssVoltageConstraintBuilder::ensure_words(
                    words.saturating_add(temporary),
                    available,
                )?;
                let value = (&pivot * &matrix[index]
                    - &matrix[row * size + pivot_index] * &matrix[pivot_index * size + col])
                    / &previous;
                words = words
                    .saturating_sub(word_count(&matrix[index]))
                    .saturating_add(word_count(&value));
                if row != col {
                    let transpose = col * size + row;
                    words = words
                        .saturating_sub(word_count(&matrix[transpose]))
                        .saturating_add(word_count(&value));
                    matrix[transpose] = value.clone();
                }
                matrix[index] = value;
            }
        }
        previous = pivot;
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    #[test]
    fn nonlinear_jacobian_recovers_finite_corrections_from_overflowed_products() {
        let plan = Plan {
            ports: vec![vec![]; 2],
            response: vec![-1e300, 0.0, 0.0, 0.0],
            rates: vec![true; 2],
        };
        let initial = Arc::new(Sample::default());
        let forcing = NonlinearForcing {
            plan: Arc::new(plan),
            initial: initial.clone(),
            samples: std::array::from_fn(|_| (0.0, initial.clone())),
        };
        let correction = forcing
            .solve(&[1e300, 0.0], &[1e300, -0.25], &NoAbort)
            .unwrap();
        assert!((correction[0] / 1e-300 - 1.0).abs() < 2e-14);
        assert_eq!(correction[1], -0.25);
        // A derivative RHS uses the same Jacobian. The physical current rate
        // remains finite even though the unscaled tangent exceeds binary64.
        let rate = forcing
            .solve(&[1e300, 0.0], &[2e300, 0.0], &NoAbort)
            .unwrap();
        assert!((rate[0] * 1e300 - 2.0).abs() < 4e-14);
        assert_eq!(rate[1], 0.0);
    }

    #[test]
    fn nonlinear_relative_norm_preserves_extreme_scales() {
        for (left, right) in [
            ((1e300, 1e-300), (2e300, 1e-300)),
            ((1e-300, 1e300), (2e-300, 1e300)),
            ((0.0, 1.0), (Value::from_bits(1), Value::MAX)),
            ((1.0, 1.0), (1.0, 0.0)),
        ] {
            assert!(relative_less(left, right));
            assert!(!relative_less(right, left));
            assert!(!relative_less(left, left));
        }
    }

    #[test]
    fn nonlinear_descriptor_passivity_is_exact_and_allows_feedforward() {
        for (matrix, expected) in [
            (vec![-2.0, -1.0, -1.0, -2.0], true),
            (vec![-1.0, -1.0, -1.0, -1.0], true),
            (vec![-1.0, -1.0, -1.0, -1.0 + Value::EPSILON], false),
            (vec![0.0, 1e200, 0.0, 0.0], true),
            (vec![-1.0, 4.0, -4.0, -1.0], true),
            (vec![0.0, 1.0, 1.0, 0.0], false),
            (vec![-1e-300, 0.0, 0.0, 1e-300], false),
        ] {
            assert_eq!(
                passive_feedback(&matrix, 2, 100_000, &NoAbort).unwrap(),
                expected,
                "{matrix:?}"
            );
        }
        // A singular interior pivot must not erase the later negative minor.
        let q = [-2.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, -2.0];
        assert!(negative_semidefinite(&q, 3, &[0, 1, 2], 100_000, &NoAbort).unwrap());
        assert!(matches!(
            passive_feedback(&q, 3, 1, &NoAbort),
            Err(SimulationError::ResourceLimit(_))
        ));
        assert!(matches!(
            passive_feedback(&q, 3, 100_000, &crate::abort_signal::CountingAbort::new(3)),
            Err(SimulationError::Aborted)
        ));
    }
}
