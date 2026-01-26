//! Commercial-Grade Waveform Viewer for egui
//!
//! This module provides a professional waveform viewing system matching
//! the capabilities of commercial EDA tools like Cadence Spectre's ViVA.
//!
//! # Architecture
//!
//! The waveform viewer follows a clean separation of concerns:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    WaveformViewer                           │
//! ├─────────────────────────────────────────────────────────────┤
//! │ Header: Fit buttons, Zoom, Add Trace, Analysis panels       │
//! ├──────────┬──────────────────────────────────────┬───────────┤
//! │ Y-Axis   │            Plot Canvas               │  Legend   │
//! │ Labels   │  - Grid lines                        │  - Traces │
//! │          │  - Waveform traces                   │  - Toggle │
//! │          │  - Cursors & measurements            │  - X-probe│
//! │          │  - Box selection overlay             │           │
//! ├──────────┴──────────────────────────────────────┴───────────┤
//! │ X-Axis: Time labels with SI prefix                          │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! # Features
//!
//! - **Pan/Zoom**: Scroll wheel zoom (cursor-centered), drag to pan
//! - **Cursors**: Click to place, dual cursor with delta measurements
//! - **Box Zoom**: Shift+drag to select region
//! - **Legend**: Toggle trace visibility, cross-probe highlighting
//! - **Measurements**: Min, max, pk-pk, rise/fall, frequency
//! - **Export**: CSV data, PNG screenshots
//!
//! # Commercial Standards
//!
//! This implementation follows Cadence Spectre conventions:
//! - Cursor readouts interpolate to exact time positions
//! - SI prefixes automatically scale (ns, µs, ms, mV, etc.)
//! - Keyboard shortcuts match industry expectations (F=fit, H=horizontal fit)

pub mod axis;
pub mod cursors;
pub mod export;
pub mod interactions;
pub mod legend;
pub mod measurements;
pub mod rendering;
pub mod state;

// Re-exports for convenient access
pub use rendering::render_waveform_viewer;
pub use state::{
    BoxSelection, CursorMode, CursorState, TraceData, TraceStyle, ViewTransform,
    WaveformViewerState,
};

use crate::egui_app::app::AppState;
use egui::Ui;

/// Main entry point for rendering the waveform viewer panel
///
/// This function orchestrates all waveform rendering, handling the header,
/// axes, plot canvas, cursors, legend, and any overlay panels.
pub fn render_waveform_panel(ui: &mut Ui, state: &mut AppState) {
    rendering::render_waveform_viewer(ui, state);
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_structure() {
        // Verify all submodules are accessible
        let _ = std::any::type_name::<WaveformViewerState>();
        let _ = std::any::type_name::<ViewTransform>();
        let _ = std::any::type_name::<TraceData>();
        let _ = std::any::type_name::<CursorState>();
    }
}
