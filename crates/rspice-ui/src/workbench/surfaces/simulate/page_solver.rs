//! Solver & convergence.
//!
//! Seven cards in the order the solve happens: what counts as converged, how
//! the solve recovers when it is not, how many iterations each stage gets,
//! how time advances, how the matrix is factored, what temperature the models
//! are read at, and what the topology refuses outright — closing on the ledger
//! of the value each analysis actually resolves to.
//!
//! This page is the only editor of the plan's engine options. Its cards edit
//! [`SimSetupState::options_draft`] and are applied through the
//! plan-configuration transaction, so a numerical change produces a
//! configuration receipt and invalidates preflight.
//! Every field reaches the engine through one channel and only one: the
//! `.OPTIONS` block `SimulationOptions::to_spice_options` writes into the
//! prepared deck, which the engine then re-parses. A value that emitter does
//! not write reaches nothing, so a control added here is not wired until both
//! the emitter and the netlist parser name it.
//!
//! Device bypass is identified as a BSIMSOI-specific optimization because
//! those are the compact models that consume its configured tolerances. The
//! product policy's step bounds, transient Newton budget, and absolute pivot
//! threshold differ from the core fallbacks, so they are always emitted into
//! the prepared deck; an untouched page therefore executes exactly the values
//! its ledger reports.
//!
//! The ledger is a reader, not an editor. A per-analysis override belongs to
//! its analysis — it is that analysis's own record, and it reaches the engine as
//! a second options block in that task's deck — so it is authored in the
//! Advanced options section under the analysis form on the Analyses page. This
//! page carried a second editor over the same fact for a while, a single-row
//! one that could aim at any analysis; two editors over one record is how the
//! two surfaces came to disagree about what an override is. What is left here
//! is the plan-wide view neither form can give: every option each enabled
//! analysis resolves to, and a press on a departed row to reach the section
//! that authored it.

use std::cell::Cell;

use egui::Ui;

use crate::product::AnalysisInstanceId;
use crate::simulation::accuracy::AnalysisAccuracy;
use crate::simulation::dialog::{
    DampingStrategy, IntegrationMethod, MatrixSolver, OptionsDialogState, SimulationOptions,
};
use crate::simulation::plan::{AnalysisNumericOverride, NumericOverrideOption};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, mono_input, select};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::SimSetupState;

use super::advanced_options::REVEAL_ACTION;
use super::page_kit::{
    CARD_PAD_X, RowPress, Tone, card, card_body, card_head_row, card_note, card_row, cell_ui,
    field_pair, ledger_group, ledger_head, ledger_row, ledger_row_cells, rule_row,
};

/// What choosing each named policy means, positionally matched to
/// `SimulationOptions::PRESETS`.
///
/// The labels and the option sets belong to the options themselves, because
/// the navigator reports the active policy too and cannot reach a page. Only
/// this presentation copy lives here. `preset_intents_line_up_with_the_named_presets`
/// pins the pairing.
pub(super) const PRESET_INTENT: [&str; 4] = [
    "Exploratory · relaxed update bounds · device bypass on",
    "SPICE-compatible defaults · full continuation ladder",
    "Tight update and residual bounds · verification intent",
    "Aggressive continuation · recovers a solve that stalls",
];

/// The preset the effective options match exactly, if any.
pub(super) fn active_preset(app: &RSpiceApp) -> Option<&'static str> {
    app.state.sim_setup.options.preset_name()
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    policy_strip(ui, app);
    card_row(ui, app, convergence_criteria, continuation_ladder);
    card_row(ui, app, iteration_budgets, time_integration);
    card_row(ui, app, matrix_policy, |ui, app| {
        if temperature_reference(ui, &mut app.state.sim_setup) {
            commit_draft(app);
        }
    });
    topology_contract(ui);
    resolution_ledger(ui, app);
}

// ---------------------------------------------------------------- preset strip

/// The chooser, what the active policy means, and the apply/revert pair.
///
/// Everything packs left and nothing grows: a grown summary or a right-pinned
/// action opens a gap in the middle of the strip at wide widths.
fn policy_strip(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let active = active_preset(app);
    let summary = active
        .and_then(|label| {
            SimulationOptions::PRESETS
                .iter()
                .position(|(name, _)| *name == label)
                .and_then(|index| PRESET_INTENT.get(index))
                .map(|summary| (*summary).to_owned())
        })
        .unwrap_or_else(|| {
            "Edited from a named preset · the resolved values below are what runs".to_owned()
        });
    let pending = pending_change(app);
    let mut requested = None;
    let mut apply = false;
    let mut revert = false;

    let strip_width = ui.available_width();
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(CARD_PAD_X as i8, 8))
        .show(ui, |ui| {
            ui.set_width(strip_width - CARD_PAD_X * 2.0 - 2.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(12.0, 8.0);
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    for (index, (label, _)) in SimulationOptions::PRESETS.iter().enumerate() {
                        let tooltip = PRESET_INTENT.get(index).copied().unwrap_or_default();
                        if preset_segment(ui, label, active == Some(*label), tooltip) {
                            requested = Some(*label);
                        }
                    }
                });
                ui.label(
                    egui::RichText::new(summary)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
                match &pending {
                    PendingChange::None => {
                        ui.label(
                            egui::RichText::new("applied · frozen into the next run manifest")
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                    }
                    PendingChange::Invalid(errors) => {
                        ui.label(
                            egui::RichText::new(errors.join(" · "))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.err),
                        );
                        revert = Button::new("Revert").show(ui).clicked();
                    }
                    PendingChange::Ready(_) => {
                        ui.label(
                            egui::RichText::new("unapplied edit")
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.warn),
                        );
                        apply = Button::new("Apply").accent().show(ui).clicked();
                        revert = Button::new("Revert").show(ui).clicked();
                    }
                }
            });
        });

    if let Some(label) = requested
        && let Some((_, build)) = SimulationOptions::PRESETS
            .iter()
            .find(|(name, _)| *name == label)
    {
        let options = build();
        app.state.sim_setup.options_draft = OptionsDialogState::from_options(&options);
        apply_options(app, &options);
    }
    if apply && let PendingChange::Ready(options) = pending {
        apply_options(app, &options);
    }
    if revert {
        app.state.sim_setup.options_draft =
            OptionsDialogState::from_options(&app.state.sim_setup.options);
    }
}

