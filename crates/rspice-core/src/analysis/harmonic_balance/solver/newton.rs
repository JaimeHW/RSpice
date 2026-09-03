//! Full nonlinear harmonic-balance Newton iteration helpers.

use super::*;
use crate::solver::convergence::{PseudoTransient, SourceStepper};
use crate::solver::limit_pn_voltage;
use std::f64::consts::PI;

type PeriodicSpectrum = (usize, usize, Vec<Complex64>);

#[inline]
fn exact_krylov_dense_recovery_allowed(size: usize, error: &rspice_matrix::SolverError) -> bool {
    size < super::krylov::KRYLOV_AUTO_THRESHOLD
        && matches!(
            error,
            rspice_matrix::SolverError::ConvergenceFailed(_)
                | rspice_matrix::SolverError::InaccurateSolution(_)
        )
}

#[derive(Debug)]
struct HbNewtonStep {
    node_voltages: Vec<Vec<Complex64>>,
    branch_currents: Vec<Vec<Complex64>>,
}

#[derive(Debug, Clone)]
struct HbNewtonCheckpoint {
    node_voltages: Vec<Vec<Complex64>>,
    branch_currents: Vec<Vec<Complex64>>,
}

/// Exact real-split HB Jacobian as an operator.  Storage is proportional to
/// the sparse linear stamps plus the device coupling spectra; the global
/// `(nodes * harmonics)^2` matrix is never materialized.
struct ExactHbOperator<'a> {
    num_nodes: usize,
    num_components: usize,
    real_width: usize,
    omega0: Value,
    gmin: Value,
    g_matrix: &'a [(usize, usize, Value)],
    c_matrix: &'a [(usize, usize, Value)],
    l_matrix: &'a [(usize, usize, Value)],
    g_spectra: &'a [PeriodicSpectrum],
    c_spectra: &'a [PeriodicSpectrum],
    mna_branches: &'a [ExactMnaBranch],
    mna_static_entries: &'a [(usize, usize, Value)],
    mna_inductance_entries: &'a [(usize, usize, Value)],
    periodic_networks: &'a [ExactPeriodicNetwork],
}

impl ExactHbOperator<'_> {
    #[inline]
    fn entity_count(&self) -> usize {
        self.num_nodes + self.mna_branches.len()
    }

    #[inline]
    fn re_idx(&self, entity: usize, harmonic: usize) -> usize {
        if harmonic == 0 {
            entity * self.real_width
        } else {
            entity * self.real_width + 2 * harmonic - 1
        }
    }

    #[inline]
    fn im_idx(&self, entity: usize, harmonic: usize) -> usize {
        entity * self.real_width + 2 * harmonic
    }

    fn validate(&self) -> Result<(), HbError> {
        let expected_width = self
            .num_components
            .checked_mul(2)
            .and_then(|value| value.checked_sub(1))
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "exact HB operator harmonic layout exceeds this platform".to_string(),
                )
            })?;
        if self.num_components == 0 || self.real_width != expected_width {
            return Err(HbError::InvalidCircuit(format!(
                "exact HB operator real width {} does not match {} harmonic components",
                self.real_width, self.num_components
            )));
        }
        if !self.omega0.is_finite()
            || self.omega0 <= 0.0
            || !self.gmin.is_finite()
            || self.gmin < 0.0
        {
            return Err(HbError::InvalidCircuit(
                "exact HB operator frequency/GMIN is non-finite or outside its valid range"
                    .to_string(),
            ));
        }
        for (kind, entries) in [
            ("conductance", self.g_matrix),
            ("capacitance", self.c_matrix),
            ("legacy inductance", self.l_matrix),
        ] {
            if let Some(&(row, column, value)) = entries.iter().find(|(row, column, value)| {
                *row >= self.num_nodes || *column >= self.num_nodes || !value.is_finite()
            }) {
                return Err(HbError::InvalidCircuit(format!(
                    "exact HB {kind} operator entry ({row}, {column}, {value}) is malformed"
                )));
            }
        }
        for (kind, spectra) in [
            ("conductance", self.g_spectra),
            ("capacitance", self.c_spectra),
        ] {
            if let Some((row, column, _)) = spectra.iter().find(|(row, column, spectrum)| {
                *row >= self.num_nodes
                    || *column >= self.num_nodes
                    || spectrum
                        .iter()
                        .any(|value| !value.re.is_finite() || !value.im.is_finite())
            }) {
                return Err(HbError::InvalidCircuit(format!(
                    "exact HB periodic {kind} operator stamp ({row}, {column}) is malformed or non-finite"
                )));
            }
        }
        for (index, branch) in self.mna_branches.iter().enumerate() {
            let expected_ordinal = index.checked_add(1).ok_or_else(|| {
                HbError::InvalidCircuit(
                    "exact HB canonical branch ordinal exceeds this platform".to_string(),
                )
            })?;
            let (ordinal, node_pos, node_neg) = branch.ordinal_and_terminals();
            if ordinal != expected_ordinal
                || node_pos > self.num_nodes
                || node_neg > self.num_nodes
                || node_pos == node_neg
                || matches!(branch, ExactMnaBranch::Inductor { inductance, .. } if !inductance.is_finite() || *inductance == 0.0)
                || matches!(branch, ExactMnaBranch::Resistor { resistance, small_signal_resistance, .. } if !resistance.is_finite() || !small_signal_resistance.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "exact HB canonical MNA branch {index} is malformed"
                )));
            }
        }
        let unknowns = self.entity_count();
        if let Some(&(row, column, value)) =
            self.mna_static_entries.iter().find(|(row, column, value)| {
                *row >= unknowns || *column >= unknowns || !value.is_finite()
            })
        {
            return Err(HbError::InvalidCircuit(format!(
                "exact HB controlled-source operator entry ({row}, {column}, {value}) is malformed for {unknowns} unknowns"
            )));
        }
        if let Some(&(row, column, inductance)) =
            self.mna_inductance_entries
                .iter()
                .find(|(row, column, inductance)| {
                    *row < self.num_nodes
                        || *column < self.num_nodes
                        || *row >= unknowns
                        || *column >= unknowns
                        || *row == *column
                        || !inductance.is_finite()
                })
        {
            return Err(HbError::InvalidCircuit(format!(
                "exact HB mutual-inductance operator entry ({row}, {column}, {inductance}) is malformed for {unknowns} unknowns"
            )));
        }
        for (entry, &(_, _, inductance)) in self.mna_inductance_entries.iter().enumerate() {
            for harmonic in 1..self.num_components {
                let impedance = self.omega0 * harmonic as Value * inductance;
                if !impedance.is_finite() || (inductance != 0.0 && impedance == 0.0) {
                    return Err(HbError::InvalidCircuit(format!(
                        "exact HB mutual-inductance operator entry #{entry} has non-representable impedance at harmonic {harmonic}"
                    )));
                }
            }
        }
        for network in self.periodic_networks {
            for harmonic in 0..self.num_components {
                network.try_visit_direct_entries(
                    self.omega0 * harmonic as Value,
                    unknowns,
                    |_, _, _| {},
                )?;
            }
        }
        Ok(())
    }

    fn emit_entry(
        visitor: &mut dyn FnMut(usize, usize, Value),
        row: usize,
        column: usize,
        value: Value,
    ) {
        if value != 0.0 {
            visitor(row, column, value);
        }
    }

    /// Visit the realified entries contributed by a complex-linear term
    /// `y[row,k] += value * x[column,l]`.
    fn visit_linear_term(
        &self,
        row_entity: usize,
        row_harmonic: usize,
        column_entity: usize,
        column_harmonic: usize,
        value: Complex64,
        visitor: &mut dyn FnMut(usize, usize, Value),
    ) {
        let row_re = self.re_idx(row_entity, row_harmonic);
        let column_re = self.re_idx(column_entity, column_harmonic);
        Self::emit_entry(visitor, row_re, column_re, value.re);
        if column_harmonic > 0 {
            Self::emit_entry(
                visitor,
                row_re,
                self.im_idx(column_entity, column_harmonic),
                -value.im,
            );
        }
        if row_harmonic > 0 {
            let row_im = self.im_idx(row_entity, row_harmonic);
            Self::emit_entry(visitor, row_im, column_re, value.im);
            if column_harmonic > 0 {
                Self::emit_entry(
                    visitor,
                    row_im,
                    self.im_idx(column_entity, column_harmonic),
                    value.re,
                );
            }
        }
    }

    /// Visit the realified entries contributed by an antilinear term
    /// `y[row,k] += value * conj(x[column,m])`.
    fn visit_antilinear_term(
        &self,
        row_entity: usize,
        row_harmonic: usize,
        column_entity: usize,
        column_harmonic: usize,
        value: Complex64,
        visitor: &mut dyn FnMut(usize, usize, Value),
    ) {
        debug_assert!(column_harmonic > 0);
        let row_re = self.re_idx(row_entity, row_harmonic);
        let column_re = self.re_idx(column_entity, column_harmonic);
        let column_im = self.im_idx(column_entity, column_harmonic);
        Self::emit_entry(visitor, row_re, column_re, value.re);
        Self::emit_entry(visitor, row_re, column_im, value.im);
        if row_harmonic > 0 {
            let row_im = self.im_idx(row_entity, row_harmonic);
            Self::emit_entry(visitor, row_im, column_re, value.im);
            Self::emit_entry(visitor, row_im, column_im, -value.re);
        }
    }

    /// Stream the exact realified Jacobian entries without materializing the
    /// global matrix. The same stream drives both matvecs and the independent
    /// componentwise backward-error certificate.
    fn visit_entries(&self, mut visitor: impl FnMut(usize, usize, Value)) {
        let n = self.num_nodes;
        let h = self.num_components;
        for k in 0..h {
            let omega_k = (k as Value) * self.omega0;
            let jw = Complex64::new(0.0, omega_k);
            for &(i, j, g) in self.g_matrix {
                if i < n && j < n {
                    self.visit_linear_term(i, k, j, k, Complex64::new(-g, 0.0), &mut visitor);
                }
            }
            for &(i, j, c) in self.c_matrix {
                if i < n && j < n {
                    self.visit_linear_term(i, k, j, k, -jw * c, &mut visitor);
                }
            }
            for &(i, j, l) in self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    let admittance = if k == 0 {
                        Complex64::new(inductor_dc_short_admittance(l), 0.0)
                    } else {
                        Complex64::new(0.0, -1.0 / (omega_k * l))
                    };
                    self.visit_linear_term(i, k, j, k, -admittance, &mut visitor);
                }
            }
            for node in 0..n {
                self.visit_linear_term(
                    node,
                    k,
                    node,
                    k,
                    Complex64::new(-self.gmin, 0.0),
                    &mut visitor,
                );
            }

            for (branch_index, branch) in self.mna_branches.iter().enumerate() {
                let branch_entity = n + branch_index;
                let (_, node_pos, node_neg) = branch.ordinal_and_terminals();
                if node_pos > 0 {
                    let node = node_pos - 1;
                    self.visit_linear_term(
                        node,
                        k,
                        branch_entity,
                        k,
                        Complex64::new(-1.0, 0.0),
                        &mut visitor,
                    );
                    if !matches!(branch, ExactMnaBranch::NetworkPort { .. }) {
                        self.visit_linear_term(
                            branch_entity,
                            k,
                            node,
                            k,
                            Complex64::new(-1.0, 0.0),
                            &mut visitor,
                        );
                    }
                }
                if node_neg > 0 {
                    let node = node_neg - 1;
                    self.visit_linear_term(
                        node,
                        k,
                        branch_entity,
                        k,
                        Complex64::new(1.0, 0.0),
                        &mut visitor,
                    );
                    if !matches!(branch, ExactMnaBranch::NetworkPort { .. }) {
                        self.visit_linear_term(
                            branch_entity,
                            k,
                            node,
                            k,
                            Complex64::new(1.0, 0.0),
                            &mut visitor,
                        );
                    }
                }
                if let ExactMnaBranch::Inductor { inductance, .. } = branch {
                    self.visit_linear_term(
                        branch_entity,
                        k,
                        branch_entity,
                        k,
                        jw * *inductance,
                        &mut visitor,
                    );
                }
                if let ExactMnaBranch::Resistor { resistance, .. } = branch {
                    self.visit_linear_term(
                        branch_entity,
                        k,
                        branch_entity,
                        k,
                        Complex64::new(*resistance, 0.0),
                        &mut visitor,
                    );
                }
            }
            for &(row, column, value) in self.mna_static_entries {
                self.visit_linear_term(
                    row,
                    k,
                    column,
                    k,
                    Complex64::new(-value, 0.0),
                    &mut visitor,
                );
            }
            for &(row, column, inductance) in self.mna_inductance_entries {
                self.visit_linear_term(row, k, column, k, jw * inductance, &mut visitor);
            }
            for network in self.periodic_networks {
                network
                    .try_visit_direct_entries(jw.im, self.entity_count(), |row, column, value| {
                        self.visit_linear_term(row, k, column, k, -value, &mut visitor);
                    })
                    .expect("validated exact HB distributed network remains representable");
            }
        }

        for &(i, j, ref spectrum) in self.g_spectra {
            if i >= n || j >= n {
                continue;
            }
            for k in 0..h {
                for l in 0..h {
                    let diff = k as isize - l as isize;
                    if let Some(&coefficient) = spectrum.get(diff.unsigned_abs()) {
                        let coefficient = if diff >= 0 {
                            coefficient
                        } else {
                            coefficient.conj()
                        };
                        self.visit_linear_term(i, k, j, l, -coefficient, &mut visitor);
                    }
                }
                for m in 1..h {
                    if let Some(&coefficient) = spectrum.get(k + m) {
                        self.visit_antilinear_term(i, k, j, m, -coefficient, &mut visitor);
                    }
                }
            }
        }
        for &(i, j, ref spectrum) in self.c_spectra {
            if i >= n || j >= n {
                continue;
            }
            for k in 0..h {
                let jw = Complex64::new(0.0, (k as Value) * self.omega0);
                for l in 0..h {
                    let diff = k as isize - l as isize;
                    if let Some(&coefficient) = spectrum.get(diff.unsigned_abs()) {
                        let coefficient = if diff >= 0 {
                            coefficient
                        } else {
                            coefficient.conj()
                        };
                        self.visit_linear_term(i, k, j, l, -jw * coefficient, &mut visitor);
                    }
                }
                for m in 1..h {
                    if let Some(&coefficient) = spectrum.get(k + m) {
                        self.visit_antilinear_term(i, k, j, m, -jw * coefficient, &mut visitor);
                    }
                }
            }
        }
    }

    fn apply(&self, input: &[Complex64]) -> Vec<Complex64> {
        let size = self.entity_count() * self.real_width;
        debug_assert_eq!(input.len(), size);
        let mut output = vec![Complex64::new(0.0, 0.0); size];
        self.visit_entries(|row, column, value| {
            output[row] += value * input[column];
        });
        output
    }

    /// Canonicalize exact homogeneous singleton voltage constraints before a
    /// full streamed certificate is formed. A grounded ideal voltage-source
    /// branch row contains exactly one node-voltage coordinate. When that
    /// realified row has an exactly zero RHS, the coordinate is mathematically
    /// zero; spelling it as positive zero avoids treating Krylov roundoff as a
    /// componentwise error of one in that homogeneous row.
    ///
    /// This is deliberately an exact, topology-based operation: nonzero RHS
    /// rows, floating two-node constraints, and non-voltage branches are left
    /// untouched.
    fn project_homogeneous_grounded_voltage_singletons(
        &self,
        solution: &mut [Complex64],
        rhs: &[Complex64],
    ) {
        debug_assert_eq!(solution.len(), self.entity_count() * self.real_width);
        debug_assert_eq!(rhs.len(), solution.len());
        let zero = Complex64::new(0.0, 0.0);
        for (branch_index, branch) in self.mna_branches.iter().enumerate() {
            let ExactMnaBranch::VoltageSource {
                node_pos, node_neg, ..
            } = branch
            else {
                continue;
            };
            let node = match (*node_pos, *node_neg) {
                (node, 0) if node > 0 => node - 1,
                (0, node) if node > 0 => node - 1,
                _ => continue,
            };
            let branch_entity = self.num_nodes + branch_index;
            for harmonic in 0..self.num_components {
                let branch_re = self.re_idx(branch_entity, harmonic);
                if rhs[branch_re] == zero {
                    solution[self.re_idx(node, harmonic)] = zero;
                }
                if harmonic > 0 {
                    let branch_im = self.im_idx(branch_entity, harmonic);
                    if rhs[branch_im] == zero {
                        solution[self.im_idx(node, harmonic)] = zero;
                    }
                }
            }
        }
    }

    /// Assemble only the independent per-harmonic diagonal blocks used by
    /// the block-Jacobi preconditioner.  This costs O(H*N^2), not
    /// O((H*N)^2), and includes the exact same-harmonic Hankel coupling.
    fn harmonic_blocks(&self) -> Vec<Vec<Complex64>> {
        let entities = self.entity_count();
        let h = self.num_components;
        let mut blocks = (0..h)
            .map(|harmonic| {
                let size = if harmonic == 0 {
                    entities
                } else {
                    2 * entities
                };
                vec![Complex64::new(0.0, 0.0); size * size]
            })
            .collect::<Vec<_>>();
        self.visit_entries(|row, column, value| {
            let row_entity = row / self.real_width;
            let row_local = row % self.real_width;
            let column_entity = column / self.real_width;
            let column_local = column % self.real_width;
            let row_harmonic = if row_local == 0 {
                0
            } else {
                row_local.div_ceil(2)
            };
            let column_harmonic = if column_local == 0 {
                0
            } else {
                column_local.div_ceil(2)
            };
            if row_harmonic != column_harmonic {
                return;
            }
            let harmonic = row_harmonic;
            if harmonic == 0 {
                blocks[0][row_entity * entities + column_entity] += value;
            } else {
                let block_size = 2 * entities;
                let row_component = (row_local + 1) % 2;
                let column_component = (column_local + 1) % 2;
                let block_row = 2 * row_entity + row_component;
                let block_column = 2 * column_entity + column_component;
                blocks[harmonic][block_row * block_size + block_column] += value;
            }
        });
        blocks
    }
}

