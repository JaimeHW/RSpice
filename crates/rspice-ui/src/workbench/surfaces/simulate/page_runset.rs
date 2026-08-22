//! PVT, sweeps & variation.
//!
//! The page is the run space: a strip of axis cards joined by the composition
//! operator and closing on the forecast they multiply out to, over the editors
//! that declare them. Every edit is a transaction against the plan's run set —
//! it moves a working revision, leaves a receipt, and clears any frozen
//! forecast — so what the page shows and what a dispatch would execute are the
//! same declaration read twice rather than two things kept in step.

use egui::{Rect, Sense, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::simulation::plan::AnalysisKind;
use crate::simulation::run_set::{
    self, InvalidValuePolicy, RunSetAction, RunSetAdaptivePolicy, RunSetBudgets,
    RunSetCompositionMode, RunSetDimension, RunSetDimensionKind, RunSetReceiptStatus, RunSetState,
    RunSetValidation,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, IconButton, mono_input, select};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::RunSetBudgetDrafts;
use crate::workbench::{AppState, RSpiceApp};

use super::page_kit::{
    CARD_PAD_X, ReceiptRow, Tone, card, card_body, card_head_row, card_note, card_row, field_pair,
    ledger_head, receipts_card, rule_row,
};

/// Rows shown before the point table truncates. A composed run space can be
/// large; a table that listed all of it would be a scrolling wall rather than a
/// check on the composition. An excluded point is drawn wherever it falls: the
/// cap hides rows that behave alike, and an exclusion the user cannot see is
/// one they cannot lift.
const POINT_TABLE_LIMIT: usize = 40;

/// Value chips drawn on an axis card before the rest are counted.
const AXIS_CHIP_LIMIT: usize = 8;

/// Width of one axis card in the run-space strip. Fixed rather than shared:
/// axes with different value counts must still read as the same kind of thing.
const AXIS_CARD_W: f32 = 214.0;

/// Width of the closing forecast tile.
const FORECAST_TILE_W: f32 = 232.0;

/// Outside width of an axis card, including its horizontal frame margins. What
/// the strip packs its terms by; the cards themselves still own their exact
/// layout.
const AXIS_CARD_OUTER_W: f32 = AXIS_CARD_W + 18.0;

/// Width of the composition glyph between two terms of the run.
const OPERATOR_TILE_W: f32 = 18.0;

/// Gap between adjacent terms of the run, and between its rows.
const SPACE_TERM_GAP: f32 = 6.0;

/// Hit area of an axis card's enable switch. Wider than the 30-point switch so
/// the target clears the pointer minimum without moving the painted control.
const ANALYSIS_SWITCH_HIT_W: f32 = 34.0;
const ANALYSIS_SWITCH_HIT_H: f32 = 20.0;

/// Stable row height for every axis slot. Equal allocation keeps adjacent
/// cards top-aligned even when one source label or chip row measures wider.
const AXIS_CARD_SLOT_H: f32 = 130.0;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let validation = plan_run_set_validation(app);

    toolbar(ui, app, &validation);
    if !validation.errors.is_empty() || !validation.warnings.is_empty() {
        issue_summary(ui, app, &validation);
    }
    run_space(ui, app, &validation);
    super::workload::task_rate_card(ui, app, &validation);
    card_row(ui, app, selected_dimension, composition);
    card_row(ui, app, budgets, receipts);
    point_table(ui, app, &validation);
}

// ------------------------------------------------------------------ toolbar

/// The transaction commands, above the space they act on.
fn toolbar(ui: &mut Ui, app: &mut RSpiceApp, validation: &RunSetValidation) {
    let t = Tokens::get(ui.ctx());
    let addable = app.state.sim_setup.run_set.addable_kinds();
    let can_undo = !app.state.sim_setup.run_set.history.is_empty();
    let can_redo = !app.state.sim_setup.run_set.future.is_empty();
    let revision = app.state.sim_setup.run_set.revision;
    let mut action: Option<RunSetAction> = None;

    let width = ui.available_width();
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(CARD_PAD_X as i8, 7))
        .show(ui, |ui| {
            ui.set_width(width - CARD_PAD_X * 2.0 - 2.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 6.0;
                // The leading glyph was U+FF0B, which the bundled faces do not
                // carry: the page's first control opened with a replacement
                // box. The plus is painted as a vector by the button's own
                // icon slot, where no font has to have it.
                let choices = addable
                    .iter()
                    .map(|kind| super::page_kit::PopupChoice {
                        label: kind.default_name().to_owned(),
                        unavailable: kind.execution_blocker(),
                    })
                    .collect::<Vec<_>>();
                if let Some(index) = super::page_kit::command_popup(
                    ui,
                    "run-set.add-dimension",
                    Button::new("Add dimension").icon(Icon::Add),
                    "Every axis the executor binds is already declared.",
                    &choices,
                ) && let Some(kind) = addable.get(index)
                {
                    action = Some(RunSetAction::AddDimension(*kind));
                }
                if Button::new("Undo").enabled(can_undo).show(ui).clicked() {
                    action = Some(RunSetAction::Undo);
                }
                if Button::new("Redo").enabled(can_redo).show(ui).clicked() {
                    action = Some(RunSetAction::Redo);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if Button::new("Validate and preview")
                        .accent()
                        .icon(Icon::Grid)
                        .show(ui)
                        .clicked()
                    {
                        action = Some(RunSetAction::Preview);
                    }
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "working revision {revision} · {}",
                            validation.status.as_str()
                        ))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                    );
                });
            });
        });

    if let Some(action) = action {
        commit(app, action);
    }
}

// ------------------------------------------------------------ issue summary

/// Every standing refusal and advisory: a refusal is a jump to the control that
/// caused it, an advisory states something the space is doing that the axes do
/// not show. Advisories appear here rather than nowhere, because a report the
/// page never draws is a report nobody receives.
fn issue_summary(ui: &mut Ui, app: &mut RSpiceApp, validation: &RunSetValidation) {
    let t = Tokens::get(ui.ctx());
    let errors = validation.errors.len();
    let advisories = validation.warnings.len();
    let (title, status) = if errors > 0 {
        (
            format!(
                "Run-set preview blocked · {errors} issue{}",
                if errors == 1 { "" } else { "s" }
            ),
            ("invalid input retained", Tone::Error),
        )
    } else {
        (
            format!(
                "Run-set advisories · {advisories} note{}",
                if advisories == 1 { "" } else { "s" }
            ),
            ("preview available", Tone::Warn),
        )
    };
    let mut focus = None;
    card(ui, &title, Some(status), |ui| {
        card_body(ui, |ui| {
            for error in &validation.errors {
                let text = egui::RichText::new(format!("{} · {}", error.id, error.message))
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.err);
                // A refusal is a control only when it names a dimension to send
                // the operator to. Composition and budget refusals name none, so
                // they are read as statements rather than sensing a click that
                // would resolve to nothing.
                let Some(dimension_id) = error.dimension_id.clone() else {
                    ui.add(egui::Label::new(text).wrap())
                        .on_hover_text("Composition or budget refusal");
                    continue;
                };
                let response = ui
                    .add(egui::Label::new(text).wrap().sense(Sense::click()))
                    .on_hover_text("Select the dimension this refusal is about");
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled(),
                        format!("Select the dimension {} is about", error.id),
                    )
                });
                theme::paint_focus_ring(ui, &response, response.rect);
                if response.clicked() {
                    focus = Some(dimension_id);
                }
            }
            for warning in &validation.warnings {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{} · {}", warning.id, warning.message))
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.warn),
                    )
                    .wrap(),
                );
            }
        });
        card_note(
            ui,
            if errors > 0 {
                "Invalid input is preserved exactly as written. No task matrix, no dispatch and no \
                 result were created, and every prior dataset is retained unchanged."
            } else {
                "An advisory does not block a preview or a dispatch. It states a consequence of the \
                 declaration that the axis strip cannot show, and nothing it names has been \
                 discarded."
            },
        );
    });
    if let Some(id) = focus {
        app.state.workbench.selected_run_set_dimension = Some(id);
        app.state.workbench.run_set_values_draft = None;
    }
}

// ---------------------------------------------------------------- run space

