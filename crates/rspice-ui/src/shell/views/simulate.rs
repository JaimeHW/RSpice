//! Simulate view — a centered card stack: analyses in run order, saved
//! outputs, corner selection, and the run bar with live progress. The
//! run-set card is the single source of truth: tick to enable, click to
//! configure in the right inspector; adding rides the anchored analysis
//! palette, never a modal.

use egui::Ui;

use crate::common::AppState;
use crate::common::simulation_analysis_tabs::{SIMULATION_ANALYSIS_CATEGORIES, TAB_TRANSIENT};
use crate::shell::panels::simulate::{ANALYSES, analysis_meta};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight, mix};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, chip};

/// Card stack max width (centered in the view).
const CARD_WIDTH: f32 = 880.0;

/// The core analyses every project lists permanently; exotics join the
/// card through the palette and leave through the row's remove control.
fn is_core(index: usize) -> bool {
    matches!(index, 0..=4 | 7)
}

/// Render the simulate view.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    state.sim_setup.ensure_initialized();

    // `a` opens the palette from anywhere in the view (matches the spec's
    // keyboard-first add path); inert while anything has keyboard focus.
    let mut palette_opened = false;
    if !state.sim_setup.palette_open
        && ui.ctx().memory(|m| m.focused().is_none())
        && ui
            .ctx()
            .input(|i| i.key_pressed(egui::Key::A) && i.modifiers.is_none())
    {
        open_palette(&mut state.sim_setup);
        palette_opened = true;
    }

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

                    analyses_card(ui, state, palette_opened);
                    outputs_card(ui, state);
                    corners_card(ui, state);
                    run_bar(ui, state);
                    ui.add_space(24.0);
                });
            });
        });

    // Keep the panel inspector in sync with a sensible default selection.
    if state.shell.selected_analysis.is_none() {
        state.shell.selected_analysis = Some(TAB_TRANSIENT);
    }
}

/// Reset the palette to a fresh open state.
fn open_palette(setup: &mut crate::common::app::SimSetupState) {
    setup.palette_open = true;
    setup.palette_query.clear();
    setup.palette_active = 0;
}

