//! Separable multidimensional Fourier transforms on the independent phase grid.
use super::{QuasiPeriodicError as Error, QuasiPeriodicGrid, check_abort, finite, zero_buffer};
use crate::abort_signal::AbortSignal;
use crate::{Complex64, Value};
use rustfft::{Fft, FftPlanner};
use std::sync::Arc;

#[derive(Clone)]
pub struct QuasiPeriodicTransform {
    grid: Arc<QuasiPeriodicGrid>,
    forward: Vec<Arc<dyn Fft<Value>>>,
    inverse: Vec<Arc<dyn Fft<Value>>>,
    line: Vec<Complex64>,
    scratch: Vec<Complex64>,
}

impl QuasiPeriodicTransform {
    pub fn new_with_abort(
        grid: Arc<QuasiPeriodicGrid>,
        abort: &dyn AbortSignal,
    ) -> Result<Self, Error> {
        check_abort(abort)?;
        let mut planner = FftPlanner::new();
        let mut forward = Vec::new();
        let mut inverse = Vec::new();
        let mut scratch = 0;
        for size in grid.dimensions() {
            check_abort(abort)?;
            let fft = planner.plan_fft_forward(*size);
            let ifft = planner.plan_fft_inverse(*size);
            scratch = scratch
                .max(fft.get_inplace_scratch_len())
                .max(ifft.get_inplace_scratch_len());
            forward.push(fft);
            inverse.push(ifft);
        }
        let line = zero_buffer(*grid.dimensions().iter().max().unwrap())?;
        let scratch = zero_buffer(scratch)?;
        check_abort(abort)?;
        Ok(Self {
            grid,
            forward,
            inverse,
            line,
            scratch,
        })
    }

    pub fn grid(&self) -> &QuasiPeriodicGrid {
        &self.grid
    }

    /// Full complex coefficients, with the Fourier-series normalization:
    /// x(θ)=Σ_k X_k exp(i k·θ). Complex translated perturbations are supported.
    pub fn to_samples_with_abort(
        &mut self,
        spectrum: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        check_abort(abort)?;
        if spectrum.len() != self.grid.len() {
            return Err(Error::InvalidConfig(
                "spectrum length differs from the tone lattice".into(),
            ));
        }
        let mut values = zero_buffer(self.grid.sample_count())?;
        for (i, (bin, value)) in self.grid.bins().iter().zip(spectrum).enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if !finite(*value) {
                return Err(Error::Numerical(
                    "spectrum contains a non-finite coefficient".into(),
                ));
            }
            values[*bin] = *value;
        }
        self.transform(&mut values, true, abort)?;
        Ok(values)
    }

    pub fn to_spectrum_with_abort(
        &mut self,
        samples: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        check_abort(abort)?;
        if samples.len() != self.grid.sample_count() {
            return Err(Error::InvalidConfig(
                "sample count differs from the phase grid".into(),
            ));
        }
        let mut values = zero_buffer(samples.len())?;
        // Normalize before each dimension, avoiding unnecessary overflow in
        // intermediate forward sums while preserving the Fourier convention.
        for (i, (target, value)) in values.iter_mut().zip(samples).enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if !finite(*value) {
                return Err(Error::Numerical(
                    "phase samples contain a non-finite value".into(),
                ));
            }
            *target = *value;
        }
        self.transform(&mut values, false, abort)?;
        let mut spectrum = Vec::with_capacity(self.grid.len());
        for (i, bin) in self.grid.bins().iter().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            spectrum.push(values[*bin]);
        }
        Ok(spectrum)
    }

    pub fn to_real_samples_with_abort(
        &mut self,
        spectrum: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        check_abort(abort)?;
        if spectrum.len() != self.grid.len() {
            return Err(Error::InvalidConfig(
                "spectrum length differs from the tone lattice".into(),
            ));
        }
        let mut scale = 0.0_f64;
        for (i, value) in spectrum.iter().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            if !finite(*value) {
                return Err(Error::Numerical(
                    "spectrum contains a non-finite coefficient".into(),
                ));
            }
            scale = scale.max(value.norm());
        }
        for (i, value) in spectrum.iter().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let conjugate = spectrum[spectrum.len() - 1 - i].conj();
            if !finite(*value)
                || !scale.is_finite()
                || (*value - conjugate).norm() > 128.0 * Value::EPSILON * scale
            {
                return Err(Error::InvalidConfig(
                    "real phase samples require conjugate-symmetric tone coefficients".into(),
                ));
            }
        }
        let values = self.to_samples_with_abort(spectrum, abort)?;
        let mut real = Vec::with_capacity(values.len());
        for (i, value) in values.into_iter().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            real.push(value.re);
        }
        Ok(real)
    }

    fn transform(
        &mut self,
        values: &mut [Complex64],
        inverse: bool,
        abort: &dyn AbortSignal,
    ) -> Result<(), Error> {
        // Scale the complete tensor before the separable passes. Per-axis
        // normalization alone would erase a constant subnormal input, while
        // unscaled forward sums can overflow a finite constant near f64::MAX.
        let mut scale = 0.0_f64;
        for (i, value) in values.iter().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            scale = scale.max(value.re.abs()).max(value.im.abs());
        }
        if scale == 0.0 {
            return check_abort(abort);
        }
        for (i, value) in values.iter_mut().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            *value /= scale;
        }
        let mut stride = 1usize;
        for axis in 0..self.grid.dimensions().len() {
            let size = self.grid.dimensions()[axis];
            let block = stride * size;
            let plan = if inverse {
                &self.inverse[axis]
            } else {
                &self.forward[axis]
            };
            let mut lines = 0usize;
            for base in (0..values.len()).step_by(block) {
                for inner in 0..stride {
                    if lines.is_multiple_of(64) {
                        check_abort(abort)?;
                    }
                    lines += 1;
                    for i in 0..size {
                        self.line[i] = values[base + inner + i * stride];
                        if !inverse {
                            self.line[i] /= size as Value;
                        }
                    }
                    plan.process_with_scratch(&mut self.line[..size], &mut self.scratch);
                    for i in 0..size {
                        if !finite(self.line[i]) {
                            return Err(Error::Numerical(
                                "Fourier transform exceeds the finite range".into(),
                            ));
                        }
                        values[base + inner + i * stride] = self.line[i];
                    }
                }
            }
            stride = block;
        }
        for (i, value) in values.iter_mut().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            *value *= scale;
            if !finite(*value) {
                return Err(Error::Numerical(
                    "Fourier transform exceeds the finite range".into(),
                ));
            }
        }
        check_abort(abort)
    }
}