/// The composed space: one card per axis, the operator between them, and the
/// forecast they resolve to.
fn run_space(ui: &mut Ui, app: &mut RSpiceApp, validation: &RunSetValidation) {
    let mode = app.state.sim_setup.run_set.composition.mode;
    let status = format!("{} composition", mode.as_str());
    let tone = match validation.status {
        run_set::RunSetStatus::Ready => Tone::Ok,
        run_set::RunSetStatus::Invalid => Tone::Error,
        run_set::RunSetStatus::NotEvaluated => Tone::Neutral,
    };
    let mut action = None;
    let mut selection = None;
    let ceiling_per_point = task_ceiling_per_point(app);

    super::page_kit::card_with_head(
        ui,
        |ui| card_head_row(ui, "Run space", Some((status.as_str(), tone)), |_| {}),
        |ui| {
            card_body(ui, |ui| {
                let selected = app.state.workbench.selected_run_set_dimension.clone();
                let dimensions = app.state.sim_setup.run_set.dimensions.clone();
                for row in space_rows(ui.available_width(), dimensions.len()) {
                    // Top-aligned, because an egui row places each item
                    // against the row height known when the item is added: a
                    // centred row hung the forecast tile -- the tallest term,
                    // and the last -- forty points below the axis cards it is
                    // the product of, so the expression had no common edge to
                    // read along.
                    ui.horizontal_top(|ui| {
                        ui.spacing_mut().item_spacing.x = SPACE_TERM_GAP;
                        ui.add_space(((ui.available_width() - row.width) * 0.5).max(0.0));
                        for term in row.terms {
                            let pending = term.index.checked_sub(1).map(|previous| {
                                let active = match dimensions.get(term.index) {
                                    Some(dimension) => {
                                        dimensions[previous].enabled && dimension.enabled
                                    }
                                    None => validation.is_ready(),
                                };
                                let glyph = if term.index < dimensions.len() {
                                    mode.operator()
                                } else {
                                    "="
                                };
                                operator_tile(ui, glyph, active)
                            });
                            let placed = match dimensions.get(term.index) {
                                Some(dimension) => {
                                    let is_selected =
                                        selected.as_deref() == Some(dimension.id.as_str());
                                    let (rect, event) =
                                        axis_card(ui, dimension, term.index, is_selected);
                                    match event {
                                        Some(AxisEvent::Select) => {
                                            selection = Some(dimension.id.clone());
                                        }
                                        Some(AxisEvent::SetEnabled(enabled)) => {
                                            action = Some(RunSetAction::SetEnabled {
                                                id: dimension.id.clone(),
                                                enabled,
                                            });
                                        }
                                        None => {}
                                    }
                                    rect
                                }
                                None => forecast_tile(ui, validation, ceiling_per_point),
                            };
                            if let Some(pending) = pending {
                                pending.settle(ui, placed);
                            }
                        }
                    });
                    ui.add_space(SPACE_TERM_GAP);
                }
            });
            variation_strip(ui, app);
        },
    );

    if let Some(id) = selection {
        app.state.workbench.selected_run_set_dimension = Some(id);
        app.state.workbench.run_set_values_draft = None;
    }
    if let Some(action) = action {
        commit(app, action);
    }
}

enum AxisEvent {
    Select,
    SetEnabled(bool),
}

/// One term of the composed expression, and where it sits in the run.
///
/// `index` addresses the declared dimensions; the one index past the last is
/// the forecast the run resolves to. Every term but the first is preceded by an
/// operator, which is what binds the two into one atom that wraps together.
struct SpaceTerm {
    index: usize,
}

/// One packed row of the strip: its terms and the width they occupy.
struct SpaceRow {
    terms: Vec<SpaceTerm>,
    width: f32,
}

/// Pack the cards, their operators and the forecast tile into as few rows as
/// `available` holds.
///
/// The strip is one expression, so its terms are packed and centred rather than
/// laid out on a grid: a fixed two-column grid left a 2×2 island with empty
/// gutters at 1600 points and a 500-point island inside a 2540-point card at
/// 2560, and it stranded the forecast on a row of its own however much room the
/// axes had left beside it. Terms neither grow nor shrink — every axis reads at
/// the same weight, and the run wraps where the width runs out.
fn space_rows(available: f32, dimension_count: usize) -> Vec<SpaceRow> {
    let mut rows: Vec<SpaceRow> = Vec::new();
    let mut terms: Vec<SpaceTerm> = Vec::new();
    let mut width = 0.0f32;
    for index in 0..=dimension_count {
        let term_width = if index < dimension_count {
            AXIS_CARD_OUTER_W
        } else {
            FORECAST_TILE_W + 20.0
        };
        // The operator belongs to the term it introduces rather than to the one
        // it follows, so a wrap carries it onto the next row with its term
        // instead of leaving it dangling at the end of the previous one.
        let atom = if index == 0 {
            term_width
        } else {
            OPERATOR_TILE_W + SPACE_TERM_GAP + term_width
        };
        let advance = if terms.is_empty() {
            atom
        } else {
            SPACE_TERM_GAP + atom
        };
        if !terms.is_empty() && width + advance > available {
            rows.push(SpaceRow {
                terms: std::mem::take(&mut terms),
                width,
            });
            width = atom;
        } else {
            width += advance;
        }
        terms.push(SpaceTerm { index });
    }
    rows.push(SpaceRow { terms, width });
    rows
}

/// A composition glyph whose vertical placement waits for the term it joins.
struct PendingOperator {
    shape: egui::layers::ShapeIdx,
    rect: Rect,
    glyph: String,
    color: egui::Color32,
}

impl PendingOperator {
    /// Paint the glyph level with the term it introduces.
    ///
    /// An egui row places each item against the row height known when the item
    /// is added, and never moves it once a taller neighbour arrives. Painting
    /// the glyph before the tile it belongs to left it at the tile's top edge
    /// rather than beside it.
    fn settle(self, ui: &Ui, term: Rect) {
        let galley = ui.painter().layout_no_wrap(
            self.glyph,
            theme::mono(tokens::FS_2, FontWeight::Regular),
            self.color,
        );
        let position = egui::pos2(
            self.rect.center().x - galley.size().x * 0.5,
            term.center().y - galley.size().y * 0.5,
        );
        ui.painter().set(
            self.shape,
            egui::epaint::TextShape::new(position, galley, self.color),
        );
    }
}

/// One axis of the space: what it contributes and where it comes from.
///
/// Returns the extent it took, which the operator before it is placed against.
fn axis_card(
    ui: &mut Ui,
    dimension: &RunSetDimension,
    index: usize,
    selected: bool,
) -> (Rect, Option<AxisEvent>) {
    let t = Tokens::get(ui.ctx());
    let mut event = None;
    let fill = if selected {
        t.color.accent_dim
    } else if dimension.enabled {
        t.color.bg_panel_2
    } else {
        t.color.bg_inset
    };
    let stroke = egui::Stroke::new(
        1.0,
        if selected {
            t.color.accent
        } else {
            t.color.border
        },
    );
    let text_color = if dimension.enabled {
        t.color.text
    } else {
        t.color.text_faint
    };

    let slot = ui.allocate_ui_with_layout(
        vec2(AXIS_CARD_OUTER_W, AXIS_CARD_SLOT_H),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_width(AXIS_CARD_OUTER_W);
            egui::Frame::new()
                .fill(fill)
                .stroke(stroke)
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(8, 7))
                .show(ui, |ui| {
                    ui.set_width(AXIS_CARD_W);
                    ui.spacing_mut().item_spacing.y = 5.0;

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("{:02}", index + 1))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                        let title = ui
                            .add(
                                egui::Label::new(
                                    egui::RichText::new(&dimension.name)
                                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                        .color(text_color),
                                )
                                .sense(Sense::click()),
                            )
                            .on_hover_text("Edit this dimension");
                        title.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Button,
                                ui.is_enabled(),
                                format!("Edit dimension {}", dimension.name),
                            )
                        });
                        theme::paint_focus_ring(ui, &title, title.rect);
                        if title.clicked() {
                            event = Some(AxisEvent::Select);
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // The same switch the analysis rail draws, because
                            // it is the same act: an entity declared in the plan
                            // is either in the run or out of it. An egui
                            // checkbox here was a third enable idiom on a page
                            // that already had two.
                            let can_toggle =
                                dimension.enabled || dimension.kind.execution_blocker().is_none();
                            ui.add_enabled_ui(can_toggle, |ui| {
                                let (hit, response) = ui.allocate_exact_size(
                                    vec2(ANALYSIS_SWITCH_HIT_W, ANALYSIS_SWITCH_HIT_H),
                                    Sense::click(),
                                );
                                response.widget_info(|| {
                                    egui::WidgetInfo::selected(
                                        egui::WidgetType::Checkbox,
                                        ui.is_enabled(),
                                        dimension.enabled,
                                        format!("Include {} in the run space", dimension.name),
                                    )
                                });
                                let response = response.on_hover_text(format!(
                                    "Include {} in the run space",
                                    dimension.name
                                ));
                                let response = match dimension.kind.execution_blocker() {
                                    Some(reason) => response.on_disabled_hover_text(reason),
                                    None => response,
                                };
                                super::paint_switch(
                                    ui,
                                    hit.center(),
                                    dimension.enabled,
                                    response.hovered(),
                                    hit,
                                );
                                theme::paint_focus_ring(ui, &response, hit);
                                if response.clicked() {
                                    event = Some(AxisEvent::SetEnabled(!dimension.enabled));
                                }
                            });
                        });
                    });
                    ui.label(
                        egui::RichText::new(format!(
                            "{} · {}",
                            dimension.kind.as_str(),
                            dimension.kind.value_type().as_str()
                        ))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                    );

                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = vec2(4.0, 4.0);
                        for value in dimension.values.iter().take(AXIS_CHIP_LIMIT) {
                            let tone = if value.canonical.is_some() {
                                text_color
                            } else {
                                t.color.err
                            };
                            value_chip(ui, &value.lexical, tone);
                        }
                        if dimension.values.len() > AXIS_CHIP_LIMIT {
                            value_chip(
                                ui,
                                &format!("+{}", dimension.values.len() - AXIS_CHIP_LIMIT),
                                t.color.text_faint,
                            );
                        }
                        if dimension.values.is_empty() {
                            value_chip(ui, "no values", t.color.err);
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "{} value{}",
                                dimension.values.len(),
                                if dimension.values.len() == 1 { "" } else { "s" }
                            ))
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let short = dimension
                                .source
                                .rsplit(':')
                                .next()
                                .unwrap_or(&dimension.source);
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(short)
                                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                        .color(t.color.text_faint),
                                )
                                .truncate(),
                            )
                            .on_hover_text(&dimension.source);
                        });
                    });
                });
        },
    );
    (slot.response.rect, event)
}

