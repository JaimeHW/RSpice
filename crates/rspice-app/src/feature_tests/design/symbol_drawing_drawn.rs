//! Drawing in the symbol editor, through whole application frames: the shape
//! being drawn is on the canvas before it is in the document, a drawn shape
//! is reshaped by its handles, and the canvas's context menu runs the same
//! commands its keys do.

use std::collections::HashMap;

use crate::feature_tests::design::symbol_actions_drawn::open_symbol;
use crate::state::{Point, SymbolEditorMetadata, SymbolShape};
use crate::workbench::RSpiceApp;
use crate::workbench::chrome::status_bar::design::symbol_tool_hint;
use crate::workbench::design::commands::DesignCommand;
use egui::epaint::ColorMode;
use rspice_schematic_editor::symbol_editor::SymbolViewport;
use rspice_schematic_editor::symbol_editor::session::SymbolTool;

struct Canvas {
    ctx: egui::Context,
    app: RSpiceApp,
    time: f64,
    viewport: SymbolViewport,
    /// Every labelled control the last frame drew: where it is, and the key
    /// it shows.
    controls: HashMap<String, (egui::Rect, Option<String>)>,
}

impl Canvas {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = open_symbol();
        let editor = &mut app.state.design.symbol.editor;
        editor.needs_fit = false;
        editor.zoom = 2.0;
        editor.pan = (0.0, 0.0);
        let mut canvas = Self {
            ctx,
            app,
            time: 0.0,
            viewport: SymbolViewport::new(egui::Rect::NOTHING, 2.0, egui::Vec2::ZERO),
            controls: HashMap::new(),
        };
        canvas.frame(vec![]);
        canvas
    }

    /// The symbol with `body` for its artwork and its pins as they were.
    fn with_body(body: Vec<SymbolShape>) -> Self {
        let mut canvas = Self::new();
        let state = &mut canvas.app.state;
        let mut document = state.model.load_active_symbol_document().unwrap();
        document.body = body;
        state
            .store_active_symbol_editor_bundle(
                &document,
                &SymbolEditorMetadata::for_document(&document),
            )
            .unwrap();
        canvas.frame(vec![]);
        canvas
    }

    /// One frame of the symbol view. Frames are a quarter second apart, so
    /// two clicks are never one double-click.
    fn frame(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        self.time += 0.25;
        let Self {
            ctx,
            app,
            time,
            viewport,
            controls,
        } = self;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                events,
                focused: true,
                time: Some(*time),
                ..Default::default()
            },
            |ui| {
                let editor = &app.state.design.symbol.editor;
                *viewport = SymbolViewport::new(
                    ui.available_rect_before_wrap(),
                    editor.zoom,
                    egui::vec2(editor.pan.0, editor.pan.1),
                );
                app.handle_shortcuts(ui.ctx());
                crate::schematic::symbol_editor::show(
                    ui,
                    &mut crate::schematic::view::canvas_frame(&mut app.state),
                    None,
                );
            },
        );
        // What a row raised runs once the surface has drawn, as in the
        // application.
        app.after_contribution();
        if let Some(update) = &output.platform_output.accesskit_update {
            controls.clear();
            for (_, node) in &update.nodes {
                if let (Some(label), Some(bounds)) = (node.label(), node.bounds()) {
                    controls.insert(
                        label.to_owned(),
                        (
                            egui::Rect::from_min_max(
                                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                            ),
                            node.keyboard_shortcut().map(str::to_owned),
                        ),
                    );
                }
            }
        }
        output
    }

    fn at(&self, point: Point) -> egui::Pos2 {
        self.viewport.world_to_screen(point)
    }

    fn button(&self, point: Point, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: self.at(point),
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn click(&mut self, point: Point) {
        self.frame(vec![egui::Event::PointerMoved(self.at(point))]);
        self.frame(vec![self.button(point, true)]);
        self.frame(vec![self.button(point, false)]);
    }

    fn right_click(&mut self, point: Point) {
        let pos = self.at(point);
        let button = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Secondary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![egui::Event::PointerMoved(pos)]);
        self.frame(vec![button(true)]);
        self.frame(vec![button(false)]);
        // A menu lays itself out unseen the frame it opens.
        self.frame(vec![]);
    }

    /// Click the control the last frame labelled `label`.
    fn click_control(&mut self, label: &str) {
        let pos = self.controls[label].0.center();
        let button = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![egui::Event::PointerMoved(pos)]);
        self.frame(vec![button(true)]);
        self.frame(vec![button(false)]);
    }

    /// The key the control labelled `label` shows.
    fn shortcut(&self, label: &str) -> Option<&str> {
        self.controls.get(label)?.1.as_deref()
    }

    /// Press at `from`, carry the pointer to `to` and let go there, with
    /// `midway` sent while the button is held halfway.
    fn drag_with(&mut self, from: Point, to: Point, midway: Vec<egui::Event>) {
        self.frame(vec![egui::Event::PointerMoved(self.at(from))]);
        self.frame(vec![self.button(from, true)]);
        let half = self.at(from) + (self.at(to) - self.at(from)) * 0.5;
        self.frame(vec![egui::Event::PointerMoved(half)]);
        self.frame(midway);
        self.frame(vec![egui::Event::PointerMoved(self.at(to))]);
        self.frame(vec![self.button(to, false)]);
    }

    fn drag(&mut self, from: Point, to: Point) {
        self.drag_with(from, to, Vec::new());
    }

    /// The step Undo would take back, by name.
    fn undo_step(&self) -> Option<String> {
        let key = self.app.state.model.workspace.content.active_key();
        self.app
            .state
            .design
            .symbol
            .history
            .undo_step(&key)
            .map(str::to_owned)
    }

    /// The frame painted with the pointer resting at `point`.
    fn hover(&mut self, point: Point) -> egui::FullOutput {
        self.frame(vec![egui::Event::PointerMoved(self.at(point))]);
        self.frame(vec![])
    }

    fn key(&mut self, key: egui::Key) {
        let event = |pressed| egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        self.frame(vec![event(true)]);
        self.frame(vec![event(false)]);
    }

    fn arm(&mut self, command: DesignCommand) {
        self.app.execute_command_key(command.key());
        self.frame(vec![]);
    }

    fn tool(&self) -> SymbolTool {
        self.app.state.design.symbol.editor.tool
    }

    fn body(&self) -> Vec<SymbolShape> {
        self.app
            .state
            .model
            .load_active_symbol_document()
            .unwrap()
            .body
    }

    fn hint(&self) -> Option<&'static str> {
        symbol_tool_hint(
            &self.app.state.design.symbol,
            &self.app.state.model,
            &self.app.state.workbench,
        )
    }

    /// The draft's two tones: the points placed, and what follows the pointer.
    fn draft_colours(&self) -> (egui::Color32, egui::Color32) {
        let placed = crate::ui::tokens::Tokens::get(&self.ctx)
            .color
            .canvas_select
            .gamma_multiply(0.85);
        (placed, placed.gamma_multiply(0.6))
    }
}

