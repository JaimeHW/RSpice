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
                spec: Box::new(WorkerAnalysisSpec::from(spec.as_ref())),
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
    #[serde(default)]
    pub study_base: Option<WorkerStudyRunConfig>,
    #[serde(default)]
    pub mc_checkpoint:
        Option<crate::simulation::runner::monte_carlo_checkpoint::MonteCarloCheckpointRequest>,
    #[serde(default)]
    pub mc_histogram_bins: Option<usize>,
    #[serde(default)]
    pub mc_statistics: Option<crate::simulation::dialog::mc::statistics::McStatisticsConfig>,
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
            mc_histogram_bins: value.mc_histogram_bins,
            mc_statistics: value.mc_statistics.clone(),
            mc_checkpoint: value.mc_checkpoint.clone(),
            study_base: value.study_base.as_ref().map(WorkerStudyRunConfig::from),
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
            mc_histogram_bins: value.mc_histogram_bins,
            mc_statistics: value.mc_statistics,
            mc_checkpoint: value.mc_checkpoint,
            study_base: value
                .study_base
                .map(crate::simulation::runner::study::StudyRunConfig::from),
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

/// Basic analyses retain their existing wire representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum WorkerStudyAnalysis {
    Basic(WorkerAnalysisConfig),
    Native {
        native_spec: AnalysisSpec,
    },
    Pss {
        pss: Box<crate::simulation::runner::study::StudyPssConfig>,
    },
    Qpss {
        qpss: Box<crate::simulation::runner::study::StudyQpssConfig>,
    },
    Hb {
        hb: Box<crate::simulation::runner::study::StudyHbConfig>,
    },
}

impl WorkerStudyAnalysis {
    pub(super) fn op_config(&self) -> Option<&crate::simulation::dialog::OpConfig> {
        match self {
            Self::Basic(WorkerAnalysisConfig::DcOp(config)) => Some(config),
            Self::Pss { pss } => Some(&pss.operating_point.config),
            Self::Qpss { qpss } => Some(&qpss.operating_point.config),
            Self::Hb { hb } => Some(&hb.operating_point.config),
            _ => None,
        }
    }
    pub(super) fn op_config_mut(&mut self) -> Option<&mut crate::simulation::dialog::OpConfig> {
        match self {
            Self::Basic(WorkerAnalysisConfig::DcOp(config)) => Some(config),
            Self::Pss { pss } => Some(&mut pss.operating_point.config),
            Self::Qpss { qpss } => Some(&mut qpss.operating_point.config),
            Self::Hb { hb } => Some(&mut hb.operating_point.config),
            _ => None,
        }
    }
}

impl From<&crate::simulation::runner::study::StudyAnalysis> for WorkerStudyAnalysis {
    fn from(value: &crate::simulation::runner::study::StudyAnalysis) -> Self {
        use crate::simulation::runner::study::StudyAnalysis;
        match value {
            StudyAnalysis::Basic(config) => Self::Basic(WorkerAnalysisConfig::from(config)),
            StudyAnalysis::Pss(pss) => Self::Pss { pss: pss.clone() },
            StudyAnalysis::Qpss(qpss) => Self::Qpss { qpss: qpss.clone() },
            StudyAnalysis::Hb(hb) => Self::Hb { hb: hb.clone() },
            StudyAnalysis::Native(spec) => Self::Native {
                native_spec: spec.clone(),
            },
        }
    }
}

