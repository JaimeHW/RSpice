//! The noise form: the frequency axis, the two ends of the measurement, and
//! how much of the contribution breakdown is retained.
//!
//! The sweep row holds two controls, because an explicit frequency list is an
//! axis in its own right and a retained list has to stay readable while a
//! graded sweep is selected. Output and input are offered from the design's
//! own elaborated vocabulary rather than typed blind: a name the design does
//! not carry is not a suggestion, it is a run that fails at validation.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::config::{NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType};
use crate::simulation::plan::NoiseDraft;
use crate::ui::tokens::Tokens;

use super::{
    ENVELOPE_INLINE_CONTROL_GAP, NOISE_DOMAIN_PRESET_LIMIT, NOISE_INPUT_CUSTOM_CHOICE,
    NOISE_OUTPUT_CUSTOM_CHOICE, NoiseDomain, QuantityPresentationPolicy, SWEEP_POINT_NEUTRAL_LABEL,
    UiNumberLocale, choice_row, field_cell, full_width_field, input_row_enabled, mono_input,
    noise_domain_advisory, noise_domain_hint, noise_domain_row, noise_point_field_label,
    noise_sweep_control_widths, quantity_input_row_enabled, select_mono_with_response,
    uses_two_column_fields,
};

pub(super) const NOISE_FIELD_LABELS: [&str; 8] = [
    "Sweep",
    // The point field names its own units, so the frozen entry is the ungraded
    // spelling and the rendered one re-resolves. See
    // `the_sweep_point_label_names_what_a_point_is_in_each_mode`.
    SWEEP_POINT_NEUTRAL_LABEL,
    "Start frequency",
    "Stop frequency",
    "Output node",
    "Input source",
    "Contribution detail",
    "Integrated noise",
];

pub(super) const NOISE_SWEEP_CHOICES: [&str; 4] = NoiseSweepType::OPTIONS;

pub(super) const NOISE_CONTRIBUTION_CHOICES: [&str; 4] = NoiseContributionDetail::OPTIONS;

pub(super) const NOISE_INTEGRATION_CHOICES: [&str; 3] = NoiseIntegrationMode::OPTIONS;

pub(super) fn noise_enum_choice_row(
    ui: &mut Ui,
    label: &str,
    options: &[&str],
    selected: usize,
) -> Option<usize> {
    let mut next = selected;
    choice_row(ui, label, options, &mut next).then_some(next)
}

pub(super) fn noise_sweep_control(
    ui: &mut Ui,
    sweep: &mut NoiseSweepType,
    explicit_frequencies: &mut String,
) {
    ui.spacing_mut().item_spacing.x = ENVELOPE_INLINE_CONTROL_GAP;
    let selected = sweep.selection_index();
    let current = selected
        .and_then(|index| NOISE_SWEEP_CHOICES.get(index))
        .copied()
        .unwrap_or("Schema unavailable");
    let options = NOISE_SWEEP_CHOICES.map(str::to_owned);
    let width = ui.available_width();
    let explicit = matches!(sweep, NoiseSweepType::ExplicitFrequencyList);
    let (selector_width, editor_width) = noise_sweep_control_widths(width);
    let salt = format!("analysis-noise-sweep-{}", ui.id().value());
    if let Some(index) = select_mono_with_response(
        ui,
        &salt,
        NOISE_FIELD_LABELS[0],
        current,
        &options,
        selector_width,
    )
    .picked
    {
        *sweep = NoiseSweepType::from_selection_index(index);
    }
    ui.add_enabled_ui(explicit, |ui| {
        mono_input(
            ui,
            NOISE_FIELD_LABELS[0],
            explicit_frequencies,
            editor_width,
        )
        .on_hover_text(if explicit {
            "Comma- or space-separated frequencies in Hz"
        } else {
            "Select Explicit frequency list to edit this retained axis"
        });
    });
}

