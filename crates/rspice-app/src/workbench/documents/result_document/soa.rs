//! SOA source qualification, cached presentation lifetime and shared application actions.

use super::{
    AnalysisPresentationKey, SoaRuleSelection,
    frame_work::{self, DatasetWalk},
};
#[cfg(test)]
use crate::state::{SoaParameterEvidence, WaveformData};
#[cfg(test)]
use crate::ui::plot::XScale;
use crate::{
    state::{AnalysisResult, AnalysisResultPayload, AnalysisType, SoaEvaluationEvidence},
    workbench::AppState,
};
use egui::Ui;
use rspice_results_ui::{presentation::well_hint, soa as viewer};

/// The active SOA analysis, if it is one and its evidence validated.
///
/// The validation goes through the workspace memo rather than the validator:
/// the tab strip asks this on every frame to decide whether to offer the
/// sheet, and the validator walks every retained stress sample in the run.
fn active_soa(
    simulation: &crate::state::SimulationState,
    evidence_is_valid: bool,
) -> Option<(&AnalysisResult, &[SoaEvaluationEvidence], usize)> {
    let analysis = simulation.active_analysis()?;
    let payload = analysis.result_payload.as_ref()?;
    let AnalysisResultPayload::Soa {
        source_history: _,
        evaluations,
        violations,
    } = payload
    else {
        return None;
    };
    if !analysis.success || analysis.analysis_type != AnalysisType::Soa || !evidence_is_valid {
        return None;
    }
    Some((analysis, evaluations, violations.len()))
}

/// The memoized retained-evidence verdict for whichever analysis is active.
fn active_evidence_is_valid(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    state
        .simulation
        .active_analysis()
        .is_some_and(|analysis| super::analysis_evidence_is_valid(state, run.dataset_id, analysis))
}

pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    active_soa(&state.simulation, active_evidence_is_valid(state)).is_some()
}

/// Every rule's scanned facts, built once per retained SOA analysis.
///
/// The visible-row lists are here too, one per filter. They cost no scan,
/// but the table reads one of them by ordinal every frame and building them
/// alongside the facts keeps the row a viewport names addressable in O(1).
#[derive(Debug, Clone, PartialEq)]
pub(super) struct SoaPlan {
    source: crate::state::RunHistoryRevision,
    version: u64,
    analysis: AnalysisPresentationKey,
    presentation: viewer::SoaPlan,
}