/// Card frame helper.
fn card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    egui::Frame::NONE
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
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 34.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    let title_galley = ui.fonts_mut(|f| {
        f.layout_no_wrap(
            title.to_owned(),
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            c.text,
        )
    });
    painter.galley(
        egui::pos2(
            rect.left() + 14.0,
            rect.center().y - title_galley.size().y * 0.5,
        ),
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
    let galley = ui.fonts_mut(|f| {
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
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("{action_label}, {title}"),
        )
    });
    if response.hovered() {
        ui.painter().rect_filled(action_rect, t.radius, c.bg_hover);
    }
    ui.painter().galley(
        egui::pos2(
            action_rect.left() + 8.0,
            action_rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        if response.hovered() {
            c.text
        } else {
            c.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, action_rect);
    Some(response.on_hover_cursor(egui::CursorIcon::PointingHand))
}

fn analyses_card(ui: &mut Ui, state: &mut AppState, mut palette_opened: bool) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut remove: Option<usize> = None;
    let mut palette_anchor: Option<egui::Rect> = None;

    card(ui, |ui| {
        if let Some(action) = card_header(
            ui,
            "Analyses",
            "run in listed order",
            Some("+ Add analysis"),
        ) {
            if action.clicked() {
                if state.sim_setup.palette_open {
                    state.sim_setup.palette_open = false;
                } else {
                    open_palette(&mut state.sim_setup);
                    palette_opened = true;
                }
            }
            palette_anchor = Some(action.rect);
        }

        for (tab_idx, name, description) in ANALYSES {
            let enabled = state.sim_setup.enabled.contains(tab_idx);
            let listed = state.sim_setup.listed.contains(tab_idx);
            let selected = state.shell.selected_analysis == Some(*tab_idx);
            // The core analyses stay visible; exotics appear once added via
            // the palette and stay listed (dimmed) while unticked.
            let core = is_core(*tab_idx);
            if !core && !enabled && !listed {
                continue;
            }

            let (rect, response) = ui
                .allocate_exact_size(egui::vec2(ui.available_width(), 33.0), egui::Sense::click());
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    format!(
                        "Configure {name}: {description}. {}",
                        state.sim_setup.summary(*tab_idx)
                    ),
                )
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::ListBoxOption);
                node.set_selected(selected);
            });
            let hovered = ui.rect_contains_pointer(rect);
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
            box_response.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::Checkbox,
                    ui.is_enabled(),
                    enabled,
                    format!("Include {name} in run set"),
                )
            });
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
                    egui::StrokeKind::Inside,
                );
            }
            if box_response.clicked() {
                if enabled {
                    state.sim_setup.enabled.remove(tab_idx);
                } else {
                    state.sim_setup.enabled.insert(*tab_idx);
                }
            }
            theme::paint_focus_ring(ui, &box_response, box_rect.expand(4.0));

            // Name + description (dimmed while unticked) + live summary.
            let alpha = if enabled { 1.0 } else { 0.55 };
            painter.text(
                egui::pos2(rect.left() + 44.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                name,
                theme::mono(tokens::FS_1, FontWeight::Medium),
                c.text.gamma_multiply(alpha),
            );
            painter.text(
                egui::pos2(rect.left() + 110.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                description,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim.gamma_multiply(alpha),
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

            // Far right: the state label — swapped for the remove control on
            // hovered non-core rows ("invalid" never hides).
            if hovered && !core && !invalid {
                let x_rect = egui::Rect::from_center_size(
                    egui::pos2(rect.right() - 22.0, rect.center().y),
                    egui::vec2(18.0, 18.0),
                );
                let x_response = ui.interact(
                    x_rect,
                    ui.id().with(("an-remove", tab_idx)),
                    egui::Sense::click(),
                );
                x_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        format!("Remove {name} from analyses list"),
                    )
                });
                if x_response.hovered() {
                    ui.painter().rect_filled(x_rect, t.radius, c.bg_hover);
                }
                let x_color = if x_response.hovered() {
                    c.err
                } else {
                    c.text_faint
                };
                ui.painter().text(
                    x_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "×",
                    theme::mono(tokens::FS_2, FontWeight::Regular),
                    x_color,
                );
                theme::paint_focus_ring(ui, &x_response, x_rect);
                if x_response
                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                    .on_hover_text("Remove from the list")
                    .clicked()
                {
                    remove = Some(*tab_idx);
                }
            } else {
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
            }

            if response.clicked() {
                state.shell.selected_analysis = Some(*tab_idx);
            }
            theme::paint_focus_ring(ui, &response, rect);
        }
    });

    if let Some(index) = remove {
        state.sim_setup.listed.remove(&index);
        state.sim_setup.enabled.remove(&index);
        if state.shell.selected_analysis == Some(index) {
            state.shell.selected_analysis = Some(TAB_TRANSIENT);
        }
    }

    if let Some(anchor) = palette_anchor {
        analysis_palette(ui, state, anchor, palette_opened);
    }
}

