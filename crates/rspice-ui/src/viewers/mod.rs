//! Viewers Module
//!
//! Integrated and specialized viewers for simulation results.
//!
//! - `active_viewer` - Specialized viewer selector for waveform panel
//! - `workspace` - Tabbed workspace model for open viewers

pub mod active_viewer;
pub mod workspace;

// Re-export main types
pub use active_viewer::ActiveViewer;
pub use workspace::ViewerWorkspace;
