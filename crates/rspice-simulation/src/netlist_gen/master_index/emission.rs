//! Which master each occurrence of a deck was emitted against.
//!
//! The deck is hierarchical: a master is written once, however many times it
//! is placed. The map is too. It holds one node per distinct way the walk
//! reached a placement, with that placement's master and the placements under
//! it, so it is as large as the deck rather than as large as the design's
//! expansion. An occurrence's master is found by walking its path, and the
//! rows naming every occurrence are made only when something reads them.

use std::sync::Arc;

use rspice_design_model::cell_view::CellViewRef;
use rspice_vocabulary::hierarchy_path::InstancePath;

use super::EmissionRow;

/// One placement context: the master it was emitted against, when it was,
/// and the placements under it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct EmissionNode {
    pub(super) master: Option<usize>,
    pub(super) children: Vec<EmissionEdge>,
    /// Indices into `children`, ordered by folded name.
    by_name: Vec<usize>,
}

/// One placed child and the node it was emitted as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct EmissionEdge {
    pub(super) name: String,
    folded: String,
    pub(super) node: usize,
}

impl EmissionEdge {
    pub(super) fn new(name: &str, node: usize) -> Self {
        Self {
            name: name.to_owned(),
            folded: fold(name),
            node,
        }
    }
}

/// A master the deck emitted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct EmittedMaster {
    pub(super) name: String,
    pub(super) reference: CellViewRef,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct EmissionGraph {
    /// The occurrence the deck's top level was generated at.
    root: InstancePath,
    /// The top level is the first node.
    nodes: Vec<EmissionNode>,
    masters: Vec<EmittedMaster>,
    len: usize,
}

/// Which master each occurrence of the deck was emitted against.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EmissionMap {
    graph: Arc<EmissionGraph>,
}

fn fold(name: &str) -> String {
    name.chars().flat_map(char::to_lowercase).collect()
}

impl EmissionMap {
    /// Seal the walk's nodes, the first being the top level at `root`.
    pub(super) fn new(
        root: InstancePath,
        mut nodes: Vec<EmissionNode>,
        masters: Vec<EmittedMaster>,
    ) -> Self {
        for node in &mut nodes {
            let children = &node.children;
            let mut by_name = (0..children.len()).collect::<Vec<_>>();
            by_name.sort_by(|left, right| children[*left].folded.cmp(&children[*right].folded));
            by_name.dedup_by(|right, left| children[*right].folded == children[*left].folded);
            node.by_name = by_name;
        }
        // Each edge carries every occurrence of its parent to its child.
        let mut reaching = vec![0_usize; nodes.len()];
        let mut len = 0_usize;
        if !nodes.is_empty() {
            reaching[0] = 1;
            for node in topological_order(&nodes) {
                if node != 0 && nodes[node].master.is_some() {
                    len = len.saturating_add(reaching[node]);
                }
                for at in 0..nodes[node].children.len() {
                    let child = nodes[node].children[at].node;
                    reaching[child] = reaching[child].saturating_add(reaching[node]);
                }
            }
        }
        Self {
            graph: Arc::new(EmissionGraph {
                root,
                nodes,
                masters,
                len,
            }),
        }
    }

    /// How many occurrences the deck emitted a master for.
    pub fn len(&self) -> usize {
        self.graph.len
    }

    pub fn is_empty(&self) -> bool {
        self.graph.len == 0
    }

    /// The master `occurrence` was emitted against.
    pub fn master_at(&self, occurrence: &InstancePath) -> Option<&str> {
        let graph = &*self.graph;
        if graph.nodes.is_empty() || !occurrence.starts_with(&graph.root) {
            return None;
        }
        let node = occurrence.segments()[graph.root.depth()..]
            .iter()
            .try_fold(0, |node, segment| {
                let node = &graph.nodes[node];
                let folded = fold(segment);
                node.by_name
                    .binary_search_by(|&at| node.children[at].folded.as_str().cmp(&folded))
                    .ok()
                    .map(|at| node.children[node.by_name[at]].node)
            })?;
        if node == 0 {
            return None;
        }
        let master = graph.nodes[node].master?;
        Some(graph.masters[master].name.as_str())
    }

    /// Every occurrence and its master, each after the occurrences under it:
    /// the order the deck defines them in.
    pub fn iter(&self) -> impl Iterator<Item = EmissionRow> + '_ {
        let graph = &*self.graph;
        let mut stack = Vec::new();
        if !graph.nodes.is_empty() {
            stack.push((0_usize, graph.root.clone(), 0_usize));
        }
        std::iter::from_fn(move || {
            loop {
                let (node, path, next) = stack.last_mut()?;
                if let Some(edge) = graph.nodes[*node].children.get(*next) {
                    *next += 1;
                    if let Ok(child) = path.child(&edge.name) {
                        stack.push((edge.node, child, 0));
                    }
                    continue;
                }
                let (node, occurrence, _) = stack.pop()?;
                if node == 0 {
                    continue;
                }
                let Some(master) = graph.nodes[node].master else {
                    continue;
                };
                let master = &graph.masters[master];
                return Some(EmissionRow {
                    engine_prefix: occurrence
                        .to_engine_name()
                        .map(|scope| scope.to_ascii_uppercase())
                        .unwrap_or_default(),
                    master: master.name.clone(),
                    master_reference: master.reference.clone(),
                    occurrence,
                });
            }
        })
    }
}

/// Every node reachable from the first, each before the nodes under it.
fn topological_order(nodes: &[EmissionNode]) -> Vec<usize> {
    let mut finished = Vec::with_capacity(nodes.len());
    let mut seen = vec![false; nodes.len()];
    let mut stack = vec![(0_usize, 0_usize)];
    seen[0] = true;
    while let Some((node, next)) = stack.last_mut() {
        let children = &nodes[*node].children;
        if *next < children.len() {
            let child = children[*next].node;
            *next += 1;
            if !seen[child] {
                seen[child] = true;
                stack.push((child, 0));
            }
        } else {
            finished.push(*node);
            stack.pop();
        }
    }
    finished.reverse();
    finished
}
