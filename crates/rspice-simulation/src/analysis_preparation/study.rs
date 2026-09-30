//! Freeze a study's selected base and exact producer configuration.
use super::AnalysisInputs;
use rspice_simulation_contract::analysis_kind::AnalysisKind;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::config::AnalysisConfig;
use rspice_simulation_contract::plan_model::{FrozenAnalysisInstance, FrozenSimulationPlan};

fn compile_study_seeded_periodic<'a, R, A, W>(
    state: &AnalysisInputs<'a, R, A>,
    plan: &FrozenSimulationPlan,
    base: &rspice_simulation_contract::plan_model::FrozenAnalysisInstance,
    spec: &AnalysisSpec,
) -> Result<crate::study::StudyAnalysis, String>
where
    R: AsRef<rspice_results::run::SimulationRun<A>>,
    A: AsRef<rspice_results::analysis_result::AnalysisResult<W>> + 'a,
    W: AsRef<rspice_results::waveform::RetainedWaveform> + 'a,
{
    let producers = base
        .dependencies()
        .iter()
        .filter(|edge| edge.prerequisite() == AnalysisKind::OperatingPoint)
        .filter_map(|edge| {
            plan.instances()
                .iter()
                .find(|instance| instance.id() == edge.target())
        })
        .collect::<Vec<_>>();
    let [producer] = producers.as_slice() else {
        return Err("A periodic study requires exactly one explicitly bound, enabled operating-point producer".into());
    };
    let producer_state_setup = state
        .sim_setup
        .frozen_instance_projection(plan, producer)
        .map_err(|error| error.to_string())?;
    let producer_state = AnalysisInputs {
        sim_setup: &producer_state_setup,
        ..*state
    };
    let producer_spec =
        crate::analysis_preparation::analysis_draft_spec(&producer_state, producer.draft())?;
    let AnalysisConfig::DcOp(config) =
        crate::analysis_preparation::analysis_spec_to_config(&producer_spec)?
    else {
        return Err("Periodic study dependency is not an operating-point configuration".into());
    };
    use crate::study::{
        StudyAnalysis, StudyHbConfig, StudyOperatingPoint, StudyPssConfig, StudyQpssConfig,
    };
    let operating_point = StudyOperatingPoint {
        instance_id: producer.id(),
        source_revision: plan.revision(),
        config,
        numeric_options: producer
            .numeric_override()
            .map(|options| options.to_spice_options())
            .unwrap_or_default(),
    };
    if matches!(spec, AnalysisSpec::HarmonicBalance { .. }) {
        Ok(StudyAnalysis::Hb(Box::new(StudyHbConfig {
            request: spec.clone(),
            operating_point,
        })))
    } else if matches!(spec, AnalysisSpec::Qpss { .. }) {
        Ok(StudyAnalysis::Qpss(Box::new(StudyQpssConfig {
            request: spec.clone(),
            operating_point,
        })))
    } else {
        Ok(StudyAnalysis::Pss(Box::new(StudyPssConfig {
            request: spec.clone(),
            operating_point,
        })))
    }
}

