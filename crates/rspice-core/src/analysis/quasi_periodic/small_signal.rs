//! Complex translated F/Q linearization around a real independent-phase orbit.
mod linear;
mod operator;
#[cfg(test)]
mod tests;

use super::solve::{Circuit, JacobianEntry, LinearEntry};
use super::{
    QuasiPeriodicError as Error, QuasiPeriodicGrid, QuasiPeriodicLinearConfig,
    QuasiPeriodicTransform, check_abort, finite,
};
use crate::abort_signal::AbortSignal;
use crate::{Complex64, ResourceKind, ResourceLimitError, ResourceLimits, Value};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct QuasiPeriodicAcConfig {
    pub linear: QuasiPeriodicLinearConfig,
    /// Absolute KCL residual tolerance, in amperes.
    pub current_absolute_tolerance: Value,
    /// Absolute branch-equation residual tolerance, in volts.
    pub voltage_absolute_tolerance: Value,
}

impl Default for QuasiPeriodicAcConfig {
    fn default() -> Self {
        Self {
            linear: QuasiPeriodicLinearConfig::default(),
            current_absolute_tolerance: 1e-12,
            voltage_absolute_tolerance: 1e-9,
        }
    }
}

impl QuasiPeriodicAcConfig {
    pub fn validate(&self) -> Result<(), Error> {
        self.linear.validate()?;
        if !self.current_absolute_tolerance.is_finite()
            || self.current_absolute_tolerance <= 0.0
            || !self.voltage_absolute_tolerance.is_finite()
            || self.voltage_absolute_tolerance <= 0.0
        {
            return Err(Error::InvalidConfig(
                "QPAC requires positive finite current and voltage absolute tolerances".into(),
            ));
        }
        Ok(())
    }
}

/// Complex phasors for one probe offset. Neither an implicit factor of two
/// nor conjugate reflection is applied: every signed tuple is independent.
#[derive(Debug, Clone)]
pub struct QuasiPeriodicAcSolution {
    pub offset_hz: Value,
    /// MNA-coordinate order, then the operating point's signed tuple order.
    pub spectra: Vec<Vec<Complex64>>,
    pub normalized_residual: Value,
}

struct Derivatives {
    conductance: Vec<JacobianEntry>,
    capacitance: Vec<JacobianEntry>,
}

pub(crate) struct Linearization {
    grid: Arc<QuasiPeriodicGrid>,
    config: QuasiPeriodicAcConfig,
    transform: QuasiPeriodicTransform,
    unknowns: usize,
    voltage_rows: Vec<bool>,
    derivatives: Vec<Derivatives>,
    base_values: usize,
    value_limit: usize,
}

