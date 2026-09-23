//! The envelope-following form: the carriers the envelope rides on, the slow
//! window it is walked over, and how the periodic solve it starts from is
//! obtained.
//!
//! Every time field here carries its unit inside the control rather than in
//! the caption, because an envelope step and a carrier period differ by orders
//! of magnitude and a bare number beside a bare number is the one place that
//! has been misread.

use egui::{Align, Layout, Response, Ui, vec2};

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::EnvelopeDialogState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::{
    QuantityPresentationPolicy, UiNumberLocale, field_cell, input_row, inspector_input_row,
    mono_input, mono_input_with_suffix, named_periodic_source_row,
    normalize_quantity_on_focus_loss, select_mono_with_response, uses_two_column_fields,
};

pub(super) const ENVELOPE_FIELD_LABELS: [&str; 7] = [
    "Carrier tones",
    "Envelope stop",
    "Envelope step",
    "Harmonic order",
    "Modulation sources",
    "Initial periodic solve",
    "Output schedule",
];

pub(super) const ENVELOPE_INITIAL_SOLVE_CHOICES: &[&str] =
    &["HB", "PSS", "Transient spectral estimate"];

pub(super) const ENVELOPE_ADAPTIVE_CHOICES: &[&str] = &[
    "Adaptive solver samples",
    "Fixed envelope step",
    "Event-aligned only",
];

pub(super) const ENVELOPE_HARMONIC_ORDER_HELPER: &str = "positive integer";

/// A time field whose unit is painted inside the control.
pub(super) fn envelope_time_input_row(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Response {
    let response = if uses_two_column_fields(ui) {
        field_cell(ui, label, Some("engineering notation"), |ui| {
            mono_input_with_suffix(ui, label, value, "s")
        })
    } else {
        let t = Tokens::get(ui.ctx());
        let row_h = t.metrics.row_h;
        let color = t.color.text_dim;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), row_h),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                let (label_rect, _) =
                    ui.allocate_exact_size(vec2(96.0, row_h), egui::Sense::hover());
                ui.painter().text(
                    label_rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    label,
                    theme::sans(tokens::FS_1, FontWeight::Regular),
                    color,
                );
                mono_input_with_suffix(ui, label, value, "s")
            },
        )
        .inner
    };
    normalize_quantity_on_focus_loss(&response, value, QuantityInputKind::Time, policy, locale);
    response
}

/// The harmonic-order field, with the domain it accepts beside its caption.
pub(super) fn envelope_harmonic_order_row(ui: &mut Ui, value: &mut String) -> Response {
    if !uses_two_column_fields(ui) {
        return inspector_input_row(ui, ENVELOPE_FIELD_LABELS[3], value);
    }
    field_cell(
        ui,
        ENVELOPE_FIELD_LABELS[3],
        Some(ENVELOPE_HARMONIC_ORDER_HELPER),
        |ui| mono_input(ui, ENVELOPE_FIELD_LABELS[3], value, ui.available_width()),
    )
}

/// A mono choice row, in the grid or stacked under its own label.
pub(super) fn envelope_choice_row(
    ui: &mut Ui,
    label: &str,
    options: &[&str],
    value: &mut usize,
) -> bool {
    let mut add_control = |ui: &mut Ui| {
        let options = options
            .iter()
            .map(|option| (*option).to_owned())
            .collect::<Vec<_>>();
        let current = options
            .get(*value)
            .map_or("Schema unavailable", String::as_str);
        let salt = format!("analysis-envelope-field-{}-{label}", ui.id().value());
        let selection =
            select_mono_with_response(ui, &salt, label, current, &options, ui.available_width());
        if label == ENVELOPE_FIELD_LABELS[6] {
            selection.response.on_hover_text(
                "Output controls can override this schedule with a strobe interval or explicit reporting times.",
            );
        }
        if let Some(index) = selection.picked {
            *value = index;
            true
        } else {
            false
        }
    };

    if uses_two_column_fields(ui) {
        return field_cell(ui, label, Some("domain constrained"), add_control);
    }

    let t = Tokens::get(ui.ctx());
    let row_h = t.metrics.row_h;
    let color = t.color.text_dim;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (label_rect, _) = ui.allocate_exact_size(vec2(96.0, row_h), egui::Sense::hover());
            ui.painter().text(
                label_rect.left_center(),
                egui::Align2::LEFT_CENTER,
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                color,
            );
            add_control(ui)
        },
    )
    .inner
}