fn outputs_card(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    card(ui, |ui| {
        card_header(ui, "Outputs", "saved to results database", None);
        if state.simulation.waveforms.is_empty() {
            let (rect, _) = ui
                .allocate_exact_size(egui::vec2(ui.available_width(), 40.0), egui::Sense::hover());
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
            let (rect, _) = ui
                .allocate_exact_size(egui::vec2(ui.available_width(), 30.0), egui::Sense::hover());
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
            plot_response.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::Checkbox,
                    ui.is_enabled(),
                    *visible,
                    format!("Plot output {name}"),
                )
            });
            flag(ui, plot_rect, "plot", *visible);
            if plot_response.clicked() {
                state.simulation.toggle_waveform_visibility(name);
            }
            theme::paint_focus_ring(ui, &plot_response, plot_rect);
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
    painter.rect(
        rect,
        t.radius,
        fill,
        egui::Stroke::new(1.0, border),
        egui::StrokeKind::Inside,
    );
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
// Analysis palette
// ---------------------------------------------------------------------------
//
// Anchored under the card's "+ Add analysis" action; add-only — it puts a
// type in the run set and selects it so the right inspector is already on
// the new analysis when it closes. Enter adds and closes, Shift+Enter adds
// and keeps the palette open, Esc closes, `a` opens it from the view.

/// Palette surface width.
const PALETTE_WIDTH: f32 = 322.0;
/// Maximum height of the scrolling type list.
const PALETTE_LIST_MAX_H: f32 = 320.0;
/// Palette row height.
const PALETTE_ROW_H: f32 = 27.0;

/// Stable id for the palette filter field.
fn palette_input_id() -> egui::Id {
    egui::Id::new("volta.simulate.palette.input")
}

/// The categorized analysis list matching the filter, flattened in rail
/// order: `(analysis index, category)`.
fn palette_items(query: &str) -> Vec<(usize, &'static str)> {
    let query = query.trim().to_lowercase();
    let mut items = Vec::new();
    for (category, entries) in SIMULATION_ANALYSIS_CATEGORIES {
        for (index, label) in *entries {
            let (id, description) = analysis_meta(*index);
            if query.is_empty()
                || label.to_lowercase().contains(&query)
                || id.contains(query.as_str())
                || description.to_lowercase().contains(&query)
                || category.to_lowercase().contains(&query)
            {
                items.push((*index, *category));
            }
        }
    }
    items
}

/// Put `index` in the run set and focus it; the palette closes unless the
/// add is chained (Shift), in which case the filter field re-arms.
fn palette_commit(ctx: &egui::Context, state: &mut AppState, index: usize, chain: bool) {
    state.sim_setup.listed.insert(index);
    state.sim_setup.enabled.insert(index);
    state.shell.selected_analysis = Some(index);
    if chain {
        state.sim_setup.palette_query.clear();
        state.sim_setup.palette_active = 0;
        // Enter surrenders the TextEdit's focus; take it back for the
        // next filter keystrokes.
        ctx.memory_mut(|m| m.request_focus(palette_input_id()));
    } else {
        state.sim_setup.palette_open = false;
    }
}

fn analysis_palette(ui: &Ui, state: &mut AppState, anchor: egui::Rect, just_opened: bool) {
    if !state.sim_setup.palette_open {
        return;
    }
    let ctx = ui.ctx().clone();
    let t = Tokens::get(&ctx);
    let c = t.color;

    // Keys first, against the pre-frame filtered list, so Enter lands on
    // the row the user sees highlighted.
    let items = palette_items(&state.sim_setup.palette_query);
    let (up, down, enter, shift, escape) = ctx.input(|i| {
        (
            i.key_pressed(egui::Key::ArrowUp),
            i.key_pressed(egui::Key::ArrowDown),
            i.key_pressed(egui::Key::Enter),
            i.modifiers.shift,
            i.key_pressed(egui::Key::Escape),
        )
    });
    if escape {
        state.sim_setup.palette_open = false;
        return;
    }
    let mut keyed = false;
    if down && !items.is_empty() {
        state.sim_setup.palette_active = (state.sim_setup.palette_active + 1).min(items.len() - 1);
        keyed = true;
    } else if up {
        state.sim_setup.palette_active = state.sim_setup.palette_active.saturating_sub(1);
        keyed = true;
    }
    if state.sim_setup.palette_active >= items.len() {
        state.sim_setup.palette_active = items.len().saturating_sub(1);
    }
    if enter && let Some((index, _)) = items.get(state.sim_setup.palette_active).copied() {
        palette_commit(&ctx, state, index, shift);
        if !state.sim_setup.palette_open {
            return;
        }
    }

    let response = egui::Area::new(egui::Id::new("volta.simulate.palette"))
        .order(egui::Order::Foreground)
        .fixed_pos(egui::pos2(anchor.right(), anchor.bottom() + 4.0))
        .pivot(egui::Align2::RIGHT_TOP)
        .show(&ctx, |ui| {
            egui::Frame::NONE
                .fill(c.bg_elevated)
                .stroke(egui::Stroke::new(1.0, c.border_strong))
                .rounding(t.radius_lg)
                .shadow(t.shadow())
                .show(ui, |ui| {
                    ui.set_width(PALETTE_WIDTH);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    palette_search_strip(ui, state, just_opened);
                    palette_list(ui, state, &items, keyed);
                    palette_hint_strip(ui);
                });
        })
        .response;

    // Click-away closes — but not on the click that opened it.
    if !just_opened && response.clicked_elsewhere() && !anchor.contains(pointer_press(&ctx)) {
        state.sim_setup.palette_open = false;
    }
}

/// Where the latest pointer press started (so the anchor button's own
/// click never counts as click-away).
fn pointer_press(ctx: &egui::Context) -> egui::Pos2 {
    ctx.input(|i| i.pointer.press_origin().or(i.pointer.interact_pos()))
        .unwrap_or(egui::pos2(f32::MIN, f32::MIN))
}

/// The filter field strip at the palette top.
fn palette_search_strip(ui: &mut Ui, state: &mut AppState, just_opened: bool) {
    let c = Tokens::get(ui.ctx()).color;
    egui::Frame::NONE
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            let edit = ui.add(
                egui::TextEdit::singleline(&mut state.sim_setup.palette_query)
                    .id(palette_input_id())
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .hint_text(format!("Filter {} analyses…", ANALYSES.len()))
                    .desired_width(f32::INFINITY),
            );
            if edit.changed() {
                state.sim_setup.palette_active = 0;
            }
            if just_opened {
                edit.request_focus();
            }
        });
    let y = ui.cursor().top();
    ui.painter().hline(
        ui.min_rect().x_range(),
        y - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
}