fn value_chip(ui: &mut Ui, text: &str, color: egui::Color32) {
    let t = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let width = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), color)
        .size()
        .x;
    let (rect, _) = ui.allocate_exact_size(vec2(width + 10.0, 17.0), Sense::hover());
    ui.painter().rect(
        rect,
        3.0,
        t.color.bg_inset,
        egui::Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        font,
        color,
    );
}

/// Reserve the composition glyph drawn before a term; it is painted once that
/// term's own extent is known.
fn operator_tile(ui: &mut Ui, glyph: &str, active: bool) -> PendingOperator {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(OPERATOR_TILE_W, 26.0), Sense::hover());
    PendingOperator {
        shape: ui.painter().add(egui::Shape::Noop),
        rect,
        glyph: glyph.to_owned(),
        color: if active {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    }
}

/// What the axes multiply out to.
///
/// Returns the extent it took, which the `=` before it is placed against.
fn forecast_tile(
    ui: &mut Ui,
    validation: &RunSetValidation,
    ceiling_per_point: Option<usize>,
) -> Rect {
    let t = Tokens::get(ui.ctx());
    let forecast = validation.forecast;
    let tile = ui.allocate_ui_with_layout(
        vec2(FORECAST_TILE_W + 20.0, 0.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_width(FORECAST_TILE_W + 20.0);
            egui::Frame::new()
                .fill(t.color.bg_panel_2)
                .stroke(egui::Stroke::new(
                    1.0,
                    if validation.is_ready() {
                        t.color.ok
                    } else {
                        t.color.border
                    },
                ))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(9, 7))
                .show(ui, |ui| {
                    ui.set_width(FORECAST_TILE_W);
                    ui.spacing_mut().item_spacing.y = 4.0;
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(if forecast.exact {
                                forecast.point_count.to_string()
                            } else {
                                format!(
                                    "{}–{}",
                                    forecast.point_count_minimum, forecast.point_count_maximum
                                )
                            })
                            .font(theme::mono(tokens::FS_4, FontWeight::SemiBold))
                            .color(t.color.text),
                        );
                        ui.label(
                            egui::RichText::new(if forecast.exact && forecast.point_count == 1 {
                                "point"
                            } else {
                                "points"
                            })
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                        );
                    });
                    ui.label(
                        egui::RichText::new(format!(
                            "{} enabled analysis instance{}",
                            forecast.enabled_analysis_count,
                            if forecast.enabled_analysis_count == 1 {
                                ""
                            } else {
                                "s"
                            }
                        ))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                    );
                    let mut rows = vec![("Tasks", forecast.task_count.to_string())];
                    // Stated only where it is a bound the reader cannot derive
                    // from the two numbers above. With every analysis at every
                    // point the ceiling is just tasks ÷ points, and a row
                    // restating that is noise on a tile this small.
                    if let Some(ceiling) = ceiling_per_point
                        && forecast.exact
                        && forecast.task_count != ceiling.saturating_mul(forecast.point_count)
                    {
                        rows.push(("Per point", format!("\u{2264} {ceiling} tasks")));
                    }
                    rows.push(("Cost", run_set::format_duration_ms(forecast.cost_ms)));
                    rows.push(("Storage", run_set::format_bytes(forecast.storage_bytes)));
                    for (label, value) in rows {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(label)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(value)
                                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                            .color(t.color.text),
                                    );
                                },
                            );
                        });
                    }
                });
        },
    );
    tile.response.rect
}

/// Statistical variation is a Monte Carlo analysis, not an axis of this space.
fn variation_strip(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let active = app
        .state
        .sim_setup
        .has_enabled_analysis_kind(AnalysisKind::MonteCarlo);
    let sample_count = if active {
        let mut setup = app.state.sim_setup.mc.clone();
        setup.ensure_initialized();
        setup.to_config().ok().map(|config| config.num_runs)
    } else {
        None
    };
    // Raised out of the frame rather than acted on inside it: inserting an
    // instance takes the whole application, and the strip is still holding a
    // borrow of the state it read `active` from.
    let mut add_monte_carlo = false;
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(egui::Margin {
            left: CARD_PAD_X as i8,
            right: CARD_PAD_X as i8,
            top: 7,
            bottom: 7,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = vec2(8.0, 4.0);
                ui.label(
                    egui::RichText::new(if active { "active" } else { "not in run set" })
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(if active {
                            t.color.ok
                        } else {
                            t.color.text_faint
                        }),
                );
                // The strip's whole message when inactive is that variation is
                // owned by an analysis this plan does not have. Naming the fix
                // without offering it sends the reader to the navigator to do
                // what this page just told them to do.
                if !active {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // No ellipsis: this inserts the instance outright,
                        // through the same catalog path the Analyses page uses.
                        // An ellipsis promises a dialog that never opens.
                        if Button::new("Add Monte Carlo").show(ui).clicked() {
                            add_monte_carlo = true;
                        }
                    });
                }
            });
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(8.0, 4.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(match sample_count {
                            Some(samples) => format!(
                                "Each PVT point executes one Monte Carlo task with {samples} \
                                 configured samples. The task forecast remains analysis-level; \
                                 the declared seed keeps the trial stream reproducible."
                            ),
                            None if active => "Monte Carlo is enabled, but its sample count is \
                                invalid. Correct the analysis before dispatch."
                                .to_owned(),
                            None => "Statistical variation is owned by a Monte Carlo analysis \
                                instance. None is enabled in this plan, so the matrix carries \
                                deterministic points only."
                                .to_owned(),
                        })
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            });
        });

    if add_monte_carlo {
        // The same insertion the catalog performs, so the instance arrives
        // with its prerequisites bound and its receipt recorded rather than as
        // a second way of adding an analysis that behaves almost the same.
        super::lifecycle::insert_analysis_instance(app, AnalysisKind::MonteCarlo);
    }
}

// -------------------------------------------------------- dimension editor

