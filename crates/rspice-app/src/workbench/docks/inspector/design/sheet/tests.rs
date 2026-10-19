//! The sheet inspector edits the sheet in place and repeats nothing.

use std::collections::HashMap;

use super::super::tests::{accesskit_nodes, painted_panel};
use super::*;
use crate::state::{
    DrawingSheetStandard, SchematicPageOrientation, SheetDefinition, SheetPortPolicy, SheetTemplate,
};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;

/// The sheet panel rendered at the dock's width, driven by real input.
struct SheetPanel {
    ctx: egui::Context,
    app: RSpiceApp,
    controls: HashMap<String, egui::Rect>,
}

impl SheetPanel {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        Self {
            ctx,
            app: RSpiceApp::test_instance(),
            controls: HashMap::new(),
        }
    }

    fn pass(&mut self, events: Vec<egui::Event>) {
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(320.0, 1_400.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| sheet_panel(ui, &mut self.app.state));
            },
        );
        if let Some(update) = output.platform_output.accesskit_update {
            for (_, node) in update.nodes {
                if node.role() == egui::accesskit::Role::TextInput
                    && let (Some(label), Some(bounds)) = (node.label(), node.bounds())
                {
                    self.controls.insert(
                        label.to_owned(),
                        egui::Rect::from_min_max(
                            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                        ),
                    );
                }
            }
        }
    }

    fn click(&mut self, label: &str) {
        let pos = self.controls[label].center();
        self.pass(vec![
            egui::Event::PointerMoved(pos),
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]);
        self.pass(vec![egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        }]);
    }

    /// Click into the field and type over its value, without leaving it.
    fn type_into(&mut self, label: &str, text: &str) {
        self.pass(Vec::new());
        self.click(label);
        self.pass(vec![
            key(egui::Key::A, egui::Modifiers::COMMAND),
            egui::Event::Text(text.to_owned()),
        ]);
    }

    fn title_value(&self, field: DrawingSheetTitleFieldId) -> String {
        resolve_active_drawing_sheet(&self.app.state)
            .format
            .title_block
            .fields[&field]
            .value
            .clone()
    }

    /// Type a value into the field and press Enter.
    fn enter(&mut self, label: &str, text: &str) {
        self.type_into(label, text);
        self.pass(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
        self.pass(Vec::new());
    }

    fn document_value(&self, field: DrawingSheetTitleFieldId) -> String {
        let format = resolve_active_drawing_sheet(&self.app.state).format;
        stored_title_value(&self.app.state, &format, field)
    }
}

fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    }
}

/// With nothing selected the panel is the sheet's property editor: its size,
/// orientation, where its format comes from, its frame and units, its title
/// block and the optional boxes it shows, and every value its title block
/// shows that someone types, editable where they stand — this sheet's own,
/// then the document's, each in the order ISO 7200's form is read and named
/// as the form names it. An empty value shows what the block letters
/// instead. The toolbar's Check cellview is not repeated as a button, and a
/// check never run adds no section.
#[test]
fn with_nothing_selected_the_panel_edits_the_sheet_and_repeats_no_command() {
    let mut app = RSpiceApp::test_instance();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    // Each editable value paints its hint and then its (empty) text.
    assert_eq!(
        painted,
        "Sheet\nSize\nISO A4 · 297 × 210 mm\nOrientation\nLandscape\n\
         Format from\nProject default\nFrame\nISO 5457 frame\nZones\n6 × 4\n\
         Centring marks\nDrawn\nUnits\nMillimetres\n\
         Title block\nTemplate\nISO 7200\nOptional boxes\n3 of 6 shown\n\
         Created by\n\nApproved by\n\nDocument type\nCircuit diagram\n\nTitle\ntop\n\n\
         Document\nAll sheets\nLegal owner\n\nDocument status\nIn preparation\n\
         Identification number\n\nRevision\n\nDate of issue\nYYYY-MM-DD\n\n\
         Change reference\nECO or ticket\n\n\
         Schematic\nGrid\n2.5 mm\nNet names\nA\u{2013}Z a\u{2013}z 0\u{2013}9 _.$:/![]<>-\n"
    );
}

