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
    pub mc_checkpoint: Option<crate::monte_carlo_checkpoint::MonteCarloCheckpointRequest>,
    #[serde(default)]
    pub mc_histogram_bins: Option<usize>,
    #[serde(default)]
    pub mc_statistics: Option<rspice_simulation_contract::mc_statistics::McStatisticsConfig>,
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
            study_base: value.study_base.map(crate::study::StudyRunConfig::from),
            temp: value.temp.map(crate::sweeps::TempRunConfig::from),
            parametric_base: value
                .parametric_base
                .map(crate::sweeps::CornerBaseMode::from),
            corner: value.corner.map(crate::sweeps::CornerRunConfig::from),
            pac: value.pac.map(crate::periodic::PacRunConfig::from),
            pxf: value.pxf.map(crate::periodic::PxfRunConfig::from),
            pnoise: value.pnoise.map(crate::periodic::PnoiseRunConfig::from),
            pstb: value.pstb.map(crate::periodic::PstbRunConfig::from),
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
        pss: Box<crate::study::StudyPssConfig>,
    },
    Qpss {
        qpss: Box<crate::study::StudyQpssConfig>,
    },
    Hb {
        hb: Box<crate::study::StudyHbConfig>,
    },
}

impl WorkerStudyAnalysis {
    #[cfg(any(feature = "browser-worker", test))]
    pub(super) fn op_config(&self) -> Option<&rspice_simulation_contract::config::OpConfig> {
        match self {
            Self::Basic(WorkerAnalysisConfig::DcOp(config)) => Some(config),
            Self::Pss { pss } => Some(&pss.operating_point.config),
            Self::Qpss { qpss } => Some(&qpss.operating_point.config),
            Self::Hb { hb } => Some(&hb.operating_point.config),
            _ => None,
        }
    }
    pub(super) fn op_config_mut(
        &mut self,
    ) -> Option<&mut rspice_simulation_contract::config::OpConfig> {
        match self {
            Self::Basic(WorkerAnalysisConfig::DcOp(config)) => Some(config),
            Self::Pss { pss } => Some(&mut pss.operating_point.config),
            Self::Qpss { qpss } => Some(&mut qpss.operating_point.config),
            Self::Hb { hb } => Some(&mut hb.operating_point.config),
            _ => None,
        }
    }
}

impl From<&crate::study::StudyAnalysis> for WorkerStudyAnalysis {
    fn from(value: &crate::study::StudyAnalysis) -> Self {
        use crate::study::StudyAnalysis;
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

impl From<WorkerStudyAnalysis> for crate::study::StudyAnalysis {
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
    postprocess: Option<crate::study::StudyPostprocess>,
    instance_id: rspice_app_types::product::AnalysisInstanceId,
    source_revision: rspice_app_types::product::ObjectRevision,
    pub(super) analysis: WorkerStudyAnalysis,
    analysis_line: String,
    numeric_options: String,
    measurements: Vec<String>,
    histogram_bins: usize,
    #[serde(default)]
    objective_terms: Vec<rspice_results::optimization::OptimizationObjectiveTerm>,
    #[serde(default)]
    constraints: Vec<rspice_results::optimization::OptimizationConstraint>,
}

impl From<&crate::study::StudyRunConfig> for WorkerStudyRunConfig {
    fn from(value: &crate::study::StudyRunConfig) -> Self {
        let crate::study::StudyRunConfig {
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

impl From<WorkerStudyRunConfig> for crate::study::StudyRunConfig {
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