/// Everything the selected axis declares.
fn selected_dimension(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected = app
        .state
        .workbench
        .selected_run_set_dimension
        .clone()
        .filter(|id| app.state.sim_setup.run_set.dimension(id).is_some())
        .or_else(|| {
            app.state
                .sim_setup
                .run_set
                .dimensions
                .first()
                .map(|dimension| dimension.id.clone())
        });
    let Some(id) = selected else {
        card(
            ui,
            "Selected dimension",
            Some(("none declared", Tone::Warn)),
            |ui| {
                card_note(
                    ui,
                    "The run space declares no axes, so every analysis runs once at the plan's \
                     reference point. Add a dimension to sweep.",
                );
            },
        );
        return;
    };
    let dimension = app
        .state
        .sim_setup
        .run_set
        .dimension(&id)
        .expect("the selected identity was just resolved")
        .clone();
    let index = app
        .state
        .sim_setup
        .run_set
        .index_of(&id)
        .unwrap_or_default();
    let last = app.state.sim_setup.run_set.dimensions.len() - 1;
    let title = format!("Selected dimension · {}", dimension.name);
    let status = if dimension.kind.execution_blocker().is_some() {
        ("unavailable", Tone::Error)
    } else if dimension.enabled {
        ("enabled", Tone::Ok)
    } else {
        ("disabled", Tone::Neutral)
    };
    // The head and the body both raise commands, and two closures cannot each
    // hold the same `&mut` — the pending command is shared through a cell.
    let action: std::cell::RefCell<Option<RunSetAction>> = std::cell::RefCell::new(None);

    super::page_kit::card_with_head(
        ui,
        |ui| {
            card_head_row(ui, &title, Some(status), |ui| {
                if IconButton::new(Icon::Trash)
                    .tooltip("Remove this dimension")
                    .show(ui)
                    .clicked()
                {
                    *action.borrow_mut() = Some(RunSetAction::RemoveDimension { id: id.clone() });
                }
                if IconButton::new(Icon::ChevronDown)
                    .enabled(index < last)
                    .tooltip("Move later")
                    .show(ui)
                    .clicked()
                {
                    *action.borrow_mut() = Some(RunSetAction::MoveDimension {
                        id: id.clone(),
                        later: true,
                    });
                }
                if IconButton::new(Icon::ChevronUp)
                    .enabled(index > 0)
                    .tooltip("Move earlier")
                    .show(ui)
                    .clicked()
                {
                    *action.borrow_mut() = Some(RunSetAction::MoveDimension {
                        id: id.clone(),
                        later: false,
                    });
                }
            });
        },
        |ui| {
            card_body(ui, |ui| {
                let mut name = dimension.name.clone();
                let mut source = dimension.source.clone();
                let mut name_released = false;
                let mut source_released = false;
                field_pair(
                    ui,
                    ("Name", &mut |ui: &mut Ui, width: f32| {
                        name_released |= mono_input(ui, &mut name, width).lost_focus();
                    }),
                    Some(("Source authority", &mut |ui: &mut Ui, width: f32| {
                        source_released |= mono_input(ui, &mut source, width).lost_focus();
                    })),
                );
                if name_released && name != dimension.name {
                    *action.borrow_mut() = Some(RunSetAction::Rename {
                        id: id.clone(),
                        name,
                    });
                } else if source_released && source != dimension.source {
                    *action.borrow_mut() = Some(RunSetAction::SetSource {
                        id: id.clone(),
                        source,
                    });
                }

                rule_row(
                    ui,
                    "Binds to",
                    match dimension.kind {
                        RunSetDimensionKind::Parameter => {
                            "an exact design variable through a point-specific .param override"
                        }
                        RunSetDimensionKind::Source => "an explicitly named independent source",
                        RunSetDimensionKind::ProcessSection => {
                            "the library section every model resolves through"
                        }
                        RunSetDimensionKind::Supply => "the DC value of the design's supplies",
                        RunSetDimensionKind::Temperature => {
                            "the ambient solve temperature used for device and model evaluation"
                        }
                        RunSetDimensionKind::Model => {
                            "a section of one ordered plan-owned model binding"
                        }
                        RunSetDimensionKind::Frequency => {
                            "a compatible analysis instance's frequency control"
                        }
                        RunSetDimensionKind::Time => {
                            "a compatible analysis instance's time control"
                        }
                        RunSetDimensionKind::Seed => {
                            "the reproducible seed of a statistical analysis"
                        }
                        RunSetDimensionKind::Sample => "the sample count of a statistical analysis",
                        RunSetDimensionKind::AnalysisSelection => {
                            "the plan analysis instances executed at each point"
                        }
                        RunSetDimensionKind::DigitalConfiguration => {
                            "a frozen mixed-signal digital configuration"
                        }
                        RunSetDimensionKind::ExternalDataset => {
                            "a sealed external dataset identity"
                        }
                    },
                );
                if let Some(reason) = dimension.kind.execution_blocker() {
                    rule_row(ui, "Execution", reason);
                }

                // The value list is a draft until focus leaves: committing per
                // keystroke would move the plan revision on every character and
                // refuse the space while a number was half-typed.
                let draft_matches = app
                    .state
                    .workbench
                    .run_set_values_draft
                    .as_ref()
                    .is_some_and(|(draft_id, _)| *draft_id == id);
                if !draft_matches {
                    app.state.workbench.run_set_values_draft =
                        Some((id.clone(), dimension.values_text()));
                }
                let mut text = app
                    .state
                    .workbench
                    .run_set_values_draft
                    .as_ref()
                    .map(|(_, text)| text.clone())
                    .unwrap_or_default();
                let unit = dimension
                    .unit()
                    .map(|unit| format!(" · {unit}"))
                    .unwrap_or_default();
                ui.label(
                    egui::RichText::new(format!("Typed values · one per line{unit}"))
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(Tokens::get(ui.ctx()).color.text_dim),
                );
                let response = ui.add(
                    egui::TextEdit::multiline(&mut text)
                        .desired_rows(5)
                        .desired_width(ui.available_width())
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                );
                if response.changed() {
                    app.state.workbench.run_set_values_draft = Some((id.clone(), text.clone()));
                }
                if response.lost_focus() && text != dimension.values_text() {
                    *action.borrow_mut() = Some(RunSetAction::SetValues {
                        id: id.clone(),
                        text,
                    });
                    app.state.workbench.run_set_values_draft = None;
                }

                let policies = InvalidValuePolicy::ALL
                    .map(|policy| policy_label(policy).to_owned())
                    .to_vec();
                let current = policy_label(dimension.invalid_value_policy).to_owned();
                let mut picked = None;
                field_pair(
                    ui,
                    ("Invalid-value policy", &mut |ui: &mut Ui, width: f32| {
                        picked = select(
                            ui,
                            "simulation.runset.invalid-policy",
                            "Invalid-value policy",
                            &current,
                            &policies,
                            width,
                        );
                    }),
                    None,
                );
                if let Some(index) = picked {
                    *action.borrow_mut() = Some(RunSetAction::SetInvalidValuePolicy {
                        id: id.clone(),
                        policy: InvalidValuePolicy::ALL[index.min(1)],
                    });
                }
            });
            card_note(
                ui,
                "A value that does not parse keeps its place and its identity so the points it \
                 blocks can be named. Nothing is corrected, rounded or dropped on the way to the \
                 engine.",
            );
        },
    );

    if let Some(action) = action.into_inner() {
        commit(app, action);
    }
}

fn policy_label(policy: InvalidValuePolicy) -> &'static str {
    match policy {
        InvalidValuePolicy::PreserveAndBlockAffectedPoints => "Preserve · block affected points",
        InvalidValuePolicy::BlockEntireRunSet => "Preserve · block the entire run set",
    }
}

// ------------------------------------------------------------- composition

