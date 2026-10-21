//! Tests for the publish dialog's output contract.
//!
//! The cases pin that the dialog never misrepresents a print target, and that
//! the mapping stage stays transactional until publication actually commits.

use super::super::{GovernedSheetPageAuthority, SchematicPageSetupAuthority};
use super::*;

#[test]
fn governed_design_print_selects_the_active_sheet_identity() {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(crate::workbench::state::WorkspaceDocumentId::CellView(
            reference,
        ));
    let key = app.state.workspace.content.active_key();
    let sheet_id = app
        .state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();

    let (source_key, scope) = active_retained_source_selection(&app).unwrap();

    assert!(source_key.ends_with(&format!(":sheet:{sheet_id}")));
    assert_eq!(scope, rspice_hardcopy_contract::HardcopyScope::CurrentSheet);
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
    app.state
        .workspace
        .content
        .visualization_documents
        .push(document);
    app.state.workbench.activate(Workspace::Results);
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::VisualizationDocument(document_id));
    app.state
        .ui
        .results
        .session
        .select_persistent_document_pane(document_id, pane_id);

    let (source_key, scope) = active_retained_source_selection(&app).unwrap();

    assert!(source_key.contains(&format!(
        ":result-document:{document_id}:pane:{}",
        pane_id.get()
    )));
    assert_eq!(
        scope,
        rspice_hardcopy_contract::HardcopyScope::ActivePlotDocument
    );
}

/// Bleed is the print setup's alone. A sheet saved with an export bleed of
/// its own starts its first print setup with it where the page's margins
/// hold it; otherwise, and for every other sheet, the default bleed stands.
#[test]
fn a_sheet_saved_with_a_bleed_seeds_its_first_print_setups_bleed() {
    use crate::state::SchematicSheetFormat;
    use rspice_hardcopy_contract::{Bleed, HardcopySetup, Length};

    let saved_with = |bleed_um: u64| -> SchematicSheetFormat {
        let mut saved = serde_json::to_value(SchematicSheetFormat::default()).unwrap();
        saved["bleed_um"] = serde_json::json!(bleed_um);
        serde_json::from_value(saved).unwrap()
    };
    let seeded_bleed = |format| {
        setup_seeded_from_sheet_format(format)
            .unwrap()
            .physical_page()
            .bleed()
    };
    let default_bleed = HardcopySetup::default().physical_page().bleed();
    assert_eq!(
        seeded_bleed(saved_with(3_175)),
        Bleed::Uniform(Length::from_micrometres(3_175))
    );
    assert_eq!(seeded_bleed(SchematicSheetFormat::default()), default_bleed);
    assert_eq!(seeded_bleed(saved_with(500_000)), default_bleed);
}

#[test]
fn authored_sheet_formats_seed_output_media_without_reverse_coercion() {
    use crate::state::{SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat};
    use rspice_hardcopy_contract::{
        HardcopySetup, Orientation, PaperSize, PhysicalPageSetup, StandardPaper,
    };

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
    // Output on paper the size of the sheet is already trimmed, so it starts
    // without trimming marks; everything else the sheet draws stays on.
    let schematic = setup.schematic();
    assert!(!schematic.crop_marks());
    assert!(
        schematic.includes_paper()
            && schematic.includes_border()
            && schematic.includes_title_block()
            && schematic.includes_zones()
    );
    let PaperSize::Custom(paper) = setup.physical_page().paper() else {
        panic!("a custom authored sheet seeds custom output media");
    };
    assert_eq!(
        paper.dimensions(),
        (
            rspice_hardcopy_contract::Length::from_micrometres(304_800),
            rspice_hardcopy_contract::Length::from_micrometres(457_200),
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
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let key = app.state.workspace.content.active_key();
    let sheet_id = app
        .state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [71])
        .unwrap();
    let catalog = app
        .state
        .workspace
        .content
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
            .content
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
            .content
            .hardcopy_setups
            .setup_for(resolved.authority())
            .unwrap()
            .unwrap()
            .setup(),
        &setup
    );
    assert!(app.state.workspace.content.hardcopy_setups_dirty);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn legacy_output_page_setup_does_not_rewrite_document_policy() {
    use crate::state::{Point, SchematicPageOrientation, SchematicSheetFormat, Wire};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved =
        crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(
            &app.state,
        )
        .expect("legacy schematic resolves");
    let policy_before = app.state.schematic.document().document_policy;
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
        app.state.schematic.document().document_policy,
        policy_before,
        "hardcopy output media must not mutate the authored schematic policy"
    );
    assert_eq!(app.state.schematic.history().undo_description(), None);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn page_setup_rejects_late_read_only_authority_without_partial_commit() {
    use crate::state::write_policy::LocalSafeModeOptions;
    use crate::state::{
        Point, SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat, Wire,
    };
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.content.active_view.clone();
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
            .content
            .hardcopy_setups
            .setup_for(resolved.authority())
            .unwrap()
            .is_none()
    );
    assert_eq!(
        app.state.schematic.document().document_policy.page_size,
        SchematicPageSize::A4
    );
}

