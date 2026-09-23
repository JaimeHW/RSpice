//! Real-coordinate Galerkin Newton solve on an independent-phase lattice.
//!
//! Small systems use a direct Jacobian factorization; larger systems use
//! matrix-free Krylov updates and a bounded-memory inverse certificate. Both
//! evaluate the same physical F/Q model on independent phases. Workspace is
//! checked before allocating the selected backend.

mod autonomous;
mod coordinates;
pub use autonomous::QuasiPeriodicAutonomousConfig;
pub(crate) use autonomous::solve_autonomous_with_abort;
pub(crate) use coordinates::validate as validate_spectra;
mod evaluation;
mod iterative;
mod linear_config;
mod preconditioner;
pub use linear_config::{QuasiPeriodicLinearConfig, QuasiPeriodicLinearMethod};
mod newton;
mod operator;
#[cfg(test)]
mod tests;

use super::{QuasiPeriodicError as Error, QuasiPeriodicGrid, QuasiPeriodicTransform};
use crate::abort_signal::AbortSignal;
use crate::{Complex64, ResourceKind, ResourceLimitError, ResourceLimits, Value};
use std::sync::Arc;

const MAX_DENSE_UNKNOWNS: usize = 512;
const MAX_WORKSPACE_VALUES: usize = 32_000_000;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicSolveConfig {
    pub relative_tolerance: Value,
    pub current_absolute_tolerance: Value,
    pub voltage_absolute_tolerance: Value,
    pub max_iterations: usize,
    pub max_backtracks: usize,
    #[serde(default, skip_serializing_if = "QuasiPeriodicLinearConfig::is_default")]
    pub linear: QuasiPeriodicLinearConfig,
}

impl Default for QuasiPeriodicSolveConfig {
    fn default() -> Self {
        Self {
            relative_tolerance: 1e-6,
            current_absolute_tolerance: 1e-12,
            voltage_absolute_tolerance: 1e-9,
            max_iterations: 100,
            max_backtracks: 20,
            linear: QuasiPeriodicLinearConfig::default(),
        }
    }
}

impl QuasiPeriodicSolveConfig {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        self.validate_relative_tolerance(true)
    }

    fn validate_relative_tolerance(&self, below_one: bool) -> Result<(), Error> {
        self.linear.validate()?;
        if !self.relative_tolerance.is_finite()
            || self.relative_tolerance <= 0.0
            || (below_one && self.relative_tolerance >= 1.0)
            || !self.current_absolute_tolerance.is_finite()
            || self.current_absolute_tolerance <= 0.0
            || !self.voltage_absolute_tolerance.is_finite()
            || self.voltage_absolute_tolerance <= 0.0
            || self.max_iterations == 0
            || self.max_backtracks > 60
        {
            return Err(Error::InvalidConfig(
                "positive finite tolerances, relative tolerance below one, at least one Newton update, and at most 60 backtracks are required".into(),
            ));
        }
        Ok(())
    }
}

/// A solved driven torus, including every node and exact MNA branch current.
/// Only the solver constructs this value after certifying every retained
/// equation. This numerical result is not yet an authenticated engine
/// operating point and must not be substituted for an HbOperatingPoint.
#[derive(Debug, Clone)]
pub struct QuasiPeriodicSolution {
    grid: Arc<QuasiPeriodicGrid>,
    spectra: Vec<Vec<Complex64>>,
    iterations: usize,
    normalized_residual: Value,
}

impl QuasiPeriodicSolution {
    pub fn grid(&self) -> &Arc<QuasiPeriodicGrid> {
        &self.grid
    }
    /// Fourier coefficients in MNA-coordinate order, then signed tuple order.
    pub fn spectra(&self) -> &[Vec<Complex64>] {
        &self.spectra
    }
    pub(crate) fn into_spectra(self) -> Vec<Vec<Complex64>> {
        self.spectra
    }
    pub(crate) fn with_preparation_iterations(mut self, iterations: usize) -> Self {
        self.iterations += iterations;
        self
    }
    pub fn iterations(&self) -> usize {
        self.iterations
    }
    /// Largest |residual| / (absolute tolerance + relative tolerance * sum
    /// of magnitudes of individual physical terms), across all equations.
    pub fn normalized_residual(&self) -> Value {
        self.normalized_residual
    }
}

pub(crate) type LinearEntry = (usize, usize, Complex64);
pub(crate) type JacobianEntry = (usize, usize, Value);

/// Delivered F/Q terms have the negative sign of the physical equations;
/// Jacobians have the positive physical sign. Term order and rows remain
/// stable over the orbit. Keep individual device contributions separate so
/// cancellations do not erase the scale used by convergence certificates.
#[derive(Default)]
pub(crate) struct Sample {
    pub current: Vec<(usize, Value)>,
    pub charge: Vec<(usize, Value)>,
    pub conductance: Vec<JacobianEntry>,
    pub capacitance: Vec<JacobianEntry>,
}

