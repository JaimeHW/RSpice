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

    /// Tap what the last frame named `name`: a strip button or a menu row.
    fn press(&mut self, name: &str) {
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
    fn drag(&mut self, from: Pos2, to: Pos2) {
        self.down(from);
        for step in 1..=4 {
            self.slide(from + (to - from) * (step as f32 / 4.0));
        }
        self.up(to);
        self.frame(Vec::new());
    }

    /// A finger held still past egui's long-press time, then lifted: the
    /// canvas menu opens.
    fn long_press(&mut self, pos: Pos2) {
        self.down(pos);
        self.frame_after(0.9, vec![Self::touch(TouchPhase::Move, pos)]);
        self.up(pos);
        self.frame(Vec::new());
    }

    /// A touch that lands and lifts with no tap, as a palm brushing the
    /// screen does: the canvas is in touch mode and nothing is clicked.
    fn wake(&mut self) {
        let corner = self.canvas.left_top() + egui::vec2(4.0, 4.0);
        self.frame(vec![
            Self::touch(TouchPhase::Start, corner),
            Self::touch(TouchPhase::End, corner),
        ]);
        self.frame(Vec::new());
    }

    /// Scroll the open canvas menu with a finger until the row named
    /// `name` is inside it.
    fn reveal(&mut self, name: &str) {
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
            let travel = if row.top() > menu.bottom() {
                -120.0
            } else {
                120.0
            };
            self.drag(from, from + egui::vec2(0.0, travel));
        }
        panic!("{name:?} never scrolled into the menu");
    }

    fn pan(&self) -> (f64, f64) {
        self.app.state.schematic.session.editor.pan
    }

    fn selection(&self) -> &rspice_design::schematic::selection::Selection {
        &self.app.state.schematic.session.editor.selection
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

/// E-11, R-3: a finger dragged across a part that is not selected pans the
/// sheet and leaves the part where it is. On a dense sheet there is little
/// empty canvas to pan from, and the strip once said "Drag to pan" while
/// this drag moved the part.
#[test]
fn a_finger_drag_on_an_unselected_part_pans_and_leaves_it() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    finger.resistor(Point::new(200, 200));
    let from = finger.screen(Point::new(200, 200));
    let before = finger.pan();
    finger.drag(from, from + egui::vec2(120.0, 40.0));

    assert_eq!(
        finger.app.state.schematic.document().components[0].pos,
        Point::new(200, 200),
        "the part stays put"
    );
    let after = finger.pan();
    assert_eq!((after.0 - before.0, after.1 - before.1), (120.0, 40.0));
    assert!(finger.selection().is_empty(), "a pan selects nothing");
    assert!(!finger.app.state.schematic.can_undo(), "a pan is no edit");
}

/// A finger on empty canvas pans too; with the box latch on, the same drag
/// draws a selection box instead, as a mouse drag there does.
#[test]
fn a_finger_drag_on_empty_canvas_pans_unless_the_box_latch_is_on() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    let id = finger.resistor(Point::new(200, 200));
    let empty = finger.screen(Point::new(400, 400));
    let before = finger.pan();
    finger.drag(empty, empty + egui::vec2(-40.0, 80.0));
    let after = finger.pan();
    assert_eq!((after.0 - before.0, after.1 - before.1), (-40.0, 80.0));
    assert!(finger.selection().is_empty());

    finger.press("Select with a box");
    let still = finger.pan();
    let corner = finger.screen(Point::new(120, 120));
    finger.drag(corner, finger.screen(Point::new(300, 300)));
    assert!(
        finger.selection().has_component(id),
        "the box takes the part"
    );
    assert_eq!(finger.pan(), still, "a box does not pan");
}

/// Only a drag that starts on the selection moves it.
#[test]
fn a_finger_drag_on_the_selection_moves_it() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    let id = finger.resistor(Point::new(200, 200));
    let at = finger.screen(Point::new(200, 200));
    finger.tap(at);
    assert!(finger.selection().has_component(id));
    let before = finger.pan();
    finger.drag(at, at + egui::vec2(60.0, 0.0));
    assert_eq!(
        finger.app.state.schematic.document().components[0].pos,
        Point::new(260, 200)
    );
    assert_eq!(finger.pan(), before, "a move does not pan");
}

/// The add latch stands in for Shift: a tap takes an object into the
/// selection or out of it.
#[test]
fn the_add_latch_makes_a_tap_add_or_remove() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    let first = finger.resistor(Point::new(200, 200));
    let second = finger.resistor(Point::new(400, 200));
    finger.tap(finger.screen(Point::new(200, 200)));
    finger.press("Add to selection");
    finger.tap(finger.screen(Point::new(400, 200)));
    assert!(finger.selection().has_component(first));
    assert!(finger.selection().has_component(second));
    finger.tap(finger.screen(Point::new(200, 200)));
    assert!(
        !finger.selection().has_component(first),
        "a second tap takes it out"
    );
    assert!(finger.selection().has_component(second));

    finger.press("Add to selection");
    finger.tap(finger.screen(Point::new(200, 200)));
    assert!(finger.selection().has_component(first));
    assert!(
        !finger.selection().has_component(second),
        "unlatched, a tap replaces"
    );
}

