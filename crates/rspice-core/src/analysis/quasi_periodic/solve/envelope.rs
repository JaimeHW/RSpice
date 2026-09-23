//! Implicit slow-time evolution of the full Fourier F/Q equations.
//!
//! The inner periodic solve sees dQ/dt + omega*dQ/dphase + F = B. History
//! retains physical charge, including linear capacitor charge and inductor
//! flux, rather than recomputing it with the next step's circuit parameters.
use super::*;
use crate::analysis::quasi_periodic::{check_abort, finite};
mod adaptive;
pub(crate) mod event;
pub use event::{SpectralEnvelopeEvent, SpectralEnvelopeEventConfig, SpectralEnvelopeEventEquation};
pub use adaptive::{
    SpectralEnvelopeAdvance, SpectralEnvelopeControl, advance_spectral_envelope_with_abort,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectralEnvelopeMethod {
    BackwardEuler,
    /// Variable-step BDF2, starting/restarting with backward Euler when no
    /// older sample exists or the step grows by more than a factor of two.
    Bdf2,
}

/// Accepted numerical envelope history, not an authenticated engine artifact.
/// The caller owns the physical circuit and its consistent MNA coordinate order.
#[derive(Debug, Clone)]
pub struct SpectralEnvelopeState {
    time: Value,
    solution: QuasiPeriodicSolution,
    /// Delivered charge has the negative sign of the physical Q equations.
    charge: Vec<Vec<Complex64>>,
    older: Option<(Value, Vec<Vec<Complex64>>)>,
    voltage_rows: Vec<bool>,
    order: usize,
}

impl SpectralEnvelopeState {
    pub fn time(&self) -> Value {
        self.time
    }
    pub fn grid(&self) -> &Arc<QuasiPeriodicGrid> {
        self.solution.grid()
    }
    pub fn spectra(&self) -> &[Vec<Complex64>] {
        self.solution.spectra()
    }
    pub fn iterations(&self) -> usize {
        self.solution.iterations()
    }
    pub fn normalized_residual(&self) -> Value {
        self.solution.normalized_residual()
    }
    /// Zero at initialization, otherwise the order actually used for this step.
    pub fn order(&self) -> usize {
        self.order
    }
    /// Restart the multistep stencil after a source discontinuity. The
    /// accepted physical state and its charge/flux remain intact; this does
    /// not itself solve an algebraic jump or choose a source's event side.
    pub fn restart_integration_history(&mut self) {
        self.older = None;
    }
}

fn storage_valid(storage: &[JacobianEntry], unknowns: usize) -> Result<(), Error> {
    if storage
        .iter()
        .any(|&(row, col, value)| row >= unknowns || col >= unknowns || !value.is_finite())
    {
        return Err(Error::InvalidCircuit(
            "invalid envelope charge/flux stamp".into(),
        ));
    }
    Ok(())
}

fn workspace_limits(
    unknowns: usize,
    grid: &QuasiPeriodicGrid,
    storage: &[JacobianEntry],
    limits: &ResourceLimits,
) -> Result<ResourceLimits, Error> {
    // Charge capture, both immutable history levels, the returned history,
    // source overlay, and nonlinear sample buffers coexist with Newton work.
    let retained = unknowns
        .saturating_mul(
            grid.sample_count()
                .saturating_mul(4)
                .saturating_add(grid.len().saturating_mul(24)),
        )
        .saturating_add(storage.len().saturating_mul(3));
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        retained,
        limits.max_result_values,
    )?;
    let mut bounded = *limits;
    bounded.max_result_values -= retained;
    Ok(bounded)
}

fn charge_spectra(
    circuit: &mut impl Circuit,
    grid: Arc<QuasiPeriodicGrid>,
    spectra: &[Vec<Complex64>],
    storage: &[JacobianEntry],
    abort: &dyn AbortSignal,
) -> Result<Vec<Vec<Complex64>>, Error> {
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?;
    let waves = spectra
        .iter()
        .map(|row| transform.to_real_samples_with_abort(row, abort))
        .collect::<Result<Vec<_>, _>>()?;
    let mut charges = vec![vec![Complex64::ZERO; grid.sample_count()]; circuit.unknowns()];
    let mut state = vec![0.0; circuit.unknowns()];
    for time in 0..grid.sample_count() {
        check_abort(abort)?;
        for (value, row) in state.iter_mut().zip(&waves) {
            *value = row[time];
        }
        let sample =
            circuit.sample_at_phases(&state, &grid.phases(time).expect("bounded index"), false)?;
        for (row, charge) in sample.charge {
            let target = charges.get_mut(row).ok_or_else(|| {
                Error::InvalidCircuit("envelope charge has an invalid row".into())
            })?;
            target[time] += Complex64::new(charge, 0.0);
            if !finite(target[time]) {
                return Err(Error::Numerical(
                    "envelope charge accumulation is not finite".into(),
                ));
            }
        }
    }
    let mut result = charges
        .iter()
        .map(|row| transform.to_spectrum_with_abort(row, abort))
        .collect::<Result<Vec<_>, _>>()?;
    for &(row, col, value) in storage {
        check_abort(abort)?;
        for (charge, coordinate) in result[row].iter_mut().zip(&spectra[col]) {
            *charge -= value * coordinate;
            if !finite(*charge) {
                return Err(Error::Numerical(
                    "envelope charge/flux is not finite".into(),
                ));
            }
        }
    }
    // Q was sampled from real states. Project transform roundoff onto the
    // exact real Fourier subspace before retaining it as a future source.
    for row in &mut result {
        check_abort(abort)?;
        for k in 0..grid.len() / 2 {
            let opposite = grid.len() - 1 - k;
            let value = row[k] * 0.5 + row[opposite].conj() * 0.5;
            row[k] = value;
            row[opposite] = value.conj();
        }
        row[grid.dc_index()].im = 0.0;
    }
    Ok(result)
}

