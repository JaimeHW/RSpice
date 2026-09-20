//! Real-coordinate Galerkin Newton solve on an independent-phase lattice.
//!
//! The initial backend is deliberately bounded: it assembles an analytic
//! Jacobian and uses the shared certified matrix solver. Circuit adapters
//! supply physical F/Q terms, never a fitted surrogate or a common-period HB
//! orbit. Resource limits are checked before constructing the dense operator.

mod coordinates;
mod evaluation;
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
}

impl Default for QuasiPeriodicSolveConfig {
    fn default() -> Self {
        Self {
            relative_tolerance: 1e-6,
            current_absolute_tolerance: 1e-12,
            voltage_absolute_tolerance: 1e-9,
            max_iterations: 100,
            max_backtracks: 20,
        }
    }
}

impl QuasiPeriodicSolveConfig {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if !self.relative_tolerance.is_finite()
            || self.relative_tolerance <= 0.0
            || self.relative_tolerance >= 1.0
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
    fn sample(&mut self, state: &[Value], jacobian: bool) -> Result<Sample, Error>;
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
    limits: &ResourceLimits,
) -> Result<(usize, usize), Error> {
    if unknowns == 0 {
        return Err(Error::InvalidCircuit(
            "circuit has no MNA coordinates".into(),
        ));
    }
    let size = unknowns.saturating_mul(grid.len());
    ResourceLimitError::ensure(
        ResourceKind::MatrixUnknowns,
        size,
        limits.max_matrix_unknowns.min(MAX_DENSE_UNKNOWNS),
    )?;
    ResourceLimitError::ensure(
        ResourceKind::AnalysisPoints,
        grid.sample_count(),
        limits.max_analysis_points,
    )?;
    // Includes dense matrix/factorization and triplet storage, active and
    // candidate spectra, phase fields, Fourier scratch and retained tuples.
    let base_values = size
        .saturating_mul(size)
        .saturating_mul(16)
        .saturating_add(size.saturating_mul(32))
        .saturating_add(
            grid.sample_count()
                .saturating_mul(unknowns.saturating_mul(16).saturating_add(16)),
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
    super::check_abort(abort)?;
    config.validate()?;
    let unknowns = circuit.unknowns();
    let (base_values, value_limit) = check_workload(unknowns, &grid, limits)?;
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
    workspace.newton(circuit, sources, seed, abort)
}
