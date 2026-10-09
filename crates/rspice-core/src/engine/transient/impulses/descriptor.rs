//! Atomic publication of original-coordinate descriptor actions.
use super::*;
use crate::{ImpulseDerivative, VoltageImpulsePoint, VoltageImpulseTrace};

pub(super) struct Plan {
    nodes: usize,
    coordinates: usize,
    currents: Vec<Projection>,
}

enum Projection {
    Coordinate(usize),
    Difference {
        positive: usize,
        negative: usize,
        gain: Value,
    },
    Capacitor {
        index: usize,
        positive: usize,
        negative: usize,
        capacitance: Value,
    },
    Zero,
}

pub(super) fn initialize(
    circuit: &crate::CircuitData,
    derived: &[DerivedTransientBranchCurrent],
    result: &TransientResult,
    retained: usize,
    limits: &crate::resource::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(Plan, Vec<VoltageImpulseTrace>, usize), SimulationError> {
    let nodes = circuit.num_nodes();
    if result.node_names.len() != nodes {
        return Err(failure("unaligned voltage owners"));
    }
    let added = result
        .node_names
        .iter()
        .fold(0usize, |sum, name| {
            sum.saturating_add(1 + name.len().div_ceil(std::mem::size_of::<Value>()))
        })
        .saturating_add(result.branch_names.len().saturating_mul(6));
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::ResultValues,
        retained.saturating_add(added),
        limits.max_result_values,
    )?;
    let mut traces = allocated(nodes)?;
    for name in &result.node_names {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        traces.push(VoltageImpulseTrace {
            node_name: copied(name)?,
            complete: true,
            points: Vec::new(),
            derivatives: Vec::new(),
        });
    }
    let mut currents = allocated(result.branch_names.len())?;
    currents.extend((nodes..circuit.matrix_size()).map(Projection::Coordinate));
    for branch in derived {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let i = branch.index;
        currents.push(match branch.kind {
            DerivedTransientBranchCurrentKind::LinearResistor => {
                let stamp = &circuit.resistors.stamps[i];
                Projection::Difference {
                    positive: stamp.pp.row,
                    negative: stamp.nn.row,
                    gain: circuit.resistors.conductances[i],
                }
            }
            DerivedTransientBranchCurrentKind::VoltageControlledCurrentSource => {
                Projection::Difference {
                    positive: circuit.vccs.ctrl_pos[i],
                    negative: circuit.vccs.ctrl_neg[i],
                    gain: circuit.vccs.transconductances[i],
                }
            }
            DerivedTransientBranchCurrentKind::CurrentControlledCurrentSource => {
                Projection::Difference {
                    positive: nodes + circuit.cccs.ctrl_branch[i],
                    negative: 0,
                    gain: circuit.cccs.gains[i],
                }
            }
            DerivedTransientBranchCurrentKind::LinearCapacitor => {
                let stamp = &circuit.capacitors.stamps[i];
                Projection::Capacitor {
                    index: i,
                    positive: stamp.pp.row,
                    negative: stamp.nn.row,
                    capacitance: circuit.capacitors.capacitances[i],
                }
            }
            DerivedTransientBranchCurrentKind::IndependentCurrentSource => Projection::Zero,
            _ => return Err(failure("nonlinear current in constant descriptor")),
        });
    }
    Ok((
        Plan {
            nodes,
            coordinates: circuit.matrix_size(),
            currents,
        },
        traces,
        added,
    ))
}

#[derive(Default)]
struct Actions {
    point: Option<Value>,
    derivatives: Vec<ImpulseDerivative>,
}

pub(in crate::engine::transient) struct Prepared<'a> {
    target: &'a mut TransientResult,
    time: Value,
    currents: Vec<Actions>,
    voltages: Vec<Actions>,
    added: usize,
}

fn difference(
    values: &[Value],
    positive: usize,
    negative: usize,
    gain: Value,
) -> Result<Value, SimulationError> {
    let value = |node: usize| if node == 0 { 0.0 } else { values[node - 1] };
    rspice_veriloga_runtime::arithmetic::sum_products(
        [(gain, value(positive)), (-gain, value(negative))].into_iter(),
    )
    .map_err(|error| failure(format!("unrepresentable descriptor observation: {error:?}")))
}