/// The categorized, scrolling type list.
fn palette_list(ui: &mut Ui, state: &mut AppState, items: &[(usize, &'static str)], keyed: bool) {
    let c = Tokens::get(ui.ctx()).color;

    if items.is_empty() {
        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(10, 12))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "No analysis matches “{}”",
                        state.sim_setup.palette_query.trim()
                    ))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_faint),
                );
            });
        return;
    }

    let mut clicked: Option<(usize, bool)> = None;
    egui::ScrollArea::vertical()
        .id_salt("volta.simulate.palette.list")
        .max_height(PALETTE_LIST_MAX_H)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let mut last_category = "";
            for (position, (index, category)) in items.iter().enumerate() {
                if *category != last_category {
                    last_category = category;
                    palette_section_label(ui, category);
                }
                let active = position == state.sim_setup.palette_active;
                let response = palette_row(ui, state, *index, active);
                if active && keyed {
                    response.scroll_to_me(Some(egui::Align::Center));
                }
                // A resting cursor must not fight the arrow keys: only an
                // actually-moving pointer steals the active row.
                if response.hovered() && !active && ui.input(|i| i.pointer.is_moving()) {
                    state.sim_setup.palette_active = position;
                }
                if response.clicked() {
                    let shift = ui.input(|i| i.modifiers.shift);
                    clicked = Some((*index, shift));
                }
            }
            ui.add_space(4.0);
        });
    if let Some((index, shift)) = clicked {
        let ctx = ui.ctx().clone();
        palette_commit(&ctx, state, index, shift);
    }
}

/// Small-caps category label inside the palette list.
fn palette_section_label(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 24.0), egui::Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
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
    let galley = ui.fonts_mut(|f| f.layout_job(job));
    ui.painter().galley(
        egui::pos2(rect.left() + 10.0, rect.bottom() - galley.size().y - 3.0),
        galley,
        t.color.text_faint,
    );
}

