//! Complete execution options lowered from the frozen authored draft.
use crate::execution_options::SpecExecutionOptions;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::setup_state::SimulationSetup;

pub fn analysis_spec_execution_options(
    sim_setup: &SimulationSetup,
    draft: &rspice_simulation_contract::analysis_draft::AnalysisDraft,
    periodic_producer: Option<&rspice_simulation_contract::analysis_draft::AnalysisDraft>,
    spec: &AnalysisSpec,
    sealed_model_sources: &crate::model_sources::SealedModelExecutionSources,
) -> Result<SpecExecutionOptions, String> {
    use rspice_simulation_contract::analysis_draft::AnalysisDraft;

    match spec {
        AnalysisSpec::MonteCarlo { .. } => {
            let AnalysisDraft::MonteCarlo(draft) = draft else {
                return Err("Monte Carlo specification requires its authored draft".into());
            };
            let mut draft = draft.clone();
            draft.ensure_initialized();
            let config = draft.to_config()?;
            Ok(SpecExecutionOptions {
                mc_checkpoint: request_from_config(config.checkpoint.as_ref()),
                mc_histogram_bins: config
                    .base_analysis
                    .is_none()
                    .then_some(config.histogram_bins),
                mc_statistics: config.statistics,
                ..Default::default()
            })
        }
        AnalysisSpec::Parametric => {
            let AnalysisDraft::Temperature(temp_state) = draft else {
                return Err("Temperature specification requires its authored draft".into());
            };
            let mut temp_state = temp_state.clone();
            temp_state.ensure_initialized();
            if temp_state.base_analysis.is_some() {
                // The frozen projection resolved the bound instance's kind.
                // The authored index is only a legacy fallback and may be stale.
                temp_state.base_idx = sim_setup.temp.base_idx;
            }
            let temp_cfg = temp_state
                .to_config(&sim_setup.run_set, sim_setup.reference_pvt)
                .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
            Ok(SpecExecutionOptions {
                mc_histogram_bins: None,
                mc_statistics: None,
                mc_checkpoint: None,
                tran_checkpoint: None,
                study_base: None,
                temp: Some(crate::analysis_preparation::temp_run_config_from_dialog(
                    sim_setup, &temp_cfg,
                )?),
                parametric_base: None,
                corner: None,
                pac: None,
                pxf: None,
                pnoise: None,
                pstb: None,
            })
        }
        AnalysisSpec::Corner => {
            let AnalysisDraft::Corner(corner_state) = draft else {
                return Err("Corner specification requires its authored draft".into());
            };
            let mut corner_state = corner_state.clone();
            corner_state.ensure_initialized();
            if corner_state.base_analysis.is_some() {
                // The bound base, not the legacy index, selects the run mode.
                corner_state.base_analysis_idx = sim_setup.corner.base_analysis_idx;
            }
            let corner_cfg = corner_state
                .to_config(&sim_setup.run_set, sim_setup.reference_pvt)
                .map_err(|e| format!("invalid corner settings: {}", e))?;
            Ok(SpecExecutionOptions {
                mc_histogram_bins: None,
                mc_statistics: None,
                mc_checkpoint: None,
                tran_checkpoint: None,
                study_base: None,
                temp: None,
                parametric_base: None,
                corner: Some(crate::analysis_preparation::corner_run_config_from_dialog(
                    sim_setup,
                    &corner_cfg,
                    sealed_model_sources,
                )?),
                pac: None,
                pxf: None,
                pnoise: None,
                pstb: None,
            })
        }
        AnalysisSpec::Pac => {
            let AnalysisDraft::Pac(draft) = draft else {
                return Err("PAC specification requires its authored draft".into());
            };
            Ok(SpecExecutionOptions {
                pac: Some(crate::analysis_preparation::pac_run_config_from_dialog(
                    sim_setup,
                    draft,
                    periodic_producer,
                )?),
                ..Default::default()
            })
        }
        AnalysisSpec::Pxf => {
            let AnalysisDraft::Pxf(draft) = draft else {
                return Err("PXF specification requires its authored draft".into());
            };
            Ok(SpecExecutionOptions {
                pxf: Some(crate::analysis_preparation::pxf_run_config_from_dialog(
                    sim_setup,
                    draft,
                    periodic_producer,
                )?),
                ..Default::default()
            })
        }
        AnalysisSpec::Tf { .. } => Ok(SpecExecutionOptions::default()),
        AnalysisSpec::Pnoise => {
            let AnalysisDraft::Pnoise(draft) = draft else {
                return Err("PNOISE specification requires its authored draft".into());
            };
            Ok(SpecExecutionOptions {
                pnoise: Some(crate::analysis_preparation::pnoise_run_config_from_dialog(
                    sim_setup,
                    draft,
                    periodic_producer,
                )?),
                ..Default::default()
            })
        }
        AnalysisSpec::Pstb => {
            let AnalysisDraft::Pstb(draft) = draft else {
                return Err("PSTB specification requires its authored draft".into());
            };
            Ok(SpecExecutionOptions {
                pstb: Some(crate::analysis_preparation::pstb_run_config_from_dialog(
                    sim_setup,
                    draft,
                    periodic_producer,
                )?),
                ..Default::default()
            })
        }
        AnalysisSpec::Psp { .. } => Ok(SpecExecutionOptions::default()),
        _ => Ok(SpecExecutionOptions {
            mc_histogram_bins: None,
            mc_statistics: None,
            mc_checkpoint: None,
            tran_checkpoint: None,
            study_base: None,
            temp: None,
            parametric_base: None,
            corner: None,
            pac: None,
            pxf: None,
            pnoise: None,
            pstb: None,
        }),
    }
}

/// Capture cadence is shared across points; selected trial data is routed after
/// the Run Set has materialized each point's source and numerical environment.
fn request_from_config(
    config: Option<&rspice_simulation_contract::mc_checkpoint::McCheckpointConfig>,
) -> Option<crate::monte_carlo_checkpoint::MonteCarloCheckpointRequest> {
    config.map(
        |config| crate::monte_carlo_checkpoint::MonteCarloCheckpointRequest {
            publish_every: config.publish_every,
            trial_range: None,
            resume: None,
        },
    )
}
