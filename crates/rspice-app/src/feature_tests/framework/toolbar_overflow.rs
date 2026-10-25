use crate::state::Tool;
use crate::ui::panels::WorkbenchIcon;
use crate::ui::tokens::Tokens;
use crate::workbench::chrome::toolbar::overflow::*;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::{AppState, RSpiceApp};
use egui::{Event, Key, Pos2, Stroke};
use egui::{Rect, Vec2};

/// The toolbar's last slot with two tools set aside, driven by real frames
/// and read back through the AccessKit tree.
struct Strip {
    ctx: egui::Context,
    app: RSpiceApp,
    layout: LayoutSpec,
    accent: egui::Color32,
}

/// What one frame published and painted.
struct Frame {
    /// Each named node: its label, role and bounds.
    named: Vec<(String, egui::accesskit::Role, Rect)>,
    /// The label of the node holding keyboard focus, if any.
    focused: Option<String>,
    /// The design-system focus rings painted.
    rings: Vec<Rect>,
}

impl Frame {
    fn row(&self, label: &str) -> Rect {
        self.named
            .iter()
            .filter(|(name, role, _)| name == label && *role == egui::accesskit::Role::Button)
            .map(|(_, _, rect)| *rect)
            .max_by(|a, b| a.width().total_cmp(&b.width()))
            .unwrap_or_else(|| panic!("no button named {label:?}: {:?}", self.named))
    }

    fn ring_around(&self, rect: Rect) -> bool {
        self.rings
            .iter()
            .any(|ring| rect.expand(0.5).contains_rect(*ring))
    }
}

impl Strip {
    fn new() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let app = RSpiceApp::test_instance();
        let layout = LayoutSpec::resolve(390.0, 844.0, &app.state.workbench);
        let accent = Tokens::get(&ctx).color.accent;
        let mut strip = Self {
            ctx,
            app,
            layout,
            accent,
        };
        strip.frame(Vec::new());
        strip
    }

    /// One frame a tenth of a second after the last, so the menu's
    /// fade-in is over a frame after it opens.
    fn frame(&mut self, events: Vec<Event>) -> Frame {
        let Self {
            ctx, app, layout, ..
        } = self;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(390.0, 844.0))),
                time: Some(ctx.input(|input| input.time) + 0.1),
                events,
                focused: true,
                ..Default::default()
            },
            |ui| {
                reset::<AppState>(ui);
                for (label, icon, run) in [
                    (
                        "Draw wire",
                        WorkbenchIcon::Wire,
                        (|state: &mut AppState| {
                            state.model.schematic.session.editor.tool = Tool::Wire;
                        }) as fn(&mut AppState),
                    ),
                    ("Place probe", WorkbenchIcon::Probe, |state| {
                        state.model.schematic.session.editor.tool = Tool::Probe;
                    }),
                ] {
                    set_aside(
                        ui,
                        OverflowTool {
                            action: OverflowAction::Run(run),
                            icon,
                            label: label.to_owned(),
                            selected: false,
                        },
                    );
                }
                more_tools(ui, app, *layout);
            },
        );
        let update = output
            .platform_output
            .accesskit_update
            .expect("an AccessKit update");
        let label_of =
            |node: &egui::accesskit::Node| node.label().or_else(|| node.value()).map(str::to_owned);
        let focused = update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .and_then(|(_, node)| label_of(node));
        let named = update
            .nodes
            .iter()
            .filter_map(|(_, node)| {
                let bounds = node.bounds()?;
                Some((
                    label_of(node)?,
                    node.role(),
                    Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                ))
            })
            .collect();
        let ring = Stroke::new(2.0, self.accent);
        let mut rings = Vec::new();
        let mut shapes: Vec<egui::Shape> = output
            .shapes
            .into_iter()
            .map(|clipped| clipped.shape)
            .collect();
        while let Some(shape) = shapes.pop() {
            match shape {
                egui::Shape::Vec(inner) => shapes.extend(inner),
                egui::Shape::Rect(rect) if rect.stroke == ring => rings.push(rect.rect),
                _ => {}
            }
        }
        Frame {
            named,
            focused,
            rings,
        }
    }

    fn key(&mut self, key: Key) -> Frame {
        self.frame(vec![Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }])
    }

    fn button(&mut self, at: Pos2, pressed: bool) -> Frame {
        self.frame(vec![
            Event::PointerMoved(at),
            Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            },
        ])
    }

    fn tool(&self) -> Tool {
        self.app.state.model.schematic.session.editor.tool
    }
}

/// The set-aside tools are buttons a screen reader names. Tab walks from
/// More tools into them and the arrows move between them, each framed by
/// the focus ring while it holds the keyboard; Enter takes the one framed.
/// A pointer press on a row gives it no focus and no ring.
#[test]
fn more_tools_rows_take_the_keyboard_and_ring_only_for_it() {
    let mut strip = Strip::new();
    strip.key(Key::Tab);
    let opened = {
        strip.key(Key::Enter);
        strip.frame(Vec::new())
    };
    let wire = opened.row("Draw wire");
    let probe = opened.row("Place probe");
    assert!(
        wire.width() > 100.0,
        "a menu row, not a strip button: {wire:?}"
    );
    assert!(!opened.ring_around(wire) && !opened.ring_around(probe));

    // egui gives the popup's own frame a stop of its own, so the walk is
    // bounded rather than counted.
    let mut tabbed = strip.key(Key::Tab);
    for _ in 0..3 {
        if tabbed.focused.as_deref() == Some("Draw wire") {
            break;
        }
        tabbed = strip.key(Key::Tab);
    }
    assert_eq!(tabbed.focused.as_deref(), Some("Draw wire"));
    assert!(
        tabbed.ring_around(tabbed.row("Draw wire")),
        "Tab frames the row it reached: {:?}",
        tabbed.rings
    );

    // egui moves focus by arrow at the end of the pass, so the ring
    // follows on the next.
    strip.key(Key::ArrowDown);
    let arrowed = strip.frame(Vec::new());
    assert_eq!(arrowed.focused.as_deref(), Some("Place probe"));
    assert!(arrowed.ring_around(arrowed.row("Place probe")));
    assert!(!arrowed.ring_around(arrowed.row("Draw wire")));

    strip.key(Key::Enter);
    assert_eq!(strip.tool(), Tool::Probe, "Enter takes the framed row");
    let closed = strip.frame(Vec::new());
    assert!(
        !closed.named.iter().any(|(name, _, _)| name == "Draw wire"),
        "taking a row closes the menu"
    );

    let more = closed.row("More tools").center();
    strip.button(more, true);
    strip.button(more, false);
    let reopened = strip.frame(Vec::new());
    let wire = reopened.row("Draw wire");
    let pressed = strip.button(wire.center(), true);
    assert_ne!(pressed.focused.as_deref(), Some("Draw wire"));
    assert!(
        pressed.rings.is_empty(),
        "a pointer press frames nothing: {:?}",
        pressed.rings
    );
    strip.button(wire.center(), false);
    assert_eq!(strip.tool(), Tool::Wire, "a click takes the row");
}
