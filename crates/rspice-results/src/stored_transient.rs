//! A transient's rows and events read back from the results-store segment its run wrote, into
//! the engine's own result.
//!
//! A transient run into a result sink returns its result as a shell: everything but the time
//! axis, the saved columns and the event traces, which went to the segment as the run accepted
//! its points. This maps each value slot of the segment's rows to the result column it came
//! from, and moves rows and events back into the shell, so every reader of a stored run (the
//! command line's outputs, the simulation crate's post-processing) reads it the same way.

use rspice_core::analysis::transient::TransientResult;
use rspice_results_store::{SegmentIo, SegmentReader, StoreError};

/// Where one value slot of a row belongs in the in-memory result.
#[derive(Debug, Clone, Copy)]
pub enum SlotTarget {
    /// A node's voltage: its index in the result's nodes.
    Voltage(usize),
    /// A branch's current: its index in the result's branches.
    Current(usize),
    /// An instance parameter the run kept as a constant column: its index in
    /// [`instance_parameters`].
    Parameter(usize),
    /// The step that reached each point.
    StepSize,
    /// A store trace (a memristor's resistance): its index in the result's store traces.
    Store(usize),
    /// A device operating-point value: its index in the result's device traces.
    DeviceOp(usize),
}

impl SlotTarget {
    /// The result column this slot's values fill; an instance parameter's is adopted whole.
    pub fn column(self, result: &mut TransientResult) -> Option<&mut Vec<f64>> {
        match self {
            SlotTarget::Voltage(node) => result.voltages.get_mut(node),
            SlotTarget::Current(branch) => result.branch_currents.get_mut(branch),
            SlotTarget::StepSize => Some(&mut result.step_sizes),
            SlotTarget::Store(index) => result
                .store_traces
                .get_mut(index)
                .map(|trace| &mut trace.values),
            SlotTarget::DeviceOp(index) => result
                .device_op_traces
                .get_mut(index)
                .map(|trace| &mut trace.values),
            SlotTarget::Parameter(_) => None,
        }
    }
}

/// Read the run's event signals back into the traces the result names: its digital and real
/// event traces and its impulse traces, each of which the run moved to the store as it went.
pub fn fill_events<I: SegmentIo>(
    reader: &mut SegmentReader<I>,
    shell: &mut TransientResult,
) -> Result<(), StoreError> {
    use rspice_core::analysis::transient::{DigitalTracePoint, RealTracePoint};
    use rspice_core::xspice::DigitalValue;
    use rspice_core::{
        CurrentImpulseOwner, CurrentImpulsePoint, ImpulseDerivative, VoltageImpulsePoint,
    };
    use rspice_results_store::EventValue;
    let events: Vec<(usize, String, String, String)> = reader
        .schema()
        .signals
        .iter()
        .enumerate()
        .filter(|(_, signal)| signal.kind.slots() == 0)
        .map(|(id, signal)| {
            (
                id,
                signal.quantity.clone(),
                signal.name.clone(),
                signal.path.clone(),
            )
        })
        .collect();
    let malformed = |what: &str, name: &str| StoreError::Io {
        kind: std::io::ErrorKind::InvalidData,
        message: format!("the transient store's {what} signal '{name}' holds a change it cannot"),
    };
    for (id, quantity, name, path) in events {
        let changes = reader.events(id, f64::NEG_INFINITY, f64::INFINITY)?;
        let real = |value: &EventValue| match value {
            EventValue::Real(value) => Some(*value),
            _ => None,
        };
        let current = |owner: &CurrentImpulseOwner| match owner {
            CurrentImpulseOwner::Branch { branch_name } => path.is_empty() && *branch_name == name,
            CurrentImpulseOwner::DeviceLead {
                device_name,
                parameter,
            } => *device_name == path && *parameter == name,
        };
        match quantity.as_str() {
            "digital" => {
                let Some(trace) = shell
                    .digital_traces
                    .iter_mut()
                    .find(|t| t.node_name == name)
                else {
                    continue;
                };
                trace.points = changes
                    .iter()
                    .map(|change| {
                        DigitalValue::from_store_strength(change.strength)
                            .map(|value| DigitalTracePoint {
                                time: change.time,
                                value,
                            })
                            .ok_or_else(|| malformed("digital", &name))
                    })
                    .collect::<Result<_, _>>()?;
            }
            "real_event" => {
                let Some(trace) = shell.real_traces.iter_mut().find(|t| t.node_name == name) else {
                    continue;
                };
                trace.points = changes
                    .iter()
                    .map(|change| {
                        real(&change.value)
                            .map(|value| RealTracePoint {
                                time: change.time,
                                value,
                            })
                            .ok_or_else(|| malformed("real event", &name))
                    })
                    .collect::<Result<_, _>>()?;
            }
            "current_impulse" | "current_impulse_derivative" => {
                let Some(trace) = shell
                    .current_impulses
                    .iter_mut()
                    .flatten()
                    .find(|trace| current(&trace.owner))
                else {
                    continue;
                };
                if quantity == "current_impulse" {
                    trace.points = changes
                        .iter()
                        .map(|change| {
                            real(&change.value)
                                .map(|charge_coulombs| CurrentImpulsePoint {
                                    time: change.time,
                                    charge_coulombs,
                                })
                                .ok_or_else(|| malformed("impulse", &name))
                        })
                        .collect::<Result<_, _>>()?;
                } else {
                    trace.derivatives =
                        derivatives(&changes).ok_or_else(|| malformed("impulse", &name))?;
                }
            }
            "voltage_impulse" | "voltage_impulse_derivative" => {
                let Some(trace) = shell
                    .voltage_impulses
                    .iter_mut()
                    .flatten()
                    .find(|trace| trace.node_name == name)
                else {
                    continue;
                };
                if quantity == "voltage_impulse" {
                    trace.points = changes
                        .iter()
                        .map(|change| {
                            real(&change.value)
                                .map(|volt_seconds| VoltageImpulsePoint {
                                    time: change.time,
                                    volt_seconds,
                                })
                                .ok_or_else(|| malformed("impulse", &name))
                        })
                        .collect::<Result<_, _>>()?;
                } else {
                    trace.derivatives =
                        derivatives(&changes).ok_or_else(|| malformed("impulse", &name))?;
                }
            }
            _ => {}
        }
    }
    /// An impulse derivative's order is its change's sequence.
    fn derivatives(
        changes: &[rspice_results_store::EventChange],
    ) -> Option<Vec<ImpulseDerivative>> {
        changes
            .iter()
            .map(|change| match change.value {
                EventValue::Real(coefficient) => Some(ImpulseDerivative {
                    time: change.time,
                    order: change.sequence,
                    coefficient,
                }),
                _ => None,
            })
            .collect()
    }
    Ok(())
}

