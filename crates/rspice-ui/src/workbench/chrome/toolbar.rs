//! Context toolbar.  Tool groups change with the canonical workspace while
//! global run state remains visible at the trailing edge.

use egui::{Align, Context, Frame, Layout, TopBottomPanel, Vec2};

use crate::common::RSpiceApp;
use crate::state::Tool;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{WorkbenchIcon, icon_button, labeled_icon_button_sized};
use super::super::layout::LayoutSpec;
use super::super::state::{Drawer, ModelsPage, VerificationPage, Workspace};

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    TopBottomPanel::top("workbench.context_toolbar")
        .exact_height(layout.toolbar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                egui::Stroke::new(1.0, t.color.border),
            );
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                if layout.navigator_uses_drawer {
                    if icon_button(
                        ui,
                        WorkbenchIcon::Navigator,
                        "Open navigator",
                        app.state.workbench.drawer == Some(Drawer::Navigator),
                        Vec2::splat(layout.toolbar_control_height),
                    )
                    .clicked()
                    {
                        app.state.workbench.toggle_drawer(Drawer::Navigator);
                    }
                    separator(ui);
                }

                // Context tools own the flexible middle lane. The mockup
                // keeps navigator/run/inspector controls reachable and lets
                // this lane scroll when a dense engineering toolbar exceeds
                // the tablet or phone budget.
                let context_width = context_tools_width(ui.available_width(), layout);
                egui::ScrollArea::horizontal()
                    .id_salt("workbench.context_toolbar.tools")
                    .max_width(context_width)
                    .auto_shrink([false, true])
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .show(ui, |ui| {
                        ui.horizontal_centered(|ui| workspace_tools(ui, app, layout));
                    });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if layout.inspector_uses_drawer
                        && icon_button(
                            ui,
                            WorkbenchIcon::Inspector,
                            "Open inspector",
                            app.state.workbench.drawer == Some(Drawer::Inspector),
                            Vec2::splat(layout.toolbar_control_height),
                        )
                        .clicked()
                    {
                        app.state.workbench.toggle_drawer(Drawer::Inspector);
                    }
                    run_controls(ui, app, layout);
                    if layout.show_run_config_selector {
                        run_config_selector(ui, app);
                    }
                    if layout.show_pvt_selector {
                        pvt_selector(ui, app);
                    }
                });
            });
        });
}

fn context_tools_width(available_width: f32, layout: LayoutSpec) -> f32 {
    (available_width - trailing_controls_width(layout)).max(1.0)
}

fn trailing_controls_width(layout: LayoutSpec) -> f32 {
    const TOOLBAR_GAP: f32 = 3.0;
    let run_width = if layout.width_class.is_phone() {
        100.0
    } else {
        132.0
    };
    let mut width = run_width;
    let mut controls = 1;
    if layout.show_pvt_selector {
        width += 96.0;
        controls += 1;
    }
    if layout.show_run_config_selector {
        width += 190.0;
        controls += 1;
    }
    if layout.inspector_uses_drawer {
        width += layout.toolbar_control_height;
        controls += 1;
    }
    // One gap separates the flexible context lane from the trailing lane;
    // the remaining gaps separate controls within that lane.
    width + controls as f32 * TOOLBAR_GAP
}

fn workspace_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let workspace = app.state.workbench.workspace;
    let layout = toolbar_layout_for_workspace(workspace, layout);
    ui.data_mut(|data| {
        data.insert_temp(projected_tool_count_id(), 0_usize);
    });
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

fn toolbar_layout_for_workspace(workspace: Workspace, mut layout: LayoutSpec) -> LayoutSpec {
    // The canonical compact Design toolbar keeps every precision editing
    // command in a horizontally scrollable lane. Other workspaces retain the
    // compact projection limit defined by the responsive shell.
    if workspace == Workspace::Design {
        layout.toolbar_tool_limit = None;
    }
    layout
}

fn project_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_command(ui, app, Command::OpenProject, WorkbenchIcon::Folder, layout);
    toolbar_command(ui, app, Command::Save, WorkbenchIcon::Save, layout);
    context_separator(ui, layout);
    toolbar_command(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Design),
        WorkbenchIcon::Design,
        layout,
    );
}

