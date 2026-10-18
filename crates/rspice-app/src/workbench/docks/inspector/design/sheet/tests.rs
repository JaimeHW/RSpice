//! The sheet inspector edits the sheet in place and repeats nothing.

use std::collections::HashMap;

use super::super::tests::{accesskit_nodes, painted_panel};
use super::*;
use crate::state::{DrawingSheetStandard, SheetDefinition, SheetPortPolicy, SheetTemplate};
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
                    egui::vec2(320.0, 720.0),
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
/// orientation and title block, and every value its title block shows that
/// someone types, editable where they stand — this sheet's own, then the
/// document's. An empty value shows what the block letters instead. The
/// toolbar's Check cellview and the File menu's Page setup are not repeated
/// as buttons, and a check never run adds no section.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn with_nothing_selected_the_panel_edits_the_sheet_and_repeats_no_command() {
    let mut app = RSpiceApp::test_instance();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    // Each editable value paints its hint and then its (empty) text.
    assert_eq!(
        painted,
        "Sheet\nSize\nISO A4 · 297 × 210 mm\nOrientation\nLandscape\n\
         Title block\nTemplate\nRSpice standard\n\
         Sheet title\ntop\n\nDocument type\nCircuit diagram\n\n\
         Drawn by\n\nApproved by\n\n\
         Document\nAll sheets\nRevision\n\nDate\nYYYY-MM-DD\n\n\
         Document status\nDraft\nOrganization\n\nDocument ID\n\n"
    );
}

/// The document's values every sheet letters — owner, number, revision —
/// are typed here too, and each lands on the project as its own undo step
/// without disturbing the others.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn document_values_typed_in_the_panel_land_on_the_project() {
    let mut panel = SheetPanel::new();
    panel.enter("Organization", "Whitfield Analog Devices");
    panel.enter("Document ID", "RS-0001-SCH-0042");
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
    assert!(!panel.app.state.dialogs.drawing_sheet_setup.open);

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

/// A date Page Setup cannot read is refused in its words and changes
/// nothing; the release status is chosen from its list.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_document_date_and_status_are_held_to_page_setups_rules() {
    let mut panel = SheetPanel::new();
    panel.enter("Date", "18/10/2026");
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
    assert_eq!(refused, "A released or obsolete document needs a revision.");

    panel.enter("Revision", "A");
    panel.enter("Date", "2026-10-18");
    apply_drawing_sheet_edit(
        &mut panel.app.state,
        DrawingSheetEdit::Status(DrawingSheetReleaseStatus::Released),
    )
    .unwrap();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut panel.app.state));
    assert!(
        painted.contains("Revision\nA\nDate\n2026-10-18\nDocument status\nReleased\n"),
        "{painted}"
    );
}

/// Fields the sheet does not show offer nothing to type, and a block with no
/// fields at all has no document section.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn only_the_fields_this_block_shows_are_offered() {
    let mut app = RSpiceApp::test_instance();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(!painted.contains("Classification"), "{painted}");
    assert!(!painted.contains("Checked by"), "{painted}");
    assert!(!painted.contains("Page\n"), "{painted}");

    apply_drawing_sheet_edit(
        &mut app.state,
        DrawingSheetEdit::TitleBlock(DrawingSheetTitleBlockTemplate::None),
    )
    .unwrap();
    let painted = painted_panel(|ui| sheet_panel(ui, &mut app.state));
    assert!(!painted.contains("Document\n"), "{painted}");
    assert!(!painted.contains("Revision"), "{painted}");
}

/// A typed title value lands when the field is left, as one undo step; the
/// keystrokes before it are not transactions.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_title_value_typed_and_entered_lands_on_the_sheet_as_one_undo_step() {
    let mut panel = SheetPanel::new();
    panel.type_into("Drawn by", "J. Whitfield");
    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnBy), "");
    assert!(!panel.app.state.can_undo_project_design());

    panel.pass(vec![key(egui::Key::Enter, egui::Modifiers::NONE)]);
    assert_eq!(
        panel.title_value(DrawingSheetTitleFieldId::DrawnBy),
        "J. Whitfield"
    );
    assert!(!panel.app.state.dialogs.drawing_sheet_setup.open);

    assert!(panel.app.state.undo_project_design().unwrap().is_some());
    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnBy), "");
    assert!(!panel.app.state.can_undo_project_design());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn escape_abandons_a_typed_title_value() {
    let mut panel = SheetPanel::new();
    panel.type_into("Drawn by", "J. Whitfield");
    panel.pass(vec![key(egui::Key::Escape, egui::Modifiers::NONE)]);
    panel.pass(Vec::new());
    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnBy), "");
    assert!(!panel.app.state.can_undo_project_design());
}

/// Selecting something on the canvas mid-edit swaps the panel in the same
/// frame; what was typed still lands on the sheet it was typed on.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_value_still_being_typed_lands_when_the_panel_moves_on() {
    let mut panel = SheetPanel::new();
    panel.type_into("Drawn by", "J. Whitfield");
    commit_pending_title_field(&panel.ctx, &mut panel.app.state);
    assert_eq!(
        panel.title_value(DrawingSheetTitleFieldId::DrawnBy),
        "J. Whitfield"
    );
}

/// A value typed on one sheet is dropped, not written, once another sheet is
/// active.
#[cfg(not(target_arch = "wasm32"))]
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

    panel.type_into("Drawn by", "J. Whitfield");
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
    commit_pending_title_field(&panel.ctx, &mut panel.app.state);

    assert_eq!(panel.title_value(DrawingSheetTitleFieldId::DrawnBy), "");
    assert!(!panel.app.state.can_undo_project_design());
}

/// The size menu names each standard as Page Setup does, and a choice lands
/// on the sheet at once.
#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
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
#[cfg(not(target_arch = "wasm32"))]
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
