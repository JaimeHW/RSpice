//! Analysis Viewers Module
//!
//! Commercial-grade analysis visualization components for circuit simulation results.
//! Each viewer provides specialized plotting and measurement capabilities:
//!
//! - `bode` - Bode magnitude/phase plots for frequency response
//! - `fft` - FFT spectrum analyzer for time-domain signals
//! - `histogram` - Statistical histogram for Monte Carlo and corners
//! - `nyquist` - Nyquist stability plots for control systems
//! - `pole_zero` - Pole-zero diagrams for transfer functions
//! - `smith_chart` - RF Smith chart for impedance matching
//! - `eye_diagram` - High-speed signal integrity eye diagrams
//!
//! # Architecture
//!
//! These modules hold analysis *data and state* (`data.rs`, `state.rs`,
//! compute pipelines). Rendering lives in `crate::workbench::documents::result_document`, built on
//! the `crate::ui::plot` engine.

pub(crate) mod bode;
pub(crate) mod calculator;
pub(crate) mod eye_diagram;
pub(crate) mod fft;
pub(crate) mod histogram;
pub(crate) mod measurements;
pub(crate) mod nyquist;
pub(crate) mod pole_zero;
pub(crate) mod smith_chart;

// These re-exports were "for convenience (optional -- users can also access via
// submodule)", and every caller took the submodule. Bode, Nyquist, Smith chart,
// eye diagram, and pole-zero are all reached as `analysis::<module>::Type`, so
// only the handful of names below were ever used flattened.
// The `*Data` types below are reached through this flattened path only by
// tests; the product code uses `analysis::<module>::Type`. The lib build
// therefore reports them unused and will keep doing so until the tests are
// moved onto the module path -- removing them here breaks the test build.
//
// This is a whole class, not a quirk of this module, and rustc cannot separate
// it from real dead imports: it reports both from the plain lib pass. The
// discriminator is `cargo test -p rspice-ui --lib --no-run`, which compiles
// the lib *with* `cfg(test)`; an import still reported unused there is used by
// nobody and is safe to delete. Cross-check `--target wasm32-unknown-unknown`
// the same way before deleting anything a browser-only path might name.
pub use fft::{FftState, InputFidelity, WindowFunction};
// Reached only by tests, and `bode`, `eye_diagram` and `fft` are private
// modules, so these aliases are the only path to the types.
pub use histogram::{HistogramBuilder, HistogramDisplayMode};
#[cfg(test)]
pub use {bode::BodeData, eye_diagram::EyeData, eye_diagram::EyeTrace, fft::FftData};
#[cfg(test)]
pub use {nyquist::NyquistData, pole_zero::PoleZeroData};
