//! Viewers Module
//!
//! Integrated and specialized viewers for simulation results.
//!
//! - `integrated` - Combined multi-view viewer with synchronized cursors
//! - `results_browser` - Hierarchical tree browser for simulation outputs

pub mod integrated;
pub mod results_browser;

// Re-export main types
pub use integrated::{
    ActiveViewer, AnalysisTab, IntegratedViewerState, SimulationConfigs, ViewerStates,
};
pub use results_browser::{ResultNode, ResultNodeType, ResultsBrowserTree};