pub fn compile_study_base<'a, R, A, W>(
    state: &AnalysisInputs<'a, R, A>,
    plan: &FrozenSimulationPlan,
    draft: &rspice_simulation_contract::analysis_draft::AnalysisDraft,
) -> Result<Option<crate::study::StudyRunConfig>, String>
where
    R: AsRef<rspice_results::run::SimulationRun<A>>,
    A: AsRef<rspice_results::analysis_result::AnalysisResult<W>> + 'a,
    W: AsRef<rspice_results::waveform::RetainedWaveform> + 'a,
{
    use rspice_simulation_contract::analysis_draft::AnalysisDraft;
    let (id, measurements, histogram_bins, objective_terms, constraints) = match draft {
        AnalysisDraft::MonteCarlo(draft) if draft.base_analysis.is_some() => {
            let config = draft.to_config()?;
            (
                config.base_analysis.unwrap(),
                config.measurements,
                config.histogram_bins,
                Vec::new(),
                Vec::new(),
            )
        }
        AnalysisDraft::Optimization(draft) if draft.base_analysis.is_some() => {
            let config = draft.to_config()?;
            (
                config.base_analysis.unwrap(),
                config.measurement_names(),
                20,
                config.objective_terms,
                config.constraints,
            )
        }
        _ => return Ok(None),
    };
    let base = plan
        .instances()
        .iter()
        .find(|instance| instance.id() == id)
        .ok_or_else(|| format!("Study base analysis {id} is missing or disabled"))?;
    if !base.kind().supports_study_base() {
        return Err(format!(
            "{} cannot yet be used as a study base",
            base.display_name()
        ));
    }
    let projected_setup = state
        .sim_setup
        .frozen_instance_projection(plan, base)
        .map_err(|error| error.to_string())?;
    let projected = AnalysisInputs {
        sim_setup: &projected_setup,
        ..*state
    };
    let spec = crate::analysis_preparation::analysis_draft_spec(&projected, base.draft())?;
    let periodic_producer = bound_periodic_producer(plan, base)?;
    use crate::study::StudyPeriodicOptions;
    let periodic_options = match base.draft() {
        AnalysisDraft::Pac(draft) => Some(StudyPeriodicOptions::Pac(
            crate::analysis_preparation::pac_run_config_from_dialog(
                projected.sim_setup,
                draft,
                periodic_producer.map(|producer| producer.draft()),
            )?,
        )),
        AnalysisDraft::Pxf(draft) => Some(StudyPeriodicOptions::Pxf(
            crate::analysis_preparation::pxf_run_config_from_dialog(
                projected.sim_setup,
                draft,
                periodic_producer.map(|producer| producer.draft()),
            )?,
        )),
        AnalysisDraft::Pnoise(draft) => Some(StudyPeriodicOptions::Pnoise(
            crate::analysis_preparation::pnoise_run_config_from_dialog(
                projected.sim_setup,
                draft,
                periodic_producer.map(|producer| producer.draft()),
            )?,
        )),
        AnalysisDraft::Pstb(draft) => Some(StudyPeriodicOptions::Pstb(
            crate::analysis_preparation::pstb_run_config_from_dialog(
                projected.sim_setup,
                draft,
                periodic_producer.map(|producer| producer.draft()),
            )?,
        )),
        _ => None,
    };
    let execution_options = periodic_options
        .as_ref()
        .map(StudyPeriodicOptions::execution_options)
        .unwrap_or_default();
    let producer_kind = match spec {
        AnalysisSpec::Fourier { .. } | AnalysisSpec::Fft { .. } => Some(AnalysisKind::Transient),
        AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. } => {
            Some(AnalysisKind::HarmonicBalance)
        }
        AnalysisSpec::Psp { .. } | AnalysisSpec::Pstb => Some(AnalysisKind::Pss),
        AnalysisSpec::Qpac { .. } | AnalysisSpec::Qpxf { .. } | AnalysisSpec::Qpnoise { .. } => {
            Some(AnalysisKind::Qpss)
        }
        AnalysisSpec::Pac | AnalysisSpec::Pxf | AnalysisSpec::Pnoise => {
            let carriers = base
                .dependencies()
                .iter()
                .filter(|edge| {
                    matches!(
                        edge.prerequisite(),
                        AnalysisKind::Pss | AnalysisKind::HarmonicBalance
                    )
                })
                .collect::<Vec<_>>();
            let [carrier] = carriers.as_slice() else {
                return Err(
                    "A periodic study requires exactly one explicitly bound PSS or HB producer"
                        .into(),
                );
            };
            Some(carrier.prerequisite())
        }
        _ => None,
    };
    let (analysis, postprocess) = if let Some(producer_kind) = producer_kind {
        let producers = base
            .dependencies()
            .iter()
            .filter(|edge| edge.prerequisite() == producer_kind)
            .filter_map(|edge| {
                plan.instances()
                    .iter()
                    .find(|instance| instance.id() == edge.target())
            })
            .collect::<Vec<_>>();
        let [producer] = producers.as_slice() else {
            return Err("A spectral study requires exactly one explicitly bound, enabled producer of the required analysis kind".into());
        };
        let producer_state_setup = state
            .sim_setup
            .frozen_instance_projection(plan, producer)
            .map_err(|error| error.to_string())?;
        let producer_state = AnalysisInputs {
            sim_setup: &producer_state_setup,
            ..*state
        };
        let producer_spec =
            crate::analysis_preparation::analysis_draft_spec(&producer_state, producer.draft())?;
        crate::prepared_dependency::validate_prepared_dependency_contract_with_options(
            &spec,
            &execution_options,
            &producer_spec,
        )
        .map_err(|error| error.to_string())?;
        (
            if matches!(
                producer_spec,
                AnalysisSpec::Pss { .. }
                    | AnalysisSpec::Qpss { .. }
                    | AnalysisSpec::HarmonicBalance { .. }
            ) {
                compile_study_seeded_periodic(state, plan, producer, &producer_spec)?
            } else {
                crate::analysis_preparation::analysis_spec_to_config(&producer_spec)?.into()
            },
            Some(crate::study::StudyPostprocess {
                producer_instance_id: producer.id(),
                producer_source_revision: plan.revision(),
                producer_analysis_line: crate::analysis_preparation::analysis_spec_to_spice_line(
                    &producer_state,
                    producer.draft(),
                    &producer_spec,
                )?,
                producer_numeric_options: producer
                    .numeric_override()
                    .map(|options| options.to_spice_options())
                    .unwrap_or_default(),
                request: spec.clone(),
                periodic_options,
            }),
        )
    } else if matches!(
        spec,
        AnalysisSpec::Pss { .. } | AnalysisSpec::Qpss { .. } | AnalysisSpec::HarmonicBalance { .. }
    ) {
        (
            compile_study_seeded_periodic(state, plan, base, &spec)?,
            None,
        )
    } else {
        (
            crate::analysis_preparation::analysis_spec_to_config(&spec)?.into(),
            None,
        )
    };
    analysis.validate().map_err(|errors| errors.join("; "))?;
    Ok(Some(crate::study::StudyRunConfig {
        postprocess,
        instance_id: id,
        source_revision: plan.revision(),
        analysis,
        analysis_line: crate::analysis_preparation::analysis_spec_to_spice_line(
            &projected,
            base.draft(),
            &spec,
        )?,
        numeric_options: base
            .numeric_override()
            .map(|options| options.to_spice_options())
            .unwrap_or_default(),
        measurements,
        histogram_bins,
        objective_terms,
        constraints,
    }))
}

/// Select the same frozen carrier for solver options and periodic consumer
/// basis. The projection remains for legacy builders, not producer identity.
pub fn bound_periodic_producer<'a>(
    plan: &'a FrozenSimulationPlan,
    instance: &FrozenAnalysisInstance,
) -> Result<Option<&'a FrozenAnalysisInstance>, String> {
    if !instance.kind().inherits_periodic_solver_options() {
        return Ok(None);
    }
    let carriers = instance
        .dependencies()
        .iter()
        .filter_map(|edge| {
            plan.instances().iter().find(|producer| {
                producer.id() == edge.target()
                    && matches!(
                        producer.kind(),
                        AnalysisKind::Pss | AnalysisKind::HarmonicBalance | AnalysisKind::Qpss
                    )
            })
        })
        .collect::<Vec<_>>();
    let [carrier] = carriers.as_slice() else {
        return Err(format!(
            "{} requires exactly one bound periodic producer for its solver options",
            instance.display_name()
        ));
    };
    Ok(Some(carrier))
}
