//! Context toolbar.  Tool groups change with the canonical workspace while
//! global run state remains visible at the trailing edge.

use egui::containers::menu::MenuButton;
use egui::{Align, Context, Frame, Layout, TopBottomPanel, Vec2};

use crate::common::RSpiceApp;
use crate::state::Tool;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
use super::super::layout::LayoutSpec;
use super::super::state::{Drawer, Workspace};

const TOOLBAR_CONTEXT_GAP: f32 = 3.0;

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let viewport_width = ctx.content_rect().width();
    let padding = toolbar_horizontal_padding(viewport_width);
    let outer_gap = toolbar_outer_gap(viewport_width);
    let show_simulation_context = app.state.project_lifecycle.project_open
        && matches!(
            app.state.workbench.workspace,
            Workspace::Design | Workspace::Simulate
        );
    TopBottomPanel::top("workbench.context_toolbar")
        .exact_height(layout.toolbar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                (rect.bottom() - 0.5).max(rect.top()),
                egui::Stroke::new(1.0, t.color.border),
            );
            let toolbar = ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = outer_gap;
                ui.add_space(padding);
                if layout.navigator_uses_drawer {
                    let navigator_toggle = icon_button(
                        ui,
                        WorkbenchIcon::Navigator,
                        "Open navigator",
                        app.state.workbench.drawer == Some(Drawer::Navigator),
                        panel_toggle_size(layout),
                    );
                    if navigator_toggle.clicked() {
                        let opening = app.state.workbench.drawer != Some(Drawer::Navigator);
                        remember_drawer_invoker(ui.ctx(), Drawer::Navigator, navigator_toggle.id);
                        if opening {
                            app.state.workbench.focus_navigator_search = true;
                            mark_drawer_focus_pending(ui.ctx(), Drawer::Navigator);
                        }
                        app.state.workbench.toggle_drawer(Drawer::Navigator);
                    }
                }

                // Context tools own the flexible middle lane. The mockup
                // keeps navigator/run/inspector controls reachable and lets
                // this lane scroll when a dense engineering toolbar exceeds
                // the tablet or phone budget.
                let run_width = reserved_run_control_width(ui, app, layout);
                let pvt_width = reserved_pvt_selector_width(ui, app);
                let context_width = context_tools_width(
                    ui.available_width(),
                    layout,
                    padding,
                    outer_gap,
                    show_simulation_context,
                    run_width,
                    pvt_width,
                );
                egui::ScrollArea::horizontal()
                    .id_salt("workbench.context_toolbar.tools")
                    .max_width(context_width)
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.x = TOOLBAR_CONTEXT_GAP;
                        ui.horizontal_centered(|ui| workspace_tools(ui, app, layout));
                    });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = TOOLBAR_CONTEXT_GAP;
                    ui.add_space(padding);
                    if layout.inspector_uses_drawer {
                        let inspector_toggle = icon_button(
                            ui,
                            WorkbenchIcon::Inspector,
                            "Open inspector",
                            app.state.workbench.drawer == Some(Drawer::Inspector),
                            panel_toggle_size(layout),
                        );
                        if inspector_toggle.clicked() {
                            let opening = app.state.workbench.drawer != Some(Drawer::Inspector);
                            remember_drawer_invoker(
                                ui.ctx(),
                                Drawer::Inspector,
                                inspector_toggle.id,
                            );
                            if opening {
                                mark_drawer_focus_pending(ui.ctx(), Drawer::Inspector);
                            }
                            app.state.workbench.toggle_drawer(Drawer::Inspector);
                        }
                    }
                    if show_simulation_context {
                        run_controls(ui, app, layout);
                        if layout.show_run_config_selector {
                            run_config_selector(ui, app, layout.run_control_height);
                        }
                        if layout.show_pvt_selector {
                            pvt_selector(ui, app, layout.toolbar_control_height);
                        }
                    }
                });
            });
            ui.ctx()
                .accesskit_node_builder(toolbar.response.id, |node| {
                    node.set_role(egui::accesskit::Role::Toolbar);
                    node.set_label("Context toolbar");
                });
        });
}

fn toolbar_horizontal_padding(viewport_width: f32) -> f32 {
    if viewport_width <= 560.0 {
        3.0
    } else if viewport_width <= 820.0 {
        5.0
    } else {
        8.0
    }
}

