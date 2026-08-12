//! Solver & convergence.
//!
//! Six cards in the order the solve happens: what counts as converged, how
//! the solve recovers when it is not, how many iterations each stage gets,
//! how time advances, how the matrix is factored, and what the topology
//! refuses outright — closing on the ledger of the value each analysis
//! actually resolves to.
//!
//! The six cards edit [`SimSetupState::options_draft`] and are applied through
//! the plan-configuration transaction, so a numerical change produces a
//! configuration receipt and invalidates preflight exactly as the dialog does.
//! Every field reaches the engine through one channel and only one: the
//! `.OPTIONS` block `SimulationOptions::to_spice_options` writes into the
//! prepared deck, which the engine then re-parses. A value that emitter does
//! not write reaches nothing, so a control added here is not wired until both
//! the emitter and the netlist parser name it.
//!
//! Two things this page still states more confidently than it can. Device
//! bypass has no engine consumer at all. And each step bound is emitted only
//! when it differs from the default shown here, which is not the engine's
//! default — so an untouched project runs under the engine's bound, not this
//! one.
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
use crate::workbench::app_state::sim_setup::dialogs::options_dialog::commit_options_transaction;
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
    card_row(ui, app, matrix_policy, |ui, _| topology_contract(ui));
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
        app.state.sim_setup.options_errors.clear();
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

