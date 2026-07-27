//! Derivative lane packing for the width-parameterized lowering.
//!
//! The new lowering gives each value one `[f64; L]` derivative local instead of
//! one named scalar per lane, so emitted source scales with operations rather
//! than operations x lanes. This module decides what `L` is and where each lane
//! sits inside the array.
//!
//! `L` is a single width for the whole device rather than a per-value mask. The
//! probe in `benchmarks/reference/lowering-probe` measured why: the array form
//! scales sub-linearly in `L` — eight times the lanes cost 1.33x the time — so
//! narrowing individual values would buy little while forcing every call site
//! to carry its own width.
//!
//! ## Where the lowering reads its graph
//!
//! Not from the finished artifact. `OptModel` construction runs
//! `add_sparse_derivatives`, which expands the chain rule into the *same* flat
//! value graph and then lets compaction dissolve the links: for HiSIM-HV that
//! takes 10,065 primal values to 149,437 and settles at 75,300, of which only
//! 27 still record a `derivatives` list. The primal/derivative structure is not
//! recoverable afterwards, so the lowering consumes the graph *before* that
//! pass and differentiates it itself. Sizing the rewrite against the finished
//! artifact would measure the already-expanded graph and report no saving.

use std::collections::{BTreeSet, HashMap};

use crate::canonical_ir::{CanonicalIrArtifact, DerivativeLane, DerivativeLaneKind};

/// The derivative lanes a device uses, and their packing into `[f64; L]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LaneSet {
    lanes: Vec<DerivativeLane>,
    slot_of: HashMap<DerivativeLane, usize>,
}

impl LaneSet {
    /// Collect every lane any value in the model carries a derivative for.
    ///
    /// Reading the finished artifact is correct *for this purpose*: compaction
    /// drops the primal-to-lane links but keeps the equation roots, and a lane
    /// live anywhere in the device is live at a root by construction.
    pub(super) fn from_artifact(artifact: &CanonicalIrArtifact) -> Self {
        let mut seen: BTreeSet<(u8, u32)> = BTreeSet::new();
        for value in &artifact.opt.values {
            for derivative in &value.derivatives {
                seen.insert((lane_order(derivative.lane.kind), derivative.lane.index));
            }
        }

        let lanes: Vec<DerivativeLane> = seen
            .into_iter()
            .map(|(order, index)| DerivativeLane {
                kind: lane_kind(order),
                index,
            })
            .collect();
        let slot_of = lanes
            .iter()
            .enumerate()
            .map(|(slot, lane)| (*lane, slot))
            .collect();
        Self { lanes, slot_of }
    }

    /// `L` — the width of every emitted derivative array.
    pub(super) fn width(&self) -> usize {
        self.lanes.len()
    }

    /// Position of a lane within an emitted array, if the device uses it.
    pub(super) fn slot(&self, lane: DerivativeLane) -> Option<usize> {
        self.slot_of.get(&lane).copied()
    }

    pub(super) fn lanes(&self) -> &[DerivativeLane] {
        &self.lanes
    }
}

/// Packing order for a lane kind.
///
/// Nodes and branch unknowns come first and in that order, matching the stamp
/// ABI. The limiter correction is not a matrix topology lane at all — it is the
/// affine term stateful Newton limiting contributes to the equivalent source —
/// so it packs last, where widening the topology lanes cannot disturb it.
fn lane_order(kind: DerivativeLaneKind) -> u8 {
    match kind {
        DerivativeLaneKind::Node => 0,
        DerivativeLaneKind::BranchUnknown => 1,
        DerivativeLaneKind::LimiterCorrection => 2,
    }
}

fn lane_kind(order: u8) -> DerivativeLaneKind {
    match order {
        0 => DerivativeLaneKind::Node,
        1 => DerivativeLaneKind::BranchUnknown,
        _ => DerivativeLaneKind::LimiterCorrection,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_ir::{BranchUnknownId, NodeId};

    fn pack(mut lanes: Vec<DerivativeLane>) -> LaneSet {
        lanes.sort_by_key(|lane| (lane_order(lane.kind), lane.index));
        lanes.dedup();
        let slot_of = lanes
            .iter()
            .enumerate()
            .map(|(slot, lane)| (*lane, slot))
            .collect();
        LaneSet { lanes, slot_of }
    }

    #[test]
    fn nodes_pack_before_branches_before_the_limiter_correction() {
        let set = pack(vec![
            DerivativeLane::limiter_correction(),
            DerivativeLane::branch_unknown(BranchUnknownId::from(0)),
            DerivativeLane::node(NodeId::from(3)),
            DerivativeLane::node(NodeId::from(1)),
        ]);

        assert_eq!(set.width(), 4);
        assert_eq!(set.slot(DerivativeLane::node(NodeId::from(1))), Some(0));
        assert_eq!(set.slot(DerivativeLane::node(NodeId::from(3))), Some(1));
        assert_eq!(
            set.slot(DerivativeLane::branch_unknown(BranchUnknownId::from(0))),
            Some(2)
        );
        assert_eq!(set.slot(DerivativeLane::limiter_correction()), Some(3));
    }

    #[test]
    fn adding_a_node_lane_cannot_move_the_limiter_correction_off_the_end() {
        // The limiter lane is not matrix topology; keeping it last means a model
        // that grows a node does not silently reindex it into a topology slot.
        let narrow = pack(vec![
            DerivativeLane::node(NodeId::from(0)),
            DerivativeLane::limiter_correction(),
        ]);
        let wide = pack(vec![
            DerivativeLane::node(NodeId::from(0)),
            DerivativeLane::node(NodeId::from(9)),
            DerivativeLane::limiter_correction(),
        ]);

        assert_eq!(
            narrow.slot(DerivativeLane::limiter_correction()),
            Some(narrow.width() - 1)
        );
        assert_eq!(
            wide.slot(DerivativeLane::limiter_correction()),
            Some(wide.width() - 1)
        );
    }

    #[test]
    fn a_lane_the_device_never_uses_has_no_slot() {
        let set = pack(vec![DerivativeLane::node(NodeId::from(0))]);
        assert_eq!(set.slot(DerivativeLane::node(NodeId::from(7))), None);
        assert_eq!(set.slot(DerivativeLane::limiter_correction()), None);
    }
}
