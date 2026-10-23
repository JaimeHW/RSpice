//! The last digital edge a mixed module's own D/A took, for a terminal
//! transient failure to name.
//!
//! A module drives the analog circuit without a converter in two ways: a D/A
//! bridge, and a discrete value its analog block reads at its left limit
//! ([`DiscreteAnalogInput::reads_left_limit`]). Either steps the analog
//! problem from the first instant after an accepted point where the discrete
//! half moved it, so the step leaving that point is where an ideal step fails
//! Newton. The circuit names an inserted converter's edge from the event
//! state (`circuit/boundary_edges.rs`); a module's own drive has no event net,
//! so each acceptance keeps the last edge one of them took here.
//!
//! The record is accepted state. Only [`MixedSignalHost::accept_trial`]'s
//! commit writes it, so a rejected trial leaves it as it was, and a checkpoint
//! carries it as an optional section of its own.

use super::*;

/// Which of a module's own drives an edge moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Drive {
    /// `bridges.dac[index]`.
    Bridge(usize),
    /// `discrete_inputs[index]`.
    Read(usize),
}

/// One accepted step of a module's own drive: at `time`, from `before` to
/// `after`, spelled as the drive carries them.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct DriveEdge {
    pub(super) drive: Drive,
    pub(super) time: f64,
    pub(super) before: String,
    pub(super) after: String,
}

/// What a terminal failure after a module's own drive edge names: the net
/// with its values before and after, the edge time, the processes writing it,
/// and the drive it crossed into the analog circuit through.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DriveEdgeReport {
    pub(crate) net: String,
    pub(crate) before: String,
    pub(crate) after: String,
    pub(crate) time: f64,
    pub(crate) drivers: Vec<String>,
    pub(crate) into: String,
    /// Whether the drive steps between its values at once: a D/A bridge
    /// does. A read may pass through `transition()`, so it is not said to.
    pub(crate) ideal: bool,
}

impl MixedSignalHost {
    /// The first of this module's own drives `trial` moved, as an edge at the
    /// trial's timepoint, comparing the settled store with the image the
    /// trial opened with. An operating point's trial has no edge: its value is
    /// the initial state.
    pub(super) fn drive_edge(&self, trial: &ActiveTrial) -> Option<DriveEdge> {
        if trial.timestep_seconds <= 0.0 {
            return None;
        }
        let edge = |drive, before: String, after: String| DriveEdge {
            drive,
            time: trial.time_seconds,
            before,
            after,
        };
        for (index, bridge) in self.state.bridges.dac.iter().enumerate() {
            let bit = |digital: &MixedDigital| {
                digital
                    .read(bridge.signal)
                    .map_or(FourStateBit::HighImpedance, |value| value.bit(bridge.bit))
            };
            let (before, after) = (bit(&trial.rollback), bit(&self.state.digital));
            if before != after {
                return Some(edge(
                    Drive::Bridge(index),
                    BoundaryNetHistory::spelling(BoundaryNetHistory::code(before)).into(),
                    BoundaryNetHistory::spelling(BoundaryNetHistory::code(after)).into(),
                ));
            }
        }
        for (index, input) in self.discrete_inputs.iter().enumerate() {
            if !input.reads_left_limit() || !input.feeds_equations {
                continue;
            }
            let before = read_discrete_value(&trial.rollback, input);
            let after = read_discrete_value(&self.state.digital, input);
            if before.map(f64::to_bits) != after.map(f64::to_bits) {
                return Some(edge(Drive::Read(index), number(before), number(after)));
            }
        }
        None
    }

    /// The last accepted edge of this module's own drives, when a terminal
    /// failure at `time` lies after it by no more than two steps of `step`:
    /// the window an inserted converter with no delay and no transition time
    /// is given.
    pub(crate) fn drive_edge_before_failure(
        &self,
        time: f64,
        step: f64,
    ) -> Option<DriveEdgeReport> {
        let edge = self.state.accepted_drive_edge.as_ref()?;
        if edge.time <= 0.0 || edge.time >= time || time - edge.time > 2.0 * step {
            return None;
        }
        let (net, signal, into, ideal) = match edge.drive {
            Drive::Bridge(index) => {
                let bridge = self.state.bridges.dac.get(index)?;
                let net = self
                    .deck_net_name(bridge.positive, &bridge.signal_name)
                    .unwrap_or_else(|| format!("{}.{}", self.instance, bridge.signal_name));
                let into = format!(
                    "the D/A bridge of {} on `{}` (low={}, high={}, r={})",
                    self.instance,
                    bridge.signal_name,
                    number(Some(bridge.low)),
                    number(Some(bridge.high)),
                    number(Some(bridge.resistance)),
                );
                (net, bridge.signal, into, true)
            }
            Drive::Read(index) => {
                let input = self.discrete_inputs.get(index)?;
                let into = format!("the analog block of {}, which reads it", self.instance);
                (
                    format!("{}.{}", self.instance, input.name),
                    input.signal,
                    into,
                    false,
                )
            }
        };
        let mut drivers: Vec<String> = self
            .state
            .digital
            .plan()
            .processes
            .iter()
            .filter(|process| super::super::host::process_writes(process).contains(&signal))
            .map(|process| self.process_name(process))
            .collect();
        if drivers.is_empty() {
            drivers.push(self.instance.clone());
        }
        Some(DriveEdgeReport {
            net,
            before: edge.before.clone(),
            after: edge.after.clone(),
            time: edge.time,
            drivers,
            into,
            ideal,
        })
    }
}

/// A value the analog equations read, as an author would write it, and `x`
/// for one that is not a number.
fn number(value: Option<f64>) -> String {
    match value {
        None => "x".into(),
        Some(value) if value == 0.0 || (1.0e-3..1.0e6).contains(&value.abs()) => {
            format!("{value}")
        }
        Some(value) => format!("{value:e}"),
    }
}
