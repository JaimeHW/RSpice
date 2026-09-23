//! A single, frequency-shaped random process shared by two physical ports.
//!
//! BSIM4 TNOIMOD=2 assigns a real channel amplitude and a quadrature induced
//! gate amplitude to the same process. Signed frequencies conjugate the
//! quadrature amplitude. Ports belong to each bias sample because reversing
//! the channel must not create a new independent random source.

use crate::Value;
use crate::numerics::scaled_noise::{ScaledComplex, ScaledComplexAccumulator};

/// Frozen-bias factorization of one BSIM4 correlated channel/gate mechanism.
/// Amplitudes share the waveform's binary power scale. Terminal indices are
/// zero-based MNA coordinates; `usize::MAX` denotes ground.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bsim4CorrelatedNoiseSample {
    #[serde(with = "portable_port")]
    pub drain_port: [usize; 2],
    #[serde(with = "portable_port")]
    pub gate_port: [usize; 2],
    pub drain_amplitude: Value,
    pub gate_amplitude: Value,
    pub gate_time_constant: Value,
}

// The solver sentinel is platform-sized. Retained evidence uses null for
// ground, so JSON, packed-result identities and exports agree on 32/64-bit
// hosts and do not require JavaScript to preserve an integer above 2^53.
mod portable_port {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub(super) fn serialize<S: Serializer>(
        port: &[usize; 2],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        port.map(|node| (node != usize::MAX).then_some(node))
            .serialize(serializer)
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<[usize; 2], D::Error> {
        let port = <[Option<usize>; 2]>::deserialize(deserializer)?;
        if port.contains(&Some(usize::MAX)) {
            return Err(serde::de::Error::custom(
                "ground must use null, not a platform-sized sentinel",
            ));
        }
        Ok(port.map(|node| node.unwrap_or(usize::MAX)))
    }
}

impl Bsim4CorrelatedNoiseSample {
    pub(crate) const ZERO: Self = Self {
        drain_port: [usize::MAX; 2],
        gate_port: [usize::MAX; 2],
        drain_amplitude: 0.0,
        gate_amplitude: 0.0,
        gate_time_constant: 0.0,
    };

    pub(crate) fn validate(&self, coordinates: usize) -> Result<(), &'static str> {
        if self
            .drain_port
            .iter()
            .chain(&self.gate_port)
            .any(|&node| node != usize::MAX && node >= coordinates)
        {
            return Err("correlated noise terminal is outside the MNA basis");
        }
        if [
            self.drain_amplitude,
            self.gate_amplitude,
            self.gate_time_constant,
        ]
        .iter()
        .any(|v| !v.is_finite() || *v < 0.0)
        {
            return Err(
                "correlated noise requires finite nonnegative amplitudes and time constant",
            );
        }
        Ok(())
    }
}

/// Samples on the producer's uniform phase grid. This keeps the complete
/// frozen-bias frequency law, rather than factoring it at one sweep frequency.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bsim4CorrelatedNoiseWaveform {
    pub samples: Vec<Bsim4CorrelatedNoiseSample>,
    /// Multiplies the final covariance, not the individual amplitudes.
    pub binary_scale_exponent: i32,
}

/// |omega*tau| / sqrt(1 + (omega*tau)^2), evaluated without squaring a
/// potentially huge argument. Keeping the sign implements conjugation at
/// negative translated source frequencies, including frequencies below -f0.
pub(crate) fn induced_gate_factor(frequency: Value, tau: Value) -> Value {
    if frequency == 0.0 || tau == 0.0 {
        return 0.0;
    }
    let x = frequency.abs() * tau.abs();
    let magnitude = if x.is_infinite() {
        1.0
    } else {
        x / x.hypot(1.0 / std::f64::consts::TAU)
    };
    magnitude.copysign(frequency)
}

/// Sum all ports and mixing paths before forming noise power. A common
/// binary scale is selected from the terms, so large cancelling transfer
/// paths need not be representable as an unscaled intermediate amplitude.
pub(crate) fn coherent_sum(
    mut visit: impl FnMut(
        &mut dyn FnMut(ScaledComplex) -> Result<(), &'static str>,
    ) -> Result<(), &'static str>,
) -> Result<ScaledComplex, &'static str> {
    let mut exponent = None;
    visit(&mut |term| {
        if !term.is_zero() {
            exponent = Some(exponent.map_or(term.exponent, |e: i32| e.max(term.exponent)));
        }
        Ok(())
    })?;
    let Some(exponent) = exponent else {
        return Ok(ScaledComplex::ZERO);
    };
    let mut sum = ScaledComplexAccumulator::new(exponent);
    visit(&mut |term| sum.add(term))?;
    sum.into_scaled()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsim4_correlated_ports_use_portable_ground_encoding() {
        let sample = Bsim4CorrelatedNoiseSample {
            drain_port: [3, usize::MAX],
            gate_port: [5, 3],
            drain_amplitude: 0.25,
            gate_amplitude: 1.0,
            gate_time_constant: 1e-12,
        };
        let encoded = serde_json::to_value(&sample).unwrap();
        assert_eq!(encoded["drain_port"], serde_json::json!([3, null]));
        assert_eq!(encoded["gate_port"], serde_json::json!([5, 3]));
        assert_eq!(
            serde_json::from_value::<Bsim4CorrelatedNoiseSample>(encoded.clone()).unwrap(),
            sample
        );
        let mut malformed = encoded;
        malformed["drain_port"] = serde_json::json!([3, usize::MAX]);
        assert!(serde_json::from_value::<Bsim4CorrelatedNoiseSample>(malformed).is_err());
    }

    #[test]
    fn induced_gate_shape_retains_quadrature_sign_and_high_frequency_limit() {
        for x in [1e-150, 1e-6, 0.1, 1.0, 10.0, 1e150, 1e200, f64::MAX] {
            let actual = induced_gate_factor(x, 1.0 / std::f64::consts::TAU);
            let expected = if x <= 1.0 {
                x / (1.0 + x * x).sqrt()
            } else {
                1.0 / (1.0 + (1.0 / x).powi(2)).sqrt()
            };
            assert!((actual / expected - 1.0).abs() < 5e-15, "x={x}");
            assert_eq!(
                induced_gate_factor(-x, 1.0 / std::f64::consts::TAU),
                -actual
            );
        }
        assert_eq!(induced_gate_factor(f64::MAX, f64::MAX), 1.0);
        assert_eq!(induced_gate_factor(0.0, f64::MAX), 0.0);
    }
}
