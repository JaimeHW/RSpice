//! One unknown tone frequency and a Fourier phase condition on a real torus.
use super::evaluation::Evaluation;
use super::*;
use crate::analysis::quasi_periodic::{check_abort, finite};
use crate::numerics::krylov::{GmresError, try_gmres_with_abort};
use crate::solver::{SolverError, StaticMatrix};

const MAX_EXTENDED_BORDER_UNKNOWNS: usize = 256;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicAutonomousConfig {
    /// Zero-based independent frequency to solve; the other clocks stay fixed.
    pub tone: usize,
    /// Physical MNA coordinate observed by the phase condition.
    pub phase_coordinate: usize,
    /// Retained mixing tuple whose imaginary coefficient is pinned to zero.
    pub phase_tuple: Vec<i32>,
    /// Minimum peak amplitude of this Fourier component, in coordinate units.
    pub minimum_amplitude: Value,
    /// Bound each accepted frequency ratio and its reciprocal by 1 + this value.
    pub max_relative_frequency_step: Value,
}

impl QuasiPeriodicAutonomousConfig {
    pub(crate) fn phase_index(
        &self,
        grid: &QuasiPeriodicGrid,
        unknowns: usize,
    ) -> Result<usize, Error> {
        if self.tone >= grid.config().frequencies_hz.len()
            || self.phase_coordinate >= unknowns
            || self.phase_tuple.len() != grid.config().frequencies_hz.len()
            || self.phase_tuple[self.tone] == 0
            || !self.minimum_amplitude.is_finite()
            || self.minimum_amplitude <= 0.0
            || !self.max_relative_frequency_step.is_finite()
            || self.max_relative_frequency_step <= 0.0
            || self.max_relative_frequency_step > 1.0
        {
            return Err(Error::InvalidConfig(
                "invalid autonomous tone, phase coordinate, amplitude or frequency-step limit"
                    .into(),
            ));
        }
        grid.index_of(&self.phase_tuple)
            .filter(|&index| index != grid.dc_index())
            .ok_or_else(|| {
                Error::InvalidConfig("the oscillator phase tuple is not retained".into())
            })
    }
}

fn phase_column(
    config: &QuasiPeriodicAutonomousConfig,
    grid: &QuasiPeriodicGrid,
    index: usize,
) -> (usize, Value) {
    let dc = grid.dc_index();
    let positive = index.max(grid.len() - 1 - index);
    (
        config.phase_coordinate * grid.len() + 2 * (positive - dc),
        if index > dc { 1.0 } else { -1.0 },
    )
}

