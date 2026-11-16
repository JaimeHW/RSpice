//! The event histories a transient written to a result sink hands to it as the run goes.
//!
//! A run kept in memory grows each digital and real event trace and each impulse trace in its
//! result. A run written to a sink names one event signal per trace before it starts integrating,
//! and every few thousand recorded changes moves what its traces gathered to the sink, keeping
//! only each trace's last changes: the recorders compare a new change with them (a digital node
//! that did not change records nothing, an impulse at an accepted time is not replayed), and an HDL
//! variable may still replace the last one at the same instant. The rest go when the run ends. The result
//! keeps each trace's name and coverage, so a reader that fills the points back from the sink
//! has the in-memory run's traces in the in-memory run's order. A trace that begins after the run
//! started under a name it did not declare keeps its changes in the result.
//!
//! A digital change is stored as its logic level with [`DigitalValue::store_strength`] as its
//! strength, which gives back the exact state and strength. An impulse trace is two real event
//! signals: its impulses, and its higher derivatives with each one's order as its sequence.

use rustc_hash::FxHashMap;

use super::result_rows::TransientRows;
use crate::analysis::transient::{EventOnlyNetKind, TransientResult};
use crate::result_sink::{EventChange, EventValue, ResultSignal, ResultSignalKind};
use crate::{CurrentImpulseOwner, SimulationError};

/// Values the traces may gather between two moves to the sink.
const GATHERED_VALUES: usize = 4096;

/// The event signals one run writes to its sink, and what it has gathered since it last wrote.
#[derive(Default)]
pub(super) struct SinkEvents {
    /// Signal id by trace name, for the digital and the real event traces the run may record.
    digital_ids: FxHashMap<String, u32>,
    real_ids: FxHashMap<String, u32>,
    /// The signal id of each of the result's traces, by trace index, found as each is first moved.
    digital: Vec<Option<u32>>,
    real: Vec<Option<u32>>,
    /// The impulse and derivative signals of each impulse trace, by trace index.
    current_impulses: Vec<(u32, u32)>,
    voltage_impulses: Vec<(u32, u32)>,
    /// The retained value count at the last move.
    moved_at: usize,
    changes: Vec<EventChange>,
}

fn signal(name: &str, unit: &str, kind: ResultSignalKind, quantity: &str) -> ResultSignal {
    let mut signal = ResultSignal::new(name, unit, kind);
    signal.quantity = quantity.to_owned();
    signal
}

impl SinkEvents {
    /// Name a signal for every event net the run keeps (`retained`, by node) and every HDL
    /// variable slot `.ESAVE` traces, each in its domain.
    pub(super) fn declare_nets(
        rows: &mut TransientRows<'_>,
        node_names: &[String],
        event_only_nodes: &[Option<EventOnlyNetKind>],
        retained: &[bool],
        hdl_slots: &[(&str, bool)],
    ) -> Result<Self, SimulationError> {
        let mut events = Self::default();
        for (index, name) in node_names.iter().enumerate() {
            if !retained.get(index).copied().unwrap_or(false) {
                continue;
            }
            match event_only_nodes.get(index).copied().flatten() {
                Some(EventOnlyNetKind::Digital) => events.declare_digital(rows, name)?,
                Some(EventOnlyNetKind::Real) => events.declare_real(rows, name)?,
                None => {}
            }
        }
        for &(name, real) in hdl_slots {
            if real {
                events.declare_real(rows, name)?;
            } else {
                events.declare_digital(rows, name)?;
            }
        }
        Ok(events)
    }

    fn declare_digital(
        &mut self,
        rows: &mut TransientRows<'_>,
        name: &str,
    ) -> Result<(), SimulationError> {
        if !self.digital_ids.contains_key(name) {
            let id = rows.declare(signal(name, "", ResultSignalKind::Logic, "digital"))?;
            self.digital_ids.insert(name.to_owned(), id);
        }
        Ok(())
    }

    fn declare_real(
        &mut self,
        rows: &mut TransientRows<'_>,
        name: &str,
    ) -> Result<(), SimulationError> {
        if !self.real_ids.contains_key(name) {
            let id = rows.declare(signal(name, "", ResultSignalKind::RealEvent, "real_event"))?;
            self.real_ids.insert(name.to_owned(), id);
        }
        Ok(())
    }

    /// Name a signal for every event trace the run's set-up already recorded that no net or
    /// variable named, so a trace's changes always have a signal to go to.
    pub(super) fn declare_recorded(
        &mut self,
        rows: &mut TransientRows<'_>,
        result: &TransientResult,
    ) -> Result<(), SimulationError> {
        for trace in &result.digital_traces {
            self.declare_digital(rows, &trace.node_name)?;
        }
        for trace in &result.real_traces {
            self.declare_real(rows, &trace.node_name)?;
        }
        Ok(())
    }

    /// Name the impulse and derivative signals of every impulse trace the run keeps.
    pub(super) fn declare_impulses(
        &mut self,
        rows: &mut TransientRows<'_>,
        result: &TransientResult,
    ) -> Result<(), SimulationError> {
        for trace in result.current_impulses.iter().flatten() {
            let (name, path) = match &trace.owner {
                CurrentImpulseOwner::Branch { branch_name } => (branch_name.as_str(), ""),
                CurrentImpulseOwner::DeviceLead {
                    device_name,
                    parameter,
                } => (parameter.as_str(), device_name.as_str()),
            };
            let mut points = signal(name, "C", ResultSignalKind::RealEvent, "current_impulse");
            points.path = path.to_owned();
            let mut derivatives = signal(
                name,
                "",
                ResultSignalKind::RealEvent,
                "current_impulse_derivative",
            );
            derivatives.path = path.to_owned();
            let ids = (rows.declare(points)?, rows.declare(derivatives)?);
            self.current_impulses.push(ids);
        }
        for trace in result.voltage_impulses.iter().flatten() {
            let points = signal(
                &trace.node_name,
                "V*s",
                ResultSignalKind::RealEvent,
                "voltage_impulse",
            );
            let derivatives = signal(
                &trace.node_name,
                "",
                ResultSignalKind::RealEvent,
                "voltage_impulse_derivative",
            );
            let ids = (rows.declare(points)?, rows.declare(derivatives)?);
            self.voltage_impulses.push(ids);
        }
        Ok(())
    }

