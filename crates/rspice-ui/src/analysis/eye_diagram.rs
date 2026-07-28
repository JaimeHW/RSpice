//! Eye Diagram Viewer
//!
//! Commercial-grade eye diagram visualization for signal integrity analysis.
//!
//! # Where the engine lives
//!
//! Eye construction and measurement are simulation mathematics, not
//! presentation, so they live in `rspice_core::analysis::signal_integrity`
//! next to the jitter decomposition and bathtub curves that consume the same
//! crossings. This module owns only what the viewer adds on top: display
//! mode, colour map, cursors, mask polygons, and the persistence cache.
//!
//! The engine types are re-exported here so viewer code keeps one import path.
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

pub(crate) mod state;

pub use rspice_core::analysis::signal_integrity::{
    AmplitudeStats, Edge, EyeData, EyeDataBuilder, EyeMeasurements, EyeTrace, calculate_edge_rate,
    calculate_eye_measurements, find_edges,
};
pub use state::{EyeDiagramState, EyeDisplayMode};

// =============================================================================
// Tests
// =============================================================================
