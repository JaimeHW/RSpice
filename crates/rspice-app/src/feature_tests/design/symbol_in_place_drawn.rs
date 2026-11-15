//! Editing a symbol in place, driven through whole frames of the application.

use egui::accesskit::Role;

use crate::feature_tests::replay::{Control, Replay};
use crate::state::{CellViewRef, Point, Rotation};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design::commands::DesignCommand;

use super::symbol_in_place::amp_project;

/// A whole application on a sheet placing the `amp` symbol once, unturned
/// at (100, 80), its camera at 200%.
struct InPlace {
    replay: Replay,
    parent: CellViewRef,
    symbol: CellViewRef,
    instance: u64,
}

impl InPlace {
    fn new() -> Self {
        let mut replay = Replay::new();
        let (parent, symbol, instances) = amp_project(
            &mut replay.app,
            &[(Point::new(100, 80), Rotation::R0, false)],
        );
        replay.frames(vec![]);
        Self {
            replay,
            parent,
            symbol,
            instance: instances[0],
        }
    }

    /// Select the instance and edit its symbol in place by the command.
    fn enter(&mut self) -> Vec<Control> {
        self.replay
            .app
            .state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(self.instance);
        self.replay
            .app
            .execute_command_key(DesignCommand::EditSymbolInPlace.key());
        assert_eq!(
            self.replay.app.state.model.workspace.content.active_view,
            self.symbol
        );
        self.replay.frames(vec![])
    }

    fn in_place_zoom(&self) -> f64 {
        crate::workbench::app::in_place_session(
            &self.replay.app.state.design.symbol.in_place,
            &self.replay.app.state.model,
        )
        .expect("the symbol is edited in place")
        .zoom
    }
}

/// Press and let go of the primary button at `at`, a frame each.
fn click_at(replay: &mut Replay, at: egui::Pos2) -> Vec<Control> {
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(at), button(true)]);
    replay.frames(vec![button(false)])
}

/// The centre of the canvas the frame announces.
fn canvas_centre(controls: &[Control]) -> egui::Pos2 {
    controls
        .iter()
        .find(|control| control.role == Role::Canvas)
        .expect("the canvas is announced")
        .center
}

/// The status bar's prompt.
fn prompt(controls: &[Control]) -> String {
    controls
        .iter()
        .find(|control| control.role == Role::Status)
        .map(|control| control.value.clone())
        .unwrap_or_default()
}

/// A line half drawn in place holds the session when the breadcrumb is
/// pressed: the prompt names what is in flight, the line keeps its point,
/// and Escape then cancels the line, as it would anywhere.
#[test]
fn a_shape_in_flight_holds_the_in_place_edit_by_name() {
    let mut fixture = InPlace::new();
    let entered = fixture.enter();
    // The first press gives the canvas the keyboard; the second, with the
    // line tool armed, sets the line's first point.
    click_at(&mut fixture.replay, canvas_centre(&entered));
    fixture.replay.key(egui::Key::L, egui::Modifiers::NONE);
    click_at(&mut fixture.replay, canvas_centre(&entered));
    assert_eq!(
        fixture
            .replay
            .app
            .state
            .design
            .symbol
            .editor
            .draft_activity(),
        Some("a shape is being drawn")
    );

    let after = fixture.replay.click("top", Role::Link);
    assert_eq!(
        fixture.replay.app.state.model.workspace.content.active_view, fixture.symbol,
        "the breadcrumb did not leave"
    );
    assert_eq!(
        fixture
            .replay
            .app
            .state
            .design
            .symbol
            .editor
            .pending_polyline
            .len(),
        1,
        "the line kept its point"
    );
    assert!(
        prompt(&after).contains("a shape is being drawn"),
        "{}",
        prompt(&after)
    );

    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    assert_eq!(
        fixture
            .replay
            .app
            .state
            .design
            .symbol
            .editor
            .draft_activity(),
        None,
        "Escape cancelled the line"
    );
    fixture.replay.click("top", Role::Link);
    assert_eq!(
        fixture.replay.app.state.model.workspace.content.active_view, fixture.parent,
        "with nothing in flight the breadcrumb leaves"
    );
}

/// The status bar's zoom chip: the one button whose name is a percentage.
fn zoom_chip(controls: &[Control]) -> String {
    let chips: Vec<&str> = controls
        .iter()
        .filter(|control| control.role == Role::Button && control.label.ends_with('%'))
        .map(|control| control.label.as_str())
        .collect();
    assert_eq!(chips.len(), 1, "one zoom chip: {chips:?}");
    chips[0].to_owned()
}

/// The zoom chip reads the canvas in front. Edited in place, that is the
/// parent sheet's camera the session carries, not the zoom the symbol's own
/// view was last left at, and it follows the in-place view as it zooms.
#[test]
fn the_zoom_chip_reads_the_in_place_view() {
    let mut fixture = InPlace::new();
    let before = fixture.replay.frames(vec![]);
    assert_eq!(zoom_chip(&before), "200%");
    let entered = fixture.enter();
    assert_eq!(zoom_chip(&entered), "200%", "the sheet's own zoom");

    Command::ZoomIn.execute(&mut fixture.replay.app);
    let nearer = fixture.replay.frames(vec![]);
    let zoom = fixture.in_place_zoom();
    assert!(zoom > 2.0, "Zoom in magnified the in-place view");
    assert_eq!(
        zoom_chip(&nearer),
        format!("{}%", (zoom * 100.0).round()),
        "the chip follows the in-place view"
    );
}

