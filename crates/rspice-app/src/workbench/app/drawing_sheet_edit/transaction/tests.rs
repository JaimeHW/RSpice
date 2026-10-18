//! What one sheet edit lands, as one undo entry, and what it refuses.

use super::*;
use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};
use crate::workbench::app::RSpiceApp;

fn governed_app_with_sheets(count: usize) -> (RSpiceApp, String, Vec<SheetId>) {
    let mut app = RSpiceApp::test_instance();
    let key = app.state.workspace.content.active_key();
    let designs = &mut app.state.workspace.content.design_management;
    let first = designs
        .bootstrap_for_cell_view(&key, "Sheet 1", [])
        .unwrap();
    let mut ids = vec![first];
    for index in 1..count {
        let catalog = designs.sheet_catalog_mut(&key).unwrap();
        let id = catalog
            .create_sheet(
                SheetDefinition {
                    name: format!("Sheet {}", index + 1),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some((index + 1) as u32),
                },
                ids.last().copied(),
            )
            .unwrap();
        ids.push(id);
    }
    designs
        .sheet_catalog_mut(&key)
        .unwrap()
        .set_active(first)
        .unwrap();
    (app, key, ids)
}

fn saved_format(app: &RSpiceApp, key: &str, id: SheetId) -> SchematicSheetFormat {
    app.state
        .workspace
        .content
        .design_management
        .sheet_catalog(key)
        .unwrap()
        .find(id)
        .unwrap()
        .page_format()
        .clone()
}

/// Write `format` to sheet `id` of the catalog `key` names.
fn put_format(app: &mut RSpiceApp, key: &str, id: SheetId, format: SchematicSheetFormat) {
    let catalog = app
        .state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(key)
        .unwrap();
    let revision = catalog.find(id).unwrap().revision();
    catalog
        .update_sheet_page_format(id, revision, format)
        .unwrap();
}

fn titled(format: SchematicSheetFormat, title: &str) -> SchematicSheetFormat {
    format
        .try_update(|draft| {
            draft
                .title_block
                .fields
                .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                .unwrap()
                .value = title.to_owned();
        })
        .unwrap()
}

#[test]
fn a_sheet_is_edited_only_on_a_writable_schematic() {
    let mut app = RSpiceApp::test_instance();
    assert!(drawing_sheet_editable(&app.state));

    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );

    assert!(app.state.schematic_edit_read_only());
    assert!(!drawing_sheet_editable(&app.state));
    assert_eq!(
        crate::workbench::commands::vocabulary::Command::SheetProperties.availability(&app),
        crate::workbench::commands::CommandAvailability::Disabled(
            "the active schematic is read-only"
        )
    );
    assert!(
        apply_drawing_sheet_edit(
            &mut app.state,
            DrawingSheetEdit::Orientation(SchematicPageOrientation::Portrait),
        )
        .is_err()
    );
    assert!(!app.state.can_undo_project_design());
}

/// An edit to a governed sheet is one presentation transaction: the sheet
/// takes it whole, nothing about the circuit moves, and one undo removes it.
#[test]
fn an_edit_is_one_exact_undoable_presentation_transaction() {
    let (mut app, key, ids) = governed_app_with_sheets(1);
    let topology = app.state.schematic.topology_version();
    let drc_version = app.state.dialogs.drc_checked_version;
    let retained_runs = app.state.simulation.retained.runs.len();
    let before = saved_format(&app, &key, ids[0]);

    apply_drawing_sheet_edit(&mut app.state, DrawingSheetEdit::Zones(false)).unwrap();

    let after = saved_format(&app, &key, ids[0]);
    assert_eq!(after.zones.mode, DrawingSheetZoneMode::None);
    assert_eq!(after.inheritance, DrawingSheetInheritance::Explicit);
    assert_eq!(after.authored_size, before.authored_size);
    assert_eq!(after.border, before.border);
    assert!(app.state.can_undo_project_design());
    assert!(app.state.workspace.content.project_metadata_dirty);
    assert_eq!(app.state.schematic.topology_version(), topology);
    assert_eq!(app.state.dialogs.drc_checked_version, drc_version);
    assert_eq!(app.state.simulation.retained.runs.len(), retained_runs);
    let receipt = app
        .state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .transaction_receipts
        .last()
        .unwrap()
        .clone();
    assert_eq!(receipt.kind, DrawingSheetTransactionKind::PageSetup);
    assert_eq!(receipt.applied_sheet_ids, vec![ids[0]]);
    assert!(app.state.undo_project_design().unwrap().is_some());
    assert!(!app.state.can_undo_project_design());
}