#[test]
fn governed_page_authority_rejects_catalog_or_active_sheet_drift() {
    use crate::state::{SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat};

    let mut app = RSpiceApp::test_instance();
    let key = app.state.workspace.content.active_key();
    let sheet_id = app
        .state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [])
        .unwrap();
    let catalog = app
        .state
        .workspace
        .content
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
        .content
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
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.content.active_view.clone();
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
    use crate::state::{Point, Wire};
    use crate::workbench::state::WorkspaceDocumentId;
    use rspice_hardcopy_contract::{HardcopyOutcome, PrinterRasterGeometry};

    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.content.active_view.clone();
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
        HardcopyRenderer::render_preview_pages_resolved(
            &plan,
            &resolved,
            metadata,
            &[0],
            72,
            || false,
        )
        .expect("preview")
        .pop()
        .expect("one requested preview page"),
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
    use rspice_hardcopy_contract::{PrintMappingSaveScope, PrintMappingTable};

    let mut app = RSpiceApp::test_instance();
    let before = app.state.workspace.content.project_print_mappings.clone();
    let mapping = PrintMappingTable::try_new(
        PrintMappingSaveScope::ProjectPrintSet("release-proof".to_owned()),
        Vec::new(),
    )
    .unwrap();

    let staged = stage_print_mapping_persistence(&app, &mapping).unwrap();
    assert_eq!(app.state.workspace.content.project_print_mappings, before);
    assert!(!app.state.workspace.content.project_print_mappings_dirty);
    drop(staged);
    assert_eq!(app.state.workspace.content.project_print_mappings, before);
    assert!(!app.state.workspace.content.project_print_mappings_dirty);

    let staged = stage_print_mapping_persistence(&app, &mapping).unwrap();
    commit_print_mapping_persistence(&mut app, staged).unwrap();
    assert_eq!(
        app.state
            .workspace
            .content
            .project_print_mappings
            .get("release-proof")
            .unwrap(),
        &mapping
    );
    assert!(app.state.workspace.content.project_print_mappings_dirty);
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

#[cfg(not(target_arch = "wasm32"))]
const PEER_DECK: &str = "* owned deck\n* a peer's edit\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
#[cfg(not(target_arch = "wasm32"))]
const EDITED_DECK: &str = "* owned deck\nV1 out 0 2\nR1 out 0 1k\n.op\n.end\n";

/// Export open on the active schematic of a project that owns a top deck,
/// with a sealed preview plan and no publication started.
#[cfg(not(target_arch = "wasm32"))]
fn export_open_on_a_netlist_owned_project() -> RSpiceApp {
    use crate::state::{Point, Wire};
    use crate::workbench::documents::CodeSourceFileAction;
    use crate::workbench::documents::netlist_document::{
        begin_netlist_lifecycle_action, commit_netlist_lifecycle_action,
    };
    use crate::workbench::state::{Workspace, WorkspaceDocumentId};

    let mut app = RSpiceApp::test_instance();
    begin_netlist_lifecycle_action(&mut app.state, CodeSourceFileAction::New)
        .expect("a new top deck is offered");
    commit_netlist_lifecycle_action(&mut app.state).expect("the project owns a top deck");
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.workspace.content.active_view.clone();
    app.state.workbench.activate(Workspace::Design);
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved =
        crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(
            &app.state,
        )
        .expect("active schematic resolves");
    apply_open_resolved(&mut app, HardcopyWorkflow::Export, resolved).expect("dialog opens");
    app
}

