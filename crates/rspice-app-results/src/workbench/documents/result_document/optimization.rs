//! Optimization convergence and candidate history viewer.

use rspice_results_ui::optimization as presentation;
use rspice_results_ui::presentation::{panel_note, well_hint};
use std::collections::BTreeMap;

use egui::Ui;

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, DesignVariableQuantity, RunHistoryRevision,
    SimulationState,
};
use crate::workbench::design_system::section_header;
#[cfg(test)]
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use rspice_app_state::workbench::app_state::design_history::SetupParts;
use rspice_app_state::workbench::app_state::model::Model;
use rspice_simulation_contract::design_variable_quantity::parse_design_quantity;

use std::sync::Arc;

use super::frame_work::{self, DatasetWalk};
use super::{AnalysisPresentationKey, OptimizationSelection, ResultsParts, ResultsReads};

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
/// holds only shared references; the answer is a property of immutable
/// evidence, so the cell hands back the same verdict rather than recomputing
/// it.
fn optimization_plan(state: ResultsReads<'_>) -> Option<Arc<OptimizationPlan>> {
    let source = (
        state.model.simulation.retained.runs.revision(),
        state.model.simulation.view.data_version,
    );
    let run = state.model.simulation.active_run()?;
    let analysis = state.model.simulation.active_analysis()?;
    let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    if let Some(plan) = state.results.plans.optimization.borrow().as_ref()
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
    *state.results.plans.optimization.borrow_mut() = Some(Arc::clone(&built));
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

pub(super) fn active_metadata_is_valid(state: ResultsReads<'_>) -> bool {
    active_optimization(&state.model.simulation, optimization_plan(state).as_deref()).is_some()
}

/// The sheet's bar: which table stands at the foot, and "Apply best to
/// design".
pub(super) fn domain_bar(ui: &mut Ui, state: &mut ResultsParts<'_>) -> bool {
    let plan = optimization_plan(state.results_reads());
    let Some((analysis, view)) = active_optimization(&state.model.simulation, plan.as_deref())
    else {
        return false;
    };
    let application = best_application(&*state.model, analysis);
    let offer = application.as_ref().map(|application| {
        format!(
            "Set {} in the active plan; Undo takes it back",
            application.assignments
        )
    });
    let bar = presentation::domain_bar(
        ui,
        &view,
        state.results.session.optimization_table,
        offer.as_deref().map_err(|reason| reason.as_str()),
    );
    if let Some(table) = bar.table {
        state.results.session.optimization_table = table;
    }
    if bar.apply_best
        && let Ok(application) = application
    {
        let _ = commit_best(state, application);
    }
    true
}

/// Write the shown optimization's best candidate into the active plan's
/// design variables, as one step of the setup's history that Undo takes
/// back. Returns the plan's receipt, or why nothing was written.
pub fn apply_best_to_design(state: &mut ResultsParts<'_>) -> Result<String, String> {
    let plan = optimization_plan(state.results_reads());
    let (analysis, _) = active_optimization(&state.model.simulation, plan.as_deref())
        .ok_or_else(|| "The shown analysis is no validated optimization".to_owned())?;
    let application = best_application(&*state.model, analysis)?;
    commit_best(state, application)
}

/// What "Apply best to design" writes: each optimized variable's best value
/// onto the active plan's design variable of the same name.
#[derive(Debug, Clone, PartialEq)]
struct BestApplication {
    plan: crate::product::SimulationPlanId,
    updates: Vec<(crate::product::DesignVariableId, String)>,
    /// The assignments, as the plan will read them: `RLOAD = 2345.5 ohm`.
    assignments: String,
}

/// The best candidate as design-variable writes, or why it cannot be one.
/// A candidate is applied whole or not at all: a design holding some of its
/// values is no candidate the optimizer evaluated.
fn best_application(model: &Model, analysis: &AnalysisResult) -> Result<BestApplication, String> {
    let Some(AnalysisResultFamilyMetadata::Optimization { best_variables, .. }) =
        analysis.family_metadata.as_ref()
    else {
        return Err("The shown analysis is no optimization".to_owned());
    };
    let (plan, _) = model
        .active_plan_revision()
        .ok_or_else(|| "No simulation plan is open to write the best candidate into".to_owned())?;
    let payload = model
        .workspace
        .content
        .plan_data(plan)
        .ok_or_else(|| "The active plan holds no design variables".to_owned())?;
    let mut updates = Vec::with_capacity(best_variables.len());
    let mut assignments = Vec::with_capacity(best_variables.len());
    let mut missing = Vec::new();
    let mut unchanged = 0;
    for (name, &value) in best_variables {
        let Some(variable) = payload
            .design_variables
            .iter()
            .find(|variable| variable.name.eq_ignore_ascii_case(name))
        else {
            missing.push(name.as_str());
            continue;
        };
        let held = parse_design_quantity(&variable.expression, variable.quantity).ok();
        if held.is_some_and(|held| held.to_bits() == value.to_bits()) {
            unchanged += 1;
        }
        let expression = design_expression(value, variable.quantity);
        assignments.push(format!("{} = {expression}", variable.name));
        updates.push((variable.id, expression));
    }
    if !missing.is_empty() {
        return Err(format!(
            "The active plan has no design variable named {}",
            missing.join(", ")
        ));
    }
    if updates.is_empty() {
        return Err("The optimization varied no design variable".to_owned());
    }
    if unchanged == updates.len() {
        return Err("The design already holds the best candidate".to_owned());
    }
    Ok(BestApplication {
        plan,
        updates,
        assignments: assignments.join(", "),
    })
}

/// `value` as a design variable of `quantity` spells it, at the precision
/// the optimizer found it: the shortest decimal that reads back as the same
/// number, in the unit the quantity asks for.
fn design_expression(value: f64, quantity: DesignVariableQuantity) -> String {
    let number = if value != 0.0 && !(1e-3..1e6).contains(&value.abs()) {
        format!("{value:e}")
    } else {
        format!("{value}")
    };
    match quantity {
        DesignVariableQuantity::Resistance => format!("{number} ohm"),
        DesignVariableQuantity::Capacitance => format!("{number}F"),
        DesignVariableQuantity::Voltage => format!("{number}V"),
        DesignVariableQuantity::Current => format!("{number}A"),
        DesignVariableQuantity::Temperature | DesignVariableQuantity::Dimensionless => number,
    }
}

/// Each optimized variable's unit, by its name in the history: the unit of
/// the active plan's design variable of that name, and none for a variable
/// the plan does not hold or holds without a unit.
fn variable_units<'n>(
    model: &Model,
    names: impl IntoIterator<Item = &'n str>,
) -> BTreeMap<String, &'static str> {
    let payload = model
        .active_plan_revision()
        .and_then(|(plan, _)| model.workspace.content.plan_data(plan));
    names
        .into_iter()
        .map(|name| {
            let unit = payload
                .and_then(|payload| {
                    payload
                        .design_variables
                        .iter()
                        .find(|variable| variable.name.eq_ignore_ascii_case(name))
                })
                .map_or("", |variable| quantity_unit(variable.quantity));
            (name.to_owned(), unit)
        })
        .collect()
}