/// Apply through the same transaction the dialog uses, so the change produces
/// a configuration receipt and invalidates preflight.
///
/// A refusal here is not a draft error — the draft parsed and validated, and
/// the transaction declined it — so it is announced on the plan's lifecycle
/// channel rather than added to `options_errors`, which the modal dialog owns
/// for standing draft validation. Clearing that list keeps a stale dialog error
/// from outliving the value it was about.
fn apply_options(app: &mut RSpiceApp, options: &SimulationOptions) {
    match commit_options_transaction(app, options, false) {
        Ok(_) => {
            let tab = app.state.sim_setup.options_draft.active_tab;
            app.state.sim_setup.options_draft = OptionsDialogState::from_options(options);
            app.state.sim_setup.options_draft.active_tab = tab;
            app.state.sim_setup.options_errors.clear();
        }
        Err(error) => {
            app.state.sim_setup.options_errors.clear();
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(format!("Solver options were not committed: {error}"));
        }
    }
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
                "operating point · Newton iterations per solve",
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
                let mut bypass_reltol = None;
                let bypass_on = app.state.sim_setup.options_draft.bypass_enabled;
                let bypass_options = vec!["Enabled".to_owned(), "Disabled".to_owned()];
                let bypass_selected = if bypass_on { "Enabled" } else { "Disabled" }.to_owned();
                field_pair(
                    ui,
                    ("Device bypass", &mut |ui: &mut Ui, width: f32| {
                        bypass_picked = select(
                            ui,
                            "simulation.solver.bypass",
                            "Device model bypass",
                            &bypass_selected,
                            &bypass_options,
                            width,
                        );
                    }),
                    Some(("Bypass relative bound", &mut |ui: &mut Ui, width: f32| {
                        bypass_reltol = Some(mono_input(
                            ui,
                            &mut app.state.sim_setup.options_draft.bypass_reltol,
                            width,
                        ));
                    })),
                );
                if let Some(index) = bypass_picked {
                    app.state.sim_setup.options_draft.bypass_enabled = index == 0;
                    commit_draft(app);
                }
                if let Some(response) = bypass_reltol {
                    commit_on_release(app, &response);
                }
            });
            card_note(
                ui,
                "Bypass reuses a device's last linearization while its terminal voltages move less \
                 than the bound above. It is a speed/accuracy trade, not a tolerance: a run that \
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

/// One statement of what an analysis actually resolves to.
struct PolicyRow {
    analysis: String,
    option: String,
    preset: String,
    effective: String,
    origin: &'static str,
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
fn plan_policy_rows(app: &RSpiceApp) -> Vec<PolicyRow> {
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
    let mut rows = vec![
        plan_row("Every analysis", NumericOverrideOption::Reltol),
        plan_row("Every analysis", NumericOverrideOption::ResidualReltol),
        plan_row("DC · operating point", NumericOverrideOption::Itl1),
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
fn plan_preset_value(option: NumericOverrideOption, options: &SimulationOptions) -> String {
    match option {
        NumericOverrideOption::Reltol => format_value(options.reltol),
        NumericOverrideOption::Abstol => format_value(options.abstol),
        NumericOverrideOption::Vntol => format_value(options.vntol),
        NumericOverrideOption::ResidualReltol => format_value(options.residual_reltol),
        NumericOverrideOption::Itl1 => options.itl1.to_string(),
        NumericOverrideOption::Itl4 => options.itl4.to_string(),
        NumericOverrideOption::Trtol => format_value(options.trtol),
        NumericOverrideOption::IntegrationMethod => options.method.spice_name().to_owned(),
        NumericOverrideOption::MaximumTimestep => format_value(options.max_timestep),
    }
}

/// The analyses that resolve to something other than the plan-level policy.
///
/// Only a divergence is reported: listing every analysis at its default would
/// bury the ones that actually differ, which is the only thing this section of
/// the ledger exists to show. The numeric records are enumerated from the
/// records themselves rather than from a table of options this page recognizes,
/// so an authored bound always earns a row even when the projection below has
/// nothing else to say about it.
fn analysis_overrides(app: &RSpiceApp) -> Vec<PolicyRow> {
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
                if let Some(accuracy) = OpAccuracy::ALL
                    .get(setup.accuracy_idx)
                    .filter(|accuracy| **accuracy != OpAccuracy::default())
                {
                    rows.push(PolicyRow {
                        analysis: analysis.clone(),
                        option: "Accuracy tier".to_owned(),
                        preset: OpAccuracy::default().display_name().to_owned(),
                        effective: accuracy.display_name().to_owned(),
                        origin: OVERRIDE_ORIGIN,
                        target: None,
                    });
                }
            }
            AnalysisDraft::TransferFunction(setup) => {
                if let Some(accuracy) = XfAccuracy::ALL
                    .get(setup.accuracy_idx)
                    .filter(|accuracy| **accuracy != XfAccuracy::default())
                {
                    rows.push(PolicyRow {
                        analysis: analysis.clone(),
                        option: "Accuracy tier".to_owned(),
                        preset: XfAccuracy::default().display_name().to_owned(),
                        effective: accuracy.display_name().to_owned(),
                        origin: OVERRIDE_ORIGIN,
                        target: None,
                    });
                }
            }
            // A transient that names its own step ceiling stops resolving to
            // the plan's. `auto` means it does not.
            AnalysisDraft::Transient(setup) => {
                let ceiling = setup.max_step.trim();
                if !(ceiling.is_empty() || ceiling.eq_ignore_ascii_case("auto")) {
                    rows.push(PolicyRow {
                        analysis: analysis.clone(),
                        option: NumericOverrideOption::MaximumTimestep.label().to_owned(),
                        preset: plan_preset_value(NumericOverrideOption::MaximumTimestep, options),
                        effective: ceiling.to_owned(),
                        origin: OVERRIDE_ORIGIN,
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
            rows.push(PolicyRow {
                analysis: analysis.clone(),
                option: option.label().to_owned(),
                preset: plan_preset_value(option, options),
                effective,
                origin: OVERRIDE_ORIGIN,
                target: Some(OverrideTarget {
                    instance: instance.id(),
                    option,
                }),
            });
        }
    }
    rows
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

/// Apply a text edit when it is released, not on every keystroke.
///
/// Committing per character would produce a configuration receipt for each one
/// and invalidate preflight mid-word.
fn commit_on_release(app: &mut RSpiceApp, response: &egui::Response) {
    if response.lost_focus() || (response.changed() && !response.has_focus()) {
        commit_draft(app);
    }
}

/// Apply the draft if it is valid and different; otherwise record why it is
/// not applied and leave the effective options alone.
pub(super) fn commit_draft(app: &mut RSpiceApp) {
    match pending_change(app) {
        PendingChange::None => app.state.sim_setup.options_errors.clear(),
        PendingChange::Invalid(errors) => {
            // Releasing a field is an event, and the reason it was not applied
            // has to travel with it: the policy strip states the same thing,
            // but it sits at the top of a page whose fields scroll well past
            // it. The list still goes to `options_errors` because the modal
            // dialog edits this same draft and draws them beside the fields.
            app.state
                .workbench
                .analysis_lifecycle_status
                .record_refusal(format!(
                    "Solver options were not applied · {}",
                    errors.join(" · ")
                ));
            app.state.sim_setup.options_errors = errors;
        }
        PendingChange::Ready(options) => apply_options(app, &options),
    }
}
