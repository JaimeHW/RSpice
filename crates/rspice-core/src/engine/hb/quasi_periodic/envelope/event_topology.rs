//! Structural event rows for declared lumped storage and independent sources.
use super::*;
use crate::analysis::quasi_periodic::SpectralEnvelopeEventEquation as Row;
use crate::engine::periodic_capability::PeriodicDeviceFamily as Family;

fn root(parents: &mut [usize], mut node: usize) -> usize {
    while parents[node] != node {
        parents[node] = parents[parents[node]];
        node = parents[node];
    }
    node
}

pub(super) fn prepare(
    circuit: &CircuitData,
    unknowns: usize,
    abort: &dyn AbortSignal,
) -> Result<Vec<Row>, SimulationError> {
    check_abort(abort)?;
    // The existing spectral stepper supports additional nonlinear families.
    // Their event topology needs its own charge-port declarations; a sampled
    // Jacobian cannot certify those declarations.
    for family in Family::ALL {
        if family.instance_count(circuit) != 0
            && !matches!(
                family,
                Family::Resistor
                    | Family::ResistorBranch
                    | Family::Capacitor
                    | Family::Inductor
                    | Family::VoltageSource
                    | Family::CurrentSource
                    | Family::Vccs
                    | Family::InductorCoupling
                    | Family::CoupledInductorPair
                    | Family::Diode
            )
        {
            return Err(invalid(format!(
                "{} require declared spectral event charge ports",
                family.label()
            )));
        }
    }
    if circuit.resistors.thermal.iter().any(Option::is_some)
        || circuit
            .capacitors
            .value_expressions
            .iter()
            .any(Option::is_some)
        || circuit
            .capacitors
            .ic_branch_indices
            .iter()
            .any(Option::is_some)
        || unknowns != circuit.matrix_size()
    {
        return Err(invalid(
            "thermal or auxiliary storage requires declared spectral event equations",
        ));
    }
    let nodes = circuit.num_nodes();
    let mut parents: Vec<_> = (0..=nodes).collect();
    let mut rows: Vec<Option<Row>> = (0..unknowns)
        .map(|row| (row < nodes).then_some(Row::Charge))
        .collect();
    let mut connect = |positive: usize, negative: usize| -> Result<(), SimulationError> {
        if positive > nodes || negative > nodes {
            return Err(invalid("event port is outside the prepared node basis"));
        }
        let p = root(&mut parents, positive);
        let n = root(&mut parents, negative);
        parents[p.max(n)] = p.min(n);
        Ok(())
    };
    let claim =
        |rows: &mut [Option<Row>], branch: usize, row: Row| -> Result<(), SimulationError> {
            let coordinate = branch
                .checked_sub(1)
                .and_then(|index| nodes.checked_add(index))
                .filter(|index| *index < unknowns)
                .ok_or_else(|| invalid("event branch is outside the prepared MNA basis"))?;
            if rows[coordinate].replace(row).is_some() {
                return Err(invalid("event branch has multiple physical owners"));
            }
            Ok(())
        };
    let source_row = |p: usize, n: usize| Row::VoltageSource {
        positive: p.checked_sub(1),
        negative: n.checked_sub(1),
    };
    for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
        check_abort(abort)?;
        if circuit.capacitors.capacitances[index] != 0.0 {
            connect(stamp.pp.row, stamp.nn.row)?;
        }
    }
    for diode in &circuit.diodes.devices {
        check_abort(abort)?;
        if diode.has_charge_storage() {
            // Use the native junction terminals, including an elaborated
            // internal anode when series resistance is present. Charge-free
            // junctions must not join otherwise independent storage groups.
            connect(diode.node_anode, diode.node_cathode)?;
        }
    }
    for index in 0..circuit.voltage_sources.len() {
        check_abort(abort)?;
        let source = &circuit.voltage_sources;
        connect(source.node_pos[index], source.node_neg[index])?;
        claim(
            &mut rows,
            source.branch_indices[index],
            source_row(source.node_pos[index], source.node_neg[index]),
        )?;
    }
    for index in 0..circuit.inductors.len() {
        check_abort(abort)?;
        let inductor = &circuit.inductors;
        let row = if inductor.inductances[index] == 0.0 {
            connect(inductor.node_pos[index], inductor.node_neg[index])?;
            source_row(inductor.node_pos[index], inductor.node_neg[index])
        } else {
            Row::Charge
        };
        claim(&mut rows, inductor.branch_indices[index], row)?;
    }
    for index in 0..circuit.resistor_branches.len() {
        check_abort(abort)?;
        let resistor = &circuit.resistor_branches;
        let row = if resistor.resistances[index] == 0.0 {
            connect(resistor.node_pos[index], resistor.node_neg[index])?;
            source_row(resistor.node_pos[index], resistor.node_neg[index])
        } else {
            Row::Algebraic(vec![(nodes + resistor.branch_indices[index] - 1, 1.0)])
        };
        claim(&mut rows, resistor.branch_indices[index], row)?;
    }
    let mut groups = vec![Vec::new(); nodes + 1];
    for node in 1..=nodes {
        check_abort(abort)?;
        groups[root(&mut parents, node)].push((node - 1, 1.0));
    }
    for (group, terms) in groups.into_iter().enumerate().skip(1) {
        if !terms.is_empty() {
            rows[group - 1] = Some(Row::Algebraic(terms));
        }
    }
    rows.into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| invalid("MNA coordinate has no declared spectral event owner"))
}
