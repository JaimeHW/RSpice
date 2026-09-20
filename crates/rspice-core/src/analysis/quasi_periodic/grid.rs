//! Checked tone lattice and Cartesian phase-grid construction.
use super::{QuasiPeriodicError as Error, check_abort, finite};
use crate::abort_signal::AbortSignal;
use crate::{Complex64, ResourceKind, ResourceLimitError, ResourceLimits, Value};

// Bound one non-interruptible FFT plan/transform, including callers whose
// ordinary resource limits are unlimited, as the periodic HB kernel does.
const MAX_COLLOCATION_POINTS: usize = 2_000_000;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(
    deny_unknown_fields,
    tag = "mode",
    content = "points",
    rename_all = "snake_case"
)]
pub enum QuasiPeriodicSampling {
    /// Per-tone oversampling, relative to the minimal 2H+1 grid. Each
    /// resulting dimension is rounded up to a power of two.
    Oversample(Vec<usize>),
    /// Exact per-tone collocation counts, each at least 2H+1. Odd and even
    /// counts are both supported; Nyquist modes are never retained.
    Exact(Vec<usize>),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicGridConfig {
    pub frequencies_hz: Vec<Value>,
    pub harmonics: Vec<usize>,
    /// None retains the full box. Some limits mixed products by the sum of
    /// absolute indices; each individual tone still retains its own authored
    /// harmonics. This preserves high carrier harmonics when mixing is sparse.
    pub max_mixing_order: Option<usize>,
    pub sampling: QuasiPeriodicSampling,
}

impl QuasiPeriodicGridConfig {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        let invalid = |message: &str| Error::InvalidConfig(message.into());
        let tones = self.frequencies_hz.len();
        if tones < 2 || self.harmonics.len() != tones {
            return Err(invalid(
                "at least two tone frequencies and one harmonic count per tone are required",
            ));
        }
        if self
            .frequencies_hz
            .iter()
            .any(|f| !f.is_finite() || *f <= 0.0)
        {
            return Err(invalid("tone frequencies must be finite and positive"));
        }
        if self
            .harmonics
            .iter()
            .any(|h| *h == 0 || *h > i32::MAX as usize)
        {
            return Err(invalid(
                "tone harmonic counts must be positive signed-index integers",
            ));
        }
        if self.max_mixing_order == Some(0) {
            return Err(invalid("mixing order must be positive when specified"));
        }
        let (samples, exact) = match &self.sampling {
            QuasiPeriodicSampling::Oversample(values) => (values, false),
            QuasiPeriodicSampling::Exact(values) => (values, true),
        };
        if samples.len() != tones || samples.contains(&0) {
            return Err(invalid("sampling requires one positive count per tone"));
        }
        if exact
            && samples
                .iter()
                .zip(&self.harmonics)
                .any(|(n, h)| *n < h.saturating_mul(2).saturating_add(1))
        {
            return Err(invalid("each phase grid must have at least 2H+1 points"));
        }
        Ok(())
    }

    pub fn new(frequencies_hz: Vec<Value>, harmonics: Vec<usize>) -> Self {
        let tones = frequencies_hz.len();
        Self {
            frequencies_hz,
            harmonics,
            max_mixing_order: None,
            sampling: QuasiPeriodicSampling::Oversample(vec![2; tones]),
        }
    }
}

/// Immutable, validated basis. Entries are ordered lexicographically by
/// integer tuple, never sorted or merged by approximate physical frequency.
/// For real fields entry len-1-i is the conjugate of entry i; DC is len/2.
#[derive(Debug, Clone)]
pub struct QuasiPeriodicGrid {
    config: QuasiPeriodicGridConfig,
    dimensions: Vec<usize>,
    strides: Vec<usize>,
    sample_count: usize,
    indices: Vec<Vec<i32>>,
    frequencies: Vec<Value>,
    bins: Vec<usize>,
}