/// A sheet chooses ISO 7200's form, ASME Y14.1's on an ANSI or ARCH sheet or
/// one already on it, or no block.
#[test]
fn a_sheet_is_offered_its_standards_blocks_or_none() {
    use DrawingSheetTitleBlockTemplate as Template;
    let on = |standard, template| {
        SchematicSheetFormat::from_standard(standard, SchematicPageOrientation::Landscape)
            .try_update(|draft| draft.title_block.template = template)
            .unwrap()
    };
    let a4 = DrawingSheetStandard::IsoA4;
    assert_eq!(
        template_choices(&on(a4, Template::Standard)),
        [Template::Standard, Template::None]
    );
    assert_eq!(
        template_choices(&on(a4, Template::None)),
        [Template::Standard, Template::None]
    );
    assert_eq!(
        template_choices(&on(a4, Template::AsmeY141)),
        [Template::Standard, Template::AsmeY141, Template::None]
    );
    for standard in [DrawingSheetStandard::AnsiB, DrawingSheetStandard::ArchD] {
        assert_eq!(
            template_choices(&on(standard, Template::AsmeY141)),
            [Template::Standard, Template::AsmeY141, Template::None]
        );
        assert_eq!(
            template_choices(&on(standard, Template::Standard)),
            [Template::Standard, Template::AsmeY141, Template::None]
        );
    }
    // The ASME form is named for the standard, whatever size of block the
    // sheet carries.
    assert_eq!(
        template_name(
            &on(DrawingSheetStandard::AnsiD, Template::AsmeY141),
            Template::AsmeY141
        ),
        "ASME Y14.1"
    );
}

/// A revision index ISO 7200 advises against is flagged where it is typed,
/// and kept.
#[test]
fn a_revision_index_with_an_o_is_flagged_and_kept() {
    let mut panel = SheetPanel::new();
    panel.enter("Revision", "O");
    assert_eq!(
        panel.document_value(DrawingSheetTitleFieldId::Revision),
        "O"
    );
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(
        painted.contains(
            "Revision\nO\nISO 7200 advises against I and O in a revision index: they read as \
             1 and 0.\n"
        ),
        "{painted}"
    );
}

/// On the ASME form a revision is held to ASME Y14.35's sequence instead,
/// and flagged, not refused, outside it.
#[test]
fn a_revision_outside_y14_35_is_flagged_and_kept_on_an_asme_sheet() {
    let mut panel = SheetPanel::new();
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::AnsiB),
    )
    .unwrap();
    panel.enter("Revision", "S");
    assert_eq!(
        panel.document_value(DrawingSheetTitleFieldId::Revision),
        "S"
    );
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(
        painted.contains(
            "Revision\nS\nASME Y14.35 letters a revision in capitals from A, without I, O, Q, \
             S, X or Z.\n"
        ),
        "{painted}"
    );
    panel.enter("Revision", "B");
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(!painted.contains("Y14.35"), "{painted}");
}

/// The document's values every sheet letters — owner, number, revision —
/// are typed here too, and each lands on the project as its own undo step
/// without disturbing the others.
#[test]
fn document_values_typed_in_the_panel_land_on_the_project() {
    let mut panel = SheetPanel::new();
    panel.enter("Legal owner", "Whitfield Analog Devices");
    panel.enter("Identification number", "RS-0001-SCH-0042");
    panel.enter("Revision", "B");

    let settings = panel
        .app
        .state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .clone();
    assert_eq!(
        settings.title_block_field_values[&DrawingSheetTitleFieldId::Organization],
        "Whitfield Analog Devices"
    );
    assert_eq!(
        settings.title_block_field_values[&DrawingSheetTitleFieldId::DocumentId],
        "RS-0001-SCH-0042"
    );
    assert_eq!(settings.document_control.revision, "B");

    assert!(panel.app.state.undo_project_design().unwrap().is_some());
    assert_eq!(panel.document_value(DrawingSheetTitleFieldId::Revision), "");
    assert!(panel.app.state.undo_project_design().unwrap().is_some());
    assert_eq!(
        panel.document_value(DrawingSheetTitleFieldId::DocumentId),
        ""
    );
    assert_eq!(
        panel.document_value(DrawingSheetTitleFieldId::Organization),
        "Whitfield Analog Devices"
    );
}

