//! Solver & convergence.
//!
//! Compatibility and the solve policy: what counts as converged, how
//! the solve recovers when it is not, how many iterations each stage gets,
//! how time advances, how the matrix is factored and what temperature the
//! models are read at.
//!
//! This page is the only editor of the plan's engine options. Its cards edit
//! the `options_draft` in [`SimSetupState::session`], and a field commits when
//! it is let go of (Enter or focus loss) through the plan-configuration
//! transaction, so a numerical change produces a configuration receipt and
//! invalidates preflight. There is no second, deferred way to apply an edit.
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
//! it shows.

use egui::Ui;

use crate::product::AnalysisInstanceId;
use crate::simulation::accuracy::AnalysisAccuracy;
use crate::simulation::dialog::{
    DampingStrategy, IntegrationMethod, MatrixSolver, SimulationCompatibility, SimulationOptions,
};
use crate::simulation::plan::{AnalysisNumericOverride, NumericOverrideOption, OptionsDialogState};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{SegmentedWidth, mono_input, segmented, select};
use crate::workbench::app_state::design_history::SetupParts;
use crate::workbench::app_state::{Model, SimSetupState};
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, invalidate_preflight, record_plan_refusal};

mod startup;

use super::page_kit::{
    CARD_PAD_X, TWO_COLUMN_BREAKPOINT, Tone, card, card_body, card_row, cell_ui, field_pair,
    ledger_group, ledger_row_cells,
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
pub(super) fn active_preset(model: &Model) -> Option<&'static str> {
    model.sim_setup.options.preset_name()
}

pub(super) fn show(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    policy_strip(ui, cx);
    compatibility_policy(ui, cx);
    even_card_row(
        ui,
        cx,
        "criteria",
        convergence_criteria,
        continuation_ladder,
    );
    startup::show(ui, cx);
    even_card_row(ui, cx, "limits", iteration_budgets, time_integration);
    even_card_row(ui, cx, "matrix", matrix_policy, |ui, cx| {
        if temperature_reference(ui, &mut cx.model.sim_setup) {
            commit_draft(cx);
        }
    });
}

// ------------------------------------------------------------------ card rows

/// Where an [`even_card`] learns which pair it belongs to.
const EVEN_CARD: &str = "simulation.solver.even-card";

/// The two halves of one side-by-side pair: where this card keeps its natural
/// height, and where its neighbour keeps its own.
#[derive(Clone, Copy)]
struct EvenCard {
    mine: egui::Id,
    other: egui::Id,
}

/// Two cards side by side whose frames end on one line, or stacked where the
/// page is too narrow for two.
///
/// Every pair on this page differs in height by a row or more, and two frames
/// ending at different heights read as a mistake. Each half is drawn with an
/// [`even_card`], which pads its body down to the taller of the pair.
fn even_card_row<T>(
    ui: &mut Ui,
    subject: &mut T,
    row: &'static str,
    left: impl FnOnce(&mut Ui, &mut T),
    right: impl FnOnce(&mut Ui, &mut T),
) {
    if ui.available_width() < TWO_COLUMN_BREAKPOINT {
        card_row(ui, subject, left, right);
        return;
    }
    let left_id = egui::Id::new((EVEN_CARD, row, "left"));
    let right_id = egui::Id::new((EVEN_CARD, row, "right"));
    let handed = egui::Id::new(EVEN_CARD);
    card_row(
        ui,
        subject,
        |ui, subject| {
            let pair = EvenCard {
                mine: left_id,
                other: right_id,
            };
            ui.data_mut(|data| data.insert_temp(handed, pair));
            left(ui, subject);
            ui.data_mut(|data| data.remove::<EvenCard>(handed));
        },
        |ui, subject| {
            let pair = EvenCard {
                mine: right_id,
                other: left_id,
            };
            ui.data_mut(|data| data.insert_temp(handed, pair));
            right(ui, subject);
            ui.data_mut(|data| data.remove::<EvenCard>(handed));
        },
    );
}