impl QuasiPeriodicGrid {
    pub fn new_with_abort(
        config: QuasiPeriodicGridConfig,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, Error> {
        check_abort(abort)?;
        let invalid = |message: &str| Error::InvalidConfig(message.into());
        config.validate()?;
        let tones = config.frequencies_hz.len();
        let (samples, exact) = match &config.sampling {
            QuasiPeriodicSampling::Oversample(values) => (values, false),
            QuasiPeriodicSampling::Exact(values) => (values, true),
        };
        let minimal_grid = 3usize
            .checked_pow(u32::try_from(tones).unwrap_or(u32::MAX))
            .unwrap_or(usize::MAX);
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            minimal_grid,
            limits.max_analysis_points.min(MAX_COLLOCATION_POINTS),
        )?;
        let mut dimensions = Vec::with_capacity(tones);
        let mut strides = Vec::with_capacity(tones);
        let mut sample_count = 1usize;
        let point_limit = limits.max_analysis_points.min(MAX_COLLOCATION_POINTS);
        for (harmonics, points) in config.harmonics.iter().zip(samples) {
            check_abort(abort)?;
            let minimum = harmonics
                .checked_mul(2)
                .and_then(|h| h.checked_add(1))
                .ok_or_else(|| invalid("harmonic grid dimension overflowed"))?;
            let size = if exact {
                *points
            } else {
                minimum
                    .checked_mul(*points)
                    .and_then(usize::checked_next_power_of_two)
                    .ok_or_else(|| invalid("oversampled grid dimension overflowed"))?
            };
            if size < minimum {
                return Err(invalid("each phase grid must have at least 2H+1 points"));
            }
            strides.push(sample_count);
            sample_count = sample_count.checked_mul(size).unwrap_or(usize::MAX);
            ResourceLimitError::ensure(ResourceKind::AnalysisPoints, sample_count, point_limit)?;
            dimensions.push(size);
        }
        // FFT workspace, input/output and scratch are accounted for before any
        // plans are allocated. Each lattice tuple adds its own retained cost.
        let workspace_values = sample_count.checked_mul(8).unwrap_or(usize::MAX);
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            workspace_values,
            limits.max_result_values,
        )?;
        let mut grid = Self {
            config,
            dimensions,
            strides,
            sample_count,
            indices: Vec::new(),
            frequencies: Vec::new(),
            bins: Vec::new(),
        };
        let mut tuple = vec![0i32; tones];
        let mut visited = 0usize;
        grid.generate(0, &mut tuple, workspace_values, limits, abort, &mut visited)?;
        grid.validate_frequencies(abort)?;
        check_abort(abort)?;
        Ok(grid)
    }

    fn generate(
        &mut self,
        axis: usize,
        tuple: &mut [i32],
        workspace_values: usize,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
        visited: &mut usize,
    ) -> Result<(), Error> {
        if axis == tuple.len() {
            *visited += 1;
            if visited.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let nonzero = tuple.iter().filter(|k| **k != 0).count();
            let order = tuple
                .iter()
                .map(|k| u64::from(k.unsigned_abs()))
                .sum::<u64>();
            if nonzero > 1
                && self
                    .config
                    .max_mixing_order
                    .is_some_and(|max| order > max as u64)
            {
                return Ok(());
            }
            let cost = (self.indices.len() + 1)
                .checked_mul(tuple.len() + 6)
                .and_then(|n| n.checked_add(workspace_values))
                .unwrap_or(usize::MAX);
            ResourceLimitError::ensure(ResourceKind::ResultValues, cost, limits.max_result_values)?;
            self.indices.push(tuple.to_vec());
            self.bins.push(
                self.bin(tuple)
                    .expect("generated tuple lies inside the grid"),
            );
            let mut frequency = 0.0;
            let mut correction = 0.0;
            for (k, f) in tuple.iter().zip(&self.config.frequencies_hz) {
                crate::numerics::compensated_add(
                    &mut frequency,
                    &mut correction,
                    f64::from(*k) * f,
                );
            }
            let frequency = frequency + correction;
            if !frequency.is_finite() || !(std::f64::consts::TAU * frequency).is_finite() {
                return Err(Error::InvalidConfig(
                    "a mixing frequency or angular frequency exceeds the finite range".into(),
                ));
            }
            self.frequencies.push(frequency);
            return Ok(());
        }
        let bound = self.config.harmonics[axis] as i32;
        for k in -bound..=bound {
            tuple[axis] = k;
            self.generate(axis + 1, tuple, workspace_values, limits, abort, visited)?;
        }
        Ok(())
    }

    fn validate_frequencies(&self, abort: &dyn AbortSignal) -> Result<(), Error> {
        let mut order = (0..self.len()).collect::<Vec<_>>();
        order.sort_unstable_by(|a, b| self.frequencies[*a].total_cmp(&self.frequencies[*b]));
        let error_scale = |slot: usize| {
            self.indices[slot]
                .iter()
                .zip(&self.config.frequencies_hz)
                .map(|(k, f)| f64::from(k.unsigned_abs()) * f)
                .sum::<Value>()
        };
        for (i, pair) in order.windows(2).enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let (a, b) = (pair[0], pair[1]);
            let gap = self.frequencies[b] - self.frequencies[a];
            let tolerance = 8.0 * Value::EPSILON * (error_scale(a) + error_scale(b));
            if !tolerance.is_finite() || gap <= tolerance {
                return Err(Error::InvalidConfig(format!(
                    "tone tuples {:?} and {:?} have indistinguishable physical frequencies; use a common-period HB basis or a nondegenerate tone lattice",
                    self.indices[a], self.indices[b]
                )));
            }
        }
        Ok(())
    }

    fn bin(&self, tuple: &[i32]) -> Option<usize> {
        if tuple.len() != self.dimensions.len() {
            return None;
        }
        let mut bin = 0;
        for (axis, k) in tuple.iter().enumerate() {
            if k.unsigned_abs() as usize > self.config.harmonics[axis] {
                return None;
            }
            let coordinate = i64::from(*k).rem_euclid(self.dimensions[axis] as i64) as usize;
            bin += coordinate * self.strides[axis];
        }
        Some(bin)
    }

    pub fn config(&self) -> &QuasiPeriodicGridConfig {
        &self.config
    }
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }
    pub fn sample_count(&self) -> usize {
        self.sample_count
    }
    pub fn indices(&self) -> &[Vec<i32>] {
        &self.indices
    }
    /// Physical frequency at `tuple` when `anchor` is driven at `frequency_hz`.
    /// Subtract integer coordinates before summing tones, so a low-frequency
    /// output at a nonzero carrier tuple is not lost to floating-point cancellation.
    pub fn frequency_relative_to(
        &self,
        frequency_hz: Value,
        anchor: &[i32],
        tuple: &[i32],
    ) -> Result<Value, Error> {
        if !frequency_hz.is_finite()
            || anchor.len() != self.config.frequencies_hz.len()
            || tuple.len() != anchor.len()
        {
            return Err(Error::InvalidConfig("a translated frequency requires finite frequency and one anchor/target coordinate per tone".into()));
        }
        let mut sum = frequency_hz;
        let mut correction = 0.0;
        for ((&a, &b), tone) in anchor.iter().zip(tuple).zip(&self.config.frequencies_hz) {
            crate::numerics::compensated_add(
                &mut sum,
                &mut correction,
                (i64::from(b) - i64::from(a)) as Value * tone,
            );
        }
        let frequency = sum + correction;
        if !frequency.is_finite() || !(std::f64::consts::TAU * frequency).is_finite() {
            return Err(Error::InvalidConfig(
                "a translated frequency or angular frequency exceeds the finite range".into(),
            ));
        }
        Ok(frequency)
    }

    pub fn frequencies_hz(&self) -> &[Value] {
        &self.frequencies
    }
    pub fn len(&self) -> usize {
        self.indices.len()
    }
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }
    pub fn dc_index(&self) -> usize {
        self.len() / 2
    }
    pub fn index_of(&self, tuple: &[i32]) -> Option<usize> {
        self.indices
            .binary_search_by(|entry| entry.as_slice().cmp(tuple))
            .ok()
    }
    pub(super) fn bins(&self) -> &[usize] {
        &self.bins
    }

    /// Independent phase angles of a collocation sample, in authored tone order.
    /// These coordinates are not samples of a fabricated common time period.
    pub fn phases(&self, sample: usize) -> Option<Vec<Value>> {
        (sample < self.sample_count).then(|| {
            self.dimensions
                .iter()
                .zip(&self.strides)
                .map(|(size, stride)| {
                    std::f64::consts::TAU * ((sample / stride) % size) as Value / *size as Value
                })
                .collect()
        })
    }

    /// Physical derivative along the trajectory θ_j(t)=2π f_j t+θ_j(0).
    pub fn differentiate_with_abort(
        &self,
        spectrum: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        check_abort(abort)?;
        if spectrum.len() != self.len() {
            return Err(Error::InvalidConfig(
                "spectrum length differs from the tone lattice".into(),
            ));
        }
        let mut result = Vec::with_capacity(self.len());
        for (i, (value, frequency)) in spectrum.iter().zip(&self.frequencies).enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let derivative = Complex64::new(0.0, std::f64::consts::TAU * frequency) * value;
            if !finite(derivative) {
                return Err(Error::Numerical(
                    "spectral derivative exceeds the finite range".into(),
                ));
            }
            result.push(derivative);
        }
        Ok(result)
    }
}
