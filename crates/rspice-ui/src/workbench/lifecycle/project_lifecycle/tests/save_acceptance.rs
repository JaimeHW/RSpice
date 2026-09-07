//! A published snapshot remains authoritative even if newer drafts are invalid.

use super::*;

#[test]
fn acknowledged_save_adopts_published_content_despite_invalid_newer_draft() {
    for scope in [SaveScope::AllDocuments, SaveScope::ActiveDocument] {
        let path = unique_path("acknowledged-invalid-draft");
        let mut state = AppState::default();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .unwrap();
        let saved_document = active_document(&state);
        let previous_generation = accepted_generation(&state);
        let previous_digest = state
            .project_lifecycle
            .accepted()
            .unwrap()
            .binding
            .as_ref()
            .unwrap()
            .accepted_digest();
        state.schematic.with_undo("Place resistor", |schematic| {
            schematic.add_component(ComponentType::Resistor, Point::new(1, 1));
        });
        let candidate = snapshot(&state).unwrap();
        let candidate_content = registry::content_digest(&candidate).unwrap();
        let (bytes, staged_digest) = persistence::serialized_project(&candidate).unwrap();

        // Freeze bytes before the write. Inject a validation failure and a
        // newer authored edit while that write is pending. The browser's
        // post-publication adoption path must not depend on this newer draft.
        let valid_reltol = state.sim_setup.options.reltol;
        state.sim_setup.options.reltol = -1.0;
        state.schematic.with_undo("Place capacitor", |schematic| {
            schematic.add_component(ComponentType::Capacitor, Point::new(4, 1));
        });
        assert!(snapshot(&state).is_err());
        let digest = persistence::publish_canonical_native(
            &path,
            crate::io::durable_file::ExpectedContent::Digest(*previous_digest.as_bytes()),
            &bytes,
        )
        .unwrap();
        assert_eq!(digest, staged_digest);
        let binding = PersistenceBinding::Native {
            canonical_path: path.clone(),
            accepted_digest: digest,
        };
        finish_successful_save(&mut state, candidate, binding, scope);

        assert_eq!(
            accepted_generation(&state),
            previous_generation + 1,
            "an invalid newer draft must not discard an acknowledged save"
        );
        let accepted = state.project_lifecycle.accepted().unwrap();
        assert_eq!(
            accepted.fingerprints().unwrap().content_digest(),
            candidate_content
        );
        assert_eq!(accepted.binding.as_ref().unwrap().accepted_digest(), digest);
        assert_eq!(
            registry::content_digest(&crate::io::load_project_file(&path).unwrap()).unwrap(),
            candidate_content
        );
        assert_eq!(
            state.schematic.components.len(),
            2,
            "later edits survive adoption"
        );
        assert_eq!(state.sim_setup.options.reltol, -1.0);
        assert!(state.log_buffer.entries().any(|entry| {
            entry.severity == LogSeverity::Warning && entry.message.contains("current draft")
        }));
        assert!(has_unsaved_changes(&state));
        assert!(active_document_is_dirty(&state));
        assert!(state.schematic.is_dirty);
        assert!(
            state
                .project_lifecycle
                .registry
                .is_dirty(&ProjectDocumentId::SimulationPlan)
        );
        for continuation_scope in [SaveScope::AllDocuments, SaveScope::ActiveDocument] {
            assert!(!saved_snapshot_authorizes_continuation(
                &state,
                continuation_scope,
                &saved_document
            ));
        }

        // Repairing the draft restores an exact comparison with the newly
        // written baseline. Undo removes only the edit made after staging.
        state.sim_setup.options.reltol = valid_reltol;
        refresh_registry(&mut state).unwrap();
        assert!(
            !state
                .project_lifecycle
                .registry
                .is_dirty(&ProjectDocumentId::SimulationPlan)
        );
        assert!(active_document_is_dirty(&state));
        assert!(state.schematic.undo());
        refresh_registry(&mut state).unwrap();
        assert!(!has_unsaved_changes(&state));
        assert!(!state.schematic.is_dirty);
        assert!(saved_snapshot_authorizes_continuation(
            &state,
            scope,
            &saved_document
        ));
        remove_project_artifacts(&path);
    }
}

#[test]
fn failed_registry_refresh_cannot_leave_clean_document_indicators() {
    let mut state = AppState::default();
    state.project_lifecycle.accepted = Some(AcceptedProject::new(snapshot(&state).unwrap(), None));
    refresh_registry(&mut state).unwrap();
    let library_revision = state.library_manager.revision();
    let valid_reltol = state.sim_setup.options.reltol;
    state.sim_setup.options.reltol = -1.0;
    assert!(refresh_registry(&mut state).is_err());
    assert!(state.schematic.is_dirty);
    assert!(state.workspace.open_views.iter().all(|view| view.dirty));
    assert!(
        state
            .workspace
            .schematic_buffers
            .values()
            .all(|schematic| schematic.is_dirty)
    );
    assert!(state.workspace.project_metadata_dirty);
    assert!(state.workspace.netlist_source_dirty);
    assert!(state.workspace.project_sources_dirty);
    assert!(
        state
            .project_lifecycle
            .registry
            .records()
            .iter()
            .all(|record| record.dirty)
    );
    assert!(
        state
            .project_lifecycle
            .registry
            .is_dirty(&ProjectDocumentId::CellView(CellViewRef::new(
                "new",
                "uncompared",
                "schematic"
            )))
    );
    assert!(
        state
            .library_manager
            .libraries_by_key()
            .all(|(_, library)| {
                library
                    .cells
                    .values()
                    .all(|cell| cell.views.values().all(|view| view.modified))
            })
    );
    assert_eq!(state.library_manager.revision(), library_revision);

    state.sim_setup.options.reltol = valid_reltol;
    refresh_registry(&mut state).unwrap();
    assert!(!has_unsaved_changes(&state));
    assert!(!state.schematic.is_dirty);
    assert!(state.workspace.open_views.iter().all(|view| !view.dirty));
    assert!(!state.workspace.project_metadata_dirty);
    assert!(!state.workspace.netlist_source_dirty);
    assert!(!state.workspace.project_sources_dirty);
    assert!(
        state
            .project_lifecycle
            .registry
            .records()
            .iter()
            .all(|record| !record.dirty)
    );
    assert!(
        state
            .library_manager
            .libraries_by_key()
            .all(|(_, library)| {
                library
                    .cells
                    .values()
                    .all(|cell| cell.views.values().all(|view| !view.modified))
            })
    );
    assert_eq!(state.library_manager.revision(), library_revision);
}
