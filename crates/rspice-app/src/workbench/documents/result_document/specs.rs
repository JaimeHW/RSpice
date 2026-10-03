//! SPECS — the active retained dataset's governed measurement results.
//!
//! Each row binds an authored `.MEAS` expression and project-owned limit to
//! the exact retained value and worst source in the active immutable dataset.
//! Ambiguous lineage, missing results, and invalid evaluations fail closed;
//! the viewer and inspector share this same projection. Bounds and authored
//! expressions persist with the workspace ([`SpecEntry`]), while the docbar
//! opens their inline editor.

mod reference_import;

use std::collections::HashSet;

use egui::Ui;
use rspice_results::specification::report::resolved_specifications;
#[cfg(test)]
use rspice_results::specification::report::{
    SpecResultStatus, result_row, result_rows, summarize_rows,
};

use crate::state::{SimulationRun, SpecEntry, SpecPointScope, SpecificationDefinition};
use crate::workbench::AppState;
use rspice_results_ui::specs::editor::{self, SpecDraft};

use super::ResultViewer;

/// Measurements present in the active dataset but not yet bound by the
/// active requirement contract. The editor never discovers names from a
/// different historical dataset.
fn untracked_measurements(state: &AppState) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = HashSet::new();
    if let Some(run) = state.simulation.active_run() {
        for analysis in &run.analyses {
            for name in analysis.scalar_evidence_names() {
                if seen.insert(name.to_ascii_lowercase()) {
                    names.push(name);
                }
            }
        }
    }
    names
        .into_iter()
        .filter(|name| {
            !state
                .workspace
                .content
                .specs
                .iter()
                .any(|spec| spec.measurement.eq_ignore_ascii_case(name))
        })
        .collect()
}

/// The requirements the workspace's active retained dataset was judged
/// against, owned.
///
/// The whole-state spelling of [`resolved_specifications`], for the hardcopy
/// capture, which takes a presentation of the Results workspace rather than
/// drawing one. It captured `workspace.specs` directly, and both the printed
/// table and the printed document's identity derive from what it captured —
/// so a limit edited after a completed run restated itself on the page beside
/// the verdict the run had actually earned, and moved the identity of an
/// immutable result.
pub(crate) fn active_run_specifications(state: &AppState) -> Vec<SpecEntry> {
    state.simulation.active_run().map_or_else(
        || state.workspace.content.specs.clone(),
        |run| resolved_specifications(run, &state.workspace.content.specs).into_owned(),
    )
}

/// Serialize the same worst-case projection rendered by the Specs sheet,
/// against the same requirements it was rendered against.
///
/// `workspace_specs` is the legacy fallback only: the run's own frozen
/// contract is resolved here rather than by the caller, so an export arm
/// holding the live workspace cannot write a bound the sheet never showed.
pub(crate) fn export_csv(
    run: &SimulationRun,
    workspace_specs: &[SpecEntry],
) -> super::ResultSheetCsv {
    let encoded = rspice_formats::result_csv::encode_specification_csv(run, workspace_specs);
    super::ResultSheetCsv {
        default_name: "rspice-specifications.csv",
        detail: format!("{} specification rows", encoded.row_count),
        contents: encoded.contents,
    }
}

/// Render the active dataset's specification evidence as the upgraded
/// seven-column engineering table (or the inline contract editor).
pub fn show(ui: &mut Ui, state: &mut AppState) {
    if state.ui.results.session.spec_drafts.is_some() {
        show_editor(ui, state);
        return;
    }

    let focus_analysis = rspice_results_ui::specs::show(
        ui,
        state.simulation.active_run().map(|run| &run.data),
        &state.workspace.content.specs,
        state.workbench.selected_specification.as_deref(),
    );
    if let Some(analysis_index) = focus_analysis {
        let viewer = state
            .simulation
            .active_run()
            .and_then(|run| run.analyses.get(analysis_index))
            .map_or(ResultViewer::Manifest, source_viewer);
        if state.simulation.select_analysis(analysis_index) {
            state.ui.results.session.viewer = viewer;
            state.ui.results.session.clear_cursors();
        }
    }
}

