//! An accepted snapshot's clones share its content, and later edits cannot
//! change their fingerprints.

use super::*;

#[test]
fn accepted_snapshot_clones_share_content_and_edits_cannot_change_their_fingerprints() {
    let mut state = AppState::default();
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    let baseline = snapshot(&state).unwrap();
    state
        .platform
        .project_lifecycle
        .accept_project(baseline, None);
    let accepted = state.platform.project_lifecycle.accepted().unwrap();
    let original_digest = accepted.fingerprints().unwrap().content_digest();
    registry::FINGERPRINT_PASSES.with(|passes| passes.set(0));
    let retained = state.platform.project_lifecycle.clone();
    assert!(std::ptr::eq(
        accepted.baseline(),
        retained.accepted().unwrap().baseline()
    ));
    assert!(std::ptr::eq(
        accepted.fingerprints().unwrap(),
        retained.accepted().unwrap().fingerprints().unwrap()
    ));
    assert_eq!(registry::FINGERPRINT_PASSES.with(std::cell::Cell::get), 0);

    assert!(
        state
            .model
            .schematic
            .with_undo("Place resistor", |schematic| {
                schematic.add_component(ComponentType::Resistor, Point::new(10, 20));
            })
    );
    assert!(has_unsaved_changes(&state));
    assert!(active_document_is_dirty(&state));
    assert_eq!(dirty_document_count(&state), 1);
    assert_eq!(
        retained
            .accepted()
            .unwrap()
            .fingerprints()
            .unwrap()
            .content_digest(),
        original_digest
    );
    assert!(state.model.schematic.undo());
    assert!(!has_unsaved_changes(&state));
    assert!(state.model.schematic.redo());
    assert!(has_unsaved_changes(&state));

    let replacement = snapshot(&state).unwrap();
    accept_loaded_project(&mut state, replacement, None);
    state.assert_project_open_fact();
    assert!(!has_unsaved_changes(&state));
    let current = state.platform.project_lifecycle.accepted().unwrap();
    assert!(!std::ptr::eq(
        current.baseline(),
        retained.accepted().unwrap().baseline()
    ));
    assert_ne!(
        current.fingerprints().unwrap().content_digest(),
        original_digest
    );
    assert_eq!(
        retained
            .accepted()
            .unwrap()
            .fingerprints()
            .unwrap()
            .content_digest(),
        original_digest
    );
}
