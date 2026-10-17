//! The schematic driven by a finger alone, on a phone and on a tablet.
//!
//! Every frame is a whole application frame, as `eframe::App::ui` runs one,
//! and every input is a touch as a tablet reports it: the touch event and the
//! primary pointer the platform drives from it. Nothing is set up by hand once
//! the sheet exists.

use super::*;
use crate::state::{ComponentType, Point, SchematicState, Tool};
use crate::workbench::state::Workspace;
use egui::{Event, Pos2, Rect, TouchPhase, Vec2};

/// Someone at the application with nothing but a finger.
struct Finger {
    ctx: egui::Context,
    app: RSpiceApp,
    size: Vec2,
    time: f64,
    /// What the last frame named for a screen reader, and where.
    named: Vec<(String, Rect)>,
}

impl Finger {
    fn new(size: Vec2) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.symbol_library = crate::schematic::symbols::SymbolLibrary::load_embedded().ok();
        app.state.project_lifecycle.authority.open_session();
        app.state.workbench.activate(Workspace::Design);
        app.state.schematic = SchematicState::default();
        app.state.schematic.session.editor.needs_fit = false;
        app.state.schematic.session.editor.needs_drawing_sheet_fit = false;
        app.state.schematic.clear_undo_history();
        app.state.sync_active_schematic_to_workspace();
        let mut finger = Self {
            ctx,
            app,
            size,
            time: 0.0,
            named: Vec::new(),
        };
        finger.frame(Vec::new());
        finger.frame(Vec::new());
        finger
    }

    fn frame(&mut self, events: Vec<Event>) {
        self.frame_after(0.02, events);
    }

    fn frame_after(&mut self, seconds: f64, events: Vec<Event>) {
        self.time += seconds;
        let Self {
            ctx,
            app,
            size,
            time,
            named,
        } = self;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, *size)),
                events,
                focused: true,
                time: Some(*time),
                ..Default::default()
            },
            |ui| {
                let ctx = ui.ctx().clone();
                let before = FrameSimulationInput::capture(&app.state);
                app.prepare_frame(&ctx);
                app.render_frame_chrome(ui);
                app.render_secondary_application_windows(&ctx);
                app.refresh_live_checks(&ctx);
                app.render_frame_dialogs(&ctx);
                app.end_frame(&before);
            },
        );
        named.clear();
        for (_, node) in output
            .platform_output
            .accesskit_update
            .into_iter()
            .flat_map(|update| update.nodes)
        {
            if let (Some(name), Some(bounds)) =
                (node.label().or_else(|| node.value()), node.bounds())
            {
                named.push((
                    name.to_owned(),
                    Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                ));
            }
        }
    }

    /// Where the last frame drew `name`; the widest, when several share it
    /// (a menu row is wider than the strip button of the same name).
    fn find(&self, name: &str) -> Option<Rect> {
        self.named
            .iter()
            .filter(|(candidate, _)| candidate == name)
            .map(|(_, rect)| *rect)
            .max_by(|a, b| a.width().total_cmp(&b.width()))
    }

    fn expect(&self, name: &str) -> Rect {
        self.find(name).unwrap_or_else(|| {
            panic!(
                "nothing on screen is named {name:?}; named: {:?}",
                self.named.iter().map(|(name, _)| name).collect::<Vec<_>>()
            )
        })
    }

    fn touch(phase: TouchPhase, pos: Pos2) -> Event {
        Event::Touch {
            device_id: egui::TouchDeviceId(1),
            id: egui::TouchId(1),
            phase,
            pos,
            force: None,
        }
    }

    fn down(&mut self, pos: Pos2) {
        self.frame(vec![
            Self::touch(TouchPhase::Start, pos),
            Event::PointerMoved(pos),
            Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
        ]);
    }

    fn up(&mut self, pos: Pos2) {
        self.frame(vec![
            Self::touch(TouchPhase::End, pos),
            Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
            Event::PointerGone,
        ]);
    }

    /// A moment with the finger off the glass: a flung menu stops scrolling,
    /// and the next tap is not counted with the last one (egui takes a second
    /// click within 0.3 s anywhere as a double click, a third within 0.6 s as
    /// a triple).
    fn settle(&mut self) {
        for _ in 0..10 {
            self.frame_after(0.1, Vec::new());
        }
    }

    /// A deliberate tap, after a moment.
    fn tap(&mut self, pos: Pos2) {
        self.settle();
        self.touch_at(pos);
    }

    fn touch_at(&mut self, pos: Pos2) {
        self.down(pos);
        self.up(pos);
        self.frame(Vec::new());
        self.frame(Vec::new());
    }

    /// Tap what is named `name` once the screen has settled.
    fn press(&mut self, name: &str) {
        self.settle();
        let at = self.expect(name).center();
        self.touch_at(at);
    }

    /// Tap the widest thing named starting with `prefix` once the screen has
    /// settled: a toolbar button's name carries its key, as "Draw wire (W)".
    fn press_starting(&mut self, prefix: &str) {
        self.settle();
        let at = self
            .named
            .iter()
            .filter(|(name, _)| name.starts_with(prefix))
            .map(|(_, rect)| *rect)
            .max_by(|a, b| a.width().total_cmp(&b.width()))
            .unwrap_or_else(|| panic!("nothing on screen starts {prefix:?}: {:?}", self.named))
            .center();
        self.touch_at(at);
    }

    /// The screen point over the world point `at`, through the canvas's
    /// camera.
    fn screen(&self, at: Point) -> Pos2 {
        let editor = &self.app.state.schematic.session.editor;
        self.expect("Schematic canvas").min
            + egui::vec2(
                (editor.pan.0 + f64::from(at.x) * editor.zoom) as f32,
                (editor.pan.1 + f64::from(at.y) * editor.zoom) as f32,
            )
    }

    /// Two taps in quick succession: egui's double click.
    fn double_tap(&mut self, pos: Pos2) {
        self.settle();
        self.down(pos);
        self.up(pos);
        self.down(pos);
        self.up(pos);
        self.frame(Vec::new());
        self.frame(Vec::new());
    }

    /// A finger held still past egui's long-press time, then lifted.
    fn long_press(&mut self, pos: Pos2) {
        self.settle();
        self.down(pos);
        self.frame_after(0.9, vec![Self::touch(TouchPhase::Move, pos)]);
        self.up(pos);
        self.frame(Vec::new());
        self.frame(Vec::new());
    }

    /// One finger from `from` to `to` in four steps, held there until it has
    /// no speed left to fling with, then lifted.
    fn drag(&mut self, from: Pos2, to: Pos2) {
        self.down(from);
        for step in 1..=4 {
            let at = from + (to - from) * (step as f32 / 4.0);
            self.frame(vec![
                Self::touch(TouchPhase::Move, at),
                Event::PointerMoved(at),
            ]);
        }
        for _ in 0..3 {
            self.frame_after(
                0.1,
                vec![Self::touch(TouchPhase::Move, to), Event::PointerMoved(to)],
            );
        }
        self.up(to);
        self.frame(Vec::new());
    }

    /// Scroll the open canvas menu with a finger until the row named `name`
    /// is inside it, measured once the fling has stopped.
    fn reveal(&mut self, name: &str) {
        for _ in 0..12 {
            self.settle();
            let menu = self.expect("Schematic selection");
            let row = self.expect(name);
            if menu.contains_rect(row) {
                return;
            }
            // Just far enough to bring the row in, past the drag threshold.
            let travel = if row.bottom() > menu.bottom() {
                (menu.bottom() - row.bottom() - 12.0).min(-12.0)
            } else {
                (menu.top() - row.top() + 12.0).max(12.0)
            };
            let travel = travel.clamp(-menu.height() / 2.0, menu.height() / 2.0);
            self.drag(menu.center(), menu.center() + egui::vec2(0.0, travel));
        }
        panic!("{name:?} never scrolled into the menu");
    }

    /// A cell `amp` in the top schematic's library, with a schematic to
    /// descend into and a one-pin symbol, placed once at `at`.
    fn place_subcircuit(&mut self, at: Point) {
        use crate::state::{
            Cell, CellViewRef, LibraryCellInstance, OpenCellView, PortDirection, PortSpec,
            SymbolDocument, SymbolPin, View, ViewType,
        };
        let state = &mut self.app.state;
        let top = state.workspace.content.active_schematic_reference();
        let master = CellViewRef::new(&top.library, "amp", "schematic");
        let mut cell = Cell::new(&master.cell);
        cell.add_view(View::new(&master.view, ViewType::Schematic));
        let mut symbol = View::new("symbol", ViewType::Symbol);
        SymbolDocument {
            pins: vec![SymbolPin::new(
                "OUT",
                PortDirection::Out,
                Some(Point::new(40, 0)),
            )],
            ..SymbolDocument::default()
        }
        .store_in_view(&mut symbol)
        .expect("the symbol stores");
        cell.add_view(symbol);
        state
            .library_manager
            .get_library_mut(&master.library)
            .expect("the top schematic's library")
            .add_cell(cell);
        state
            .workspace
            .content
            .open_views
            .push(OpenCellView::new(master.clone(), ViewType::Schematic));
        state
            .workspace
            .insert_schematic_editor(master.key(), SchematicState::default());
        let mut binding = LibraryCellInstance::new(&master.library, &master.cell, &master.view);
        binding.bind_interface(&[PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        }]);
        state.schematic.add_library_cell_component(at, binding);
        state.schematic.clear_undo_history();
        state.sync_active_schematic_to_workspace();
        self.frame(Vec::new());
    }

    /// A touch that lands and lifts with no tap: the shell and the canvas
    /// are in touch mode and nothing is clicked.
    fn wake(&mut self) {
        let corner = Pos2::new(self.size.x - 2.0, self.size.y - 2.0);
        self.frame(vec![
            Self::touch(TouchPhase::Start, corner),
            Self::touch(TouchPhase::End, corner),
        ]);
        self.frame(Vec::new());
        self.frame(Vec::new());
    }
}

