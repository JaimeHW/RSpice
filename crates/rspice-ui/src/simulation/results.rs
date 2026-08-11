//! Simulation Results and Waveform Data
//!
//! Containers for simulation results that bridge rspice-core outputs
//! to the waveform viewer and other UI components.

use crate::services::safety::{SoAEvaluation, SoAViolation};
use crate::simulation::reliability_engine::ReliabilityResult;
use std::collections::HashMap;

mod accessors;
mod measurements;
mod monte_carlo;
mod operating_point;
mod waveform;

pub use monte_carlo::MonteCarloVariableResult;
pub use operating_point::DcOpResult;
pub use waveform::WaveformData;

/// Electrical quantity represented by a transfer-function input or output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferFunctionQuantity {
    Voltage,
    Current,
}

/// JSON-safe scalar result. Transfer-function resistance can be infinite for
/// an open circuit, so infinity is represented explicitly rather than placed
/// in a floating-point field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransferFunctionScalar {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

impl TransferFunctionScalar {
    #[must_use]
    pub fn from_f64(value: f64) -> Self {
        if value == f64::INFINITY {
            Self::PositiveInfinity
        } else if value == f64::NEG_INFINITY {
            Self::NegativeInfinity
        } else {
            Self::Finite(value)
        }
    }
}
/// One committed event on an XSPICE digital node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DigitalEventPoint {
    /// Event time in seconds.
    pub time_s: f64,
    /// XSPICE 12-state code, produced by `DigitalValue::event_code`.
    pub value_code: u8,
}

/// One committed event on an XSPICE real-valued node.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RealEventPoint {
    /// Event time in seconds.
    pub time_s: f64,
    pub value: f64,
}

/// The committed event history of one node.
#[derive(Debug, Clone, PartialEq)]
pub struct EventNodeHistory<P> {
    pub node_name: String,
    pub points: Vec<P>,
}

/// Every event node a transient run committed.
///
/// Empty for the overwhelming majority of decks, which have no event nodes at
/// all — hence `is_empty`, so callers can skip the whole payload rather than
/// retain an empty one.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransientEventHistory {
    pub digital: Vec<EventNodeHistory<DigitalEventPoint>>,
    pub real: Vec<EventNodeHistory<RealEventPoint>>,
}

impl TransientEventHistory {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.digital.is_empty() && self.real.is_empty()
    }
}

//=============================================================================
// Simulation Result Container
//=============================================================================

/// Container for all simulation results
#[derive(Debug, Clone)]
pub enum SimulationResult {
    /// DC operating point. Boxed: the operating-point result is three
    /// times the next largest variant, and every result value paid for it.
    DcOp(Box<DcOpResult>),

    /// DC sweep results
    DcSweep {
        /// Sweep variable name
        sweep_var: String,
        /// Sweep values
        sweep_values: Vec<f64>,
        /// Waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
        /// Evaluated `.MEAS DC` results.
        measurements: Vec<rspice_core::MeasureResult>,
    },

    /// Transient analysis results
    Transient {
        /// Time vector
        time: Vec<f64>,
        /// Waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
        /// Evaluated `.MEAS TRAN` results.
        measurements: Vec<rspice_core::MeasureResult>,
        /// Exact shooting-PSS numerical state when this transient-shaped
        /// result was produced by PSS. Ordinary transient results carry none.
        periodic_state: Option<std::sync::Arc<rspice_core::engine::PssOperatingPoint>>,
        /// What the solver had to do to produce these waveforms.
        ///
        /// Travels with the data because it qualifies it: a force-accepted
        /// point is a sample the solver could not converge and kept anyway,
        /// so a curve that looks smooth can still be wrong there.
        convergence: rspice_core::diagnostics::ConvergenceQuality,
        /// Committed XSPICE event histories, when the deck has event nodes.
        ///
        /// Events keep their own sparse schedule instead of being resampled
        /// onto `time`: the instant a digital node changed is the datum, and
        /// the analog grid would only approximate it.
        events: TransientEventHistory,
    },

    /// AC analysis results
    Ac {
        /// Frequency vector
        frequencies: Vec<f64>,
        /// Complex waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
        /// Evaluated `.MEAS AC` results (against magnitude data).
        measurements: Vec<rspice_core::MeasureResult>,
    },

    /// Noise analysis results
    Noise {
        /// Frequency vector
        frequencies: Vec<f64>,
        /// Output noise spectral density (V²/Hz or A²/Hz)
        output_noise: Vec<f64>,
        /// Input-referred noise (optional)
        input_noise: Option<Vec<f64>>,
        /// Noise contributors by source
        contributors: HashMap<String, Vec<f64>>,
        /// Ranked band-integrated contributor summary (per device and
        /// mechanism), when the analysis provides it.
        summary: Option<crate::state::NoiseSummary>,
    },

