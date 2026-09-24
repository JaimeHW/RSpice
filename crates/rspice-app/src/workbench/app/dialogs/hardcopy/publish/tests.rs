//! Tests for the publish dialog's output contract.
//!
//! The cases pin that the dialog never misrepresents a print target, and that
//! the mapping stage stays transactional until publication actually commits.

use super::super::{GovernedSheetPageAuthority, SchematicPageSetupAuthority};
use super::*;

#[test]
fn governed_design_print_selects_the_active_sheet_identity() {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.workspace.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(crate::workbench::state::WorkspaceDocumentId::CellView(
            reference,
        ));
    let key = app.state.workspace.active_key();
    let sheet_id = app
        .state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();

    let (source_key, scope) = active_retained_source_selection(&app).unwrap();

    assert!(source_key.ends_with(&format!(":sheet:{sheet_id}")));
    assert_eq!(scope, crate::hardcopy::HardcopyScope::CurrentSheet);
}

#[test]
fn results_export_selects_the_active_project_document_pane_authority() {
    use crate::product::{AnalysisInstanceId, ContentDigest, DatasetBinding, DatasetId};
    use crate::results::visualization_document::{
        ColumnRole, DocumentEdit, PaneDataBinding, SourceColumn, SourceDataset, SourceRow,
        TypedValue, ValueType, VisualizationDocument,
    };
    use crate::workbench::state::{Workspace, WorkspaceDocumentId};

    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x6e; 32]));
    let source = SourceDataset::new(
        binding,
        vec![
            SourceColumn::new("x", "X", ValueType::Real, ColumnRole::Coordinate, None).unwrap(),
            SourceColumn::new("y", "Y", ValueType::Real, ColumnRole::Signal, None).unwrap(),
        ],
        vec![SourceRow::new(vec![
            TypedValue::Real(0.0),
            TypedValue::Real(1.0),
        ])],
    )
    .unwrap();
    let mut document = VisualizationDocument::new("Results export", vec![source]).unwrap();
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneSource {
                pane_id,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(PaneDataBinding {
                    analysis_id: AnalysisInstanceId::new(),
                    dataset: binding,
                }),
            }],
        )
        .unwrap();
    let document_id = document.id();
    let mut app = RSpiceApp::test_instance();
    app.state.workspace.visualization_documents.push(document);
    app.state.workbench.activate(Workspace::Results);
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::VisualizationDocument(document_id));
    app.state.workbench.visualization_studio.active_pane = Some(pane_id.get());

    let (source_key, scope) = active_retained_source_selection(&app).unwrap();

    assert!(source_key.contains(&format!(
        ":result-document:{document_id}:pane:{}",
        pane_id.get()
    )));
    assert_eq!(scope, crate::hardcopy::HardcopyScope::ActivePlotDocument);
}

