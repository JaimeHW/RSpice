//! Resolve independent nonlinear input orbits before the joint HB solve.
use super::nonlinear::InputBasis;
use super::prescribed::PrescribedIntegral;
use super::*;
use crate::ResourceLimits;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicError, QuasiPeriodicGrid, QuasiPeriodicLinearConfig, QuasiPeriodicLinearMethod,
    QuasiPeriodicSolveConfig, solve::NewtonStepPolicy,
};
use std::sync::Arc;

fn preparation_error(error: QuasiPeriodicError) -> HbError {
    match error {
        QuasiPeriodicError::Aborted => HbError::Aborted,
        other => HbError::InvalidCircuit(format!("nonlinear integral input preparation: {other}")),
    }
}

impl HbSolver {
    pub(in crate::analysis::harmonic_balance::solver) fn prepare_nonlinear_integral_inputs(
        &mut self,
        state: &mut HbSolverState,
        mut needed: Vec<bool>,
        abort: &dyn AbortSignal,
    ) -> Result<(), HbError> {
        if !needed.iter().any(|&value| value) {
            return Ok(());
        }
        let Some(max_values) = self.periodic_integral_budget else {
            return Ok(());
        };
        let n = self.num_nodes + self.exact_mna_branches().len();
        if needed.len() != n || n > max_values {
            return Err(HbError::InvalidCircuit(
                "nonlinear integral input metadata exceeds the periodic basis or allocation limit"
                    .into(),
            ));
        }
        let mut known_inputs = vec![false; n];
        let start = state.total_iterations;
        while needed.iter().any(|&value| value) {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let used = state.total_iterations - start;
            if used >= self.config.max_iterations {
                break;
            }
            needed = self.prepare_nonlinear_integral_input_pass(
                state,
                needed,
                &mut known_inputs,
                self.config.max_iterations - used,
                abort,
            )?;
        }
        Ok(())
    }