/// A date the document control cannot read is refused in its words and
/// changes nothing; the release status is chosen from its list.
#[test]
fn the_document_date_and_status_are_held_to_the_document_controls_rules() {
    let mut panel = SheetPanel::new();
    panel.enter("Date of issue", "18/10/2026");
    assert_eq!(panel.document_value(DrawingSheetTitleFieldId::Date), "");
    assert_eq!(
        panel.app.state.prompt_text(),
        Some("The date must be a calendar date written YYYY-MM-DD.")
    );
    assert!(!panel.app.state.can_undo_project_design());

    let refused = apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::Status(DrawingSheetReleaseStatus::Released),
    )
    .unwrap_err();
    assert_eq!(
        refused,
        "A released or withdrawn document needs a revision."
    );

    panel.enter("Revision", "A");
    panel.enter("Date of issue", "2026-10-18");
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::Status(DrawingSheetReleaseStatus::Released),
    )
    .unwrap();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(painted.contains("Document status\nReleased\n"), "{painted}");
    assert!(
        painted.contains("Revision\nA\nDate of issue\n2026-10-18\n"),
        "{painted}"
    );
}

/// On an ANSI sheet the block follows the size to ASME Y14.1's form, and the
/// panel offers every value its approvals block letters: the engineer and
/// each signature's date on the sheet, the CAGE code and contract number on
/// the project. A signature date is held to the document date's form.
#[test]
fn the_asme_forms_approvals_are_typed_in_the_panel() {
    let mut panel = SheetPanel::new();
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::AnsiB),
    )
    .unwrap();
    let format = resolve_active_drawing_sheet(&panel.app.state).format;
    assert_eq!(
        format.title_block.template,
        DrawingSheetTitleBlockTemplate::AsmeY141
    );
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(painted.contains("Template\nASME Y14.1\n"), "{painted}");
    for label in [
        "Created by",
        "Created date",
        "Checked by",
        "Checked date",
        "Engineer",
        "Engineer date",
        "Approved by",
        "Approved date",
        "Contract number",
        "CAGE code",
    ] {
        assert!(
            painted.contains(&format!("{label}\n")),
            "{label}: {painted}"
        );
    }
    // The ASME form has no document type.
    assert!(!painted.contains("Document type"), "{painted}");

    panel.enter("Engineer", "R. Okafor");
    panel.enter("Engineer date", "2026-10-18");
    panel.enter("Created date", "18/10/2026");
    assert_eq!(
        panel.app.state.prompt_text(),
        Some("The date must be a calendar date written YYYY-MM-DD.")
    );
    panel.enter("CAGE code", "1ABC2");
    panel.enter("Contract number", "N00024-26-C-0001");
    assert_eq!(
        panel.title_value(DrawingSheetTitleFieldId::Engineer),
        "R. Okafor"
    );
    assert_eq!(
        panel.title_value(DrawingSheetTitleFieldId::EngineerDate),
        "2026-10-18"
    );
    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnDate), "");
    assert_eq!(
        panel.document_value(DrawingSheetTitleFieldId::CageCode),
        "1ABC2"
    );
    assert_eq!(
        panel.document_value(DrawingSheetTitleFieldId::ContractNumber),
        "N00024-26-C-0001"
    );
}

/// The ASME form letters no date of issue, yet a release is refused without
/// one, so the document's revision, date and status are offered on every
/// block that has a document section and an ASME sheet can be released.
#[test]
fn an_asme_sheet_offers_the_release_record_its_block_does_not_letter() {
    let mut panel = SheetPanel::new();
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::AnsiB),
    )
    .unwrap();
    let format = resolve_active_drawing_sheet(&panel.app.state).format;
    assert!(!format.title_field_shown(DrawingSheetTitleFieldId::Date));
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    for label in ["Document status\n", "Revision\n", "Date of issue\n"] {
        assert!(painted.contains(label), "{label}: {painted}");
    }

    panel.enter("Revision", "A");
    panel.enter("Date of issue", "2026-10-18");
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::Status(DrawingSheetReleaseStatus::Released),
    )
    .unwrap();
    assert_eq!(
        panel.document_value(DrawingSheetTitleFieldId::Date),
        "2026-10-18"
    );
}

