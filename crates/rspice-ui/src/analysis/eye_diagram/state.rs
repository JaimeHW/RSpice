//! Eye Diagram State Management
//!
//! Viewer state for eye diagram display including mode selection,
//! persistence settings, and mask configuration.

mod cache;
mod cursor;
mod diagram;
mod display;
mod mask;

pub use cache::{EyePersistenceCache, EyePersistenceCacheKey};
pub use cursor::{EyeCursorMode, EyeCursorState, EyeViewRange};
pub use diagram::EyeDiagramState;
pub use display::{ColorMap, EyeDisplayMode};
pub use mask::{EyeMask, MaskPolygon};

pub(super) const EYE_VIEW_TIME_DIVISIONS: f64 = 10.0;
pub(super) const EYE_VIEW_VOLTAGE_DIVISIONS: f64 = 8.0;
pub(super) const EYE_MAX_MARKERS: usize = 16;
