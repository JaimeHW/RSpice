//! The publish dialog's output contract, in the case that exports the pane
//! the Results workspace has selected.

use crate::feature_tests::hardcopy::{hardcopy_parts, resolve_retained_hardcopy_source};
use crate::product::ContentDigest;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hardcopy::GovernedSheetPageAuthority;
use crate::workbench::app::dialogs::hardcopy::HardcopyWorkflow;
use crate::workbench::app::dialogs::hardcopy::SchematicPageSetupAuthority;
use crate::workbench::app::dialogs::hardcopy::publish::active_retained_source_selection;
use crate::workbench::app::dialogs::hardcopy::publish::*;
use crate::workbench::app::dialogs::hardcopy::render_hardcopy_dialog;
use crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument;
use rspice_hardcopy::render::HardcopyRenderer;
use rspice_hardcopy::render::HardcopySceneMetadata;
use rspice_hardcopy_contract::DuplexMode;
use rspice_hardcopy_contract::HardcopyFailureCode;
use rspice_hardcopy_contract::HardcopyOutcome;
use rspice_hardcopy_contract::HardcopyPlan;
use rspice_hardcopy_contract::HardcopyReceipt;
use rspice_hardcopy_contract::PrinterJobSettings;
use rspice_hardcopy_contract::PrinterMediaSource;

/// The setup a sheet of `format` prints with: its own paper at 1:1.
fn sheet_output_setup(
    format: &crate::state::SchematicSheetFormat,
) -> rspice_hardcopy_contract::HardcopySetup {
    use rspice_hardcopy_contract::{AuthoredSheetMedia, HardcopySetup, Length};
    let (width, height) = format.oriented_dimensions_um();
    HardcopySetup::for_authored_sheets(vec![
        AuthoredSheetMedia::try_new(
            0,
            format.display(),
            Length::from_micrometres(width),
            Length::from_micrometres(height),
        )
        .unwrap(),
    ])
    .unwrap()
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
        .model
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
        .results
        .results
        .session
        .select_persistent_document_pane(document_id, pane_id);

    let (source_key, scope) = active_retained_source_selection(&hardcopy_parts(&mut app)).unwrap();

    assert!(source_key.contains(&format!(
        ":result-document:{document_id}:pane:{}",
        pane_id.get()
    )));
    assert_eq!(
        scope,
        rspice_hardcopy_contract::HardcopyScope::ActivePlotDocument
    );
}

/// The active document as File > Print opens it: the dialog's own source
/// selection, resolved through the retained path its worker runs.
fn resolve_active_source(app: &mut RSpiceApp) -> Result<ResolvedHardcopyDocument, String> {
    let (source_key, scope) = active_retained_source_selection(&hardcopy_parts(app))?;
    resolve_retained_hardcopy_source(&app.state, &source_key, scope)
        .map_err(|error| error.to_string())
}

