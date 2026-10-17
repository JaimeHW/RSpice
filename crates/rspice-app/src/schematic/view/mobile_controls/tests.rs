//! The touch strip and the one-finger gestures, driven through real frames
//! by touch events the way a tablet reports them.

use super::*;
use crate::state::{ComponentType, Point, SchematicState};
use crate::workbench::state::Workspace;
use egui::{Event, Pos2, RawInput, TouchPhase, Vec2};
use rspice_schematic_editor::view::navigation::ZoomRequest;

/// A Design workspace under a finger: the canvas and the strip, drawn as the
/// design surface draws them.
struct Finger {
    ctx: Context,
    app: RSpiceApp,
    canvas: Rect,
    size: Vec2,
    time: f64,
    /// What the last frame named for a screen reader, and where.
    named: Vec<(String, Rect)>,
}

impl Finger {
    fn new(size: Vec2) -> Self {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.authority.open_session();
        app.state.workbench.activate(Workspace::Design);
        app.state.schematic = SchematicState::default();
        app.state.schematic.session.editor.needs_fit = false;
        app.state.schematic.session.editor.needs_drawing_sheet_fit = false;
        app.state.schematic.clear_undo_history();
        app.state.schematic.session.is_dirty = false;
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
    fn resistor(&mut self, at: Point) -> u64 {
        let id = self
            .app
            .state
            .schematic
            .add_component(ComponentType::Resistor, at);
        self.app.state.schematic.clear_undo_history();
        self.app.state.sync_active_schematic_to_workspace();
        id
    }

    fn frame(&mut self, events: Vec<Event>) {
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
                crate::schematic::view::render_schematic_view(ui, &mut app.state, None);
                show(&ui.ctx().clone(), app, *canvas);
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
    fn find(&self, name: &str) -> Option<Rect> {
        self.named
            .iter()
            .find(|(candidate, _)| candidate == name)
            .map(|(_, rect)| *rect)
    }

    /// The screen point over the world point `at` (the camera sits at zoom
    /// 1 and no pan until a gesture moves it).
    fn screen(&self, at: Point) -> Pos2 {
        let editor = &self.app.state.schematic.session.editor;
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

    fn tap(&mut self, pos: Pos2) {
        self.down(pos);
        self.up(pos);
        self.frame(Vec::new());
    }

    /// Tap the strip button named `name`.
    fn press(&mut self, name: &str) {
        let at = self
            .find(name)
            .unwrap_or_else(|| panic!("no {name:?} on screen: {:?}", self.named))
            .center();
        self.tap(at);
    }
}

/// G-12: a landscape iPad is 1180 pt wide, so the old 620 px breakpoint
/// gave a tablet no touch controls at all. Touch input, not width, decides;
/// a mouse hides the strip at any width.
#[test]
fn the_strip_follows_touch_input_at_every_width() {
    for size in [
        Vec2::new(1180.0, 820.0),
        Vec2::new(1024.0, 768.0),
        Vec2::new(390.0, 844.0),
    ] {
        let mut finger = Finger::new(size);
        assert!(finger.find("Touch editing controls").is_none(), "{size:?}");
        let middle = finger.canvas.center();
        finger.tap(middle);
        assert!(
            finger.find("Touch editing controls").is_some(),
            "a touch at {size:?} shows the strip"
        );
        finger.frame(vec![Event::PointerMoved(middle + egui::vec2(5.0, 5.0))]);
        assert!(
            finger.find("Touch editing controls").is_none(),
            "a mouse at {size:?} hides it"
        );
    }
}

/// Each command button runs the product command a key would.
#[test]
fn the_strip_buttons_run_the_canvas_commands() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    let id = finger.resistor(Point::new(200, 200));
    finger.tap(finger.screen(Point::new(200, 200)));
    assert!(
        finger
            .app
            .state
            .schematic
            .session
            .editor
            .selection
            .has_component(id)
    );
    let pose = |finger: &Finger| {
        let part = &finger.app.state.schematic.document().components[0];
        (part.rotation, part.mirror_h, part.mirror_v)
    };
    let (before, ..) = pose(&finger);

    finger.press("Rotate clockwise");
    assert_eq!(
        pose(&finger).0,
        before.rotate_cw(),
        "Rotate turns the selection"
    );
    let turned = pose(&finger);
    finger.press("Mirror");
    let mirrored = pose(&finger);
    assert_ne!(mirrored, turned, "Mirror flips it");
    finger.press("Undo");
    assert_eq!(pose(&finger), turned, "Undo takes the flip back");
    finger.press("Redo");
    assert_eq!(pose(&finger), mirrored, "Redo puts it back");

    finger.app.state.schematic.session.editor.zoom = 6.0;
    finger.frame(Vec::new());
    finger.press("Fit drawing sheet");
    assert!(
        finger.app.state.schematic.session.editor.zoom < 6.0,
        "Fit frames the sheet"
    );
}

/// Cancel is offered only while there is something to cancel, and it
/// cancels it: here, a wire tool's route under way.
#[test]
fn cancel_is_live_only_while_an_operation_is() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    finger.tap(finger.canvas.center());
    assert!(!operation_live(&finger.app));
    finger
        .app
        .state
        .schematic
        .arm_tool(crate::state::Tool::Wire);
    assert!(operation_live(&finger.app), "an armed drawing tool is live");
    finger.frame(Vec::new());
    finger.press("Cancel");
    assert_eq!(
        finger.app.state.schematic.session.editor.tool,
        crate::state::Tool::Select,
        "Cancel puts the wire tool down, as Esc does"
    );
    assert!(!operation_live(&finger.app));
}

#[test]
fn touch_requests_recheck_document_route_and_modal_state() {
    for change in ["document", "route", "modal", "read-only", "none"] {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Design);
        app.state.schematic.session.editor.needs_drawing_sheet_fit = false;
        let request = MobileCanvasRequest {
            source: super::super::requests::editor_request_source(&app.state),
            action: MobileCanvasAction::Fit,
        };
        match change {
            "document" => app.state.bump_active_schematic_epoch(),
            "route" => app.state.workbench.activate(Workspace::Results),
            "modal" => app.state.dialogs.preferences_open = true,
            "read-only" => app.state.schematic.session.read_only = true,
            "none" => {}
            _ => unreachable!(),
        }
        execute_action(&Context::default(), &mut app, request);
        assert_eq!(
            app.state.schematic.session.editor.needs_drawing_sheet_fit,
            matches!(change, "none" | "read-only"),
            "{change}"
        );
    }
}

/// The view buttons dispatch the same `Command` as every other surface,
/// so a fit from the strip is the fit from the View menu and `F`.
#[test]
fn fit_executes_the_schematic_camera_command() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.session.editor.zoom = 2.0;
    app.state.schematic.session.editor.zoom_request = Some(ZoomRequest::By(1.2));
    let request = MobileCanvasRequest {
        source: super::super::requests::editor_request_source(&app.state),
        action: MobileCanvasAction::Fit,
    };
    execute_action(&Context::default(), &mut app, request);
    assert!(app.state.schematic.session.editor.needs_drawing_sheet_fit);
    assert!(!app.state.schematic.session.editor.needs_fit);
}
