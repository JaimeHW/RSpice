//! Naming the processes behind a tick that did not settle, and the loop they
//! close.
//!
//! The kernel knows which drivers fired at the instant and how often. It cannot
//! know what they are; this host can. Each is a process of the plan, with a
//! kind, the nets it drives and the nets it waits on, and a zero-delay loop is
//! a cycle of that static sensitivity graph among the processes that kept
//! firing (IEEE 1364-2005 section 11.4: a write wakes every process waiting on
//! the net it changed, in the same time slot).
//!
//! This runs only on failure and only rewrites the diagnostic's text, so
//! nothing here can change what a run computes. The kernel's interned
//! identities, which a checkpoint carries, are untouched.

use std::collections::{BTreeSet, VecDeque};

use rspice_veriloga::canonical_ir::cfg::{CfgTerminator, CfgValueKind, DigitalWait};
use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalProcessKind,
};
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;

use super::{DigitalHost, DigitalRunError, SchedulerError, signal_name};
use crate::xspice::event_scheduler::ZeroDelayLoop;

impl DigitalHost {
    /// Name this host's drivers in an oscillation, and the loop among them.
    ///
    /// A driver the host does not own — an external participant's — keeps the
    /// identity it was enrolled with.
    pub(super) fn explain_oscillation(&self, error: DigitalRunError) -> DigitalRunError {
        let DigitalRunError::Scheduler(SchedulerError::Oscillation(mut diagnostic)) = error else {
            return error;
        };
        let nba = self.scheduler.target(self.nba_target).clone();
        let mut members: Vec<&CfgDigitalProcess> = Vec::new();
        for (target, _) in &mut diagnostic.entities {
            if *target == nba {
                target.instance = "nonblocking update region".into();
                target.port_name.clear();
                continue;
            }
            let Some((index, process)) = self
                .targets
                .iter()
                .position(|id| self.scheduler.target(*id) == target)
                .and_then(|slot| self.slots.get(slot))
                .and_then(|slot| Some((slot.process, self.plan.processes.get(slot.process)?)))
            else {
                continue;
            };
            // A forked thread's target is keyed by its slot; the copy in the
            // diagnostic is keyed by the process, so whoever names the
            // process's owner next finds it.
            target.node_id = index;
            target.instance = process_label(process, usize::from(process.id));
            target.port_name = process_writes(process)
                .into_iter()
                .map(|signal| signal_name(&self.plan, signal))
                .collect::<Vec<_>>()
                .join(", ");
            if !members.iter().any(|member| member.id == process.id) {
                members.push(process);
            }
        }
        let nets = zero_delay_loop(&self.plan, &members);
        diagnostic.zero_delay_loop = (!nets.is_empty()).then(|| Box::new(ZeroDelayLoop { nets }));
        DigitalRunError::Scheduler(SchedulerError::Oscillation(diagnostic))
    }
}

/// A process as a diagnostic names it: its keyword and ordinal, then its kind
/// as an author calls it and the file and line it was written on,
/// `assign#1 (continuous assign, l.vams:13)`.
///
/// The ordinal is the caller's: the plan's own for a design run alone, and
/// the module's for an instance whose plan was linked into a larger one. The
/// file is named without its directory, as an author reads it in an editor's
/// tab; a process with no recorded origin is named without one.
pub(crate) fn process_label(process: &CfgDigitalProcess, ordinal: usize) -> String {
    let kind = process.kind;
    let name = match kind {
        DigitalProcessKind::Always => "always",
        DigitalProcessKind::Initial => "initial",
        DigitalProcessKind::ContinuousAssign => "continuous assign",
    };
    match &process.origin {
        Some(origin) => {
            let file = std::path::Path::new(origin.file.as_str())
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(origin.file.as_str());
            format!(
                "{}#{ordinal} ({name}, {file}:{})",
                kind.keyword(),
                origin.line
            )
        }
        None => format!("{}#{ordinal} ({name})", kind.keyword()),
    }
}

/// Every net or variable a process writes, from its lowered body.
pub(crate) fn process_writes(process: &CfgDigitalProcess) -> BTreeSet<DigitalSignalId> {
    process
        .function
        .values
        .iter()
        .filter_map(|value| match &value.kind {
            CfgValueKind::DigitalDriverWrite { target, .. }
            | CfgValueKind::DigitalBlockingWrite { target, .. }
            | CfgValueKind::DigitalNonblockingWrite { target, .. } => Some(target.signal),
            CfgValueKind::DigitalPackedBlockingWrite { signal, .. }
            | CfgValueKind::DigitalPackedNonblockingWrite { signal, .. } => Some(*signal),
            _ => None,
        })
        .collect()
}

/// Every net a process suspends on.
///
/// The static list when the process has one; otherwise every event control
/// written in its body, since each is a place it can be woken from.
pub(crate) fn process_reads(process: &CfgDigitalProcess) -> BTreeSet<DigitalSignalId> {
    if let Some(sensitivity) = &process.static_sensitivity {
        return sensitivity.terms.iter().map(|term| term.signal).collect();
    }
    fn event_terms(wait: &DigitalWait, into: &mut BTreeSet<DigitalSignalId>) {
        match wait {
            DigitalWait::Event(terms) => into.extend(terms.iter().map(|term| term.signal)),
            DigitalWait::Repeat { event, .. } => event_terms(event, into),
            _ => {}
        }
    }
    let mut signals = BTreeSet::new();
    for block in &process.function.blocks {
        if let CfgTerminator::Wait { wait, .. } = &block.terminator {
            event_terms(wait, &mut signals);
        }
    }
    signals
}

/// The shortest loop through the busiest member that closes one, as the nets
/// it runs through in order.
///
/// An edge runs from one process to another when the first writes a net the
/// second waits on. Members come busiest first, so the loop named is the one
/// the evidence points at; an empty answer means none of them closes one
/// statically, which is itself worth knowing (a loop through a routine or a
/// computed event expression).
fn zero_delay_loop(plan: &CanonicalDigitalPlan, members: &[&CfgDigitalProcess]) -> Vec<String> {
    let written: Vec<_> = members
        .iter()
        .map(|process| process_writes(process))
        .collect();
    let read: Vec<_> = members
        .iter()
        .map(|process| process_reads(process))
        .collect();
    // The lowest net through which `from` wakes `to`, if any.
    let link = |from: usize, to: usize| written[from].intersection(&read[to]).next().copied();
    for start in 0..members.len() {
        // Breadth first, so the loop found is the shortest through `start`.
        let mut previous: Vec<Option<(usize, DigitalSignalId)>> = vec![None; members.len()];
        let mut queue = VecDeque::from([start]);
        let mut closing = None;
        'search: while let Some(from) = queue.pop_front() {
            for (to, reached) in previous.iter_mut().enumerate() {
                let Some(net) = link(from, to) else {
                    continue;
                };
                if to == start {
                    closing = Some((from, net));
                    break 'search;
                }
                if reached.is_none() {
                    *reached = Some((from, net));
                    queue.push_back(to);
                }
            }
        }
        let Some((mut at, net)) = closing else {
            continue;
        };
        let mut nets = vec![net];
        while at != start {
            let (from, net) = previous[at].expect("every reached member records its link");
            nets.push(net);
            at = from;
        }
        nets.reverse();
        return nets
            .into_iter()
            .map(|signal| signal_name(plan, signal))
            .collect();
    }
    Vec::new()
}
