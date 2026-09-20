//! QPSS tone and solver authoring with the small-signal analyses that consume
//! its independent-phase orbit. Each analysis retains its own source,
//! observation, sideband selection, frequency axis and numerical controls.

use egui::Ui;

use crate::simulation::plan::{
    QpssDraft, QuasiPeriodicAcDraft, QuasiPeriodicNoiseDraft, QuasiPeriodicTransferDraft,
};

use super::{
    QuantityPresentationPolicy, UiNumberLocale, choice_row, clear_pending_cell,
    frequency_sweep_fields, full_width_field, input_row, input_row_enabled, switch_row,
};

/// Render the QPSS fields.
pub(super) fn shooting_fields(ui: &mut Ui, setup: &mut QpssDraft) {
    ui.small("Driven QPSS preview. Automatic mode uses the iterative solver above 512 coupled coordinates; the direct solver is limited to 512. Both enforce the configured memory limits.");
    input_row(ui, "Tone frequencies", &mut setup.tones);
    input_row(ui, "Harmonic orders", &mut setup.harmonics);
    input_row(
        ui,
        "Mixing order (blank: full grid)",
        &mut setup.max_mixing_order,
    );
    input_row(
        ui,
        "Exact phase points (optional)",
        &mut setup.collocation_points,
    );
    input_row_enabled(
        ui,
        "Oversampling per tone",
        &mut setup.oversample,
        setup.collocation_points.trim().is_empty(),
    );
    ui.small("Use one sampling count for all tones, or comma-separated counts for each tone.");
    input_row(ui, "AC-only source tones", &mut setup.source_tones);
    ui.small("Assignments use V1=1, V2=2. SIN, PULSE and other waveforms use their authored frequencies.");
    switch_row(
        ui,
        "Initialize from DC operating point",
        &mut setup.dc_initialization,
    );
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Method;
    let mut method = match setup.linear_method {
        Method::Auto => 0,
        Method::Direct => 1,
        Method::Krylov => 2,
    };
    choice_row(
        ui,
        "Linear solver",
        &["Automatic", "Direct", "Krylov (matrix-free)"],
        &mut method,
    );
    setup.linear_method = match method {
        1 => Method::Direct,
        2 => Method::Krylov,
        _ => Method::Auto,
    };
    let iterative = setup.linear_method != Method::Direct;
    input_row_enabled(
        ui,
        "Krylov restart vectors",
        &mut setup.krylov_restart,
        iterative,
    );
    input_row_enabled(
        ui,
        "Maximum restart cycles",
        &mut setup.krylov_cycles,
        iterative,
    );
    input_row_enabled(
        ui,
        "Linear relative tolerance",
        &mut setup.linear_tolerance,
        iterative,
    );
    ui.small("Each restart cycle uses up to the requested number of vectors (8–64).");
    input_row(ui, "Max iterations", &mut setup.max_iterations);
    input_row(ui, "Relative tolerance", &mut setup.relative_tolerance);
    input_row(
        ui,
        "Current tolerance (A)",
        &mut setup.current_absolute_tolerance,
    );
    input_row(
        ui,
        "Voltage tolerance (V)",
        &mut setup.voltage_absolute_tolerance,
    );
    input_row(ui, "Maximum backtracks", &mut setup.max_backtracks);
    switch_row(
        ui,
        "Autonomous oscillator (unavailable)",
        &mut setup.autonomous,
    );
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
    input_row(
        ui,
        "Explicit probe offsets (optional)",
        &mut setup.explicit_offsets,
    );
    ui.add_enabled_ui(setup.explicit_offsets.trim().is_empty(), |ui| {
        frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
    });
    ui.small("Offsets are in Hz. Enter an increasing list, or leave it blank to generate a sweep. A tuple k selects the physical frequency offset + k·tones.");
    input_row(ui, "Input source", &mut setup.input_source);
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
    input_row(ui, "Input lattice", &mut setup.input_lattice);
    input_row(ui, "Output lattice", &mut setup.output_lattice);
    ui.small("Enter one signed integer per QPSS tone. Both tuples must be retained by the selected QPSS analysis.");
    input_row(ui, "Drive magnitude (V or A)", &mut setup.magnitude);
    input_row(ui, "Drive phase (degrees)", &mut setup.phase_degrees);
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Method;
    let mut method = match setup.linear_method {
        Method::Auto => 0,
        Method::Direct => 1,
        Method::Krylov => 2,
    };
    choice_row(
        ui,
        "Linear solver",
        &["Automatic", "Direct", "Krylov"],
        &mut method,
    );
    setup.linear_method = match method {
        1 => Method::Direct,
        2 => Method::Krylov,
        _ => Method::Auto,
    };
    input_row_enabled(
        ui,
        "Krylov restart vectors",
        &mut setup.krylov_restart,
        setup.linear_method != Method::Direct,
    );
    input_row_enabled(
        ui,
        "Krylov restart cycles",
        &mut setup.krylov_cycles,
        setup.linear_method != Method::Direct,
    );
    input_row(
        ui,
        "Relative equation tolerance",
        &mut setup.linear_tolerance,
    );
    input_row(
        ui,
        "Absolute current tolerance (A)",
        &mut setup.current_absolute_tolerance,
    );
    input_row(
        ui,
        "Absolute voltage tolerance (V)",
        &mut setup.voltage_absolute_tolerance,
    );
    ui.small("Automatic mode uses Krylov above 512 coupled coordinates. Relative and absolute tolerances qualify the physical equations for either solver.");
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

/// Render all QPXF sweep, source, observation, lattice and numerical controls.
pub(super) fn transfer_fields(
    ui: &mut Ui,
    setup: &mut QuasiPeriodicTransferDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    use crate::simulation::plan::{
        QpxfSidebandSelection as Sidebands, QpxfSourceSelection as Sources,
    };
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Method;
    use rspice_core::engine::QpxfFrequencyAxis as Axis;
    let mut axis = usize::from(setup.frequency_axis == Axis::Offset);
    choice_row(
        ui,
        "Sweep frequencies",
        &["Physical output", "Offset from tone lattice"],
        &mut axis,
    );
    setup.frequency_axis = if axis == 0 {
        Axis::Output
    } else {
        Axis::Offset
    };
    input_row(
        ui,
        "Explicit frequencies (Hz, optional)",
        &mut setup.explicit_frequencies,
    );
    ui.add_enabled_ui(setup.explicit_frequencies.trim().is_empty(), |ui| {
        frequency_sweep_fields(ui, &mut setup.sweep, policy, locale)
    });
    clear_pending_cell(ui);
    ui.small("Use an increasing list or generate a sweep. Linear sweeps and lists accept negative frequencies and zero. Logarithmic sweeps require positive frequencies.");
    let mut source = match setup.source_selection {
        Sources::Single => 0,
        Sources::Named => 1,
        Sources::AllIndependent => 2,
    };
    choice_row(
        ui,
        "Input sources",
        &["One source", "Selected sources", "All independent sources"],
        &mut source,
    );
    setup.source_selection = match source {
        1 => Sources::Named,
        2 => Sources::AllIndependent,
        _ => Sources::Single,
    };
    match setup.source_selection {
        Sources::Single => {
            input_row(ui, "Input source", &mut setup.input_source);
        }
        Sources::Named => {
            transfer_list(ui, "Source names, one per line", &mut setup.input_sources);
        }
        Sources::AllIndependent => {}
    }
    clear_pending_cell(ui);
    ui.small("Each transfer is per unit voltage or current at its source. Source AC magnitude and phase do not scale it.");
    let mut output = usize::from(setup.current_output);
    choice_row(
        ui,
        "Output quantity",
        &["Voltage", "Branch current"],
        &mut output,
    );
    setup.current_output = output == 1;
    if setup.current_output {
        input_row(ui, "Current branch", &mut setup.output_branch);
        clear_pending_cell(ui);
        ui.small("Select a voltage source/current probe, inductor, or another retained branch current. Insert a current probe before running QPSS to measure a branch without its own current unknown.");
    } else {
        input_row(ui, "Output node", &mut setup.output_node);
        input_row(ui, "Reference node", &mut setup.output_ref);
    }
    let mut sidebands = match setup.sideband_selection {
        Sidebands::Single => 0,
        Sidebands::Explicit => 1,
        Sidebands::MaxOrders => 2,
        Sidebands::AllRetained => 3,
    };
    choice_row(
        ui,
        "Input sidebands",
        &[
            "One tuple",
            "Selected tuples",
            "Per-tone maximum orders",
            "All retained tuples",
        ],
        &mut sidebands,
    );
    setup.sideband_selection = match sidebands {
        1 => Sidebands::Explicit,
        2 => Sidebands::MaxOrders,
        3 => Sidebands::AllRetained,
        _ => Sidebands::Single,
    };
    match setup.sideband_selection {
        Sidebands::Single => {
            input_row(ui, "Input tuple", &mut setup.input_lattice);
        }
        Sidebands::Explicit => {
            transfer_list(ui, "Input tuples, one per line", &mut setup.input_lattices);
        }
        Sidebands::MaxOrders => {
            input_row(ui, "Maximum order per tone", &mut setup.max_orders);
        }
        Sidebands::AllRetained => {}
    }
    input_row(ui, "Output tuple", &mut setup.output_lattice);
    clear_pending_cell(ui);
    ui.small("A tuple has one signed integer per QPSS tone, separated by commas. Input frequency = output frequency + (input tuple − output tuple)·tones. Tuples and order limits must fit the selected QPSS lattice.");
    switch_row(ui, "Group delay", &mut setup.group_delay);
    input_row_enabled(
        ui,
        "Group-delay magnitude floor",
        &mut setup.group_delay_magnitude_floor,
        setup.group_delay,
    );
    if setup.group_delay {
        clear_pending_cell(ui);
        ui.small("Delay is the phase derivative sampled across the sweep. Use enough frequency points to resolve phase rotations. Zero or low-magnitude transfers and ambiguous phase steps retain an undefined status.");
    }
    let mut method = match setup.linear_method {
        Method::Auto => 0,
        Method::Direct => 1,
        Method::Krylov => 2,
    };
    choice_row(
        ui,
        "Linear solver",
        &["Automatic", "Direct", "Krylov"],
        &mut method,
    );
    setup.linear_method = match method {
        1 => Method::Direct,
        2 => Method::Krylov,
        _ => Method::Auto,
    };
    input_row_enabled(
        ui,
        "Krylov restart vectors",
        &mut setup.krylov_restart,
        setup.linear_method != Method::Direct,
    );
    input_row_enabled(
        ui,
        "Krylov restart cycles",
        &mut setup.krylov_cycles,
        setup.linear_method != Method::Direct,
    );
    input_row(
        ui,
        "Relative adjoint tolerance",
        &mut setup.linear_tolerance,
    );
    clear_pending_cell(ui);
    ui.small("Automatic mode uses Krylov above 512 coupled coordinates. The relative tolerance qualifies the adjoint equations for either solver.");
}

fn transfer_list(ui: &mut Ui, label: &str, value: &mut String) {
    clear_pending_cell(ui);
    full_width_field(ui, label, None, 76.0, |ui| {
        ui.add_sized(
            [ui.available_width(), 76.0],
            egui::TextEdit::multiline(value)
                .code_editor()
                .desired_rows(3),
        )
    });
}
