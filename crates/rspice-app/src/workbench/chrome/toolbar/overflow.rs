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
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::CommandAvailability;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{WorkbenchIcon, icon_button};
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
    Command(Command, ToolbarCommandPresentation),
    /// A toolbar action that is not a command, such as opening the
    /// generated deck. Always available.
    Run(fn(&mut AppState)),
}

impl OverflowTool {
    pub(super) fn command(command: Command, presentation: ToolbarCommandPresentation) -> Self {
        let (icon, label, selected) = match presentation {
            ToolbarCommandPresentation::Icon {
                icon,
                selected,
                label,
            } => (icon, label.unwrap_or(command.spec().label), selected),
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
            OverflowAction::Command(command, _) => command.availability(app),
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
                        command.execute(app);
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
    let response = match availability {
        CommandAvailability::Disabled(reason) => response.on_disabled_hover_text(reason),
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    enabled && response.clicked()
}
