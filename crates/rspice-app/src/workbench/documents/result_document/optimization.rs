//! Optimization convergence and candidate history viewer.

use rspice_results_ui::optimization as presentation;
use rspice_results_ui::presentation::{panel_note, well_hint};
#[cfg(test)]
use std::collections::BTreeMap;

use egui::Ui;

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, RunHistoryRevision, SimulationState,
};
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use std::sync::Arc;

use super::frame_work::{self, DatasetWalk};
use super::{AnalysisPresentationKey, OptimizationSelection};

use rspice_results::optimization::history::{OptimizationIndices, OptimizationView};

/// The located history for one analysis, or the verdict that there is none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OptimizationPlan {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    located: Option<OptimizationIndices>,
}

/// Locate the active analysis' optimizer history, once per generation.
///
/// Behind a cell because the availability gate that calls this every frame
/// holds only `&AppState`; the answer is a property of immutable evidence, so
/// the cell hands back the same verdict rather than recomputing it.
fn optimization_plan(state: &AppState) -> Option<Arc<OptimizationPlan>> {
    let source = (
        state.simulation.runs.revision(),
        state.simulation.data_version,
    );
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    if let Some(plan) = state.ui.results.plans.optimization.borrow().as_ref()
        && plan.source == source
        && plan.analysis == analysis_key
    {
        return Some(Arc::clone(plan));
    }
    let evidence_is_valid = super::analysis_evidence_is_valid(state, run.dataset_id, analysis);
    let built = Arc::new(OptimizationPlan {
        source,
        analysis: analysis_key,
        located: locate_optimization(analysis, evidence_is_valid),
    });
    *state.ui.results.plans.optimization.borrow_mut() = Some(Arc::clone(&built));
    Some(built)
}

fn active_optimization<'a>(
    simulation: &'a SimulationState,
    plan: Option<&OptimizationPlan>,
) -> Option<(&'a AnalysisResult, OptimizationView<'a>)> {
    let analysis = simulation.active_analysis()?;
    Some((analysis, plan?.located.as_ref()?.view(analysis)?))
}

fn locate_optimization(
    analysis: &AnalysisResult,
    evidence_is_valid: bool,
) -> Option<OptimizationIndices> {
    frame_work::note(DatasetWalk::OptimizationView);
    OptimizationIndices::locate(analysis, evidence_is_valid)
}

/// Serialize the validated candidate history that the optimization sheet
/// draws, including the terminal optimum metadata.
pub(crate) fn export_csv(analysis: &AnalysisResult) -> Option<super::ResultSheetCsv> {
    // Export revalidates the retained history; the UI generation cache is not an export permit.
    frame_work::note(DatasetWalk::OptimizationView);
    let encoded = rspice_formats::result_csv::encode_optimization_csv(analysis)?;
    Some(super::ResultSheetCsv {
        default_name: "rspice-optimization.csv",
        detail: format!("{} optimization iterations", encoded.iteration_count),
        contents: encoded.contents,
    })
}

