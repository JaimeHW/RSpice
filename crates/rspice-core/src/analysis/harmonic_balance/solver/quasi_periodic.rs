//! Reuse registered physical devices and exact MNA for a driven QPSS solve.
#[cfg(test)]
mod integral_tests;
mod noise_sources;
pub(super) mod prescribed;

use super::*;
use crate::ResourceLimits;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicAcConfig, QuasiPeriodicAcSolution, QuasiPeriodicAdjointSolution,
    QuasiPeriodicError as Error, QuasiPeriodicGrid, QuasiPeriodicLinearConfig,
    QuasiPeriodicNoiseConfig, QuasiPeriodicNoisePoint, QuasiPeriodicNoiseSource,
    QuasiPeriodicSolution, QuasiPeriodicSolveConfig,
    solve::{self, Circuit, LinearEntry, Sample},
};
use std::sync::Arc;

fn device_error(error: HbError) -> Error {
    match error {
        HbError::Aborted => Error::Aborted,
        other => Error::InvalidCircuit(other.to_string()),
    }
}

impl HbSolver {
    /// Solve the registered circuit on independent tone phases. `sources`
    /// supplies the entire MNA right hand side in full signed Fourier-series
    /// coefficients (a cosine of peak A has coefficients A/2). Existing HB
    /// harmonic source tables and the HB fundamental are not used.
    ///
    /// Branch devices require the canonical exact-MNA registry. Auto selects
    /// a bounded-memory Krylov backend above 512 real spectral unknowns;
    /// caller resource limits can further restrict either backend. The result
    /// certifies retained equations, not truncation or aliasing error.
    pub fn solve_quasi_periodic_with_abort(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        config: &QuasiPeriodicSolveConfig,
        sources: &[Vec<Complex64>],
        seed: Option<&[Vec<Complex64>]>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicSolution, Error> {
        if abort.is_aborted() {
            return Err(Error::Aborted);
        }
        self.validate_quasi_periodic_circuit()?;
        let (prepared, limits, preparation_iterations) = self.prepare_quasi_periodic_carrier(
            grid.clone(),
            config,
            sources,
            seed,
            limits,
            abort,
        )?;
        let result = if preparation_iterations == 0 {
            solve::solve_with_abort(
                self,
                grid,
                config,
                sources,
                prepared.as_deref().or(seed),
                &limits,
                abort,
            )
        } else {
            solve::solve_with_iteration_budget(
                self,
                grid,
                config,
                sources,
                prepared.as_deref().or(seed),
                &limits,
                config.max_iterations.saturating_sub(preparation_iterations),
                abort,
            )
        };
        result
            .map(|solution| solution.with_preparation_iterations(preparation_iterations))
            .map_err(|error| match error {
                Error::ConvergenceFailed { iterations, merit } => Error::ConvergenceFailed {
                    iterations: iterations + preparation_iterations,
                    merit,
                },
                other => other,
            })
    }

    /// Linearize a real driven QP orbit once, then solve complex translated
    /// phasors at each requested offset. The source rows are arbitrary complex
    /// amplitudes on the full signed lattice, with no conjugate reflection.
    pub fn solve_quasi_periodic_ac_with_abort(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        config: &QuasiPeriodicAcConfig,
        orbit: &[Vec<Complex64>],
        offsets_hz: &[Value],
        sources: &[Vec<Complex64>],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<QuasiPeriodicAcSolution>, Error> {
        if abort.is_aborted() {
            return Err(Error::Aborted);
        }
        if offsets_hz.is_empty() || offsets_hz.iter().any(|v| !v.is_finite()) {
            return Err(Error::InvalidConfig(
                "QPAC needs at least one finite probe offset".into(),
            ));
        }
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::AnalysisPoints,
            offsets_hz.len(),
            limits.max_analysis_points,
        )?;
        let values = self
            .unknowns()
            .saturating_mul(grid.len())
            .saturating_mul(offsets_hz.len())
            .saturating_mul(2)
            .saturating_add(offsets_hz.len().saturating_mul(2));
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ResultValues,
            values,
            limits.max_result_values,
        )?;
        // Charge the retained sweep alongside the numerical workspace.
        let mut working_limits = limits.clone();
        working_limits.max_result_values = limits.max_result_values.saturating_sub(values);
        self.validate_quasi_periodic_circuit()?;
        let working_limits = self.prepare_quasi_periodic_integrals(
            grid.clone(),
            &working_limits,
            true,
            None,
            None,
            abort,
        )?;
        let mut work = crate::analysis::quasi_periodic::small_signal::Linearization::prepare(
            self,
            grid,
            orbit,
            config,
            &working_limits,
            abort,
        )?;
        offsets_hz
            .iter()
            .map(|&offset| work.solve(self, offset, sources, abort))
            .collect()
    }

