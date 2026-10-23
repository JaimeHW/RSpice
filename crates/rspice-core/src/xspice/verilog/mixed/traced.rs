//! The HDL signals a run traces by name: the nets and variables of an
//! instance's design that `.ESAVE` lists (`.esave xtb.pass xtb.mv`), which
//! the run publishes through the observer the value change dump uses.
//!
//! Also where each traced signal was declared and which of its instance's
//! processes drive it: what a waveform's "go to declaration" and "go to
//! drivers" open. Both are read from the instance's own compiled design, so
//! they are the same for every run of it and for a replay of its cached
//! artifact. The drivers are static: every process whose code, or a task or
//! function it calls, writes the signal. Which of them caused one particular
//! change is not recorded, so for a signal with one driver the list is exact
//! and for any other it is the set to look in.

use std::collections::BTreeSet;

use rspice_veriloga::canonical_ir::cfg::{CfgFunction, CfgTerminator, CfgValueKind, DigitalWait};
use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalProcessKind, DigitalSignal, DigitalSourceLine,
};
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;

use super::MixedSignalHost;

/// One HDL net or variable, as its instance declared it.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HdlSignalSource {
    /// The deck instance (`xtb`).
    pub instance: String,
    /// Its name in the instance's design (`ok`; `u1.q` inside a child).
    pub identifier: String,
    pub declaration: Option<DigitalSourceLine>,
    pub drivers: Vec<HdlSignalDriver>,
}

/// A process that writes a signal.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HdlSignalDriver {
    /// The scope the driven signal is declared in, which a process writing
    /// it is written in: the instance, then any child path (`xtb.u1`).
    pub scope: String,
    pub kind: DigitalProcessKind,
    pub origin: Option<DigitalSourceLine>,
}

/// Every signal's static writers in one design, by process index.
#[derive(Debug, Clone, Default)]
pub(crate) struct SignalWriters(Vec<BTreeSet<usize>>);

impl SignalWriters {
    pub(crate) fn of(plan: &CanonicalDigitalPlan) -> Self {
        // What each task and function writes, itself or through what it
        // calls in turn, to a fixed point over the call graph.
        let routines: Vec<_> = plan
            .routines
            .iter()
            .map(|routine| function_writes(&routine.function))
            .collect();
        let mut closed: Vec<BTreeSet<DigitalSignalId>> =
            routines.iter().map(|(writes, _)| writes.clone()).collect();
        loop {
            let mut grew = false;
            for (index, (_, calls)) in routines.iter().enumerate() {
                for callee in calls {
                    let Some(written) = closed.get(*callee).cloned() else {
                        continue;
                    };
                    let before = closed[index].len();
                    closed[index].extend(written);
                    grew |= closed[index].len() != before;
                }
            }
            if !grew {
                break;
            }
        }
        let mut writers = vec![BTreeSet::new(); plan.signals.len()];
        for (process, body) in plan.processes.iter().enumerate() {
            let (mut writes, calls) = function_writes(&body.function);
            for callee in calls {
                if let Some(written) = closed.get(callee) {
                    writes.extend(written.iter().copied());
                }
            }
            for signal in writes {
                if let Some(slot) = writers.get_mut(usize::from(signal)) {
                    slot.insert(process);
                }
            }
        }
        Self(writers)
    }
}

/// The signals a function writes, and the routines it calls.
fn function_writes(function: &CfgFunction) -> (BTreeSet<DigitalSignalId>, BTreeSet<usize>) {
    let mut writes = BTreeSet::new();
    let mut calls = BTreeSet::new();
    for value in &function.values {
        match &value.kind {
            CfgValueKind::DigitalDriverWrite { target, .. }
            | CfgValueKind::DigitalBlockingWrite { target, .. }
            | CfgValueKind::DigitalNonblockingWrite { target, .. } => {
                writes.insert(target.signal);
            }
            CfgValueKind::DigitalPackedBlockingWrite { signal, .. }
            | CfgValueKind::DigitalPackedNonblockingWrite { signal, .. } => {
                writes.insert(*signal);
            }
            CfgValueKind::DigitalCall { routine, .. } => {
                calls.insert(routine.index());
            }
            _ => {}
        }
    }
    for block in &function.blocks {
        if let CfgTerminator::Wait {
            wait: DigitalWait::Call { routine, .. },
            ..
        } = &block.terminator
        {
            calls.insert(routine.index());
        }
    }
    (writes, calls)
}

