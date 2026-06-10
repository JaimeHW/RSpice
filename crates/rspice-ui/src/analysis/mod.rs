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
//! Each analysis viewer follows a consistent pattern:
//! - `mod.rs` - Public API and viewer struct
//! - `rendering.rs` - GPU-optimized egui rendering
//! - `state.rs` - Viewer-specific state management
//! - `data.rs` - Data structures for analysis results

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
pub use fft::{
    FftData, FftPoint, FftState, InputFidelity, SpectrumAnalysis, WindowFunction,
};

// Histogram
pub use histogram::{
    Histogram, HistogramBuilder, HistogramDisplayMode, HistogramState, HistogramStats,
};

// Nyquist
pub use nyquist::{NyquistData, NyquistState, render_nyquist_plot};

// Pole-Zero
pub use pole_zero::{ComplexRoot, PoleZeroData, PoleZeroState, RootType, render_pz_plot};

// Smith Chart
pub use smith_chart::{
    Admittance, Complex, Impedance, SmithChartMode, SmithChartState, render_smith_chart,
};

// Eye Diagram
pub use eye_diagram::{
    EyeData, EyeDiagramState, EyeDisplayMode, EyeMeasurements, EyeTrace,
};
