//! Tests for what a hardcopy source will and will not resolve to.
//!
//! Most cases pin a refusal: oversized input is rejected before parsing, a
//! stale results registry is not read, and the resolver never guesses a
//! background document when no source is active.

use std::collections::HashMap;

use super::*;
use crate::product::{DatasetBinding, DatasetId, ResultDocumentId, VerificationEvidenceId};
use crate::results::report_document::{
    DataTableBlock, EvidenceBlock, PlotFigureBlock, ReportEdit, ReportEntityRef,
    ReportFigureSourceLocator, ReportReferenceFigureArtifact, ReportReferenceInventoryEntry,
    ReportReferenceSnapshot, ReportSourceId, TableCell, TableColumn,
};
use crate::results::visualization_document::{
    AxisOrientation, AxisScale, ColumnRole, DocumentEdit, EntityRef, NewAxis, NewTrace,
    SourceColumn, SourceDataset, SourceRow, TypedValue, ValueType, VisualizationDocument,
};
use crate::state::{
    AnalysisType, Cell, ComplexResultValue, Library, LibraryCellInstance, LibraryManager,
    MonteCarloVariableMetadata, PortDirection, SheetDefinition, SheetPortPolicy, SheetTemplate,
    SimulationRunLifecycle, View, ViewType,
};

fn identity(key: &str) -> HardcopySourceIdentity {
    HardcopySourceIdentity::try_new(
        key,
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Active document",
    )
    .unwrap()
}

fn quick_view_state(analysis: AnalysisResult, viewer: ResultViewer) -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(analysis);
    state.simulation.runs.push(run);
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    state.ui.results.viewer = viewer;
    state
}

fn resolve_quick_view(state: &AppState) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    resolve_results_quick_view_source(ResultsQuickViewHardcopySource {
        source_key: "results-quick-view".to_owned(),
        project_id: state.workspace.project.id(),
        state,
        scope: HardcopyScope::ActivePlotDocument,
    })
}

fn report_with_block(kind: ReportBlockKind) -> ReportDocument {
    let mut report = ReportDocument::new("Authenticated report").unwrap();
    let receipt = report
        .transact(
            report.revision(),
            vec![ReportEdit::AddPage {
                title: "Results".to_owned(),
            }],
            1,
        )
        .unwrap();
    let ReportEntityRef::Page(page_id) = receipt.created[0] else {
        unreachable!()
    };
    let receipt = report
        .transact(
            report.revision(),
            vec![ReportEdit::AddSection {
                page_id,
                title: "Evidence".to_owned(),
            }],
            2,
        )
        .unwrap();
    let ReportEntityRef::Section(section_id) = receipt.created[0] else {
        unreachable!()
    };
    report
        .transact(
            report.revision(),
            vec![ReportEdit::AddBlock { section_id, kind }],
            3,
        )
        .unwrap();
    report
}

fn opaque_rgb8_png(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, width, height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer
            .write_image_data(&vec![0x55; width as usize * height as usize * 3])
            .unwrap();
    }
    bytes
}

