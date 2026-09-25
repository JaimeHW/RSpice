//! The analysis specification that crosses the worker boundary.
//!
//! One enum, its serde attributes, and the `serde(default)` functions those
//! attributes name — which have to sit in the same module as the derive that
//! resolves them by path.
//!
//! Kept in its own module because this is the part of the contract that grows:
//! every analysis kind is a variant, and a kind that learns a parameter learns
//! a field here. The wire format is the protocol, so a field added without a
//! `serde(default)` refuses to decode a request an older worker sent.

use serde::{Deserialize, Serialize};

use crate::analysis_spec::{
    AnalysisSpec, EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve,
    HbToneSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable, PssMethod, SpPort,
    TfAccuracy, TfNormalization,
};
use crate::config::{
    AcSweepType, FrequencySweep, NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType,
};
use crate::options::IntegrationMethod;

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
pub enum WorkerAnalysisSpec {
    #[serde(rename = "DcOp")]
    LegacyDcOp,
    #[serde(rename = "DcOpConfigured")]
    DcOp(crate::config::OpConfig),
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
        modes: crate::config::DcSweepModes,
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
        table_options: crate::config::AcDataTableOptions,
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
        #[serde(default = "crate::config::design_parameters_filter")]
        filter: String,
        #[serde(default)]
        sweep: Option<crate::config::SensitivitySweepSpec>,
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
        variation_source: crate::mc_draft::McVariationSource,
        /// An older worker's request carries no subset, which the card spells
        /// as an absent `PARAMS` list: vary everything eligible.
        #[serde(default)]
        params: Vec<String>,
    },
    Optimization {
        #[serde(default)]
        search: crate::optimization_search::OptimizationSearchControls,
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
        observation: crate::soa_observation::SoaObservationConfig,
        #[serde(default)]
        rules: Vec<crate::soa_rule::SoaRuleConfig>,
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
        multirate: Option<crate::envelope_multirate::EnvelopeMultirateConfig>,
        #[serde(default)]
        initialization: crate::envelope_initialization::EnvelopeInitializationConfig,
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
        #[serde(default = "crate::config::default_fourier_periods")]
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

/// Stable frequency-sweep tag in worker requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkerSweepType {
    Decade,
    Octave,
    Linear,
}

impl From<AcSweepType> for WorkerSweepType {
    fn from(value: AcSweepType) -> Self {
        match value {
            AcSweepType::Decade => Self::Decade,
            AcSweepType::Octave => Self::Octave,
            AcSweepType::Linear => Self::Linear,
        }
    }
}