/// The sweep row: the graded axis, and the explicit list beside it.
pub(super) fn noise_sweep_row(
    ui: &mut Ui,
    sweep: &mut NoiseSweepType,
    explicit_frequencies: &mut String,
) {
    if uses_two_column_fields(ui) {
        field_cell(
            ui,
            NOISE_FIELD_LABELS[0],
            Some("domain constrained"),
            |ui| {
                ui.horizontal(|ui| noise_sweep_control(ui, sweep, explicit_frequencies));
            },
        );
    } else {
        full_width_field(
            ui,
            NOISE_FIELD_LABELS[0],
            Some("domain constrained"),
            Tokens::get(ui.ctx()).metrics.ctl_h,
            |ui| ui.horizontal(|ui| noise_sweep_control(ui, sweep, explicit_frequencies)),
        );
    }
}

/// Render the noise fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut NoiseDraft,
    noise_domain: NoiseDomain<'_>,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    noise_sweep_row(ui, &mut setup.sweep, &mut setup.explicit_frequencies);
    let fixed_grid = !matches!(setup.sweep, NoiseSweepType::ExplicitFrequencyList);
    input_row_enabled(
        ui,
        noise_point_field_label(setup.sweep),
        &mut setup.points,
        fixed_grid,
    );
    quantity_input_row_enabled(
        ui,
        NOISE_FIELD_LABELS[2],
        &mut setup.fstart,
        QuantityInputKind::Frequency,
        policy,
        locale,
        fixed_grid,
    );
    quantity_input_row_enabled(
        ui,
        NOISE_FIELD_LABELS[3],
        &mut setup.fstop,
        QuantityInputKind::Frequency,
        policy,
        locale,
        fixed_grid,
    );
    let offered_nodes = noise_domain
        .nodes
        .get(..NOISE_DOMAIN_PRESET_LIMIT)
        .unwrap_or(noise_domain.nodes);
    let offered_sources = noise_domain
        .sources
        .get(..NOISE_DOMAIN_PRESET_LIMIT)
        .unwrap_or(noise_domain.sources);
    let previous_output = setup.output.clone();
    noise_domain_row(
        ui,
        NOISE_FIELD_LABELS[4],
        "output",
        &noise_domain_hint(
            "nodes",
            offered_nodes.len(),
            noise_domain.nodes.len(),
            noise_domain.unavailable.is_some(),
        ),
        offered_nodes,
        NOISE_OUTPUT_CUSTOM_CHOICE,
        &mut setup.output,
    );
    if setup.output != previous_output {
        // Once the exact output-expression field is edited it owns
        // both nodes; a hidden legacy reference must not leak into it.
        setup.reference = "0".to_owned();
    }
    noise_domain_row(
        ui,
        NOISE_FIELD_LABELS[5],
        "input",
        &noise_domain_hint(
            "sources",
            offered_sources.len(),
            noise_domain.sources.len(),
            noise_domain.unavailable.is_some(),
        ),
        offered_sources,
        NOISE_INPUT_CUSTOM_CHOICE,
        &mut setup.input,
    );
    if let Some(reason) = noise_domain.unavailable {
        noise_domain_advisory(ui, reason);
    }
    if let Some(selection) = noise_enum_choice_row(
        ui,
        NOISE_FIELD_LABELS[6],
        &NOISE_CONTRIBUTION_CHOICES,
        setup.contribution_detail.selection_index(),
    ) && let Some(detail) = NoiseContributionDetail::from_selection_index(selection)
    {
        setup.contribution_detail = detail;
    }
    if let Some(selection) = noise_enum_choice_row(
        ui,
        NOISE_FIELD_LABELS[7],
        &NOISE_INTEGRATION_CHOICES,
        setup.integration_mode.selection_index(),
    ) && let Some(mode) = NoiseIntegrationMode::from_selection_index(selection)
    {
        setup.integration_mode = mode;
    }
}