fn preset_segment(ui: &mut Ui, label: &str, selected: bool, tooltip: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(
        tokens::FS_1,
        if selected {
            FontWeight::SemiBold
        } else {
            FontWeight::Medium
        },
    );
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), t.color.text);
    let size = egui::vec2((galley.size().x + 28.0).max(84.0), t.metrics.row_h + 4.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    let fill = if selected {
        t.color.accent_dim
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        t.color.bg_app
    };
    ui.painter().rect(
        rect,
        egui::CornerRadius::ZERO,
        fill,
        egui::Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(rect.left(), rect.bottom() - 2.0),
                rect.right_bottom(),
            ),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().galley(
        rect.center() - galley.size() * 0.5,
        galley,
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(tooltip).clicked()
}

/// What the draft would do if applied.
enum PendingChange {
    /// The draft resolves to exactly the effective options.
    None,
    /// The draft does not parse or does not validate.
    Invalid(Vec<String>),
    /// The draft is a valid, different option set.
    Ready(SimulationOptions),
}

fn pending_change(app: &RSpiceApp) -> PendingChange {
    let options = match app.state.sim_setup.options_draft.to_options() {
        Ok(options) => options,
        Err(errors) => return PendingChange::Invalid(errors),
    };
    if let Err(errors) = options.validate() {
        return PendingChange::Invalid(errors.into_iter().map(|e| e.to_string()).collect());
    }
    let current = serde_json::to_vec(&app.state.sim_setup.options).ok();
    let requested = serde_json::to_vec(&options).ok();
    if current.is_some() && current == requested {
        PendingChange::None
    } else {
        PendingChange::Ready(options)
    }
}

/// Apply through the plan-configuration transaction, so the change produces a
/// configuration receipt and invalidates preflight.
///
/// A refusal here is not a draft error — the draft parsed and validated, and
/// the transaction declined it — so it is announced on the plan's lifecycle
/// channel, which is the one place this page reports an unapplied edit.
fn apply_options(app: &mut RSpiceApp, options: &SimulationOptions) {
    match commit_options_transaction(app, options) {
        Ok(_) => {
            app.state.sim_setup.options_draft = OptionsDialogState::from_options(options);
        }
        Err(error) => {
            app.state
                .record_plan_refusal(format!("Solver options were not committed: {error}"));
        }
    }
}

/// Make `options` the plan's effective solver policy, atomically.
///
/// The comparison is over the serialized form rather than field by field: a
/// field added to [`SimulationOptions`] and forgotten here would make an edit
/// to it look like a no-op, and a no-op must not manufacture a plan revision.
/// Nothing is written until the plan accepts the configuration change, so a
/// refused commit leaves the effective options and the plan revision alone.
pub(super) fn commit_options_transaction(
    app: &mut RSpiceApp,
    options: &SimulationOptions,
) -> Result<bool, String> {
    options.validate().map_err(|errors| {
        errors
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join(" · ")
    })?;
    let current_bytes = serde_json::to_vec(&app.state.sim_setup.options)
        .map_err(|error| format!("Could not compare the current solver options: {error}"))?;
    let requested_bytes = serde_json::to_vec(options)
        .map_err(|error| format!("Could not compare the requested solver options: {error}"))?;
    if current_bytes == requested_bytes {
        return Ok(false);
    }
    let mut candidate = app.state.sim_setup.clone();
    candidate.commit_options(options);
    candidate.options_draft = OptionsDialogState::from_options(options);
    let receipt = candidate
        .commit_active_plan_configuration_change("Updated simulation solver options.")
        .map_err(|error| error.to_string())?;
    app.state.sim_setup = candidate;
    app.invalidate_simulation_preflight();
    app.state.record_plan_receipt(receipt.status_line());
    Ok(true)
}

// --------------------------------------------------------- convergence criteria

const CRITERIA_COLUMNS: [f32; 3] = [0.36, 0.34, 0.30];

fn convergence_criteria(ui: &mut Ui, app: &mut RSpiceApp) {
    card(
        ui,
        "Convergence criteria",
        Some(("two tests · judged per node", Tone::Neutral)),
        |ui| {
            ledger_group(ui, "Solution update · |Δx| ≤ RELTOL·|x| + floor");
            criterion_row(ui, app, "RELTOL", "relative update", |draft| {
                &mut draft.reltol
            });
            criterion_row(ui, app, "VNTOL", "voltage floor", |draft| &mut draft.vntol);
            criterion_row(ui, app, "ABSTOL", "current floor", |draft| {
                &mut draft.abstol
            });
            criterion_row(ui, app, "CHGTOL", "charge floor", |draft| &mut draft.chgtol);
            ledger_group(ui, "Equation residual · per node, tested separately");
            criterion_row(ui, app, "RESIDUAL_RELTOL", "relative residual", |draft| {
                &mut draft.residual_reltol
            });
            criterion_row(ui, app, "IABSTOL", "residual floor", |draft| {
                &mut draft.iabstol
            });
            card_note(
                ui,
                "A solve is accepted only when every node passes both tests — the update bound and \
                 its own residual bound — never by a vector norm over the whole system. Values \
                 below the engine's own floor are rejected rather than silently clamped.",
            );
        },
    );
}

fn criterion_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    symbol: &str,
    meaning: &str,
    field: fn(&mut OptionsDialogState) -> &mut String,
) {
    let t = Tokens::get(ui.ctx());
    let (_, cells) = ledger_row_cells(ui, &CRITERIA_COLUMNS);
    super::page_kit::paint_text(
        ui,
        cells[0].shrink2(egui::vec2(CARD_PAD_X * 0.8, 0.0)),
        symbol,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    super::page_kit::paint_text(
        ui,
        cells[1].shrink2(egui::vec2(CARD_PAD_X * 0.8, 0.0)),
        meaning,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let value_rect = cells[2].shrink2(egui::vec2(6.0, 4.0));
    let mut cell = cell_ui(ui, value_rect);
    let width = value_rect.width();
    let response = mono_input(
        &mut cell,
        meaning,
        field(&mut app.state.sim_setup.options_draft),
        width,
    );
    commit_on_release(app, &response);
}

// ------------------------------------------------------------ iteration budgets

const BUDGET_COLUMNS: [f32; 3] = [0.18, 0.52, 0.30];

fn iteration_budgets(ui: &mut Ui, app: &mut RSpiceApp) {
    card(
        ui,
        "Iteration budgets",
        Some(("per stage, not per task", Tone::Neutral)),
        |ui| {
            budget_row(
                ui,
                app,
                "ITL1",
                "DC · iterations per solve · a tiered analysis uses its tier's",
                |draft| &mut draft.itl1,
            );
            budget_row(
                ui,
                app,
                "ITL4",
                "transient · iterations per accepted timestep",
                |draft| &mut draft.itl4,
            );
            budget_row(
                ui,
                app,
                "TRTOL",
                "transient · truncation error the step controller will accept",
                |draft| &mut draft.trtol,
            );
            budget_row(
                ui,
                app,
                "LTE rel",
                "transient · relative truncation bound · blank keeps the engine's",
                |draft| &mut draft.transient_lte_reltol,
            );
            budget_row(
                ui,
                app,
                "LTE abs",
                "transient · absolute truncation bound · blank keeps the engine's",
                |draft| &mut draft.transient_lte_abstol,
            );
            budget_row(
                ui,
                app,
                "SEED",
                "statistical draws · agauss/gauss/unif · blank is not reproducible",
                |draft| &mut draft.statistical_seed,
            );
            card_note(
                ui,
                "A budget bounds one attempt, not the task: exhausting it hands the point to the \
                 next rung of the continuation ladder rather than failing it. Only when every \
                 enabled rung is exhausted does the point fail, with its per-node residuals \
                 retained. TRTOL scales the accepted local truncation error, so raising it buys \
                 transient run time at the cost of waveform fidelity.",
            );
        },
    );
}

fn budget_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    symbol: &str,
    meaning: &str,
    field: fn(&mut OptionsDialogState) -> &mut String,
) {
    let t = Tokens::get(ui.ctx());
    let (_, cells) = ledger_row_cells(ui, &BUDGET_COLUMNS);
    super::page_kit::paint_text(
        ui,
        cells[0].shrink2(egui::vec2(CARD_PAD_X * 0.8, 0.0)),
        symbol,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    super::page_kit::paint_text(
        ui,
        cells[1].shrink2(egui::vec2(CARD_PAD_X * 0.8, 0.0)),
        meaning,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let value_rect = cells[2].shrink2(egui::vec2(6.0, 4.0));
    let mut cell = cell_ui(ui, value_rect);
    let width = value_rect.width();
    let response = mono_input(
        &mut cell,
        meaning,
        field(&mut app.state.sim_setup.options_draft),
        width,
    );
    commit_on_release(app, &response);
}

// ----------------------------------------------------------- continuation ladder

const LADDER_CONTROL_WIDTH: f32 = 128.0;

fn continuation_ladder(ui: &mut Ui, app: &mut RSpiceApp) {
    let stages = [
        (
            "Damped Newton",
            "the first attempt · ITL1 per solve",
            LadderControl::Damping,
        ),
        (
            "Adaptive GMIN stepping",
            "shunt ramp terminates at the GMIN floor",
            LadderControl::Toggle(Stage::Gmin),
        ),
        (
            "Source stepping",
            "supplies and sources ramp 0 → 100 %",
            LadderControl::Toggle(Stage::Source),
        ),
        (
            "Pseudo-transient continuation",
            "artificial time ramp · capacitive damping",
            LadderControl::Toggle(Stage::PseudoTransient),
        ),
        (
            "Arc-length continuation",
            "follows a fold the other ramps cannot pass",
            LadderControl::Toggle(Stage::ArcLength),
        ),
        (
            "Typed failure with residual report",
            "per-node worst residuals retained",
            LadderControl::Terminal,
        ),
    ];
    let enabled_rungs = 1
        + usize::from(app.state.sim_setup.options_draft.gmin_stepping)
        + usize::from(app.state.sim_setup.options_draft.source_stepping)
        + usize::from(app.state.sim_setup.options_draft.pseudo_transient)
        + usize::from(app.state.sim_setup.options_draft.arc_length);
    let status = format!("{enabled_rungs} of 5 rungs enabled");
    card(
        ui,
        "Continuation ladder",
        Some((status.as_str(), Tone::Neutral)),
        |ui| {
            for (index, (name, contract, control)) in stages.into_iter().enumerate() {
                ladder_stage(ui, app, index + 1, name, contract, control);
            }
            card_note(
                ui,
                "Stages run in this order and only in this order; a disabled stage is skipped, \
                 never resequenced. The order and every stage parameter are frozen into the run \
                 manifest.",
            );
        },
    );
}

#[derive(Clone, Copy)]
enum Stage {
    Gmin,
    Source,
    PseudoTransient,
    ArcLength,
}

#[derive(Clone, Copy)]
enum LadderControl {
    Damping,
    Toggle(Stage),
    Terminal,
}

fn ladder_stage(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    index: usize,
    name: &str,
    contract: &str,
    control: LadderControl,
) {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.ctl_h + 16.0;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let index_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + CARD_PAD_X, rect.top()),
        egui::pos2(rect.left() + CARD_PAD_X + 18.0, rect.bottom()),
    );
    ui.painter().text(
        egui::pos2(index_rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        format!("{index:02}"),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let control_left = rect.right() - CARD_PAD_X - LADDER_CONTROL_WIDTH;
    let body = egui::Rect::from_min_max(
        egui::pos2(index_rect.right() + 8.0, rect.top()),
        egui::pos2(
            (control_left - 10.0).max(index_rect.right() + 8.0),
            rect.bottom(),
        ),
    );
    super::page_kit::paint_text(
        ui,
        egui::Rect::from_min_max(body.left_top(), egui::pos2(body.right(), rect.center().y)),
        name,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
    );
    super::page_kit::paint_text(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(body.left(), rect.center().y),
            body.right_bottom(),
        ),
        contract,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    let control_rect = egui::Rect::from_min_max(
        egui::pos2(control_left, rect.center().y - t.metrics.ctl_h * 0.5),
        egui::pos2(
            rect.right() - CARD_PAD_X,
            rect.center().y + t.metrics.ctl_h * 0.5,
        ),
    );
    let mut cell = cell_ui(ui, control_rect);
    match control {
        LadderControl::Damping => {
            let options: Vec<String> = DampingStrategy::all()
                .iter()
                .map(|strategy| strategy.display_name().to_owned())
                .collect();
            let current = app.state.sim_setup.options_draft.damping;
            let selected = options
                .get(current)
                .cloned()
                .unwrap_or_else(|| options[0].clone());
            if let Some(picked) = select(
                &mut cell,
                "simulation.solver.damping",
                "Newton damping strategy",
                &selected,
                &options,
                LADDER_CONTROL_WIDTH,
            ) {
                app.state.sim_setup.options_draft.damping = picked;
                commit_draft(app);
            }
        }
        LadderControl::Toggle(stage) => {
            let value = match stage {
                Stage::Gmin => &mut app.state.sim_setup.options_draft.gmin_stepping,
                Stage::Source => &mut app.state.sim_setup.options_draft.source_stepping,
                Stage::PseudoTransient => &mut app.state.sim_setup.options_draft.pseudo_transient,
                Stage::ArcLength => &mut app.state.sim_setup.options_draft.arc_length,
            };
            let selected = if *value { "Enabled" } else { "Skipped" }.to_owned();
            let options = vec!["Enabled".to_owned(), "Skipped".to_owned()];
            if let Some(picked) = select(
                &mut cell,
                &format!("simulation.solver.stage.{index}"),
                name,
                &selected,
                &options,
                LADDER_CONTROL_WIDTH,
            ) {
                *value = picked == 0;
                commit_draft(app);
            }
        }
        LadderControl::Terminal => {
            cell.label(
                egui::RichText::new("always")
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_faint),
            );
        }
    }
}

// ------------------------------------------------------------- time integration

fn time_integration(ui: &mut Ui, app: &mut RSpiceApp) {
    card(
        ui,
        "Time integration",
        Some(("every time-domain analysis", Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                let methods: Vec<String> = IntegrationMethod::all()
                    .iter()
                    .map(|method| method.display_name().to_owned())
                    .collect();
                let method_index = app.state.sim_setup.options_draft.method;
                let method_selected = methods
                    .get(method_index)
                    .cloned()
                    .unwrap_or_else(|| methods[0].clone());
                let mut picked_method = None;
                field_pair(
                    ui,
                    ("Integration method", &mut |ui: &mut Ui, width: f32| {
                        picked_method = select(
                            ui,
                            "simulation.solver.method",
                            "Integration method",
                            &method_selected,
                            &methods,
                            width,
                        );
                    }),
                    None,
                );
                if let Some(index) = picked_method {
                    app.state.sim_setup.options_draft.method = index;
                    commit_draft(app);
                }

                let mut min_response = None;
                let mut max_response = None;
                field_pair(
                    ui,
                    ("Minimum timestep", &mut |ui: &mut Ui, width: f32| {
                        min_response = Some(mono_input(
                            ui,
                            "Minimum timestep",
                            &mut app.state.sim_setup.options_draft.min_timestep,
                            width,
                        ));
                    }),
                    Some(("Maximum timestep", &mut |ui: &mut Ui, width: f32| {
                        max_response = Some(mono_input(
                            ui,
                            "Maximum timestep",
                            &mut app.state.sim_setup.options_draft.max_timestep,
                            width,
                        ));
                    })),
                );
                for response in [min_response, max_response].into_iter().flatten() {
                    commit_on_release(app, &response);
                }

                let mut bypass_picked = None;
                let bypass_on = app.state.sim_setup.options_draft.bypass_enabled;
                let bypass_options = vec!["Enabled".to_owned(), "Disabled".to_owned()];
                let bypass_selected = if bypass_on { "Enabled" } else { "Disabled" }.to_owned();
                field_pair(
                    ui,
                    ("BSIMSOI device bypass", &mut |ui: &mut Ui, width: f32| {
                        bypass_picked = select(
                            ui,
                            "simulation.solver.bypass",
                            "Device model bypass",
                            &bypass_selected,
                            &bypass_options,
                            width,
                        );
                    }),
                    None,
                );
                if let Some(index) = bypass_picked {
                    app.state.sim_setup.options_draft.bypass_enabled = index == 0;
                    commit_draft(app);
                }

                let mut bypass_reltol = None;
                let mut bypass_abstol = None;
                field_pair(
                    ui,
                    ("Bypass relative bound", &mut |ui: &mut Ui, width: f32| {
                        bypass_reltol = Some(mono_input(
                            ui,
                            "Bypass relative bound",
                            &mut app.state.sim_setup.options_draft.bypass_reltol,
                            width,
                        ));
                    }),
                    Some(("Bypass voltage floor", &mut |ui: &mut Ui, width: f32| {
                        bypass_abstol = Some(mono_input(
                            ui,
                            "Bypass voltage floor",
                            &mut app.state.sim_setup.options_draft.bypass_abstol,
                            width,
                        ));
                    })),
                );
                for response in [bypass_reltol, bypass_abstol].into_iter().flatten() {
                    commit_on_release(app, &response);
                }
            });
            card_note(
                ui,
                "Bypass reuses a BSIMSOI device's last linearization across a transient timestep. \
                 No other model family reads it yet, and inside BSIMSOI it reaches only an \
                 instance in the ngspice dialect, only where the previous iterate evaluated \
                 without limiting, and never inside an operating point. It also needs a \
                 linearization to reuse and a state from this timestep to compare against: a \
                 device evaluating for the first time has neither, and every timestep attempt \
                 starts with one full evaluation before any iteration can be frozen. Both \
                 bounds above gate \
                 the terminal-voltage test: the relative bound scales with the terminal voltage \
                 itself, and the voltage floor decides a terminal sitting at or near zero, where \
                 a relative bound admits nothing. Passing that test is not enough — the device \
                 freezes only if the drain and body currents its stored linearization predicts \
                 also agree with the last evaluation, within the run's own current floor \
                 (IABSTOL) rather than anything set here. Both bounds are emitted only while \
                 bypass is enabled. It is a speed/accuracy trade, not a tolerance: a run that \
                 must be compared against another should keep it off.",
            );
        },
    );
}

// -------------------------------------------------------- matrix & determinism

fn matrix_policy(ui: &mut Ui, app: &mut RSpiceApp) {
    card(
        ui,
        "Matrix, pivoting & conditioning",
        Some(("changes invalidate parity", Tone::Warn)),
        |ui| {
            card_body(ui, |ui| {
                let solvers: Vec<String> = MatrixSolver::all()
                    .iter()
                    .map(|solver| solver.display_name().to_owned())
                    .collect();
                let index = app
                    .state
                    .sim_setup
                    .options_draft
                    .solver
                    .min(solvers.len() - 1);
                let selected = solvers[index].clone();
                let mut picked = None;
                let mut gmin_response = None;
                field_pair(
                    ui,
                    ("Factorization", &mut |ui: &mut Ui, width: f32| {
                        picked = select(
                            ui,
                            "simulation.solver.matrix",
                            "Matrix factorization",
                            &selected,
                            &solvers,
                            width,
                        );
                    }),
                    Some(("GMIN floor", &mut |ui: &mut Ui, width: f32| {
                        gmin_response = Some(mono_input(
                            ui,
                            "GMIN floor",
                            &mut app.state.sim_setup.options_draft.gmin,
                            width,
                        ));
                    })),
                );
                if let Some(index) = picked {
                    app.state.sim_setup.options_draft.solver = index;
                    commit_draft(app);
                }
                if let Some(response) = gmin_response {
                    commit_on_release(app, &response);
                }

                let mut pivrel_response = None;
                let mut pivtol_response = None;
                field_pair(
                    ui,
                    (
                        "Relative pivot · PIVREL",
                        &mut |ui: &mut Ui, width: f32| {
                            pivrel_response = Some(mono_input(
                                ui,
                                "Relative pivot \u{b7} PIVREL",
                                &mut app.state.sim_setup.options_draft.pivrel,
                                width,
                            ));
                        },
                    ),
                    Some((
                        "Absolute pivot · PIVTOL",
                        &mut |ui: &mut Ui, width: f32| {
                            pivtol_response = Some(mono_input(
                                ui,
                                "Absolute pivot \u{b7} PIVTOL",
                                &mut app.state.sim_setup.options_draft.pivtol,
                                width,
                            ));
                        },
                    )),
                );
                for response in [pivrel_response, pivtol_response].into_iter().flatten() {
                    commit_on_release(app, &response);
                }
            });
            card_note(
                ui,
                "The factorization backend and the pivot thresholds decide whether two runs of the \
                 same plan produce bit-identical results. A change here is recorded as a \
                 configuration change and invalidates any parity evidence taken under the previous \
                 setting.",
            );
        },
    );
}

// --------------------------------------------------------- temperature reference

/// The two temperatures a run resolves against, and which surface owns each.
///
/// TEMP is the run set's: it is one axis of the declared run space, so a plan
/// that sweeps temperature has no single value for this page to edit and the
/// card mirrors what the reference point resolves to. TNOM is the plan's, and
/// this is its only editor: it says nothing about where the circuit runs, only
/// about how every model card without its own `TNOM=` is to be read.
///
/// Returns whether the draft was released and is ready to commit — the card
/// edits the setup it is given and does not reach the rest of the application.
fn temperature_reference(ui: &mut Ui, setup: &mut SimSetupState) -> bool {
    let reference = format!("{:.1} °C · owned by the run space", setup.options.temp);
    let mut release = false;
    card(
        ui,
        "Temperature reference",
        Some(("model parameters vs. operating point", Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(ui, "Simulation temperature · TEMP", reference.as_str());
                let mut tnom = None;
                field_pair(
                    ui,
                    (
                        "Model reference temperature · TNOM",
                        &mut |ui: &mut Ui, width: f32| {
                            tnom = Some(mono_input(
                                ui,
                                "Model reference temperature \u{b7} TNOM",
                                &mut setup.options_draft.tnom,
                                width,
                            ));
                        },
                    ),
                    None,
                );
                release = tnom.is_some_and(|response| released(&response));
            });
            card_note(
                ui,
                "TEMP is the temperature the circuit runs at; TNOM is the temperature its model \
                 parameters were extracted at. A device applies its temperature coefficients over \
                 the difference between the two, so TNOM does not warm or cool the circuit — it \
                 re-interprets every model card that does not state a TNOM of its own. Both are \
                 stated in degrees Celsius, and a model card's own TNOM always wins over this one.",
            );
        },
    );
    release
}