/// The scanned facts for the active SOA analysis, rebuilding them only when
/// the retained source, dataset generation, or selected analysis changes.
///
/// Both readers enter here before borrowing evidence. A cached plan is usable
/// only for the currently active, successful, validated analysis. The plot
/// cache shares its source boundary so rebuilding facts also refreshes samples.
fn soa_plan(
    state: &mut AppState,
    analysis_key: AnalysisPresentationKey,
) -> Option<std::sync::Arc<SoaPlan>> {
    // Eligibility and validation precede even a cache hit. Neither a stable
    // presentation key nor a numeric version proves the evidence is current.
    let dataset_id = state.simulation.active_run()?.dataset_id;
    let (analysis, evaluations, _) =
        active_soa(&state.simulation, active_evidence_is_valid(state))?;
    if AnalysisPresentationKey::new(dataset_id, analysis) != analysis_key {
        return None;
    }
    let source = state.simulation.runs.revision();
    let version = state.simulation.data_version;
    state.ui.results.cache.ensure_source(&source);
    if let Some(plan) = state.ui.results.plans.soa.as_ref()
        && plan.source == source
        && plan.version == version
        && plan.analysis == analysis_key
    {
        return Some(std::sync::Arc::clone(plan));
    }
    let built = std::sync::Arc::new(SoaPlan {
        source,
        version,
        analysis: analysis_key,
        presentation: viewer::build_soa_plan(
            version,
            analysis_key,
            &analysis.data,
            evaluations,
            || frame_work::note(DatasetWalk::SoaStressScan),
        ),
    });
    state.ui.results.plans.soa = Some(std::sync::Arc::clone(&built));
    Some(built)
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(dataset_id) = state.simulation.active_run().map(|run| run.dataset_id) else {
        well_hint(ui, "Select a dataset with retained SOA evidence");
        return;
    };
    let evidence_is_valid = active_evidence_is_valid(state);
    let Some(analysis_key) = state
        .simulation
        .active_analysis()
        .map(|analysis| AnalysisPresentationKey::new(dataset_id, analysis))
    else {
        well_hint(ui, "Select a validated safe-operating-area analysis");
        return;
    };
    // Built before the retained evidence is borrowed, so the scan happens at
    // most once per dataset generation rather than once per row per frame.
    let Some(plan) = soa_plan(state, analysis_key) else {
        well_hint(ui, "Select a validated safe-operating-area analysis");
        return;
    };
    let Some((analysis, evaluations, violation_count)) =
        active_soa(&state.simulation, evidence_is_valid)
    else {
        well_hint(ui, "Select a validated safe-operating-area analysis");
        return;
    };
    let response = viewer::show(
        ui,
        viewer::SoaSource {
            analysis: &analysis.data,
            analysis_key,
            evaluations,
            violation_count,
            plan: &plan.presentation,
        },
        viewer::SoaControls {
            selected: state.ui.results.selected_soa_rule.as_ref(),
            filter: state.ui.results.soa_rule_filter,
            trace_open: state.ui.results.soa_stress_trace_open,
            stress_view: state.ui.results.plot_view(super::ResultViewer::Soa, 0),
        },
        &mut state.ui.results.cache,
        |key, device| {
            soa_device_target(
                &state.simulation,
                &state.workspace,
                &state.schematic,
                key,
                device,
            )
            .is_some()
        },
    );
    if response.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::Soa, 0);
    }
    if let Some(change) = response.stress_view_change {
        state
            .ui
            .results
            .plot_view_mut(super::ResultViewer::Soa, 0)
            .apply(&change);
    }
    state.ui.results.soa_rule_filter = response.filter;
    state.ui.results.soa_stress_trace_open = response.trace_open;
    if response.selection != state.ui.results.selected_soa_rule {
        if response.selection.is_some() && response.trace_open {
            state
                .ui
                .results
                .reset_plot_view(super::ResultViewer::Soa, 0);
        }
        state.ui.results.selected_soa_rule = response.selection;
    }
    if let Some(selection) = response.cross_probe {
        apply_schematic_cross_probe(ui, state, &selection);
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(selection) = state.ui.results.selected_soa_rule.clone() else {
        viewer::selection_note(ui, viewer::SoaSelectionAbsence::Unselected);
        return;
    };
    // The panel reads the same scanned facts the table does, so opening it
    // does not re-scan the histories the sheet already walked.
    let Some(plan) = soa_plan(state, selection.analysis) else {
        viewer::selection_note(ui, viewer::SoaSelectionAbsence::Unavailable);
        return;
    };
    // The plan has checked this active analysis and its complete evidence.
    let Some(analysis) = state.simulation.active_analysis() else {
        return;
    };
    let Some(AnalysisResultPayload::Soa {
        source_history: _,
        evaluations,
        violations,
    }) = analysis.result_payload.as_ref()
    else {
        return;
    };
    let Some((rule, evaluation)) = evaluations.iter().enumerate().find(|(_, evaluation)| {
        evaluation.device_id == selection.device_id && evaluation.parameter == selection.parameter
    }) else {
        state.ui.results.selected_soa_rule = None;
        viewer::selection_note(ui, viewer::SoaSelectionAbsence::Removed);
        return;
    };
    let facts = plan.presentation.facts(rule);
    let event_count = violations
        .iter()
        .filter(|event| {
            event.device_id == evaluation.device_id && event.parameter == evaluation.parameter
        })
        .count();
    let response = viewer::right_panel(
        ui,
        viewer::SoaInspector {
            evaluation,
            facts,
            event_count,
            schematic_available: soa_device_target(
                &state.simulation,
                &state.workspace,
                &state.schematic,
                selection.analysis,
                &evaluation.device_id,
            )
            .is_some(),
        },
    );
    if response.open_trace {
        state.ui.results.soa_stress_trace_open = true;
    }
    if response.locate_device {
        apply_schematic_cross_probe(ui, state, &selection);
    }
}

fn result_mapping_is_current(
    simulation: &crate::state::SimulationState,
    workspace: &crate::state::ProjectWorkspace,
    schematic: &crate::state::SchematicState,
    analysis_key: AnalysisPresentationKey,
) -> bool {
    let Some(run) = simulation.active_run() else {
        return false;
    };
    analysis_key.resolve(run).is_some()
        && run.prepared_receipt().is_some_and(|receipt| {
            receipt.project_revision() == workspace.content.project.revision()
        })
        && simulation
            .cross_probe
            .is_current_for(&workspace.content.active_view, schematic.topology_version())
}

