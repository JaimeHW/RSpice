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
pub mod eye_diagram;
pub mod fft;
pub mod hb_tones;
pub mod histogram;
pub mod nyquist;
pub mod phase_noise;
pub mod pole_zero;
pub mod rf_measurements;
pub mod smith_chart;

// Re-export main types for convenience (optional - users can also access via submodule)
// Bode
pub use bode::{render_bode_panel, BodeData, BodeDisplayMode, BodePlotState, FrequencyResponse};

// FFT
pub use fft::{render_fft_plot, FftData, FftPoint, FftState, SpectrumAnalysis, WindowFunction};

// Histogram
pub use histogram::{
    render_histogram, Histogram, HistogramBuilder, HistogramDisplayMode, HistogramState,
    HistogramStats,
};

// Nyquist
pub use nyquist::{render_nyquist_plot, NyquistData, NyquistState};

// Pole-Zero
pub use pole_zero::{render_pz_plot, ComplexRoot, PoleZeroData, PoleZeroState, RootType};

// Smith Chart
pub use smith_chart::{
    render_smith_chart, Admittance, Complex, Impedance, SmithChartMode, SmithChartState,
};

// Eye Diagram
pub use eye_diagram::{
    render_eye_diagram, EyeData, EyeDiagramState, EyeDisplayMode, EyeMeasurements, EyeTrace,
};
