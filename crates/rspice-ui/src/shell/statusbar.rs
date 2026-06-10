//! Status bar — cursor coordinates, grid/snap, selection, corner, thread
//! count, engine state, and zoom, in an all-mono 25 px strip.

use egui::{Context, Frame, TopBottomPanel, Ui};

use crate::common::AppState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Pill, PillState};

/// Status bar height.
pub const STATUSBAR_HEIGHT: f32 = 25.0;

/// Render the status bar panel.
pub fn show(ctx: &Context, state: &mut AppState) {
    let t = Tokens::get(ctx);
    let c = t.color;

    TopBottomPanel::bottom("volta.statusbar")
        .exact_height(STATUSBAR_HEIGHT)
        .frame(Frame::none().fill(c.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let panel_rect = ui.max_rect();
            ui.painter().hline(
                panel_rect.x_range(),
                panel_rect.top() + 0.5,
                egui::Stroke::new(1.0, c.border),
            );

            ui.horizontal_centered(|ui| {
                ui.add_space(10.0);
                ui.spacing_mut().item_spacing.x = 18.0;

                if state.shell.view == crate::shell::WorkspaceView::Results {
                    segment(ui, &crate::shell::results::status_summary(state));
                    if state.shell.results.cursors.any() {
                        segment(ui, "cursors active · Esc clears");
                    } else {
                        segment(ui, "cursors: click a plot for A · B");
                    }
                } else {
                    segment(ui, &coords_text(state));
                    segment(ui, &format!("Grid {} · Snap on", state.schematic.grid_size));
                    segment(ui, &selection_text(state));
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    ui.spacing_mut().item_spacing.x = 18.0;

                    segment(ui, &format!("{:.0} %", state.schematic.zoom * 100.0));
                    engine_pill(ui, state);
                    segment(ui, &format!("{} threads", thread_count()));
                    segment(ui, &state.shell.corner.clone());
                });
            });
        });
}

fn segment(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn coords_text(state: &AppState) -> String {
    match state.shell.canvas_hover {
        Some((x, y)) => format!("x {x:.2}  y {y:.2}"),
        None => "x −·−−  y −·−−".to_owned(),
    }
}

fn selection_text(state: &AppState) -> String {
    let selection = &state.schematic.selection;
    if let Some(id) = selection.single_component() {
        if let Some(component) = state.schematic.components.iter().find(|c| c.id == id) {
            if component.value.is_empty() {
                return format!("Sel: {}", component.name);
            }
            return format!("Sel: {} ({})", component.name, component.value);
        }
    }
    let count = selection.count();
    if count == 0 {
        "No selection".to_owned()
    } else {
        format!("Sel: {count} items")
    }
}

fn engine_pill(ui: &mut Ui, state: &AppState) {
    if state.simulation.is_running {
        let progress = (state.simulation.progress * 100.0).round() as u32;
        let label = if state.simulation.status.is_empty() {
            format!("running {progress} %")
        } else {
            // Keep the pill compact: status strings can be long.
            let status: String = state.simulation.status.chars().take(28).collect();
            format!("{status} · {progress} %")
        };
        Pill::new(PillState::Running, &label).show(ui);
    } else {
        Pill::new(PillState::Ok, "engine idle").show(ui);
    }
}

fn thread_count() -> usize {
    std::thread::available_parallelism()
        .map(std::num::NonZeroUsize::get)
        .unwrap_or(1)
}