// -------------------------------------------------------------- topology rules

fn topology_contract(ui: &mut Ui) {
    card(
        ui,
        "Topology & conditioning contract",
        Some(("enforced at elaboration", Tone::Ok)),
        |ui| {
            card_body(ui, |ui| {
                rule_row(
                    ui,
                    "Nodes without a DC path",
                    "refused at preflight · offending nodes listed",
                );
                rule_row(
                    ui,
                    "Voltage-source loops",
                    "refused · loop membership listed",
                );
                rule_row(
                    ui,
                    "Conditioning floor",
                    "GMIN is never raised above its floor to force a solve",
                );
                rule_row(
                    ui,
                    "Node shunt",
                    "an author-sized element · satisfies the check as a real device",
                );
            });
            card_note(
                ui,
                "Topology acceptance is not configurable: a design without a DC path to every node \
                 cannot produce a defensible operating point, so preflight refuses it rather than \
                 reporting a bias a conditioning shunt invented.",
            );
        },
    );
}

// ------------------------------------------------------------ resolution ledger

const LEDGER_COLUMNS: [f32; 5] = [0.19, 0.25, 0.16, 0.18, 0.22];

/// The origin cell of a row an analysis owns rather than the plan.
const OVERRIDE_ORIGIN: &str = "analysis override";
/// The origin of a value the plan's own ceiling took back from an override.
const PLAN_TIGHTER_THAN_OVERRIDE_ORIGIN: &str = "plan preset \u{b7} tighter than the override";
/// The origin of a step ceiling nobody authored.
const STEP_TIME_ORIGIN: &str = "transient step time \u{b7} no ceiling authored";
/// The same, where the plan's ceiling is the tighter of the two.
const PLAN_TIGHTER_THAN_STEP_ORIGIN: &str = "plan preset \u{b7} tighter than the step time";