    /// Move what the traces gathered to the sink once they hold a few thousand values, taking
    /// the values moved off `retained`.
    pub(super) fn gathered(
        &mut self,
        rows: &mut TransientRows<'_>,
        result: &mut TransientResult,
        retained: &mut usize,
    ) -> Result<(), SimulationError> {
        if retained.saturating_sub(self.moved_at) < GATHERED_VALUES {
            return Ok(());
        }
        let moved = self.move_to_sink(rows, result, false)?;
        *retained = retained.saturating_sub(moved);
        self.moved_at = *retained;
        Ok(())
    }

    /// Move every change the traces still hold to the sink: the run has ended.
    pub(super) fn finish(
        &mut self,
        rows: &mut TransientRows<'_>,
        result: &mut TransientResult,
    ) -> Result<(), SimulationError> {
        self.move_to_sink(rows, result, true).map(|_| ())
    }

    /// Move each trace's changes but its last (all of them when `all`) to the sink; returns how
    /// many result values they were.
    fn move_to_sink(
        &mut self,
        rows: &mut TransientRows<'_>,
        result: &mut TransientResult,
        all: bool,
    ) -> Result<usize, SimulationError> {
        // An event trace keeps its last two changes: an HDL variable that replaces the last one
        // at its instant compares the replacement with the one before.
        let (keep, keep_events) = if all { (0, 0) } else { (1, 2) };
        let mut moved = 0usize;
        let mut changes = std::mem::take(&mut self.changes);
        changes.clear();
        for (index, trace) in result.digital_traces.iter_mut().enumerate() {
            let Some(id) = Self::trace_id(
                &mut self.digital,
                &self.digital_ids,
                index,
                &trace.node_name,
            ) else {
                continue;
            };
            let count = trace.points.len().saturating_sub(keep_events);
            changes.extend(trace.points.drain(..count).map(|point| EventChange {
                signal: id,
                time: point.time,
                sequence: 0,
                value: EventValue::Logic(point.value.store_logic()),
                strength: point.value.store_strength(),
            }));
            moved = moved.saturating_add(count.saturating_mul(2));
        }
        for (index, trace) in result.real_traces.iter_mut().enumerate() {
            let Some(id) = Self::trace_id(&mut self.real, &self.real_ids, index, &trace.node_name)
            else {
                continue;
            };
            let count = trace.points.len().saturating_sub(keep_events);
            changes.extend(trace.points.drain(..count).map(|point| EventChange {
                signal: id,
                time: point.time,
                sequence: 0,
                value: EventValue::Real(point.value),
                strength: 0,
            }));
            moved = moved.saturating_add(count.saturating_mul(2));
        }
        for (trace, &(points, derivatives)) in result
            .current_impulses
            .iter_mut()
            .flatten()
            .zip(&self.current_impulses)
        {
            let count = trace.points.len().saturating_sub(keep);
            changes.extend(trace.points.drain(..count).map(|point| EventChange {
                signal: points,
                time: point.time,
                sequence: 0,
                value: EventValue::Real(point.charge_coulombs),
                strength: 0,
            }));
            moved = moved.saturating_add(count.saturating_mul(2));
            let count = trace.derivatives.len().saturating_sub(keep);
            changes.extend(trace.derivatives.drain(..count).map(|term| EventChange {
                signal: derivatives,
                time: term.time,
                sequence: term.order,
                value: EventValue::Real(term.coefficient),
                strength: 0,
            }));
            moved = moved.saturating_add(count.saturating_mul(3));
        }
        for (trace, &(points, derivatives)) in result
            .voltage_impulses
            .iter_mut()
            .flatten()
            .zip(&self.voltage_impulses)
        {
            let count = trace.points.len().saturating_sub(keep);
            changes.extend(trace.points.drain(..count).map(|point| EventChange {
                signal: points,
                time: point.time,
                sequence: 0,
                value: EventValue::Real(point.volt_seconds),
                strength: 0,
            }));
            moved = moved.saturating_add(count.saturating_mul(2));
            let count = trace.derivatives.len().saturating_sub(keep);
            changes.extend(trace.derivatives.drain(..count).map(|term| EventChange {
                signal: derivatives,
                time: term.time,
                sequence: term.order,
                value: EventValue::Real(term.coefficient),
                strength: 0,
            }));
            moved = moved.saturating_add(count.saturating_mul(3));
        }
        let written = rows.events(&changes);
        self.changes = changes;
        written.map(|()| moved)
    }

    /// The signal of trace `index`, found by its name the first time it is moved. A trace no
    /// signal was named for, one that began after the sink did under a name no net or variable
    /// declared, or a second trace of one name, has none and keeps its changes in the result.
    fn trace_id(
        ids: &mut Vec<Option<u32>>,
        named: &FxHashMap<String, u32>,
        index: usize,
        name: &str,
    ) -> Option<u32> {
        if let Some(&id) = ids.get(index) {
            return id;
        }
        let id = named
            .get(name)
            .copied()
            .filter(|id| !ids.contains(&Some(*id)));
        debug_assert_eq!(ids.len(), index, "traces are found in index order");
        ids.push(id);
        id
    }
}