#[test]
fn authored_sheet_formats_seed_output_media_without_reverse_coercion() {
    use crate::hardcopy::{
        HardcopySetup, Orientation, PaperSize, PhysicalPageSetup, StandardPaper,
    };
    use crate::state::{SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat};

    for size in [
        SchematicPageSize::A4,
        SchematicPageSize::A3,
        SchematicPageSize::UsLetter,
        SchematicPageSize::UsLedger,
    ] {
        for orientation in [
            SchematicPageOrientation::Portrait,
            SchematicPageOrientation::Landscape,
        ] {
            let expected = SchematicSheetFormat::standard(size, orientation);
            let setup = setup_seeded_from_sheet_format(expected).unwrap();
            assert_eq!(
                setup.physical_page().orientation(),
                match orientation {
                    SchematicPageOrientation::Portrait => Orientation::Portrait,
                    SchematicPageOrientation::Landscape => Orientation::Landscape,
                }
            );
        }
    }

    let custom = SchematicSheetFormat::try_custom(
        "Review board",
        304_800,
        457_200,
        SchematicPageOrientation::Landscape,
    )
    .unwrap();
    let setup = setup_seeded_from_sheet_format(custom).unwrap();
    let PaperSize::Custom(paper) = setup.physical_page().paper() else {
        panic!("a custom authored sheet seeds custom output media");
    };
    assert_eq!(
        paper.dimensions(),
        (
            crate::hardcopy::Length::from_micrometres(304_800),
            crate::hardcopy::Length::from_micrometres(457_200),
        )
    );

    let default = HardcopySetup::default();
    let automatic = HardcopySetup::try_new(
        PhysicalPageSetup::try_new(
            PaperSize::Standard(StandardPaper::A4),
            default.physical_page().margins(),
            default.physical_page().bleed(),
            Orientation::AutomaticPerPage,
        )
        .unwrap(),
        default.scale(),
        default.tiling(),
        default.render().clone(),
        default.decorations().clone(),
        default.print_mapping().clone(),
    )
    .unwrap();
    assert_eq!(
        automatic.physical_page().orientation(),
        Orientation::AutomaticPerPage,
        "automatic orientation remains an output-media choice and is never converted into authored-sheet state"
    );
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn governed_output_page_setup_preserves_authored_sheet_and_saves_hardcopy() {
    use crate::state::{Point, SchematicPageOrientation, SchematicSheetFormat, Wire};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let key = app.state.workspace.active_key();
    let sheet_id = app
        .state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [71])
        .unwrap();
    let catalog = app
        .state
        .workspace
        .design_management
        .sheet_catalog(&key)
        .unwrap();
    let authored_before = catalog.find(sheet_id).unwrap().page_format().clone();
    let authority = SchematicPageSetupAuthority {
        edit: crate::workbench::app::SchematicEditAuthority::capture(&app.state),
        governed_sheet: Some(GovernedSheetPageAuthority {
            cell_view_key: key.clone(),
            catalog_revision: catalog.revision(),
            sheet_id,
            sheet_revision: catalog.find(sheet_id).unwrap().revision(),
        }),
    };
    let resolved =
        crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(
            &app.state,
        )
        .expect("active governed sheet resolves");
    let format = SchematicSheetFormat::try_custom(
        "Review board",
        304_800,
        457_200,
        SchematicPageOrientation::Landscape,
    )
    .unwrap();
    let setup = setup_seeded_from_sheet_format(format.clone()).unwrap();
    let pending = PendingPageSetup {
        opened_source: std::sync::Arc::new(resolved.clone()),
        setup: setup.clone(),
        staged_mapping: StagedPrintMappingPersistence::Document,
        schematic_authority: Some(authority),
    };

    commit_authenticated_page_setup(&mut app, resolved.clone(), pending).unwrap();

    assert_eq!(
        app.state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .unwrap()
            .find(sheet_id)
            .unwrap()
            .page_format(),
        &authored_before
    );
    assert!(
        !app.state.can_undo_project_design(),
        "output-media setup must not create an authored-sheet transaction"
    );
    assert_eq!(
        app.state
            .workspace
            .hardcopy_setups
            .setup_for(resolved.authority())
            .unwrap()
            .unwrap()
            .setup(),
        &setup
    );
    assert!(app.state.workspace.hardcopy_setups_dirty);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn legacy_output_page_setup_does_not_rewrite_document_policy() {
    use crate::state::{Point, SchematicPageOrientation, SchematicSheetFormat, Wire};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved =
        crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(
            &app.state,
        )
        .expect("legacy schematic resolves");
    let policy_before = app.state.schematic.document_policy;
    let format = SchematicSheetFormat::try_custom(
        "Custom",
        300_123,
        450_987,
        SchematicPageOrientation::Portrait,
    )
    .unwrap();
    let pending = PendingPageSetup {
        opened_source: std::sync::Arc::new(resolved.clone()),
        setup: setup_seeded_from_sheet_format(format).unwrap(),
        staged_mapping: StagedPrintMappingPersistence::Document,
        schematic_authority: Some(SchematicPageSetupAuthority {
            edit: crate::workbench::app::SchematicEditAuthority::capture(&app.state),
            governed_sheet: None,
        }),
    };

    commit_authenticated_page_setup(&mut app, resolved, pending).unwrap();

    assert_eq!(
        app.state.schematic.document_policy, policy_before,
        "hardcopy output media must not mutate the authored schematic policy"
    );
    assert_eq!(app.state.schematic.undo_history.undo_description(), None);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn page_setup_rejects_late_read_only_authority_without_partial_commit() {
    use crate::state::{
        Point, SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat, Wire,
    };
    use crate::workbench::state::{LocalSafeModeOptions, WorkspaceDocumentId};

    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved =
        crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(
            &app.state,
        )
        .expect("legacy schematic resolves");
    let authority = SchematicPageSetupAuthority {
        edit: crate::workbench::app::SchematicEditAuthority::capture(&app.state),
        governed_sheet: None,
    };
    app.state.workbench.safe_mode.activate(
        LocalSafeModeOptions {
            open_project_read_only: true,
            ..LocalSafeModeOptions::default()
        },
        String::new(),
    );
    let pending = PendingPageSetup {
        opened_source: std::sync::Arc::new(resolved.clone()),
        setup: setup_seeded_from_sheet_format(SchematicSheetFormat::standard(
            SchematicPageSize::A3,
            SchematicPageOrientation::Portrait,
        ))
        .unwrap(),
        staged_mapping: StagedPrintMappingPersistence::Document,
        schematic_authority: Some(authority),
    };

    assert!(
        commit_authenticated_page_setup(&mut app, resolved.clone(), pending)
            .expect_err("late read-only activation revokes Page Setup")
            .contains("read-only")
    );
    assert!(
        app.state
            .workspace
            .hardcopy_setups
            .setup_for(resolved.authority())
            .unwrap()
            .is_none()
    );
    assert_eq!(
        app.state.schematic.document_policy.page_size,
        SchematicPageSize::A4
    );
}

#[test]
fn governed_page_authority_rejects_catalog_or_active_sheet_drift() {
    use crate::state::{SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat};

    let mut app = RSpiceApp::test_instance();
    let key = app.state.workspace.active_key();
    let sheet_id = app
        .state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [])
        .unwrap();
    let catalog = app
        .state
        .workspace
        .design_management
        .sheet_catalog(&key)
        .unwrap();
    let authority = SchematicPageSetupAuthority {
        edit: crate::workbench::app::SchematicEditAuthority::capture(&app.state),
        governed_sheet: Some(GovernedSheetPageAuthority {
            cell_view_key: key.clone(),
            catalog_revision: catalog.revision(),
            sheet_id,
            sheet_revision: catalog.find(sheet_id).unwrap().revision(),
        }),
    };
    let catalog = app
        .state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
        .unwrap();
    let sheet_revision = catalog.find(sheet_id).unwrap().revision();
    let changed_format =
        SchematicSheetFormat::standard(SchematicPageSize::A3, SchematicPageOrientation::Portrait)
            .try_update(|draft| {
                draft
                    .title_block
                    .fields
                    .get_mut(&crate::state::DrawingSheetTitleFieldId::SheetTitle)
                    .expect("canonical title field")
                    .value = "Input".to_owned();
            })
            .unwrap();
    catalog
        .update_sheet_page_format(sheet_id, sheet_revision, changed_format)
        .unwrap();

    assert!(
        validate_schematic_page_authority(&app.state, &authority)
            .expect_err("catalog drift revokes authority")
            .contains("governed active sheet changed")
    );
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn publication_reuses_only_the_exact_sealed_preview_plan() {
    use crate::state::{Point, Wire};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved =
        crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(
            &app.state,
        )
        .expect("active schematic resolves");
    app.state
        .dialogs
        .hardcopy
        .open_resolved(HardcopyWorkflow::Export, resolved.clone(), None)
        .expect("dialog opens");

    let preview = app
        .state
        .dialogs
        .hardcopy
        .preview_plan
        .as_ref()
        .expect("sealed preview")
        .clone();
    let publication = current_plan(&app, &resolved).expect("unchanged plan");
    assert!(std::sync::Arc::ptr_eq(&preview, &publication));

    app.state.dialogs.hardcopy.margin_left = "0.5".to_owned();
    assert!(
        current_plan(&app, &resolved)
            .expect_err("unpreviewed settings must fail closed")
            .contains("sealed preview plan")
    );
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn app_state_clone_drops_runtime_hardcopy_authority_and_payloads() {
    use crate::hardcopy::{HardcopyOutcome, PrinterRasterGeometry};
    use crate::state::{Point, Wire};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved =
        crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(
            &app.state,
        )
        .expect("active schematic resolves");
    app.state
        .dialogs
        .hardcopy
        .open_resolved(HardcopyWorkflow::Export, resolved.clone(), None)
        .expect("dialog opens");
    let plan = app
        .state
        .dialogs
        .hardcopy
        .preview_plan
        .as_ref()
        .expect("sealed preview")
        .clone();
    let metadata = HardcopySceneMetadata::try_new(resolved.authority().display_name(), "RSpice")
        .expect("metadata");
    app.state.dialogs.hardcopy.preview = Some(std::sync::Arc::new(
        HardcopyRenderer::render_preview_page_resolved(&plan, &resolved, metadata, 0, 72)
            .expect("preview"),
    ));
    app.state.dialogs.hardcopy.source_resolution_generation = 19;
    app.state.dialogs.hardcopy.printer_report =
        Some(crate::workbench::hardcopy_adapters::print::PrinterDiscoveryReport::default());
    app.state.dialogs.hardcopy.printer_job = Some(
        PrinterJobSettings::try_new(
            ContentDigest::from_bytes([0x50; 32]),
            "1",
            PrinterRasterGeometry::try_new(1_000, 800, 0, 0, 1_000, 800).unwrap(),
            PrinterMediaSource::AutomaticCompatibleTray,
            600,
            DuplexMode::Off,
            1,
            false,
        )
        .unwrap(),
    );
    app.state.dialogs.hardcopy.last_receipt = Some(
        HardcopyReceipt::record(
            &plan,
            HardcopyOutcome::Failed {
                code: HardcopyFailureCode::InternalFailure,
                message: "test failure".to_owned(),
                pages_completed: 0,
                retryable: false,
            },
        )
        .expect("receipt"),
    );
    app.state.dialogs.hardcopy.busy = true;
    app.state.dialogs.hardcopy.error = Some("test error".to_owned());

    let cloned = app.state.clone();
    let hardcopy = &cloned.dialogs.hardcopy;
    assert!(!hardcopy.open);
    assert!(hardcopy.source.is_none());
    assert!(hardcopy.resolved_document.is_none());
    assert!(hardcopy.preview_plan.is_none());
    assert!(hardcopy.preview.is_none());
    assert!(hardcopy.preview_adjacent.is_none());
    assert_eq!(hardcopy.source_resolution_generation, 0);
    assert!(hardcopy.printer_report.is_none());
    assert!(hardcopy.printer_job.is_none());
    assert!(hardcopy.last_receipt.is_none());
    assert!(!hardcopy.busy);
    assert!(hardcopy.error.is_none());
}

#[test]
fn artifact_names_are_portable_and_nonempty() {
    assert_eq!(safe_filename("top / schematic"), "top-schematic");
    assert_eq!(safe_filename("***"), "rspice-hardcopy");
    assert_eq!(safe_filename("afe--out"), "afe-out");
}

#[test]
fn file_contract_does_not_misrepresent_print_targets() {
    assert_eq!(format_file_contract(OutputFormat::PdfA).unwrap().0, "pdf");
    assert!(format_file_contract(OutputFormat::NativePrinter).is_err());
    assert!(format_file_contract(OutputFormat::BrowserPrintDocument).is_err());
}

#[test]
fn project_print_mapping_stage_is_transactional_until_publication_commit() {
    use crate::hardcopy::{PrintMappingSaveScope, PrintMappingTable};

    let mut app = RSpiceApp::test_instance();
    let before = app.state.workspace.project_print_mappings.clone();
    let mapping = PrintMappingTable::try_new(
        PrintMappingSaveScope::ProjectPrintSet("release-proof".to_owned()),
        Vec::new(),
    )
    .unwrap();

    let staged = stage_print_mapping_persistence(&app, &mapping).unwrap();
    assert_eq!(app.state.workspace.project_print_mappings, before);
    assert!(!app.state.workspace.project_print_mappings_dirty);
    drop(staged);
    assert_eq!(app.state.workspace.project_print_mappings, before);
    assert!(!app.state.workspace.project_print_mappings_dirty);

    let staged = stage_print_mapping_persistence(&app, &mapping).unwrap();
    commit_print_mapping_persistence(&mut app, staged).unwrap();
    assert_eq!(
        app.state
            .workspace
            .project_print_mappings
            .get("release-proof")
            .unwrap(),
        &mapping
    );
    assert!(app.state.workspace.project_print_mappings_dirty);
}

#[test]
fn identity_bands_are_exactly_one_line_each() {
    let (header, provenance) =
        identity_lines("P", "7", "top / schematic", "source:key", "3", "0123");
    assert!(!header.contains('\n'));
    assert!(!provenance.contains('\n'));
    assert!(header.contains("top / schematic"));
    assert!(provenance.contains("source:key"));
}

#[test]
fn selected_export_name_receives_the_contract_extension() {
    let mut path = std::path::PathBuf::from("review-output");
    crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "pdf");
    assert_eq!(path, std::path::PathBuf::from("review-output.pdf"));
}

#[test]
fn default_printer_resolution_prefers_600_then_a_bounded_highest_mode() {
    assert_eq!(preferred_resolution(vec![300, 600, 1_200]), Some(600));
    assert_eq!(
        preferred_resolution(vec![300, 720, 1_200, 2_400]),
        Some(1_200)
    );
    assert_eq!(preferred_resolution(vec![2_400, 4_800]), Some(2_400));
    assert_eq!(preferred_resolution(Vec::new()), None);
}
