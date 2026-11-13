//! The S-parameter form: the frequency band, the reference impedance, and the
//! ports, from whichever of the design's two declarations owns them.

use egui::Ui;

use crate::quantity::QuantityInputKind;
use crate::simulation::dialog::{SpDialogState, TOUCHSTONE_VERSION_LABELS};
use crate::simulation::placed_sources::PlacedRfPort;

use super::{
    QuantityPresentationPolicy, SWEEP_KINDS, UiNumberLocale, action_line, choice_row,
    field_advisory, input_row, property_row, quantity_input_row, sub_header,
    sweep_point_field_label, switch_row,
};

/// The S-parameter run's ports, from whichever of the two declarations owns
/// them.
///
/// A port is a Z0 plane the run drives and measures, and a design can declare
/// one in two places: `RF Port` components on the sheet, which the netlist
/// carries as `P` cards, or node pairs typed here, which the runner
/// materializes only for a deck that declares none of its own
/// (`services::simulation_runner::sparameter::resolve_ports`). Both at once is
/// not a richer setup, it is two answers to one question — so this row picks
/// the owner, and the other declaration goes quiet rather than half-applying.
pub(super) fn sp_port_fields(
    ui: &mut Ui,
    setup: &mut crate::simulation::dialog::SpDialogState,
    placed: &[crate::simulation::placed_sources::PlacedRfPort],
) {
    use crate::simulation::dialog::SpPortSource;

    let labels: Vec<&str> = SpPortSource::ALL
        .iter()
        .map(|source| source.display_name())
        .collect();
    let resolved = setup.port_source(placed.len());
    let mut selected = resolved.index();
    choice_row(ui, "Ports", &labels, &mut selected);
    // Written back only when the reader actually moves it. Stamping the
    // resolved value every frame would record a choice on a project that never
    // made one, and pin it to whatever the sheet happened to hold the first
    // time this form was drawn.
    if selected != resolved.index() {
        setup.port_source_idx = Some(selected);
    }

    let source = setup.port_source(placed.len());
    if source == SpPortSource::Placed {
        // Read-only: these are the design's, and an editable copy of them here
        // is the second declaration this switch exists to remove.
        for port in placed {
            sub_header(ui, &format!("Port {}", port.port_number));
            property_row(ui, "Instance", &port.reference);
            property_row(ui, "Role", &port.summary());
        }
    } else {
        let mut remove: Option<usize> = None;
        let port_count = setup.ports.len();
        for (idx, port) in setup.ports.iter_mut().enumerate() {
            sub_header(ui, &format!("Port {}", idx + 1));
            input_row(ui, "Node +", &mut port.node_pos);
            switch_row(ui, "Differential", &mut port.differential);
            if port.differential {
                input_row(ui, "Node −", &mut port.node_neg);
            }
            switch_row(ui, "Z0 override", &mut port.z0_override);
            if port.z0_override {
                input_row(ui, "Port Z0", &mut port.z0);
            }
            if port_count > 1 && action_line(ui, "Remove port") {
                remove = Some(idx);
            }
        }
        if let Some(idx) = remove {
            setup.ports.remove(idx);
        }
        ui.add_space(4.0);
        if action_line(ui, "+ Add port") {
            setup.ports.push(Default::default());
        }
    }

    // The same resolution dispatch performs, stated here so the reason a run
    // will be refused is visible beside the ports it is about.
    if let Some(reason) = crate::simulation::dialog::sp::port_roster_error(setup, placed) {
        field_advisory(ui, &reason);
    }
}

/// Render the S-parameter fields.
pub(super) fn fields(
    ui: &mut Ui,
    setup: &mut SpDialogState,
    placed_rf_ports: &[PlacedRfPort],
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) {
    quantity_input_row(
        ui,
        "Start",
        &mut setup.start_freq,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    quantity_input_row(
        ui,
        "Stop",
        &mut setup.stop_freq,
        QuantityInputKind::Frequency,
        policy,
        locale,
    );
    input_row(
        ui,
        sweep_point_field_label(setup.sweep_type_idx),
        &mut setup.num_points,
    );
    choice_row(ui, "Sweep", SWEEP_KINDS, &mut setup.sweep_type_idx);
    input_row(ui, "Z0", &mut setup.z0);
    switch_row(ui, "Noise parameters", &mut setup.do_noise);
    switch_row(ui, "Touchstone export", &mut setup.touchstone_export);
    // The version belongs to the export, so it is painted only when there is
    // an export to write. Greying it would announce a choice about a file that
    // is not produced; the export switch above already answers that.
    if setup.touchstone_export {
        let mut version_idx = setup.touchstone_version_index();
        if choice_row(ui, "Version", &TOUCHSTONE_VERSION_LABELS, &mut version_idx) {
            setup.set_touchstone_version_index(version_idx);
        }
    }
    sp_port_fields(ui, setup, placed_rf_ports)
}