    fn prepare_nonlinear_integral_input_pass(
        &mut self,
        state: &mut HbSolverState,
        needed: Vec<bool>,
        known_inputs: &mut [bool],
        remaining_iterations: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<bool>, HbError> {
        if !needed.iter().any(|&value| value) {
            return Ok(Vec::new());
        }
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let Some(max_values) = self.periodic_integral_budget else {
            return Ok(Vec::new());
        };
        let n = self.num_nodes + self.exact_mna_branches().len();
        let h = self.num_harmonics;
        if state.x.len() != self.num_nodes
            || state.mna_branch_currents.len() != n - self.num_nodes
            || state
                .x
                .iter()
                .chain(&state.mna_branch_currents)
                .any(|row| row.len() != h + 1)
        {
            return Err(HbError::InvalidCircuit(
                "nonlinear input preparation requires the complete HB startup state".into(),
            ));
        }
        let cache_values = self
            .prescribed_integrals
            .capacity()
            .saturating_mul(4)
            .saturating_add(
                self.prescribed_integrals
                    .iter()
                    .flatten()
                    .map(|row| match row {
                        PrescribedIntegral::Primitive(values)
                        | PrescribedIntegral::Driven(values) => values.capacity().saturating_mul(2),
                        PrescribedIntegral::Retained => 0,
                    })
                    .sum::<usize>(),
            );
        let ensure = |values: usize| {
            if values > max_values {
                Err(HbError::InvalidCircuit(format!(
                    "nonlinear integral inputs exceed the {max_values}-value periodic allocation limit"
                )))
            } else {
                Ok(())
            }
        };
        // Both signed RHS and seed coexist with the solver's selected copies.
        let signed_values = n.saturating_mul((2 * h + 1).saturating_mul(4).saturating_add(8));
        let retained = cache_values
            .saturating_add(needed.len())
            .saturating_add(known_inputs.len())
            .saturating_add(signed_values);
        ensure(retained)?;
        let mut limits = ResourceLimits {
            max_result_values: max_values.saturating_sub(retained),
            ..ResourceLimits::unlimited()
        };
        let grid = Arc::new(
            QuasiPeriodicGrid::periodic_with_abort(
                self.config.fundamental_freq,
                h,
                self.fft.size(),
                &limits,
                abort,
            )
            .map_err(preparation_error)?,
        );
        let grid_values = grid.len().saturating_mul(8);
        ensure(retained.saturating_add(grid_values))?;
        limits.max_result_values = limits.max_result_values.saturating_sub(grid_values);
        let mut sources = Vec::with_capacity(n);
        let mut seed = Vec::with_capacity(n);
        for (row, initial) in state.x.iter().chain(&state.mna_branch_currents).enumerate() {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let source = |k| {
                if row < self.num_nodes {
                    self.source_spectra[row][k]
                } else if let ExactMnaBranch::VoltageSource {
                    source: Some(source),
                    ..
                } = &self.exact_mna_branches()[row - self.num_nodes]
                {
                    Self::voltage_source_value_at_harmonic(source, k)
                } else {
                    Complex64::ZERO
                }
            };
            sources.push(
                (1..=h)
                    .rev()
                    .map(|k| source(k).conj())
                    .chain((0..=h).map(source))
                    .collect(),
            );
            seed.push(
                initial[1..]
                    .iter()
                    .rev()
                    .map(|v| v.conj())
                    .chain(initial.iter().copied())
                    .collect(),
            );
        }
        // This is an exact physical input certificate, like linear input
        // preparation, not a replacement for HB's selected Jacobian or its
        // continuation ladder. It owns one bounded initialization-stage
        // budget; the ordinary HB phases retain their configured budgets.
        let config = QuasiPeriodicSolveConfig {
            relative_tolerance: self.config.tolerance,
            current_absolute_tolerance: self.config.abstol,
            voltage_absolute_tolerance: self.voltage_abstol,
            max_iterations: remaining_iterations,
            linear: QuasiPeriodicLinearConfig {
                method: if self.config.use_krylov
                    || n.saturating_mul(grid.len()) >= super::super::krylov::KRYLOV_AUTO_THRESHOLD
                {
                    QuasiPeriodicLinearMethod::Krylov
                } else {
                    QuasiPeriodicLinearMethod::Auto
                },
                restart: self.config.gmres_restart.max(8),
                ..Default::default()
            },
            ..Default::default()
        };
        let driven = self
            .nonlinear_driven_spectra_in_basis(
                grid,
                &config,
                &sources,
                Some(&seed),
                &needed,
                &limits,
                InputBasis::PeriodicTime {
                    frequency_hz: self.config.fundamental_freq,
                    steps: NewtonStepPolicy::HarmonicBalance {
                        damping: self.config.damping,
                        minimum_damping: self.config.min_damping,
                    },
                },
                abort,
            )
            .map_err(preparation_error)?;
        state.total_iterations = state
            .total_iterations
            .checked_add(driven.iterations)
            .ok_or_else(|| {
                HbError::InvalidCircuit("HB preparation iteration counter overflowed".into())
            })?;
        drop(sources);
        drop(seed);
        drop(needed);
        let mut driven = driven.spectra;
        if driven.iter().all(Option::is_none) {
            return Ok(Vec::new());
        }
        for spectrum in driven.iter_mut().flatten() {
            spectrum.drain(..h);
        }
        let mut retained_inputs = driven.capacity().saturating_mul(4).saturating_add(
            driven
                .iter()
                .flatten()
                .map(|row| row.capacity().saturating_mul(2))
                .sum::<usize>(),
        );
        // Keep inputs that qualified earlier primitives, including separate
        // circuit components absent from this pass's newly needed inputs.
        for ((target, &known), spectrum) in driven
            .iter_mut()
            .zip(known_inputs.iter())
            .zip(state.x.iter().chain(&state.mna_branch_currents))
        {
            if target.is_none() && known {
                retained_inputs = retained_inputs.saturating_add(spectrum.len().saturating_mul(2));
                ensure(
                    cache_values
                        .saturating_add(retained_inputs)
                        .saturating_add(known_inputs.len()),
                )?;
                *target = Some(spectrum.clone());
            }
        }
        let driven_values = driven.capacity().saturating_mul(4).saturating_add(
            driven
                .iter()
                .flatten()
                .map(|row| row.capacity().saturating_mul(2))
                .sum::<usize>(),
        );
        ensure(cache_values.saturating_add(driven_values))?;
        let initially_prescribed = self.prescribed_integrals.iter().flatten().count();
        let sources = std::mem::take(&mut self.behavioral_sources);
        let prepared = self.prepare_integrals_with_inputs(
            &sources,
            max_values
                .saturating_sub(cache_values)
                .saturating_sub(driven_values)
                .saturating_sub(known_inputs.len()),
            false,
            true,
            Some(&driven),
            abort,
        );
        self.behavioral_sources = sources;
        let (prepared, needed) = prepared?;
        if prepared.iter().flatten().count() <= initially_prescribed {
            return Ok(Vec::new());
        }
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        self.prescribed_integrals = prepared;
        // Apply only after a new primitive was qualified, preserving every
        // other supplied coordinate and the selected nonlinear branch.
        for (row, (target, spectrum)) in state
            .x
            .iter_mut()
            .chain(&mut state.mna_branch_currents)
            .zip(driven)
            .enumerate()
        {
            if let Some(spectrum) = spectrum {
                *target = spectrum;
                known_inputs[row] = true;
            }
        }
        for (target, primitive) in state.mna_branch_currents[self.physical_branch_count()..]
            .iter_mut()
            .zip(&self.prescribed_integrals)
        {
            if let Some(
                PrescribedIntegral::Primitive(values) | PrescribedIntegral::Driven(values),
            ) = primitive
            {
                target.copy_from_slice(values);
            }
        }
        // A continuing pass has strictly increased the qualified count;
        // dependency depth and the one shared stage budget bound repetition.
        Ok(needed)
    }
}