fn drawer_owner(drawer: Drawer) -> &'static str {
    match drawer {
        Drawer::Navigator => "navigator",
        Drawer::Inspector => "inspector",
        Drawer::Workspaces => "workspaces",
    }
}

fn remember_drawer_invoker(ctx: &Context, drawer: Drawer, invoker: egui::Id) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.drawer.invoker", drawer_owner(drawer))),
            invoker,
        );
    });
}

fn mark_drawer_focus_pending(ctx: &Context, drawer: Drawer) {
    ctx.data_mut(|data| {
        data.insert_temp(
            egui::Id::new(("workbench.drawer.focus_pending", drawer_owner(drawer))),
            true,
        );
    });
}

fn toolbar_outer_gap(viewport_width: f32) -> f32 {
    if viewport_width <= 560.0 { 2.0 } else { 5.0 }
}

fn reserved_run_control_width(ui: &egui::Ui, app: &RSpiceApp, layout: LayoutSpec) -> f32 {
    let label = if app.state.simulation.is_running {
        "Stop".to_owned()
    } else if layout.compact_shell {
        format!(
            "Run · {}",
            app.state.sim_setup.reference_pvt.process.short_name()
        )
    } else {
        "Run plan".to_owned()
    };
    let t = Tokens::get(ui.ctx());
    let label_width = ui
        .painter()
        .layout_no_wrap(
            label,
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        )
        .size()
        .x;
    let primary_width = (42.0 + label_width).max(67.0);
    let menu_width = if layout.run_control_height >= 44.0 {
        44.0
    } else {
        25.0
    };
    primary_width + menu_width
}

fn reserved_pvt_selector_width(ui: &egui::Ui, app: &RSpiceApp) -> f32 {
    let reference = app.state.sim_setup.reference_pvt;
    let quantity_policy = app.state.ui.preferences.quantity_presentation_policy();
    let label = format!(
        "{} · {}",
        reference.process.short_name(),
        pvt_temperature_label(reference.temperature_celsius, quantity_policy),
    );
    let t = Tokens::get(ui.ctx());
    ui.painter()
        .layout_no_wrap(
            label,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
        )
        .size()
        .x
        + 42.0
}

fn context_tools_width(
    available_width: f32,
    layout: LayoutSpec,
    right_padding: f32,
    outer_gap: f32,
    show_simulation_context: bool,
    run_width: f32,
    pvt_width: f32,
) -> f32 {
    (available_width
        - trailing_controls_width(
            layout,
            outer_gap,
            show_simulation_context,
            run_width,
            pvt_width,
        )
        - right_padding)
        .max(1.0)
}

fn trailing_controls_width(
    layout: LayoutSpec,
    outer_gap: f32,
    show_simulation_context: bool,
    run_width: f32,
    pvt_width: f32,
) -> f32 {
    let mut width = 0.0;
    let mut controls: usize = 0;
    if show_simulation_context {
        width += run_width;
        controls += 1;
        if layout.show_pvt_selector {
            width += pvt_width;
            controls += 1;
        }
        if layout.show_run_config_selector {
            width += 190.0;
            controls += 1;
        }
    }
    if layout.inspector_uses_drawer {
        width += panel_toggle_size(layout).x;
        controls += 1;
    }
    width + (controls.saturating_sub(1) as f32 * TOOLBAR_CONTEXT_GAP) + outer_gap
}

fn workspace_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let workspace = app.state.workbench.workspace;
    ui.data_mut(|data| {
        data.insert_temp(projected_tool_count_id(), 0_usize);
    });
    if !app.state.project_lifecycle.project_open {
        no_project_tools(ui, app, layout);
        return;
    }
    match workspace {
        Workspace::Project => project_tools(ui, app, layout),
        Workspace::Design => design_tools(ui, app, layout),
        Workspace::Simulate => simulation_tools(ui, app, layout),
        Workspace::Results => results_tools(ui, app, layout),
        Workspace::Verify => verification_tools(ui, app, layout),
        Workspace::Models => models_tools(ui, app, layout),
        Workspace::Netlist => netlist_tools(ui, app, layout),
    }
}

fn no_project_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_text_command(
        ui,
        app,
        Command::ProjectLauncher,
        WorkbenchIcon::Folder,
        "Open project launcher",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::NewProject,
        WorkbenchIcon::Add,
        "New project",
        layout,
    );
}

fn project_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_text_command(
        ui,
        app,
        Command::NewCell,
        WorkbenchIcon::Add,
        "New cell",
        layout,
    );
}

