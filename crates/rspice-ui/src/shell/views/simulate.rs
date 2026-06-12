//! Simulate view — a centered card stack: analyses in run order, saved
//! outputs, corner selection, and the run bar with live progress. The
//! add-analysis picker rides on the modal primitive.

use egui::Ui;

use crate::common::AppState;
use crate::common::simulation_analysis_tabs::{SIMULATION_ANALYSIS_CATEGORIES, TAB_TRANSIENT};
use crate::shell::panels::simulate::{ANALYSES, analysis_meta};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, PaneSide, chip, two_pane};

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
        if let Some(action) =
            card_header(ui, "Analyses", "run in listed order", Some("+ Add analysis"))
            && action.clicked()
        {
            state.sim_setup.picker_open = true;
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

// ---------------------------------------------------------------------------
// Add-analysis picker (design/volta-dialogs-v2.html §4)
// ---------------------------------------------------------------------------

/// Picker pane height — rail and detail share it.
const PICKER_HEIGHT: f32 = 392.0;
/// Analysis rail width inside the Lg dialog.
const PICKER_RAIL_WIDTH: f32 = 224.0;

/// The add-analysis picker: a categorized analysis rail on the left, the
/// selected analysis' parameter form on the right over a live mono preview
/// of the card it writes. Add commits and closes; "Add & configure
/// another" commits and stays open.
fn analysis_picker(ctx: &egui::Context, state: &mut AppState) {
    if !state.sim_setup.picker_open {
        return;
    }

    let shell = &mut state.shell;
    let setup = &mut state.sim_setup;

    let already_enabled = setup
        .enabled
        .contains(&setup.picker_selected.unwrap_or(TAB_TRANSIENT));
    let hint = format!("{} in run set · runs in list order", setup.enabled.len());
    let primary_label = if already_enabled { "Done" } else { "Add analysis" };

    let mut add_and_close = false;
    let choice = Dialog::new("Simulate", "Add analysis", primary_label)
        .size(DialogSize::Lg)
        .secondary("Add & configure another")
        .ghost("Cancel")
        .hint(&hint)
        .show(ctx, |ui| picker_body(ui, setup, &mut add_and_close));

    let selected = setup.picker_selected.unwrap_or(TAB_TRANSIENT);
    let mut close = || {
        setup.picker_open = false;
        setup.picker_query.clear();
    };
    match choice {
        DialogChoice::None => {
            if add_and_close {
                setup.enabled.insert(selected);
                shell.selected_analysis = Some(selected);
                close();
            }
        }
        DialogChoice::Primary => {
            setup.enabled.insert(selected);
            shell.selected_analysis = Some(selected);
            close();
        }
        DialogChoice::Secondary => {
            if setup.enabled.insert(selected) {
                let (id, _) = analysis_meta(selected);
                shell.toasts.info(ctx, format!("{id} added to the run set"));
            }
            shell.selected_analysis = Some(selected);
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => close(),
    }
}

/// Dialog body: filterable rail + parameter form + writes strip.
fn picker_body(ui: &mut Ui, setup: &mut crate::common::app::SimSetupState, add_and_close: &mut bool) {
    let c = Tokens::get(ui.ctx()).color;

    two_pane(ui, PICKER_RAIL_WIDTH, PICKER_HEIGHT, |ui, side| match side {
        // ── Left rail: filter over the categorized analysis list.
        PaneSide::Rail => {
            picker_filter_strip(ui, &mut setup.picker_query);

            let query = setup.picker_query.trim().to_lowercase();
            egui::ScrollArea::vertical()
                .id_salt("volta.simulate.picker.rail")
                .max_height(PICKER_HEIGHT - 34.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (category, items) in SIMULATION_ANALYSIS_CATEGORIES {
                        let visible: Vec<(usize, &str)> = items
                            .iter()
                            .filter(|(index, label)| {
                                let (id, description) = analysis_meta(*index);
                                query.is_empty()
                                    || label.to_lowercase().contains(&query)
                                    || id.contains(query.as_str())
                                    || description.to_lowercase().contains(&query)
                            })
                            .copied()
                            .collect();
                        if visible.is_empty() {
                            continue;
                        }
                        picker_section_label(ui, category);
                        for (index, label) in visible {
                            let response = picker_rail_row(ui, setup, index, label);
                            if response.clicked() {
                                setup.picker_selected = Some(index);
                            }
                            if response.double_clicked() {
                                setup.picker_selected = Some(index);
                                *add_and_close = true;
                            }
                        }
                    }
                    ui.add_space(6.0);
                });
        }

        // ── Detail: selected analysis header, parameter form, writes strip.
        PaneSide::Detail => {
            let selected = setup.picker_selected.unwrap_or(TAB_TRANSIENT);
            picker_detail_header(ui, setup, selected);

            egui::ScrollArea::vertical()
                .id_salt("volta.simulate.picker.form")
                .max_height(PICKER_HEIGHT - 34.0 - 46.0)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    egui::Frame::none()
                        .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 2.0;
                            let note =
                                crate::shell::panels::simulate_forms::form(ui, setup, selected);
                            if !note.is_empty() {
                                ui.add_space(6.0);
                                ui.label(
                                    egui::RichText::new(note)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(c.text_faint),
                                );
                            }
                        });
                });

            picker_writes_strip(ui, setup, selected);
        }
    });
}