/// Every text painted in `output`.
fn painted(output: &egui::FullOutput) -> Vec<String> {
    fn collect(shape: &egui::Shape, into: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => into.push(text.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| collect(shape, into)),
            _ => {}
        }
    }
    let mut texts = Vec::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut texts);
    }
    texts
}

/// The canvas's bounds as the frame announces them.
fn canvas_rect(output: &egui::FullOutput) -> egui::Rect {
    let bounds = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AccessKit is on")
        .nodes
        .iter()
        .find_map(|(_, node)| {
            (node.role() == Role::Canvas)
                .then(|| node.bounds())
                .flatten()
        })
        .expect("the canvas is announced");
    egui::Rect::from_min_max(
        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
    )
}

/// While the symbol is edited in place, every place that names the document
/// says so and names the instance: the status bar, the window title, the
/// inspector's header and the canvas's announced name. Leaving takes it all
/// back to the sheet.
#[test]
fn the_shell_says_the_symbol_is_edited_in_place_and_where() {
    let mut fixture = InPlace::new();
    let project = fixture
        .replay
        .app
        .state
        .model
        .workspace
        .content
        .project
        .display_name()
        .to_owned();
    let entered = fixture.enter();
    let status: Vec<&str> = entered
        .iter()
        .filter(|control| control.role == Role::Label)
        .map(|control| control.value.as_str())
        .collect();
    assert!(
        status.contains(&"In place on X1 \u{00b7} top"),
        "the status bar says where: {status:?}"
    );
    let title = fixture.replay.app.last_window_title.clone();
    assert!(title.starts_with("amp in place on X1"), "{title}");
    assert!(
        title.ends_with(&format!(" \u{2014} {project} \u{2014} RSpice")),
        "{title}"
    );
    let canvas = entered
        .iter()
        .find(|control| control.role == Role::Canvas)
        .expect("the canvas is announced");
    assert!(
        canvas
            .label
            .starts_with("Symbol editor canvas, in place on X1 in top. "),
        "{}",
        canvas.label
    );
    let output = fixture.replay.frame(vec![]);
    assert!(
        painted(&output).contains(&"In place on X1".to_owned()),
        "the inspector's header says where"
    );

    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    let back = fixture.replay.frames(vec![]);
    assert!(
        back.iter()
            .all(|control| !control.value.starts_with("In place on")),
        "the status bar no longer says it"
    );
    let title = fixture.replay.app.last_window_title.clone();
    assert!(title.starts_with("top"), "{title}");
}

/// The placed preview has nothing to add in place, where the symbol already
/// stands placed on its sheet: its toolbar toggle gives way to the way back,
/// and its command says why it is not offered. In the symbol's own view the
/// toggle is there.
#[test]
fn the_placed_preview_is_not_offered_in_place() {
    use rspice_app_workbench::workbench::framework::command_registry::CommandAvailability;

    let mut fixture = InPlace::new();
    let entered = fixture.enter();
    let buttons: Vec<&str> = entered
        .iter()
        .filter(|control| control.role == Role::Button)
        .map(|control| control.label.as_str())
        .collect();
    assert!(
        !buttons.contains(&"Preview as placed on a sheet"),
        "{buttons:?}"
    );
    // The key is lettered as the host writes it: Enter, or ↩ on a Mac.
    let enter = crate::state::shortcuts::ShortcutStroke::new(egui::Key::Enter, false, false, false)
        .display_label_for(egui::os::OperatingSystem::default());
    let back = format!("Return to parent ({enter})");
    assert!(buttons.contains(&back.as_str()), "{buttons:?}");
    assert_eq!(
        fixture
            .replay
            .app
            .command_availability(DesignCommand::SymbolPlacedPreview.key()),
        CommandAvailability::Disabled("the symbol is shown placed on its sheet")
    );

    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    let symbol = fixture.symbol.clone();
    fixture.replay.app.state.open_workspace_view(symbol);
    let own = fixture.replay.frames(vec![]);
    assert!(
        own.iter()
            .any(|control| control.label == "Preview as placed on a sheet"),
        "the symbol's own view offers the preview"
    );
}

/// Shift+X, Virtuoso's Edit In Place key, edits the symbol of the instance
/// the pointer selected in place, as the command does.
#[test]
fn shift_x_edits_the_selected_instance_in_place() {
    let mut fixture = InPlace::new();
    let output = fixture.replay.frame(vec![]);
    // The sheet's camera is at 200%, panned (60, 40): the instance's body
    // centre (100, 80) is at canvas (260, 200).
    let body = canvas_rect(&output).min + egui::vec2(260.0, 200.0);
    click_at(&mut fixture.replay, body);
    assert_eq!(
        fixture
            .replay
            .app
            .state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component(),
        Some(fixture.instance),
        "the click selected the instance"
    );
    fixture.replay.key(egui::Key::X, egui::Modifiers::SHIFT);
    let state = &fixture.replay.app.state;
    assert_eq!(state.model.workspace.content.active_view, fixture.symbol);
    let session =
        crate::workbench::app::in_place_session(&state.design.symbol.in_place, &state.model)
            .expect("Shift+X edits the symbol in place");
    assert_eq!(session.instance_id, fixture.instance);
}