    /// Adjoint sweep on the common offset axis (the zero tone tuple).
    pub fn solve_quasi_periodic_adjoint_with_abort(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        linear: &QuasiPeriodicLinearConfig,
        orbit: &[Vec<Complex64>],
        offsets_hz: &[Value],
        observation: &[Vec<Complex64>],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<QuasiPeriodicAdjointSolution>, Error> {
        let anchor = vec![0; grid.dimensions().len()];
        self.solve_quasi_periodic_adjoint_at_frequency_with_abort(
            grid,
            linear,
            orbit,
            offsets_hz,
            &anchor,
            observation,
            limits,
            abort,
        )
    }

    /// Solve Aᴴ λ = c once per offset for the observation y = cᴴ x.
    /// Every small-signal source/tuple transfer is then λᴴ b. Complete exact
    /// MNA and all signed tuples participate, including frequency-dependent
    /// linear networks and native F/Q derivatives.
    pub fn solve_quasi_periodic_adjoint_at_frequency_with_abort(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        linear: &QuasiPeriodicLinearConfig,
        orbit: &[Vec<Complex64>],
        frequencies_hz: &[Value],
        frequency_anchor: &[i32],
        observation: &[Vec<Complex64>],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<QuasiPeriodicAdjointSolution>, Error> {
        if abort.is_aborted() {
            return Err(Error::Aborted);
        }
        if frequencies_hz.is_empty()
            || frequencies_hz.iter().any(|v| !v.is_finite())
            || observation.len() != self.unknowns()
            || observation.iter().any(|row| {
                row.len() != grid.len()
                    || row.iter().any(|v| !v.re.is_finite() || !v.im.is_finite())
            })
            || !observation.iter().flatten().any(|v| *v != Complex64::ZERO)
        {
            return Err(Error::InvalidConfig(
                "QPXF requires finite probe offsets and a nonzero finite complete MNA observation"
                    .into(),
            ));
        }
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::AnalysisPoints,
            frequencies_hz.len(),
            limits.max_analysis_points,
        )?;
        let values = self
            .unknowns()
            .saturating_mul(grid.len())
            .saturating_mul(frequencies_hz.len())
            .saturating_mul(2)
            .saturating_add(frequencies_hz.len().saturating_mul(2));
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ResultValues,
            values,
            limits.max_result_values,
        )?;
        let mut working_limits = limits.clone();
        working_limits.max_result_values = limits.max_result_values.saturating_sub(values);
        self.validate_quasi_periodic_circuit()?;
        let working_limits = self.prepare_quasi_periodic_integrals(
            grid.clone(),
            &working_limits,
            true,
            None,
            None,
            abort,
        )?;
        let mut work =
            crate::analysis::quasi_periodic::small_signal::Linearization::prepare_adjoint(
                self,
                grid,
                orbit,
                linear,
                &working_limits,
                abort,
            )?;
        frequencies_hz
            .iter()
            .map(|&offset| {
                work.solve_adjoint_at_frequency(self, offset, frequency_anchor, observation, abort)
            })
            .collect()
    }

    /// Stream QPNOISE covariance and certified adjoints from a complete physical
    /// orbit. Reuses one F/Q linearization for all frequencies and observations;
    /// every callback point retains its independent source contributions.
    /// Sources must be sampled on this exact grid and supplied by the engine's
    /// physical noise catalog. This numerical entry point does not authenticate
    /// netlists or construct device noise laws.
    pub fn visit_quasi_periodic_noise_with_abort(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        config: &QuasiPeriodicNoiseConfig,
        orbit: &[Vec<Complex64>],
        observations: &[Vec<Vec<Complex64>>],
        sources: &[QuasiPeriodicNoiseSource],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
        consume: impl FnMut(usize, QuasiPeriodicNoisePoint) -> Result<(), Error>,
    ) -> Result<(), Error> {
        if abort.is_aborted() {
            return Err(Error::Aborted);
        }
        self.validate_quasi_periodic_circuit()?;
        let limits =
            self.prepare_quasi_periodic_integrals(grid.clone(), limits, true, None, None, abort)?;
        crate::analysis::quasi_periodic::noise::visit_with_abort(
            self,
            grid,
            config,
            orbit,
            observations,
            sources,
            &limits,
            abort,
            consume,
        )
    }

