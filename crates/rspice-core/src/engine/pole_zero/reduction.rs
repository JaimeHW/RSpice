//! Original-equation audit of a sparse descriptor reduction.
use super::*;
use crate::analysis::pole_zero::{RootSetEvidence, SpectrumCertificate};
use crate::numerics::exact_constraints::{
    RESIDUAL_SCRATCH_WORDS, equation_backward_error, has_full_rank,
};
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};

/// Audit borrowed maps x=T*q+U*u, q'=A*q+B*u, y=L*x=c*q+d*u.
/// T has an identity dynamic block; C has no algebraic rows or columns.
pub(super) struct SparseReduction<'a> {
    pub g: &'a crate::solver::ComplexMatrix,
    pub c: &'a crate::solver::ComplexMatrix,
    pub dynamic_map: &'a [usize],
    pub algebraic_map: &'a [usize],
    pub algebraic_count: usize,
    pub inverse_g_ad: &'a [Value],
    pub inverse_b_a: &'a [Value],
    pub input: &'a [Value],
    pub output: &'a [Value],
    pub a: &'a Matrix,
    pub b: &'a [Value],
    pub output_c: &'a [Value],
    pub output_d: Value,
}

fn workspace_floor(n: usize) -> usize {
    n.saturating_mul(n.saturating_mul(8).saturating_add(1))
}

/// Charge the existing descriptor workspace together with the exact proof.
pub(super) fn prove_block_rank(
    matrix: &crate::solver::StaticMatrix,
    original_order: usize,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<bool, SimulationError> {
    let retained = workspace_floor(original_order);
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        retained,
        limits.max_result_values,
    )?;
    let mut remaining = limits;
    remaining.max_result_values -= retained;
    has_full_rank(matrix.nrows, matrix.stored_entries(), remaining, abort).map_err(|error| {
        let mut error = SimulationError::from(error);
        if let SimulationError::ResourceLimit(resource) = &mut error
            && resource.resource == ResourceKind::ResultValues
        {
            resource.requested = resource.requested.saturating_add(retained);
            resource.limit = limits.max_result_values;
        }
        error
    })
}

impl SparseReduction<'_> {
    fn basis(&self, row: usize, state: usize) -> Value {
        let dynamic = self.dynamic_map[row];
        if dynamic != usize::MAX {
            Value::from(dynamic == state)
        } else {
            -self.inverse_g_ad[state * self.algebraic_count + self.algebraic_map[row]]
        }
    }

    fn forcing(&self, row: usize) -> Value {
        let algebraic = self.algebraic_map[row];
        if algebraic == usize::MAX {
            0.0
        } else {
            self.inverse_b_a[algebraic]
        }
    }

    pub(super) fn backward_error(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Value, SimulationError> {
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            workspace_floor(self.g.nrows).saturating_add(RESIDUAL_SCRATCH_WORDS),
            limits.max_result_values,
        )?;
        let states = self.b.len();
        let mut error: Value = 0.0;
        for row in 0..self.g.nrows {
            for state in 0..states {
                // G*T + C*T*A = 0. Only the dynamic identity block of T
                // participates in C*T, so no rounded intermediate CT is used.
                let terms = self
                    .g
                    .row_entries(row)?
                    .map(|(col, g)| (g.re, self.basis(col, state)))
                    .chain(
                        self.c
                            .row_entries(row)?
                            .filter(|(col, _)| self.dynamic_map[*col] != usize::MAX)
                            .map(|(col, c)| (c.im, self.a.get(self.dynamic_map[col], state))),
                    );
                error = error.max(equation_backward_error(terms, abort)?);
            }
            // G*U + C*T*B = input, with C*U exactly zero by partition.
            let terms = self
                .g
                .row_entries(row)?
                .map(|(col, g)| (g.re, self.forcing(col)))
                .chain(
                    self.c
                        .row_entries(row)?
                        .filter(|(col, _)| self.dynamic_map[*col] != usize::MAX)
                        .map(|(col, c)| (c.im, self.b[self.dynamic_map[col]])),
                )
                .chain([(-1.0, self.input[row])]);
            error = error.max(equation_backward_error(terms, abort)?);
        }
        for state in 0..states {
            let terms = self
                .output
                .iter()
                .enumerate()
                .map(|(row, &weight)| (weight, self.basis(row, state)))
                .chain([(-1.0, self.output_c[state])]);
            error = error.max(equation_backward_error(terms, abort)?);
        }
        let terms = self
            .output
            .iter()
            .enumerate()
            .map(|(row, &weight)| (weight, self.forcing(row)))
            .chain([(-1.0, self.output_d)]);
        Ok(error.max(equation_backward_error(terms, abort)?))
    }
}