#[test]
fn schematic_digest_ignores_viewport_state_but_changes_with_authored_content() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(1, Point::new(-10, 0), Point::new(20, 0)));
    let topology = schematic.topology_version();
    let first = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: &schematic,
        expected_topology_version: topology,
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap();

    schematic.zoom = 7.5;
    schematic.pan = (123.0, -44.0);
    let second = resolve_schematic_source(SchematicHardcopySource {
        identity: HardcopySourceIdentity {
            document_id: first.authority().document_id(),
            ..identity("schematic")
        },
        schematic: &schematic,
        expected_topology_version: topology,
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap();
    assert_eq!(
        first.authority().content_digest(),
        second.authority().content_digest()
    );

    schematic.wires[0].points[1].x += 1;
    let third = resolve_schematic_source(SchematicHardcopySource {
        identity: HardcopySourceIdentity {
            document_id: first.authority().document_id(),
            ..identity("schematic")
        },
        schematic: &schematic,
        expected_topology_version: topology,
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap();
    assert_ne!(
        first.authority().content_digest(),
        third.authority().content_digest()
    );
}

fn sheet_definition(name: &str) -> SheetDefinition {
    SheetDefinition {
        name: name.to_owned(),
        template: SheetTemplate::AnalogSchematic,
        port_policy: SheetPortPolicy::TypedOffSheetPorts,
        explicit_page_number: None,
    }
}

#[test]
fn governed_current_sheet_never_leaks_and_all_sheets_preserve_catalog_order() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(11, Point::new(0, 0), Point::new(20, 0)));
    schematic
        .wires
        .push(Wire::segment(22, Point::new(100, 0), Point::new(120, 0)));
    let mut catalog = SheetCatalog::default();
    let first_id = catalog
        .create_sheet(sheet_definition("Input"), None)
        .unwrap();
    let second_id = catalog
        .create_sheet(sheet_definition("Output"), Some(first_id))
        .unwrap();
    let empty_id = catalog
        .create_sheet(sheet_definition("Reserved"), Some(second_id))
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), first_id, [11])
        .unwrap();
    catalog
        .assign_objects(catalog.revision(), second_id, [22])
        .unwrap();

    let mut project_settings = crate::state::DrawingSheetProjectSettings::default();
    project_settings.default_format = crate::state::SchematicSheetFormat::from_standard(
        crate::state::DrawingSheetStandard::IsoA3,
        crate::state::SchematicPageOrientation::Landscape,
    )
    .try_update(|draft| {
        draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
    })
    .unwrap();
    let inherited_format = catalog
        .find(second_id)
        .unwrap()
        .page_format()
        .try_update(|draft| {
            draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
        })
        .unwrap();
    catalog
        .update_sheet_page_format(
            second_id,
            catalog.find(second_id).unwrap().revision(),
            inherited_format,
        )
        .unwrap();
    project_settings.title_block_field_values.insert(
        DrawingSheetTitleFieldId::Organization,
        "RSpice Engineering".to_owned(),
    );
    let base_identity = identity("governed-schematic");
    let second = resolve_schematic_source(SchematicHardcopySource {
        identity: schematic_sheet_identity(&base_identity, catalog.find(second_id).unwrap())
            .unwrap(),
        schematic: &schematic,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: Some(&catalog),
        sheet_id: Some(second_id),
        project_default_drawing_sheet: Some(&project_settings.default_format),
        project_title_block_field_values: Some(&project_settings.title_block_field_values),
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap();
    let HardcopySemanticDocument::Schematic(second_semantic) = second.semantic_document() else {
        panic!("expected schematic")
    };
    assert_eq!(
        second_semantic
            .wires
            .iter()
            .map(|wire| wire.id)
            .collect::<Vec<_>>(),
        [22]
    );
    let expected_inherited_format = project_settings
        .default_format
        .with_target_sheet_title_fields(catalog.find(second_id).unwrap().page_format());
    assert_eq!(
        second_semantic.drawing_sheet.as_ref(),
        Some(&expected_inherited_format)
    );
    assert_eq!(
        second_semantic
            .drawing_sheet_title_values
            .get(&DrawingSheetTitleFieldId::Organization)
            .map(String::as_str),
        Some("RSpice Engineering")
    );

    let all = resolve_all_schematic_sheets(SchematicSheetSetHardcopySource {
        identity: base_identity,
        schematic: &schematic,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: &catalog,
        project_default_drawing_sheet: &project_settings.default_format,
        project_title_block_field_values: &project_settings.title_block_field_values,
    })
    .unwrap();
    let HardcopySemanticDocument::Aggregate(aggregate) = all.semantic_document() else {
        panic!("expected aggregate")
    };
    assert_eq!(aggregate.children.len(), 3);
    assert_eq!(
        aggregate
            .children
            .iter()
            .map(|child| child.source_key.clone())
            .collect::<Vec<_>>(),
        [
            format!("governed-schematic:sheet:{first_id}"),
            format!("governed-schematic:sheet:{second_id}"),
            format!("governed-schematic:sheet:{empty_id}"),
        ]
    );
    for (index, expected_wire) in [Some(11), Some(22), None].into_iter().enumerate() {
        let HardcopySemanticDocument::Schematic(sheet) =
            aggregate.children[index].document.as_ref()
        else {
            panic!("expected schematic child")
        };
        assert_eq!(
            sheet.wires.first().map(|wire| wire.id),
            expected_wire,
            "sheet {index} must contain only its own assigned wire"
        );
        let stored_format = catalog.sheets()[index].page_format();
        let effective_format =
            if stored_format.inheritance == crate::state::DrawingSheetInheritance::ProjectDefault {
                project_settings
                    .default_format
                    .with_target_sheet_title_fields(stored_format)
            } else {
                stored_format.clone()
            };
        assert_eq!(sheet.drawing_sheet.as_ref(), Some(&effective_format));
        assert_eq!(
            sheet
                .drawing_sheet_title_values
                .get(&DrawingSheetTitleFieldId::Page)
                .map(String::as_str),
            Some(match index {
                0 => "1 / 3",
                1 => "2 / 3",
                _ => "3 / 3",
            })
        );
        assert_eq!(
            sheet
                .drawing_sheet_title_values
                .get(&DrawingSheetTitleFieldId::Format)
                .map(String::as_str),
            Some(effective_format.authored_size.label())
        );
        assert_eq!(
            sheet
                .drawing_sheet_title_values
                .get(&DrawingSheetTitleFieldId::Organization)
                .map(String::as_str),
            Some("RSpice Engineering")
        );
        assert_eq!(aggregate.children[index].page_break_before, index != 0);
    }
    assert_eq!(
        aggregate.children[2]
            .local_bounds
            .content_extent()
            .unwrap()
            .width()
            .micrometres(),
        297_000
    );
    assert_eq!(
        aggregate.children[2]
            .local_bounds
            .content_extent()
            .unwrap()
            .height()
            .micrometres(),
        210_000
    );
    assert_eq!(all.hardcopy_sections().unwrap().len(), 3);

    let worker_bytes = all.worker_snapshot_json().unwrap();
    let round_trip = ResolvedHardcopyDocument::from_worker_snapshot_json(&worker_bytes).unwrap();
    assert_eq!(round_trip, all);

    let mut tampered: serde_json::Value = serde_json::from_slice(&worker_bytes).unwrap();
    tampered["source_key"] = serde_json::Value::String("tampered-source".to_owned());
    assert!(matches!(
        ResolvedHardcopyDocument::from_worker_snapshot_json(
            &serde_json::to_vec(&tampered).unwrap()
        ),
        Err(HardcopySourceError::InvalidWorkerSnapshot(_))
    ));
    let mut unknown: serde_json::Value = serde_json::from_slice(&worker_bytes).unwrap();
    unknown["unexpected"] = serde_json::Value::Bool(true);
    assert!(matches!(
        ResolvedHardcopyDocument::from_worker_snapshot_json(&serde_json::to_vec(&unknown).unwrap()),
        Err(HardcopySourceError::InvalidWorkerSnapshot(_))
    ));
}

#[test]
fn source_sets_authenticate_definition_and_every_member_atomically() {
    let first =
        resolve_blank_schematic_sheet(identity("sheet-one"), HardcopyScope::CurrentSheet).unwrap();
    let second =
        resolve_blank_schematic_sheet(identity("sheet-two"), HardcopyScope::CurrentSheet).unwrap();
    let members = [&first, &second]
        .into_iter()
        .map(HardcopySourceSetMember::from_resolved)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Release set",
        HardcopyDocumentKind::SchematicOrSymbol,
        HardcopyScope::NamedPrintSet("Release set".to_owned()),
        members,
    )
    .unwrap();
    source_set.validate().unwrap();

    let stale_second = resolve_blank_schematic_sheet(
        HardcopySourceIdentity {
            display_name: "Changed sheet".to_owned(),
            ..identity("sheet-two")
        },
        HardcopyScope::CurrentSheet,
    )
    .unwrap();
    let mut candidates = vec![first, stale_second].into_iter();
    let error = resolve_hardcopy_source_set_with(&source_set, |_| Ok(candidates.next().unwrap()))
        .unwrap_err();
    assert!(matches!(
        error,
        HardcopySourceError::StaleSourceSetMember { ref source_key }
            if source_key == "sheet-two"
    ));

    let mut tampered = source_set.clone();
    tampered.reverse_members_for_test();
    assert!(matches!(
        tampered.validate(),
        Err(HardcopySourceError::SourceSetDigestMismatch { .. })
    ));
}