/// Return to parent, the hierarchy's way up, leaves a symbol edited in place
/// as Return does, by its shortcut: the symbol is a level below its sheet.
#[test]
fn return_to_parent_leaves_the_in_place_edit() {
    let mut fixture = InPlace::new();
    fixture.enter();
    fixture.replay.key(egui::Key::H, egui::Modifiers::SHIFT);
    assert_eq!(
        fixture.replay.app.state.model.workspace.content.active_view,
        fixture.parent
    );
    assert!(fixture.replay.app.state.design.symbol.in_place.is_none());
    assert_eq!(
        fixture
            .replay
            .app
            .state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component(),
        Some(fixture.instance)
    );
}

/// Entering brings the instance into view: a sheet whose camera shows the
/// whole instance keeps it exactly, and one that leaves the instance out is
/// framed on it.
#[test]
fn entering_frames_the_instance_only_when_the_sheet_leaves_it_out() {
    let mut fixture = InPlace::new();
    fixture.enter();
    let session = crate::workbench::app::in_place_session(
        &fixture.replay.app.state.design.symbol.in_place,
        &fixture.replay.app.state.model,
    )
    .unwrap();
    assert_eq!(
        (session.zoom, session.pan),
        (2.0, (60.0, 40.0)),
        "a shown instance keeps the sheet's view"
    );
    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);

    fixture.replay.app.state.model.schematic.session.editor.pan = (-4000.0, -4000.0);
    fixture.replay.frames(vec![]);
    fixture.enter();
    let output = fixture.replay.frame(vec![]);
    let canvas = canvas_rect(&output);
    let model = &fixture.replay.app.state.model;
    let session = crate::workbench::app::in_place_session(
        &fixture.replay.app.state.design.symbol.in_place,
        model,
    )
    .unwrap();
    let (min_x, min_y, max_x, max_y) =
        crate::workbench::app::in_place_bounds(session, model).unwrap();
    let screen = |x: f64, y: f64| {
        egui::pos2(
            canvas.min.x + (session.pan.0 + x * session.zoom) as f32,
            canvas.min.y + (session.pan.1 + y * session.zoom) as f32,
        )
    };
    for corner in [screen(min_x, min_y), screen(max_x, max_y)] {
        assert!(
            canvas.expand(1.0).contains(corner),
            "{corner:?} lies on the canvas {canvas:?}"
        );
    }
    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    assert_eq!(
        fixture.replay.app.state.model.schematic.session.editor.pan,
        (-4000.0, -4000.0),
        "leaving gives the sheet its own view back"
    );
}

/// Deleting a pin in place lets go of the wire it held: the wire stays where
/// it was, open, joined to nothing, and the status bar says how many ends
/// were left and where.
#[test]
fn a_deleted_pin_leaves_its_wire_open_and_says_so() {
    let mut fixture = InPlace::new();
    fixture.wire_pin_a();
    let wire = vec![Point::new(60, 70), Point::new(0, 70)];
    fixture.enter();
    // A press on the pin's tip selects it and gives the canvas the keyboard.
    let tip = fixture.screen(Point::new(60, 70));
    click_at(&mut fixture.replay, tip);
    let after = fixture.replay.key(egui::Key::Delete, egui::Modifiers::NONE);
    let document = fixture
        .replay
        .app
        .state
        .model
        .load_symbol_document(&fixture.symbol)
        .unwrap();
    assert!(document.pin("A").is_none(), "the pin is deleted");
    assert_eq!(
        prompt(&after),
        "1 wire end left unconnected where pin A stood."
    );
    let parent = fixture
        .replay
        .app
        .state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&fixture.parent.key())
        .unwrap();
    assert_eq!(parent.document().wires[0].points, wire, "the wire stayed");
}

/// A project file path of its own for one case, and what saving it leaves
/// beside it, removed when the case ends.
struct ProjectPath(std::path::PathBuf);

impl ProjectPath {
    fn new(label: &str) -> Self {
        Self(std::env::temp_dir().join(format!(
            "rspice-in-place-{label}-{}-{}.rspiceproj",
            std::process::id(),
            uuid::Uuid::new_v4()
        )))
    }
}

impl Drop for ProjectPath {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
        let _ = std::fs::remove_file(self.0.with_extension("rspiceproj.bak"));
        let mut lock = self.0.as_os_str().to_os_string();
        lock.push(".rspice.lock");
        let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
    }
}

impl InPlace {
    /// A wire held at the instance's pin A, which the unturned instance at
    /// (100, 80) puts at (60, 70), running left to (0, 70).
    fn wire_pin_a(&mut self) {
        let schematic = &mut self.replay.app.state.model.schematic;
        schematic.document_mut_for_test().wires = vec![crate::state::Wire::new(
            5,
            vec![Point::new(60, 70), Point::new(0, 70)],
        )];
        schematic.recalculate_runtime_state();
        self.replay.app.state.sync_active_schematic_to_workspace();
    }