/// The unit a design variable of `quantity` is stated in.
const fn quantity_unit(quantity: DesignVariableQuantity) -> &'static str {
    match quantity {
        DesignVariableQuantity::Resistance => "\u{3a9}",
        DesignVariableQuantity::Capacitance => "F",
        DesignVariableQuantity::Voltage => "V",
        DesignVariableQuantity::Current => "A",
        DesignVariableQuantity::Temperature => "\u{b0}C",
        DesignVariableQuantity::Dimensionless => "",
    }
}

/// Adopt the writes as one validated plan transaction and one step of the
/// setup's history, as every plan edit is.
fn commit_best(
    state: &mut ResultsParts<'_>,
    application: BestApplication,
) -> Result<String, String> {
    let step = state.model.setup_snapshot(SetupParts::PLAN_DATA);
    let mut workspace = state.model.workspace.clone();
    let mut setup = state.model.sim_setup.clone();
    let outcome = workspace
        .content
        .update_design_variable_expressions(application.plan, &application.updates)
        .map_err(|error| error.to_string())
        .and_then(|_| {
            workspace
                .content
                .validate_simulation_configuration()
                .map_err(|error| error.to_string())
        })
        .and_then(|()| {
            setup
                .commit_active_plan_configuration_change(format!(
                    "Applied the best optimization candidate: {}",
                    application.assignments
                ))
                .map_err(|error| error.to_string())
        });
    match outcome {
        Ok(receipt) => {
            state.model.workspace = workspace;
            state.model.sim_setup = setup;
            state
                .model
                .record_setup_change("Apply best optimization candidate", step);
            state.preflight.invalidate();
            let line = receipt.status_line();
            state.model.record_plan_receipt(line.clone());
            Ok(line)
        }
        Err(error) => {
            if let Some(prompt) = state.model.record_plan_refusal(error.clone()) {
                state.workbench.prompt(prompt);
            }
            Err(error)
        }
    }
}