/// What a step ceiling resolves to once both bounds are applied.
///
/// A step ceiling is the one option an analysis cannot replace. The plan's
/// ceiling and the analysis's own reach the engine as two separate fields —
/// `MAXTIMESTEP` and `TIMEINT DELMAX` — and the transient clamps its step
/// against both by `min` (`rspice-core/src/engine/transient.rs:1995-2011`), so
/// the run steps at whichever is tighter. Reporting the authored value as
/// effective would state a bound the run does not honour whenever an analysis
/// asks for a looser one than the plan already allows.
///
/// Visible to the Advanced options section, which reports the same option for
/// one analysis and used to show the authored number: two surfaces disagreeing
/// about the ceiling the same run would step at.
pub(super) fn resolved_step_ceiling(authored: &str, plan_ceiling: f64) -> (String, &'static str) {
    match step_ceiling(authored, plan_ceiling) {
        Some((value, true)) => (value, PLAN_TIGHTER_THAN_OVERRIDE_ORIGIN),
        Some((value, false)) => (value, OVERRIDE_ORIGIN),
        None => (authored.to_owned(), OVERRIDE_ORIGIN),
    }
}

/// What a transient that authors no ceiling actually steps at.
///
/// `auto` does not mean "the plan's ceiling". The `.tran` line carries no
/// max-step at all for it. The ceiling is an argument of the engine call the
/// bridge makes, and `simulation/engine_bridge/transient.rs:114-119` resolves
/// that argument as `config.max_timestep.unwrap_or(config.step_time)` — so the
/// analysis's own output step time arrives as the ceiling, and the engine then
/// mins *that* with the plan's `MAXTIMESTEP` exactly as it mins an authored
/// one. The run steps at `min(step time, plan preset)`, which for the stock
/// transient is 10 ns rather than the plan's 1 ms.
///
/// `None` where the step time does not parse: such a plan has no ceiling to
/// state, and its run is refused before it acquires one.
pub(super) fn inherited_step_ceiling(
    step_time: &str,
    plan_ceiling: f64,
) -> Option<(String, &'static str)> {
    step_ceiling(step_time, plan_ceiling).map(|(value, from_plan)| {
        if from_plan {
            (value, PLAN_TIGHTER_THAN_STEP_ORIGIN)
        } else {
            (value, STEP_TIME_ORIGIN)
        }
    })
}

