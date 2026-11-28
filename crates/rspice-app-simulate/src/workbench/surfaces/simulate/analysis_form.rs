//! Per-analysis configuration forms for the Simulate right panel.
//!
//! Each form edits one typed [`AnalysisDraft`] owned by a stable analysis
//! instance. A form says nothing about what its kind *is* — the reader picked
//! the kind — so the only prose here belongs to a field: a constraint that
//! would refuse the run, or a consequence of the current configuration that
//! the controls do not state. Both are painted beside the control they are
//! about. Validation is rendered by the caller.

mod ac;
mod ac_data;
mod dc_mismatch;
mod dc_sweep;
mod disto;
mod envelope;
mod fourier;
mod harmonic_balance;
mod monte_carlo;
pub(super) use monte_carlo::checkpoint_sources;
pub(super) use transient::checkpoint_sources as transient_checkpoint_sources;
mod name_domain;
mod noise;
mod operating_point;
mod optimization;
/// Kept a module of its own rather than a branch of [`form`]: an analysis's
/// advanced options are the same fields in the same grid, but which ones an
/// analysis owns is a question about the option catalogue, not about the kind's
/// own parameters. It is `pub(super)` so the route can draw them after the
/// form's own fields; the primitives it draws with stay private here.
pub(super) mod options;
mod pac;
mod periodic_network;
mod pnoise;
mod pole_zero;
mod pss;
mod pstb;
mod pxf;
mod quasi_periodic;
mod recorded_fft;
mod rows;
mod run_space;
mod s_parameter;
mod sensitivity;
mod soa;
mod stb;
mod stb_probe;
mod sweep_point_label;
mod transfer_function;
mod transient;
mod transient_noise;

pub(crate) use name_domain::*;
pub(crate) use rows::*;
pub(super) use run_space::RunSpaceContext;
use sweep_point_label::{
    SWEEP_KINDS, SWEEP_POINT_NEUTRAL_LABEL, noise_point_field_label, sweep_point_field_label,
};

use egui::{Align, Layout, Rect, Response, Ui, UiBuilder, vec2};

use super::page_kit;

use crate::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};
use crate::simulation::plan::{AnalysisDraft, FrequencySweepDraft};
use crate::state::format_engineering;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, choice_row as inspector_choice_row, input_row as inspector_input_row, mono_input,
    select, select_mono_with_response, select_with_disabled, switch_row as inspector_switch_row,
};
use crate::workbench::design_system::property_row as inspector_property_row;
use rspice_simulation::runner::TfRunConfig;

const ENVELOPE_INLINE_CONTROL_GAP: f32 = 6.0;
const FIELD_ROW_GAP: f32 = 10.0;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct OpContextAvailability {
    pub previous_state: bool,
    pub compatible_previous_state: bool,
    pub soa_violations: bool,
}

/// Whether `draft`'s form chooses its probe from the loop probes the drawing
/// places, which the page has to gather for it.
pub(super) fn offers_placed_loop_probes(draft: &AnalysisDraft) -> bool {
    matches!(draft, AnalysisDraft::Stb(_) | AnalysisDraft::Pstb(_))
}