pub(crate) trait Circuit {
    fn unknowns(&self) -> usize;
    fn voltage_equation(&self, row: usize) -> bool;
    fn linear_entries(&self, frequency_hz: Value) -> Result<Vec<LinearEntry>, Error>;
    /// Analytic dY/df in Hz, needed when one carrier frequency is unknown.
    /// A model must supply its own derivative rather than silently treating
    /// a frequency-dependent network as a constant matrix.
    fn linear_frequency_derivative(&self, frequency_hz: Value) -> Result<Vec<LinearEntry>, Error> {
        if self.linear_entries(frequency_hz)?.is_empty() {
            Ok(Vec::new())
        } else {
            Err(Error::InvalidCircuit(
                "autonomous QPSS needs the linear network frequency derivative".into(),
            ))
        }
    }
    fn small_signal_entries(&self, frequency_hz: Value) -> Result<Vec<LinearEntry>, Error> {
        self.linear_entries(frequency_hz)
    }
    /// (Y(f + offset) - Y(f)) / (j 2 pi offset), evaluated without
    /// subtracting nearby stamps. Required by autonomous phase response.
    fn small_signal_frequency_difference(
        &self,
        frequency_hz: Value,
        offset_hz: Value,
    ) -> Result<Vec<LinearEntry>, Error> {
        if self.small_signal_entries(frequency_hz)?.is_empty()
            && self
                .small_signal_entries(frequency_hz + offset_hz)?
                .is_empty()
        {
            Ok(Vec::new())
        } else {
            Err(Error::InvalidCircuit(
                "autonomous QP response needs the linear network divided frequency difference"
                    .into(),
            ))
        }
    }
    fn sample(&mut self, state: &[Value], jacobian: bool) -> Result<Sample, Error>;
    fn sample_at_phases(
        &mut self,
        state: &[Value],
        _phases: &[Value],
        jacobian: bool,
    ) -> Result<Sample, Error> {
        self.sample(state, jacobian)
    }
}

struct Workspace<'a> {
    grid: Arc<QuasiPeriodicGrid>,
    transform: QuasiPeriodicTransform,
    config: &'a QuasiPeriodicSolveConfig,
    unknowns: usize,
    linear: Vec<Vec<LinearEntry>>,
    voltage_rows: Vec<bool>,
    base_values: usize,
    value_limit: usize,
}

/// Private initialization policy. This does not change the serialized QPSS
/// controls or its historical unit-step/backtracking behavior.
#[derive(Clone, Copy, Default)]
pub(crate) enum NewtonStepPolicy {
    #[default]
    QuasiPeriodic,
    HarmonicBalance {
        damping: Value,
        minimum_damping: Value,
    },
}

impl Workspace<'_> {
    fn budget(&self, extra: usize) -> Result<(), Error> {
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.base_values.saturating_add(extra),
            self.value_limit,
        )?;
        Ok(())
    }
}

/// Shared preflight for engine projection/initialization and the solver itself.
pub(crate) fn check_workload(
    unknowns: usize,
    grid: &QuasiPeriodicGrid,
    linear: &QuasiPeriodicLinearConfig,
    limits: &ResourceLimits,
) -> Result<(usize, usize), Error> {
    check_bordered_workload(unknowns, grid, linear, limits, 0)
}

pub(crate) fn check_bordered_workload(
    unknowns: usize,
    grid: &QuasiPeriodicGrid,
    linear: &QuasiPeriodicLinearConfig,
    limits: &ResourceLimits,
    border: usize,
) -> Result<(usize, usize), Error> {
    if unknowns == 0 {
        return Err(Error::InvalidCircuit(
            "circuit has no MNA coordinates".into(),
        ));
    }
    let size = unknowns.saturating_mul(grid.len()).saturating_add(border);
    ResourceLimitError::ensure(
        ResourceKind::MatrixUnknowns,
        size,
        if linear.uses_krylov(size) {
            limits.max_matrix_unknowns
        } else {
            limits.max_matrix_unknowns.min(MAX_DENSE_UNKNOWNS)
        },
    )?;
    ResourceLimitError::ensure(
        ResourceKind::AnalysisPoints,
        grid.sample_count(),
        limits.max_analysis_points,
    )?;
    // The iterative backend stores circuit-sized frequency blocks and a
    // bounded Arnoldi basis, never a full coupled Jacobian or inverse.
    let linear_values = if linear.uses_krylov(size) {
        grid.len()
            .saturating_mul(unknowns.saturating_mul(unknowns))
            .saturating_mul(64)
            .saturating_add(
                size.saturating_mul(linear.restart.saturating_mul(4).saturating_add(64)),
            )
    } else {
        size.saturating_mul(size).saturating_mul(16)
    };
    let base_values = linear_values
        .saturating_add(size.saturating_mul(32))
        .saturating_add(
            grid.sample_count()
                .saturating_mul(unknowns.saturating_mul(24).saturating_add(16)),
        )
        .saturating_add(
            grid.len()
                .saturating_mul(grid.dimensions().len().saturating_add(6)),
        );
    let value_limit = limits.max_result_values.min(MAX_WORKSPACE_VALUES);
    ResourceLimitError::ensure(ResourceKind::ResultValues, base_values, value_limit)?;
    Ok((base_values, value_limit))
}