#[test]
fn schematic_selection_exports_only_selected_durable_objects() {
    let mut schematic = SchematicState::default();
    schematic
        .wires
        .push(Wire::segment(1, Point::new(0, 0), Point::new(10, 0)));
    schematic
        .wires
        .push(Wire::segment(2, Point::new(100, 0), Point::new(110, 0)));
    schematic.selection.select_wire(2);
    let resolved = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: &schematic,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::Selection,
    })
    .unwrap();
    let HardcopySemanticDocument::Schematic(scene) = resolved.semantic_document() else {
        panic!("expected schematic")
    };
    assert_eq!(
        scene.wires.iter().map(|wire| wire.id).collect::<Vec<_>>(),
        [2]
    );
}

#[test]
fn authored_cell_symbol_is_frozen_into_the_semantic_source() {
    let document = SymbolDocument {
        pins: vec![crate::state::SymbolPin::new(
            "IN",
            PortDirection::In,
            Some(Point::new(-20, 0)),
        )],
        body: vec![SymbolShape::Circle {
            center: Point::origin(),
            radius: 9,
        }],
        ..SymbolDocument::default()
    };
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document.store_in_view(&mut symbol_view).unwrap();
    let mut cell = Cell::new("amp");
    cell.add_view(symbol_view);
    let mut library = Library::new("work");
    library.add_cell(cell);
    let mut libraries = LibraryManager::new();
    libraries.add_library(library);
    let buffers = HashMap::new();
    let resolver = SymbolResolver::new(&libraries, &buffers);

    let mut schematic = SchematicState::default();
    schematic.components.push(
        Component::new(7, ComponentType::CellInstance, Point::new(20, 30))
            .with_library_cell(LibraryCellInstance::new("work", "amp", "symbol")),
    );
    let resolved = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: &schematic,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: Some(&resolver),
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap();
    let HardcopySemanticDocument::Schematic(scene) = resolved.semantic_document() else {
        panic!("expected schematic")
    };
    assert_eq!(
        scene.components[0].resolved_symbol.as_ref(),
        Some(&document)
    );
    assert_eq!(
        scene.components[0].symbol_source,
        Some(SemanticSymbolSource::Authored)
    );
}

#[test]
fn stale_schematic_authority_is_rejected_before_digesting() {
    let schematic = SchematicState::default();
    let error = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: &schematic,
        expected_topology_version: schematic.topology_version() + 1,
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap_err();
    assert!(matches!(error, HardcopySourceError::StaleSchematic { .. }));
}

#[test]
fn selected_probe_is_rejected_explicitly_without_mutating_the_schematic() {
    let mut schematic = SchematicState::default();
    schematic.probes.push(
        crate::state::SchematicProbe::new(
            91,
            Point::new(20, 30),
            "V(out)",
            Some("V(out)".to_owned()),
        )
        .unwrap(),
    );
    schematic.selection.select_only_probe(91);
    let probes_before = schematic.probes.clone();
    let selection_before = schematic.selection.clone();
    let topology_before = schematic.topology_version();
    let dirty_before = schematic.is_dirty;
    let undo_before = schematic.can_undo();

    let error = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: &schematic,
        expected_topology_version: schematic.topology_version(),
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::Selection,
    })
    .unwrap_err();

    assert_eq!(error, HardcopySourceError::ProbeSelectionUnsupported);
    assert_eq!(
        error.to_string(),
        "probe markers are not publishable hardcopy objects; deselect every probe or publish the owning waveform instead"
    );
    assert_eq!(schematic.probes, probes_before);
    assert_eq!(schematic.selection, selection_before);
    assert_eq!(schematic.topology_version(), topology_before);
    assert_eq!(schematic.is_dirty, dirty_before);
    assert_eq!(schematic.can_undo(), undo_before);
}

#[test]
fn symbol_scene_retains_negative_authored_coordinates_and_deterministic_extent() {
    let document = SymbolDocument {
        body: vec![SymbolShape::Polyline {
            points: vec![Point::new(-20, -10), Point::new(30, 10)],
            closed: false,
        }],
        ..SymbolDocument::default()
    };
    let resolved = resolve_symbol_source(SymbolHardcopySource {
        identity: identity("symbol"),
        document: &document,
        selection: None,
        scope: HardcopyScope::ActiveDocument,
    })
    .unwrap();
    assert!(resolved.bounds().minimum.x_um < 0);
    assert_eq!(
        resolved.content_extent().width().micrometres(),
        70 * SCHEMATIC_UNIT_UM as u64
    );
}

#[test]
fn report_source_uses_authenticated_current_revision_snapshot() {
    let mut report = ReportDocument::new("Release report").unwrap();
    report
        .transact_with_context(
            report.revision(),
            vec![ReportEdit::AddPage {
                title: "Summary".to_owned(),
            }],
            10,
            "tester",
            "Add summary",
        )
        .unwrap();
    let record = report
        .revision_record(report.id(), report.revision())
        .unwrap();
    let resolved = resolve_report_source(ReportHardcopySource {
        source_key: "report".to_owned(),
        document: &report,
        reference_inventory: None,
        scope: HardcopyScope::CompleteReport,
    })
    .unwrap();
    assert_eq!(
        resolved.authority().content_digest(),
        record.snapshot_digest()
    );
    assert_eq!(
        resolved.content_extent().height().micrometres(),
        REPORT_PAGE_HEIGHT_UM as u64
    );
}

