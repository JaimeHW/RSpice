//! Algebraic voltage constraints of a singular, constant winding flux matrix.
//! L = S K S, where S contains positive square roots of winding inductance.
//! Exact rank belongs to the authored dimensionless K matrix. Rounding M
//! must not turn K=1 into a fictitious leakage mode. Storage stays untouched.
use super::*;
use crate::numerics::exact_constraints::{
    ExactElimination, ExactRow, coefficient_ratio, integer_coefficient,
};
use std::collections::BTreeMap;

pub(super) struct FluxConservation {
    pub rows: BTreeMap<usize, Vec<(usize, Value)>>,
    pub retained_values: usize,
    size: usize,
}

impl FluxConservation {
    pub(super) fn new(
        size: usize,
        windings: &[(usize, Value)],
        couplings: &[(usize, usize, Value)],
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Self>> {
        check_abort(abort)?;
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            size,
            options.limits.max_matrix_unknowns,
        )?;
        let overhead = size
            .saturating_mul(64)
            .saturating_add(windings.len().saturating_mul(64))
            .saturating_add(couplings.len().saturating_mul(32));
        ExactElimination::<usize>::ensure_words(overhead, options.limits.max_result_values)?;
        let mut coefficients = BTreeMap::new();
        let mut roots = BTreeMap::new();
        for &(branch, inductance) in windings {
            check_abort(abort)?;
            if branch >= size
                || !inductance.is_finite()
                || inductance <= 0.0
                || coefficients.insert(branch, vec![(branch, 1.0)]).is_some()
            {
                return Err(error("invalid winding in flux descriptor"));
            }
            roots.insert(branch, integer_coefficient(inductance.sqrt()).unwrap());
        }
        for &(first, second, value) in couplings {
            check_abort(abort)?;
            if first == second
                || !value.is_finite()
                || !coefficients.contains_key(&first)
                || !coefficients.contains_key(&second)
            {
                return Err(error("invalid mutual term in flux descriptor"));
            }
            coefficients.get_mut(&first).unwrap().push((second, value));
            coefficients.get_mut(&second).unwrap().push((first, value));
        }
        let mut reducer = ExactElimination::<usize>::new(size, options.limits)?;
        reducer.reserve_retained_words(overhead)?;
        let mut rows = BTreeMap::new();
        for (branch, terms) in coefficients {
            check_abort(abort)?;
            let mut equation = ExactRow::default();
            equation
                .values
                .insert(branch, integer_coefficient(1.0).unwrap());
            for (column, value) in terms {
                check_abort(abort)?;
                reducer.check_cost(equation.words().saturating_mul(3).saturating_add(160))?;
                ExactElimination::<usize>::add_integer(
                    &mut equation.nodes,
                    column,
                    integer_coefficient(value).unwrap(),
                );
            }
            if let Some(constraint) = reducer.admit(equation, 0, abort)? {
                // The original row labels undergo exactly the same elimination
                // as K. If alpha annihilates K, alpha/S annihilates L. Work
                // with exact products of the integer weights and finite roots
                // until the final coefficient ratio, avoiding intermediate
                // overflow/underflow when winding scales differ greatly.
                let mut normalizer = *constraint
                    .values
                    .first_key_value()
                    .ok_or_else(|| error("missing dependent flux constraint"))?
                    .0;
                reducer.check_cost(constraint.words().saturating_mul(4).saturating_add(1024))?;
                for (&source, coefficient) in &constraint.values {
                    check_abort(abort)?;
                    if coefficient.magnitude() * roots[&normalizer].magnitude()
                        > constraint.values[&normalizer].magnitude() * roots[&source].magnitude()
                    {
                        normalizer = source;
                    }
                }
                reducer.reserve_retained_words(
                    constraint.values.len().saturating_mul(4).saturating_add(16),
                )?;
                let mut weights = Vec::with_capacity(constraint.values.len());
                for (&source, coefficient) in &constraint.values {
                    check_abort(abort)?;
                    let numerator = coefficient * &roots[&normalizer];
                    let denominator = &constraint.values[&normalizer] * &roots[&source];
                    let weight = coefficient_ratio(&numerator, &denominator).ok_or_else(|| {
                        error("flux constraint projection exceeds finite precision")
                    })?;
                    weights.push((source, weight));
                }
                rows.insert(branch, weights);
            }
        }
        if rows.is_empty() {
            return Ok(None);
        }
        let retained_values =
            rows.len()
                .saturating_mul(16)
                .saturating_add(rows.values().fold(0usize, |sum, row| {
                    sum.saturating_add(row.capacity().saturating_mul(4))
                }));
        Ok(Some(Self {
            rows,
            retained_values,
            size,
        }))
    }
}

impl ChargeEventTopology {
    pub(super) fn install_flux_conservation(
        &mut self,
        prepared: Arc<FluxConservation>,
    ) -> Result<()> {
        let owns_flux = |row: usize| {
            row >= self.nodes
                && row < self.size
                && self.branch_equations[row - self.nodes]
                    .flux_tolerance()
                    .is_some()
        };
        if prepared.size != self.size
            || prepared.rows.iter().any(|(&row, terms)| {
                !owns_flux(row)
                    || terms.is_empty()
                    || terms
                        .iter()
                        .any(|&(source, weight)| !owns_flux(source) || !weight.is_finite())
            })
        {
            return Err(error(
                "flux conservation bindings do not match event topology",
            ));
        }
        self.flux = Some(prepared);
        Ok(())
    }

