//! A frozen spectral consumer and the exact producer it runs per trial.

use super::*;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::results::SimulationResult;
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
            StudyAnalysis::Native(spec @ AnalysisSpec::HarmonicBalance { .. }) => spec.clone(),
            StudyAnalysis::Pss(pss) => pss.request.clone(),
            _ => {
                return Err(SimulationError::InvalidConfig(
                    "Study requires its configured transient, PSS or HB producer".into(),
                ));
            }
        };
        crate::simulation::execution::validate_prepared_dependency_contract_with_options(
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
    pub(super) fn execution_source(&self, source: &str) -> Result<String, SimulationError> {
        if let StudyAnalysis::Pss(pss) = &self.analysis {
            pss.validate().map_err(SimulationError::InvalidConfig)?;
            if pss.operating_point.instance_id == self.instance_id
                || self.postprocess.as_ref().is_some_and(|post| {
                    post.producer_instance_id == pss.operating_point.instance_id
                })
                || pss.operating_point.source_revision != self.source_revision
            {
                return Err(SimulationError::InvalidConfig(
                    "PSS study requires a distinct OP producer from the same frozen plan revision"
                        .into(),
                ));
            }
            if let Some(post) = &self.postprocess {
                post.validate(self)?;
                return Ok(services::splice_before_terminal_end_card(
                    source,
                    &format!("{}\n{}", post.producer_analysis_line, self.analysis_line),
                ));
            }
            // Each stage overlays its own numerical controls after variation.
            return Ok(services::splice_before_terminal_end_card(
                source,
                &self.analysis_line,
            ));
        }
        if let Some(post) = self.postprocess.as_ref().filter(|post| post.is_periodic()) {
            post.validate(self)?;
            return Ok(services::splice_before_terminal_end_card(
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
        Ok(services::splice_before_terminal_end_card(source, &block))
    }

    pub(super) fn run_trial(
        &self,
        engine: &rspice_core::Engine,
        analysis: &StudyAnalysis,
        circuit: &rspice_core::Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let Some(postprocess) = &self.postprocess else {
            return match analysis {
                StudyAnalysis::Basic(config) => {
                    EngineBridge::run_materialized_with_abort(engine, config, circuit, abort)
                }
                StudyAnalysis::Pss(pss) => pss.run(engine, circuit, &self.numeric_options, abort),
                StudyAnalysis::Native(spec) => {
                    super::super::spec::run_native_study_on_materialized(
                        spec.clone(),
                        circuit,
                        abort,
                    )
                }
            };
        };
        super::super::spec::ensure_not_aborted(abort)?;
        if postprocess.is_periodic() {
            return postprocess.run_periodic(
                engine,
                analysis,
                circuit,
                &self.numeric_options,
                abort,
            );
        }
        if matches!(
            postprocess.request,
            AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. }
        ) {
            let StudyAnalysis::Native(producer @ AnalysisSpec::HarmonicBalance { .. }) = analysis
            else {
                return Err(SimulationError::InvalidConfig(
                    "HBSP/HBNOISE study requires its configured HB producer".into(),
                ));
            };
            return super::super::spec::run_hb_study_on_materialized(
                producer.clone(),
                postprocess.request.clone(),
                circuit,
                abort,
            );
        }
        let mut trial = circuit.clone();
        // Unrelated FFT cards must not alter this producer's integration grid.
        // Retain only the request frozen into the selected consumer.
        match &postprocess.request {
            AnalysisSpec::Fft { request } => {
                let key = request
                    .engine_key()
                    .map_err(SimulationError::InvalidConfig)?;
                let selected = trial
                    .fft_analyses
                    .iter()
                    .find(|analysis| {
                        crate::simulation::config::FftRequest::from_core(analysis).to_card() == key
                    })
                    .cloned()
                    .ok_or_else(|| {
                        SimulationError::InvalidConfig(
                            "The study transient does not carry its selected FFT request".into(),
                        )
                    })?;
                trial.fft_analyses = vec![selected];
            }
            AnalysisSpec::Fourier { .. } => trial.fft_analyses.clear(),
            _ => {
                return Err(SimulationError::InvalidConfig(
                    "Unsupported study postprocessor".into(),
                ));
            }
        }
        let config = analysis.as_basic().ok_or_else(|| {
            SimulationError::InvalidConfig("A spectral study requires a transient producer".into())
        })?;
        let result = EngineBridge::run_materialized_with_abort(engine, config, &trial, abort)?;
        super::super::spec::ensure_not_aborted(abort)?;
        let SimulationResult::Transient { waveforms, .. } = &result else {
            return Err(SimulationError::InvalidConfig(
                "Study producer returned no transient trajectory".into(),
            ));
        };
        let carry_spectra = matches!(postprocess.request, AnalysisSpec::Fft { .. });
        let required = if carry_spectra {
            Vec::new()
        } else {
            waveforms.keys().cloned().collect()
        };
        let trajectory = crate::simulation::execution::TransientTrajectoryArtifact::from_result(
            &result,
            &required,
            carry_spectra,
        )
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?
        .ok_or_else(|| {
            SimulationError::InvalidConfig("Study producer returned no transient trajectory".into())
        })?;
        super::super::spec::run_spectral_from_trajectory(
            postprocess.request.clone(),
            &trajectory,
            abort,
        )
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod hb_rf_tests;
