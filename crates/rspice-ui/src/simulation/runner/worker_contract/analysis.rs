//! The analysis half of the worker contract.
//!
//! Every analysis configuration and specification that crosses the worker
//! boundary, as a `Worker*` mirror of the domain type plus the conversion in
//! each direction.
//!
//! The mirror is deliberate rather than redundant. The wire format has to
//! stay stable across a build, so it must not silently follow a domain type
//! that someone refactors — a field added to `PnoiseRunConfig` should fail to
//! compile here, not cross the boundary as a default.
//!
//! That is also why the conversions are written out exhaustively. A `..` rest
//! pattern would turn the same mistake into a wrong answer instead of a build
//! error.

use super::*;

impl TryFrom<&SimulationRequest> for WorkerSimulationRequest {
    type Error = SimulationError;

    fn try_from(value: &SimulationRequest) -> Result<Self, Self::Error> {
        match value {
            SimulationRequest::Config(config) => Ok(Self::Config(Box::new(
                WorkerAnalysisConfig::from(config.as_ref()),
            ))),
            SimulationRequest::Spec { spec, options } => Ok(Self::Spec {
                spec: Box::new(WorkerAnalysisSpec::try_from(spec.as_ref())?),
                options: Box::new(WorkerSpecExecutionOptions::from(options.as_ref())),
            }),
        }
    }
}

