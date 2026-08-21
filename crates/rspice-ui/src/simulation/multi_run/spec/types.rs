//! Analysis specification types.

use crate::simulation::config::{NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType};
use crate::simulation::dialog::{
    OpAccuracy, OpAnnotation, OpDeviceDetail, OpHomotopy, OpInitialGuess, OpNodeInitialization,
    OpPreviousState, OpRunPointContext, OpSaveDevice, OpTemperatureMode,
};
use crate::simulation::multi_run::FrequencySweep;
use serde::{Deserialize, Serialize};

/// Numerical formulation used to solve a periodic steady-state request.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PssMethod {
    /// Time-domain shooting Newton solve.
    #[default]
    Shooting,
    /// Frequency-domain harmonic-balance solve.
    HarmonicBalance,
}

/// Periodic initialization used before an envelope-following solve.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeInitialPeriodicSolve {
    HarmonicBalance,
    PeriodicSteadyState,
    /// Compatibility default for specifications saved before this control existed.
    #[default]
    TransientSpectralEstimate,
}

/// Slow-time step policy for envelope analysis.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeAdaptiveMode {
    Enabled,
    /// Compatibility default preserving legacy fixed-step envelope requests.
    #[default]
    FixedEnvelopeStep,
    EventAlignedOnly,
}

/// Available envelope result-extraction implementation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeExtractionPath {
    #[serde(alias = "preview")]
    #[default]
    Projection,
}

const fn default_pss_stabilization_cycles() -> usize {
    20
}

const fn default_pss_shooting_points() -> usize {
    512
}

const fn default_true() -> bool {
    true
}

fn default_noise_reference_node() -> String {
    "0".to_owned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationGoal {
    /// Minimize objective value.
    Minimize,
    /// Maximize objective value.
    Maximize,
    /// Reach target objective value.
    Target,
}

/// Optimization algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationAlgorithm {
    /// Gradient descent with line search.
    GradientDescent,
    /// Pattern search.
    PatternSearch,
    /// Simulated annealing.
    SimulatedAnnealing,
}

/// Optimization variable bounds and initial value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptimizationVariable {
    /// Parameter name from netlist `.param`.
    pub name: String,
    /// Lower bound.
    pub min: f64,
    /// Upper bound.
    pub max: f64,
    /// Initial value.
    pub initial: f64,
}

/// S-parameter port definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpPort {
    /// Positive node.
    pub node_pos: String,
    /// Negative/reference node.
    pub node_neg: String,
    /// Optional per-port reference impedance override.
    pub z0: Option<f64>,
}

/// Harmonic balance tone request used by pipeline execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HbToneSpec {
    /// Tone frequency in Hz.
    pub frequency: f64,
    /// Number of harmonics requested for this tone.
    pub harmonics: usize,
    /// Optional independent source name this tone should drive.
    pub source: Option<String>,
    /// Optional label for display/debug.
    pub name: Option<String>,
}

impl HbToneSpec {
    pub fn new(frequency: f64, harmonics: usize) -> Self {
        Self {
            frequency,
            harmonics,
            source: None,
            name: None,
        }
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        let source = source.into();
        self.source = if source.trim().is_empty() {
            None
        } else {
            Some(source)
        };
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.name = if name.trim().is_empty() {
            None
        } else {
            Some(name)
        };
        self
    }
}

