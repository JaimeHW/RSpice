//! Explicit scalar rational code-model equations, without transfer cancellation.
use super::*;
use crate::xspice::{
    PortConnection, PortType, XspiceInstance, XspiceRationalTransfer, XspiceSmallSignalDescriptor,
};
use std::sync::Arc;

type PortTerms = [Option<(usize, Value)>; 2];

pub(super) struct RationalDescriptor<'a> {
    instance: &'a XspiceInstance,
    coefficients: Arc<XspiceRationalTransfer>,
    input: PortTerms,
    output: PortTerms,
    voltage_output: Option<(usize, usize, usize)>,
}

fn node_pair(pos: usize, neg: usize) -> PortTerms {
    [
        pos.checked_sub(1).map(|row| (row, 1.0)),
        neg.checked_sub(1).map(|row| (row, -1.0)),
    ]
}

fn invalid(instance: &XspiceInstance, detail: &str) -> SimulationError {
    SimulationError::Circuit(format!("XSPICE instance '{}': {detail}", instance.name))
}

impl<'a> RationalDescriptor<'a> {
    pub(super) fn collect(circuit: &'a CircuitData) -> Result<Vec<Self>, SimulationError> {
        let mut descriptors = Vec::new();
        for instance in &circuit.xspice_instances {
            let XspiceSmallSignalDescriptor::Rational {
                input_port,
                output_port,
                coefficients,
            } = instance
                .small_signal_descriptor()
                .map_err(|error| invalid(instance, &error.to_string()))?
            else {
                continue;
            };
            if coefficients.numerator.is_empty()
                || coefficients.denominator.is_empty()
                || coefficients.numerator.len() > coefficients.denominator.len()
                || coefficients.denominator.last() == Some(&0.0)
                || !coefficients.gain.is_finite()
                || !coefficients
                    .numerator
                    .iter()
                    .chain(&coefficients.denominator)
                    .all(|v| v.is_finite())
            {
                return Err(invalid(
                    instance,
                    "invalid rational descriptor coefficients",
                ));
            }
            let input = match instance.connection(input_port) {
                Some(PortConnection::Analog(node)) => node_pair(*node, 0),
                Some(PortConnection::Differential(pos, neg)) => node_pair(*pos, *neg),
                Some(
                    PortConnection::CurrentProbe { branch_ordinal, .. }
                    | PortConnection::BranchCurrent { branch_ordinal }
                    | PortConnection::Hybrid { branch_ordinal, .. }
                    | PortConnection::NamedBranchCurrent {
                        branch_ordinal: Some(branch_ordinal),
                        ..
                    },
                ) if *branch_ordinal > 0 => [
                    Some((circuit.get_branch_matrix_index(*branch_ordinal) - 1, 1.0)),
                    None,
                ],
                _ => {
                    return Err(invalid(
                        instance,
                        "rational descriptor requires a resolved scalar input",
                    ));
                }
            };
            let port_index = instance
                .ports()
                .iter()
                .position(|port| port.name.eq_ignore_ascii_case(output_port))
                .ok_or_else(|| invalid(instance, "rational descriptor output port is missing"))?;
            let connection = instance.connection(output_port).ok_or_else(|| {
                invalid(instance, "rational descriptor output connection is missing")
            })?;
            let (pos, neg) = match connection {
                PortConnection::Analog(node) => (*node, 0),
                PortConnection::Differential(pos, neg)
                | PortConnection::CurrentOutput { pos, neg }
                | PortConnection::Hybrid { pos, neg, .. } => (*pos, *neg),
                _ => {
                    return Err(invalid(
                        instance,
                        "rational descriptor requires a scalar output",
                    ));
                }
            };
            let current_output = matches!(connection, PortConnection::CurrentOutput { .. })
                || matches!(
                    instance.ports()[port_index].default_type,
                    PortType::Current | PortType::DifferentialCurrent
                );
            let (output, voltage_output) = if current_output {
                (node_pair(pos, neg), None)
            } else {
                let branch = instance
                    .branch_ordinal_at(port_index)
                    .filter(|ordinal| *ordinal > 0)
                    .ok_or_else(|| {
                        invalid(instance, "rational voltage output branch is missing")
                    })?;
                let row = circuit.get_branch_matrix_index(branch) - 1;
                ([Some((row, -1.0)), None], Some((pos, neg, row)))
            };
            descriptors.push(Self {
                instance,
                coefficients,
                input,
                output,
                voltage_output,
            });
        }
        Ok(descriptors)
    }

    pub(super) fn state_count(&self) -> usize {
        // Constant transfers use one algebraic coordinate to avoid forming a
        // potentially overflowing product of gain and numerator coefficients.
        self.coefficients.denominator.len().saturating_sub(1).max(1)
    }

    pub(super) fn stamp_all(
        descriptors: &[Self],
        circuit: &CircuitData,
        g: &mut Matrix,
        c: &mut Matrix,
    ) {
        let count = descriptors
            .iter()
            .map(Self::state_count)
            .fold(0usize, usize::saturating_add);
        if count == 0 {
            return;
        }
        let mut first = Engine::descriptor_expand_square(g, c, count);
        for descriptor in descriptors {
            for (pos, neg, branch) in descriptor.instance.current_probe_branches() {
                stamp_branch(g, pos, neg, circuit.get_branch_matrix_index(branch) - 1);
            }
            if let Some((pos, neg, row)) = descriptor.voltage_output {
                stamp_branch(g, pos, neg, row);
            }
            let coefficients = &descriptor.coefficients;
            let order = coefficients.denominator.len() - 1;
            let last = first + order.saturating_sub(1);
            // x[k+1] = s*x[k]; D(s)*x[0] = gain*input.
            // The highest numerator term is stamped on C directly. This
            // avoids polynomial division and preserves unobservable modes.
            for index in 0..order.saturating_sub(1) {
                c.add(first + index, first + index, 1.0);
                g.add(first + index, first + index + 1, -1.0);
            }
            for (index, &coefficient) in coefficients.denominator.iter().enumerate() {
                if order > 0 && index == order {
                    c.add(last, last, coefficient);
                } else {
                    g.add(last, first + index, coefficient);
                }
            }
            for &(column, sign) in descriptor.input.iter().flatten() {
                g.add(last, column, -sign * coefficients.gain);
            }
            for (index, &coefficient) in coefficients.numerator.iter().enumerate() {
                for &(row, sign) in descriptor.output.iter().flatten() {
                    if order > 0 && index == order {
                        c.add(row, last, sign * coefficient);
                    } else {
                        g.add(row, first + index, sign * coefficient);
                    }
                }
            }
            first += descriptor.state_count();
        }
    }
}

fn stamp_branch(g: &mut Matrix, pos: usize, neg: usize, branch: usize) {
    for (node, sign) in node_pair(pos, neg).into_iter().flatten() {
        g.add(branch, node, sign);
        g.add(node, branch, sign);
    }
}
