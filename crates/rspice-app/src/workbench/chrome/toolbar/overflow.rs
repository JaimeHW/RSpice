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
use crate::ui::panels::{WorkbenchIcon, icon_button};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::workbench_host::WorkbenchHost;
use crate::workbench::layout::LayoutSpec;

/// A tool the row had no slot for this frame, run over the session `S`.
pub struct OverflowTool<S> {
    pub action: OverflowAction<S>,
    pub icon: WorkbenchIcon,
    pub label: String,
    pub selected: bool,
}

// By hand: a derive would ask the session itself to be `Clone`.
impl<S> Clone for OverflowTool<S> {
    fn clone(&self) -> Self {
        Self {
            action: self.action,
            icon: self.icon,
            label: self.label.clone(),
            selected: self.selected,
        }
    }
}

pub enum OverflowAction<S> {
    /// A command, with the way the row draws it.
    Command(CommandKey, ToolbarCommandPresentation),
    /// A toolbar action that is not a command, such as opening the
    /// generated deck. Always available.
    Run(fn(&mut S)),
}

// By hand: a derive would ask the session itself to be `Clone` and `Copy`.
impl<S> Clone for OverflowAction<S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for OverflowAction<S> {}

impl<S> OverflowTool<S> {
    pub fn command<H>(
        host: &H,
        command: CommandKey,
        presentation: ToolbarCommandPresentation,
    ) -> Self
    where
        H: WorkbenchHost<Session = S>,
    {
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

    fn availability(&self, app: &impl WorkbenchHost) -> CommandAvailability {
        match self.action {
            OverflowAction::Command(command, _) => app.availability(command),
            OverflowAction::Run(_) => CommandAvailability::Available,
        }
    }
}

fn overflow_id() -> egui::Id {
    egui::Id::new("workbench.context_toolbar.overflow_tools")
}

/// Start a frame's toolbar with nothing set aside for the session `S`.
pub fn reset<S: 'static>(ui: &egui::Ui) {
    ui.data_mut(|data| data.remove::<Vec<OverflowTool<S>>>(overflow_id()));
}

/// Set aside a tool the row had no slot for.
pub fn set_aside<S: 'static>(ui: &egui::Ui, tool: OverflowTool<S>) {
    ui.data_mut(|data| {
        data.get_temp_mut_or_default::<Vec<OverflowTool<S>>>(overflow_id())
            .push(tool);
    });
}

/// The last slot: More tools when the row set two or more tools aside this
/// frame, the one tool itself when it set one aside, nothing otherwise.
pub fn more_tools<H: WorkbenchHost>(ui: &mut egui::Ui, app: &mut H, layout: LayoutSpec) {
    let tools = ui
        .data_mut(|data| data.remove_temp::<Vec<OverflowTool<H::Session>>>(overflow_id()))
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
                    run(app.session_mut());
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
                    OverflowAction::Run(run) => run(app.session_mut()),
                }
                ui.close();
            }
        }
    });
}

/// One set-aside tool: its icon and label, accented when it is the armed
/// tool. Returns whether it was taken.
fn tool_row<H: WorkbenchHost>(
    ui: &mut egui::Ui,
    app: &H,
    tool: &OverflowTool<H::Session>,
    height: f32,
) -> bool {
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