/// Strongly-typed analysis request used by queue execution.
///
/// This removes hidden/default execution parameters from the run executor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnalysisSpec {
    /// Compatibility representation written by plans before OP acquired its
    /// typed eight-field payload. Current code never emits this variant and
    /// executes it with the exact current default contract.
    #[serde(rename = "DcOp")]
    LegacyDcOp,
    /// DC operating point with an exact solve and retention contract.
    #[serde(rename = "DcOpConfigured")]
    DcOp {
        temperature_mode: OpTemperatureMode,
        temperature_celsius: f64,
        initial_guess: OpInitialGuess,
        node_initialization: OpNodeInitialization,
        homotopy: OpHomotopy,
        annotation: OpAnnotation,
        device_detail: OpDeviceDetail,
        save_device_op: OpSaveDevice,
        accuracy: OpAccuracy,
        selected_devices: Vec<String>,
        previous_state: Option<OpPreviousState>,
        violation_devices: Vec<String>,
        violation_source_content_digest: Option<crate::product::ContentDigest>,
        run_point: OpRunPointContext,
    },
    /// DC sweep
    DcSweep {
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
        source2: Option<String>,
        start2: Option<f64>,
        stop2: Option<f64>,
        step2: Option<f64>,
    },
    /// AC analysis
    Ac {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
    },
    /// AC analysis at explicit table-provided frequency points
    AcData {
        table_name: String,
        frequencies: Vec<f64>,
    },
    /// Distortion analysis
    Disto {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        f2_over_f1: Option<f64>,
    },
    /// Transient analysis
    Transient {
        stop_time: f64,
        step_time: f64,
        start_time: f64,
        max_timestep: Option<f64>,
        uic: bool,
    },
    /// Noise analysis
    Noise {
        output_node: String,
        /// Exact differential-output reference. Legacy specifications default
        /// to canonical ground; no live UI singleton is consulted.
        #[serde(default = "default_noise_reference_node")]
        reference_node: String,
        /// Independent source used to normalize input-referred density.
        /// Empty legacy values deliberately fail validation instead of being
        /// guessed from mutable setup state.
        #[serde(default)]
        input_source: String,
        start_freq: f64,
        stop_freq: f64,
        points_per_decade: usize,
        /// Exact selected sweep mode, including explicit-list mode.
        #[serde(default)]
        sweep: NoiseSweepType,
        /// Exact authored axis when `sweep` is explicit-list.
        #[serde(default)]
        explicit_frequencies: Option<Vec<f64>>,
        /// Named `.DATA` table for imported manual decks. The table remains
        /// bound to the immutable source while the resolved axis is retained
        /// above for validation and identity.
        #[serde(default)]
        data_table_name: Option<String>,
        #[serde(default)]
        contribution_detail: NoiseContributionDetail,
        #[serde(default)]
        integration_mode: NoiseIntegrationMode,
        temperature: f64,
    },
    /// Periodic steady-state
    Pss {
        /// Numerical mode. Only authenticated shooting is accepted; the HB
        /// variant exists solely to reject legacy HB-PSS state explicitly.
        #[serde(default)]
        method: PssMethod,
        fundamental_freq: f64,
        /// Exact named periodic large-signal sources. A spec that named none
        /// restores as one that named none; no reader can supply a source
        /// name the design does not carry.
        #[serde(default)]
        tone_sources: Vec<String>,
        /// Settling periods before the shooting solve.
        #[serde(default = "default_pss_stabilization_cycles")]
        tstab_periods: usize,
        /// Shooting waveform samples per period.
        #[serde(default = "default_pss_shooting_points")]
        points_per_period: usize,
        /// Relative periodicity tolerance.
        #[serde(alias = "period_tolerance")]
        tolerance: f64,
        /// Whether the fundamental is an oscillator-period initial guess.
        #[serde(default)]
        oscillator_mode: bool,
        /// Oscillator waveform used for period detection in autonomous mode.
        #[serde(default)]
        oscillator_node: Option<String>,
        /// Numeric harmonic retention count. Zero retains no spectrum.
        num_harmonics: usize,
    },
    /// Harmonic content of a converged periodic steady state.
    ///
    /// Retained as its own analysis rather than folded into the PSS result:
    /// harmonics are indexed by frequency and the periodic waveform by time,
    /// so one shared abscissa cannot describe both. It depends on the PSS
    /// task's periodic state exactly as Fourier depends on a transient
    /// trajectory, which is what gives it a prepared identity of its own
    /// instead of an alias of the PSS task's.
    PssSpectrum {
        /// Harmonics to retain, matching the PSS request that produced the
        /// periodic state this reads.
        num_harmonics: usize,
    },
    /// Harmonic balance
    HarmonicBalance {
        tones: Vec<HbToneSpec>,
        reltol: f64,
        abstol: f64,
        max_iterations: usize,
        damping: f64,
        oversample: usize,
        #[serde(default)]
        collocation_points: Option<usize>,
        max_mixing_order: usize,
        use_krylov: bool,
        gmres_restart: usize,
        source_stepping: bool,
        verbose: bool,
    },
    /// DC-linearized transfer function.
    Tf {
        input_source: String,
        output_expression: String,
        transfer_gain: bool,
        input_resistance: bool,
        output_resistance: bool,
        normalization: TfNormalization,
        accuracy: TfAccuracy,
    },
    /// Sensitivity
    Sensitivity {
        output_var: String,
        ac_mode: bool,
        frequency: Option<f64>,
    },
    /// Pole-zero
    PoleZero {
        input_node: String,
        input_ref: String,
        output_node: String,
        output_ref: String,
        transfer_type: String,
        analysis_type: String,
    },
    /// Periodic AC
    Pac,
    /// Periodic noise
    Pnoise,
    /// Periodic transfer function
    Pxf,
    /// Periodic stability
    Pstb,
    /// Stability analysis
    Stb {
        probe_node: String,
        start_freq: f64,
        stop_freq: f64,
        #[serde(default)]
        sweep: FrequencySweep,
        points_per_decade: usize,
        /// Retain the Nyquist contour. Older specifications predate the
        /// control and were executed with the contour on, so that is the
        /// serde default.
        #[serde(default = "default_true")]
        compute_nyquist: bool,
    },
    /// Monte Carlo
    MonteCarlo {
        /// Where a trial's variation comes from. Older specifications predate
        /// the choice and were executed against the deck's eligible parameter
        /// values, so that is the serde default.
        #[serde(default)]
        variation_source: crate::simulation::dialog::McVariationSource,
    },
    /// Parametric sweep
    Parametric,
    /// Corner analysis
    Corner,
    /// Reliability aging analysis
    Reliability {
        target_years: Vec<f64>,
        enable_hci: bool,
        enable_nbti: bool,
        enable_em: bool,
        min_stress_voltage: f64,
    },
    /// Optimization analysis.
    Optimization {
        variables: Vec<OptimizationVariable>,
        objective_node: String,
        objective_ref: String,
        goal: OptimizationGoal,
        target: Option<f64>,
        algorithm: OptimizationAlgorithm,
        max_iterations: usize,
        cost_tolerance: f64,
        fd_step: f64,
        initial_step: f64,
        min_step: f64,
    },
    /// Safety / SOA analysis.
    Soa {
        stop_time: f64,
        step_time: f64,
        check_vgs_max: bool,
        max_vgs: f64,
        check_vds_max: bool,
        max_vds: f64,
        check_vbe_max: bool,
        max_vbe: f64,
        check_vce_max: bool,
        max_vce: f64,
    },
    /// S-parameter analysis
    SParameter {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        z0: f64,
        ports: Vec<SpPort>,
    },
    /// Envelope transient analysis
    Envelope {
        /// First carrier frequency. Retained as a scalar for legacy payload
        /// compatibility; subsequent carriers are stored separately.
        fundamental_freq: f64,
        #[serde(default)]
        additional_carrier_tones: Vec<f64>,
        stop_time: f64,
        num_harmonics: usize,
        #[serde(default, alias = "max_step")]
        envelope_step: Option<f64>,
        #[serde(default)]
        modulation_sources: Vec<String>,
        #[serde(default)]
        initial_periodic_solve: EnvelopeInitialPeriodicSolve,
        #[serde(default)]
        adaptive_mode: EnvelopeAdaptiveMode,
        #[serde(default)]
        extraction_path: EnvelopeExtractionPath,
    },
    /// Fourier analysis
    Fourier {
        fundamental_freq: f64,
        num_harmonics: usize,
        output_node: String,
        output_ref: String,
        start_time: f64,
        stop_time: f64,
        /// Retain the derived total-harmonic-distortion result.
        #[serde(default = "default_true")]
        compute_thd: bool,
        /// Scale spectral components and DC by the fundamental magnitude.
        ///
        /// Older serialized specifications omitted this field and produced
        /// dimensional, unnormalized results, so its serde default is false.
        #[serde(default)]
        normalize: bool,
    },
    /// Quasi-periodic multi-tone steady state. Configuration is transportable,
    /// but execution currently fails closed until the QPSS solver is present.
    Qpss {
        tones: Vec<HbToneSpec>,
        max_iterations: usize,
        relative_tolerance: f64,
        autonomous: bool,
        oscillator_node: Option<String>,
    },
    /// Large-signal S-parameters around harmonic balance.
    Hbsp {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        ports: Vec<SpPort>,
        max_sideband: usize,
        mixed_mode: bool,
        noise_parameters: bool,
    },
    /// Harmonic-balance noise.
    Hbnoise {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        output_node: String,
        output_ref: String,
        input_source: String,
        max_sideband: usize,
        integrated_noise: bool,
        noise_figure: bool,
        contributor_ranking: bool,
    },
    /// Periodic S-parameters around PSS.
    Psp {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        ports: Vec<SpPort>,
        max_sideband: usize,
        mixed_mode: bool,
        noise_parameters: bool,
    },
    /// Quasi-periodic AC conversion matrix.
    Qpac {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        input_source: String,
        output_node: String,
        output_ref: String,
        input_lattice: [i32; 2],
        output_lattice: [i32; 2],
    },
    /// Quasi-periodic noise folding and correlation.
    Qpnoise {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        output_node: String,
        output_ref: String,
        input_source: String,
        lattice_min: [i32; 2],
        lattice_max: [i32; 2],
        integrated_noise: bool,
        contributor_ranking: bool,
    },
    /// Quasi-periodic translated transfer.
    Qpxf {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: FrequencySweep,
        input_source: String,
        output_node: String,
        output_ref: String,
        input_lattice: [i32; 2],
        output_lattice: [i32; 2],
        group_delay: bool,
    },
    /// Stochastic transient device-noise simulation.
    TransientNoise {
        stop_time: f64,
        step_time: f64,
        start_time: f64,
        max_timestep: f64,
        seed: u64,
        noise_fmax: f64,
        scale: f64,
        uic: bool,
    },
    /// Linearized local DC mismatch contribution.
    DcMismatch {
        output_expression: String,
        sigma_multiplier: f64,
        contributor_limit: usize,
        include_process: bool,
        include_mismatch: bool,
        normalized_contributions: bool,
    },
}

