//! Solve structurally independent nonlinear inputs without clamping integrals.
use super::*;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicError as Error, QuasiPeriodicGrid, QuasiPeriodicSolveConfig,
    solve::{self, Circuit, LinearEntry, Sample},
};
use crate::{ResourceKind, ResourceLimitError, ResourceLimits};
use std::collections::BTreeSet;
use std::sync::Arc;

fn check_abort(abort: &dyn AbortSignal) -> Result<(), Error> {
    if abort.is_aborted() {
        Err(Error::Aborted)
    } else {
        Ok(())
    }
}

pub(in crate::analysis::harmonic_balance::solver) struct NonlinearInputs {
    pub spectra: Vec<Option<Vec<Complex64>>>,
    pub iterations: usize,
}

#[derive(Clone, Copy)]
pub(super) enum InputBasis {
    IndependentPhases,
    PeriodicTime {
        frequency_hz: Value,
        steps: solve::NewtonStepPolicy,
    },
}

struct Subcircuit<'a> {
    solver: &'a mut HbSolver,
    rows: Vec<usize>,
    columns: Vec<usize>,
    row_index: Vec<usize>,
    column_index: Vec<usize>,
    selected: Vec<bool>,
    state: Vec<Value>,
    basis: InputBasis,
    known_integrals: &'a [bool],
    known_samples: &'a [Option<Vec<Value>>],
    grid: &'a QuasiPeriodicGrid,
}

impl Circuit for Subcircuit<'_> {
    fn unknowns(&self) -> usize {
        self.columns.len()
    }
    fn voltage_equation(&self, row: usize) -> bool {
        self.solver.voltage_equation(self.rows[row])
    }
    fn linear_entries(&self, frequency: Value) -> Result<Vec<LinearEntry>, Error> {
        let mut entries = Vec::new();
        self.solver
            .visit_periodic_linear_entries(frequency, false, |row, col, value| {
                if self.selected[row] && value != Complex64::ZERO {
                    if self.column_index[col] == usize::MAX {
                        return Err(Error::InvalidCircuit(
                            "nonlinear input subsystem has an external linear dependency".into(),
                        ));
                    }
                    entries.push((self.row_index[row], self.column_index[col], value));
                }
                Ok(())
            })?;
        Ok(entries)
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
        if state.len() != self.columns.len() {
            return Err(Error::InvalidCircuit(
                "nonlinear input subsystem state has the wrong size".into(),
            ));
        }
        for (&col, &value) in self.columns.iter().zip(state) {
            self.state[col] = value;
        }
        // Only previously qualified producer primitives are external inputs.
        // Use their retained circuit coordinates, not omitted interpolation
        // modes; nonlinear downstream laws must see the same state as HB/QPSS.
        if self.known_samples.iter().any(Option::is_some) {
            if phases.len() != self.grid.dimensions().len() {
                return Err(Error::InvalidCircuit(
                    "prescribed inputs require the registered phase grid".into(),
                ));
            }
            let mut sample = 0;
            let mut stride = 1;
            for (&phase, &size) in phases.iter().zip(self.grid.dimensions()) {
                let index = (phase * size as Value / std::f64::consts::TAU).round() as usize;
                if index >= size || phase != std::f64::consts::TAU * index as Value / size as Value
                {
                    return Err(Error::InvalidCircuit(
                        "prescribed input is outside the collocation grid".into(),
                    ));
                }
                sample += index * stride;
                stride *= size;
            }
            for (target, values) in self.state.iter_mut().zip(self.known_samples) {
                if let Some(values) = values {
                    *target = values[sample];
                }
            }
        }
        let sample = match self.basis {
            InputBasis::IndependentPhases => self.solver.quasi_periodic_sample_selected(
                &self.state,
                phases,
                jacobian,
                Some(&self.selected),
            )?,
            InputBasis::PeriodicTime { frequency_hz, .. } => {
                let [phase] = phases else {
                    return Err(Error::InvalidCircuit(
                        "HB input preparation requires one phase".into(),
                    ));
                };
                self.solver.periodic_sample_selected(
                    &self.state,
                    (phase / std::f64::consts::TAU) / frequency_hz,
                    &[],
                    jacobian,
                    Some(&self.selected),
                )?
            }
        };
        let residual = |terms: Vec<(usize, Value)>| {
            terms
                .into_iter()
                .filter_map(|(row, value)| {
                    self.selected[row].then_some((self.row_index[row], value))
                })
                .collect()
        };
        let jacobian = |terms: Vec<(usize, usize, Value)>| -> Result<Vec<_>, Error> {
            let mut selected = Vec::new();
            for (row, col, value) in terms {
                if !self.selected[row] {
                    continue;
                }
                if self.column_index[col] == usize::MAX {
                    if value != 0.0 && !self.known_integrals[col] {
                        return Err(Error::InvalidCircuit(
                            "nonlinear input subsystem has an external derivative".into(),
                        ));
                    }
                } else {
                    selected.push((self.row_index[row], self.column_index[col], value));
                }
            }
            Ok(selected)
        };
        Ok(Sample {
            current: residual(sample.current),
            charge: residual(sample.charge),
            conductance: jacobian(sample.conductance)?,
            capacitance: jacobian(sample.capacitance)?,
        })
    }
}

