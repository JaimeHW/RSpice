//! Signal Integrity Analysis
//!
//! High-speed digital signal analysis for SerDes, DDR, PCIe, and other
//! high-speed interfaces. Includes eye diagram generation, jitter analysis,
//! and signal quality metrics.
//!
//! # Features
//!
//! - **Eye Diagram**: traces folded on the estimated unit interval, a stated
//!   one, or the edges of a clock signal
//! - **Eye Measurements**: height, width, area, margins, edge times, jitter
//!   and crossing level, for eyes of two to four levels
//! - **Jitter separation**: random and deterministic jitter from a dual-Dirac
//!   tail fit of the crossings, and the bathtub it extrapolates

mod clock_trigger;
mod eye_data;
mod eye_measurements;
#[cfg(test)]
mod eye_test_signals;
mod jitter_fit;
mod normal;
mod unit_interval;

pub use clock_trigger::{
    ClockTrigger, ClockTriggerRejection, MIN_CLOCK_EDGES, centring_delay, clock_trigger,
};
pub use eye_data::{Edge, EyeData, EyeDataBuilder, EyeTrace, find_edges};
pub use eye_measurements::{
    BerEstimate, EyeLevel, EyeMeasureOptions, EyeMeasurements, EyeOpening, MAX_EYE_LEVELS,
    MIN_EYE_LEVELS, calculate_eye_measurements, calculate_eye_measurements_with,
};
pub use jitter_fit::{
    Bathtub, DEFAULT_OPENING_BER, DualDiracFit, EdgeTail, MIN_FIT_CROSSINGS, bathtub,
};
pub use normal::{inverse_q_function, q_function};
pub use unit_interval::{
    CrossingPhaseFit, UiEstimate, UiEstimateRejection, crossing_phase_at, estimate_unit_interval,
    fold_anchor,
};
