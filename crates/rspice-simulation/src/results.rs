//! Runtime simulation results and projections over exact engine evidence.

use rspice_results::safety::{SoAEvaluation, SoAViolation};
use std::collections::HashMap;

mod measurements;
mod qpac;
mod qpnoise;
pub mod qpss;
mod qpxf;
#[cfg(test)]
pub(crate) mod test_fixtures;
#[cfg(test)]
mod units_tests;
mod yield_input;
pub use yield_input::yield_provenance_from_monte_carlo_result;

pub use rspice_results::fft::recorded::RecordedFftSpectrum;
pub use rspice_results::waveform::WaveformData;

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

pub use rspice_results::events::TransientEventHistory;

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
        evidence: Option<std::sync::Arc<rspice_results::dc_sweep::DcSweepEvidence>>,
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
        convergence: Option<
            std::sync::Arc<rspice_results::convergence_quality::TransientConvergenceEvidence>,
        >,
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
        convergence: Option<
            std::sync::Arc<rspice_results::convergence_quality::TransientConvergenceEvidence>,
        >,
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
        summary: Option<rspice_results::noise::NoiseSummary>,
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
        pole_evidence: rspice_results::pole_zero::PoleZeroRootSetEvidence,
        /// Completeness and numerical qualification evidence for `zeros`.
        zero_evidence: rspice_results::pole_zero::PoleZeroRootSetEvidence,
        /// Finite DC transfer gain, when the selected transfer has one.
        gain: Option<f64>,
    },

    /// Linearized DC mismatch spread and its ranked contributors.
    ///
    /// The whole answer is the evidence: `.DCMATCH` produces five standard
    /// deviations and one ranked table, no waveform, and nothing a viewer
    /// should re-derive.
    DcMismatch {
        evidence: std::sync::Arc<rspice_results::dc_mismatch::DcMismatchEvidence>,
    },

    /// One `.SENS` study, exactly as the engine's complete entries answered
    /// it: one filter, one grid, and one column per variable per point.
    SensitivityStudy {
        evidence: std::sync::Arc<rspice_results::sensitivity::SensitivityStudyEvidence>,
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
        normalization: rspice_simulation_contract::analysis_spec::TfNormalization,
        accuracy: rspice_simulation_contract::analysis_spec::TfAccuracy,
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
        member_measurements: Vec<rspice_results::family_measurements::FamilyMemberMeasurements>,
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
        member_measurements: Vec<rspice_results::family_measurements::FamilyMemberMeasurements>,
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
        member_measurements: Vec<rspice_results::family_measurements::FamilyMemberMeasurements>,
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
        best_objectives: Vec<rspice_results::optimization::OptimizationObjectiveObservation>,
        best_constraints: Vec<rspice_results::optimization::OptimizationConstraintObservation>,
        /// Whether convergence criterion was met.
        converged: bool,
    },

    /// Safety / SOA analysis result.
    Soa {
        /// Complete observation history when waveforms use a reporting grid.
        source_history: Option<std::sync::Arc<rspice_results::soa_source::SoaSourceHistory>>,
        convergence: Option<
            std::sync::Arc<rspice_results::convergence_quality::TransientConvergenceEvidence>,
        >,
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
        convergence: Option<
            std::sync::Arc<rspice_results::convergence_quality::TransientConvergenceEvidence>,
        >,
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

/// DC operating point data with the exact applied solve and retention contract.
#[derive(Debug, Clone, Default)]
pub struct DcOpResult {
    /// Exact solve, annotation, and retention contract applied to this result.
    pub configuration: rspice_simulation_contract::config::OpConfig,

    /// Number of authored startup directives validated before any selected
    /// ignore/validate-only execution filtering was applied.
    pub validated_startup_directives: usize,

    /// Exact core MNA ordering and values. Ground is omitted; node values are
    /// followed by branch values. This is retained so a later compatible OP
    /// can use the converged state without reconstructing order from maps.
    pub mna_node_names: Vec<String>,
    pub mna_branch_names: Vec<String>,
    pub mna_solution: Vec<f64>,

    /// Node voltages
    pub node_voltages: HashMap<String, f64>,

    /// Branch currents
    pub branch_currents: HashMap<String, f64>,

    /// Per-device operating-point report from the engine (bias and
    /// small-signal parameters with regions, in netlist order) — the data
    /// behind the OP inspector.
    pub device_report: Option<rspice_core::circuit::DeviceOpReport>,
}

/// Monte Carlo samples and their per-variable statistics.
#[derive(Debug, Clone)]
pub struct MonteCarloVariableResult {
    /// Confidence in the mean, with estimator and successful-trial population.
    pub mean_confidence: Option<rspice_results::monte_carlo::MonteCarloMeanConfidence>,
    /// Variable name (e.g., V(out), I(V1))
    pub name: String,
    /// Exact finite sample values in engine execution order.
    pub samples: Vec<f64>,
    /// Arithmetic mean over converged runs
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum observed value
    pub min: f64,
    /// Maximum observed value
    pub max: f64,
    /// Histogram counts for post-processing/visualization
    pub histogram: Vec<usize>,
    /// Histogram bin edges (length = histogram.len() + 1)
    pub bin_edges: Vec<f64>,
}

impl SimulationResult {
    pub fn transient_convergence(
        &self,
    ) -> Option<&std::sync::Arc<rspice_results::convergence_quality::TransientConvergenceEvidence>>
    {
        match self {
            Self::Transient { convergence, .. }
            | Self::Ac { convergence, .. }
            | Self::Soa { convergence, .. }
            // A recorded FFT runs no solve of its own, so the quality of the
            // transient that computed it is the only quality it has.
            | Self::Fft { convergence, .. } => convergence.as_ref(),
            _ => None,
        }
    }
}
