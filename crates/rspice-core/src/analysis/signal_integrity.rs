//! Signal Integrity Analysis
//!
//! High-speed digital signal analysis for SerDes, DDR, PCIe, and other
//! high-speed interfaces. Includes eye diagram generation, jitter analysis,
//! and signal quality metrics.
//!
//! # Features
//!
//! - **Eye Diagram**: traces folded on the estimated unit interval
//! - **Eye Measurements**: height, width, area, margins, edge times, jitter
//!   and crossing level

mod eye_data;
mod eye_measurements;
#[cfg(test)]
mod eye_test_signals;
mod unit_interval;

pub use eye_data::{Edge, EyeData, EyeDataBuilder, EyeTrace, find_edges};
pub use eye_measurements::{EyeMeasurements, calculate_eye_measurements};
pub use unit_interval::{
    CrossingPhaseFit, UiEstimate, UiEstimateRejection, crossing_phase_at, estimate_unit_interval,
    fold_anchor,
};
