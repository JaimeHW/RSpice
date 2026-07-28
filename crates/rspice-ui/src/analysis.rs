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
pub(crate) mod hb_tones;
pub(crate) mod histogram;
pub(crate) mod measurements;
pub(crate) mod nyquist;
pub(crate) mod phase_noise;
pub(crate) mod pole_zero;
pub(crate) mod smith_chart;

// These re-exports were "for convenience (optional -- users can also access via
// submodule)", and every caller took the submodule. Bode, Nyquist, Smith chart,
// eye diagram, and pole-zero are all reached as `analysis::<module>::Type`, so
// only the handful of names below were ever used flattened.
pub use bode::{BodeData, FrequencyResponse};
pub use eye_diagram::{EyeData, EyeTrace};
pub use fft::{FftData, FftState, InputFidelity, WindowFunction};
pub use histogram::{HistogramBuilder, HistogramDisplayMode};
pub use nyquist::NyquistData;
pub use pole_zero::PoleZeroData;
