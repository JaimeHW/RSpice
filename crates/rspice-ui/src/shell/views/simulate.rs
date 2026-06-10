//! Simulate view — a centered card stack: analyses in run order, saved
//! outputs, corner selection, and the run bar with live progress. The
//! add-analysis picker rides on the modal primitive.

use egui::Ui;

use crate::common::AppState;
use crate::common::simulation_analysis_tabs::SIMULATION_ANALYSIS_CATEGORIES;
use crate::shell::panels::simulate::{ANALYSES, analysis_meta};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, chip};

/// Card stack max width (centered in the view).
const CARD_WIDTH: f32 = 880.0;

/// Render the simulate view.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    state.sim_setup.ensure_initialized();
    analysis_picker(ui.ctx(), state);
    egui::ScrollArea::vertical()
        .id_salt("volta.simulate.scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.add_space(14.0);
            let full_width = ui.available_width();
            let card_width = CARD_WIDTH.min(full_width - 36.0);
            let left_pad = ((full_width - card_width) * 0.5).max(18.0);

            ui.horizontal(|ui| {
                ui.add_space(left_pad);
                ui.vertical(|ui| {
                    ui.set_width(card_width);
                    ui.spacing_mut().item_spacing.y = 14.0;

                    analyses_card(ui, state);
                    outputs_card(ui, state);
                    corners_card(ui, state);
                    run_bar(ui, state);
                    ui.add_space(24.0);
                });
            });
        });

    // Keep the panel inspector in sync with a sensible default selection.
    if state.shell.selected_analysis.is_none() {
        state.shell.selected_analysis = Some(1);
    }
}

/// Card frame helper.
fn card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    egui::Frame::none()
        .fill(c.bg_panel)
        .stroke(egui::Stroke::new(1.0, c.border))
        .rounding(t.radius_lg)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            add_contents(ui);
        });
}

/// Card header strip with title, sub-label and optional action.
fn card_header(
    ui: &mut Ui,
    title: &str,
    sub: &str,
    action: Option<&str>,
) -> Option<egui::Response> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 34.0),
        egui::Sense::hover(),
    );
    let painter = ui.painter();
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    let title_galley = ui.fonts(|f| {
        f.layout_no_wrap(
            title.to_owned(),
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            c.text,
        )
    });
    painter.galley(
        egui::pos2(rect.left() + 14.0, rect.center().y - title_galley.size().y * 0.5),
        title_galley.clone(),
        c.text,
    );
    if !sub.is_empty() {
        painter.text(
            egui::pos2(
                rect.left() + 14.0 + title_galley.size().x + 8.0,
                rect.center().y,
            ),
            egui::Align2::LEFT_CENTER,
            sub,
            theme::sans(tokens::FS_1, FontWeight::Regular),
            c.text_faint,
        );
    }

    let action_label = action?;
    let galley = ui.fonts(|f| {
        f.layout_no_wrap(
            action_label.to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            c.text_dim,
        )
    });
    let action_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.right() - 14.0 - galley.size().x - 16.0,
            rect.center().y - 11.0,
        ),
        egui::vec2(galley.size().x + 16.0, 22.0),
    );
    let response = ui.interact(
        action_rect,
        ui.id().with(("card-action", title)),
        egui::Sense::click(),
    );
    if response.hovered() {
        ui.painter().rect_filled(action_rect, t.radius, c.bg_hover);
    }
    ui.painter().galley(
        egui::pos2(action_rect.left() + 8.0, action_rect.center().y - galley.size().y * 0.5),
        galley,
        if response.hovered() { c.text } else { c.text_dim },
    );
    Some(response.on_hover_cursor(egui::CursorIcon::PointingHand))
}

