//! The tools a narrow toolbar has no room for, behind one More tools button.
//!
//! At 560 px and less the tool row has room for two buttons. It used to keep
//! the first two tools and drop the rest, so a phone had no button for Draw
//! wire, Undo or Redo and reached them only by typing in the search palette
//! (audit U-11). Now the last slot is kept for More tools, which lists every
//! tool the row could not show, in toolbar order, each two taps away. When
//! only one tool is left over, it takes that slot itself.

use egui::{Popup, Rect, Sense, Vec2};

use super::ToolbarCommandPresentation;
use crate::state::command_catalog::CommandKey;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::design_system::{WorkbenchIcon, icon_button};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::host::Host;
use crate::workbench::layout::LayoutSpec;

/// A tool the row had no slot for this frame.
#[derive(Clone)]
pub(super) struct OverflowTool {
    pub action: OverflowAction,
    pub icon: WorkbenchIcon,
    pub label: String,
    pub selected: bool,
}

#[derive(Clone, Copy)]
pub(super) enum OverflowAction {
    /// A command, with the way the row draws it.
    Command(CommandKey, ToolbarCommandPresentation),
    /// A toolbar action that is not a command, such as opening the
    /// generated deck. Always available.
    Run(fn(&mut AppState)),
}

impl OverflowTool {
    pub(super) fn command(
        host: &impl Host,
        command: CommandKey,
        presentation: ToolbarCommandPresentation,
    ) -> Self {
        let (icon, label, selected) = match presentation {
            ToolbarCommandPresentation::Icon {
                icon,
                selected,
                label,
            } => (icon, label.unwrap_or(host.label(command)), selected),
            ToolbarCommandPresentation::Text { icon, label } => (icon, label, false),
        };
        Self {
            action: OverflowAction::Command(command, presentation),
            icon,
            label: label.to_owned(),
            selected,
        }
    }

    fn availability(&self, app: &RSpiceApp) -> CommandAvailability {
        match self.action {
            OverflowAction::Command(command, _) => app.availability(command),
            OverflowAction::Run(_) => CommandAvailability::Available,
        }
    }
}

fn overflow_id() -> egui::Id {
    egui::Id::new("workbench.context_toolbar.overflow_tools")
}

/// Start a frame's toolbar with nothing set aside.
pub(super) fn reset(ui: &egui::Ui) {
    ui.data_mut(|data| data.remove::<Vec<OverflowTool>>(overflow_id()));
}

/// Set aside a tool the row had no slot for.
pub(super) fn set_aside(ui: &egui::Ui, tool: OverflowTool) {
    ui.data_mut(|data| {
        data.get_temp_mut_or_default::<Vec<OverflowTool>>(overflow_id())
            .push(tool);
    });
}

/// The last slot: More tools when the row set two or more tools aside this
/// frame, the one tool itself when it set one aside, nothing otherwise.
pub(super) fn more_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let tools = ui
        .data_mut(|data| data.remove_temp::<Vec<OverflowTool>>(overflow_id()))
        .unwrap_or_default();
    let size = super::toolbar_icon_button_size(layout);
    if let [tool] = &tools[..] {
        match tool.action {
            OverflowAction::Command(command, presentation) => {
                // Hand the slot kept for More tools back to the row.
                ui.data_mut(|data| data.insert_temp(super::projected_tool_count_id(), 0_usize));
                super::toolbar_command(ui, app, command, layout, presentation);
            }
            OverflowAction::Run(run) => {
                if icon_button(ui, tool.icon, &tool.label, tool.selected, size).clicked() {
                    run(&mut app.state);
                }
            }
        }
        return;
    }
    if tools.is_empty() {
        return;
    }
    let popup_id = ui.id().with("more-tools");
    let open = Popup::is_id_open(ui.ctx(), popup_id);
    let response = icon_button(ui, WorkbenchIcon::More, "More tools", open, size);
    let metrics = Tokens::get(ui.ctx()).metrics;
    let row_height = if metrics.is_touch() {
        metrics.touch_target()
    } else {
        28.0
    };
    Popup::menu(&response).id(popup_id).width(240.0).show(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        for tool in &tools {
            if tool_row(ui, app, tool, row_height) {
                match tool.action {
                    OverflowAction::Command(command, _) => {
                        app.execute(command);
                    }
                    OverflowAction::Run(run) => run(&mut app.state),
                }
                ui.close();
            }
        }
    });
}

/// One set-aside tool: its icon and label, accented when it is the armed
/// tool. Returns whether it was taken.
fn tool_row(ui: &mut egui::Ui, app: &RSpiceApp, tool: &OverflowTool, height: f32) -> bool {
    let t = Tokens::get(ui.ctx());
    let availability = tool.availability(app);
    let enabled = availability == CommandAvailability::Available;
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), height),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            enabled,
            tool.selected,
            &tool.label,
        )
    });
    if enabled && (response.hovered() || response.is_pointer_button_down_on()) {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    let ink = if !enabled {
        t.color.text_faint
    } else if tool.selected {
        t.color.accent
    } else {
        t.color.text
    };
    let icon = Rect::from_center_size(
        egui::pos2(rect.left() + 10.0 + 8.0, rect.center().y),
        Vec2::splat(16.0),
    );
    tool.icon.paint(ui.painter(), icon, ink);
    ui.painter().text(
        egui::pos2(icon.right() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &tool.label,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    // An available row is a keyboard stop: Tab and the arrows reach it and
    // Enter takes it. A press never gives it focus, so only the keyboard
    // frames it.
    theme::paint_focus_ring(ui, &response, rect);
    let response = match availability {
        CommandAvailability::Disabled(reason) => response.on_disabled_hover_text(reason),
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    enabled && response.clicked()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Tool;
    use egui::{Event, Key, Pos2, Stroke};

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
                    reset(ui);
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
            let label_of = |node: &egui::accesskit::Node| {
                node.label().or_else(|| node.value()).map(str::to_owned)
            };
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
}