/// Each edit is its own undo step.
#[test]
fn each_edit_is_its_own_undo_step() {
    let (mut app, key, ids) = governed_app_with_sheets(1);
    let before = saved_format(&app, &key, ids[0]);

    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::AnsiB),
    )
    .unwrap();
    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::TitleField(DrawingSheetTitleFieldId::DrawnBy, "J. Whitfield".into()),
    )
    .unwrap();

    let after = saved_format(&app, &key, ids[0]);
    assert_eq!(
        after.authored_size,
        AuthoredDrawingSheetSize::Standard {
            standard: DrawingSheetStandard::AnsiB
        }
    );
    assert_eq!(
        after.title_block.fields[&DrawingSheetTitleFieldId::DrawnBy].value,
        "J. Whitfield"
    );
    assert!(app.state.undo_project_design().unwrap().is_some());
    assert!(app.state.undo_project_design().unwrap().is_some());
    assert_eq!(saved_format(&app, &key, ids[0]), before);
}

/// The draft's validation refuses an edit in its words, and the refusal
/// leaves nothing behind.
#[test]
fn a_refused_edit_leaves_nothing_behind() {
    let (mut app, _, _) = governed_app_with_sheets(1);
    let error = apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::TitleField(DrawingSheetTitleFieldId::SheetTitle, String::new()),
    )
    .unwrap_err();
    assert!(error.contains("Title is required"), "{error}");
    assert!(!app.state.can_undo_project_design());
}

/// A sheet's own title is edited without taking it off the project default
/// it follows; only a physical edit gives it a format of its own.
#[test]
fn an_inherited_sheets_title_edit_keeps_it_on_the_project_default() {
    let (mut app, key, ids) = governed_app_with_sheets(1);
    let inherited = app
        .state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .default_format
        .clone();
    put_format(&mut app, &key, ids[0], titled(inherited, "Original title"));

    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::TitleField(
            DrawingSheetTitleFieldId::SheetTitle,
            "Released amplifier".to_owned(),
        ),
    )
    .unwrap();

    let saved = saved_format(&app, &key, ids[0]);
    assert_eq!(saved.inheritance, DrawingSheetInheritance::ProjectDefault);
    assert_eq!(
        saved.title_block.fields[&DrawingSheetTitleFieldId::SheetTitle].value,
        "Released amplifier"
    );

    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Orientation(SchematicPageOrientation::Portrait),
    )
    .unwrap();
    assert_eq!(
        saved_format(&app, &key, ids[0]).inheritance,
        DrawingSheetInheritance::Explicit
    );
}

