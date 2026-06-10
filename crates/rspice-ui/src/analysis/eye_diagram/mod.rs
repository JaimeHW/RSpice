//! Eye Diagram Viewer
//!
//! Commercial-grade eye diagram visualization for signal integrity analysis.
//!
//! # Features
//!
//! - Overlay of signal transitions aligned to bit period
//! - Persistence/density display mode
//! - Eye opening measurements (height, width, area)
//! - Jitter calculation (RJ, DJ, TJ)
//! - Rise/fall time measurement
//! - Q-factor from BER
//! - Mask testing support
//!
//! # Architecture
//!
//! Follows Cadence-style signal integrity analysis workflow.

pub mod data;
pub mod measurements;
pub mod state;

pub use data::{EyeData, EyeTrace};
pub use measurements::EyeMeasurements;
pub use state::{EyeDiagramState, EyeDisplayMode};


// =============================================================================
// Tests
// =============================================================================
