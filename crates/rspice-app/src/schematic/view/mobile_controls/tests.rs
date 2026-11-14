//! The touch strip and the one-finger gestures, driven through real frames
//! by touch events the way a tablet reports them.

use super::*;
use crate::state::{ComponentType, Point, SchematicState};
use crate::workbench::state::Workspace;
use crate::workbench::{RSpiceApp, features};
use egui::{Event, Pos2, RawInput, TouchPhase, Vec2};

/// A Design workspace under a finger: the canvas and the strip, drawn as the
/// design surface draws them.
pub(crate) struct Finger {
    ctx: Context,
    pub(crate) app: RSpiceApp,
    pub(crate) canvas: Rect,
    size: Vec2,
    time: f64,
    /// What the last frame named for a screen reader, and where.
    pub(crate) named: Vec<(String, Rect)>,
}

impl Finger {
    pub(crate) fn new(size: Vec2) -> Self {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.open_project_session();
        app.state.workbench.activate(Workspace::Design);
        app.state.model.schematic = SchematicState::default();
        app.state.model.schematic.session.editor.needs_fit = false;
        app.state
            .model
            .schematic
            .session
            .editor
            .needs_drawing_sheet_fit = false;
        app.state.model.schematic.clear_undo_history();
        app.state.model.schematic.session.is_dirty = false;
        app.state.sync_active_schematic_to_workspace();
        let mut finger = Self {
            ctx,
            app,
            canvas: Rect::NOTHING,
            size,
            time: 0.0,
            named: Vec::new(),
        };
        finger.frame(Vec::new());
        finger
    }

    /// A resistor placed at `at`, committed as its own undo step.
    pub(crate) fn resistor(&mut self, at: Point) -> u64 {
        let id = self
            .app
            .state
            .model
            .schematic
            .add_component(ComponentType::Resistor, at);
        self.app.state.model.schematic.clear_undo_history();
        self.app.state.sync_active_schematic_to_workspace();
        id
    }

    pub(crate) fn frame(&mut self, events: Vec<Event>) {
        self.frame_after(0.02, events);
    }

