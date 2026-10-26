//! The sheet format manager over a whole application.

use crate::state::{
    DrawingSheetStandard, DrawingSheetTitleBlockTemplate, DrawingSheetTitleFieldId,
    DrawingSheetTransactionKind, SchematicPageOrientation, SchematicSheetFormat, SheetDefinition,
    SheetPortPolicy, SheetTemplate,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::drawing_sheet_support::*;

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
fn sheet_format_manager_controls_stack_at_phone_width() {
    assert!(sheet_format_manager_controls_stack(366.0));
    assert!(sheet_format_manager_controls_stack(619.0));
    assert!(!sheet_format_manager_controls_stack(620.0));
    assert!(!sheet_format_manager_controls_stack(760.0));
}

#[test]
fn thumbnail_strip_collapses_the_tracks_a_short_document_does_not_fill() {
    // Three sheets are three cards across the strip, exactly as the
    // reference's `auto-fit` track does, rather than three 140 pt tiles
    // bunched against the left edge of an empty panel.
    assert_eq!(manager_thumbnail_columns(1_136.0, 3), 3);
    // Once the document is longer than the row, the row is the limit.
    assert_eq!(manager_thumbnail_columns(1_136.0, 12), 7);
    assert_eq!(manager_thumbnail_columns(300.0, 12), 2);
    // A phone-width strip still lays one readable card.
    assert_eq!(manager_thumbnail_columns(120.0, 4), 1);
}

#[test]
fn thumbnail_card_is_sized_to_the_page_it_draws() {
    // A4 landscape: the card a three-sheet strip hands out is spent on the
    // page, with only the desk margin and the caption left over.
    let landscape = 210.0 / 297.0;
    let card = 300.0;
    let height = manager_thumbnail_preview_height(card, landscape);
    assert!(
        height < MANAGER_THUMB_MAX_PREVIEW,
        "{height} hit the ceiling"
    );
    let page_height = height - MANAGER_THUMB_PREVIEW_INSET_Y - MANAGER_THUMB_CAPTION;
    let page_width = page_height / landscape;
    let card_inner = card - 2.0 * f32::from(MANAGER_THUMB_PADDING);
    assert!(
        page_width >= card_inner - MANAGER_THUMB_PREVIEW_INSET_X - 1.0,
        "the page should fill its card: {page_width} in {card_inner}"
    );
    // Both bounds hold: a two-sheet strip stops at the ceiling and a full
    // row of narrow cards never draws a page too small to read.
    assert_eq!(
        manager_thumbnail_preview_height(900.0, landscape),
        MANAGER_THUMB_MAX_PREVIEW
    );
    assert_eq!(
        manager_thumbnail_preview_height(140.0, landscape),
        MANAGER_THUMB_MIN_PREVIEW
    );
    // A portrait set is taller for the same card, which is the whole point
    // of ruling the strip on the set's own bounding page.
    assert!(
        manager_thumbnail_preview_height(372.0, 297.0 / 210.0)
            > manager_thumbnail_preview_height(372.0, landscape)
    );
}

#[test]
fn manager_columns_spend_the_whole_table_width() {
    // The body width of the drawing-sheet dialog, which is the width the
    // reference table's column shares were read at.
    const REFERENCE_WIDTH: f32 = 1_136.0;
    let (last, leading) = MANAGER_COLUMNS.split_last().unwrap();
    let shares: f32 = leading.iter().map(|(_, share, _)| share).sum();
    // The seven stated columns leave the eighth a remainder wide enough for
    // its longest value, so the header band and a selected row's wash reach
    // the right edge of the table instead of stopping mid-surface.
    assert!(shares < 1.0, "the stated shares must leave a remainder");
    let stated: f32 = leading
        .iter()
        .map(|(_, share, minimum)| (REFERENCE_WIDTH * share).max(*minimum))
        .sum();
    let remainder = REFERENCE_WIDTH - stated;
    assert!(
        remainder >= last.2,
        "the last column is {remainder}, under its own {} minimum",
        last.2
    );
    // Every column can still state its content at the width below which the
    // body scrolls sideways rather than shortening a value.
    let minima: f32 = MANAGER_COLUMNS.iter().map(|(_, _, minimum)| minimum).sum();
    assert!(
        minima <= MANAGER_TABLE_MIN_WIDTH,
        "{minima} exceeds the {MANAGER_TABLE_MIN_WIDTH} scroll floor"
    );
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
                    template: SheetTemplate::AnalogSchematic,
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
                    template: SheetTemplate::AnalogSchematic,
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
    assert_eq!(
        ordinary_format.title_block.fields[&DrawingSheetTitleFieldId::SheetTitle].value,
        "Ordinary"
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