fn analyses_card(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    card(ui, |ui| {
        if let Some(action) = card_header(ui, "Analyses", "run in listed order", Some("+ Add analysis")) {
            if action.clicked() {
                state.sim_setup.picker_open = true;
            }
        }

        for (tab_idx, name, description) in ANALYSES {
            let enabled = state.sim_setup.enabled.contains(tab_idx);
            let selected = state.shell.selected_analysis == Some(*tab_idx);
            // Only show disabled exotic analyses when enabled — the core five
            // stay visible like the design.
            let core = matches!(*tab_idx, 0..=4 | 7);
            if !core && !enabled {
                continue;
            }

            let (rect, response) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), 33.0),
                egui::Sense::click(),
            );
            let painter = ui.painter();
            if selected {
                painter.rect_filled(rect, 0.0, c.accent_dim);
            } else if response.hovered() {
                painter.rect_filled(rect, 0.0, c.bg_hover);
            }
            painter.hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );

            // Checkbox.
            let box_rect = egui::Rect::from_center_size(
                egui::pos2(rect.left() + 22.0, rect.center().y),
                egui::vec2(14.0, 14.0),
            );
            let box_response = ui.interact(
                box_rect.expand(4.0),
                ui.id().with(("an-enable", tab_idx)),
                egui::Sense::click(),
            );
            if enabled {
                painter.rect_filled(box_rect, t.radius.min(2.0), c.accent);
                let s = box_rect.width();
                painter.add(egui::Shape::line(
                    vec![
                        box_rect.left_top() + egui::vec2(0.25 * s, 0.55 * s),
                        box_rect.left_top() + egui::vec2(0.42 * s, 0.72 * s),
                        box_rect.left_top() + egui::vec2(0.78 * s, 0.30 * s),
                    ],
                    egui::Stroke::new(1.6, c.accent_ink),
                ));
            } else {
                painter.rect(
                    box_rect,
                    t.radius.min(2.0),
                    c.bg_inset,
                    egui::Stroke::new(1.0, c.border_strong),
                );
            }
            if box_response.clicked() {
                if enabled {
                    state.sim_setup.enabled.remove(tab_idx);
                } else {
                    state.sim_setup.enabled.insert(*tab_idx);
                }
            }

            // Name + description + live summary + state.
            painter.text(
                egui::pos2(rect.left() + 44.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                name,
                theme::mono(tokens::FS_1, FontWeight::Medium),
                c.text,
            );
            painter.text(
                egui::pos2(rect.left() + 110.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                description,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            );
            let invalid = enabled && state.sim_setup.validation_error(*tab_idx).is_some();
            if rect.width() > 560.0 {
                painter.text(
                    egui::pos2(rect.right() - 88.0, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    state.sim_setup.summary(*tab_idx),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    if invalid { c.err } else { c.text_faint },
                );
            }
            let (state_label, state_color) = if invalid {
                ("invalid", c.err)
            } else if selected {
                ("selected", c.text_faint)
            } else if enabled {
                ("enabled", c.text_faint)
            } else {
                ("off", c.text_faint)
            };
            painter.text(
                egui::pos2(rect.right() - 14.0, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                state_label,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                state_color,
            );

            if response.clicked() {
                state.shell.selected_analysis = Some(*tab_idx);
            }
        }
    });
}

fn outputs_card(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    card(ui, |ui| {
        card_header(ui, "Outputs", "saved to results database", None);
        if state.simulation.waveforms.is_empty() {
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), 40.0),
                egui::Sense::hover(),
            );
            ui.painter().text(
                egui::pos2(rect.left() + 14.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                "Outputs appear here after the first run — probe nets to add more",
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_faint,
            );
            return;
        }

        let names: Vec<(String, bool)> = state
            .simulation
            .waveforms
            .iter()
            .map(|waveform| (waveform.name.clone(), waveform.visible))
            .collect();
        for (idx, (name, visible)) in names.iter().enumerate() {
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), 30.0),
                egui::Sense::hover(),
            );
            let painter = ui.painter();
            if idx + 1 < names.len() {
                painter.hline(
                    rect.x_range(),
                    rect.bottom() - 0.5,
                    egui::Stroke::new(1.0, c.border),
                );
            }
            painter.text(
                egui::pos2(rect.left() + 14.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                name,
                theme::mono(tokens::FS_1, FontWeight::Regular),
                c.text,
            );

            // plot / save flags.
            let plot_rect = egui::Rect::from_min_size(
                egui::pos2(rect.right() - 14.0 - 44.0 - 6.0 - 44.0, rect.top() + 4.0),
                egui::vec2(44.0, 22.0),
            );
            let plot_response = ui.interact(
                plot_rect,
                ui.id().with(("out-plot", idx)),
                egui::Sense::click(),
            );
            flag(ui, plot_rect, "plot", *visible);
            if plot_response.clicked() {
                state.simulation.toggle_waveform_visibility(name);
            }
            let save_rect = egui::Rect::from_min_size(
                egui::pos2(rect.right() - 14.0 - 44.0, rect.top() + 4.0),
                egui::vec2(44.0, 22.0),
            );
            flag(ui, save_rect, "save", true);
        }
    });
}

/// A small bordered flag ("plot"/"save"), accent when on.
fn flag(ui: &Ui, rect: egui::Rect, label: &str, on: bool) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let painter = ui.painter();
    let (fill, border, text) = if on {
        (c.accent_dim, c.accent, c.accent)
    } else {
        (egui::Color32::TRANSPARENT, c.border, c.text_faint)
    };
    painter.rect(rect, t.radius, fill, egui::Stroke::new(1.0, border));
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        text,
    );
}

fn corners_card(ui: &mut Ui, state: &mut AppState) {
    card(ui, |ui| {
        card_header(ui, "Corners", "applied to model selection", None);
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.add_space(14.0);
            ui.spacing_mut().item_spacing.x = 5.0;
            for corner in crate::state::model_library::ProcessCorner::standard_corners() {
                let on = state.shell.corner == corner.name;
                if chip(ui, &corner.name, on).clicked() {
                    state.shell.corner = corner.name.clone();
                }
            }
        });
        ui.add_space(12.0);
    });
}