/// Render the form for `draft`.
pub(super) fn form(
    ui: &mut Ui,
    draft: &mut AnalysisDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
    envelope_modulation_sources: &[String],
    placed_loop_probes: &[String],
    // The RF ports the design places, resolved once by the caller. The
    // S-parameter form reads these rather than keeping a copy: an editable
    // second declaration of the same ports is what this list replaces.
    placed_rf_ports: &[crate::simulation::placed_sources::PlacedRfPort],
    noise_domain: NoiseDomain<'_>,
    // What the design deck offers the transfer-function form's two ports, or
    // why it offers nothing. Resolved once per design by the caller, and
    // `None` for every analysis that is not a transfer function.
    tf_inference: Option<&Result<TfRunConfig, String>>,
    op_context: OpContextAvailability,
    run_space: &run_space::RunSpaceContext<'_>,
    study_bases: &[(crate::product::AnalysisInstanceId, String)],
    route: &mut Option<crate::state::SimulationPage>,
) {
    clear_pending_cell(ui);
    ui.spacing_mut().item_spacing.y = FIELD_ROW_GAP;
    match draft {
        AnalysisDraft::OperatingPoint(setup) => operating_point::fields(ui, setup, op_context),
        AnalysisDraft::Transient(setup) => transient::fields(ui, setup, policy, locale),
        AnalysisDraft::Ac(setup) => ac::fields(ui, setup, policy, locale),
        AnalysisDraft::DcSweep(setup) => dc_sweep::fields(ui, setup),
        AnalysisDraft::Noise(setup) => noise::fields(ui, setup, noise_domain, policy, locale),
        AnalysisDraft::PoleZero(setup) => pole_zero::fields(ui, setup),
        AnalysisDraft::Sensitivity(setup) => sensitivity::fields(ui, setup, policy, locale),
        AnalysisDraft::MonteCarlo(setup) => monte_carlo::fields(ui, setup, study_bases),
        AnalysisDraft::Pss(setup) => {
            pss::fields(ui, setup, envelope_modulation_sources, policy, locale)
        }
        AnalysisDraft::Stb(setup) => stb::fields(ui, setup, placed_loop_probes, policy, locale),
        // The plan may already declare a temperature axis. This instance
        // either reads it or states its own, and says which in place —
        // because two temperature declarations that silently disagree is
        // the same defect the corner form above was built to stop.
        AnalysisDraft::Temperature(setup) => {
            run_space::temperature_form(ui, setup, run_space, route, policy, locale, study_bases)
        }
        AnalysisDraft::HarmonicBalance(setup) => {
            harmonic_balance::fields(ui, setup, policy, locale)
        }
        AnalysisDraft::SParameter(setup) => {
            s_parameter::fields(ui, setup, placed_rf_ports, policy, locale)
        }
        AnalysisDraft::Pac(setup) => pac::fields(ui, setup, policy, locale),
        AnalysisDraft::Pnoise(setup) => pnoise::fields(ui, setup, policy, locale),
        AnalysisDraft::Pxf(setup) => pxf::fields(ui, setup, policy, locale),
        AnalysisDraft::Pstb(setup) => pstb::fields(ui, setup, placed_loop_probes),
        AnalysisDraft::TransferFunction(setup) => {
            transfer_function::fields(ui, setup, noise_domain, tf_inference)
        }
        // The run space has one editor — PVT, sweeps & variation — and one
        // owner, the plan. This form reads that declaration; it does not
        // keep one. A second set of axis controls here, or a second copy
        // behind them, would be a second owner of the same fact, and the
        // two would eventually disagree about how many points run.
        AnalysisDraft::Corner(setup) => {
            run_space::corner_form(ui, setup, run_space, route, study_bases, policy)
        }
        AnalysisDraft::Envelope(setup) => {
            envelope::fields(ui, setup, envelope_modulation_sources, policy, locale)
        }
        AnalysisDraft::Fourier(setup) => fourier::fields(ui, setup, policy, locale),
        AnalysisDraft::Optimization(setup) => optimization::fields(ui, setup, study_bases),
        AnalysisDraft::Soa(setup) => soa::fields(ui, setup, policy, locale),
        AnalysisDraft::Disto(setup) => disto::fields(ui, setup, policy, locale),
        AnalysisDraft::Qpss(setup) => quasi_periodic::shooting_fields(ui, setup),
        AnalysisDraft::Hbsp(setup) => periodic_network::fields(ui, setup, policy, locale),
        AnalysisDraft::Hbnoise(setup) => periodic_network::noise_fields(ui, setup, policy, locale),
        AnalysisDraft::Psp(setup) => periodic_network::fields(ui, setup, policy, locale),
        AnalysisDraft::Qpac(setup) => quasi_periodic::ac_fields(ui, setup, policy, locale),
        AnalysisDraft::Qpnoise(setup) => quasi_periodic::noise_fields(ui, setup, policy, locale),
        AnalysisDraft::Qpxf(setup) => quasi_periodic::transfer_fields(ui, setup, policy, locale),
        AnalysisDraft::TransientNoise(setup) => transient_noise::fields(ui, setup, policy, locale),
        AnalysisDraft::DcMismatch(setup) => dc_mismatch::fields(ui, setup),
        AnalysisDraft::AcData(setup) => ac_data::fields(ui, setup),
        AnalysisDraft::Fft(setup) => recorded_fft::fields(ui, setup, policy, locale),
    }
    clear_pending_cell(ui);
}