/// The `min` both ceilings are composed by, and which side won.
///
/// One owner for the arithmetic; the two callers above differ only in what they
/// call each side. `None` where the candidate does not parse at all.
fn step_ceiling(candidate: &str, plan_ceiling: f64) -> Option<(String, bool)> {
    let value = crate::simulation::dialog::parse_si_value(candidate).ok()?;
    if !plan_ceiling.is_finite() || plan_ceiling <= 0.0 || value <= plan_ceiling {
        return Some((candidate.trim().to_owned(), false));
    }
    Some((format_value(plan_ceiling), true))
}

/// One statement of what an analysis actually resolves to.
pub(super) struct PolicyRow {
    pub(super) analysis: String,
    pub(super) option: String,
    pub(super) preset: String,
    pub(super) effective: String,
    pub(super) origin: &'static str,
    /// The analysis this row reports an authored override of, when it reports
    /// one. A row without a target states the plan's own policy, so there is no
    /// analysis for a press to carry the reader to.
    ///
    /// The instance alone, and not the option beside it: the option is a field
    /// on that analysis's own form, always drawn, so a hop that named it would
    /// be carrying something the destination does not need.
    target: Option<AnalysisInstanceId>,
}

/// The head that sits over the resolution ledger, derived from the rows it
/// heads.
///
/// It used to count the rows this card can *remove* — authored overrides — and
/// so read "every analysis resolves to the plan policy" over a default plan
/// whose own rows said otherwise: the operating point resolves its Newton
/// budget from an accuracy tier (`50` → `150 · Balanced`) and the transient
/// steps at its own output step rather than the plan's ceiling (`1m` → `10n`).
/// Neither is authored, and neither resolves to the plan policy. A head that
/// contradicts the two rows under it is worse than no head at all.
///
/// So it counts what the ledger's own columns show: the analyses named by rows
/// whose effective value differs from the preset beside it. Analyses rather
/// than rows, because one analysis can depart on two options and a reader
/// counting names would find fewer than the head claimed.
fn resolution_summary(rows: &[PolicyRow]) -> String {
    let mut departed: Vec<&str> = rows
        .iter()
        .filter(|row| row.effective != row.preset)
        .map(|row| row.analysis.as_str())
        .collect();
    departed.sort_unstable();
    departed.dedup();
    match departed.len() {
        0 => "every analysis resolves to the plan policy".to_owned(),
        1 => "1 analysis resolves away from the plan policy".to_owned(),
        count => format!("{count} analyses resolve away from the plan policy"),
    }
}

