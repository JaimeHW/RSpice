//! The Simulate and Verify toolbars, and the run controls at the toolbar's
//! trailing edge: Run or Stop with its menu.

use egui::containers::menu::MenuButton;
use egui::{Align, Layout, Vec2};

use crate::state::command_catalog::CommandKey;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::Model;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::framework::workbench_host::WorkbenchHost;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::simulate::commands::SimulateCommand;

use super::{
    context_separator, pvt_temperature_label, run_control_label_for, toolbar_icon_command,
    toolbar_text_command,
};

pub fn simulation_tools(ui: &mut egui::Ui, app: &mut impl WorkbenchHost, layout: LayoutSpec) {
    toolbar_icon_command(
        ui,
        app,
        SimulateCommand::Options.key(),
        WorkbenchIcon::Sliders,
        layout,
    );
    context_separator(ui, layout);
    toolbar_text_command(
        ui,
        app,
        SimulateCommand::Preflight.key(),
        WorkbenchIcon::Check,
        "Run preflight",
        layout,
    );
}

/// Verify's specifications are its own first view, so the toolbar opens that
/// view rather than a second place to edit them.
pub fn verification_tools(ui: &mut egui::Ui, app: &mut impl WorkbenchHost, layout: LayoutSpec) {
    toolbar_icon_command(
        ui,
        app,
        SimulateCommand::Verify(crate::state::VerificationPage::Specifications).key(),
        WorkbenchIcon::Target,
        layout,
    );
}

/// What the run control and its menu show and offer: whether a preflight is
/// preparing its run, Run or Stop with the menu's two other entries, and why
/// Run or Stop is unavailable while it is.
pub(in crate::workbench) struct RunControls<'a> {
    pub(in crate::workbench) model: &'a Model,
    /// A preflight is preparing its run or building its decks.
    pub(in crate::workbench) checking: bool,
    /// A deck's run is being prepared.
    pub(in crate::workbench) preparing_deck: bool,
    /// The plan's run waits for Run anyway or Stop: it needs more memory
    /// than is free.
    pub(in crate::workbench) held_for_memory: bool,
    /// Run, or Stop while a run or a preflight is active.
    pub(in crate::workbench) primary: RunEntry,
    pub(in crate::workbench) preflight: RunEntry,
    pub(in crate::workbench) options: RunEntry,
    /// Why the primary is unavailable, read only while it is.
    pub(in crate::workbench) unavailable_reason: Option<String>,
    /// The deck Run runs, by its file's name, when Run runs a deck rather
    /// than the plan: on the Netlist workspace the deck tab Run runs, and
    /// elsewhere the project's deck while the deck is its simulation.
    pub(in crate::workbench) deck: Option<String>,
}

/// One run command as the control or its menu offers it: what it is called,
/// whether it would run now, and the shortcut the reader's profile binds to
/// it, or nothing.
pub(in crate::workbench) struct RunEntry {
    pub(in crate::workbench) key: CommandKey,
    pub(in crate::workbench) label: &'static str,
    pub(in crate::workbench) enabled: bool,
    pub(in crate::workbench) shortcut: String,
}