/// The export as it stood before `transaction` committed, still in place
/// after it: open, busy with its job, and holding the same sealed plan.
#[cfg(not(target_arch = "wasm32"))]
fn assert_export_survived(app: &RSpiceApp, plan: &std::sync::Arc<HardcopyPlan>, transaction: &str) {
    let hardcopy = &app.state.dialogs.hardcopy;
    assert!(hardcopy.open, "{transaction} closed the export");
    assert!(hardcopy.busy, "{transaction} dropped the export's job");
    assert!(
        hardcopy
            .preview_plan
            .as_ref()
            .is_some_and(|kept| std::sync::Arc::ptr_eq(kept, plan)),
        "{transaction} dropped the export's sealed plan"
    );
}

/// A live-session update and a netlist editor edit each commit a project
/// transaction. Neither may take the export the operator started with it.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn netlist_transactions_keep_an_export_in_flight() {
    use crate::workbench::documents::netlist_document::{
        apply_live_owned_source, replace_owned_source,
    };

    let mut app = export_open_on_a_netlist_owned_project();
    app.state.dialogs.hardcopy.busy = true;
    let plan = app
        .state
        .dialogs
        .hardcopy
        .preview_plan
        .clone()
        .expect("sealed preview");

    assert!(apply_live_owned_source(
        &mut app.state,
        PEER_DECK.to_owned()
    ));
    assert_eq!(
        app.state.workspace.content.netlist_source.as_deref(),
        Some(PEER_DECK)
    );
    assert_export_survived(&app, &plan, "a live-session update");

    assert!(replace_owned_source(&mut app.state, EDITED_DECK.to_owned()));
    assert_eq!(
        app.state.workspace.content.netlist_source.as_deref(),
        Some(EDITED_DECK)
    );
    assert_export_survived(&app, &plan, "a netlist editor edit");
}

/// Reverting a document is a project transaction too, and one with nothing
/// to do with the netlist.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn reverting_a_document_keeps_an_export_in_flight() {
    use crate::state::{Point, Wire};
    use crate::workbench::lifecycle::project_lifecycle::{
        accept_loaded_project, confirm_revert_active_document, prepare_revert_active_document,
        snapshot,
    };

    let mut app = export_open_on_a_netlist_owned_project();
    let baseline = snapshot(&app.state).expect("the project snapshots");
    accept_loaded_project(&mut app.state, baseline, None);
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(72, Point::new(0, 10), Point::new(20, 10)));
    app.state.dialogs.hardcopy.busy = true;
    let plan = app
        .state
        .dialogs
        .hardcopy
        .preview_plan
        .clone()
        .expect("sealed preview");

    let review =
        prepare_revert_active_document(&app.state).expect("the edited schematic can be reverted");
    confirm_revert_active_document(&mut app.state, &review).expect("the revert commits");
    assert_eq!(
        app.state.schematic.document().wires.len(),
        1,
        "the revert restored the accepted schematic"
    );
    assert_export_survived(&app, &plan, "a revert");
}

