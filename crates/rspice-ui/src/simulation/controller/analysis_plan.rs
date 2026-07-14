use super::*;

impl SimulationController {
    pub(super) fn enabled_analysis_indices(state: &AppState) -> Vec<usize> {
        // The run set is the single source of truth: no silent fallback to
        // the selected row — start_simulation refuses an empty set instead.
        state.sim_setup.ordered_enabled_indices()
    }

    pub(super) fn analysis_label_for_index(idx: usize) -> &'static str {
        match idx {
            0 => "DC Operating Point",
            1 => "Transient",
            2 => "AC",
            24 => "DISTO",
            3 => "DC Sweep",
            4 => "Noise",
            5 => "Pole-Zero",
            6 => "Sensitivity",
            7 => "Monte Carlo",
            8 => "PSS",
            9 => "STB",
            10 => "Temperature Sweep",
            11 => "Harmonic Balance",
            12 => "S-Parameter",
            13 => "PAC",
            14 => "PNoise",
            15 => "PXF",
            16 => "PSTB",
            17 => "Transfer Function",
            18 => "Corner",
            19 => "Envelope",
            20 => "Fourier",
            21 => "Reliability",
            22 => "Optimization",
            23 => "Safety (SOA)",
            _ => "Unknown",
        }
    }

    pub(super) fn build_analysis_plan(
        &self,
        state: &AppState,
    ) -> Result<AnalysisPlan, Vec<String>> {
        let mut plan = AnalysisPlan::new();
        plan.stop_on_error = false;

        let mut errors = Vec::new();
        for idx in Self::enabled_analysis_indices(state) {
            match self.build_analysis_spec_for_index(state, idx) {
                Ok(spec) => plan.analyses.push(spec),
                Err(e) => errors.push(format!("{}: {}", Self::analysis_label_for_index(idx), e)),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        plan.validate()?;
        Ok(plan)
    }

    pub(super) fn build_queue_from_plan(
        &self,
        state: &AppState,
        plan: &AnalysisPlan,
        sealed_model_sources: &crate::state::model_library::SealedModelExecutionSources,
    ) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
        let mut queue = Vec::with_capacity(plan.analyses.len());
        let mut errors = Vec::new();

        for spec in &plan.analyses {
            let analysis_line = match self.analysis_spec_to_spice_line(state, spec) {
                Ok(line) => line,
                Err(e) => {
                    errors.push(format!("{}: {}", spec.run_type().display_name(), e));
                    continue;
                }
            };
            let spec_options =
                match self.analysis_spec_execution_options(state, spec, sealed_model_sources) {
                    Ok(opts) => opts,
                    Err(e) => {
                        errors.push(format!("{}: {}", spec.run_type().display_name(), e));
                        continue;
                    }
                };

            if Self::executes_via_spec(spec) {
                queue.push(QueuedAnalysis {
                    spec: spec.clone(),
                    config: None,
                    spec_options,
                    analysis_line,
                });
                continue;
            }

            match self.analysis_spec_to_config(state, spec) {
                Ok(config) => {
                    if let Err(errs) = config.validate() {
                        errors.push(format!(
                            "{} config is invalid: {}",
                            spec.run_type().display_name(),
                            errs.join(", ")
                        ));
                    } else {
                        queue.push(QueuedAnalysis {
                            spec: spec.clone(),
                            config: Some(config),
                            spec_options,
                            analysis_line,
                        });
                    }
                }
                Err(e) => errors.push(format!("{}: {}", spec.run_type().display_name(), e)),
            }
        }

        if errors.is_empty() {
            Ok(queue)
        } else {
            Err(errors)
        }
    }

    pub(super) fn executes_via_spec(spec: &AnalysisSpec) -> bool {
        matches!(
            spec,
            AnalysisSpec::Tf
                | AnalysisSpec::Disto { .. }
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pstb
                | AnalysisSpec::Stb { .. }
                | AnalysisSpec::MonteCarlo
                | AnalysisSpec::Parametric
                | AnalysisSpec::Corner
                | AnalysisSpec::Pss { .. }
                | AnalysisSpec::HarmonicBalance { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::SParameter { .. }
                | AnalysisSpec::Envelope { .. }
                | AnalysisSpec::Fourier { .. }
                | AnalysisSpec::Reliability { .. }
                | AnalysisSpec::Optimization { .. }
                | AnalysisSpec::Soa { .. }
        )
    }

    pub(super) fn analysis_spec_execution_options(
        &self,
        state: &AppState,
        spec: &AnalysisSpec,
        sealed_model_sources: &crate::state::model_library::SealedModelExecutionSources,
    ) -> Result<SpecExecutionOptions, String> {
        match spec {
            AnalysisSpec::Parametric => {
                let mut temp_state = state.sim_setup.temp.clone();
                temp_state.ensure_initialized();
                let temp_cfg = temp_state
                    .to_config()
                    .map_err(|e| format!("invalid temperature sweep settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    temp: Some(Self::temp_run_config_from_dialog(state, &temp_cfg)?),
                    corner: None,
                    pac: None,
                    pxf: None,
                    tf: None,
                    pnoise: None,
                    pstb: None,
                })
            }
            AnalysisSpec::Corner => {
                let mut corner_state = state.sim_setup.corner.clone();
                corner_state.ensure_initialized();
                let corner_cfg = corner_state
                    .to_config()
                    .map_err(|e| format!("invalid corner settings: {}", e))?;
                Ok(SpecExecutionOptions {
                    temp: None,
                    corner: Some(Self::corner_run_config_from_dialog(
                        state,
                        &corner_cfg,
                        sealed_model_sources,
                    )?),
                    pac: None,
                    pxf: None,
                    tf: None,
                    pnoise: None,
                    pstb: None,
                })
            }
            AnalysisSpec::Pac => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: Some(Self::pac_run_config_from_dialog(state)?),
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Pxf => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: Some(Self::pxf_run_config_from_dialog(state)?),
                tf: None,
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Tf => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: Some(Self::tf_run_config_from_dialog(state)?),
                pnoise: None,
                pstb: None,
            }),
            AnalysisSpec::Pnoise => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: Some(Self::pnoise_run_config_from_dialog(state)?),
                pstb: None,
            }),
            AnalysisSpec::Pstb => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: Some(Self::pstb_run_config_from_dialog(state)?),
            }),
            _ => Ok(SpecExecutionOptions {
                temp: None,
                corner: None,
                pac: None,
                pxf: None,
                tf: None,
                pnoise: None,
                pstb: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::simulation_analysis_tabs::{TAB_AC, TAB_NOISE, TAB_TRANSIENT};

    #[test]
    fn controller_consumes_authoritative_analysis_order() {
        let mut state = AppState::default();
        state.sim_setup.enabled.extend([TAB_AC, TAB_NOISE]);
        state.sim_setup.analysis_order = vec![TAB_NOISE, TAB_TRANSIENT, TAB_AC];

        assert_eq!(
            SimulationController::enabled_analysis_indices(&state),
            vec![TAB_NOISE, TAB_TRANSIENT, TAB_AC]
        );
    }

    #[test]
    fn normalization_removes_disabled_and_appends_newly_enabled_analyses() {
        let mut state = AppState::default();
        state.sim_setup.enabled.extend([TAB_AC, TAB_NOISE]);
        state.sim_setup.analysis_order = vec![TAB_NOISE, TAB_TRANSIENT, TAB_AC];

        state.sim_setup.enabled.remove(&TAB_NOISE);
        state.sim_setup.normalize_analysis_order();
        assert_eq!(state.sim_setup.analysis_order, vec![TAB_TRANSIENT, TAB_AC]);

        state.sim_setup.enabled.insert(TAB_NOISE);
        state.sim_setup.normalize_analysis_order();
        assert_eq!(
            state.sim_setup.analysis_order,
            vec![TAB_TRANSIENT, TAB_AC, TAB_NOISE]
        );
    }
}