impl AnalysisSpec {
    /// Canonical fresh operating-point request used by imported `.OP` decks.
    #[must_use]
    pub fn dc_op() -> Self {
        let config = crate::simulation::dialog::OpConfig::default();
        Self::DcOp {
            temperature_mode: config.temperature_mode,
            temperature_celsius: config.temperature_celsius,
            initial_guess: config.initial_guess,
            node_initialization: config.node_initialization,
            homotopy: config.homotopy,
            annotation: config.annotation,
            device_detail: config.device_detail,
            save_device_op: config.save_device_op,
            accuracy: config.accuracy,
            selected_devices: config.selected_devices,
            previous_state: config.previous_state,
            violation_devices: config.violation_devices,
            violation_source_content_digest: config.violation_source_content_digest,
            run_point: config.run_point,
        }
    }
}

#[cfg(test)]
mod operating_point_serde_tests {
    use super::*;

    #[test]
    fn legacy_unit_operating_point_plan_migrates_without_guessing_fields() {
        let legacy: AnalysisSpec = serde_json::from_str("\"DcOp\"").expect("legacy unit OP");
        assert_eq!(legacy, AnalysisSpec::LegacyDcOp);
        assert_eq!(
            legacy.run_type(),
            crate::simulation::multi_run::AnalysisRunType::DcOp
        );

        let current = AnalysisSpec::dc_op();
        let encoded = serde_json::to_string(&current).expect("current OP serializes");
        assert!(encoded.contains("DcOpConfigured"));
        assert_eq!(
            serde_json::from_str::<AnalysisSpec>(&encoded).unwrap(),
            current
        );
    }