/// A [`card`] that, drawn as half of an [`even_card_row`], pads its body to the
/// taller half's height.
///
/// The neighbour's height is the one it had on the previous frame, so a card
/// whose own height changed asks for one more frame, and the pair settles
/// before it is seen. Outside a pair it is a plain card.
fn even_card<R>(
    ui: &mut Ui,
    title: &str,
    status: Option<(&str, Tone)>,
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    let handed = egui::Id::new(EVEN_CARD);
    let pair = ui.data_mut(|data| {
        let pair = data.get_temp::<EvenCard>(handed);
        data.remove::<EvenCard>(handed);
        pair
    });
    card(ui, title, status, |ui| {
        let inner = body(ui);
        if let Some(pair) = pair {
            let natural = ui.min_rect().height();
            let (was, other) = ui.data_mut(|data| {
                let was = data.get_temp::<f32>(pair.mine);
                data.insert_temp(pair.mine, natural);
                (was, data.get_temp::<f32>(pair.other))
            });
            if was != Some(natural) {
                ui.ctx().request_repaint();
            }
            // Measured from the card's own top: `set_min_height` would count
            // from the cursor and add the neighbour's height below this card's.
            if let Some(other) = other {
                ui.expand_to_include_y(ui.min_rect().top() + natural.max(other));
            }
        }
        inner
    })
}

// ---------------------------------------------------------------- preset strip

fn compatibility_policy(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    // Only a netlist test has a source of its own to defer to; a plan drawn
    // from a schematic inherits RSpice's own numerics.
    let netlist_test = cx.model.is_netlist_first_without_schematic();
    card(ui, "Simulation compatibility", None, |ui| {
        card_body(ui, |ui| {
            let offered = SimulationCompatibility::offered(netlist_test);
            let choices: Vec<String> = offered
                .iter()
                .map(|policy| policy.label(netlist_test).to_owned())
                .collect();
            let current = cx
                .model
                .sim_setup
                .session
                .options_draft
                .compatibility
                .label(netlist_test)
                .to_owned();
            let mut picked = None;
            field_pair(
                ui,
                (
                    "Device models and numerics",
                    &mut |ui: &mut Ui, width: f32| {
                        picked = select(
                            ui,
                            "simulation.solver.compatibility",
                            "Simulation compatibility",
                            &current,
                            &choices,
                            width,
                        );
                    },
                ),
                None,
            );
            if let Some(&policy) = picked.and_then(|index| offered.get(index)) {
                cx.model.sim_setup.session.options_draft.compatibility = policy;
                commit_draft(cx);
            }
        });
    });
}

/// The named presets, and what the active one means.
///
/// The group fills the preset the plan's options match and none once they are
/// edited away from every preset. Everything packs left and nothing grows: a
/// grown summary opens a gap in the middle of the strip at wide widths.
fn policy_strip(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let t = Tokens::get(ui.ctx());
    let labels: Vec<&str> = SimulationOptions::PRESETS
        .iter()
        .map(|(label, _)| *label)
        .collect();
    let active =
        active_preset(cx.model).and_then(|label| labels.iter().position(|name| *name == label));
    let summary = active
        .and_then(|index| PRESET_INTENT.get(index))
        .map_or("Edited from a named preset", |summary| *summary);
    let mut chosen = active.unwrap_or(usize::MAX);
    let mut requested = false;

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
                requested = segmented(
                    ui,
                    "simulation.solver.preset",
                    &labels,
                    &mut chosen,
                    SegmentedWidth::Natural,
                );
                ui.label(
                    egui::RichText::new(summary)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
        });

    if requested && let Some(label) = labels.get(chosen) {
        apply_preset(cx, label);
    }
}

/// Apply the named preset over the plan's own options: its tolerances,
/// budgets and convergence aids, and nothing else.
pub(super) fn apply_preset(cx: &mut Cx<'_, SimulateFeature>, label: &str) {
    if let Some((_, build)) = SimulationOptions::PRESETS
        .iter()
        .find(|(name, _)| *name == label)
    {
        let options = cx.model.sim_setup.options.with_preset(*build);
        cx.model.sim_setup.session.options_draft = OptionsDialogState::from_options(&options);
        apply_options(cx, &options);
    }
}

