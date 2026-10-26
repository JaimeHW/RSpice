//! The drawing-sheet defaults dialog over a whole application.

use crate::state::{
    AuthoredDrawingSheetSize, DrawingSheetBorderTemplate, DrawingSheetDisplayUnit,
    DrawingSheetInheritance, DrawingSheetStandard, DrawingSheetTitleBlockTemplate,
    SchematicPageOrientation, SchematicSheetFormat,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::drawing_sheet_defaults::*;

/// A format reaches several sheets through the project default: a new
/// default is the new format of every sheet that follows it, a sheet with
/// a format of its own keeps it, and the dialog names both.
#[test]
fn a_new_project_default_redraws_the_sheets_that_follow_it() {
    use crate::state::{DrawingSheetTitleFieldId, SheetDefinition, SheetPortPolicy, SheetTemplate};

    let mut app = RSpiceApp::test_instance();
    assert_eq!(DefaultReach::of(&app.state.model), None);
    let key = app.state.model.workspace.content.active_key();
    let designs = &mut app.state.model.workspace.content.design_management;
    let first = designs
        .bootstrap_for_cell_view(&key, "Sheet 1", [])
        .unwrap();
    let default = designs.drawing_sheet_settings().default_format.clone();
    let catalog = designs.sheet_catalog_mut(&key).unwrap();
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Sheet 2".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .unwrap();
    let titled = |format: &SchematicSheetFormat, title: &str, inheritance| {
        format
            .try_update(|draft| {
                draft.inheritance = inheritance;
                draft
                    .title_block
                    .fields
                    .get_mut(&DrawingSheetTitleFieldId::SheetTitle)
                    .unwrap()
                    .value = title.to_owned();
            })
            .unwrap()
    };
    let following = titled(&default, "Sheet 1", DrawingSheetInheritance::ProjectDefault);
    let own = titled(
        &SchematicSheetFormat::from_standard(
            DrawingSheetStandard::IsoA3,
            SchematicPageOrientation::Landscape,
        ),
        "Sheet 2",
        DrawingSheetInheritance::Explicit,
    );
    for (sheet, format) in [(first, following), (second, own)] {
        let current = catalog.find(sheet).unwrap();
        if current.page_format() != &format {
            let revision = current.revision();
            catalog
                .update_sheet_page_format(sheet, revision, format)
                .unwrap();
        }
    }
    catalog.set_active(first).unwrap();

    let reach = DefaultReach::of(&app.state.model).unwrap();
    assert_eq!(reach.following_text(), "Sheet 1");
    assert_eq!(reach.own_text().as_deref(), Some("Sheet 2"));

    open_drawing_sheet_defaults(
        &mut app.state.sheets.drawing_sheet_defaults,
        &app.state.model,
    );
    app.state
        .sheets
        .drawing_sheet_defaults
        .draft_project
        .default_format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiA,
        SchematicPageOrientation::Landscape,
    )
    .try_update(|draft| draft.inheritance = DrawingSheetInheritance::ProjectDefault)
    .unwrap();
    apply_drawing_sheet_defaults(
        &app.state.sheets.drawing_sheet_defaults,
        &mut app.state.model,
    )
    .unwrap();

    let drawn = |app: &RSpiceApp| {
        app.state
            .model
            .active_drawing_sheet()
            .format
            .authored_size
            .label()
            .to_owned()
    };
    assert_eq!(drawn(&app), "ANSI A");
    app.state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .unwrap()
        .set_active(second)
        .unwrap();
    assert_eq!(drawn(&app), "ISO A3");
}

/// The preview names the sheets a default reaches briefly enough to read
/// under it.
#[test]
fn the_sheets_a_default_reaches_are_named_briefly() {
    let names = |count: usize| {
        (1..=count)
            .map(|page| format!("Sheet {page}"))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        sheet_names(&names(5)),
        "Sheet 1, Sheet 2, Sheet 3 and 2 more"
    );
    let every = DefaultReach {
        following: names(4),
        own: Vec::new(),
    };
    assert_eq!(every.following_text(), "every sheet");
    assert_eq!(every.own_text(), None);
    let none = DefaultReach {
        following: Vec::new(),
        own: names(2),
    };
    assert_eq!(none.following_text(), "no sheet");
    assert_eq!(none.own_text().as_deref(), Some("Sheet 1, Sheet 2"));
}

#[test]
fn unchanged_defaults_are_a_real_no_op() {
    let mut app = RSpiceApp::test_instance();
    open_drawing_sheet_defaults(
        &mut app.state.sheets.drawing_sheet_defaults,
        &app.state.model,
    );
    let revision = app
        .state
        .model
        .workspace
        .content
        .design_management
        .revision();

    assert_eq!(
        apply_drawing_sheet_defaults(
            &app.state.sheets.drawing_sheet_defaults,
            &mut app.state.model
        )
        .unwrap(),
        "Drawing-sheet defaults already matched the saved values."
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .revision(),
        revision
    );
}

#[test]
fn project_default_is_one_history_transaction_and_personal_uses_preferences() {
    let mut app = RSpiceApp::test_instance();
    open_drawing_sheet_defaults(
        &mut app.state.sheets.drawing_sheet_defaults,
        &app.state.model,
    );
    app.state
        .sheets
        .drawing_sheet_defaults
        .draft_project
        .default_format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiA,
        SchematicPageOrientation::Landscape,
    )
    .try_update(|draft| {
        draft.inheritance = crate::state::DrawingSheetInheritance::ProjectDefault;
    })
    .unwrap();
    app.state
        .sheets
        .drawing_sheet_defaults
        .draft_personal
        .default_format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::JisB4,
        SchematicPageOrientation::Portrait,
    )
    .try_update(|draft| {
        draft.inheritance = crate::state::DrawingSheetInheritance::UserDefault;
    })
    .unwrap();

    let before = app
        .state
        .model
        .workspace
        .content
        .design_management
        .revision();
    apply_drawing_sheet_defaults(
        &app.state.sheets.drawing_sheet_defaults,
        &mut app.state.model,
    )
    .unwrap();

    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .revision(),
        before + 1
    );
    assert!(app.state.model.can_undo_project_design());
    assert_eq!(
        app.state
            .model
            .preferences
            .drawing_sheet_personal_preferences()
            .default_format
            .authored_size
            .label(),
        "JIS B4"
    );
}

