//! Eliminate a charge-free sensing cutset using its canonical physical KCL.
use super::*;
use crate::device::MatrixStamper;

#[derive(Clone, Copy)]
pub(super) struct CurrentRow {
    pub source: usize,
    pub node: usize,
    pub sign: Value,
}

impl PreparedEventCircuit<'_> {
    /// With exactly one ideal voltage branch and no charge or other branch
    /// currents at a node, KCL gives I_control = -sign * F_node(V,t).
    /// The same F/Jacobian/time partial used by the event solver owns this
    /// reduction; it is not a second implementation of the load's equations.
    pub(super) fn ccvs_current_row(
        circuit: &crate::CircuitData,
        source: usize,
    ) -> Option<CurrentRow> {
        let control = *circuit.ccvs.ctrl_branch.get(source)?;
        let table = &circuit.voltage_sources;
        if control == 0
            || control > circuit.num_branches()
            || !circuit.ccvs.transresistances.get(source)?.is_finite()
        {
            return None;
        }
        // Capability checks also run before preparation validates the SoA
        // tables. Malformed storage must refuse admission without indexing it.
        for (count, positive, negative) in [
            (table.len(), &table.node_pos, &table.node_neg),
            (
                circuit.vcvs.len(),
                &circuit.vcvs.node_pos,
                &circuit.vcvs.node_neg,
            ),
            (
                circuit.ccvs.len(),
                &circuit.ccvs.node_pos,
                &circuit.ccvs.node_neg,
            ),
            (
                circuit.resistor_branches.len(),
                &circuit.resistor_branches.node_pos,
                &circuit.resistor_branches.node_neg,
            ),
            (
                circuit.inductors.len(),
                &circuit.inductors.node_pos,
                &circuit.inductors.node_neg,
            ),
            (
                circuit.cccs.len(),
                &circuit.cccs.node_pos,
                &circuit.cccs.node_neg,
            ),
        ] {
            if positive.len() != count || negative.len() != count {
                return None;
            }
        }
        let capacitors = &circuit.capacitors;
        if table.branch_indices.len() != table.len()
            || circuit.cccs.gains.len() != circuit.cccs.len()
            || [
                capacitors.stamps.len(),
                capacitors.capacitances.len(),
                capacitors.value_expressions.len(),
                capacitors.ic_branch_indices.len(),
            ]
            .into_iter()
            .any(|length| length != capacitors.len())
        {
            return None;
        }
        let probe = table
            .branch_indices
            .iter()
            .position(|&branch| branch == control)?;
        let touches = |node, p, n| node == p || node == n;
        for (node, sign) in [(table.node_pos[probe], 1.0), (table.node_neg[probe], -1.0)] {
            if node == 0
                || node > circuit.num_nodes()
                || table.node_pos[probe] == table.node_neg[probe]
            {
                continue;
            }
            // Never infer a charge-free node from a zero charge or derivative
            // at one trial point. These are the providers' storage predicates.
            let has_charge = capacitors.stamps.iter().enumerate().any(|(index, stamp)| {
                touches(node, stamp.pp.row, stamp.nn.row)
                    && (capacitors.capacitances[index] != 0.0
                        || capacitors.value_expressions[index].is_some()
                        || capacitors.ic_branch_indices[index].is_some())
            }) || circuit.diodes.devices.iter().any(|diode| {
                diode.has_charge_storage() && touches(node, diode.node_anode, diode.node_cathode)
            }) || circuit.bjts.devices.iter().any(|model| {
                model
                    .charge_storage_nodes()
                    .into_iter()
                    .flatten()
                    .any(|(p, n)| touches(node, p, n))
            });
            if has_charge {
                continue;
            }
            let other_voltage = table.branch_indices.iter().enumerate().any(|(index, _)| {
                index != probe && touches(node, table.node_pos[index], table.node_neg[index])
            }) || (0..circuit.vcvs.len()).any(|index| {
                touches(
                    node,
                    circuit.vcvs.node_pos[index],
                    circuit.vcvs.node_neg[index],
                )
            }) || (0..circuit.ccvs.len()).any(|index| {
                touches(
                    node,
                    circuit.ccvs.node_pos[index],
                    circuit.ccvs.node_neg[index],
                )
            }) || circuit
                .behavioral_sources
                .voltage_sources
                .iter()
                .any(|source| touches(node, source.node_pos, source.node_neg));
            if other_voltage {
                continue;
            }
            // These providers introduce a current coordinate or a delayed port,
            // whose full descriptor is not a nodal voltage equation.
            let other_current = (0..circuit.resistor_branches.len()).any(|index| {
                touches(
                    node,
                    circuit.resistor_branches.node_pos[index],
                    circuit.resistor_branches.node_neg[index],
                )
            }) || (0..circuit.inductors.len()).any(|index| {
                touches(
                    node,
                    circuit.inductors.node_pos[index],
                    circuit.inductors.node_neg[index],
                )
            }) || (0..circuit.cccs.len()).any(|index| {
                circuit.cccs.gains[index] != 0.0
                    && touches(
                        node,
                        circuit.cccs.node_pos[index],
                        circuit.cccs.node_neg[index],
                    )
            }) || circuit.bjts.devices.iter().any(|model| {
                model.needs_mna_rbi_branch() && touches(node, model.node_bx, model.node_bi)
            }) || circuit.tlines.iter().any(|line| {
                touches(node, line.node1_pos, line.node1_neg)
                    || touches(node, line.node2_pos, line.node2_neg)
            });
            if !other_current {
                return Some(CurrentRow { source, node, sign });
            }
        }
        None
    }

    pub(super) fn stamp_ccvs_current_rows(
        &self,
        sample: &mut EventSample,
        state: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<()> {
        let nodes = self.circuit.num_nodes();
        let sources = &self.circuit.ccvs;
        for control in &self.ccvs_current_rows {
            check_abort(abort)?;
            let index = control.source;
            let row = nodes + sources.branch_indices[index];
            let input = control.node - 1;
            if sample.q.values[input] != 0.0
                || sample.q_time[input] != 0.0
                || sample.q.rows[input]
                    .iter()
                    .any(|&(_, coefficient)| coefficient != 0.0)
            {
                return Err(error("CCVS sensing cutset has physical charge storage"));
            }
            // Vp - Vn - Rm*I_control = Vp - Vn + sign*Rm*F_node.
            let scale = control.sign * sources.transresistances[index];
            for (node, coefficient) in [
                (sources.node_pos[index], 1.0),
                (sources.node_neg[index], -1.0),
            ] {
                sample.f.stamp(row, node, coefficient);
                sample.f.stamp_rhs(row, -coefficient * voltage(state, node));
            }
            sample.f.stamp_rhs(
                row,
                -sum([(sample.f.values[input], scale)].into_iter()).unwrap_or(Value::NAN),
            );
            sample.f.scales[row - 1] = sample.f.scales[row - 1].max(
                sum([(sample.f.scales[input], scale.abs())].into_iter()).unwrap_or(Value::NAN),
            );
            for position in 0..sample.f.rows[input].len() {
                if position.is_multiple_of(64) {
                    check_abort(abort)?;
                }
                let (column, coefficient) = sample.f.rows[input][position];
                if column >= nodes && coefficient != 0.0 {
                    return Err(error(
                        "CCVS sensing cutset has a non-nodal current dependency",
                    ));
                }
                sample.f.stamp(
                    row,
                    column + 1,
                    sum([(coefficient, scale)].into_iter()).unwrap_or(Value::NAN),
                );
            }
            sample.f_time[row - 1] =
                sum([(sample.f_time[input], scale)].into_iter()).unwrap_or(Value::NAN);
        }
        Ok(())
    }
}