/// Every path `output` painted in `colour`, by its points.
fn paths_in(output: &egui::FullOutput, colour: egui::Color32) -> Vec<(Vec<egui::Pos2>, bool)> {
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::Path(path) if path.stroke.color == ColorMode::Solid(colour) => {
                Some((path.points.clone(), path.closed))
            }
            _ => None,
        })
        .collect()
}

/// Whether `output` painted the ring that marks where a click would land at
/// `at`.
fn marks_landing_at(output: &egui::FullOutput, at: egui::Pos2, colour: egui::Color32) -> bool {
    output.shapes.iter().any(|clipped| {
        matches!(&clipped.shape, egui::Shape::Circle(circle)
            if circle.center == at && circle.radius == 5.0 && circle.stroke.color == colour)
    })
}

/// A line part drawn shows its placed segments, a rubber band from the last
/// point to the snapped pointer and a mark where the next click lands; the
/// status bar says what the tool takes next; Enter puts the line in the
/// document and the tool stays armed for another.
#[test]
fn a_line_being_drawn_shows_its_segments_and_a_rubber_band_to_the_pointer() {
    let mut canvas = Canvas::new();
    let before = canvas.body();
    canvas.arm(DesignCommand::SymbolPolylineTool);
    assert_eq!(canvas.hint(), Some("Line: click the first point"));
    let (a, b, c) = (
        Point::new(-30, -30),
        Point::new(30, -30),
        Point::new(30, 20),
    );
    canvas.click(a);
    canvas.click(b);
    // Just off the grid point a click there would land on.
    let output = canvas.hover(Point::new(32, 19));

    let (placed, following) = canvas.draft_colours();
    assert!(
        paths_in(&output, placed).contains(&(vec![canvas.at(a), canvas.at(b)], false)),
        "the placed segment: {:?}",
        paths_in(&output, placed)
    );
    assert!(
        paths_in(&output, following)
            .contains(&(vec![canvas.at(a), canvas.at(b), canvas.at(c)], false)),
        "the rubber band to the snapped pointer: {:?}",
        paths_in(&output, following)
    );
    assert!(marks_landing_at(&output, canvas.at(c), placed));
    assert_eq!(
        canvas.body(),
        before,
        "nothing is drawn until it is finished"
    );
    assert_eq!(
        canvas.hint(),
        Some("Line: click the next point, Enter to finish")
    );

    canvas.key(egui::Key::Enter);

    let body = canvas.body();
    assert_eq!(body.len(), before.len() + 1);
    assert_eq!(
        body.last(),
        Some(&SymbolShape::Polyline {
            points: vec![a, b],
            closed: false,
        })
    );
    assert_eq!(canvas.tool(), SymbolTool::Line, "the tool stays armed");
    let output = canvas.hover(c);
    assert!(
        paths_in(&output, following)
            .iter()
            .all(|(points, _)| points.len() < 2)
    );
}