#[test]
fn linked_report_table_requires_exact_source_and_dataset_inventory() {
    let mut app = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    let dataset_id = run.dataset_id;
    let dataset_digest = run.dataset_content_digest();
    app.simulation.runs.push(run);
    let binding = DatasetBinding::new(dataset_id, dataset_digest);
    let snapshot = ReportReferenceSnapshot::new(
        ReportSourceId::Dataset { dataset_id },
        None,
        dataset_digest,
        vec![binding],
    )
    .unwrap();
    let report = report_with_block(ReportBlockKind::DataTable(DataTableBlock {
        title: "Exact samples".to_owned(),
        columns: vec![TableColumn {
            key: "value".to_owned(),
            heading: "Value".to_owned(),
            unit: Some("V".to_owned()),
        }],
        rows: vec![vec![TableCell::Number {
            value: 1.25,
            unit: Some("V".to_owned()),
        }]],
        reference: ReportReferenceMode::Linked {
            snapshot: snapshot.clone(),
        },
    }));
    assert!(matches!(
        resolve_report_source(ReportHardcopySource {
            source_key: "report".to_owned(),
            document: &report,
            reference_inventory: None,
            scope: HardcopyScope::CompleteReport,
        }),
        Err(HardcopySourceError::ReportReferenceInventoryRequired)
    ));

    // Application resolution derives an exact inventory from retained source
    // owners. It never substitutes a most-recent or background run.
    let report_id = report.id();
    app.workspace.report_documents.push(report.clone());
    app.workbench.report_authoring.selected_document = Some(report_id);
    let app_resolved = report_inventory::resolve(&app, &report, HardcopyScope::CompleteReport)
        .expect("the retained dataset must authenticate the linked report table");
    let HardcopySemanticDocument::Report(app_semantic) = app_resolved.semantic_document() else {
        panic!("expected semantic report")
    };
    assert_eq!(app_semantic.authenticated_references.len(), 1);

    let missing_dataset = ReportReferenceInventory {
        sources: vec![
            ReportReferenceInventoryEntry::new(
                snapshot.source.clone(),
                snapshot.source_revision,
                snapshot.content_digest,
                snapshot.dataset_bindings.clone(),
            )
            .unwrap(),
        ],
        available_datasets: Vec::new(),
        figure_artifacts: Vec::new(),
    };
    assert!(matches!(
        resolve_report_source(ReportHardcopySource {
            source_key: "report".to_owned(),
            document: &report,
            reference_inventory: Some(&missing_dataset),
            scope: HardcopyScope::CompleteReport,
        }),
        Err(HardcopySourceError::UnauthenticatedReportReference {
            currentness: ReportReferenceCurrentness::DatasetMissing,
            ..
        })
    ));

    let exact_inventory = ReportReferenceInventory {
        sources: missing_dataset.sources.clone(),
        available_datasets: vec![binding],
        figure_artifacts: Vec::new(),
    };
    let resolved = resolve_report_source(ReportHardcopySource {
        source_key: "report".to_owned(),
        document: &report,
        reference_inventory: Some(&exact_inventory),
        scope: HardcopyScope::CompleteReport,
    })
    .unwrap();
    let HardcopySemanticDocument::Report(semantic) = resolved.semantic_document() else {
        panic!("expected semantic report")
    };
    assert_eq!(semantic.authenticated_references.len(), 1);
    assert!(matches!(
        semantic.pages[0].sections()[0].blocks()[0].kind(),
        ReportBlockKind::DataTable(_)
    ));
}

#[test]
fn frozen_report_evidence_is_self_contained_and_remains_typed() {
    let digest = ContentDigest::from_bytes([0x44; 32]);
    let snapshot = ReportReferenceSnapshot::new(
        ReportSourceId::VerificationEvidence {
            evidence_id: VerificationEvidenceId::new(),
        },
        Some(ObjectRevision::INITIAL),
        digest,
        Vec::new(),
    )
    .unwrap();
    let report = report_with_block(ReportBlockKind::Evidence(EvidenceBlock {
        title: "Verification receipt".to_owned(),
        summary: "All retained checks passed.".to_owned(),
        reference: ReportReferenceMode::Frozen {
            snapshot,
            artifact: FrozenReportArtifact::new(
                "application/json",
                br#"{"status":"pass"}"#.to_vec(),
            )
            .unwrap(),
        },
    }));
    let resolved = resolve_report_source(ReportHardcopySource {
        source_key: "report".to_owned(),
        document: &report,
        reference_inventory: None,
        scope: HardcopyScope::CompleteReport,
    })
    .unwrap();
    let HardcopySemanticDocument::Report(semantic) = resolved.semantic_document() else {
        panic!("expected semantic report")
    };
    let ReportBlockKind::Evidence(evidence) = semantic.pages[0].sections()[0].blocks()[0].kind()
    else {
        panic!("expected typed evidence")
    };
    assert_eq!(evidence.summary, "All retained checks passed.");
    assert_eq!(semantic.authenticated_references.len(), 1);
}

#[test]
fn retained_linked_report_figure_resolves_identically_in_process_and_worker_snapshot() {
    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x6b; 32]));
    let dataset = SourceDataset::new(
        binding,
        vec![
            SourceColumn::new(
                "time",
                "Time",
                ValueType::Real,
                ColumnRole::Coordinate,
                Some("s".to_owned()),
            )
            .unwrap(),
            SourceColumn::new(
                "out",
                "V(out)",
                ValueType::Real,
                ColumnRole::Signal,
                Some("V".to_owned()),
            )
            .unwrap(),
        ],
        vec![
            SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(-0.25)]),
            SourceRow::new(vec![TypedValue::Real(1.0), TypedValue::Real(0.75)]),
        ],
    )
    .unwrap();
    let mut visualization = VisualizationDocument::new("Retained waveform", vec![dataset]).unwrap();
    let page_id = visualization.pages()[0].id;
    let pane_id = visualization.panes()[0].id;
    let axes = visualization
        .transact(
            visualization.revision(),
            vec![
                DocumentEdit::AddAxis(NewAxis {
                    pane_id,
                    label: "Time".to_owned(),
                    orientation: AxisOrientation::Horizontal,
                    scale: AxisScale::Linear,
                    unit: Some("s".to_owned()),
                    range: None,
                }),
                DocumentEdit::AddAxis(NewAxis {
                    pane_id,
                    label: "Voltage".to_owned(),
                    orientation: AxisOrientation::VerticalLeft,
                    scale: AxisScale::Linear,
                    unit: Some("V".to_owned()),
                    range: None,
                }),
            ],
        )
        .unwrap();
    let x_axis = match axes.created[0] {
        EntityRef::Axis(id) => id,
        _ => unreachable!(),
    };
    let y_axis = match axes.created[1] {
        EntityRef::Axis(id) => id,
        _ => unreachable!(),
    };
    visualization
        .transact(
            visualization.revision(),
            vec![DocumentEdit::AddTrace(NewTrace {
                pane_id,
                binding,
                signal_key: "out".to_owned(),
                coordinate_key: "time".to_owned(),
                x_axis_id: x_axis,
                y_axis_id: y_axis,
                label: "V(out)".to_owned(),
            })],
        )
        .unwrap();
    let snapshot = ReportReferenceSnapshot::new(
        ReportSourceId::VisualizationDocument {
            document_id: visualization.id(),
        },
        Some(visualization.revision()),
        visualization.content_digest().unwrap(),
        vec![binding],
    )
    .unwrap();
    let mut report = report_with_block(ReportBlockKind::PlotFigure(PlotFigureBlock {
        caption: "Retained waveform".to_owned(),
        alternative_text: "Voltage versus time".to_owned(),
        sizing: FigureSizing::FitWidth,
        source_locator: Some(ReportFigureSourceLocator {
            page_id: page_id.get(),
            pane_id: pane_id.get(),
        }),
        reference: ReportReferenceMode::Linked {
            snapshot: snapshot.clone(),
        },
    }));
    let section_id = report.pages()[0].sections()[0].id();
    report
        .transact(
            report.revision(),
            vec![ReportEdit::AddBlock {
                section_id,
                kind: ReportBlockKind::PlotFigure(PlotFigureBlock {
                    caption: "Frozen follow-up".to_owned(),
                    alternative_text: "A frozen figure after the linked figure.".to_owned(),
                    sizing: FigureSizing::Natural,
                    source_locator: None,
                    reference: ReportReferenceMode::Frozen {
                        snapshot,
                        artifact: FrozenReportArtifact::new("image/png", opaque_rgb8_png(128, 128))
                            .unwrap(),
                    },
                }),
            }],
            95,
        )
        .unwrap();

    let mut state = AppState::default();
    let report_id = report.id();
    state.workspace.visualization_documents.push(visualization);
    state.workspace.report_documents.push(report);
    state.workbench.report_authoring.selected_document = Some(report_id);
    let source_key = format!(
        "project:{}:report:{}",
        state.workspace.project.id().as_uuid(),
        report_id
    );

    let synchronous =
        resolve_retained_hardcopy_source(&state, &source_key, HardcopyScope::CompleteReport)
            .unwrap();
    let HardcopySemanticDocument::Report(semantic) = synchronous.semantic_document() else {
        panic!("expected semantic report")
    };
    assert_eq!(semantic.figures.len(), 2);
    assert_eq!(semantic.figures[0].caption, "Retained waveform");
    assert_eq!(semantic.figures[1].caption, "Frozen follow-up");
    assert_eq!(semantic.figures[0].media_type, "image/png");
    assert!(
        semantic.figures[0]
            .payload
            .starts_with(b"\x89PNG\r\n\x1a\n")
    );

    let prepared =
        prepare_retained_hardcopy_resolution(&state, &source_key, HardcopyScope::CompleteReport)
            .unwrap();
    let worker_bytes = prepared.into_worker_snapshot_json().unwrap();
    let restored =
        PreparedRetainedHardcopyResolution::from_worker_snapshot_json(&worker_bytes).unwrap();
    assert_eq!(restored.resolve_owned().unwrap(), synchronous);
}

