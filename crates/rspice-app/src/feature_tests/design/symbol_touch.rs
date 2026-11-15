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