/// What the draft would do if applied.
enum PendingChange {
    /// The draft resolves to exactly the effective options.
    None,
    /// The draft does not parse or does not validate.
    Invalid(Vec<String>),
    /// The draft is a valid, different option set. Boxed: the options are
    /// ten times the size of either other answer.
    Ready(Box<SimulationOptions>),
}

fn pending_change(model: &Model) -> PendingChange {
    let options = match model.sim_setup.session.options_draft.to_options() {
        Ok(options) => options,
        Err(errors) => return PendingChange::Invalid(errors),
    };
    if let Err(errors) = options.validate() {
        return PendingChange::Invalid(errors.into_iter().map(|e| e.to_string()).collect());
    }
    let current = serde_json::to_vec(&model.sim_setup.options).ok();
    let requested = serde_json::to_vec(&options).ok();
    if current.is_some() && current == requested {
        PendingChange::None
    } else {
        PendingChange::Ready(Box::new(options))
    }
}

/// Apply through the plan-configuration transaction, so the change produces a
/// configuration receipt and invalidates preflight.
///
/// A refusal here is not a draft error — the draft parsed and validated, and
/// the transaction declined it — so it is announced on the plan's lifecycle
/// channel, which is where this page reports a value the plan refused.
fn apply_options(cx: &mut Cx<'_, SimulateFeature>, options: &SimulationOptions) {
    match commit_options_transaction(cx, options) {
        Ok(_) => {
            cx.model.sim_setup.session.options_draft = OptionsDialogState::from_options(options);
        }
        Err(error) => {
            record_plan_refusal(cx, format!("Solver options were not committed: {error}"));
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
    cx: &mut Cx<'_, SimulateFeature>,
    options: &SimulationOptions,
) -> Result<bool, String> {
    options.validate().map_err(|errors| {
        errors
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join(" · ")
    })?;
    let current_bytes = serde_json::to_vec(&cx.model.sim_setup.options)
        .map_err(|error| format!("Could not compare the current solver options: {error}"))?;
    let requested_bytes = serde_json::to_vec(options)
        .map_err(|error| format!("Could not compare the requested solver options: {error}"))?;
    if current_bytes == requested_bytes {
        return Ok(false);
    }
    let mut candidate = cx.model.sim_setup.clone();
    candidate.commit_options(options);
    candidate.session.options_draft = OptionsDialogState::from_options(options);
    let receipt = candidate
        .commit_active_plan_configuration_change("Updated simulation solver options.")
        .map_err(|error| error.to_string())?;
    let before = cx.model.setup_snapshot(SetupParts::OPTIONS);
    cx.model.sim_setup = candidate;
    // One step of the setup's history, named for the page that took it.
    cx.model
        .record_setup_change(cx.state.simulation_page.label(), before);
    invalidate_preflight(cx);
    cx.model.record_plan_receipt(receipt.status_line());
    Ok(true)
}

// --------------------------------------------------------- convergence criteria

const CRITERIA_COLUMNS: [f32; 3] = [0.36, 0.34, 0.30];

fn convergence_criteria(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    even_card(
        ui,
        "Convergence criteria",
        Some(("two tests · judged per node", Tone::Neutral)),
        |ui| {
            ledger_group(ui, "Solution update · |Δx| ≤ RELTOL·|x| + floor");
            criterion_row(ui, cx, "RELTOL", "relative update", |draft| {
                &mut draft.reltol
            });
            criterion_row(ui, cx, "VNTOL", "voltage floor", |draft| &mut draft.vntol);
            criterion_row(ui, cx, "IABSTOL", "current tolerance", |draft| {
                &mut draft.iabstol
            });
            criterion_row(ui, cx, "CHGTOL", "charge floor", |draft| &mut draft.chgtol);
            ledger_group(ui, "Equation residual · per node, tested separately");
            criterion_row(ui, cx, "RESIDUAL_RELTOL", "relative residual", |draft| {
                &mut draft.residual_reltol
            });
        },
    );
}

fn criterion_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
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
        field(&mut cx.model.sim_setup.session.options_draft),
        width,
    );
    commit_on_release(cx, &response);
}