/// Draw Run or Stop and its menu from `controls`. Returns the command the
/// reader pressed, for the caller to raise.
pub(in crate::workbench) fn run_controls(
    ui: &mut egui::Ui,
    controls: &RunControls<'_>,
    layout: LayoutSpec,
) -> Option<CommandKey> {
    let model = controls.model;
    // A preflight preparing its run or building its decks, a deck's run
    // being prepared, and a plan's run waiting for memory are stopped the way
    // a run is: the control is Stop until they finish.
    let checking = controls.checking;
    let preparing = checking || controls.preparing_deck || controls.held_for_memory;
    let execution_active = model.simulation.execution.has_active_execution() || preparing;
    let cancellation_pending = model.simulation.cancellation_is_pending();
    let command = controls.primary.key;
    let mut pressed = None;
    let icon = if execution_active {
        WorkbenchIcon::Stop
    } else {
        WorkbenchIcon::Run
    };
    let t = Tokens::get(ui.ctx());
    let radius = t.radius.round().clamp(0.0, u8::MAX as f32) as u8;
    let label = match &controls.deck {
        Some(_) if !execution_active => "Run deck".to_owned(),
        _ => run_control_label_for(model, preparing, layout),
    };
    let label_width = ui
        .painter()
        .layout_no_wrap(
            label.clone(),
            theme::sans(tokens::FS_1, FontWeight::SemiBold),
            t.color.text,
        )
        .size()
        .x;
    let more_width = if layout.run_control_height >= 44.0 {
        44.0
    } else {
        25.0
    };
    let primary_width = (22.0 + 14.0 + 6.0 + label_width).max(67.0);
    let width = primary_width + more_width;
    ui.allocate_ui_with_layout(
        Vec2::new(width, layout.run_control_height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let accessibility_label = if checking {
                "Stop the preflight check".to_owned()
            } else if controls.preparing_deck {
                "Stop preparing the deck's run".to_owned()
            } else if controls.held_for_memory {
                "Stop the run waiting for memory".to_owned()
            } else if cancellation_pending {
                "Simulation cancellation in progress".to_owned()
            } else if execution_active {
                "Stop active simulation".to_owned()
            } else if let Some(deck) = &controls.deck {
                format!("Run deck {deck}")
            } else {
                let reference = model.sim_setup.reference_pvt;
                let quantity_policy = model.preferences.quantity_presentation_policy();
                format!(
                    "Run active simulation plan at {} · {}",
                    reference.process.short_name(),
                    pvt_temperature_label(reference.temperature_celsius, quantity_policy)
                )
            };
            let shortcut = &controls.primary.shortcut;
            let accessibility_label = if shortcut.is_empty() {
                accessibility_label
            } else {
                format!("{accessibility_label} ({shortcut})")
            };
            let enabled = controls.primary.enabled;
            let (rect, response) = ui.allocate_exact_size(
                Vec2::new(primary_width, layout.run_control_height),
                if enabled {
                    egui::Sense::click()
                } else {
                    egui::Sense::hover()
                },
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    enabled && ui.is_enabled(),
                    &accessibility_label,
                )
            });
            if enabled && !shortcut.is_empty() {
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_keyboard_shortcut(shortcut.as_str());
                });
            }
            let base_fill = if enabled {
                if execution_active {
                    t.color.err
                } else {
                    t.color.accent
                }
            } else {
                t.color.bg_inset
            };
            let fill = if enabled && response.hovered() {
                brighten_srgb(base_fill, 1.06)
            } else {
                base_fill
            };
            let ink = if !enabled {
                t.color.text_faint
            } else if execution_active {
                t.color.text
            } else {
                t.color.accent_ink
            };
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius {
                    nw: radius,
                    sw: radius,
                    ne: 0,
                    se: 0,
                },
                fill,
            );
            icon.paint(
                ui.painter(),
                egui::Rect::from_center_size(
                    egui::Pos2::new(rect.left() + 16.0, rect.center().y),
                    Vec2::splat(14.0),
                ),
                ink,
            );
            ui.painter().text(
                egui::Pos2::new(rect.left() + 30.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &label,
                // The mockup run control inherits the 12 px application body type.
                theme::sans(tokens::FS_1, FontWeight::SemiBold),
                ink,
            );
            theme::paint_focus_ring_outset(ui, &response, rect);
            if response.clicked() && enabled {
                pressed = Some(command);
            }
            if enabled {
                response.clone().on_hover_text(&accessibility_label);
            }
            if !enabled && let Some(reason) = &controls.unavailable_reason {
                response.on_hover_text(reason);
            }

            let (more_response, _) = ui
                .scope(|ui| {
                    ui.spacing_mut().button_padding = Vec2::ZERO;
                    MenuButton::from_button(
                        egui::Button::new("")
                            .frame(false)
                            .min_size(Vec2::new(more_width, layout.run_control_height)),
                    )
                    .ui(ui, |ui| {
                        ui.set_min_width(210.0);
                        ui.spacing_mut().item_spacing.y = 0.0;
                        for entry in [&controls.primary, &controls.preflight] {
                            if run_menu_item(ui, entry) {
                                pressed = Some(entry.key);
                            }
                        }
                        // The plan's options are no deck's: a deck states
                        // its own, on its `.options` cards.
                        if controls.deck.is_none() {
                            ui.separator();
                            if run_menu_item(ui, &controls.options) {
                                pressed = Some(controls.options.key);
                            }
                        }
                    })
                })
                .inner;
            more_response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    "More run commands",
                )
            });
            let more_fill = if more_response.hovered() {
                brighten_srgb(base_fill, 1.06)
            } else {
                base_fill
            };
            ui.painter().rect_filled(
                more_response.rect,
                egui::CornerRadius {
                    nw: 0,
                    sw: 0,
                    ne: radius,
                    se: radius,
                },
                more_fill,
            );
            ui.painter().vline(
                more_response.rect.left(),
                more_response.rect.y_range().shrink(5.0),
                egui::Stroke::new(1.0, ink.gamma_multiply(0.28)),
            );
            WorkbenchIcon::ChevronDown.paint(
                ui.painter(),
                egui::Rect::from_center_size(more_response.rect.center(), Vec2::splat(12.0)),
                ink,
            );
            theme::paint_focus_ring_outset(ui, &more_response, more_response.rect);
        },
    );
    pressed
}

/// One entry of the run menu. Returns whether the reader pressed it.
fn run_menu_item(ui: &mut egui::Ui, entry: &RunEntry) -> bool {
    let shortcut = &entry.shortcut;
    let label = if shortcut.is_empty() {
        entry.label.to_owned()
    } else {
        format!("{} ({shortcut})", entry.label)
    };
    let response = ui.add_enabled(entry.enabled, egui::Button::new(label).frame(false));
    if entry.enabled && !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    let pressed = response.clicked();
    if pressed {
        ui.close();
    }
    pressed
}

fn brighten_srgb(color: egui::Color32, factor: f32) -> egui::Color32 {
    let channel = |value: u8| (f32::from(value) * factor).round().clamp(0.0, 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(
        channel(color.r()),
        channel(color.g()),
        channel(color.b()),
        color.a(),
    )
}