fn soa_device_target(
    simulation: &crate::state::SimulationState,
    workspace: &crate::state::ProjectWorkspace,
    schematic: &crate::state::SchematicState,
    analysis_key: AnalysisPresentationKey,
    device_id: &str,
) -> Option<(u64, crate::state::Point)> {
    if !result_mapping_is_current(simulation, workspace, schematic, analysis_key) {
        return None;
    }
    schematic
        .document()
        .components
        .iter()
        .find(|component| {
            component
                .spice_instance_name()
                .eq_ignore_ascii_case(device_id)
        })
        .map(|component| (component.id, component.pos))
}

fn apply_schematic_cross_probe(ui: &Ui, state: &mut AppState, selection: &SoaRuleSelection) {
    let Some((component_id, position)) = soa_device_target(
        &state.simulation,
        &state.workspace,
        &state.schematic,
        selection.analysis,
        &selection.device_id,
    ) else {
        state.ui.toasts.warn_with_title(
            ui.ctx(),
            "Cannot locate SOA device",
            "The selected result is stale or its retained device no longer resolves to the active schematic revision.",
        );
        return;
    };
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(component_id);
    state.schematic.session.editor.center_request = Some(position);
    state.ui.schematic_visibility.annotations =
        crate::state::SchematicAnnotationVisibility::ViolationsOnly;
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
}

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn soa_state(rules: usize, samples: usize, peak: f64) -> AppState {
        let analysis = AnalysisResult {
            data: viewer::test_support::soa_analysis(rules, samples, peak),
        };
        let mut state = AppState::default();
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state
    }
    pub(super) fn active_key(state: &AppState) -> AnalysisPresentationKey {
        let run = state.simulation.active_run().expect("retained run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    }

    #[test]
    fn a_new_dataset_generation_rebuilds_the_stress_facts() {
        let mut state = soa_state(2, 32, 3.0);
        let key = active_key(&state);
        let before = soa_plan(&mut state, key).expect("a validated SOA plan");
        let before_interval = before
            .presentation
            .facts(0)
            .expect("rule 0")
            .interval_compact
            .clone();

        // Replace the stress history with one that never crosses the warning
        // band, so the interval the plan reports has to change.
        let flattened: Vec<f64> = vec![0.1; 32];
        state.simulation.runs[0].analyses[0].waveforms[0].y = std::sync::Arc::new(flattened);
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

        let after = soa_plan(&mut state, key).expect("a validated SOA plan");
        assert_ne!(
            after
                .presentation
                .facts(0)
                .expect("rule 0")
                .interval_compact,
            before_interval,
            "the plan reported the previous dataset generation's interval"
        );
        assert!(
            after
                .presentation
                .facts(0)
                .expect("rule 0")
                .stress_waveform
                .is_none(),
            "the replaced history no longer reproduces the rule, so no trace is offered"
        );
    }

    #[test]
    fn a_plan_is_never_keyed_to_an_analysis_it_did_not_read() {
        let mut state = soa_state(2, 32, 3.0);
        let mut donor = soa_state(1, 8, 3.0);
        let second = donor.simulation.runs[0].analyses.remove(0);
        state.simulation.runs[0].add_analysis(second);
        assert_eq!(
            state.simulation.active_analysis_idx,
            Some(0),
            "the first analysis stays active"
        );

        let inactive = {
            let run = state.simulation.active_run().expect("retained run");
            AnalysisPresentationKey::new(run.dataset_id, &run.analyses[1])
        };
        assert_ne!(
            inactive,
            active_key(&state),
            "the fixture retains two distinguishable analyses"
        );

        let plan = soa_plan(&mut state, inactive);
        assert!(
            plan.is_none(),
            "a plan was served for an analysis whose evidence the build never read"
        );
        assert!(
            state.ui.results.plans.soa.is_none(),
            "the memo kept an entry stamped with an analysis it did not read"
        );

        // A key check, not a shutdown: the analysis that is active still gets
        // its plan, and it is stamped with its own key.
        let active = active_key(&state);
        let served = soa_plan(&mut state, active).expect("a validated SOA plan");
        assert_eq!(served.analysis, active);
    }

    #[test]
    fn the_rule_table_lays_out_only_the_rows_the_viewport_shows() {
        let mut state = soa_state(400, 8, 3.0);
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
            .expect("the SOA sheet publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.label() == Some("Stress trace"))
            .count();
        assert!(drawn > 0, "the sheet drew no rules at all");
        assert!(
            drawn < 100,
            "the sheet laid out {drawn} of 400 retained rules for a 1020 px viewport"
        );
    }
}

#[cfg(test)]
mod source_tests;