    /// Where a sheet point stands on screen, through the in-place view.
    fn screen(&mut self, point: Point) -> egui::Pos2 {
        let output = self.replay.frame(vec![]);
        let canvas = canvas_rect(&output);
        let session = crate::workbench::app::in_place_session(
            &self.replay.app.state.design.symbol.in_place,
            &self.replay.app.state.model,
        )
        .expect("the symbol is edited in place");
        egui::pos2(
            canvas.min.x + (session.pan.0 + f64::from(point.x) * session.zoom) as f32,
            canvas.min.y + (session.pan.1 + f64::from(point.y) * session.zoom) as f32,
        )
    }

    /// Drag on the in-place canvas from one sheet point to another.
    fn drag(&mut self, from: Point, to: Point) {
        let (from, to) = (self.screen(from), self.screen(to));
        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.replay
            .frame(vec![egui::Event::PointerMoved(from), button(from, true)]);
        self.replay
            .frame(vec![egui::Event::PointerMoved(from + egui::vec2(8.0, 0.0))]);
        self.replay.frame(vec![egui::Event::PointerMoved(to)]);
        self.replay.frame(vec![button(to, false)]);
        self.replay.frames(vec![]);
    }

    fn pin_a(&self) -> Option<Point> {
        self.replay
            .app
            .state
            .model
            .load_symbol_document(&self.symbol)
            .unwrap()
            .pin("A")
            .and_then(|pin| pin.position)
    }

    /// The first point of the parent's first wire, as the parent holds it.
    fn wire_end(&self) -> Point {
        self.replay
            .app
            .state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&self.parent.key())
            .unwrap()
            .document()
            .wires[0]
            .points[0]
    }
}

/// Leaving the edit by another way than its own exit, here the sheet's tab,
/// ends the session as its exit does: the symbol tab it opened goes, and the
/// sheet is as it was, with the instance selected.
#[test]
fn leaving_by_the_sheet_tab_closes_the_tab_the_session_opened() {
    let mut fixture = InPlace::new();
    fixture.enter();
    fixture.replay.click("top \u{00b7} schematic", Role::Tab);
    let state = &fixture.replay.app.state;
    assert_eq!(state.model.workspace.content.active_view, fixture.parent);
    assert!(state.design.symbol.in_place.is_none());
    assert!(
        !state
            .model
            .workspace
            .content
            .open_views
            .iter()
            .any(|open| open.reference == fixture.symbol),
        "the symbol tab the session opened is closed"
    );
    let editor = &state.model.schematic.session.editor;
    assert_eq!((editor.zoom, editor.pan), (2.0, (60.0, 40.0)));
    assert_eq!(editor.selection.single_component(), Some(fixture.instance));
}

/// Revert to saved, in place, puts the symbol back as the project last saved
/// it and the instance's wire back on its pin, and the session goes on.
#[test]
fn reverting_in_place_brings_the_instance_and_its_wire_back() {
    let mut fixture = InPlace::new();
    fixture.wire_pin_a();
    let path = ProjectPath::new("revert");
    assert!(fixture.replay.app.state.save_project_to_path(&path.0));
    fixture.enter();
    fixture.drag(Point::new(60, 70), Point::new(60, 100));
    assert_eq!(fixture.pin_a(), Some(Point::new(-40, 20)));
    assert_eq!(fixture.wire_end(), Point::new(60, 100));

    let token = crate::workbench::lifecycle::project_lifecycle::prepare_revert_active_document(
        &fixture.replay.app.state,
    )
    .expect("the revert is reviewed");
    assert!(
        fixture
            .replay
            .app
            .state
            .confirm_revert_active_document(&token)
    );
    fixture.replay.frames(vec![]);
    assert_eq!(fixture.pin_a(), Some(Point::new(-40, -10)), "the saved pin");
    assert_eq!(fixture.wire_end(), Point::new(60, 70), "the wire came back");
    assert!(
        crate::workbench::app::in_place_session(
            &fixture.replay.app.state.design.symbol.in_place,
            &fixture.replay.app.state.model,
        )
        .is_some(),
        "the session goes on"
    );
}

/// Saving while editing in place saves the symbol, the document in front, as
/// Save does in its own view, and with it the sheet whose wire followed the
/// moved pin, so no file holds a wire off the saved pin. The project reopens
/// with the edit in both and no half-open session.
#[test]
fn saving_in_place_writes_the_edit_and_reopens_without_the_session() {
    use crate::workbench::lifecycle::project_lifecycle::dirty_documents;
    use crate::workbench::project::commands::ProjectCommand;

    let mut fixture = InPlace::new();
    fixture.wire_pin_a();
    let path = ProjectPath::new("save");
    assert!(fixture.replay.app.state.save_project_to_path(&path.0));
    fixture.enter();
    fixture.drag(Point::new(60, 70), Point::new(60, 100));
    assert_eq!(fixture.wire_end(), Point::new(60, 100));
    fixture
        .replay
        .app
        .execute_command_key(ProjectCommand::Save.key());
    fixture.replay.frames(vec![]);
    assert!(
        crate::workbench::app::in_place_session(
            &fixture.replay.app.state.design.symbol.in_place,
            &fixture.replay.app.state.model,
        )
        .is_some(),
        "saving keeps the session"
    );
    assert_eq!(
        dirty_documents(&fixture.replay.app.state),
        Vec::new(),
        "the symbol and the sheet it rewired are both saved"
    );

    let reopened = fixture.reopen(&path);
    let state = &reopened.app.state;
    assert_eq!(
        state
            .model
            .load_symbol_document(&fixture.symbol)
            .unwrap()
            .pin("A")
            .and_then(|pin| pin.position),
        Some(Point::new(-40, 20)),
        "the symbol holds the edit"
    );
    assert_eq!(
        fixture.saved_wire_end(&reopened),
        Point::new(60, 100),
        "the sheet holds the wire where the pin put it"
    );
    assert!(
        state.design.symbol.in_place.is_none(),
        "no session survives the reopen"
    );
}

