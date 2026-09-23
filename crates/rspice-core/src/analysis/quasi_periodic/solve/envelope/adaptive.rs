//! One accepted adaptive advance, with all trial history kept private.
use super::*;

#[derive(Debug, Clone)]
pub struct SpectralEnvelopeControl {
    pub method: SpectralEnvelopeMethod,
    pub minimum_step: Value,
    pub maximum_step: Value,
    pub relative_tolerance: Value,
    /// Absolute state tolerance per MNA coordinate, in that coordinate's
    /// physical units. Equation units do not identify state units.
    pub absolute_tolerances: Vec<Value>,
    pub max_rejections: usize,
}

impl SpectralEnvelopeControl {
    pub(crate) fn validate(&self, unknowns: usize) -> Result<(), Error> {
        if !self.minimum_step.is_finite()
            || self.minimum_step <= 0.0
            || !self.maximum_step.is_finite()
            || self.maximum_step < self.minimum_step
            || !self.relative_tolerance.is_finite()
            || self.relative_tolerance < 0.0
            || self.relative_tolerance >= 1.0
            || self.absolute_tolerances.len() != unknowns
            || self
                .absolute_tolerances
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.0)
        {
            return Err(Error::InvalidConfig("adaptive Envelope requires positive finite ordered step bounds, relative tolerance in [0,1), and a positive absolute tolerance for every MNA coordinate".into()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SpectralEnvelopeAdvance {
    pub state: SpectralEnvelopeState,
    pub suggested_step: Value,
    pub error_ratio: Value,
    pub rejected_steps: usize,
    pub spectral_solves: usize,
}

fn retryable(error: &Error) -> bool {
    use crate::solver::SolverError;
    matches!(
        error,
        Error::ConvergenceFailed { .. }
            | Error::LinearSolve(
                SolverError::SingularMatrix
                    | SolverError::ConvergenceFailed(_)
                    | SolverError::PivotGrowth
                    | SolverError::InaccurateSolution(_)
            )
    )
}

fn compare(
    previous: &SpectralEnvelopeState,
    coarse: &SpectralEnvelopeState,
    refined: &SpectralEnvelopeState,
    control: &SpectralEnvelopeControl,
    factor: Value,
    abort: &dyn AbortSignal,
) -> Result<Value, Error> {
    let mut transform = QuasiPeriodicTransform::new_with_abort(previous.grid().clone(), abort)?;
    let mut largest = 0.0_f64;
    for (row, &absolute) in control.absolute_tolerances.iter().enumerate() {
        check_abort(abort)?;
        let old = transform.to_real_samples_with_abort(&previous.spectra()[row], abort)?;
        let a = transform.to_real_samples_with_abort(&coarse.spectra()[row], abort)?;
        let b = transform.to_real_samples_with_abort(&refined.spectra()[row], abort)?;
        for (index, ((a, b), old)) in a.into_iter().zip(b).zip(old).enumerate() {
            if index.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let magnitude = a.abs().max(b.abs()).max(old.abs());
            let scale = magnitude.max(absolute);
            let difference = factor * (a / scale - b / scale).abs();
            let allowance = absolute / scale + control.relative_tolerance * (magnitude / scale);
            let ratio = if difference == 0.0 {
                0.0
            } else {
                difference / allowance
            };
            largest = largest.max(ratio);
        }
    }
    Ok(largest)
}

/// Choose an interval from the requested step and configured bounds, stopping
/// no later than the exact next deadline. The caller supplies event-sided physical circuit/source data in
/// `solve`, and restarts history after a discontinuity. This routine does not
/// infer source events or project algebraic jumps.
///
/// `solve` may be called at nonmonotonic trial times. It must reconstruct the
/// circuit for each request, use the supplied limits/abort signal, and publish
/// no results until this function returns success. The accepted interval can
/// be smaller than `minimum_step` only to land on a closer deadline. Embedded
/// error-estimation probes can be half an interval; they are never accepted.
///
/// BDF2 uses its embedded backward-Euler difference as a conservative local
/// indicator. BE startup/restarts use two half steps and twice their difference
/// from the full step, estimating the error of the accepted full BE step.
#[expect(
    clippy::too_many_arguments,
    reason = "state, scheduling, tolerance and resource inputs are independent"
)]
pub fn advance_spectral_envelope_with_abort<F>(
    previous: &SpectralEnvelopeState,
    requested_step: Value,
    deadline: Value,
    control: &SpectralEnvelopeControl,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
    mut solve: F,
) -> Result<SpectralEnvelopeAdvance, Error>
where
    F: FnMut(
        &SpectralEnvelopeState,
        Value,
        SpectralEnvelopeMethod,
        &ResourceLimits,
        &dyn AbortSignal,
    ) -> Result<SpectralEnvelopeState, Error>,
{
    check_abort(abort)?;
    control.validate(previous.spectra().len())?;
    if !requested_step.is_finite()
        || requested_step <= 0.0
        || !deadline.is_finite()
        || deadline <= previous.time()
    {
        return Err(Error::InvalidConfig(
            "adaptive Envelope needs a finite positive requested step and a future deadline".into(),
        ));
    }
    let retained = previous.spectra().len().saturating_mul(
        previous
            .grid()
            .len()
            .saturating_mul(24)
            .saturating_add(previous.grid().sample_count().saturating_mul(6)),
    );
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        retained,
        limits.max_result_values,
    )?;
    let mut bounded = *limits;
    bounded.max_result_values -= retained;
    let mut solves = 0usize;
    let mut trial = |state: &SpectralEnvelopeState, time: Value, method| {
        check_abort(abort)?;
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            solves.saturating_add(1),
            limits.max_analysis_points,
        )?;
        solves += 1;
        let next = solve(state, time, method, &bounded, abort)?;
        check_abort(abort)?;
        if next.time().to_bits() != time.to_bits()
            || !Arc::ptr_eq(next.grid(), previous.grid())
            || next.voltage_rows != previous.voltage_rows
            || !matches!(next.order(), 1 | 2)
        {
            return Err(Error::InvalidCircuit(
                "Envelope trial changed its time or coordinate basis".into(),
            ));
        }
        Ok(next)
    };
    let remaining = deadline - previous.time();
    let floor = control.minimum_step.min(remaining);
    let mut step = requested_step
        .clamp(control.minimum_step, control.maximum_step)
        .min(remaining);
    let mut rejected = 0usize;
    loop {
        check_abort(abort)?;
        let time = if step >= remaining {
            deadline
        } else {
            (previous.time() + step).min(deadline)
        };
        if time <= previous.time() {
            return Err(Error::InvalidConfig(
                "Envelope step does not advance representable time".into(),
            ));
        }
        let actual_step = time - previous.time();
        let attempt = (|| {
            let coarse = trial(previous, time, SpectralEnvelopeMethod::BackwardEuler)?;
            let bdf2 = control.method == SpectralEnvelopeMethod::Bdf2
                && previous
                    .older
                    .as_ref()
                    .is_some_and(|(older, _)| actual_step / (previous.time() - older) <= 2.0);
            if bdf2 {
                let refined = trial(previous, time, SpectralEnvelopeMethod::Bdf2)?;
                let error = compare(previous, &coarse, &refined, control, 1.0, abort)?;
                Ok((refined, error))
            } else {
                let middle = previous.time() + actual_step * 0.5;
                if middle <= previous.time() || middle >= time {
                    return Err(Error::InvalidConfig(
                        "Envelope interval has no representable midpoint for error estimation"
                            .into(),
                    ));
                }
                let half = trial(previous, middle, SpectralEnvelopeMethod::BackwardEuler)?;
                let refined = trial(&half, time, SpectralEnvelopeMethod::BackwardEuler)?;
                let error = compare(previous, &coarse, &refined, control, 2.0, abort)?;
                Ok((coarse, error))
            }
        })();
        let (failure, shrink) = match attempt {
            Ok((state, error)) if error <= 1.0 => {
                let growth = if error == 0.0 {
                    2.0
                } else {
                    (0.9 / error.sqrt()).clamp(0.5, 2.0)
                };
                let suggested_step =
                    (actual_step * growth).clamp(control.minimum_step, control.maximum_step);
                return Ok(SpectralEnvelopeAdvance {
                    state,
                    suggested_step,
                    error_ratio: error,
                    rejected_steps: rejected,
                    spectral_solves: solves,
                });
            }
            Ok((_, error)) => (
                Error::Numerical(format!(
                    "Envelope local error ratio {error:e} exceeds one at step {actual_step:e}"
                )),
                (0.9 / error.sqrt()).clamp(0.1, 0.8),
            ),
            Err(error) if retryable(&error) => (error, 0.5),
            Err(error) => return Err(error),
        };
        if rejected >= control.max_rejections || step <= floor {
            return Err(failure);
        }
        rejected += 1;
        let smaller = (step * shrink).max(floor);
        if smaller >= step {
            return Err(failure);
        }
        step = smaller;
    }
}

#[cfg(test)]
mod tests;
