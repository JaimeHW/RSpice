//! Eye diagram viewer state: the loaded eye, its measurements, and the mask.
//!
//! The sibling `view` module draws the density map and caches its texture on
//! `data_revision`; this module retains the measurements and local controls.

mod diagram;
mod timebase;

pub use diagram::{EyeDiagramState, EyeRateEditor};
pub use rspice_results::eye_mask::EyeMask;
pub use timebase::{EyeTimebase, EyeTimebaseKey, EyeTimebaseProvenance, parse_eye_timebase};