/// Fields the sheet does not show offer nothing to type, and a block with no
/// fields at all has no document section.
#[test]
fn only_the_fields_this_block_shows_are_offered() {
    let mut app = RSpiceApp::test_instance();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(!painted.contains("Classification"), "{painted}");
    assert!(!painted.contains("Checked by"), "{painted}");
    assert!(!painted.contains("Language"), "{painted}");
    // RSpice fills in the sheet number itself, so the only "Sheet" is the
    // section's own heading.
    assert_eq!(painted.matches("Sheet\n").count(), 1, "{painted}");

    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::TitleBlock(DrawingSheetTitleBlockTemplate::None),
    )
    .unwrap();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(!painted.contains("Document\n"), "{painted}");
    assert!(!painted.contains("Revision"), "{painted}");
}

/// The grid and net-name characters belong to the open schematic, not to
/// Preferences: each change lands as one undo step, and a new grid moves the
/// snap with it.
#[test]
fn the_open_schematics_grid_and_net_names_change_in_the_panel() {
    use crate::state::{NetNamingPolicy, SchematicGridPitch};

    let mut app = RSpiceApp::test_instance();
    apply_schematic_edit(
        &mut app.state,
        SchematicEdit::Grid(SchematicGridPitch::Mil25),
    );
    apply_schematic_edit(
        &mut app.state,
        SchematicEdit::NetNameCharacters(NetNamingPolicy::SpiceCompatibleRelaxed),
    );
    let schematic = &app.state.schematic;
    let policy = &schematic.document().document_policy;
    assert_eq!(policy.grid_pitch, SchematicGridPitch::Mil25);
    assert_eq!(policy.net_naming, NetNamingPolicy::SpiceCompatibleRelaxed);
    assert_eq!(schematic.document().grid_size, 5);
    assert_eq!(schematic.session.editor.snap_engine.grid_size, 5);
    assert_eq!(app.state.ui.schematic_snap.grid_size, 5);
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(
        painted.ends_with("Grid\n1.25 mm\nNet names\nAny printable but spaces\n"),
        "{painted}"
    );

    assert!(app.state.schematic.undo());
    let policy = &app.state.schematic.document().document_policy;
    assert_eq!(policy.net_naming, NetNamingPolicy::StrictCaseSensitive);
    assert_eq!(policy.grid_pitch, SchematicGridPitch::Mil25);
}

/// A typed title value lands when the field is left, as one undo step; the
/// keystrokes before it are not transactions.
#[test]
fn a_title_value_typed_and_entered_lands_on_the_sheet_as_one_undo_step() {
    let mut panel = SheetPanel::new();
    panel.type_into("Created by", "J. Whitfield");
    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnBy), "");
    assert!(!panel.app.state.can_undo_project_design());

    panel.pass(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
    assert_eq!(
        panel.title_value(DrawingSheetTitleFieldId::DrawnBy),
        "J. Whitfield"
    );

    assert!(panel.app.state.undo_project_design().unwrap().is_some());
    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnBy), "");
    assert!(!panel.app.state.can_undo_project_design());
}

#[test]
fn escape_abandons_a_typed_title_value() {
    let mut panel = SheetPanel::new();
    panel.type_into("Created by", "J. Whitfield");
    panel.pass(vec![key(egui::Key::Escape, egui::Modifiers::NONE)]);
    panel.pass(Vec::new());
    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnBy), "");
    assert!(!panel.app.state.can_undo_project_design());
}