struct ExactHbPreconditioner {
    num_entities: usize,
    num_components: usize,
    real_width: usize,
    factors: Vec<super::krylov::LuFactors>,
}

impl ExactHbPreconditioner {
    fn build(operator: &ExactHbOperator<'_>) -> Self {
        let factors = operator
            .harmonic_blocks()
            .into_iter()
            .enumerate()
            .map(|(k, block)| {
                let size = if k == 0 {
                    operator.entity_count()
                } else {
                    2 * operator.entity_count()
                };
                super::krylov::LuFactors::factor(block, size)
            })
            .collect();
        Self {
            num_entities: operator.entity_count(),
            num_components: operator.num_components,
            real_width: operator.real_width,
            factors,
        }
    }

    fn apply_with_input_scale(
        &self,
        residual: &[Complex64],
        input_scale: Option<&[Value]>,
    ) -> Vec<Complex64> {
        debug_assert_eq!(
            input_scale.map_or(residual.len(), <[Value]>::len),
            residual.len()
        );
        let scaled_value =
            |index: usize| residual[index] * input_scale.map_or(1.0, |scale| scale[index]);
        let mut output = vec![Complex64::new(0.0, 0.0); residual.len()];
        let mut dc = (0..self.num_entities)
            .map(|entity| scaled_value(entity * self.real_width))
            .collect::<Vec<_>>();
        self.factors[0].solve_in_place(&mut dc);
        for (entity, &value) in dc.iter().enumerate() {
            output[entity * self.real_width] = value;
        }
        for k in 1..self.num_components {
            let mut block = vec![Complex64::new(0.0, 0.0); 2 * self.num_entities];
            for entity in 0..self.num_entities {
                block[2 * entity] = scaled_value(entity * self.real_width + 2 * k - 1);
                block[2 * entity + 1] = scaled_value(entity * self.real_width + 2 * k);
            }
            self.factors[k].solve_in_place(&mut block);
            for entity in 0..self.num_entities {
                output[entity * self.real_width + 2 * k - 1] = block[2 * entity];
                output[entity * self.real_width + 2 * k] = block[2 * entity + 1];
            }
        }
        output
    }
}

impl super::krylov::KrylovPreconditioner for ExactHbPreconditioner {
    fn apply(&self, residual: &[Complex64]) -> Vec<Complex64> {
        self.apply_with_input_scale(residual, None)
    }
}

struct RowScaledExactHbPreconditioner<'a> {
    base: &'a ExactHbPreconditioner,
    inverse_row_scale: &'a [Value],
}

impl super::krylov::KrylovPreconditioner for RowScaledExactHbPreconditioner<'_> {
    fn apply(&self, residual: &[Complex64]) -> Vec<Complex64> {
        debug_assert_eq!(residual.len(), self.inverse_row_scale.len());
        self.base
            .apply_with_input_scale(residual, Some(self.inverse_row_scale))
    }
}

fn exact_hb_candidate_report(
    operator: &ExactHbOperator<'_>,
    solution: &mut [Complex64],
    rhs: &[Complex64],
) -> Result<(rspice_matrix::ComplexTransposeBackwardErrorReport, Value), rspice_matrix::SolverError>
{
    operator.project_homogeneous_grounded_voltage_singletons(solution, rhs);
    let size = rhs.len();
    let report = rspice_matrix::analyze_complex_transpose_solution_by_entry_visitor(
        size,
        size,
        solution,
        rhs,
        |visitor| {
            // The matrix helper analyzes A^T*x=b. Transposed coordinates
            // therefore analyze this forward J*x=b.
            operator.visit_entries(|row, column, value| {
                visitor(column, row, Complex64::new(value, 0.0));
            });
        },
    )?;
    let rhs_norm = stable_complex_l2_norm(rhs);
    let residual_norm = stable_complex_l2_norm(report.residual());
    let relative_residual = if rhs_norm == 0.0 {
        residual_norm
    } else {
        residual_norm / rhs_norm
    };
    if !relative_residual.is_finite() {
        return Err(rspice_matrix::SolverError::Overflow);
    }
    Ok((report, relative_residual))
}

fn stable_complex_l2_norm(values: &[Complex64]) -> Value {
    values
        .iter()
        .fold(0.0, |norm, value| norm.hypot(value.re).hypot(value.im))
}

impl HbSolver {
    /// Full nonlinear HB solve with cooperative cancellation.
    pub fn solve_newton_with_abort(
        &mut self,
        state: &mut HbSolverState,
        abort: &dyn AbortSignal,
    ) -> Result<(), HbError> {
        self.solve_newton_with_abort_seed_policy(state, abort, HbDcSeedPolicy::Enabled)
    }

    /// Nonlinear HB solve after the engine has resolved the authored
    /// initialization policy. Direct-frequency-domain Xyce HB (`TAHB=0`)
    /// must enter Newton from the supplied state without constructing a DC
    /// operating-point trajectory first.
    pub(crate) fn solve_newton_with_abort_seed_policy(
        &mut self,
        state: &mut HbSolverState,
        abort: &dyn AbortSignal,
        dc_seed_policy: HbDcSeedPolicy,
    ) -> Result<(), HbError> {
        self.validate_configuration()?;
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let tol = self.config.tolerance;
        let abstol = self.config.abstol;

        // For linear circuits, use direct solve
        if !self.has_nonlinear_devices() {
            return self.solve_linear(state);
        }

        self.validate_exact_large_signal_mna()?;
        state.try_prepare_mna_branches(self.exact_mna_branches().len(), self.num_harmonics)?;

        // GMIN is a continuation aid, never part of the authored circuit.
        // Commercial SPICE implementations may walk a shunted homotopy, but
        // the result is accepted only after Newton converges on the physical
        // zero-GMIN equations.
        let target_gmin = 0.0;
        let homotopy_floor = 1.0e-12;

        if dc_seed_policy == HbDcSeedPolicy::Enabled {
            // Step 0: Solve DC operating point first. This is the historical
            // RSpice default when no TAHB mode is authored.
            match self.solve_dc_operating_point_with_abort(state, abort) {
                Ok(_dc_solution) => {
                    // DC solution is now stored in state.x[node][0]. Initialize
                    // harmonic components to zero around that operating point.
                    for node in 0..self.num_nodes {
                        if node < state.x.len() {
                            for k in 1..state.x[node].len() {
                                state.x[node][k] = Complex64::new(0.0, 0.0);
                            }
                        }
                    }
                }
                Err(HbError::ConvergenceFailed { .. } | HbError::SingularMatrix) => {}
                Err(err) => return Err(err),
            }
            // If the DC seed does not converge, continue with the existing
            // fallback strategy. Deterministic model/runtime faults have
            // already returned above.
        }

        // Step 1: Try direct Newton first
        if self.newton_inner_loop(
            state,
            target_gmin,
            self.config.max_iterations,
            tol,
            abstol,
            1.0,
            abort,
        )? {
            state.converged = true;
            return Ok(());
        }

        // Step 2: If direct Newton fails, try with progressively larger GMIN
        // This helps regularize ill-conditioned Jacobians
        // Include high GMIN levels (0.1, 1.0) for very difficult circuits
        for gmin_level in [1e-6, 1e-4, 1e-2, 0.1, 1.0] {
            if self.newton_inner_loop(
                state,
                gmin_level,
                self.config.max_iterations,
                tol * 10.0,
                abstol,
                1.0,
                abort,
            )? {
                // Converged at higher GMIN - now refine with progressively lower GMIN
                // Save state before refinement in case we need to restore
                let mut last_good_state = HbNewtonCheckpoint {
                    node_voltages: state.x.clone(),
                    branch_currents: state.mna_branch_currents.clone(),
                };
                let mut current_gmin = gmin_level;
                while current_gmin > homotopy_floor {
                    // Use factor of 2 for very gradual refinement
                    current_gmin /= 2.0;
                    if self.newton_inner_loop(
                        state,
                        current_gmin,
                        self.config.max_iterations,
                        tol,
                        abstol,
                        1.0,
                        abort,
                    )? {
                        // Success - update last good state
                        last_good_state = HbNewtonCheckpoint {
                            node_voltages: state.x.clone(),
                            branch_currents: state.mna_branch_currents.clone(),
                        };
                    } else {
                        // Failed - restore last good state and stop refining
                        state.x = last_good_state.node_voltages;
                        state.mna_branch_currents = last_good_state.branch_currents;
                        break;
                    }
                }
                // The continuation state is only an initial guess. Require a
                // complete Newton solve and residual check on the unmodified
                // circuit before publishing it.
                if self.newton_inner_loop(
                    state,
                    target_gmin,
                    self.config.max_iterations,
                    tol,
                    abstol,
                    1.0,
                    abort,
                )? {
                    state.converged = true;
                    return Ok(());
                }
            }
        }

        // Step 3: Try source stepping
        // Scale sources from 0 to full, using previous converged solution as starting point
        let mut source_stepper = SourceStepper::new();
        let mut total_iterations = 0; // Reset for source stepping
        let max_total_iter = self.config.max_iterations * 20;

        // Reset state to zero - with sources=0, solution=0 trivially
        for node in 0..self.num_nodes {
            if node < state.x.len() {
                for k in 0..state.x[node].len() {
                    state.x[node][k] = Complex64::new(0.0, 0.0);
                }
            }
        }
        for spectrum in &mut state.mna_branch_currents {
            spectrum.fill(Complex64::new(0.0, 0.0));
        }

        while !source_stepper.is_complete() && total_iterations < max_total_iter {
            let factor = source_stepper.factor();

            // Don't reset state.x - keep converged solution from previous source level

            // Try Newton at this source level (using previous solution as starting point)
            let converged = self.newton_inner_loop(
                state,
                target_gmin,
                self.config.max_iterations / 2,
                tol * 10.0,
                abstol,
                factor,
                abort,
            )?;

            total_iterations += state.iteration;

            if converged {
                // Keep state.x as-is for next step (converged solution)
                source_stepper.advance_on_success();
            } else {
                if !source_stepper.reduce_on_failure() {
                    break;
                }
            }
        }

        // If source stepping completed, do final Newton with original sources
        if source_stepper.is_complete()
            && self.newton_inner_loop(
                state,
                target_gmin,
                self.config.max_iterations,
                tol,
                abstol,
                1.0,
                abort,
            )?
        {
            state.converged = true;
            state.iteration = total_iterations;
            return Ok(());
        }

        // Step 4: Try pseudo-transient
        // Add damping capacitors to each node and integrate to steady-state
        let mut ptran = PseudoTransient::new();
        let mut ptran_iterations = 0;
        let max_ptran_iter = self.config.max_iterations * 5;

        while !ptran.is_complete() && ptran_iterations < max_ptran_iter {
            // Pseudo-transient adds G_eq = C_pseudo/dt to each node diagonal
            // This damps oscillations and helps find DC solution
            let ptran_gmin = target_gmin + ptran.conductance(0);

            let converged = self.newton_inner_loop(
                state,
                ptran_gmin,
                self.config.max_iterations / 4,
                tol * 100.0, // Relaxed tolerance during stepping
                abstol,
                1.0,
                abort,
            )?;

            ptran_iterations += state.iteration;

            if converged {
                ptran.advance_on_success();
            } else {
                if !ptran.reduce_on_failure() {
                    break;
                }
            }
        }

        // If pseudo-transient completed, do final high-accuracy Newton
        if ptran.is_complete()
            && self.newton_inner_loop(
                state,
                target_gmin,
                self.config.max_iterations,
                tol,
                abstol,
                1.0,
                abort,
            )?
        {
            state.converged = true;
            state.iteration = total_iterations + ptran_iterations;
            return Ok(());
        }

        Err(HbError::ConvergenceFailed {
            iterations: total_iterations + ptran_iterations,
            residual: state.residual_norm,
        })
    }

