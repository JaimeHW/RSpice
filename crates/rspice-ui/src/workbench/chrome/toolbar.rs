//! Context toolbar.  Tool groups change with the canonical workspace while
//! global run state remains visible at the trailing edge.

use egui::{Align, Context, Frame, Layout, TopBottomPanel, Vec2};

use crate::common::RSpiceApp;
use crate::state::Tool;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{TOOL_BAR_H, WorkbenchIcon, icon_button, labeled_icon_button};
use super::super::layout::LayoutSpec;
use super::super::state::{Drawer, ModelsPage, VerificationPage, Workspace};

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    TopBottomPanel::top("workbench.context_toolbar")
        .exact_height(TOOL_BAR_H)
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
                if layout.width_class.uses_drawers() {
                    if icon_button(
                        ui,
                        WorkbenchIcon::Navigator,
                        "Open navigator",
                        app.state.workbench.drawer == Some(Drawer::Navigator),
                        Vec2::splat(36.0),
                    )
                    .clicked()
                    {
                        app.state.workbench.toggle_drawer(Drawer::Navigator);
                    }
                    separator(ui);
                }

                workspace_tools(ui, app, layout);

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if layout.width_class.uses_drawers()
                        && icon_button(
                            ui,
                            WorkbenchIcon::Inspector,
                            "Open inspector",
                            app.state.workbench.drawer == Some(Drawer::Inspector),
                            Vec2::splat(36.0),
                        )
                        .clicked()
                    {
                        app.state.workbench.toggle_drawer(Drawer::Inspector);
                    }
                    run_controls(ui, app, layout);
                    if !layout.width_class.is_phone() {
                        pvt_selector(ui, app);
                    }
                });
            });
        });
}

fn workspace_tools(ui: &mut egui::Ui, app: &mut RSpiceApp, layout: LayoutSpec) {
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
    separator(ui);
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
    if icon_button(
        ui,
        WorkbenchIcon::Select,
        "Select (Esc)",
        select,
        Vec2::splat(36.0),
    )
    .clicked()
    {
        app.state.schematic.tool = Tool::Select;
    }
    toolbar_command(ui, app, Command::PlaceWire, WorkbenchIcon::Wire, layout);
    toolbar_command(ui, app, Command::PlaceLabel, WorkbenchIcon::Label, layout);
    toolbar_command(ui, app, Command::PlaceProbe, WorkbenchIcon::Probe, layout);
    toolbar_command(ui, app, Command::PlaceInstance, WorkbenchIcon::Add, layout);
    separator(ui);
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
    separator(ui);
    toolbar_command(ui, app, Command::Undo, WorkbenchIcon::Undo, layout);
    toolbar_command(ui, app, Command::Redo, WorkbenchIcon::Redo, layout);
    if !layout.width_class.is_phone() {
        separator(ui);
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
    let spec = command.spec();
    let enabled = command.is_enabled(app);
    let response = ui.add_enabled_ui(enabled, |ui| {
        if layout.toolbar_labels {
            labeled_icon_button(ui, icon, spec.label, false, label_width(spec.label))
        } else {
            icon_button(ui, icon, spec.label, false, Vec2::splat(36.0))
        }
    });
    if response.inner.clicked() {
        command.execute(app);
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
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 34.0), egui::Sense::click());
    let enabled = command.is_enabled(app);
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
    egui::ComboBox::from_id_salt("workbench.pvt")
        .selected_text(
            egui::RichText::new(format!(
                "{} · 27 °C",
                app.state.workbench.corner.to_uppercase()
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text),
        )
        .width(96.0)
        .show_ui(ui, |ui| {
            for corner in ["ff", "fs", "tt", "sf", "ss"] {
                ui.selectable_value(
                    &mut app.state.workbench.corner,
                    corner.to_owned(),
                    corner.to_uppercase(),
                );
            }
        });
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

    #[test]
    fn long_toolbar_labels_remain_bounded() {
        assert_eq!(label_width("Run"), 76.0);
        assert!(label_width("A very long engineering command label") <= 162.0);
    }
}
