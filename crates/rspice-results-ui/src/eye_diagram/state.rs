//! Eye diagram viewer state: the loaded eye, its measurements, and the mask.
//!
//! The sibling `view` module draws the density map and caches its texture on
//! `data_revision`; this module retains the measurements and local controls.

mod diagram;
mod settings;
mod timebase;

pub use diagram::EyeDiagramState;
pub use rspice_results::eye_mask::{EyeMask, MaskFrame, MaskShape};
pub use settings::{EyeSettings, OPENING_RATES};
pub use timebase::{
    EyeClockSignal, EyeFold, EyeTimebase, EyeTimebaseKey, EyeTimebaseProvenance, parse_eye_timebase,
};