fn resolution_ledger(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut rows = plan_policy_rows(app);
    // The overrides the page's own title promises. An analysis that carries its
    // own bound does not resolve to the plan preset, and the ledger is the only
    // place that difference is visible across the whole plan.
    rows.extend(analysis_overrides(app));
    let status = resolution_summary(&rows);

    // The press writes through a cell: `card_with_head` takes two closures and
    // neither may hold `&mut app` while the other runs.
    let picked_row = Cell::new(None::<usize>);

    super::page_kit::card_with_head(
        ui,
        |ui| {
            card_head_row(
                ui,
                "Resolved policy",
                Some((status.as_str(), Tone::Neutral)),
                |_| {},
            )
        },
        |ui| {
            ledger_head(
                ui,
                &LEDGER_COLUMNS,
                &[
                    "Analysis",
                    "Option",
                    "Preset value",
                    "Effective value",
                    "Origin",
                ],
            );
            for (index, row) in rows.iter().enumerate() {
                let response = ledger_row(
                    ui,
                    &LEDGER_COLUMNS,
                    &[
                        (row.analysis.as_str(), Tone::Neutral),
                        (row.option.as_str(), Tone::Neutral),
                        (row.preset.as_str(), Tone::Neutral),
                        (
                            row.effective.as_str(),
                            if row.target.is_some() {
                                Tone::Warn
                            } else {
                                Tone::Accent
                            },
                        ),
                        (row.origin, Tone::Neutral),
                    ],
                    false,
                    // A row with no target states the plan's own policy rather
                    // than an authored override, so there is no analysis for a
                    // press to carry the reader to.
                    if row.target.is_some() {
                        RowPress::Taken
                    } else {
                        RowPress::Ignored
                    },
                );
                // The origin is the row's longest cell and the first to elide,
                // and one of them is a whole sentence about who owns the value,
                // so the hover restates it rather than leaving a reader with
                // half. A row that hops names the hop underneath it: the press
                // leaves this page, which is not something a reader should have
                // to discover by pressing.
                let response = response.on_hover_text(if row.target.is_some() {
                    format!("{}\n\n{REVEAL_ACTION}", row.origin)
                } else {
                    row.origin.to_owned()
                });
                if response.clicked() {
                    picked_row.set(Some(index));
                }
            }
            card_note(
                ui,
                "The reference temperature is owned by the run set and mirrored here; every other \
                 preset on this page is owned by this page. An override is owned by its analysis \
                 and authored in the Advanced options section under that analysis's form on the \
                 Analyses page; this ledger reports where the plan is departed from, and a press \
                 on a departed row opens the section that authored it.",
            );
        },
    );

    if let Some(index) = picked_row.get()
        && let Some(instance) = rows[index].target
    {
        // The row names the analysis; the analysis's own form is where the
        // option it names is a field. Nothing else needs carrying: the field is
        // always on screen there, under the analysis's own parameters.
        super::advanced_options::reveal_in_analyses(&mut app.state.workbench, instance);
    }
}

/// What the plan states before any analysis departs from it.
pub(super) fn plan_policy_rows(app: &RSpiceApp) -> Vec<PolicyRow> {
    let options = &app.state.sim_setup.options;
    // A row scoped to time integration is a claim about a run, so it is only
    // shown when a solve that steps time is enabled — the alternative states a
    // policy for a solve that will never happen.
    let time_stepped = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .is_ok_and(|plan| {
            plan.instances()
                .iter()
                .any(|instance| instance.enabled() && instance.kind().advances_time())
        });

    let plan_row = |analysis: &str, option: NumericOverrideOption| PolicyRow {
        analysis: analysis.to_owned(),
        option: option.label().to_owned(),
        preset: plan_preset_value(option, options),
        effective: plan_preset_value(option, options),
        origin: "plan preset",
        target: None,
    };
    // ITL1 is claimed only for the solves it actually bounds. An analysis that
    // carries an accuracy tier resolves its Newton budget from that tier — see
    // the per-analysis rows below — so naming the operating point here would
    // report a number no `.op` task runs.
    let mut rows = vec![
        plan_row("Every analysis", NumericOverrideOption::Reltol),
        plan_row("Every analysis", NumericOverrideOption::ResidualReltol),
        plan_row("DC · untiered solves", NumericOverrideOption::Itl1),
    ];
    if time_stepped {
        rows.extend([
            plan_row("Time stepped", NumericOverrideOption::IntegrationMethod),
            plan_row("Time stepped", NumericOverrideOption::Itl4),
            plan_row("Time stepped", NumericOverrideOption::MaximumTimestep),
        ]);
    }
    let solver = options.solver.display_name().to_owned();
    let temperature = format!("{:.1} °C", options.temp);
    rows.extend([
        PolicyRow {
            analysis: "Every analysis".to_owned(),
            option: "Factorization".to_owned(),
            preset: solver.clone(),
            effective: solver,
            origin: "plan level",
            target: None,
        },
        PolicyRow {
            analysis: "Every analysis".to_owned(),
            option: "Reference temperature".to_owned(),
            preset: temperature.clone(),
            effective: temperature,
            origin: "run set",
            target: None,
        },
    ]);
    rows
}

/// What the plan resolves one option to, for the preset column.
///
/// This is the policy an analysis departs *from*, so it must be the value the
/// plan's own block actually resolves to — not the field that happens to share
/// the option's name.
pub(super) fn plan_preset_value(
    option: NumericOverrideOption,
    options: &SimulationOptions,
) -> String {
    use NumericOverrideOption as O;
    match option {
        O::Reltol => format_value(options.reltol),
        // The resolver reads `iabstol.or(abstol)`, so the plan's effective
        // current floor is its `iabstol` exactly when the plan states one.
        O::Abstol => format_value(plan_current_floor(options)),
        O::Vntol => format_value(options.vntol),
        O::ResidualReltol => format_value(options.residual_reltol),
        O::Gmin => format_value(options.gmin),
        O::Itl1 => options.itl1.to_string(),
        O::Itl4 => options.itl4.to_string(),
        O::GminStepping => flag_preset(options.gmin_stepping),
        O::SourceStepping => flag_preset(options.source_stepping),
        O::PseudoTransient => flag_preset(options.pseudo_transient),
        O::ArcLength => flag_preset(options.arc_length),
        O::Damping => options.damping.display_name().to_owned(),
        O::Chgtol => format_value(options.chgtol),
        O::Trtol => format_value(options.trtol),
        O::IntegrationMethod => options.method.spice_name().to_owned(),
        // The plan may state no LTE bound at all, in which case the engine's
        // own dialect default stands and the plan has no number to show.
        O::LteReltol => optional_preset(options.transient_lte_reltol),
        O::LteAbstol => optional_preset(options.transient_lte_abstol),
        O::MinTimestep => format_value(options.min_timestep),
        O::MaximumTimestep => format_value(options.max_timestep),
        O::Pivrel => format_value(options.pivrel),
        O::Pivtol => format_value(options.pivtol),
        O::Solver => options.solver.display_name().to_owned(),
        O::Bypass => flag_preset(options.bypass_enabled),
        O::BypassReltol => format_value(options.bypass_reltol),
        O::BypassAbstol => format_value(options.bypass_abstol),
        // The packages the engine reads straight off the deck. The plan states
        // no policy for any of them — there is no `SimulationOptions` field to
        // read, because an output schedule and a harmonic-balance initial
        // state belong to one analysis and not to the whole deck — so the
        // engine's own default stands until an analysis states otherwise, and
        // that is what the ledger and the hint slot say.
        O::TransientNewtonReltol
        | O::TransientNewtonAbstol
        | O::TransientNewtonUpdateBound
        | O::TransientNewtonResidualBound
        | O::TransientNewtonBudget
        | O::TransientDeviceConvergence
        | O::TransientNoxSolver
        | O::StrobeInterval
        | O::OutputTimePoints
        | O::RetainEverySignal
        | O::HbInitialState => super::advanced_options::ENGINE_ORIGIN.to_owned(),
    }
}

/// The current floor the plan's block resolves to.
fn plan_current_floor(options: &SimulationOptions) -> f64 {
    let default = SimulationOptions::default();
    if (options.iabstol - default.iabstol).abs() > 1e-20 {
        options.iabstol
    } else {
        options.abstol
    }
}

