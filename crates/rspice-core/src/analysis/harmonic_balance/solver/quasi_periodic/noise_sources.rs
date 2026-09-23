//! Registered compact-device noise sampled on independent phases, not an HB period.
use super::*;
use crate::analysis::quasi_periodic::{QuasiPeriodicNoiseSpectrum, QuasiPeriodicTransform};
use crate::numerics::scaled_noise::scale_complex_component_exactly;
use crate::{ResourceKind, ResourceLimitError};

fn abort_if_requested(abort: &dyn AbortSignal) -> Result<(), Error> {
    if abort.is_aborted() {
        Err(Error::Aborted)
    } else {
        Ok(())
    }
}
fn noise_error(message: impl Into<String>) -> Error {
    Error::InvalidCircuit(format!("QPNOISE: {}", message.into()))
}
fn budget(values: usize, limits: &ResourceLimits) -> Result<(), Error> {
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        values,
        limits.max_result_values.min(32_000_000),
    )?;
    Ok(())
}

impl HbSolver {
    /// Collect white noise from registered compact diodes, FETs and switches.
    /// Uses the same physical intensity laws, instance temperatures and dialect
    /// constants as PNOISE, sampled over the full Cartesian phase grid.
    /// The engine separately supplies resistor and native-device mechanisms.
    pub fn quasi_periodic_device_noise_sources_with_abort(
        &self,
        grid: Arc<QuasiPeriodicGrid>,
        orbit: &[Vec<Complex64>],
        temperature: Value,
        constants: crate::analysis::noise::NoisePhysicalConstants,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<QuasiPeriodicNoiseSource>, Error> {
        abort_if_requested(abort)?;
        self.validate_quasi_periodic_response()?;
        if !temperature.is_finite()
            || temperature <= 0.0
            || !constants.boltzmann.is_finite()
            || constants.boltzmann <= 0.0
            || !constants.electron_charge.is_finite()
            || constants.electron_charge <= 0.0
        {
            return Err(noise_error(
                "absolute temperature and physical constants must be finite and positive",
            ));
        }
        if self.nonlinear_device_names.len() != self.nonlinear_devices.len()
            || self.nonlinear_noise_temperatures.len() != self.nonlinear_devices.len()
        {
            return Err(noise_error(
                "registered device names and noise temperatures are misaligned",
            ));
        }
        if orbit.len() != self.unknowns() {
            return Err(noise_error(
                "noise orbit differs from the complete MNA basis",
            ));
        }
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            self.unknowns().saturating_mul(grid.len()),
            limits.max_matrix_unknowns,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            grid.sample_count()
                .saturating_mul(self.nonlinear_devices.len().max(1)),
            limits.max_analysis_points,
        )?;
        let mut retained = self
            .unknowns()
            .saturating_mul(
                grid.len()
                    .saturating_mul(2)
                    .saturating_add(grid.sample_count())
                    .saturating_add(1),
            )
            .saturating_add(grid.sample_count().saturating_mul(8));
        budget(retained, limits)?;
        let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?;
        let waves = orbit
            .iter()
            .map(|row| transform.to_real_samples_with_abort(row, abort))
            .collect::<Result<Vec<_>, _>>()?;
        let mut voltages = vec![0.0; self.num_nodes];
        let mut sources = Vec::new();
        for (index, device) in self.nonlinear_devices.iter().enumerate() {
            abort_if_requested(abort)?;
            let name = &self.nonlinear_device_names[index];
            if name.trim().is_empty() {
                return Err(noise_error("registered noise device has an empty name"));
            }
            let source_temperature = self.nonlinear_noise_temperatures[index].resolve(temperature);
            if !source_temperature.is_finite() || source_temperature <= 0.0 {
                return Err(noise_error(format!(
                    "source '{name}' absolute temperature must be finite and positive"
                )));
            }
            let branches = device.noise_branches();
            let samples = grid.sample_count();
            // Temporary scaled waveforms plus normalized result and branch metadata.
            budget(
                retained.saturating_add(
                    branches
                        .len()
                        .saturating_mul(samples.saturating_mul(3).saturating_add(8)),
                ),
                limits,
            )?;
            let mut intensities = vec![vec![ScaledNonnegative::ZERO; samples]; branches.len()];
            for phase in 0..samples {
                abort_if_requested(abort)?;
                for (row, value) in voltages.iter_mut().enumerate() {
                    if row.is_multiple_of(256) {
                        abort_if_requested(abort)?;
                    }
                    *value = waves[row][phase];
                }
                // The voltage slice excludes MNA branches: the legacy ground
                // sentinel is num_nodes and must never become branch current.
                let values = device
                    .noise_intensities(
                        &voltages,
                        source_temperature,
                        constants.electron_charge,
                        constants.boltzmann,
                    )
                    .map_err(|error| {
                        noise_error(format!("source '{name}' at phase {phase}: {error}"))
                    })?;
                if values.len() != branches.len() {
                    return Err(noise_error(format!(
                        "source '{name}' has inconsistent mechanism count"
                    )));
                }
                for (wave, value) in intensities.iter_mut().zip(values) {
                    wave[phase] = value;
                }
            }
            for (branch, ((p, q), wave)) in branches.into_iter().zip(intensities).enumerate() {
                if p == q || (p >= self.num_nodes && q >= self.num_nodes) {
                    continue;
                }
                let Some((density, binary_scale_exponent)) = normalized_intensity(&wave, abort)?
                else {
                    continue;
                };
                let mut injections = Vec::new();
                if p < self.num_nodes {
                    injections.push((p, Complex64::ONE));
                }
                if q < self.num_nodes {
                    injections.push((q, -Complex64::ONE));
                }
                retained = retained.saturating_add(density.len()).saturating_add(8);
                sources.push(QuasiPeriodicNoiseSource {
                    name: format!("{name} {}", device.noise_branch_label(branch)),
                    injections,
                    spectrum: QuasiPeriodicNoiseSpectrum::White {
                        density,
                        binary_scale_exponent,
                    },
                });
            }
        }
        abort_if_requested(abort)?;
        Ok(sources)
    }
    /// Visit native physical probes on every independent phase. The callback
    /// receives the remaining value budget after the live real-orbit workspace.
    pub(crate) fn visit_quasi_periodic_native_noise_samples_with_abort(
        &mut self,
        grid: Arc<QuasiPeriodicGrid>,
        orbit: &[Vec<Complex64>],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
        mut visit: impl FnMut(
            usize,
            usize,
            (&[crate::device::Bjt], &[crate::device::Bsim3v3Device]),
            &[Value],
            &ResourceLimits,
        ) -> Result<(), Error>,
    ) -> Result<(), Error> {
        abort_if_requested(abort)?;
        if self.native_bjts.is_empty() && self.native_bsim3.is_empty() {
            return Ok(());
        }
        self.validate_quasi_periodic_response()?;
        if orbit.len() != self.unknowns() {
            return Err(noise_error(
                "native noise orbit differs from complete MNA basis",
            ));
        }
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            self.unknowns().saturating_mul(grid.len()),
            limits.max_matrix_unknowns,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            grid.sample_count().saturating_mul(
                self.native_bjts
                    .len()
                    .saturating_add(self.native_bsim3.len()),
            ),
            limits.max_analysis_points,
        )?;
        let resident = self
            .unknowns()
            .saturating_mul(
                grid.len()
                    .saturating_mul(2)
                    .saturating_add(grid.sample_count())
                    .saturating_add(1),
            )
            .saturating_add(grid.sample_count().saturating_mul(8));
        budget(resident, limits)?;
        let mut remaining = *limits;
        remaining.max_result_values = limits
            .max_result_values
            .min(32_000_000)
            .saturating_sub(resident);
        let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?;
        let waves = orbit
            .iter()
            .map(|row| transform.to_real_samples_with_abort(row, abort))
            .collect::<Result<Vec<_>, _>>()?;
        let mut solution = vec![0.0; self.unknowns()];
        for phase in 0..grid.sample_count() {
            abort_if_requested(abort)?;
            for (i, (value, wave)) in solution.iter_mut().zip(&waves).enumerate() {
                if i.is_multiple_of(256) {
                    abort_if_requested(abort)?;
                }
                *value = wave[phase];
            }
            for bjt in &mut self.native_bjts {
                abort_if_requested(abort)?;
                bjt.update_mna_static_probe(&solution);
            }
            for device in &mut self.native_bsim3 {
                abort_if_requested(abort)?;
                device
                    .update_periodic_noise_probe(&solution)
                    .map_err(noise_error)?;
            }
            visit(
                phase,
                grid.sample_count(),
                (&self.native_bjts, &self.native_bsim3),
                &solution,
                &remaining,
            )?;
        }
        abort_if_requested(abort)?;
        Ok(())
    }
}

fn normalized_intensity(
    wave: &[ScaledNonnegative],
    abort: &dyn AbortSignal,
) -> Result<Option<(Vec<Value>, i32)>, Error> {
    let mut exponent = None;
    for (i, sample) in wave.iter().enumerate() {
        if i.is_multiple_of(256) {
            abort_if_requested(abort)?;
        }
        if sample.mantissa == 0.0 {
            continue;
        }
        if !(1.0..2.0).contains(&sample.mantissa) {
            return Err(noise_error("nonnegative intensity mantissa is invalid"));
        }
        exponent = Some(exponent.map_or(sample.exponent, |e: i32| e.max(sample.exponent)));
    }
    let Some(exponent) = exponent else {
        return Ok(None);
    };
    let values = wave
        .iter()
        .enumerate()
        .map(|(i, sample)| {
            if i.is_multiple_of(256) {
                abort_if_requested(abort)?;
            }
            if sample.mantissa == 0.0 {
                return Ok(0.0);
            }
            let shift = sample
                .exponent
                .checked_sub(exponent)
                .ok_or_else(|| noise_error("intensity scale exceeds signed range"))?;
            scale_complex_component_exactly(sample.mantissa, shift).map_err(noise_error)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Some((values, exponent)))
}
