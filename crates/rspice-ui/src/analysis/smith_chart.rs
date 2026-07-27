//! Smith Chart Viewer
//!
//! Commercial-grade Smith chart visualization for RF/microwave circuit analysis.
//!
//! # Features
//!
//! - Impedance (Z) and Admittance (Y) chart modes
//! - Combined Z-Y overlay mode
//! - Normalized and denormalized impedance display
//! - S-parameter data overlay
//! - VSWR circles
//! - Constant resistance and reactance circles
//! - Interactive marker with impedance readout
//! - Reference impedance selection (default 50Ω)
//!
//! # Architecture
//!
//! This follows the same architecture as Cadence Virtuoso ADE and
//! other commercial RF design tools.

pub mod complex;
pub mod impedance;
pub mod state;

pub use complex::Complex;
pub use impedance::{Admittance, Impedance};
pub use state::{SmithChartMode, SmithChartState};

// =============================================================================
// Tests
// =============================================================================