/// A rectangle being drawn shows its outline from the first corner to the
/// pointer. Escape lets it go with the tool still armed, and a second Escape
/// puts the tool down.
#[test]
fn a_rectangle_being_drawn_shows_its_outline_and_escape_lets_it_go() {
    let mut canvas = Canvas::new();
    let before = canvas.body();
    canvas.arm(DesignCommand::SymbolRectangleTool);
    let (corner, opposite) = (Point::new(-20, -20), Point::new(40, 30));
    canvas.click(corner);
    let output = canvas.hover(opposite);

    let (_, following) = canvas.draft_colours();
    let outline = [
        corner,
        Point::new(opposite.x, corner.y),
        opposite,
        Point::new(corner.x, opposite.y),
    ]
    .map(|point| canvas.at(point))
    .to_vec();
    assert!(
        paths_in(&output, following).contains(&(outline, true)),
        "{:?}",
        paths_in(&output, following)
    );
    assert_eq!(canvas.hint(), Some("Rectangle: click the opposite corner"));

    canvas.key(egui::Key::Escape);

    assert!(!canvas.app.state.design.symbol.editor.has_draft());
    assert_eq!(canvas.tool(), SymbolTool::Rectangle);
    assert_eq!(canvas.body(), before);

    canvas.key(egui::Key::Escape);

    assert_eq!(canvas.tool(), SymbolTool::Select);
    assert_eq!(canvas.hint(), None);
}