    fn frame_after(&mut self, seconds: f64, events: Vec<Event>) {
        self.time += seconds;
        let Self {
            ctx,
            app,
            canvas,
            size,
            time,
            named,
        } = self;
        let output = ctx.run_ui(
            RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, *size)),
                events,
                focused: true,
                time: Some(*time),
                ..Default::default()
            },
            |ui| {
                app.state.reconcile_schematic_drag(ui.ctx(), false);
                *canvas = ui.available_rect_before_wrap();
                crate::schematic::view::render_schematic_view(
                    ui,
                    &mut crate::schematic::view::canvas_frame(&mut app.state),
                    None,
                );
                show(&ui.ctx().clone(), &mut strip_cx(app), *canvas);
                app.after_contribution();
                app.state.reconcile_schematic_drag(ui.ctx(), true);
            },
        );
        named.clear();
        for (_, node) in output
            .platform_output
            .accesskit_update
            .into_iter()
            .flat_map(|update| update.nodes)
        {
            if let (Some(name), Some(bounds)) = (node.label(), node.bounds()) {
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

    /// Where the last frame drew `name`.
    pub(crate) fn find(&self, name: &str) -> Option<Rect> {
        self.named
            .iter()
            .find(|(candidate, _)| candidate == name)
            .map(|(_, rect)| *rect)
    }

    /// The screen point over the world point `at` (the camera sits at zoom
    /// 1 and no pan until a gesture moves it).
    pub(crate) fn screen(&self, at: Point) -> Pos2 {
        let editor = &self.app.state.model.schematic.session.editor;
        self.canvas.min
            + egui::vec2(
                (editor.pan.0 + f64::from(at.x) * editor.zoom) as f32,
                (editor.pan.1 + f64::from(at.y) * editor.zoom) as f32,
            )
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

    /// A finger lands at `pos`: the touch, and the primary pointer the
    /// platform drives from it.
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

    pub(crate) fn tap(&mut self, pos: Pos2) {
        self.down(pos);
        self.up(pos);
        self.frame(Vec::new());
    }

    /// Tap what the last frame named `name`: a strip button or a menu row.
    pub(crate) fn press(&mut self, name: &str) {
        let at = self
            .find(name)
            .unwrap_or_else(|| panic!("no {name:?} on screen: {:?}", self.named))
            .center();
        self.tap(at);
    }

    fn slide(&mut self, pos: Pos2) {
        self.frame(vec![
            Self::touch(TouchPhase::Move, pos),
            Event::PointerMoved(pos),
        ]);
    }

    /// One finger from `from` to `to` in four steps, then lifted.
    pub(crate) fn drag(&mut self, from: Pos2, to: Pos2) {
        self.down(from);
        for step in 1..=4 {
            self.slide(from + (to - from) * (step as f32 / 4.0));
        }
        self.up(to);
        self.frame(Vec::new());
    }

    /// A finger held still past egui's long-press time, then lifted: the
    /// canvas menu opens.
    pub(crate) fn long_press(&mut self, pos: Pos2) {
        self.down(pos);
        self.frame_after(0.9, vec![Self::touch(TouchPhase::Move, pos)]);
        self.up(pos);
        self.frame(Vec::new());
    }

    /// A touch that lands and lifts with no tap, as a palm brushing the
    /// screen does: the canvas is in touch mode and nothing is clicked.
    pub(crate) fn wake(&mut self) {
        let corner = self.canvas.left_top() + egui::vec2(4.0, 4.0);
        self.frame(vec![
            Self::touch(TouchPhase::Start, corner),
            Self::touch(TouchPhase::End, corner),
        ]);
        self.frame(Vec::new());
    }

    /// Scroll the open canvas menu with a finger until the row named
    /// `name` is inside it.
    pub(crate) fn reveal(&mut self, name: &str) {
        for _ in 0..12 {
            let menu = self
                .find("Schematic selection")
                .expect("the canvas menu is open");
            let row = self
                .find(name)
                .unwrap_or_else(|| panic!("no {name:?} in the menu: {:?}", self.named));
            if menu.contains_rect(row) {
                return;
            }
            let from = menu.center();
            let travel = if row.bottom() > menu.bottom() {
                -120.0
            } else {
                120.0
            };
            self.drag(from, from + egui::vec2(0.0, travel));
        }
        panic!("{name:?} never scrolled into the menu");
    }

    pub(crate) fn pan(&self) -> (f64, f64) {
        self.app.state.model.schematic.session.editor.pan
    }

    pub(crate) fn selection(&self) -> &rspice_design::schematic::selection::Selection {
        &self.app.state.model.schematic.session.editor.selection
    }
}

/// The design feature's context, as the surface lends the strip it.
pub(crate) fn strip_cx(app: &mut RSpiceApp) -> Cx<'_, DesignFeature> {
    features::surface_cx::<DesignFeature>(app, |_| true)
}

/// Every button's command the strip asks the dispatcher about is one the
/// design feature declares it reads, in the strip's order and ahead of Show
/// in netlist and Tuning, which the navigator and the inspector read;
/// Cancel's is offered while an operation is live, and the two latches run
/// no command.
#[test]
fn the_strip_asks_only_the_answers_the_design_feature_reads() {
    let asked = [
        MobileCanvasAction::Rotate,
        MobileCanvasAction::Mirror,
        MobileCanvasAction::Undo,
        MobileCanvasAction::Redo,
        MobileCanvasAction::Fit,
    ]
    .map(|action| product_command(action).expect("a command button"));
    let foreign =
        <DesignFeature as crate::workbench::framework::feature::Feature>::FOREIGN_AVAILABILITY;
    assert_eq!(&foreign[..asked.len()], &asked);
    assert_eq!(
        &foreign[asked.len()..],
        &[
            crate::workbench::framework::command_registry::ids::SHOW_IN_NETLIST,
            crate::workbench::framework::command_registry::ids::TUNING,
        ]
    );
    assert!(!foreign.contains(&Command::Cancel.key()));
}