fn flag_preset(enabled: bool) -> String {
    if enabled { "on" } else { "off" }.to_owned()
}

/// A bound the plan does not state leaves the engine's own default in force,
/// which is a fact about the plan and is said rather than guessed at.
fn optional_preset(value: Option<f64>) -> String {
    value.map_or_else(|| "engine default".to_owned(), format_value)
}

/// The analyses that resolve to something other than the plan-level policy.
///
/// Only a divergence is reported: listing every analysis at its default would
/// bury the ones that actually differ, which is the only thing this section of
/// the ledger exists to show. The numeric records are enumerated from the
/// records themselves rather than from a table of options this page recognizes,
/// so an authored bound always earns a row even when the projection below has
/// nothing else to say about it.
pub(super) fn analysis_overrides(app: &RSpiceApp) -> Vec<PolicyRow> {
    use crate::simulation::plan::AnalysisDraft;

    let options = &app.state.sim_setup.options;
    let Ok(plan) = app.state.sim_setup.stable_analysis_plan() else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    for (index, instance) in plan
        .instances()
        .iter()
        .enumerate()
        .filter(|(_, instance)| instance.enabled())
    {
        let analysis = plan
            .instance_list_label(index)
            .unwrap_or_else(|| instance.display_name().to_owned());
        // Accuracy tiers are owned by their analysis's own form and are only
        // reported here. There is no plan-level tier to depart from, so the
        // preset column names the tier every analysis starts at.
        match instance.draft() {
            // One arm for both kinds that offer a tier: `OpAccuracy` and
            // `XfAccuracy` are two spellings of `AnalysisAccuracy`, and two arms
            // reading the same field were two places for the answer to drift.
            AnalysisDraft::OperatingPoint(_) | AnalysisDraft::TransferFunction(_) => {
                let accuracy = draft_accuracy_tier(instance.draft()).unwrap_or_default();
                if accuracy != AnalysisAccuracy::default() {
                    rows.push(PolicyRow {
                        analysis: analysis.clone(),
                        option: "Accuracy tier".to_owned(),
                        preset: AnalysisAccuracy::default().display_name().to_owned(),
                        effective: accuracy.display_name().to_owned(),
                        origin: OVERRIDE_ORIGIN,
                        target: None,
                    });
                }
                rows.push(tier_iteration_budget_row(&analysis, accuracy, options));
            }
            // A transient that names its own step ceiling departs from the
            // plan's, though it can only tighten it. `auto` does not mean it
            // steps at the plan's ceiling either — see
            // [`inherited_step_ceiling`], which owns that derivation — so there
            // is a row to state in both cases, and the ledger emitted none for
            // `auto`, which is where the largest gap between the reported and
            // the honoured ceiling was.
            AnalysisDraft::Transient(setup) => {
                let ceiling = setup.max_step.trim();
                let authored = !(ceiling.is_empty() || ceiling.eq_ignore_ascii_case("auto"));
                let resolved = if authored {
                    Some(resolved_step_ceiling(ceiling, options.max_timestep))
                } else {
                    inherited_step_ceiling(&setup.step, options.max_timestep)
                };
                if let Some((effective, origin)) = resolved {
                    rows.push(PolicyRow {
                        analysis: analysis.clone(),
                        option: NumericOverrideOption::MaximumTimestep.label().to_owned(),
                        preset: plan_preset_value(NumericOverrideOption::MaximumTimestep, options),
                        effective,
                        origin,
                        // Only an authored ceiling can be removed from here.
                        // An inherited one is what the analysis's own step time
                        // implies, and there is nothing on this row to clear.
                        target: authored.then(|| instance.id()),
                    });
                }
            }
            _ => {}
        }
        let Some(record) = instance.numeric_override() else {
            continue;
        };
        for (option, effective) in record.entries() {
            let (effective, origin) = if option == NumericOverrideOption::MaximumTimestep {
                resolved_step_ceiling(&effective, options.max_timestep)
            } else {
                (effective, OVERRIDE_ORIGIN)
            };
            rows.push(PolicyRow {
                analysis: analysis.clone(),
                option: option.label().to_owned(),
                preset: plan_preset_value(option, options),
                effective,
                origin,
                target: Some(instance.id()),
            });
        }
    }
    rows
}

/// What one tiered analysis's Newton budget actually resolves to.
///
/// The plan's ITL1 reaches this task's deck like any other option and is then
/// overwritten: `AccuracyPolicy::apply` assigns `max_iterations` from the tier
/// after the deck has been resolved. The preset column still names the plan's
/// value, because that is the policy this analysis departs from, and the origin
/// column carries the one sentence that says why.
fn tier_iteration_budget_row(
    analysis: &str,
    accuracy: AnalysisAccuracy,
    options: &SimulationOptions,
) -> PolicyRow {
    PolicyRow {
        analysis: analysis.to_owned(),
        option: NumericOverrideOption::Itl1.label().to_owned(),
        preset: plan_preset_value(NumericOverrideOption::Itl1, options),
        effective: tier_iteration_budget(accuracy),
        origin: NumericOverrideOption::ACCURACY_TIER_OWNS_ITERATIONS,
        target: None,
    }
}

/// The Newton budget a tier assigns, and the tier that assigned it.
///
/// Read by both surfaces that report ITL1 for a tiered analysis — this page's
/// resolution ledger and the Advanced options section, whose ITL1 row is
/// refused and therefore has to state the owner's value rather than the plan's.
pub(super) fn tier_iteration_budget(accuracy: AnalysisAccuracy) -> String {
    format!(
        "{} \u{00b7} {}",
        accuracy.solver_policy().iteration_budget,
        accuracy.display_name()
    )
}

/// The accuracy tier a draft resolves to, for the two kinds that offer one.
///
/// Projected from [`AnalysisDraft::solver_ownership`], which is what the
/// option gate asks the same question through. The tier decides which of an
/// analysis's numeric options survive the deck as well as what this ledger
/// prints, and one stored index cannot be allowed two readings of it.
pub(super) fn draft_accuracy_tier(
    draft: &crate::simulation::plan::AnalysisDraft,
) -> Option<AnalysisAccuracy> {
    draft.solver_ownership().accuracy
}

fn format_value(value: f64) -> String {
    crate::simulation::dialog::format_si_value(value)
}

// ---------------------------------------------------------- authored overrides

/// The one writer of an authored numeric record.
///
/// It lives on this page rather than beside the Advanced options section that
/// calls it because this page owns the plan-configuration transaction every
/// numerical change on it goes through, and an override is a numerical change
/// like any other: a second writer beside the section could invent a different
/// applicability rule, or skip the transaction and leave a run resolving to a
/// bound no receipt records.
pub(super) fn write_numeric_record(
    app: &mut RSpiceApp,
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
    authored: &str,
) -> Result<(), String> {
    match prepared_numeric_record(app, instance, option, authored) {
        Ok(record) => super::lifecycle::commit_numeric_override(app, instance, Some(record)),
        Err(error) => {
            // A value the record itself refuses never reaches the plan
            // transaction, so nothing downstream would announce it. Stated
            // through the funnel every refused plan command uses, which is what
            // makes one refused field one toast and one Console line.
            super::lifecycle::record_failure(&mut app.state, "Advanced option", &error);
            Err(error)
        }
    }
}

