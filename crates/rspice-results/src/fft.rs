//! FFT formats, retained spectrum evidence, and exact coefficient storage.

#[cfg(feature = "engine-evidence")]
pub mod data;
#[cfg(feature = "engine-evidence")]
pub mod pipeline;
#[cfg(feature = "engine-evidence")]
pub mod recorded;
#[cfg(feature = "engine-evidence")]
pub mod spectrum;
pub mod window;

/// Positive doubles split into a mantissa in `[1, 2)` and a binary exponent,
/// so spectrum scale products and ratios form without intermediate overflow
/// or underflow and round once, to the nearest double, at the end.
#[cfg(feature = "engine-evidence")]
mod binary_parts {
    pub(super) fn positive_binary_parts(value: f64) -> Option<(f64, i32)> {
        const FRACTION_MASK: u64 = (1_u64 << 52) - 1;
        const EXPONENT_ONE: u64 = 1023_u64 << 52;

        if !value.is_finite() || value <= 0.0 {
            return None;
        }
        let bits = value.to_bits();
        let raw_exponent = ((bits >> 52) & 0x7ff) as i32;
        let fraction = bits & FRACTION_MASK;
        if raw_exponent != 0 {
            return Some((f64::from_bits(EXPONENT_ONE | fraction), raw_exponent - 1023));
        }

        let highest_bit = 63_i32 - fraction.leading_zeros() as i32;
        let shift = u32::try_from(52_i32 - highest_bit).ok()?;
        let normalized_significand = fraction.checked_shl(shift)?;
        Some((
            f64::from_bits(EXPONENT_ONE | (normalized_significand & FRACTION_MASK)),
            highest_bit - 1074,
        ))
    }

    pub(super) fn normalize_binary_parts(mantissa: &mut f64, exponent: &mut i32) -> Option<()> {
        if !mantissa.is_finite() || *mantissa <= 0.0 {
            return None;
        }
        while *mantissa >= 2.0 {
            *mantissa *= 0.5;
            *exponent = exponent.checked_add(1)?;
        }
        while *mantissa < 1.0 {
            *mantissa *= 2.0;
            *exponent = exponent.checked_sub(1)?;
        }
        Some(())
    }

    pub(super) fn materialize_binary_parts(mantissa: f64, exponent: i32) -> Option<f64> {
        const FRACTION_MASK: u64 = (1_u64 << 52) - 1;
        const MAX_MANTISSA: f64 = f64::from_bits((1023_u64 << 52) | FRACTION_MASK);

        if !(1.0..2.0).contains(&mantissa) || !(-1075..=1023).contains(&exponent) {
            return None;
        }
        if exponent == 1023 && mantissa > MAX_MANTISSA {
            return None;
        }

        let fraction = mantissa.to_bits() & FRACTION_MASK;
        if exponent >= -1022 {
            let raw_exponent = u64::try_from(exponent + 1023).ok()?;
            let value = f64::from_bits((raw_exponent << 52) | fraction);
            return (value.is_finite() && value > 0.0).then_some(value);
        }

        let significand = (1_u64 << 52) | fraction;
        let shift = u32::try_from(-1022 - exponent).ok()?;
        let truncated = significand >> shift;
        let remainder_mask = (1_u64 << shift) - 1;
        let remainder = significand & remainder_mask;
        let halfway = 1_u64 << (shift - 1);
        let round_up =
            remainder > halfway || (remainder == halfway && !truncated.is_multiple_of(2));
        let rounded = truncated.checked_add(u64::from(round_up))?;
        let value = f64::from_bits(rounded);
        (value > 0.0).then_some(value)
    }
}

use serde::{Deserialize, Serialize};

/// Effective `.FFT FORMAT`, as the engine resolved it for this spectrum.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FftSpectrumFormatEvidence {
    #[default]
    Normalized,
    Unnormalized,
}

impl FftSpectrumFormatEvidence {
    /// The keyword `.FFT FORMAT=` spells this with.
    #[must_use]
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Normalized => "NORM",
            Self::Unnormalized => "UNORM",
        }
    }
}

/// Convert the engine's resolved FFT format to its portable result representation.
#[cfg(feature = "engine-evidence")]
#[must_use]
pub const fn fft_format_from_core(
    format: rspice_core::netlist::FftFormat,
) -> FftSpectrumFormatEvidence {
    match format {
        rspice_core::netlist::FftFormat::Normalized => FftSpectrumFormatEvidence::Normalized,
        rspice_core::netlist::FftFormat::Unnormalized => FftSpectrumFormatEvidence::Unnormalized,
    }
}

/// Convert retained FFT format evidence to the engine's unit-policy vocabulary.
#[cfg(feature = "engine-evidence")]
#[must_use]
pub const fn fft_format_to_core(
    format: FftSpectrumFormatEvidence,
) -> rspice_core::netlist::FftFormat {
    match format {
        FftSpectrumFormatEvidence::Normalized => rspice_core::netlist::FftFormat::Normalized,
        FftSpectrumFormatEvidence::Unnormalized => rspice_core::netlist::FftFormat::Unnormalized,
    }
}

/// FFT input fidelity mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InputFidelity {
    /// Preserve source detail for analysis-grade spectra (default).
    #[default]
    Reference,
    /// Enforce capped point count for faster interaction on large datasets.
    Interactive,
}

#[cfg(feature = "engine-evidence")]
impl InputFidelity {
    /// Pipeline policy for this fidelity.
    pub fn input_policy(&self) -> pipeline::FftInputPolicy {
        match self {
            Self::Reference => pipeline::FftInputPolicy::reference(),
            Self::Interactive => pipeline::FftInputPolicy::interactive_default(),
        }
    }
}
