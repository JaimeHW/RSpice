//! Tests for the publish dialog's output contract.
//!
//! The cases pin that the dialog never misrepresents a print target, and that
//! the mapping stage stays transactional until publication actually commits.

use super::*;

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
