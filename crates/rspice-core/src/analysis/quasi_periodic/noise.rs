//! Cyclostationary noise on independent tone phases, retaining sideband correlation.
//!
//! For Aᴴλ=c and a source direction b, g_k=bᴴλ_k. White source intensity
//! q(θ) contributes <conj(g_r(θ)) g_c(θ) q(θ)> to C[r,c]=E[y_r conj(y_c)].
//! A colored stationary process modulated by real a(θ) instead contributes
//! Σ_m conj(h_rm) h_cm S(f_m), h_rm=Σ_k g_rk conj(a_(k-m)). The latter sum
//! extends outside the circuit's retained lattice, with no cyclic FFT wrap.
//! Both are the multidimensional conversion/covariance formulation described
//! at https://qucs.sourceforge.net/tech/node36.html. No common period is used.
mod colored;
mod projection;
mod sweep;
pub(crate) use sweep::visit_with_abort;
pub use sweep::{QuasiPeriodicNoiseConfig, QuasiPeriodicNoisePoint};
#[cfg(test)]
mod tests;

use super::{
    QuasiPeriodicAdjointSolution, QuasiPeriodicError as Error, QuasiPeriodicGrid,
    QuasiPeriodicTransform, check_abort, finite,
};
use crate::numerics::scaled_noise::*;
use crate::{
    Complex64, ResourceKind, ResourceLimitError, ResourceLimits, Value, abort_signal::AbortSignal,
};
use std::{collections::HashSet, sync::Arc};

/// One-sided physical source law; negative translated frequencies use |f|.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum QuasiPeriodicNoiseSpectrum {
    /// A single stationary density, or one nonnegative density per torus sample.
    /// Values are multiplied by 2^binary_scale_exponent after transfer products.
    White {
        density: Vec<Value>,
        binary_scale_exponent: i32,
    },
    /// coefficient * 2^binary_scale_exponent / |f|^exponent, multiplied by a
    /// signed real modulation waveform represented in full Fourier coefficients.
    /// In particular, do not rectify a resistor's signed current modulation.
    PowerLaw {
        coefficient: Value,
        exponent: Value,
        modulation: Vec<Complex64>,
        binary_scale_exponent: i32,
    },
}

/// One independent stochastic mechanism. Its injections share one random
/// process, so their signed/complex cross terms must survive projection.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicNoiseSource {
    pub name: String,
    pub injections: Vec<(usize, Complex64)>,
    pub spectrum: QuasiPeriodicNoiseSpectrum,
}

/// Row-major output covariance for one source and one authored frequency.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicNoiseCovariance {
    pub outputs: usize,
    pub values: Vec<Complex64>,
    /// Absolute accumulation-roundoff bounds; not a truncation-error estimate.
    pub roundoff_bounds: Vec<Value>,
}

/// Reusable projection workspace. The selected tuples bound noise injection
/// into the circuit; the adjoint still comes from the complete circuit basis.
pub struct QuasiPeriodicNoiseProjector {
    grid: Arc<QuasiPeriodicGrid>,
    transform: QuasiPeriodicTransform,
    selected: Vec<usize>,
    coordinates: usize,
    limits: ResourceLimits,
}

fn invalid(message: impl Into<String>) -> Error {
    Error::InvalidConfig(format!("QPNOISE: {}", message.into()))
}
fn numerical(message: impl Into<String>) -> Error {
    Error::Numerical(format!("QPNOISE: {}", message.into()))
}
fn checked_exponent(a: i32, b: i32, c: i32) -> Result<i32, Error> {
    i32::try_from(i64::from(a) + i64::from(b) + i64::from(c))
        .map_err(|_| numerical("combined binary scale exceeds the supported range"))
}

impl QuasiPeriodicNoiseProjector {
    pub fn new_with_abort(
        grid: Arc<QuasiPeriodicGrid>,
        coordinates: usize,
        input_lattices: &[Vec<i32>],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, Error> {
        check_abort(abort)?;
        if coordinates == 0 || input_lattices.is_empty() {
            return Err(invalid(
                "at least one MNA coordinate and noise-input tuple are required",
            ));
        }
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            coordinates.saturating_mul(grid.len()),
            limits.max_matrix_unknowns,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            input_lattices.len(),
            limits.max_analysis_points,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            grid.sample_count()
                .saturating_mul(8)
                .saturating_add(input_lattices.len().saturating_mul(2)),
            limits.max_result_values.min(32_000_000),
        )?;
        let mut seen = HashSet::new();
        let mut selected = Vec::with_capacity(input_lattices.len());
        for tuple in input_lattices {
            check_abort(abort)?;
            let index = grid.index_of(tuple).ok_or_else(|| {
                invalid(format!(
                    "noise-input tuple {tuple:?} is outside the retained circuit lattice"
                ))
            })?;
            if !seen.insert(index) {
                return Err(invalid("noise-input tuples must be unique"));
            }
            selected.push(index);
        }
        selected.sort_unstable();
        let transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?;
        Ok(Self {
            grid,
            transform,
            selected,
            coordinates,
            limits: limits.clone(),
        })
    }

