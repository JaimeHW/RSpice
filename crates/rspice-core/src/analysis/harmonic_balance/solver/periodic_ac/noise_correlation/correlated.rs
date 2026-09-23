//! Coherent multiport injection at each signed stationary-noise frequency.
use super::*;
use crate::analysis::noise::{
    Bsim4CorrelatedNoiseWaveform,
    correlated::{coherent_sum, induced_gate_factor},
};
use std::collections::BTreeSet;

fn error(reason: impl std::fmt::Display) -> HbError {
    HbError::InvalidCircuit(format!("pnoise correlated source: {reason}"))
}

fn coefficient(spectrum: &[Complex64], harmonic: i64) -> Complex64 {
    let value = spectrum
        .get(harmonic.unsigned_abs() as usize)
        .copied()
        .unwrap_or(Complex64::ZERO);
    if harmonic < 0 { value.conj() } else { value }
}

impl HbSolver {
    fn correlated_spectrum(&mut self, samples: &[Value]) -> Result<Vec<Complex64>, HbError> {
        use crate::numerics::scaled_noise::scale_complex_component_exactly as scale;
        let maximum = samples.iter().map(|v| v.abs()).fold(0.0, Value::max);
        if maximum == 0.0 {
            return Ok(vec![]);
        }
        let exponent = libm::ilogb(maximum);
        let normalized = samples
            .iter()
            .map(|&v| scale(v, -exponent).map_err(error))
            .collect::<Result<Vec<_>, _>>()?;
        self.fft
            .complete_noise_spectrum(&normalized)
            .into_iter()
            .map(|v| {
                Ok(Complex64::new(
                    scale(v.re, exponent).map_err(error)?,
                    scale(v.im, exponent).map_err(error)?,
                ))
            })
            .collect()
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "explicit adjoint, source, frequency and resource inputs"
    )]
    pub(super) fn fold_correlated_noise(
        &mut self,
        waveform: &Bsim4CorrelatedNoiseWaveform,
        adjoints: &[Complex64],
        channels: usize,
        size: usize,
        window: PeriodicSidebandWindow,
        limits: &crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Vec<ScaledComplex>>, HbError> {
        let bands = (i64::from(window.sideband_max) - i64::from(window.sideband_min) + 1) as usize;
        let harmonics = waveform.samples.len() / 2;
        let mut ports = BTreeSet::new();
        for sample in &waveform.samples {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            for port in [sample.drain_port, sample.gate_port] {
                if port[0] != port[1] {
                    ports.insert(port);
                }
            }
        }
        let inputs = bands.saturating_add(2 * harmonics);
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::AnalysisPoints,
            inputs.saturating_mul(bands).saturating_mul(ports.len()),
            limits.max_analysis_points,
        )
        .map_err(error)?;
        let values = inputs
            .saturating_mul(channels.saturating_mul(3).saturating_add(3))
            .saturating_add(
                (harmonics + 1)
                    .saturating_mul(ports.len())
                    .saturating_mul(4),
            )
            .saturating_add(waveform.samples.len().saturating_mul(16))
            .saturating_add(adjoints.len().saturating_mul(2));
        crate::ResourceLimitError::ensure(
            crate::ResourceKind::ResultValues,
            values,
            limits.max_result_values.min(32_000_000),
        )
        .map_err(error)?;
        // The channel amplitude is independent of the input frequency. Its
        // Fourier coefficients are reused for all stationary source bands.
        let mut drain_spectra = Vec::new();
        for &port in &ports {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let samples = waveform
                .samples
                .iter()
                .map(|sample| {
                    if sample.drain_port == port {
                        sample.drain_amplitude
                    } else {
                        0.0
                    }
                })
                .collect::<Vec<_>>();
            drain_spectra.push(self.correlated_spectrum(&samples)?);
        }
        let first = i64::from(window.sideband_min) - harmonics as i64;
        let last = i64::from(window.sideband_max) + harmonics as i64;
        let mut folded = Vec::new();
        for input in first..=last {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let frequency =
                (input as Value).mul_add(self.config.fundamental_freq, window.offset_hz);
            if !frequency.is_finite() {
                return Err(error("translated frequency overflows"));
            }
            let mut gate_spectra = Vec::new();
            for &port in &ports {
                let samples = waveform
                    .samples
                    .iter()
                    .map(|sample| {
                        if sample.gate_port == port {
                            sample.gate_amplitude
                                * induced_gate_factor(frequency, sample.gate_time_constant)
                        } else {
                            0.0
                        }
                    })
                    .collect::<Vec<_>>();
                gate_spectra.push(self.correlated_spectrum(&samples)?);
            }
            let mut transfers = Vec::with_capacity(channels);
            for channel in 0..channels {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                // Sum ports as well as conversion paths before squaring.
                // The quadrature multiplier is applied after conjugating a
                // negative modulation harmonic, never as part of a real FFT.
                let transfer = coherent_sum(|visit| {
                    for (port_index, port) in ports.iter().enumerate() {
                        for band in 0..bands {
                            if band.is_multiple_of(256) && abort.is_aborted() {
                                return Err("aborted");
                            }
                            let harmonic = i64::from(window.sideband_min) + band as i64 - input;
                            if harmonic.unsigned_abs() > harmonics as u64 {
                                continue;
                            }
                            let drain = coefficient(&drain_spectra[port_index], harmonic);
                            let gate = coefficient(&gate_spectra[port_index], harmonic);
                            for (terminal, sign) in [(port[0], 1.0), (port[1], -1.0)] {
                                if terminal >= self.num_nodes {
                                    continue;
                                }
                                let gain = adjoints[channel * size + terminal * bands + band];
                                visit(scaled_complex_product3(
                                    gain,
                                    drain,
                                    Complex64::new(sign, 0.0),
                                    0,
                                )?)?;
                                visit(scaled_complex_product3(
                                    gain,
                                    gate,
                                    Complex64::new(0.0, sign),
                                    0,
                                )?)?;
                            }
                        }
                    }
                    Ok(())
                })
                .map_err(|reason| {
                    if abort.is_aborted() {
                        HbError::Aborted
                    } else {
                        error(reason)
                    }
                })?;
                transfers.push(transfer);
            }
            folded.push(transfers);
        }
        Ok(folded)
    }
}

pub(super) fn covariance(
    folded: &[Vec<ScaledComplex>],
    row: usize,
    column: usize,
    scale: i32,
    abort: &dyn AbortSignal,
) -> Result<Complex64, HbError> {
    let term = |values: &[ScaledComplex]| {
        scaled_flicker_cross_term(values[row], values[column], 1.0, scale, 1.0, 0.0).map_err(error)
    };
    let mut exponent = None;
    for values in folded {
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let value = term(values)?;
        if !value.is_zero() {
            exponent = Some(exponent.map_or(value.exponent, |e: i32| e.max(value.exponent)));
        }
    }
    let Some(exponent) = exponent else {
        return Ok(Complex64::ZERO);
    };
    let mut sum = ScaledComplexAccumulator::new(exponent);
    for values in folded {
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        sum.add(term(values)?).map_err(error)?;
    }
    let (value, _) = sum.finish().map_err(error)?;
    if row == column && (value.re < 0.0 || value.im != 0.0) {
        return Err(error(
            "correlated covariance diagonal is not real and nonnegative",
        ));
    }
    Ok(value)
}
