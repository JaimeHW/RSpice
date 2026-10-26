//! The engine's own result types, as the result crate hands them to
//! presentation.
//!
//! A viewer reads what a run produced: operating-point reports, the periodic
//! small-signal and noise results, folded eyes, network checks, HDL output
//! records and bus event histories, and the measurements taken over them.
//! Those types are the engine's, and this crate already depends on the engine
//! for them, so presentation names them here and never the engine itself.

pub use rspice_core::MeasureResult;
pub use rspice_core::abort_signal::NoAbort;
pub use rspice_core::analysis::s_param::{
    MAX_NETWORK_DIAGNOSTIC_PORTS, network_quality_with_abort,
};
pub use rspice_core::analysis::signal_integrity::{
    EyeData, EyeDataBuilder, EyeMeasurements, EyeTrace, UiEstimateRejection,
    calculate_eye_measurements, crossing_phase_at, estimate_unit_interval, fold_anchor,
};
pub use rspice_core::circuit::{DeviceOpEntry, DeviceOpReport};
pub use rspice_core::config::{HdlOrigin, HdlOutputRecord, HdlPoint, HdlSeverity, HdlStream};
pub use rspice_core::engine::{
    QpacAnalysisResult, QpnoiseAnalysisResult, QpnoiseIntegrationMethod, QpnoiseObservation,
    QpnoiseUnavailable, QpnoiseValue, QpssOperatingPoint, QpxfAnalysisResult, QpxfGroupDelay,
    QpxfOutput, QpxfQuantity,
};
pub use rspice_core::execution::{
    BusMemberHistory, bus_events, event_code_to_vcd_bit, split_bus_notation,
};
pub use rspice_core::io::VcdBit;