    /// Inner Newton iteration loop at a fixed GMIN level
    fn newton_inner_loop(
        &mut self,
        state: &mut HbSolverState,
        gmin: Value,
        max_iter: usize,
        tol: Value,
        abstol: Value,
        source_scale: Value,
        abort: &dyn AbortSignal,
    ) -> Result<bool, HbError> {
        for iter in 0..max_iter {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            state.iteration = iter;
            state.total_iterations += 1;

            // 1. Compute full residual: linear + nonlinear + GMIN contributions
            self.compute_full_residual_with_gmin(state, gmin, source_scale)?;

            // 2. Check convergence: per-row KCL test. A global norm hides a
            // microamp imbalance at a high-impedance node behind the amp
            // scale of stiff source rows, accepting grossly wrong bias.
            if state.rows_converged_with_branch_tolerances(tol, abstol, crate::constants::VNTOL) {
                return Ok(true);
            }

            // 3+4. Build the Jacobian and solve J * dX = -R. The exact path
            // carries the conjugate (Hankel) coupling in a real-split system
            // and restores quadratic convergence; the Toeplitz-only complex
            // path remains selectable for A/B comparison and for the large-
            // system Krylov fast path.
            let delta_x = if self.config.use_exact_jacobian {
                match self.solve_jacobian_system_exact(state, gmin, abort) {
                    Ok(dx) => dx,
                    Err(HbError::SingularMatrix) => return Ok(false),
                    Err(err) => return Err(err),
                }
            } else {
                let jacobian = self.build_full_jacobian_with_gmin(state, gmin)?;
                match self.solve_jacobian_system(&jacobian, state) {
                    Ok(dx) => dx,
                    Err(HbError::SingularMatrix) => return Ok(false),
                    Err(err) => return Err(err),
                }
            };

            // 5. Apply line search for robust convergence
            match self.apply_line_search_with_gmin(
                state,
                &delta_x,
                gmin,
                source_scale,
                tol,
                abstol,
                abort,
            ) {
                Ok(()) => {}
                Err(HbError::SingularMatrix) => return Ok(false),
                Err(err) => return Err(err),
            }
        }

        Ok(false) // Max iterations reached
    }

    /// Compute full residual including GMIN contribution
    ///
    /// Residual = I_source - Y*V - gmin*V - I_nonlinear
    /// (KCL: sum of currents INTO node = 0)
    fn compute_full_residual_with_gmin(
        &mut self,
        state: &mut HbSolverState,
        gmin: Value,
        source_scale: Value,
    ) -> Result<(), HbError> {
        if !source_scale.is_finite() || !(0.0..=1.0).contains(&source_scale) {
            return Err(HbError::InvalidCircuit(format!(
                "nonlinear HB source scale must be finite and within 0..=1, found {source_scale:e}"
            )));
        }
        // Start with linear residual (I_source - Y*V)
        self.compute_linear_residual(state);
        if source_scale != 1.0 {
            for (node, spectrum) in self.source_spectra.iter().enumerate() {
                for (harmonic, &source) in spectrum.iter().enumerate() {
                    state.residual[node][harmonic] += (source_scale - 1.0) * source;
                    state.residual_scale[node][harmonic] =
                        (state.residual_scale[node][harmonic] - source.norm()).max(0.0)
                            + (source_scale * source).norm();
                }
            }
        }
        self.add_exact_mna_residual(state, source_scale)?;

        // Subtract GMIN contribution: I_gmin = gmin * V (current leaves node via GMIN)
        for node in 0..self.num_nodes {
            for k in 0..=self.num_harmonics {
                if node < state.residual.len() && k < state.residual[node].len() {
                    state.residual[node][k] -= gmin * state.x[node][k];
                    state.residual_scale[node][k] += gmin * state.x[node][k].norm();
                }
            }
        }

        // Subtract nonlinear device currents (evaluated in time domain via FFT)
        // Note: add_nonlinear_residual adds currents with correct sign already
        if self.has_nonlinear_devices() {
            self.add_nonlinear_residual(state)?;
        }
        if !state.residual_norm.is_finite()
            || state
                .residual_scale
                .iter()
                .chain(&state.mna_branch_residual_scale)
                .flatten()
                .any(|scale| !scale.is_finite() || *scale < 0.0)
        {
            return Err(HbError::InvalidCircuit(
                "nonlinear HB residual certificate contains a non-finite value".to_string(),
            ));
        }
        Ok(())
    }

    /// Add canonical exact-MNA KCL incidence and KVL branch equations to the
    /// full-spectrum residual. Nonlinear current and charge contributions are
    /// node-only and are accumulated after this seam.
    fn add_exact_mna_residual(
        &self,
        state: &mut HbSolverState,
        source_scale: Value,
    ) -> Result<(), HbError> {
        let harmonic_count = self.num_harmonics + 1;
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        if state.mna_branch_currents.len() != self.exact_mna_branches().len()
            || state.mna_branch_residual.len() != self.exact_mna_branches().len()
            || state.mna_branch_residual_scale.len() != self.exact_mna_branches().len()
        {
            return Err(HbError::InvalidCircuit(
                "nonlinear HB exact-MNA state does not match the canonical branch registry"
                    .to_string(),
            ));
        }
        for row in &mut state.mna_branch_residual {
            row.fill(Complex64::new(0.0, 0.0));
        }
        for row in &mut state.mna_branch_residual_scale {
            row.fill(0.0);
        }

        for (branch_index, branch) in self.exact_mna_branches().iter().enumerate() {
            let currents = &state.mna_branch_currents[branch_index];
            if currents.len() != harmonic_count {
                return Err(HbError::InvalidCircuit(format!(
                    "nonlinear HB branch {} current spectrum has {} harmonics; expected {harmonic_count}",
                    branch_index + 1,
                    currents.len()
                )));
            }
            let (_, node_pos, node_neg) = branch.ordinal_and_terminals();
            for (harmonic, &current) in currents.iter().enumerate() {
                if harmonic == 0 && current.im != 0.0 {
                    return Err(HbError::InvalidCircuit(format!(
                        "nonlinear HB branch {} has a nonzero imaginary DC current",
                        branch_index + 1
                    )));
                }
                if node_pos > 0 {
                    state.residual[node_pos - 1][harmonic] -= current;
                    state.residual_scale[node_pos - 1][harmonic] += current.norm();
                }
                if node_neg > 0 {
                    state.residual[node_neg - 1][harmonic] += current;
                    state.residual_scale[node_neg - 1][harmonic] += current.norm();
                }

                let mut voltage_drop = Complex64::new(0.0, 0.0);
                let mut voltage_scale = 0.0;
                if node_pos > 0 {
                    let voltage = state.x[node_pos - 1][harmonic];
                    voltage_drop += voltage;
                    voltage_scale += voltage.norm();
                }
                if node_neg > 0 {
                    let voltage = state.x[node_neg - 1][harmonic];
                    voltage_drop -= voltage;
                    voltage_scale += voltage.norm();
                }
                let (residual, constitutive_scale) = match branch {
                    ExactMnaBranch::VoltageSource { source, .. } => {
                        let source = source.as_ref().ok_or_else(|| {
                            HbError::InvalidCircuit(format!(
                                "nonlinear HB voltage branch {} has no authored source spectrum",
                                branch_index + 1
                            ))
                        })?;
                        let source_value =
                            source_scale * Self::voltage_source_value_at_harmonic(source, harmonic);
                        (source_value - voltage_drop, source_value.norm())
                    }
                    ExactMnaBranch::Inductor { inductance, .. } => {
                        let impedance_current =
                            Complex64::new(0.0, harmonic as Value * omega0 * *inductance) * current;
                        (impedance_current - voltage_drop, impedance_current.norm())
                    }
                    ExactMnaBranch::Resistor { resistance, .. } => {
                        let resistor_voltage = current * *resistance;
                        (resistor_voltage - voltage_drop, resistor_voltage.norm())
                    }
                    ExactMnaBranch::ControlledVoltageSource { .. } => (-voltage_drop, 0.0),
                    ExactMnaBranch::NetworkPort { .. } => (Complex64::new(0.0, 0.0), 0.0),
                };
                if !residual.re.is_finite()
                    || !residual.im.is_finite()
                    || !voltage_scale.is_finite()
                    || !constitutive_scale.is_finite()
                {
                    return Err(HbError::InvalidCircuit(format!(
                        "nonlinear HB branch {} produced a non-finite KVL residual",
                        branch_index + 1
                    )));
                }
                state.mna_branch_residual[branch_index][harmonic] = residual;
                state.mna_branch_residual_scale[branch_index][harmonic] =
                    voltage_scale + constitutive_scale;
            }
        }
        for &(row, column, coefficient) in &self.exact_mna_static_entries {
            for harmonic in 0..harmonic_count {
                let input = if column < self.num_nodes {
                    state.x[column][harmonic]
                } else {
                    state.mna_branch_currents[column - self.num_nodes][harmonic]
                };
                let contribution = coefficient * input;
                if row < self.num_nodes {
                    state.residual[row][harmonic] -= contribution;
                    state.residual_scale[row][harmonic] += contribution.norm();
                } else {
                    let branch = row - self.num_nodes;
                    state.mna_branch_residual[branch][harmonic] -= contribution;
                    state.mna_branch_residual_scale[branch][harmonic] += contribution.norm();
                }
            }
        }
        for &(row, column, inductance) in &self.exact_mna_inductance_entries {
            let branch = row - self.num_nodes;
            let control_branch = column - self.num_nodes;
            for harmonic in 0..harmonic_count {
                let contribution = Complex64::new(0.0, harmonic as Value * omega0 * inductance)
                    * state.mna_branch_currents[control_branch][harmonic];
                state.mna_branch_residual[branch][harmonic] += contribution;
                state.mna_branch_residual_scale[branch][harmonic] += contribution.norm();
            }
        }
        let unknowns = self.num_nodes + self.exact_mna_branches().len();
        for harmonic in 0..harmonic_count {
            let omega = harmonic as Value * omega0;
            for network in &self.exact_periodic_networks {
                network.try_visit_direct_entries(omega, unknowns, |row, column, value| {
                    let input = if column < self.num_nodes {
                        state.x[column][harmonic]
                    } else {
                        state.mna_branch_currents[column - self.num_nodes][harmonic]
                    };
                    let contribution = value * input;
                    if row < self.num_nodes {
                        state.residual[row][harmonic] -= contribution;
                        state.residual_scale[row][harmonic] += contribution.norm();
                    } else {
                        let branch = row - self.num_nodes;
                        state.mna_branch_residual[branch][harmonic] -= contribution;
                        state.mna_branch_residual_scale[branch][harmonic] += contribution.norm();
                    }
                })?;
            }
        }
        state.compute_residual_norm();
        Ok(())
    }

    /// Build Jacobian with GMIN on diagonal
    ///
    /// Residual = I_source - Y*V - gmin*V, so J = ∂res/∂V = -Y - gmin
    fn build_full_jacobian_with_gmin(
        &mut self,
        state: &HbSolverState,
        gmin: Value,
    ) -> Result<Vec<Vec<Complex64>>, HbError> {
        let mut jac = self.build_full_jacobian(state)?;

        // Subtract GMIN from all diagonal entries (consistent with residual -= gmin*V)
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        for i in 0..n {
            for k in 0..h {
                let idx = i * h + k;
                if idx < jac.len() {
                    jac[idx][idx] -= gmin;
                }
            }
        }

        Ok(jac)
    }

