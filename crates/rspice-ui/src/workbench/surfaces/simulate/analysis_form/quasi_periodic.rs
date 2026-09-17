//! The quasi-periodic family: QPSS, and the three small-signal analyses taken
//! about its solution — QPAC, QP noise and QP transfer.
//!
//! One module because the three small-signal forms are the same form with a
//! row or two each of their own: a frequency axis, the two ends of a
//! measurement, and the lattice the mixing products are counted over. QPSS is
//! here because it is what defines that lattice.

use egui::Ui;

use crate::simulation::plan::{
    QpssDraft, QuasiPeriodicAcDraft, QuasiPeriodicNoiseDraft, QuasiPeriodicTransferDraft,
};

use super::{
    QuantityPresentationPolicy, UiNumberLocale, frequency_sweep_fields, input_row,
    input_row_enabled, switch_row,
};

/// Render the QPSS fields.
pub(super) fn shooting_fields(ui: &mut Ui, setup: &mut QpssDraft) {
    input_row(ui, "Tone frequencies", &mut setup.tones);
    input_row(ui, "Harmonic orders", &mut setup.harmonics);
    input_row(ui, "Max iterations", &mut setup.max_iterations);
    input_row(ui, "Relative tolerance", &mut setup.relative_tolerance);
    switch_row(ui, "Autonomous oscillator", &mut setup.autonomous);
    input_row_enabled(
        ui,
        "Oscillator node",
        &mut setup.oscillator_node,
        setup.autonomous,
    );
}

/// Render the QPAC fields.
pub(super) fn ac_fields(
    ui: &mut Ui,
    setup: &mut QuasiPeriodicAcDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
    input_row(ui, "Input source", &mut setup.input_source);
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
    input_row(ui, "Input lattice", &mut setup.input_lattice);
    input_row(ui, "Output lattice", &mut setup.output_lattice);
}

/// Render the QP noise fields.
pub(super) fn noise_fields(
    ui: &mut Ui,
    setup: &mut QuasiPeriodicNoiseDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
    input_row(ui, "Input source", &mut setup.input_source);
    input_row(ui, "Lattice ranges", &mut setup.lattice_products);
    switch_row(ui, "Integrated noise", &mut setup.integrated_noise);
    switch_row(ui, "Contributor ranking", &mut setup.contributor_ranking);
}

/// Render the QP transfer fields.
pub(super) fn transfer_fields(
    ui: &mut Ui,
    setup: &mut QuasiPeriodicTransferDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
    input_row(ui, "Input source", &mut setup.input_source);
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
    input_row(ui, "Input lattice", &mut setup.input_lattice);
    input_row(ui, "Output lattice", &mut setup.output_lattice);
    switch_row(ui, "Group delay", &mut setup.group_delay);
}
