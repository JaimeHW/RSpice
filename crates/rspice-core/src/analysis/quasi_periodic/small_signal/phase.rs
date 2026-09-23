//! Autonomous response with an explicit phase integrator.
//!
//! For t = dx/dtheta_free and r = A(0)t, qualify r, then preserve the neutral
//! mode of A(f) - r p, where p t = 1. Solve [A(f) q; p 0] [z; beta] = [b; 0]
//! with q = (A(f)-A(0))t/(j 2 pi f). The physical response is z+t beta/(j 2 pi f).
//! Keeping the divided difference analytic avoids losing phase diffusion when
//! the offset is smaller than one ulp of a carrier. Adjoint RHS transformation
//! gives exactly the same transfer for arbitrary complex sources/observations.
use super::*;
use crate::numerics::krylov::{GmresError, try_gmres_with_abort};
use crate::solver::SolverError;

#[cfg(test)]
mod tests;

pub(super) struct PhaseTangent {
    tangent: Vec<Complex64>,
    dual: Vec<Complex64>,
    charge: Vec<Complex64>,
}

impl Linearization {
    #[expect(
        clippy::too_many_arguments,
        reason = "independent orbit, phase, resource and cancellation inputs"
    )]
    pub(crate) fn prepare_autonomous(
        &mut self,
        circuit: &impl Circuit,
        orbit: &[Vec<Complex64>],
        tone: usize,
        tolerance: Value,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), Error> {
        check_abort(abort)?;
        if tone >= self.grid.dimensions().len()
            || !tolerance.is_finite()
            || tolerance <= 0.0
            || tolerance >= 1.0
            || orbit.len() != self.unknowns
            || orbit.iter().any(|row| row.len() != self.grid.len())
        {
            return Err(Error::InvalidConfig(
                "invalid autonomous response phase coordinates or tolerance".into(),
            ));
        }
        let (old, _) = super::super::solve::check_workload(
            self.unknowns,
            &self.grid,
            &self.config.linear,
            limits,
        )?;
        let (bordered, _) = super::super::solve::check_bordered_workload(
            self.unknowns,
            &self.grid,
            &self.config.linear,
            limits,
            1,
        )?;
        let size = self.unknowns * self.grid.len();
        let budget = self
            .base_values
            .saturating_add(bordered.saturating_sub(old))
            .saturating_add(size.saturating_add(1).saturating_mul(64));
        ResourceLimitError::ensure(ResourceKind::ResultValues, budget, self.value_limit)?;
        let mut tangent: Vec<_> = orbit
            .iter()
            .flat_map(|row| {
                row.iter()
                    .zip(self.grid.indices())
                    .map(|(x, tuple)| Complex64::new(0.0, tuple[tone] as Value) * x)
            })
            .collect();
        let scale = tangent.iter().map(|v| v.norm()).fold(0.0_f64, Value::max);
        if !scale.is_finite() || scale == 0.0 {
            return Err(Error::InvalidConfig(
                "autonomous response requires a nonzero finite phase tangent".into(),
            ));
        }
        tangent.iter_mut().for_each(|v| *v /= scale);
        let norm: Value = tangent.iter().map(|v| v.norm_sqr()).sum();
        let dual = tangent.iter().map(|v| v.conj() / norm).collect();
        // The sum of retained coefficient magnitudes bounds the waveform of
        // each coordinate, including zeros caused by symmetry in a single bin.
        let scales: Vec<Value> = tangent
            .chunks_exact(self.grid.len())
            .map(|row| row.iter().map(|v| v.norm()).sum())
            .collect();
        let frequencies = self.grid.frequencies_hz().to_vec();
        let mut retained = budget;
        let linear = frequencies
            .iter()
            .map(|&f| {
                check_abort(abort)?;
                let entries = circuit.small_signal_entries(f)?;
                retained = retained.saturating_add(entries.len().saturating_mul(4));
                ResourceLimitError::ensure(ResourceKind::ResultValues, retained, self.value_limit)?;
                if entries
                    .iter()
                    .any(|&(r, c, v)| r >= self.unknowns || c >= self.unknowns || !finite(v))
                {
                    return Err(Error::InvalidCircuit(
                        "invalid autonomous response linear entry".into(),
                    ));
                }
                Ok(entries)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let orientation = self.orientation;
        self.orientation = Orientation::Forward;
        let defect = self.apply(&frequencies, &linear, &tangent, abort);
        self.orientation = orientation;
        let defect = defect?;
        let mut g = vec![0.0_f64; self.unknowns];
        let mut c = g.clone();
        for sample in &self.derivatives {
            check_abort(abort)?;
            for (terms, bound) in [(&sample.conductance, &mut g), (&sample.capacitance, &mut c)] {
                let mut sums = vec![0.0; self.unknowns];
                for &(row, col, v) in terms {
                    sums[row] += v.abs() * scales[col];
                }
                for (maximum, sum) in bound.iter_mut().zip(sums) {
                    *maximum = maximum.max(sum);
                }
            }
        }
        for (terms, bound) in [
            (&self.stationary.conductance, &mut g),
            (&self.stationary.capacitance, &mut c),
        ] {
            for &(row, col, v) in terms {
                bound[row] += v.abs() * scales[col];
            }
        }
        for (k, &f) in frequencies.iter().enumerate() {
            let mut bounds: Vec<_> = g
                .iter()
                .zip(&c)
                .map(|(g, c)| g + std::f64::consts::TAU * f.abs() * c)
                .collect();
            for &(row, col, v) in &linear[k] {
                bounds[row] += v.norm() * scales[col];
            }
            for (row, bound) in bounds.into_iter().enumerate() {
                if !bound.is_finite()
                    || defect[row * self.grid.len() + k].norm()
                        > (tolerance + 1024.0 * Value::EPSILON) * bound
                {
                    return Err(Error::Numerical("autonomous QP phase tangent is unresolved; refine the QPSS tolerance, harmonics or sampling".into()));
                }
            }
        }
        let charge = self.phase_charge(&tangent, abort)?;
        if charge.iter().any(|v| !finite(*v)) {
            return Err(Error::Numerical(
                "autonomous phase charge overflowed".into(),
            ));
        }
        self.phase = Some(PhaseTangent {
            tangent,
            dual,
            charge,
        });
        self.base_values = budget;
        Ok(())
    }

    fn phase_charge(
        &mut self,
        tangent: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        let waves = tangent
            .chunks_exact(self.grid.len())
            .map(|row| self.transform.to_samples_with_abort(row, abort))
            .collect::<Result<Vec<_>, _>>()?;
        let mut charge = vec![vec![Complex64::ZERO; self.grid.sample_count()]; self.unknowns];
        for (time, sample) in self.derivatives.iter().enumerate() {
            check_abort(abort)?;
            for &(row, col, v) in &sample.capacitance {
                charge[row][time] += v * waves[col][time];
            }
        }
        let mut result = Vec::with_capacity(tangent.len());
        for row in charge {
            result.extend(self.transform.to_spectrum_with_abort(&row, abort)?);
        }
        for &(row, col, v) in &self.stationary.capacitance {
            for k in 0..self.grid.len() {
                result[row * self.grid.len() + k] += v * tangent[col * self.grid.len() + k];
            }
        }
        Ok(result)
    }

    fn phase_action(
        &mut self,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        rate: &[Complex64],
        direction: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        let n = rate.len();
        let mut applied = self.apply(frequencies, linear, &direction[..n], abort)?;
        let phase = self.phase.as_ref().expect("prepared phase tangent");
        let mut constraint = Complex64::ZERO;
        for (i, a) in applied.iter_mut().enumerate() {
            let (column, row) = if self.orientation == Orientation::Forward {
                (rate[i], phase.dual[i])
            } else {
                (phase.dual[i].conj(), rate[i].conj())
            };
            *a += column * direction[n];
            constraint += row * direction[i];
        }
        applied.push(constraint);
        if applied.iter().any(|v| !finite(*v)) {
            return Err(Error::Numerical(
                "autonomous response border overflowed".into(),
            ));
        }
        Ok(applied)
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "independent spectral operator and source inputs"
    )]
    pub(super) fn solve_phase(
        &mut self,
        circuit: &impl Circuit,
        offset: Value,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        rhs: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicAcSolution, Error> {
        let jw = Complex64::new(0.0, std::f64::consts::TAU * offset);
        if offset == 0.0 || !finite(jw) || jw == Complex64::ZERO {
            return Err(Error::InvalidConfig("autonomous QP response requires a nonzero representable offset; absolute oscillator phase has no finite DC response".into()));
        }
        let phase = self.phase.as_ref().expect("prepared phase tangent");
        let mut rate = phase.charge.clone();
        let n = rate.len();
        for (k, &f) in self.grid.frequencies_hz().iter().enumerate() {
            check_abort(abort)?;
            if (0..self.unknowns)
                .all(|row| phase.tangent[row * self.grid.len() + k] == Complex64::ZERO)
            {
                continue;
            }
            for (row, col, v) in circuit.small_signal_frequency_difference(f, offset)? {
                if row >= self.unknowns || col >= self.unknowns || !finite(v) {
                    return Err(Error::InvalidCircuit(
                        "invalid autonomous response frequency difference".into(),
                    ));
                }
                rate[row * self.grid.len() + k] += v * phase.tangent[col * self.grid.len() + k];
            }
        }
        let mut rhs = rhs.to_vec();
        rhs.push(if self.orientation == Orientation::Adjoint {
            phase
                .tangent
                .iter()
                .zip(&rhs)
                .map(|(t, c)| t.conj() * c)
                .sum::<Complex64>()
                / jw.conj()
        } else {
            Complex64::ZERO
        });
        if rhs.iter().chain(&rate).any(|v| !finite(*v)) {
            return Err(Error::Numerical(
                "autonomous phase response is not representable".into(),
            ));
        }
        let mut divisors = self.equation_divisors(frequencies, linear, abort)?;
        let border_column: Vec<_> = if self.orientation == Orientation::Forward {
            rate.clone()
        } else {
            phase.dual.iter().map(|v| v.conj()).collect()
        };
        for (row, values) in border_column.chunks_exact(self.grid.len()).enumerate() {
            divisors[row] =
                divisors[row].max(values.iter().map(|v| v.norm()).fold(0.0_f64, Value::max));
        }
        let mut scales: Vec<_> = (0..n).map(|i| divisors[i / self.grid.len()]).collect();
        scales.push(if self.orientation == Orientation::Forward {
            1.0
        } else {
            rate.iter().map(|v| v.norm()).fold(1.0_f64, Value::max)
        });
        let config = self.config.clone();
        let solution = if config.linear.uses_krylov(n + 1) {
            let mut blocks = linear::Blocks::build(self, frequencies, linear, abort)?;
            let mut solve = |rhs: &[Complex64]| -> Result<Vec<Complex64>, Error> {
                let outcome = try_gmres_with_abort(
                    &mut |v| {
                        let mut a = self.phase_action(frequencies, linear, &rate, v, abort)?;
                        for (a, s) in a.iter_mut().zip(&scales) {
                            *a /= s;
                        }
                        Ok(a)
                    },
                    &mut |v| {
                        let mut p = blocks.apply(&v[..n], &divisors, abort)?;
                        p.push(v[n] * scales[n]);
                        Ok(p)
                    },
                    rhs,
                    config.linear.restart,
                    config.linear.max_cycles,
                    config.linear.relative_tolerance,
                    &|| abort.is_aborted(),
                )
                .map_err(|e| match e {
                    GmresError::Aborted => Error::Aborted,
                    GmresError::Operator(e) => e,
                    GmresError::InvalidData(s) => Error::Numerical(s.into()),
                })?;
                // Final acceptance and inverse qualification use the actual
                // bordered residual, including homogeneous constraints.
                if outcome.solution.iter().any(|v| !finite(*v)) {
                    return Err(Error::Numerical(
                        "nonfinite autonomous Krylov response".into(),
                    ));
                }
                Ok(outcome.solution)
            };
            // A compatible forcing cannot certify that the only free mode is
            // phase. Stream inverse columns; never retain a dense inverse.
            let scaled_rhs: Vec<_> = rhs.iter().zip(&scales).map(|(v, s)| v / s).collect();
            let candidate = solve(&scaled_rhs)?;
            self.certify_phase_inverse(
                frequencies,
                linear,
                &rate,
                &scales,
                &divisors,
                &mut blocks,
                abort,
            )?;
            candidate
        } else {
            let mut entries = Vec::new();
            let mut unit = vec![Complex64::ZERO; n + 1];
            for column in 0..=n {
                unit[column] = Complex64::ONE;
                for (row, v) in self
                    .phase_action(frequencies, linear, &rate, &unit, abort)?
                    .into_iter()
                    .enumerate()
                {
                    if v != Complex64::ZERO {
                        entries.push((row, column, v / scales[row]));
                    }
                }
                unit[column] = Complex64::ZERO;
            }
            let mut matrix = linear::matrix(n + 1, &entries)?;
            let scaled: Vec<_> = rhs.iter().zip(&scales).map(|(v, s)| v / s).collect();
            match matrix.solve(&scaled) {
                Ok(solution) => solution,
                Err(SolverError::InaccurateSolution(_)) => {
                    ResourceLimitError::ensure(
                        ResourceKind::ResultValues,
                        self.base_values
                            .saturating_add((n + 1).saturating_mul(n + 1).saturating_mul(24)),
                        self.value_limit,
                    )?;
                    matrix.solve_dense_extended(&scaled)?
                }
                Err(e) => return Err(e.into()),
            }
        };
        let applied = self.phase_action(frequencies, linear, &rate, &solution, abort)?;
        let norm = |v: &[Complex64]| {
            v.iter()
                .zip(&scales)
                .map(|(v, s)| v.norm() / s)
                .fold(0.0_f64, Value::max)
        };
        let adjoint_scale = config.linear.relative_tolerance * norm(&rhs).max(norm(&applied));
        let mut merit = 0.0_f64;
        for (i, (a, b)) in applied.iter().zip(&rhs).enumerate() {
            let scale = if self.orientation == Orientation::Adjoint {
                adjoint_scale * scales[i]
            } else if i == n {
                config.linear.relative_tolerance
                    * solution.iter().map(|v| v.norm()).fold(1.0_f64, Value::max)
            } else {
                (if self.voltage_rows[i / self.grid.len()] {
                    config.voltage_absolute_tolerance
                } else {
                    config.current_absolute_tolerance
                }) + config.linear.relative_tolerance * a.norm().max(b.norm())
            };
            let residual = (*a - *b).norm() / scale;
            if !residual.is_finite() || !scale.is_finite() {
                return Err(Error::Numerical(
                    "autonomous response certificate overflowed".into(),
                ));
            }
            merit = merit.max(residual);
        }
        if merit > 1.0 {
            return Err(Error::Numerical(format!(
                "autonomous QP bordered residual exceeds tolerance ({merit:e})"
            )));
        }
        let mut response = solution[..n].to_vec();
        if self.orientation == Orientation::Forward {
            for (x, t) in response
                .iter_mut()
                .zip(&self.phase.as_ref().unwrap().tangent)
            {
                *x += t * (solution[n] / jw);
            }
        }
        if response.iter().any(|v| !finite(*v)) {
            return Err(Error::Numerical("autonomous response overflowed".into()));
        }
        Ok(QuasiPeriodicAcSolution {
            offset_hz: offset,
            spectra: response
                .chunks_exact(self.grid.len())
                .map(<[_]>::to_vec)
                .collect(),
            normalized_residual: merit,
        })
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "independent bordered operator, scaling and preconditioner inputs"
    )]
    fn certify_phase_inverse(
        &mut self,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        rate: &[Complex64],
        scales: &[Value],
        divisors: &[Value],
        blocks: &mut linear::Blocks,
        abort: &dyn AbortSignal,
    ) -> Result<(), Error> {
        let n = rate.len();
        let config = self.config.linear.clone();
        let mut bounds = vec![0.0; n + 1];
        let mut unit = vec![Complex64::ZERO; n + 1];
        for column in 0..=n {
            check_abort(abort)?;
            unit[column] = Complex64::ONE;
            let outcome = try_gmres_with_abort(
                &mut |v| {
                    let mut a = self.phase_action(frequencies, linear, rate, v, abort)?;
                    for (a, s) in a.iter_mut().zip(scales) {
                        *a /= s;
                    }
                    Ok(a)
                },
                &mut |v| {
                    let mut p = blocks.apply(&v[..n], divisors, abort)?;
                    p.push(v[n] * scales[n]);
                    Ok(p)
                },
                &unit,
                config.restart,
                config.max_cycles,
                config.relative_tolerance,
                &|| abort.is_aborted(),
            )
            .map_err(|e| match e {
                GmresError::Aborted => Error::Aborted,
                GmresError::Operator(e) => e,
                GmresError::InvalidData(s) => Error::Numerical(s.into()),
            })?;
            let applied = self.phase_action(frequencies, linear, rate, &outcome.solution, abort)?;
            for (row, ((bound, a), s)) in bounds.iter_mut().zip(applied).zip(scales).enumerate() {
                let a = a / s;
                *bound += (a - unit[row]).norm() + 1024.0 * Value::EPSILON * (1.0 + a.norm());
                if !bound.is_finite() || *bound >= 0.5 {
                    return Err(Error::Numerical(
                        "autonomous QP response could not certify a nonsingular phase border"
                            .into(),
                    ));
                }
            }
            unit[column] = Complex64::ZERO;
        }
        Ok(())
    }
}
