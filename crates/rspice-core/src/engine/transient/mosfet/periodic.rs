//! Accepted classic MOS histories reconstructed from a periodic orbit.
use super::*;

impl Engine {
    pub(in crate::engine) fn initialize_periodic_mosfet_history(
        circuit: &mut crate::circuit::CircuitData,
        solutions: [&[Value]; 3],
        node_rates: &[Value],
        history_step: Value,
        increments: &[MosfetPeriodicChargeIncrements],
    ) -> Result<MosfetTransientHistory, String> {
        if solutions.iter().any(|values| {
            values.len() != circuit.matrix_size() || values.iter().any(|value| !value.is_finite())
        }) || node_rates.len() != circuit.num_nodes()
            || node_rates.iter().any(|value| !value.is_finite())
            || increments.len() != circuit.mosfets.len()
            || !(history_step.is_finite() && history_step > 0.0)
        {
            return Err("periodic classic MOS history has invalid samples or rates".into());
        }
        // Establish the accepted raw bias before the ordinary history allocator
        // reads cached voltages; authored OFF/IC startup must not replace it.
        for device in &mut circuit.mosfets.devices {
            device.seed_accepted_periodic_bias(solutions[2]);
        }
        let mut history =
            Self::initialize_mosfet_history(circuit, solutions[2], ReactiveHistorySeed::SolvedBias);
        let extended = circuit
            .mosfets
            .devices
            .iter()
            .any(|device| device.uses_legacy_bsim());
        let rate = |node: usize| node.checked_sub(1).map_or(0.0, |i| node_rates[i]);
        for (i, device) in circuit.mosfets.devices.iter().enumerate() {
            let bias = solutions.map(|solution| device.unlimited_branch_voltages_at(solution));
            let branch_voltages = bias.map(|(gs, ds, bs)| [gs, gs - ds, gs - bs]);
            let (vgs, vds, vbs) = bias[2];
            let control_rates = [
                rate(device.node_gate) - rate(device.node_source),
                rate(device.node_drain) - rate(device.node_source),
                rate(device.node_bulk) - rate(device.node_source),
            ];
            let (charges, currents) = if let Some(current) =
                device.legacy_gate_charge_at(vgs, vds, vbs)
            {
                let charges = bias
                    .map(|(gs, ds, bs)| device.legacy_gate_charge_at(gs, ds, bs).unwrap().charges);
                let currents = current.derivatives.map(|gradient| {
                    gradient
                        .into_iter()
                        .zip(control_rates)
                        .map(|(c, rate)| c * rate)
                        .sum()
                });
                (charges, currents)
            } else {
                // Absolute Meyer charge is a gauge. Differences are actual
                // integrals along the retained orbit, never instantaneous C*V.
                let delta = increments[i];
                let charges = [
                    std::array::from_fn(|j| {
                        -delta.previous_to_current[j] - delta.older_to_previous[j]
                    }),
                    delta.previous_to_current.map(|q| -q),
                    [0.0; 3],
                ];
                let (gs, gd, gb) = device.ac_capacitances_at(vgs, vds, vbs);
                (
                    charges,
                    [
                        gs * control_rates[0],
                        gd * (control_rates[0] - control_rates[1]),
                        gb * (control_rates[0] - control_rates[2]),
                    ],
                )
            };
            for (
                port,
                (voltage, previous_voltage, charge, previous_charge, older_charge, current),
            ) in [
                (
                    &mut history.vgs_prev,
                    &mut history.vgs_prev_prev,
                    &mut history.qgs_prev,
                    &mut history.qgs_prev_prev,
                    &mut history.qgs_prev_prev_prev,
                    &mut history.cqgs_prev,
                ),
                (
                    &mut history.vgd_prev,
                    &mut history.vgd_prev_prev,
                    &mut history.qgd_prev,
                    &mut history.qgd_prev_prev,
                    &mut history.qgd_prev_prev_prev,
                    &mut history.cqgd_prev,
                ),
                (
                    &mut history.vgb_prev,
                    &mut history.vgb_prev_prev,
                    &mut history.qgb_prev,
                    &mut history.qgb_prev_prev,
                    &mut history.qgb_prev_prev_prev,
                    &mut history.cqgb_prev,
                ),
            ]
            .into_iter()
            .enumerate()
            {
                voltage[i] = branch_voltages[2][port];
                previous_voltage[i] = branch_voltages[1][port];
                charge[i] = charges[2][port];
                previous_charge[i] = charges[1][port];
                older_charge[i] = charges[0][port];
                current[i] = currents[port];
                history.accepted_displacement_currents[i][port] = currents[port];
            }
            for (
                port,
                (voltage, previous_voltage, charge, previous_charge, older_charge, current),
            ) in [
                (
                    &mut history.vbs_j_prev,
                    &mut history.vbs_j_prev_prev,
                    &mut history.qbs_prev,
                    &mut history.qbs_prev_prev,
                    &mut history.qbs_prev_prev_prev,
                    &mut history.cqbs_prev,
                ),
                (
                    &mut history.vbd_j_prev,
                    &mut history.vbd_j_prev_prev,
                    &mut history.qbd_prev,
                    &mut history.qbd_prev_prev,
                    &mut history.qbd_prev_prev_prev,
                    &mut history.cqbd_prev,
                ),
            ]
            .into_iter()
            .enumerate()
            {
                let source = port == 0;
                let q = bias.map(|(_, ds, bs)| {
                    if source {
                        device.body_source_junction_charge_and_capacitance_at(bs)
                    } else {
                        device.body_drain_junction_charge_and_capacitance_at(ds, bs)
                    }
                });
                let v = bias.map(|(_, ds, bs)| {
                    if source {
                        device.body_source_charge_branch_voltage(bs)
                    } else {
                        device.body_drain_charge_branch_voltage(ds, bs)
                    }
                });
                let (pos, neg) = if source {
                    device.body_source_charge_nodes()
                } else {
                    device.body_drain_charge_nodes()
                };
                voltage[i] = v[2];
                previous_voltage[i] = v[1];
                charge[i] = q[2].0;
                previous_charge[i] = q[1].0;
                if extended {
                    older_charge[i] = q[0].0;
                }
                current[i] = q[2].1 * (rate(pos) - rate(neg));
                history.accepted_displacement_currents[i][3 + port] = current[i];
            }
        }
        history.accepted_dt_prev = history_step;
        history.accepted_dt_prev_prev = history_step;
        history.validate(circuit.mosfets.len(), extended)?;
        Ok(history)
    }
}
