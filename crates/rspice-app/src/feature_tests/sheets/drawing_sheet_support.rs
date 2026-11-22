//! The sheet format manager over a whole application.

use crate::workbench::app::RSpiceApp;
use rspice_app_sheets::workbench::app::dialogs::drawing_sheet_support::*;
use rspice_app_state::state::{
    DrawingSheetStandard, DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId,
    DrawingSheetTransactionKind, SchematicPageOrientation, SchematicSheetFormat, SheetDefinition,
    SheetPortPolicy,
};

/// [`open_sheet_format_manager`] on the application's own parts.
fn open_manager(state: &mut crate::workbench::app_state::AppState) -> Result<(), String> {
    open_sheet_format_manager(
        &mut state.sheets.drawing_sheet_support,
        &mut state.model,
        &mut state.workbench,
        state.design.schematic_visibility.review_markers,
        &mut *state.design,
    )
}

#[test]
fn manager_refuses_to_open_without_schematic_edit_authority() {
    let mut app = RSpiceApp::test_instance();
    let key = app.state.model.workspace.content.active_key();
    app.state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();
    app.state.model.schematic.session.read_only = true;

    let error = open_manager(&mut app.state)
        .expect_err("read-only documents must not start an edit transaction");

    assert_eq!(error, "The active schematic is read-only.");
    assert!(!app.state.sheets.drawing_sheet_support.manager.open);
    assert!(
        app.state
            .sheets
            .drawing_sheet_support
            .manager
            .edit
            .is_none()
    );
}

/// A new schematic has no sheet catalog until a sheet workflow is first
/// used. Opening Sheet formats… is that use: it creates the catalog with
/// the drawing as Sheet 1 and opens on it, where it used to refuse.
#[test]
fn opening_the_manager_on_a_drawing_without_a_catalog_creates_it() {
    let mut app = RSpiceApp::test_instance();
    let key = app.state.model.workspace.content.active_key();
    assert!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog(&key)
            .is_none()
    );

    open_manager(&mut app.state).expect("the first use creates the catalog");

    let catalog = app
        .state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
        .expect("a catalog");
    assert_eq!(catalog.sheets().len(), 1);
    assert_eq!(catalog.sheets()[0].name(), "Sheet 1");
    assert!(app.state.sheets.drawing_sheet_support.manager.open);
}

#[test]
fn manager_partially_applies_and_names_organization_managed_skips() {
    let mut app = RSpiceApp::test_instance();
    let key = app.state.model.workspace.content.active_key();
    let first = app
        .state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();
    let (managed, ordinary) = {
        let catalog = app
            .state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .unwrap();
        let managed = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Managed".to_owned(),
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .unwrap();
        let ordinary = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Ordinary".to_owned(),
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(3),
                },
                Some(managed),
            )
            .unwrap();
        (managed, ordinary)
    };
    let source = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA3,
        SchematicPageOrientation::Landscape,
    )
    .try_update(|draft| {
        draft
            .title_block
            .fields
            .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
            .unwrap()
            .value = "Sheet 1".to_owned();
    })
    .unwrap();
    let managed_format = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                .unwrap()
                .value = "Managed".to_owned();
        })
        .unwrap();
    {
        let catalog = app
            .state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog_mut(&key)
            .unwrap();
        let revision = catalog.find(first).unwrap().revision();
        catalog
            .update_sheet_page_format(first, revision, source.clone())
            .unwrap();
        let revision = catalog.find(managed).unwrap().revision();
        catalog
            .update_sheet_page_format(managed, revision, managed_format.clone())
            .unwrap();
        catalog.set_active(first).unwrap();
    }
    let topology = app.state.model.schematic.topology_version();
    open_manager(&mut app.state).unwrap();
    for row in &mut app.state.sheets.drawing_sheet_support.manager.rows {
        row.selected = true;
    }

    let message = apply_sheet_format_manager(
        &app.state.sheets.drawing_sheet_support.manager,
        &mut app.state.model,
    )
    .unwrap();

    let catalog = app
        .state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
        .unwrap();
    assert_eq!(
        catalog.find(managed).unwrap().page_format(),
        &managed_format
    );
    let ordinary_format = catalog.find(ordinary).unwrap().page_format();
    assert_eq!(ordinary_format.authored_size, source.authored_size);
    assert_eq!(ordinary_format.orientation, source.orientation);
    // The source's Title stays on its own sheet: the untitled target keeps
    // lettering its name.
    assert_eq!(
        ordinary_format.title_block.fields[&DrawingSheetTitleFieldId::SheetTitle].value,
        ""
    );
    assert!(message.contains("Managed"));
    let receipt = app
        .state
        .model
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .transaction_receipts
        .last()
        .unwrap();
    assert_eq!(
        receipt.kind,
        DrawingSheetTransactionKind::SheetFormatManager
    );
    assert_eq!(receipt.selected_sheet_ids.len(), 3);
    assert_eq!(receipt.applied_sheet_ids, vec![ordinary]);
    assert_eq!(receipt.unchanged_sheet_ids, vec![first]);
    assert_eq!(receipt.skipped[0].sheet_id, managed);
    assert!(app.state.model.can_undo_project_design());
    assert_eq!(app.state.model.schematic.topology_version(), topology);
}

/// The manager's way out to the open sheet's own properties closes it and
/// shows the sheet panel, rather than applying anything.
#[test]
fn the_manager_hands_the_open_sheet_to_sheet_properties() {
    let mut app = RSpiceApp::test_instance();
    let key = app.state.model.workspace.content.active_key();
    app.state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();
    open_manager(&mut app.state).unwrap();
    app.state.sheets.drawing_sheet_support.manager.source =
        SheetManagerFormatSource::SheetProperties;
    assert_eq!(
        apply_sheet_format_manager(
            &app.state.sheets.drawing_sheet_support.manager,
            &mut app.state.model
        )
        .unwrap_err(),
        "Edit the open sheet in Sheet properties."
    );
    assert!(!app.state.model.can_undo_project_design());
}