/// The record one authored value would produce, or why it is refused.
///
/// Split out so the writer above can announce a refusal exactly once: the plan
/// transaction reports its own, and this reports the ones that never reach it.
fn prepared_numeric_record(
    app: &RSpiceApp,
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
    authored: &str,
) -> Result<AnalysisNumericOverride, String> {
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let target = plan
        .instance(instance)
        .ok_or_else(|| "The selected analysis is no longer in the plan.".to_owned())?;
    let kind = target.kind();
    let ownership = target.draft().solver_ownership();
    let mut record = target.numeric_override().cloned().unwrap_or_default();
    record.set_for_instance(kind, ownership, option, authored)?;
    Ok(record)
}

// ------------------------------------------------------------------- committing

/// Whether a text edit has been let go of, rather than typed into.
///
/// Committing per character would produce a configuration receipt for each one
/// and invalidate preflight mid-word.
fn released(response: &egui::Response) -> bool {
    response.lost_focus() || (response.changed() && !response.has_focus())
}

/// Apply a text edit when it is released, not on every keystroke.
fn commit_on_release(app: &mut RSpiceApp, response: &egui::Response) {
    if released(response) {
        commit_draft(app);
    }
}

/// Apply the draft if it is valid and different; otherwise record why it is
/// not applied and leave the effective options alone.
pub(super) fn commit_draft(app: &mut RSpiceApp) {
    match pending_change(app) {
        PendingChange::None => {}
        PendingChange::Invalid(errors) => {
            // Releasing a field is an event, and the reason it was not applied
            // has to travel with it: the policy strip states the same thing,
            // but it sits at the top of a page whose fields scroll well past
            // it.
            app.state.record_plan_refusal(format!(
                "Solver options were not applied · {}",
                errors.join(" · ")
            ));
        }
        PendingChange::Ready(options) => apply_options(app, &options),
    }
}

#[cfg(test)]
mod ledger_head_tests {
    use super::{PolicyRow, analysis_overrides, plan_policy_rows, resolution_summary};
    use crate::workbench::RSpiceApp;

    /// Every row of one plan's ledger, as the card assembles them.
    fn ledger_rows(app: &RSpiceApp) -> Vec<PolicyRow> {
        let mut rows = plan_policy_rows(app);
        rows.extend(analysis_overrides(app));
        rows
    }

    /// The departures a ledger shows, spelled the way its columns spell them.
    fn departures(rows: &[PolicyRow]) -> Vec<String> {
        let mut named: Vec<String> = rows
            .iter()
            .filter(|row| row.effective != row.preset)
            .map(|row| {
                format!(
                    "{} \u{b7} {} \u{b7} {} \u{2192} {}",
                    row.analysis, row.option, row.preset, row.effective
                )
            })
            .collect();
        named.sort();
        named
    }

    /// The ledger's head agrees with the rows it heads.
    ///
    /// The default plan is the case, and it authors no override at all: the
    /// head counted the rows it can *remove*, found none, and printed "every
    /// analysis resolves to the plan policy" directly above `Transient · Step
    /// ceiling · 1m → 10n`. Add an operating point and a second row disagrees
    /// with it — `ITL1 · 50 → 150 · Balanced` — and the head still said none.
    #[test]
    fn the_ledger_head_counts_the_analyses_its_rows_show_departing() {
        let app = RSpiceApp::test_instance();
        let rows = ledger_rows(&app);

        // The shape that made the old head wrong: nothing here is removable,
        // so counting authored rows answers zero over a ledger that shows one.
        // Both halves are asserted rather than assumed.
        assert_eq!(
            rows.iter().filter(|row| row.target.is_some()).count(),
            0,
            "the default plan authors no override"
        );
        assert_eq!(
            departures(&rows),
            vec!["Transient \u{b7} Step ceiling \u{b7} 1m \u{2192} 10n".to_owned()]
        );
        assert_eq!(
            resolution_summary(&rows),
            "1 analysis resolves away from the plan policy"
        );

        let mut app = app;
        app.state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("the test instance holds a stable plan")
            .insert(crate::simulation::plan::AnalysisKind::OperatingPoint)
            .expect("an operating point joins the plan");
        let rows = ledger_rows(&app);
        assert_eq!(
            rows.iter().filter(|row| row.target.is_some()).count(),
            0,
            "an inserted operating point authors no override either"
        );
        assert_eq!(
            departures(&rows),
            vec![
                "Operating point \u{b7} Newton budget \u{b7} ITL1 \u{b7} 50 \u{2192} 150 \u{b7} Balanced"
                    .to_owned(),
                "Transient \u{b7} Step ceiling \u{b7} 1m \u{2192} 10n".to_owned(),
            ]
        );
        assert_eq!(
            resolution_summary(&rows),
            "2 analyses resolve away from the plan policy"
        );
    }

    /// A ledger whose rows all resolve to the preset says so, and one departure
    /// is spelled in the singular.
    #[test]
    fn the_ledger_head_agrees_with_its_own_verb() {
        let mut rows = ledger_rows(&RSpiceApp::test_instance());
        for row in &mut rows {
            row.effective = row.preset.clone();
        }
        assert_eq!(
            resolution_summary(&rows),
            "every analysis resolves to the plan policy"
        );

        // Two rows, one analysis: a reader counting names must find the number
        // the head states.
        let analysis = rows[0].analysis.clone();
        for row in rows.iter_mut().take(2) {
            row.analysis = analysis.clone();
            row.effective = format!("{} \u{b7} elsewhere", row.preset);
        }
        assert_eq!(
            resolution_summary(&rows),
            "1 analysis resolves away from the plan policy"
        );
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::*;

    #[test]
    fn solver_options_commit_advances_the_plan_and_invalidates_preflight() {
        let mut app = RSpiceApp::test_instance();
        let (plan_id, source_revision) = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .map(|plan| (plan.id(), plan.revision()))
            .expect("default plan");
        let topology_root = app.state.workspace.simulation_root_reference().key();
        let topology_revision = app.state.schematic.topology_version();
        let topology_closure = vec![(topology_root.to_ascii_lowercase(), topology_revision)];
        app.state.workbench.preflight.report = Some(crate::workbench::state::PreflightReport {
            project_revision: app.state.workspace.project.revision().get(),
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan_id: Some(plan_id),
            simulation_plan_revision: Some(source_revision),
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: None,
        });
        let mut options = app.state.sim_setup.options.clone();
        options.reltol *= 2.0;

        assert!(
            commit_options_transaction(&mut app, &options)
                .expect("validated options commit atomically")
        );

        assert_eq!(
            app.state
                .sim_setup
                .stable_analysis_plan()
                .expect("plan remains available")
                .revision(),
            source_revision.next().expect("revision advances")
        );
        assert_eq!(app.state.sim_setup.options.reltol, options.reltol);
        assert!(app.state.workbench.preflight.report.is_none());
    }

    #[test]
    fn unchanged_solver_options_do_not_fabricate_a_plan_revision() {
        let mut app = RSpiceApp::test_instance();
        let revision = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .revision();
        let options = app.state.sim_setup.options.clone();

        assert!(!commit_options_transaction(&mut app, &options).expect("no-op succeeds"));
        assert_eq!(
            app.state
                .sim_setup
                .stable_analysis_plan()
                .expect("plan remains available")
                .revision(),
            revision
        );
    }
}