#[test]
fn governed_design_print_selects_the_active_sheet_identity() {
    let mut app = RSpiceApp::test_instance();
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(crate::workbench::state::WorkspaceDocumentId::CellView(
            reference,
        ));
    let key = app.state.model.workspace.content.active_key();
    let sheet_id = app
        .state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();

    let (source_key, scope) = active_retained_source_selection(&hardcopy_parts(&mut app)).unwrap();

    assert!(source_key.ends_with(&format!(":sheet:{sheet_id}")));
    assert_eq!(scope, rspice_hardcopy_contract::HardcopyScope::CurrentSheet);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn governed_output_page_setup_preserves_authored_sheet_and_saves_hardcopy() {
    use crate::state::{Point, SchematicPageOrientation, SchematicSheetFormat, Wire};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let key = app.state.model.workspace.content.active_key();
    let sheet_id = app
        .state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [71])
        .unwrap();
    let catalog = app
        .state
        .model
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
    let resolved = resolve_active_source(&mut app).expect("active governed sheet resolves");
    let format = SchematicSheetFormat::try_custom(
        "Review board",
        304_800,
        457_200,
        SchematicPageOrientation::Landscape,
    )
    .unwrap();
    let setup = sheet_output_setup(&format);
    let pending = PendingPageSetup {
        opened_source: std::sync::Arc::new(resolved.clone()),
        setup: setup.clone(),
        staged_mapping: StagedPrintMappingPersistence::Document,
        schematic_authority: Some(authority),
    };

    commit_authenticated_page_setup(&mut hardcopy_parts(&mut app), resolved.clone(), pending)
        .unwrap();

    assert_eq!(
        app.state
            .model
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
        !app.state.model.can_undo_project_design(),
        "output-media setup must not create an authored-sheet transaction"
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .hardcopy_setups
            .setup_for(resolved.authority())
            .unwrap()
            .unwrap()
            .setup(),
        &setup
    );
    assert!(app.state.model.workspace.content.hardcopy_setups_dirty);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn legacy_output_page_setup_does_not_rewrite_document_policy() {
    use crate::state::{Point, SchematicPageOrientation, SchematicSheetFormat, Wire};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved = resolve_active_source(&mut app).expect("legacy schematic resolves");
    let policy_before = app.state.model.schematic.document().document_policy;
    let format = SchematicSheetFormat::try_custom(
        "Custom",
        300_123,
        450_987,
        SchematicPageOrientation::Portrait,
    )
    .unwrap();
    let pending = PendingPageSetup {
        opened_source: std::sync::Arc::new(resolved.clone()),
        setup: sheet_output_setup(&format),
        staged_mapping: StagedPrintMappingPersistence::Document,
        schematic_authority: Some(SchematicPageSetupAuthority {
            edit: crate::workbench::app::SchematicEditAuthority::capture(&app.state),
            governed_sheet: None,
        }),
    };

    commit_authenticated_page_setup(&mut hardcopy_parts(&mut app), resolved, pending).unwrap();

    assert_eq!(
        app.state.model.schematic.document().document_policy,
        policy_before,
        "hardcopy output media must not mutate the authored schematic policy"
    );
    assert_eq!(app.state.model.schematic.history().undo_description(), None);
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
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved = resolve_active_source(&mut app).expect("legacy schematic resolves");
    let authority = SchematicPageSetupAuthority {
        edit: crate::workbench::app::SchematicEditAuthority::capture(&app.state),
        governed_sheet: None,
    };
    app.state.model.safe_mode.activate(
        LocalSafeModeOptions {
            open_project_read_only: true,
            ..LocalSafeModeOptions::default()
        },
        String::new(),
    );
    let pending = PendingPageSetup {
        opened_source: std::sync::Arc::new(resolved.clone()),
        setup: sheet_output_setup(&SchematicSheetFormat::standard(
            SchematicPageSize::A3,
            SchematicPageOrientation::Portrait,
        )),
        staged_mapping: StagedPrintMappingPersistence::Document,
        schematic_authority: Some(authority),
    };

    assert!(
        commit_authenticated_page_setup(&mut hardcopy_parts(&mut app), resolved.clone(), pending)
            .expect_err("late read-only activation revokes Page Setup")
            .contains("read-only")
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .hardcopy_setups
            .setup_for(resolved.authority())
            .unwrap()
            .is_none()
    );
    assert_eq!(
        app.state
            .model
            .schematic
            .document()
            .document_policy
            .page_size,
        SchematicPageSize::A4
    );
}

#[test]
fn governed_page_authority_rejects_catalog_or_active_sheet_drift() {
    use crate::state::{SchematicPageOrientation, SchematicPageSize, SchematicSheetFormat};

    let mut app = RSpiceApp::test_instance();
    let key = app.state.model.workspace.content.active_key();
    let sheet_id = app
        .state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Input", [])
        .unwrap();
    let catalog = app
        .state
        .model
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
        .model
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
        validate_schematic_page_authority(&app.state.model, &authority)
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
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved = resolve_active_source(&mut app).expect("active schematic resolves");
    app.state
        .hardcopy
        .hardcopy
        .open_resolved(HardcopyWorkflow::Export, resolved.clone(), None)
        .expect("dialog opens");

    let preview = app
        .state
        .hardcopy
        .hardcopy
        .preview_plan
        .as_ref()
        .expect("sealed preview")
        .clone();
    let publication = current_plan(&hardcopy_parts(&mut app), &resolved).expect("unchanged plan");
    assert!(std::sync::Arc::ptr_eq(&preview, &publication));

    app.state.hardcopy.hardcopy.margin_left = "0.5".to_owned();
    assert!(
        current_plan(&hardcopy_parts(&mut app), &resolved)
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
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved = resolve_active_source(&mut app).expect("active schematic resolves");
    app.state
        .hardcopy
        .hardcopy
        .open_resolved(HardcopyWorkflow::Export, resolved.clone(), None)
        .expect("dialog opens");
    let plan = app
        .state
        .hardcopy
        .hardcopy
        .preview_plan
        .as_ref()
        .expect("sealed preview")
        .clone();
    let metadata = HardcopySceneMetadata::try_new(resolved.authority().display_name(), "RSpice")
        .expect("metadata");
    app.state.hardcopy.hardcopy.preview = Some(std::sync::Arc::new(
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
    app.state.hardcopy.hardcopy.source_resolution_generation = 19;
    app.state.hardcopy.hardcopy.printer_report =
        Some(crate::workbench::hardcopy_adapters::print::PrinterDiscoveryReport::default());
    app.state.hardcopy.hardcopy.printer_job = Some(
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
    app.state.hardcopy.hardcopy.last_receipt = Some(
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
    app.state.hardcopy.hardcopy.busy = true;
    app.state.hardcopy.hardcopy.error = Some("test error".to_owned());

    let cloned = app.state.clone();
    let hardcopy = &cloned.hardcopy.hardcopy;
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
fn project_print_mapping_stage_is_transactional_until_publication_commit() {
    use rspice_hardcopy_contract::{PrintMappingSaveScope, PrintMappingTable};

    let mut app = RSpiceApp::test_instance();
    let before = app
        .state
        .model
        .workspace
        .content
        .project_print_mappings
        .clone();
    let mapping = PrintMappingTable::try_new(
        PrintMappingSaveScope::ProjectPrintSet("release-proof".to_owned()),
        Vec::new(),
    )
    .unwrap();

    let staged = stage_print_mapping_persistence(&hardcopy_parts(&mut app), &mapping).unwrap();
    assert_eq!(
        app.state.model.workspace.content.project_print_mappings,
        before
    );
    assert!(
        !app.state
            .model
            .workspace
            .content
            .project_print_mappings_dirty
    );
    drop(staged);
    assert_eq!(
        app.state.model.workspace.content.project_print_mappings,
        before
    );
    assert!(
        !app.state
            .model
            .workspace
            .content
            .project_print_mappings_dirty
    );

    let staged = stage_print_mapping_persistence(&hardcopy_parts(&mut app), &mapping).unwrap();
    commit_print_mapping_persistence(&mut hardcopy_parts(&mut app), staged).unwrap();
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .project_print_mappings
            .get("release-proof")
            .unwrap(),
        &mapping
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .project_print_mappings_dirty
    );
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
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
    let reference = app.state.model.workspace.content.active_view.clone();
    app.state.workbench.activate(Workspace::Design);
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::CellView(reference));
    let resolved = resolve_active_source(&mut app).expect("active schematic resolves");
    apply_open_resolved(
        &mut hardcopy_parts(&mut app),
        HardcopyWorkflow::Export,
        resolved,
    )
    .expect("dialog opens");
    app
}

/// The export as it stood before `transaction` committed, still in place
/// after it: open, busy with its job, and holding the same sealed plan.
#[cfg(not(target_arch = "wasm32"))]
fn assert_export_survived(app: &RSpiceApp, plan: &std::sync::Arc<HardcopyPlan>, transaction: &str) {
    let hardcopy = &app.state.hardcopy.hardcopy;
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
    use crate::workbench::app_state::model::LiveOwnedSource;
    use crate::workbench::documents::netlist_document::replace_owned_source;
    use crate::workbench::framework::feature::CodePort;

    let mut app = export_open_on_a_netlist_owned_project();
    app.state.hardcopy.hardcopy.busy = true;
    let plan = app
        .state
        .hardcopy
        .hardcopy
        .preview_plan
        .clone()
        .expect("sealed preview");

    let shows_owned_root = app.state.code.shows_owned_root();
    assert!(matches!(
        app.state
            .model
            .apply_live_owned_source(PEER_DECK.to_owned(), shows_owned_root),
        LiveOwnedSource::Applied(_)
    ));
    assert_eq!(
        app.state.model.workspace.content.netlist_source.as_deref(),
        Some(PEER_DECK)
    );
    assert_export_survived(&app, &plan, "a live-session update");

    assert!(replace_owned_source(&mut app.state, EDITED_DECK.to_owned()));
    assert_eq!(
        app.state.model.workspace.content.netlist_source.as_deref(),
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
    use crate::workbench::app_state::services_access::{
        accept_loaded_project, confirm_revert_active_document,
    };
    use crate::workbench::lifecycle::project_lifecycle::{
        prepare_revert_active_document, snapshot,
    };

    let mut app = export_open_on_a_netlist_owned_project();
    let baseline = snapshot(&app.state).expect("the project snapshots");
    accept_loaded_project(&mut app.state, baseline, None);
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(72, Point::new(0, 10), Point::new(20, 10)));
    app.state.hardcopy.hardcopy.busy = true;
    let plan = app
        .state
        .hardcopy
        .hardcopy
        .preview_plan
        .clone()
        .expect("sealed preview");

    let review =
        prepare_revert_active_document(&app.state).expect("the edited schematic can be reverted");
    confirm_revert_active_document(&mut app.state, &review).expect("the revert commits");
    assert_eq!(
        app.state.model.schematic.document().wires.len(),
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
        .hardcopy
        .hardcopy
        .resolved_document
        .clone()
        .expect("the dialog holds its source");
    let plan = current_plan(&hardcopy_parts(&mut app), &source).expect("the sealed plan publishes");
    let (_, media_type, _) =
        format_file_contract(plan.setup().render().format()).expect("an export format");
    let staged_mapping =
        stage_print_mapping_persistence(&hardcopy_parts(&mut app), plan.setup().print_mapping())
            .expect("the print mapping stages");
    let metadata =
        current_metadata(&hardcopy_parts(&mut app)).expect("the identity block is validated");
    app.state.hardcopy.hardcopy.busy = true;
    begin_publication_authentication(
        &mut hardcopy_parts(&mut app),
        PendingNativePublication {
            plan,
            opened_source: source,
            metadata,
            staged_mapping,
            operation: crate::workbench::app::dialogs::hardcopy::execution::ExecutionOperation::Publication,
            destination: NativePublicationDestination::Export {
                path: path.clone(),
                destination,
                media_type,
                multi_part: false,
            },
        },
    )
    .expect("the export starts");

    let shows_owned_root =
        crate::workbench::framework::feature::CodePort::shows_owned_root(&*app.state.code);
    assert!(matches!(
        app.state
            .model
            .apply_live_owned_source(PEER_DECK.to_owned(), shows_owned_root),
        crate::workbench::app_state::model::LiveOwnedSource::Applied(_)
    ));

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1_440.0, 900.0),
        )),
        ..Default::default()
    };
    let _ = ctx.run_ui(input(), |ui| {
        render_hardcopy_dialog(&mut hardcopy_parts(&mut app), ui)
    });
    let job_active = PENDING_PUBLICATION.with(|slot| slot.borrow().is_some())
        || PUBLICATION.with(|slot| slot.borrow().is_some())
        || FINALIZATION.with(|slot| slot.borrow().is_some());
    assert!(
        app.state.hardcopy.hardcopy.open && app.state.hardcopy.hardcopy.busy && job_active,
        "the frame after the live update cancelled the export: {:?}",
        app.state.hardcopy.hardcopy.error
    );

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    while app.state.hardcopy.hardcopy.busy {
        assert!(
            std::time::Instant::now() < deadline,
            "the export never finished"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
        let _ = ctx.run_ui(input(), |ui| {
            render_hardcopy_dialog(&mut hardcopy_parts(&mut app), ui)
        });
    }

    assert_eq!(app.state.hardcopy.hardcopy.error, None);
    assert!(
        !app.state.hardcopy.hardcopy.open,
        "a finished export closes its dialog"
    );
    let receipt = app
        .state
        .hardcopy
        .hardcopy
        .last_receipt
        .as_ref()
        .expect("the export recorded a receipt");
    assert!(matches!(
        receipt.outcome(),
        HardcopyOutcome::ArtifactExported { .. }
    ));
    assert!(app.state.model.workspace.content.hardcopy_receipts_dirty);
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
        &mut hardcopy_parts(&mut app),
        Err("Hardcopy export was cancelled before it was written.".to_owned()),
    );
    app.state.drain_notices(&ctx);
    assert!(app.state.workbench.toasts.activity().is_empty());

    app.state.hardcopy.hardcopy.open = true;
    conclude_native_finalization(
        &mut hardcopy_parts(&mut app),
        Err("The disk is full.".to_owned()),
    );
    app.state.drain_notices(&ctx);
    assert_eq!(
        app.state.hardcopy.hardcopy.error.as_deref(),
        Some("The disk is full.")
    );
    assert!(app.state.workbench.toasts.activity().is_empty());

    app.state.hardcopy.hardcopy.close();
    conclude_native_finalization(
        &mut hardcopy_parts(&mut app),
        Err("The disk is full.".to_owned()),
    );
    app.state.drain_notices(&ctx);
    let activity = app.state.workbench.toasts.activity();
    assert_eq!(activity.len(), 1);
    assert_eq!(activity[0].title(), "Hardcopy failed");
    assert_eq!(activity[0].message(), "The disk is full.");
}