pub fn show(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    let significant_digits = significant_digits(state.results_reads());
    let Some(run) = state.model.simulation.active_run() else {
        well_hint(ui, "Select a dataset with optimization results");
        return;
    };
    let plan = optimization_plan(state.results_reads());
    let Some((analysis, view)) = active_optimization(&state.model.simulation, plan.as_deref())
    else {
        well_hint(
            ui,
            "Pick an optimization analysis with a cost history in the data browser",
        );
        return;
    };
    let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let plot_view = state
        .results
        .session
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
        .results
        .session
        .derived
        .range_or(cost_key, || super::finite_extremes(&view.cost.y))
        .unwrap_or((0.0, 1.0));
    // Each variable's axis fits its history, which is as fixed as the cost's.
    let variable_ranges = view
        .variables
        .iter()
        .map(|(name, waveform)| {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            (analysis_key, "optimization-variable", *name).hash(&mut hasher);
            state
                .results
                .session
                .derived
                .range_or(hasher.finish(), || super::finite_extremes(&waveform.y))
                .unwrap_or((0.0, 1.0))
        })
        .collect::<Vec<_>>();
    // Each goal's axis fits its measured history, as fixed as the cost's.
    let goal_ranges = view
        .goals
        .iter()
        .map(|(name, waveform)| {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            (analysis_key, "optimization-goal", *name).hash(&mut hasher);
            state
                .results
                .session
                .derived
                .range_or(hasher.finish(), || super::finite_extremes(&waveform.y))
                .unwrap_or((0.0, 1.0))
        })
        .collect::<Vec<_>>();
    let variable_units =
        variable_units(&*state.model, view.variables.iter().map(|(name, _)| *name));
    let response = presentation::show(
        ui,
        presentation::OptimizationPlot {
            analysis_id: analysis.id,
            label: &analysis.label,
            selected: state
                .results
                .session
                .selected_optimization
                .filter(|selection| selection.analysis == analysis_key)
                .map(|selection| selection.iteration_index),
            history: view,
            plot_view,
            cost_range: (cost_min, cost_max),
            variable_ranges: &variable_ranges,
            variable_units: &variable_units,
            goal_ranges: &goal_ranges,
            table: state.results.session.optimization_table,
            significant_digits,
        },
        &mut state.results.session.cache,
    );
    if response.fit {
        state
            .results
            .session
            .reset_plot_view(super::ResultViewer::Optimization, 0);
    }
    if response.view.any() {
        state
            .results
            .session
            .plot_view_mut(super::ResultViewer::Optimization, 0)
            .apply(&response.view);
    }
    if let Some(axes) = response.axes {
        state
            .results
            .session
            .drawn_axes
            .insert(super::ResultViewer::Optimization, axes);
    }
    if let Some(index) = response.selected {
        state.results.session.selected_optimization = Some(OptimizationSelection {
            analysis: analysis_key,
            iteration_index: index,
        });
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut ResultsParts<'_>) {
    // Every way of losing the selection ends the same way: the selection is
    // cleared and the reader picks another row. Five sentences for one state
    // were five ways of saying "Select an iteration."
    if paint_selected_candidate(ui, state.results_reads()) {
        return;
    }
    state.results.session.selected_optimization = None;
    section_header(ui, "Candidate selection", None);
    panel_note(ui, "Select an iteration.");
}

/// Paint the selected candidate, or report that there is none to paint.
fn paint_selected_candidate(ui: &mut Ui, state: ResultsReads<'_>) -> bool {
    let Some(selection) = state.results.session.selected_optimization else {
        return false;
    };
    let Some(run) = state.model.simulation.active_run() else {
        return false;
    };
    let Some((analysis_index, analysis)) = selection.analysis.resolve(run) else {
        return false;
    };
    if state.model.simulation.view.active_analysis_idx != Some(analysis_index) {
        return false;
    }
    let Some(AnalysisResultFamilyMetadata::Optimization {
        iterations,
        best_cost,
        best_variables,
        best_objectives: _,
        best_constraints: _,
        converged: _,
    }) = analysis.family_metadata.as_ref()
    else {
        return false;
    };
    let index = selection.iteration_index;
    if index >= iterations.len() {
        return false;
    }
    let variable_units = variable_units(state.model, best_variables.keys().map(String::as_str));
    presentation::right_panel(
        ui,
        presentation::CandidateView {
            analysis,
            index,
            iterations,
            best_cost,
            best_variables,
            variable_units: &variable_units,
            significant_digits: significant_digits(state),
        },
    );
    true
}

/// The reader's displayed significant digits, for every number the sheet
/// and its inspector state.
fn significant_digits(state: ResultsReads<'_>) -> usize {
    usize::from(
        state
            .presentation_policy()
            .displayed_significant_digits()
            .get(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisType, RetainedTrace};

    /// A retained optimizer run of `iterations` candidates over one variable.
    fn optimization_state(iterations: usize) -> ResultsTestHost {
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
                RetainedTrace::new("OPT_COST", axis.clone(), cost, "#0af"),
                RetainedTrace::new("OPT_GAIN", axis, gain, "#fa0"),
            ]);
        let mut run = SimulationRun::new(5);
        run.add_analysis(analysis);
        let mut state = ResultsTestHost::default();
        state.model.simulation.retained.runs = vec![run].into();
        assert!(state.model.simulation.select_run(0));
        state.model.simulation.view.active_analysis_idx = Some(0);
        state
    }

    #[test]
    fn retained_view_source_optimizer_gate_refreshes_without_frame_preparation() {
        let mut state = optimization_state(64);
        let original = optimization_plan(state.results_reads()).unwrap();
        assert_eq!(original.located.as_ref().unwrap().best_index(), 63);
        let version = state.model.simulation.view.data_version;
        state.model.simulation.retained.runs[0].analyses[0]
            .waveforms
            .remove(1);
        assert!(!active_metadata_is_valid(state.results_reads()));
        assert!(
            optimization_plan(state.results_reads())
                .unwrap()
                .located
                .is_none()
        );

        state.model.simulation.retained.runs[0].analyses[0] =
            optimization_state(3).model.simulation.retained.runs[0].analyses[0].clone();
        assert!(active_metadata_is_valid(state.results_reads()));
        let repaired = optimization_plan(state.results_reads()).unwrap();
        assert_eq!(repaired.analysis, original.analysis);
        let (_, view) = active_optimization(&state.model.simulation, Some(&repaired)).unwrap();
        assert_eq!(view.best_index, 2);
        assert_eq!(view.cost.y.len(), 3);
        assert_eq!(view.variables[0].0, "GAIN");
        assert_eq!(original.located.as_ref().unwrap().best_index(), 63);
        assert_eq!(state.model.simulation.view.data_version, version);
    }

    /// Locating the history verifies every candidate series against the
    /// iteration axis, so it happens once per dataset generation — and has to
    /// happen again when that generation moves.
    #[test]
    fn the_history_is_located_once_per_dataset_generation() {
        let state = optimization_state(64);
        let first = optimization_plan(state.results_reads()).expect("a located optimizer history");
        assert!(first.located.is_some());
        let again = optimization_plan(state.results_reads()).expect("a located optimizer history");
        assert!(Arc::ptr_eq(&first, &again));

        // The projection through the memo is the projection it replaced.
        let analysis = &state.model.simulation.retained.runs[0].analyses[0];
        let direct = locate_optimization(analysis, true)
            .and_then(|indices| indices.view(analysis))
            .expect("a direct projection");
        let (_, memoized) = active_optimization(&state.model.simulation, Some(&first))
            .expect("a memoized projection");
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
        state.model.simulation.retained.runs[0].analyses[0]
            .waveforms
            .retain(|waveform| waveform.name != "OPT_GAIN");
        state.model.simulation.view.data_version =
            state.model.simulation.view.data_version.wrapping_add(1);
        let after =
            optimization_plan(state.results_reads()).expect("a plan for the new generation");
        assert!(
            after.located.is_none(),
            "the sheet offered a history the new dataset generation no longer retains"
        );
        assert!(!active_metadata_is_valid(state.results_reads()));
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
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state.results_parts()));
            },
        );
        let drawn = output
            .platform_output
            .accesskit_update
            .expect("the optimization sheet publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Row)
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
        let analysis = &mut state.model.simulation.retained.runs[0].analyses[0];
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
        let analysis = &mut state.model.simulation.retained.runs[0].analyses[0];
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