fn design_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let select = app.state.schematic.tool == Tool::Select;
    let wire = app.state.schematic.tool == Tool::Wire;
    let label = app.state.schematic.tool == Tool::Label;
    let probe = app.state.schematic.tool == Tool::Probe;
    let grid = app.state.ui.grid != crate::workbench::GridStyle::Off;
    toolbar_icon_command_selected(
        ui,
        app,
        Command::SelectTool,
        WorkbenchIcon::Select,
        select,
        layout,
    );
    toolbar_icon_command_selected(
        ui,
        app,
        Command::PlaceWire,
        WorkbenchIcon::Wire,
        wire,
        layout,
    );
    toolbar_icon_command_selected(
        ui,
        app,
        Command::PlaceLabel,
        WorkbenchIcon::Label,
        label,
        layout,
    );
    toolbar_icon_command_selected(
        ui,
        app,
        Command::PlaceProbe,
        WorkbenchIcon::Probe,
        probe,
        layout,
    );
    context_separator(ui, layout);
    toolbar_icon_command(
        ui,
        app,
        Command::RotateSelection,
        WorkbenchIcon::Rotate,
        layout,
    );
    toolbar_icon_command(
        ui,
        app,
        Command::MirrorSelectionHorizontal,
        WorkbenchIcon::Mirror,
        layout,
    );
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::Undo, WorkbenchIcon::Undo, layout);
    toolbar_icon_command(ui, app, Command::Redo, WorkbenchIcon::Redo, layout);
    context_separator(ui, layout);
    toolbar_icon_command(ui, app, Command::ZoomOut, WorkbenchIcon::ZoomOut, layout);
    toolbar_icon_command(ui, app, Command::ZoomIn, WorkbenchIcon::ZoomIn, layout);
    toolbar_icon_command(ui, app, Command::ZoomFit, WorkbenchIcon::ZoomFit, layout);
    toolbar_icon_command_selected(
        ui,
        app,
        Command::CycleGrid,
        WorkbenchIcon::Grid,
        grid,
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::RunChecks,
        WorkbenchIcon::Check,
        "Run schematic checks",
        layout,
    );
}

fn simulation_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_icon_command(
        ui,
        app,
        Command::SimulationOptions,
        WorkbenchIcon::Sliders,
        layout,
    );
    context_separator(ui, layout);
    toolbar_text_command(
        ui,
        app,
        Command::PreflightChecks,
        WorkbenchIcon::Check,
        "Run preflight",
        layout,
    );
}

fn results_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_icon_command(
        ui,
        app,
        Command::WaveformCalculator,
        WorkbenchIcon::Target,
        layout,
    );
    context_separator(ui, layout);
    toolbar_text_command(
        ui,
        app,
        Command::ExportWaveformsCsv,
        WorkbenchIcon::Export,
        "Export active result document data",
        layout,
    );
}

fn verification_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_icon_command(
        ui,
        app,
        Command::EditSpecifications,
        WorkbenchIcon::Target,
        layout,
    );
}

fn models_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_icon_command(
        ui,
        app,
        Command::CompileVerilogA,
        WorkbenchIcon::Netlist,
        layout,
    );
}

fn netlist_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let generated = app.state.ui.netlist.active_document
        == crate::workbench::netlist_document::ActiveNetlistDocument::Generated;
    toolbar_text_command(
        ui,
        app,
        Command::ExportNetlist(crate::io::NetlistFormat::Spice),
        WorkbenchIcon::Export,
        "Export generated deck",
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::FindCodeDocument,
        WorkbenchIcon::Search,
        if generated {
            "Find generated netlist"
        } else {
            "Find source"
        },
        layout,
    );
    toolbar_text_command(
        ui,
        app,
        Command::ValidateCodeDocument,
        WorkbenchIcon::Check,
        if generated {
            "Validate generated netlist"
        } else {
            "Validate source"
        },
        layout,
    );
    context_separator(ui, layout);
    toolbar_text_command(
        ui,
        app,
        Command::CompareGeneratedRevisions,
        WorkbenchIcon::Compare,
        "Compare generated revisions",
        layout,
    );
    toolbar_icon_command(ui, app, Command::RunSimulation, WorkbenchIcon::Run, layout);
}