/// U-11: at 560 pt and less the tool row kept its first two tools and
/// dropped the rest, so a 390 pt phone had no button for Draw wire, Undo or
/// Redo. The rest now sit behind More tools, each two taps away.
#[test]
fn a_phone_reaches_draw_wire_and_undo_from_the_tool_row() {
    let mut phone = Finger::new(Vec2::new(390.0, 844.0));
    phone.wake();
    // The row has room for two buttons beside Run: Select, and More tools.
    let more = phone.expect("More tools");
    let run = phone.expect("Run active simulation plan at TT · 27 °C (F5)");
    assert!(
        more.right() <= run.left(),
        "More tools {more:?} clears Run {run:?}"
    );
    phone.press("More tools");
    phone.press("Draw wire");
    assert_eq!(phone.app.state.schematic.session.editor.tool, Tool::Wire);

    phone.press("More tools");
    let undo = phone.expect("Undo");
    assert!(
        undo.width() > 100.0,
        "Undo is a row of the More tools menu, not only the strip's button: {undo:?}"
    );
    let redo = phone.expect("Redo");
    assert!(redo.width() > 100.0, "and so is Redo: {redo:?}");

    // A row with no more tools than slots shows them all, and no More tools.
    phone.app.state.workbench.activate(Workspace::Project);
    phone.frame(Vec::new());
    phone.frame(Vec::new());
    assert!(phone.find("More tools").is_none(), "{:?}", phone.named);
    let shown = |prefix: &str| phone.named.iter().any(|(name, _)| name.starts_with(prefix));
    assert!(
        shown("New cell") && shown("Focus workspace"),
        "{:?}",
        phone.named
    );
}

