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
//! compute pipelines). Rendering lives in `crate::workbench::result_document`, built on
//! the `crate::ui::plot` engine.

pub mod bode;
pub mod calculator;
pub mod eye_diagram;
pub mod fft;
pub mod hb_tones;
pub mod histogram;
pub mod nyquist;
pub mod phase_noise;
pub mod pole_zero;
pub mod smith_chart;

// Re-export main types for convenience (optional - users can also access via submodule)
// Bode
pub use bode::{BodeData, BodeDisplayMode, BodePlotState, FrequencyResponse};

// FFT
pub use fft::{FftData, FftPoint, FftState, InputFidelity, SpectrumAnalysis, WindowFunction};

// Histogram
pub use histogram::{
    Histogram, HistogramBuilder, HistogramDisplayMode, HistogramState, HistogramStats,
};

// Nyquist
pub use nyquist::{NyquistData, NyquistState};

// Pole-Zero
pub use pole_zero::{ComplexRoot, PoleZeroData, PoleZeroState, RootType};

// Smith Chart
pub use smith_chart::{Admittance, Complex, Impedance, SmithChartMode, SmithChartState};

// Eye Diagram
pub use eye_diagram::{EyeData, EyeDiagramState, EyeDisplayMode, EyeMeasurements, EyeTrace};
