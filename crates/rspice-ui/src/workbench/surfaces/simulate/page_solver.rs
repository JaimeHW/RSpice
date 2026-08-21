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
//! The ledger authors the other half of the page's title. A per-analysis
//! override is not a second copy of a plan field: it is the analysis's own
//! record, committed through the same plan transaction the analysis forms use,
//! and it reaches the engine as a second options block in that task's deck.
//! Options an analysis form already owns are written back to that form's field
//! rather than duplicated here.

use std::cell::{Cell, RefCell};

use egui::Ui;

use crate::product::AnalysisInstanceId;
use crate::simulation::dialog::{
    DampingStrategy, IntegrationMethod, MatrixSolver, OptionsDialogState, SimulationOptions,
};
use crate::simulation::plan::{AnalysisKind, NumericOverrideOption};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, mono_input, select};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::SimSetupState;
use crate::workbench::state::AnalysisOverrideDraft;

use super::page_kit::{
    CARD_PAD_X, Tone, card, card_body, card_head_row, card_note, card_row, cell_ui, field_pair,
    ledger_group, ledger_head, ledger_row, ledger_row_cells, rule_row,
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
    // Below the ledger, and only when a reader asks for one analysis: the
    // ledger reports departures across the plan, and this reports every option
    // of the one analysis in hand. Stacking them puts the summary above the
    // detail it summarizes.
    super::advanced_options::panel(ui, app);
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
                .workbench
                .analysis_lifecycle_status
                .record_refusal(format!("Solver options were not committed: {error}"));
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
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_receipt(receipt.status_line());
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
                            &mut app.state.sim_setup.options_draft.min_timestep,
                            width,
                        ));
                    }),
                    Some(("Maximum timestep", &mut |ui: &mut Ui, width: f32| {
                        max_response = Some(mono_input(
                            ui,
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
                            &mut app.state.sim_setup.options_draft.bypass_reltol,
                            width,
                        ));
                    }),
                    Some(("Bypass voltage floor", &mut |ui: &mut Ui, width: f32| {
                        bypass_abstol = Some(mono_input(
                            ui,
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
                "Bypass reuses a BSIMSOI device's last linearization across a transient timestep \
                 while its terminal voltages move less than both bounds above; no other model \
                 family reads it yet. The relative bound scales with the terminal voltage itself, \
                 and the voltage floor is what decides a terminal sitting at or near zero, where \
                 a relative bound admits nothing. Both are emitted only while bypass is enabled. \
                 It is a speed/accuracy trade, not a tolerance: a run that must be compared \
                 against another should keep it off.",
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
                            tnom = Some(mono_input(ui, &mut setup.options_draft.tnom, width));
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

/// What a step ceiling resolves to once both bounds are applied.
///
/// A step ceiling is the one option an analysis cannot replace. The plan's
/// ceiling and the analysis's own reach the engine as two separate fields —
/// `MAXTIMESTEP` and `TIMEINT DELMAX` — and the transient clamps its step
/// against both, so the run steps at whichever is tighter. Reporting the
/// authored value as effective would state a bound the run does not honour
/// whenever an analysis asks for a looser one than the plan already allows.
fn resolved_step_ceiling(authored: &str, plan_ceiling: f64) -> (String, &'static str) {
    let Ok(authored_value) = crate::simulation::dialog::parse_si_value(authored) else {
        return (authored.to_owned(), OVERRIDE_ORIGIN);
    };
    if !plan_ceiling.is_finite() || plan_ceiling <= 0.0 || authored_value <= plan_ceiling {
        return (authored.to_owned(), OVERRIDE_ORIGIN);
    }
    (
        format_value(plan_ceiling),
        "plan preset · tighter than the override",
    )
}

/// One statement of what an analysis actually resolves to.
pub(super) struct PolicyRow {
    pub(super) analysis: String,
    pub(super) option: String,
    pub(super) preset: String,
    pub(super) effective: String,
    pub(super) origin: &'static str,
    /// The authored override this row reports, when it reports one. Rows
    /// without a target state the plan policy and cannot be removed from here.
    target: Option<OverrideTarget>,
}

/// An authored per-analysis option, as a row reports it.
///
/// Storage is not carried: it follows from the analysis's kind and the option,
/// and deriving it in one place keeps a row from claiming an owner the writer
/// would then disagree with.
#[derive(Clone, Copy, PartialEq, Eq)]
struct OverrideTarget {
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
}

/// Where an authored option's value lives.
///
/// A step ceiling can be typed here or on the transient's own form, and both
/// write the same field. Two editors over one fact is deliberate; two copies of
/// it would let this ledger and that form disagree about what runs.
#[derive(Clone, Copy, PartialEq, Eq)]
enum OverrideStorage {
    NumericRecord,
    TransientStepCeiling,
}

fn resolution_ledger(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut rows = plan_policy_rows(app);
    // The overrides the page's own title promises. An analysis that carries its
    // own bound does not resolve to the plan preset, and the ledger is the only
    // place that difference is visible.
    rows.extend(analysis_overrides(app));
    let editor = override_editor(app);
    let selected = editor
        .as_ref()
        .map(|editor| (editor.instance, editor.option));
    let removable = selected.is_some_and(|(instance, option)| {
        rows.iter().any(|row| {
            row.target
                .is_some_and(|target| target.instance == instance && target.option == option)
        })
    });
    let departures = rows.iter().filter(|row| row.target.is_some()).count();
    let status = if departures == 0 {
        "every analysis resolves to the plan policy".to_owned()
    } else {
        format!("{departures} authored departures from the plan policy")
    };

    // Every control writes through a cell: `card_with_head` takes two closures
    // and neither may hold `&mut app` while the other runs.
    let open = Cell::new(false);
    let close = Cell::new(false);
    let remove = Cell::new(false);
    let apply = Cell::new(false);
    let picked_row = Cell::new(None::<usize>);
    let picked_analysis = Cell::new(None::<usize>);
    let picked_option = Cell::new(None::<usize>);
    let value = RefCell::new(
        editor
            .as_ref()
            .map_or_else(String::new, |editor| editor.value.clone()),
    );

    super::page_kit::card_with_head(
        ui,
        |ui| {
            card_head_row(
                ui,
                "Resolved policy",
                Some((status.as_str(), Tone::Neutral)),
                |ui| {
                    if editor.is_some() {
                        remove.set(Button::new("Remove").enabled(removable).show(ui).clicked());
                        close.set(Button::new("Close").show(ui).clicked());
                    } else {
                        open.set(
                            Button::new("Add override…")
                                .enabled(!analysis_choices(app).is_empty())
                                .show(ui)
                                .clicked(),
                        );
                    }
                },
            );
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
                let is_selected = row
                    .target
                    .is_some_and(|target| selected == Some((target.instance, target.option)));
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
                    is_selected,
                );
                // The origin is the row's longest cell and the first to elide,
                // and one of them is a whole sentence about who owns the value.
                // The hover restates it rather than leaving a reader with half.
                let response = response.on_hover_text(row.origin);
                if response.clicked() && row.target.is_some() {
                    picked_row.set(Some(index));
                }
            }
            if let Some(editor) = editor.as_ref() {
                override_editor_row(ui, editor, &picked_analysis, &picked_option, &value, &apply);
            }
            card_note(
                ui,
                "The reference temperature is owned by the run set and mirrored here; every other \
                 preset on this page is owned by this page. An authored override reaches its \
                 analysis as a second options block in that task's own deck, so it wins over the \
                 preset for exactly the keys it names and inherits the rest.",
            );
        },
    );

    if open.get() {
        app.state.workbench.analysis_override_draft = fresh_override_draft(app);
    }
    if close.get() {
        app.state.workbench.analysis_override_draft = None;
    }
    if let Some(index) = picked_row.get()
        && let Some(target) = rows[index].target
    {
        app.state.workbench.analysis_override_draft = Some(AnalysisOverrideDraft {
            instance: target.instance,
            option: target.option,
            value: rows[index].effective.clone(),
            error: None,
        });
        return;
    }
    if let Some(draft) = app.state.workbench.analysis_override_draft.as_mut() {
        draft.value = value.into_inner();
    }
    if let Some(index) = picked_analysis.get() {
        retarget_override_draft(app, index);
    }
    if let Some(index) = picked_option.get() {
        reoption_override_draft(app, index);
    }
    if remove.get() {
        remove_authored_override(app);
    }
    if apply.get() {
        apply_override_draft(app);
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
    use crate::simulation::dialog::{OpAccuracy, XfAccuracy};
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
        let analysis = format!("#{} · {}", index + 1, instance.kind().code());
        // Accuracy tiers are owned by their analysis's own form and are only
        // reported here. There is no plan-level tier to depart from, so the
        // preset column names the tier every analysis starts at.
        match instance.draft() {
            AnalysisDraft::OperatingPoint(setup) => {
                let accuracy = OpAccuracy::ALL
                    .get(setup.accuracy_idx)
                    .copied()
                    .unwrap_or_default();
                if accuracy != OpAccuracy::default() {
                    rows.push(PolicyRow {
                        analysis: analysis.clone(),
                        option: "Accuracy tier".to_owned(),
                        preset: OpAccuracy::default().display_name().to_owned(),
                        effective: accuracy.display_name().to_owned(),
                        origin: OVERRIDE_ORIGIN,
                        target: None,
                    });
                }
                rows.push(tier_iteration_budget_row(&analysis, accuracy, options));
            }
            AnalysisDraft::TransferFunction(setup) => {
                let accuracy = XfAccuracy::ALL
                    .get(setup.accuracy_idx)
                    .copied()
                    .unwrap_or_default();
                if accuracy != XfAccuracy::default() {
                    rows.push(PolicyRow {
                        analysis: analysis.clone(),
                        option: "Accuracy tier".to_owned(),
                        preset: XfAccuracy::default().display_name().to_owned(),
                        effective: accuracy.display_name().to_owned(),
                        origin: OVERRIDE_ORIGIN,
                        target: None,
                    });
                }
                rows.push(tier_iteration_budget_row(&analysis, accuracy, options));
            }
            // A transient that names its own step ceiling departs from the
            // plan's, though it can only tighten it. `auto` means it does not.
            AnalysisDraft::Transient(setup) => {
                let ceiling = setup.max_step.trim();
                if !(ceiling.is_empty() || ceiling.eq_ignore_ascii_case("auto")) {
                    let (effective, origin) = resolved_step_ceiling(ceiling, options.max_timestep);
                    rows.push(PolicyRow {
                        analysis: analysis.clone(),
                        option: NumericOverrideOption::MaximumTimestep.label().to_owned(),
                        preset: plan_preset_value(NumericOverrideOption::MaximumTimestep, options),
                        effective,
                        origin,
                        target: Some(OverrideTarget {
                            instance: instance.id(),
                            option: NumericOverrideOption::MaximumTimestep,
                        }),
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
                target: Some(OverrideTarget {
                    instance: instance.id(),
                    option,
                }),
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
    accuracy: crate::simulation::accuracy::AnalysisAccuracy,
    options: &SimulationOptions,
) -> PolicyRow {
    PolicyRow {
        analysis: analysis.to_owned(),
        option: NumericOverrideOption::Itl1.label().to_owned(),
        preset: plan_preset_value(NumericOverrideOption::Itl1, options),
        effective: format!(
            "{} · {}",
            accuracy.solver_policy().iteration_budget,
            accuracy.display_name()
        ),
        origin: NumericOverrideOption::ACCURACY_TIER_OWNS_ITERATIONS,
        target: None,
    }
}

fn format_value(value: f64) -> String {
    crate::simulation::dialog::format_si_value(value)
}

// ------------------------------------------------------------ override authoring

/// The authoring row's resolved choices, derived fresh each frame so a plan
/// edit made elsewhere cannot leave a stale analysis or option on offer.
struct OverrideEditorModel {
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
    value: String,
    error: Option<String>,
    analyses: Vec<(AnalysisInstanceId, String)>,
    options: Vec<(NumericOverrideOption, OverrideStorage)>,
}

/// Enabled analyses, as the analysis picker offers them.
fn analysis_choices(app: &RSpiceApp) -> Vec<(AnalysisInstanceId, String)> {
    let Ok(plan) = app.state.sim_setup.stable_analysis_plan() else {
        return Vec::new();
    };
    plan.instances()
        .iter()
        .enumerate()
        .filter(|(_, instance)| instance.enabled())
        .map(|(index, instance)| {
            (
                instance.id(),
                format!("#{} · {}", index + 1, instance.kind().code()),
            )
        })
        .collect()
}

/// The options one kind may be given, and where each one is stored.
///
/// The numeric record refuses a step ceiling on a transient because the
/// transient's own form owns that field; the option is still offered here and
/// routed to that field, so the page keeps its promise without holding a second
/// copy of the value.
fn authorable_options(kind: AnalysisKind) -> Vec<(NumericOverrideOption, OverrideStorage)> {
    let mut options: Vec<_> = NumericOverrideOption::applicable_to(kind)
        .into_iter()
        .map(|option| (option, OverrideStorage::NumericRecord))
        .collect();
    if matches!(kind, AnalysisKind::Transient) {
        options.push((
            NumericOverrideOption::MaximumTimestep,
            OverrideStorage::TransientStepCeiling,
        ));
    }
    options
}

fn override_editor(app: &RSpiceApp) -> Option<OverrideEditorModel> {
    let draft = app.state.workbench.analysis_override_draft.as_ref()?;
    let plan = app.state.sim_setup.stable_analysis_plan().ok()?;
    let kind = plan.instance(draft.instance)?.kind();
    Some(OverrideEditorModel {
        instance: draft.instance,
        option: draft.option,
        value: draft.value.clone(),
        error: draft.error.clone(),
        analyses: analysis_choices(app),
        options: authorable_options(kind),
    })
}

/// A draft aimed at the first enabled analysis and its first usable option.
fn fresh_override_draft(app: &RSpiceApp) -> Option<AnalysisOverrideDraft> {
    let plan = app.state.sim_setup.stable_analysis_plan().ok()?;
    let (instance, _) = analysis_choices(app).into_iter().next()?;
    let (option, _) = authorable_options(plan.instance(instance)?.kind())
        .into_iter()
        .next()?;
    Some(AnalysisOverrideDraft {
        instance,
        option,
        value: String::new(),
        error: None,
    })
}

/// Open the typed per-analysis numerical editor for one exact plan instance.
///
/// The Analyses-page action and the Solver ledger converge here so neither
/// surface can invent a different set of authorable fields or applicability
/// rules. The draft begins with the first supported option and its exact
/// authored value when one already exists.
pub(super) fn open_for_analysis(
    app: &mut RSpiceApp,
    instance: AnalysisInstanceId,
) -> Result<(), String> {
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let target = plan
        .instance(instance)
        .ok_or_else(|| format!("Analysis instance {instance} is no longer in the plan"))?;
    let (option, storage) = authorable_options(target.kind())
        .into_iter()
        .next()
        .ok_or_else(|| {
            format!(
                "{} has no engine-supported per-analysis numerical options",
                target.kind().label()
            )
        })?;
    let value = match storage {
        OverrideStorage::NumericRecord => target
            .numeric_override()
            .and_then(|record| record.value(option))
            .unwrap_or_default(),
        OverrideStorage::TransientStepCeiling => match target.draft() {
            crate::simulation::plan::AnalysisDraft::Transient(config) => config.max_step.clone(),
            _ => String::new(),
        },
    };
    app.state.workbench.analysis_override_draft = Some(AnalysisOverrideDraft {
        instance,
        option,
        value,
        error: None,
    });
    // The same command opens the sectioned panel on the same analysis. Asking
    // for one analysis's options and being shown a single row of them is the
    // gap this panel closes, and routing both from one entry point is what
    // stops the two surfaces disagreeing about which analysis is in hand.
    super::advanced_options::open_for_analysis(app, instance);
    app.state.workbench.simulation_page = crate::workbench::state::SimulationPage::Solver;
    Ok(())
}

/// Point the draft at another analysis, keeping its option only if that
/// analysis can carry it.
fn retarget_override_draft(app: &mut RSpiceApp, index: usize) {
    let Some((instance, _)) = analysis_choices(app).into_iter().nth(index) else {
        return;
    };
    let Some(kind) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| plan.instance(instance))
        .map(|instance| instance.kind())
    else {
        return;
    };
    let options = authorable_options(kind);
    let Some(draft) = app.state.workbench.analysis_override_draft.as_mut() else {
        return;
    };
    draft.instance = instance;
    draft.error = None;
    if !options.iter().any(|(option, _)| *option == draft.option) {
        let Some((option, _)) = options.into_iter().next() else {
            return;
        };
        draft.option = option;
    }
}

fn reoption_override_draft(app: &mut RSpiceApp, index: usize) {
    let Some(kind) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .zip(app.state.workbench.analysis_override_draft.as_ref())
        .and_then(|(plan, draft)| plan.instance(draft.instance))
        .map(|instance| instance.kind())
    else {
        return;
    };
    let Some((option, _)) = authorable_options(kind).into_iter().nth(index) else {
        return;
    };
    if let Some(draft) = app.state.workbench.analysis_override_draft.as_mut() {
        draft.option = option;
        draft.error = None;
    }
}

/// Commit the draft, or leave the plan alone and report why it was refused.
fn apply_override_draft(app: &mut RSpiceApp) {
    let Some(draft) = app.state.workbench.analysis_override_draft.clone() else {
        return;
    };
    let Some(model) = override_editor(app) else {
        return;
    };
    let storage = model
        .options
        .iter()
        .find(|(option, _)| *option == draft.option)
        .map(|(_, storage)| *storage);
    let outcome = match storage {
        Some(OverrideStorage::TransientStepCeiling) => {
            write_transient_step_ceiling(app, draft.instance, draft.value.trim())
        }
        Some(OverrideStorage::NumericRecord) => {
            write_numeric_record(app, draft.instance, draft.option, draft.value.trim())
        }
        None => Err(format!(
            "This analysis cannot carry {}.",
            draft.option.key()
        )),
    };
    if let Some(draft) = app.state.workbench.analysis_override_draft.as_mut() {
        draft.error = outcome.err();
    }
}

/// Return the drafted option to the plan policy.
fn remove_authored_override(app: &mut RSpiceApp) {
    let Some(draft) = app.state.workbench.analysis_override_draft.clone() else {
        return;
    };
    let Some(model) = override_editor(app) else {
        return;
    };
    let storage = model
        .options
        .iter()
        .find(|(option, _)| *option == draft.option)
        .map(|(_, storage)| *storage);
    let outcome = match storage {
        Some(OverrideStorage::TransientStepCeiling) => {
            write_transient_step_ceiling(app, draft.instance, "auto")
        }
        _ => {
            let record = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| plan.instance(draft.instance))
                .and_then(|instance| instance.numeric_override().cloned())
                .map(|mut record| {
                    record.clear(draft.option);
                    record
                });
            match record {
                Some(record) => {
                    super::lifecycle::commit_numeric_override(app, draft.instance, Some(record))
                }
                None => Ok(()),
            }
        }
    };
    if let Some(draft) = app.state.workbench.analysis_override_draft.as_mut() {
        draft.error = outcome.err();
    }
}

fn write_numeric_record(
    app: &mut RSpiceApp,
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
    authored: &str,
) -> Result<(), String> {
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let target = plan
        .instance(instance)
        .ok_or_else(|| "The selected analysis is no longer in the plan.".to_owned())?;
    let kind = target.kind();
    let mut record = target.numeric_override().cloned().unwrap_or_default();
    record.set(kind, option, authored)?;
    super::lifecycle::commit_numeric_override(app, instance, Some(record))
}

/// Write the transient's own step ceiling through the transaction its form
/// uses, so the ledger and the form remain one editor over one field.
fn write_transient_step_ceiling(
    app: &mut RSpiceApp,
    instance: AnalysisInstanceId,
    authored: &str,
) -> Result<(), String> {
    use crate::simulation::plan::AnalysisDraft;

    if !authored.eq_ignore_ascii_case("auto") {
        let value = crate::simulation::dialog::parse_si_value(authored)
            .map_err(|error| format!("Step ceiling: {error}"))?;
        if !value.is_finite() || value <= 0.0 {
            return Err("Step ceiling must be a positive time.".to_owned());
        }
    }
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let mut draft = plan
        .instance(instance)
        .ok_or_else(|| "The selected analysis is no longer in the plan.".to_owned())?
        .draft()
        .clone();
    let AnalysisDraft::Transient(setup) = &mut draft else {
        return Err("Only a transient owns a step ceiling of its own.".to_owned());
    };
    setup.max_step = authored.to_owned();
    super::lifecycle::commit_draft(app, instance, draft);
    Ok(())
}

const OVERRIDE_CONTROL_WIDTH: f32 = 148.0;

fn override_editor_row(
    ui: &mut Ui,
    editor: &OverrideEditorModel,
    picked_analysis: &Cell<Option<usize>>,
    picked_option: &Cell<Option<usize>>,
    value: &RefCell<String>,
    apply: &Cell<bool>,
) {
    let t = Tokens::get(ui.ctx());
    card_body(ui, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let analyses: Vec<String> = editor
                .analyses
                .iter()
                .map(|(_, label)| label.clone())
                .collect();
            let selected_analysis = editor
                .analyses
                .iter()
                .position(|(id, _)| *id == editor.instance)
                .and_then(|index| analyses.get(index).cloned())
                .unwrap_or_default();
            if let Some(index) = select(
                ui,
                "simulation.solver.override.analysis",
                "Analysis to override",
                &selected_analysis,
                &analyses,
                OVERRIDE_CONTROL_WIDTH,
            ) {
                picked_analysis.set(Some(index));
            }

            let options: Vec<String> = editor
                .options
                .iter()
                .map(|(option, _)| option.label().to_owned())
                .collect();
            let selected_option = editor
                .options
                .iter()
                .position(|(option, _)| *option == editor.option)
                .and_then(|index| options.get(index).cloned())
                .unwrap_or_default();
            if let Some(index) = select(
                ui,
                "simulation.solver.override.option",
                "Option to override",
                &selected_option,
                &options,
                OVERRIDE_CONTROL_WIDTH * 1.4,
            ) {
                picked_option.set(Some(index));
            }

            let mut buffer = value.borrow_mut();
            let response = mono_input(ui, &mut buffer, OVERRIDE_CONTROL_WIDTH);
            drop(buffer);
            if Button::new("Apply").accent().show(ui).clicked()
                || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                apply.set(true);
            }
            ui.label(
                egui::RichText::new(
                    editor
                        .error
                        .clone()
                        .unwrap_or_else(|| editor.option.value_hint().to_owned()),
                )
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if editor.error.is_some() {
                    t.color.err
                } else {
                    t.color.text_faint
                }),
            );
        });
    });
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
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(format!(
                    "Solver options were not applied · {}",
                    errors.join(" · ")
                ));
        }
        PendingChange::Ready(options) => apply_options(app, &options),
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