/// A net or variable of an instance's design that has a value to trace.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HdlVariable {
    pub signal: DigitalSignalId,
    /// Its name as declared.
    pub identifier: String,
    pub real: bool,
    /// `[msb:lsb]` as declared, for a vector; a scalar and a real have none.
    pub range: Option<(i64, i64)>,
}

impl HdlVariable {
    /// How many trace slots it publishes: one per bit, or one real.
    pub(crate) fn slots(&self) -> usize {
        match self.range {
            Some((msb, lsb)) => {
                usize::try_from(msb.abs_diff(lsb)).map_or(usize::MAX, |span| span.saturating_add(1))
            }
            None => 1,
        }
    }
}

/// Whether a signal holds a value: a named event has none.
fn valued(signal: &DigitalSignal) -> bool {
    signal.kind.is_real() || signal.width > 0
}

impl MixedSignalHost {
    /// The net or variable `identifier` names in this instance's design: the
    /// one declared with exactly that name, or else the only one whose name
    /// matches it ignoring case, as a deck that folds case spells it.
    pub(crate) fn hdl_variable(&self, identifier: &str) -> Option<HdlVariable> {
        let plan = self.digital_plan();
        let candidates = || plan.signals.iter().filter(|signal| valued(signal));
        let signal = match candidates().find(|signal| signal.name == identifier) {
            Some(signal) => signal,
            None => {
                let mut folded =
                    candidates().filter(|signal| signal.name.eq_ignore_ascii_case(identifier));
                let only = folded.next()?;
                if folded.next().is_some() {
                    return None;
                }
                only
            }
        };
        let real = signal.kind.is_real();
        let range = if real {
            None
        } else {
            signal
                .bounds
                .or_else(|| (signal.width > 1).then(|| (i64::from(signal.width) - 1, 0)))
        };
        Some(HdlVariable {
            signal: signal.id,
            identifier: signal.name.to_string(),
            real,
            range,
        })
    }

    /// Every circuit node one of this instance's ports may be traced on,
    /// with the port's signal: a bridge's node, an event port's net, and the
    /// deck net a port moved onto a converter's endpoint is recorded under.
    /// A node may come more than once.
    pub(crate) fn traced_port_signals(&self, mut sink: impl FnMut(usize, DigitalSignalId)) {
        self.boundary_trace_bindings(|node, signal, _, _| sink(node, signal));
        for port in &self.event_ports {
            sink(port.node, port.signal);
            if let Some(deck) = port.trace_node.filter(|deck| *deck > 0) {
                sink(deck, port.signal);
            }
        }
    }

    /// Where `signal` was declared, and the processes `writers` says write it.
    pub(crate) fn signal_source(
        &self,
        signal: DigitalSignalId,
        writers: &SignalWriters,
    ) -> Option<HdlSignalSource> {
        let plan = self.digital_plan();
        let declared = plan.signals.get(usize::from(signal))?;
        let scope = match declared.name.rsplit_once('.') {
            Some((child, _)) => format!("{}.{child}", self.instance),
            None => self.instance.clone(),
        };
        let drivers = writers
            .0
            .get(usize::from(signal))
            .into_iter()
            .flatten()
            .filter_map(|process| plan.processes.get(*process))
            .map(|process| HdlSignalDriver {
                scope: scope.clone(),
                kind: process.kind,
                origin: process.origin.clone(),
            })
            .collect();
        Some(HdlSignalSource {
            instance: self.instance.clone(),
            identifier: declared.name.to_string(),
            declaration: declared.origin.clone(),
            drivers,
        })
    }
}