/// The sheet that shows this retained analysis, in tab order.
///
/// Through the shared compatibility predicate and the shared pair refinement,
/// never a table of its own. This module kept an analysis-type-to-viewer map
/// beside them, and it drifted in two ways at once: it named viewers by
/// analysis kind rather than by the evidence the analysis actually retained —
/// so a Monte Carlo campaign or an optimizer run resolved to `Waves` — and it
/// asked whether its answer was available of the *globally selected* analysis
/// rather than the one whose row was clicked, which for every analysis-scoped
/// sheet is a different question. Both dropped the reader on the manifest
/// instead of the source they had asked for.
///
/// The Specs sheet itself is excluded: it is the sheet the button was pressed
/// on, so it is not somewhere to open.
fn source_viewer(analysis: &crate::state::AnalysisResult) -> ResultViewer {
    ResultViewer::all()
        .filter(|viewer| viewer.viewer_document_id().is_some())
        .filter(|viewer| {
            *viewer != ResultViewer::Specs
                && super::view_context::analysis_supports_viewer(*viewer, analysis)
        })
        .map(|viewer| super::project_viewer_for_analysis(viewer, analysis))
        .next()
        .unwrap_or(ResultViewer::Manifest)
}

/// The inline governed-requirement editor. Exact comparison kind, limits,
/// guard band, role, and durable requirement identity remain explicit rather
/// than being flattened through the legacy min/max projection.
fn show_editor(ui: &mut Ui, state: &mut AppState) {
    let untracked = untracked_measurements(state);
    let analysis_options = state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| {
            plan.instances()
                .iter()
                .enumerate()
                .map(|(index, instance)| {
                    (
                        instance.id(),
                        format!(
                            "{} · {}",
                            plan.instance_list_label(index)
                                .unwrap_or_else(|| instance.display_name().to_owned()),
                            instance.id()
                        ),
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut scope_options = vec![
        ("All PVT points".to_owned(), SpecPointScope::AllPoints),
        ("Nominal only".to_owned(), SpecPointScope::Nominal),
    ];
    if let Some(dimension) = state
        .sim_setup
        .run_set
        .enabled_dimension_of(crate::simulation::run_set::RunSetDimensionKind::ProcessSection)
    {
        scope_options.extend(dimension.values.iter().map(|value| {
            let corner = value.lexical.trim().to_owned();
            (
                format!("Corner {corner} only"),
                SpecPointScope::SelectedCorners {
                    corners: vec![corner],
                },
            )
        }));
    }

    let Some(drafts) = state.ui.results.session.spec_drafts.as_mut() else {
        return;
    };
    editor::show(
        ui,
        drafts,
        &untracked,
        &analysis_options,
        &scope_options,
        reference_import::start,
    );
}

/// Apply the open editor's drafts to the workspace. Returns false (and
/// leaves the editor open) when a bound fails to parse.
pub fn apply_drafts(state: &mut AppState) -> bool {
    let Some(drafts) = state.ui.results.session.spec_drafts.clone() else {
        return true;
    };
    let mut specs = Vec::with_capacity(drafts.len());
    for draft in &drafts {
        match draft.parse() {
            Ok(Some(entry)) => specs.push(entry),
            Ok(None) => {}
            Err(_) => return false,
        }
    }
    let Ok(plan_id) = state
        .sim_setup
        .stable_analysis_plan()
        .map(crate::simulation::plan::SimulationPlan::id)
    else {
        return false;
    };
    let mut workspace = state.workspace.clone();
    workspace
        .content
        .replace_active_specification_definitions(plan_id, specs);
    if workspace
        .content
        .validate_simulation_configuration()
        .is_err()
    {
        return false;
    }
    let mut setup = state.sim_setup.clone();
    if setup
        .commit_active_plan_configuration_change("Updated output specifications.")
        .is_err()
    {
        return false;
    }
    state.workspace = workspace;
    state.sim_setup = setup;
    state.workbench.preflight = Default::default();
    state.ui.results.session.spec_drafts = None;
    true
}

/// Open the editor seeded from the current workspace specs.
pub fn open_editor(state: &mut AppState) {
    let drafts = state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(crate::simulation::plan::SimulationPlan::id)
        .map_or_else(Vec::new, |plan_id| {
            state
                .workspace
                .content
                .plan_data(plan_id)
                .filter(|payload| !payload.specification_definitions.is_empty())
                .map(|payload| {
                    payload
                        .specification_definitions
                        .iter()
                        .map(SpecDraft::from_definition)
                        .collect()
                })
                .unwrap_or_else(|| {
                    state
                        .workspace
                        .content
                        .specs
                        .iter()
                        .enumerate()
                        .map(|(index, entry)| {
                            SpecificationDefinition::from_legacy(plan_id, index, entry)
                        })
                        .map(|definition| SpecDraft::from_definition(&definition))
                        .collect()
                })
        });
    state.ui.results.session.spec_drafts = Some(drafts);
}

/// Right panel: the same active-dataset projection shown in the document.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    rspice_results_ui::specs::right_panel(
        ui,
        state.simulation.active_run().map(|run| &run.data),
        &state.workspace.content.specs,
    );
}

#[cfg(test)]
mod tests;
