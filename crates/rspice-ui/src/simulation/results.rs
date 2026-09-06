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

/// The waveform a stability run retains its Nyquist contour under.
///
/// This is a contract between two modules that never meet: the run writes the
/// contour into its result's waveform map, and the post-run population reads
/// it back out by name to seed the Nyquist sheet. Nothing compiles the pair
/// together, so as two independent string literals a rename on either side
/// left the sheet quietly showing nothing at all. It lives here, beside the
/// result the name is a key into, so both ends resolve the same constant.
pub const STB_NYQUIST_CONTOUR_WAVEFORM: &str = "Nyquist L(jw)";

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

/// One retained Floquet mode from a periodic stability analysis.
///
/// The vector containing these records is the complete authenticated spectrum,
/// sorted by decreasing multiplier magnitude. Presentation limits are applied
/// only to the separate waveform vectors in [`SimulationResult::Pstb`].
#[derive(Debug, Clone, PartialEq)]
pub struct PstbFloquetMode {
    /// Complex Floquet multiplier `(real, imaginary)`.
    pub multiplier: (f64, f64),
    /// Complex Floquet exponent `(real, imaginary)` in `1/s`.
    pub exponent: (f64, f64),
    /// Normalized participation of the configured PSTB probe in this mode.
    pub probe_participation: f64,
    /// Whether the mode lies outside the configured outer stability boundary.
    pub is_unstable: bool,
    /// Whether this is the explicitly authenticated autonomous phase mode.
    pub is_trivial: bool,
    /// Detected root-of-unity order, when subharmonic detection was enabled.
    pub subharmonic_order: Option<usize>,
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
    /// Buses the run declared over `digital`, in declaration order.
    ///
    /// A declaration only, carried beside the member histories it names — the
    /// engine states which conductors are one word, never what the word is.
    pub digital_buses: Vec<crate::state::DigitalBusEvidence>,
}

impl TransientEventHistory {
    /// Whether this run committed no event history at all.
    ///
    /// A bus is a claim *about* member traces, so a table with no traces
    /// under it says nothing and does not make a history non-empty; the
    /// engine cannot produce one, and `validate_digital_bus_table` refuses it
    /// if anything ever does.
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

    /// Authenticated periodic stability result.
    ///
    /// `modes` always retains the complete sorted Floquet spectrum. The mode
    /// axis and waveforms are display projections and may contain only the
    /// configured leading modes.
    Pstb {
        period: f64,
        fundamental_frequency: f64,
        modes: Vec<PstbFloquetMode>,
        floquet_evidence: rspice_core::analysis::FloquetSpectrumEvidence,
        orbit_kind: rspice_core::analysis::FloquetOrbitKind,
        stability_threshold: f64,
        probe_instance: String,
        detect_subharmonics: bool,
        trivial_multiplier_index: Option<usize>,
        stability_verdict: rspice_core::analysis::FloquetStabilityVerdict,
        stability_classification: rspice_core::analysis::pstb::StabilityType,
        min_stability_margin_db: Option<f64>,
        max_multiplier_magnitude: f64,
        num_unstable: usize,
        subharmonics: Vec<usize>,
        converged: bool,
        iterations: usize,
        mode_indices: Vec<f64>,
        waveforms: HashMap<String, WaveformData>,
    },

    /// Harmonic-balance spectra plus the exact retained numerical state used
    /// by HB-dependent analyses. Display behavior matches frequency-domain
    /// AC data; the operating point is an execution artifact, not a trace.
    HarmonicBalance {
        frequencies: Vec<f64>,
        waveforms: HashMap<String, WaveformData>,
        measurements: Vec<rspice_core::MeasureResult>,
        operating_point: std::sync::Arc<rspice_core::engine::HbOperatingPoint>,
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
        /// Evaluated `.MEAS NOISE` results. Periodic-noise analyses leave
        /// this empty until their own measurement grammar is supported.
        measurements: Vec<rspice_core::MeasureResult>,
    },

    /// Pole-zero analysis results
    PoleZero {
        /// Poles (complex values)
        poles: Vec<(f64, f64)>,
        /// Zeros (complex values)
        zeros: Vec<(f64, f64)>,
        /// Completeness and numerical qualification evidence for `poles`.
        pole_evidence: crate::state::PoleZeroRootSetEvidence,
        /// Completeness and numerical qualification evidence for `zeros`.
        zero_evidence: crate::state::PoleZeroRootSetEvidence,
        /// Finite DC transfer gain, when the selected transfer has one.
        gain: Option<f64>,
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
        /// What each retained trial measured, with the trial's own identity.
        ///
        /// Per-variable statistics describe the distribution; they cannot say
        /// which trial produced the worst value or how many trials held a
        /// bound. A specification judging a distribution needs both, so the
        /// trials keep their measurements rather than only their moments.
        member_measurements: Vec<crate::state::FamilyMemberMeasurements>,
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
        /// What each swept point measured, with the point's own identity.
        member_measurements: Vec<crate::state::FamilyMemberMeasurements>,
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
        /// What each corner measured, with the corner's own identity.
        member_measurements: Vec<crate::state::FamilyMemberMeasurements>,
    },

    /// Historical reliability aging result retained so saved runs and worker
    /// responses from earlier builds remain readable while execution is
    /// blocked until qualified PDK aging models exist.
    #[allow(dead_code)]
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