    #[test]
    fn legacy_noise_spec_is_retained_but_fails_closed_without_input_identity() {
        let legacy = r#"{"Noise":{"output_node":"out","start_freq":1.0,"stop_freq":1000.0,"points_per_decade":10,"temperature":300.15}}"#;
        let decoded: AnalysisSpec =
            serde_json::from_str(legacy).expect("legacy noise spec decodes");
        assert!(matches!(
            &decoded,
            AnalysisSpec::Noise {
                reference_node,
                input_source,
                sweep: NoiseSweepType::Decade,
                contribution_detail: NoiseContributionDetail::Top50,
                integration_mode: NoiseIntegrationMode::Enabled,
                ..
            } if reference_node == "0" && input_source.is_empty()
        ));
        assert!(decoded.validate().is_err());
    }

    #[test]
    fn legacy_preview_envelope_extraction_migrates_to_the_named_projection_method() {
        let decoded: EnvelopeExtractionPath =
            serde_json::from_str("\"preview\"").expect("legacy extraction path decodes");
        assert_eq!(decoded, EnvelopeExtractionPath::Projection);
        assert_eq!(
            serde_json::to_string(&decoded).expect("current extraction path encodes"),
            "\"projection\""
        );
    }
}

/// Retained transfer-gain normalization policy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TfNormalization {
    #[default]
    None,
    RelativeToNominal,
    PerSourceUnit,
}

/// Numerical policy for the DC operating point and zero-hertz solves.
///
/// The same tier the transfer-function form offers; see
/// [`crate::simulation::accuracy`] for what a tier name resolves to.
pub type TfAccuracy = crate::simulation::accuracy::AnalysisAccuracy;
