//! Compact application status with exact engineering context.

use egui::{Align, Context, Frame, Layout, TopBottomPanel};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::STATUS_BAR_H;
use super::super::layout::LayoutSpec;

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    TopBottomPanel::bottom("workbench.status_bar")
        .exact_height(STATUS_BAR_H)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if status_action(ui, &check_summary(app)) {
                    Command::RunChecks.execute(app);
                }

                if !layout.width_class.is_phone() {
                    separator(ui);
                    if let Some((x, y)) = app.state.shell.canvas_hover {
                        segment(ui, &format!("x {x:.2} · y {y:.2}"));
                    }
                    if let Some(selection) = selection_summary(app) {
                        separator(ui);
                        segment(ui, &selection);
                    }
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    segment(
                        ui,
                        &format!("{}%", (app.state.schematic.zoom * 100.0).round()),
                    );
                    separator(ui);
                    let engine = if app.state.simulation.is_running {
                        format!(
                            "Engine running · {}%",
                            app.state.simulation.progress.round()
                        )
                    } else {
                        "Engine ready".to_owned()
                    };
                    status(
                        ui,
                        &engine,
                        if app.state.simulation.is_running {
                            t.color.accent
                        } else {
                            t.color.ok
                        },
                    );
                    if !layout.width_class.is_phone() {
                        separator(ui);
                        segment(ui, platform_label());
                        separator(ui);
                        segment(ui, app.state.workbench.workspace.owner_label());
                    }
                });
            });
        });
}

fn check_summary(app: &RSpiceApp) -> String {
    match &app.state.dialogs.drc_results {
        None => "Checks not run".to_owned(),
        Some(_)
            if app.state.dialogs.drc_checked_version != app.state.schematic.topology_version() =>
        {
            "Checks stale · run again".to_owned()
        }
        Some(result) => {
            let summary = result.summary();
            format!(
                "Schematic checks · {} error{} · {} advisor{}",
                summary.critical + summary.errors,
                if summary.critical + summary.errors == 1 {
                    ""
                } else {
                    "s"
                },
                summary.warnings,
                if summary.warnings == 1 { "y" } else { "ies" }
            )
        }
    }
}

fn status_action(ui: &mut egui::Ui, text: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let color = if text.contains("error") && !text.contains("0 error") {
        t.color.err
    } else if text.contains("stale") || text.contains("not run") {
        t.color.warn
    } else {
        t.color.ok
    };
    ui.add(
        egui::Button::new(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
        )
        .frame(false),
    )
    .clicked()
}

fn selection_summary(app: &RSpiceApp) -> Option<String> {
    let selection = &app.state.schematic.selection;
    if let Some(id) = selection.single_component() {
        app.state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .map(|component| format!("{} · {}", component.name, component.value))
    } else if selection.count() > 0 {
        Some(format!("{} items selected", selection.count()))
    } else {
        None
    }
}

fn segment(ui: &mut egui::Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn status(ui: &mut egui::Ui, text: &str, color: egui::Color32) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        let (dot, _) = ui.allocate_exact_size(egui::Vec2::splat(8.0), egui::Sense::hover());
        ui.painter().circle_filled(dot.center(), 3.0, color);
        ui.label(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
    });
}

fn separator(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(egui::vec2(7.0, STATUS_BAR_H), egui::Sense::hover());
    ui.painter().vline(
        rect.center().x,
        egui::Rangef::new(rect.top() + 6.0, rect.bottom() - 6.0),
        egui::Stroke::new(1.0, t.color.border),
    );
}

#[cfg(target_arch = "wasm32")]
const fn platform_label() -> &'static str {
    "Browser · WebGPU"
}

#[cfg(not(target_arch = "wasm32"))]
const fn platform_label() -> &'static str {
    "Desktop · local"
}