impl From<FrequencySweep> for WorkerSweepType {
    fn from(value: FrequencySweep) -> Self {
        match value {
            FrequencySweep::Decade => Self::Decade,
            FrequencySweep::Octave => Self::Octave,
            FrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for AcSweepType {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for FrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<&AnalysisSpec> for WorkerAnalysisSpec {
    fn from(value: &AnalysisSpec) -> Self {
        match value {
            AnalysisSpec::LegacyDcOp => Self::LegacyDcOp,
            AnalysisSpec::DcOp {
                temperature_mode,
                temperature_celsius,
                initial_guess,
                node_initialization,
                homotopy,
                annotation,
                device_detail,
                save_device_op,
                accuracy,
                selected_devices,
                previous_state,
                violation_devices,
                violation_source_content_digest,
                run_point,
            } => Self::DcOp(crate::config::OpConfig {
                temperature_mode: *temperature_mode,
                temperature_celsius: *temperature_celsius,
                initial_guess: *initial_guess,
                node_initialization: *node_initialization,
                homotopy: *homotopy,
                annotation: *annotation,
                device_detail: *device_detail,
                save_device_op: *save_device_op,
                accuracy: *accuracy,
                selected_devices: selected_devices.clone(),
                previous_state: previous_state.clone(),
                violation_devices: violation_devices.clone(),
                violation_source_content_digest: *violation_source_content_digest,
                run_point: run_point.clone(),
            }),
            AnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
                hysteresis,
                modes,
            } => Self::DcSweep {
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                hysteresis: *hysteresis,
                modes: modes.clone(),
                source2: source2.clone(),
                start2: *start2,
                stop2: *stop2,
                step2: *step2,
            },
            AnalysisSpec::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::Transient {
                stop_time: *stop_time,
                step_time: *step_time,
                start_time: *start_time,
                max_timestep: *max_timestep,
                uic: *uic,
            },
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Self::Ac {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
            },
            AnalysisSpec::AcData {
                table_name,
                frequencies,
                table_options,
            } => Self::AcData {
                table_name: table_name.clone(),
                frequencies: frequencies.clone(),
                table_options: table_options.clone(),
            },
            AnalysisSpec::Noise {
                output_node,
                reference_node,
                input_source,
                start_freq,
                stop_freq,
                points_per_decade,
                sweep,
                explicit_frequencies,
                data_table_name,
                contribution_detail,
                integration_mode,
                temperature,
            } => Self::Noise {
                output_node: output_node.clone(),
                reference_node: reference_node.clone(),
                input_source: input_source.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_decade: *points_per_decade,
                sweep: *sweep,
                explicit_frequencies: explicit_frequencies.clone(),
                data_table_name: data_table_name.clone(),
                contribution_detail: *contribution_detail,
                integration_mode: *integration_mode,
                temperature: *temperature,
            },
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
                filter,
                sweep,
            } => Self::Sensitivity {
                output_var: output_var.clone(),
                ac_mode: *ac_mode,
                frequency: *frequency,
                filter: filter.clone(),
                sweep: *sweep,
            },
            AnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => Self::PoleZero {
                input_node: input_node.clone(),
                input_ref: input_ref.clone(),
                output_node: output_node.clone(),
                output_ref: output_ref.clone(),
                transfer_type: transfer_type.clone(),
                analysis_type: analysis_type.clone(),
            },
            AnalysisSpec::Tf {
                input_source,
                output_expression,
                transfer_gain,
                input_resistance,
                output_resistance,
                normalization,
                accuracy,
            } => Self::Tf {
                input_source: input_source.clone(),
                output_expression: output_expression.clone(),
                transfer_gain: *transfer_gain,
                input_resistance: *input_resistance,
                output_resistance: *output_resistance,
                normalization: *normalization,
                accuracy: *accuracy,
            },
            AnalysisSpec::Pac => Self::Pac,
            AnalysisSpec::Pxf => Self::Pxf,
            AnalysisSpec::Pnoise => Self::Pnoise,
            AnalysisSpec::Pstb => Self::Pstb,
            AnalysisSpec::Parametric => Self::Parametric,
            AnalysisSpec::Corner => Self::Corner,
            AnalysisSpec::MonteCarlo {
                variation_source,
                params,
            } => Self::MonteCarlo {
                variation_source: *variation_source,
                params: params.clone(),
            },
            AnalysisSpec::Optimization {
                search,
                variables,
                objective_unit,
                objective_expression,
                objective_node,
                objective_ref,
                goal,
                target,
                algorithm,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            } => Self::Optimization {
                search: search.clone(),
                variables: variables.clone(),
                objective_unit: objective_unit.clone(),
                objective_expression: objective_expression.clone(),
                objective_node: objective_node.clone(),
                objective_ref: objective_ref.clone(),
                goal: *goal,
                target: *target,
                algorithm: *algorithm,
                max_iterations: *max_iterations,
                cost_tolerance: *cost_tolerance,
                fd_step: *fd_step,
                initial_step: *initial_step,
                min_step: *min_step,
            },
            AnalysisSpec::Soa {
                import_model_voltage_ratings,
                observation,
                rules,
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            } => Self::Soa {
                import_model_voltage_ratings: *import_model_voltage_ratings,
                observation: observation.clone(),
                rules: rules.clone(),
                stop_time: *stop_time,
                step_time: *step_time,
                check_vgs_max: *check_vgs_max,
                max_vgs: *max_vgs,
                check_vds_max: *check_vds_max,
                max_vds: *max_vds,
                check_vbe_max: *check_vbe_max,
                max_vbe: *max_vbe,
                check_vce_max: *check_vce_max,
                max_vce: *max_vce,
            },
            AnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                sweep,
                points_per_decade,
                compute_nyquist,
            } => Self::Stb {
                probe_node: probe_node.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                sweep: WorkerSweepType::from(*sweep),
                points_per_decade: *points_per_decade,
                compute_nyquist: *compute_nyquist,
            },
            AnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
                do_noise,
            } => Self::SParameter {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
                z0: *z0,
                ports: ports.clone(),
                do_noise: *do_noise,
            },
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            } => Self::Disto {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
                f2_over_f1: *f2_over_f1,
            },
            AnalysisSpec::Pss {
                method,
                fundamental_freq,
                tone_sources,
                tstab_periods,
                points_per_period,
                tolerance,
                oscillator_mode,
                oscillator_node,
                num_harmonics,
                integration_method,
                tstab,
                max_iterations,
                abstol,
                damping,
                max_period_change,
                verbose,
            } => Self::Pss {
                method: *method,
                fundamental_freq: *fundamental_freq,
                tone_sources: tone_sources.clone(),
                tstab_periods: *tstab_periods,
                points_per_period: *points_per_period,
                tolerance: *tolerance,
                oscillator_mode: *oscillator_mode,
                oscillator_node: oscillator_node.clone(),
                num_harmonics: *num_harmonics,
                integration_method: *integration_method,
                tstab: *tstab,
                max_iterations: *max_iterations,
                abstol: *abstol,
                damping: *damping,
                max_period_change: *max_period_change,
                verbose: *verbose,
            },
            AnalysisSpec::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                min_damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                use_exact_jacobian,
                verbose,
            } => Self::HarmonicBalance {
                tones: tones.clone(),
                reltol: *reltol,
                abstol: *abstol,
                max_iterations: *max_iterations,
                damping: *damping,
                min_damping: *min_damping,
                oversample: *oversample,
                collocation_points: *collocation_points,
                max_mixing_order: *max_mixing_order,
                use_krylov: *use_krylov,
                gmres_restart: *gmres_restart,
                source_stepping: *source_stepping,
                use_exact_jacobian: *use_exact_jacobian,
                verbose: *verbose,
            },
            AnalysisSpec::Envelope {
                multirate,
                initialization,
                fundamental_freq,
                additional_carrier_tones,
                stop_time,
                num_harmonics,
                envelope_step,
                modulation_sources,
                initial_periodic_solve,
                adaptive_mode,
                extraction_path,
            } => Self::Envelope {
                multirate: multirate.clone(),
                initialization: initialization.clone(),
                fundamental_freq: *fundamental_freq,
                additional_carrier_tones: additional_carrier_tones.clone(),
                stop_time: *stop_time,
                num_harmonics: *num_harmonics,
                envelope_step: *envelope_step,
                modulation_sources: modulation_sources.clone(),
                initial_periodic_solve: *initial_periodic_solve,
                adaptive_mode: *adaptive_mode,
                extraction_path: *extraction_path,
            },
            AnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                num_periods,
                output_node,
                output_ref,
                additional_outputs,
                start_time,
                stop_time,
                compute_thd,
                normalize,
            } => Self::Fourier {
                fundamental_freq: *fundamental_freq,
                num_harmonics: *num_harmonics,
                num_periods: *num_periods,
                output_node: output_node.clone(),
                output_ref: output_ref.clone(),
                additional_outputs: additional_outputs.clone(),
                start_time: *start_time,
                stop_time: *stop_time,
                compute_thd: *compute_thd,
                normalize: *normalize,
            },
            // Carried verbatim: these domain shapes are already stable serde
            // payloads, so a second exhaustive mirror would add no transport
            // behavior. Execution capability is still validated downstream.
            AnalysisSpec::PssSpectrum { .. }
            | AnalysisSpec::Qpss { .. }
            | AnalysisSpec::Hbsp { .. }
            | AnalysisSpec::Hbnoise { .. }
            | AnalysisSpec::Psp { .. }
            | AnalysisSpec::Qpac { .. }
            | AnalysisSpec::Qpnoise { .. }
            | AnalysisSpec::Qpxf { .. }
            | AnalysisSpec::TransientNoise { .. }
            | AnalysisSpec::DcMismatch { .. }
            // The FFT request is the card, which is already a stable serde
            // payload; a second mirror of it would add no transport behavior.
            | AnalysisSpec::Fft { .. } => Self::CanonicalSpec(value.clone()),
        }
    }
}

