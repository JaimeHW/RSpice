//! A frozen spectral consumer and the exact transient producer it runs per trial.

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
            AnalysisSpec::Fourier { .. } | AnalysisSpec::Fft { .. }
        ) {
            return Err(SimulationError::InvalidConfig(
                "Spectral study requires a Fourier or FFT consumer".into(),
            ));
        }
        self.request
            .validate()
            .map_err(SimulationError::InvalidConfig)?;
        let AnalysisConfig::Transient(config) = &base.analysis else {
            return Err(SimulationError::InvalidConfig(
                "Spectral study requires its configured transient producer".into(),
            ));
        };
        let producer = AnalysisSpec::Transient {
            stop_time: config.stop_time,
            step_time: config.step_time,
            start_time: config.start_time,
            max_timestep: config.max_timestep,
            uic: config.uic,
        };
        crate::simulation::execution::validate_prepared_dependency_contract_with_options(
            &self.request,
            &Default::default(),
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
                || (mode.eq_ignore_ascii_case("last")
                    && matches!(self.request, AnalysisSpec::Fourier { .. }));
            if !valid {
                return Err(SimulationError::InvalidConfig(format!(
                    "Spectral study measurement {request:?} requires scalar:name or bin:index:quantity[:signal]; Fourier also supports last:signal"
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
        analysis: &AnalysisConfig,
        circuit: &rspice_core::Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        let Some(postprocess) = &self.postprocess else {
            return EngineBridge::run_materialized_with_abort(engine, analysis, circuit, abort);
        };
        super::super::spec::ensure_not_aborted(abort)?;
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
        let result = EngineBridge::run_materialized_with_abort(engine, analysis, &trial, abort)?;
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