    /// Apply line search with GMIN and PN voltage limiting
    ///
    /// Advanced implementation following standard methodology:
    /// - Armijo backtracking line search
    /// - PN junction voltage limiting on DC component
    fn apply_line_search_with_gmin(
        &mut self,
        state: &mut HbSolverState,
        delta: &HbNewtonStep,
        gmin: Value,
        source_scale: Value,
        reltol: Value,
        current_abstol: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), HbError> {
        let initial_merit =
            state.certificate_merit(reltol, current_abstol, crate::constants::VNTOL, false)?;
        let armijo_c = 1e-4;
        let min_alpha = 0.01;
        let vt = 0.02585; // Thermal voltage at 300K

        let mut alpha = 1.0;
        let mut best_alpha = alpha;
        let mut best_merit = f64::INFINITY;

        let x_orig = state.x.clone();
        let branch_orig = state.mna_branch_currents.clone();
        let harmonic_count = self.num_harmonics + 1;
        if delta.node_voltages.len() != self.num_nodes
            || delta
                .node_voltages
                .iter()
                .any(|row| row.len() != harmonic_count)
            || delta.branch_currents.len() != self.exact_mna_branches().len()
            || delta
                .branch_currents
                .iter()
                .any(|row| row.len() != harmonic_count)
        {
            return Err(HbError::InvalidCircuit(
                "nonlinear HB line search received a malformed node/branch Newton step".to_string(),
            ));
        }

        while alpha >= min_alpha {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            for (node, dx_node) in delta.node_voltages.iter().enumerate() {
                for (k, &dx) in dx_node.iter().enumerate() {
                    if node < state.x.len() && k < state.x[node].len() {
                        let v_old = x_orig[node][k].re;
                        let v_new_raw = v_old + alpha * dx.re;

                        // Apply PN voltage limiting to DC component only
                        let v_new = if k == 0 {
                            limit_pn_voltage(v_old, v_new_raw, vt)
                        } else {
                            v_new_raw
                        };

                        // The DC coefficient of a real waveform has no
                        // imaginary degree of freedom. Keeping a stale or
                        // updated imaginary part here would make the line
                        // search evaluate a state that the realified Newton
                        // system cannot represent.
                        let im_new = if k == 0 {
                            0.0
                        } else {
                            x_orig[node][k].im + alpha * dx.im
                        };
                        state.x[node][k] = Complex64::new(v_new, im_new);
                    }
                }
            }
            for (branch, delta_spectrum) in delta.branch_currents.iter().enumerate() {
                for (harmonic, &delta_current) in delta_spectrum.iter().enumerate() {
                    let value = branch_orig[branch][harmonic] + alpha * delta_current;
                    state.mna_branch_currents[branch][harmonic] = if harmonic == 0 {
                        Complex64::new(value.re, 0.0)
                    } else {
                        value
                    };
                }
            }

            self.compute_full_residual_with_gmin(state, gmin, source_scale)?;
            let merit =
                state.certificate_merit(reltol, current_abstol, crate::constants::VNTOL, false)?;

            if merit < initial_merit * (1.0 - armijo_c * alpha) {
                return Ok(());
            }

            if merit < best_merit {
                best_merit = merit;
                best_alpha = alpha;
            }

            alpha *= 0.5;
        }

        // Use best step found with voltage limiting
        for (node, dx_node) in delta.node_voltages.iter().enumerate() {
            for (k, &dx) in dx_node.iter().enumerate() {
                if node < state.x.len() && k < state.x[node].len() {
                    let v_old = x_orig[node][k].re;
                    let v_new_raw = v_old + best_alpha * dx.re;

                    let v_new = if k == 0 {
                        limit_pn_voltage(v_old, v_new_raw, vt)
                    } else {
                        v_new_raw
                    };

                    let im_new = if k == 0 {
                        0.0
                    } else {
                        x_orig[node][k].im + best_alpha * dx.im
                    };
                    state.x[node][k] = Complex64::new(v_new, im_new);
                }
            }
        }
        for (branch, delta_spectrum) in delta.branch_currents.iter().enumerate() {
            for (harmonic, &delta_current) in delta_spectrum.iter().enumerate() {
                let value = branch_orig[branch][harmonic] + best_alpha * delta_current;
                state.mna_branch_currents[branch][harmonic] = if harmonic == 0 {
                    Complex64::new(value.re, 0.0)
                } else {
                    value
                };
            }
        }
        self.compute_full_residual_with_gmin(state, gmin, source_scale)?;

        Ok(())
    }

    /// Add nonlinear device contributions to residual
    fn add_nonlinear_residual(&mut self, state: &mut HbSolverState) -> Result<(), HbError> {
        let n_time = self.fft.size();

        // Convert spectral voltages to time domain
        let v_time: Vec<Vec<Value>> = (0..self.num_nodes)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Accumulate nonlinear currents at each time point
        let mut i_time = vec![vec![0.0; n_time]; self.num_nodes];

        if !self.nonlinear_devices.is_empty() {
            let mut node_voltages = vec![0.0; self.num_nodes];
            for t in 0..n_time {
                for node in 0..self.num_nodes {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for (node, current) in device.evaluate(&node_voltages) {
                        if node < i_time.len() {
                            i_time[node][t] += current;
                        }
                    }
                }
            }
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            let mut circuit_voltages = vec![0.0; self.num_nodes];
            for t in 0..n_time {
                for node in 0..self.num_nodes {
                    circuit_voltages[node] = v_time[node][t];
                }
                for device in &mut self.veriloga_nonlinear_devices {
                    device.device.update_all_voltages(&circuit_voltages);
                    let values = device.try_evaluate("time-domain residual evaluation")?;
                    for (program_idx, value) in values.iter().enumerate() {
                        let Some(rows) = device.rhs_rows.get(program_idx) else {
                            continue;
                        };
                        for &(row, sign) in rows {
                            if row < self.num_nodes {
                                i_time[row][t] += sign * *value;
                            }
                        }
                    }
                }
            }
        }

        // Convert nonlinear currents to frequency domain and ADD to residual
        // Device returns stamped contribution (current INTO node, already with correct sign)
        for (node, i_waveform) in i_time.iter().enumerate().take(self.num_nodes) {
            let i_spectrum = self.fft.to_frequency_domain(i_waveform);
            for (k, &i_k) in i_spectrum.iter().enumerate() {
                if k <= self.num_harmonics && node < state.residual.len() {
                    state.residual[node][k] += i_k;
                    state.residual_scale[node][k] += i_k.norm();
                }
            }
        }

        // Charge storage: the capacitive current delivered into a node is
        // d/dt of the delivered charge, i.e. jw_k * Q_k per harmonic. The
        // charge waveform comes from the same time grid as the resistive
        // currents, so charge and current stay phase-consistent.
        if self
            .nonlinear_devices
            .iter()
            .any(|d| d.has_charge_storage())
        {
            let omega0 = 2.0 * PI * self.config.fundamental_freq;
            let mut q_time = vec![vec![0.0; n_time]; self.num_nodes];
            let mut node_voltages = vec![0.0; self.num_nodes];
            for t in 0..n_time {
                for node in 0..self.num_nodes {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for (node, charge) in device.charge(&node_voltages) {
                        if node < q_time.len() {
                            q_time[node][t] += charge;
                        }
                    }
                }
            }

            for (node, q_waveform) in q_time.iter().enumerate().take(self.num_nodes) {
                let q_spectrum = self.fft.to_frequency_domain(q_waveform);
                for (k, &q_k) in q_spectrum.iter().enumerate() {
                    if k <= self.num_harmonics && node < state.residual.len() {
                        let omega_k = (k as f64) * omega0;
                        state.residual[node][k] += Complex64::new(0.0, omega_k) * q_k;
                        state.residual_scale[node][k] += omega_k * q_k.norm();
                    }
                }
            }
        }

        state.compute_residual_norm();
        Ok(())
    }

    /// Build full Jacobian matrix for Newton iteration
    ///
    /// Structure: block matrix [node_i, k][node_j, l] where:
    /// - Diagonal blocks (k == l): linear admittance + linearized nonlinear
    /// - Off-diagonal blocks: nonlinear coupling via FFT convolution
    ///
    /// For efficiency, we flatten to one complex matrix over canonical
    /// `[node + branch][harmonic]` coordinates. Nonlinear blocks remain
    /// strictly node-to-node.
    fn build_full_jacobian(
        &mut self,
        state: &HbSolverState,
    ) -> Result<Vec<Vec<Complex64>>, HbError> {
        let n = self.num_nodes;
        let branch_count = self.exact_mna_branches().len();
        let h = self.num_harmonics + 1;
        let entity_count = n.checked_add(branch_count).ok_or_else(|| {
            HbError::InvalidCircuit("nonlinear HB MNA dimension exceeds this platform".to_string())
        })?;
        let size = entity_count.checked_mul(h).ok_or_else(|| {
            HbError::InvalidCircuit(
                "nonlinear HB spectral MNA dimension exceeds this platform".to_string(),
            )
        })?;
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Initialize Jacobian
        let mut jac = vec![vec![Complex64::new(0.0, 0.0); size]; size];

        // --- Linear part: block-diagonal per harmonic ---
        // Residual = I_source - Y*V, so J = ∂res/∂V = -Y
        for k in 0..h {
            let omega_k = (k as f64) * omega0;

            // Conductance contribution: -G (negative because residual = ... - G*V)
            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    let row = i * h + k;
                    let col = j * h + k;
                    jac[row][col] -= g;
                }
            }