/// Selecting something on the canvas mid-edit swaps the panel in the same
/// frame; what was typed still lands on the sheet it was typed on.
#[test]
fn a_value_still_being_typed_lands_when_the_panel_moves_on() {
    let mut panel = SheetPanel::new();
    panel.type_into("Created by", "J. Whitfield");
    commit_pending_sheet_text(&panel.ctx, &mut panel.app.state);
    assert_eq!(
        panel.title_value(DrawingSheetTitleFieldId::DrawnBy),
        "J. Whitfield"
    );
}

/// A value typed on one sheet is dropped, not written, once another sheet is
/// active.
#[test]
fn a_value_typed_on_one_sheet_never_lands_on_another() {
    let mut panel = SheetPanel::new();
    let key = panel.app.state.workspace.content.active_key();
    let designs = &mut panel.app.state.workspace.content.design_management;
    let first = designs
        .bootstrap_for_cell_view(&key, "Sheet 1", [])
        .unwrap();
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
    catalog.set_active(first).unwrap();

    panel.type_into("Created by", "J. Whitfield");
    panel
        .app
        .state
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .unwrap()
        .set_active(second)
        .unwrap();
    commit_pending_sheet_text(&panel.ctx, &mut panel.app.state);

    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnBy), "");
    assert!(!panel.app.state.can_undo_project_design());
}

/// The size menu names each standard with its oriented size, and a choice
/// lands on the sheet at once.
#[test]
fn a_size_chosen_in_the_panel_lands_on_the_sheet() {
    let mut app = RSpiceApp::test_instance();
    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::IsoA3),
    )
    .unwrap();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(
        painted.contains("Size\nISO A3 · 420 × 297 mm\n"),
        "{painted}"
    );
}

/// A read-only sheet states the same properties, with nothing to operate.
#[test]
fn a_read_only_sheet_states_its_properties_without_controls() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );
    assert!(app.state.schematic_edit_read_only());

    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(painted.starts_with("Sheet\nSize\nISO A4"), "{painted}");
    let controls = accesskit_nodes(|ui| sheet_panel(ui, &mut app.state))
        .into_iter()
        .filter(|(_, node)| {
            matches!(
                node.role(),
                egui::accesskit::Role::ComboBox | egui::accesskit::Role::TextInput
            )
        })
        .count();
    assert_eq!(controls, 0);
}

/// Checks add a section only for this sheet's findings, and never the
/// toolbar's Check cellview button.
#[test]
fn checks_appear_only_for_findings_on_this_sheet() {
    let mut app = RSpiceApp::test_instance();
    crate::workbench::examples::load_example("RC Lowpass Filter", &mut app.state.schematic);
    Command::CheckCellview.execute(&mut app);
    let clean = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(!clean.contains("Checks"), "{clean}");

    app.state.schematic.document_mut_for_test().wires.pop();
    Command::CheckCellview.execute(&mut app);
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(painted.contains("Checks\n"), "{painted}");
    assert!(painted.contains("Connectivity\n"), "{painted}");
    assert!(!painted.contains("Check cellview"), "{painted}");
}

fn sheet_format(state: &AppState) -> SchematicSheetFormat {
    resolve_active_drawing_sheet(state).format
}

/// A size the standard list does not hold is typed where the size is: the
/// sheet turns to a one-off size of its own edges, each edge lands as one
/// undo step, the edges stand the sheet the way they are typed, and the size
/// is named and saved to the project's custom sizes from the same rows.
#[test]
fn a_custom_size_is_typed_in_the_panel_and_saved_to_custom_sizes() {
    let mut panel = SheetPanel::new();
    let (width_um, height_um) = sheet_format(&panel.app.state).oriented_dimensions_um();
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::CustomSize {
            width_um,
            height_um,
        },
    )
    .unwrap();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(
        painted.contains(
            "Size\nCustom size\nWidth (mm)\n297\nHeight (mm)\n210\n\
             Size name\nOne-off custom size\nSave as project size\nSave as personal size\n"
        ),
        "{painted}"
    );

    panel.enter("Width (mm)", "250");
    panel.enter("Height (mm)", "400");
    let format = sheet_format(&panel.app.state);
    assert_eq!(format.oriented_dimensions_um(), (250_000, 400_000));
    assert_eq!(format.orientation, SchematicPageOrientation::Portrait);
    assert!(panel.app.state.undo_project_design().unwrap().is_some());
    assert_eq!(
        sheet_format(&panel.app.state).oriented_dimensions_um(),
        (250_000, 210_000)
    );
    panel.enter("Height (mm)", "400");

    panel.enter("Size name", "Lab panel");
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::SaveCustomSize(crate::state::DrawingSheetPresetScope::Project),
    )
    .unwrap();
    let settings = panel
        .app
        .state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .clone();
    assert_eq!(settings.presets.len(), 1);
    assert_eq!(settings.presets[0].name, "Lab panel");
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(
        painted.contains("Size\nLab panel · 250 × 400 mm\nOrientation\nPortrait\n"),
        "{painted}"
    );
    assert!(!painted.contains("Width (mm)"), "{painted}");
}

