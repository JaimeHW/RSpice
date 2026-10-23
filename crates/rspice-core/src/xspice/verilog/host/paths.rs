//! Module path delays (IEEE 1364-2005 section 14): which delay a path
//! destination's change waits for.
//!
//! The store holds a destination's new value back as a net's
//! (`store/delays.rs`), and the pending slot applies the inertial rule to it
//! as to any delayed net. Its delay is chosen here, as Icarus Verilog
//! chooses it except where IEEE 1364-2005 says otherwise:
//!
//! - each path keeps the tick its source last changed (for an
//!   edge-sensitive path, last made its edge);
//! - of the paths that count, those whose source changed last decide, and
//!   among them the least delay; a conditional path counts unless its
//!   condition net is 0, so a condition that is `x` or `z` counts as true
//!   (section 14.2.4.1; Icarus Verilog reads it as false), and an `ifnone`
//!   path only when no other path counts or its own source changed later
//!   than theirs;
//! - the change is due that delay after that source's change, or at once
//!   when that is already past (section 14.4: the larger of the path delay
//!   and the distributed delays already behind the value);
//! - the transition is the least significant bit's (table 14-3), and no
//!   path at all delays nothing.

use super::*;
use rspice_veriloga::canonical_ir::digital::DigitalEdge;
use rspice_veriloga::four_state::FourStateBit;

/// One module path, its delays in the plan's ticks.
#[derive(Debug)]
struct Path {
    edge: Option<DigitalEdge>,
    condition: Option<DigitalSignalId>,
    ifnone: bool,
    ticks: [u64; 12],
}

/// The paths by source and by destination.
#[derive(Debug)]
struct PathTables {
    paths: Vec<Path>,
    by_source: BTreeMap<DigitalSignalId, Vec<usize>>,
    by_destination: BTreeMap<DigitalSignalId, Vec<usize>>,
}

/// The module paths and the tick each one's source last changed.
#[derive(Debug, Clone)]
pub(super) struct PathDelays {
    tables: Arc<PathTables>,
    wakes: Vec<u64>,
}

impl PathDelays {
    /// The plan's paths; `None` for a plan with none.
    pub(super) fn for_plan(plan: &CanonicalDigitalPlan) -> Option<Self> {
        if plan.delays.paths.is_empty() {
            return None;
        }
        let precision = plan.timing.precision_exponent;
        let mut tables = PathTables {
            paths: Vec::with_capacity(plan.delays.paths.len()),
            by_source: BTreeMap::new(),
            by_destination: BTreeMap::new(),
        };
        for (index, path) in plan.delays.paths.iter().enumerate() {
            tables.paths.push(Path {
                edge: path.edge,
                condition: path.condition,
                ifnone: path.ifnone,
                // A delay the design's precision cannot count is past any
                // run's end.
                ticks: path.delay.ticks_at(precision).unwrap_or([u64::MAX; 12]),
            });
            tables.by_source.entry(path.source).or_default().push(index);
            tables
                .by_destination
                .entry(path.destination)
                .or_default()
                .push(index);
        }
        Some(Self {
            wakes: vec![0; tables.paths.len()],
            tables: Arc::new(tables),
        })
    }

    /// Note a change of `signal` at `tick` on every path it is the source
    /// of: any change, or for an edge-sensitive path its edge of the least
    /// significant bit (section 14.2.4.2).
    pub(super) fn note_change(&mut self, transition: &SignalTransition, tick: u64) {
        let Some(paths) = self.tables.by_source.get(&transition.signal) else {
            return;
        };
        let TransitionValues::FourState { previous, next } = &transition.values else {
            return;
        };
        let (from, to) = (previous.bit(0), next.bit(0));
        for &index in paths {
            let counts = match self.tables.paths[index].edge {
                None => true,
                Some(DigitalEdge::Posedge) => {
                    from != to && (from == FourStateBit::Zero || to == FourStateBit::One)
                }
                Some(DigitalEdge::Negedge) => {
                    from != to && (from == FourStateBit::One || to == FourStateBit::Zero)
                }
            };
            if counts {
                self.wakes[index] = tick;
            }
        }
    }

    /// The delay, from `now`, of `destination`'s change from `from` to `to`.
    pub(super) fn delay(
        &self,
        store: &DigitalSignalStore,
        destination: DigitalSignalId,
        (from, to): (&FourStateValue, &FourStateValue),
        now: u64,
    ) -> u64 {
        let Some(paths) = self.tables.by_destination.get(&destination) else {
            return 0;
        };
        // Section 14.2.4.1: a condition that is `x` or `z` is true; its net
        // already holds just the least significant bit of a vector's.
        let counts = |index: usize| {
            self.tables.paths[index].condition.is_none_or(|condition| {
                store
                    .value(condition)
                    .is_none_or(|value| value.bit(0) != FourStateBit::Zero)
            })
        };
        // The paths whose source changed last, then any `ifnone` path whose
        // source changed later still, or the latest `ifnone` paths when no
        // other path counts (two of those tie only past tick 0, as in Icarus
        // Verilog).
        let mut chosen: Vec<usize> = Vec::new();
        let mut latest = 0;
        for &index in paths {
            let path = &self.tables.paths[index];
            if path.ifnone || !counts(index) {
                continue;
            }
            let wake = self.wakes[index];
            if chosen.is_empty() || wake > latest {
                chosen.clear();
                latest = wake;
            }
            if wake == latest {
                chosen.push(index);
            }
        }
        let normal = (!chosen.is_empty()).then_some(latest);
        for &index in paths {
            if !self.tables.paths[index].ifnone {
                continue;
            }
            let wake = self.wakes[index];
            if chosen.is_empty() || wake > latest {
                chosen.clear();
                latest = wake;
                chosen.push(index);
            } else if wake == latest && latest > normal.unwrap_or(0) {
                chosen.push(index);
            }
        }
        let transition = transition_index(from.bit(0), to.bit(0));
        chosen
            .iter()
            .map(|index| self.tables.paths[*index].ticks[transition])
            .min()
            .map_or(0, |delay| latest.saturating_add(delay).saturating_sub(now))
    }

    /// Each path's last counted change, in plan order, as the host image's
    /// `specify` section stores it.
    pub(super) fn image(&self) -> &[u64] {
        &self.wakes
    }

    /// Install the last counted changes a host image's `specify` section
    /// names, one per path.
    pub(super) fn restore(&mut self, wakes: &[u64]) -> Result<(), String> {
        if wakes.len() != self.wakes.len() {
            return Err("host specify section names other than one change per path".into());
        }
        self.wakes.copy_from_slice(wakes);
        Ok(())
    }
}

/// A transition's place in a path's twelve delays (table 14-3), as Icarus
/// Verilog reads a bit that did not change: `0` to `0` as a rise, `1` to
/// `1` as a fall, `x` to `x` as `x` to `0` and `z` to `z` as `z` to `0`.
fn transition_index(from: FourStateBit, to: FourStateBit) -> usize {
    use FourStateBit::{HighImpedance as Z, One, Unknown as X, Zero};
    match (from, to) {
        (Zero, Zero | One) => 0,
        (One, Zero | One) => 1,
        (Zero, Z) => 2,
        (Z, One) => 3,
        (One, Z) => 4,
        (Z, Zero | Z) => 5,
        (Zero, X) => 6,
        (X, One) => 7,
        (One, X) => 8,
        (X, Zero | X) => 9,
        (X, Z) => 10,
        (Z, X) => 11,
    }
}