    /// Pole-zero analysis results
    PoleZero {
        /// Poles (complex values)
        poles: Vec<(f64, f64)>,
        /// Zeros (complex values)
        zeros: Vec<(f64, f64)>,
        /// Transfer function gain
        gain: f64,
    },

    /// Sensitivity analysis results
    Sensitivity {
        /// Canonical output expression evaluated by the analysis.
        output: String,
        /// Whether the analysis used an AC small-signal basis.
        ac_mode: bool,
        /// Resolved AC frequency in hertz. `None` for DC sensitivity.
        frequency_hz: Option<f64>,
        /// Parameter sensitivities
        sensitivities: HashMap<String, f64>,
        /// Normalized sensitivities (% change in output / % change in param)
        normalized: HashMap<String, f64>,
    },

    /// Scalar DC small-signal transfer function around the converged
    /// operating point.
    TransferFunction {
        input_source: String,
        output_expression: String,
        input_quantity: TransferFunctionQuantity,
        output_quantity: TransferFunctionQuantity,
        input_unit: String,
        output_unit: String,
        normalization: crate::simulation::multi_run::TfNormalization,
        accuracy: crate::simulation::multi_run::TfAccuracy,
        gain: Option<TransferFunctionScalar>,
        input_resistance: Option<TransferFunctionScalar>,
        output_resistance: Option<TransferFunctionScalar>,
        nominal_input: Option<f64>,
        nominal_output: Option<f64>,
    },

    /// Monte Carlo statistical analysis results.
    MonteCarlo {
        /// Effective random seed used by the Monte Carlo engine.
        seed: u64,
        /// Number of samples requested by .MC
        runs_requested: usize,
        /// Number of converged runs completed
        runs_completed: usize,
        /// Number of failed/non-converged runs
        num_failures: usize,
        /// Whether all runs converged according to engine summary
        all_converged: bool,
        /// Per-variable statistical summaries
        variables: Vec<MonteCarloVariableResult>,
    },

    /// Parametric sweep result (including .STEP TEMP).
    Parametric {
        /// Sweep target label (e.g., PARAM rload, TEMP)
        target: String,
        /// Sweep values in execution order
        sweep_values: Vec<f64>,
        /// Waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
        /// Number of failed points (if any)
        num_failures: usize,
    },

    /// Corner sweep result.
    Corner {
        /// X-axis values in execution order.
        x_values: Vec<f64>,
        /// X-axis label (e.g. Temperature, Corner Index).
        x_label: String,
        /// X-axis unit (e.g. C for temperature).
        x_unit: String,
        /// Corner temperatures (Celsius) in execution order.
        temperatures_c: Vec<f64>,
        /// Corner labels in execution order (e.g. `TT_1.000000V_25.000000C`).
        corner_labels: Vec<String>,
        /// Waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
        /// Number of failed corners
        num_failures: usize,
    },

    /// Reliability aging analysis result.
    Reliability {
        /// Lifetime checkpoints in years.
        years: Vec<f64>,
        /// Waveforms indexed by signal name.
        waveforms: HashMap<String, WaveformData>,
        /// Structured per-device reliability outputs.
        device_results: Vec<ReliabilityResult>,
    },

    /// Optimization analysis result.
    Optimization {
        /// Iteration axis values.
        iterations: Vec<f64>,
        /// Waveforms indexed by signal name.
        waveforms: HashMap<String, WaveformData>,
        /// Best cost reached.
        best_cost: f64,
        /// Best variable values.
        best_variables: HashMap<String, f64>,
        /// Whether convergence criterion was met.
        converged: bool,
    },

    /// Safety / SOA analysis result.
    Soa {
        /// Time axis for SOA checks.
        time: Vec<f64>,
        /// Waveforms indexed by signal name.
        waveforms: HashMap<String, WaveformData>,
        /// Collected SOA violations.
        violations: Vec<SoAViolation>,
        /// Complete evaluated-rule evidence, including passing rules.
        evaluations: Vec<SoAEvaluation>,
    },

    /// Scalar-only result payload for analyses that report measurements
    /// without generating waveform families.
    MeasurementsOnly {
        /// Associated measurements
        measurements: HashMap<String, f64>,
    },
}

impl Default for SimulationResult {
    fn default() -> Self {
        Self::MeasurementsOnly {
            measurements: HashMap::new(),
        }
    }
}