#[test]
fn frozen_png_figure_is_fully_validated_and_retained_semantically() {
    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x52; 32]));
    let snapshot = ReportReferenceSnapshot::new(
        ReportSourceId::VisualizationDocument {
            document_id: ResultDocumentId::new(),
        },
        Some(ObjectRevision::INITIAL),
        ContentDigest::from_bytes([0x53; 32]),
        vec![binding],
    )
    .unwrap();
    let png = opaque_rgb8_png(128, 128);
    let report = report_with_block(ReportBlockKind::PlotFigure(PlotFigureBlock {
        caption: "Authenticated locus".to_owned(),
        alternative_text: "Exact retained visualization.".to_owned(),
        sizing: FigureSizing::FitWidth,
        source_locator: None,
        reference: ReportReferenceMode::Frozen {
            snapshot: snapshot.clone(),
            artifact: FrozenReportArtifact::new("image/png", png.clone()).unwrap(),
        },
    }));
    let resolved = resolve_report_source(ReportHardcopySource {
        source_key: "report".to_owned(),
        document: &report,
        reference_inventory: None,
        scope: HardcopyScope::CompleteReport,
    })
    .unwrap();
    let HardcopySemanticDocument::Report(semantic) = resolved.semantic_document() else {
        panic!("expected semantic report")
    };
    assert_eq!(semantic.figures.len(), 1);
    assert_eq!(semantic.figures[0].payload, png);
    assert_eq!(
        (
            semantic.figures[0].width_pixels,
            semantic.figures[0].height_pixels
        ),
        (128, 128)
    );
    assert_eq!(semantic.figures[0].caption, "Authenticated locus");

    let linked_locator = ReportFigureSourceLocator {
        page_id: 1,
        pane_id: 1,
    };
    let linked = report_with_block(ReportBlockKind::PlotFigure(PlotFigureBlock {
        caption: "Linked".to_owned(),
        alternative_text: "Identity only".to_owned(),
        sizing: FigureSizing::FitPage,
        source_locator: Some(linked_locator.clone()),
        reference: ReportReferenceMode::Linked {
            snapshot: snapshot.clone(),
        },
    }));
    let linked_block_id = linked.pages()[0].sections()[0].blocks()[0].id();
    let linked_inventory = ReportReferenceInventory {
        sources: vec![
            ReportReferenceInventoryEntry::new(
                snapshot.source.clone(),
                snapshot.source_revision,
                snapshot.content_digest,
                snapshot.dataset_bindings.clone(),
            )
            .unwrap(),
        ],
        available_datasets: snapshot.dataset_bindings.clone(),
        figure_artifacts: vec![
            ReportReferenceFigureArtifact::new(
                linked_block_id,
                snapshot.clone(),
                linked_locator,
                FrozenReportArtifact::new("image/png", png.clone()).unwrap(),
            )
            .unwrap(),
        ],
    };
    let linked_resolved = resolve_report_source(ReportHardcopySource {
        source_key: "linked-report".to_owned(),
        document: &linked,
        reference_inventory: Some(&linked_inventory),
        scope: HardcopyScope::CompleteReport,
    })
    .unwrap();
    let HardcopySemanticDocument::Report(linked_semantic) = linked_resolved.semantic_document()
    else {
        panic!("expected linked semantic report")
    };
    assert_eq!(linked_semantic.figures.len(), 1);
    assert_eq!(linked_semantic.figures[0].payload, png);

    let mut trailing = opaque_rgb8_png(128, 128);
    trailing.extend_from_slice(b"trailing");
    let invalid = report_with_block(ReportBlockKind::PlotFigure(PlotFigureBlock {
        caption: "Invalid".to_owned(),
        alternative_text: "Trailing data".to_owned(),
        sizing: FigureSizing::Natural,
        source_locator: None,
        reference: ReportReferenceMode::Frozen {
            snapshot,
            artifact: FrozenReportArtifact::new("image/png", trailing).unwrap(),
        },
    }));
    assert!(matches!(
        resolve_report_source(ReportHardcopySource {
            source_key: "invalid-report".to_owned(),
            document: &invalid,
            reference_inventory: None,
            scope: HardcopyScope::CompleteReport,
        }),
        Err(HardcopySourceError::InvalidReportSource(reason))
            if reason.contains("after IEND")
    ));
}

