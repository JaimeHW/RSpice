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
    assert!(buttons.contains(&"Return to parent (Enter)"), "{buttons:?}");
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
/// Save does in its own view; the sheet whose wire followed the moved pin
/// stays unsaved work the close review lists, and Save All writes it too.
/// The project reopens with the edit in both and no half-open session.
#[test]
fn saving_in_place_writes_the_edit_and_reopens_without_the_session() {
    use crate::workbench::lifecycle::project_lifecycle::{ProjectDocumentId, dirty_documents};
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
    let unsaved = dirty_documents(&fixture.replay.app.state);
    assert!(
        unsaved.contains(&ProjectDocumentId::CellView(fixture.parent.clone()))
            && !unsaved.contains(&ProjectDocumentId::CellView(fixture.symbol.clone())),
        "the symbol is saved and the sheet is still unsaved work: {unsaved:?}"
    );
    fixture
        .replay
        .app
        .execute_command_key(ProjectCommand::SaveAll.key());
    fixture.replay.frames(vec![]);
    assert_eq!(dirty_documents(&fixture.replay.app.state), Vec::new());

    let mut reopened = Replay::new();
    assert!(reopened.app.state.load_project_from_path(&path.0));
    reopened.frames(vec![]);
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
    let sheet = state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&fixture.parent.key())
        .filter(|_| state.model.workspace.content.active_view != fixture.parent)
        .map_or_else(
            || state.model.schematic.document().clone(),
            |buffer| buffer.document().clone(),
        );
    assert_eq!(
        sheet.wires[0].points[0],
        Point::new(60, 100),
        "the sheet holds the wire where the pin put it"
    );
    assert!(
        state.design.symbol.in_place.is_none(),
        "no session survives the reopen"
    );
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