            // Capacitance contribution: -jωC
            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    let row = i * h + k;
                    let col = j * h + k;
                    jac[row][col] -= Complex64::new(0.0, omega_k) * c;
                }
            }

            // Inductance contribution: -1/(jωL)
            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    let row = i * h + k;
                    let col = j * h + k;
                    if k == 0 {
                        // DC: short circuit
                        jac[row][col] -= inductor_dc_short_admittance(l);
                    } else {
                        // AC: Y_L = -j/(ωL)
                        jac[row][col] -= Complex64::new(0.0, -1.0 / (omega_k * l));
                    }
                }
            }

            // Exact MNA incidence and branch constitutive equations. The
            // residual convention is source-minus-current for KCL and
            // authored/constitutive voltage minus terminal drop for KVL.
            for (branch_index, branch) in self.exact_mna_branches().iter().enumerate() {
                let branch_entity = n + branch_index;
                let branch_coordinate = branch_entity * h + k;
                let (_, node_pos, node_neg) = branch.ordinal_and_terminals();
                if node_pos > 0 {
                    let node_coordinate = (node_pos - 1) * h + k;
                    jac[node_coordinate][branch_coordinate] -= 1.0;
                    if !matches!(branch, ExactMnaBranch::NetworkPort { .. }) {
                        jac[branch_coordinate][node_coordinate] -= 1.0;
                    }
                }
                if node_neg > 0 {
                    let node_coordinate = (node_neg - 1) * h + k;
                    jac[node_coordinate][branch_coordinate] += 1.0;
                    if !matches!(branch, ExactMnaBranch::NetworkPort { .. }) {
                        jac[branch_coordinate][node_coordinate] += 1.0;
                    }
                }
                if let ExactMnaBranch::Inductor { inductance, .. } = branch {
                    jac[branch_coordinate][branch_coordinate] +=
                        Complex64::new(0.0, omega_k * *inductance);
                }
                if let ExactMnaBranch::Resistor { resistance, .. } = branch {
                    jac[branch_coordinate][branch_coordinate] += *resistance;
                }
            }
            for &(row, column, value) in &self.exact_mna_static_entries {
                jac[row * h + k][column * h + k] -= value;
            }
            for &(row, column, inductance) in &self.exact_mna_inductance_entries {
                jac[row * h + k][column * h + k] += Complex64::new(0.0, omega_k * inductance);
            }
            for network in &self.exact_periodic_networks {
                network.try_visit_direct_entries(omega_k, entity_count, |row, column, value| {
                    jac[row * h + k][column * h + k] -= value;
                })?;
            }
        }

        // --- Nonlinear part: requires FFT-based evaluation ---
        if self.has_nonlinear_devices() {
            self.add_nonlinear_jacobian(&mut jac, state)?;
        }

        Ok(jac)
    }

    /// Add nonlinear Jacobian contributions via FFT (Toeplitz/convolution)
    ///
    /// For nonlinear devices, the frequency-domain Jacobian is a Toeplitz matrix
    /// representing convolution: J`[k,l]` = G[k-l] where G is the DFT of g(t).
    ///
    /// This is the implementation that exactly matches the
    /// FFT-based residual computation, ensuring proper Newton convergence.
    fn add_nonlinear_jacobian(
        &mut self,
        jac: &mut [Vec<Complex64>],
        state: &HbSolverState,
    ) -> Result<(), HbError> {
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let n_time = self.fft.size();

        // Convert voltages to time domain
        let v_time: Vec<Vec<Value>> = (0..n)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Accumulate conductance stamps in time domain for each node pair
        let mut g_time = vec![vec![vec![0.0; n_time]; n]; n]; // [i][j][t]

        if !self.nonlinear_devices.is_empty() {
            let mut node_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for ((i, j), g) in device.jacobian(&node_voltages) {
                        if i < n && j < n {
                            g_time[i][j][t] += g;
                        }
                    }
                }
            }
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            let mut circuit_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    circuit_voltages[node] = v_time[node][t];
                }
                for device in &mut self.veriloga_nonlinear_devices {
                    device.device.update_all_voltages(&circuit_voltages);
                    let jac_entries =
                        device.try_compute_jacobian("time-domain Jacobian evaluation")?;
                    for entry in jac_entries {
                        let Some(prog_locs) = device.jacobian_locs.get(entry.program_idx) else {
                            continue;
                        };
                        let Some(&(row, col)) = prog_locs.get(entry.jacobian_idx) else {
                            continue;
                        };
                        if let (Some(i), Some(j)) = (row, col)
                            && i < n
                            && j < n
                        {
                            g_time[i][j][t] += entry.value;
                        }
                    }
                }
            }
        }

        // Convert each conductance waveform to frequency domain (Toeplitz row)
        // Then build the proper convolution Jacobian
        for (i, g_row) in g_time.iter().enumerate().take(n) {
            for (j, g_waveform) in g_row.iter().enumerate().take(n) {
                // Check if there's any significant conductance
                let max_g: Value = g_waveform.iter().fold(0.0, |a, &b| a.max(b.abs()));
                if max_g < 1e-30 {
                    continue;
                }

                // FFT the conductance waveform to get G[k] spectrum
                let g_spectrum = self.fft.to_frequency_domain(g_waveform);

                // Build Toeplitz block for this (i,j) node pair
                // J[i*h+k][j*h+l] = G[k-l] (with periodic extension for negative indices)
                for k in 0..h {
                    for l in 0..h {
                        let row = i * h + k;
                        let col = j * h + l;

                        // Compute index for G[k-l] with wrap-around
                        let diff = k as isize - l as isize;
                        let g_idx = if diff >= 0 {
                            diff as usize
                        } else {
                            // Negative index - use conjugate symmetry: G[-m] = G[m]*
                            // For real g(t), G[-m] = conj(G[m])
                            (-diff) as usize
                        };

                        if g_idx < g_spectrum.len() {
                            let g_val = if diff >= 0 {
                                g_spectrum[g_idx]
                            } else {
                                // Use conjugate for negative frequency
                                g_spectrum[g_idx].conj()
                            };
                            // SUBTRACT device Jacobian for KCL: residual = I_source - I_device
                            // So J = ∂res/∂V = -∂I_device/∂V = -gd
                            jac[row][col] -= g_val;
                        }
                    }
                }
            }
        }

        // Charge-storage coupling: the residual carries jw_k * Q_k, so its
        // derivative is jw_k * C[k-m] - the same Toeplitz structure as the
        // conductances with the ROW harmonic's frequency in front.
        if self
            .nonlinear_devices
            .iter()
            .any(|d| d.has_charge_storage())
        {
            let omega0 = 2.0 * PI * self.config.fundamental_freq;
            let mut c_time = vec![vec![vec![0.0; n_time]; n]; n];
            let mut node_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for ((i, j), c) in device.charge_jacobian(&node_voltages) {
                        if i < n && j < n {
                            c_time[i][j][t] += c;
                        }
                    }
                }
            }

            for (i, c_row) in c_time.iter().enumerate().take(n) {
                for (j, c_waveform) in c_row.iter().enumerate().take(n) {
                    let max_c: Value = c_waveform.iter().fold(0.0, |a, &b| a.max(b.abs()));
                    if max_c < 1e-30 {
                        continue;
                    }

                    let c_spectrum = self.fft.to_frequency_domain(c_waveform);

                    for k in 0..h {
                        let omega_k = (k as f64) * omega0;
                        let jw = Complex64::new(0.0, omega_k);
                        for l in 0..h {
                            let row = i * h + k;
                            let col = j * h + l;

                            let diff = k as isize - l as isize;
                            let c_idx = diff.unsigned_abs();
                            if c_idx < c_spectrum.len() {
                                let c_val = if diff >= 0 {
                                    c_spectrum[c_idx]
                                } else {
                                    c_spectrum[c_idx].conj()
                                };
                                // Residual carries +jw_k*Q_k; J = d(res)/dV
                                // gets -(jw_k * dQ/dV) like the linear caps.
                                jac[row][col] -= jw * c_val;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Solve the Newton step with the EXACT Jacobian in real-split form.
    ///
    /// The one-sided residual depends on both `c_m` and `c_m*` (perturbing a
    /// coefficient perturbs its implied conjugate), so the exact derivative
    /// carries a Toeplitz part `T = dI_k/dc_m = -G[k-m]` AND a Hankel part
    /// `H = dI_k/dc_m* = -G[k+m]` (plus the matching `jw_k*C` charge terms).
    /// `H` is antilinear, so the system is assembled over real unknowns
    /// `[a_0, a_1, b_1, ...]` per node (`c_k = a_k + j b_k`, `b_0 = 0`):
    /// the Toeplitz block maps to `[[Re T, -Im T], [Im T, Re T]]` and the
    /// Hankel block to `[[Re H, Im H], [Im H, -Re H]]`; the DC row keeps only
    /// its real equation and the DC column only its real unknown. Hankel
    /// indices reach `k + m = 2H`, so the coupling spectra are sampled out to
    /// twice the solution's harmonic count (alias-capped by the FFT grid).
    fn solve_jacobian_system_exact(
        &mut self,
        state: &HbSolverState,
        gmin: Value,
        abort: &dyn AbortSignal,
    ) -> Result<HbNewtonStep, HbError> {
        let n = self.num_nodes;
        let branch_count = self.exact_mna_branches().len();
        let entity_count = n.checked_add(branch_count).ok_or_else(|| {
            HbError::InvalidCircuit("nonlinear HB MNA dimension exceeds this platform".to_string())
        })?;
        let h = self.num_harmonics.checked_add(1).ok_or_else(|| {
            HbError::InvalidCircuit(
                "nonlinear HB harmonic component count exceeds this platform".to_string(),
            )
        })?;
        let w = self
            .num_harmonics
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "nonlinear HB realified harmonic width exceeds this platform".to_string(),
                )
            })?;
        let size = entity_count.checked_mul(w).ok_or_else(|| {
            HbError::InvalidCircuit(
                "nonlinear HB realified MNA dimension exceeds this platform".to_string(),
            )
        })?;
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Row/column index helpers in the real layout.
        let re_idx = |entity: usize, k: usize| -> usize {
            if k == 0 {
                entity * w
            } else {
                entity * w + 2 * k - 1
            }
        };
        let im_idx = |entity: usize, k: usize| -> usize { entity * w + 2 * k };

        // Realified RHS: -residual, DC keeps only its real equation.
        let mut rhs = vec![0.0; size];
        for node in 0..n {
            for k in 0..h {
                let r = state.residual[node][k];
                rhs[re_idx(node, k)] = -r.re;
                if k > 0 {
                    rhs[im_idx(node, k)] = -r.im;
                }
            }
        }
        for branch in 0..branch_count {
            for k in 0..h {
                let residual = state.mna_branch_residual[branch][k];
                let entity = n + branch;
                rhs[re_idx(entity, k)] = -residual.re;
                if k > 0 {
                    rhs[im_idx(entity, k)] = -residual.im;
                }
            }
        }

        // Large exact systems take the branch-aware matrix-free route. A
        // candidate is accepted only after the matrix package independently
        // certifies its componentwise backward error from the streamed
        // operator entries. Numerical failure may recover through dense
        // elimination only below the automatic Krylov threshold; structural,
        // non-finite, allocation, and large-system failures are fail-closed.
        let try_krylov = self.config.use_krylov || size >= super::krylov::KRYLOV_AUTO_THRESHOLD;
        if try_krylov && entity_count > 0 {
            let extended = self.num_harmonics.checked_mul(2).ok_or_else(|| {
                HbError::InvalidCircuit(
                    "nonlinear HB coupling span exceeds this platform".to_string(),
                )
            })?;
            let g_spectra = if self.has_nonlinear_devices() {
                self.conductance_spectra(state, extended)?
            } else {
                Vec::new()
            };
            let c_spectra = if self.has_nonlinear_devices() {
                self.capacitance_spectra(state, extended)?
            } else {
                Vec::new()
            };
            let operator = ExactHbOperator {
                num_nodes: n,
                num_components: h,
                real_width: w,
                omega0,
                gmin,
                g_matrix: &self.g_matrix,
                c_matrix: &self.c_matrix,
                l_matrix: &self.l_matrix,
                g_spectra: &g_spectra,
                c_spectra: &c_spectra,
                mna_branches: self.exact_mna_branches(),
                mna_static_entries: &self.exact_mna_static_entries,
                mna_inductance_entries: &self.exact_mna_inductance_entries,
                periodic_networks: &self.exact_periodic_networks,
            };
            operator.validate()?;
            let preconditioner = ExactHbPreconditioner::build(&operator);
            let rhs_complex = rhs
                .iter()
                .map(|&value| Complex64::new(value, 0.0))
                .collect::<Vec<_>>();
            let restart = super::krylov::bounded_gmres_restart(self.config.gmres_restart, size);
            let mut outcome = super::krylov::gmres_with_abort(
                &|input| operator.apply(input),
                &preconditioner,
                &rhs_complex,
                restart,
                6,
                &|| abort.is_aborted(),
            )
            .map_err(|_| HbError::Aborted)?;
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let mut report = None;
            let mut qualification = if outcome.solution.len() != size {
                Err(rspice_matrix::SolverError::InvalidCircuit(format!(
                    "exact HB GMRES returned {} values for a {size}-unknown system",
                    outcome.solution.len()
                )))
            } else if outcome
                .solution
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
                || !outcome.relative_residual.is_finite()
            {
                Err(rspice_matrix::SolverError::Overflow)
            } else if outcome.converged {
                match exact_hb_candidate_report(&operator, &mut outcome.solution, &rhs_complex) {
                    Ok((candidate_report, relative_residual)) => {
                        outcome.relative_residual = relative_residual;
                        let result = if candidate_report.is_accepted() {
                            Ok(())
                        } else {
                            Err(rspice_matrix::SolverError::InaccurateSolution(
                                candidate_report.componentwise_error(),
                            ))
                        };
                        report = Some(candidate_report);
                        result
                    }
                    Err(error) => Err(error),
                }
            } else {
                Err(rspice_matrix::SolverError::ConvergenceFailed(
                    outcome.iterations,
                ))
            };

            // A global normwise GMRES tolerance can hide a poor equation in
            // mixed KCL/KVL systems. Keep the strict componentwise
            // certificate and refine only an otherwise finite, converged
            // candidate. Power-of-two row scaling makes GMRES spend its
            // residual budget in proportion to each equation's exact
            // acceptance threshold without changing the represented system.
            if matches!(
                &qualification,
                Err(rspice_matrix::SolverError::InaccurateSolution(_))
            ) && let Some(mut current_report) = report.take()
            {
                const MAX_REFINEMENTS: usize = 5;
                const MIN_IMPROVEMENT_FACTOR: Value = 0.5;
                let mut row_scale = Vec::new();
                let mut inverse_row_scale = Vec::new();
                let mut scaled_rhs = Vec::new();
                if row_scale.try_reserve_exact(size).is_err()
                    || inverse_row_scale.try_reserve_exact(size).is_err()
                    || scaled_rhs.try_reserve_exact(size).is_err()
                {
                    qualification = Err(rspice_matrix::SolverError::OutOfMemory);
                }
                for _ in 0..MAX_REFINEMENTS {
                    if matches!(&qualification, Err(rspice_matrix::SolverError::OutOfMemory)) {
                        break;
                    }
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }

                    let previous_acceptance_ratio = current_report.acceptance_ratio();
                    row_scale.clear();
                    inverse_row_scale.clear();
                    scaled_rhs.clear();
                    for row in 0..size {
                        let Some(scale) = current_report.refinement_row_scale(row) else {
                            qualification = Err(rspice_matrix::SolverError::InvalidCircuit(
                                "exact HB refinement report has inconsistent dimensions"
                                    .to_string(),
                            ));
                            break;
                        };
                        row_scale.push(scale);
                        inverse_row_scale.push(scale.recip());
                        scaled_rhs.push(current_report.residual()[row] * scale);
                    }
                    if row_scale.len() != size {
                        break;
                    }
                    if inverse_row_scale.iter().any(|value| !value.is_finite())
                        || scaled_rhs
                            .iter()
                            .any(|value| !value.re.is_finite() || !value.im.is_finite())
                    {
                        qualification = Err(rspice_matrix::SolverError::Overflow);
                        break;
                    }

                    let scaled_preconditioner = RowScaledExactHbPreconditioner {
                        base: &preconditioner,
                        inverse_row_scale: &inverse_row_scale,
                    };
                    let correction = super::krylov::gmres_with_abort(
                        &|input| {
                            let mut output = operator.apply(input);
                            for (value, &scale) in output.iter_mut().zip(&row_scale) {
                                *value *= scale;
                            }
                            output
                        },
                        &scaled_preconditioner,
                        &scaled_rhs,
                        restart,
                        6,
                        &|| abort.is_aborted(),
                    )
                    .map_err(|_| HbError::Aborted)?;
                    if abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    outcome.iterations = outcome.iterations.saturating_add(correction.iterations);
                    if correction.solution.len() != size {
                        qualification = Err(rspice_matrix::SolverError::InvalidCircuit(format!(
                            "exact HB refinement returned {} values for a {size}-unknown system",
                            correction.solution.len()
                        )));
                        break;
                    }
                    if !correction.converged {
                        qualification = Err(rspice_matrix::SolverError::ConvergenceFailed(
                            outcome.iterations,
                        ));
                        break;
                    }
                    if !correction.relative_residual.is_finite()
                        || correction
                            .solution
                            .iter()
                            .any(|value| !value.re.is_finite() || !value.im.is_finite())
                    {
                        qualification = Err(rspice_matrix::SolverError::Overflow);
                        break;
                    }
                    for (value, correction) in outcome.solution.iter_mut().zip(&correction.solution)
                    {
                        *value += correction;
                        if !value.re.is_finite() || !value.im.is_finite() {
                            qualification = Err(rspice_matrix::SolverError::Overflow);
                            break;
                        }
                    }
                    if matches!(&qualification, Err(rspice_matrix::SolverError::Overflow)) {
                        break;
                    }

                    let (refined_report, relative_residual) = match exact_hb_candidate_report(
                        &operator,
                        &mut outcome.solution,
                        &rhs_complex,
                    ) {
                        Ok(report) => report,
                        Err(error) => {
                            qualification = Err(error);
                            break;
                        }
                    };
                    outcome.relative_residual = relative_residual;
                    if refined_report.is_accepted() {
                        qualification = Ok(());
                        break;
                    }

                    qualification = Err(rspice_matrix::SolverError::InaccurateSolution(
                        refined_report.componentwise_error(),
                    ));
                    let improved = refined_report.acceptance_ratio()
                        < previous_acceptance_ratio * MIN_IMPROVEMENT_FACTOR;
                    current_report = refined_report;
                    if !improved {
                        break;
                    }
                }
            }
            match qualification {
                Ok(()) => {
                    if self.config.verbose {
                        log::debug!(
                            "HB exact matrix-free solve: {} iterations, componentwise certified \
                             (reported normwise relative residual {:.2e})",
                            outcome.iterations,
                            outcome.relative_residual
                        );
                    }
                    let mut node_voltages = vec![vec![Complex64::new(0.0, 0.0); h]; n];
                    for (node, harmonics) in node_voltages.iter_mut().enumerate().take(n) {
                        for (k, phasor) in harmonics.iter_mut().enumerate().take(h) {
                            let re = outcome.solution[re_idx(node, k)].re;
                            let im = if k > 0 {
                                outcome.solution[im_idx(node, k)].re
                            } else {
                                0.0
                            };
                            *phasor = Complex64::new(re, im);
                        }
                    }
                    let mut branch_currents = vec![vec![Complex64::new(0.0, 0.0); h]; branch_count];
                    for (branch, spectrum) in branch_currents.iter_mut().enumerate() {
                        let entity = n + branch;
                        for (k, coefficient) in spectrum.iter_mut().enumerate() {
                            let re = outcome.solution[re_idx(entity, k)].re;
                            let im = if k > 0 {
                                outcome.solution[im_idx(entity, k)].re
                            } else {
                                0.0
                            };
                            *coefficient = Complex64::new(re, im);
                        }
                    }
                    return Ok(HbNewtonStep {
                        node_voltages,
                        branch_currents,
                    });
                }
                Err(error) if exact_krylov_dense_recovery_allowed(size, &error) => {
                    log::debug!(
                        "HB exact matrix-free step was not certified after {} iterations \
                         (reported relative residual {:.2e}: {}); using bounded dense recovery",
                        outcome.iterations,
                        outcome.relative_residual,
                        error
                    );
                }
                Err(error) => {
                    return Err(HbError::InvalidCircuit(format!(
                        "HB exact {size}x{size} matrix-free Newton step is uncertified after {} \
                         iterations (reported normwise relative residual {:.3e}): {error}",
                        outcome.iterations, outcome.relative_residual
                    )));
                }
            }
        }

        // Toeplitz part (linear + GMIN + nonlinear G and charge), expanded
        // from the existing complex assembly.
        let jac_c = self.build_full_jacobian_with_gmin(state, gmin)?;
        let mut a = vec![vec![0.0; size]; size];
        for i in 0..entity_count {
            for k in 0..h {
                for j in 0..entity_count {
                    for l in 0..h {
                        let t = jac_c[i * h + k][j * h + l];
                        if t.re == 0.0 && t.im == 0.0 {
                            continue;
                        }
                        let row_re = re_idx(i, k);
                        let col_re = re_idx(j, l);
                        a[row_re][col_re] += t.re;
                        if l > 0 {
                            a[row_re][im_idx(j, l)] += -t.im;
                        }
                        if k > 0 {
                            let row_im = im_idx(i, k);
                            a[row_im][col_re] += t.im;
                            if l > 0 {
                                a[row_im][im_idx(j, l)] += t.re;
                            }
                        }
                    }
                }
            }
        }

        // Hankel part: H = -(G[k+m]) and -(jw_k * C[k+m]), m >= 1.
        if self.has_nonlinear_devices() {
            let extended = self.num_harmonics.checked_mul(2).ok_or_else(|| {
                HbError::InvalidCircuit(
                    "nonlinear HB coupling span exceeds this platform".to_string(),
                )
            })?;
            let g_spectra = self.conductance_spectra(state, extended)?;
            let c_spectra = self.capacitance_spectra(state, extended)?;

            let mut add_hankel = |i: usize, j: usize, k: usize, m: usize, hval: Complex64| {
                let row_re = re_idx(i, k);
                a[row_re][re_idx(j, m)] += hval.re;
                a[row_re][im_idx(j, m)] += hval.im;
                if k > 0 {
                    let row_im = im_idx(i, k);
                    a[row_im][re_idx(j, m)] += hval.im;
                    a[row_im][im_idx(j, m)] += -hval.re;
                }
            };

            for (i, j, spec) in &g_spectra {
                for k in 0..h {
                    for m in 1..h {
                        if let Some(&g) = spec.get(k + m) {
                            add_hankel(*i, *j, k, m, -g);
                        }
                    }
                }
            }
            for (i, j, spec) in &c_spectra {
                for k in 0..h {
                    let jw = Complex64::new(0.0, (k as f64) * omega0);
                    for m in 1..h {
                        if let Some(&c) = spec.get(k + m) {
                            add_hankel(*i, *j, k, m, -(jw * c));
                        }
                    }
                }
            }
        }

        let solution = self.solve_real_linear_system(&a, &rhs)?;

        let mut node_voltages = vec![vec![Complex64::new(0.0, 0.0); h]; n];
        for node in 0..n {
            for k in 0..h {
                let re = solution[re_idx(node, k)];
                let im = if k > 0 {
                    solution[im_idx(node, k)]
                } else {
                    0.0
                };
                node_voltages[node][k] = Complex64::new(re, im);
            }
        }
        let mut branch_currents = vec![vec![Complex64::new(0.0, 0.0); h]; branch_count];
        for (branch, spectrum) in branch_currents.iter_mut().enumerate() {
            let entity = n + branch;
            for (k, coefficient) in spectrum.iter_mut().enumerate() {
                let re = solution[re_idx(entity, k)];
                let im = if k > 0 {
                    solution[im_idx(entity, k)]
                } else {
                    0.0
                };
                *coefficient = Complex64::new(re, im);
            }
        }

        Ok(HbNewtonStep {
            node_voltages,
            branch_currents,
        })
    }

    /// Solve the Jacobian system: J * ΔX = -R
    ///
    /// Large systems use restarted GMRES with a per-harmonic block-Jacobi
    /// preconditioner (O(size²) per Krylov iteration instead of the dense
    /// solve's O(size³)); small systems and any Krylov stagnation take the
    /// exact dense elimination, so the Krylov path can change only speed,
    /// never convergence.
    ///
    /// Returns flattened delta_x vector that maps back to `[node][harmonic]`.
    fn solve_jacobian_system(
        &self,
        jac: &[Vec<Complex64>],
        state: &HbSolverState,
    ) -> Result<HbNewtonStep, HbError> {
        let n = self.num_nodes;
        let branch_count = self.exact_mna_branches().len();
        if branch_count > 0 && self.config.use_krylov {
            return Err(HbError::InvalidCircuit(
                "forced Krylov nonlinear HB with exact MNA branches requires the branch-aware complex operator"
                    .to_string(),
            ));
        }
        let entity_count = n.checked_add(branch_count).ok_or_else(|| {
            HbError::InvalidCircuit("nonlinear HB MNA dimension exceeds this platform".to_string())
        })?;
        let h = self.num_harmonics + 1;
        let size = entity_count.checked_mul(h).ok_or_else(|| {
            HbError::InvalidCircuit(
                "nonlinear HB complex MNA dimension exceeds this platform".to_string(),
            )
        })?;

        // Flatten RHS (negative residual)
        let mut rhs = Vec::with_capacity(size);
        for node in 0..n {
            for k in 0..h {
                rhs.push(-state.residual[node][k]);
            }
        }
        for branch in 0..branch_count {
            for k in 0..h {
                rhs.push(-state.mna_branch_residual[branch][k]);
            }
        }
        if jac.len() != size || jac.iter().any(|row| row.len() != size) || rhs.len() != size {
            return Err(HbError::InvalidCircuit(format!(
                "nonlinear HB complex Newton system has Jacobian/RHS dimensions {}/{}; expected {size}",
                jac.len(),
                rhs.len()
            )));
        }

        // The legacy complex Krylov preconditioner is node-only. Preserve the
        // dense exact-MNA seam until the dedicated branch-aware operator lands.
        let try_krylov = branch_count == 0
            && (self.config.use_krylov || size >= super::krylov::KRYLOV_AUTO_THRESHOLD);
        let flat_solution = if try_krylov && n > 0 && h > 0 {
            match self.solve_jacobian_krylov(jac, &rhs, n, h) {
                Some(solution) => solution,
                None => self.solve_complex_linear_system(jac, &rhs)?,
            }
        } else {
            self.solve_complex_linear_system(jac, &rhs)?
        };

        if flat_solution.len() != size
            || flat_solution
                .iter()
                .any(|value| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(HbError::InvalidCircuit(
                "nonlinear HB complex Newton solve produced a malformed or non-finite step"
                    .to_string(),
            ));
        }
        let mut node_voltages = vec![vec![Complex64::new(0.0, 0.0); h]; n];
        for (node, spectrum) in node_voltages.iter_mut().enumerate() {
            for (harmonic, coefficient) in spectrum.iter_mut().enumerate() {
                let value = flat_solution[node * h + harmonic];
                *coefficient = if harmonic == 0 {
                    Complex64::new(value.re, 0.0)
                } else {
                    value
                };
            }
        }
        let mut branch_currents = vec![vec![Complex64::new(0.0, 0.0); h]; branch_count];
        for (branch, spectrum) in branch_currents.iter_mut().enumerate() {
            let entity = n + branch;
            for (harmonic, coefficient) in spectrum.iter_mut().enumerate() {
                let value = flat_solution[entity * h + harmonic];
                *coefficient = if harmonic == 0 {
                    Complex64::new(value.re, 0.0)
                } else {
                    value
                };
            }
        }

        Ok(HbNewtonStep {
            node_voltages,
            branch_currents,
        })
    }

    /// Attempt the Newton-step solve via block-Jacobi-preconditioned GMRES.
    ///
    /// Returns `None` when GMRES stagnates or fails to reach the inner
    /// tolerance — the caller then takes the exact dense path.
    fn solve_jacobian_krylov(
        &self,
        jac: &[Vec<Complex64>],
        rhs: &[Complex64],
        num_nodes: usize,
        num_components: usize,
    ) -> Option<Vec<Complex64>> {
        use super::krylov::{BlockJacobiPreconditioner, gmres};

        let preconditioner = BlockJacobiPreconditioner::build(jac, num_nodes, num_components);
        let matvec = |x: &[Complex64]| -> Vec<Complex64> {
            jac.iter()
                .map(|row| row.iter().zip(x).map(|(m, v)| m * v).sum())
                .collect()
        };

        let restart = super::krylov::bounded_gmres_restart(self.config.gmres_restart, rhs.len());
        let outcome = gmres(&matvec, &preconditioner, rhs, restart, 4);

        if outcome.converged {
            if self.config.verbose {
                log::debug!(
                    "HB Krylov solve: {} iterations, relative residual {:.2e}",
                    outcome.iterations,
                    outcome.relative_residual
                );
            }
            Some(outcome.solution)
        } else {
            log::debug!(
                "HB Krylov solve stagnated after {} iterations (relative residual {:.2e}); \
                 falling back to dense elimination",
                outcome.iterations,
                outcome.relative_residual
            );
            None
        }
    }
}

