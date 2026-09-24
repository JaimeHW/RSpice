//! The analysis specification that crosses the worker boundary.
//!
//! One enum, its serde attributes, and the `serde(default)` functions those
//! attributes name — which have to sit in the same module as the derive that
//! resolves them by path.
//!
//! Split from the contract it belongs to because this is the part that grows:
//! every analysis kind is a variant, and a kind that learns a parameter learns
//! a field here. The wire format is the protocol, so a field added without a
//! `serde(default)` refuses to decode a request an older worker sent.

use serde::{Deserialize, Serialize};

use crate::simulation::config::{NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType};
use crate::simulation::dialog::IntegrationMethod;
use crate::simulation::multi_run::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable, PssMethod, SpPort,
    TfAccuracy, TfNormalization,
};

use super::WorkerSweepType;

const fn worker_default_pss_stabilization_cycles() -> usize {
    20
}

const fn worker_default_pss_shooting_points() -> usize {
    512
}

// The engine's `.PSS` card defaults, so an older worker's request restores as
// the run it described rather than failing to decode.
const fn worker_default_pss_max_iterations() -> usize {
    100
}

const fn worker_default_pss_abstol() -> f64 {
    1.0e-12
}

const fn worker_default_pss_damping() -> f64 {
    1.0
}

const fn worker_default_pss_max_period_change() -> f64 {
    0.1
}

const fn worker_default_true() -> bool {
    true
}

/// The harmonic-balance line-search floor a request an older worker encoded
/// ran at: the literal the solver backtracked to before it read the field.
const fn worker_default_hb_min_damping() -> f64 {
    0.01
}

/// Every harmonic-balance request an older worker encoded solved with the
/// exact real-split Jacobian, which is the engine's own default.
const fn worker_default_hb_use_exact_jacobian() -> bool {
    true
}