/// Render a canonical compact precision tool. The design mockup uses these
/// for all direct canvas transforms and placement modes.
fn toolbar_icon_command(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    layout: LayoutSpec,
) {
    toolbar_icon_command_selected(ui, app, command, icon, false, layout);
}

fn toolbar_icon_command_selected(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    selected: bool,
    layout: LayoutSpec,
) {
    toolbar_command(
        ui,
        app,
        command,
        layout,
        ToolbarCommandPresentation::Icon { icon, selected },
    );
}

/// Render one explicitly labeled toolbar action. The design mockup reserves
/// this treatment for checks so its state-changing validation scope remains
/// visible without opening a menu.
fn toolbar_text_command(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    label: &'static str,
    layout: LayoutSpec,
) {
    toolbar_command(
        ui,
        app,
        command,
        layout,
        ToolbarCommandPresentation::Text { icon, label },
    );
}

#[derive(Clone, Copy)]
enum ToolbarCommandPresentation {
    Icon {
        icon: WorkbenchIcon,
        selected: bool,
    },
    Text {
        icon: WorkbenchIcon,
        label: &'static str,
    },
}

fn toolbar_command(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    layout: LayoutSpec,
    presentation: ToolbarCommandPresentation,
) {
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    let spec = command.spec();
    let (icon, base_label, show_label, selected) = match presentation {
        ToolbarCommandPresentation::Icon { icon, selected } => (icon, spec.label, false, selected),
        ToolbarCommandPresentation::Text { icon, label } => {
            (icon, label, layout.toolbar_labels, false)
        }
    };
    let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
        command,
        crate::common::app::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    let label = if shortcut.is_empty() {
        base_label.to_owned()
    } else {
        format!("{base_label} ({shortcut})")
    };
    let enabled = command.is_enabled(app);
    let response = ui.add_enabled_ui(enabled, |ui| {
        if show_label {
            labeled_icon_button_sized(
                ui,
                icon,
                &label,
                selected,
                explicit_label_width(&label),
                layout.toolbar_control_height,
            )
        } else {
            icon_button(ui, icon, &label, selected, toolbar_icon_button_size(layout))
        }
    });
    if enabled && !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.inner.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    if response.inner.clicked() {
        command.execute(app);
    }
}

fn projected_tool_count_id() -> egui::Id {
    egui::Id::new("workbench.context_toolbar.projected_tool_count")
}

fn take_projected_tool_slot(ui: &mut egui::Ui, layout: LayoutSpec) -> bool {
    let Some(limit) = layout.toolbar_tool_limit else {
        return true;
    };
    let id = projected_tool_count_id();
    let count = ui.data(|data| data.get_temp::<usize>(id).unwrap_or_default());
    if count >= limit {
        return false;
    }
    ui.data_mut(|data| data.insert_temp(id, count + 1));
    true
}

fn context_separator(ui: &mut egui::Ui, layout: LayoutSpec) {
    if layout.toolbar_tool_limit.is_none() {
        separator(ui);
    }
}

