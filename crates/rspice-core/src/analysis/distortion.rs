//! Third-order small-signal Volterra distortion result types.

use crate::Value;
use crate::analysis::AcResult;

/// Spectral product calculated by a `.DISTO` analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DistortionProduct {
    /// Second harmonic at `2*F1`.
    SecondHarmonic,
    /// Third harmonic at `3*F1`.
    ThirdHarmonic,
    /// Second-order sum product at `F1+F2`.
    Sum,
    /// Second-order difference product at `F1-F2`.
    Difference,
    /// Third-order intermodulation product at `2*F1-F2`.
    ThirdOrderDifference,
    /// Second harmonic of the fixed second tone at `2*F2`.
    SecondHarmonicF2,
    /// Third-order intermodulation product at `abs(2*F2-F1)`.
    ThirdOrderDifferenceF2,
}

impl DistortionProduct {
    /// Stable, SPICE-oriented product label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SecondHarmonic => "2f1",
            Self::ThirdHarmonic => "3f1",
            Self::Sum => "f1+f2",
            Self::Difference => "f1-f2",
            Self::ThirdOrderDifference => "2f1-f2",
            Self::SecondHarmonicF2 => "2f2",
            Self::ThirdOrderDifferenceF2 => "2f2-f1",
        }
    }

    /// Volterra order of this product.
    pub const fn order(self) -> usize {
        match self {
            Self::SecondHarmonic | Self::SecondHarmonicF2 | Self::Sum | Self::Difference => 2,
            Self::ThirdHarmonic | Self::ThirdOrderDifference | Self::ThirdOrderDifferenceF2 => 3,
        }
    }

    /// Complete product population returned by the core for one tone mode.
    /// Consumers use this inventory to retain every computed product.
    pub const fn for_mode(two_tone: bool) -> &'static [Self] {
        if two_tone {
            &[
                Self::Sum,
                Self::Difference,
                Self::ThirdOrderDifference,
                Self::SecondHarmonic,
                Self::SecondHarmonicF2,
                Self::ThirdOrderDifferenceF2,
            ]
        } else {
            &[Self::SecondHarmonic, Self::ThirdHarmonic]
        }
    }
}

/// Complex circuit response for one distortion product.
#[derive(Debug, Clone)]
pub struct DistortionProductResult {
    pub product: DistortionProduct,
    /// Actual sinusoidal peak phasors, not an unscaled internal Volterra
    /// kernel. Frequencies are nonnegative; a negative signed product is
    /// conjugated onto its positive frequency. At zero frequency the value
    /// is the real DC contribution. Coincident products remain separate
    /// order/tone contributions, not a measurement of their summed spectrum.
    pub response: AcResult,
}

/// All first-order and nonlinear products for one swept F1 frequency.
#[derive(Debug, Clone)]
pub struct DistortionPointResult {
    /// Actual first-order F1 response to all `DISTOF1` source annotations.
    pub fundamental_f1: AcResult,
    /// Actual first-order F2 response in two-tone mode.
    pub fundamental_f2: Option<AcResult>,
    pub products: Vec<DistortionProductResult>,
}

impl DistortionPointResult {
    /// Find one product without depending on vector ordering.
    pub fn product(&self, product: DistortionProduct) -> Option<&DistortionProductResult> {
        self.products.iter().find(|value| value.product == product)
    }

    /// Voltage distortion ratio at a matrix-aligned, one-based node.
    /// Ground returns zero. A zero fundamental produces infinity for a
    /// nonzero product and zero for a zero product.
    pub fn voltage_ratio(&self, product: DistortionProduct, node: usize) -> Option<Value> {
        let product_value = self.product(product)?;
        if node == 0 {
            return Some(0.0);
        }
        let numerator = product_value.response.voltages.get(node - 1)?.norm();
        let denominator = self.fundamental_f1.voltages.get(node - 1)?.norm();
        Some(if denominator == 0.0 {
            if numerator == 0.0 {
                0.0
            } else {
                Value::INFINITY
            }
        } else {
            numerator / denominator
        })
    }
}

/// Complete harmonic or two-tone `.DISTO` sweep.
#[derive(Debug, Clone)]
pub struct DistortionAnalysisResult {
    /// `None` for harmonic mode; otherwise the fixed `F2/F1_start` ratio.
    pub f2_over_f1: Option<Value>,
    pub points: Vec<DistortionPointResult>,
}

impl DistortionAnalysisResult {
    /// Numeric response storage, including each tone's physical frequency.
    pub fn retained_value_count(&self) -> usize {
        self.points.iter().fold(0usize, |count, point| {
            std::iter::once(&point.fundamental_f1)
                .chain(point.fundamental_f2.iter())
                .chain(point.products.iter().map(|product| &product.response))
                .fold(count, |count, response| {
                    count.saturating_add(
                        response
                            .voltages
                            .len()
                            .saturating_add(response.currents.len())
                            .saturating_mul(2)
                            .saturating_add(1),
                    )
                })
        })
    }

    pub fn is_two_tone(&self) -> bool {
        self.f2_over_f1.is_some()
    }
}
