//! Shared exact-MNA observation and independent-source binding for translated analyses.
use super::*;
fn invalid(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("quasi-periodic binding: {}", message.into()))
}

pub(super) fn output(
    netlist: &Netlist,
    point: &QpssOperatingPoint,
    output: &QpxfOutput,
) -> Result<Vec<(usize, Complex64)>, SimulationError> {
    output_from_names(
        netlist.ground_policy(),
        point.node_names(),
        point.branch_names(),
        output,
    )
}

pub(super) fn output_from_names(
    ground_policy: crate::netlist::GroundPolicy,
    node_names: &[String],
    branch_names: &[String],
    output: &QpxfOutput,
) -> Result<Vec<(usize, Complex64)>, SimulationError> {
    let mut rows = match output {
        QpxfOutput::Voltage { positive, negative } => {
            let node = |name: &str| -> Result<Option<usize>, SimulationError> {
                if ground_policy.is_ground(name.trim()) {
                    return Ok(None);
                }
                node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name.trim()))
                    .map(Some)
                    .ok_or_else(|| {
                        invalid(format!("output node '{name}' is absent from the circuit"))
                    })
            };
            let positive = node(positive)?;
            let negative = node(negative)?;
            if positive == negative {
                return Err(invalid("output and reference must be distinct nodes"));
            }
            let mut rows = Vec::new();
            if let Some(row) = positive {
                rows.push((row, Complex64::ONE));
            }
            if let Some(row) = negative {
                rows.push((row, -Complex64::ONE));
            }
            rows
        }
        QpxfOutput::BranchCurrent { branch } => {
            let row=branch_names.iter().position(|n|n.eq_ignore_ascii_case(branch.trim())).ok_or_else(||invalid(format!("output '{branch}' is not a retained MNA current; select a voltage-source/current-probe, inductor or another exact branch current")))?;
            vec![(node_names.len() + row, Complex64::ONE)]
        }
    };
    rows.sort_by_key(|(row, _)| *row);
    Ok(rows)
}

pub(super) fn input(
    circuit: &CircuitData,
    solver: &HbSolver,
    name: &str,
    node_count: usize,
) -> Result<QpxfInputSource, SimulationError> {
    let source = Engine::pac_input_port(circuit, name, node_count)?;
    let (quantity, injections, name) = if let Some(index) = source.voltage_source_index {
        let row = solver
            .periodic_voltage_source_branch(index)
            .ok_or_else(|| invalid("input voltage source has no exact MNA branch"))?;
        (
            QpxfQuantity::Voltage,
            vec![(node_count + row, Complex64::ONE)],
            circuit.voltage_sources.names[index].clone(),
        )
    } else {
        let canonical = circuit
            .current_sources
            .names
            .iter()
            .find(|n| n.eq_ignore_ascii_case(name.trim()))
            .expect("bound current source");
        (
            QpxfQuantity::Current,
            source.node_injections,
            canonical.clone(),
        )
    };
    Ok(QpxfInputSource {
        name,
        quantity,
        injections,
    })
}