    fn quasi_periodic_linear_entries(
        &self,
        frequency_hz: Value,
        small_signal: bool,
    ) -> Result<Vec<LinearEntry>, Error> {
        let mut entries = Vec::new();
        self.visit_periodic_linear_entries(frequency_hz, small_signal, |row, col, value| {
            entries.push((row, col, value));
            Ok(())
        })?;
        Ok(entries)
    }

    pub(in crate::analysis::harmonic_balance::solver) fn visit_periodic_linear_entries(
        &self,
        frequency_hz: Value,
        small_signal: bool,
        mut visit: impl FnMut(usize, usize, Complex64) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let omega = std::f64::consts::TAU * frequency_hz;
        let conductance = if small_signal {
            &self.periodic_g_matrix
        } else {
            &self.g_matrix
        };
        for (matrix, reactive) in [(conductance, false), (&self.c_matrix, true)] {
            for &(row, col, value) in matrix {
                if row >= self.num_nodes || col >= self.num_nodes {
                    return Err(Error::InvalidCircuit(
                        "QPSS nodal stamp is outside the node table".into(),
                    ));
                }
                let coefficient = if reactive {
                    Complex64::new(0.0, omega * value)
                } else {
                    Complex64::new(value, 0.0)
                };
                visit(row, col, coefficient)?;
            }
        }
        for (index, branch) in self.periodic_mna_branches.iter().enumerate() {
            let row = self.num_nodes + index;
            let (_, pos, neg) = branch.ordinal_and_terminals();
            for (node, sign) in [(pos, 1.0), (neg, -1.0)] {
                if node > 0 {
                    visit(node - 1, row, Complex64::new(sign, 0.0))?;
                    if !matches!(
                        branch,
                        ExactMnaBranch::ConstitutivePort { .. }
                            | ExactMnaBranch::IntegralState { .. }
                    ) {
                        visit(row, node - 1, Complex64::new(sign, 0.0))?;
                    }
                }
            }
            match branch {
                ExactMnaBranch::Inductor { inductance, .. } => {
                    visit(row, row, Complex64::new(0.0, -omega * inductance))?;
                }
                ExactMnaBranch::Resistor {
                    resistance,
                    small_signal_resistance,
                    ..
                } => {
                    let resistance = if small_signal {
                        small_signal_resistance
                    } else {
                        resistance
                    };
                    visit(row, row, Complex64::new(-resistance, 0.0))?;
                }
                _ => {}
            }
        }
        for &(row, col, value) in &self.exact_mna_static_entries {
            visit(row, col, Complex64::new(value, 0.0))?;
        }
        for &(row, col, value) in &self.exact_mna_inductance_entries {
            visit(row, col, Complex64::new(0.0, -omega * value))?;
        }
        for network in &self.exact_periodic_networks {
            let mut result = Ok(());
            network
                .try_visit_direct_entries(omega, self.unknowns(), |row, col, value| {
                    if result.is_ok() {
                        result = visit(row, col, value);
                    }
                })
                .map_err(device_error)?;
            result?;
        }
        Ok(())
    }