/// An edge the sheet cannot have is refused in the words that say why, and
/// changes nothing.
#[test]
fn an_edge_the_sheet_cannot_have_is_refused_in_its_words() {
    let mut panel = SheetPanel::new();
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::CustomSize {
            width_um: 297_000,
            height_um: 210_000,
        },
    )
    .unwrap();
    let before = sheet_format(&panel.app.state);
    panel.enter("Width (mm)", "twelve");
    assert_eq!(
        panel.app.state.prompt_text(),
        Some("Use a decimal physical dimension.")
    );
    panel.enter("Width (mm)", "3000");
    assert_eq!(
        panel.app.state.prompt_text(),
        Some("Sheet edges must be between 50.8 mm and 2540 mm.")
    );
    assert_eq!(sheet_format(&panel.app.state), before);
}

/// The frame of the sheet's standard, its zones, its marks and the unit its
/// sizes are stated in are chosen in the panel, each as one undo step. A
/// sheet without a frame offers neither zones nor marks.
#[test]
fn the_frame_its_zones_its_marks_and_the_units_are_chosen_in_the_panel() {
    let mut app = RSpiceApp::test_instance();
    for edit in [
        DrawingSheetEdit::Zones(false),
        DrawingSheetEdit::CentringMarks(false),
        DrawingSheetEdit::Units(crate::state::DrawingSheetDisplayUnit::Inches),
    ] {
        apply_drawing_sheet_edit(&mut app.state, edit).unwrap();
    }
    let format = sheet_format(&app.state);
    assert_eq!(format.zones.mode, crate::state::DrawingSheetZoneMode::None);
    assert!(!format.marks.centring);
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(
        painted
            .contains("Frame\nISO 5457 frame\nZones\nNone\nCentring marks\nNone\nUnits\nInches\n"),
        "{painted}"
    );
    assert!(
        painted.contains("Size\nISO A4 · 11.69291 × 8.26772 in\n"),
        "{painted}"
    );

    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Frame(crate::state::DrawingSheetBorderTemplate::None),
    )
    .unwrap();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(painted.contains("Frame\nNone\nUnits\n"), "{painted}");
    assert!(!painted.contains("Zones"), "{painted}");
    assert!(
        apply_drawing_sheet_edit(&mut app.state, DrawingSheetEdit::Zones(true)).is_err(),
        "a sheet without a frame has no zones"
    );

    for _ in 0..4 {
        assert!(app.state.undo_project_design().unwrap().is_some());
    }
    assert_eq!(
        sheet_format(&app.state).zones.mode,
        crate::state::DrawingSheetZoneMode::Automatic
    );
}

/// An ANSI sheet takes ASME Y14.1's frame: its marks are microfilm arrows,
/// and the binding margin its standard allows is chosen beside them.
#[test]
fn an_asme_sheet_offers_its_arrows_and_its_binding_margin() {
    let mut app = RSpiceApp::test_instance();
    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::AnsiB),
    )
    .unwrap();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(
        painted.contains(
            "Frame\nASME Y14.1 border\nZones\n4 × 2\nMicrofilm arrows\nNone\n\
             Binding margin\nNone\n"
        ),
        "{painted}"
    );
    apply_drawing_sheet_edit(&mut app.state, DrawingSheetEdit::BindingMargin(true)).unwrap();
    let format = sheet_format(&app.state);
    assert!(format.binding_margin);
    assert_eq!(format.margins().left_um, 19_050);
}