// ------------------------------------------------------------ iteration budgets

const BUDGET_COLUMNS: [f32; 3] = [0.18, 0.52, 0.30];

fn iteration_budgets(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    even_card(
        ui,
        "Iteration limits",
        Some(("per stage, not per task", Tone::Neutral)),
        |ui| {
            budget_row(
                ui,
                cx,
                "ITL1",
                "DC · iterations per solve · a tiered analysis uses its tier's",
                |draft| &mut draft.itl1,
            );
            let native_time_controls =
                cx.model.sim_setup.options.compatibility != SimulationCompatibility::Xyce;
            ui.add_enabled_ui(native_time_controls, |ui| {
                budget_row(
                    ui,
                    cx,
                    "ITL4",
                    "transient · iterations per accepted timestep",
                    |draft| &mut draft.itl4,
                );
                budget_row(
                    ui,
                    cx,
                    "TRTOL",
                    "transient · truncation error the step controller will accept",
                    |draft| &mut draft.trtol,
                );
            }).response.on_hover_text(
                "Xyce uses each analysis's NONLIN-TRAN MAXSTEP limit and TIMEINT truncation tolerances."
            );
            budget_row(
                ui,
                cx,
                "LTE rel",
                "transient · relative truncation bound · blank keeps the engine's",
                |draft| &mut draft.transient_lte_reltol,
            );
            budget_row(
                ui,
                cx,
                "LTE abs",
                "transient · absolute truncation bound · blank keeps the engine's",
                |draft| &mut draft.transient_lte_abstol,
            );
            budget_row(
                ui,
                cx,
                "SEED",
                "statistical draws · agauss/gauss/unif · blank is not reproducible",
                |draft| &mut draft.statistical_seed,
            );
        },
    );
}

fn budget_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, SimulateFeature>,
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
        field(&mut cx.model.sim_setup.session.options_draft),
        width,
    );
    commit_on_release(cx, &response);
}

// ----------------------------------------------------------- continuation ladder

const LADDER_CONTROL_WIDTH: f32 = 128.0;

