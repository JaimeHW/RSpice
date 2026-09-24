//! Eye diagram viewer state: the loaded eye, its measurements, and the mask.
//!
//! `result_document::eye` owns the drawing — it rasterises the density map,
//! derives its axes from `data.ui_count`, and caches the texture on
//! `data_revision`. So there is no display policy here.
//!
//! The persistence cache, the cursor model with its single/delta modes, the
//! pan-zoom view range, and the colour-map and display-mode enums were removed
//! with the controls that would have driven them; none had a caller.

mod diagram;
mod mask;
mod timebase;

pub use diagram::{EyeDiagramState, EyeRateEditor};
pub use mask::EyeMask;
pub use timebase::{EyeTimebase, EyeTimebaseKey, EyeTimebaseProvenance, parse_eye_timebase};