fn composition(ui: &mut Ui, app: &mut RSpiceApp) {
    let current_composition = app.state.sim_setup.run_set.composition.clone();
    let mode = current_composition.mode;
    let excluded = current_composition.excluded_points.len();
    let variation = app
        .state
        .sim_setup
        .has_enabled_analysis_kind(AnalysisKind::MonteCarlo);
    let mut action: Option<RunSetAction> = None;
    card(
        ui,
        "Composition",
        Some((mode.as_str(), Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                let available_modes = RunSetCompositionMode::ALL
                    .into_iter()
                    .filter(|mode| mode.execution_blocker().is_none())
                    .collect::<Vec<_>>();
                let options: Vec<String> = available_modes
                    .iter()
                    .map(|mode| mode.label().to_owned())
                    .collect();
                let current = mode.label().to_owned();
                let mut picked = None;
                field_pair(
                    ui,
                    ("Mode", &mut |ui: &mut Ui, width: f32| {
                        picked = select(
                            ui,
                            "simulation.runset.composition",
                            "Axis composition",
                            &current,
                            &options,
                            width,
                        );
                    }),
                    None,
                );
                if let Some(index) = picked
                    && let Some(mode) = available_modes.get(index).copied()
                {
                    action = Some(RunSetAction::SetComposition(
                        current_composition.with_mode(mode),
                    ));
                }
                rule_row(
                    ui,
                    "Adaptive",
                    RunSetCompositionMode::Adaptive
                        .execution_blocker()
                        .expect("adaptive composition is deliberately unavailable"),
                );
                if mode == RunSetCompositionMode::Conditional {
                    let mut edited = current_composition.clone();
                    let mut predicate = edited.predicate.clone();
                    let released =
                        mono_input(ui, &mut predicate, ui.available_width()).lost_focus();
                    ui.label("Predicate grammar: dimension-id == value; join clauses with &&");
                    if released && predicate != edited.predicate {
                        edited.predicate = predicate;
                        action = Some(RunSetAction::SetComposition(edited.clone()));
                    }
                    ui.label("Authorized upstream dimensions");
                    for dimension in app.state.sim_setup.run_set.enabled_dimensions() {
                        let mut selected = edited
                            .upstream_dimension_ids
                            .iter()
                            .any(|id| id == &dimension.id);
                        if ui.checkbox(&mut selected, &dimension.name).changed() {
                            if selected {
                                edited.upstream_dimension_ids.push(dimension.id.clone());
                            } else {
                                edited
                                    .upstream_dimension_ids
                                    .retain(|id| id != &dimension.id);
                            }
                            action = Some(RunSetAction::SetComposition(edited.clone()));
                        }
                    }
                }
                if mode == RunSetCompositionMode::Nested {
                    let mut edited = current_composition.clone();
                    let mut depth = edited.maximum_depth.to_string();
                    if mono_input(ui, &mut depth, 120.0).lost_focus()
                        && let Ok(parsed) = depth.trim().parse::<u8>()
                        && parsed != edited.maximum_depth
                    {
                        edited.maximum_depth = parsed;
                        action = Some(RunSetAction::SetComposition(edited));
                    }
                }
                if mode == RunSetCompositionMode::Adaptive {
                    let mut edited = current_composition.clone();
                    let mut policy =
                        edited
                            .adaptive_policy
                            .clone()
                            .unwrap_or(RunSetAdaptivePolicy {
                                id: String::new(),
                                objective: String::new(),
                                seed: 1,
                                bounds: "{}".to_owned(),
                                stop_rule: String::new(),
                                maximum_proposals: 1,
                            });
                    let mut seed = policy.seed.to_string();
                    let mut maximum = policy.maximum_proposals.to_string();
                    let released = std::cell::Cell::new(false);
                    field_pair(
                        ui,
                        ("Policy ID", &mut |ui: &mut Ui, width: f32| {
                            released.set(
                                released.get() | mono_input(ui, &mut policy.id, width).lost_focus(),
                            );
                        }),
                        Some(("Objective", &mut |ui: &mut Ui, width: f32| {
                            released.set(
                                released.get()
                                    | mono_input(ui, &mut policy.objective, width).lost_focus(),
                            );
                        })),
                    );
                    field_pair(
                        ui,
                        ("Seed", &mut |ui: &mut Ui, width: f32| {
                            released.set(
                                released.get() | mono_input(ui, &mut seed, width).lost_focus(),
                            );
                        }),
                        Some(("Maximum proposals", &mut |ui: &mut Ui, width: f32| {
                            released.set(
                                released.get() | mono_input(ui, &mut maximum, width).lost_focus(),
                            );
                        })),
                    );
                    field_pair(
                        ui,
                        ("Bounds · JSON", &mut |ui: &mut Ui, width: f32| {
                            released.set(
                                released.get()
                                    | mono_input(ui, &mut policy.bounds, width).lost_focus(),
                            );
                        }),
                        Some(("Stop rule", &mut |ui: &mut Ui, width: f32| {
                            released.set(
                                released.get()
                                    | mono_input(ui, &mut policy.stop_rule, width).lost_focus(),
                            );
                        })),
                    );
                    if released.get()
                        && let (Ok(seed), Ok(maximum_proposals)) =
                            (seed.trim().parse::<u64>(), maximum.trim().parse::<usize>())
                    {
                        policy.seed = seed;
                        policy.maximum_proposals = maximum_proposals;
                        edited.adaptive_policy = Some(policy);
                        action = Some(RunSetAction::SetComposition(edited));
                    }
                }
                rule_row(ui, "Contract", mode.contract());
                rule_row(
                    ui,
                    "Statistical variation",
                    if variation {
                        "enabled · one Monte Carlo task executes at every point"
                    } else {
                        "disabled · no Monte Carlo analysis is enabled"
                    },
                );
                rule_row(
                    ui,
                    "Excluded points",
                    &match (mode, excluded) {
                        (_, 0) => "none · the composition runs whole".to_owned(),
                        (RunSetCompositionMode::Filtered, count) => {
                            format!("{count} removed in the point table below")
                        }
                        (_, count) => format!(
                            "{count} held but not applied · only a filtered composition subtracts \
                             them"
                        ),
                    },
                );
                rule_row(
                    ui,
                    "Point identity",
                    "every coordinate is carried into the run manifest",
                );
            });
            card_note(
                ui,
                "Conditional predicates and nested traversal resolve deterministically into exact point identities. Adaptive declarations remain fail-closed until the campaign scheduler can authorize proposals from completed evidence; a direct run never pretends the maximum proposal budget is an executed matrix.",
            );
        },
    );
    if let Some(action) = action {
        commit(app, action);
    }
}

// ----------------------------------------------------------------- budgets

fn budgets(ui: &mut Ui, app: &mut RSpiceApp) {
    let current = app.state.sim_setup.run_set.budgets;
    let validation = plan_run_set_validation(app);
    let exceeded = validation
        .errors
        .iter()
        .any(|error| error.id.ends_with("BUDGET"));
    if app.state.workbench.run_set_budget_drafts.is_none() {
        app.state.workbench.run_set_budget_drafts = Some(drafts_from(&current));
    }
    let mut action: Option<RunSetAction> = None;

    card(
        ui,
        "Execution budgets",
        Some((
            if exceeded {
                "exceeded"
            } else {
                "within budget"
            },
            if exceeded { Tone::Error } else { Tone::Ok },
        )),
        |ui| {
            card_body(ui, |ui| {
                let mut drafts = app
                    .state
                    .workbench
                    .run_set_budget_drafts
                    .clone()
                    .unwrap_or_else(|| drafts_from(&current));
                let released = std::cell::Cell::new(false);
                field_pair(
                    ui,
                    ("Maximum tasks", &mut |ui: &mut Ui, width: f32| {
                        released.set(
                            released.get()
                                | mono_input(ui, &mut drafts.maximum_tasks, width).lost_focus(),
                        );
                    }),
                    Some(("Maximum storage", &mut |ui: &mut Ui, width: f32| {
                        released.set(
                            released.get()
                                | mono_input(ui, &mut drafts.maximum_storage, width).lost_focus(),
                        );
                    })),
                );
                field_pair(
                    ui,
                    ("Cost / task · ms", &mut |ui: &mut Ui, width: f32| {
                        released.set(
                            released.get()
                                | mono_input(ui, &mut drafts.cost_per_point_ms, width).lost_focus(),
                        );
                    }),
                    Some(("Storage / task", &mut |ui: &mut Ui, width: f32| {
                        released.set(
                            released.get()
                                | mono_input(ui, &mut drafts.bytes_per_point, width).lost_focus(),
                        );
                    })),
                );
                app.state.workbench.run_set_budget_drafts = Some(drafts.clone());
                if released.get() {
                    match budgets_from(&drafts) {
                        Ok(budgets) if budgets != current => {
                            action = Some(RunSetAction::SetBudgets(budgets));
                        }
                        Ok(_) => {}
                        Err(error) => {
                            app.state
                                .workbench
                                .analysis_lifecycle_status
                                .record_refusal(error);
                            app.state.workbench.run_set_budget_drafts = Some(drafts_from(&current));
                        }
                    }
                }
            });
            card_note(
                ui,
                "Exceeding a budget blocks preview and dispatch. Nothing is truncated silently, \
                 and the cost and storage models are the plan's own — they state what this run \
                 would consume, not a measured result.",
            );
        },
    );
    if let Some(action) = action {
        commit(app, action);
        app.state.workbench.run_set_budget_drafts = None;
    }
}

fn drafts_from(budgets: &RunSetBudgets) -> RunSetBudgetDrafts {
    RunSetBudgetDrafts {
        maximum_tasks: budgets.maximum_tasks.to_string(),
        maximum_storage: run_set::format_bytes(budgets.maximum_storage_bytes),
        cost_per_point_ms: budgets.cost_per_point_ms.to_string(),
        bytes_per_point: run_set::format_bytes(budgets.bytes_per_point),
    }
}

fn budgets_from(drafts: &RunSetBudgetDrafts) -> Result<RunSetBudgets, String> {
    Ok(RunSetBudgets {
        maximum_tasks: drafts
            .maximum_tasks
            .trim()
            .replace([',', '_'], "")
            .parse()
            .map_err(|_| format!("{:?} is not a task count", drafts.maximum_tasks))?,
        maximum_storage_bytes: run_set::parse_bytes(&drafts.maximum_storage)?,
        cost_per_point_ms: drafts
            .cost_per_point_ms
            .trim()
            .parse()
            .map_err(|_| format!("{:?} is not a duration in ms", drafts.cost_per_point_ms))?,
        bytes_per_point: run_set::parse_bytes(&drafts.bytes_per_point)?,
    })
}

// ---------------------------------------------------------------- receipts

fn receipts(ui: &mut Ui, app: &mut RSpiceApp) {
    let rows = app
        .state
        .sim_setup
        .run_set
        .receipts
        .iter()
        .map(|receipt| ReceiptRow {
            sequence: receipt.sequence.to_string(),
            action: receipt.action.to_owned(),
            tone: match receipt.status {
                RunSetReceiptStatus::Completed => Tone::Ok,
                RunSetReceiptStatus::Blocked => Tone::Error,
            },
            revision: format!("{} → {}", receipt.before_revision, receipt.after_revision),
            digest: receipt.digest.clone(),
        })
        .collect::<Vec<_>>();
    receipts_card(
        ui,
        "Transaction receipts",
        "no transaction",
        (
            "No run-set transaction has run in this session. Editing an axis, a composition or a \
             budget records one; so does a validate-and-preview, which creates planning evidence \
             only and no numerical result.",
            "Every prior dataset is retained unchanged. A blocked transaction leaves the \
             declaration exactly as it was and still records that it was attempted.",
        ),
        &rows,
    );
}

