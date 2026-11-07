//! Loops the DC operating-point system closes through fixed branch voltages.
//!
//! An independent voltage source states `V(+) - V(-)` outright, and the DC
//! solve stamps an inductor as a zero-volt branch (`stamp_dc_short`). A cycle
//! of such branches makes its branch rows sum to a constant and leaves the
//! current circulating around it undetermined, so the DC matrix is singular
//! whatever the source values. Nodal GMIN and `.OPTIONS RSHUNT` stamp node
//! rows, never branch rows, so no conditioning reaches it: every DC solve of
//! the circuit is refused.
//!
//! QSPICE solves such a loop when an inductor is in it, because its DC solve
//! gives every inductor 1 pOhm. Read the native way, RSpice gives that
//! resistance to the windings that close a loop and to no other, and raises a
//! notice naming them ([`CircuitData::resolve_dc_inductor_loops`]). A loop of voltage
//! sources alone stays singular, as it does in QSPICE, and so does every loop
//! read the ngspice or Xyce way, as those simulators leave it.

use std::collections::VecDeque;

use super::*;
use crate::circuit::storage::DC_LOOP_RESISTANCE;

/// One loop of fixed-voltage DC branches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DcBranchVoltageLoop {
    /// Element names around the loop, in build order, the element that closed
    /// it last.
    pub members: Vec<String>,
    /// Whether an inductor is a member. A loop of voltage sources alone is
    /// also refused outside the DC solve, because the independent-source
    /// constraint projection is singular; a loop through an inductor is
    /// refused only where a DC operating point is solved, since a transient
    /// step stamps the winding as an impedance.
    pub through_inductor: bool,
}

/// One fixed-voltage DC branch, by its index in its own storage.
#[derive(Debug, Clone, Copy)]
enum DcBranch {
    Source(usize),
    Inductor(usize),
}

impl CircuitData {
    /// Every loop the DC operating-point system closes through independent
    /// voltage sources and DC-shorted inductors.
    ///
    /// An inductor with `IC=` under Xyce's all-operating-points mode is a
    /// current constraint in the DC solve, not a short, and is not an edge;
    /// nor is a winding the DC solve gives [`DC_LOOP_RESISTANCE`].
    pub fn dc_branch_voltage_loops(&self) -> Vec<DcBranchVoltageLoop> {
        self.dc_branch_loops()
            .into_iter()
            .map(|branches| DcBranchVoltageLoop {
                through_inductor: branches
                    .iter()
                    .any(|branch| matches!(branch, DcBranch::Inductor(_))),
                members: branches
                    .into_iter()
                    .map(|branch| self.dc_branch_name(branch).to_owned())
                    .collect(),
            })
            .collect()
    }

    /// Give every winding that closes a loop of fixed DC branch voltages
    /// [`DC_LOOP_RESISTANCE`] in the DC solve, as QSPICE gives every
    /// inductor, and raise a notice naming the windings of each loop. A loop
    /// of voltage sources alone is left singular.
    pub(crate) fn resolve_dc_inductor_loops(&mut self) {
        let mut dc_loop = vec![false; self.inductors.len()];
        for branches in self.dc_branch_loops() {
            let windings: Vec<usize> = branches
                .iter()
                .filter_map(|branch| match *branch {
                    DcBranch::Inductor(index) => Some(index),
                    DcBranch::Source(_) => None,
                })
                .collect();
            if windings.is_empty() {
                continue;
            }
            let inductors: Vec<&str> = windings
                .iter()
                .map(|&index| self.inductors.names[index].as_str())
                .collect();
            let members: Vec<&str> = branches
                .iter()
                .map(|&branch| self.dc_branch_name(branch))
                .collect();
            crate::notice::record(
                crate::notice::EngineNotice::DcInductorLoop {
                    inductors: inductors.iter().map(|name| (*name).to_owned()).collect(),
                    resistance: DC_LOOP_RESISTANCE,
                },
                crate::log_scope::warn_line!(
                    "the DC operating point treats inductor(s) {} as {DC_LOOP_RESISTANCE:e} ohm, \
                     as QSPICE treats every inductor at DC: they close the loop {}, which \
                     shorted windings leave without a unique DC solution",
                    inductors.join(", "),
                    members.join(", "),
                ),
            );
            for index in windings {
                dc_loop[index] = true;
            }
        }
        if dc_loop.contains(&true) {
            self.inductors.dc_loop = dc_loop;
        }
    }