/// Each slot's place in the result: the store's voltages are the saved nodes in node order, then
/// its currents the saved branches in branch order, so each name is the next one of its kind.
pub fn slot_targets<I: SegmentIo>(
    reader: &SegmentReader<I>,
    shell: &TransientResult,
) -> Result<Vec<SlotTarget>, String> {
    let (mut next_node, mut next_branch, mut parameters) = (0usize, 0usize, 0usize);
    let mut targets = Vec::with_capacity(reader.slots());
    for signal in &reader.schema().signals {
        // An event signal takes no slot; its changes are read on their own.
        if signal.kind.slots() == 0 {
            continue;
        }
        if signal.quantity == "instance_parameter" {
            targets.push(SlotTarget::Parameter(parameters));
            parameters += 1;
            continue;
        }
        if signal.quantity == "step_size" {
            targets.push(SlotTarget::StepSize);
            continue;
        }
        if signal.quantity == "device_op" {
            let index = shell
                .device_op_traces
                .iter()
                .position(|trace| {
                    trace.device_name == signal.path && trace.parameter == signal.name
                })
                .ok_or_else(|| {
                    format!(
                        "the transient store's device value '{}' of '{}' is not a result device \
                         trace",
                        signal.name, signal.path
                    )
                })?;
            targets.push(SlotTarget::DeviceOp(index));
            continue;
        }
        if signal.quantity == "store" {
            let index = shell
                .store_traces
                .iter()
                .position(|trace| trace.name == signal.name)
                .ok_or_else(|| {
                    format!(
                        "the transient store's trace '{}' is not a result store trace",
                        signal.name
                    )
                })?;
            targets.push(SlotTarget::Store(index));
            continue;
        }
        let (names, next, voltage) = match signal.quantity.as_str() {
            "voltage" => (&shell.node_names, &mut next_node, true),
            "current" => (&shell.branch_names, &mut next_branch, false),
            other => {
                return Err(format!(
                    "the transient store holds a '{other}' signal '{}' the result has no column for",
                    signal.name
                ));
            }
        };
        let found = names[(*next).min(names.len())..]
            .iter()
            .position(|name| *name == signal.name)
            .map(|offset| *next + offset)
            .ok_or_else(|| {
                format!(
                    "the transient store's signal '{}' is not a result column",
                    signal.name
                )
            })?;
        *next = found + 1;
        targets.push(if voltage {
            SlotTarget::Voltage(found)
        } else {
            SlotTarget::Current(found)
        });
    }
    Ok(targets)
}