/// Repair plan F5, end to end: a schematic edited on a tablet with nothing
/// but a finger. R1 comes from the component shelf, a wire is drawn and
/// ended with a double tap, R1 is turned on the touch strip, R1 and the wire
/// are selected with the Add latch and deleted from the long-press menu, the
/// strip's Undo brings both back, and the long-press menu descends into a
/// subcircuit instance.
#[test]
fn a_tablet_edits_a_schematic_by_finger_alone() {
    let mut tablet = Finger::new(Vec2::new(1024.0, 768.0));
    let subcircuit = Point::new(600, 400);
    tablet.place_subcircuit(subcircuit);
    tablet.wake();
    let components = |tablet: &Finger| tablet.app.state.schematic.document().components.clone();
    let wires = |tablet: &Finger| tablet.app.state.schematic.document().wires.len();

    // Place R1 from the shelf.
    tablet.press_starting("Place instance");
    tablet.press_starting("Resistor");
    assert_eq!(
        tablet.app.state.schematic.session.editor.tool,
        Tool::Place(ComponentType::Resistor)
    );
    tablet.tap(tablet.screen(Point::new(200, 200)));
    let r1 = components(&tablet)
        .into_iter()
        .find(|part| part.kind == ComponentType::Resistor)
        .expect("the tap placed a resistor");
    assert_eq!(r1.pos, Point::new(200, 200));
    tablet.press("Cancel");
    assert_eq!(tablet.app.state.schematic.session.editor.tool, Tool::Select);

    // Draw a wire, ended with a double tap.
    tablet.press_starting("Draw wire");
    tablet.tap(tablet.screen(Point::new(300, 100)));
    assert!(
        tablet
            .app
            .state
            .schematic
            .session
            .editor
            .wire_drawing
            .active
    );
    tablet.double_tap(tablet.screen(Point::new(420, 100)));
    assert_eq!(wires(&tablet), 1, "the double tap ended the wire");
    tablet.press("Cancel");

    // Turn R1.
    tablet.tap(tablet.screen(Point::new(200, 200)));
    tablet.press("Rotate clockwise");
    let turned = components(&tablet)
        .into_iter()
        .find(|part| part.id == r1.id)
        .expect("R1");
    assert_eq!(turned.rotation, r1.rotation.rotate_cw());

    // Select R1 and the wire.
    tablet.press("Add to selection");
    tablet.tap(tablet.screen(Point::new(360, 100)));
    tablet.press("Add to selection");
    let selection = &tablet.app.state.schematic.session.editor.selection;
    assert!(selection.has_component(r1.id) && selection.wires.len() == 1);

    // Delete both from the long-press menu.
    tablet.long_press(tablet.screen(Point::new(200, 200)));
    tablet.reveal("Delete");
    tablet.press("Delete");
    assert!(components(&tablet).iter().all(|part| part.id != r1.id));
    assert_eq!(wires(&tablet), 0);

    // Undo brings both back.
    tablet.press("Undo");
    assert!(components(&tablet).iter().any(|part| part.id == r1.id));
    assert_eq!(wires(&tablet), 1);

    // Descend into the subcircuit from its long-press menu.
    tablet.long_press(tablet.screen(subcircuit + Point::new(20, 0)));
    tablet.reveal("Descend into selected instance");
    tablet.press("Descend into selected instance");
    assert_eq!(
        tablet
            .app
            .state
            .workspace
            .content
            .active_schematic_reference()
            .cell,
        "amp"
    );
}