fn design_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    let select = app.state.schematic.tool == Tool::Select;
    if take_projected_tool_slot(ui, layout)
        && icon_button(
            ui,
            WorkbenchIcon::Select,
            "Select (Esc)",
            select,
            Vec2::splat(layout.toolbar_control_height),
        )
        .clicked()
    {
        app.state.schematic.tool = Tool::Select;
    }
    toolbar_icon_command(ui, app, Command::PlaceWire, WorkbenchIcon::Wire, layout);
    toolbar_icon_command(ui, app, Command::PlaceLabel, WorkbenchIcon::Label, layout);
    toolbar_icon_command(ui, app, Command::PlaceProbe, WorkbenchIcon::Probe, layout);
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
    toolbar_icon_command(ui, app, Command::CycleGrid, WorkbenchIcon::Grid, layout);
    toolbar_text_command(
        ui,
        app,
        Command::RunChecks,
        WorkbenchIcon::Check,
        "Run schematic checks (Ctrl+E)",
        layout,
    );
}

fn simulation_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_command(
        ui,
        app,
        Command::SimulationOptions,
        WorkbenchIcon::Sliders,
        layout,
    );
    toolbar_command(
        ui,
        app,
        Command::GenerateNetlist,
        WorkbenchIcon::Netlist,
        layout,
    );
    if !layout.width_class.is_phone() {
        toolbar_command(
            ui,
            app,
            Command::OpenWorkspace(Workspace::Results),
            WorkbenchIcon::Results,
            layout,
        );
    }
}

fn results_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_command(
        ui,
        app,
        Command::WaveformCalculator,
        WorkbenchIcon::Sliders,
        layout,
    );
    toolbar_command(
        ui,
        app,
        Command::ExportWaveformsCsv,
        WorkbenchIcon::Export,
        layout,
    );
    if !layout.width_class.is_phone() {
        toolbar_command(ui, app, Command::ZoomFit, WorkbenchIcon::ZoomFit, layout);
    }
}

fn verification_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_command(ui, app, Command::RunChecks, WorkbenchIcon::Check, layout);
    toolbar_command(
        ui,
        app,
        Command::EditSpecifications,
        WorkbenchIcon::Sliders,
        layout,
    );
    if !layout.width_class.is_phone() {
        toolbar_command(
            ui,
            app,
            Command::VerificationPage(VerificationPage::Reliability),
            WorkbenchIcon::Verify,
            layout,
        );
    }
}

fn models_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_command(
        ui,
        app,
        Command::ModelBrowser,
        WorkbenchIcon::Search,
        layout,
    );
    toolbar_command(
        ui,
        app,
        Command::PdkSettings,
        WorkbenchIcon::Settings,
        layout,
    );
    toolbar_command(
        ui,
        app,
        Command::CompileVerilogA,
        WorkbenchIcon::Netlist,
        layout,
    );
    if !layout.width_class.is_phone() {
        toolbar_command(
            ui,
            app,
            Command::ModelsPage(ModelsPage::Libraries),
            WorkbenchIcon::Models,
            layout,
        );
    }
}

fn netlist_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    toolbar_command(
        ui,
        app,
        Command::GenerateNetlist,
        WorkbenchIcon::Netlist,
        layout,
    );
    toolbar_command(
        ui,
        app,
        Command::ImportNetlist,
        WorkbenchIcon::Folder,
        layout,
    );
    toolbar_command(
        ui,
        app,
        Command::AutomationConsole,
        WorkbenchIcon::Console,
        layout,
    );
}

fn toolbar_command(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    layout: LayoutSpec,
) {
    toolbar_command_with_label_policy(ui, app, command, icon, layout, layout.toolbar_labels, None);
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
    toolbar_command_with_label_policy(ui, app, command, icon, layout, false, None);
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
    toolbar_command_with_label_policy(ui, app, command, icon, layout, true, Some(label));
}