/// One palette row: run-set dot, mono id, description, status meta.
fn palette_row(ui: &mut Ui, state: &AppState, index: usize, active: bool) -> egui::Response {
    let c = Tokens::get(ui.ctx()).color;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PALETTE_ROW_H),
        egui::Sense::click(),
    );
    if !ui.is_rect_visible(rect) {
        return response;
    }

    let enabled = state.sim_setup.enabled.contains(&index);
    let listed = enabled || state.sim_setup.listed.contains(&index) || is_core(index);
    let painter = ui.painter();
    if active {
        painter.rect_filled(rect, 0.0, c.bg_hover);
    }

    let dot_center = egui::pos2(rect.left() + 13.0, rect.center().y);
    if enabled {
        painter.circle_filled(dot_center, 3.0, c.ok);
    } else {
        painter.circle_stroke(dot_center, 3.0, egui::Stroke::new(1.0, c.border_strong));
    }

    let (id, description) = analysis_meta(index);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            format!(
                "{id}: {description}. {meta}",
                meta = if enabled {
                    "Already in run set"
                } else if listed {
                    "Already listed"
                } else {
                    "Available to add"
                }
            ),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::ListBoxOption);
        node.set_selected(active);
    });
    painter.text(
        egui::pos2(rect.left() + 24.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        id,
        theme::mono(tokens::FS_1, FontWeight::Medium),
        if active {
            c.text
        } else {
            mix(c.text_dim, c.text, 0.5)
        },
    );
    let meta = if enabled {
        "in run set"
    } else if listed {
        "listed"
    } else {
        ""
    };
    let mut desc_right = rect.right() - 10.0;
    if !meta.is_empty() {
        let galley = ui.fonts_mut(|f| {
            f.layout_no_wrap(
                meta.to_owned(),
                theme::mono(10.0, FontWeight::Regular),
                c.text_faint,
            )
        });
        painter.galley(
            egui::pos2(
                rect.right() - 10.0 - galley.size().x,
                rect.center().y - galley.size().y * 0.5,
            ),
            galley.clone(),
            c.text_faint,
        );
        desc_right -= galley.size().x + 8.0;
    }
    let desc_left = rect.left() + 80.0;
    let desc_galley = ui.fonts_mut(|f| {
        let mut job = egui::text::LayoutJob::simple_singleline(
            description.to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            egui::Color32::PLACEHOLDER,
        );
        job.wrap = egui::text::TextWrapping::truncate_at_width((desc_right - desc_left).max(20.0));
        f.layout_job(job)
    });
    painter.galley(
        egui::pos2(desc_left, rect.center().y - desc_galley.size().y * 0.5),
        desc_galley,
        c.text_dim,
    );

    theme::paint_focus_ring(ui, &response, rect);

    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Keyboard hint strip at the palette foot.
fn palette_hint_strip(ui: &mut Ui) {
    let c = Tokens::get(ui.ctx()).color;
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 26.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.hline(
        rect.x_range(),
        rect.top() + 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    painter.text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "↑↓ · Enter adds · Shift+Enter keeps adding · Esc",
        theme::mono(tokens::FS_0, FontWeight::Regular),
        c.text_faint,
    );
}

fn run_bar(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    egui::Frame::NONE
        .fill(c.bg_panel)
        .stroke(egui::Stroke::new(1.0, c.border))
        .rounding(t.radius_lg)
        .inner_margin(egui::Margin::symmetric(14, 12))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 12.0;

                let run_block_reason = state.simulation_run_block_reason();
                let can_run = run_block_reason.is_none();
                let run_response = Button::new("Run")
                    .icon(Icon::Run)
                    .accent()
                    .enabled(can_run)
                    .show(ui);
                let run_clicked = run_response.clicked();
                if let Some(reason) = run_block_reason.as_deref() {
                    run_response.on_hover_text(reason);
                }
                if run_clicked {
                    state.simulation.request_simulate_run_set();
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
                let track_width = (ui.available_width() - status_width - 12.0).max(60.0);
                let (track_rect, _) =
                    ui.allocate_exact_size(egui::vec2(track_width, 5.0), egui::Sense::hover());
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
                } else if state.sim_setup.enabled.is_empty() {
                    "idle — nothing in the run set".to_owned()
                } else if let Some(run) = state.simulation.active_run() {
                    format!(
                        "idle — {} in run set · last run #{} {}",
                        state.sim_setup.enabled.len(),
                        run.id,
                        if run.success { "ok" } else { "failed" }
                    )
                } else {
                    format!("idle — {} in run set", state.sim_setup.enabled.len())
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