impl From<WorkerAnalysisSpec> for AnalysisSpec {
    fn from(value: WorkerAnalysisSpec) -> Self {
        match value {
            WorkerAnalysisSpec::LegacyDcOp => Self::LegacyDcOp,
            WorkerAnalysisSpec::DcOp(config) => Self::DcOp {
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
            },
            WorkerAnalysisSpec::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
                hysteresis,
                modes,
            } => Self::DcSweep {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
                hysteresis,
                modes,
            },
            WorkerAnalysisSpec::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            },
            WorkerAnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Self::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: FrequencySweep::from(sweep),
            },
            WorkerAnalysisSpec::AcData {
                table_name,
                frequencies,
                table_options,
            } => Self::AcData {
                table_name,
                frequencies,
                table_options,
            },
            WorkerAnalysisSpec::Noise {
                output_node,
                reference_node,
                input_source,
                start_freq,
                stop_freq,
                points_per_decade,
                sweep,
                explicit_frequencies,
                data_table_name,
                contribution_detail,
                integration_mode,
                temperature,
            } => Self::Noise {
                output_node,
                reference_node,
                input_source,
                start_freq,
                stop_freq,
                points_per_decade,
                sweep,
                explicit_frequencies,
                data_table_name,
                contribution_detail,
                integration_mode,
                temperature,
            },
            WorkerAnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
                filter,
                sweep,
            } => Self::Sensitivity {
                output_var,
                ac_mode,
                frequency,
                filter,
                sweep,
            },
            WorkerAnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => Self::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            },
            WorkerAnalysisSpec::Tf {
                input_source,
                output_expression,
                transfer_gain,
                input_resistance,
                output_resistance,
                normalization,
                accuracy,
            } => Self::Tf {
                input_source,
                output_expression,
                transfer_gain,
                input_resistance,
                output_resistance,
                normalization,
                accuracy,
            },
            WorkerAnalysisSpec::Pac => Self::Pac,
            WorkerAnalysisSpec::Pxf => Self::Pxf,
            WorkerAnalysisSpec::Pnoise => Self::Pnoise,
            WorkerAnalysisSpec::Pstb => Self::Pstb,
            WorkerAnalysisSpec::Parametric => Self::Parametric,
            WorkerAnalysisSpec::Corner => Self::Corner,
            WorkerAnalysisSpec::MonteCarlo {
                variation_source,
                params,
            } => Self::MonteCarlo {
                variation_source,
                params,
            },
            WorkerAnalysisSpec::Optimization {
                search,
                variables,
                objective_unit,
                objective_expression,
                objective_node,
                objective_ref,
                goal,
                target,
                algorithm,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            } => Self::Optimization {
                search,
                variables,
                objective_unit,
                objective_expression,
                objective_node,
                objective_ref,
                goal,
                target,
                algorithm,
                max_iterations,
                cost_tolerance,
                fd_step,
                initial_step,
                min_step,
            },
            WorkerAnalysisSpec::Soa {
                import_model_voltage_ratings,
                observation,
                rules,
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            } => Self::Soa {
                import_model_voltage_ratings,
                observation,
                rules,
                stop_time,
                step_time,
                check_vgs_max,
                max_vgs,
                check_vds_max,
                max_vds,
                check_vbe_max,
                max_vbe,
                check_vce_max,
                max_vce,
            },
            WorkerAnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                sweep,
                points_per_decade,
                compute_nyquist,
            } => Self::Stb {
                probe_node,
                start_freq,
                stop_freq,
                sweep: FrequencySweep::from(sweep),
                points_per_decade,
                compute_nyquist,
            },
            WorkerAnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
                do_noise,
            } => Self::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: FrequencySweep::from(sweep),
                z0,
                ports,
                do_noise,
            },
            WorkerAnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            } => Self::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: FrequencySweep::from(sweep),
                f2_over_f1,
            },
            WorkerAnalysisSpec::Pss {
                method,
                fundamental_freq,
                tone_sources,
                tstab_periods,
                points_per_period,
                tolerance,
                oscillator_mode,
                oscillator_node,
                num_harmonics,
                integration_method,
                tstab,
                max_iterations,
                abstol,
                damping,
                max_period_change,
                verbose,
            } => Self::Pss {
                method,
                fundamental_freq,
                tone_sources,
                tstab_periods,
                points_per_period,
                tolerance,
                oscillator_mode,
                oscillator_node,
                num_harmonics,
                integration_method,
                tstab,
                max_iterations,
                abstol,
                damping,
                max_period_change,
                verbose,
            },
            WorkerAnalysisSpec::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                min_damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                use_exact_jacobian,
                verbose,
            } => Self::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                min_damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                use_exact_jacobian,
                verbose,
            },
            WorkerAnalysisSpec::Envelope {
                multirate,
                initialization,
                fundamental_freq,
                additional_carrier_tones,
                stop_time,
                num_harmonics,
                envelope_step,
                modulation_sources,
                initial_periodic_solve,
                adaptive_mode,
                extraction_path,
            } => Self::Envelope {
                multirate,
                initialization,
                fundamental_freq,
                additional_carrier_tones,
                stop_time,
                num_harmonics,
                envelope_step,
                modulation_sources,
                initial_periodic_solve,
                adaptive_mode,
                extraction_path,
            },
            WorkerAnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                num_periods,
                output_node,
                output_ref,
                additional_outputs,
                start_time,
                stop_time,
                compute_thd,
                normalize,
            } => Self::Fourier {
                fundamental_freq,
                num_harmonics,
                num_periods,
                output_node,
                output_ref,
                additional_outputs,
                start_time,
                stop_time,
                compute_thd,
                normalize,
            },
            WorkerAnalysisSpec::CanonicalSpec(spec) => spec,
        }
    }
}
