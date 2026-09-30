//! A frozen spectral consumer and the exact producer it runs per trial.

use super::*;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StudyPostprocess {
    pub producer_instance_id: AnalysisInstanceId,
    pub producer_source_revision: ObjectRevision,
    pub producer_analysis_line: String,
    pub producer_numeric_options: String,
    pub request: AnalysisSpec,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodic_options: Option<StudyPeriodicOptions>,
}

impl StudyPostprocess {
    fn validate(&self, base: &StudyRunConfig) -> Result<(), SimulationError> {
        if self.producer_instance_id == base.instance_id
            || self.producer_source_revision != base.source_revision
        {
            return Err(SimulationError::InvalidConfig(
                "Study producer must be a distinct instance from the same frozen plan revision"
                    .into(),
            ));
        }
        if !matches!(
            self.request,
            AnalysisSpec::Fourier { .. }
                | AnalysisSpec::Fft { .. }
                | AnalysisSpec::Hbsp { .. }
                | AnalysisSpec::Hbnoise { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pstb
                | AnalysisSpec::Psp { .. }
                | AnalysisSpec::Qpac { .. }
                | AnalysisSpec::Qpxf { .. }
                | AnalysisSpec::Qpnoise { .. }
        ) {
            return Err(SimulationError::InvalidConfig(
                "Study requires a configured spectral or periodic consumer".into(),
            ));
        }
        self.request
            .validate()
            .map_err(SimulationError::InvalidConfig)?;
        let producer = match &base.analysis {
            StudyAnalysis::Basic(AnalysisConfig::Transient(config)) => AnalysisSpec::Transient {
                stop_time: config.stop_time,
                step_time: config.step_time,
                start_time: config.start_time,
                max_timestep: config.max_timestep,
                uic: config.uic,
            },
            StudyAnalysis::Native(
                spec @ (AnalysisSpec::HarmonicBalance { .. } | AnalysisSpec::Qpss { .. }),
            ) => spec.clone(),
            StudyAnalysis::Pss(pss) => pss.request.clone(),
            StudyAnalysis::Qpss(qpss) => qpss.request.clone(),
            StudyAnalysis::Hb(hb) => hb.request.clone(),
            _ => {
                return Err(SimulationError::InvalidConfig(
                    "Study requires its configured transient, PSS, HB or QPSS producer".into(),
                ));
            }
        };
        crate::prepared_dependency::validate_prepared_dependency_contract_with_options(
            &self.request,
            &self.periodic_execution_options()?,
            &producer,
        )
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))
    }

    pub(super) fn validate_measurements(
        &self,
        base: &StudyRunConfig,
    ) -> Result<(), SimulationError> {
        self.validate(base)?;
        for request in &base.measurements {
            let (mode, key) = request.split_once(':').unwrap_or(("meas", request));
            let valid = mode.eq_ignore_ascii_case("scalar")
                || mode.eq_ignore_ascii_case("bin")
                || (mode.eq_ignore_ascii_case("meas")
                    && matches!(self.request, AnalysisSpec::Pnoise))
                || (mode.eq_ignore_ascii_case("last")
                    && matches!(
                        self.request,
                        AnalysisSpec::Fourier { .. }
                            | AnalysisSpec::Hbsp { .. }
                            | AnalysisSpec::Hbnoise { .. }
                            | AnalysisSpec::Pac
                            | AnalysisSpec::Pxf
                            | AnalysisSpec::Pnoise
                            | AnalysisSpec::Pstb
                            | AnalysisSpec::Psp { .. }
                            | AnalysisSpec::Qpac { .. }
                            | AnalysisSpec::Qpxf { .. }
                            | AnalysisSpec::Qpnoise { .. }
                    ));
            if !valid {
                return Err(SimulationError::InvalidConfig(format!(
                    "Spectral study measurement {request:?} requires scalar:name or bin:index:quantity[:signal]; periodic and Fourier consumers also support last:signal, and PNOISE supports meas:name"
                )));
            }
            if key.eq_ignore_ascii_case("THD(%)")
                && matches!(
                    self.request,
                    AnalysisSpec::Fourier {
                        compute_thd: false,
                        ..
                    }
                )
            {
                return Err(SimulationError::InvalidConfig(
                    "Enable Fourier THD to measure THD(%)".into(),
                ));
            }
        }
        Ok(())
    }
}

impl StudyRunConfig {
    pub fn execution_source(&self, source: &str) -> Result<String, SimulationError> {
        let operating_point = match &self.analysis {
            StudyAnalysis::Pss(pss) => Some(&pss.operating_point),
            StudyAnalysis::Qpss(qpss) => Some(&qpss.operating_point),
            StudyAnalysis::Hb(hb) => Some(&hb.operating_point),
            _ => None,
        };
        if let Some(operating_point) = operating_point {
            self.analysis
                .validate()
                .map_err(|errors| SimulationError::InvalidConfig(errors.join("; ")))?;
            if operating_point.instance_id == self.instance_id
                || self
                    .postprocess
                    .as_ref()
                    .is_some_and(|post| post.producer_instance_id == operating_point.instance_id)
                || operating_point.source_revision != self.source_revision
            {
                return Err(SimulationError::InvalidConfig(
                    "Periodic study requires a distinct OP producer from the same frozen plan revision"
                        .into(),
                ));
            }
            if let Some(post) = &self.postprocess {
                post.validate(self)?;
                return Ok(crate::netlist_preparation::splice_before_terminal_end_card(
                    source,
                    &format!("{}\n{}", post.producer_analysis_line, self.analysis_line),
                ));
            }
            // Each stage overlays its own numerical controls after variation.
            return Ok(crate::netlist_preparation::splice_before_terminal_end_card(
                source,
                &self.analysis_line,
            ));
        }
        if let Some(post) = self.postprocess.as_ref().filter(|post| post.is_periodic()) {
            post.validate(self)?;
            return Ok(crate::netlist_preparation::splice_before_terminal_end_card(
                source,
                &format!("{}\n{}", post.producer_analysis_line, self.analysis_line),
            ));
        }
        let mut block = String::new();
        if let Some(postprocess) = &self.postprocess {
            postprocess.validate(self)?;
            block.push_str(&postprocess.producer_analysis_line);
            block.push('\n');
            block.push_str(&postprocess.producer_numeric_options);
            block.push('\n');
        }
        block.push_str(&self.analysis_line);
        block.push('\n');
        block.push_str(&self.numeric_options);
        Ok(crate::netlist_preparation::splice_before_terminal_end_card(
            source, &block,
        ))
    }
}
