//! Accepted content must not be rescanned by recurring dirty-state queries.

use super::*;
use crate::state::result_presentation::AnalysisPresentationKey;

/// A new session holding one completed transient run, selected, with the
/// results workspace in front.
fn retained_results() -> (ServicesHost, AnalysisPresentationKey) {
    let mut state = ServicesHost::default();
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "Tran")
            .with_waveforms(vec![crate::state::RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![1.0, 2.0],
                "#ffffff",
            )]),
    );
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .unwrap();
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state.model.simulation.retained.runs.push(run);
    assert!(state.model.simulation.select_run(0));
    state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Results);
    (state, key)
}

#[test]
fn accepted_content_is_fingerprinted_only_once_across_dirty_queries() {
    for samples in [2, 100_000] {
        let (mut state, _) = retained_results();
        state.model.simulation.retained.runs[0].analyses[0].waveforms =
            vec![crate::state::RetainedTrace::new(
                "V(out)",
                (0..samples).map(|index| index as f64).collect::<Vec<_>>(),
                vec![1.0; samples],
                "#ffffff",
            )];
        assert!(state.model.simulation.select_run(0));
        let baseline = snapshot(&state).unwrap();
        registry::FINGERPRINT_PASSES.with(|passes| passes.set(0));
        state
            .platform
            .project_lifecycle
            .accept_project(baseline, None);
        for _ in 0..3 {
            assert!(!has_unsaved_changes(&state));
            assert!(!active_document_is_dirty(&state));
            assert_eq!(dirty_document_count(&state), 0);
            refresh_registry(&mut state).unwrap();
        }
        let passes = registry::FINGERPRINT_PASSES.with(std::cell::Cell::get);
        // Until mutation revisions cover all working edits, each query may
        // still scan that working state. The accepted state gets one pass
        // for its entire lifetime, regardless of dataset size or frame count.
        assert!(
            passes <= 13,
            "{samples} samples required {passes} full fingerprint passes; budget is one accepted pass plus twelve working queries"
        );
    }
}
