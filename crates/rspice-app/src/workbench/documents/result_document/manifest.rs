//! Dataset-manifest cache ownership, exact export and application actions.

use super::frame_work::{self, DatasetWalk};
use crate::{
    state::{RunHistoryRevision, SimulationRun},
    workbench::AppState,
};
use egui::Ui;
use rspice_results::manifest::ManifestViewModel;
use rspice_results_ui::manifest as viewer;
use std::sync::Arc;
pub(crate) fn manifest_for_run(run: &SimulationRun) -> ManifestViewModel {
    frame_work::note(DatasetWalk::ManifestViewModel);
    frame_work::note(DatasetWalk::DatasetDigest);
    ManifestViewModel::from_run(run, |analysis| analysis.is_live_partial())
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManifestPlan {
    source: (RunHistoryRevision, u64),
    run: u64,
    pub(crate) model: ManifestViewModel,
}
pub(crate) fn active_manifest(state: &mut AppState) -> Option<Arc<ManifestPlan>> {
    let source = (
        state.simulation.runs.revision(),
        state.simulation.data_version,
    );
    let run_id = state.simulation.active_run()?.id;
    if let Some(plan) = state.ui.results.plans.manifest.as_ref()
        && plan.source == source
        && plan.run == run_id
    {
        return Some(Arc::clone(plan));
    }
    let model = manifest_for_run(state.simulation.active_run()?);
    let built = Arc::new(ManifestPlan {
        source,
        run: run_id,
        model,
    });
    state.ui.results.plans.manifest = Some(Arc::clone(&built));
    Some(built)
}
pub(crate) fn show(ui: &mut Ui, state: &mut AppState) {
    let plan = active_manifest(state);
    let lifecycle_is_terminal = state
        .simulation
        .active_run()
        .is_some_and(|run| run.lifecycle.is_terminal());
    viewer::show(
        ui,
        plan.as_ref().map(|plan| &plan.model),
        lifecycle_is_terminal,
    );
}
pub(crate) fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(plan) = active_manifest(state) else {
        return;
    };
    let Some(run) = state.simulation.active_run() else {
        return;
    };
    let run_sequence = run.id;
    let saved_outputs = state
        .simulation
        .active_analysis()
        .map(|analysis| viewer::SavedOutputs {
            run_id: run.run_id,
            analysis_id: analysis.id,
            analysis_label: &analysis.label,
            receipts: &analysis.saved_output_receipts,
        });
    let executed_deck_points = state
        .simulation
        .executed_decks
        .get(run_sequence)
        .map_or(0, |deck| deck.points.len());
    let plan_block = crate::workbench::state::plan_provenance::producing_plan_block(
        &state.simulation,
        state.sim_setup.stable_analysis_plan().ok(),
    );
    let response = viewer::right_panel(
        ui,
        &viewer::ManifestPanel {
            manifest: &plan.model,
            saved_outputs,
            executed_deck_points,
            plan_block,
            open_task_deck_label: crate::workbench::commands::vocabulary::OPEN_TASK_DECK,
        },
    );
    if let Some(viewer::MaterializeRequest {
        run_id,
        analysis_id,
        receipt_index,
    }) = response.materialize
    {
        match state
            .simulation
            .materialize_deferred_saved_output(run_id, analysis_id, receipt_index)
        {
            Ok(()) => {
                state.synchronize_specialized_viewer_cache_authority();
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(
                    "Deferred saved output materialized from retained source data".to_owned(),
                ));
            }
            Err(error) => state.push_sim_message(crate::diagnostics::ConsoleMessage::error(
                format!("Saved output could not be materialized: {error}"),
            )),
        }
    }
    if response.open_plan {
        state.ui.open_producing_plan_requested = true;
    }
    // Acted on after the panel is drawn, like the plan route above it, so the
    // workspace switch happens between frames rather than under the widget
    // that asked for it. The route answers whether the bytes are still held,
    // and a released deck is refused by name rather than opening nothing.
    if response.open_task_deck
        && !crate::workbench::documents::netlist_document::reveal_executed_deck(
            state,
            run_sequence,
            0,
        )
    {
        state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
            "The source Run {run_sequence} executed cannot be opened: {}.",
            crate::state::absent_deck_reason()
        )));
    }
}
pub(crate) fn export_csv(run: &SimulationRun) -> super::ResultSheetCsv {
    let manifest = manifest_for_run(run);
    super::ResultSheetCsv {
        default_name: "rspice-result-manifest.csv",
        detail: format!("{} retained analyses", manifest.rows.len()),
        contents: rspice_formats::result_csv::encode_manifest_csv(&manifest),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisType, SimulationRun, SimulationRunLifecycle, WaveformData,
    };
    use rspice_results::manifest::ManifestRow;
    fn row_for_analysis(analysis: AnalysisResult) -> ManifestRow {
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        manifest_for_run(&run).rows.remove(0)
    }
    fn state_with_run(label: &str) -> AppState {
        let mut run = SimulationRun::new(7);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, label).with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#ffbd2e"),
            ]),
        );
        let mut state = AppState::default();
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state
    }
    #[test]
    fn the_periodic_small_signal_family_names_one_quantity_on_both_surfaces() {
        for periodic in [
            AnalysisType::Pac,
            AnalysisType::Pxf,
            AnalysisType::Qpac,
            AnalysisType::Qpxf,
            AnalysisType::Pnoise,
            AnalysisType::Qpnoise,
            AnalysisType::Hbnoise,
        ] {
            let meta = row_for_analysis(AnalysisResult::new(1, periodic, "periodic"));
            assert_eq!(
                meta.domain_axis,
                if matches!(periodic, AnalysisType::Qpxf | AnalysisType::Qpnoise) {
                    "output frequency"
                } else {
                    "offset frequency"
                },
                "{periodic:?}"
            );
            assert_eq!(
                periodic.axis_info().0.to_ascii_lowercase(),
                meta.domain_axis,
                "the Studio caption and the manifest domain disagree for {periodic:?}"
            );
        }
        // The quantity these were named after is a different number, and
        // nothing shipped here publishes it as an abscissa. Split so this
        // test's own prose cannot trip the scan.
        let retired = ["translated ", "frequency"].concat();
        assert!(
            [
                include_str!("manifest.rs"),
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../rspice-results-ui/src/manifest.rs"
                )),
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../rspice-results/src/manifest.rs"
                )),
            ]
            .into_iter()
            .all(|source| !crate::source_guard::production_source(source).contains(&retired)),
            "the translated frequency is `offset + n*f0`, not the swept axis"
        );
    }
    #[test]
    fn retained_view_source_manifest_tracks_restoration_and_nested_edits() {
        let mut state = AppState::default();
        let run = state.simulation.start_run();
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 2.0], "#ffbd2e"),
            ]),
        );
        run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
            .unwrap();
        run.mark_running().unwrap();
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .unwrap();
        state.simulation.complete_run();
        let original = active_manifest(&mut state).unwrap();
        let version = state.simulation.data_version;
        let mut replacement = state.simulation.clone();
        replacement.runs[0].analyses[0].waveforms[0].y = Arc::new(vec![0.0, 9.0]);
        state.simulation = crate::io::simulation_state_from_results(
            crate::io::capture_simulation_results(&replacement),
        )
        .unwrap();
        assert_eq!(state.simulation.data_version, version);
        let restored = active_manifest(&mut state).unwrap();
        assert_eq!(restored.model.dataset_id, original.model.dataset_id);
        assert_ne!(restored.model.dataset_digest, original.model.dataset_digest);
        assert_eq!(
            restored.model,
            manifest_for_run(state.simulation.active_run().unwrap())
        );

        state.simulation.runs[0].analyses[0]
            .waveforms
            .push(WaveformData::new(
                "I(V1)",
                vec![0.0, 1.0],
                vec![0.0, -0.01],
                "#55aaff",
            ));
        let edited = active_manifest(&mut state).unwrap();
        assert_eq!(
            edited.model,
            manifest_for_run(state.simulation.active_run().unwrap())
        );
        assert_ne!(edited.model, restored.model);
        assert_eq!(state.simulation.data_version, version);
    }
    #[test]
    fn retained_view_source_manifest_reuses_large_unchanged_history_clones() {
        let mut state = state_with_run("Transient");
        state.simulation.runs[0].analyses[0].waveforms = vec![WaveformData::new(
            "V(out)",
            (0..100_000).map(f64::from).collect::<Vec<_>>(),
            vec![2.0; 100_000],
            "#ffbd2e",
        )];
        let original = active_manifest(&mut state).unwrap();
        let mut other = AppState::default();
        other.simulation = state.simulation.clone();
        other.ui.results = state.ui.results.clone();
        let work = frame_work::WorkCounts::reset();
        for _ in 0..12 {
            assert!(Arc::ptr_eq(
                &original,
                &active_manifest(&mut state).unwrap()
            ));
            assert!(Arc::ptr_eq(
                &original,
                &active_manifest(&mut other).unwrap()
            ));
        }
        assert_eq!(work.since().total(), 0);
        other.simulation.runs[0].analyses[0].waveforms[0].y = Arc::new(vec![3.0; 100_000]);
        assert_ne!(
            active_manifest(&mut other).unwrap().model.dataset_digest,
            original.model.dataset_digest
        );
        assert!(Arc::ptr_eq(
            &original,
            &active_manifest(&mut state).unwrap()
        ));
    }
    #[test]
    fn the_memoized_manifest_is_the_projection_it_replaced() {
        let mut state = state_with_run("Transient");
        let direct = manifest_for_run(
            state
                .simulation
                .active_run()
                .expect("the fixture selects a run"),
        );

        let plan = active_manifest(&mut state).expect("a manifest for the active run");
        assert_eq!(plan.model, direct);
        assert_eq!(
            plan.model.dataset_digest,
            state
                .simulation
                .active_run()
                .expect("the fixture selects a run")
                .dataset_content_digest()
                .to_string()
        );

        // Asking again is the same answer from the same allocation.
        let again = active_manifest(&mut state).expect("a manifest for the active run");
        assert!(Arc::ptr_eq(&plan, &again));
    }
    #[test]
    fn a_new_dataset_generation_reprojects_the_manifest() {
        let mut state = state_with_run("Transient");
        let before = active_manifest(&mut state).expect("a manifest for the active run");
        let before_digest = before.model.dataset_digest.clone();

        state.simulation.runs[0].analyses[0].waveforms[0].y = std::sync::Arc::new(vec![0.0, 9.0]);
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

        let after = active_manifest(&mut state).expect("a manifest for the active run");
        assert_ne!(
            after.model.dataset_digest, before_digest,
            "the sheet kept the previous dataset generation's content digest"
        );
        assert_eq!(
            after.model.dataset_digest,
            state.simulation.runs[0]
                .dataset_content_digest()
                .to_string()
        );
    }
    #[test]
    fn selecting_another_run_reprojects_the_manifest() {
        let mut state = state_with_run("First");
        let first = active_manifest(&mut state)
            .expect("a manifest for the active run")
            .model
            .clone();

        let mut second = SimulationRun::new(9);
        second.lifecycle = SimulationRunLifecycle::Completed;
        second.add_analysis(
            AnalysisResult::new(2, AnalysisType::Transient, "Second").with_waveforms(vec![
                WaveformData::new("V(out)", vec![0.0, 1.0], vec![5.0, 6.0], "#ffbd2e"),
            ]),
        );
        state.simulation.runs.push(second);
        assert!(state.simulation.select_run(1));

        let after = active_manifest(&mut state).expect("a manifest for the active run");
        assert_ne!(after.model.run_sequence, first.run_sequence);
        assert_ne!(after.model.dataset_digest, first.dataset_digest);
    }
}
