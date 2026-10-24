//! The Simulate and Verify toolbars, and the run controls at the toolbar's
//! trailing edge: Run or Stop with its menu, and the reference PVT chip.

use egui::containers::menu::MenuButton;
use egui::{Align, Layout, Vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::{Command, CommandRegistry};
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::Workspace;

use super::{
    context_separator, pvt_temperature_label, run_control_label, toolbar_icon_command,
    toolbar_text_command,
};

pub(in crate::workbench) fn simulation_tools(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    toolbar_icon_command(
        ui,
        app,
        Command::SimulationOptions.key(),
        WorkbenchIcon::Sliders,
        layout,
    );
    context_separator(ui, layout);
    toolbar_text_command(
        ui,
        app,
        Command::PreflightChecks.key(),
        WorkbenchIcon::Check,
        "Run preflight",
        layout,
    );
}

pub(in crate::workbench) fn verification_tools(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    toolbar_icon_command(
        ui,
        app,
        Command::EditSpecifications.key(),
        WorkbenchIcon::Target,
        layout,
    );
}

pub(in crate::workbench) fn run_controls(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    layout: LayoutSpec,
) {
    // A preflight preparing its run or building its decks is stopped the way
    // a run is: the control is Stop until it finishes.
    let checking = app.state.runs.preflight.checking.is_some();
    let execution_active = app.state.model.simulation.execution.has_active_execution() || checking;
    let cancellation_pending = app.state.model.simulation.cancellation_is_pending();
    let command = if execution_active {
        Command::StopSimulation
    } else {
        Command::RunSimulation
    };
    let icon = if execution_active {
        WorkbenchIcon::Stop
    } else {
        WorkbenchIcon::Run
    };
    let t = Tokens::get(ui.ctx());
    let radius = t.radius.round().clamp(0.0, u8::MAX as f32) as u8;
    let label = run_control_label(app, layout);
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
            } else if cancellation_pending {
                "Simulation cancellation in progress".to_owned()
            } else if execution_active {
                "Stop active simulation".to_owned()
            } else {
                let reference = app.state.model.sim_setup.reference_pvt;
                let quantity_policy = app.state.model.preferences.quantity_presentation_policy();
                format!(
                    "Run active simulation plan at {} · {}",
                    reference.process.short_name(),
                    pvt_temperature_label(reference.temperature_celsius, quantity_policy)
                )
            };
            let shortcut = app.state.model.preferences.shortcuts().resolved_label(
                &CommandRegistry,
                command.key(),
                crate::workbench::app_state::runtime_command_platform(ui.ctx()),
                ui.ctx().os(),
            );
            let accessibility_label = if shortcut.is_empty() {
                accessibility_label
            } else {
                format!("{accessibility_label} ({shortcut})")
            };
            let enabled = command.is_enabled(app);
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
                command.execute(app);
            }
            if enabled {
                response.clone().on_hover_text(&accessibility_label);
            }
            if !enabled {
                let reason = if execution_active {
                    match command.availability(app) {
                        CommandAvailability::Disabled(reason) => reason.to_owned(),
                        CommandAvailability::Available | CommandAvailability::Hidden => {
                            "Command unavailable".to_owned()
                        }
                    }
                } else if app.state.workbench.workspace == Workspace::Netlist {
                    app.manual_deck_run_block_reason()
                        .unwrap_or_else(|| "Command unavailable".to_owned())
                } else {
                    app.state
                        .simulation_run_block_reason()
                        .unwrap_or_else(|| "Command unavailable".to_owned())
                };
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
                        let primary = if execution_active {
                            Command::StopSimulation
                        } else {
                            Command::RunSimulation
                        };
                        run_menu_item(ui, app, primary);
                        run_menu_item(ui, app, Command::PreflightChecks);
                        ui.separator();
                        run_menu_item(ui, app, Command::SimulationOptions);
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
}