/// The modulation-source row: the design's periodic sources, or a declared list.
pub(super) fn envelope_modulation_source_row(
    ui: &mut Ui,
    value: &mut String,
    circuit_sources: &[String],
) {
    named_periodic_source_row(
        ui,
        ENVELOPE_FIELD_LABELS[4],
        "envelope-modulation",
        value,
        circuit_sources,
    );
}

/// Render the envelope-following fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut EnvelopeDialogState,
    envelope_modulation_sources: &[String],
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    super::switch_row(ui, "Multirate solver", &mut setup.multirate_enabled);
    input_row(ui, ENVELOPE_FIELD_LABELS[0], &mut setup.carrier_tones);
    envelope_time_input_row(
        ui,
        ENVELOPE_FIELD_LABELS[1],
        &mut setup.stop_time,
        policy,
        locale,
    );
    envelope_time_input_row(
        ui,
        ENVELOPE_FIELD_LABELS[2],
        &mut setup.envelope_step,
        policy,
        locale,
    );
    envelope_harmonic_order_row(ui, &mut setup.harmonic_order).on_hover_text(if setup.multirate_enabled {
        "Harmonic order applies to each independent carrier unless overridden below."
    } else { "Periodic initialization uses the carriers' common period. Harmonic order sets the upper frequency to the first carrier × order." });
    envelope_modulation_source_row(
        ui,
        &mut setup.modulation_sources,
        envelope_modulation_sources,
    );
    if !setup.multirate_enabled {
        envelope_choice_row(
            ui,
            ENVELOPE_FIELD_LABELS[5],
            ENVELOPE_INITIAL_SOLVE_CHOICES,
            &mut setup.initial_periodic_solve_idx,
        );
    }
    envelope_choice_row(
        ui,
        ENVELOPE_FIELD_LABELS[6],
        ENVELOPE_ADAPTIVE_CHOICES,
        &mut setup.adaptive_mode_idx,
    );
    if setup.multirate_enabled {
        multirate_fields(ui, setup, policy, locale);
    } else {
        initializer_fields(ui, setup, policy, locale);
    }
}

fn multirate_fields(
    ui: &mut Ui,
    setup: &mut EnvelopeDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    use super::{choice_row, field_note, input_row_enabled, sub_header, switch_row};
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Linear;
    let c = &mut setup.multirate;
    field_note(
        ui,
        "Multirate evolves Fourier coefficients between source events. It starts from a frozen periodic solution. Use full-waveform execution for models without supported slow-time storage or event equations.",
    );
    sub_header(ui, "Slow-time integration");
    switch_row(ui, "Adaptive integration", &mut c.adaptive);
    let mut method = usize::from(c.bdf2);
    choice_row(
        ui,
        "Integration method",
        &["Backward Euler", "BDF2"],
        &mut method,
    );
    c.bdf2 = method == 1;
    envelope_time_input_row(ui, "Maximum slow step", &mut c.maximum_step, policy, locale);
    field_note(
        ui,
        "Blank maximum step uses Envelope step, which also sets the initial integration step. Output schedule selects reports independently.",
    );
    ui.add_enabled_ui(c.adaptive, |ui| {
        envelope_time_input_row(ui, "Minimum slow step", &mut c.minimum_step, policy, locale);
        for (label, value) in [
            ("Relative error tolerance", &mut c.relative_tolerance),
            (
                "Voltage error tolerance (V)",
                &mut c.voltage_absolute_tolerance,
            ),
            (
                "Current error tolerance (A)",
                &mut c.current_absolute_tolerance,
            ),
            (
                "Auxiliary error tolerance",
                &mut c.auxiliary_absolute_tolerance,
            ),
            ("Maximum rejections per step", &mut c.maximum_rejections),
        ] {
            input_row(ui, label, value);
        }
    });
    input_row(ui, "Maximum accepted steps", &mut c.maximum_steps);
    sub_header(ui, "Carrier solution");
    for (label, value) in [
        ("Harmonic orders (optional)", &mut c.harmonics),
        ("Exact phase points (optional)", &mut c.collocation_points),
        ("AC-only source tones", &mut c.source_tones),
    ] {
        input_row(ui, label, value);
    }
    input_row_enabled(
        ui,
        "Mixing order (blank: full grid)",
        &mut c.max_mixing_order,
        setup
            .carrier_tones
            .split([',', ';', '\n'])
            .filter(|s| !s.trim().is_empty())
            .count()
            > 1,
    );
    input_row_enabled(
        ui,
        "Oversampling per carrier",
        &mut c.oversample,
        c.collocation_points.trim().is_empty(),
    );
    field_note(
        ui,
        "Counts accept one value for all carriers or one per carrier. Blank harmonic orders uses Harmonic order above. AC assignments use Vrf=1; waveforms use their authored frequencies.",
    );
    switch_row(
        ui,
        "DC operating point initial guess",
        &mut c.dc_initialization,
    );
    envelope_time_input_row(
        ui,
        "Waveform default step",
        &mut c.source_time_step,
        policy,
        locale,
    );
    field_note(
        ui,
        "Blank uses Envelope step for omitted waveform timing parameters. This basis stays fixed during adaptive integration.",
    );
    if c.dc_initialization {
        field_note(
            ui,
            "Advanced conventional solver overrides configure the DC initial guess. The controls below configure the spectral solves.",
        );
    }
    for (label, value) in [
        ("Newton relative tolerance", &mut c.solver_relative),
        ("Newton current tolerance (A)", &mut c.solver_current),
        ("Newton voltage tolerance (V)", &mut c.solver_voltage),
        ("Newton iteration limit", &mut c.solver_iterations),
        ("Maximum Newton backtracks", &mut c.solver_backtracks),
    ] {
        input_row(ui, label, value);
    }
    let mut method = match c.linear_method {
        Linear::Auto => 0,
        Linear::Direct => 1,
        Linear::Krylov => 2,
    };
    choice_row(
        ui,
        "Linear solver",
        &["Automatic", "Direct", "Krylov"],
        &mut method,
    );
    c.linear_method = match method {
        1 => Linear::Direct,
        2 => Linear::Krylov,
        _ => Linear::Auto,
    };
    ui.add_enabled_ui(c.linear_method != Linear::Direct, |ui| {
        for (label, value) in [
            ("Krylov restart vectors", &mut c.linear_restart),
            ("Krylov restart cycles", &mut c.linear_cycles),
            ("Linear relative tolerance", &mut c.linear_relative),
        ] {
            input_row(ui, label, value);
        }
    });
    sub_header(ui, "Source-event tolerances");
    for (label, value) in [
        ("Charge conservation (C)", &mut c.event_charge),
        ("Flux conservation (Wb)", &mut c.event_flux),
        ("Event current (A)", &mut c.event_current),
        ("Event voltage (V)", &mut c.event_voltage),
        ("Current rate (A/s)", &mut c.event_current_rate),
        ("Voltage rate (V/s)", &mut c.event_voltage_rate),
    ] {
        input_row(ui, label, value);
    }
}