impl From<WorkerSimulationRequest> for SimulationRequest {
    fn from(value: WorkerSimulationRequest) -> Self {
        match value {
            WorkerSimulationRequest::Config(config) => {
                SimulationRequest::Config(Box::new(AnalysisConfig::from(*config)))
            }
            WorkerSimulationRequest::Spec { spec, options } => SimulationRequest::Spec {
                spec: Box::new(AnalysisSpec::from(*spec)),
                options: Box::new(SpecExecutionOptions::from(*options)),
            },
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerSpecExecutionOptions {
    pub temp: Option<WorkerTempRunConfig>,
    pub parametric_base: Option<WorkerCornerBaseMode>,
    pub corner: Option<WorkerCornerRunConfig>,
    pub pac: Option<WorkerPacRunConfig>,
    pub pxf: Option<WorkerPxfRunConfig>,
    pub pnoise: Option<WorkerPnoiseRunConfig>,
    pub pstb: Option<WorkerPstbRunConfig>,
}

impl From<&SpecExecutionOptions> for WorkerSpecExecutionOptions {
    fn from(value: &SpecExecutionOptions) -> Self {
        Self {
            temp: value.temp.as_ref().map(WorkerTempRunConfig::from),
            parametric_base: value
                .parametric_base
                .as_ref()
                .map(WorkerCornerBaseMode::from),
            corner: value.corner.as_ref().map(WorkerCornerRunConfig::from),
            pac: value.pac.as_ref().map(WorkerPacRunConfig::from),
            pxf: value.pxf.as_ref().map(WorkerPxfRunConfig::from),
            pnoise: value.pnoise.as_ref().map(WorkerPnoiseRunConfig::from),
            pstb: value.pstb.as_ref().map(WorkerPstbRunConfig::from),
        }
    }
}

impl From<WorkerSpecExecutionOptions> for SpecExecutionOptions {
    fn from(value: WorkerSpecExecutionOptions) -> Self {
        Self {
            temp: value
                .temp
                .map(crate::services::simulation_runner::TempRunConfig::from),
            parametric_base: value
                .parametric_base
                .map(crate::services::simulation_runner::CornerBaseMode::from),
            corner: value
                .corner
                .map(crate::services::simulation_runner::CornerRunConfig::from),
            pac: value
                .pac
                .map(crate::services::simulation_runner::PacRunConfig::from),
            pxf: value
                .pxf
                .map(crate::services::simulation_runner::PxfRunConfig::from),
            pnoise: value
                .pnoise
                .map(crate::services::simulation_runner::PnoiseRunConfig::from),
            pstb: value
                .pstb
                .map(crate::services::simulation_runner::PstbRunConfig::from),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerTempRunConfig {
    pub temperatures_c: Vec<f64>,
    pub base_mode: WorkerCornerBaseMode,
}

impl From<&crate::services::simulation_runner::TempRunConfig> for WorkerTempRunConfig {
    fn from(value: &crate::services::simulation_runner::TempRunConfig) -> Self {
        Self {
            temperatures_c: value.temperatures_c.clone(),
            base_mode: WorkerCornerBaseMode::from(&value.base_mode),
        }
    }
}

impl From<WorkerTempRunConfig> for crate::services::simulation_runner::TempRunConfig {
    fn from(value: WorkerTempRunConfig) -> Self {
        Self {
            temperatures_c: value.temperatures_c,
            base_mode: crate::services::simulation_runner::CornerBaseMode::from(value.base_mode),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerCornerRunConfig {
    pub process_corners: Vec<WorkerCornerProcess>,
    pub voltages: Vec<f64>,
    #[serde(default)]
    pub supply_source_names: Vec<String>,
    pub temperatures_c: Vec<f64>,
    pub full_matrix: bool,
    pub nominal_voltage: Option<f64>,
    pub base_mode: WorkerCornerBaseMode,
    pub model_bindings: Vec<WorkerCornerModelBinding>,
    /// The exact points to run, when the space is a filtered one. A worker that
    /// received only the axes would expand the cross product and solve the
    /// points the declaration removed.
    #[serde(default)]
    pub points: Vec<WorkerCornerPoint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerCornerPoint {
    pub process: WorkerCornerProcess,
    pub voltage: f64,
    pub temperature_c: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct WorkerCornerModelBinding {
    pub process: WorkerCornerProcess,
    pub source_label: String,
    pub section: Option<String>,
    pub materialized_model_cards: String,
}

impl From<&crate::services::simulation_runner::CornerModelBinding> for WorkerCornerModelBinding {
    fn from(value: &crate::services::simulation_runner::CornerModelBinding) -> Self {
        Self {
            process: WorkerCornerProcess::from(value.process),
            source_label: value.source_label.clone(),
            section: value.section.clone(),
            materialized_model_cards: value.materialized_model_cards.clone(),
        }
    }
}

impl From<WorkerCornerModelBinding> for crate::services::simulation_runner::CornerModelBinding {
    fn from(value: WorkerCornerModelBinding) -> Self {
        Self {
            process: crate::services::simulation_runner::CornerProcess::from(value.process),
            source_label: value.source_label,
            section: value.section,
            materialized_model_cards: value.materialized_model_cards,
        }
    }
}

impl From<&crate::services::simulation_runner::CornerRunConfig> for WorkerCornerRunConfig {
    fn from(value: &crate::services::simulation_runner::CornerRunConfig) -> Self {
        Self {
            process_corners: value
                .process_corners
                .iter()
                .copied()
                .map(WorkerCornerProcess::from)
                .collect(),
            voltages: value.voltages.clone(),
            supply_source_names: value.supply_source_names.clone(),
            temperatures_c: value.temperatures_c.clone(),
            full_matrix: value.full_matrix,
            nominal_voltage: value.nominal_voltage,
            base_mode: WorkerCornerBaseMode::from(&value.base_mode),
            model_bindings: value
                .model_bindings
                .iter()
                .map(WorkerCornerModelBinding::from)
                .collect(),
            points: value
                .points
                .iter()
                .map(|point| WorkerCornerPoint {
                    process: WorkerCornerProcess::from(point.process),
                    voltage: point.voltage,
                    temperature_c: point.temperature_c,
                })
                .collect(),
        }
    }
}

impl From<WorkerCornerRunConfig> for crate::services::simulation_runner::CornerRunConfig {
    fn from(value: WorkerCornerRunConfig) -> Self {
        Self {
            process_corners: value
                .process_corners
                .into_iter()
                .map(crate::services::simulation_runner::CornerProcess::from)
                .collect(),
            voltages: value.voltages,
            supply_source_names: value.supply_source_names,
            temperatures_c: value.temperatures_c,
            full_matrix: value.full_matrix,
            nominal_voltage: value.nominal_voltage,
            base_mode: crate::services::simulation_runner::CornerBaseMode::from(value.base_mode),
            model_bindings: value
                .model_bindings
                .into_iter()
                .map(crate::services::simulation_runner::CornerModelBinding::from)
                .collect(),
            points: value
                .points
                .into_iter()
                .map(|point| crate::services::simulation_runner::CornerPoint {
                    process: crate::services::simulation_runner::CornerProcess::from(point.process),
                    voltage: point.voltage,
                    temperature_c: point.temperature_c,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerCornerBaseMode {
    Op,
    DcSweep {
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
    },
    DcSweepNested {
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
        source2: String,
        start2: f64,
        stop2: f64,
        step2: f64,
    },
    Transient {
        stop_time: f64,
        step_time: f64,
    },
    TransientWindow {
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
}

impl From<&crate::services::simulation_runner::CornerBaseMode> for WorkerCornerBaseMode {
    fn from(value: &crate::services::simulation_runner::CornerBaseMode) -> Self {
        match value {
            crate::services::simulation_runner::CornerBaseMode::Op => Self::Op,
            crate::services::simulation_runner::CornerBaseMode::DcSweep {
                source_name,
                start,
                stop,
                step,
            } => Self::DcSweep {
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
            },
            crate::services::simulation_runner::CornerBaseMode::DcSweepNested {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Self::DcSweepNested {
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                source2: source2.clone(),
                start2: *start2,
                stop2: *stop2,
                step2: *step2,
            },
            crate::services::simulation_runner::CornerBaseMode::Transient {
                stop_time,
                step_time,
            } => Self::Transient {
                stop_time: *stop_time,
                step_time: *step_time,
            },
            crate::services::simulation_runner::CornerBaseMode::TransientWindow {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::TransientWindow {
                stop_time: *stop_time,
                step_time: *step_time,
                start_time: *start_time,
                max_timestep: *max_timestep,
                uic: *uic,
            },
            crate::services::simulation_runner::CornerBaseMode::Ac {
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
        }
    }
}

impl From<WorkerCornerBaseMode> for crate::services::simulation_runner::CornerBaseMode {
    fn from(value: WorkerCornerBaseMode) -> Self {
        match value {
            WorkerCornerBaseMode::Op => Self::Op,
            WorkerCornerBaseMode::DcSweep {
                source_name,
                start,
                stop,
                step,
            } => Self::DcSweep {
                source_name,
                start,
                stop,
                step,
            },
            WorkerCornerBaseMode::DcSweepNested {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Self::DcSweepNested {
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            },
            WorkerCornerBaseMode::Transient {
                stop_time,
                step_time,
            } => Self::Transient {
                stop_time,
                step_time,
            },
            WorkerCornerBaseMode::TransientWindow {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::TransientWindow {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            },
            WorkerCornerBaseMode::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Self::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: crate::services::simulation_runner::CornerFrequencySweep::from(sweep),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPacRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub max_sideband: i32,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub pac_magnitude: f64,
    pub include_dc: bool,
    pub reltol: f64,
    pub abstol: f64,
}

impl From<&crate::services::simulation_runner::PacRunConfig> for WorkerPacRunConfig {
    fn from(value: &crate::services::simulation_runner::PacRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            max_sideband: value.max_sideband,
            input_source: value.input_source.clone(),
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            pac_magnitude: value.pac_magnitude,
            include_dc: value.include_dc,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

impl From<WorkerPacRunConfig> for crate::services::simulation_runner::PacRunConfig {
    fn from(value: WorkerPacRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: crate::services::simulation_runner::PacFrequencySweep::from(value.sweep),
            max_sideband: value.max_sideband,
            input_source: value.input_source,
            output_node: value.output_node,
            output_ref: value.output_ref,
            pac_magnitude: value.pac_magnitude,
            include_dc: value.include_dc,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPxfRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub input_source: String,
    pub input_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub output_sideband: i32,
    pub max_sideband: i32,
    pub reltol: f64,
    pub abstol: f64,
}

impl From<&crate::services::simulation_runner::PxfRunConfig> for WorkerPxfRunConfig {
    fn from(value: &crate::services::simulation_runner::PxfRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            input_source: value.input_source.clone(),
            input_sideband: value.input_sideband,
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            output_sideband: value.output_sideband,
            max_sideband: value.max_sideband,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

impl From<WorkerPxfRunConfig> for crate::services::simulation_runner::PxfRunConfig {
    fn from(value: WorkerPxfRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: crate::services::simulation_runner::PxfFrequencySweep::from(value.sweep),
            input_source: value.input_source,
            input_sideband: value.input_sideband,
            output_node: value.output_node,
            output_ref: value.output_ref,
            output_sideband: value.output_sideband,
            max_sideband: value.max_sideband,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPnoiseRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub start_freq: f64,
    pub stop_freq: f64,
    pub points_per_unit: usize,
    pub sweep: WorkerSweepType,
    pub max_sideband: i32,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub input_source: String,
    pub noise_ref: WorkerPnoiseReference,
    pub integrated_noise: bool,
    pub noise_summary: bool,
    pub reltol: f64,
    pub abstol: f64,
}

impl From<&crate::services::simulation_runner::PnoiseRunConfig> for WorkerPnoiseRunConfig {
    fn from(value: &crate::services::simulation_runner::PnoiseRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: WorkerSweepType::from(value.sweep),
            max_sideband: value.max_sideband,
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            input_source: value.input_source.clone(),
            noise_ref: WorkerPnoiseReference::from(value.noise_ref),
            integrated_noise: value.integrated_noise,
            noise_summary: value.noise_summary,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

impl From<WorkerPnoiseRunConfig> for crate::services::simulation_runner::PnoiseRunConfig {
    fn from(value: WorkerPnoiseRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            start_freq: value.start_freq,
            stop_freq: value.stop_freq,
            points_per_unit: value.points_per_unit,
            sweep: crate::services::simulation_runner::PnoiseFrequencySweep::from(value.sweep),
            max_sideband: value.max_sideband,
            output_node: value.output_node,
            output_ref: value.output_ref,
            input_source: value.input_source,
            noise_ref: crate::services::simulation_runner::PnoiseReference::from(value.noise_ref),
            integrated_noise: value.integrated_noise,
            noise_summary: value.noise_summary,
            reltol: value.reltol,
            abstol: value.abstol,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPstbRunConfig {
    pub pss_fundamental_freq: f64,
    pub pss_num_harmonics: usize,
    pub pss_tolerance: f64,
    pub probe_instance: String,
    pub max_harmonics: usize,
    pub num_multipliers: usize,
    pub stability_threshold: f64,
    pub detect_subharmonics: bool,
    pub eigenvalue_tolerance: f64,
}

impl From<&crate::services::simulation_runner::PstbRunConfig> for WorkerPstbRunConfig {
    fn from(value: &crate::services::simulation_runner::PstbRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            probe_instance: value.probe_instance.clone(),
            max_harmonics: value.max_harmonics,
            num_multipliers: value.num_multipliers,
            stability_threshold: value.stability_threshold,
            detect_subharmonics: value.detect_subharmonics,
            eigenvalue_tolerance: value.eigenvalue_tolerance,
        }
    }
}

impl From<WorkerPstbRunConfig> for crate::services::simulation_runner::PstbRunConfig {
    fn from(value: WorkerPstbRunConfig) -> Self {
        Self {
            pss_fundamental_freq: value.pss_fundamental_freq,
            pss_num_harmonics: value.pss_num_harmonics,
            pss_tolerance: value.pss_tolerance,
            probe_instance: value.probe_instance,
            max_harmonics: value.max_harmonics,
            num_multipliers: value.num_multipliers,
            stability_threshold: value.stability_threshold,
            detect_subharmonics: value.detect_subharmonics,
            eigenvalue_tolerance: value.eigenvalue_tolerance,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum WorkerAnalysisConfig {
    #[serde(rename = "DcOp")]
    LegacyDcOp,
    #[serde(rename = "DcOpConfigured")]
    DcOp(crate::simulation::dialog::OpConfig),
    DcSweep {
        source: String,
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
    },
    Transient {
        stop_time: f64,
        step_time: f64,
        start_time: f64,
        max_timestep: Option<f64>,
        uic: bool,
    },
    Ac {
        sweep_type: WorkerSweepType,
        num_points: usize,
        start_freq: f64,
        stop_freq: f64,
    },
    Noise {
        output_node: String,
        reference_node: String,
        input_source: String,
        sweep_type: WorkerSweepType,
        num_points: usize,
        start_freq: f64,
        stop_freq: f64,
        #[serde(default)]
        explicit_frequencies: Option<Vec<f64>>,
        #[serde(default)]
        data_table_name: Option<String>,
        #[serde(default)]
        contribution_detail: NoiseContributionDetail,
        #[serde(default)]
        integration_mode: NoiseIntegrationMode,
        #[serde(default = "worker_default_noise_temperature")]
        temperature_kelvin: f64,
    },
    PoleZero {
        input_node: String,
        input_ref: String,
        output_node: String,
        output_ref: String,
        transfer_type: String,
        analysis_type: WorkerPzAnalysisType,
    },
    Sensitivity {
        output_var: String,
        ac_mode: bool,
        frequency: Option<f64>,
    },
}

impl From<&AnalysisConfig> for WorkerAnalysisConfig {
    fn from(value: &AnalysisConfig) -> Self {
        match value {
            AnalysisConfig::DcOp(config) => Self::DcOp(config.clone()),
            AnalysisConfig::DcSweep(config) => Self::DcSweep {
                source: config.source.clone(),
                start: config.start,
                stop: config.stop,
                step: config.step,
                source2: config.source2.clone(),
                start2: config.start2,
                stop2: config.stop2,
                step2: config.step2,
                hysteresis: config.hysteresis,
            },
            AnalysisConfig::Transient(config) => Self::Transient {
                stop_time: config.stop_time,
                step_time: config.step_time,
                start_time: config.start_time,
                max_timestep: config.max_timestep,
                uic: config.uic,
            },
            AnalysisConfig::Ac(config) => Self::Ac {
                sweep_type: WorkerSweepType::from(config.sweep_type),
                num_points: config.num_points,
                start_freq: config.start_freq,
                stop_freq: config.stop_freq,
            },
            AnalysisConfig::Noise(config) => Self::Noise {
                output_node: config.output_node.clone(),
                reference_node: config.reference_node.clone(),
                input_source: config.input_source.clone(),
                sweep_type: WorkerSweepType::from(config.sweep_type),
                num_points: config.num_points,
                start_freq: config.start_freq,
                stop_freq: config.stop_freq,
                explicit_frequencies: config.explicit_frequencies.clone(),
                data_table_name: config.data_table_name.clone(),
                contribution_detail: config.contribution_detail,
                integration_mode: config.integration_mode,
                temperature_kelvin: config.temperature_kelvin,
            },
            AnalysisConfig::PoleZero(config) => Self::PoleZero {
                input_node: config.input_node.clone(),
                input_ref: config.input_ref.clone(),
                output_node: config.output_node.clone(),
                output_ref: config.output_ref.clone(),
                transfer_type: config.transfer_type.clone(),
                analysis_type: WorkerPzAnalysisType::from(config.analysis_type),
            },
            AnalysisConfig::Sensitivity(config) => Self::Sensitivity {
                output_var: config.output_var.clone(),
                ac_mode: config.ac_mode,
                frequency: config.frequency,
            },
        }
    }
}

impl From<WorkerAnalysisConfig> for AnalysisConfig {
    fn from(value: WorkerAnalysisConfig) -> Self {
        match value {
            WorkerAnalysisConfig::LegacyDcOp => Self::dc_op(),
            WorkerAnalysisConfig::DcOp(config) => Self::DcOp(config),
            WorkerAnalysisConfig::DcSweep {
                source,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
                hysteresis,
            } => Self::DcSweep(DcSweepConfig {
                source,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
                hysteresis,
            }),
            WorkerAnalysisConfig::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Self::Transient(TransientAnalysisConfig {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            }),
            WorkerAnalysisConfig::Ac {
                sweep_type,
                num_points,
                start_freq,
                stop_freq,
            } => Self::Ac(AcAnalysisConfig {
                sweep_type: AcSweepType::from(sweep_type),
                num_points,
                start_freq,
                stop_freq,
            }),
            WorkerAnalysisConfig::Noise {
                output_node,
                reference_node,
                input_source,
                sweep_type,
                num_points,
                start_freq,
                stop_freq,
                explicit_frequencies,
                data_table_name,
                contribution_detail,
                integration_mode,
                temperature_kelvin,
            } => Self::Noise(NoiseAnalysisConfig {
                output_node,
                reference_node,
                input_source,
                sweep_type: AcSweepType::from(sweep_type),
                num_points,
                start_freq,
                stop_freq,
                explicit_frequencies,
                data_table_name,
                contribution_detail,
                integration_mode,
                temperature_kelvin,
            }),
            WorkerAnalysisConfig::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => Self::PoleZero(PoleZeroConfig {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type: PzAnalysisType::from(analysis_type),
            }),
            WorkerAnalysisConfig::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => Self::Sensitivity(SensitivityConfig {
                output_var,
                ac_mode,
                frequency,
            }),
        }
    }
}

impl TryFrom<&AnalysisSpec> for WorkerAnalysisSpec {
    type Error = SimulationError;

    fn try_from(value: &AnalysisSpec) -> Result<Self, Self::Error> {
        match value {
            AnalysisSpec::LegacyDcOp => Ok(Self::LegacyDcOp),
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
            } => Ok(Self::DcOp(crate::simulation::dialog::OpConfig {
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
            })),
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
            } => Ok(Self::DcSweep {
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                hysteresis: *hysteresis,
                source2: source2.clone(),
                start2: *start2,
                stop2: *stop2,
                step2: *step2,
            }),
            AnalysisSpec::Transient {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } => Ok(Self::Transient {
                stop_time: *stop_time,
                step_time: *step_time,
                start_time: *start_time,
                max_timestep: *max_timestep,
                uic: *uic,
            }),
            AnalysisSpec::Ac {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
            } => Ok(Self::Ac {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
            }),
            AnalysisSpec::AcData {
                table_name,
                frequencies,
            } => Ok(Self::AcData {
                table_name: table_name.clone(),
                frequencies: frequencies.clone(),
            }),
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
            } => Ok(Self::Noise {
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
            }),
            AnalysisSpec::Sensitivity {
                output_var,
                ac_mode,
                frequency,
            } => Ok(Self::Sensitivity {
                output_var: output_var.clone(),
                ac_mode: *ac_mode,
                frequency: *frequency,
            }),
            AnalysisSpec::PoleZero {
                input_node,
                input_ref,
                output_node,
                output_ref,
                transfer_type,
                analysis_type,
            } => Ok(Self::PoleZero {
                input_node: input_node.clone(),
                input_ref: input_ref.clone(),
                output_node: output_node.clone(),
                output_ref: output_ref.clone(),
                transfer_type: transfer_type.clone(),
                analysis_type: analysis_type.clone(),
            }),
            AnalysisSpec::Tf {
                input_source,
                output_expression,
                transfer_gain,
                input_resistance,
                output_resistance,
                normalization,
                accuracy,
            } => Ok(Self::Tf {
                input_source: input_source.clone(),
                output_expression: output_expression.clone(),
                transfer_gain: *transfer_gain,
                input_resistance: *input_resistance,
                output_resistance: *output_resistance,
                normalization: *normalization,
                accuracy: *accuracy,
            }),
            AnalysisSpec::Pac => Ok(Self::Pac),
            AnalysisSpec::Pxf => Ok(Self::Pxf),
            AnalysisSpec::Pnoise => Ok(Self::Pnoise),
            AnalysisSpec::Pstb => Ok(Self::Pstb),
            AnalysisSpec::Parametric => Ok(Self::Parametric),
            AnalysisSpec::Corner => Ok(Self::Corner),
            AnalysisSpec::MonteCarlo { variation_source } => Ok(Self::MonteCarlo {
                variation_source: *variation_source,
            }),
            AnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => Ok(Self::Reliability {
                target_years: target_years.clone(),
                enable_hci: *enable_hci,
                enable_nbti: *enable_nbti,
                enable_em: *enable_em,
                min_stress_voltage: *min_stress_voltage,
            }),
            AnalysisSpec::Optimization {
                variables,
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
            } => Ok(Self::Optimization {
                variables: variables.clone(),
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
            }),
            AnalysisSpec::Soa {
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
            } => Ok(Self::Soa {
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
            }),
            AnalysisSpec::Stb {
                probe_node,
                start_freq,
                stop_freq,
                sweep,
                points_per_decade,
                compute_nyquist,
            } => Ok(Self::Stb {
                probe_node: probe_node.clone(),
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                sweep: WorkerSweepType::from(*sweep),
                points_per_decade: *points_per_decade,
                compute_nyquist: *compute_nyquist,
            }),
            AnalysisSpec::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
            } => Ok(Self::SParameter {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
                z0: *z0,
                ports: ports.clone(),
            }),
            AnalysisSpec::Disto {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                f2_over_f1,
            } => Ok(Self::Disto {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points_per_unit: *points_per_unit,
                sweep: WorkerSweepType::from(*sweep),
                f2_over_f1: *f2_over_f1,
            }),
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
            } => Ok(Self::Pss {
                method: *method,
                fundamental_freq: *fundamental_freq,
                tone_sources: tone_sources.clone(),
                tstab_periods: *tstab_periods,
                points_per_period: *points_per_period,
                tolerance: *tolerance,
                oscillator_mode: *oscillator_mode,
                oscillator_node: oscillator_node.clone(),
                num_harmonics: *num_harmonics,
            }),
            AnalysisSpec::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            } => Ok(Self::HarmonicBalance {
                tones: tones.clone(),
                reltol: *reltol,
                abstol: *abstol,
                max_iterations: *max_iterations,
                damping: *damping,
                oversample: *oversample,
                collocation_points: *collocation_points,
                max_mixing_order: *max_mixing_order,
                use_krylov: *use_krylov,
                gmres_restart: *gmres_restart,
                source_stepping: *source_stepping,
                verbose: *verbose,
            }),
            AnalysisSpec::Envelope {
                fundamental_freq,
                additional_carrier_tones,
                stop_time,
                num_harmonics,
                envelope_step,
                modulation_sources,
                initial_periodic_solve,
                adaptive_mode,
                extraction_path,
            } => Ok(Self::Envelope {
                fundamental_freq: *fundamental_freq,
                additional_carrier_tones: additional_carrier_tones.clone(),
                stop_time: *stop_time,
                num_harmonics: *num_harmonics,
                envelope_step: *envelope_step,
                modulation_sources: modulation_sources.clone(),
                initial_periodic_solve: *initial_periodic_solve,
                adaptive_mode: *adaptive_mode,
                extraction_path: *extraction_path,
            }),
            AnalysisSpec::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
                compute_thd,
                normalize,
            } => Ok(Self::Fourier {
                fundamental_freq: *fundamental_freq,
                num_harmonics: *num_harmonics,
                output_node: output_node.clone(),
                output_ref: output_ref.clone(),
                start_time: *start_time,
                stop_time: *stop_time,
                compute_thd: *compute_thd,
                normalize: *normalize,
            }),
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
            | AnalysisSpec::DcMismatch { .. } => Ok(Self::CanonicalSpec(value.clone())),
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
            } => Self::AcData {
                table_name,
                frequencies,
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
            } => Self::Sensitivity {
                output_var,
                ac_mode,
                frequency,
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
            WorkerAnalysisSpec::MonteCarlo { variation_source } => {
                Self::MonteCarlo { variation_source }
            }
            WorkerAnalysisSpec::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            } => Self::Reliability {
                target_years,
                enable_hci,
                enable_nbti,
                enable_em,
                min_stress_voltage,
            },
            WorkerAnalysisSpec::Optimization {
                variables,
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
                variables,
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
            } => Self::SParameter {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep: FrequencySweep::from(sweep),
                z0,
                ports,
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
            },
            WorkerAnalysisSpec::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            } => Self::HarmonicBalance {
                tones,
                reltol,
                abstol,
                max_iterations,
                damping,
                oversample,
                collocation_points,
                max_mixing_order,
                use_krylov,
                gmres_restart,
                source_stepping,
                verbose,
            },
            WorkerAnalysisSpec::Envelope {
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
                output_node,
                output_ref,
                start_time,
                stop_time,
                compute_thd,
                normalize,
            } => Self::Fourier {
                fundamental_freq,
                num_harmonics,
                output_node,
                output_ref,
                start_time,
                stop_time,
                compute_thd,
                normalize,
            },
            WorkerAnalysisSpec::CanonicalSpec(spec) => spec,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerSweepType {
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

impl From<crate::services::simulation_runner::PacFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::PacFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::PacFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::PacFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::PacFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<crate::services::simulation_runner::PxfFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::PxfFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::PxfFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::PxfFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::PxfFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<crate::services::simulation_runner::PnoiseFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::PnoiseFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::PnoiseFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::PnoiseFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::PnoiseFrequencySweep::Linear => Self::Linear,
        }
    }
}

impl From<crate::services::simulation_runner::CornerFrequencySweep> for WorkerSweepType {
    fn from(value: crate::services::simulation_runner::CornerFrequencySweep) -> Self {
        match value {
            crate::services::simulation_runner::CornerFrequencySweep::Decade => Self::Decade,
            crate::services::simulation_runner::CornerFrequencySweep::Octave => Self::Octave,
            crate::services::simulation_runner::CornerFrequencySweep::Linear => Self::Linear,
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

impl From<WorkerSweepType> for crate::services::simulation_runner::PacFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for crate::services::simulation_runner::PxfFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for crate::services::simulation_runner::PnoiseFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

impl From<WorkerSweepType> for crate::services::simulation_runner::CornerFrequencySweep {
    fn from(value: WorkerSweepType) -> Self {
        match value {
            WorkerSweepType::Decade => Self::Decade,
            WorkerSweepType::Octave => Self::Octave,
            WorkerSweepType::Linear => Self::Linear,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerCornerProcess {
    TT,
    SS,
    FF,
    SF,
    FS,
}

impl From<crate::services::simulation_runner::CornerProcess> for WorkerCornerProcess {
    fn from(value: crate::services::simulation_runner::CornerProcess) -> Self {
        match value {
            crate::services::simulation_runner::CornerProcess::TT => Self::TT,
            crate::services::simulation_runner::CornerProcess::SS => Self::SS,
            crate::services::simulation_runner::CornerProcess::FF => Self::FF,
            crate::services::simulation_runner::CornerProcess::SF => Self::SF,
            crate::services::simulation_runner::CornerProcess::FS => Self::FS,
        }
    }
}

impl From<WorkerCornerProcess> for crate::services::simulation_runner::CornerProcess {
    fn from(value: WorkerCornerProcess) -> Self {
        match value {
            WorkerCornerProcess::TT => Self::TT,
            WorkerCornerProcess::SS => Self::SS,
            WorkerCornerProcess::FF => Self::FF,
            WorkerCornerProcess::SF => Self::SF,
            WorkerCornerProcess::FS => Self::FS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerPnoiseReference {
    Output,
    Input,
    Phase,
}

impl From<crate::services::simulation_runner::PnoiseReference> for WorkerPnoiseReference {
    fn from(value: crate::services::simulation_runner::PnoiseReference) -> Self {
        match value {
            crate::services::simulation_runner::PnoiseReference::Output => Self::Output,
            crate::services::simulation_runner::PnoiseReference::Input => Self::Input,
            crate::services::simulation_runner::PnoiseReference::Phase => Self::Phase,
        }
    }
}

impl From<WorkerPnoiseReference> for crate::services::simulation_runner::PnoiseReference {
    fn from(value: WorkerPnoiseReference) -> Self {
        match value {
            WorkerPnoiseReference::Output => Self::Output,
            WorkerPnoiseReference::Input => Self::Input,
            WorkerPnoiseReference::Phase => Self::Phase,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerPzAnalysisType {
    PoleZero,
    PolesOnly,
    ZerosOnly,
}

impl From<PzAnalysisType> for WorkerPzAnalysisType {
    fn from(value: PzAnalysisType) -> Self {
        match value {
            PzAnalysisType::PoleZero => Self::PoleZero,
            PzAnalysisType::PolesOnly => Self::PolesOnly,
            PzAnalysisType::ZerosOnly => Self::ZerosOnly,
        }
    }
}

impl From<WorkerPzAnalysisType> for PzAnalysisType {
    fn from(value: WorkerPzAnalysisType) -> Self {
        match value {
            WorkerPzAnalysisType::PoleZero => Self::PoleZero,
            WorkerPzAnalysisType::PolesOnly => Self::PolesOnly,
            WorkerPzAnalysisType::ZerosOnly => Self::ZerosOnly,
        }
    }
}