/// The add-analysis picker: searchable, grouped by category, on the modal
/// primitive. Clicking a row toggles it in the run set and selects it so
/// the right panel shows its form.
fn analysis_picker(ctx: &egui::Context, state: &mut AppState) {
    if !state.sim_setup.picker_open {
        return;
    }
    let t = Tokens::get(ctx);
    let c = t.color;

    let shell = &mut state.shell;
    let setup = &mut state.sim_setup;
    let hint = format!("{} enabled", setup.enabled.len());

    let choice = Dialog::new("Simulate", "Add analysis", "Done")
        .size(DialogSize::Md)
        .hint(&hint)
        .show(ctx, |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut setup.picker_query)
                    .hint_text("Filter analyses…")
                    .desired_width(f32::INFINITY),
            );
            ui.add_space(6.0);

            let query = setup.picker_query.trim().to_lowercase();
            for (category, items) in SIMULATION_ANALYSIS_CATEGORIES {
                let visible: Vec<(usize, &str)> = items
                    .iter()
                    .filter(|(index, label)| {
                        query.is_empty()
                            || label.to_lowercase().contains(&query)
                            || analysis_meta(*index).0.contains(query.as_str())
                    })
                    .copied()
                    .collect();
                if visible.is_empty() {
                    continue;
                }

                ui.add_space(8.0);
                let mut header = egui::text::LayoutJob::default();
                header.append(
                    &category.to_uppercase(),
                    0.0,
                    egui::TextFormat {
                        font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                        color: c.text_faint,
                        extra_letter_spacing: 0.08 * tokens::FS_0,
                        ..Default::default()
                    },
                );
                ui.label(header);
                ui.add_space(2.0);

                for (index, label) in visible {
                    let enabled = setup.enabled.contains(&index);
                    let (rect, response) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width(), 28.0),
                        egui::Sense::click(),
                    );
                    let painter = ui.painter();
                    if response.hovered() {
                        painter.rect_filled(rect, t.radius, c.bg_hover);
                    }
                    let (id, _) = analysis_meta(index);
                    painter.text(
                        egui::pos2(rect.left() + 8.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        id,
                        theme::mono(tokens::FS_1, FontWeight::Medium),
                        if enabled { c.accent } else { c.text_dim },
                    );
                    painter.text(
                        egui::pos2(rect.left() + 76.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        label,
                        theme::sans(tokens::FS_1, FontWeight::Regular),
                        c.text,
                    );
                    if enabled {
                        painter.text(
                            egui::pos2(rect.right() - 8.0, rect.center().y),
                            egui::Align2::RIGHT_CENTER,
                            "added",
                            theme::mono(tokens::FS_0, FontWeight::Regular),
                            c.accent,
                        );
                    }
                    if response.clicked() {
                        if enabled {
                            setup.enabled.remove(&index);
                        } else {
                            setup.enabled.insert(index);
                            shell.selected_analysis = Some(index);
                        }
                    }
                    if response.hovered() {
                        response.on_hover_cursor(egui::CursorIcon::PointingHand);
                    }
                }
            }
        });

    if choice != DialogChoice::None {
        setup.picker_open = false;
        setup.picker_query.clear();
    }
}

fn run_bar(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    egui::Frame::none()
        .fill(c.bg_panel)
        .stroke(egui::Stroke::new(1.0, c.border))
        .rounding(t.radius_lg)
        .inner_margin(egui::Margin::symmetric(14.0, 12.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;

                let can_run =
                    !state.schematic.components.is_empty() && !state.simulation.is_running;
                if Button::new("Run")
                    .icon(Icon::Run)
                    .accent()
                    .enabled(can_run)
                    .show(ui)
                    .clicked()
                {
                    state.simulation.trigger_simulation = true;
                }
                if Button::new("Stop")
                    .icon(Icon::Stop)
                    .enabled(state.simulation.is_running)
                    .show(ui)
                    .clicked()
                {
                    state.simulation.trigger_abort = true;
                }

                // Progress track.
                let status_width = 200.0;
                let track_width =
                    (ui.available_width() - status_width - 12.0).max(60.0);
                let (track_rect, _) = ui.allocate_exact_size(
                    egui::vec2(track_width, 5.0),
                    egui::Sense::hover(),
                );
                let painter = ui.painter();
                painter.rect_filled(track_rect, 3.0, c.bg_inset);
                let progress = state.simulation.progress.clamp(0.0, 1.0) as f32;
                if progress > 0.0 {
                    let fill_rect = egui::Rect::from_min_size(
                        track_rect.min,
                        egui::vec2(track_rect.width() * progress, track_rect.height()),
                    );
                    painter.rect_filled(fill_rect, 3.0, c.accent);
                }

                // Status readout.
                let status = if state.simulation.is_running {
                    state.simulation.status.clone()
                } else if let Some(run) = state.simulation.active_run() {
                    format!(
                        "idle — last run #{} {}",
                        run.id,
                        if run.success { "ok" } else { "failed" }
                    )
                } else {
                    "idle".to_owned()
                };
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(status)
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(c.text_dim),
                    );
                });
            });
        });
}