    fn dc_branch_name(&self, branch: DcBranch) -> &str {
        match branch {
            DcBranch::Source(index) => &self.voltage_sources.names[index],
            DcBranch::Inductor(index) => &self.inductors.names[index],
        }
    }

    /// Each loop of fixed DC branch voltages, its branches in build order,
    /// the branch that closed it last.
    fn dc_branch_loops(&self) -> Vec<Vec<DcBranch>> {
        let sources = &self.voltage_sources;
        let inductors = &self.inductors;
        let shorted = |index: usize| {
            inductors.dc_loop_resistance(index).is_none()
                && (inductors.ic_mode != InductorIcMode::AllOperatingPoints
                    || inductors.ic.get(index).copied().flatten().is_none())
        };
        let edges: Vec<(DcBranch, NodeId, NodeId)> = (0..sources.names.len())
            .map(|index| {
                (
                    DcBranch::Source(index),
                    sources.node_pos[index],
                    sources.node_neg[index],
                )
            })
            .chain(
                (0..inductors.names.len())
                    .filter(|&index| shorted(index))
                    .map(|index| {
                        (
                            DcBranch::Inductor(index),
                            inductors.node_pos[index],
                            inductors.node_neg[index],
                        )
                    }),
            )
            .collect();
        let node_count = edges
            .iter()
            .map(|&(_, positive, negative)| positive.max(negative))
            .max()
            .map_or(0, |highest| highest + 1);

        let mut parent: Vec<usize> = (0..node_count).collect();
        let mut tree: Vec<Vec<(usize, usize)>> = vec![Vec::new(); node_count];
        let mut loops = Vec::new();
        for (edge, &(branch, positive, negative)) in edges.iter().enumerate() {
            let (left, right) = (root(&mut parent, positive), root(&mut parent, negative));
            if left != right {
                parent[right] = left;
                tree[positive].push((negative, edge));
                tree[negative].push((positive, edge));
                continue;
            }
            let mut members: Vec<DcBranch> = tree_path(&tree, positive, negative)
                .into_iter()
                .map(|member| edges[member].0)
                .collect();
            members.push(branch);
            loops.push(members);
        }
        loops
    }
}

fn root(parent: &mut [usize], node: usize) -> usize {
    let mut representative = node;
    while parent[representative] != representative {
        representative = parent[representative];
    }
    let mut cursor = node;
    while parent[cursor] != representative {
        let next = parent[cursor];
        parent[cursor] = representative;
        cursor = next;
    }
    representative
}

/// The spanning-forest edges between two nodes already joined, in path order.
fn tree_path(tree: &[Vec<(usize, usize)>], from: usize, to: usize) -> Vec<usize> {
    let mut arrival: Vec<Option<(usize, usize)>> = vec![None; tree.len()];
    let mut seen = vec![false; tree.len()];
    seen[from] = true;
    let mut queue = VecDeque::from([from]);
    while let Some(node) = queue.pop_front() {
        if node == to {
            break;
        }
        for &(next, edge) in &tree[node] {
            if !seen[next] {
                seen[next] = true;
                arrival[next] = Some((node, edge));
                queue.push_back(next);
            }
        }
    }
    let mut edges = Vec::new();
    let mut cursor = to;
    while let Some((previous, edge)) = arrival[cursor] {
        edges.push(edge);
        cursor = previous;
    }
    edges.sort_unstable();
    edges
}