impl Workspace<'_> {
    /// d(F + dQ/dt)/d(log f_free). Charge is differentiated symbolically, so
    /// large cancelling static terms do not contaminate the frequency column.
    fn frequency_column(
        &self,
        circuit: &impl Circuit,
        evaluation: &Evaluation,
        spectra: &[Vec<Complex64>],
        tone: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let mut column = vec![vec![Complex64::ZERO; self.grid.len()]; self.unknowns];
        for (k, tuple) in self.grid.indices().iter().enumerate() {
            check_abort(abort)?;
            let rate = tuple[tone] as Value * self.grid.config().frequencies_hz[tone];
            if rate == 0.0 {
                continue;
            }
            for (row, output) in column.iter_mut().enumerate() {
                output[k] =
                    -Complex64::new(0.0, std::f64::consts::TAU * rate) * evaluation.charge[row][k];
            }
            for (row, col, derivative) in
                circuit.linear_frequency_derivative(self.grid.frequencies_hz()[k])?
            {
                if row >= self.unknowns || col >= self.unknowns || !finite(derivative) {
                    return Err(Error::InvalidCircuit(
                        "invalid autonomous frequency derivative".into(),
                    ));
                }
                column[row][k] += rate * derivative * spectra[col][k];
            }
        }
        if column.iter().flatten().any(|value| !finite(*value)) {
            return Err(Error::Numerical(
                "autonomous frequency column overflowed".into(),
            ));
        }
        Ok(coordinates::encode(&column))
    }

    fn bordered_action(
        &mut self,
        evaluation: &Evaluation,
        frequency: &[Value],
        phase: (usize, Value),
        direction: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let size = frequency.len();
        let mut result = self.jacobian_action(evaluation, &direction[..size], abort)?;
        for (value, derivative) in result.iter_mut().zip(frequency) {
            *value += derivative * direction[size];
        }
        result.push(phase.1 * direction[phase.0]);
        Ok(result)
    }

    fn bordered_correction(
        &mut self,
        evaluation: &Evaluation,
        frequency: &[Value],
        phase: (usize, Value),
        phase_residual: Value,
        certify: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let size = frequency.len() + 1;
        let mut rhs = coordinates::encode(&evaluation.residual);
        rhs.push(-phase_residual);
        if !self.config.linear.uses_krylov(size) {
            let mut triplets = Vec::new();
            let mut direction = vec![0.0; size];
            for col in 0..size {
                direction[col] = 1.0;
                let values =
                    self.bordered_action(evaluation, frequency, phase, &direction, abort)?;
                direction[col] = 0.0;
                triplets.extend(values.into_iter().enumerate().filter_map(|(row, value)| {
                    (row == col || value != 0.0).then_some((row, col, value))
                }));
            }
            let mut matrix = StaticMatrix::from_triplets(size, size, &triplets)?;
            let result = match matrix.solve(&rhs) {
                // The sparse bordered factorization can lose accuracy for a
                // near-neutral oscillator mode. Keep the extended-precision
                // fallback bounded while covering moderate QP lattices.
                Err(SolverError::InaccurateSolution(_)) if size <= MAX_EXTENDED_BORDER_UNKNOWNS => {
                    matrix.solve_dense_extended(&rhs)
                }
                other => other,
            }?;
            check_abort(abort)?;
            return Ok(result);
        }
        let row_divisors = self.equation_divisors(evaluation, abort)?;
        let mut divisors = (0..frequency.len())
            .map(|i| row_divisors[i / self.grid.len()].max(frequency[i].abs()))
            .collect::<Vec<_>>();
        divisors.push(1.0);
        let correction = if certify {
            vec![0.0; size]
        } else {
            self.bordered_krylov(evaluation, frequency, phase, &divisors, &rhs, true, abort)?
        };
        if certify {
            // A phase condition removes one neutral direction, not arbitrary
            // floating coordinates or an unresolved oscillator amplitude.
            let mut basis = vec![0.0; size];
            let mut bounds = vec![0.0; size];
            for col in 0..size {
                check_abort(abort)?;
                basis[col] = divisors[col];
                let inverse = self.bordered_krylov(
                    evaluation, frequency, phase, &divisors, &basis, false, abort,
                )?;
                basis[col] = 0.0;
                let applied =
                    self.bordered_action(evaluation, frequency, phase, &inverse, abort)?;
                for (row, bound) in bounds.iter_mut().enumerate() {
                    let value = applied[row] / divisors[row];
                    *bound += (value - if row == col { 1.0 } else { 0.0 }).abs()
                        + 1024.0 * Value::EPSILON * (1.0 + value.abs());
                    if !bound.is_finite() || *bound >= 0.5 {
                        return Err(Error::Numerical("autonomous QPSS could not certify a nonsingular phase/frequency system".into()));
                    }
                }
            }
        }
        Ok(correction)
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "the bordered solve keeps its numerical inputs explicit"
    )]
    fn bordered_krylov(
        &mut self,
        evaluation: &Evaluation,
        frequency: &[Value],
        phase: (usize, Value),
        divisors: &[Value],
        rhs: &[Value],
        require_convergence: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        let settings = self.config.linear.clone();
        let rhs = rhs
            .iter()
            .zip(divisors)
            .map(|(v, d)| Complex64::new(v / d, 0.0))
            .collect::<Vec<_>>();
        let outcome = try_gmres_with_abort(
            &mut |values: &[Complex64]| {
                if values.iter().any(|v| !finite(*v) || v.im != 0.0) {
                    return Err(Error::Numerical(
                        "autonomous Krylov direction is not real".into(),
                    ));
                }
                let direction = values.iter().map(|v| v.re).collect::<Vec<_>>();
                Ok(self
                    .bordered_action(evaluation, frequency, phase, &direction, abort)?
                    .iter()
                    .zip(divisors)
                    .map(|(v, d)| Complex64::new(v / d, 0.0))
                    .collect())
            },
            &mut |values: &[Complex64]| Ok(values.to_vec()),
            &rhs,
            settings.restart,
            settings.max_cycles,
            settings.relative_tolerance,
            &|| abort.is_aborted(),
        )
        .map_err(|error| match error {
            GmresError::Aborted => Error::Aborted,
            GmresError::Operator(error) => error,
            GmresError::InvalidData(reason) => Error::Numerical(reason.into()),
        })?;
        if require_convergence && !outcome.converged {
            return Err(Error::Numerical(format!(
                "autonomous QPSS Krylov solve did not converge (relative residual {:e})",
                outcome.relative_residual
            )));
        }
        if outcome.solution.iter().any(|v| !finite(*v) || v.im != 0.0) {
            return Err(Error::Numerical(
                "autonomous Krylov solution is not real".into(),
            ));
        }
        Ok(outcome.solution.iter().map(|v| v.re).collect())
    }
}