fn toolbar_command_with_label_policy(
    ui: &mut egui::Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: WorkbenchIcon,
    layout: LayoutSpec,
    show_label: bool,
    label_override: Option<&str>,
) {
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    let spec = command.spec();
    let label = label_override.unwrap_or(spec.label);
    let enabled = command.is_enabled(app);
    let response = ui.add_enabled_ui(enabled, |ui| {
        if show_label {
            labeled_icon_button_sized(
                ui,
                icon,
                label,
                false,
                if label_override.is_some() {
                    explicit_label_width(label)
                } else {
                    label_width(label)
                },
                layout.toolbar_control_height,
            )
        } else {
            icon_button(
                ui,
                icon,
                label,
                false,
                Vec2::splat(layout.toolbar_control_height),
            )
        }
    });
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
    let width = if layout.width_class.is_phone() {
        100.0
    } else {
        132.0
    };
    let enabled = command.is_enabled(app);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width, layout.run_control_height),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled && ui.is_enabled(), &label)
    });
    ui.painter().rect_filled(
        rect,
        t.radius,
        if enabled {
            if running { t.color.err } else { t.color.accent }
        } else {
            t.color.bg_inset
        },
    );
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 16.0, rect.center().y),
            Vec2::splat(14.0),
        ),
        if enabled {
            t.color.accent_ink
        } else {
            t.color.text_faint
        },
    );
    ui.painter().text(
        egui::Pos2::new(rect.left() + 30.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        &label,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        if enabled {
            t.color.accent_ink
        } else {
            t.color.text_faint
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    if response.clicked() && enabled {
        command.execute(app);
    }
    if !enabled {
        response.on_hover_text(
            app.state
                .simulation_run_block_reason()
                .unwrap_or_else(|| "Command unavailable".to_owned()),
        );
    }
}

fn pvt_selector(ui: &mut egui::Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let reference = app.state.sim_setup.reference_pvt;
    egui::ComboBox::from_id_salt("workbench.pvt")
        .selected_text(
            egui::RichText::new(format!(
                "{} · {} °C",
                reference.process.short_name(),
                pvt_temperature_label(reference.temperature_celsius),
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text),
        )
        .width(96.0)
        .show_ui(ui, |ui| {
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
                        "{} · {} °C",
                        process.short_name(),
                        pvt_temperature_label(temperature)
                    );
                    if ui.selectable_label(selected, label).clicked() {
                        app.state
                            .sim_setup
                            .set_reference_pvt(process, temperature)
                            .expect("the mockup-defined reference PVT points are valid");
                    }
                }
            }
        });
}

fn run_config_selector(ui: &mut egui::Ui, app: &mut RSpiceApp) {
    const WIDTH: f32 = 190.0;
    let t = Tokens::get(ui.ctx());
    let analysis_count = app.state.sim_setup.enabled_analysis_instance_count();
    let pvt_count = configured_pvt_count(app);
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(WIDTH, 31.0),
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
    WorkbenchIcon::Simulate.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 13.0, rect.center().y),
            Vec2::splat(14.0),
        ),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::Pos2::new(rect.left() + 25.0, rect.top() + 9.0),
        egui::Align2::LEFT_CENTER,
        "Lab characterization",
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    ui.painter().text(
        egui::Pos2::new(rect.left() + 25.0, rect.bottom() - 8.0),
        egui::Align2::LEFT_CENTER,
        format!("{pvt_count} PVT · {analysis_count} analyses"),
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
    theme::paint_focus_ring(ui, &response, rect);
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

fn pvt_temperature_label(value: f64) -> String {
    let text = if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        value.to_string()
    };
    text.replace('-', "−")
}

fn separator(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(9.0, 30.0), egui::Sense::hover());
    ui.painter().vline(
        rect.center().x,
        egui::Rangef::new(rect.top() + 6.0, rect.bottom() - 6.0),
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn label_width(label: &str) -> f32 {
    (label.chars().count() as f32 * 6.7 + 36.0).clamp(76.0, 162.0)
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
        assert_eq!(label_width("Run"), 76.0);
        assert!(label_width("A very long engineering command label") <= 162.0);
        assert!(explicit_label_width("Run schematic checks (Ctrl+E)") > 162.0);
        assert!(explicit_label_width("A very long engineering command label") <= 224.0);
    }

    #[test]
    fn responsive_toolbar_reserves_fixed_trailing_controls() {
        let tablet = LayoutSpec::resolve(834.0, 1112.0, &WorkbenchState::default());
        let available_after_navigator = 780.0;
        assert_eq!(trailing_controls_width(tablet), 266.0);
        assert_eq!(
            context_tools_width(available_after_navigator, tablet),
            514.0
        );

        let phone = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(trailing_controls_width(phone), 140.0);
        assert!(context_tools_width(336.0, phone) > 0.0);
    }

    #[test]
    fn compact_design_toolbar_scrolls_every_tool_without_changing_other_workspaces() {
        let compact = LayoutSpec::resolve(390.0, 844.0, &WorkbenchState::default());
        assert_eq!(compact.toolbar_tool_limit, Some(2));

        assert_eq!(
            toolbar_layout_for_workspace(Workspace::Design, compact).toolbar_tool_limit,
            None
        );
        assert_eq!(
            toolbar_layout_for_workspace(Workspace::Simulate, compact).toolbar_tool_limit,
            Some(2)
        );
    }
}