impl HbSolver {
    /// Return complete solved components so the caller can preserve the chosen
    /// orbit as the full-circuit seed. Only already-qualified primitives can
    /// act as external inputs; unresolved integral feedback remains joint.
    #[expect(
        clippy::too_many_arguments,
        reason = "numerical API keeps independent circuit, spectral, and resource inputs explicit"
    )]
    pub(in crate::analysis::harmonic_balance::solver) fn nonlinear_driven_spectra(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        config: &QuasiPeriodicSolveConfig,
        sources: &[Vec<Complex64>],
        seed: Option<&[Vec<Complex64>]>,
        needed: &[bool],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<NonlinearInputs, Error> {
        self.nonlinear_driven_spectra_in_basis(
            grid,
            config,
            sources,
            seed,
            needed,
            limits,
            InputBasis::IndependentPhases,
            abort,
        )
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "numerical API keeps independent circuit, spectral, and resource inputs explicit"
    )]
    pub(super) fn nonlinear_driven_spectra_in_basis(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        config: &QuasiPeriodicSolveConfig,
        sources: &[Vec<Complex64>],
        seed: Option<&[Vec<Complex64>]>,
        needed: &[bool],
        limits: &ResourceLimits,
        basis: InputBasis,
        abort: &dyn AbortSignal,
    ) -> Result<NonlinearInputs, Error> {
        check_abort(abort)?;
        let n = self.num_nodes + self.exact_mna_branches().len();
        let physical = self.num_nodes + self.physical_branch_count();
        if needed.len() != n || !self.l_matrix.is_empty() {
            return Err(Error::InvalidCircuit(
                "nonlinear input subsystem requires a complete exact MNA basis".into(),
            ));
        }
        solve::validate_spectra(sources, n, &grid, "source", abort)?;
        if let Some(seed) = seed {
            solve::validate_spectra(seed, n, &grid, "initial state", abort)?;
        }
        let budget = |values| {
            ResourceLimitError::ensure(
                ResourceKind::ResultValues,
                values,
                limits.max_result_values.min(32_000_000),
            )
            .map_err(Error::from)
        };
        let mut storage = n.saturating_mul(64);
        budget(storage)?;
        let known_integrals: Vec<_> = (0..n)
            .map(|column| {
                if column < physical {
                    return false;
                }
                let index = column - physical;
                match basis {
                    InputBasis::IndependentPhases => self
                        .quasi_prescribed_integrals
                        .as_ref()
                        .is_some_and(|cache| cache.known_input(index)),
                    InputBasis::PeriodicTime { .. } => matches!(
                        self.prescribed_integrals.get(index),
                        Some(Some(
                            super::prescribed::PrescribedIntegral::Primitive(_)
                                | super::prescribed::PrescribedIntegral::Driven(_)
                        ))
                    ),
                }
            })
            .collect();
        let mut pattern = BTreeSet::new();
        let rate_start = self.num_nodes + self.capacitor_rate_start();
        let independent_coordinate = |coordinate| coordinate < physical || coordinate >= rate_start;
        let mut insert = |row: usize, col: usize| -> Result<(), Error> {
            check_abort(abort)?;
            // Lifted phase coordinates are parameters, never circuit unknowns.
            if independent_coordinate(row)
                && col < n
                && !known_integrals[col]
                && !pattern.contains(&(row, col))
            {
                storage = storage.saturating_add(16);
                budget(storage)?;
                pattern.insert((row, col));
            }
            Ok(())
        };
        for &frequency in grid.frequencies_hz() {
            self.visit_periodic_linear_entries(frequency, false, |row, col, value| {
                if value != Complex64::ZERO {
                    insert(row, col)?;
                }
                Ok(())
            })?;
        }
        for device in &self.nonlinear_devices {
            for &row in &device.terminals {
                for &col in &device.terminals {
                    if row < self.num_nodes && col < self.num_nodes {
                        insert(row, col)?;
                    }
                }
            }
        }
        for device in &self.native_bjts {
            for row in device.mna_coupling_nodes() {
                for col in device.mna_coupling_nodes() {
                    if row > 0 && col > 0 {
                        insert(row - 1, col - 1)?;
                    }
                }
            }
        }
        let mut capacitor_state = physical + self.behavioral_sources.integral_count();
        for (index, capacitor) in self.periodic_capacitors.iter().enumerate() {
            let rate = rate_start + index;
            let start = capacitor_state;
            capacitor_state += capacitor.expression.program.sdt_count;
            insert(rate, rate)?;
            for node in [capacitor.pos, capacitor.neg] {
                if node == 0 {
                    continue;
                }
                insert(rate, node - 1)?;
            }
            let rows = capacitor
                .branch
                .map_or([Some(capacitor.pos), Some(capacitor.neg)], |branch| {
                    [Some(branch + 1), None]
                });
            for row in rows.into_iter().flatten().filter(|row| *row > 0) {
                insert(row - 1, rate)?;
                if capacitor.branch.is_some() {
                    insert(row - 1, row - 1)?;
                }
                for column in capacitor
                    .expression
                    .bound_solution_indices()
                    .chain(start..capacitor_state)
                {
                    insert(row - 1, column)?;
                }
            }
        }
        let mut result = Ok(());
        self.behavioral_sources.visit_periodic_output_dependencies(
            self.num_nodes,
            physical,
            |row, col| {
                if result.is_ok() {
                    result = insert(row, col);
                }
            },
        );
        result?;
        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            return Err(Error::InvalidCircuit(
                "native Verilog-A quasiperiodic sampling is unavailable".into(),
            ));
        }
        let mut rows = vec![Vec::new(); n];
        for &(row, col) in &pattern {
            rows[row].push(col);
        }
        let components: Vec<_> = super::linear::closed_components(&rows, abort)?
            .into_iter()
            .filter(|component| {
                component
                    .iter()
                    .all(|&(_, col)| independent_coordinate(col))
                    && component.iter().any(|&(_, col)| needed[col])
            })
            .collect();
        let count: usize = components.iter().map(Vec::len).sum();
        storage = storage.saturating_add(count.saturating_mul(grid.len()).saturating_mul(2));
        budget(storage)?;
        let mut known_samples = vec![None; n];
        if !components.is_empty() && known_integrals.iter().any(|&known| known) {
            storage = storage.saturating_add(
                known_integrals
                    .iter()
                    .filter(|&&known| known)
                    .count()
                    .saturating_mul(grid.sample_count()),
            );
            budget(
                storage
                    .saturating_add(grid.sample_count().saturating_mul(8))
                    .saturating_add(grid.len().saturating_mul(2)),
            )?;
            let mut transform =
                crate::analysis::quasi_periodic::QuasiPeriodicTransform::new_with_abort(
                    grid.clone(),
                    abort,
                )?;
            let mut spectrum = vec![Complex64::ZERO; grid.len()];
            for (column, &known) in known_integrals.iter().enumerate() {
                if !known {
                    continue;
                }
                check_abort(abort)?;
                let index = column - physical;
                match basis {
                    InputBasis::IndependentPhases => self
                        .quasi_prescribed_integrals
                        .as_ref()
                        .expect("producer cache")
                        .project_input(index, &mut spectrum, abort)?,
                    InputBasis::PeriodicTime { .. } => {
                        let primitive = self.prescribed_integrals[index]
                            .as_ref()
                            .expect("producer primitive");
                        let (super::prescribed::PrescribedIntegral::Primitive(values)
                        | super::prescribed::PrescribedIntegral::Driven(values)) = primitive
                        else {
                            unreachable!()
                        };
                        let h = grid.dc_index();
                        spectrum[h..].copy_from_slice(values);
                        for k in 1..=h {
                            spectrum[h - k] = values[k].conj();
                        }
                    }
                }
                known_samples[column] =
                    Some(transform.to_real_samples_with_abort(&spectrum, abort)?);
            }
        }
        let mut spectra = vec![None; n];
        let mut iterations = 0usize;
        for component in components {
            check_abort(abort)?;
            let mut row_index = vec![usize::MAX; n];
            let mut column_index = vec![usize::MAX; n];
            let mut selected = vec![false; n];
            for (i, &(row, col)) in component.iter().enumerate() {
                row_index[row] = i;
                column_index[col] = i;
                selected[row] = true;
            }
            let mut working = *limits;
            // Selected RHS and seed coexist with the shared solver workspace.
            let retained = storage
                .saturating_add(component.len().saturating_mul(grid.len()).saturating_mul(4));
            budget(retained)?;
            working.max_result_values = working
                .max_result_values
                .min(32_000_000)
                .saturating_sub(retained);
            let rhs: Vec<_> = component
                .iter()
                .map(|&(row, _)| sources[row].clone())
                .collect();
            let initial: Option<Vec<_>> = seed.map(|seed| {
                component
                    .iter()
                    .map(|&(_, col)| seed[col].clone())
                    .collect()
            });
            let mut circuit = Subcircuit {
                rows: component.iter().map(|&(row, _)| row).collect(),
                columns: component.iter().map(|&(_, col)| col).collect(),
                solver: self,
                row_index,
                column_index,
                selected,
                state: vec![0.0; n],
                basis,
                known_integrals: &known_integrals,
                known_samples: &known_samples,
                grid: &grid,
            };
            let steps = match basis {
                InputBasis::IndependentPhases => solve::NewtonStepPolicy::default(),
                InputBasis::PeriodicTime { steps, .. } => steps,
            };
            let solution = match solve::solve_with_step_policy(
                &mut circuit,
                grid.clone(),
                config,
                &rhs,
                initial.as_deref(),
                &working,
                config.max_iterations.saturating_sub(iterations),
                steps,
                abort,
            ) {
                Ok(solution) => solution,
                // Periodic closure outside this component may fix a neutral
                // mode. An inconclusive preliminary solve is not a new gauge.
                Err(Error::LinearSolve(
                    crate::solver::SolverError::SingularMatrix
                    | crate::solver::SolverError::InaccurateSolution(_),
                )) => continue,
                Err(Error::ConvergenceFailed {
                    iterations: used,
                    merit,
                }) => {
                    iterations += used;
                    if iterations >= config.max_iterations {
                        match basis {
                            InputBasis::IndependentPhases => {
                                return Err(Error::ConvergenceFailed { iterations, merit });
                            }
                            InputBasis::PeriodicTime { .. } => break,
                        }
                    }
                    continue;
                }
                Err(error) => return Err(error),
            };
            iterations += solution.iterations();
            for ((_, col), values) in component.into_iter().zip(solution.into_spectra()) {
                spectra[col] = Some(values);
            }
        }
        check_abort(abort)?;
        Ok(NonlinearInputs {
            spectra,
            iterations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::CountingAbort;
    use crate::analysis::quasi_periodic::QuasiPeriodicGridConfig;
    use crate::device::behavioral::{
        BehavioralBranchResolution, BehavioralCurrentSource, BehavioralSources,
    };

    #[test]
    fn periodic_nonlinear_inputs_preserve_time_seed_and_damping() {
        let limits = ResourceLimits::default();
        let rate = 1e3;
        let grid = Arc::new(
            QuasiPeriodicGrid::periodic_with_abort(rate, 3, 17, &limits, &NoAbort).unwrap(),
        );
        assert_eq!(grid.sample_count(), 17);
        assert_eq!(grid.len(), 7);
        // The public QPSS API must still reject a single tone.
        assert!(
            QuasiPeriodicGrid::new_with_abort(
                QuasiPeriodicGridConfig::new(vec![rate], vec![3]),
                &limits,
                &NoAbort
            )
            .is_err()
        );
        let mut solver = HbSolver::new(
            HbConfig::new(rate)
                .with_harmonics(3)
                .with_collocation_points(17),
            2,
        );
        let mut active = BehavioralCurrentSource::new(
            "Bactive".into(),
            1,
            0,
            "(v(input)-sin(2*pi*1k*time))^3-(v(input)-sin(2*pi*1k*time))",
        )
        .unwrap();
        active
            .bind_references(|_| Some(1), |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        let mut unrelated =
            BehavioralCurrentSource::new("Bunrelated".into(), 2, 0, "ln(v(other))").unwrap();
        unrelated
            .bind_references(|_| Some(2), |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        solver
            .set_periodic_behavioral_sources(
                &BehavioralSources {
                    voltage_sources: vec![],
                    current_sources: vec![active, unrelated],
                },
                false,
                limits.max_result_values,
                false,
                &NoAbort,
            )
            .unwrap();
        let sources = vec![vec![Complex64::ZERO; grid.len()]; 2];
        let mut seed = sources.clone();
        seed[0][grid.dc_index() + 1] = Complex64::new(0.0, -0.5);
        seed[0][grid.dc_index() - 1] = Complex64::new(0.0, 0.5);
        let mut config = QuasiPeriodicSolveConfig {
            relative_tolerance: 1e-10,
            ..Default::default()
        };
        for sign in [-1.0, 1.0] {
            seed[0][grid.dc_index()] = Complex64::new(sign * 0.8, 0.0);
            config.linear.method = if sign < 0.0 {
                crate::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov
            } else {
                crate::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Direct
            };
            let solved = solver
                .nonlinear_driven_spectra_in_basis(
                    grid.clone(),
                    &config,
                    &sources,
                    Some(&seed),
                    &[true, false],
                    &limits,
                    InputBasis::PeriodicTime {
                        frequency_hz: rate,
                        steps: solve::NewtonStepPolicy::HarmonicBalance {
                            damping: 0.5,
                            minimum_damping: 0.125,
                        },
                    },
                    &NoAbort,
                )
                .unwrap();
            let values = solved.spectra[0].as_ref().unwrap();
            assert!((values[grid.dc_index()].re - sign).abs() < 1e-9);
            assert!((values[grid.dc_index() + 1] - Complex64::new(0.0, -0.5)).norm() < 1e-9);
            assert!(
                solved.iterations > 20 && solved.iterations <= config.max_iterations,
                "the configured half-step must be used: {}",
                solved.iterations
            );
            assert!(
                solved.spectra[1].is_none(),
                "unrelated logarithm must not be sampled"
            );
        }
    }

    #[test]
    fn independent_nonlinear_inputs_preserve_seed_and_skip_unrelated_expressions() {
        let limits = ResourceLimits::default();
        let grid = Arc::new(
            QuasiPeriodicGrid::new_with_abort(
                QuasiPeriodicGridConfig::new(vec![1e3, 1e3 * std::f64::consts::SQRT_2], vec![1, 1]),
                &limits,
                &NoAbort,
            )
            .unwrap(),
        );
        let mut solver = HbSolver::new(HbConfig::new(17.0).with_harmonics(1), 2);
        let mut active =
            BehavioralCurrentSource::new("Bactive".into(), 1, 0, "v(input)^3-v(input)").unwrap();
        active
            .bind_references(|_| Some(1), |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        let mut unrelated =
            BehavioralCurrentSource::new("Bunrelated".into(), 2, 0, "ln(v(other))").unwrap();
        unrelated
            .bind_references(|_| Some(2), |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        solver
            .set_quasi_periodic_behavioral_sources(
                &BehavioralSources {
                    voltage_sources: vec![],
                    current_sources: vec![active, unrelated],
                },
                &grid,
            )
            .unwrap();
        let mut config = QuasiPeriodicSolveConfig {
            relative_tolerance: 1e-10,
            ..Default::default()
        };
        let sources = vec![vec![Complex64::ZERO; grid.len()]; 2];
        for sign in [-1.0, 1.0] {
            config.linear.method = if sign < 0.0 {
                crate::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov
            } else {
                crate::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Direct
            };
            let mut seed = sources.clone();
            seed[0][grid.dc_index()] = Complex64::new(sign * 0.8, 0.0);
            let solved = solver
                .nonlinear_driven_spectra(
                    grid.clone(),
                    &config,
                    &sources,
                    Some(&seed),
                    &[true, false],
                    &limits,
                    &NoAbort,
                )
                .unwrap();
            assert!((solved.spectra[0].as_ref().unwrap()[grid.dc_index()].re - sign).abs() < 1e-9);
            assert!(
                solved.spectra[1].is_none(),
                "unrelated logarithm must not be sampled at zero"
            );
        }
        assert!(matches!(
            solver.nonlinear_driven_spectra(
                grid.clone(),
                &config,
                &sources,
                None,
                &[true, false],
                &ResourceLimits {
                    max_result_values: 1,
                    ..limits
                },
                &NoAbort
            ),
            Err(Error::ResourceLimit(_))
        ));
        assert!(matches!(
            solver.nonlinear_driven_spectra(
                grid.clone(),
                &config,
                &sources,
                None,
                &[true, false],
                &limits,
                &CountingAbort::new(1)
            ),
            Err(Error::Aborted)
        ));
        let mut feedback =
            BehavioralCurrentSource::new("Bfeedback".into(), 1, 0, "sdt(v(input))").unwrap();
        feedback
            .bind_references(|_| Some(1), |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        solver
            .set_quasi_periodic_behavioral_sources(
                &BehavioralSources {
                    voltage_sources: vec![],
                    current_sources: vec![feedback],
                },
                &grid,
            )
            .unwrap();
        let sources = vec![vec![Complex64::ZERO; grid.len()]; 3];
        let unresolved = solver
            .nonlinear_driven_spectra(
                grid,
                &config,
                &sources,
                None,
                &[true, false, false],
                &limits,
                &NoAbort,
            )
            .unwrap();
        assert!(
            unresolved.spectra.iter().all(Option::is_none),
            "integral feedback remains in the joint solve"
        );
    }
}