fn worker_default_noise_reference_node() -> String {
    "0".to_owned()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerAnalysisSpec {
    #[serde(rename = "DcOp")]
    LegacyDcOp,
    #[serde(rename = "DcOpConfigured")]
    DcOp(crate::simulation::dialog::OpConfig),
    DcSweep {
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
        source2: Option<String>,
        start2: Option<f64>,
        stop2: Option<f64>,
        step2: Option<f64>,
        /// Sweep out and back as one continued solve. Defaulted on read so a
        /// worker message from an older build is understood as the one-way
        /// sweep it described.
        #[serde(default)]
        hysteresis: bool,
        #[serde(default)]
        modes: crate::simulation::config::DcSweepModes,
    },
    Transient {
        stop_time: f64,
        step_time: f64,
        start_time: f64,
        max_timestep: Option<f64>,
        uic: bool,
    },
    Ac {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
    },
    AcData {
        table_name: String,
        frequencies: Vec<f64>,
        #[serde(default)]
        table_options: crate::simulation::config::AcDataTableOptions,
    },
    Noise {
        output_node: String,
        #[serde(default = "worker_default_noise_reference_node")]
        reference_node: String,
        #[serde(default)]
        input_source: String,
        start_freq: f64,
        stop_freq: f64,
        points_per_decade: usize,
        #[serde(default)]
        sweep: NoiseSweepType,
        #[serde(default)]
        explicit_frequencies: Option<Vec<f64>>,
        #[serde(default)]
        data_table_name: Option<String>,
        #[serde(default)]
        contribution_detail: NoiseContributionDetail,
        #[serde(default)]
        integration_mode: NoiseIntegrationMode,
        temperature: f64,
    },
    Sensitivity {
        output_var: String,
        ac_mode: bool,
        frequency: Option<f64>,
        #[serde(default = "crate::simulation::config::design_parameters_filter")]
        filter: String,
        #[serde(default)]
        sweep: Option<crate::simulation::multi_run::SensitivitySweepSpec>,
    },
    PoleZero {
        input_node: String,
        input_ref: String,
        output_node: String,
        output_ref: String,
        transfer_type: String,
        analysis_type: String,
    },
    Tf {
        input_source: String,
        output_expression: String,
        transfer_gain: bool,
        input_resistance: bool,
        output_resistance: bool,
        normalization: TfNormalization,
        accuracy: TfAccuracy,
    },
    Pac,
    Pxf,
    Pnoise,
    Pstb,
    Parametric,
    Corner,
    MonteCarlo {
        #[serde(default)]
        variation_source: crate::simulation::dialog::McVariationSource,
        /// An older worker's request carries no subset, which the card spells
        /// as an absent `PARAMS` list: vary everything eligible.
        #[serde(default)]
        params: Vec<String>,
    },
    Optimization {
        #[serde(default)]
        search: crate::services::simulation_runner::OptimizationSearchControls,
        variables: Vec<OptimizationVariable>,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        objective_unit: String,
        #[serde(default)]
        objective_expression: Option<String>,
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
    Soa {
        #[serde(default)]
        import_model_voltage_ratings: bool,
        #[serde(default)]
        observation: crate::services::simulation_runner::SoaObservationConfig,
        #[serde(default)]
        rules: Vec<crate::services::simulation_runner::SoaRuleConfig>,
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
    Stb {
        probe_node: String,
        start_freq: f64,
        stop_freq: f64,
        sweep: WorkerSweepType,
        points_per_decade: usize,
        #[serde(default = "worker_default_true")]
        compute_nyquist: bool,
    },
    SParameter {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
        z0: f64,
        ports: Vec<SpPort>,
        #[serde(default)]
        do_noise: bool,
    },
    Disto {
        start_freq: f64,
        stop_freq: f64,
        points_per_unit: usize,
        sweep: WorkerSweepType,
        f2_over_f1: Option<f64>,
    },
    Pss {
        #[serde(default)]
        method: PssMethod,
        fundamental_freq: f64,
        /// A request that named no tone restores as one that named no tone;
        /// no reader can supply a source name the design does not carry.
        #[serde(default)]
        tone_sources: Vec<String>,
        #[serde(default = "worker_default_pss_stabilization_cycles")]
        tstab_periods: usize,
        #[serde(default = "worker_default_pss_shooting_points")]
        points_per_period: usize,
        #[serde(alias = "period_tolerance")]
        tolerance: f64,
        #[serde(default)]
        oscillator_mode: bool,
        #[serde(default)]
        oscillator_node: Option<String>,
        num_harmonics: usize,
        /// Integration method for the shooting solve's inner transients, or
        /// `None` for the engine's default. A wire written before the control
        /// existed restores as the default it ran under.
        #[serde(default)]
        integration_method: Option<IntegrationMethod>,
        /// Stabilization window in seconds; zero defers to the period count.
        #[serde(default)]
        tstab: f64,
        #[serde(default = "worker_default_pss_max_iterations")]
        max_iterations: usize,
        #[serde(default = "worker_default_pss_abstol")]
        abstol: f64,
        #[serde(default = "worker_default_pss_damping")]
        damping: f64,
        #[serde(default = "worker_default_pss_max_period_change")]
        max_period_change: f64,
        #[serde(default)]
        verbose: bool,
    },
    HarmonicBalance {
        tones: Vec<HbToneSpec>,
        reltol: f64,
        abstol: f64,
        max_iterations: usize,
        damping: f64,
        #[serde(default = "worker_default_hb_min_damping")]
        min_damping: f64,
        oversample: usize,
        #[serde(default)]
        collocation_points: Option<usize>,
        max_mixing_order: usize,
        use_krylov: bool,
        gmres_restart: usize,
        source_stepping: bool,
        #[serde(default = "worker_default_hb_use_exact_jacobian")]
        use_exact_jacobian: bool,
        verbose: bool,
    },
    Envelope {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        multirate: Option<crate::services::simulation_runner::EnvelopeMultirateConfig>,
        #[serde(default)]
        initialization: crate::services::simulation_runner::EnvelopeInitializationConfig,
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
    Fourier {
        fundamental_freq: f64,
        num_harmonics: usize,
        #[serde(default = "crate::simulation::config::default_fourier_periods")]
        num_periods: usize,
        output_node: String,
        output_ref: String,
        /// An older worker's request carries no further outputs, which is the
        /// one-output card it was asked to run.
        #[serde(default)]
        additional_outputs: Vec<String>,
        start_time: f64,
        stop_time: f64,
        #[serde(default = "worker_default_true")]
        compute_thd: bool,
        #[serde(default)]
        normalize: bool,
    },
    /// Canonical complex analysis carried verbatim when a dedicated wire
    /// mirror would merely duplicate the domain shape. The dispatcher remains
    /// responsible for capability validation after lossless reconstruction.
    #[serde(alias = "ManifestPreview")]
    CanonicalSpec(AnalysisSpec),
}
