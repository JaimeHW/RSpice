//! Accepted content must not be rescanned by recurring dirty-state queries.

use super::*;

#[test]
fn accepted_content_is_fingerprinted_only_once_across_dirty_queries() {
    for samples in [2, 100_000] {
        let (mut state, _) = super::durable_content::retained_results();
        state.simulation.runs[0].analyses[0].waveforms = vec![crate::state::WaveformData::new(
            "V(out)",
            (0..samples).map(|index| index as f64).collect::<Vec<_>>(),
            vec![1.0; samples],
            "#ffffff",
        )];
        assert!(state.simulation.select_run(0));
        let baseline = snapshot(&state).unwrap();
        registry::FINGERPRINT_PASSES.with(|passes| passes.set(0));
        state.project_lifecycle.accepted = Some(AcceptedProject::new(baseline, None));
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

#[test]
fn accepted_snapshot_clones_share_content_and_edits_cannot_change_their_fingerprints() {
    let mut state = AppState::default();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    let accepted = AcceptedProject::new(snapshot(&state).unwrap(), None);
    let original_digest = accepted.fingerprints().unwrap().content_digest();
    registry::FINGERPRINT_PASSES.with(|passes| passes.set(0));
    let retained = accepted.clone();
    assert!(std::ptr::eq(accepted.baseline(), retained.baseline()));
    assert!(std::ptr::eq(
        accepted.fingerprints().unwrap(),
        retained.fingerprints().unwrap()
    ));
    assert_eq!(registry::FINGERPRINT_PASSES.with(std::cell::Cell::get), 0);
    state.project_lifecycle.accepted = Some(accepted);

    assert!(state.schematic.with_undo("Place resistor", |schematic| {
        schematic.add_component(ComponentType::Resistor, Point::new(10, 20));
    }));
    assert!(has_unsaved_changes(&state));
    assert!(active_document_is_dirty(&state));
    assert_eq!(dirty_document_count(&state), 1);
    assert_eq!(
        retained.fingerprints().unwrap().content_digest(),
        original_digest
    );
    assert!(state.schematic.undo());
    assert!(!has_unsaved_changes(&state));
    assert!(state.schematic.redo());
    assert!(has_unsaved_changes(&state));

    let replacement = snapshot(&state).unwrap();
    accept_loaded_project(&mut state, replacement, None);
    assert!(!has_unsaved_changes(&state));
    let current = state.project_lifecycle.accepted().unwrap();
    assert!(!std::ptr::eq(current.baseline(), retained.baseline()));
    assert_ne!(
        current.fingerprints().unwrap().content_digest(),
        original_digest
    );
    assert_eq!(
        retained.fingerprints().unwrap().content_digest(),
        original_digest
    );
}
