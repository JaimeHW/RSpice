//! Application-level hardcopy source resolution and worker-snapshot tests.
//!
//! The cases verify exact active-document authority, isolation from mutable
//! application state, and strict tamper rejection at the worker boundary.

use super::*;

#[test]
fn global_app_resolver_uses_exact_active_design_registry_identity() {
    let mut state = AppState::default();
    state
        .schematic
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = state.workspace.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference.clone()));

    let first = resolve_active_app_hardcopy_source(&state).unwrap();
    let second = resolve_active_app_hardcopy_source(&state).unwrap();
    assert_eq!(
        first.authority().document_id(),
        second.authority().document_id()
    );
    assert_eq!(
        first.authority().revision(),
        state.workspace.project.revision()
    );
    assert!(first.source_key().contains(&reference.key()));
    let HardcopySemanticDocument::Schematic(schematic) = first.semantic_document() else {
        panic!("expected schematic")
    };
    assert_eq!(
        schematic.drawing_sheet.as_ref(),
        Some(
            &state
                .workspace
                .design_management
                .drawing_sheet_settings()
                .default_format
        )
    );
}

#[test]
fn ungoverned_current_sheet_and_worker_use_the_canvas_project_default() {
    let mut state = AppState::default();
    let reference = state.workspace.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let mut settings = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .clone();
    settings.default_format = settings
        .default_format
        .try_update(|draft| {
            draft.authored_size = crate::state::AuthoredDrawingSheetSize::Standard {
                standard: crate::state::DrawingSheetStandard::IsoA3,
            };
            draft.orientation = crate::state::SchematicPageOrientation::Landscape;
        })
        .unwrap();
    settings.title_block_field_values.insert(
        DrawingSheetTitleFieldId::Organization,
        "RSpice Engineering".to_owned(),
    );
    state
        .workspace
        .design_management
        .update_drawing_sheet_settings(state.workspace.design_management.revision(), settings)
        .unwrap();
    assert!(
        state
            .workspace
            .design_management
            .sheet_catalog(&state.workspace.active_key())
            .is_none()
    );
    let expected_format = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .default_format
        .clone();
    let source_key = format!(
        "project:{}:cell-view:{}",
        state.workspace.project.id().as_uuid(),
        state.workspace.active_key()
    );

    let synchronous =
        resolve_retained_hardcopy_source(&state, &source_key, HardcopyScope::CurrentSheet).unwrap();
    let HardcopySemanticDocument::Schematic(schematic) = synchronous.semantic_document() else {
        panic!("expected schematic")
    };
    assert_eq!(schematic.drawing_sheet.as_ref(), Some(&expected_format));
    assert_eq!(
        schematic
            .drawing_sheet_title_values
            .get(&DrawingSheetTitleFieldId::Organization)
            .map(String::as_str),
        Some("RSpice Engineering")
    );

    let prepared =
        prepare_retained_hardcopy_resolution(&state, &source_key, HardcopyScope::CurrentSheet)
            .unwrap();
    let bytes = prepared.into_worker_snapshot_json().unwrap();
    let restored = PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&bytes).unwrap();
    assert_eq!(restored.resolve_owned().unwrap(), synchronous);
}

#[test]
fn prepared_resolution_is_send_owned_and_snapshot_isolated() {
    fn assert_send<T: Send>() {}
    assert_send::<PreparedRetainedHardcopyResolution>();

    let mut state = AppState::default();
    state
        .schematic
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = state.workspace.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let source_key = format!(
        "project:{}:cell-view:{}",
        state.workspace.project.id().as_uuid(),
        state.workspace.active_key()
    );
    let synchronous =
        resolve_retained_hardcopy_source(&state, &source_key, HardcopyScope::ActiveDocument)
            .unwrap();
    let prepared =
        prepare_retained_hardcopy_resolution(&state, &source_key, HardcopyScope::ActiveDocument)
            .unwrap();
    state.schematic.wires[0].points[1].x = 9_999;
    let worker_resolved = prepared.resolve_owned().unwrap();
    assert_eq!(worker_resolved, synchronous);
}

fn prepared_design_worker_fixture() -> (PreparedRetainedHardcopyResolution, ResolvedHardcopyDocument)
{
    let mut state = AppState::default();
    state
        .schematic
        .wires
        .push(Wire::segment(771, Point::new(-4, 3), Point::new(29, 3)));
    let active_view = state.workspace.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(active_view));
    let source_key = format!(
        "project:{}:cell-view:{}",
        state.workspace.project.id().as_uuid(),
        state.workspace.active_key()
    );
    let expected =
        resolve_retained_hardcopy_source(&state, &source_key, HardcopyScope::ActiveDocument)
            .unwrap();
    let prepared =
        prepare_retained_hardcopy_resolution(&state, &source_key, HardcopyScope::ActiveDocument)
            .unwrap();
    (prepared, expected)
}

#[test]
fn prepared_worker_snapshot_round_trips_exact_owner_before_resolution() {
    let (prepared, expected) = prepared_design_worker_fixture();
    let bytes = prepared.into_worker_snapshot_json().unwrap();
    assert!(bytes.len() <= MAX_WORKER_SNAPSHOT_BYTES);
    let restored = PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&bytes).unwrap();
    assert_eq!(restored.resolve_owned().unwrap(), expected);
}