/// Text opens an entry where the tool was clicked; what is typed is placed
/// there on Enter, as one shape, and Escape on the next one places nothing.
#[test]
fn text_is_typed_where_it_was_clicked_and_placed_on_enter() {
    let mut canvas = Canvas::new();
    let before = canvas.body();
    canvas.arm(DesignCommand::SymbolTextTool);
    let anchor = Point::new(0, -40);
    canvas.click(anchor);
    assert!(canvas.app.state.design.symbol.editor.text_draft.is_some());
    canvas.frame(vec![egui::Event::Text("GAIN".to_owned())]);
    assert_eq!(
        canvas
            .app
            .state
            .design
            .symbol
            .editor
            .text_draft
            .as_ref()
            .map(|draft| draft.text.as_str()),
        Some("GAIN")
    );
    assert_eq!(canvas.body(), before);

    canvas.key(egui::Key::Enter);

    let body = canvas.body();
    assert_eq!(body.len(), before.len() + 1);
    assert!(
        matches!(body.last(), Some(SymbolShape::Text { anchor: at, text, .. })
            if *at == anchor && text == "GAIN"),
        "{:?}",
        body.last()
    );
    assert!(canvas.app.state.design.symbol.editor.text_draft.is_none());
    assert_eq!(canvas.tool(), SymbolTool::Text);

    canvas.click(Point::new(40, -40));
    canvas.frame(vec![egui::Event::Text("VOUT".to_owned())]);
    canvas.key(egui::Key::Escape);

    assert!(canvas.app.state.design.symbol.editor.text_draft.is_none());
    assert_eq!(canvas.body(), body);
    assert_eq!(canvas.tool(), SymbolTool::Text);
}

/// Every straight segment `output` painted in `colour`, by its ends.
fn segments_in(output: &egui::FullOutput, colour: egui::Color32) -> Vec<[egui::Pos2; 2]> {
    output
        .shapes
        .iter()
        .filter_map(|clipped| match &clipped.shape {
            egui::Shape::LineSegment { points, stroke } if stroke.color == colour => Some(*points),
            _ => None,
        })
        .collect()
}

/// The Pin tool shows the pin it would place at the pointer, in the draft
/// tone, and R turns it rather than arming the rectangle. A click puts it
/// there with its name selected for typing; Enter keeps what was typed, and
/// the tool stays armed for the next pin, which it makes when none waits.
#[test]
fn the_pin_tool_shows_its_pin_places_it_and_takes_its_name() {
    let mut canvas = Canvas::new();
    canvas.arm(DesignCommand::SymbolPinTool);
    assert_eq!(canvas.tool(), SymbolTool::PlacePin);
    assert_eq!(
        canvas.hint(),
        Some("Pin: click where its tip goes, R to turn it")
    );
    let tip = Point::new(-40, -30);
    let output = canvas.hover(tip);
    let (placed, _) = canvas.draft_colours();
    let ghost = segments_in(&output, placed);
    assert!(
        ghost.iter().any(|[from, _]| *from == canvas.at(tip)),
        "the pin about to be placed runs its lead from the pointer: {ghost:?}"
    );

    canvas.key(egui::Key::R);
    assert_eq!(canvas.tool(), SymbolTool::PlacePin, "R is the pin's");
    let turned = canvas
        .app
        .state
        .design
        .symbol
        .editor
        .pin_side
        .expect("R turned the pin");

    canvas.click(tip);
    let document = |canvas: &Canvas| {
        canvas
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap()
    };
    let pin = document(&canvas)
        .pin("in_COPY")
        .cloned()
        .expect("the waiting pin");
    assert_eq!((pin.position, pin.side), (Some(tip), turned));
    assert_eq!(canvas.hint(), Some("Pin: type its name, Enter to keep it"));
    canvas.frame(vec![egui::Event::Text("CLK".to_owned())]);
    canvas.key(egui::Key::Enter);

    let named = document(&canvas);
    assert!(named.pin("in_COPY").is_none());
    assert_eq!(named.pin("CLK").and_then(|pin| pin.position), Some(tip));
    assert_eq!(canvas.tool(), SymbolTool::PlacePin, "the tool stays armed");
    assert!(
        canvas
            .app
            .state
            .design
            .symbol
            .editor
            .pin_name_draft
            .is_none()
    );

    let next = Point::new(40, -30);
    canvas.click(next);
    assert_eq!(
        document(&canvas).pin("PIN1").and_then(|pin| pin.position),
        Some(next),
        "with no pin waiting the tool makes one"
    );
    canvas.key(egui::Key::Escape);
    assert!(
        document(&canvas).pin("PIN1").is_some(),
        "Escape keeps the name it has"
    );
    assert_eq!(canvas.tool(), SymbolTool::PlacePin);
}