/// Restore the original algebraic multiplicity and include reduction error in
/// both the natural-pole and transfer-zero certificates.
pub(super) fn retain_original_evidence(
    result: &mut PoleZeroResult,
    algebraic_count: usize,
    error: Value,
) -> Option<()> {
    for (roots, evidence) in [
        (&result.poles, &mut result.pole_evidence),
        (&result.zeros, &mut result.zero_evidence),
    ] {
        if matches!(evidence, RootSetEvidence::NotRequested) {
            continue;
        }
        let previous = evidence.certificate()?;
        let mut certificate = SpectrumCertificate::exact(
            previous.problem_order.checked_add(algebraic_count)?,
            previous.infinite_count.checked_add(algebraic_count)?,
        )?;
        certificate.max_backward_error = error.max(previous.max_backward_error);
        *evidence = RootSetEvidence::from_certificate(roots.len(), certificate)?;
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::CountingAbort;

    #[test]
    fn original_equations_audit_state_forcing_and_observation_maps_at_all_scales() {
        let structure = crate::solver::StaticMatrix::from_triplets(
            3,
            3,
            &(0..3)
                .flat_map(|r| (0..3).map(move |c| (r, c, 0.0)))
                .collect::<Vec<_>>(),
        )
        .unwrap();
        for exponent in [-900, 0, 900] {
            let scale = 2.0_f64.powi(exponent);
            let mut g = crate::solver::ComplexMatrix::from_real_structure(&structure);
            let mut c = crate::solver::ComplexMatrix::from_real_structure(&structure);
            for (row, values) in [[5.0, 1.0, 1.0], [2.0, 4.0, -1.0], [2.0, 1.0, 3.0]]
                .iter()
                .enumerate()
            {
                for (col, &value) in values.iter().enumerate() {
                    g.add_real(row, col, value * scale);
                }
            }
            c.add_imag(0, 0, 2.0 * scale);
            c.add_imag(1, 1, 4.0 * scale);
            let limits = ResourceLimits::default();
            for perturbation in 0..6 {
                let mut a = Matrix::from_dense(vec![
                    vec![-13.0 / 6.0, -1.0 / 3.0],
                    vec![-2.0 / 3.0, -13.0 / 12.0],
                ]);
                let mut inverse_g_ad = [2.0 / 3.0, 1.0 / 3.0];
                let mut b = [0.5, 0.0];
                let mut output_c = [-2.0 / 3.0, -1.0 / 3.0];
                let mut inverse_b_a = [0.0];
                let mut output_d = 0.0;
                match perturbation {
                    1 => a.add(0, 0, 0.125),
                    2 => inverse_g_ad[0] += 0.125,
                    3 => b[0] += 0.125,
                    4 => output_c[0] += 0.125,
                    5 => {
                        inverse_b_a[0] = 0.125;
                        output_d = 0.125;
                    }
                    _ => {}
                }
                let audit = SparseReduction {
                    g: &g,
                    c: &c,
                    dynamic_map: &[0, 1, usize::MAX],
                    algebraic_map: &[usize::MAX, usize::MAX, 0],
                    algebraic_count: 1,
                    inverse_g_ad: &inverse_g_ad,
                    inverse_b_a: &inverse_b_a,
                    input: &[scale, 0.0, 0.0],
                    output: &[0.0, 0.0, 1.0],
                    a: &a,
                    b: &b,
                    output_c: &output_c,
                    output_d,
                };
                let error = audit.backward_error(limits, &NoAbort).unwrap();
                if perturbation == 0 {
                    assert!(error < 1e-15, "{exponent}: {error}");
                } else {
                    assert!(error > 1e-3, "{exponent}/{perturbation}: {error}");
                }
                let abort = CountingAbort::new(3);
                assert!(matches!(
                    audit.backward_error(limits, &abort),
                    Err(SimulationError::Aborted)
                ));
                let mut limited = limits;
                limited.max_result_values = RESIDUAL_SCRATCH_WORDS;
                assert!(matches!(
                    audit.backward_error(limited, &NoAbort),
                    Err(SimulationError::ResourceLimit(_))
                ));
            }
        }
    }
}
