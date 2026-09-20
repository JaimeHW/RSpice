//! Noise parameters for a selected pair of frequency-converting wave channels.
//!
//! Unselected channels remain matched, with independently configured thermal
//! noise. The selected output load is noiseless. SSB optimization varies only
//! the selected input-channel reflection; other terminations remain fixed.

use super::{SMatrix, TwoPortNoise};
use crate::abort_signal::AbortSignal;
use crate::{Complex64, Value};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicPortNoiseReference {
    /// One-based wave-port index; mixed-mode callers use d1,c1,d2,c2,... .
    pub input_port: usize,
    pub output_port: usize,
    pub input_sideband: i32,
    pub output_sideband: i32,
    pub reference_temperature_kelvin: Value,
    /// All unselected channels have this matched termination temperature.
    /// Zero disables their noise; it does not change their impedance.
    pub termination_temperature_kelvin: Value,
    /// Optional second desired input band at the same wave port. Its source
    /// temperature is Tref; DSB gain is the sum of the two signal power gains.
    pub image_sideband: Option<i32>,
}

impl Default for PeriodicPortNoiseReference {
    fn default() -> Self {
        Self {
            input_port: 1,
            output_port: 2,
            input_sideband: 0,
            output_sideband: 0,
            reference_temperature_kelvin: 290.0,
            termination_temperature_kelvin: 290.0,
            image_sideband: None,
        }
    }
}

