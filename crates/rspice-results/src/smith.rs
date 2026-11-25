//! How a Smith chart reads what it draws: the grid it rules and how a complex
//! response that is not a network coefficient lands on it. The sheet and its
//! print take both from here, so a printed chart is read as the sheet is.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};

/// The reference impedance a Smith chart opens on, in ohms.
pub const DEFAULT_REFERENCE_OHM: f64 = 50.0;

/// Which coordinate grid a Smith chart rules.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SmithGrid {
    /// Constant resistance and reactance.
    #[default]
    Impedance,
    /// Constant conductance and susceptance.
    Admittance,
    /// Both, the admittance grid drawn in its own ink.
    ImpedanceAdmittance,
}

impl SmithGrid {
    /// Segmented-control labels, in [`Self::index`] order.
    pub const OPTIONS: [&'static str; 3] = ["Z", "Y", "Z+Y"];

    pub const fn index(self) -> usize {
        match self {
            Self::Impedance => 0,
            Self::Admittance => 1,
            Self::ImpedanceAdmittance => 2,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Admittance,
            2 => Self::ImpedanceAdmittance,
            _ => Self::Impedance,
        }
    }

    pub const fn draws_impedance(self) -> bool {
        matches!(self, Self::Impedance | Self::ImpedanceAdmittance)
    }

    pub const fn draws_admittance(self) -> bool {
        matches!(self, Self::Admittance | Self::ImpedanceAdmittance)
    }
}

/// How a complex response that is not a network coefficient lands on the
/// chart.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResponseReading {
    /// The response is a reflection coefficient and is drawn as it is.
    #[default]
    Reflection,
    /// The response is an impedance in ohms: Γ = (Z − Z₀)/(Z + Z₀).
    Impedance,
    /// The response is an admittance in siemens: Γ = (1 − Y·Z₀)/(1 + Y·Z₀).
    Admittance,
}

impl ResponseReading {
    /// Segmented-control labels, in [`Self::index`] order.
    pub const OPTIONS: [&'static str; 3] = ["Γ", "Z", "Y"];

    pub const fn index(self) -> usize {
        match self {
            Self::Reflection => 0,
            Self::Impedance => 1,
            Self::Admittance => 2,
        }
    }

    pub const fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Impedance,
            2 => Self::Admittance,
            _ => Self::Reflection,
        }
    }

    /// The reflection coefficient a response sample stands for against
    /// `reference_ohm`. A response at the chart's singular point (Z = −Z₀)
    /// is not finite and is not drawn.
    pub fn reflection(self, value: Complex64, reference_ohm: f64) -> Complex64 {
        match self {
            Self::Reflection => value,
            Self::Impedance => (value - reference_ohm) / (value + reference_ohm),
            Self::Admittance => {
                let normalized = value * reference_ohm;
                (1.0 - normalized) / (1.0 + normalized)
            }
        }
    }

    /// [`Self::reflection`] of a sample given by its real and imaginary
    /// parts, as a print holds it.
    pub fn reflection_of(self, (real, imaginary): (f64, f64), reference_ohm: f64) -> (f64, f64) {
        let gamma = self.reflection(Complex64::new(real, imaginary), reference_ohm);
        (gamma.re, gamma.im)
    }
}
