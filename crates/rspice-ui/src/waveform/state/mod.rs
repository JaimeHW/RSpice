//! Waveform Viewer State Management
//!
//! This module defines all state types for the waveform viewer, following
//! commercial EDA patterns for centralized, observable, serializable state.

mod bounds;
mod cursor;
mod panel;
mod selection;
mod trace;
mod transform;
mod viewer;

pub use self::bounds::DataBounds;
pub use self::cursor::{CursorMode, CursorState};
pub use self::panel::WaveformPanel;
pub use self::selection::BoxSelection;
pub use self::trace::{TraceData, TraceStyle};
pub use self::transform::ViewTransform;
pub use self::viewer::{MeasurementScope, WaveformViewerState};
