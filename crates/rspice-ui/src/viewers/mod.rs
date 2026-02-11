//! Viewers Module
//!
//! Integrated and specialized viewers for simulation results.
//!
//! - `active_viewer` - Specialized viewer selector for waveform panel

pub mod active_viewer;

// Re-export main types
pub use active_viewer::ActiveViewer;