    pub fn source_covariance_with_abort(
        &mut self,
        adjoints: &[QuasiPeriodicAdjointSolution],
        source: &QuasiPeriodicNoiseSource,
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicNoiseCovariance, Error> {
        check_abort(abort)?;
        self.validate(adjoints, source, abort)?;
        let gains = adjoints
            .iter()
            .map(|a| projection::project(&self.grid, &self.selected, a, &source.injections, abort))
            .collect::<Result<Vec<_>, _>>()?;
        match &source.spectrum {
            QuasiPeriodicNoiseSpectrum::White {
                density,
                binary_scale_exponent,
            } => self.white(&gains, density, *binary_scale_exponent, abort),
            QuasiPeriodicNoiseSpectrum::PowerLaw {
                coefficient,
                exponent,
                modulation,
                binary_scale_exponent,
            } => self.colored(
                &gains,
                &adjoints[0],
                *coefficient,
                *exponent,
                modulation,
                *binary_scale_exponent,
                abort,
            ),
        }
    }

    fn validate(
        &self,
        adjoints: &[QuasiPeriodicAdjointSolution],
        source: &QuasiPeriodicNoiseSource,
        abort: &dyn AbortSignal,
    ) -> Result<(), Error> {
        let Some(first) = adjoints.first() else {
            return Err(invalid("at least one output adjoint is required"));
        };
        self.check_values(adjoints.len(), 0)?;
        if source.name.trim().is_empty() || source.injections.is_empty() {
            return Err(invalid(
                "noise mechanisms require a name and nonempty injection direction",
            ));
        }
        let mut rows = HashSet::new();
        for &(row, value) in &source.injections {
            check_abort(abort)?;
            if row >= self.coordinates
                || !finite(value)
                || value == Complex64::ZERO
                || !rows.insert(row)
            {
                return Err(invalid(
                    "source injections must be finite, nonzero, unique MNA coordinates",
                ));
            }
        }
        if !first.frequency_hz.is_finite()
            || first.frequency_lattice.len() != self.grid.dimensions().len()
        {
            return Err(invalid(
                "adjoints require a finite authored frequency and exact tuple anchor",
            ));
        }
        for a in adjoints {
            check_abort(abort)?;
            if a.frequency_hz.to_bits() != first.frequency_hz.to_bits()
                || a.frequency_lattice != first.frequency_lattice
                || a.sensitivities.len() != self.coordinates
                || !a.normalized_residual.is_finite()
                || !(0.0..=1.0).contains(&a.normalized_residual)
            {
                return Err(invalid(
                    "output adjoints must share a certified frequency, anchor and MNA basis",
                ));
            }
            for row in &a.sensitivities {
                check_abort(abort)?;
                if row.len() != self.grid.len() || invalid_values(row, |v| !finite(*v), abort)? {
                    return Err(invalid(
                        "adjoint spectrum differs from its finite full signed lattice",
                    ));
                }
            }
        }
        match &source.spectrum {
            QuasiPeriodicNoiseSpectrum::White { density, .. } => {
                if (density.len() != 1 && density.len() != self.grid.sample_count())
                    || invalid_values(density, |q| !q.is_finite() || *q < 0.0, abort)?
                {
                    return Err(invalid(
                        "white intensity requires one or one-per-phase finite nonnegative density",
                    ));
                }
            }
            QuasiPeriodicNoiseSpectrum::PowerLaw {
                coefficient,
                exponent,
                modulation,
                ..
            } => {
                if !coefficient.is_finite()
                    || *coefficient < 0.0
                    || !exponent.is_finite()
                    || modulation.len() != self.grid.len()
                    || invalid_values(modulation, |a| !finite(*a), abort)?
                {
                    return Err(invalid(
                        "colored noise requires finite coefficient, exponent and full modulation spectrum",
                    ));
                }
                for (i, a) in modulation.iter().enumerate() {
                    if i.is_multiple_of(256) {
                        check_abort(abort)?;
                    }
                    let reflected = modulation[modulation.len() - 1 - i].conj();
                    let scale =
                        a.re.abs()
                            .max(a.im.abs())
                            .max(reflected.re.abs())
                            .max(reflected.im.abs());
                    if scale > 0.0
                        && (*a / scale - reflected / scale).norm() > 128.0 * Value::EPSILON
                    {
                        return Err(invalid(
                            "physical colored-noise modulation must be real on independent phases",
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn check_values(&self, outputs: usize, extra: usize) -> Result<(), Error> {
        let values = self.workspace_values(outputs, extra);
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            values,
            self.limits.max_result_values.min(32_000_000),
        )?;
        Ok(())
    }

    fn workspace_values(&self, outputs: usize, extra: usize) -> usize {
        let values = self
            .grid
            .sample_count()
            .saturating_mul(8usize.saturating_add(outputs.saturating_mul(2)))
            .saturating_add(outputs.saturating_mul(self.grid.len()).saturating_mul(12))
            .saturating_add(outputs.saturating_mul(outputs).saturating_mul(6))
            .saturating_add(extra);
        values
    }

    fn white(
        &mut self,
        gains: &[projection::Spectrum],
        density: &[Value],
        binary_scale: i32,
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicNoiseCovariance, Error> {
        let waves = if density.len() == 1 {
            None
        } else {
            Some(
                gains
                    .iter()
                    .map(|g| self.transform.to_samples_with_abort(&g.values, abort))
                    .collect::<Result<Vec<_>, _>>()?,
            )
        };
        covariance(
            gains.len(),
            |r, c| {
                let exponent =
                    checked_exponent(gains[r].exponent, gains[c].exponent, binary_scale)?;
                let (left, right, count) = if let Some(waves) = &waves {
                    (&waves[r], &waves[c], self.grid.sample_count())
                } else {
                    (&gains[r].values, &gains[c].values, self.grid.len())
                };
                sum_terms(
                    |visit| {
                        for i in 0..count {
                            if i.is_multiple_of(256) {
                                check_abort(abort)?;
                            }
                            let q = if density.len() == 1 {
                                density[0]
                            } else {
                                density[i]
                            };
                            let mut term = scaled_complex_product3(
                                left[i].conj(),
                                right[i],
                                Complex64::new(q, 0.0),
                                exponent,
                            )
                            .map_err(numerical)?;
                            if density.len() != 1 {
                                term = scaled_complex_product3(
                                    term.mantissa,
                                    Complex64::new(1.0 / count as Value, 0.0),
                                    Complex64::ONE,
                                    term.exponent,
                                )
                                .map_err(numerical)?;
                            }
                            visit(term)?;
                        }
                        Ok(())
                    },
                    r == c,
                )
            },
            abort,
        )
    }
}

fn covariance(
    outputs: usize,
    mut entry: impl FnMut(usize, usize) -> Result<(Complex64, Value), Error>,
    abort: &dyn AbortSignal,
) -> Result<QuasiPeriodicNoiseCovariance, Error> {
    let mut values = vec![Complex64::ZERO; outputs * outputs];
    let mut roundoff_bounds = vec![0.0; outputs * outputs];
    for r in 0..outputs {
        for c in r..outputs {
            check_abort(abort)?;
            let (value, bound) = entry(r, c)?;
            values[r * outputs + c] = value;
            values[c * outputs + r] = value.conj();
            roundoff_bounds[r * outputs + c] = bound;
            roundoff_bounds[c * outputs + r] = bound;
        }
    }
    Ok(QuasiPeriodicNoiseCovariance {
        outputs,
        values,
        roundoff_bounds,
    })
}

fn sum_terms(
    mut visit: impl FnMut(&mut dyn FnMut(ScaledComplex) -> Result<(), Error>) -> Result<(), Error>,
    diagonal: bool,
) -> Result<(Complex64, Value), Error> {
    let mut common = None;
    let mut count = 0usize;
    visit(&mut |term| {
        validate_scaled_complex(term).map_err(numerical)?;
        if !term.is_zero() {
            common = Some(common.map_or(term.exponent, |e: i32| e.max(term.exponent)));
            count += 1;
        }
        Ok(())
    })?;
    let Some(common) = common else {
        return Ok((Complex64::ZERO, 0.0));
    };
    let mut sum = ScaledComplexAccumulator::new(common);
    visit(&mut |term| sum.add(term).map_err(numerical))?;
    let (mut value, absolute) = sum.finish().map_err(numerical)?;
    let bound = absolute * (Value::EPSILON * 32.0 * count.max(1) as Value);
    if !bound.is_finite() {
        return Err(numerical("noise accumulation roundoff bound overflowed"));
    }
    if diagonal {
        if value.im.abs() > bound || value.re < -bound {
            return Err(numerical("noise covariance is not Hermitian nonnegative"));
        }
        value = Complex64::new(value.re.max(0.0), 0.0);
    }
    Ok((value, bound))
}

fn invalid_values<T>(
    values: &[T],
    invalid: impl Fn(&T) -> bool,
    abort: &dyn AbortSignal,
) -> Result<bool, Error> {
    for (i, value) in values.iter().enumerate() {
        if i.is_multiple_of(256) {
            check_abort(abort)?;
        }
        if invalid(value) {
            return Ok(true);
        }
    }
    Ok(false)
}