/// The first edit to a schematic's sheet gives it its governed sheet, with
/// the edit on it, the project's values moved to the project, and a physical
/// edit taking it off the project default.
#[test]
fn the_first_edit_gives_the_schematic_its_governed_sheet() {
    let mut app = RSpiceApp::test_instance();
    let key = app.state.workspace.content.active_key();
    let mut settings = app
        .state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .clone();
    settings.title_block_field_values.insert(
        DrawingSheetTitleFieldId::Organization,
        "Previous organization".to_owned(),
    );
    let revision = app.state.workspace.content.design_management.revision();
    app.state
        .workspace
        .content
        .design_management
        .update_drawing_sheet_settings(revision, settings)
        .unwrap();

    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::TitleField(
            DrawingSheetTitleFieldId::Organization,
            "Example Labs".to_owned(),
        ),
    )
    .unwrap();

    let designs = &app.state.workspace.content.design_management;
    let sheet = designs.sheet_catalog(&key).unwrap().active().unwrap();
    assert_eq!(
        sheet.page_format().title_block.fields[&DrawingSheetTitleFieldId::Organization].value,
        ""
    );
    assert_eq!(
        designs.drawing_sheet_settings().title_block_field_values
            [&DrawingSheetTitleFieldId::Organization],
        "Example Labs"
    );
    assert!(app.state.can_undo_project_design());

    let mut app = RSpiceApp::test_instance();
    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Orientation(SchematicPageOrientation::Portrait),
    )
    .unwrap();
    let sheet = app
        .state
        .workspace
        .content
        .design_management
        .sheet_catalog(&key)
        .and_then(|catalog| catalog.active())
        .expect("the first edit gives the schematic its governed sheet")
        .page_format()
        .clone();
    assert_eq!(sheet.inheritance, DrawingSheetInheritance::Explicit);
    assert_eq!(sheet.orientation, SchematicPageOrientation::Portrait);
    assert_eq!(
        resolve_active_drawing_sheet(&app.state).format.orientation,
        SchematicPageOrientation::Portrait
    );
}

fn organization_managed(title: &str) -> SchematicSheetFormat {
    titled(
        SchematicSheetFormat::default()
            .try_update(|draft| {
                draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
            })
            .unwrap(),
        title,
    )
}

/// The organization's block is the organization's: no edit replaces it, and
/// the frame and size around it stay the sheet's to edit.
#[test]
fn an_organization_managed_block_survives_every_edit() {
    let (mut app, key, ids) = governed_app_with_sheets(1);
    put_format(
        &mut app,
        &key,
        ids[0],
        organization_managed("Managed sheet"),
    );

    for edit in [
        DrawingSheetEdit::TitleBlock(DrawingSheetTitleBlockTemplate::None),
        DrawingSheetEdit::Zones(false),
        DrawingSheetEdit::Size(DrawingSheetStandard::IsoA3),
    ] {
        let _ = apply_drawing_sheet_edit(&mut app.state, edit);
    }

    let saved = saved_format(&app, &key, ids[0]);
    assert_eq!(
        saved.title_block.template,
        DrawingSheetTitleBlockTemplate::OrganizationManaged
    );
    assert_eq!(saved.zones.mode, DrawingSheetZoneMode::None);
    assert_eq!(saved.authored_size.label(), "ISO A3");
}

/// A value the organization's block locks is refused at commit in its words,
/// and nothing is published.
#[test]
fn a_value_the_organization_locks_is_refused_and_changes_nothing() {
    let (mut app, key, ids) = governed_app_with_sheets(1);
    put_format(
        &mut app,
        &key,
        ids[0],
        organization_managed("Managed sheet"),
    );
    let mut settings = app
        .state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .clone();
    settings.title_block_field_values.insert(
        DrawingSheetTitleFieldId::Classification,
        "INTERNAL".to_owned(),
    );
    let revision = app.state.workspace.content.design_management.revision();
    app.state
        .workspace
        .content
        .design_management
        .update_drawing_sheet_settings(revision, settings)
        .unwrap();
    let design_revision = app.state.workspace.content.design_management.revision();

    let error = apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::TitleField(
            DrawingSheetTitleFieldId::Classification,
            "PUBLIC".to_owned(),
        ),
    )
    .unwrap_err();

    assert!(error.contains("classification"), "{error}");
    assert_eq!(
        app.state.workspace.content.design_management.revision(),
        design_revision
    );
    assert_eq!(
        app.state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .title_block_field_values[&DrawingSheetTitleFieldId::Classification],
        "INTERNAL"
    );
}

