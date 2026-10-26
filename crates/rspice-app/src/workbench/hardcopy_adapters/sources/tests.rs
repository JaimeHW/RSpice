//! Tests for what a hardcopy source will and will not resolve to.
//!
//! Most cases pin a refusal: oversized input is rejected before parsing, a
//! stale results registry is not read, and the resolver never guesses a
//! background document when no source is active.

use super::*;

use crate::product::{DatasetBinding, DatasetId, ResultDocumentId, VerificationEvidenceId};
use crate::results::report_document::{
    DataTableBlock, EvidenceBlock, PlotFigureBlock, ReportEdit, ReportEntityRef,
    ReportFigureSourceLocator, ReportReferenceFigureArtifact, ReportReferenceInventoryEntry,
    ReportReferenceSnapshot, ReportSourceId, TableCell, TableColumn,
};
use crate::results::visualization_document::{
    AnnotationAnchor, AxisOrientation, AxisScale, ColumnRole, DocumentEdit, EntityRef, NewAxis,
    NewTrace, PlotMarkerKind, PlotMarkerScope, SourceColumn, SourceDataset, SourceRow, TypedValue,
    ValueType, VisualizationDocument,
};
use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate, SimulationRunLifecycle};

fn identity(key: &str) -> HardcopySourceIdentity {
    HardcopySourceIdentity::try_new(
        key,
        HardcopyDocumentId::new(),
        ObjectRevision::INITIAL,
        "Active document",
    )
    .unwrap()
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
    schematic.document_mut_for_test().wires.push(Wire::segment(
        1,
        Point::new(-10, 0),
        Point::new(20, 0),
    ));
    let topology = schematic.topology_version();
    let first = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: schematic.editor_ref().design,
        selection: None,
        expected_topology_version: topology,
        symbol_resolver: None,
        sheet_catalog: None,
        sheet_id: None,
        project_default_drawing_sheet: None,
        project_title_block_field_values: None,
        scope: HardcopyScope::CurrentSheet,
    })
    .unwrap();

    schematic.session.editor.zoom = 7.5;
    schematic.session.editor.pan = (123.0, -44.0);
    let second = resolve_schematic_source(SchematicHardcopySource {
        identity: HardcopySourceIdentity {
            document_id: first.authority().document_id(),
            ..identity("schematic")
        },
        schematic: schematic.editor_ref().design,
        selection: None,
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

    schematic.document_mut_for_test().wires[0].points[1].x += 1;
    let third = resolve_schematic_source(SchematicHardcopySource {
        identity: HardcopySourceIdentity {
            document_id: first.authority().document_id(),
            ..identity("schematic")
        },
        schematic: schematic.editor_ref().design,
        selection: None,
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
fn source_sets_authenticate_definition_and_every_member_atomically() {
    let first =
        resolve_blank_schematic_sheet(identity("sheet-one"), HardcopyScope::CurrentSheet).unwrap();
    let second =
        resolve_blank_schematic_sheet(identity("sheet-two"), HardcopyScope::CurrentSheet).unwrap();
    let members = [&first, &second]
        .into_iter()
        .map(source_set_member_from_resolved)
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

    let mut tampered_wire = serde_json::to_value(&source_set).unwrap();
    tampered_wire["members"].as_array_mut().unwrap().reverse();
    let tampered: HardcopySourceSet = serde_json::from_value(tampered_wire).unwrap();
    assert!(matches!(
        tampered.validate(),
        Err(HardcopySourceError::SourceSetDigestMismatch { .. })
    ));
}

#[test]
fn schematic_selection_exports_only_selected_durable_objects() {
    let mut schematic = SchematicState::default();
    schematic.document_mut_for_test().wires.push(Wire::segment(
        1,
        Point::new(0, 0),
        Point::new(10, 0),
    ));
    schematic.document_mut_for_test().wires.push(Wire::segment(
        2,
        Point::new(100, 0),
        Point::new(110, 0),
    ));
    let mut whole_wire = Selection::default();
    whole_wire.select_wire(2);
    let mut segment = Selection::default();
    segment.select_wire_segment(2, 0);
    let mut vertex = Selection::default();
    vertex.select_wire_vertex(2, 1);
    for selection in [whole_wire, segment, vertex] {
        schematic.session.editor.selection = selection;
        let resolved = resolve_schematic_source(SchematicHardcopySource {
            identity: identity("schematic"),
            schematic: schematic.editor_ref().design,
            selection: SchematicHardcopySelection::capture(
                &schematic.session.editor.selection,
                &HardcopyScope::Selection,
            ),
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
}

#[test]
fn selected_probe_is_rejected_explicitly_without_mutating_the_schematic() {
    let mut schematic = SchematicState::default();
    schematic.document_mut_for_test().probes.push(
        crate::state::SchematicProbe::new(
            91,
            Point::new(20, 30),
            "V(out)",
            Some("V(out)".to_owned()),
        )
        .unwrap(),
    );
    schematic.session.editor.selection.select_only_probe(91);
    let probes_before = schematic.document().probes.clone();
    let selection_before = schematic.session.editor.selection.clone();
    let topology_before = schematic.topology_version();
    let dirty_before = schematic.session.is_dirty;
    let undo_before = schematic.can_undo();

    let error = resolve_schematic_source(SchematicHardcopySource {
        identity: identity("schematic"),
        schematic: schematic.editor_ref().design,
        selection: SchematicHardcopySelection::capture(
            &schematic.session.editor.selection,
            &HardcopyScope::Selection,
        ),
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
    assert_eq!(schematic.document().probes, probes_before);
    assert_eq!(schematic.session.editor.selection, selection_before);
    assert_eq!(schematic.topology_version(), topology_before);
    assert_eq!(schematic.session.is_dirty, dirty_before);
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
    let resolved =
        resolve_symbol_document(identity("symbol"), document, HardcopyScope::ActiveDocument)
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
    app.model.simulation.retained.runs.push(run);
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
    app.model
        .workspace
        .content
        .report_documents
        .push(report.clone());
    app.hardcopy.report_authoring.selected_document = Some(report_id);
    let app_resolved = report_inventory::resolve(
        SourceReads::of(&app),
        &report,
        HardcopyScope::CompleteReport,
    )
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
                row_predicates: Vec::new(),
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
    let project = &mut state.model.workspace.content;
    project.visualization_documents.push(visualization);
    project.report_documents.push(report);
    state.hardcopy.report_authoring.selected_document = Some(report_id);
    let project_id = project.project.id().as_uuid();
    let source_key = format!("project:{project_id}:report:{report_id}");

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

    let prepared = prepare_retained_hardcopy_resolution(
        SourceReads::of(&state),
        &source_key,
        HardcopyScope::CompleteReport,
    )
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
    let trace_receipt = document
        .transact(
            document.revision(),
            vec![DocumentEdit::AddTrace(NewTrace {
                pane_id,
                binding,
                signal_key: "out".to_owned(),
                coordinate_key: "time".to_owned(),
                row_predicates: Vec::new(),
                x_axis_id: x_axis,
                y_axis_id: y_axis,
                label: "V(out)".to_owned(),
            })],
        )
        .unwrap();
    let trace_id = trace_receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            EntityRef::Trace(id) => Some(*id),
            _ => None,
        })
        .unwrap();
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddTypedMarker {
                    pane_id,
                    trace_id,
                    coordinate: TypedValue::Real(1.0),
                    label: "Peak".to_owned(),
                    kind: PlotMarkerKind::Peak,
                    scope: PlotMarkerScope::Pane,
                    source_specification: None,
                },
                DocumentEdit::AddAnnotation {
                    pane_id,
                    anchor: AnnotationAnchor::Trace {
                        trace_id,
                        coordinate: TypedValue::Real(0.0),
                    },
                    text: "Settling begins".to_owned(),
                },
                DocumentEdit::AddCursor {
                    pane_id,
                    axis_id: x_axis,
                    position: TypedValue::Real(0.5),
                    label: "A".to_owned(),
                },
            ],
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
    let HardcopySemanticDocument::Plot(plot) = first.semantic_document() else {
        panic!("expected semantic plot")
    };
    assert_eq!(plot.markers.len(), 1);
    assert_eq!(plot.annotations.len(), 1);
    assert_eq!(plot.cursors.len(), 1);
    assert_eq!(plot.cursors[0].label, "A");
    assert_eq!(plot.cursors[0].source_x_bits, 0.5f64.to_bits());
    assert_ne!(plot.cursors[0].start, plot.cursors[0].end);
    assert!(first.default_print_mapping().entries().iter().any(|entry| {
        entry.object().kind() == PrintObjectKind::Marker
            && entry.object().stable_id().starts_with("cursor:")
    }));
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

mod app_resolution;

#[test]
fn linear_axes_are_captioned_on_both_dimensions_and_keep_offset_anchors() {
    let frame = PlotFrame {
        x_minimum: 1e9,
        x_maximum: 1e9 + 0.001,
        y_minimum: 0.0,
        y_maximum: 1.0,
        x_span: (1e9 + 0.001) - 1e9,
        y_span: 1.0,
        plot_width: PLOT_WIDTH_UM - 2 * PLOT_INSET_UM,
        plot_height: PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM,
    };
    let (ticks, captions) = plot_axes(AxisScale::Linear, AxisScale::Linear, &frame).unwrap();
    let screen = crate::ui::plot::Axis::linear(frame.x_minimum, frame.x_maximum, "");
    let anchor = screen
        .offset_anchor()
        .expect("this span requires an offset anchor");
    assert!(
        captions
            .iter()
            .any(|caption| caption.text == format!("x: {anchor}"))
    );
    for kind in [SemanticAxisKind::Horizontal, SemanticAxisKind::Vertical] {
        let axis: Vec<_> = ticks.iter().filter(|tick| tick.axis == kind).collect();
        assert!(!axis.is_empty());
        assert!(axis.iter().all(|tick| tick.major && !tick.label.is_empty()));
        for tick in axis {
            match kind {
                SemanticAxisKind::Horizontal => assert_eq!(tick.start.x_um, tick.end.x_um),
                SemanticAxisKind::Vertical => assert_eq!(tick.start.y_um, tick.end.y_um),
            }
        }
    }
}