    fn validate_quasi_periodic_circuit(&self) -> Result<(), Error> {
        self.validate_nonlinear_device_parameters()
            .map_err(device_error)?;
        if self
            .behavioral_sources
            .voltage_sources
            .iter()
            .any(|source| !source.has_quasi_periodic_equation(self.behavioral_phase_dimensions))
            || self
                .behavioral_sources
                .current_sources
                .iter()
                .any(|source| !source.has_quasi_periodic_equation(self.behavioral_phase_dimensions))
        {
            return Err(Error::InvalidCircuit(
                "QPSS behavioral clocks require independent-phase forcing projection".into(),
            ));
        }
        if !self.l_matrix.is_empty() {
            return Err(Error::InvalidCircuit(
                "QPSS inductors require exact branch equations".into(),
            ));
        }
        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            return Err(Error::InvalidCircuit(
                "QPSS Verilog-A F/Q sampling is not connected".into(),
            ));
        }
        if self.periodic_mna_branches.len() != self.periodic_mna_branch_names.len() {
            return Err(Error::InvalidCircuit(
                "QPSS branch descriptors and names are misaligned".into(),
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for (index, branch) in self.periodic_mna_branches.iter().enumerate() {
            let (ordinal, pos, neg) = branch.ordinal_and_terminals();
            if ordinal != index + 1
                || pos > self.num_nodes
                || neg > self.num_nodes
                || (pos == neg && !branch.is_integral())
            {
                return Err(Error::InvalidCircuit(
                    "QPSS branch has invalid canonical MNA coordinates".into(),
                ));
            }
            match branch {
                ExactMnaBranch::Inductor { inductance, .. }
                    if !inductance.is_finite() || *inductance == 0.0 =>
                {
                    return Err(Error::InvalidCircuit(
                        "QPSS branch inductance must be finite and nonzero".into(),
                    ));
                }
                ExactMnaBranch::VoltageSource { source_index, .. } => {
                    if !seen.insert(*source_index) {
                        return Err(Error::InvalidCircuit(
                            "QPSS source has duplicate branch equations".into(),
                        ));
                    }
                    if let Some(source) = self.voltage_source_branches.get(*source_index)
                        && (source.node_pos != pos || source.node_neg != neg)
                    {
                        return Err(Error::InvalidCircuit(
                            "QPSS voltage source terminals disagree with its branch".into(),
                        ));
                    }
                }
                _ => {}
            }
        }
        if (0..self.voltage_source_branches.len()).any(|index| !seen.contains(&index)) {
            return Err(Error::InvalidCircuit(
                "QPSS voltage source is missing its exact branch equation".into(),
            ));
        }
        Ok(())
    }
}

impl Circuit for HbSolver {
    fn unknowns(&self) -> usize {
        self.num_nodes
            .saturating_add(self.periodic_mna_branches.len())
    }

    fn voltage_equation(&self, row: usize) -> bool {
        row >= self.num_nodes
            && self
                .periodic_mna_branches
                .get(row - self.num_nodes)
                .is_some_and(|coordinate| !coordinate.is_integral())
    }

    fn linear_entries(&self, frequency_hz: Value) -> Result<Vec<LinearEntry>, Error> {
        self.quasi_periodic_linear_entries(frequency_hz, false)
    }

    fn small_signal_entries(&self, frequency_hz: Value) -> Result<Vec<LinearEntry>, Error> {
        self.quasi_periodic_linear_entries(frequency_hz, true)
    }

    fn sample(&mut self, state: &[Value], jacobian: bool) -> Result<Sample, Error> {
        self.sample_at_phases(state, &[], jacobian)
    }

    fn sample_at_phases(
        &mut self,
        state: &[Value],
        phases: &[Value],
        jacobian: bool,
    ) -> Result<Sample, Error> {
        self.quasi_periodic_sample_selected(state, phases, jacobian, None)
    }
}

impl HbSolver {
    pub(in crate::analysis::harmonic_balance::solver) fn quasi_periodic_sample_selected(
        &mut self,
        state: &[Value],
        phases: &[Value],
        jacobian: bool,
        selected: Option<&[bool]>,
    ) -> Result<Sample, Error> {
        if selected.is_some_and(|rows| rows.len() != state.len()) {
            return Err(Error::InvalidCircuit(
                "selected periodic rows do not match the full state".into(),
            ));
        }
        // A registry without phase inputs contains only autonomous constitutive laws.
        let phases = if self.behavioral_phase_dimensions == 0 {
            &[]
        } else {
            phases
        };
        let mut sample = self
            .quasi_periodic_native_sample_selected(state, phases, jacobian, selected)
            .map_err(device_error)?;
        // Legacy compact devices use num_nodes as the ground sentinel.
        // Including branch-current coordinates would turn ground into the
        // first branch current when evaluating their terminal voltages.
        let voltages = &state[..self.num_nodes];
        for device in &self.nonlinear_devices {
            if selected.is_some_and(|rows| {
                !device
                    .terminals
                    .iter()
                    .any(|&node| node < self.num_nodes && rows[node])
            }) {
                continue;
            }
            sample.current.extend(
                device
                    .evaluate(voltages)
                    .into_iter()
                    .filter(|(row, _)| *row < self.num_nodes),
            );
            sample.charge.extend(
                device
                    .charge(voltages)
                    .into_iter()
                    .filter(|(row, _)| *row < self.num_nodes),
            );
            if jacobian {
                for (derivatives, target) in [
                    (device.jacobian(voltages), &mut sample.conductance),
                    (device.charge_jacobian(voltages), &mut sample.capacitance),
                ] {
                    target.extend(derivatives.into_iter().filter_map(|((row, col), value)| {
                        (row < self.num_nodes && col < self.num_nodes).then_some((row, col, value))
                    }));
                }
            }
        }
        Ok(sample)
    }
}

#[cfg(test)]
mod tests;