/// An export started before a live-session update still writes its file and
/// records its receipt, frame by frame, the way the application drives it.
///
/// The export starts where `publish_export` hands over once the save picker
/// has answered: the destination is observed by the application's own export
/// backend and the publication is authenticated from the sealed plan.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn an_export_started_before_a_live_update_lands_its_file_and_receipt() {
    let path = std::env::temp_dir().join(format!(
        "rspice-hardcopy-live-update-{}-{}.pdf",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let mut app = export_open_on_a_netlist_owned_project();
    let destination = app
        .export_workflow_io
        .observe_destination(&path)
        .expect("the destination is observed");
    let source = app
        .state
        .dialogs
        .hardcopy
        .resolved_document
        .clone()
        .expect("the dialog holds its source");
    let plan = current_plan(&app, &source).expect("the sealed plan publishes");
    let (_, media_type, _) =
        format_file_contract(plan.setup().render().format()).expect("an export format");
    let staged_mapping = stage_print_mapping_persistence(&app, plan.setup().print_mapping())
        .expect("the print mapping stages");
    let metadata = current_metadata(&app).expect("the identity block is validated");
    app.state.dialogs.hardcopy.busy = true;
    begin_publication_authentication(
        &mut app,
        PendingNativePublication {
            plan,
            opened_source: source,
            metadata,
            staged_mapping,
            operation: super::super::execution::ExecutionOperation::Publication,
            destination: NativePublicationDestination::Export {
                path: path.clone(),
                destination,
                media_type,
                multi_part: false,
            },
        },
    )
    .expect("the export starts");

    assert!(
        crate::workbench::documents::netlist_document::apply_live_owned_source(
            &mut app.state,
            PEER_DECK.to_owned(),
        )
    );

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1_440.0, 900.0),
        )),
        ..Default::default()
    };
    let _ = ctx.run_ui(input(), |ui| app.render_hardcopy_dialog(ui));
    let job_active = PENDING_PUBLICATION.with(|slot| slot.borrow().is_some())
        || PUBLICATION.with(|slot| slot.borrow().is_some())
        || FINALIZATION.with(|slot| slot.borrow().is_some());
    assert!(
        app.state.dialogs.hardcopy.open && app.state.dialogs.hardcopy.busy && job_active,
        "the frame after the live update cancelled the export: {:?}",
        app.state.dialogs.hardcopy.error
    );

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while app.state.dialogs.hardcopy.busy {
        assert!(
            std::time::Instant::now() < deadline,
            "the export never finished"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
        let _ = ctx.run_ui(input(), |ui| app.render_hardcopy_dialog(ui));
    }

    assert_eq!(app.state.dialogs.hardcopy.error, None);
    assert!(
        !app.state.dialogs.hardcopy.open,
        "a finished export closes its dialog"
    );
    let receipt = app
        .state
        .dialogs
        .hardcopy
        .last_receipt
        .as_ref()
        .expect("the export recorded a receipt");
    assert!(matches!(
        receipt.outcome(),
        HardcopyOutcome::ArtifactExported { .. }
    ));
    assert!(app.state.workspace.content.hardcopy_receipts_dirty);
    let written = std::fs::read(&path).expect("the export wrote its file");
    std::fs::remove_file(&path).expect("remove the exported fixture");
    assert!(written.starts_with(b"%PDF"), "the export wrote a PDF");
}

/// A hardcopy write that failed after its dialog was closed is said, once;
/// one the reader cancelled by closing the dialog is not, and an open dialog
/// shows its own failure.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_hardcopy_that_fails_after_its_dialog_closed_is_reported() {
    let ctx = egui::Context::default();
    let mut app = RSpiceApp::test_instance();
    conclude_native_finalization(
        &mut app,
        Err("Hardcopy export was cancelled before it was written.".to_owned()),
    );
    app.state.drain_notices(&ctx);
    assert!(app.state.ui.toasts.activity().is_empty());

    app.state.dialogs.hardcopy.open = true;
    conclude_native_finalization(&mut app, Err("The disk is full.".to_owned()));
    app.state.drain_notices(&ctx);
    assert_eq!(
        app.state.dialogs.hardcopy.error.as_deref(),
        Some("The disk is full.")
    );
    assert!(app.state.ui.toasts.activity().is_empty());

    app.state.dialogs.hardcopy.close();
    conclude_native_finalization(&mut app, Err("The disk is full.".to_owned()));
    app.state.drain_notices(&ctx);
    let activity = app.state.ui.toasts.activity();
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].title(), "Hardcopy failed");
    assert_eq!(activity[0].message(), "The disk is full.");
}