fn continuation_ladder(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
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
        + usize::from(cx.model.sim_setup.session.options_draft.gmin_stepping)
        + usize::from(cx.model.sim_setup.session.options_draft.source_stepping)
        + usize::from(cx.model.sim_setup.session.options_draft.pseudo_transient)
        + usize::from(cx.model.sim_setup.session.options_draft.arc_length);
    let status = format!("{enabled_rungs} of 5 rungs enabled");
    even_card(
        ui,
        "Continuation ladder",
        Some((status.as_str(), Tone::Neutral)),
        |ui| {
            for (index, (name, contract, control)) in stages.into_iter().enumerate() {
                ladder_stage(ui, cx, index + 1, name, contract, control);
            }
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
    cx: &mut Cx<'_, SimulateFeature>,
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
            let current = cx.model.sim_setup.session.options_draft.damping;
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
                cx.model.sim_setup.session.options_draft.damping = picked;
                commit_draft(cx);
            }
        }
        LadderControl::Toggle(stage) => {
            let value = match stage {
                Stage::Gmin => &mut cx.model.sim_setup.session.options_draft.gmin_stepping,
                Stage::Source => &mut cx.model.sim_setup.session.options_draft.source_stepping,
                Stage::PseudoTransient => {
                    &mut cx.model.sim_setup.session.options_draft.pseudo_transient
                }
                Stage::ArcLength => &mut cx.model.sim_setup.session.options_draft.arc_length,
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
                commit_draft(cx);
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

fn time_integration(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    even_card(
        ui,
        "Time integration",
        Some(("every time-domain analysis", Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                let methods: Vec<String> = IntegrationMethod::all()
                    .iter()
                    .map(|method| method.display_name().to_owned())
                    .collect();
                let method_index = cx.model.sim_setup.session.options_draft.method;
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
                    cx.model.sim_setup.session.options_draft.method = index;
                    commit_draft(cx);
                }

                let mut min_response = None;
                let mut max_response = None;
                field_pair(
                    ui,
                    ("Minimum timestep", &mut |ui: &mut Ui, width: f32| {
                        min_response = Some(mono_input(
                            ui,
                            "Minimum timestep",
                            &mut cx.model.sim_setup.session.options_draft.min_timestep,
                            width,
                        ));
                    }),
                    Some(("Maximum timestep", &mut |ui: &mut Ui, width: f32| {
                        max_response = Some(mono_input(
                            ui,
                            "Maximum timestep",
                            &mut cx.model.sim_setup.session.options_draft.max_timestep,
                            width,
                        ));
                    })),
                );
                for response in [min_response, max_response].into_iter().flatten() {
                    commit_on_release(cx, &response);
                }

                let mut bypass_picked = None;
                let bypass_on = cx.model.sim_setup.session.options_draft.bypass_enabled;
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
                    cx.model.sim_setup.session.options_draft.bypass_enabled = index == 0;
                    commit_draft(cx);
                }

                let mut bypass_reltol = None;
                let mut bypass_abstol = None;
                field_pair(
                    ui,
                    ("Bypass relative bound", &mut |ui: &mut Ui, width: f32| {
                        bypass_reltol = Some(mono_input(
                            ui,
                            "Bypass relative bound",
                            &mut cx.model.sim_setup.session.options_draft.bypass_reltol,
                            width,
                        ));
                    }),
                    Some(("Bypass voltage floor", &mut |ui: &mut Ui, width: f32| {
                        bypass_abstol = Some(mono_input(
                            ui,
                            "Bypass voltage floor",
                            &mut cx.model.sim_setup.session.options_draft.bypass_abstol,
                            width,
                        ));
                    })),
                );
                for response in [bypass_reltol, bypass_abstol].into_iter().flatten() {
                    commit_on_release(cx, &response);
                }
            });
        },
    );
}

// -------------------------------------------------------- matrix & determinism

fn matrix_policy(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    even_card(ui, "Matrix, pivoting & conditioning", None, |ui| {
        card_body(ui, |ui| {
            let solvers: Vec<String> = MatrixSolver::all()
                .iter()
                .map(|solver| solver.display_name().to_owned())
                .collect();
            let index = cx
                .model
                .sim_setup
                .session
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
                        &mut cx.model.sim_setup.session.options_draft.gmin,
                        width,
                    ));
                })),
            );
            if let Some(index) = picked {
                cx.model.sim_setup.session.options_draft.solver = index;
                commit_draft(cx);
            }
            if let Some(response) = gmin_response {
                commit_on_release(cx, &response);
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
                            &mut cx.model.sim_setup.session.options_draft.pivrel,
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
                            &mut cx.model.sim_setup.session.options_draft.pivtol,
                            width,
                        ));
                    },
                )),
            );
            for response in [pivrel_response, pivtol_response].into_iter().flatten() {
                commit_on_release(cx, &response);
            }

            let mut rshunt_response = None;
            field_pair(
                ui,
                (
                    "Node shunt to ground · RSHUNT",
                    &mut |ui: &mut Ui, width: f32| {
                        rshunt_response = Some(mono_input(
                            ui,
                            "Node shunt to ground \u{b7} RSHUNT",
                            &mut cx.model.sim_setup.session.options_draft.rshunt,
                            width,
                        ));
                    },
                ),
                None,
            );
            if let Some(response) = rshunt_response {
                commit_on_release(cx, &response);
            }
        });
    });
}

// --------------------------------------------------------- temperature reference