// The public pane adapter deliberately invokes the visualization domain's
// exact-scene resolver; it never accepts an unverified pane plus loose
// samples. This fixture therefore exercises document/revision/digest and
// immutable-dataset validation before physical mapping.
#[test]
fn retained_plot_scene_maps_to_platform_independent_integer_geometry() {
    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x5a; 32]));
    let dataset = SourceDataset::new(
        binding,
        vec![
            SourceColumn::new(
                "time",
                "Time",
                ValueType::Real,
                ColumnRole::Coordinate,
                Some("s".to_owned()),
            )
            .unwrap(),
            SourceColumn::new(
                "out",
                "V(out)",
                ValueType::Real,
                ColumnRole::Signal,
                Some("V".to_owned()),
            )
            .unwrap(),
        ],
        vec![
            SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(-1.0)]),
            SourceRow::new(vec![TypedValue::Real(1.0), TypedValue::Real(1.0)]),
        ],
    )
    .unwrap();
    let mut document = VisualizationDocument::new("Waveform", vec![dataset]).unwrap();
    let page_id = document.pages()[0].id;
    let pane_id = document.panes()[0].id;
    let axes = document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddAxis(NewAxis {
                    pane_id,
                    label: "Time".to_owned(),
                    orientation: AxisOrientation::Horizontal,
                    scale: AxisScale::Linear,
                    unit: Some("s".to_owned()),
                    range: None,
                }),
                DocumentEdit::AddAxis(NewAxis {
                    pane_id,
                    label: "V".to_owned(),
                    orientation: AxisOrientation::VerticalLeft,
                    scale: AxisScale::Linear,
                    unit: Some("V".to_owned()),
                    range: None,
                }),
            ],
        )
        .unwrap();
    let x_axis = match axes.created[0] {
        EntityRef::Axis(id) => id,
        _ => unreachable!(),
    };
    let y_axis = match axes.created[1] {
        EntityRef::Axis(id) => id,
        _ => unreachable!(),
    };
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddTrace(NewTrace {
                pane_id,
                binding,
                signal_key: "out".to_owned(),
                coordinate_key: "time".to_owned(),
                x_axis_id: x_axis,
                y_axis_id: y_axis,
                label: "V(out)".to_owned(),
            })],
        )
        .unwrap();
    let digest = document.content_digest().unwrap();
    let reference = ReportReferenceSnapshot::new(
        ReportSourceId::VisualizationDocument {
            document_id: document.id(),
        },
        Some(document.revision()),
        digest,
        vec![binding],
    )
    .unwrap();
    let first = resolve_visualization_pane_source(VisualizationPaneHardcopySource {
        source_key: "plot".to_owned(),
        display_name: "Waveform".to_owned(),
        document: &document,
        reference: &reference,
        page_id,
        pane_id,
        scope: HardcopyScope::ActivePlotDocument,
    })
    .unwrap();
    let second = resolve_visualization_pane_source(VisualizationPaneHardcopySource {
        source_key: "plot".to_owned(),
        display_name: "Waveform".to_owned(),
        document: &document,
        reference: &reference,
        page_id,
        pane_id,
        scope: HardcopyScope::ActivePlotDocument,
    })
    .unwrap();
    assert_eq!(first.semantic_document(), second.semantic_document());
    assert_eq!(
        first.authority().content_digest(),
        second.authority().content_digest()
    );
    assert_ne!(
        first.authority().content_digest(),
        digest,
        "the publication digest also binds the resolved physical semantics"
    );
}

#[test]
fn plot_line_clipping_preserves_true_axis_boundary_intersections() {
    let clipped = clip_line_to_axis_rect((-1.0, 0.25), (3.0, 0.75), 0.0, 2.0, 0.0, 1.0)
        .expect("line crosses the visible axis rectangle");
    assert_eq!(clipped.0, (0.0, 0.375));
    assert_eq!(clipped.1, (2.0, 0.625));
    assert!(clip_line_to_axis_rect((-2.0, 2.0), (-1.0, 3.0), 0.0, 1.0, 0.0, 1.0).is_none());
}

#[test]
fn viewer_partition_covers_every_results_family() {
    let curve_viewers = [
        ResultViewer::Waves,
        ResultViewer::Bode,
        ResultViewer::Fft,
        ResultViewer::Eye,
        ResultViewer::Hist,
        ResultViewer::Nyquist,
        ResultViewer::Smith,
    ];
    let summary_viewers = [
        ResultViewer::Op,
        ResultViewer::NoiseContrib,
        ResultViewer::Contribution,
        ResultViewer::TransferFunction,
        ResultViewer::Specs,
        ResultViewer::PoleZero,
    ];
    assert!(curve_viewers.into_iter().all(is_curve_viewer));
    assert!(
        summary_viewers
            .into_iter()
            .all(|viewer| !is_curve_viewer(viewer))
    );
}

#[test]
fn quick_view_reads_exact_active_retained_waveform_without_report_reference() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(7, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![-0.25, 0.5, 1.25],
                "#00ffff",
            ),
        ]),
    );
    state.simulation.runs.push(run);
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    state.simulation.data_version = 9;
    state.ui.results.viewer = ResultViewer::Waves;

    let resolved = resolve_results_quick_view_source(ResultsQuickViewHardcopySource {
        source_key: "results-quick-view".to_owned(),
        project_id: state.workspace.project.id(),
        state: &state,
        scope: HardcopyScope::ActivePlotDocument,
    })
    .unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected semantic plot")
    };
    assert_eq!(plot.viewer, ResultViewer::Waves);
    assert_eq!(
        plot.traces[0].source_samples,
        vec![
            (0.0f64.to_bits(), (-0.25f64).to_bits()),
            (1.0f64.to_bits(), 0.5f64.to_bits()),
            (2.0f64.to_bits(), 1.25f64.to_bits()),
        ]
    );
    assert!(
        resolved
            .default_print_mapping()
            .entries()
            .iter()
            .any(|entry| {
                entry.object().kind() == PrintObjectKind::Trace
                    && entry.object().stable_id() == format!("trace:{}", plot.traces[0].trace_id)
            })
    );
    assert_eq!(resolved.authority().revision(), ObjectRevision::INITIAL);
}