fn workspace<'a>(
    circuit: &impl Circuit,
    grid: Arc<QuasiPeriodicGrid>,
    config: &'a QuasiPeriodicSolveConfig,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Workspace<'a>, Error> {
    let unknowns = circuit.unknowns();
    let size = unknowns.saturating_mul(grid.len()).saturating_add(1);
    ResourceLimitError::ensure(
        ResourceKind::MatrixUnknowns,
        size,
        if config.linear.uses_krylov(size) {
            limits.max_matrix_unknowns
        } else {
            limits.max_matrix_unknowns.min(MAX_DENSE_UNKNOWNS)
        },
    )?;
    // Select and budget the backend for the bordered size, including its
    // frequency unknown. Auto can cross the dense cutoff at this extra row.
    let mut linear = config.linear.clone();
    linear.method = if linear.uses_krylov(size) {
        QuasiPeriodicLinearMethod::Krylov
    } else {
        QuasiPeriodicLinearMethod::Direct
    };
    let (base_values, value_limit) = check_workload(unknowns, &grid, &linear, limits)?;
    let mut work = Workspace {
        transform: QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?,
        grid,
        config,
        unknowns,
        linear: Vec::new(),
        voltage_rows: (0..unknowns)
            .map(|row| circuit.voltage_equation(row))
            .collect(),
        base_values: base_values.saturating_add(size.saturating_mul(64)),
        value_limit,
    };
    work.budget(0)?;
    for &frequency in work.grid.frequencies_hz() {
        check_abort(abort)?;
        let entries = circuit.linear_entries(frequency)?;
        if entries
            .iter()
            .any(|&(r, c, v)| r >= unknowns || c >= unknowns || !finite(v))
        {
            return Err(Error::InvalidCircuit(
                "invalid autonomous linear network".into(),
            ));
        }
        work.base_values = work
            .base_values
            .saturating_add(entries.len().saturating_mul(4));
        work.budget(0)?;
        work.linear.push(entries);
    }
    Ok(work)
}

