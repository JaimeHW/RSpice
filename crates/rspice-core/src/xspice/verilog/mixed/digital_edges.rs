//! Analog event controls on digital edges: `@(posedge d)` and `@(negedge d)`
//! on a discrete signal inside an `analog` block.
//!
//! Verilog-AMS LRM 2.4 section 7.3.4 lists `posedge expression` beside the
//! analog event functions, and section 7.3.6.2 runs the guarded statement "at
//! the time corresponding to a real promotion of the digital time". The engine
//! already lands an accepted timepoint on every digital tick it schedules, and
//! an edge an interior A/D crossing causes is refined to that crossing, so the
//! instant the clause names is the coupled trial whose digital settle produced
//! the edge. The compiled body reads one integer flag per watched edge (a
//! discrete input carrying [`DiscreteAnalogInput::edge`]); this module raises
//! it for exactly that trial.
//!
//! The edge itself is classified where the value changed — one entry of the
//! store's transition list, by the canonical IR's IEEE 1364-2005 section 9.7.2
//! rule on the signal's least significant bit — so a pulse that rises and
//! falls inside one settle is still a rising edge, as it is for a process.
//!
//! Every piece of this state is trial-scoped and none of it is checkpointed:
//!
//! * The [`AnalogEdgeLatch`] lives in the digital store, so a rejected trial's
//!   rollback image takes its edges back with everything else. The shared
//!   coordinator clears it when it opens a trial, before taking that image.
//! * An enrolled instance reads the latch through its signal view, which the
//!   coordinator's `synchronize` refreshes with the values.
//! * Acceptance lowers the flags in the analog device, so its accepted record
//!   never carries a raised flag into the next timepoint, a re-evaluation of
//!   the accepted point, or a checkpoint.
//!
//! Small-signal analyses run no trial and see no events.

use super::super::store::TransitionValues;
use super::*;
use rspice_veriloga::canonical_ir::digital::{DigitalEdge, DigitalSensitivityTerm};
use rspice_veriloga::canonical_ir::digital_eval::any_term_is_satisfied;
use rspice_veriloga::canonical_ir::digital_link::DigitalLinkedInstance;

/// The latch bit for one edge.
const fn edge_bit(edge: DigitalEdge) -> u8 {
    match edge {
        DigitalEdge::Posedge => 1,
        DigitalEdge::Negedge => 2,
    }
}

/// The digital edges analog event controls wait on, and which of them the
/// store has seen since it was last cleared.
#[derive(Clone, Default)]
pub(crate) struct AnalogEdgeLatch {
    /// Per signal, the edge bits some analog event control waits on. Empty
    /// when none does, which is every design without one.
    watched: Arc<[u8]>,
    /// Per signal, the watched edges seen. Empty until one is.
    seen: Vec<u8>,
}

impl AnalogEdgeLatch {
    /// Install the watch list, indexed by this store's signal ids.
    pub(crate) fn watch(&mut self, watched: Vec<u8>) {
        self.watched = if watched.iter().any(|edges| *edges != 0) {
            watched.into()
        } else {
            Arc::from([])
        };
        self.seen.clear();
    }

    /// Keep another store's watch list, with nothing seen.
    pub(crate) fn inherit(&mut self, source: &Self) {
        self.watched = Arc::clone(&source.watched);
        self.seen.clear();
    }

    /// Classify one value change of the store.
    pub(crate) fn observe(&mut self, signal: DigitalSignalId, values: &TransitionValues) {
        let index = usize::from(signal);
        let Some(&watched) = self.watched.get(index) else {
            return;
        };
        let TransitionValues::FourState { previous, next } = values else {
            return;
        };
        let mut edges = 0;
        for edge in [DigitalEdge::Posedge, DigitalEdge::Negedge] {
            let term = DigitalSensitivityTerm {
                signal,
                edge: Some(edge),
            };
            if watched & edge_bit(edge) != 0
                && any_term_is_satisfied(&[term], signal, previous, next)
            {
                edges |= edge_bit(edge);
            }
        }
        if edges != 0 {
            self.seen.resize(self.watched.len(), 0);
            self.seen[index] |= edges;
        }
    }

    /// The watched edges seen on one signal.
    pub(crate) fn seen(&self, signal: DigitalSignalId) -> u8 {
        self.seen.get(usize::from(signal)).copied().unwrap_or(0)
    }

    /// Whether any watched edge has been seen.
    pub(crate) fn is_raised(&self) -> bool {
        !self.seen.is_empty()
    }