impl PeriodicPortNoiseReference {
    pub fn validate(&self, ports: Option<usize>, min: i32, max: i32) -> Result<(), String> {
        if min > max {
            return Err("noise sideband window is empty".into());
        }
        for (label, port, band) in [
            ("input", self.input_port, self.input_sideband),
            ("output", self.output_port, self.output_sideband),
        ] {
            if port == 0 || ports.is_some_and(|count| port > count) {
                return Err(format!(
                    "noise {label} port is outside the resolved wave-port list"
                ));
            }
            if band < min || band > max {
                return Err(format!(
                    "noise {label} sideband is outside the folding window"
                ));
            }
        }
        if self.input_port == self.output_port && self.input_sideband == self.output_sideband {
            return Err("noise input and output must be distinct wave channels".into());
        }
        if !self.reference_temperature_kelvin.is_finite()
            || self.reference_temperature_kelvin <= 0.0
        {
            return Err("noise reference temperature must be finite and positive".into());
        }
        if !self.termination_temperature_kelvin.is_finite()
            || self.termination_temperature_kelvin < 0.0
        {
            return Err("unused-channel noise temperature must be finite and nonnegative".into());
        }
        if let Some(image) = self.image_sideband {
            if image < min || image > max || image == self.input_sideband {
                return Err(
                    "DSB image sideband must be a distinct input band within the folding window"
                        .into(),
                );
            }
            if self.input_port == self.output_port && image == self.output_sideband {
                return Err("DSB image and output must be distinct wave channels".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PeriodicPortNoiseParameters {
    pub single_sideband: TwoPortNoise,
    pub double_sideband_noise_factor: Option<Value>,
}

/// Intrinsic covariance is in W/Hz in the full scattering channel order.
/// Other incident-channel thermal noise is added before reducing to two
/// observations, so image conversion remains part of the SSB numerator.
#[allow(clippy::too_many_arguments)]
pub fn derive_periodic_port_noise_with_abort(
    scattering: &SMatrix,
    intrinsic: &[Vec<Complex64>],
    references: &[Value],
    sideband_min: i32,
    sideband_max: i32,
    request: &PeriodicPortNoiseReference,
    boltzmann: Value,
    abort: &dyn AbortSignal,
) -> Result<PeriodicPortNoiseParameters, super::NetworkError> {
    use super::NetworkError;
    let failure = |message: &str| NetworkError::NumericalFailure(message.into());
    if abort.is_aborted() {
        return Err(NetworkError::Aborted);
    }
    request
        .validate(Some(references.len()), sideband_min, sideband_max)
        .map_err(NetworkError::NumericalFailure)?;
    if !boltzmann.is_finite() || boltzmann <= 0.0 {
        return Err(failure(
            "noise Boltzmann constant must be finite and positive",
        ));
    }
    let bands = usize::try_from(i64::from(sideband_max) - i64::from(sideband_min) + 1)
        .map_err(|_| failure("noise sideband count exceeds this platform"))?;
    let channels = references
        .len()
        .checked_mul(bands)
        .ok_or_else(|| failure("noise channel count overflowed"))?;
    if intrinsic.len() != channels
        || intrinsic.iter().any(|row| row.len() != channels)
        || scattering.num_ports() != channels
    {
        return Err(failure(
            "periodic scattering and noise covariance dimensions differ",
        ));
    }
    if references.iter().any(|z| !z.is_finite() || *z <= 0.0) {
        return Err(failure(
            "periodic noise requires positive finite wave impedances",
        ));
    }
    let channel = |port: usize, band: i32| {
        (port - 1) * bands + usize::try_from(i64::from(band) - i64::from(sideband_min)).unwrap()
    };
    let input = channel(request.input_port, request.input_sideband);
    let output = channel(request.output_port, request.output_sideband);
    let image = request
        .image_sideband
        .map(|band| channel(request.input_port, band));
    let selected = [input, output];
    let thermal = boltzmann * request.reference_temperature_kelvin;
    if !thermal.is_finite() || thermal <= 0.0 {
        return Err(failure("reference noise power is outside the finite range"));
    }
    let mut covariance = [[Complex64::ZERO; 2]; 2];
    for r in 0..2 {
        for c in 0..2 {
            let mut sum = intrinsic[selected[r]][selected[c]];
            let mut correction = Complex64::ZERO;
            for j in 0..channels {
                if j.is_multiple_of(256) && abort.is_aborted() {
                    return Err(NetworkError::Aborted);
                }
                if j == input || j == output {
                    continue;
                }
                let temperature = if image == Some(j) {
                    request.reference_temperature_kelvin
                } else {
                    request.termination_temperature_kelvin
                };
                let term = scattering.get(selected[r] + 1, j + 1)
                    * scattering.get(selected[c] + 1, j + 1).conj()
                    * (boltzmann * temperature);
                crate::numerics::compensated_add(&mut sum.re, &mut correction.re, term.re);
                crate::numerics::compensated_add(&mut sum.im, &mut correction.im, term.im);
            }
            covariance[r][c] = (sum + correction) / thermal;
        }
    }
    let transfer = scattering.get(output + 1, input + 1);
    let reflection = scattering.get(input + 1, input + 1);
    let single_sideband = wave_noise_parameters(reflection, transfer, covariance,
        references[request.input_port - 1]).ok_or_else(|| failure(
            "selected periodic noise channels do not have finite physical noise parameters (check forward conversion gain)"))?;
    let double_sideband_noise_factor = image
        .map(|image| {
            let image_gain = scattering.get(output + 1, image + 1).norm_sqr();
            let signal_gain = transfer.norm_sqr();
            let sum = signal_gain + image_gain;
            let factor = single_sideband.noise_factor * (signal_gain / sum);
            if !sum.is_finite() || sum <= 0.0 || !factor.is_finite() || factor < 1.0 - 1e-10 {
                return Err(failure("DSB noise factor has no finite physical solution"));
            }
            Ok(factor.max(1.0))
        })
        .transpose()?;
    Ok(PeriodicPortNoiseParameters {
        single_sideband,
        double_sideband_noise_factor,
    })
}

/// Normalize by the selected forward wave gain. For source reflection Γ,
/// F(Γ)=1+(a+b|Γ|²+2 Re[d Γ])/(1-|Γ|²). Optimizing this quadratic
/// avoids an admittance inverse, including ideal noiseless through networks.
fn wave_noise_parameters(
    s11: Complex64,
    s21: Complex64,
    c: [[Complex64; 2]; 2],
    z0: Value,
) -> Option<TwoPortNoise> {
    let finite = |v: Complex64| v.re.is_finite() && v.im.is_finite();
    if !finite(s11) || !finite(s21) || c.iter().flatten().any(|v| !finite(*v)) {
        return None;
    }
    let gain = s21.norm_sqr();
    if !gain.is_finite() || gain <= 0.0 {
        return None;
    }
    let scale = c.iter().flatten().map(|v| v.norm()).fold(0.0, Value::max);
    let tolerance = 512.0 * Value::EPSILON * scale;
    if !tolerance.is_finite()
        || c[0][0].im.abs() > tolerance
        || c[1][1].im.abs() > tolerance
        || (c[0][1] - c[1][0].conj()).norm() > tolerance
        || c[0][0].re < -tolerance
        || c[1][1].re < -tolerance
    {
        return None;
    }
    let a = c[1][1].re.max(0.0) / gain;
    let correlation = c[0][1] / s21.conj();
    let mut b = c[0][0].re + s11.norm_sqr() * a - 2.0 * (s11.conj() * correlation).re;
    let d = correlation - s11 * a;
    let bound = (c[0][0].re.abs() + s11.norm_sqr() * a + 2.0 * (s11.conj() * correlation).norm())
        * 4096.0
        * Value::EPSILON;
    if !a.is_finite() || !b.is_finite() || !finite(d) || !bound.is_finite() || b < -bound {
        return None;
    }
    b = b.max(0.0);
    let total = a + b;
    if !total.is_finite() {
        return None;
    }
    if total == 0.0 {
        return (d == Complex64::ZERO).then_some(TwoPortNoise {
            noise_resistance: 0.0,
            noise_factor: 1.0,
            minimum_noise_factor: 1.0,
            optimum_source_reflection: Complex64::ZERO,
            valid: true,
        });
    }
    let ratio = 2.0 * (d.norm() / total);
    if !ratio.is_finite() || ratio > 1.0 + 4096.0 * Value::EPSILON {
        return None;
    }
    let root = total * ((1.0 - ratio.min(1.0)) * (1.0 + ratio.min(1.0))).sqrt();
    let optimum = -d.conj() / (0.5 * total + 0.5 * root);
    let minimum = 1.0 + 0.5 * (a - b + root);
    let factor = 1.0 + a;
    let resistance = z0 * 0.25 * (a + b - 2.0 * d.re).max(0.0);
    if !finite(optimum)
        || !minimum.is_finite()
        || !factor.is_finite()
        || !resistance.is_finite()
        || minimum < 1.0 - 1e-10
        || factor < minimum - 1e-10
    {
        return None;
    }
    Some(TwoPortNoise {
        noise_resistance: resistance,
        noise_factor: factor,
        minimum_noise_factor: minimum.max(1.0),
        optimum_source_reflection: optimum,
        valid: true,
    })
}

#[cfg(test)]
mod tests;
