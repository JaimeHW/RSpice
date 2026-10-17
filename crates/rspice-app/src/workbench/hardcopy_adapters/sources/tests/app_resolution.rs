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
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = state.workspace.content.active_view.clone();
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
        state.workspace.content.project.revision()
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
                .content
                .design_management
                .drawing_sheet_settings()
                .default_format
        )
    );
}

#[test]
fn ungoverned_current_sheet_and_worker_use_the_canvas_project_default() {
    let mut state = AppState::default();
    let reference = state.workspace.content.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let mut settings = state
        .workspace
        .content
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
    let revision = state.workspace.content.design_management.revision();
    state
        .workspace
        .content
        .design_management
        .update_drawing_sheet_settings(revision, settings)
        .unwrap();
    assert!(
        state
            .workspace
            .content
            .design_management
            .sheet_catalog(&state.workspace.content.active_key())
            .is_none()
    );
    let expected_format = state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .default_format
        .clone();
    let source_key = format!(
        "project:{}:cell-view:{}",
        state.workspace.content.project.id().as_uuid(),
        state.workspace.content.active_key()
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
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = state.workspace.content.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let source_key = format!(
        "project:{}:cell-view:{}",
        state.workspace.content.project.id().as_uuid(),
        state.workspace.content.active_key()
    );
    let synchronous =
        resolve_retained_hardcopy_source(&state, &source_key, HardcopyScope::ActiveDocument)
            .unwrap();
    let prepared =
        prepare_retained_hardcopy_resolution(&state, &source_key, HardcopyScope::ActiveDocument)
            .unwrap();
    state.schematic.document_mut_for_test().wires[0].points[1].x = 9_999;
    let worker_resolved = prepared.resolve_owned().unwrap();
    assert_eq!(worker_resolved, synchronous);
}

#[test]
fn enumeration_exposes_all_sheets_exact_members_and_available_named_sets() {
    let mut state = AppState::default();
    state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(91, Point::new(0, 0), Point::new(20, 0)));
    let reference = state.workspace.content.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let active_key = state.workspace.content.active_key();
    let first_id = state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&active_key, "First", [91])
        .unwrap();
    let second_id = state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&active_key)
        .unwrap()
        .create_sheet(sheet_definition("Second"), Some(first_id))
        .unwrap();
    let base_key = format!(
        "project:{}:cell-view:{}",
        state.workspace.content.project.id().as_uuid(),
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
        vec![source_set_member_from_resolved(&first).unwrap()],
    )
    .unwrap();
    let set_key = source_set.source_key();
    state
        .workspace
        .content
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

/// One cell view split into two sheets whose only wires lie on exactly the
/// same coordinates. A print path that ignores the sheet partition draws the
/// two sheets on top of each other, and these wire ids show it.
fn two_sheets_drawn_at_the_same_place() -> (AppState, String, [SheetId; 2]) {
    let mut state = AppState::default();
    for wire_id in [11, 22] {
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(wire_id, Point::new(0, 0), Point::new(20, 0)));
    }
    let reference = state.workspace.content.active_view.clone();
    state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let active_key = state.workspace.content.active_key();
    let design_management = &mut state.workspace.content.design_management;
    let first = design_management
        .bootstrap_for_cell_view(&active_key, "First", [11])
        .unwrap();
    let catalog = design_management.sheet_catalog_mut(&active_key).unwrap();
    let second = catalog
        .create_sheet(sheet_definition("Second"), Some(first))
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), second, [22])
        .unwrap();
    let base_key = format!(
        "project:{}:cell-view:{active_key}",
        state.workspace.content.project.id().as_uuid()
    );
    (state, base_key, [first, second])
}

/// The wires on each printed page, in page order.
fn printed_wire_ids(resolved: &ResolvedHardcopyDocument) -> Vec<Vec<u64>> {
    let wires = |document: &HardcopySemanticDocument| match document {
        HardcopySemanticDocument::Schematic(schematic) => {
            schematic.wires.iter().map(|wire| wire.id).collect()
        }
        other => panic!("expected a schematic page, got {other:?}"),
    };
    match resolved.semantic_document() {
        HardcopySemanticDocument::Aggregate(aggregate) => aggregate
            .children
            .iter()
            .map(|child| wires(&child.document))
            .collect(),
        document => vec![wires(document)],
    }
}

fn resolve_prepared(
    state: &AppState,
    source_key: &str,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    prepare_retained_hardcopy_resolution(state, source_key, scope)?.resolve_owned()
}

#[test]
fn a_multi_sheet_cell_view_prints_every_sheet_on_its_own_page() {
    let (mut state, base_key, [first, second]) = two_sheets_drawn_at_the_same_place();
    let base = enumerate_retained_hardcopy_sources(&state)
        .into_iter()
        .find(|descriptor| descriptor.source_key == base_key)
        .unwrap();
    // "Active document" would only repeat "All sheets" under a second name,
    // and it printed every sheet overlaid on one page.
    assert_eq!(
        base.allowed_scopes,
        [
            HardcopyScope::Selection,
            HardcopyScope::CurrentSheet,
            HardcopyScope::AllSheetsOrPanes,
        ]
    );
    assert!(matches!(
        resolve_prepared(&state, &base_key, HardcopyScope::ActiveDocument),
        Err(HardcopySourceError::UnsupportedScope(
            HardcopyScope::ActiveDocument
        ))
    ));
    assert_eq!(
        printed_wire_ids(
            &resolve_prepared(&state, &base_key, HardcopyScope::AllSheetsOrPanes).unwrap()
        ),
        [vec![11], vec![22]]
    );
    for (sheet, wire) in [(first, 11), (second, 22)] {
        let sheet_key = format!("{base_key}:sheet:{sheet}");
        assert_eq!(
            printed_wire_ids(
                &resolve_prepared(&state, &sheet_key, HardcopyScope::CurrentSheet).unwrap()
            ),
            [[wire]]
        );
    }
    assert_eq!(
        printed_wire_ids(
            &resolve_prepared(&state, &base_key, HardcopyScope::CurrentSheet).unwrap()
        ),
        [[11]]
    );
    let active_key = state.workspace.content.active_key();
    state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&active_key)
        .unwrap()
        .set_active(second)
        .unwrap();
    assert_eq!(
        printed_wire_ids(
            &resolve_prepared(&state, &base_key, HardcopyScope::CurrentSheet).unwrap()
        ),
        [[22]]
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
    state.simulation.retained.runs.push(run);
    state.simulation.view.active_run_idx = Some(0);
    state.simulation.view.active_analysis_idx = Some(0);
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
        .document_mut_for_test()
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(10, 0)));
    let error = resolve_active_app_hardcopy_source(&state).unwrap_err();
    assert!(matches!(
        error,
        HardcopySourceError::NoActiveDocumentAuthority("design")
    ));
}