fn run_controls(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let running = app.state.simulation.is_running;
    let command = if running {
        Command::StopSimulation
    } else {
        Command::RunSimulation
    };
    let icon = if running {
        WorkbenchIcon::Stop
    } else {
        WorkbenchIcon::Run
    };
    let t = Tokens::get(ui.ctx());
    let radius = t.radius.round().clamp(0.0, u8::MAX as f32) as u8;
    let label = if running {
        "Stop".to_owned()
    } else if layout.compact_shell {
        format!(
            "Run · {}",
            app.state.sim_setup.reference_pvt.process.short_name()
        )
    } else {
        "Run plan".to_owned()
    };
    let label_width = ui
        .painter()
        .layout_no_wrap(
            label.clone(),
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
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
            let accessibility_label = if running {
                "Stop active simulation".to_owned()
            } else {
                let reference = app.state.sim_setup.reference_pvt;
                let quantity_policy = app.state.ui.preferences.quantity_presentation_policy();
                format!(
                    "Run active simulation plan at {} · {}",
                    reference.process.short_name(),
                    pvt_temperature_label(reference.temperature_celsius, quantity_policy)
                )
            };
            let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
                command,
                crate::common::app::runtime_command_platform(ui.ctx()),
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
                if running { t.color.err } else { t.color.accent }
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
            } else if running {
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
                // The mockup run control inherits the 13 px application body type.
                theme::sans(tokens::FS_2, FontWeight::SemiBold),
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
                let reason = if app.state.workbench.workspace == Workspace::Netlist {
                    app.manual_deck_run_block_reason()
                } else {
                    app.state.simulation_run_block_reason()
                };
                response.on_hover_text(reason.unwrap_or_else(|| "Command unavailable".to_owned()));
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
                        let primary = if running {
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
    let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
        command,
        crate::common::app::runtime_command_platform(ui.ctx()),
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

fn pvt_selector(ui: &mut egui::Ui, app: &mut RSpiceApp, height: f32) {
    let t = Tokens::get(ui.ctx());
    let reference = app.state.sim_setup.reference_pvt;
    let quantity_policy = app.state.ui.preferences.quantity_presentation_policy();
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
        + 42.0;
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
                use crate::simulation::dialog::corner::ProcessCorner;
                for process in [
                    ProcessCorner::FF,
                    ProcessCorner::FS,
                    ProcessCorner::TT,
                    ProcessCorner::SF,
                    ProcessCorner::SS,
                ] {
                    for temperature in [-40.0, 27.0, 125.0] {
                        let selected = reference.process == process
                            && (reference.temperature_celsius - temperature).abs() < f64::EPSILON;
                        let label = format!(
                            "{} · {}",
                            process.short_name(),
                            pvt_temperature_label(temperature, quantity_policy)
                        );
                        let option_height = if height >= 44.0 { 44.0 } else { 29.0 };
                        if ui
                            .add_sized(
                                [190.0, option_height],
                                egui::Button::selectable(selected, label),
                            )
                            .clicked()
                        {
                            app.state
                                .sim_setup
                                .set_reference_pvt(process, temperature)
                                .expect("the mockup-defined reference PVT points are valid");
                            ui.close();
                        }
                    }
                }
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
    ui.painter().circle_filled(
        egui::pos2(response.rect.left() + 11.0, response.rect.center().y),
        3.0,
        t.color.accent,
    );
    let text_clip = egui::Rect::from_min_max(
        egui::pos2(response.rect.left() + 21.0, response.rect.top()),
        egui::pos2(response.rect.right() - 21.0, response.rect.bottom()),
    );
    ui.painter().with_clip_rect(text_clip).text(
        egui::pos2(response.rect.left() + 21.0, response.rect.center().y),
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

fn brighten_srgb(color: egui::Color32, factor: f32) -> egui::Color32 {
    let channel = |value: u8| (f32::from(value) * factor).round().clamp(0.0, 255.0) as u8;
    egui::Color32::from_rgba_unmultiplied(
        channel(color.r()),
        channel(color.g()),
        channel(color.b()),
        color.a(),
    )
}

fn run_config_selector(ui: &mut egui::Ui, app: &mut RSpiceApp, height: f32) {
    let t = Tokens::get(ui.ctx());
    let analysis_count = app.state.sim_setup.enabled_analysis_instance_count();
    let pvt_count = configured_pvt_count(app);
    let summary = format!("{pvt_count} PVT · {analysis_count} analyses");
    let title_width = ui
        .painter()
        .layout_no_wrap(
            "Lab characterization".to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.text,
        )
        .size()
        .x;
    let summary_width = ui
        .painter()
        .layout_no_wrap(
            summary.clone(),
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        )
        .size()
        .x;
    let width = (31.0 + title_width.max(summary_width) + 22.0).min(190.0);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width, height),
        if ui.is_enabled() {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Open Lab characterization simulation plan",
        )
    });

    ui.painter().rect(
        rect,
        t.radius,
        if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
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
    WorkbenchIcon::Sliders.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 16.0, rect.center().y),
            Vec2::splat(16.0),
        ),
        t.color.text_dim,
    );
    let copy_clip = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 31.0, rect.top()),
        egui::pos2(rect.right() - 22.0, rect.bottom()),
    );
    let copy_painter = ui.painter().with_clip_rect(copy_clip);
    copy_painter.text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y - 6.5),
        egui::Align2::LEFT_CENTER,
        "Lab characterization",
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    copy_painter.text(
        egui::Pos2::new(rect.left() + 31.0, rect.center().y + 7.0),
        egui::Align2::LEFT_CENTER,
        summary,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.right() - 10.0, rect.center().y),
            Vec2::splat(11.0),
        ),
        t.color.text_faint,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    if response.clicked() {
        Command::OpenWorkspace(Workspace::Simulate).execute(app);
    }
}

