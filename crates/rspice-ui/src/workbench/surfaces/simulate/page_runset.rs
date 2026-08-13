//! PVT, sweeps & variation.
//!
//! The page is the run space: a strip of axis cards joined by the composition
//! operator and closing on the forecast they multiply out to, over the editors
//! that declare them. Every edit is a transaction against the plan's run set —
//! it moves a working revision, leaves a receipt, and clears any frozen
//! forecast — so what the page shows and what a dispatch would execute are the
//! same declaration read twice rather than two things kept in step.

use egui::{Sense, Ui, vec2};

use crate::simulation::plan::AnalysisKind;
use crate::simulation::run_set::{
    self, InvalidValuePolicy, RunSetAction, RunSetBudgets, RunSetCompositionMode, RunSetDimension,
    RunSetDimensionKind, RunSetReceiptStatus, RunSetState, RunSetValidation,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, IconButton, mono_input, select};
use crate::workbench::RSpiceApp;
use crate::workbench::state::RunSetBudgetDrafts;

use super::page_kit::{
    CARD_PAD_X, Tone, card, card_body, card_head_row, card_note, card_row, field_pair, ledger_head,
    ledger_row, rule_row,
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

/// Approximate outside width of an axis card, including its horizontal frame
/// margins. Used only to decide whether the run-space body can hold two cards;
/// the cards themselves still own their exact layout.
const AXIS_CARD_OUTER_W: f32 = AXIS_CARD_W + 18.0;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let analyses = app.state.sim_setup.enabled_analysis_instance_count();
    let validation = run_set::validate(&app.state.sim_setup.run_set, analyses);

    toolbar(ui, app, &validation);
    if !validation.errors.is_empty() || !validation.warnings.is_empty() {
        issue_summary(ui, app, &validation);
    }
    run_space(ui, app, &validation);
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
                ui.menu_button("＋ Add dimension", |ui| {
                    if addable.is_empty() {
                        ui.label(
                            egui::RichText::new(
                                "Every axis the executor binds is already declared.",
                            )
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                        );
                    }
                    for kind in addable {
                        if ui.button(kind.default_name()).clicked() {
                            action = Some(RunSetAction::AddDimension(kind));
                            ui.close();
                        }
                    }
                });
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

    super::page_kit::card_with_head(
        ui,
        |ui| card_head_row(ui, "Run space", Some((status.as_str(), tone)), |_| {}),
        |ui| {
            card_body(ui, |ui| {
                let selected = app.state.workbench.selected_run_set_dimension.clone();
                let dimensions = app.state.sim_setup.run_set.dimensions.clone();
                let two_columns = ui.available_width() >= AXIS_CARD_OUTER_W * 2.0 + 42.0;
                let columns = if two_columns { 2 } else { 1 };
                for row_start in (0..dimensions.len()).step_by(columns) {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        let row_end = (row_start + columns).min(dimensions.len());
                        let card_count = row_end - row_start;
                        let row_width = AXIS_CARD_OUTER_W * card_count as f32
                            + if card_count > 1 { 30.0 } else { 0.0 };
                        ui.add_space(((ui.available_width() - row_width) * 0.5).max(0.0));
                        for index in row_start..row_end {
                            let dimension = &dimensions[index];
                            if index > row_start {
                                let previous = &dimensions[index - 1];
                                operator_tile(
                                    ui,
                                    mode.operator(),
                                    previous.enabled && dimension.enabled,
                                );
                            }
                            let is_selected = selected.as_deref() == Some(dimension.id.as_str());
                            match axis_card(ui, dimension, index, is_selected) {
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
                        }
                    });
                    ui.add_space(6.0);
                }
                ui.horizontal(|ui| {
                    let forecast_width = FORECAST_TILE_W + 20.0;
                    let row_width = forecast_width + if dimensions.is_empty() { 0.0 } else { 30.0 };
                    ui.add_space(((ui.available_width() - row_width) * 0.5).max(0.0));
                    if !dimensions.is_empty() {
                        operator_tile(ui, "=", validation.is_ready());
                    }
                    forecast_tile(ui, validation);
                });
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

/// One axis of the space: what it contributes and where it comes from.
fn axis_card(
    ui: &mut Ui,
    dimension: &RunSetDimension,
    index: usize,
    selected: bool,
) -> Option<AxisEvent> {
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
                    let mut enabled = dimension.enabled;
                    if ui
                        .add(egui::Checkbox::without_text(&mut enabled))
                        .on_hover_text(format!("Include {} in the run space", dimension.name))
                        .changed()
                    {
                        event = Some(AxisEvent::SetEnabled(enabled));
                    }
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
    event
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

/// The composition glyph drawn between two adjacent axes.
fn operator_tile(ui: &mut Ui, glyph: &str, active: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(18.0, 26.0), Sense::hover());
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        glyph,
        theme::mono(tokens::FS_2, FontWeight::Regular),
        if active {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
    );
}

/// What the axes multiply out to.
fn forecast_tile(ui: &mut Ui, validation: &RunSetValidation) {
    let t = Tokens::get(ui.ctx());
    let forecast = validation.forecast;
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
                    egui::RichText::new(forecast.point_count.to_string())
                        .font(theme::mono(tokens::FS_4, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                ui.label(
                    egui::RichText::new(if forecast.point_count == 1 {
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
                    "× {} task{} / point",
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
            for (label, value) in [
                ("Tasks", forecast.task_count.to_string()),
                ("Cost", run_set::format_duration_ms(forecast.cost_ms)),
                ("Storage", run_set::format_bytes(forecast.storage_bytes)),
            ] {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(label)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(value)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text),
                        );
                    });
                });
            }
        });
}

/// Statistical variation is a Monte Carlo analysis, not an axis of this space.
fn variation_strip(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let active = app
        .state
        .sim_setup
        .has_enabled_analysis_kind(AnalysisKind::MonteCarlo);
    egui::Frame::new()
        .fill(t.color.bg_panel_2)
        .inner_margin(egui::Margin {
            left: CARD_PAD_X as i8,
            right: CARD_PAD_X as i8,
            top: 7,
            bottom: 7,
        })
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
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
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(if active {
                            "Statistical variation multiplies every point by the Monte Carlo \
                             sample count; each point draws its own deterministic seed."
                        } else {
                            "Statistical variation is owned by a Monte Carlo analysis instance. \
                             None is enabled in this plan, so the matrix carries deterministic \
                             points only."
                        })
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                    )
                    .wrap(),
                );
            });
        });
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
    let status = if dimension.enabled {
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
                        RunSetDimensionKind::ProcessSection => {
                            "the library section every model resolves through"
                        }
                        RunSetDimensionKind::Supply => "the DC value of the design's supplies",
                        RunSetDimensionKind::Temperature => {
                            "the solve temperature and every model's TNOM reference"
                        }
                    },
                );

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
                let options: Vec<String> = RunSetCompositionMode::ALL
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
                    && let Some(mode) = RunSetCompositionMode::ALL.get(index).copied()
                {
                    action = Some(RunSetAction::SetComposition(
                        current_composition.with_mode(mode),
                    ));
                }
                rule_row(ui, "Contract", mode.contract());
                rule_row(
                    ui,
                    "Statistical variation",
                    if variation {
                        "enabled · a Monte Carlo analysis is in the plan"
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
                "Conditional, adaptive and nested compositions are not offered: the executor \
                 expands a declared matrix, and a mode it could not expand would be a choice that \
                 changed nothing about what runs. A filtered composition is still that matrix — it \
                 removes points the design cannot reach by naming each one, not by evaluating a \
                 predicate.",
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
    let analyses = app.state.sim_setup.enabled_analysis_instance_count();
    let validation = run_set::validate(&app.state.sim_setup.run_set, analyses);
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

const RECEIPT_COLUMNS: [f32; 4] = [0.10, 0.34, 0.20, 0.36];
/// Receipts shown. The log is evidence, not a history the page has to hold.
const RECEIPT_LIMIT: usize = 6;

fn receipts(ui: &mut Ui, app: &mut RSpiceApp) {
    let receipts = &app.state.sim_setup.run_set.receipts;
    let total = receipts.len();
    let status = if total == 0 {
        "no transaction".to_owned()
    } else {
        format!("{total} recorded")
    };
    card(
        ui,
        "Transaction receipts",
        Some((status.as_str(), Tone::Neutral)),
        |ui| {
            if total == 0 {
                card_note(
                    ui,
                    "No run-set transaction has run in this session. Editing an axis, a \
                     composition or a budget records one; so does a validate-and-preview, which \
                     creates planning evidence only and no numerical result.",
                );
                return;
            }
            ledger_head(ui, &RECEIPT_COLUMNS, &["#", "Action", "Revision", "Digest"]);
            for receipt in receipts.iter().rev().take(RECEIPT_LIMIT) {
                let tone = match receipt.status {
                    RunSetReceiptStatus::Completed => Tone::Ok,
                    RunSetReceiptStatus::Blocked => Tone::Error,
                };
                ledger_row(
                    ui,
                    &RECEIPT_COLUMNS,
                    &[
                        (receipt.sequence.to_string().as_str(), Tone::Neutral),
                        (receipt.action, tone),
                        (
                            format!("{} → {}", receipt.before_revision, receipt.after_revision)
                                .as_str(),
                            Tone::Neutral,
                        ),
                        (receipt.digest.as_str(), Tone::Neutral),
                    ],
                    false,
                );
            }
            card_note(
                ui,
                "Every prior dataset is retained unchanged. A blocked transaction leaves the \
                 declaration exactly as it was and still records that it was attempted.",
            );
        },
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
    let status = if composed == running {
        format!(
            "{composed} point{} · {} task{}",
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
            "{running} of {composed} points · {} task{}",
            validation.forecast.task_count,
            if validation.forecast.task_count == 1 {
                ""
            } else {
                "s"
            }
        )
    };

    let mut fractions = Vec::with_capacity(axes.len() + 3);
    fractions.push(0.09);
    if excludable {
        fractions.push(0.07);
    }
    let axis_share = (if excludable { 0.68 } else { 0.75 }) / axes.len().max(1) as f32;
    for _ in &axes {
        fractions.push(axis_share);
    }
    fractions.push(0.16);
    let mut headers: Vec<String> = Vec::with_capacity(fractions.len());
    headers.push("Point".to_owned());
    if excludable {
        headers.push("Run".to_owned());
    }
    for axis in &axes {
        headers.push(axis.name.clone());
    }
    headers.push("Tasks".to_owned());
    let header_refs: Vec<&str> = headers.iter().map(String::as_str).collect();
    let tasks = validation.forecast.enabled_analysis_count.to_string();
    let mut action: Option<RunSetAction> = None;

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
                point_row(
                    ui,
                    PointRow {
                        index: *index,
                        point,
                        key,
                        excluded: *is_excluded,
                        excludable,
                        axes: &axes,
                        fractions: &fractions,
                        tasks: &tasks,
                    },
                    &mut action,
                );
            }
            card_note(ui, &point_table_note(composed, rows.len(), excludable));
        },
    );

    if let Some(action) = action {
        commit(app, action);
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
    tasks: &'a str,
}

/// One composed point, and the control that keeps or removes it.
fn point_row(ui: &mut Ui, row: PointRow<'_>, action: &mut Option<RunSetAction>) {
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
    cells.push(if row.excluded {
        "excluded".to_owned()
    } else {
        row.tasks.to_owned()
    });

    // Only the trailing cell carries the exclusion's colour. The coordinates
    // are still what the point is; recolouring them would read as an invalid
    // value rather than a point that is not being run.
    let last = cells.len() - 1;
    let painted: Vec<(&str, Tone)> = cells
        .iter()
        .enumerate()
        .map(|(index, cell)| {
            (
                cell.as_str(),
                if index == last && row.excluded {
                    Tone::Warn
                } else {
                    Tone::Neutral
                },
            )
        })
        .collect();

    if !row.excludable {
        ledger_row(ui, row.fractions, &painted, false).on_hover_text(row.point.label());
        return;
    }

    // The checkbox owns the row's height, so the text is painted into the
    // reserved cells rather than laid out beside it.
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
    // The row's own hover is registered before the checkbox so the control
    // keeps its tooltip: within a layer, the later widget wins the pointer.
    ui.interact(rect, ui.id().with(row.key), Sense::hover())
        .on_hover_text(row.point.label());
    let Some(cell) = columns.get(1) else {
        return;
    };
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
        return "Each row is one point of the declared matrix, and every enabled analysis \
                contributes one task per point it runs. Clearing a row removes that point by \
                identity: the axes keep every value they declare, and the exclusion is recorded in \
                the plan rather than applied by rewriting the space."
            .to_owned();
    }
    "Each row is one execution point of the declared matrix, and every enabled analysis \
     contributes one task per point. The run manifest carries these exact identities."
        .to_owned()
}

// ------------------------------------------------------------------ commit

/// Apply a run-set command and record it as a plan-configuration change.
///
/// The composed run space decides how many points a dispatch executes, so a
/// change to it has to move the plan revision and invalidate preflight.
/// Without this an authorized preflight could be followed by a sweep change
/// and then a dispatch that ran a different space than the one checked.
fn commit(app: &mut RSpiceApp, action: RunSetAction) {
    let analyses = app.state.sim_setup.enabled_analysis_instance_count();
    let previewing = matches!(action, RunSetAction::Preview);
    let transaction = run_set::dispatch(&mut app.state.sim_setup.run_set, action, analyses.max(1));

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
                "Run-set preview · {} point{} · {} task{} · {} · receipt {}",
                forecast.point_count,
                if forecast.point_count == 1 { "" } else { "s" },
                forecast.task_count,
                if forecast.task_count == 1 { "" } else { "s" },
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

    match app
        .state
        .sim_setup
        .commit_active_plan_configuration_change(format!(
            "Run set · {}",
            transaction.receipt.action
        )) {
        Ok(receipt) => {
            app.invalidate_simulation_preflight();
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_receipt(receipt.status_line());
        }
        Err(error) => {
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(error.to_string());
        }
    }
}