    pub(crate) fn clear(&mut self) {
        self.seen.clear();
    }
}

/// The watch list a plan's signal space needs for these instances' edge
/// inputs, each mapped through its instance's signal map when there is one.
fn watch_list<'a>(
    signals: usize,
    inputs: impl Iterator<Item = (&'a [DiscreteAnalogInput], Option<&'a [DigitalSignalId]>)>,
) -> Vec<u8> {
    let mut watched = vec![0; signals];
    for (inputs, map) in inputs {
        for input in inputs {
            let Some(edge) = input.edge else {
                continue;
            };
            let signal = map.map_or(input.signal, |map| map[usize::from(input.signal)]);
            if let Some(slot) = watched.get_mut(usize::from(signal)) {
                *slot |= edge_bit(edge);
            }
        }
    }
    watched
}

/// Watch every enrolled instance's digital edges on the circuit's shared store.
pub(super) fn watch_enrolled(
    digital: &mut DigitalHost,
    hosts: &[MixedSignalHost],
    maps: &[DigitalLinkedInstance],
) {
    if !hosts
        .iter()
        .flat_map(|host| &host.discrete_inputs)
        .any(|input| input.edge.is_some())
    {
        return;
    }
    let watched = watch_list(
        digital.plan().signals.len(),
        hosts.iter().zip(maps).map(|(host, map)| {
            (
                host.discrete_inputs.as_slice(),
                Some(map.signals.as_slice()),
            )
        }),
    );
    digital.analog_edges_mut().watch(watched);
}

/// Clear the shared store's latch before a trial images it, so the image a
/// rejection restores holds no edge either.
pub(super) fn open_trial(digital: &mut MixedCell<DigitalHost>) {
    if digital.analog_edges().is_raised() {
        digital.make_mut().analog_edges_mut().clear();
    }
}

/// Whether a view's edge bits differ from what the shared latch holds for its
/// instance.
pub(super) fn view_differs(view: &[u8], map: &[DigitalSignalId], latch: &AnalogEdgeLatch) -> bool {
    if !latch.is_raised() {
        return !view.is_empty();
    }
    map.iter()
        .enumerate()
        .any(|(local, global)| view.get(local).copied().unwrap_or(0) != latch.seen(*global))
}

/// Refresh a view's edge bits from the shared latch.
pub(super) fn refresh_view(view: &mut Vec<u8>, map: &[DigitalSignalId], latch: &AnalogEdgeLatch) {
    view.clear();
    if latch.is_raised() {
        view.extend(map.iter().map(|global| latch.seen(*global)));
    }
}

/// The flag an edge input reads: one when the trial saw that edge.
pub(super) fn flag(seen: u8, edge: DigitalEdge) -> f64 {
    f64::from(u8::from(seen & edge_bit(edge) != 0))
}

impl MixedSignalHost {
    /// Watch this instance's own digital edges on the store it owns. An
    /// enrolled instance is watched on the circuit's store instead.
    pub(super) fn watch_own_digital_edges(&mut self) {
        if !self
            .discrete_inputs
            .iter()
            .any(|input| input.edge.is_some())
        {
            return;
        }
        let MixedDigital::Owned(_) = &*self.state.digital else {
            return;
        };
        let watched = watch_list(
            self.state.digital.plan().signals.len(),
            std::iter::once((self.discrete_inputs.as_slice(), None)),
        );
        if let MixedDigital::Owned(host) = self.state.digital.make_mut() {
            host.analog_edges_mut().watch(watched);
        }
    }

    /// Lower every edge flag once a trial is accepted: in the analog device's
    /// accepted record, and in the discrete half this instance reads.
    ///
    /// Called after the trial's rollback image is dropped, so clearing the
    /// discrete half's bits copies nothing.
    pub(super) fn lower_digital_edges(&mut self) {
        let mut watched = false;
        for input in self
            .discrete_inputs
            .iter()
            .filter(|input| input.edge.is_some())
        {
            watched = true;
            if self
                .analog
                .discrete_state_value(input.variable)
                .is_some_and(|value| value != 0.0)
            {
                // A slot the device validated when the model was built.
                let _ = self
                    .analog
                    .make_mut()
                    .sample_discrete_state(input.variable, 0.0);
            }
        }
        if watched && self.state.digital.has_analog_edges() {
            self.state.digital.make_mut().clear_analog_edges();
        }
    }
}
