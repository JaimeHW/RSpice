//! Simulation Results and Waveform Data
//!
//! Containers for simulation results that bridge rspice-core outputs
//! to the waveform viewer and other UI components.

use crate::results::safety::{SoAEvaluation, SoAViolation};
use std::collections::HashMap;

mod accessors;
#[cfg(any(target_arch = "wasm32", test))]
mod convergence_transport;
#[cfg(any(target_arch = "wasm32", test))]
pub(crate) use convergence_transport::ConvergenceTransport;
mod measurements;
pub(crate) use measurements::{parse_study_bin, parse_study_tuple};
mod monte_carlo;
mod operating_point;
mod qpac;
mod qpnoise;
mod qpss;
mod qpxf;
mod recorded_fft;
mod waveform;

pub use monte_carlo::MonteCarloVariableResult;
pub use operating_point::DcOpResult;
pub use recorded_fft::RecordedFftSpectrum;
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

pub use rspice_results::simulation_values::{
    DigitalEventPoint, EventNodeHistory, PstbFloquetMode, RealEventPoint, TransferFunctionQuantity,
    TransferFunctionScalar,
};

/// Committed digital and real events, and exact signed current impulses.
/// Empty current traces can retain an explicit coverage claim even when no
/// charge events occurred; unavailable legacy histories carry `None`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransientEventHistory {
    /// Exact charge observations; absent means unknown, not zero charge.
    pub current_impulses: Option<crate::state::CurrentImpulseHistoryEvidence>,
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
        self.digital.is_empty() && self.real.is_empty() && self.current_impulses.is_none()
    }
}

//=============================================================================
// Simulation Result Container
//=============================================================================

/// Container for all simulation results
#[derive(Debug, Clone)]
#[allow(
    clippy::large_enum_variant,
    reason = "result variants preserve the existing typed payload shape"
)]
pub enum SimulationResult {
    /// DC operating point. Boxed: the operating-point result is three
    /// times the next largest variant, and every result value paid for it.
    DcOp(Box<DcOpResult>),

    /// DC sweep results
    DcSweep {
        /// Exact curve identities and traversal; absent on legacy/imported data.
        evidence: Option<std::sync::Arc<crate::state::DcSweepEvidence>>,
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
        /// Source times remain tied to the engine trajectory after cropping
        /// or projection. Force-accepted points converged in Newton but failed
        /// the local truncation error test. Missing evidence means unknown.
        convergence: Option<std::sync::Arc<crate::state::TransientConvergenceEvidence>>,
        /// Committed XSPICE event histories, when the deck has event nodes.
        ///
        /// Events keep their own sparse schedule instead of being resampled
        /// onto `time`: the instant a digital node changed is the datum, and
        /// the analog grid would only approximate it.
        events: TransientEventHistory,
        /// Spectra the engine computed for the `.fft` cards this solve carried.
        ///
        /// Recorded rather than derived: a `.fft` card makes every requested
        /// sample time a solver stop, so its spectrum belongs to this solve and
        /// nothing downstream can reproduce it from the retained waveforms.
        /// Empty for a transient that carried no card, which is every transient
        /// with no bound FFT analysis.
        spectra: Vec<std::sync::Arc<RecordedFftSpectrum>>,
    },

    /// AC analysis results
    Ac {
        /// Quality of a transient source used to derive this spectrum, when available.
        convergence: Option<std::sync::Arc<crate::state::TransientConvergenceEvidence>>,
        /// Frequency vector
        frequencies: Vec<f64>,
        /// Complex waveforms indexed by signal name
        waveforms: HashMap<String, WaveformData>,
        /// Evaluated `.MEAS AC` results (against magnitude data).
        measurements: Vec<rspice_core::MeasureResult>,
        /// Resolved power-wave references for SP/PSP/HBSP; absent for ordinary AC.
        reference_impedances_ohm: Option<Vec<f64>>,
        /// Temperature qualifying SP Norton covariance and two-port noise factors.
        noise_reference_temperature_kelvin: Option<f64>,
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

    /// Complete quasi-periodic response, plus the selected tuple display.
    Qpac {
        /// Probe offsets, not the translated physical frequencies.
        frequencies: Vec<f64>,
        waveforms: HashMap<String, WaveformData>,
        response: std::sync::Arc<rspice_core::engine::QpacAnalysisResult>,
    },

    /// Complete physical noise evidence and all requested measurement outputs.
    Qpnoise {
        /// Physical output frequencies, including negative values and zero.
        frequencies: Vec<f64>,
        waveforms: HashMap<String, WaveformData>,
        response: std::sync::Arc<rspice_core::engine::QpnoiseAnalysisResult>,
    },

    Qpxf {
        /// Physical output frequencies, including negative values and zero.
        frequencies: Vec<f64>,
        waveforms: HashMap<String, WaveformData>,
        response: std::sync::Arc<rspice_core::engine::QpxfAnalysisResult>,
    },

    /// Independent-tone spectra with the signed tuple for every displayed bin
    /// and the complete MNA operating point for persistence and consumers.
    Qpss {
        frequencies: Vec<f64>,
        tuples: Vec<Vec<i32>>,
        waveforms: HashMap<String, WaveformData>,
        operating_point: std::sync::Arc<rspice_core::engine::QpssOperatingPoint>,
    },

    /// Noise analysis results
    Noise {
        /// Explicit spectrum unit; absent on imported or historical data.
        output_unit: Option<rspice_core::analysis::MeasurementUnit>,
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

    /// Linearized DC mismatch spread and its ranked contributors.
    ///
    /// The whole answer is the evidence: `.DCMATCH` produces five standard
    /// deviations and one ranked table, no waveform, and nothing a viewer
    /// should re-derive.
    DcMismatch {
        evidence: std::sync::Arc<crate::state::DcMismatchEvidence>,
    },

    /// One `.SENS` study, exactly as the engine's complete entries answered
    /// it: one filter, one grid, and one column per variable per point.
    SensitivityStudy {
        evidence: std::sync::Arc<crate::state::SensitivityStudyEvidence>,
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
        best_objectives: Vec<crate::simulation::optimizer::OptimizationObjectiveObservation>,
        best_constraints: Vec<crate::simulation::optimizer::OptimizationConstraintObservation>,
        /// Whether convergence criterion was met.
        converged: bool,
    },

    /// Safety / SOA analysis result.
    Soa {
        /// Complete observation history when waveforms use a reporting grid.
        source_history: Option<std::sync::Arc<crate::state::SoaSourceHistory>>,
        convergence: Option<std::sync::Arc<crate::state::TransientConvergenceEvidence>>,
        /// Reporting time axis; source_history retains the full check axis when resampled.
        time: Vec<f64>,
        /// Waveforms indexed by signal name.
        waveforms: HashMap<String, WaveformData>,
        /// Collected SOA violations.
        violations: Vec<SoAViolation>,
        /// Complete evaluated-rule evidence, including passing rules.
        evaluations: Vec<SoAEvaluation>,
    },

    /// One recorded `.FFT` spectrum, selected from the transient that
    /// computed it. The analysis runs no solve of its own, so the numbers here
    /// are the bound transient's; its convergence evidence travels with them.
    Fft {
        spectrum: std::sync::Arc<RecordedFftSpectrum>,
        convergence: Option<std::sync::Arc<crate::state::TransientConvergenceEvidence>>,
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