/// Source spectra are the complete MNA right hand side, with full signed
/// Fourier coefficients. Source projection and netlist identity belong to the
/// engine. A missing seed means zero; an authored seed must match exactly.
pub(crate) fn solve_with_abort(
    circuit: &mut impl Circuit,
    grid: Arc<QuasiPeriodicGrid>,
    config: &QuasiPeriodicSolveConfig,
    sources: &[Vec<Complex64>],
    seed: Option<&[Vec<Complex64>]>,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<QuasiPeriodicSolution, Error> {
    solve_with_iteration_budget(
        circuit,
        grid,
        config,
        sources,
        seed,
        limits,
        config.max_iterations,
        abort,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "numerical API keeps independent circuit, spectral, and resource inputs explicit"
)]
pub(crate) fn solve_with_iteration_budget(
    circuit: &mut impl Circuit,
    grid: Arc<QuasiPeriodicGrid>,
    config: &QuasiPeriodicSolveConfig,
    sources: &[Vec<Complex64>],
    seed: Option<&[Vec<Complex64>]>,
    limits: &ResourceLimits,
    remaining_iterations: usize,
    abort: &dyn AbortSignal,
) -> Result<QuasiPeriodicSolution, Error> {
    solve_with_step_policy(
        circuit,
        grid,
        config,
        sources,
        seed,
        limits,
        remaining_iterations,
        NewtonStepPolicy::default(),
        abort,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "numerical API keeps independent circuit, spectral, and resource inputs explicit"
)]
pub(crate) fn solve_with_step_policy(
    circuit: &mut impl Circuit,
    grid: Arc<QuasiPeriodicGrid>,
    config: &QuasiPeriodicSolveConfig,
    sources: &[Vec<Complex64>],
    seed: Option<&[Vec<Complex64>]>,
    limits: &ResourceLimits,
    remaining_iterations: usize,
    steps: NewtonStepPolicy,
    abort: &dyn AbortSignal,
) -> Result<QuasiPeriodicSolution, Error> {
    super::check_abort(abort)?;
    match steps {
        NewtonStepPolicy::QuasiPeriodic => config.validate()?,
        NewtonStepPolicy::HarmonicBalance {
            damping,
            minimum_damping,
        } => {
            config.validate_relative_tolerance(false)?;
            if !damping.is_finite()
                || damping <= 0.0
                || damping > 1.0
                || !minimum_damping.is_finite()
                || minimum_damping <= 0.0
                || minimum_damping > damping
            {
                return Err(Error::InvalidConfig(
                    "invalid periodic Newton damping envelope".into(),
                ));
            }
        }
    }
    if remaining_iterations > config.max_iterations {
        return Err(Error::InvalidConfig(
            "periodic iteration budget exceeds the configured maximum".into(),
        ));
    }
    let mut bounded = config.clone();
    bounded.max_iterations = remaining_iterations;
    let config = &bounded;
    let unknowns = circuit.unknowns();
    let (base_values, value_limit) = check_workload(unknowns, &grid, &config.linear, limits)?;
    coordinates::validate(sources, unknowns, &grid, "source", abort)?;
    if let Some(seed) = seed {
        coordinates::validate(seed, unknowns, &grid, "initial state", abort)?;
    }
    let transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?;
    let mut workspace = Workspace {
        grid,
        transform,
        config,
        unknowns,
        linear: Vec::new(),
        voltage_rows: (0..unknowns)
            .map(|row| circuit.voltage_equation(row))
            .collect(),
        base_values,
        value_limit,
    };
    for &frequency in workspace.grid.frequencies_hz() {
        super::check_abort(abort)?;
        let entries = circuit.linear_entries(frequency)?;
        for &(row, column, value) in &entries {
            if row >= unknowns || column >= unknowns || !super::finite(value) {
                return Err(Error::InvalidCircuit(
                    "invalid frequency-domain MNA entry".into(),
                ));
            }
        }
        workspace.base_values = workspace
            .base_values
            .saturating_add(entries.len().saturating_mul(4));
        workspace.budget(0)?;
        workspace.linear.push(entries);
    }
    workspace.newton(circuit, sources, seed, steps, abort)
}
