//! Portable worker result values and their domain conversions.

use serde::{Deserialize, Serialize};

use rspice_results::simulation_values::{
    DigitalEventPoint, EventNodeHistory, PstbFloquetMode, RealEventPoint, TransferFunctionQuantity,
    TransferFunctionScalar,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerPstbFloquetMode {
    pub multiplier: (f64, f64),
    pub exponent: (f64, f64),
    pub probe_participation: f64,
    pub is_unstable: bool,
    pub is_trivial: bool,
    pub subharmonic_order: Option<usize>,
}

impl From<PstbFloquetMode> for WorkerPstbFloquetMode {
    fn from(value: PstbFloquetMode) -> Self {
        Self {
            multiplier: value.multiplier,
            exponent: value.exponent,
            probe_participation: value.probe_participation,
            is_unstable: value.is_unstable,
            is_trivial: value.is_trivial,
            subharmonic_order: value.subharmonic_order,
        }
    }
}

impl From<WorkerPstbFloquetMode> for PstbFloquetMode {
    fn from(value: WorkerPstbFloquetMode) -> Self {
        Self {
            multiplier: value.multiplier,
            exponent: value.exponent,
            probe_participation: value.probe_participation,
            is_unstable: value.is_unstable,
            is_trivial: value.is_trivial,
            subharmonic_order: value.subharmonic_order,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerPstbStabilityClassification {
    Stable,
    UnstableReal,
    UnstableComplex,
    PeriodDoubling,
    NeimarkSacker,
    SaddleNode,
    Marginal,
    Indeterminate,
}

impl TryFrom<rspice_core::analysis::pstb::StabilityType> for WorkerPstbStabilityClassification {
    type Error = String;

    fn try_from(value: rspice_core::analysis::pstb::StabilityType) -> Result<Self, Self::Error> {
        use rspice_core::analysis::pstb::StabilityType;
        match value {
            StabilityType::Stable => Ok(Self::Stable),
            StabilityType::UnstableReal => Ok(Self::UnstableReal),
            StabilityType::UnstableComplex => Ok(Self::UnstableComplex),
            StabilityType::PeriodDoubling => Ok(Self::PeriodDoubling),
            StabilityType::NeimarkSacker => Ok(Self::NeimarkSacker),
            StabilityType::SaddleNode => Ok(Self::SaddleNode),
            StabilityType::Marginal => Ok(Self::Marginal),
            StabilityType::Indeterminate => Ok(Self::Indeterminate),
            _ => Err("PSTB returned an unsupported stability classification".to_owned()),
        }
    }
}

impl From<WorkerPstbStabilityClassification> for rspice_core::analysis::pstb::StabilityType {
    fn from(value: WorkerPstbStabilityClassification) -> Self {
        match value {
            WorkerPstbStabilityClassification::Stable => Self::Stable,
            WorkerPstbStabilityClassification::UnstableReal => Self::UnstableReal,
            WorkerPstbStabilityClassification::UnstableComplex => Self::UnstableComplex,
            WorkerPstbStabilityClassification::PeriodDoubling => Self::PeriodDoubling,
            WorkerPstbStabilityClassification::NeimarkSacker => Self::NeimarkSacker,
            WorkerPstbStabilityClassification::SaddleNode => Self::SaddleNode,
            WorkerPstbStabilityClassification::Marginal => Self::Marginal,
            WorkerPstbStabilityClassification::Indeterminate => Self::Indeterminate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerTransferFunctionQuantity {
    Voltage,
    Current,
}

impl From<TransferFunctionQuantity> for WorkerTransferFunctionQuantity {
    fn from(value: TransferFunctionQuantity) -> Self {
        match value {
            TransferFunctionQuantity::Voltage => Self::Voltage,
            TransferFunctionQuantity::Current => Self::Current,
        }
    }
}

impl From<WorkerTransferFunctionQuantity> for TransferFunctionQuantity {
    fn from(value: WorkerTransferFunctionQuantity) -> Self {
        match value {
            WorkerTransferFunctionQuantity::Voltage => Self::Voltage,
            WorkerTransferFunctionQuantity::Current => Self::Current,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WorkerTransferFunctionScalar {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

impl From<TransferFunctionScalar> for WorkerTransferFunctionScalar {
    fn from(value: TransferFunctionScalar) -> Self {
        match value {
            TransferFunctionScalar::Finite(value) => Self::Finite(value),
            TransferFunctionScalar::PositiveInfinity => Self::PositiveInfinity,
            TransferFunctionScalar::NegativeInfinity => Self::NegativeInfinity,
        }
    }
}

impl From<WorkerTransferFunctionScalar> for TransferFunctionScalar {
    fn from(value: WorkerTransferFunctionScalar) -> Self {
        match value {
            WorkerTransferFunctionScalar::Finite(value) => Self::Finite(value),
            WorkerTransferFunctionScalar::PositiveInfinity => Self::PositiveInfinity,
            WorkerTransferFunctionScalar::NegativeInfinity => Self::NegativeInfinity,
        }
    }
}

/// One committed digital event, as it crosses the worker edge.
///
/// Points are transported whole rather than as parallel time/value arrays:
/// an event history is short enough that the buffer split buys nothing, and
/// paired arrays can arrive with different lengths.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WorkerDigitalEventPoint {
    pub time_s: f64,
    pub value_code: u8,
}

impl From<DigitalEventPoint> for WorkerDigitalEventPoint {
    fn from(value: DigitalEventPoint) -> Self {
        Self {
            time_s: value.time_s,
            value_code: value.value_code,
        }
    }
}

impl From<WorkerDigitalEventPoint> for DigitalEventPoint {
    fn from(value: WorkerDigitalEventPoint) -> Self {
        Self {
            time_s: value.time_s,
            value_code: value.value_code,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WorkerRealEventPoint {
    pub time_s: f64,
    pub value: f64,
}

impl From<RealEventPoint> for WorkerRealEventPoint {
    fn from(value: RealEventPoint) -> Self {
        Self {
            time_s: value.time_s,
            value: value.value,
        }
    }
}

impl From<WorkerRealEventPoint> for RealEventPoint {
    fn from(value: WorkerRealEventPoint) -> Self {
        Self {
            time_s: value.time_s,
            value: value.value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerDigitalEventTrace {
    pub node_name: String,
    pub points: Vec<WorkerDigitalEventPoint>,
}

impl From<EventNodeHistory<DigitalEventPoint>> for WorkerDigitalEventTrace {
    fn from(value: EventNodeHistory<DigitalEventPoint>) -> Self {
        Self {
            node_name: value.node_name,
            points: value.points.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<WorkerDigitalEventTrace> for EventNodeHistory<DigitalEventPoint> {
    fn from(value: WorkerDigitalEventTrace) -> Self {
        Self {
            node_name: value.node_name,
            points: value.points.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerRealEventTrace {
    pub node_name: String,
    pub points: Vec<WorkerRealEventPoint>,
}

impl From<EventNodeHistory<RealEventPoint>> for WorkerRealEventTrace {
    fn from(value: EventNodeHistory<RealEventPoint>) -> Self {
        Self {
            node_name: value.node_name,
            points: value.points.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<WorkerRealEventTrace> for EventNodeHistory<RealEventPoint> {
    fn from(value: WorkerRealEventTrace) -> Self {
        Self {
            node_name: value.node_name,
            points: value.points.into_iter().map(Into::into).collect(),
        }
    }
}
