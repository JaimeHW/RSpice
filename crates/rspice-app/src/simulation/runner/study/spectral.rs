//! Spectral consumer execution on a freshly solved trial.
use super::*;
use crate::simulation::multi_run::AnalysisSpec;
use rspice_simulation::results::SimulationResult;

pub(super) fn run_trial(
    request_config: &StudyRunConfig,
    engine: &rspice_core::Engine,
    analysis: &StudyAnalysis,
    circuit: &rspice_core::Netlist,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let Some(postprocess) = &request_config.postprocess else {
        return match analysis {
            StudyAnalysis::Basic(config) => {
                EngineBridge::run_materialized_with_abort(engine, config, circuit, abort)
            }
            StudyAnalysis::Pss(pss) => {
                super::pss::run(pss, engine, circuit, &request_config.numeric_options, abort)
            }
            StudyAnalysis::Qpss(qpss) => super::qpss::run_with_circuit(
                qpss,
                engine,
                circuit,
                &request_config.numeric_options,
                abort,
            )
            .map(|(_, result)| result),
            StudyAnalysis::Hb(hb) => super::hb::run_with_circuit(
                hb,
                engine,
                circuit,
                &request_config.numeric_options,
                abort,
            )
            .map(|(_, result)| result),
            StudyAnalysis::Native(spec) => {
                super::super::spec::run_native_study_on_materialized(spec.clone(), circuit, abort)
            }
        };
    };
    super::super::spec::ensure_not_aborted(abort)?;
    if postprocess.is_periodic() {
        return super::periodic::run_periodic(
            postprocess,
            engine,
            analysis,
            circuit,
            &request_config.numeric_options,
            abort,
        );
    }
    if matches!(
        postprocess.request,
        AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. }
    ) {
        if let StudyAnalysis::Hb(hb) = analysis {
            let (physical, result) = super::hb::run_with_circuit(
                hb,
                engine,
                circuit,
                &postprocess.producer_numeric_options,
                abort,
            )?;
            let SimulationResult::HarmonicBalance {
                operating_point, ..
            } = result
            else {
                return Err(SimulationError::SolverError(
                    "HB study producer returned no retained orbit".into(),
                ));
            };
            let consumer = super::pss::circuit_with_options(
                &physical,
                &request_config.numeric_options,
                abort,
            )?;
            return super::super::spec::run_hb_consumer(
                postprocess.request.clone(),
                &consumer,
                &operating_point,
                abort,
            );
        }
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
    let trajectory =
        rspice_simulation::execution_artifact::TransientTrajectoryArtifact::from_result(
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

#[cfg(test)]
mod tests;

#[cfg(test)]
mod hb_rf_tests;