/// Saving the symbol in its own view, after a pin move there carried a wire
/// on an open sheet, writes that sheet with it.
#[test]
fn saving_the_symbol_view_writes_the_sheet_its_pin_move_rewired() {
    use crate::workbench::lifecycle::project_lifecycle::dirty_documents;
    use crate::workbench::project::commands::ProjectCommand;

    let mut fixture = InPlace::new();
    fixture.wire_pin_a();
    let path = ProjectPath::new("save-view");
    assert!(fixture.replay.app.state.save_project_to_path(&path.0));
    fixture
        .replay
        .app
        .state
        .open_workspace_view(fixture.symbol.clone());
    fixture.replay.frames(vec![]);
    let state = &mut fixture.replay.app.state;
    let mut document = state.model.load_active_symbol_document().unwrap();
    document
        .pins
        .iter_mut()
        .find(|pin| pin.name == "A")
        .expect("the symbol has pin A")
        .position = Some(Point::new(-40, 20));
    state.store_active_symbol_document(&document).unwrap();
    fixture.replay.frames(vec![]);
    assert_eq!(fixture.wire_end(), Point::new(60, 100));
    fixture
        .replay
        .app
        .execute_command_key(ProjectCommand::Save.key());
    fixture.replay.frames(vec![]);
    assert_eq!(
        dirty_documents(&fixture.replay.app.state),
        Vec::new(),
        "the symbol and the sheet it rewired are both saved"
    );
    let reopened = fixture.reopen(&path);
    assert_eq!(fixture.saved_wire_end(&reopened), Point::new(60, 100));
}

/// Undo names the symbol's step whenever it is the one Undo takes back, a
/// project step older than it notwithstanding: the row reads "Undo move pin A",
/// and choosing it puts the pin back, the project step still to undo.
#[test]
fn undo_names_the_symbol_step_over_an_older_project_step() {
    use crate::workbench::framework::command_registry::ids;
    use crate::workbench::framework::host::Host;

    let mut fixture = InPlace::new();
    let state = &mut fixture.replay.app.state;
    let instance = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == fixture.instance)
        .cloned()
        .expect("the placed amp");
    assert!(
        state
            .model
            .rename_component_transaction(&instance, "X7".to_owned())
            .expect("the rename commits")
    );
    let project_step = state.model.project_undo_sequence();
    assert!(project_step.is_some(), "a rename is a project step");
    fixture.enter();
    fixture.drag(Point::new(60, 70), Point::new(60, 100));
    assert_eq!(fixture.pin_a(), Some(Point::new(-40, 20)));
    assert_eq!(
        Host::menu_label(&fixture.replay.app, ids::UNDO),
        "Undo move pin A"
    );

    fixture
        .replay
        .key(egui::Key::Z, crate::feature_tests::replay::PRIMARY);
    assert_eq!(
        fixture.pin_a(),
        Some(Point::new(-40, -10)),
        "the pin is back"
    );
    assert_eq!(
        fixture.replay.app.state.model.project_undo_sequence(),
        project_step,
        "the rename is still to undo"
    );
}

/// An instance whose cell has a symbol and no schematic master opens Descend
/// with options too, its symbol in place the one way in, stated rather than
/// offered as a choice of one; the form commits to the in-place edit.
#[test]
fn descend_with_options_edits_a_symbol_only_cell_in_place() {
    let mut fixture = InPlace::new();
    let schematic = CellViewRef::new(&fixture.symbol.library, &fixture.symbol.cell, "schematic");
    let state = &mut fixture.replay.app.state;
    assert!(
        state
            .model
            .library_manager
            .get_library_mut(&schematic.library)
            .and_then(|library| library.get_cell_mut(&schematic.cell))
            .expect("the amp cell")
            .remove_view(&schematic.view)
    );
    state
        .model
        .workspace
        .remove_schematic_editor(&schematic.key());
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(fixture.instance);
    fixture.replay.frames(vec![]);

    let form = fixture.replay.choose_command("Descend with options");
    assert!(
        form.iter().any(|control| control.role == Role::Label
            && control.label == "Edit mode"
            && control.value == "Edit symbol in place"),
        "the form states the one mode"
    );
    assert!(
        !form.iter().any(|control| control.role == Role::ComboBox
            && control.value.contains("Edit symbol in place")),
        "a single way in is not a choice"
    );
    fixture.replay.click("Open edit context", Role::Button);
    let state = &fixture.replay.app.state;
    assert_eq!(state.model.workspace.content.active_view, fixture.symbol);
    assert!(
        crate::workbench::app::in_place_session(&state.design.symbol.in_place, &state.model)
            .is_some(),
        "the symbol is edited in place"
    );
}