/// A sheet follows the project default until it is given a format of its
/// own, and can be put back on the default from the same row.
#[test]
fn a_sheet_follows_the_project_default_until_it_takes_its_own_format() {
    let mut app = RSpiceApp::test_instance();
    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::Size(DrawingSheetStandard::IsoA3),
    )
    .unwrap();
    let format = sheet_format(&app.state);
    assert_eq!(
        format.inheritance,
        crate::state::DrawingSheetInheritance::Explicit
    );
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(painted.contains("Format from\nThis sheet\n"), "{painted}");

    apply_drawing_sheet_edit(&mut app.state, DrawingSheetEdit::FollowProjectDefault(true)).unwrap();
    let format = sheet_format(&app.state);
    assert_eq!(
        format.inheritance,
        crate::state::DrawingSheetInheritance::ProjectDefault
    );
    assert_eq!(format.authored_size.label(), "ISO A4");
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(
        painted.contains("Format from\nProject default\n"),
        "{painted}"
    );
}

/// The boxes a form leaves optional are shown and hidden from one row, and a
/// box shown offers its value where its kind of value is typed: Checked by
/// on the sheet, the language on the document. A box the form requires
/// stays.
#[test]
fn the_optional_boxes_are_shown_and_hidden_in_the_panel() {
    let mut panel = SheetPanel::new();
    for edit in [
        DrawingSheetEdit::BoxShown(DrawingSheetTitleFieldId::CheckedBy, true),
        DrawingSheetEdit::BoxShown(DrawingSheetTitleFieldId::Language, true),
        DrawingSheetEdit::BoxShown(DrawingSheetTitleFieldId::Revision, false),
    ] {
        apply_drawing_sheet_edit(&mut panel.app.state, edit).unwrap();
    }
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(
        painted.contains("Optional boxes\n4 of 6 shown\nCreated by\n\nChecked by\n\n"),
        "{painted}"
    );
    assert!(
        painted.contains("Date of issue\nYYYY-MM-DD\n\nLanguage\n\n"),
        "{painted}"
    );
    // The revision leaves the block, not the document: a release still
    // needs one, so it is still typed with the document's values.
    assert!(painted.contains("Revision\n\nDate of issue\n"), "{painted}");
    assert!(
        !resolve_active_drawing_sheet(&panel.app.state)
            .format
            .title_field_shown(DrawingSheetTitleFieldId::Revision)
    );

    panel.enter("Language", "en");
    assert_eq!(
        panel.document_value(DrawingSheetTitleFieldId::Language),
        "en"
    );
    assert_eq!(
        apply_drawing_sheet_edit(
            &mut panel.app.state,
            DrawingSheetEdit::BoxShown(DrawingSheetTitleFieldId::DrawnBy, false),
        )
        .unwrap_err(),
        "ISO 7200 makes this field mandatory on its form."
    );
}

/// The change order a revision answers to is the document's, typed with the
/// rest of its values.
#[test]
fn the_change_reference_is_typed_with_the_documents_values() {
    let mut panel = SheetPanel::new();
    panel.enter("Change reference", "ECO-1042");
    assert_eq!(
        panel
            .app
            .state
            .workspace
            .content
            .design_management
            .drawing_sheet_settings()
            .document_control
            .change_reference,
        "ECO-1042"
    );
    assert!(panel.app.state.undo_project_design().unwrap().is_some());
}

