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
    if layout.inspector_uses_drawer {
        width += layout.toolbar_control_height;
        controls += 1;
    }
    // One gap separates the flexible context lane from the trailing lane;
    // the remaining gaps separate controls within that lane.
    width + controls as f32 * TOOLBAR_GAP
}

fn workspace_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
    ui.data_mut(|data| {
        data.insert_temp(projected_tool_count_id(), 0_usize);
    });
    match app.state.workbench.workspace {
        Workspace::Project => project_tools(ui, app, layout),
        Workspace::Design => design_tools(ui, app, layout),
        Workspace::Simulate => simulation_tools(ui, app, layout),
        Workspace::Results => results_tools(ui, app, layout),
        Workspace::Verify => verification_tools(ui, app, layout),
        Workspace::Models => models_tools(ui, app, layout),
        Workspace::Netlist => netlist_tools(ui, app, layout),
    }
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
    toolbar_command(ui, app, Command::PlaceWire, WorkbenchIcon::Wire, layout);
    toolbar_command(ui, app, Command::PlaceLabel, WorkbenchIcon::Label, layout);
    toolbar_command(ui, app, Command::PlaceProbe, WorkbenchIcon::Probe, layout);
    toolbar_command(ui, app, Command::PlaceInstance, WorkbenchIcon::Add, layout);
    context_separator(ui, layout);
    toolbar_command(
        ui,
        app,
        Command::RotateSelection,
        WorkbenchIcon::Rotate,
        layout,
    );
    toolbar_command(
        ui,
        app,
        Command::MirrorSelectionHorizontal,
        WorkbenchIcon::Mirror,
        layout,
    );
    context_separator(ui, layout);
    toolbar_command(ui, app, Command::Undo, WorkbenchIcon::Undo, layout);
    toolbar_command(ui, app, Command::Redo, WorkbenchIcon::Redo, layout);
    if !layout.width_class.is_phone() {
        context_separator(ui, layout);
        toolbar_command(ui, app, Command::ZoomOut, WorkbenchIcon::ZoomOut, layout);
        toolbar_command(ui, app, Command::ZoomIn, WorkbenchIcon::ZoomIn, layout);
        toolbar_command(ui, app, Command::ZoomFit, WorkbenchIcon::ZoomFit, layout);
        toolbar_command(ui, app, Command::CycleGrid, WorkbenchIcon::Grid, layout);
        toolbar_command(ui, app, Command::RunChecks, WorkbenchIcon::Check, layout);
    }
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
    if !take_projected_tool_slot(ui, layout) {
        return;
    }
    let spec = command.spec();
    let enabled = command.is_enabled(app);
    let response = ui.add_enabled_ui(enabled, |ui| {
        if layout.toolbar_labels {
            labeled_icon_button_sized(
                ui,
                icon,
                spec.label,
                false,
                label_width(spec.label),
                layout.toolbar_control_height,
            )
        } else {
            icon_button(
                ui,
                icon,
                spec.label,
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
        "Stop"
    } else if layout.width_class.is_phone() {
        "Run plan"
    } else {
        "Run active plan"
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
        egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled && ui.is_enabled(), label)
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
        label,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::state::WorkbenchState;

    #[test]
    fn long_toolbar_labels_remain_bounded() {
        assert_eq!(label_width("Run"), 76.0);
        assert!(label_width("A very long engineering command label") <= 162.0);
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
}