/// The layout saved while a symbol is edited in place is the sheet's: the
/// project reopens on the parent sheet, which opens as it does saved without
/// the edit rather than at the edit's view, and the tab the session opened
/// for the symbol is not part of it.
#[test]
fn the_layout_saved_in_place_is_the_sheets() {
    use crate::workbench::project::commands::ProjectCommand;

    let mut fixture = InPlace::new();
    let path = ProjectPath::new("layout");
    assert!(fixture.replay.app.state.save_project_to_path(&path.0));
    let opened_view = |reopened: &Replay| {
        let editor = &reopened.app.state.model.schematic.session.editor;
        (editor.zoom, editor.pan)
    };
    let sheet_view = opened_view(&fixture.reopen(&path));
    // A camera that leaves the instance out, which entering frames on it.
    fixture.replay.app.state.model.schematic.session.editor.pan = (-4000.0, -4000.0);
    fixture.replay.frames(vec![]);
    fixture.enter();
    let in_place_view = {
        let session = crate::workbench::app::in_place_session(
            &fixture.replay.app.state.design.symbol.in_place,
            &fixture.replay.app.state.model,
        )
        .expect("the symbol is edited in place");
        (session.zoom, session.pan)
    };
    fixture.drag(Point::new(60, 70), Point::new(60, 100));
    fixture
        .replay
        .app
        .execute_command_key(ProjectCommand::SaveAll.key());
    fixture.replay.frames(vec![]);

    let reopened = fixture.reopen(&path);
    let content = &reopened.app.state.model.workspace.content;
    assert_eq!(content.active_view, fixture.parent, "it lands on the sheet");
    assert!(
        !content
            .open_views
            .iter()
            .any(|open| open.reference == fixture.symbol),
        "the in-place tab is not saved: {:?}",
        content.open_views
    );
    assert_eq!(
        opened_view(&reopened),
        sheet_view,
        "the sheet opens as it does saved without the edit"
    );
    assert_ne!(
        opened_view(&reopened),
        in_place_view,
        "not at the edit's view"
    );
}

impl InPlace {
    /// The project saved at `path`, opened in a new application.
    fn reopen(&self, path: &ProjectPath) -> Replay {
        let mut reopened = Replay::new();
        assert!(reopened.app.state.load_project_from_path(&path.0));
        reopened.frames(vec![]);
        reopened
    }

    /// The first point of the parent's first wire, as `reopened` holds it.
    fn saved_wire_end(&self, reopened: &Replay) -> Point {
        let content = &reopened.app.state.model.workspace.content;
        let sheet = content
            .schematic_buffers
            .get(&self.parent.key())
            .filter(|_| content.active_view != self.parent)
            .map_or_else(
                || reopened.app.state.model.schematic.document().clone(),
                |buffer| buffer.document().clone(),
            );
        sheet.wires[0].points[0]
    }
}

/// Drawing in place draws in the symbol's own coordinates, read back through
/// the instance: a rectangle dragged on the sheet lands where it was drawn,
/// and the shape's context menu works there as in the symbol's own view,
/// its Rotate one step that Undo takes back.
#[test]
fn a_shape_drawn_in_place_lands_where_it_was_drawn_and_its_menu_works() {
    let mut fixture = InPlace::new();
    let entered = fixture.enter();
    click_at(&mut fixture.replay, canvas_centre(&entered));
    fixture.replay.key(egui::Key::R, egui::Modifiers::NONE);
    // The unturned instance stands at (100, 80): symbol (60, 40) to
    // (100, 60), clear of the body and the pins, is sheet (160, 120) to
    // (200, 140).
    fixture.drag(Point::new(160, 120), Point::new(200, 140));
    let body = |fixture: &InPlace| {
        fixture
            .replay
            .app
            .state
            .model
            .load_symbol_document(&fixture.symbol)
            .unwrap()
            .body
    };
    let drawn = crate::state::SymbolShape::Rectangle {
        min: Point::new(60, 40),
        max: Point::new(100, 60),
        style: Default::default(),
    };
    assert_eq!(body(&fixture).last(), Some(&drawn));
    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);

    // The rectangle's lower edge, at symbol (80, 60).
    let edge = fixture.screen(Point::new(180, 140));
    let button = |pressed| egui::Event::PointerButton {
        pos: edge,
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    fixture.replay.frame(vec![egui::Event::PointerMoved(edge)]);
    fixture.replay.frame(vec![button(true)]);
    fixture.replay.frame(vec![button(false)]);
    fixture.replay.frames(vec![]);
    let menu = fixture.replay.frames(vec![]);
    let rotate = menu
        .iter()
        .find(|control| control.label == "Rotate 90\u{b0}")
        .unwrap_or_else(|| {
            panic!(
                "the shape's menu offers Rotate: {:?}",
                menu.iter()
                    .map(|control| &control.label)
                    .collect::<Vec<_>>()
            )
        })
        .center;
    click_at(&mut fixture.replay, rotate);
    assert_eq!(
        body(&fixture).last(),
        Some(&crate::state::SymbolShape::Rectangle {
            min: Point::new(70, 30),
            max: Point::new(90, 70),
            style: Default::default(),
        }),
        "turned about its middle"
    );
    Command::Undo.execute(&mut fixture.replay.app);
    fixture.replay.frames(vec![]);
    assert_eq!(
        body(&fixture).last(),
        Some(&drawn),
        "Undo took the turn back"
    );
}