#[cfg(test)]
mod exact_matrix_free_tests {
    use super::*;

    fn fixture<'a>(
        g: &'a [(usize, usize, Value)],
        c: &'a [(usize, usize, Value)],
        g_spectra: &'a [PeriodicSpectrum],
        c_spectra: &'a [PeriodicSpectrum],
    ) -> ExactHbOperator<'a> {
        ExactHbOperator {
            num_nodes: 2,
            num_components: 3,
            real_width: 5,
            omega0: 2.0 * PI * 1.0e6,
            gmin: 1e-12,
            g_matrix: g,
            c_matrix: c,
            l_matrix: &[],
            g_spectra,
            c_spectra,
            mna_branches: &[],
            mna_static_entries: &[],
            mna_inductance_entries: &[],
            periodic_networks: &[],
        }
    }

    fn assert_close(actual: Complex64, expected: Complex64) {
        let scale = actual.norm().max(expected.norm()).max(1.0);
        assert!(
            (actual - expected).norm() <= 3e-12 * scale,
            "actual={actual:?}, expected={expected:?}"
        );
    }

    #[test]
    fn exact_real_split_operator_matches_toeplitz_plus_hankel_definition() {
        let g = vec![(0, 0, 7.0), (0, 1, -0.8), (1, 0, -0.3), (1, 1, 6.0)];
        let c = vec![(0, 0, 2e-12), (1, 1, 1e-12)];
        let g_spectra = vec![
            (
                0,
                0,
                vec![
                    Complex64::new(0.5, 0.0),
                    Complex64::new(0.04, -0.02),
                    Complex64::new(-0.01, 0.006),
                    Complex64::new(0.004, -0.002),
                    Complex64::new(0.001, 0.0005),
                ],
            ),
            (
                1,
                0,
                vec![
                    Complex64::new(-0.1, 0.0),
                    Complex64::new(0.02, 0.01),
                    Complex64::new(0.005, -0.003),
                    Complex64::new(0.001, 0.002),
                    Complex64::new(-0.0005, 0.0),
                ],
            ),
        ];
        let c_spectra = vec![(
            1,
            1,
            vec![
                Complex64::new(1e-12, 0.0),
                Complex64::new(0.1e-12, 0.04e-12),
                Complex64::new(0.03e-12, -0.01e-12),
                Complex64::new(0.01e-12, 0.0),
                Complex64::new(0.002e-12, 0.001e-12),
            ],
        )];
        let operator = fixture(&g, &c, &g_spectra, &c_spectra);
        let input = (0..10)
            .map(|index| Complex64::new(index as Value * 0.11 - 0.35, 0.0))
            .collect::<Vec<_>>();
        let actual = operator.apply(&input);

        let n = operator.num_nodes;
        let h = operator.num_components;
        let mut x = vec![Complex64::new(0.0, 0.0); n * h];
        for node in 0..n {
            x[node * h] = Complex64::new(input[node * 5].re, 0.0);
            for k in 1..h {
                x[node * h + k] =
                    Complex64::new(input[node * 5 + 2 * k - 1].re, input[node * 5 + 2 * k].re);
            }
        }
        let mut toeplitz = vec![vec![Complex64::new(0.0, 0.0); n * h]; n * h];
        let mut hankel = vec![vec![Complex64::new(0.0, 0.0); n * h]; n * h];
        for k in 0..h {
            let jw = Complex64::new(0.0, k as Value * operator.omega0);
            for &(i, j, value) in &g {
                toeplitz[i * h + k][j * h + k] -= value;
            }
            for &(i, j, value) in &c {
                toeplitz[i * h + k][j * h + k] -= jw * value;
            }
            for node in 0..n {
                toeplitz[node * h + k][node * h + k] -= operator.gmin;
            }
        }
        for &(i, j, ref spectrum) in &g_spectra {
            for k in 0..h {
                for l in 0..h {
                    let d = k as isize - l as isize;
                    let coefficient = spectrum[d.unsigned_abs()];
                    toeplitz[i * h + k][j * h + l] -= if d >= 0 {
                        coefficient
                    } else {
                        coefficient.conj()
                    };
                }
                for m in 1..h {
                    hankel[i * h + k][j * h + m] -= spectrum[k + m];
                }
            }
        }
        for &(i, j, ref spectrum) in &c_spectra {
            for k in 0..h {
                let jw = Complex64::new(0.0, k as Value * operator.omega0);
                for l in 0..h {
                    let d = k as isize - l as isize;
                    let coefficient = spectrum[d.unsigned_abs()];
                    toeplitz[i * h + k][j * h + l] -= jw
                        * if d >= 0 {
                            coefficient
                        } else {
                            coefficient.conj()
                        };
                }
                for m in 1..h {
                    hankel[i * h + k][j * h + m] -= jw * spectrum[k + m];
                }
            }
        }
        let mut expected_complex = vec![Complex64::new(0.0, 0.0); n * h];
        for row in 0..n * h {
            for col in 0..n * h {
                expected_complex[row] +=
                    toeplitz[row][col] * x[col] + hankel[row][col] * x[col].conj();
            }
        }
        for node in 0..n {
            assert_close(
                actual[node * 5],
                Complex64::new(expected_complex[node * h].re, 0.0),
            );
            for k in 1..h {
                assert_close(
                    actual[node * 5 + 2 * k - 1],
                    Complex64::new(expected_complex[node * h + k].re, 0.0),
                );
                assert_close(
                    actual[node * 5 + 2 * k],
                    Complex64::new(expected_complex[node * h + k].im, 0.0),
                );
            }
        }
    }

    #[test]
    fn exact_matrix_free_gmres_matches_direct_real_lu() {
        let g = vec![(0, 0, 9.0), (0, 1, -0.5), (1, 0, -0.25), (1, 1, 8.0)];
        let g_spectra = vec![(
            0,
            0,
            vec![
                Complex64::new(0.4, 0.0),
                Complex64::new(0.008, -0.003),
                Complex64::new(0.002, 0.001),
                Complex64::new(0.0005, 0.0),
                Complex64::new(0.0001, 0.0),
            ],
        )];
        let operator = fixture(&g, &[], &g_spectra, &[]);
        let preconditioner = ExactHbPreconditioner::build(&operator);
        let rhs = (0..10)
            .map(|index| Complex64::new(0.2 - index as Value * 0.017, 0.0))
            .collect::<Vec<_>>();
        let outcome = super::super::krylov::gmres(
            &|input| operator.apply(input),
            &preconditioner,
            &rhs,
            10,
            6,
        );
        assert!(outcome.converged, "relative={}", outcome.relative_residual);

        let size = rhs.len();
        let mut dense = vec![Complex64::new(0.0, 0.0); size * size];
        let mut basis = vec![Complex64::new(0.0, 0.0); size];
        for column in 0..size {
            basis[column] = Complex64::new(1.0, 0.0);
            let image = operator.apply(&basis);
            for row in 0..size {
                dense[row * size + column] = image[row];
            }
            basis[column] = Complex64::new(0.0, 0.0);
        }
        let factors = super::super::krylov::LuFactors::factor(dense, size);
        let mut direct = rhs.clone();
        factors.solve_in_place(&mut direct);
        for (actual, expected) in outcome.solution.into_iter().zip(direct) {
            assert_close(actual, expected);
        }
    }

    #[test]
    fn branch_aware_operator_matches_explicit_mna_orientation_and_dense_stream() {
        let g = vec![(0, 0, 3.0), (1, 1, 4.0)];
        let branches = vec![
            ExactMnaBranch::VoltageSource {
                branch_ordinal: 1,
                node_pos: 1,
                node_neg: 0,
                source_index: 0,
                source: None,
            },
            ExactMnaBranch::Inductor {
                branch_ordinal: 2,
                node_pos: 2,
                node_neg: 1,
                inductance: 2.5e-6,
            },
        ];
        let base = fixture(&g, &[], &[], &[]);
        let operator = ExactHbOperator {
            mna_branches: &branches,
            ..fixture(&g, &[], &[], &[])
        };
        operator.validate().expect("branch-aware operator is valid");

        let size = operator.entity_count() * operator.real_width;
        let input = (0..size)
            .map(|index| Complex64::new(0.03 * index as Value - 0.2, 0.0))
            .collect::<Vec<_>>();
        let actual = operator.apply(&input);

        // Start from the independently checked nodal operator, then apply
        // the canonical branch equations explicitly in complex form.
        let mut expected = vec![Complex64::new(0.0, 0.0); size];
        expected[..2 * operator.real_width]
            .copy_from_slice(&base.apply(&input[..2 * operator.real_width]));
        let coefficient = |entity: usize, harmonic: usize| {
            let re = input[operator.re_idx(entity, harmonic)].re;
            let im = if harmonic == 0 {
                0.0
            } else {
                input[operator.im_idx(entity, harmonic)].re
            };
            Complex64::new(re, im)
        };
        let mut add = |entity: usize, harmonic: usize, value: Complex64| {
            expected[operator.re_idx(entity, harmonic)] += value.re;
            if harmonic > 0 {
                expected[operator.im_idx(entity, harmonic)] += value.im;
            }
        };
        for harmonic in 0..operator.num_components {
            let v0 = coefficient(0, harmonic);
            let v1 = coefficient(1, harmonic);
            let source_current = coefficient(2, harmonic);
            let inductor_current = coefficient(3, harmonic);
            add(0, harmonic, -source_current + inductor_current);
            add(1, harmonic, -inductor_current);
            add(2, harmonic, -v0);
            add(
                3,
                harmonic,
                v0 - v1
                    + Complex64::new(0.0, harmonic as Value * operator.omega0 * 2.5e-6)
                        * inductor_current,
            );
        }
        for (&value, &reference) in actual.iter().zip(&expected) {
            assert_close(value, reference);
        }

        let mut streamed_dense = vec![0.0; size * size];
        operator.visit_entries(|row, column, value| {
            streamed_dense[row * size + column] += value;
        });
        for row in 0..size {
            let value = (0..size)
                .map(|column| streamed_dense[row * size + column] * input[column])
                .sum();
            assert_close(actual[row], value);
        }
        assert_eq!(actual[operator.im_idx(3, 1)].im, 0.0);
    }

    #[test]
    fn matrix_free_operator_applies_mutual_inductance_to_cross_branch_currents() {
        let branches = vec![
            ExactMnaBranch::Inductor {
                branch_ordinal: 1,
                node_pos: 1,
                node_neg: 0,
                inductance: 100.0e-6,
            },
            ExactMnaBranch::Inductor {
                branch_ordinal: 2,
                node_pos: 2,
                node_neg: 0,
                inductance: 25.0e-6,
            },
        ];
        let mutual = 40.0e-6;
        let mutual_entries = [(2, 3, mutual), (3, 2, mutual)];
        let uncoupled = ExactHbOperator {
            mna_branches: &branches,
            ..fixture(&[], &[], &[], &[])
        };
        let coupled = ExactHbOperator {
            mna_branches: &branches,
            mna_inductance_entries: &mutual_entries,
            ..fixture(&[], &[], &[], &[])
        };
        coupled.validate().expect("mutual operator is valid");
        let size = coupled.entity_count() * coupled.real_width;
        let input = (0..size)
            .map(|index| Complex64::new(0.02 * index as Value - 0.1, 0.0))
            .collect::<Vec<_>>();
        let actual = coupled.apply(&input);
        let baseline = uncoupled.apply(&input);
        let coefficient = |entity: usize, harmonic: usize| {
            Complex64::new(
                input[coupled.re_idx(entity, harmonic)].re,
                if harmonic == 0 {
                    0.0
                } else {
                    input[coupled.im_idx(entity, harmonic)].re
                },
            )
        };
        for harmonic in 0..coupled.num_components {
            let jw_m = Complex64::new(0.0, harmonic as Value * coupled.omega0 * mutual);
            for (row_entity, column_entity) in [(2, 3), (3, 2)] {
                let expected = jw_m * coefficient(column_entity, harmonic);
                assert_close(
                    actual[coupled.re_idx(row_entity, harmonic)]
                        - baseline[coupled.re_idx(row_entity, harmonic)],
                    Complex64::new(expected.re, 0.0),
                );
                if harmonic > 0 {
                    assert_close(
                        actual[coupled.im_idx(row_entity, harmonic)]
                            - baseline[coupled.im_idx(row_entity, harmonic)],
                        Complex64::new(expected.im, 0.0),
                    );
                }
            }
        }
    }

    #[test]
    fn matrix_free_operator_applies_exact_delay_line_branch_rows() {
        let branches = vec![
            ExactMnaBranch::NetworkPort {
                branch_ordinal: 1,
                node_pos: 1,
                node_neg: 0,
            },
            ExactMnaBranch::NetworkPort {
                branch_ordinal: 2,
                node_pos: 2,
                node_neg: 0,
            },
        ];
        let delay = 137.0e-9;
        let impedance = 50.0;
        let network = ExactPeriodicNetwork::ScalarWave {
            name: "T1".to_string(),
            node1_pos: 1,
            node1_neg: 0,
            node2_pos: 2,
            node2_neg: 0,
            branch1: 2,
            branch2: 3,
            impedance,
            delay,
            attenuation: 1.0,
        };
        let networks = [network];
        let operator = ExactHbOperator {
            gmin: 0.0,
            mna_branches: &branches,
            periodic_networks: &networks,
            ..fixture(&[], &[], &[], &[])
        };
        operator.validate().expect("delay-line operator is valid");
        let size = operator.entity_count() * operator.real_width;
        let input = (0..size)
            .map(|index| Complex64::new(0.07 * index as Value - 0.4, 0.0))
            .collect::<Vec<_>>();
        let actual = operator.apply(&input);
        let coefficient = |entity: usize, harmonic: usize| {
            Complex64::new(
                input[operator.re_idx(entity, harmonic)].re,
                if harmonic == 0 {
                    0.0
                } else {
                    input[operator.im_idx(entity, harmonic)].re
                },
            )
        };
        let output = |entity: usize, harmonic: usize| {
            Complex64::new(
                actual[operator.re_idx(entity, harmonic)].re,
                if harmonic == 0 {
                    0.0
                } else {
                    actual[operator.im_idx(entity, harmonic)].re
                },
            )
        };
        for harmonic in 0..operator.num_components {
            let q = Complex64::from_polar(1.0, -(harmonic as Value) * operator.omega0 * delay);
            let v1 = coefficient(0, harmonic);
            let v2 = coefficient(1, harmonic);
            let i1 = coefficient(2, harmonic);
            let i2 = coefficient(3, harmonic);
            assert_close(output(0, harmonic), -i1);
            assert_close(output(1, harmonic), -i2);
            assert_close(
                output(2, harmonic),
                -v1 + q * v2 + impedance * i1 + q * impedance * i2,
            );
            assert_close(
                output(3, harmonic),
                -v2 + q * v1 + impedance * i2 + q * impedance * i1,
            );
        }
    }

    #[test]
    fn nonlinear_residual_and_dense_jacobian_include_mutual_inductance() {
        let fundamental = 1.0e6;
        let mut solver = HbSolver::new(HbConfig::new(fundamental).with_harmonics(2), 2);
        solver
            .try_add_periodic_inductor_branch(1, 0, 100.0e-6, 1, "L1")
            .expect("first winding registers");
        solver
            .try_add_periodic_inductor_branch(2, 0, 25.0e-6, 2, "L2")
            .expect("second winding registers");
        let mutual = 40.0e-6;
        solver
            .try_add_exact_mna_inductance_entry(2, 3, mutual, "K1")
            .expect("forward mutual entry registers");
        solver
            .try_add_exact_mna_inductance_entry(3, 2, mutual, "K1")
            .expect("reverse mutual entry registers");
        let mut state = HbSolverState::new(2, 2);
        state
            .try_prepare_mna_branches(2, 2)
            .expect("branch spectra allocate");
        state.mna_branch_currents[0][2] = Complex64::new(0.3, -0.2);
        state.mna_branch_currents[1][2] = Complex64::new(-0.1, 0.4);
        solver
            .add_exact_mna_residual(&mut state, 1.0)
            .expect("exact residual evaluates");
        let jw = Complex64::new(0.0, 2.0 * 2.0 * PI * fundamental);
        assert_close(
            state.mna_branch_residual[0][2],
            jw * (100.0e-6 * state.mna_branch_currents[0][2]
                + mutual * state.mna_branch_currents[1][2]),
        );
        assert_close(
            state.mna_branch_residual[1][2],
            jw * (mutual * state.mna_branch_currents[0][2]
                + 25.0e-6 * state.mna_branch_currents[1][2]),
        );

        let jacobian = solver
            .build_full_jacobian(&state)
            .expect("dense exact Jacobian evaluates");
        let h = 3;
        assert_close(jacobian[(2 * h) + 2][(3 * h) + 2], jw * mutual);
        assert_close(jacobian[(3 * h) + 2][(2 * h) + 2], jw * mutual);
    }

    #[test]
    fn nonlinear_residual_and_dense_jacobian_include_exact_delay_line() {
        let fundamental = 1.0e6;
        let delay = 137.0e-9;
        let impedance = 50.0;
        let mut solver = HbSolver::new(HbConfig::new(fundamental).with_harmonics(2), 2);
        solver
            .try_add_periodic_network_port_branch(1, 0, 1, "T1#port1")
            .expect("first line port registers");
        solver
            .try_add_periodic_network_port_branch(2, 0, 2, "T1#port2")
            .expect("second line port registers");
        solver
            .try_add_exact_periodic_network(ExactPeriodicNetwork::ScalarWave {
                name: "T1".to_string(),
                node1_pos: 1,
                node1_neg: 0,
                node2_pos: 2,
                node2_neg: 0,
                branch1: 2,
                branch2: 3,
                impedance,
                delay,
                attenuation: 1.0,
            })
            .expect("delay-line equations register");
        let mut state = HbSolverState::new(2, 2);
        state
            .try_prepare_mna_branches(2, 2)
            .expect("port-current spectra allocate");
        state.x[0][2] = Complex64::new(0.3, -0.2);
        state.x[1][2] = Complex64::new(-0.1, 0.4);
        state.mna_branch_currents[0][2] = Complex64::new(0.02, 0.01);
        state.mna_branch_currents[1][2] = Complex64::new(-0.03, 0.04);
        solver
            .add_exact_mna_residual(&mut state, 1.0)
            .expect("exact delay-line residual evaluates");

        let q = Complex64::from_polar(1.0, -2.0 * 2.0 * PI * fundamental * delay);
        let expected_row1 = -state.x[0][2]
            + q * state.x[1][2]
            + impedance * state.mna_branch_currents[0][2]
            + q * impedance * state.mna_branch_currents[1][2];
        assert_close(state.mna_branch_residual[0][2], expected_row1);
        assert_close(state.residual[0][2], -state.mna_branch_currents[0][2]);

        let jacobian = solver
            .build_full_jacobian(&state)
            .expect("dense exact delay-line Jacobian evaluates");
        let h = 3;
        let row = 2 * h + 2;
        assert_close(jacobian[row][2], Complex64::new(-1.0, 0.0));
        assert_close(jacobian[row][h + 2], q);
        assert_close(jacobian[row][2 * h + 2], Complex64::new(impedance, 0.0));
        assert_close(jacobian[row][3 * h + 2], q * impedance);
    }

    #[test]
    fn branch_aware_matrix_free_gmres_matches_direct_real_lu() {
        let g = vec![(0, 0, 5.0), (1, 1, 7.0)];
        let branches = vec![ExactMnaBranch::VoltageSource {
            branch_ordinal: 1,
            node_pos: 1,
            node_neg: 0,
            source_index: 0,
            source: None,
        }];
        let operator = ExactHbOperator {
            mna_branches: &branches,
            ..fixture(&g, &[], &[], &[])
        };
        let preconditioner = ExactHbPreconditioner::build(&operator);
        let size = operator.entity_count() * operator.real_width;
        let rhs = (0..size)
            .map(|index| Complex64::new(0.1 - index as Value * 0.006, 0.0))
            .collect::<Vec<_>>();
        let outcome = super::super::krylov::gmres(
            &|input| operator.apply(input),
            &preconditioner,
            &rhs,
            size,
            4,
        );
        assert!(outcome.converged, "relative={}", outcome.relative_residual);
        rspice_matrix::certify_complex_transpose_solution_by_entry_visitor(
            size,
            size,
            &outcome.solution,
            &rhs,
            |visitor| {
                operator.visit_entries(|row, column, value| {
                    visitor(column, row, Complex64::new(value, 0.0));
                });
            },
        )
        .expect("branch-aware Krylov candidate is componentwise certified");

        let mut dense = vec![Complex64::new(0.0, 0.0); size * size];
        operator.visit_entries(|row, column, value| {
            dense[row * size + column] += value;
        });
        let factors = super::super::krylov::LuFactors::factor(dense, size);
        let mut direct = rhs.clone();
        factors.solve_in_place(&mut direct);
        for (value, reference) in outcome.solution.iter().zip(&direct) {
            assert_close(*value, *reference);
        }
    }

    #[test]
    fn grounded_voltage_singleton_projection_is_exact_and_topology_limited() {
        let branches = vec![
            ExactMnaBranch::VoltageSource {
                branch_ordinal: 1,
                node_pos: 1,
                node_neg: 0,
                source_index: 0,
                source: None,
            },
            ExactMnaBranch::VoltageSource {
                branch_ordinal: 2,
                node_pos: 0,
                node_neg: 2,
                source_index: 1,
                source: None,
            },
            ExactMnaBranch::VoltageSource {
                branch_ordinal: 3,
                node_pos: 1,
                node_neg: 2,
                source_index: 2,
                source: None,
            },
            ExactMnaBranch::Inductor {
                branch_ordinal: 4,
                node_pos: 2,
                node_neg: 0,
                inductance: 1.0e-6,
            },
        ];
        let operator = ExactHbOperator {
            mna_branches: &branches,
            ..fixture(&[], &[], &[], &[])
        };
        operator.validate().expect("projection fixture is valid");
        let size = operator.entity_count() * operator.real_width;
        let mut solution = (0..size)
            .map(|index| Complex64::new(index as Value + 1.0, -(index as Value) - 0.5))
            .collect::<Vec<_>>();
        let original = solution.clone();
        let mut rhs = vec![Complex64::new(7.0, 0.0); size];

        let positive_grounded_branch = operator.num_nodes;
        rhs[operator.re_idx(positive_grounded_branch, 0)] = Complex64::new(-0.0, -0.0);
        rhs[operator.im_idx(positive_grounded_branch, 1)] = Complex64::new(0.0, 0.0);
        let negative_grounded_branch = operator.num_nodes + 1;
        rhs[operator.re_idx(negative_grounded_branch, 1)] = Complex64::new(0.0, 0.0);
        for branch_entity in [operator.num_nodes + 2, operator.num_nodes + 3] {
            for harmonic in 0..operator.num_components {
                rhs[operator.re_idx(branch_entity, harmonic)] = Complex64::new(0.0, 0.0);
                if harmonic > 0 {
                    rhs[operator.im_idx(branch_entity, harmonic)] = Complex64::new(0.0, 0.0);
                }
            }
        }

        operator.project_homogeneous_grounded_voltage_singletons(&mut solution, &rhs);

        for index in [
            operator.re_idx(0, 0),
            operator.im_idx(0, 1),
            operator.re_idx(1, 1),
        ] {
            assert_eq!(solution[index].re.to_bits(), 0);
            assert_eq!(solution[index].im.to_bits(), 0);
        }
        // A nonzero grounded-source row is not projected, even though the
        // same coordinate also appears in a homogeneous floating constraint.
        assert_eq!(
            solution[operator.re_idx(0, 1)],
            original[operator.re_idx(0, 1)]
        );
        // A grounded inductor row is not an ideal-voltage-source singleton.
        assert_eq!(
            solution[operator.re_idx(1, 0)],
            original[operator.re_idx(1, 0)]
        );
        // Branch-current coordinates are never canonicalized.
        assert_eq!(
            solution[operator.re_idx(positive_grounded_branch, 0)],
            original[operator.re_idx(positive_grounded_branch, 0)]
        );
    }

    #[test]
    fn candidate_report_certifies_and_renorms_after_singleton_projection() {
        let g = vec![(0, 0, 2.0), (1, 1, 4.0)];
        let branches = vec![ExactMnaBranch::VoltageSource {
            branch_ordinal: 1,
            node_pos: 1,
            node_neg: 0,
            source_index: 0,
            source: None,
        }];
        let operator = ExactHbOperator {
            gmin: 0.0,
            mna_branches: &branches,
            ..fixture(&g, &[], &[], &[])
        };
        let size = operator.entity_count() * operator.real_width;
        let mut exact = vec![Complex64::new(0.0, 0.0); size];
        for harmonic in 0..operator.num_components {
            exact[operator.re_idx(1, harmonic)] = Complex64::new(2.0 + harmonic as Value, 0.0);
            if harmonic > 0 {
                exact[operator.im_idx(1, harmonic)] = Complex64::new(-2.0 - harmonic as Value, 0.0);
            }
            let branch_entity = operator.num_nodes;
            exact[operator.re_idx(branch_entity, harmonic)] =
                Complex64::new(1.0 + harmonic as Value, 0.0);
            if harmonic > 0 {
                exact[operator.im_idx(branch_entity, harmonic)] =
                    Complex64::new(-1.0 - harmonic as Value, 0.0);
            }
        }
        let rhs = operator.apply(&exact);
        let mut candidate = exact;
        candidate[operator.re_idx(0, 1)] = Complex64::new(1.0e-30, -1.0e-30);

        let (report, relative_residual) =
            exact_hb_candidate_report(&operator, &mut candidate, &rhs)
                .expect("canonical candidate has a finite full certificate");

        assert!(report.is_accepted());
        assert_eq!(relative_residual.to_bits(), 0);
        assert_eq!(candidate[operator.re_idx(0, 1)].re.to_bits(), 0);
        assert_eq!(candidate[operator.re_idx(0, 1)].im.to_bits(), 0);
    }

    #[test]
    fn exact_matrix_free_step_returns_positive_zero_for_grounded_homogeneous_source() {
        let mut config = HbConfig::new(1.0e6).with_harmonics(1);
        config.use_krylov = true;
        let mut solver = HbSolver::new(config, 2);
        solver.add_conductance(0, 0, 3.0);
        solver.add_conductance(1, 1, 5.0);
        let source_index = solver
            .try_add_named_voltage_source_branch_harmonics(1, 0, 0.0, &[], "VZERO")
            .expect("zero source registers");
        solver
            .try_add_periodic_voltage_source_branch(1, 0, source_index, 1, "VZERO")
            .expect("exact branch registers");
        let mut state = HbSolverState::new(2, 1);
        state
            .try_prepare_mna_branches(1, 1)
            .expect("exact branch state is allocated");
        state.residual[0][0] = Complex64::new(1.0, 0.0);
        state.residual[0][1] = Complex64::new(2.0, -3.0);
        state.residual[1][0] = Complex64::new(-4.0, 0.0);
        state.residual[1][1] = Complex64::new(5.0, 6.0);

        let step = solver
            .solve_jacobian_system_exact(&state, 0.0, &NoAbort)
            .expect("homogeneous grounded-source step is certified");

        for coordinate in [
            step.node_voltages[0][0].re,
            step.node_voltages[0][0].im,
            step.node_voltages[0][1].re,
            step.node_voltages[0][1].im,
        ] {
            assert_eq!(coordinate.to_bits(), 0);
        }
        assert!(
            step.branch_currents[0]
                .iter()
                .any(|value| value.norm() > 0.0)
        );
    }

    #[test]
    fn large_or_structurally_invalid_krylov_steps_never_select_dense_recovery() {
        use rspice_matrix::SolverError;

        assert!(exact_krylov_dense_recovery_allowed(
            super::super::krylov::KRYLOV_AUTO_THRESHOLD - 1,
            &SolverError::ConvergenceFailed(7),
        ));
        assert!(!exact_krylov_dense_recovery_allowed(
            super::super::krylov::KRYLOV_AUTO_THRESHOLD,
            &SolverError::InaccurateSolution(1.0e-8),
        ));
        assert!(!exact_krylov_dense_recovery_allowed(
            1,
            &SolverError::Overflow,
        ));
        assert!(!exact_krylov_dense_recovery_allowed(
            1,
            &SolverError::InvalidCircuit("malformed operator".to_string()),
        ));
        assert!(!exact_krylov_dense_recovery_allowed(
            1,
            &SolverError::OutOfMemory,
        ));
    }

    #[test]
    fn line_search_rejects_unrepresentable_dc_imaginary_state() {
        let mut solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 1);
        solver.add_conductance(0, 0, 1.0);
        solver.add_dc_source(0, 1.0);
        let mut state = HbSolverState::new(1, 1);
        state.x[0][0] = Complex64::new(1.0, 1.0);
        solver
            .compute_full_residual_with_gmin(&mut state, 0.0, 1.0)
            .expect("finite linear residual");
        assert!(state.residual_norm > 0.0);

        let delta = HbNewtonStep {
            node_voltages: vec![vec![Complex64::new(0.0, 0.0); 2]],
            branch_currents: Vec::new(),
        };
        let reltol = solver.config.tolerance;
        let abstol = solver.config.abstol;
        let error = solver
            .apply_line_search_with_gmin(&mut state, &delta, 0.0, 1.0, reltol, abstol, &NoAbort)
            .expect_err("non-real DC evidence must fail closed before line search");
        assert!(error.to_string().contains("imaginary DC"), "{error}");
    }
}