/// 34 px rail header hosting the frameless filter input.
fn picker_filter_strip(ui: &mut Ui, query: &mut String) {
    let c = Tokens::get(ui.ctx()).color;
    let response = ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 34.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(10.0);
            ui.add_sized(
                egui::vec2(ui.available_width() - 10.0, 22.0),
                egui::TextEdit::singleline(query)
                    .frame(false)
                    .hint_text("Filter analyses…")
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular)),
            );
        },
    );
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
}

/// Small-caps category label inside the rail.
fn picker_section_label(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 24.0), egui::Sense::hover());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        &text.to_uppercase(),
        0.0,
        egui::TextFormat {
            font_id: theme::mono(9.5, FontWeight::Medium),
            color: t.color.text_faint,
            extra_letter_spacing: 0.1 * 9.5,
            ..Default::default()
        },
    );
    let galley = ui.fonts(|f| f.layout_job(job));
    ui.painter().galley(
        egui::pos2(rect.left() + 10.0, rect.bottom() - galley.size().y - 3.0),
        galley,
        t.color.text_faint,
    );
}

/// One analysis row: run-set dot, mono id, human label. Click selects,
/// double-click adds and closes.
fn picker_rail_row(
    ui: &mut Ui,
    setup: &crate::common::app::SimSetupState,
    index: usize,
    label: &str,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 26.0), egui::Sense::click());
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let enabled = setup.enabled.contains(&index);
    let selected = setup.picker_selected.unwrap_or(TAB_TRANSIENT) == index;
    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered() && !selected, 0.12);
    let painter = ui.painter();
    if selected {
        painter.rect_filled(rect, 0.0, c.accent_dim);
    } else if hover > 0.0 {
        painter.rect_filled(
            rect,
            0.0,
            theme::mix(egui::Color32::TRANSPARENT, c.bg_hover, hover),
        );
    }

    // Run-set dot: filled when enabled, hollow otherwise.
    let dot_center = egui::pos2(rect.left() + 16.0, rect.center().y);
    if enabled {
        painter.circle_filled(dot_center, 3.0, c.ok);
    } else {
        painter.circle_stroke(dot_center, 3.0, egui::Stroke::new(1.0, c.border_strong));
    }

    let (id, _) = analysis_meta(index);
    painter.text(
        egui::pos2(rect.left() + 28.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        id,
        theme::mono(tokens::FS_1, FontWeight::Medium),
        if selected {
            c.text
        } else {
            theme::mix(c.text_dim, c.text, hover)
        },
    );
    painter.text(
        egui::pos2(rect.right() - 10.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// 34 px detail header: mono id, description, and the run-set badge
/// (click to remove).
fn picker_detail_header(
    ui: &mut Ui,
    setup: &mut crate::common::app::SimSetupState,
    selected: usize,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (id, description) = analysis_meta(selected);
    let enabled = setup.enabled.contains(&selected);

    let response = ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 34.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(id)
                    .font(theme::mono(tokens::FS_2, FontWeight::Medium))
                    .color(c.text),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(description)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_dim),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                if enabled {
                    let galley = ui.fonts(|f| {
                        f.layout_no_wrap(
                            "in run set".to_owned(),
                            theme::mono(tokens::FS_0, FontWeight::Regular),
                            c.ok,
                        )
                    });
                    let pad = egui::vec2(7.0, 3.0);
                    let (badge_rect, badge_response) = ui.allocate_exact_size(
                        galley.size() + 2.0 * pad,
                        egui::Sense::click(),
                    );
                    let painter = ui.painter();
                    painter.rect(
                        badge_rect,
                        badge_rect.height() * 0.5,
                        c.ok.gamma_multiply(if badge_response.hovered() { 0.26 } else { 0.14 }),
                        egui::Stroke::new(1.0, c.ok.gamma_multiply(0.55)),
                    );
                    painter.galley(badge_rect.min + pad, galley, c.ok);
                    if badge_response
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .on_hover_text("Remove from the run set")
                        .clicked()
                    {
                        setup.enabled.remove(&selected);
                    }
                }
            });
        },
    );
    ui.painter().hline(
        response.response.rect.x_range(),
        response.response.rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
}

/// Bottom strip: the exact card this analysis writes into the deck — or
/// the first validation error, in the error treatment.
fn picker_writes_strip(
    ui: &mut Ui,
    setup: &crate::common::app::SimSetupState,
    selected: usize,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 46.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.add_space(7.0);
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    let mut caption = egui::text::LayoutJob::default();
                    caption.append(
                        "WRITES",
                        0.0,
                        egui::TextFormat {
                            font_id: theme::mono(9.5, FontWeight::Medium),
                            color: c.text_faint,
                            extra_letter_spacing: 0.1 * 9.5,
                            ..Default::default()
                        },
                    );
                    ui.label(caption);
                });
                ui.add_space(2.0);
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    match setup.spice_preview(selected) {
                        Ok(card) => {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(card)
                                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                        .color(c.text_dim),
                                )
                                .truncate(),
                            );
                        }
                        Err(error) => {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(error)
                                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                        .color(c.err),
                                )
                                .truncate(),
                            );
                        }
                    }
                });
            },
        );
        ui.painter().hline(
            ui.min_rect().x_range(),
            ui.min_rect().top() + 0.5,
            egui::Stroke::new(1.0, c.border),
        );
    });
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
