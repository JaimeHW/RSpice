//! Complete noise measurements and numerical controls, including multiple outputs.
use super::*;
use crate::simulation::plan::{
    QpnoiseLatticeSelection as Lattice, QpnoiseOutputDraft, QpnoiseSourceSelection as Sources,
};
use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Method;
use rspice_core::engine::{QpnoiseFrequencyAxis as Axis, QpnoiseIntegrationMethod as Integration};
pub(super) fn fields(
    ui: &mut Ui,
    s: &mut QuasiPeriodicNoiseDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    let mut axis = usize::from(s.frequency_axis == Axis::Offset);
    choice_row(
        ui,
        "Sweep frequencies",
        &[
            "Physical first-output frequency",
            "Offset from tone lattice",
        ],
        &mut axis,
    );
    s.frequency_axis = if axis == 0 {
        Axis::Output
    } else {
        Axis::Offset
    };
    input_row(
        ui,
        "Explicit frequencies (Hz, optional)",
        &mut s.explicit_frequencies,
    );
    ui.add_enabled_ui(s.explicit_frequencies.trim().is_empty(), |ui| {
        frequency_sweep_fields(ui, &mut s.sweep, policy, locale)
    });
    clear_pending_cell(ui);
    ui.small("Lists and linear sweeps accept signed frequencies. Other outputs share the same translated offset. Integration uses each output's physical nonnegative frequency band.");
    observation_fields(
        ui,
        &mut s.current_output,
        &mut s.output_node,
        &mut s.output_ref,
        &mut s.output_branch,
        &mut s.output_lattice,
    );
    clear_pending_cell(ui);
    let mut remove = None;
    for (index, output) in s.additional_outputs.iter_mut().enumerate() {
        ui.push_id(("qpnoise-output", index), |ui| {
            clear_pending_cell(ui);
            ui.separator();
            ui.label(format!("Output {}", index + 2));
            observation_fields(
                ui,
                &mut output.current,
                &mut output.node,
                &mut output.reference,
                &mut output.branch,
                &mut output.lattice,
            );
            clear_pending_cell(ui);
            if ui.small_button("Remove output").clicked() {
                remove = Some(index);
            }
        });
    }
    if let Some(index) = remove {
        s.additional_outputs.remove(index);
    }
    clear_pending_cell(ui);
    if ui.button("Add noise output").clicked() {
        s.additional_outputs.push(QpnoiseOutputDraft {
            lattice: s.output_lattice.clone(),
            ..Default::default()
        });
    }
    ui.small("Each tuple has one signed integer per QPSS tone. Voltage and retained branch-current outputs retain their mutual complex noise correlations.");
    switch_row(ui, "Input-referred noise", &mut s.input_referral);
    if s.input_referral {
        input_row(ui, "Input source", &mut s.input_source);
        input_row(ui, "Input signal tuple", &mut s.input_lattice);
        clear_pending_cell(ui);
        ui.small("Referral uses a unit input voltage or current. A zero transfer retains an unavailable value while output noise remains valid.");
    }
    let mut lattice = match s.lattice_selection {
        Lattice::AllRetained => 0,
        Lattice::Range => 1,
        Lattice::MaxOrders => 2,
        Lattice::Explicit => 3,
    };
    choice_row(
        ui,
        "Noise input tuples",
        &[
            "All retained",
            "Signed ranges per tone",
            "Maximum orders per tone",
            "Selected tuples",
        ],
        &mut lattice,
    );
    s.lattice_selection = match lattice {
        1 => Lattice::Range,
        2 => Lattice::MaxOrders,
        3 => Lattice::Explicit,
        _ => Lattice::AllRetained,
    };
    match s.lattice_selection {
        Lattice::AllRetained => {}
        Lattice::Range => {
            input_row(ui, "Ranges (min:max per tone)", &mut s.lattice_products);
        }
        Lattice::MaxOrders => {
            input_row(ui, "Maximum orders", &mut s.max_orders);
        }
        Lattice::Explicit => {
            transfer_list(ui, "Noise tuples, one per line", &mut s.explicit_lattices)
        }
    }
    let mut selection = match s.source_selection {
        Sources::All => 0,
        Sources::Only => 1,
        Sources::Except => 2,
    };
    choice_row(
        ui,
        "Noise mechanisms",
        &[
            "All physical mechanisms",
            "Include named mechanisms",
            "Exclude named mechanisms",
        ],
        &mut selection,
    );
    s.source_selection = match selection {
        1 => Sources::Only,
        2 => Sources::Except,
        _ => Sources::All,
    };
    if s.source_selection != Sources::All {
        transfer_list(ui, "Mechanism names, one per line", &mut s.source_names);
        clear_pending_cell(ui);
        ui.small("Use complete contributor names, such as Rs thermal. Selection changes included noise, while the QPSS bias and circuit remain the same.");
    }
    switch_row(ui, "Integrated noise", &mut s.integrated_noise);
    if s.integrated_noise {
        let mut method = usize::from(s.integration_method == Integration::LogLog);
        choice_row(
            ui,
            "PSD interpolation",
            &["Linear", "Log-log power law"],
            &mut method,
        );
        s.integration_method = if method == 0 {
            Integration::Linear
        } else {
            Integration::LogLog
        };
        input_row(ui, "Band start (Hz, optional)", &mut s.band_start);
        input_row(ui, "Band stop (Hz, optional)", &mut s.band_stop);
        clear_pending_cell(ui);
        ui.small("Leave both limits empty for the full sweep. The sweep must cover the requested band. Undefined input samples are never bridged; log-log interpolation uses linear segments at zero frequency or density.");
    }
    switch_row(ui, "Rank noise contributors", &mut s.contributor_ranking);
    if s.integrated_noise || s.contributor_ranking {
        ui.small("For an autonomous oscillator, keep integration samples on one side of each oscillator spectral line. Integration across a free-phase pole is undefined.");
    }
    switch_row(ui, "Noise figure", &mut s.noise_figure);
    if s.noise_figure {
        input_row(ui, "Series source resistor", &mut s.source_resistor);
        input_row(
            ui,
            "Reference temperature (K)",
            &mut s.reference_temperature,
        );
        transfer_list(
            ui,
            "Reference tuples (optional, one per line)",
            &mut s.reference_lattices,
        );
        clear_pending_cell(ui);
        ui.small("Enable input referral to an ideal voltage source with its physical series resistor. Leave reference tuples empty for the signal's single-sideband reference, or list signal and image tuples for a summed reference. The selected noise window must include them.");
    }
    let mut method = match s.linear_method {
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
    s.linear_method = match method {
        1 => Method::Direct,
        2 => Method::Krylov,
        _ => Method::Auto,
    };
    input_row_enabled(
        ui,
        "Krylov restart vectors",
        &mut s.krylov_restart,
        s.linear_method != Method::Direct,
    );
    input_row_enabled(
        ui,
        "Krylov restart cycles",
        &mut s.krylov_cycles,
        s.linear_method != Method::Direct,
    );
    input_row(ui, "Relative adjoint tolerance", &mut s.linear_tolerance);
}
fn observation_fields(
    ui: &mut Ui,
    current: &mut bool,
    node: &mut String,
    reference: &mut String,
    branch: &mut String,
    lattice: &mut String,
) {
    let mut quantity = usize::from(*current);
    choice_row(
        ui,
        "Output quantity",
        &["Voltage", "Branch current"],
        &mut quantity,
    );
    *current = quantity == 1;
    if *current {
        input_row(ui, "Current branch", branch);
    } else {
        input_row(ui, "Output node", node);
        input_row(ui, "Reference node", reference);
    }
    input_row(ui, "Output tuple", lattice);
}