/// G-9 on touch: a long press where two parts overlap lists both at the head
/// of the menu. Choosing the one underneath selects it, and the next long
/// press there opens the menu on it, so its commands act on it.
#[test]
fn a_long_press_on_overlapping_parts_offers_each_of_them() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    let first = finger.resistor(Point::new(200, 200));
    let second = finger.resistor(Point::new(200, 200));
    let name = |finger: &Finger, id: u64| {
        let part = finger
            .app
            .state
            .schematic
            .document()
            .components
            .iter()
            .find(|part| part.id == id)
            .expect("part");
        format!("Instance {}", part.name)
    };
    let at = finger.screen(Point::new(200, 200));
    finger.long_press(at);
    let taken = finger
        .selection()
        .single_component()
        .expect("the long press takes the part on top");
    let under = if taken == first { second } else { first };
    assert!(
        finger.find(&name(&finger, taken)).is_some(),
        "the menu lists the part on top: {:?}",
        finger.named
    );
    finger.press(&name(&finger, under));
    assert_eq!(finger.selection().single_component(), Some(under));

    finger.long_press(at);
    assert_eq!(
        finger.selection().single_component(),
        Some(under),
        "the second long press keeps the chosen part"
    );
    finger.press("Rotate 90°");
    let turned: Vec<(u64, bool)> = finger
        .app
        .state
        .schematic
        .document()
        .components
        .iter()
        .map(|part| (part.id, part.rotation != Default::default()))
        .collect();
    assert!(
        turned.contains(&(under, true)) && turned.contains(&(taken, false)),
        "the menu turned the chosen part and only it: {turned:?}"
    );
}

/// G-12: R and M turn the placement ghost, and a tablet has neither key.
/// The strip's Rotate and Mirror turn the ghost while a part is being
/// placed, and the part lands as the ghost showed it.
#[test]
fn the_strip_turns_the_part_being_placed() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    finger.wake();
    finger
        .app
        .state
        .schematic
        .arm_tool(crate::state::Tool::Place(ComponentType::Resistor));
    finger.frame(Vec::new());
    finger.press("Rotate clockwise");
    finger.press("Mirror");
    let quarter = rspice_design::schematic::rotation::Rotation::default().rotate_cw();
    assert_eq!(
        finger.app.state.schematic.session.editor.preview_rotation,
        quarter
    );
    assert!(finger.app.state.schematic.session.editor.preview_mirror_h);

    finger.tap(finger.screen(Point::new(300, 200)));
    let part = finger
        .app
        .state
        .schematic
        .document()
        .components
        .last()
        .expect("the tap placed the part");
    assert_eq!((part.rotation, part.mirror_h), (quarter, true));
}

/// A finger has no Z: the empty canvas's long-press menu arms the zoom box,
/// and the next one-finger drag frames the view.
#[test]
fn a_long_press_on_empty_canvas_arms_the_zoom_box() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    finger.long_press(finger.screen(Point::new(400, 300)));
    finger.reveal("Zoom box");
    finger.press("Zoom box");
    assert!(finger.app.state.schematic.session.editor.zoom_box.armed);
    let zoom = finger.app.state.schematic.session.editor.zoom;
    let (from, to) = (
        finger.screen(Point::new(100, 100)),
        finger.screen(Point::new(300, 250)),
    );
    finger.drag(from, to);
    assert!(!finger.app.state.schematic.session.editor.zoom_box.armed);
    assert!(
        finger.app.state.schematic.session.editor.zoom > zoom * 2.0,
        "the box framed the view"
    );
}

/// A phone shows no menu bar, so the Design menu's Arrange submenu is a
/// page of the long-press menu: here, two parts lined up on the left.
#[test]
fn the_long_press_menu_reaches_the_arrange_commands() {
    let mut finger = Finger::new(Vec2::new(1024.0, 768.0));
    let first = finger.resistor(Point::new(200, 200));
    let second = finger.resistor(Point::new(400, 300));
    finger.tap(finger.screen(Point::new(200, 200)));
    finger.press("Add to selection");
    finger.tap(finger.screen(Point::new(400, 300)));
    finger.press("Add to selection");
    assert!(finger.selection().has_component(first) && finger.selection().has_component(second));

    finger.long_press(finger.screen(Point::new(400, 300)));
    finger.press("Arrange");
    assert!(
        finger.find("Back").is_some(),
        "the Arrange page has a way back"
    );
    finger.reveal("Align left");
    finger.press("Align left");
    let left: Vec<i32> = finger
        .app
        .state
        .schematic
        .document()
        .components
        .iter()
        .map(|part| part.pos.x)
        .collect();
    assert_eq!(left[0], left[1], "both parts line up on the left: {left:?}");
    assert!(finger.app.state.schematic.can_undo(), "one undoable edit");
}