/// What a size leaves outside the drawing area is counted under the size,
/// with the way to the first object and to the review of them all.
#[test]
fn content_outside_the_drawing_area_is_counted_under_the_size() {
    let mut app = RSpiceApp::test_instance();
    // 12 mm from the left edge: on the paper, outside ISO 5457's 20 mm
    // filing border.
    app.state
        .schematic
        .document_mut_for_test()
        .wires
        .push(crate::state::Wire::new(
            31,
            vec![
                crate::state::Point::new(-92, 100),
                crate::state::Point::new(-88, 100),
            ],
        ));
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(
        painted.contains(
            "Size\nISO A4 · 297 × 210 mm\n\
             1 object outside the drawing area: 1 outside the border\n"
        ),
        "{painted}"
    );
    assert!(painted.contains("Review all…\nOrientation\n"), "{painted}");
}

/// Lettering never shrinks to fit, so a value longer than its cell prints cut
/// short, and its row says so. An ANSI A sheet bound with the binding margin
/// steps down to the form's 6.25 in block, whose Created by cell holds a
/// twelve-character name and not a twenty-five-character one.
#[test]
fn a_value_the_block_cuts_short_says_so_under_its_row() {
    use crate::state::DRAWING_SHEET_TITLE_VALUE_CUT_ADVISORY as CUT;

    let mut panel = SheetPanel::new();
    for edit in [
        DrawingSheetEdit::Size(DrawingSheetStandard::AnsiA),
        DrawingSheetEdit::BindingMargin(true),
    ] {
        apply_drawing_sheet_edit(&mut panel.app.state, edit).unwrap();
    }

    panel.enter("Created by", "J. Whitfield");
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(painted.contains("Created by\nJ. Whitfield\n"), "{painted}");
    assert!(!painted.contains(CUT), "{painted}");

    panel.enter("Created by", "Christopher Featherstones");
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(
        painted.contains(&format!("Created by\nChristopher Featherstones\n{CUT}\n")),
        "{painted}"
    );
    assert_eq!(painted.matches(CUT).count(), 1, "{painted}");
}

/// Sheet properties ask the panel to bring its Sheet section into view, and
/// the panel does so the next time it shows, once.
#[test]
fn the_panel_brings_its_sheet_section_into_view_once_when_asked() {
    let mut app = RSpiceApp::test_instance();
    crate::workbench::app::reveal_sheet_properties(&mut app.state);
    assert!(app.state.workbench.reveal_sheet_section);
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(painted.starts_with("Sheet\nSize\n"), "{painted}");
    assert!(!app.state.workbench.reveal_sheet_section);
}

/// At the inspector's default width the panel stays inside the inspector: a
/// choice longer than its field is cut to the field rather than widening
/// every row after it past the edge, and an ISO sheet's words and a custom
/// size's fit whole.
#[test]
fn the_panel_fits_the_inspectors_default_width() {
    fn walk(shape: &egui::epaint::Shape, cut: &mut Vec<String>) {
        match shape {
            egui::epaint::Shape::Text(text) if text.galley.elided => {
                cut.push(text.galley.job.text.clone());
            }
            egui::epaint::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, cut)),
            _ => {}
        }
    }
    let iso_a3 = vec![DrawingSheetEdit::Size(DrawingSheetStandard::IsoA3)];
    let ansi_d = vec![
        DrawingSheetEdit::Size(DrawingSheetStandard::AnsiD),
        DrawingSheetEdit::Units(crate::state::DrawingSheetDisplayUnit::Inches),
    ];
    let custom = vec![
        DrawingSheetEdit::CustomSize {
            width_um: 500_000,
            height_um: 350_000,
        },
        DrawingSheetEdit::CustomSizeName("Lab panel".to_owned()),
    ];
    for (edits, whole) in [(iso_a3, true), (ansi_d, false), (custom, true)] {
        let mut app = RSpiceApp::test_instance();
        for edit in edits {
            apply_drawing_sheet_edit(&mut app.state, edit).unwrap();
        }
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut rows = egui::Rect::NOTHING;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(312.0, 1_400.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        sheet_panel(ui, &mut app.state);
                        rows = ui.min_rect();
                    });
            },
        );
        assert!(rows.right() <= 312.0, "the rows ran to {}", rows.right());
        let mut cut = Vec::new();
        for clipped in &output.shapes {
            walk(&clipped.shape, &mut cut);
        }
        assert!(!whole || cut.is_empty(), "cut short: {cut:?}");
    }
}