/// Whether `output` painted a filled handle of `radius` at `at`.
fn fills_handle_at(output: &egui::FullOutput, at: egui::Pos2, radius: f32) -> bool {
    output.shapes.iter().any(|clipped| {
        matches!(&clipped.shape, egui::Shape::Circle(circle)
            if circle.center == at && circle.radius == radius && circle.fill != egui::Color32::TRANSPARENT)
    })
}

/// A selected line shows a ring on each of its points, filled under the
/// pointer. Dragging one carries that point alone, snapped as drawing
/// snaps, as one undo step named for it.
#[test]
fn dragging_a_selected_line_point_moves_that_point_as_one_step() {
    let line = SymbolShape::Polyline {
        points: vec![Point::new(-60, -40), Point::new(0, -40), Point::new(0, 40)],
        closed: false,
    };
    let mut canvas = Canvas::with_body(vec![line.clone()]);
    canvas.app.state.design.symbol.editor.select_shape(0);
    let output = canvas.hover(Point::new(0, -40));
    assert!(
        fills_handle_at(&output, canvas.at(Point::new(0, -40)), 3.0),
        "the point under the pointer is marked"
    );

    canvas.drag(Point::new(0, -40), Point::new(22, -51));

    assert_eq!(
        canvas.body(),
        vec![SymbolShape::Polyline {
            points: vec![Point::new(-60, -40), Point::new(20, -50), Point::new(0, 40)],
            closed: false,
        }]
    );
    assert_eq!(canvas.undo_step().as_deref(), Some("Move vertex"));
    assert!(canvas.app.state.undo_active_symbol_document().unwrap());
    assert_eq!(canvas.body(), vec![line]);
}

/// A rectangle's corner carries that corner and holds the opposite one, and
/// the rectangle stays one.
#[test]
fn dragging_a_rectangle_corner_resizes_it() {
    let mut canvas = Canvas::with_body(vec![SymbolShape::Rectangle {
        min: Point::new(-40, -20),
        max: Point::new(40, 20),
    }]);
    canvas.app.state.design.symbol.editor.select_shape(0);
    canvas.frame(vec![]);

    canvas.drag(Point::new(40, 20), Point::new(60, 40));

    assert_eq!(
        canvas.body(),
        vec![SymbolShape::Rectangle {
            min: Point::new(-40, -20),
            max: Point::new(60, 40),
        }]
    );
    assert_eq!(canvas.undo_step().as_deref(), Some("Resize rectangle"));
}

/// An arc's end turns the arc about its centre the way it was drawn and
/// keeps where it starts.
#[test]
fn dragging_an_arc_end_turns_the_arc() {
    let mut canvas = Canvas::with_body(vec![SymbolShape::Arc {
        center: Point::origin(),
        radius: 40,
        start_degrees: 0,
        sweep_degrees: 90,
    }]);
    canvas.app.state.design.symbol.editor.select_shape(0);
    canvas.frame(vec![]);

    // Clockwise on the screen from east to south; the end goes west.
    canvas.drag(Point::new(0, 40), Point::new(-40, 0));

    assert_eq!(
        canvas.body(),
        vec![SymbolShape::Arc {
            center: Point::origin(),
            radius: 40,
            start_degrees: 0,
            sweep_degrees: 180,
        }]
    );
    assert_eq!(canvas.undo_step().as_deref(), Some("Move arc end"));
}