#[test]
fn managed_default_properties_are_reapplied_at_commit_without_locking_orientation() {
    let mut app = RSpiceApp::test_instance();
    let mut settings = app
        .state
        .model
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .clone();
    settings.default_format = settings
        .default_format
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
        })
        .unwrap();
    let revision = app
        .state
        .model
        .workspace
        .content
        .design_management
        .revision();
    app.state
        .model
        .workspace
        .content
        .design_management
        .update_drawing_sheet_settings(revision, settings)
        .unwrap();
    open_drawing_sheet_defaults(
        &mut app.state.sheets.drawing_sheet_defaults,
        &app.state.model,
    );
    let draft = &mut app
        .state
        .sheets
        .drawing_sheet_defaults
        .draft_project
        .default_format;
    *draft = draft
        .try_update(|draft| {
            draft.apply_border_template(DrawingSheetBorderTemplate::None);
            draft.title_block.template = DrawingSheetTitleBlockTemplate::None;
            draft.orientation = SchematicPageOrientation::Portrait;
        })
        .unwrap();

    apply_drawing_sheet_defaults(
        &app.state.sheets.drawing_sheet_defaults,
        &mut app.state.model,
    )
    .unwrap();

    let saved = &app
        .state
        .model
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .default_format;
    // Only the title block is managed; the border stays as edited.
    assert_eq!(saved.border, DrawingSheetBorderTemplate::None);
    assert_eq!(
        saved.title_block.template,
        DrawingSheetTitleBlockTemplate::OrganizationManaged
    );
    assert_eq!(saved.orientation, SchematicPageOrientation::Portrait);
}

#[test]
fn a_managed_default_title_block_locks_only_the_title_block() {
    let title_authority = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.title_block.template = DrawingSheetTitleBlockTemplate::OrganizationManaged;
        })
        .unwrap();
    let mut title_candidate = SchematicSheetFormat::default()
        .try_update(|draft| {
            draft.apply_border_template(DrawingSheetBorderTemplate::None);
            draft.title_block.template = DrawingSheetTitleBlockTemplate::None;
        })
        .unwrap();
    enforce_managed_default_authority(&mut title_candidate, &title_authority);
    assert_eq!(title_candidate.border, DrawingSheetBorderTemplate::None);
    assert_eq!(
        title_candidate.title_block.template,
        DrawingSheetTitleBlockTemplate::OrganizationManaged
    );
}

#[test]
fn changing_default_standard_keeps_the_title_block() {
    let source = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA0,
        SchematicPageOrientation::Landscape,
    );

    let retargeted = retarget_default_to_standard(&source, DrawingSheetStandard::IsoA4);

    assert_eq!(
        retargeted.authored_size,
        AuthoredDrawingSheetSize::Standard {
            standard: DrawingSheetStandard::IsoA4,
        }
    );
    assert_eq!(retargeted.title_block.template, source.title_block.template);
    retargeted.validate().unwrap();
}

#[test]
fn retargeting_to_a_preset_keeps_the_records_presentation() {
    let source = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::IsoA4,
        SchematicPageOrientation::Portrait,
    )
    .try_update(|draft| {
        draft.display_unit = DrawingSheetDisplayUnit::Inches;
        draft.apply_border_template(DrawingSheetBorderTemplate::None);
    })
    .unwrap();
    let base = SchematicSheetFormat::try_custom(
        "Lab panel",
        250_000,
        400_000,
        SchematicPageOrientation::Portrait,
    )
    .unwrap();

    let retargeted = retarget_default(&source, base);

    assert!(matches!(
        retargeted.authored_size,
        AuthoredDrawingSheetSize::Custom { .. }
    ));
    assert_eq!(retargeted.orientation, SchematicPageOrientation::Portrait);
    assert_eq!(retargeted.display_unit, DrawingSheetDisplayUnit::Inches);
    assert_eq!(retargeted.border, DrawingSheetBorderTemplate::None);
    retargeted.validate().unwrap();
}

#[test]
fn chip_rows_pack_within_the_row_and_stretch_never_drops_a_chip() {
    let chips: Vec<ChipSpec> = (0..7)
        .map(|index| ChipSpec {
            strong: format!("chip {index}"),
            dims: String::new(),
            action: ChipAction::NewPreset,
            selected: false,
            natural_w: 120.0,
        })
        .collect();
    let rows = pack_chip_rows(&chips, 400.0);
    assert_eq!(rows.iter().map(Vec::len).sum::<usize>(), chips.len());
    assert!(rows.iter().all(|row| {
        let natural: f32 = row.len() as f32 * 120.0 + (row.len() - 1) as f32 * CHIP_GAP;
        natural <= 400.0
    }));
    // A chip wider than the whole row still lands on its own row.
    let wide = vec![ChipSpec {
        strong: "wide".to_owned(),
        dims: String::new(),
        action: ChipAction::NewPreset,
        selected: false,
        natural_w: 900.0,
    }];
    assert_eq!(pack_chip_rows(&wide, 400.0).len(), 1);
}