// ---------------------------------------------------------- resolved points

fn point_table(ui: &mut Ui, app: &mut RSpiceApp, validation: &RunSetValidation) {
    let state: &RunSetState = &app.state.sim_setup.run_set;
    let Some(points) = run_set::compose(state) else {
        card(
            ui,
            "Resolved point table",
            Some(("unresolved", Tone::Error)),
            |ui| {
                card_note(
                    ui,
                    "The declared space does not expand exactly, so no point list is shown. The \
                     refusals above name what has to change; a partial table would read as a \
                     shorter run than the one that would execute.",
                );
            },
        );
        return;
    };

    let axes: Vec<&RunSetDimension> = state
        .enabled_dimensions()
        .filter(|dimension| !dimension.values.is_empty())
        .collect();
    // Excluding a point is what a zipped composition has no form for: a
    // pairing minus a pair is a different pairing, not the same one shorter.
    let excludable = state.composition.mode != RunSetCompositionMode::Zipped;
    let excluded = &state.composition.excluded_points;

    // A row past the cap is still drawn when it is excluded. The cap exists so
    // a large space does not become a scrolling wall, but an exclusion the user
    // cannot see is one they cannot undo, and hiding it would be the silent
    // drop this whole feature refuses.
    let rows: Vec<(usize, &run_set::RunSetPoint<'_>, String, bool)> = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let key = point.point_key();
            let is_excluded = excludable && excluded.contains(&key);
            (index, point, key, is_excluded)
        })
        .filter(|(index, _, _, is_excluded)| *index < POINT_TABLE_LIMIT || *is_excluded)
        .collect();

    let composed = points.len();
    let running = validation.forecast.point_count;
    // The same queue the table's rows add up to, so it is priced the same way
    // rather than left as a count the reader has to convert themselves.
    let duration = super::workload::modelled_duration(&app.state, validation.forecast.task_count);
    let status = if composed == running {
        format!(
            "{composed} point{} · {} task{} · {duration}",
            if composed == 1 { "" } else { "s" },
            validation.forecast.task_count,
            if validation.forecast.task_count == 1 {
                ""
            } else {
                "s"
            }
        )
    } else {
        format!(
            "{running} of {composed} points · {} task{} · {duration}",
            validation.forecast.task_count,
            if validation.forecast.task_count == 1 {
                ""
            } else {
                "s"
            }
        )
    };

    // Resolved once for the whole table rather than per row: the answer is
    // the same for every point, and a per-row manifest build would cost a
    // full family expansion on every frame of a large space.
    let family = family_target(app);
    // The family the Family cell names, resolved with the target so the cell
    // says which retained analysis it opens rather than repeating one word
    // twenty-seven times.
    let family_label = family.ok().and_then(|index| {
        super::output_evidence::selected_plan_dataset(app)
            .and_then(|run| run.analyses.get(index))
            .map(|analysis| analysis.label.clone())
    });
    // The participation cell holds two digits and a slash at most, so it takes
    // its width from the axes rather than from the two columns that hold
    // controls — squeezing those would truncate a button label, and an axis
    // value elides legibly.
    let mut fractions = Vec::with_capacity(axes.len() + 5);
    fractions.push(0.08);
    if excludable {
        fractions.push(0.07);
    }
    let axis_share = (if excludable { 0.50 } else { 0.57 }) / axes.len().max(1) as f32;
    for _ in &axes {
        fractions.push(axis_share);
    }
    fractions.push(0.09);
    fractions.push(0.13);
    fractions.push(0.13);
    let mut headers: Vec<String> = Vec::with_capacity(fractions.len());
    headers.push("Point".to_owned());
    if excludable {
        headers.push("Run".to_owned());
    }
    for axis in &axes {
        headers.push(axis.name.clone());
    }
    headers.push("At".to_owned());
    headers.push("Tasks".to_owned());
    headers.push("Family".to_owned());
    let header_refs: Vec<&str> = headers.iter().map(String::as_str).collect();
    // Resolved once for the table, like the family target above: a per-row
    // resolution would re-expand the declared space on every frame.
    let participation = super::participation::PlanParticipation::resolve(&app.state);
    // What a point costs is the workload's arithmetic, not the plan's instance
    // count: an instance that does not run here contributes nothing, and one
    // that mints two tasks per point contributes two. Resolved once for the
    // table and folded per row.
    let workload = super::workload::PlanWorkload::resolve(app).ok();
    let enabled_analyses = participation.instances.len();
    let mut action: Option<RunSetAction> = None;
    let mut family_request: Option<Vec<(String, String, String)>> = None;

    card(
        ui,
        "Resolved point table",
        Some((
            status.as_str(),
            if validation.is_ready() {
                Tone::Ok
            } else {
                Tone::Neutral
            },
        )),
        |ui| {
            ledger_head(ui, &fractions, &header_refs);
            for (index, point, key, is_excluded) in &rows {
                let absent = participation.analyses_absent_at(key).join(" · ");
                if point_row(
                    ui,
                    PointRow {
                        index: *index,
                        point,
                        key,
                        excluded: *is_excluded,
                        excludable,
                        axes: &axes,
                        fractions: &fractions,
                        tasks: workload
                            .as_ref()
                            .map(|workload| workload.tasks_at_point(&participation, key)),
                        participation: (participation.analyses_at(key), enabled_analyses),
                        absent: &absent,
                        family_block: family.err(),
                        family_label: family_label.as_deref(),
                    },
                    &mut action,
                ) {
                    family_request = Some(
                        point
                            .coordinates
                            .iter()
                            .map(|(dimension, value)| {
                                (
                                    dimension.id.clone(),
                                    dimension.name.clone(),
                                    value.lexical.clone(),
                                )
                            })
                            .collect(),
                    );
                }
            }
            card_note(ui, &point_table_note(composed, rows.len(), excludable));
        },
    );

    if let Some(action) = action {
        commit(app, action);
    }
    if let Some(coordinates) = family_request
        && let Ok(analysis_index) = family
    {
        open_point_in_family(app, analysis_index, &coordinates);
    }
}

/// The analysis in this plan's retained dataset that holds a retained family,
/// or why no point on this page can be opened in one.
///
/// Only the metadata's presence is checked, not the expanded manifest: this
/// runs every frame the page draws, and expanding a thousand-trial Monte Carlo
/// family to answer "is there one" would be a per-frame cost paid for nothing.
pub(super) fn family_target(app: &RSpiceApp) -> Result<usize, &'static str> {
    let run = super::output_evidence::selected_plan_dataset(app).ok_or_else(|| {
        if app.state.simulation.active_run().is_some() {
            "The active dataset was not produced by this plan, so its family does not describe \
             these points."
        } else {
            "No run has been retained, so there is no family to open a point in."
        }
    })?;
    // The active analysis first: the family dock renders whichever analysis is
    // selected, so preferring it keeps the hop on what the reader was already
    // looking at when that analysis has a family of its own.
    let active = app
        .state
        .simulation
        .active_analysis_idx
        .filter(|index| {
            run.analyses
                .get(*index)
                .is_some_and(|analysis| analysis.family_metadata.is_some())
        })
        .or_else(|| {
            run.analyses
                .iter()
                .position(|analysis| analysis.family_metadata.is_some())
        });
    active.ok_or(
        "The retained dataset holds no analysis with a family, so there are no members to slice.",
    )
}

/// Land in the family view narrowed to one declared point.
///
/// The family view's initial selection is its typed filter, so the point's
/// coordinates are compiled into one, clause per axis the retained family
/// actually declares. Coordinates the family does not know about are dropped
/// rather than made into a clause that fails to compile — and if nothing is
/// left, the hop refuses instead of landing on the whole family and calling
/// that the point.
pub(super) fn open_point_in_family(
    app: &mut RSpiceApp,
    analysis_index: usize,
    coordinates: &[(String, String, String)],
) {
    let Some(query) = family_query_for_point(&mut app.state, analysis_index, coordinates) else {
        return;
    };
    if !app.state.simulation.select_analysis(analysis_index) {
        app.state.push_user_message(ConsoleMessage::warning(
            "The retained analysis that holds the family could not be selected, so the family \
             view was left unchanged.",
        ));
        return;
    }
    // Results first, so the studio tool opens somewhere the reader can see it
    // rather than arming a dock behind this page. The filter is written last
    // because opening the slicing dock initializes it from the pane's own
    // policy — a carried point has to survive that initialization, not race
    // it.
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    Command::FamilySlicing.execute(app);
    app.state.workbench.visualization_studio.family_query = query;
}

