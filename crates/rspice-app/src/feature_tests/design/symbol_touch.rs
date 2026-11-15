//! The symbol editor driven by a finger alone, on a tablet.
//!
//! Every frame is a whole application frame, as `eframe::App::ui` runs one,
//! and every input is a touch as a tablet reports it: the touch event and the
//! primary pointer the platform drives from the first finger.

use crate::feature_tests::design::symbol_actions_drawn::open_symbol;
use crate::state::{Point, SymbolDocument, SymbolEditorMetadata, SymbolShape};
use crate::workbench::app::*;
use egui::{Event, Pos2, Rect, TouchId, TouchPhase, Vec2};
use rspice_schematic_editor::symbol_editor::SymbolViewport;
use rspice_schematic_editor::symbol_editor::session::{SymbolSelection, SymbolTool};

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
    fn new() -> Self {
        Self::sized(Vec2::new(1024.0, 768.0))
    }

    fn sized(size: Vec2) -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = open_symbol();
        app.state.open_project_session();
        let editor = &mut app.state.design.symbol.editor;
        editor.needs_fit = false;
        editor.zoom = 2.0;
        editor.pan = (0.0, 0.0);
        let mut finger = Self {
            ctx,
            app,
            size,
            time: 0.0,
            named: Vec::new(),
        };
        finger.frame(Vec::new());
        finger.frame(Vec::new());
        finger.wake();
        finger
    }

    /// The symbol with `body` for its artwork and its pins as they were.
    fn with_body(body: Vec<SymbolShape>) -> Self {
        let mut finger = Self::new();
        let state = &mut finger.app.state;
        let mut document = state.model.load_active_symbol_document().unwrap();
        document.body = body;
        state
            .store_active_symbol_editor_bundle(
                &document,
                &SymbolEditorMetadata::for_document(&document),
            )
            .unwrap();
        finger.settle();
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

    fn touch_id(id: u64, phase: TouchPhase, pos: Pos2) -> Event {
        Event::Touch {
            device_id: egui::TouchDeviceId(1),
            id: TouchId(id),
            phase,
            pos,
            force: None,
        }
    }

    fn touch(phase: TouchPhase, pos: Pos2) -> Event {
        Self::touch_id(1, phase, pos)
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

    /// A moment with the finger off the glass, so the next tap is not
    /// counted with the last one as a double tap.
    fn settle(&mut self) {
        for _ in 0..10 {
            self.frame_after(0.1, Vec::new());
        }
    }

    /// A deliberate tap, after a moment.
    fn tap(&mut self, pos: Pos2) {
        self.settle();
        self.down(pos);
        self.up(pos);
        self.frame(Vec::new());
        self.frame(Vec::new());
    }

    /// Tap what is named `name` once the screen has settled.
    fn press(&mut self, name: &str) {
        self.settle();
        let at = self.expect(name).center();
        self.tap(at);
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
        self.settle();
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

    /// Two fingers land at `a` and `b`, move to `a2` and `b2` in four steps,
    /// and lift. The platform drives the pointer from the first finger only.
    fn pinch(&mut self, (a, b): (Pos2, Pos2), (a2, b2): (Pos2, Pos2)) {
        self.settle();
        self.down(a);
        self.frame(vec![Self::touch_id(2, TouchPhase::Start, b)]);
        for step in 1..=4 {
            let t = step as f32 / 4.0;
            let (at, bt) = (a + (a2 - a) * t, b + (b2 - b) * t);
            self.frame(vec![
                Self::touch(TouchPhase::Move, at),
                Self::touch_id(2, TouchPhase::Move, bt),
                Event::PointerMoved(at),
            ]);
        }
        self.frame(vec![Self::touch_id(2, TouchPhase::End, b2)]);
        self.up(a2);
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

    /// The symbol canvas, where the last frame drew it.
    fn canvas(&self) -> Rect {
        self.named
            .iter()
            .find(|(name, _)| name.starts_with("Symbol editor canvas."))
            .map(|(_, rect)| *rect)
            .unwrap_or_else(|| panic!("no symbol canvas on screen: {:?}", self.named))
    }

    fn viewport(&self) -> SymbolViewport {
        let editor = &self.app.state.design.symbol.editor;
        SymbolViewport::new(
            self.canvas(),
            editor.zoom,
            egui::vec2(editor.pan.0, editor.pan.1),
        )
    }

    /// The screen point over the symbol point `at`.
    fn screen(&self, at: Point) -> Pos2 {
        self.viewport().world_to_screen(at)
    }

    /// The symbol point under the screen point `at`, unrounded.
    fn world(&self, at: Pos2) -> (f32, f32) {
        let viewport = self.viewport();
        let rel = at - viewport.rect.center() - viewport.pan;
        (rel.x / viewport.zoom, rel.y / viewport.zoom)
    }

    fn pan(&self) -> (f32, f32) {
        self.app.state.design.symbol.editor.pan
    }

    fn document(&self) -> SymbolDocument {
        self.app.state.model.load_active_symbol_document().unwrap()
    }

    fn selection(&self) -> SymbolSelection {
        self.app.state.design.symbol.editor.effective_selection()
    }

    fn arm(&mut self, tool: SymbolTool) {
        self.app.state.design.symbol.editor.tool = tool;
        self.settle();
    }
}

/// A 40-unit square about the origin.
fn square() -> SymbolShape {
    SymbolShape::Rectangle {
        min: Point::new(-20, -20),
        max: Point::new(20, 20),
        style: Default::default(),
    }
}

fn square_at(x: i32, y: i32) -> SymbolShape {
    SymbolShape::Rectangle {
        min: Point::new(x - 10, y - 10),
        max: Point::new(x + 10, y + 10),
        style: Default::default(),
    }
}

/// The strip stands on the symbol's own drawing as it does on a schematic,
/// and every button runs on the symbol what its key runs there.
#[test]
fn the_touch_strip_stands_on_the_symbol_canvas() {
    let mut finger = Finger::with_body(vec![SymbolShape::Rectangle {
        min: Point::new(-20, -10),
        max: Point::new(20, 10),
        style: Default::default(),
    }]);
    let strip = finger.expect("Touch editing controls");
    assert!(
        finger.canvas().contains_rect(strip),
        "the strip {strip:?} stands on the canvas {:?}",
        finger.canvas()
    );
    assert!(
        finger.find("Fit drawing sheet").is_none(),
        "a symbol has no drawing sheet"
    );

    // Rotate, Undo and Redo run the symbol's own commands.
    finger.tap(finger.screen(Point::new(20, 0)));
    assert_eq!(finger.selection(), SymbolSelection::single_shape(0));
    let before = finger.document().body;
    finger.press("Rotate clockwise");
    let turned = finger.document().body;
    assert_ne!(turned, before, "the strip turned the selected rectangle");
    finger.press("Undo");
    assert_eq!(finger.document().body, before);
    finger.press("Redo");
    assert_eq!(finger.document().body, turned);

    // Cancel puts an armed tool down.
    finger.arm(SymbolTool::Circle);
    finger.press("Cancel");
    assert_eq!(
        finger.app.state.design.symbol.editor.tool,
        SymbolTool::Select
    );

    // Fit frames the symbol.
    finger.press("Fit symbol");
    assert_ne!(finger.app.state.design.symbol.editor.zoom, 2.0);
}

/// The placed preview keeps a column of its own; the strip stands on the
/// drawing beside it, not over the preview.
#[test]
fn the_touch_strip_keeps_off_the_placed_preview() {
    let mut finger = Finger::sized(Vec2::new(1600.0, 900.0));
    finger.app.state.design.symbol.editor.preview_as_placed = true;
    finger.settle();
    let strip = finger.expect("Touch editing controls");
    let canvas = finger.canvas();
    assert!(
        canvas.contains_rect(strip),
        "the strip {strip:?} stands on the drawing {canvas:?}"
    );
}

/// A finger takes what lies within its reach of the ink, as it takes a wire
/// on a schematic: well past where a mouse would miss.
#[test]
fn a_finger_takes_symbol_objects_and_handles_within_its_reach() {
    let mut finger = Finger::with_body(vec![square()]);
    // 12 points outside the square's right edge.
    let beside = finger.screen(Point::new(20, 0)) + egui::vec2(12.0, 0.0);
    finger.tap(beside);
    assert_eq!(finger.selection(), SymbolSelection::single_shape(0));

    // 12 points beside its top-right corner, the corner's handle follows
    // the finger.
    let corner = finger.screen(Point::new(20, -20)) + egui::vec2(9.0, -8.0);
    finger.drag(corner, corner + egui::vec2(20.0, 0.0));
    let SymbolShape::Rectangle { min, max, .. } = finger.document().body[0].clone() else {
        panic!("the square is still a rectangle");
    };
    assert!(
        min.x == -20 && max.y == 20 && max.x > 20,
        "the top right corner went right with the finger and the bottom left \
         stayed: {min:?}..{max:?}"
    );

    // 16 points from the IN pin's tip.
    finger.tap(finger.screen(Point::new(-40, 10)) + egui::vec2(-16.0, 0.0));
    assert_eq!(
        finger.selection(),
        SymbolSelection::single_pin("IN".to_owned())
    );
}

/// A finger moves only what is selected: from anywhere else its drag pans
/// the symbol, so a pan cannot pick up what it starts on.
#[test]
fn a_finger_drag_off_the_selection_pans_the_symbol() {
    let mut finger = Finger::with_body(vec![square()]);
    let empty = finger.screen(Point::new(60, -60));
    let pan = finger.pan();
    finger.drag(empty, empty + egui::vec2(60.0, 40.0));
    assert_eq!(
        finger.pan(),
        (pan.0 + 60.0, pan.1 + 40.0),
        "the view followed the finger"
    );
    assert!(finger.selection().is_empty(), "no box was drawn");

    let body = finger.document().body;
    let edge = finger.screen(Point::new(20, 0));
    let pan = finger.pan();
    finger.drag(edge, edge + egui::vec2(-40.0, 0.0));
    assert_eq!(finger.document().body, body, "the unselected square stayed");
    assert!(finger.selection().is_empty(), "and was not selected");
    assert_eq!(finger.pan(), (pan.0 - 40.0, pan.1));

    // Selected, the square moves with the finger: twenty units left.
    finger.tap(finger.screen(Point::new(20, 0)));
    let edge = finger.screen(Point::new(20, 10));
    finger.drag(edge, edge + egui::vec2(-40.0, 0.0));
    assert_eq!(
        finger.document().body[0],
        SymbolShape::Rectangle {
            min: Point::new(-40, -20),
            max: Point::new(0, 20),
            style: Default::default(),
        }
    );
}

/// With the strip's box latch on, a finger's drag off the selection draws a
/// selection box instead of panning.
#[test]
fn the_box_latch_draws_a_selection_box_on_the_symbol() {
    let mut finger = Finger::with_body(vec![square_at(-40, -40), square_at(40, -40)]);
    finger.press("Select with a box");
    let pan = finger.pan();
    finger.drag(
        finger.screen(Point::new(-60, -60)),
        finger.screen(Point::new(60, -20)),
    );
    assert_eq!(finger.selection().shapes, [0, 1].into_iter().collect());
    assert_eq!(finger.pan(), pan, "the view stayed");
}

/// With the strip's add latch on, a tap adds to the selection or takes out
/// of it, as a click with Shift does.
#[test]
fn the_add_latch_adds_to_the_symbol_selection() {
    let mut finger = Finger::with_body(vec![square_at(-40, -40), square_at(40, -40)]);
    finger.tap(finger.screen(Point::new(-30, -40)));
    assert_eq!(finger.selection(), SymbolSelection::single_shape(0));
    finger.press("Add to selection");
    finger.tap(finger.screen(Point::new(50, -40)));
    assert_eq!(finger.selection().shapes, [0, 1].into_iter().collect());
    finger.tap(finger.screen(Point::new(-30, -40)));
    assert_eq!(finger.selection(), SymbolSelection::single_shape(1));
}

/// A long press opens the canvas menu and does nothing else: with a drawing
/// tool armed it starts no shape, and the lift that ends it places no point.
#[test]
fn a_long_press_on_the_symbol_canvas_only_opens_its_menu() {
    let mut finger = Finger::with_body(vec![square()]);
    finger.arm(SymbolTool::Rectangle);
    finger.long_press(finger.screen(Point::new(60, -60)));
    assert_eq!(
        finger.app.state.design.symbol.editor.shape_start, None,
        "the press began no rectangle"
    );
    assert!(
        finger.find("Draw rectangle").is_some(),
        "the empty canvas's menu is open"
    );
    finger.press("Draw line");

    finger.tap(finger.screen(Point::new(-60, 40)));
    finger.tap(finger.screen(Point::new(-20, 40)));
    finger.long_press(finger.screen(Point::new(60, 40)));
    assert_eq!(
        finger.app.state.design.symbol.editor.pending_polyline,
        vec![Point::new(-60, 40), Point::new(-20, 40)],
        "the long press placed no point"
    );
    finger.press("Finish");
    assert_eq!(
        finger.document().body.last(),
        Some(&SymbolShape::Polyline {
            points: vec![Point::new(-60, 40), Point::new(-20, 40)],
            closed: false,
            style: Default::default(),
        })
    );
}

/// Two fingers zoom and nothing else: a shape the first finger's drag would
/// have drawn is not drawn, and a selection it would have moved stays.
#[test]
fn a_pinch_on_the_symbol_canvas_draws_and_moves_nothing() {
    let mut finger = Finger::with_body(vec![square()]);
    finger.arm(SymbolTool::Line);
    let centre = finger.screen(Point::new(0, 60));
    let zoom = finger.app.state.design.symbol.editor.zoom;
    finger.pinch(
        (
            centre - egui::vec2(40.0, 0.0),
            centre + egui::vec2(40.0, 0.0),
        ),
        (
            centre - egui::vec2(80.0, 0.0),
            centre + egui::vec2(80.0, 0.0),
        ),
    );
    assert_eq!(finger.document().body, vec![square()], "no line was drawn");
    assert!(!finger.app.state.design.symbol.editor.has_draft());
    assert!(finger.app.state.design.symbol.editor.zoom > zoom * 1.5);

    finger.arm(SymbolTool::Select);
    finger.tap(finger.screen(Point::new(20, 0)));
    let edge = finger.screen(Point::new(20, 10));
    finger.pinch(
        (edge, edge + egui::vec2(120.0, 0.0)),
        (edge - egui::vec2(30.0, 0.0), edge + egui::vec2(150.0, 0.0)),
    );
    assert_eq!(finger.document().body, vec![square()], "the square stayed");
}

/// What lies between the fingers stays between them as they spread.
#[test]
fn a_pinch_zooms_the_symbol_about_the_point_between_the_fingers() {
    let mut finger = Finger::with_body(vec![square()]);
    let centre = finger.canvas().center() + egui::vec2(-60.0, 30.0);
    let under = finger.world(centre);
    finger.pinch(
        (
            centre - egui::vec2(40.0, 0.0),
            centre + egui::vec2(40.0, 0.0),
        ),
        (
            centre - egui::vec2(120.0, 0.0),
            centre + egui::vec2(120.0, 0.0),
        ),
    );
    let zoom = finger.app.state.design.symbol.editor.zoom;
    assert!((zoom - 6.0).abs() < 0.01, "three times as close: {zoom}");
    let after = finger.world(centre);
    assert!(
        (after.0 - under.0).abs() < 0.5 && (after.1 - under.1).abs() < 0.5,
        "{under:?} stayed between the fingers, not {after:?}"
    );
}