fn initializer_fields(
    ui: &mut Ui,
    setup: &mut EnvelopeDialogState,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    use super::{choice_row, field_note, input_row, sub_header, switch_row};
    if setup.initial_periodic_solve_idx == 2 {
        return;
    }
    sub_header(ui, "Initializer solver");
    let controls = &mut setup.initialization;
    input_row(ui, "Iteration limit", &mut controls.max_iterations);
    input_row(ui, "Relative tolerance", &mut controls.reltol);
    input_row(ui, "Absolute tolerance", &mut controls.abstol);
    input_row(ui, "Newton damping", &mut controls.damping);
    switch_row(ui, "Solver logging", &mut controls.verbose);
    if setup.initial_periodic_solve_idx == 1 {
        input_row(
            ui,
            "Stabilization periods",
            &mut controls.pss_stabilization_periods,
        );
        envelope_time_input_row(ui, "Stabilization time", &mut controls.pss_stabilization_time, policy, locale)
            .on_hover_text("A positive time replaces the stabilization periods. Empty or zero uses the period count.");
        input_row(ui, "Points per period", &mut controls.pss_points_per_period);
        field_note(
            ui,
            "Empty points chooses an automatic grid. Use zero periods and no stabilization time to start shooting immediately.",
        );
        choice_row(
            ui,
            "Shooting integration",
            &["Auto", "Euler", "Trap", "Gear2", "TrapGear"],
            &mut controls.pss_integration_idx,
        );
    } else {
        input_row(ui, "Minimum damping", &mut controls.hb_min_damping);
        input_row(ui, "Oversampling", &mut controls.hb_oversample);
        input_row(
            ui,
            "Collocation points",
            &mut controls.hb_collocation_points,
        );
        field_note(
            ui,
            "Empty collocation points uses an automatic oversampled grid; explicit grids must be odd and resolve the harmonic order.",
        );
        switch_row(ui, "Force Krylov", &mut controls.hb_use_krylov);
        input_row(ui, "GMRES restart", &mut controls.hb_gmres_restart);
        switch_row(ui, "Source stepping", &mut controls.hb_source_stepping);
        switch_row(ui, "Exact Jacobian", &mut controls.hb_exact_jacobian);
    }
}
