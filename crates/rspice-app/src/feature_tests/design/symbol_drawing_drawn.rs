//! Drawing in the symbol editor, through whole application frames: the shape
//! being drawn is on the canvas before it is in the document.

use crate::feature_tests::design::symbol_actions_drawn::open_symbol;
use crate::state::{Point, SymbolShape};
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
}

impl Canvas {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
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
        };
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
        } = self;
        ctx.run_ui(
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
        )
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