#[expect(
    clippy::too_many_arguments,
    reason = "the autonomous solve keeps its numerical inputs explicit"
)]
pub(crate) fn solve_autonomous_with_abort(
    circuit: &mut impl Circuit,
    grid: Arc<QuasiPeriodicGrid>,
    config: &QuasiPeriodicSolveConfig,
    oscillator: &QuasiPeriodicAutonomousConfig,
    sources: &[Vec<Complex64>],
    seed: &[Vec<Complex64>],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<QuasiPeriodicSolution, Error> {
    check_abort(abort)?;
    config.validate()?;
    let index = oscillator.phase_index(&grid, circuit.unknowns())?;
    coordinates::validate(sources, circuit.unknowns(), &grid, "source", abort)?;
    coordinates::validate(seed, circuit.unknowns(), &grid, "oscillator seed", abort)?;
    if sources.iter().any(|row| {
        row.iter()
            .zip(grid.indices())
            .any(|(value, tuple)| tuple[oscillator.tone] != 0 && *value != Complex64::ZERO)
    }) {
        return Err(Error::InvalidConfig(
            "the autonomous tone cannot be independently driven".into(),
        ));
    }
    if 2.0 * seed[oscillator.phase_coordinate][index].norm() < oscillator.minimum_amplitude {
        return Err(Error::InvalidConfig(
            "oscillator seed has insufficient phase-reference amplitude".into(),
        ));
    }
    let phase = phase_column(oscillator, &grid, index);
    let mut spectra = seed.to_vec();
    let mut work = workspace(circuit, grid, config, limits, abort)?;
    for iterations in 0..=config.max_iterations {
        check_abort(abort)?;
        let evaluation = work.evaluate(circuit, &spectra, sources, true, abort)?;
        let phase_residual = spectra[oscillator.phase_coordinate][index].im;
        let phase_scale = config.voltage_absolute_tolerance
            + config.relative_tolerance * spectra[oscillator.phase_coordinate][index].norm();
        let merit = evaluation.merit.max(phase_residual.abs() / phase_scale);
        let frequency =
            work.frequency_column(circuit, &evaluation, &spectra, oscillator.tone, abort)?;
        let correction = work.bordered_correction(
            &evaluation,
            &frequency,
            phase,
            phase_residual,
            merit <= 1.0,
            abort,
        )?;
        if merit <= 1.0 {
            if 2.0 * spectra[oscillator.phase_coordinate][index].norm()
                < oscillator.minimum_amplitude
            {
                return Err(Error::InvalidCircuit(
                    "autonomous QPSS converged to a non-oscillating state".into(),
                ));
            }
            return Ok(QuasiPeriodicSolution {
                grid: work.grid,
                spectra,
                iterations,
                normalized_residual: merit,
            });
        }
        if iterations == config.max_iterations {
            return Err(Error::ConvergenceFailed { iterations, merit });
        }
        let state = coordinates::encode(&spectra);
        let df = correction[state.len()];
        let mut damping = if df == 0.0 {
            1.0
        } else {
            ((1.0 + oscillator.max_relative_frequency_step).ln() / df.abs()).min(1.0)
        };
        let mut accepted = None;
        for _ in 0..=config.max_backtracks {
            check_abort(abort)?;
            let candidate = coordinates::decode(
                &state
                    .iter()
                    .zip(&correction)
                    .map(|(x, dx)| x + damping * dx)
                    .collect::<Vec<_>>(),
                work.grid.len(),
            );
            let mut basis = work.grid.config().clone();
            basis.frequencies_hz[oscillator.tone] *= (damping * df).exp();
            if candidate.iter().flatten().all(|v| finite(*v))
                && basis.frequencies_hz[oscillator.tone].is_finite()
            {
                let grid = Arc::new(QuasiPeriodicGrid::new_with_abort(basis, limits, abort)?);
                let mut trial = workspace(circuit, grid, config, limits, abort)?;
                match trial.evaluate(circuit, &candidate, sources, false, abort) {
                    Ok(value) => {
                        let phase_merit = candidate[oscillator.phase_coordinate][index].im.abs()
                            / (config.voltage_absolute_tolerance
                                + config.relative_tolerance
                                    * candidate[oscillator.phase_coordinate][index].norm());
                        let next = value.merit.max(phase_merit);
                        if next <= 1.0 || next < merit {
                            accepted = Some((candidate, trial));
                            break;
                        }
                    }
                    Err(Error::Numerical(_)) => {}
                    Err(error) => return Err(error),
                }
            }
            damping *= 0.5;
        }
        (spectra, work) = accepted.ok_or(Error::ConvergenceFailed { iterations, merit })?;
    }
    unreachable!("bounded autonomous iteration exits with a certificate or error")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NoAbort;
    use crate::analysis::quasi_periodic::{QuasiPeriodicGridConfig, QuasiPeriodicSampling};
    use std::f64::consts::{SQRT_2, TAU};

    struct ModulatedOscillator {
        linear_charge: bool,
        floating: bool,
    }
    impl Circuit for ModulatedOscillator {
        fn unknowns(&self) -> usize {
            if self.floating { 3 } else { 2 }
        }
        fn voltage_equation(&self, _: usize) -> bool {
            false
        }
        fn linear_entries(&self, frequency: Value) -> Result<Vec<LinearEntry>, Error> {
            Ok(if self.linear_charge {
                vec![
                    (0, 0, Complex64::new(0.0, TAU * frequency)),
                    (1, 1, Complex64::new(0.0, TAU * frequency)),
                ]
            } else {
                vec![]
            })
        }
        fn linear_frequency_derivative(&self, _: Value) -> Result<Vec<LinearEntry>, Error> {
            Ok(if self.linear_charge {
                vec![
                    (0, 0, Complex64::new(0.0, TAU)),
                    (1, 1, Complex64::new(0.0, TAU)),
                ]
            } else {
                vec![]
            })
        }
        fn sample(&mut self, state: &[Value], jacobian: bool) -> Result<Sample, Error> {
            self.sample_at_phases(state, &[0.0, 0.0], jacobian)
        }
        fn sample_at_phases(
            &mut self,
            state: &[Value],
            phases: &[Value],
            jacobian: bool,
        ) -> Result<Sample, Error> {
            let (x, y) = (state[0], state[1]);
            let radial = 1.0 - x * x - y * y;
            let omega = 1.0 + 0.1 * SQRT_2 * phases[1].cos();
            Ok(Sample {
                current: vec![(0, radial * x - omega * y), (1, radial * y + omega * x)],
                charge: if self.linear_charge {
                    vec![]
                } else {
                    vec![(0, -x), (1, -y)]
                },
                conductance: if jacobian {
                    vec![
                        (0, 0, -radial + 2.0 * x * x),
                        (0, 1, 2.0 * x * y + omega),
                        (1, 0, 2.0 * x * y - omega),
                        (1, 1, -radial + 2.0 * y * y),
                    ]
                } else {
                    vec![]
                },
                capacitance: if jacobian && !self.linear_charge {
                    vec![(0, 0, 1.0), (1, 1, 1.0)]
                } else {
                    vec![]
                },
            })
        }
    }

    #[test]
    fn autonomous_qpss_solves_unknown_frequency_and_modulated_torus() {
        for method in [
            QuasiPeriodicLinearMethod::Direct,
            QuasiPeriodicLinearMethod::Krylov,
        ] {
            let mut basis = QuasiPeriodicGridConfig::new(vec![1.2 / TAU, SQRT_2 / TAU], vec![2, 3]);
            basis.sampling = QuasiPeriodicSampling::Exact(vec![13, 25]);
            let limits = ResourceLimits::default();
            let grid =
                Arc::new(QuasiPeriodicGrid::new_with_abort(basis, &limits, &NoAbort).unwrap());
            let oscillator = QuasiPeriodicAutonomousConfig {
                tone: 0,
                phase_coordinate: 0,
                phase_tuple: vec![1, 0],
                minimum_amplitude: 0.01,
                max_relative_frequency_step: 0.2,
            };
            let mut seed = vec![vec![Complex64::ZERO; grid.len()]; 2];
            let k = grid.index_of(&[1, 0]).unwrap();
            seed[0][k] = Complex64::from_polar(0.4, 0.2);
            seed[1][k] = Complex64::new(0.0, -1.0) * seed[0][k];
            for row in &mut seed {
                row[grid.len() - 1 - k] = row[k].conj();
            }
            let sources = vec![vec![Complex64::ZERO; grid.len()]; 2];
            let mut config = QuasiPeriodicSolveConfig {
                relative_tolerance: 1e-8,
                ..Default::default()
            };
            config.linear.method = method;
            config.linear.restart = 64;
            config.linear.max_cycles = 40;
            let mut circuit = ModulatedOscillator {
                linear_charge: method == QuasiPeriodicLinearMethod::Krylov,
                floating: false,
            };
            let solution = solve_autonomous_with_abort(
                &mut circuit,
                grid.clone(),
                &config,
                &oscillator,
                &sources,
                &seed,
                &limits,
                &NoAbort,
            )
            .unwrap();
            assert!((solution.grid().config().frequencies_hz[0] - 1.0 / TAU).abs() < 1e-8);
            assert_eq!(solution.grid().config().frequencies_hz[1], SQRT_2 / TAU);
            assert!(solution.normalized_residual() <= 1.0);
            let mut transform =
                QuasiPeriodicTransform::new_with_abort(solution.grid().clone(), &NoAbort).unwrap();
            let x = transform
                .to_real_samples_with_abort(&solution.spectra()[0], &NoAbort)
                .unwrap();
            let y = transform
                .to_real_samples_with_abort(&solution.spectra()[1], &NoAbort)
                .unwrap();
            for point in 0..grid.sample_count() {
                let phases = grid.phases(point).unwrap();
                let angle = phases[0] + 0.1 * phases[1].sin();
                assert!(
                    (x[point] - angle.cos()).abs() < 1e-5,
                    "{method:?} x at {point}"
                );
                assert!(
                    (y[point] - angle.sin()).abs() < 1e-5,
                    "{method:?} y at {point}"
                );
            }
            // Neither a zero seed nor a driven free tone is an oscillator.
            assert!(
                solve_autonomous_with_abort(
                    &mut circuit,
                    grid.clone(),
                    &config,
                    &oscillator,
                    &sources,
                    &sources,
                    &limits,
                    &NoAbort
                )
                .is_err()
            );
            let mut driven = sources.clone();
            driven[0][k] = Complex64::new(0.1, 0.0);
            driven[0][grid.len() - 1 - k] = driven[0][k];
            assert!(
                solve_autonomous_with_abort(
                    &mut circuit,
                    grid.clone(),
                    &config,
                    &oscillator,
                    &driven,
                    &seed,
                    &limits,
                    &NoAbort
                )
                .is_err()
            );
            circuit.floating = true;
            let mut floating_seed = solution.spectra().to_vec();
            floating_seed.push(vec![Complex64::ZERO; grid.len()]);
            let mut floating_sources = sources.clone();
            floating_sources.push(vec![Complex64::ZERO; grid.len()]);
            assert!(
                solve_autonomous_with_abort(
                    &mut circuit,
                    solution.grid().clone(),
                    &config,
                    &oscillator,
                    &floating_sources,
                    &floating_seed,
                    &limits,
                    &NoAbort
                )
                .is_err()
            );
        }
    }
}