fn run_menu_item(ui: &mut egui::Ui, app: &mut RSpiceApp, command: Command) {
    let enabled = command.is_enabled(app);
    let shortcut = app.state.model.preferences.shortcuts().resolved_label(
        &CommandRegistry,
        command.key(),
        crate::workbench::app_state::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    let label = if shortcut.is_empty() {
        command.spec().label.to_owned()
    } else {
        format!("{} ({shortcut})", command.spec().label)
    };
    let response = ui.add_enabled(enabled, egui::Button::new(label).frame(false));
    if enabled && !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    if response.clicked() {
        command.execute(app);
        ui.close();
    }
}

/// From the reference PVT chip's left edge to its label. The chip is a
/// labelled control; it carries no dot in front of the label.
const PVT_CHIP_TEXT_INSET: f32 = 10.0;
/// The chip's right end: the chevron and the space around it.
const PVT_CHIP_CHEVRON_SPAN: f32 = 21.0;

pub(in crate::workbench) fn pvt_selector(ui: &mut egui::Ui, app: &mut RSpiceApp, height: f32) {
    let t = Tokens::get(ui.ctx());
    let reference = app.state.model.sim_setup.reference_pvt;
    let quantity_policy = app.state.model.preferences.quantity_presentation_policy();
    let label = format!(
        "{} · {}",
        reference.process.short_name(),
        pvt_temperature_label(reference.temperature_celsius, quantity_policy),
    );
    let width = ui
        .painter()
        .layout_no_wrap(
            label.clone(),
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
        )
        .size()
        .x
        + PVT_CHIP_TEXT_INSET
        + PVT_CHIP_CHEVRON_SPAN;
    let (response, _) = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            MenuButton::from_button(
                egui::Button::new("")
                    .frame(false)
                    .min_size(Vec2::new(width, height)),
            )
            .ui(ui, |ui| {
                ui.set_min_width(190.0);
                ui.spacing_mut().item_spacing.y = 0.0;
                use crate::product::ProcessCorner;
                let option_height = if height >= 44.0 { 44.0 } else { 29.0 };
                egui::ScrollArea::vertical()
                    .id_salt("workbench.reference_pvt.options")
                    .max_height(pvt_menu_height_for_viewport(
                        ui.ctx().content_rect().height(),
                        option_height,
                    ))
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .show(ui, |ui| {
                        for process in [
                            ProcessCorner::FF,
                            ProcessCorner::FS,
                            ProcessCorner::TT,
                            ProcessCorner::SF,
                            ProcessCorner::SS,
                        ] {
                            for temperature in [-40.0, 27.0, 125.0] {
                                let selected = reference.process == process
                                    && (reference.temperature_celsius - temperature).abs()
                                        < f64::EPSILON;
                                let label = format!(
                                    "{} · {}",
                                    process.short_name(),
                                    pvt_temperature_label(temperature, quantity_policy)
                                );
                                if ui
                                    .add_sized(
                                        [ui.available_width(), option_height],
                                        egui::Button::selectable(selected, &label),
                                    )
                                    .clicked()
                                {
                                    match commit_reference_pvt(app, process, temperature) {
                                        Ok(true) => app
                                            .state
                                            .prompt(format!("Reference PVT changed to {label}.")),
                                        Ok(false) => {}
                                        Err(error) => app.state.prompt(format!(
                                            "Reference PVT was not changed: {error}"
                                        )),
                                    }
                                    ui.close();
                                }
                            }
                        }
                    });
            })
        })
        .inner;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &label)
    });
    ui.painter().rect(
        response.rect,
        t.radius,
        t.color.bg_inset,
        egui::Stroke::new(
            1.0,
            if response.hovered() {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    let text_clip = egui::Rect::from_min_max(
        egui::pos2(
            response.rect.left() + PVT_CHIP_TEXT_INSET,
            response.rect.top(),
        ),
        egui::pos2(
            response.rect.right() - PVT_CHIP_CHEVRON_SPAN,
            response.rect.bottom(),
        ),
    );
    ui.painter().with_clip_rect(text_clip).text(
        egui::pos2(
            response.rect.left() + PVT_CHIP_TEXT_INSET,
            response.rect.center().y,
        ),
        egui::Align2::LEFT_CENTER,
        label,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(response.rect.right() - 10.0, response.rect.center().y),
            Vec2::splat(11.0),
        ),
        t.color.text_faint,
    );
    theme::paint_focus_ring_outset(ui, &response, response.rect);
}

pub(super) fn pvt_menu_height_for_viewport(viewport_height: f32, row_height: f32) -> f32 {
    (viewport_height - 72.0).clamp(row_height * 3.0, 484.0)
}

pub(in crate::workbench) fn commit_reference_pvt(
    app: &mut RSpiceApp,
    process: crate::product::ProcessCorner,
    temperature_celsius: f64,
) -> Result<bool, String> {
    let current = app.state.model.sim_setup.reference_pvt;
    if current.process == process
        && (current.temperature_celsius - temperature_celsius).abs() < f64::EPSILON
    {
        return Ok(false);
    }
    let mut setup = app.state.model.sim_setup.clone();
    setup.set_reference_pvt(process, temperature_celsius)?;
    let receipt = setup
        .commit_active_plan_configuration_change(format!(
            "Reference PVT changed to {} at {temperature_celsius} degrees Celsius.",
            process.short_name()
        ))
        .map_err(|error| error.to_string())?;
    app.state.model.sim_setup = setup;
    app.invalidate_simulation_preflight();
    app.state.record_plan_receipt(receipt.status_line());
    Ok(true)
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