#[test]
fn fft_quick_view_ignores_stale_cache_and_global_data_version() {
    let time = (0..64)
        .map(|index| index as f64 * 1.0e-6)
        .collect::<Vec<_>>();
    let values = (0..64)
        .map(|index| (index as f64 * std::f64::consts::TAU / 8.0).sin())
        .collect::<Vec<_>>();
    let analysis =
        AnalysisResult::new(7, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("V(active)", time, values, "#00ffff"),
        ]);
    let mut state = quick_view_state(analysis, ResultViewer::Fft);
    state.analysis.fft_state.selected_source = Some("V(active)".to_owned());
    state.analysis.fft_state.data = Some(crate::analysis::FftData::from_spectrum(
        "stale",
        &[9_999.0, 10_000.0],
        &[8_888.0, 7_777.0],
        &[0.0, 0.0],
        20_000.0,
    ));
    state.simulation.data_version = 9;

    let first = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = first.semantic_document() else {
        panic!("expected FFT plot")
    };
    assert_eq!(plot.viewer, ResultViewer::Fft);
    assert!(
        plot.traces[0]
            .source_samples
            .iter()
            .all(|(x, y)| *x != 9_999.0f64.to_bits() && *y != 8_888.0f64.to_bits())
    );

    state.simulation.data_version = 10_000;
    state.analysis.fft_state.data = Some(crate::analysis::FftData::from_spectrum(
        "different stale cache",
        &[123_456.0],
        &[654_321.0],
        &[0.0],
        1.0,
    ));
    let second = resolve_quick_view(&state).unwrap();
    assert_eq!(
        first.authority().document_id(),
        second.authority().document_id()
    );
    assert_eq!(first.authority().revision(), second.authority().revision());
    assert_eq!(
        first.authority().content_digest(),
        second.authority().content_digest()
    );
}

#[test]
fn eye_quick_view_reconstructs_the_interactive_source_contract() {
    let time = (0..161)
        .map(|index| index as f64 * 0.25)
        .collect::<Vec<_>>();
    let ignored = vec![42.0; time.len()];
    let selected = time
        .iter()
        .map(|time| if (*time as i64) % 2 == 0 { -1.0 } else { 1.0 })
        .collect::<Vec<_>>();
    let analysis =
        AnalysisResult::new(8, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new("A(first)", time.clone(), ignored, "#ff00ff"),
            WaveformData::new("V(selected)", time.clone(), selected.clone(), "#00ffff"),
        ]);
    let mut state = quick_view_state(analysis, ResultViewer::Eye);
    state.analysis.fft_state.selected_source = Some("|V(selected)|".to_owned());
    let mut stale_eye = crate::analysis::EyeData::new(99.0, 7);
    stale_eye.add_trace(crate::analysis::EyeTrace::new(
        vec![0.0, 1.0],
        vec![9_999.0, 9_999.0],
    ));
    state.analysis.eye_diagram_state.load_data(stale_eye);

    let period = retained_eye_bit_period(&time, &selected).unwrap();
    let expected = crate::analysis::eye_diagram::EyeDataBuilder::new()
        .bit_period(period)
        .ui_count(2)
        .skip_initial(2)
        .build(&time, &selected);
    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected eye plot")
    };
    assert_eq!(plot.traces.len(), expected.traces.len());
    for (actual, expected) in plot.traces.iter().zip(expected.traces.iter()) {
        assert_eq!(
            actual.source_samples,
            expected
                .time
                .iter()
                .copied()
                .zip(expected.amplitude.iter().copied())
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect::<Vec<_>>()
        );
        assert!(
            actual
                .source_samples
                .iter()
                .all(|(_, y)| *y != 9_999.0f64.to_bits())
        );
    }
}

#[test]
fn histogram_quick_view_derives_only_from_active_monte_carlo_metadata() {
    let samples = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    let analysis = AnalysisResult::new(9, AnalysisType::MonteCarlo, "Monte Carlo")
        .with_family_metadata(AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 17,
            runs_requested: samples.len(),
            runs_completed: samples.len(),
            failures: 0,
            all_converged: true,
            variables: vec![MonteCarloVariableMetadata {
                name: "gain".to_owned(),
                samples: samples.clone(),
                mean: 0.0,
                std_dev: 2.0f64.sqrt(),
                min: -2.0,
                max: 2.0,
            }],
        });
    let mut state = quick_view_state(analysis, ResultViewer::Hist);
    state.analysis.histogram_state.load_histogram(
        crate::analysis::HistogramBuilder::new()
            .name("stale")
            .bin_count(3)
            .build(&[9_999.0; 20]),
    );
    state.analysis.histogram_state.bin_count = 5;

    let resolved = resolve_quick_view(&state).unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected histogram plot")
    };
    assert_eq!(plot.traces[0].label, "gain");
    let retained_count = plot.traces[0]
        .source_samples
        .iter()
        .map(|(_, count)| f64::from_bits(*count))
        .sum::<f64>();
    assert_eq!(retained_count, samples.len() as f64);
    assert!(
        plot.traces[0]
            .source_samples
            .iter()
            .all(|(center, _)| *center != 9_999.0f64.to_bits())
    );
}

#[test]
fn nyquist_and_smith_require_active_retained_complex_samples() {
    let real_only =
        AnalysisResult::new(10, AnalysisType::Ac, "AC").with_waveforms(vec![WaveformData::new(
            "V(out)",
            vec![1.0; 16],
            vec![2.0; 16],
            "#00ffff",
        )]);
    let mut stale = quick_view_state(real_only, ResultViewer::Nyquist);
    stale
        .analysis
        .nyquist_state
        .load_data(crate::analysis::NyquistData::from_arrays(
            "stale",
            &[1.0, 2.0],
            &[9_999.0, 8_888.0],
            &[7_777.0, 6_666.0],
        ));
    assert!(matches!(
        resolve_quick_view(&stale),
        Err(HardcopySourceError::MissingViewerEvidence(
            "visible plot series"
        ))
    ));

    let complex = AnalysisResult::new(10, AnalysisType::Ac, "AC").with_waveforms(vec![
        WaveformData::new(
            "S(1,1)",
            (0..16).map(|index| index as f64).collect::<Vec<_>>(),
            vec![0.0; 16],
            "#00ffff",
        )
        .with_complex_components(
            "S(1,1)",
            (0..16).map(|index| index as f64 / 16.0).collect::<Vec<_>>(),
            (0..16)
                .map(|index| -(index as f64) / 32.0)
                .collect::<Vec<_>>(),
        ),
    ]);
    for viewer in [ResultViewer::Nyquist, ResultViewer::Smith] {
        let state = quick_view_state(complex.clone(), viewer);
        let resolved = resolve_quick_view(&state).unwrap();
        let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
            panic!("expected complex plot")
        };
        assert_eq!(plot.viewer, viewer);
        assert_eq!(
            plot.traces[0].source_samples[1],
            ((1.0f64 / 16.0).to_bits(), (-1.0f64 / 32.0).to_bits())
        );
    }
}

