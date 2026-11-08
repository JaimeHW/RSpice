//! The elaborated topology of a netlist's first plain build, kept so the
//! run's structural fingerprint can name the circuit its analyses built
//! without elaborating the deck a second time.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

use super::{Engine, SimulationError};
use crate::{CircuitData, Netlist};

/// What a structural fingerprint reads from an elaborated circuit.
#[derive(Clone)]
pub(crate) struct BuiltTopology {
    /// Node names, sorted, without ground.
    pub(crate) node_names: Vec<String>,
    /// Branch names, sorted.
    pub(crate) branch_names: Vec<String>,
    pub(crate) hidden_states: usize,
    pub(crate) devices: usize,
    /// Every stored position of the assembled matrix.
    pub(crate) stamp_sparsity: Vec<(usize, usize)>,
}

impl BuiltTopology {
    pub(crate) fn of(engine: &Engine, circuit: &CircuitData) -> Result<Self, SimulationError> {
        let stamp_sparsity = if circuit.matrix_size() == 0 {
            Vec::new()
        } else {
            engine.build_matrix(circuit)?.stored_positions().collect()
        };
        Ok(Self {
            node_names: circuit.node_names_sorted(),
            branch_names: circuit.branch_names_sorted(),
            hidden_states: circuit.hidden_state_count(),
            devices: circuit.device_count(),
            stamp_sparsity,
        })
    }
}

/// The topology of the first plain build of one netlist, shared by the
/// engine a [`BuiltTopologyScope`] opened it on and every engine resolved
/// from that one for the same netlist.
pub(crate) struct BuiltTopologyRecord {
    /// Address of the netlist the scope borrows.
    netlist: usize,
    /// Cleared when the scope ends: a resolved engine that outlives it may
    /// still hold this record, and a later netlist can take the address.
    live: AtomicBool,
    topology: OnceLock<BuiltTopology>,
}

impl BuiltTopologyRecord {
    fn names(&self, netlist: &Netlist) -> bool {
        self.live.load(Ordering::Acquire) && self.netlist == std::ptr::from_ref(netlist).addr()
    }
}

/// Keeps the topology of the first plain build of one netlist on an engine
/// for as long as it lives. See [`Engine::retain_built_topology`].
struct BuiltTopologyScope<'a> {
    engine: &'a Engine,
    record: Arc<BuiltTopologyRecord>,
    /// The borrow that makes the recorded address name one netlist.
    _netlist: &'a Netlist,
}

impl Drop for BuiltTopologyScope<'_> {
    fn drop(&mut self) {
        self.record.live.store(false, Ordering::Release);
        if let Ok(mut slot) = self.engine.retained_topology.lock()
            && slot
                .as_ref()
                .is_some_and(|record| Arc::ptr_eq(record, &self.record))
        {
            slot.take();
        }
    }
}

impl Engine {
    /// Keep the topology of the first circuit an analysis on this engine
    /// builds from `netlist` as written, until the returned guard is dropped.
    ///
    /// A frontend that publishes a structural fingerprint for a deck it runs
    /// once holds the guard across the run, so the fingerprint names the
    /// circuit the analyses built instead of elaborating the deck again. A
    /// build with RF ports, a transient stop time or another netlist records
    /// nothing, and the fingerprint then elaborates the deck itself.
    #[must_use = "the topology is kept only while the returned guard lives"]
    pub fn retain_built_topology<'a>(&'a self, netlist: &'a Netlist) -> impl Sized + 'a {
        let record = Arc::new(BuiltTopologyRecord {
            netlist: std::ptr::from_ref(netlist).addr(),
            live: AtomicBool::new(true),
            topology: OnceLock::new(),
        });
        if let Ok(mut slot) = self.retained_topology.lock() {
            *slot = Some(Arc::clone(&record));
        }
        BuiltTopologyScope {
            engine: self,
            record,
            _netlist: netlist,
        }
    }

    /// The record an engine resolved from this one for `netlist` shares.
    pub(super) fn built_topology_record(
        &self,
        netlist: &Netlist,
    ) -> Option<Arc<BuiltTopologyRecord>> {
        let slot = self.retained_topology.lock().ok()?;
        slot.as_ref()
            .filter(|record| record.names(netlist))
            .map(Arc::clone)
    }

    /// Record `circuit`, a plain build of `netlist`, when this engine keeps
    /// that netlist's topology and has not recorded it yet. A circuit whose
    /// matrix cannot be assembled records nothing; the fingerprint's own
    /// build then reports why.
    pub(super) fn record_built_topology(&self, netlist: &Netlist, circuit: &CircuitData) {
        let Some(record) = self.built_topology_record(netlist) else {
            return;
        };
        if record.topology.get().is_none()
            && let Ok(topology) = BuiltTopology::of(self, circuit)
        {
            let _ = record.topology.set(topology);
        }
    }

    /// The recorded topology of `netlist`'s first plain build, if this
    /// engine kept it.
    pub(crate) fn built_topology(&self, netlist: &Netlist) -> Option<BuiltTopology> {
        self.built_topology_record(netlist)?.topology.get().cloned()
    }
}
