//! Loops of independent voltage sources.
//!
//! An ideal voltage source fixes the difference between its two nodes and
//! leaves its branch current to the rest of the circuit. Sources that close a
//! loop among themselves — two across one pair of nodes, one with both
//! terminals on one node, or a longer ring such as `a`–`b`, `b`–`c`, `c`–`a` —
//! leave the current around that loop undetermined, and the engine refuses the
//! design before it solves (`VoltageSources::finalize_constraint_projection`
//! in rspice-core). Sources that merely share a node do not: V1 from `in` to
//! ground beside V2 from `in` to `vb` is an ordinary circuit.
//!
//! The engine's graph has the independent sources as its only edges, and
//! ground as one node however it is spelled, so this one does too: the deck
//! writes every ground name as `0`, so V1 from `vdd` to `0` beside V2 from
//! `vdd` to `GND!` is two sources across one pair of nodes. An inductor
//! or a dependent source can also close a loop the engine cannot solve, but
//! that is for the engine to say about the deck it actually builds; a rule
//! that guessed at it would refuse designs the engine solves.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// The node every spelling of ground is in the engine's graph.
pub(super) const GROUND_KEY: &str = "0";

/// The node a source terminal stands on, as the engine's graph has it: ground
/// is one node whatever the drawing calls it.
pub(super) fn node_key(net: &str) -> &str {
    if crate::schematic::ground_names::is_ground_reference(net) {
        GROUND_KEY
    } else {
        net
    }
}

/// Each loop the branches close, in the order they are given.
///
/// A branch closes a loop when the branches before it already join its two
/// nodes, which is how the engine meets them and the one it names: the loop
/// is the path that joined the two nodes, then the closing branch. A branch
/// from a node to itself is a loop of one. There is one loop per branch more
/// than a tree needs, however the branches are ordered.
pub(super) fn closed_loops<K: Eq + Hash>(branches: &[[K; 2]]) -> Vec<Vec<usize>> {
    let mut nodes = HashMap::<&K, usize>::new();
    let mut parents = Vec::<usize>::new();
    // The branches that joined each node, as (other node, branch). Only
    // branches that closed no loop are here, so this is a forest and the path
    // between two joined nodes is unique.
    let mut joined = Vec::<Vec<(usize, usize)>>::new();
    let mut loops = Vec::new();
    for (branch, ends) in branches.iter().enumerate() {
        let [first, second] = [&ends[0], &ends[1]].map(|key| {
            *nodes.entry(key).or_insert_with(|| {
                parents.push(parents.len());
                joined.push(Vec::new());
                parents.len() - 1
            })
        });
        let (first_root, second_root) = (root(&mut parents, first), root(&mut parents, second));
        if first_root == second_root {
            let mut members = path(&joined, first, second);
            members.push(branch);
            loops.push(members);
            continue;
        }
        parents[first_root] = second_root;
        joined[first].push((second, branch));
        joined[second].push((first, branch));
    }
    loops
}

/// The representative of `node`'s set, compressing the path to it.
fn root(parents: &mut [usize], node: usize) -> usize {
    let mut representative = node;
    while parents[representative] != representative {
        representative = parents[representative];
    }
    let mut node = node;
    while parents[node] != representative {
        let next = parents[node];
        parents[node] = representative;
        node = next;
    }
    representative
}

/// The branches of the path the forest makes from `from` to `to`, in order.
fn path(joined: &[Vec<(usize, usize)>], from: usize, to: usize) -> Vec<usize> {
    let mut reached_by = vec![None; joined.len()];
    let mut seen = vec![false; joined.len()];
    seen[from] = true;
    let mut queue = VecDeque::from([from]);
    while let Some(node) = queue.pop_front() {
        if node == to {
            break;
        }
        for &(next, branch) in &joined[node] {
            if !seen[next] {
                seen[next] = true;
                reached_by[next] = Some((node, branch));
                queue.push_back(next);
            }
        }
    }
    let mut members = Vec::new();
    let mut node = to;
    while let Some((previous, branch)) = reached_by[node] {
        members.push(branch);
        node = previous;
    }
    members.reverse();
    members
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_sources_that_close_a_ring_are_a_loop() {
        // Sharing a node is not a loop.
        assert!(closed_loops(&[["in", "0"], ["in", "vb"]]).is_empty());
        // Two across one pair, in either orientation.
        assert_eq!(closed_loops(&[["a", "0"], ["0", "a"]]), vec![vec![0, 1]]);
        // A ring of three, closed by the last: the path runs from its first
        // node, `c`, back round to its second.
        assert_eq!(
            closed_loops(&[["a", "b"], ["b", "c"], ["c", "a"]]),
            vec![vec![1, 0, 2]]
        );
        // A source on one node is a loop of its own.
        assert_eq!(closed_loops(&[["a", "a"]]), vec![vec![0]]);
        // One loop per source too many, each named by the source closing it.
        assert_eq!(
            closed_loops(&[["a", "0"], ["a", "0"], ["a", "0"], ["b", "0"]]),
            vec![vec![0, 1], vec![0, 2]]
        );
        // Two trees joined later still give the whole path, from the closing
        // source's first node to its second.
        assert_eq!(
            closed_loops(&[["a", "b"], ["c", "d"], ["b", "c"], ["d", "a"]]),
            vec![vec![1, 2, 0, 3]]
        );
    }
}