/// The instance's own context menu edits its symbol in place: a right-click
/// on the part selects it, and the menu's row lands in the same session the
/// command does, on that instance, at the sheet's own view.
#[test]
fn the_instance_menu_edits_its_symbol_in_place() {
    let mut fixture = InPlace::new();
    let output = fixture.replay.frame(vec![]);
    let canvas = canvas_rect(&output);
    // The sheet's camera is at 200%, panned (60, 40): the instance's body
    // centre (100, 80) is at canvas (260, 200).
    let body = canvas.min + egui::vec2(260.0, 200.0);
    let button = |pressed| egui::Event::PointerButton {
        pos: body,
        button: egui::PointerButton::Secondary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    fixture.replay.frame(vec![egui::Event::PointerMoved(body)]);
    fixture.replay.frame(vec![button(true)]);
    fixture.replay.frame(vec![button(false)]);
    let menu = fixture.replay.frames(vec![]);
    let row = menu
        .iter()
        .find(|control| control.label == "Edit symbol in place")
        .unwrap_or_else(|| {
            panic!(
                "the instance's menu offers it: {:?}",
                menu.iter()
                    .map(|control| &control.label)
                    .collect::<Vec<_>>()
            )
        })
        .center;
    click_at(&mut fixture.replay, row);
    let state = &fixture.replay.app.state;
    assert_eq!(state.model.workspace.content.active_view, fixture.symbol);
    let session =
        crate::workbench::app::in_place_session(&state.design.symbol.in_place, &state.model)
            .expect("the menu edits the symbol in place");
    assert_eq!(session.instance_id, fixture.instance);
    assert_eq!((session.zoom, session.pan), (2.0, (60.0, 40.0)));
}

/// Every drawing tool draws in place as in the symbol's own view, in the
/// symbol's coordinates, each point clicked on the sheet around the
/// instance: a circle, an arc, a closed polygon, typed text and a pin.
#[test]
fn every_drawing_tool_draws_in_place_in_the_symbols_coordinates() {
    use crate::state::SymbolShape;

    let mut fixture = InPlace::new();
    let entered = fixture.enter();
    click_at(&mut fixture.replay, canvas_centre(&entered));
    // The unturned instance stands at (100, 80): a symbol point is that much
    // further on the sheet. Every point here is clear of the body and pins.
    let on_sheet = |x: i32, y: i32| Point::new(x + 100, y + 80);
    let place = |fixture: &mut InPlace, tool: DesignCommand, points: &[Point]| {
        fixture.replay.app.execute_command_key(tool.key());
        fixture.replay.frames(vec![]);
        for point in points {
            let at = fixture.screen(*point);
            click_at(&mut fixture.replay, at);
        }
    };
    let document = |fixture: &InPlace| {
        fixture
            .replay
            .app
            .state
            .model
            .load_symbol_document(&fixture.symbol)
            .unwrap()
    };

    place(
        &mut fixture,
        DesignCommand::SymbolCircleTool,
        &[on_sheet(80, 50), on_sheet(90, 50)],
    );
    assert_eq!(
        document(&fixture).body.last(),
        Some(&SymbolShape::Circle {
            center: Point::new(80, 50),
            radius: 10,
            style: Default::default(),
        })
    );

    place(
        &mut fixture,
        DesignCommand::SymbolArcTool,
        &[on_sheet(80, -50), on_sheet(90, -50), on_sheet(80, -40)],
    );
    assert!(
        matches!(document(&fixture).body.last(),
            Some(SymbolShape::Arc { center, radius: 10, .. }) if *center == Point::new(80, -50)),
        "{:?}",
        document(&fixture).body.last()
    );

    place(
        &mut fixture,
        DesignCommand::SymbolPolygonTool,
        &[on_sheet(60, 80), on_sheet(100, 80), on_sheet(80, 100)],
    );
    fixture.replay.key(egui::Key::Enter, egui::Modifiers::NONE);
    assert!(
        matches!(document(&fixture).body.last(),
            Some(SymbolShape::Polyline { points, closed: true, .. })
                if points.starts_with(&[Point::new(60, 80), Point::new(100, 80), Point::new(80, 100)])),
        "{:?}",
        document(&fixture).body.last()
    );

    place(
        &mut fixture,
        DesignCommand::SymbolTextTool,
        &[on_sheet(60, -80)],
    );
    fixture
        .replay
        .frames(vec![egui::Event::Text("GAIN".to_owned())]);
    fixture.replay.key(egui::Key::Enter, egui::Modifiers::NONE);
    assert!(
        matches!(document(&fixture).body.last(),
            Some(SymbolShape::Text { anchor, text, .. })
                if *anchor == Point::new(60, -80) && text == "GAIN"),
        "{:?}",
        document(&fixture).body.last()
    );

    let pins = document(&fixture).pins.len();
    place(
        &mut fixture,
        DesignCommand::SymbolPinTool,
        &[on_sheet(-60, 60)],
    );
    fixture
        .replay
        .frames(vec![egui::Event::Text("EN".to_owned())]);
    fixture.replay.key(egui::Key::Enter, egui::Modifiers::NONE);
    let drawn = document(&fixture);
    assert_eq!(drawn.pins.len(), pins + 1, "the pin tool placed a pin");
    assert!(
        drawn
            .pins
            .iter()
            .any(|pin| pin.position == Some(Point::new(-60, 60))),
        "its tip is where it was clicked: {:?}",
        drawn.pins
    );
}

/// A drawn shape is reshaped and restyled in place as in the symbol's own
/// view: the selected rectangle's corner handle, dragged on the sheet,
/// resizes it in the symbol's coordinates, and the inspector beside the
/// edit sets its stroke.
#[test]
fn a_shape_is_reshaped_and_restyled_in_place() {
    use crate::state::SymbolShape;
    use rspice_design::symbol::SymbolStroke;
    use rspice_schematic_editor::symbol_editor::session::SymbolTool;

    let mut fixture = InPlace::new();
    fixture.replay.app.state.workbench.inspector_visible = true;
    let entered = fixture.enter();
    click_at(&mut fixture.replay, canvas_centre(&entered));
    fixture.replay.key(egui::Key::R, egui::Modifiers::NONE);
    // Symbol (60, 40) to (100, 60), clear of the body and the pins.
    fixture.drag(Point::new(160, 120), Point::new(200, 140));
    let body = |fixture: &InPlace| {
        fixture
            .replay
            .app
            .state
            .model
            .load_symbol_document(&fixture.symbol)
            .unwrap()
            .body
    };
    let index = body(&fixture).len() - 1;
    let editor = &mut fixture.replay.app.state.design.symbol.editor;
    editor.tool = SymbolTool::Select;
    editor.select_shape(index);
    fixture.replay.frames(vec![]);

    // The corner at symbol (100, 60) is sheet (200, 140).
    fixture.drag(Point::new(200, 140), Point::new(210, 150));
    assert_eq!(
        body(&fixture)[index],
        SymbolShape::Rectangle {
            min: Point::new(60, 40),
            max: Point::new(110, 70),
            style: Default::default(),
        },
        "the corner handle resized it"
    );

    let press = |fixture: &mut InPlace, label: &str| {
        let controls = fixture.replay.frames(vec![]);
        let at = controls
            .iter()
            .find(|control| control.label == label)
            .unwrap_or_else(|| {
                panic!(
                    "{label:?} is offered in place: {:?}",
                    controls
                        .iter()
                        .map(|control| &control.label)
                        .collect::<Vec<_>>()
                )
            })
            .center;
        click_at(&mut fixture.replay, at);
    };
    press(&mut fixture, "Stroke");
    press(&mut fixture, "heavy");
    assert_eq!(
        body(&fixture)[index].style().map(|style| style.stroke),
        Some(SymbolStroke::Heavy),
        "the inspector restyled it"
    );
    assert_eq!(
        fixture.replay.app.state.model.workspace.content.active_view, fixture.symbol,
        "still in place"
    );
}

/// Escape lets go of a handle before it leaves the edit in place: a handle
/// Stretch armed, and one being dragged, are each the canvas's to cancel
/// first. With nothing held, Escape leaves.
#[test]
fn escape_lets_go_of_a_held_handle_before_it_leaves_in_place() {
    use rspice_schematic_editor::symbol_editor::handles::{SymbolHandle, SymbolHandleRef};

    let mut fixture = InPlace::new();
    fixture.enter();
    let active = |fixture: &InPlace| {
        fixture
            .replay
            .app
            .state
            .model
            .workspace
            .content
            .active_view
            .clone()
    };
    let editor = &mut fixture.replay.app.state.design.symbol.editor;
    editor.select_shape(0);
    editor.stretch = Some(SymbolHandleRef {
        shape: 0,
        handle: SymbolHandle::Vertex(0),
    });
    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    assert_eq!(
        active(&fixture),
        fixture.symbol,
        "Escape let go of the stretch"
    );
    assert!(
        fixture
            .replay
            .app
            .state
            .design
            .symbol
            .editor
            .stretch
            .is_none()
    );

    // The body's first corner, symbol (-20, -20), is sheet (80, 60).
    fixture
        .replay
        .app
        .state
        .design
        .symbol
        .editor
        .select_shape(0);
    let corner = fixture.screen(Point::new(80, 60));
    let button = |pressed| egui::Event::PointerButton {
        pos: corner,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    fixture
        .replay
        .frame(vec![egui::Event::PointerMoved(corner), button(true)]);
    fixture.replay.frame(vec![egui::Event::PointerMoved(
        corner + egui::vec2(8.0, 0.0),
    )]);
    fixture.replay.frame(vec![egui::Event::PointerMoved(
        corner + egui::vec2(20.0, 0.0),
    )]);
    assert!(
        fixture
            .replay
            .app
            .state
            .design
            .symbol
            .editor
            .dragging_handle
            .is_some(),
        "the corner's handle is being dragged"
    );
    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    assert_eq!(
        active(&fixture),
        fixture.symbol,
        "Escape let go of the drag"
    );
    fixture.replay.frames(vec![button(false)]);

    fixture.replay.key(egui::Key::Escape, egui::Modifiers::NONE);
    assert_eq!(
        active(&fixture),
        fixture.parent,
        "with nothing held it leaves"
    );
}
