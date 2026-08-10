//! Solver & convergence.
//!
//! Six cards in the order the solve happens: what counts as converged, how
//! the solve recovers when it is not, how many iterations each stage gets,
//! how time advances, how the matrix is factored, and what the topology
//! refuses outright — closing on the ledger of the value each analysis
//! actually resolves to.
//!
//! Every control edits [`SimSetupState::options_draft`] and is applied through
//! the plan-configuration transaction, so a numerical change produces a
//! configuration receipt and invalidates preflight exactly as the dialog does.
//! Nothing here is presentational: every field reaches
//! `SimulationOptions::resolve_simulation_config`, which is what the engine
//! runs under.

use egui::Ui;

use crate::simulation::dialog::{
    DampingStrategy, IntegrationMethod, MatrixSolver, OptionsDialogState, SimulationOptions,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, mono_input, select};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::sim_setup::dialogs::options_dialog::commit_options_transaction;

use super::page_kit::{
    CARD_PAD_X, Tone, card, card_body, card_note, card_row, cell_ui, field_pair, ledger_group,
    ledger_head, ledger_row, ledger_row_cells, rule_row,
};

/// A named policy: its label, how to build it, and what choosing it means.
type Preset = (&'static str, fn() -> SimulationOptions, &'static str);

/// Named numerical policies, in increasing cost.
const PRESETS: [Preset; 4] = [
    (
        "Fast",
        SimulationOptions::fast,
        "Exploratory · relaxed update bounds · device bypass on",
    ),
    (
        "Default",
        SimulationOptions::default,
        "SPICE-compatible defaults · full continuation ladder",
    ),
    (
        "Accurate",
        SimulationOptions::accurate,
        "Tight update and residual bounds · verification intent",
    ),
    (
        "Robust",
        SimulationOptions::robust,
        "Aggressive continuation · recovers a solve that stalls",
    ),
];

/// The preset the effective options match exactly, if any.
///
/// Reported by exact comparison rather than by remembering which chip was
/// pressed: an edit to any field leaves the preset, and saying otherwise
/// would misreport what the run is about to use.
pub(super) fn active_preset(app: &RSpiceApp) -> Option<&'static str> {
    let current = serde_json::to_vec(&app.state.sim_setup.options).ok()?;
    PRESETS.iter().find_map(|(label, build, _)| {
        serde_json::to_vec(&build())
            .ok()
            .filter(|preset| *preset == current)
            .map(|_| *label)
    })
}

pub(super) fn active_preset_label(app: &RSpiceApp) -> String {
    active_preset(app).map_or_else(|| "Custom".to_owned(), str::to_owned)
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
            PRESETS
                .iter()
                .find(|(name, _, _)| *name == label)
                .map(|(_, _, summary)| (*summary).to_owned())
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
                    for (label, _, tooltip) in PRESETS {
                        if preset_segment(ui, label, active == Some(label), tooltip) {
                            requested = Some(label);
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
        && let Some((_, build, _)) = PRESETS.iter().find(|(name, _, _)| *name == label)
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
fn apply_options(app: &mut RSpiceApp, options: &SimulationOptions) {
    match commit_options_transaction(app, options, false) {
        Ok(_) => {
            let tab = app.state.sim_setup.options_draft.active_tab;
            app.state.sim_setup.options_draft = OptionsDialogState::from_options(options);
            app.state.sim_setup.options_draft.active_tab = tab;
            app.state.sim_setup.options_errors.clear();
        }
        Err(error) => {
            app.state.sim_setup.options_errors = vec![error];
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
            card_note(
                ui,
                "A budget bounds one attempt, not the task: exhausting it hands the point to the \
                 next rung of the continuation ladder rather than failing it. Only when every \
                 enabled rung is exhausted does the point fail, with its per-node residuals \
                 retained.",
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
                let mut factor_response = None;
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
                    Some(("Timestep growth factor", &mut |ui: &mut Ui, width: f32| {
                        factor_response = Some(mono_input(
                            ui,
                            &mut app.state.sim_setup.options_draft.timestep_factor,
                            width,
                        ));
                    })),
                );
                if let Some(index) = picked_method {
                    app.state.sim_setup.options_draft.method = index;
                    commit_draft(app);
                }
                if let Some(response) = factor_response {
                    commit_on_release(app, &response);
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

const LEDGER_COLUMNS: [f32; 4] = [0.26, 0.24, 0.24, 0.26];

fn resolution_ledger(ui: &mut Ui, app: &mut RSpiceApp) {
    let options = &app.state.sim_setup.options;
    let rows: Vec<(String, String, String, &'static str)> = vec![
        (
            "Every analysis".to_owned(),
            "Update bound · RELTOL".to_owned(),
            format_value(options.reltol),
            "plan preset",
        ),
        (
            "Every analysis".to_owned(),
            "Residual bound · RESIDUAL_RELTOL".to_owned(),
            format_value(options.residual_reltol),
            "plan preset",
        ),
        (
            "DC · operating point".to_owned(),
            "Newton budget · ITL1".to_owned(),
            options.itl1.to_string(),
            "plan preset",
        ),
        (
            "Transient".to_owned(),
            "Integration method".to_owned(),
            options.method.spice_name().to_owned(),
            "plan preset",
        ),
        (
            "Transient".to_owned(),
            "Iterations per step · ITL4".to_owned(),
            options.itl4.to_string(),
            "plan preset",
        ),
        (
            "Transient".to_owned(),
            "Step ceiling".to_owned(),
            format_value(options.max_timestep),
            "plan preset",
        ),
        (
            "Every analysis".to_owned(),
            "Factorization".to_owned(),
            options.solver.display_name().to_owned(),
            "plan level",
        ),
        (
            "Every analysis".to_owned(),
            "Reference temperature".to_owned(),
            format!("{:.1} °C", options.temp),
            "run set",
        ),
    ];
    card(
        ui,
        "Resolved policy",
        Some(("what the next run manifest freezes", Tone::Neutral)),
        |ui| {
            ledger_head(
                ui,
                &LEDGER_COLUMNS,
                &["Scope", "Option", "Resolved value", "Owner"],
            );
            for (scope, option, value, owner) in &rows {
                ledger_row(
                    ui,
                    &LEDGER_COLUMNS,
                    &[
                        (scope.as_str(), Tone::Neutral),
                        (option.as_str(), Tone::Neutral),
                        (value.as_str(), Tone::Accent),
                        (*owner, Tone::Neutral),
                    ],
                    false,
                );
            }
            card_note(
                ui,
                "The reference temperature is owned by the run set and mirrored here; every other \
                 value on this page is owned by this page. An analysis without its own override \
                 resolves to exactly these values.",
            );
        },
    );
}

fn format_value(value: f64) -> String {
    crate::simulation::dialog::format_si_value(value)
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
fn commit_draft(app: &mut RSpiceApp) {
    match pending_change(app) {
        PendingChange::None => app.state.sim_setup.options_errors.clear(),
        PendingChange::Invalid(errors) => app.state.sim_setup.options_errors = errors,
        PendingChange::Ready(options) => apply_options(app, &options),
    }
}
