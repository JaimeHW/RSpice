//! The schematic driven by a finger alone, on a phone and on a tablet.
//!
//! Every frame is a whole application frame, as `eframe::App::ui` runs one,
//! and every input is a touch as a tablet reports it: the touch event and the
//! primary pointer the platform drives from it. Nothing is set up by hand once
//! the sheet exists.

use super::*;
use crate::state::{SchematicState, Tool};
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

    fn tap(&mut self, pos: Pos2) {
        self.down(pos);
        self.up(pos);
        self.frame(Vec::new());
        self.frame(Vec::new());
    }

    /// Tap what the last frame named `name`.
    fn press(&mut self, name: &str) {
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