/// Compile one point into the family view's filter, or say why it cannot be.
pub(super) fn family_query_for_point(
    state: &mut AppState,
    analysis_index: usize,
    coordinates: &[(String, String, String)],
) -> Option<String> {
    let manifest = state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
        .and_then(|plan_id| state.simulation.active_run_for_plan(plan_id))
        .and_then(|run| run.analyses.get(analysis_index))
        .and_then(|analysis| {
            crate::workbench::documents::visualization_family::FamilyManifest::from_analysis(
                analysis,
            )
            .ok()
            .flatten()
        });
    let Some(manifest) = manifest else {
        state.push_user_message(ConsoleMessage::warning(
            "The retained analysis no longer expands to a family manifest, so this point could \
             not be opened in the family view.",
        ));
        return None;
    };
    let clauses: Vec<String> = coordinates
        .iter()
        .filter_map(|(id, name, lexical)| {
            let dimension = manifest
                .dimension(id)
                .or_else(|| manifest.dimension(name))?;
            Some(format!("{} = {lexical}", dimension.id))
        })
        .collect();
    if clauses.is_empty() {
        state.push_user_message(ConsoleMessage::warning(
            "The retained family declares none of this point's axes, so there is no member to \
             open. The dataset was produced from a different declared space.",
        ));
        return None;
    }
    let query = clauses.join(" · ");
    match manifest.matching_source_indices(&query) {
        Ok(indices) if indices.is_empty() => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "The retained family holds no member at {query}, so nothing was opened."
            )));
            None
        }
        Ok(_) => Some(query),
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "This point could not be expressed against the retained family: {error}"
            )));
            None
        }
    }
}

/// Everything one point row draws from.
struct PointRow<'a> {
    index: usize,
    point: &'a run_set::RunSetPoint<'a>,
    key: &'a str,
    excluded: bool,
    excludable: bool,
    axes: &'a [&'a RunSetDimension],
    fractions: &'a [f32],
    /// What this point costs the queue: every enabled instance that visits it,
    /// at its own task rate. `None` where the plan's workload does not resolve
    /// at all, which is a plan with no queue to price rather than a free point.
    tasks: Option<usize>,
    /// How many enabled analyses declare themselves at this point, of how many
    /// the plan enables. Equal counts are drawn as the bare total: a column of
    /// `6/6` says nothing a heading does not.
    participation: (usize, usize),
    /// The kinds that do not run here, for the cell's hover. A count that
    /// reports a shortfall without naming it tells the reader there is a
    /// problem and not where.
    absent: &'a str,
    /// Why this point cannot be opened in the family view, when it cannot.
    /// The same answer for every row, resolved once by the table.
    family_block: Option<&'static str>,
    /// What the family cell names when the point can be opened: the retained
    /// analysis the family view will land in. Also resolved once by the table.
    family_label: Option<&'a str>,
}

/// One composed point, the control that keeps or removes it, and the way out
/// to the retained family member it names.
///
/// Returns whether the reader asked for that member: the row is drawn from a
/// borrow of the composed space, and opening a family mutates the session.
fn point_row(ui: &mut Ui, row: PointRow<'_>, action: &mut Option<RunSetAction>) -> bool {
    let mut cells: Vec<String> = Vec::with_capacity(row.fractions.len());
    cells.push(format!("{:03}", row.index + 1));
    if row.excludable {
        cells.push(String::new());
    }
    for axis in row.axes {
        cells.push(
            row.point
                .coordinates
                .iter()
                .find(|(dimension, _)| dimension.id == axis.id)
                .map(|(_, value)| value.lexical.clone())
                .unwrap_or_else(|| "—".to_owned()),
        );
    }
    let (visiting, enabled) = row.participation;
    cells.push(if row.excluded {
        "—".to_owned()
    } else if visiting == enabled {
        visiting.to_string()
    } else {
        format!("{visiting}/{enabled}")
    });
    cells.push(if row.excluded {
        "excluded".to_owned()
    } else {
        row.tasks
            .map_or_else(|| "unpriced".to_owned(), |tasks| tasks.to_string())
    });
    // The family cell holds a control, not text.
    cells.push(String::new());

    // Only the tasks cell carries the exclusion's colour. The coordinates are
    // still what the point is; recolouring them would read as an invalid value
    // rather than a point that is not being run.
    let tasks_cell = cells.len() - 2;
    let participation_cell = cells.len() - 3;
    let painted: Vec<(&str, Tone)> = cells
        .iter()
        .enumerate()
        .map(|(index, cell)| {
            (
                cell.as_str(),
                if index == tasks_cell && row.excluded
                    || index == participation_cell && !row.excluded && visiting < enabled
                {
                    Tone::Warn
                } else {
                    Tone::Neutral
                },
            )
        })
        .collect();

    // Both the checkbox and the family control own the row's height, so the
    // text is painted into reserved cells rather than laid out beside them.
    let (rect, columns) = super::page_kit::ledger_row_cells(ui, row.fractions);
    let t = Tokens::get(ui.ctx());
    for (column, (text, tone)) in columns.iter().zip(&painted) {
        if text.is_empty() {
            continue;
        }
        super::page_kit::paint_text(
            ui,
            column.shrink2(vec2(CARD_PAD_X * 0.8, 0.0)),
            text,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            if *tone == Tone::Neutral {
                t.color.text_dim
            } else {
                tone.color(ui)
            },
        );
    }
    // The row's own hover is registered before the controls so each keeps its
    // tooltip: within a layer, the later widget wins the pointer.
    ui.interact(rect, ui.id().with(row.key), Sense::hover())
        .on_hover_text(row.point.label());
    if !row.excluded
        && visiting < enabled
        && let Some(cell) = columns.get(participation_cell)
    {
        ui.interact(
            *cell,
            ui.id().with((row.key, "participation")),
            Sense::hover(),
        )
        .on_hover_text(format!(
            "{visiting} of {enabled} enabled analyses declare themselves at this point. Not \
                 here: {}.",
            row.absent
        ));
    }
    if row.excludable
        && let Some(cell) = columns.get(1)
    {
        let mut child = super::page_kit::cell_ui(ui, cell.shrink2(vec2(CARD_PAD_X * 0.8, 0.0)));
        let mut included = !row.excluded;
        if child
            .add(egui::Checkbox::without_text(&mut included))
            .on_hover_text(if row.excluded {
                "Restore this point to the run"
            } else {
                "Exclude this point from the run"
            })
            .changed()
        {
            *action = Some(if included {
                RunSetAction::IncludePoint {
                    key: row.key.to_owned(),
                }
            } else {
                RunSetAction::ExcludePoint {
                    key: row.key.to_owned(),
                }
            });
        }
    }
    let Some(cell) = columns.last() else {
        return false;
    };
    // A link in the cell rather than a button per row. The answer is the same
    // for every point in the table, so twenty-seven identical "Open" buttons
    // said nothing twenty-seven times, and each one forced the row eight points
    // taller than every other ledger row in the studio. The mockup's point
    // table has no action column at all; this keeps the Family column and lets
    // it name the retained analysis it opens.
    let cell = cell.shrink2(vec2(CARD_PAD_X * 0.8, 0.0));
    let t = Tokens::get(ui.ctx());
    let openable = row.family_block.is_none();
    let (text, tone) = match (openable, row.family_label) {
        (true, Some(label)) => (label, t.color.accent),
        (true, None) => ("open", t.color.accent),
        (false, _) => ("unavailable", t.color.text_faint),
    };
    super::page_kit::paint_text(
        ui,
        cell,
        text,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        tone,
    );
    let response = ui.interact(cell, ui.id().with((row.key, "family")), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Link,
            openable,
            format!("Open {} in the family view", row.point.label()),
        )
    });
    theme::paint_focus_ring(ui, &response, cell);
    match row.family_block {
        None => response
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text("Open this point's retained family member in the family slicing view")
            .clicked(),
        Some(reason) => {
            response.on_hover_text(reason);
            false
        }
    }
}

/// What the table is and is not showing.
fn point_table_note(composed: usize, drawn: usize, excludable: bool) -> String {
    if composed > drawn {
        return format!(
            "The first {POINT_TABLE_LIMIT} of {composed} composed points are listed, together with \
             every excluded point beyond them wherever it falls. Points that are not listed are \
             executed and recorded in the run manifest; this table exists to check the \
             composition, not to enumerate the run."
        );
    }
    if excludable {
        return "Each row is one point of the declared matrix, and its Tasks cell is what that \
                point costs: every enabled analysis that runs there, at its own rate — one task \
                per point, or two where the analysis retains a spectrum or assembles its own \
                family. Clearing a row removes that point by identity: the axes keep every value \
                they declare, and the exclusion is recorded in the plan rather than applied by \
                rewriting the space."
            .to_owned();
    }
    "Each row is one execution point of the declared matrix, and its Tasks cell is what that \
     point costs: every enabled analysis that runs there, at its own rate. The run manifest \
     carries these exact identities."
        .to_owned()
}