fn configured_pvt_count(app: &RSpiceApp) -> usize {
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};

    let mut count = usize::from(
        app.state
            .sim_setup
            .enabled_analysis_instances()
            .any(|instance| {
                !matches!(
                    instance.kind(),
                    AnalysisKind::Corner | AnalysisKind::Temperature
                )
            }),
    );
    for instance in app.state.sim_setup.enabled_analysis_instances() {
        count = count.saturating_add(match instance.draft() {
            AnalysisDraft::Corner(draft) => {
                draft.to_config().map_or(0, |config| config.num_corners())
            }
            AnalysisDraft::Temperature(draft) => {
                draft.to_config().map_or(0, |config| config.num_temps())
            }
            _ => 0,
        });
    }
    count.max(1)
}

fn pvt_temperature_label(
    celsius: f64,
    policy: crate::quantity::QuantityPresentationPolicy,
) -> String {
    policy
        .format_temperature(celsius + 273.15, 1)
        .replace(".0 ", " ")
        .replace('-', "−")
}

fn separator(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(11.0, 23.0), egui::Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn toolbar_icon_button_size(layout: LayoutSpec) -> Vec2 {
    if layout.toolbar_control_height >= 44.0 {
        Vec2::splat(44.0)
    } else {
        Vec2::new(30.0, 29.0)
    }
}

fn panel_toggle_size(layout: LayoutSpec) -> Vec2 {
    if layout.toolbar_control_height >= 44.0 {
        Vec2::splat(44.0)
    } else {
        Vec2::new(28.0, 27.0)
    }
}

fn explicit_label_width(label: &str) -> f32 {
    (label.chars().count() as f32 * 6.7 + 36.0).clamp(76.0, 224.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::state::WorkbenchState;

    #[test]
    fn long_toolbar_labels_remain_bounded() {
        assert_eq!(explicit_label_width("Run"), 76.0);
        assert!(explicit_label_width("Run schematic checks (Ctrl+E)") > 162.0);
        assert!(explicit_label_width("A very long engineering command label") <= 224.0);
    }

    #[test]
    fn responsive_toolbar_reserves_fixed_trailing_controls() {
        let tablet = LayoutSpec::resolve(834.0, 1112.0, &WorkbenchState::default());
        let available_after_navigator = 780.0;
        assert_eq!(
            trailing_controls_width(tablet, 5.0, true, 117.0, 96.0),
            252.0
        );
        assert_eq!(
            context_tools_width(
                available_after_navigator,
                tablet,
                8.0,
                5.0,
                true,
                117.0,
                96.0,
            ),
            520.0
        );

        let phone = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(
            trailing_controls_width(phone, 2.0, true, 136.0, 106.0),
            185.0
        );
        assert_eq!(
            trailing_controls_width(phone, 2.0, false, 136.0, 106.0),
            46.0
        );
        assert!(context_tools_width(336.0, phone, 3.0, 2.0, true, 136.0, 106.0) > 0.0);
    }

    #[test]
    fn toolbar_padding_and_group_gaps_match_every_mockup_breakpoint() {
        assert_eq!(toolbar_horizontal_padding(1_280.0), 8.0);
        assert_eq!(toolbar_horizontal_padding(820.0), 5.0);
        assert_eq!(toolbar_horizontal_padding(560.0), 3.0);
        assert_eq!(toolbar_outer_gap(1_280.0), 5.0);
        assert_eq!(toolbar_outer_gap(820.0), 5.0);
        assert_eq!(toolbar_outer_gap(560.0), 2.0);
        assert_eq!(TOOLBAR_CONTEXT_GAP, 3.0);
    }

    #[test]
    fn toolbar_icon_and_panel_toggle_boxes_match_the_mockup_classes() {
        let desktop = LayoutSpec::resolve(1_280.0, 900.0, &WorkbenchState::default());
        assert_eq!(toolbar_icon_button_size(desktop), Vec2::new(30.0, 29.0));
        assert_eq!(panel_toggle_size(desktop), Vec2::new(28.0, 27.0));

        let phone = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(toolbar_icon_button_size(phone), Vec2::splat(44.0));
        assert_eq!(panel_toggle_size(phone), Vec2::splat(44.0));
    }

    #[test]
    fn compact_toolbar_projects_the_mockup_button_limit_for_every_workspace() {
        let compact = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(compact.toolbar_tool_limit, Some(2));
    }
}