impl From<WorkerStudyAnalysis> for crate::simulation::runner::study::StudyAnalysis {
    fn from(value: WorkerStudyAnalysis) -> Self {
        match value {
            WorkerStudyAnalysis::Basic(config) => Self::Basic(AnalysisConfig::from(config)),
            WorkerStudyAnalysis::Native { native_spec } => Self::Native(native_spec),
            WorkerStudyAnalysis::Pss { pss } => Self::Pss(pss),
            WorkerStudyAnalysis::Qpss { qpss } => Self::Qpss(qpss),
            WorkerStudyAnalysis::Hb { hb } => Self::Hb(hb),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerStudyRunConfig {
    #[serde(default)]
    postprocess: Option<crate::simulation::runner::study::StudyPostprocess>,
    instance_id: crate::product::AnalysisInstanceId,
    source_revision: crate::product::ObjectRevision,
    pub(super) analysis: WorkerStudyAnalysis,
    analysis_line: String,
    numeric_options: String,
    measurements: Vec<String>,
    histogram_bins: usize,
    #[serde(default)]
    objective_terms: Vec<crate::simulation::optimizer::OptimizationObjectiveTerm>,
    #[serde(default)]
    constraints: Vec<crate::simulation::optimizer::OptimizationConstraint>,
}

impl From<&crate::simulation::runner::study::StudyRunConfig> for WorkerStudyRunConfig {
    fn from(value: &crate::simulation::runner::study::StudyRunConfig) -> Self {
        let crate::simulation::runner::study::StudyRunConfig {
            postprocess,
            instance_id,
            source_revision,
            analysis,
            analysis_line,
            numeric_options,
            measurements,
            histogram_bins,
            objective_terms,
            constraints,
        } = value;
        Self {
            postprocess: postprocess.clone(),
            instance_id: *instance_id,
            source_revision: *source_revision,
            analysis: WorkerStudyAnalysis::from(analysis),
            analysis_line: analysis_line.clone(),
            numeric_options: numeric_options.clone(),
            measurements: measurements.clone(),
            histogram_bins: *histogram_bins,
            objective_terms: objective_terms.clone(),
            constraints: constraints.clone(),
        }
    }
}

impl From<WorkerStudyRunConfig> for crate::simulation::runner::study::StudyRunConfig {
    fn from(value: WorkerStudyRunConfig) -> Self {
        let WorkerStudyRunConfig {
            postprocess,
            instance_id,
            source_revision,
            analysis,
            analysis_line,
            numeric_options,
            measurements,
            histogram_bins,
            objective_terms,
            constraints,
        } = value;
        Self {
            postprocess,
            instance_id,
            source_revision,
            analysis: analysis.into(),
            analysis_line,
            numeric_options,
            measurements,
            histogram_bins,
            objective_terms,
            constraints,
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

impl From<&rspice_model_library::CornerModelBinding> for WorkerCornerModelBinding {
    fn from(value: &rspice_model_library::CornerModelBinding) -> Self {
        Self {
            process: WorkerCornerProcess::from(value.process),
            source_label: value.source_label.clone(),
            section: value.section.clone(),
            materialized_model_cards: value.materialized_model_cards.clone(),
        }
    }
}

impl From<WorkerCornerModelBinding> for rspice_model_library::CornerModelBinding {
    fn from(value: WorkerCornerModelBinding) -> Self {
        Self {
            process: rspice_app_types::product::ProcessCorner::from(value.process),
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
                .map(rspice_app_types::product::ProcessCorner::from)
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
                .map(rspice_model_library::CornerModelBinding::from)
                .collect(),
            points: value
                .points
                .into_iter()
                .map(|point| crate::services::simulation_runner::CornerPoint {
                    process: rspice_app_types::product::ProcessCorner::from(point.process),
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
    ConfiguredOp(Box<crate::simulation::dialog::OpConfig>),
    DcSweep {
        #[serde(default)]
        modes: crate::simulation::config::DcSweepModes,
        source_name: String,
        start: f64,
        stop: f64,
        step: f64,
    },
    DcSweepNested {
        #[serde(default)]
        modes: crate::simulation::config::DcSweepModes,
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
            crate::services::simulation_runner::CornerBaseMode::ConfiguredOp(config) => {
                Self::ConfiguredOp(config.clone())
            }
            crate::services::simulation_runner::CornerBaseMode::DcSweep {
                modes,
                source_name,
                start,
                stop,
                step,
            } => Self::DcSweep {
                modes: modes.clone(),
                source_name: source_name.clone(),
                start: *start,
                stop: *stop,
                step: *step,
            },
            crate::services::simulation_runner::CornerBaseMode::DcSweepNested {
                modes,
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Self::DcSweepNested {
                modes: modes.clone(),
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
            WorkerCornerBaseMode::ConfiguredOp(config) => Self::ConfiguredOp(config),
            WorkerCornerBaseMode::DcSweep {
                modes,
                source_name,
                start,
                stop,
                step,
            } => Self::DcSweep {
                modes,
                source_name,
                start,
                stop,
                step,
            },
            WorkerCornerBaseMode::DcSweepNested {
                modes,
                source_name,
                start,
                stop,
                step,
                source2,
                start2,
                stop2,
                step2,
            } => Self::DcSweepNested {
                modes,
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
    /// The top of the sideband range, under the wire name the single symmetric
    /// bound had: for every symmetric run the two are the same number, so an
    /// older worker's request decodes as exactly the run it described.
    #[serde(rename = "max_sideband")]
    pub sideband_max: i32,
    /// The bottom of the range. Absent on a request an older worker wrote, and
    /// absent means the symmetric `-max`.
    #[serde(default)]
    pub sideband_min: Option<i32>,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub pac_magnitude: f64,
    pub include_dc: bool,
    pub reltol: f64,
    pub abstol: f64,
    /// The carrier the run linearizes around. Absent on a request an older
    /// worker wrote, and absent is the preceding periodic solve.
    #[serde(default)]
    pub carrier: WorkerPeriodicCarrier,
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
            sideband_max: value.sideband_max,
            // Written only where the range is not symmetric, so a symmetric
            // request is the same document it has always been.
            sideband_min: (value.sideband_min != -value.sideband_max).then_some(value.sideband_min),
            input_source: value.input_source.clone(),
            output_node: value.output_node.clone(),
            output_ref: value.output_ref.clone(),
            pac_magnitude: value.pac_magnitude,
            include_dc: value.include_dc,
            reltol: value.reltol,
            abstol: value.abstol,
            carrier: WorkerPeriodicCarrier::from(value.carrier),
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
            sideband_max: value.sideband_max,
            sideband_min: value.sideband_min.unwrap_or(-value.sideband_max),
            input_source: value.input_source,
            output_node: value.output_node,
            output_ref: value.output_ref,
            pac_magnitude: value.pac_magnitude,
            include_dc: value.include_dc,
            reltol: value.reltol,
            abstol: value.abstol,
            carrier: crate::services::simulation_runner::PeriodicCarrier::from(value.carrier),
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
    #[serde(default)]
    pub carrier: WorkerPeriodicCarrier,
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
            carrier: WorkerPeriodicCarrier::from(value.carrier),
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
            carrier: crate::services::simulation_runner::PeriodicCarrier::from(value.carrier),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct WorkerPnoiseRunConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<rspice_core::analysis::pnoise::PeriodicNoiseSampling>,
    #[serde(default)]
    pub input_sideband: i32,
    #[serde(default)]
    pub output_sideband: i32,
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
    #[serde(default)]
    pub carrier: WorkerPeriodicCarrier,
}

impl From<&crate::services::simulation_runner::PnoiseRunConfig> for WorkerPnoiseRunConfig {
    fn from(value: &crate::services::simulation_runner::PnoiseRunConfig) -> Self {
        Self {
            sampling: value.sampling.clone(),
            input_sideband: value.input_sideband,
            output_sideband: value.output_sideband,
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
            carrier: WorkerPeriodicCarrier::from(value.carrier),
        }
    }
}

impl From<WorkerPnoiseRunConfig> for crate::services::simulation_runner::PnoiseRunConfig {
    fn from(value: WorkerPnoiseRunConfig) -> Self {
        Self {
            sampling: value.sampling.clone(),
            input_sideband: value.input_sideband,
            output_sideband: value.output_sideband,
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
            carrier: crate::services::simulation_runner::PeriodicCarrier::from(value.carrier),
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
        #[serde(default = "crate::simulation::config::design_parameters_filter")]
        filter: String,
        #[serde(default)]
        sweep: Option<rspice_simulation_contract::config::SensitivitySweepSpec>,
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
                modes: config.modes.clone(),
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
                filter: config.filter.clone(),
                sweep: config.sweep.map(SensitivitySweep::to_spec),
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
                modes,
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
                modes,
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
                filter,
                sweep,
            } => Self::Sensitivity(SensitivityConfig {
                output_var,
                ac_mode,
                frequency,
                filter,
                sweep: sweep.map(SensitivitySweep::from_spec),
            }),
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

impl From<rspice_app_types::product::ProcessCorner> for WorkerCornerProcess {
    fn from(value: rspice_app_types::product::ProcessCorner) -> Self {
        match value {
            rspice_app_types::product::ProcessCorner::TT => Self::TT,
            rspice_app_types::product::ProcessCorner::SS => Self::SS,
            rspice_app_types::product::ProcessCorner::FF => Self::FF,
            rspice_app_types::product::ProcessCorner::SF => Self::SF,
            rspice_app_types::product::ProcessCorner::FS => Self::FS,
        }
    }
}

impl From<WorkerCornerProcess> for rspice_app_types::product::ProcessCorner {
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

/// The carrier a periodic small-signal request names, on the wire.
///
/// The engine's `FROM=` vocabulary, spelled for transport. Defaulted on read
/// because an older worker's request carries no carrier at all, and the card
/// it was written from had no `FROM=` keyword: that request asked for the
/// preceding periodic solve, which is exactly what the default is.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum WorkerPeriodicCarrier {
    #[default]
    Preceding,
    Pss,
    Hb,
}

impl From<crate::services::simulation_runner::PeriodicCarrier> for WorkerPeriodicCarrier {
    fn from(value: crate::services::simulation_runner::PeriodicCarrier) -> Self {
        use crate::services::simulation_runner::PeriodicCarrier;

        match value {
            PeriodicCarrier::Preceding => Self::Preceding,
            PeriodicCarrier::Pss => Self::Pss,
            PeriodicCarrier::Hb => Self::Hb,
        }
    }
}

impl From<WorkerPeriodicCarrier> for crate::services::simulation_runner::PeriodicCarrier {
    fn from(value: WorkerPeriodicCarrier) -> Self {
        match value {
            WorkerPeriodicCarrier::Preceding => Self::Preceding,
            WorkerPeriodicCarrier::Pss => Self::Pss,
            WorkerPeriodicCarrier::Hb => Self::Hb,
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
