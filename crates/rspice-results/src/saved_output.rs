//! Policies and source identities retained with saved engineering outputs.

use serde::{Deserialize, Serialize};

#[cfg(feature = "engine-evidence")]
mod expressions;
#[cfg(feature = "engine-evidence")]
pub use expressions::{
    device_current_probe, parse_probe_target, raw_probe_unit, saved_output_references,
    validate_raw_probe,
};

/// Initial presentation intent, independent from whether and how the full
/// precision quantity is retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputDisplayIntent {
    #[default]
    Plot,
    DataBrowserOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputKind {
    RawVoltageOrCurrent,
    DerivedExpression,
    DeviceOperatingPointQuantity,
    NoiseContributor,
    RfPortQuantity,
}

impl SavedOutputKind {
    pub const ALL: [Self; 5] = [
        Self::RawVoltageOrCurrent,
        Self::DerivedExpression,
        Self::DeviceOperatingPointQuantity,
        Self::NoiseContributor,
        Self::RfPortQuantity,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::RawVoltageOrCurrent => "Raw voltage / current",
            Self::DerivedExpression => "Derived expression",
            Self::DeviceOperatingPointQuantity => "Device operating-point quantity",
            Self::NoiseContributor => "Noise contributor",
            Self::RfPortQuantity => "RF port quantity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputPolicy {
    EveryAcceptedPoint,
    SelectedAndFinalPoints,
    OnDemandFromRetainedState,
    FailureDiagnosticsOnly,
}

impl SavedOutputPolicy {
    pub const ALL: [Self; 4] = [
        Self::EveryAcceptedPoint,
        Self::SelectedAndFinalPoints,
        Self::OnDemandFromRetainedState,
        Self::FailureDiagnosticsOnly,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::EveryAcceptedPoint => "Every accepted point",
            Self::SelectedAndFinalPoints => "Selected + final points",
            Self::OnDemandFromRetainedState => "On demand from retained state",
            Self::FailureDiagnosticsOnly => "Failure diagnostics only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputPrecision {
    FullSourcePrecision,
    DisplayCacheWithFullSourcePrecision,
}

impl SavedOutputPrecision {
    pub const ALL: [Self; 2] = [
        Self::FullSourcePrecision,
        Self::DisplayCacheWithFullSourcePrecision,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSourcePrecision => "f64 / complex128",
            Self::DisplayCacheWithFullSourcePrecision => {
                "f32 display cache + full source precision"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedOutputStreaming {
    LivePlotAdaptiveDisplayDecimation,
    StoreOnly,
}

impl SavedOutputStreaming {
    pub const ALL: [Self; 2] = [Self::LivePlotAdaptiveDisplayDecimation, Self::StoreOnly];

    pub const fn label(self) -> &'static str {
        match self {
            Self::LivePlotAdaptiveDisplayDecimation => "Live plot · adaptive display decimation",
            Self::StoreOnly => "Store only",
        }
    }
}

/// Interpretation of complex source signals in an authored expression.
/// Missing fields preserve historical magnitude arithmetic. New expressions
/// opt into rectangular arithmetic; reopening a recipe never changes it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplexExpressionPolicy {
    #[default]
    LegacyMagnitude,
    Rectangular,
}

impl ComplexExpressionPolicy {
    pub const ALL: [Self; 2] = [Self::Rectangular, Self::LegacyMagnitude];

    pub const fn is_legacy(&self) -> bool {
        matches!(self, Self::LegacyMagnitude)
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::LegacyMagnitude => "Legacy · magnitude arithmetic",
            Self::Rectangular => "Complex · rectangular arithmetic",
        }
    }
}