#[test]
fn prepared_worker_snapshot_rejects_tamper_unknown_fields_and_stale_identity() {
    let (prepared, _) = prepared_design_worker_fixture();
    let bytes = prepared.into_worker_snapshot_json().unwrap();

    let mut tampered: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    tampered["payload"]["identity"]["display_name"] =
        serde_json::Value::String("Tampered owner".to_owned());
    assert!(matches!(
        PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
            &serde_json::to_vec(&tampered).unwrap()
        ),
        Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
    ));

    let mut unknown: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    unknown
        .as_object_mut()
        .unwrap()
        .insert("future-field".to_owned(), serde_json::Value::Bool(true));
    assert!(matches!(
        PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
            &serde_json::to_vec(&unknown).unwrap()
        ),
        Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
    ));

    let mut stale: PreparedRetainedHardcopyWorkerSnapshot = serde_json::from_slice(&bytes).unwrap();
    let PreparedRetainedHardcopyWorkerPayload::Schematic { identity, .. } = &mut stale.payload
    else {
        panic!("expected prepared schematic")
    };
    identity.document_id = HardcopyDocumentId::new();
    stale.transport_digest = stale.compute_transport_digest().unwrap();
    assert!(matches!(
        PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
            &serde_json::to_vec(&stale).unwrap()
        ),
        Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
    ));
}

#[test]
fn prepared_worker_snapshot_rejects_unknown_owner_fields_even_with_resealed_transport() {
    let (prepared, _) = prepared_design_worker_fixture();
    let bytes = prepared.into_worker_snapshot_json().unwrap();
    let mut snapshot: PreparedRetainedHardcopyWorkerSnapshot =
        serde_json::from_slice(&bytes).unwrap();
    let PreparedRetainedHardcopyWorkerPayload::Schematic { schematic, .. } = &mut snapshot.payload
    else {
        panic!("expected prepared schematic")
    };
    schematic
        .0
        .as_object_mut()
        .unwrap()
        .insert("future-owner-field".to_owned(), serde_json::json!(17));
    snapshot.transport_digest = snapshot.compute_transport_digest().unwrap();
    assert!(matches!(
        PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
            &serde_json::to_vec(&snapshot).unwrap()
        ),
        Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
    ));
}

#[test]
fn prepared_worker_snapshot_rejects_oversized_input_before_parsing() {
    let oversized = vec![b' '; MAX_WORKER_SNAPSHOT_BYTES + 1];
    assert!(matches!(
        PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&oversized),
        Err(HardcopySourceError::PreparedWorkerSnapshotTooLarge(actual))
            if actual == MAX_WORKER_SNAPSHOT_BYTES + 1
    ));
}

#[test]
fn enumeration_exposes_all_sheets_exact_members_and_available_named_sets() {
    let mut state = AppState::default();
    state
        .schematic
        .wires
        .push(Wire::segment(91, Point::new(0, 0), Point::new(20, 0)));
    let reference = state.workspace.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let active_key = state.workspace.active_key();
    let first_id = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&active_key, "First", [91])
        .unwrap();
    let second_id = state
        .workspace
        .design_management
        .sheet_catalog_mut(&active_key)
        .unwrap()
        .create_sheet(sheet_definition("Second"), Some(first_id))
        .unwrap();
    let base_key = format!(
        "project:{}:cell-view:{}",
        state.workspace.project.id().as_uuid(),
        active_key
    );
    let first_key = format!("{base_key}:sheet:{first_id}");
    let first =
        resolve_retained_hardcopy_source(&state, &first_key, HardcopyScope::CurrentSheet).unwrap();
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "First only",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::NamedPrintSet("First only".to_owned()),
        vec![HardcopySourceSetMember::from_resolved(&first).unwrap()],
    )
    .unwrap();
    let set_key = source_set.source_key();
    state
        .workspace
        .save_hardcopy_source_set(source_set)
        .unwrap();

    let descriptors = enumerate_retained_hardcopy_sources(&state);
    let base = descriptors
        .iter()
        .find(|descriptor| descriptor.source_key == base_key)
        .unwrap();
    assert!(base.supports_scope(&HardcopyScope::AllSheetsOrPanes));
    let sheet_keys = descriptors
        .iter()
        .filter(|descriptor| {
            descriptor
                .source_key
                .starts_with(&format!("{base_key}:sheet:"))
        })
        .map(|descriptor| descriptor.source_key.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        sheet_keys,
        [first_key, format!("{base_key}:sheet:{second_id}"),]
    );
    let named = descriptors
        .iter()
        .find(|descriptor| descriptor.source_key == set_key)
        .unwrap();
    assert!(named.availability.is_available());
    assert_eq!(
        named.allowed_scopes,
        [HardcopyScope::NamedPrintSet("First only".to_owned())]
    );
}

#[test]
fn global_app_resolver_rejects_stale_results_registry() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00ffff"),
        ]),
    );
    state.simulation.runs.push(run);
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    state.workbench.activate(Workspace::Results);
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(DatasetId::new()));

    let error = resolve_active_app_hardcopy_source(&state).unwrap_err();
    assert!(matches!(
        error,
        HardcopySourceError::StaleActiveDocumentAuthority(_)
    ));
}

#[test]
fn global_app_resolver_does_not_guess_a_background_design_document() {
    let mut state = AppState::default();
    state
        .schematic
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(10, 0)));
    let error = resolve_active_app_hardcopy_source(&state).unwrap_err();
    assert!(matches!(
        error,
        HardcopySourceError::NoActiveDocumentAuthority("design")
    ));
}