pub(crate) fn initialize(
    circuit: &mut impl Circuit,
    time: Value,
    solution: &QuasiPeriodicSolution,
    storage: &[JacobianEntry],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<SpectralEnvelopeState, Error> {
    check_abort(abort)?;
    if !time.is_finite() || time < 0.0 {
        return Err(Error::InvalidConfig(
            "envelope initial time must be finite and nonnegative".into(),
        ));
    }
    storage_valid(storage, circuit.unknowns())?;
    let grid = solution.grid();
    let bounded = workspace_limits(circuit.unknowns(), grid, storage, limits)?;
    check_workload(
        circuit.unknowns(),
        grid,
        &QuasiPeriodicLinearConfig::default(),
        &bounded,
    )?;
    validate_spectra(
        solution.spectra(),
        circuit.unknowns(),
        grid,
        "envelope initial state",
        abort,
    )?;
    let charge = charge_spectra(circuit, grid.clone(), solution.spectra(), storage, abort)?;
    check_abort(abort)?;
    Ok(SpectralEnvelopeState {
        time,
        solution: solution.clone(),
        charge,
        older: None,
        voltage_rows: (0..circuit.unknowns())
            .map(|row| circuit.voltage_equation(row))
            .collect(),
        order: 0,
    })
}

struct ImplicitCircuit<'a, C> {
    inner: &'a mut C,
    storage: &'a [JacobianEntry],
    alpha: Value,
}

impl<C: Circuit> Circuit for ImplicitCircuit<'_, C> {
    fn unknowns(&self) -> usize {
        self.inner.unknowns()
    }
    fn voltage_equation(&self, row: usize) -> bool {
        self.inner.voltage_equation(row)
    }
    fn linear_entries(&self, frequency: Value) -> Result<Vec<LinearEntry>, Error> {
        let mut entries = self.inner.linear_entries(frequency)?;
        entries.extend(
            self.storage
                .iter()
                .map(|&(row, col, value)| (row, col, Complex64::new(self.alpha * value, 0.0))),
        );
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
        let mut sample = self.inner.sample_at_phases(state, phases, jacobian)?;
        sample.current.extend(
            sample
                .charge
                .iter()
                .map(|&(row, value)| (row, self.alpha * value)),
        );
        sample.conductance.extend(
            sample
                .capacitance
                .iter()
                .map(|&(row, col, value)| (row, col, self.alpha * value)),
        );
        Ok(sample)
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "physical state, numerical policy and resource inputs are independent"
)]
pub(crate) fn step(
    circuit: &mut impl Circuit,
    previous: &SpectralEnvelopeState,
    time: Value,
    method: SpectralEnvelopeMethod,
    config: &QuasiPeriodicSolveConfig,
    sources: &[Vec<Complex64>],
    storage: &[JacobianEntry],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<SpectralEnvelopeState, Error> {
    check_abort(abort)?;
    config.validate()?;
    if !time.is_finite() || time <= previous.time {
        return Err(Error::InvalidConfig(
            "envelope time must advance finitely from its accepted state".into(),
        ));
    }
    if circuit.unknowns() != previous.voltage_rows.len()
        || previous
            .voltage_rows
            .iter()
            .enumerate()
            .any(|(row, &voltage)| circuit.voltage_equation(row) != voltage)
    {
        return Err(Error::InvalidCircuit(
            "envelope history MNA coordinates changed".into(),
        ));
    }
    storage_valid(storage, circuit.unknowns())?;
    let grid = previous.grid();
    let bounded = workspace_limits(circuit.unknowns(), grid, storage, limits)?;
    check_workload(circuit.unknowns(), grid, &config.linear, &bounded)?;
    validate_spectra(sources, circuit.unknowns(), grid, "envelope source", abort)?;
    let h = time - previous.time;
    let (a0, a1, a2) = if method == SpectralEnvelopeMethod::Bdf2
        && let Some((older_time, _)) = &previous.older
        && let ratio = h / (previous.time - older_time)
        && ratio <= 2.0
    {
        (
            (1.0 + 2.0 * ratio) / (1.0 + ratio) / h,
            -(1.0 + ratio) / h,
            ratio * ratio / (1.0 + ratio) / h,
        )
    } else {
        (1.0 / h, -1.0 / h, 0.0)
    };
    if [a0, a1, a2].iter().any(|value| !value.is_finite()) || a0 <= 0.0 {
        return Err(Error::InvalidConfig(
            "envelope derivative coefficients are not representable".into(),
        ));
    }
    let mut rhs = sources.to_vec();
    for (row, entries) in rhs.iter_mut().enumerate() {
        check_abort(abort)?;
        for (k, value) in entries.iter_mut().enumerate() {
            *value += a1 * previous.charge[row][k];
            if a2 != 0.0 {
                *value += a2 * previous.older.as_ref().expect("BDF2 history").1[row][k];
            }
            if !finite(*value) {
                return Err(Error::Numerical(
                    "envelope history source overflowed".into(),
                ));
            }
        }
    }
    let solution = solve_with_abort(
        &mut ImplicitCircuit {
            inner: circuit,
            storage,
            alpha: a0,
        },
        grid.clone(),
        config,
        &rhs,
        Some(previous.spectra()),
        &bounded,
        abort,
    )?;
    let charge = charge_spectra(circuit, grid.clone(), solution.spectra(), storage, abort)?;
    check_abort(abort)?;
    Ok(SpectralEnvelopeState {
        time,
        solution,
        charge,
        older: Some((previous.time, previous.charge.clone())),
        voltage_rows: previous.voltage_rows.clone(),
        order: if a2 == 0.0 { 1 } else { 2 },
    })
}
