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

/// Every size chip in the catalog is one cell of one grid: the families line
/// up and a family of six fills one row. Packing each row to its own chips
/// gave ISO A, ANSI, ARCH and JIS B four different chip widths and stretched
/// a lone ARCH E1 across the whole catalog.
#[test]
fn size_chips_share_one_grid_and_never_drop_a_chip() {
    // Room for seven 110 pt chips, but the largest family has six.
    let columns = catalog_columns(110.0, 6, 800.0);
    assert_eq!(columns, 6);
    let cell = catalog_cell_width(columns, 800.0);
    assert!((cell * 6.0 + CHIP_GAP * 5.0 - 800.0).abs() < 0.01);
    assert!(cell >= 110.0);
    // A family of six is one row; a family of four is one row of four cells.
    assert_eq!(pack_chip_rows(6, columns), vec![vec![0, 1, 2, 3, 4, 5]]);
    assert_eq!(pack_chip_rows(4, columns), vec![vec![0, 1, 2, 3]]);
    // A narrower catalog wraps onto the same columns and drops nothing.
    let narrow = catalog_columns(110.0, 6, 400.0);
    assert_eq!(narrow, 3);
    let rows = pack_chip_rows(7, narrow);
    assert_eq!(rows.iter().map(Vec::len).sum::<usize>(), 7);
    assert!(rows.iter().all(|row| row.len() <= narrow));
    // A chip wider than the catalog still gets a column of its own.
    assert_eq!(catalog_columns(900.0, 6, 400.0), 1);
}

/// The defaults read as the rest of the application does: sentence-case
/// headings, a card's frame and what it is for on lines of their own, every
/// choice a kit select named for what it picks, and Save unavailable on
/// unchanged defaults with the reason beside it. The cards were headed in
/// upper case, ran the frame and the purpose together across an em dash,
/// and offered four unnamed raw combo boxes with egui's own triangle.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_defaults_speak_in_sentence_case_through_named_selects() {
    use egui::accesskit::Role;

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    open_drawing_sheet_defaults(
        &mut app.state.sheets.drawing_sheet_defaults,
        &app.state.model,
    );
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut frame = eframe::Frame::_new_kittest();
    let mut output = None;
    for index in 0..4u32 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                time: Some(f64::from(index) * 0.25),
                focused: true,
                ..Default::default()
            },
            |ui| eframe::App::ui(&mut app, ui, &mut frame),
        ));
    }
    let output = output.expect("frames ran");
    let tree = output
        .platform_output
        .accesskit_update
        .clone()
        .expect("an AccessKit tree");
    let painted = output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some((
                text.galley.text().to_owned(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
            )),
            _ => None,
        })
        .collect::<Vec<_>>();
    let at = |wanted: &str| {
        painted
            .iter()
            .find(|(text, _)| text == wanted)
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("{wanted:?} was not painted"))
    };
    // What the dialog paints, from its title to its footer: the status bar
    // behind it has dashes of its own.
    let surface = at("Drawing-sheet defaults")
        .union(at("Save sheet defaults"))
        .expand2(egui::vec2(16.0, 0.0));
    let texts = painted
        .iter()
        .filter(|(_, rect)| surface.contains_rect(*rect))
        .map(|(text, _)| text.clone())
        .collect::<Vec<_>>();
    for wanted in [
        "Project default",
        "Personal default",
        "New sheets",
        "Custom",
        "Neither default has changed.",
    ] {
        assert!(
            texts.iter().any(|text| text == wanted),
            "{wanted:?} was not painted: {texts:?}"
        );
    }
    for gone in [
        "PROJECT DEFAULT",
        "PERSONAL DEFAULT",
        "NEW SHEETS",
        "CUSTOM",
    ] {
        assert!(
            texts.iter().all(|text| text != gone),
            "{gone:?} is still painted"
        );
    }
    assert!(
        texts.iter().all(|text| !text.contains('\u{2014}')),
        "a dash is still painted: {texts:?}"
    );
    for select in ["New sheets", "Display units", "Border", "Title block"] {
        assert!(
            tree.nodes
                .iter()
                .any(|(_, node)| node.role() == Role::ComboBox && node.label() == Some(select)),
            "no select is named {select:?}"
        );
    }
}