    pub(super) fn flux_constraint(&self, row: usize) -> Option<&[(usize, Value)]> {
        self.flux.as_ref()?.rows.get(&row).map(Vec::as_slice)
    }

    pub(super) fn has_storage_rate_equation(&self, row: usize, options: &EventOptions) -> bool {
        !self.is_group_row(row)
            && self.flux_constraint(row).is_none()
            && self.storage_tolerance(row, options).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};

    fn options() -> EventOptions {
        EventOptions {
            limits: ResourceLimits::default(),
            solver: SolverOptions::default(),
            nodal_gmin: 0.0,
            iterations: 80,
            backtracks: 32,
            voltage_tolerance: 1e-11,
            current_tolerance: 1e-13,
            charge_tolerance: 1e-25,
            relative_tolerance: 1e-11,
        }
    }

    #[test]
    fn perfect_flux_constraints_keep_voltage_ratios_at_extreme_scales() {
        for exponent in [-900, 0, 900] {
            let scale = 2.0_f64.powi(exponent);
            for sign in [-1.0, 1.0] {
                let basis = FluxConservation::new(
                    2,
                    &[(0, scale), (1, 4.0 * scale)],
                    &[(0, 1, sign)],
                    &options(),
                    &NoAbort,
                )
                .unwrap()
                .unwrap();
                assert_eq!(basis.rows.len(), 1);
                for terms in basis.rows.values() {
                    let voltage = [3.0, 6.0 * sign];
                    assert_eq!(
                        sum(terms.iter().map(|&(row, weight)| (voltage[row], weight))).unwrap(),
                        0.0
                    );
                    let voltage = [3.0, 6.0 * sign + 0.125];
                    assert!(
                        sum(terms.iter().map(|&(row, weight)| (voltage[row], weight)))
                            .unwrap()
                            .abs()
                            > 0.01
                    );
                }
            }
        }
    }

    #[test]
    fn flux_weights_survive_widely_separated_inductance_scales() {
        for (first, second) in [(1e-300, 1e300), (1e300, 1e-300), (1.0, 2.0)] {
            let basis = FluxConservation::new(
                2,
                &[(0, first), (1, second)],
                &[(0, 1, 1.0)],
                &options(),
                &NoAbort,
            )
            .unwrap()
            .unwrap();
            let voltage = [first.sqrt(), second.sqrt()];
            for terms in basis.rows.values() {
                let scale = terms
                    .iter()
                    .map(|&(row, weight)| (voltage[row] * weight).abs())
                    .fold(0.0, Value::max);
                let residual =
                    sum(terms.iter().map(|&(row, weight)| (voltage[row], weight))).unwrap();
                assert!(residual.abs() <= 4.0 * Value::EPSILON * scale);
                assert_eq!(terms.len(), 2);
                assert!(terms.iter().all(|&(_, weight)| weight != 0.0));
            }
        }
    }

    #[test]
    fn near_perfect_flux_keeps_its_nonzero_dynamic_mode() {
        for mutual in [0.0, 1.0_f64.next_down(), -1.0_f64.next_down()] {
            assert!(
                FluxConservation::new(
                    2,
                    &[(0, 1.0), (1, 4.0)],
                    &[(0, 1, mutual)],
                    &options(),
                    &NoAbort
                )
                .unwrap()
                .is_none()
            );
        }
        // Multiple authored K cards contribute to one physical flux matrix.
        assert!(
            FluxConservation::new(
                2,
                &[(0, 1.0), (1, 4.0)],
                &[(0, 1, 0.25), (0, 1, 0.75)],
                &options(),
                &NoAbort
            )
            .unwrap()
            .is_some()
        );
    }

    #[test]
    fn three_perfect_windings_retain_both_voltage_constraints() {
        let basis = FluxConservation::new(
            3,
            &[(0, 1.0), (1, 4.0), (2, 9.0)],
            &[(0, 1, 1.0), (0, 2, 1.0), (1, 2, 1.0)],
            &options(),
            &NoAbort,
        )
        .unwrap()
        .unwrap();
        assert_eq!(basis.rows.len(), 2);
        for terms in basis.rows.values() {
            let voltage = [2.0, 4.0, 6.0];
            assert!(
                sum(terms.iter().map(|&(row, weight)| (voltage[row], weight)))
                    .unwrap()
                    .abs()
                    < 1e-15
            );
        }
    }

    #[test]
    fn flux_preparation_is_cancellable_and_bounded() {
        let run = |options: &EventOptions, abort: &dyn AbortSignal| {
            FluxConservation::new(
                3,
                &[(0, 1.0), (1, 4.0), (2, 9.0)],
                &[(0, 1, 1.0), (0, 2, 1.0), (1, 2, 1.0)],
                options,
                abort,
            )
        };
        let census = CountingAbort::new(usize::MAX);
        assert!(run(&options(), &census).unwrap().is_some());
        for threshold in 0..census.count() {
            let abort = CountingAbort::new(threshold);
            assert!(matches!(
                run(&options(), &abort),
                Err(SimulationError::Aborted)
            ));
            assert_eq!(abort.polls_after_abort(), 0);
        }
        for kind in [ResourceKind::MatrixUnknowns, ResourceKind::ResultValues] {
            let mut options = options();
            match kind {
                ResourceKind::MatrixUnknowns => options.limits.max_matrix_unknowns = 2,
                ResourceKind::ResultValues => options.limits.max_result_values = 256,
                _ => unreachable!(),
            }
            assert!(
                matches!(run(&options, &NoAbort), Err(SimulationError::ResourceLimit(error)) if error.resource == kind)
            );
        }
    }
}