/// The model reference temperature.
///
/// TEMP is the run set's and is edited there, on its reference point and its
/// temperature axis. TNOM is the plan's, and this is its only editor: it says
/// nothing about where the circuit runs, only about how every model card
/// without its own `TNOM=` is to be read.
///
/// Returns whether the draft was released and is ready to commit — the card
/// edits the setup it is given and does not reach the rest of the application.
fn temperature_reference(ui: &mut Ui, setup: &mut SimSetupState) -> bool {
    let mut release = false;
    even_card(ui, "Temperature reference", None, |ui| {
        card_body(ui, |ui| {
            let mut tnom = None;
            field_pair(
                ui,
                (
                    "Model reference temperature · TNOM",
                    &mut |ui: &mut Ui, width: f32| {
                        tnom = Some(mono_input(
                            ui,
                            "Model reference temperature \u{b7} TNOM",
                            &mut setup.session.options_draft.tnom,
                            width,
                        ));
                    },
                ),
                None,
            );
            release = tnom.is_some_and(|response| released(&response));
        });
    });
    release
}

// ------------------------------------------------------------- step ceilings

/// The origin cell of a value an analysis owns rather than the plan.
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
pub(super) fn resolved_step_ceiling(
    authored: &str,
    plan_ceiling: Option<f64>,
) -> (String, &'static str) {
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
/// one. The run steps at `min(step time, plan ceiling)`, and at the step time
/// alone when the plan states no ceiling.
///
/// `None` where the step time does not parse: such a plan has no ceiling to
/// state, and its run is refused before it acquires one.
pub(super) fn inherited_step_ceiling(
    step_time: &str,
    plan_ceiling: Option<f64>,
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
fn step_ceiling(candidate: &str, plan_ceiling: Option<f64>) -> Option<(String, bool)> {
    let value = crate::simulation::dialog::parse_si_value(candidate).ok()?;
    match plan_ceiling {
        Some(ceiling) if ceiling.is_finite() && ceiling > 0.0 && value > ceiling => {
            Some((format_value(ceiling), true))
        }
        _ => Some((candidate.trim().to_owned(), false)),
    }
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
        O::Abstol => format_value(options.iabstol),
        O::Vntol => format_value(options.vntol),
        O::ResidualReltol => format_value(options.residual_reltol),
        O::Gmin => format_value(options.gmin),
        // No shunt is placed unless the plan states one.
        O::Rshunt => options
            .rshunt
            .map_or_else(|| "none".to_owned(), format_value),
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
        // A plan without a ceiling leaves each transient its own step time.
        O::MaximumTimestep => options
            .max_timestep
            .map_or_else(|| "step time".to_owned(), format_value),
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
        // that is what the hint slot says.
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

fn flag_preset(enabled: bool) -> String {
    if enabled { "on" } else { "off" }.to_owned()
}

/// A bound the plan does not state leaves the engine's own default in force,
/// which is a fact about the plan and is said rather than guessed at.
fn optional_preset(value: Option<f64>) -> String {
    value.map_or_else(|| "engine default".to_owned(), format_value)
}

/// The Newton budget a tier assigns, and the tier that assigned it.
///
/// Read by the Advanced options section, whose ITL1 row is refused for a
/// tiered analysis and therefore has to state the owner's value rather than
/// the plan's.
pub(super) fn tier_iteration_budget(accuracy: AnalysisAccuracy) -> String {
    format!(
        "{} \u{00b7} {}",
        accuracy.solver_policy().iteration_budget,
        accuracy.display_name()
    )
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
    cx: &mut Cx<'_, SimulateFeature>,
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
    authored: &str,
) -> Result<(), String> {
    match prepared_numeric_record(cx.model, instance, option, authored) {
        Ok(record) => super::lifecycle::commit_numeric_override(cx, instance, Some(record)),
        Err(error) => {
            // A value the record itself refuses never reaches the plan
            // transaction, so nothing downstream would announce it. Stated
            // through the funnel every refused plan command uses, which is what
            // makes one refused field one toast and one Console line.
            super::lifecycle::record_failure(cx, "Advanced option", &error);
            Err(error)
        }
    }
}

/// The record one authored value would produce, or why it is refused.
///
/// Split out so the writer above can announce a refusal exactly once: the plan
/// transaction reports its own, and this reports the ones that never reach it.
fn prepared_numeric_record(
    model: &Model,
    instance: AnalysisInstanceId,
    option: NumericOverrideOption,
    authored: &str,
) -> Result<AnalysisNumericOverride, String> {
    let plan = model.sim_setup.stable_analysis_plan()?;
    let target = plan
        .instance(instance)
        .ok_or_else(|| "The selected analysis is no longer in the plan.".to_owned())?;
    let kind = target.kind();
    let ownership = plan.solver_ownership(instance);
    let mut record = target.numeric_override().cloned().unwrap_or_default();
    record.set_for_instance(kind, ownership, option, authored)?;
    Ok(record)
}

// ------------------------------------------------------------------- committing

/// Whether a text edit has been let go of, rather than typed into.
///
/// Committing per character would produce a configuration receipt for each one
/// and invalidate preflight mid-word.
pub(super) fn released(response: &egui::Response) -> bool {
    response.lost_focus() || (response.changed() && !response.has_focus())
}

/// Apply a text edit when it is released, not on every keystroke.
fn commit_on_release(cx: &mut Cx<'_, SimulateFeature>, response: &egui::Response) {
    if released(response) {
        commit_draft(cx);
    }
}

/// Apply the draft if it is valid and different; otherwise record why it is
/// not applied and leave the effective options alone.
pub(super) fn commit_draft(cx: &mut Cx<'_, SimulateFeature>) {
    match pending_change(cx.model) {
        PendingChange::None => {}
        PendingChange::Invalid(errors) => {
            // Releasing a field is an event, and the reason it was not applied
            // has to travel with it: the policy strip states the same thing,
            // but it sits at the top of a page whose fields scroll well past
            // it.
            record_plan_refusal(
                cx,
                format!("Solver options were not applied · {}", errors.join(" · ")),
            );
        }
        PendingChange::Ready(options) => apply_options(cx, &options),
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::*;
    use crate::workbench::simulate::test_host::SimulateTestHost;

    #[test]
    fn solver_options_commit_advances_the_plan_and_invalidates_preflight() {
        let mut harness = SimulateTestHost::harness();
        let (plan_id, source_revision) = harness
            .model
            .sim_setup
            .stable_analysis_plan()
            .map(|plan| (plan.id(), plan.revision()))
            .expect("default plan");
        let topology_root = harness
            .model
            .workspace
            .content
            .simulation_root_reference()
            .key();
        let topology_revision = harness.model.schematic.topology_version();
        let topology_closure = vec![(topology_root.to_ascii_lowercase(), topology_revision)];
        harness.runs.preflight.report = Some(crate::state::PreflightReport {
            project_revision: harness.model.workspace.content.project.revision().get(),
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan_id: Some(plan_id),
            simulation_plan_revision: Some(source_revision),
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: None,
        });
        let mut options = harness.model.sim_setup.options.clone();
        options.reltol *= 2.0;

        assert!(
            commit_options_transaction(&mut harness.cx(), &options)
                .expect("validated options commit atomically")
        );

        assert_eq!(
            harness
                .model
                .sim_setup
                .stable_analysis_plan()
                .expect("plan remains available")
                .revision(),
            source_revision.next().expect("revision advances")
        );
        assert_eq!(harness.model.sim_setup.options.reltol, options.reltol);
        assert!(harness.runs.preflight.report.is_none());
    }

    #[test]
    fn unchanged_solver_options_do_not_fabricate_a_plan_revision() {
        let mut harness = SimulateTestHost::harness();
        let revision = harness
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .revision();
        let options = harness.model.sim_setup.options.clone();

        assert!(!commit_options_transaction(&mut harness.cx(), &options).expect("no-op succeeds"));
        assert_eq!(
            harness
                .model
                .sim_setup
                .stable_analysis_plan()
                .expect("plan remains available")
                .revision(),
            revision
        );
    }
}