impl Linearization {
    pub(crate) fn prepare(
        circuit: &mut impl Circuit,
        grid: Arc<QuasiPeriodicGrid>,
        orbit: &[Vec<Complex64>],
        config: &QuasiPeriodicAcConfig,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, Error> {
        check_abort(abort)?;
        config.validate()?;
        let unknowns = circuit.unknowns();
        let (mut base_values, value_limit) =
            super::solve::check_workload(unknowns, &grid, &config.linear, limits)?;
        if orbit.len() != unknowns {
            return Err(Error::InvalidConfig(
                "QPAC orbit differs from its MNA coordinates".into(),
            ));
        }
        let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?;
        let waves = orbit
            .iter()
            .map(|spectrum| transform.to_real_samples_with_abort(spectrum, abort))
            .collect::<Result<Vec<_>, _>>()?;
        let mut derivatives = Vec::with_capacity(grid.sample_count());
        let mut state = vec![0.0; unknowns];
        for time in 0..grid.sample_count() {
            check_abort(abort)?;
            for (row, value) in state.iter_mut().enumerate() {
                *value = waves[row][time];
            }
            let sample = circuit.sample(&state, true)?;
            base_values = base_values.saturating_add(
                sample
                    .conductance
                    .len()
                    .saturating_add(sample.capacitance.len())
                    .saturating_mul(6),
            );
            ResourceLimitError::ensure(ResourceKind::ResultValues, base_values, value_limit)?;
            for &(row, col, value) in sample.conductance.iter().chain(&sample.capacitance) {
                if row >= unknowns || col >= unknowns || !value.is_finite() {
                    return Err(Error::InvalidCircuit(
                        "QPAC device derivative has an invalid MNA coordinate or non-finite value"
                            .into(),
                    ));
                }
            }
            derivatives.push(Derivatives {
                conductance: sample.conductance,
                capacitance: sample.capacitance,
            });
        }
        Ok(Self {
            config: config.clone(),
            grid,
            transform,
            unknowns,
            derivatives,
            base_values,
            value_limit,
            voltage_rows: (0..unknowns)
                .map(|row| circuit.voltage_equation(row))
                .collect(),
        })
    }

    pub(crate) fn solve(
        &mut self,
        circuit: &impl Circuit,
        offset_hz: Value,
        sources: &[Vec<Complex64>],
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicAcSolution, Error> {
        check_abort(abort)?;
        let config = self.config.clone();
        if !offset_hz.is_finite()
            || sources.len() != self.unknowns
            || sources
                .iter()
                .any(|row| row.len() != self.grid.len() || row.iter().any(|v| !finite(*v)))
        {
            return Err(Error::InvalidConfig(
                "QPAC requires a finite offset and complete finite complex MNA source spectra"
                    .into(),
            ));
        }
        let mut linear = Vec::with_capacity(self.grid.len());
        let mut frequencies = Vec::with_capacity(self.grid.len());
        let mut retained = self.base_values;
        for &frequency in self.grid.frequencies_hz() {
            check_abort(abort)?;
            let frequency = offset_hz + frequency;
            if !frequency.is_finite() || !(std::f64::consts::TAU * frequency).is_finite() {
                return Err(Error::InvalidConfig(
                    "QPAC translated frequency overflowed".into(),
                ));
            }
            let entries = circuit.small_signal_entries(frequency)?;
            if entries
                .iter()
                .any(|&(r, c, v)| r >= self.unknowns || c >= self.unknowns || !finite(v))
            {
                return Err(Error::InvalidCircuit(
                    "QPAC frequency-domain MNA entry is invalid".into(),
                ));
            }
            retained = retained.saturating_add(entries.len().saturating_mul(4));
            ResourceLimitError::ensure(ResourceKind::ResultValues, retained, self.value_limit)?;
            linear.push(entries);
            frequencies.push(frequency);
        }
        let rhs: Vec<_> = sources.iter().flatten().copied().collect();
        let solution = if config.linear.uses_krylov(rhs.len()) {
            self.iterative(&frequencies, &linear, &rhs, &config, abort)?
        } else {
            self.direct(&frequencies, &linear, &rhs, abort)?
        };
        let applied = self.apply(&frequencies, &linear, &solution, abort)?;
        let mut merit: Value = 0.0;
        for (index, (actual, expected)) in applied.iter().zip(&rhs).enumerate() {
            let absolute = if self.voltage_rows[index / self.grid.len()] {
                config.voltage_absolute_tolerance
            } else {
                config.current_absolute_tolerance
            };
            let scale =
                absolute + config.linear.relative_tolerance * actual.norm().max(expected.norm());
            let residual = (*actual - *expected).norm() / scale;
            if !residual.is_finite() || !scale.is_finite() {
                return Err(Error::Numerical(
                    "QPAC residual certificate overflowed".into(),
                ));
            }
            merit = merit.max(residual);
        }
        if merit > 1.0 {
            return Err(Error::Numerical(format!(
                "QPAC physical equation residual exceeds tolerance ({merit:e})"
            )));
        }
        check_abort(abort)?;
        Ok(QuasiPeriodicAcSolution {
            offset_hz,
            spectra: solution
                .chunks_exact(self.grid.len())
                .map(<[_]>::to_vec)
                .collect(),
            normalized_residual: merit,
        })
    }
}