#[cfg(test)]
mod tests;

fn study_base_row(
    ui: &mut Ui,
    selected_base: &mut Option<crate::product::AnalysisInstanceId>,
    bases: &[(crate::product::AnalysisInstanceId, String)],
    default_label: &str,
) {
    let mut choices = vec![(None, default_label.to_owned())];
    choices.extend(bases.iter().map(|(id, label)| (Some(*id), label.clone())));
    if let Some(id) = *selected_base
        && !choices.iter().any(|(candidate, _)| *candidate == Some(id))
    {
        choices.push((Some(id), format!("Unavailable analysis ({id})")));
    }
    let mut selected = choices
        .iter()
        .position(|(id, _)| *id == *selected_base)
        .unwrap_or(0);
    let labels = choices
        .iter()
        .map(|(_, label)| label.as_str())
        .collect::<Vec<_>>();
    if choice_row(ui, "Base analysis", &labels, &mut selected) {
        *selected_base = choices[selected].0;
    }
}

pub(super) fn prerequisite_rows(
    ui: &mut Ui,
    selected: &super::SelectedAnalysis,
) -> Option<super::AnalysisAction> {
    let mut requested = None;
    for prerequisite in &selected.prerequisite_roles {
        let bound = selected
            .dependencies
            .iter()
            .find(|dependency| dependency.prerequisite() == *prerequisite);
        let target = bound.map_or_else(
            || {
                if selected.enabled {
                    "unbound · preflight blocked".to_owned()
                } else {
                    "unbound · required when enabled".to_owned()
                }
            },
            |dependency| {
                selected
                    .dependency_names
                    .iter()
                    .find(|(id, _)| *id == dependency.target())
                    .map_or_else(|| "missing analysis".to_owned(), |(_, name)| name.clone())
            },
        );
        let label = format!("{} prerequisite", prerequisite.stable_id().to_uppercase());
        let candidates = selected
            .prerequisite_candidates
            .iter()
            .find_map(|(kind, candidates)| (*kind == *prerequisite).then_some(candidates));
        let Some(candidates) = candidates.filter(|candidates| !candidates.is_empty()) else {
            super::property_row(ui, &label, &target);
            continue;
        };
        let options = candidates
            .iter()
            .map(|candidate| candidate.label.clone())
            .collect::<Vec<_>>();
        let current = bound
            .and_then(|dependency| {
                candidates
                    .iter()
                    .find(|candidate| candidate.id == dependency.target())
            })
            .map_or("Select compatible instance", |candidate| {
                candidate.label.as_str()
            });
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let width = ui.available_width().max(1.0);
            let trailing = crate::workbench::design_system::panel_inset(ui);
            // The property rows above and below this one are the same block,
            // so the control starts where their values do. The share this
            // replaces was a second account of the same column that collapsed
            // to the label's own text width, opening a second label column a
            // hundred points left of the first.
            super::paint_control_row_label(ui, &label, width);
            let select_width = (ui.available_width() - trailing).max(1.0);
            let salt = format!(
                "analysis.{}.prerequisite.{}",
                selected.id,
                prerequisite.stable_id()
            );
            if let Some(index) = select(
                ui,
                &salt,
                &format!("Select {label}"),
                current,
                &options,
                select_width,
            ) && let Some(candidate) = candidates.get(index)
            {
                requested = Some(super::AnalysisAction::BindDependency {
                    prerequisite: *prerequisite,
                    target: candidate.id,
                });
            }
        });
    }
    requested
}