// ------------------------------------------------------------------ commit

/// Apply a run-set command and record it as a plan-configuration change.
///
/// The composed run space decides how many points a dispatch executes, so a
/// change to it has to move the plan revision and invalidate preflight.
/// Without this an authorized preflight could be followed by a sweep change
/// and then a dispatch that ran a different space than the one checked.
///
/// An edit that removes a point some analysis is scoped to is settled here,
/// where the disappearing points can still be named — see
/// [`super::participation::reconcile_selections`]. The pre-edit space is kept
/// so a refusal restores it exactly: the space and the selections that address
/// it move together or not at all.
fn commit(app: &mut RSpiceApp, action: RunSetAction) {
    let previewing = matches!(action, RunSetAction::Preview);
    let before = app.state.sim_setup.run_set.clone();
    let kinds = enabled_analysis_kinds(app);
    let (exact_task_count, workload_error) = match exact_plan_task_count(app) {
        Ok(count) => (count, None),
        Err(error) => (None, Some(error)),
    };
    let transaction = run_set::dispatch_for_plan(
        &mut app.state.sim_setup.run_set,
        action,
        &kinds,
        exact_task_count,
        workload_error,
    );

    if !transaction.was_adopted() {
        // `was_adopted` is exactly `status == Completed`, so a transaction that
        // reaches here was blocked and its receipt states the reason.
        app.state
            .workbench
            .analysis_lifecycle_status
            .record_refusal(transaction.receipt.status_line());
        return;
    }
    if previewing {
        // A preview evaluates the declaration; it does not change it, so the
        // plan revision must not move and preflight stays valid. What it
        // produces is planning evidence: the exact space a dispatch would run.
        let forecast = transaction
            .validation
            .as_ref()
            .map(|validation| validation.forecast);
        let line = match forecast {
            Some(forecast) => format!(
                "Run-set preview · {} point{} · {} task{} · {} · {} · receipt {}",
                forecast.point_count,
                if forecast.point_count == 1 { "" } else { "s" },
                forecast.task_count,
                if forecast.task_count == 1 { "" } else { "s" },
                run_set::format_duration_ms(forecast.cost_ms),
                run_set::format_bytes(forecast.storage_bytes),
                transaction.receipt.digest,
            ),
            None => transaction.receipt.status_line(),
        };
        app.state
            .workbench
            .analysis_lifecycle_status
            .record_receipt(line);
        return;
    }

    // Judged against the edited space, then applied — the decision reads the
    // whole plan before any instance changes, so a refusal on the fourth
    // analysis cannot leave the first three already pruned.
    let prunes = match super::participation::reconcile_selections(&app.state) {
        Ok(prunes) => prunes,
        Err(reason) => {
            app.state.sim_setup.run_set = before;
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(format!(
                    "{reason} The run set is unchanged and prior datasets remain immutable."
                ));
            return;
        }
    };
    let mut pruned = Vec::with_capacity(prunes.len());
    for prune in prunes {
        if super::participation::commit_run_at(app, prune.id, prune.kept).is_ok() {
            pruned.push(prune.receipt_line);
        }
    }

    match app
        .state
        .sim_setup
        .commit_active_plan_configuration_change(format!(
            "Run set · {}",
            transaction.receipt.action
        )) {
        Ok(receipt) => {
            app.invalidate_simulation_preflight();
            let mut line = receipt.status_line();
            if !pruned.is_empty() {
                line.push_str(&format!(
                    " Point selections re-scoped: {}.",
                    pruned.join("; ")
                ));
            }
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_receipt(line);
        }
        Err(error) => {
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(error.to_string());
        }
    }
}

fn enabled_analysis_kinds(app: &RSpiceApp) -> Vec<AnalysisKind> {
    app.state
        .sim_setup
        .enabled_analysis_instances()
        .map(|instance| instance.kind())
        .collect()
}

/// The exact queue cardinality of the plan over its declared space.
///
/// A projection of [`super::workload::PlanWorkload`], which is where the
/// per-analysis arithmetic lives. The scalar and the task-rate table are the
/// same sum read at two altitudes rather than two derivations of one queue: a
/// table whose rows did not add up to the number the budget is checked against
/// would be the more convincing of the two and the wrong one.
pub(super) fn exact_plan_task_count(app: &RSpiceApp) -> Result<Option<usize>, String> {
    super::workload::PlanWorkload::resolve(app)?
        .total_tasks()
        .map(Some)
}

/// The most tasks any one point of the declared space can cost.
///
/// The ceiling is the plan with every analysis participating, which is what the
/// whole plan cost per point before participation existed. It is stated as a
/// ceiling rather than as an average because the operator budgets against the
/// worst point, and because an average over a matrix where half the analyses
/// run nominal-only describes no point that actually exists.
///
/// The rate comes from [`super::workload::instance_task_rate`] rather than from
/// a copy of its PSS arm, so the tile's bound and the point table's cells are
/// the same arithmetic read at two altitudes. The copy here knew about the
/// retained spectrum and not about a Temperature or Corner instance walking its
/// own points, so it under-stated the ceiling for exactly the plans that have
/// one worth stating.
fn task_ceiling_per_point(app: &RSpiceApp) -> Option<usize> {
    let mut ceiling = 0usize;
    for instance in app.state.sim_setup.enabled_analysis_instances() {
        let rate = super::workload::instance_task_rate(app, instance.draft())?;
        ceiling = ceiling.checked_add(rate)?;
    }
    Some(ceiling)
}

pub(super) fn plan_run_set_validation(app: &RSpiceApp) -> RunSetValidation {
    let kinds = enabled_analysis_kinds(app);
    match exact_plan_task_count(app) {
        Ok(exact_task_count) => {
            run_set::validate_for_plan(&app.state.sim_setup.run_set, &kinds, exact_task_count)
        }
        Err(error) => {
            let mut validation =
                run_set::validate_for_plan(&app.state.sim_setup.run_set, &kinds, None);
            validation.push_global_error("RUNSET-PLAN-WORKLOAD", error);
            validation
        }
    }
}

#[cfg(test)]
mod run_space_layout_tests {
    use super::{AXIS_CARD_OUTER_W, FORECAST_TILE_W, OPERATOR_TILE_W, SPACE_TERM_GAP, space_rows};

    /// What one row of the strip actually measures, from the same arithmetic
    /// the paint advances by: the first term bare, every later one preceded by
    /// its operator, and a gap between atoms.
    fn measured(row: &super::SpaceRow, dimensions: usize) -> f32 {
        let mut width = 0.0;
        for (position, term) in row.terms.iter().enumerate() {
            let term_width = if term.index < dimensions {
                AXIS_CARD_OUTER_W
            } else {
                FORECAST_TILE_W + 20.0
            };
            let atom = if term.index == 0 {
                term_width
            } else {
                OPERATOR_TILE_W + SPACE_TERM_GAP + term_width
            };
            width += if position == 0 {
                atom
            } else {
                SPACE_TERM_GAP + atom
            };
        }
        width
    }

    /// Every term appears exactly once, in order, and no row overflows the
    /// card it is packed into. The grid this replaced could not overflow --
    /// it just left holes -- so the property worth pinning is that packing
    /// by width never spills.
    #[test]
    fn the_run_space_packs_every_term_without_overflowing_its_card() {
        for available in [320.0f32, 500.0, 720.0, 980.0, 1050.0, 1600.0, 2540.0] {
            for dimensions in 0..=6usize {
                let rows = space_rows(available, dimensions);
                let mut seen = Vec::new();
                for row in &rows {
                    assert!(!row.terms.is_empty(), "no empty row at {available}");
                    let width = measured(row, dimensions);
                    assert!(
                        (row.width - width).abs() <= 0.01,
                        "row reports {} but measures {width} at {available}",
                        row.width
                    );
                    // A single term wider than the card still gets its own row;
                    // it cannot be made to fit and splitting it would hide it.
                    assert!(
                        width <= available.max(FORECAST_TILE_W + 20.0) + 0.01,
                        "row of {width} overflows {available} with {dimensions} axes"
                    );
                    seen.extend(row.terms.iter().map(|term| term.index));
                }
                assert_eq!(
                    seen,
                    (0..=dimensions).collect::<Vec<_>>(),
                    "every axis and the total, once, in order, at {available}"
                );
            }
        }
    }

    /// The defect: a two-column cap left a 2x2 island at 1600 and a 500-point
    /// island at 2560. Four axes and their total are 1214 points of terms, so
    /// any card wider than that has to hold the whole run on one row.
    #[test]
    fn a_wide_card_holds_the_whole_run_on_one_row() {
        let rows = space_rows(2540.0, 4);
        assert_eq!(rows.len(), 1, "four axes and the total fit 2540 points");
        assert_eq!(rows[0].terms.len(), 5);
    }
}
