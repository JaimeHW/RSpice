//! Loops the DC operating-point system closes through fixed branch voltages.
//!
//! An independent voltage source states `V(+) - V(-)` outright, and the DC
//! solve stamps an inductor as a zero-volt branch (`stamp_dc_short`). A cycle
//! of such branches makes its branch rows sum to a constant and leaves the
//! current circulating around it undetermined, so the DC matrix is singular
//! whatever the source values. Nodal GMIN and `.OPTIONS RSHUNT` stamp node
//! rows, never branch rows, so no conditioning reaches it: every DC solve of
//! the circuit is refused.

use std::collections::VecDeque;

use super::*;

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

impl CircuitData {
    /// Every loop the DC operating-point system closes through independent
    /// voltage sources and DC-shorted inductors.
    ///
    /// An inductor with `IC=` under Xyce's all-operating-points mode is a
    /// current constraint in the DC solve, not a short, and is not an edge.
    pub fn dc_branch_voltage_loops(&self) -> Vec<DcBranchVoltageLoop> {
        let sources = &self.voltage_sources;
        let inductors = &self.inductors;
        let shorted = |index: usize| {
            inductors.ic_mode != InductorIcMode::AllOperatingPoints
                || inductors.ic.get(index).copied().flatten().is_none()
        };
        let edges: Vec<(&str, NodeId, NodeId, bool)> = (0..sources.names.len())
            .map(|index| {
                (
                    sources.names[index].as_str(),
                    sources.node_pos[index],
                    sources.node_neg[index],
                    false,
                )
            })
            .chain((0..inductors.names.len()).filter(|&index| shorted(index)).map(
                |index| {
                    (
                        inductors.names[index].as_str(),
                        inductors.node_pos[index],
                        inductors.node_neg[index],
                        true,
                    )
                },
            ))
            .collect();
        let node_count = edges
            .iter()
            .map(|&(_, positive, negative, _)| positive.max(negative))
            .max()
            .map_or(0, |highest| highest + 1);

        let mut parent: Vec<usize> = (0..node_count).collect();
        let mut tree: Vec<Vec<(usize, usize)>> = vec![Vec::new(); node_count];
        let mut loops = Vec::new();
        for (edge, &(name, positive, negative, inductor)) in edges.iter().enumerate() {
            let (left, right) = (root(&mut parent, positive), root(&mut parent, negative));
            if left != right {
                parent[right] = left;
                tree[positive].push((negative, edge));
                tree[negative].push((positive, edge));
                continue;
            }
            let path = tree_path(&tree, positive, negative);
            let through_inductor = inductor || path.iter().any(|&member| edges[member].3);
            let mut members: Vec<String> = path
                .into_iter()
                .map(|member| edges[member].0.to_owned())
                .collect();
            members.push(name.to_owned());
            loops.push(DcBranchVoltageLoop {
                members,
                through_inductor,
            });
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