/// A one-off size is named and saved to the project's custom sizes, and the
/// sheet goes onto the saved size.
#[test]
fn a_one_off_size_is_saved_to_the_projects_custom_sizes() {
    let (mut app, key, ids) = governed_app_with_sheets(1);
    for edit in [
        DrawingSheetEdit::CustomSize {
            width_um: 250_000,
            height_um: 400_000,
        },
        DrawingSheetEdit::CustomSizeName("Lab panel".to_owned()),
        DrawingSheetEdit::SaveCustomSize(DrawingSheetPresetScope::Project),
    ] {
        apply_drawing_sheet_edit(&mut app.state, edit).unwrap();
    }

    let management = &app.state.workspace.content.design_management;
    let presets = &management.drawing_sheet_settings().presets;
    assert_eq!(presets.len(), 1);
    assert_eq!(presets[0].name, "Lab panel");
    let format = saved_format(&app, &key, ids[0]);
    assert_eq!(format.authored_size.label(), "Lab panel");
    assert_eq!(format.oriented_dimensions_um(), (250_000, 400_000));
}

/// A person's own custom size taken by a sheet is copied into the project
/// exactly, and the sheet goes onto the copy.
#[test]
fn a_personal_size_taken_by_a_sheet_is_copied_into_the_project() {
    let (mut app, key, ids) = governed_app_with_sheets(1);
    let personal = DrawingSheetPreset {
        id: "personal-review-strip".to_owned(),
        name: "Review strip".to_owned(),
        scope: DrawingSheetPresetScope::User,
        format: SchematicSheetFormat::try_custom(
            "Review strip",
            210_001,
            594_002,
            SchematicPageOrientation::Portrait,
        )
        .unwrap()
        .try_update(|draft| {
            if let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size {
                snapshot.preset_id = Some("personal-review-strip".to_owned());
            }
        })
        .unwrap(),
    };
    let mut personal_settings = app
        .state
        .ui
        .preferences
        .drawing_sheet_personal_preferences();
    personal_settings.presets.push(personal.clone());
    app.state
        .ui
        .preferences
        .set_drawing_sheet_personal_preferences(personal_settings)
        .unwrap();

    apply_drawing_sheet_edit(&mut app.state, DrawingSheetEdit::Preset(Box::new(personal))).unwrap();

    let project_preset = app
        .state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .presets
        .first()
        .unwrap()
        .clone();
    assert_eq!(project_preset.scope, DrawingSheetPresetScope::Project);
    assert_eq!(
        project_preset.format.portrait_dimensions_um(),
        (210_001, 594_002)
    );
    let AuthoredDrawingSheetSize::Custom { snapshot } =
        saved_format(&app, &key, ids[0]).authored_size
    else {
        panic!("a captured personal size stays a custom snapshot");
    };
    assert_eq!(
        snapshot.preset_id.as_deref(),
        Some(project_preset.id.as_str())
    );
}

/// An edit that changes nothing leaves no history.
#[test]
fn an_edit_that_changes_nothing_leaves_no_history() {
    let (mut app, _, _) = governed_app_with_sheets(1);
    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Orientation(SchematicPageOrientation::Landscape),
    )
    .unwrap();
    assert!(!app.state.can_undo_project_design());
}

/// ASME Y14.1 5.2 recommends one size for every sheet of a drawing: the
/// sheet advises, and refuses nothing, when an ANSI or ARCH schematic's
/// sheets stand on more than one size. ISO 5457 makes no such
/// recommendation.
#[test]
fn an_asme_schematic_on_mixed_sizes_is_advised_against() {
    let (mut app, key, ids) = governed_app_with_sheets(2);
    let on = |standard| {
        titled(
            SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape),
            "Sheet",
        )
    };
    put_format(&mut app, &key, ids[1], on(DrawingSheetStandard::AnsiD));
    let advisory = |app: &RSpiceApp, standard| mixed_size_advisory(&app.state, &on(standard));
    assert_eq!(
        advisory(&app, DrawingSheetStandard::AnsiB),
        Some(DRAWING_SHEET_MIXED_SIZES_ADVISORY)
    );
    assert_eq!(advisory(&app, DrawingSheetStandard::AnsiD), None);
    assert_eq!(advisory(&app, DrawingSheetStandard::IsoA3), None);
    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::AnsiB),
    )
    .expect("an advisory, not a refusal");
}