/// A right-click while a line is being drawn opens the drawing's menu and
/// leaves the line as it was. Its rows show the keys they answer to, and
/// Finish finishes the line as Enter does.
#[test]
fn a_right_click_while_drawing_opens_the_drawing_menu() {
    let mut canvas = Canvas::new();
    let before = canvas.body();
    canvas.arm(DesignCommand::SymbolPolylineTool);
    let (a, b) = (Point::new(-30, -30), Point::new(30, -30));
    canvas.click(a);
    canvas.click(b);

    canvas.right_click(Point::new(30, 20));

    assert_eq!(
        canvas.app.state.design.symbol.editor.pending_polyline,
        vec![a, b],
        "the right-click neither finished nor let go of the line"
    );
    assert_eq!(canvas.tool(), SymbolTool::Line);
    assert_eq!(canvas.shortcut("Finish"), Some("Enter"));
    assert_eq!(canvas.shortcut("Remove last point"), Some("Backspace"));
    assert!(canvas.controls.contains_key("Cancel"));
    assert!(
        !canvas.controls.contains_key("Delete"),
        "the drawing's menu is not the selection's"
    );

    canvas.click_control("Finish");

    let body = canvas.body();
    assert_eq!(body.len(), before.len() + 1);
    assert_eq!(
        body.last(),
        Some(&SymbolShape::Polyline {
            points: vec![a, b],
            closed: false,
        })
    );
    assert_eq!(canvas.tool(), SymbolTool::Line, "the tool stays armed");
}

/// A right-click on a shape selects it and opens the selection's menu,
/// whose Rotate runs Rotate: one undo step, about the shape's middle.
#[test]
fn a_right_click_on_a_shape_opens_its_menu_and_rotate_turns_it() {
    let mut canvas = Canvas::with_body(vec![SymbolShape::Rectangle {
        min: Point::new(60, 40),
        max: Point::new(100, 60),
    }]);
    canvas.right_click(Point::new(80, 40));
    assert_eq!(
        canvas
            .app
            .state
            .design
            .symbol
            .editor
            .effective_selection()
            .shapes,
        [0].into()
    );
    assert!(canvas.controls.contains_key("Rotate 90°"));
    assert!(canvas.controls.contains_key("Object properties…"));

    canvas.click_control("Rotate 90°");

    assert_eq!(
        canvas.body(),
        vec![SymbolShape::Rectangle {
            min: Point::new(70, 30),
            max: Point::new(90, 70),
        }]
    );
    assert_eq!(canvas.undo_step().as_deref(), Some("Rotate rectangle"));
}

/// R while a pin is dragged turns its lead about its tip, and the turn is
/// part of the drag's one undo step.
#[test]
fn r_turns_a_pin_while_it_is_dragged() {
    let mut canvas = Canvas::new();
    let pin = |canvas: &Canvas| {
        let pin = canvas
            .app
            .state
            .model
            .load_active_symbol_document()
            .unwrap()
            .pin("IN")
            .cloned()
            .unwrap();
        (pin.position, pin.side)
    };
    let (placed, side) = pin(&canvas);
    let key = |pressed| egui::Event::Key {
        key: egui::Key::R,
        physical_key: Some(egui::Key::R),
        pressed,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };

    canvas.drag_with(
        Point::new(-40, 10),
        Point::new(-40, 40),
        vec![key(true), key(false)],
    );

    assert_eq!(
        pin(&canvas),
        (Some(Point::new(-40, 40)), side.turned_clockwise())
    );
    assert_eq!(canvas.tool(), SymbolTool::Select, "R was the pin's");
    assert_eq!(canvas.undo_step().as_deref(), Some("Move pin IN"));
    assert!(canvas.app.state.undo_active_symbol_document().unwrap());
    assert_eq!(pin(&canvas), (placed, side));
}