impl Plan {
    pub(super) fn prepare<'a>(
        &self,
        result: &'a mut TransientResult,
        event: &PreparedPhysicalEvent,
        retained: usize,
        limits: &crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Prepared<'a>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let time = event.time();
        if !time.is_finite()
            || time < 0.0
            || result.time.last().is_none_or(|&previous| time < previous)
        {
            return Err(failure("invalid descriptor observation time"));
        }
        let impulses = event
            .descriptor_impulses()
            .ok_or_else(|| failure("missing descriptor actions"))?;
        if impulses
            .iter()
            .any(|values| values.len() != self.coordinates || values.iter().any(|v| !v.is_finite()))
        {
            return Err(failure("unaligned descriptor actions"));
        }
        let charges = match event.device_impulses() {
            PhysicalDeviceImpulses::Jumps { capacitors, .. } => capacitors.as_slice(),
            PhysicalDeviceImpulses::Continuous => &[],
        };
        let overhead = self
            .currents
            .len()
            .saturating_add(self.nodes)
            .saturating_mul(8);
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::ResultValues,
            retained.saturating_add(overhead),
            limits.max_result_values,
        )?;
        let mut currents = allocated(self.currents.len())?;
        currents.resize_with(self.currents.len(), Actions::default);
        let mut voltages = allocated(self.nodes)?;
        voltages.resize_with(self.nodes, Actions::default);
        let mut added = 0usize;
        let mut append = |target: &mut Actions,
                          order: usize,
                          coefficient: Value|
         -> Result<(), SimulationError> {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if !coefficient.is_finite() {
                return Err(failure("nonfinite descriptor observation"));
            }
            if coefficient == 0.0 {
                return Ok(());
            }
            added = added.saturating_add(if order == 0 { 2 } else { 3 });
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::ResultValues,
                retained
                    .saturating_add(overhead)
                    .saturating_add(added.saturating_mul(2)),
                limits.max_result_values,
            )?;
            if order == 0 {
                target.point = Some(coefficient);
            } else {
                let order = u32::try_from(order).map_err(failure)?;
                target.derivatives.try_reserve(1).map_err(failure)?;
                target.derivatives.push(ImpulseDerivative {
                    time,
                    order,
                    coefficient,
                });
            }
            Ok(())
        };
        for (node, target) in voltages.iter_mut().enumerate() {
            for (order, values) in impulses.iter().enumerate() {
                append(target, order, values[node])?;
            }
        }
        for (projection, target) in self.currents.iter().zip(&mut currents) {
            // A capacitor differentiates a nodal action, retaining one more
            // order than the original MNA coordinate expansion.
            for order in 0..=impulses.len() {
                let coefficient = match *projection {
                    Projection::Capacitor {
                        index,
                        positive,
                        negative,
                        capacitance,
                    } => {
                        if order == 0 {
                            *charges
                                .get(index)
                                .ok_or_else(|| failure("missing capacitor charge action"))?
                        } else {
                            difference(&impulses[order - 1], positive, negative, capacitance)?
                        }
                    }
                    Projection::Coordinate(index) => {
                        impulses.get(order).map_or(0.0, |values| values[index])
                    }
                    Projection::Difference {
                        positive,
                        negative,
                        gain,
                    } => match impulses.get(order) {
                        Some(values) => difference(values, positive, negative, gain)?,
                        None => 0.0,
                    },
                    Projection::Zero => 0.0,
                };
                append(target, order, coefficient)?;
            }
        }
        let current_target = result
            .current_impulses
            .as_mut()
            .filter(|traces| traces.len() == currents.len())
            .ok_or_else(|| failure("unaligned descriptor current coverage"))?;
        let voltage_target = result
            .voltage_impulses
            .as_mut()
            .filter(|traces| traces.len() == voltages.len())
            .ok_or_else(|| failure("unaligned descriptor voltage coverage"))?;
        for (trace, actions) in current_target.iter_mut().zip(&currents) {
            reserve(
                trace.complete,
                &mut trace.points,
                &mut trace.derivatives,
                actions,
                time,
                |p| p.time,
                abort,
            )?;
        }
        for (trace, actions) in voltage_target.iter_mut().zip(&voltages) {
            reserve(
                trace.complete,
                &mut trace.points,
                &mut trace.derivatives,
                actions,
                time,
                |p| p.time,
                abort,
            )?;
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        Ok(Prepared {
            target: result,
            time,
            currents,
            voltages,
            added,
        })
    }
}

fn reserve<T>(
    complete: bool,
    points: &mut Vec<T>,
    derivatives: &mut Vec<ImpulseDerivative>,
    actions: &Actions,
    time: Value,
    point_time: impl Fn(&T) -> Value,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        return Err(SimulationError::Aborted);
    }
    if !complete {
        return Err(failure("descriptor owner has incomplete coverage"));
    }
    if actions.point.is_none() && actions.derivatives.is_empty() {
        return Ok(());
    }
    if points.last().is_some_and(|p| point_time(p) >= time)
        || derivatives.last().is_some_and(|p| p.time >= time)
    {
        return Err(failure("descriptor action would replay an accepted event"));
    }
    points
        .try_reserve(usize::from(actions.point.is_some()))
        .map_err(failure)?;
    derivatives
        .try_reserve(actions.derivatives.len())
        .map_err(failure)?;
    Ok(())
}

impl Prepared<'_> {
    pub(super) fn commit(self) -> usize {
        for (trace, actions) in self
            .target
            .current_impulses
            .as_mut()
            .unwrap()
            .iter_mut()
            .zip(self.currents)
        {
            if let Some(charge_coulombs) = actions.point {
                trace.points.push(CurrentImpulsePoint {
                    time: self.time,
                    charge_coulombs,
                });
            }
            trace.derivatives.extend(actions.derivatives);
        }
        for (trace, actions) in self
            .target
            .voltage_impulses
            .as_mut()
            .unwrap()
            .iter_mut()
            .zip(self.voltages)
        {
            if let Some(volt_seconds) = actions.point {
                trace.points.push(VoltageImpulsePoint {
                    time: self.time,
                    volt_seconds,
                });
            }
            trace.derivatives.extend(actions.derivatives);
        }
        self.added
    }
}