#[test]
fn studio_adapter_reads_retained_dataset_and_places_markers_without_report_reference() {
    let project_id = ProjectId::new();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(12, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![-1.0, 0.5, 1.0],
                "#00ffff",
            ),
        ]),
    );
    let dataset_id = run.dataset_id;
    let mut simulation = SimulationState::default();
    simulation.runs.push(run);
    let mut studio = VisualizationStudioState::default();
    studio.revision = 4;
    studio.panes.push(StudioPane {
        id: 19,
        viewer: ResultViewer::Waves,
        viewer_document_id: "viewer-waves".to_owned(),
        dataset_id,
        analysis_sequence: 12,
        x_link: None,
        cursor_group: None,
        page: "Transient results".to_owned(),
        placement: Default::default(),
    });
    studio.active_pane = Some(19);
    studio.markers.push(StudioMarker {
        id: 23,
        dataset_id,
        analysis_sequence: 12,
        waveform_name: "V(out)".to_owned(),
        sample_index: 1,
        x: 1.0,
        y: 0.5,
        label: "M1".to_owned(),
    });

    let resolved = resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
        source_key: "studio-active-pane".to_owned(),
        project_id,
        studio: &studio,
        simulation: &simulation,
        pane_id: studio.active_pane.unwrap(),
        scope: HardcopyScope::ActivePlotDocument,
    })
    .unwrap();
    let HardcopySemanticDocument::Plot(plot) = resolved.semantic_document() else {
        panic!("expected studio plot")
    };
    assert_eq!(plot.viewer, ResultViewer::Waves);
    assert_eq!(plot.markers[0].source_x_bits, Some(1.0f64.to_bits()));
    assert_eq!(plot.markers[0].source_y_bits, Some(0.5f64.to_bits()));
    assert!(plot.markers[0].position.is_some());
    assert_eq!(
        resolved.authority().revision(),
        ObjectRevision::new(4).unwrap()
    );

    let initial_digest = resolved.authority().content_digest();
    studio.markers[0].label = "M1 changed".to_owned();
    let marker_changed = resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
        source_key: "studio-active-pane".to_owned(),
        project_id,
        studio: &studio,
        simulation: &simulation,
        pane_id: studio.active_pane.unwrap(),
        scope: HardcopyScope::ActivePlotDocument,
    })
    .unwrap();
    assert_ne!(
        initial_digest,
        marker_changed.authority().content_digest(),
        "marker semantics must bind the resolved visualization digest"
    );

    studio.annotations.push(StudioAnnotation {
        id: 24,
        dataset_id,
        analysis_sequence: 12,
        x: 1.5,
        text: "review point".to_owned(),
    });
    let annotation_changed = resolve_active_studio_pane_source(ActiveStudioPaneHardcopySource {
        source_key: "studio-active-pane".to_owned(),
        project_id,
        studio: &studio,
        simulation: &simulation,
        pane_id: studio.active_pane.unwrap(),
        scope: HardcopyScope::ActivePlotDocument,
    })
    .unwrap();
    assert_ne!(
        marker_changed.authority().content_digest(),
        annotation_changed.authority().content_digest(),
        "annotation semantics must bind the resolved visualization digest"
    );
}

#[test]
fn all_visualization_panes_preserve_retained_pane_order() {
    let project_id = ProjectId::new();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.analyses.push(
        AnalysisResult::new(4, AnalysisType::Transient, "Transient").with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0, 0.0],
                "#00ffff",
            ),
        ]),
    );
    let dataset_id = run.dataset_id;
    let mut simulation = SimulationState::default();
    simulation.runs.push(run);
    let mut studio = VisualizationStudioState::default();
    studio.revision = 7;
    studio.panes = vec![
        StudioPane {
            id: 41,
            viewer: ResultViewer::Waves,
            viewer_document_id: "viewer-waves".to_owned(),
            dataset_id,
            analysis_sequence: 4,
            x_link: None,
            cursor_group: None,
            page: "Page B".to_owned(),
            placement: Default::default(),
        },
        StudioPane {
            id: 17,
            viewer: ResultViewer::Waves,
            viewer_document_id: "viewer-waves".to_owned(),
            dataset_id,
            analysis_sequence: 4,
            x_link: None,
            cursor_group: None,
            page: "Page A".to_owned(),
            placement: Default::default(),
        },
    ];
    studio.active_pane = Some(41);

    let resolved = resolve_all_studio_panes(project_id, &studio, &simulation).unwrap();
    let HardcopySemanticDocument::Aggregate(aggregate) = resolved.semantic_document() else {
        panic!("expected aggregate")
    };
    assert_eq!(
        aggregate
            .children
            .iter()
            .map(|child| child.source_key.clone())
            .collect::<Vec<_>>(),
        [
            format!("project:{}:visualization-pane:41", project_id.as_uuid()),
            format!("project:{}:visualization-pane:17", project_id.as_uuid()),
        ]
    );
    assert_eq!(
        aggregate
            .children
            .iter()
            .map(|child| child.display_name.clone())
            .collect::<Vec<_>>(),
        ["Page B · WAVES", "Page A · WAVES"]
    );
}

#[test]
fn typed_pole_zero_summary_preserves_native_payload_and_exact_values() {
    let payload = AnalysisResultPayload::PoleZero {
        poles: vec![ComplexResultValue {
            real: -1.0,
            imaginary: 2.0,
        }],
        zeros: vec![ComplexResultValue {
            real: -3.0,
            imaginary: 0.0,
        }],
        gain: 4.0,
    };
    let analysis =
        AnalysisResult::new(3, AnalysisType::PoleZero, "PZ").with_result_payload(payload.clone());
    let summary = semantic_result_summary(ResultViewer::PoleZero, &analysis).unwrap();
    assert_eq!(summary.viewer, ResultViewer::PoleZero);
    assert_eq!(summary.payload, Some(payload));
    assert_eq!(summary.tables[0].rows.len(), 2);
    assert_eq!(summary.tables[0].rows[0][1], exact_number(-1.0));
    assert_eq!(summary.tables[0].rows[0][2], exact_number(2.0));
}

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
