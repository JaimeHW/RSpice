//! Context toolbar.  Tool groups change with the canonical workspace while
//! global run state remains visible at the trailing edge.

pub(in crate::workbench) mod code;
pub(crate) mod design;
pub(in crate::workbench) mod models;
pub(crate) mod overflow;
pub(in crate::workbench) mod project;
pub(crate) mod results;
pub(in crate::workbench) mod run_config_chip;
pub(crate) mod simulate;

use egui::{Align, Context, Frame, Layout, Panel, Ui, Vec2};

use crate::state::command_catalog::CommandKey;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::ToolbarTool;
use crate::workbench::framework::host::{Point, ToolbarScope, ToolbarSlot};
use crate::workbench::framework::workbench_host::WorkbenchHost;

use super::super::layout::{LayoutSpec, RUN_CONFIG_CHIP_MAX_WIDTH};
use super::super::state::{Drawer, Workspace};
use crate::ui::panels::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
use crate::workbench::framework::command_registry::CommandAvailability;

const TOOLBAR_CONTEXT_GAP: f32 = 3.0;

pub fn show<H: WorkbenchHost>(root: &mut Ui, app: &mut H, layout: LayoutSpec) {
    let ctx = root.ctx().clone();
    let ctx = &ctx;
    let t = Tokens::get(ctx);
    let viewport_width = ctx.content_rect().width();
    let padding = toolbar_horizontal_padding(viewport_width);
    let outer_gap = toolbar_outer_gap(viewport_width);
    let show_simulation_context = app.platform().project_lifecycle.is_open()
        && matches!(
            app.workbench().workspace,
            Workspace::Design | Workspace::Simulate
        );
    Panel::top("workbench.context_toolbar")
        .exact_size(layout.toolbar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(root, |ui| {
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
                        app.workbench().drawer == Some(Drawer::Navigator),
                        panel_toggle_size(layout),
                    );
                    if navigator_toggle.clicked() {
                        let opening = app.workbench().drawer != Some(Drawer::Navigator);
                        remember_drawer_invoker(ui.ctx(), Drawer::Navigator, navigator_toggle.id);
                        if opening {
                            app.workbench_mut()
                                .outbox
                                .push(crate::workbench::framework::requests::FocusNavigatorSearch);
                            mark_drawer_focus_pending(ui.ctx(), Drawer::Navigator);
                        }
                        app.workbench_mut().toggle_drawer(Drawer::Navigator);
                    }
                }
                if app.platform().project_lifecycle.is_open() {
                    global_document_tools(ui, app, layout);
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
                ui.data_mut(|data| {
                    data.insert_temp(context_lane_width_id(), context_width);
                });
                egui::ScrollArea::horizontal()
                    .id_salt("workbench.context_toolbar.tools")
                    .max_width(context_width)
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.x = TOOLBAR_CONTEXT_GAP;
                        // The workspace's tools, and behind More tools
                        // whatever a narrow row has no slot for (U-11).
                        ui.horizontal_centered(|ui| {
                            overflow::reset::<H::Session>(ui);
                            workspace_tools(ui, app, layout);
                            overflow::more_tools(ui, app, layout);
                        });
                    });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = TOOLBAR_CONTEXT_GAP;
                    ui.add_space(padding);
                    if layout.inspector_uses_drawer {
                        let inspector_toggle = icon_button(
                            ui,
                            WorkbenchIcon::Inspector,
                            "Open inspector",
                            app.workbench().drawer == Some(Drawer::Inspector),
                            panel_toggle_size(layout),
                        );
                        if inspector_toggle.clicked() {
                            let opening = app.workbench().drawer != Some(Drawer::Inspector);
                            remember_drawer_invoker(
                                ui.ctx(),
                                Drawer::Inspector,
                                inspector_toggle.id,
                            );
                            if opening {
                                mark_drawer_focus_pending(ui.ctx(), Drawer::Inspector);
                            }
                            app.workbench_mut().toggle_drawer(Drawer::Inspector);
                        }
                    }
                    if show_simulation_context {
                        app.contribute(Point::Toolbar(ToolbarSlot::Trailing, layout), ui);
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

/// Save is the only document command with identical meaning in every
/// workspace. Keep it outside the horizontally scrolling context-tool lane so
/// it remains reachable even when a dense engineering toolbar overflows.
fn global_document_tools(ui: &mut egui::Ui, app: &mut impl WorkbenchHost, layout: LayoutSpec) {
    let dirty = crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(app.session());
    let shortcut = app.shortcut_label(
        ids::SAVE,
        crate::workbench::app_state::session::shortcuts::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    let state_label = if dirty {
        "unsaved changes in this project"
    } else {
        "no unsaved changes"
    };
    let label = if shortcut.is_empty() {
        format!("Save \u{00b7} {state_label}")
    } else {
        format!("Save ({shortcut}) \u{00b7} {state_label}")
    };
    let availability = app.availability(ids::SAVE);
    let enabled = availability == CommandAvailability::Available;
    let response = ui
        .add_enabled_ui(enabled, |ui| {
            icon_button(
                ui,
                WorkbenchIcon::Save,
                &label,
                dirty,
                toolbar_icon_button_size(layout),
            )
        })
        .inner;
    let response = match availability {
        CommandAvailability::Disabled(reason) => response.on_disabled_hover_text(reason),
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    if enabled && !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    if response.clicked() {
        app.execute(ids::SAVE);
    }
    context_separator(ui, layout);
}

pub(crate) fn toolbar_horizontal_padding(viewport_width: f32) -> f32 {
    if viewport_width <= 560.0 {
        3.0
    } else if viewport_width <= 820.0 {
        5.0
    } else {
        8.0
    }
}

pub(in crate::workbench) fn drawer_owner(drawer: Drawer) -> &'static str {
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

pub(crate) fn toolbar_outer_gap(viewport_width: f32) -> f32 {
    if viewport_width <= 560.0 { 2.0 } else { 5.0 }
}

pub(crate) fn run_control_label(app: &impl WorkbenchHost, layout: LayoutSpec) -> String {
    if app.run_preflight_checking() {
        "Stop".to_owned()
    } else if app.model().simulation.cancellation_is_pending() {
        "Stopping…".to_owned()
    } else if app.model().simulation.execution.has_active_execution() {
        "Stop".to_owned()
    } else if layout.compact_shell {
        format!(
            "Run · {}",
            app.model().sim_setup.reference_pvt.process.short_name()
        )
    } else {
        "Run plan".to_owned()
    }
}

fn reserved_run_control_width(ui: &egui::Ui, app: &impl WorkbenchHost, layout: LayoutSpec) -> f32 {
    let label = run_control_label(app, layout);
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

fn reserved_pvt_selector_width(ui: &egui::Ui, app: &impl WorkbenchHost) -> f32 {
    let reference = app.model().sim_setup.reference_pvt;
    let quantity_policy = app.model().preferences.quantity_presentation_policy();
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

pub(crate) fn context_tools_width(
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

pub(crate) fn trailing_controls_width(
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
            width += RUN_CONFIG_CHIP_MAX_WIDTH;
            controls += 1;
        }
    }
    if layout.inspector_uses_drawer {
        width += panel_toggle_size(layout).x;
        controls += 1;
    }
    width + (controls.saturating_sub(1) as f32 * TOOLBAR_CONTEXT_GAP) + outer_gap
}

fn workspace_tools(ui: &mut egui::Ui, app: &mut impl WorkbenchHost, layout: LayoutSpec) {
    let workspace = app.workbench().workspace;
    ui.data_mut(|data| {
        data.insert_temp(projected_tool_count_id(), 0_usize);
    });
    let scope = app.toolbar_scope(workspace);
    app.contribute(Point::Toolbar(ToolbarSlot::Tools(scope), layout), ui);
    if !matches!(scope, ToolbarScope::NoProject) {
        workspace_focus_tool(ui, app, layout);
    }
}

/// The upgraded shell keeps focus mode at the end of every project toolbar,
/// independent of the active workspace's domain tools.
fn workspace_focus_tool(ui: &mut egui::Ui, app: &mut impl WorkbenchHost, layout: LayoutSpec) {
    context_separator(ui, layout);
    toolbar_icon_command_selected_as(
        ui,
        app,
        ids::TOGGLE_FOCUS_MODE,
        WorkbenchIcon::Focus,
        app.workbench().focus_mode,
        if app.workbench().focus_mode {
            "Restore workspace layout"
        } else {
            "Focus workspace"
        },
        layout,
    );
}

/// Render a canonical compact precision tool. The design mockup uses these
/// for all direct canvas transforms and placement modes.
fn toolbar_icon_command(
    ui: &mut egui::Ui,
    app: &mut impl WorkbenchHost,
    command: CommandKey,
    icon: WorkbenchIcon,
    layout: LayoutSpec,
) {
    toolbar_icon_command_selected(ui, app, command, icon, false, layout);
}

fn toolbar_icon_command_selected(
    ui: &mut egui::Ui,
    app: &mut impl WorkbenchHost,
    command: CommandKey,
    icon: WorkbenchIcon,
    selected: bool,
    layout: LayoutSpec,
) {
    toolbar_command(
        ui,
        app,
        command,
        layout,
        ToolbarCommandPresentation::Icon {
            icon,
            selected,
            label: None,
        },
    );
}

fn toolbar_icon_command_selected_as(
    ui: &mut egui::Ui,
    app: &mut impl WorkbenchHost,
    command: CommandKey,
    icon: WorkbenchIcon,
    selected: bool,
    label: &'static str,
    layout: LayoutSpec,
) {
    toolbar_command(
        ui,
        app,
        command,
        layout,
        ToolbarCommandPresentation::Icon {
            icon,
            selected,
            label: Some(label),
        },
    );
}

/// Render one explicitly labeled toolbar action. The design mockup reserves
/// this treatment for checks so its state-changing validation scope remains
/// visible without opening a menu.
fn toolbar_text_command(
    ui: &mut egui::Ui,
    app: &mut impl WorkbenchHost,
    command: CommandKey,
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

/// Draw the tools a feature put on the toolbar, in order, each as a
/// labelled command.
pub(in crate::workbench) fn feature_tools(
    ui: &mut egui::Ui,
    app: &mut impl WorkbenchHost,
    tools: &[ToolbarTool],
    layout: LayoutSpec,
) {
    for tool in tools {
        toolbar_text_command(ui, app, tool.key, tool.icon, tool.label, layout);
    }
}

#[derive(Clone, Copy)]
pub(crate) enum ToolbarCommandPresentation {
    Icon {
        icon: WorkbenchIcon,
        selected: bool,
        label: Option<&'static str>,
    },
    Text {
        icon: WorkbenchIcon,
        label: &'static str,
    },
}

fn toolbar_command(
    ui: &mut egui::Ui,
    app: &mut impl WorkbenchHost,
    command: CommandKey,
    layout: LayoutSpec,
    presentation: ToolbarCommandPresentation,
) {
    let has_slot = take_projected_tool_slot(ui, layout);
    let availability = app.availability(command);
    if availability == CommandAvailability::Hidden {
        return;
    }
    if !has_slot {
        overflow::set_aside(
            ui,
            overflow::OverflowTool::command(app, command, presentation),
        );
        return;
    }
    let (icon, base_label, show_label, selected) = match presentation {
        ToolbarCommandPresentation::Icon {
            icon,
            selected,
            label,
        } => (icon, label.unwrap_or(app.label(command)), false, selected),
        ToolbarCommandPresentation::Text { icon, label } => {
            (icon, label, layout.toolbar_labels, false)
        }
    };
    let shortcut = app.shortcut_label(
        command,
        crate::workbench::app_state::session::shortcuts::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    let label = if shortcut.is_empty() {
        base_label.to_owned()
    } else {
        format!("{base_label} ({shortcut})")
    };
    let enabled = availability == CommandAvailability::Available;
    let label_width = explicit_label_width(&label);
    let show_label = show_label && label_fits_the_context_lane(ui, label_width);
    let response = ui.add_enabled_ui(enabled, |ui| {
        if show_label {
            labeled_icon_button_sized(
                ui,
                icon,
                &label,
                selected,
                label_width,
                layout.toolbar_control_height,
            )
        } else {
            icon_button(ui, icon, &label, selected, toolbar_icon_button_size(layout))
        }
    });
    let response = match availability {
        CommandAvailability::Disabled(reason) => response.inner.on_disabled_hover_text(reason),
        CommandAvailability::Available | CommandAvailability::Hidden => response.inner,
    };
    if enabled && !shortcut.is_empty() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_keyboard_shortcut(shortcut.as_str());
        });
    }
    if response.clicked() {
        app.execute(command);
    }
}

fn projected_tool_count_id() -> egui::Id {
    egui::Id::new("workbench.context_toolbar.projected_tool_count")
}

fn context_lane_width_id() -> egui::Id {
    egui::Id::new("workbench.context_toolbar.lane_width")
}

/// Can a labelled tool still show its label inside the context lane?
///
/// The lane scrolls horizontally with its bar hidden, so a label that runs
/// past the lane's right edge is painted as half a word with nothing on
/// screen to say it continues — at 1440 px with a live run, "Run schematic
/// checks" was painted as "Ru". A tool that cannot show its label shows its
/// icon, which carries the same command and the same tooltip.
fn label_fits_the_context_lane(ui: &egui::Ui, label_width: f32) -> bool {
    let Some(lane) = ui.data(|data| data.get_temp::<f32>(context_lane_width_id())) else {
        return true;
    };
    ui.min_rect().width() + ui.spacing().item_spacing.x + label_width <= lane
}

/// Take one of a narrow row's slots. The last is kept for More tools, so a
/// tool that gets none sets itself aside there to stay reachable (U-11).
fn take_projected_tool_slot(ui: &mut egui::Ui, layout: LayoutSpec) -> bool {
    let Some(limit) = layout.toolbar_tool_limit else {
        return true;
    };
    let id = projected_tool_count_id();
    let count = ui.data(|data| data.get_temp::<usize>(id).unwrap_or_default());
    ui.data_mut(|data| data.insert_temp(id, count + 1));
    count + 1 < limit
}

fn context_separator(ui: &mut egui::Ui, layout: LayoutSpec) {
    if layout.toolbar_tool_limit.is_none() {
        separator(ui);
    }
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

pub(crate) fn toolbar_icon_button_size(layout: LayoutSpec) -> Vec2 {
    if layout.toolbar_control_height >= 44.0 {
        Vec2::splat(44.0)
    } else {
        Vec2::new(30.0, 29.0)
    }
}

pub(crate) fn panel_toggle_size(layout: LayoutSpec) -> Vec2 {
    if layout.toolbar_control_height >= 44.0 {
        Vec2::splat(44.0)
    } else {
        Vec2::new(28.0, 27.0)
    }
}

pub(crate) fn explicit_label_width(label: &str) -> f32 {
    (label.chars().count() as f32 * 6.7 + 36.0).clamp(76.0, 224.0)
}
