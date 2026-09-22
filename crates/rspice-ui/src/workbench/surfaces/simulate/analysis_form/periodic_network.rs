//! The periodic small-signal network forms: PSP, HBSP and HB noise.
//!
//! PSP and HBSP are one form. Both drive a periodic operating point with a
//! small signal and report a scattering matrix over the sidebands; the only
//! difference between them is which periodic solve produced the point, which
//! the analysis kind already says. HB noise measures that same periodic point
//! instead of scattering off it, so it keeps rows of its own while sharing
//! the family's frequency axis.

use egui::Ui;

use crate::simulation::plan::{HbNoiseDraft, NetworkPortDraft, PeriodicNetworkDraft};

use super::{
    QuantityPresentationPolicy, UiNumberLocale, action_line, field_note, frequency_sweep_fields,
    input_row, sub_header, switch_row,
};

/// Render the scattering fields PSP and HBSP share.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut PeriodicNetworkDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
    field_note(
        ui,
        "Frequencies are offsets from the periodic carrier. Set start equal to stop for one spot frequency.",
    );
    input_row(ui, "Max sideband", &mut setup.max_sideband);
    input_row(ui, "Relative tolerance", &mut setup.reltol);
    input_row(ui, "Absolute tolerance", &mut setup.abstol);
    switch_row(ui, "Mixed-mode matrix", &mut setup.mixed_mode);
    switch_row(ui, "Noise correlations", &mut setup.noise_parameters);
    if setup.noise_parameters {
        field_note(
            ui,
            "Intrinsic noise-wave covariance is reported in W/Hz. External port termination noise is excluded from this matrix.",
        );
        switch_row(ui, "Noise parameters", &mut setup.noise.report_parameters);
        if setup.noise.report_parameters {
            input_row(ui, "Noise input port", &mut setup.noise.input_port);
            input_row(ui, "Noise output port", &mut setup.noise.output_port);
            input_row(ui, "Noise input sideband", &mut setup.noise.input_sideband);
            input_row(
                ui,
                "Noise output sideband",
                &mut setup.noise.output_sideband,
            );
            input_row(
                ui,
                "Reference temperature (K)",
                &mut setup.noise.reference_temperature,
            );
            input_row(
                ui,
                "Unused-channel temperature (K)",
                &mut setup.noise.termination_temperature,
            );
            input_row(
                ui,
                "DSB image sideband (optional)",
                &mut setup.noise.image_sideband,
            );
            field_note(
                ui,
                "PN_F, PN_Fmin, PN_Rn and PN_Sopt use the selected SSB input channel. Unused channels remain matched at their noise temperature; 0 K disables their noise. The selected output load contributes no noise. An image sideband adds DSB noise figure using both input power gains at the reference temperature.",
            );
            if setup.mixed_mode {
                field_note(
                    ui,
                    "Mixed-mode wave ports are numbered 1=d1, 2=c1, 3=d2, 4=c2, and so on. Each pair uses adjacent equal-impedance physical ports.",
                );
            }
            field_note(
                ui,
                "Channel frequency is offset + sideband × carrier. Select a conversion path with nonzero gain.",
            );
        }
    }
    let port_count = setup.ports.len();
    let mut remove = None;
    for (index, port) in setup.ports.iter_mut().enumerate() {
        network_port_fields(ui, index, port);
        if port_count > 1 && action_line(ui, "Remove port") {
            remove = Some(index);
        }
    }
    if let Some(index) = remove {
        setup.ports.remove(index);
    }
    if action_line(ui, "+ Add port") {
        setup.ports.push(NetworkPortDraft::default());
    }
}

/// One port of the scattering network: its two nodes and its reference Z0.
pub(super) fn network_port_fields(ui: &mut Ui, index: usize, port: &mut NetworkPortDraft) {
    sub_header(ui, &format!("Port {}", index + 1));
    input_row(ui, "Node +", &mut port.node_pos);
    input_row(ui, "Node −", &mut port.node_neg);
    input_row(ui, "Reference Z0", &mut port.z0);
}

/// Render the HB noise fields.
pub(super) fn noise_fields(
    ui: &mut Ui,
    setup: &mut HbNoiseDraft,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    frequency_sweep_fields(ui, &mut setup.sweep, policy, locale);
    input_row(ui, "Output", &mut setup.output_node);
    input_row(ui, "Output ref", &mut setup.output_ref);
    input_row(ui, "Input source", &mut setup.input_source);
    input_row(ui, "Input sideband", &mut setup.input_sideband);
    input_row(ui, "Output sideband", &mut setup.output_sideband);
    field_note(
        ui,
        "Frequencies are offsets. Each channel is offset + sideband × HB fundamental; negative frequencies represent the conjugate channel. Both sidebands must lie within the folding window.",
    );
    input_row(ui, "Max sideband", &mut setup.max_sideband);
    field_note(
        ui,
        "Zero sidebands selects the central band. For a spot frequency, set start equal to stop and disable integrated noise and contributor ranking.",
    );
    switch_row(ui, "Integrated noise", &mut setup.integrated_noise);
    switch_row(ui, "Noise figure", &mut setup.noise_figure);
    if setup.noise_figure {
        input_row(ui, "Source resistor", &mut setup.source_resistor);
        input_row(
            ui,
            "Reference temperature (K)",
            &mut setup.reference_temperature,
        );
        field_note(
            ui,
            "SSB noise figure uses the named series source resistor and the selected input-to-output conversion gain. The resistor must already be in the HB circuit. Reference temperature rescales its thermal noise only.",
        );
    }
    switch_row(ui, "Contributor ranking", &mut setup.contributor_ranking);
}