/// `(device, parameter)` of each instance-parameter column, in slot order: what
/// [`SlotTarget::Parameter`] indexes.
pub fn instance_parameters<I: SegmentIo>(reader: &SegmentReader<I>) -> Vec<(String, String)> {
    reader
        .schema()
        .signals
        .iter()
        .filter(|signal| signal.quantity == "instance_parameter")
        .map(|signal| (signal.path.clone(), signal.name.clone()))
        .collect()
}

/// Move rows `first .. first + count` into `result`'s time and saved columns.
pub fn fill_rows<I: SegmentIo>(
    reader: &mut SegmentReader<I>,
    targets: &[SlotTarget],
    result: &mut TransientResult,
    first: u64,
    count: usize,
) -> Result<(), StoreError> {
    result.time = reader.read_x(first, count)?;
    for (slot, &target) in targets.iter().enumerate() {
        let Some(column) = target.column(result) else {
            continue;
        };
        *column = reader.read_slot(slot, first, count)?;
    }
    Ok(())
}

/// Move the first and last rows into `result`'s time and saved columns: what a statement that
/// reads only event operands reads of the run, and the row a PARAM reads.
pub fn fill_span<I: SegmentIo>(
    reader: &mut SegmentReader<I>,
    targets: &[SlotTarget],
    result: &mut TransientResult,
) -> Result<(), StoreError> {
    let rows = reader.rows();
    let ends: &[u64] = if rows > 1 { &[0, rows - 1] } else { &[0] };
    result.time.clear();
    for &row in ends {
        result.time.extend(reader.read_x(row, 1)?);
    }
    for (slot, &target) in targets.iter().enumerate() {
        let Some(column) = target.column(result) else {
            continue;
        };
        column.clear();
        for &row in ends {
            column.extend(reader.read_slot(slot, row, 1)?);
        }
    }
    Ok(())
}

/// Empty `result`'s time and saved columns again, giving their memory back.
pub fn clear_rows(targets: &[SlotTarget], result: &mut TransientResult) {
    result.time = Vec::new();
    for &target in targets {
        if let Some(column) = target.column(result) {
            *column = Vec::new();
        }
    }
}

/// The first non-finite saved value in result order, as the in-memory check finds it: the first
/// slot that has one, at its first such row, as `(slot, row, value)`. Step sizes, store traces
/// and device values are not checked, as the in-memory check reads the saved waveforms and
/// parameters only.
pub fn first_non_finite<I: SegmentIo>(
    reader: &mut SegmentReader<I>,
    targets: &[SlotTarget],
) -> Result<Option<(usize, u64, f64)>, StoreError> {
    let mut first: Vec<Option<(u64, f64)>> = vec![None; targets.len()];
    let spans = reader.chunk_spans().collect::<Vec<_>>();
    let mut values = Vec::new();
    for (chunk, &(start, _)) in spans.iter().enumerate() {
        for (slot, found) in first.iter_mut().enumerate() {
            if found.is_some()
                || matches!(
                    targets[slot],
                    SlotTarget::StepSize | SlotTarget::Store(_) | SlotTarget::DeviceOp(_)
                )
            {
                continue;
            }
            reader.read_chunk_slot(chunk, slot, &mut values)?;
            if let Some(at) = values.iter().position(|value| !value.is_finite()) {
                *found = Some((start + at as u64, values[at]));
            }
        }
    }
    Ok(first
        .iter()
        .enumerate()
        .find_map(|(slot, found)| found.map(|(row, value)| (slot, row, value))))
}

/// The name a slot's values carry in `shell`, as a refusal names it.
pub fn slot_name<'a>(
    target: SlotTarget,
    shell: &'a TransientResult,
    parameters: &'a [(String, String)],
) -> &'a str {
    match target {
        SlotTarget::Voltage(node) => shell.node_names.get(node).map_or("node", String::as_str),
        SlotTarget::Current(branch) => shell
            .branch_names
            .get(branch)
            .map_or("branch", String::as_str),
        SlotTarget::Parameter(index) => parameters
            .get(index)
            .map_or("parameter", |(_, parameter)| parameter.as_str()),
        SlotTarget::StepSize => "step",
        SlotTarget::Store(index) => shell
            .store_traces
            .get(index)
            .map_or("store", |trace| trace.name.as_str()),
        SlotTarget::DeviceOp(index) => shell
            .device_op_traces
            .get(index)
            .map_or("device", |trace| trace.parameter.as_str()),
    }
}