pub(super) fn active_metadata_is_valid(state: &AppState) -> bool {
    active_optimization(&state.simulation, optimization_plan(state).as_deref()).is_some()
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(run) = state.simulation.active_run() else {
        well_hint(ui, "Select a dataset with retained optimization evidence");
        return;
    };
    let plan = optimization_plan(state);
    let Some((analysis, view)) = active_optimization(&state.simulation, plan.as_deref()) else {
        well_hint(
            ui,
            "Select a validated optimization analysis with a retained cost history",
        );
        return;
    };
    let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let plot_view = state
        .ui
        .results
        .plot_view(super::ResultViewer::Optimization, 0);
    // The convergence axis fits to the retained cost history, which does not
    // change while the reader looks at it. Scanning for its bounds on every
    // frame made an idle sheet cost one pass over the whole optimizer history.
    let cost_key = {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        (analysis_key, "optimization-cost").hash(&mut hasher);
        hasher.finish()
    };
    let (cost_min, cost_max) = state
        .ui
        .results
        .derived
        .range_or(cost_key, || super::finite_extremes(&view.cost.y))
        .unwrap_or((0.0, 1.0));
    let response = presentation::show(
        ui,
        presentation::OptimizationPlot {
            analysis_id: analysis.id,
            label: &analysis.label,
            selected: state
                .ui
                .results
                .selected_optimization
                .filter(|selection| selection.analysis == analysis_key)
                .map(|selection| selection.iteration_index),
            history: view,
            plot_view,
            cost_range: (cost_min, cost_max),
        },
        &mut state.ui.results.cache,
    );
    if response.fit {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::Optimization, 0);
    }
    if response.view.any() {
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::Optimization, 0)
            .apply(&response.view);
    }
    if let Some(index) = response.selected {
        state.ui.results.selected_optimization = Some(OptimizationSelection {
            analysis: analysis_key,
            iteration_index: index,
        });
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(selection) = state.ui.results.selected_optimization else {
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "Select an iteration row to inspect its exact retained cost and variables.",
        );
        return;
    };
    let Some(run) = state.simulation.active_run() else {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(ui, "Select a retained optimization analysis and candidate.");
        return;
    };
    let Some((analysis_index, analysis)) = selection.analysis.resolve(run) else {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "The selected candidate no longer belongs to the active retained dataset.",
        );
        return;
    };
    if state.simulation.active_analysis_idx != Some(analysis_index) {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "Select an optimization candidate in the active analysis.",
        );
        return;
    }
    let Some(AnalysisResultFamilyMetadata::Optimization {
        iterations,
        best_cost,
        best_variables,
        best_objectives: _,
        best_constraints: _,
        converged,
    }) = analysis.family_metadata.as_ref()
    else {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "The active analysis no longer contains retained optimization history.",
        );
        return;
    };
    let index = selection.iteration_index;
    if index >= iterations.len() {
        state.ui.results.selected_optimization = None;
        section_header(ui, "Candidate selection", None);
        panel_note(
            ui,
            "The selected candidate no longer exists in the retained optimization history.",
        );
        return;
    }
    presentation::right_panel(
        ui,
        presentation::CandidateView {
            analysis,
            index,
            iterations,
            best_cost,
            best_variables,
            converged,
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisType, WaveformData};

    /// A retained optimizer run of `iterations` candidates over one variable.
    fn optimization_state(iterations: usize) -> AppState {
        use crate::state::SimulationRun;

        let axis: Vec<f64> = (0..iterations).map(|index| index as f64).collect();
        let cost: Vec<f64> = (0..iterations)
            .map(|index| 1.0 / (index as f64 + 1.0))
            .collect();
        let gain: Vec<f64> = (0..iterations).map(|index| index as f64 * 0.5).collect();
        let best_cost = cost[iterations - 1];
        let best_gain = gain[iterations - 1];
        let analysis = AnalysisResult::new(1, AnalysisType::Optimization, "OPT")
            .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
                best_objectives: Vec::new(),
                best_constraints: Vec::new(),
                iterations: axis.clone(),
                best_cost,
                best_variables: BTreeMap::from([("GAIN".to_owned(), best_gain)]),
                converged: true,
            })
            .with_waveforms(vec![
                WaveformData::new("OPT_COST", axis.clone(), cost, "#0af"),
                WaveformData::new("OPT_GAIN", axis, gain, "#fa0"),
            ]);
        let mut run = SimulationRun::new(5);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state.simulation.active_analysis_idx = Some(0);
        state
    }

    #[test]
    fn retained_view_source_optimizer_gate_refreshes_without_frame_preparation() {
        let mut state = optimization_state(64);
        let original = optimization_plan(&state).unwrap();
        assert_eq!(original.located.as_ref().unwrap().best_index(), 63);
        let version = state.simulation.data_version;
        state.simulation.runs[0].analyses[0].waveforms.remove(1);
        assert!(!active_metadata_is_valid(&state));
        assert!(optimization_plan(&state).unwrap().located.is_none());

        state.simulation.runs[0].analyses[0] =
            optimization_state(3).simulation.runs[0].analyses[0].clone();
        assert!(active_metadata_is_valid(&state));
        let repaired = optimization_plan(&state).unwrap();
        assert_eq!(repaired.analysis, original.analysis);
        let (_, view) = active_optimization(&state.simulation, Some(&repaired)).unwrap();
        assert_eq!(view.best_index, 2);
        assert_eq!(view.cost.y.len(), 3);
        assert_eq!(view.variables[0].0, "GAIN");
        assert_eq!(original.located.as_ref().unwrap().best_index(), 63);
        assert_eq!(state.simulation.data_version, version);
    }

    /// Locating the history verifies every candidate series against the
    /// iteration axis, so it happens once per dataset generation — and has to
    /// happen again when that generation moves.
    #[test]
    fn the_history_is_located_once_per_dataset_generation() {
        let state = optimization_state(64);
        let first = optimization_plan(&state).expect("a located optimizer history");
        assert!(first.located.is_some());
        let again = optimization_plan(&state).expect("a located optimizer history");
        assert!(Arc::ptr_eq(&first, &again));

        // The projection through the memo is the projection it replaced.
        let analysis = &state.simulation.runs[0].analyses[0];
        let direct = locate_optimization(analysis, true)
            .and_then(|indices| indices.view(analysis))
            .expect("a direct projection");
        let (_, memoized) =
            active_optimization(&state.simulation, Some(&first)).expect("a memoized projection");
        assert_eq!(memoized.best_index, direct.best_index);
        assert_eq!(memoized.best_cost, direct.best_cost);
        assert_eq!(
            memoized
                .variables
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>(),
            direct
                .variables
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>()
        );

        let mut state = state;
        state.simulation.runs[0].analyses[0]
            .waveforms
            .retain(|waveform| waveform.name != "OPT_GAIN");
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);
        let after = optimization_plan(&state).expect("a plan for the new generation");
        assert!(
            after.located.is_none(),
            "the sheet offered a history the new dataset generation no longer retains"
        );
        assert!(!active_metadata_is_valid(&state));
    }

    /// The candidate table lays out what its viewport shows, not one row per
    /// retained iteration.
    #[test]
    fn the_candidate_table_lays_out_only_the_rows_the_viewport_shows() {
        let mut state = optimization_state(2_000);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state));
            },
        );
        let drawn = output
            .platform_output
            .accesskit_update
            .expect("the optimization sheet publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| {
                // A Label-role node carries its text in `value`, not `label`.
                node.value() == Some("evaluated") || node.label() == Some("evaluated")
            })
            .count();
        assert!(drawn > 0, "the sheet drew no candidates at all");
        assert!(
            drawn < 200,
            "the sheet laid out {drawn} of 2000 retained candidates for a 1020 px viewport"
        );
    }
    #[test]
    fn weighted_optimization_evidence_persists_exports_and_rejects_inconsistent_costs() {
        use rspice_results::optimization::{
            OptimizationObjectiveGoal, OptimizationObjectiveObservation, OptimizationObjectiveTerm,
        };
        let mut state = optimization_state(4);
        let analysis = &mut state.simulation.runs[0].analyses[0];
        let AnalysisResultFamilyMetadata::Optimization {
            best_objectives, ..
        } = analysis.family_metadata.as_mut().unwrap()
        else {
            unreachable!()
        };
        best_objectives.push(OptimizationObjectiveObservation {
            objective: OptimizationObjectiveTerm {
                measurement: "gain".into(),
                unit: "mV".into(),
                goal: OptimizationObjectiveGoal::Target,
                target: Some(1.0),
                scale: 2.0,
                weight: 1.0,
            },
            value: 2.0,
            contribution: 0.25,
        });
        let saved = serde_json::to_string(&analysis.family_metadata).unwrap();
        let restored = serde_json::from_str(&saved).unwrap();
        assert_eq!(analysis.family_metadata, restored);
        analysis.family_metadata = restored;
        let csv = export_csv(analysis).unwrap().contents;
        assert!(csv.contains(
            "objective,measurement,unit,goal,target,scale,weight,value,cost_contribution"
        ));
        assert!(csv.contains("1,gain,mV,Target,"));
        let AnalysisResultFamilyMetadata::Optimization {
            best_objectives, ..
        } = analysis.family_metadata.as_mut().unwrap()
        else {
            unreachable!()
        };
        best_objectives[0].value = 3.0;
        assert!(export_csv(analysis).is_none());
    }
    #[test]
    fn constraint_optimization_evidence_persists_exports_and_rejects_false_feasibility() {
        use rspice_results::optimization::{
            OptimizationConstraint, OptimizationConstraintObservation,
        };
        let mut state = optimization_state(4);
        let analysis = &mut state.simulation.runs[0].analyses[0];
        let AnalysisResultFamilyMetadata::Optimization {
            best_constraints,
            converged,
            ..
        } = analysis.family_metadata.as_mut().unwrap()
        else {
            unreachable!()
        };
        *converged = false;
        best_constraints.push(OptimizationConstraintObservation {
            constraint: OptimizationConstraint {
                measurement: "limit".into(),
                unit: "mA".into(),
                lower: Some(2.0),
                upper: None,
                tolerance: 0.1,
                scale: 2.0,
            },
            value: 1.5,
            violation: 0.2,
        });
        let saved = serde_json::to_string(&analysis.family_metadata).unwrap();
        analysis.family_metadata = serde_json::from_str(&saved).unwrap();
        let csv = export_csv(analysis).unwrap().contents;
        assert!(csv.contains("feasible,false"));
        assert!(csv.contains("constraint,measurement,unit,lower,upper,tolerance,scale,value,normalized_violation,satisfied"));
        let AnalysisResultFamilyMetadata::Optimization { converged, .. } =
            analysis.family_metadata.as_mut().unwrap()
        else {
            unreachable!()
        };
        *converged = true;
        assert!(export_csv(analysis).is_none());
        let AnalysisResultFamilyMetadata::Optimization {
            best_constraints,
            converged,
            ..
        } = analysis.family_metadata.as_mut().unwrap()
        else {
            unreachable!()
        };
        *converged = false;
        best_constraints[0].violation = 0.0;
        assert!(export_csv(analysis).is_none());
    }
}
