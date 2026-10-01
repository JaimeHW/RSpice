use super::*;
use rspice_design::schematic::{document::SchematicDocument, owned::Schematic};

fn prepared_design_worker_fixture() -> (PreparedRetainedHardcopyResolution, ResolvedHardcopyDocument)
{
    let project_id = ProjectId::new();
    let view_key = "user:test:schematic";
    let identity = HardcopySourceIdentity::try_new(
        format!("project:{}:cell-view:{view_key}", project_id.as_uuid()),
        prepared_base_design_document_id(project_id, view_key).unwrap(),
        ObjectRevision::INITIAL,
        "Test schematic",
    )
    .unwrap();
    let schematic = Schematic::from_document(SchematicDocument {
        wires: vec![Wire::segment(771, Point::new(-4, 3), Point::new(29, 3))],
        ..Default::default()
    });
    let project_settings =
        rspice_design_model::design_management::DrawingSheetProjectSettings::default();
    let expected = resolve_schematic_source(SchematicHardcopySource {
        identity: identity.clone(),
        schematic: &schematic,
        selection: None,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: Some(&project_settings.default_format),
        project_title_block_field_values: Some(&project_settings.title_block_field_values),
        scope: HardcopyScope::ActiveDocument,
    })
    .unwrap();
    let prepared =
        PreparedRetainedHardcopyResolution::try_capture(RetainedHardcopySourceInput::Schematic {
            project_id,
            identity,
            schematic,
            selection: Selection::default(),
            library_manager: rspice_project_contract::ProjectLibraries::new(),
            schematic_buffers: Default::default(),
            sheet_catalog: None,
            sheet_id: None,
            project_default_drawing_sheet: project_settings.default_format,
            project_title_block_field_values: project_settings.title_block_field_values,
            all_sheets: false,
            scope: HardcopyScope::ActiveDocument,
        })
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
    for invalid_scope in [false, true] {
        let (prepared, _) = prepared_design_worker_fixture();
        let mut input = prepared.payload;
        let RetainedHardcopySourceInput::Schematic {
            identity, scope, ..
        } = &mut input
        else {
            panic!("expected captured schematic");
        };
        if invalid_scope {
            *scope = HardcopyScope::CompleteReport;
        } else {
            identity.document_id = HardcopyDocumentId::new();
        }
        assert!(matches!(
            PreparedRetainedHardcopyResolution::try_capture(input),
            Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(_))
        ));
    }
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
