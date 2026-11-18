//! The resolved hierarchy as a graph of occurrence contexts.
//!
//! Every occurrence the design places is reached by its path from the root,
//! but occurrences that resolve identically — the same placement of the same
//! cell, under the same cells, at the same depth, with the same configuration
//! overrides able to reach them — share one node. The graph is as large as the
//! design's distinct contexts, not as large as its expansion: a cell placed a
//! million times over resolves once, an occurrence is found by walking its
//! path, counts multiply down the graph, and paths are made only when they are
//! asked for.

use super::{ConfigurationExecutionBinding, MasterKey};
use rspice_vocabulary::hierarchy_path::InstancePath;
use std::sync::Arc;

/// One context every occurrence reaching it resolves in alike.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(super) struct OccurrenceNode {
    /// What runs here, when the occurrence resolved to something that runs.
    pub(super) binding: Option<ConfigurationExecutionBinding>,
    /// The master the deck emits for it, when it emits one.
    pub(super) master: Option<MasterKey>,
    /// The grouped rows each occurrence of this node counts in, once each:
    /// its own row first, then one per child that has no path.
    pub(super) rows: Vec<usize>,
    /// Whether its own row resolved.
    pub(super) resolved: bool,
    /// Its children, in the order its master places them.
    pub(super) children: Vec<OccurrenceEdge>,
    /// Indices into `children`, ordered by folded name, to find one by path.
    pub(super) by_name: Vec<usize>,
    /// How many occurrences reach this node.
    pub(super) multiplicity: usize,
}

/// One placed child: its name and the node it resolves to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OccurrenceEdge {
    pub(super) name: String,
    pub(super) folded: String,
    pub(super) node: usize,
}

/// Every occurrence of a resolved hierarchy, as the graph of its contexts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HierarchyOccurrences {
    /// The root is the first node, when the walk reached the design root.
    pub(super) nodes: Vec<OccurrenceNode>,
    /// Row entries no node holds, with the path each names: a configured
    /// path that names no instance, or a refusal of the whole design.
    pub(super) listed: Vec<(usize, InstancePath)>,
}

/// A name as paths compare it.
pub(super) fn fold(name: &str) -> String {
    name.chars().flat_map(char::to_lowercase).collect()
}

impl HierarchyOccurrences {
    /// The node `path` reaches, when the design places an instance there.
    pub(super) fn node_at(&self, path: &InstancePath) -> Option<usize> {
        if self.nodes.is_empty() {
            return None;
        }
        path.segments()
            .iter()
            .try_fold(0, |node, segment| self.child(node, &fold(segment)))
    }

    /// The child of `node` named `folded`.
    pub(super) fn child(&self, node: usize, folded: &str) -> Option<usize> {
        self.child_at(node, folded)
            .map(|at| self.nodes[node].children[at].node)
    }

    /// Where among `node`'s children the one named `folded` is.
    fn child_at(&self, node: usize, folded: &str) -> Option<usize> {
        let node = &self.nodes[node];
        node.by_name
            .binary_search_by(|&at| node.children[at].folded.as_str().cmp(folded))
            .ok()
            .map(|at| node.by_name[at])
    }

    /// The design root's context, when the walk reached the root.
    pub fn root(&self) -> Option<usize> {
        (!self.nodes.is_empty()).then_some(0)
    }

    /// The context the occurrence at `path` resolved in, and its path as the
    /// design spells it.
    pub fn find(&self, path: &InstancePath) -> Option<(usize, InstancePath)> {
        let mut node = self.root()?;
        let mut spelled = InstancePath::root();
        for segment in path.segments() {
            let edge = &self.nodes[node].children[self.child_at(node, &fold(segment))?];
            spelled = spelled.child(&edge.name).ok()?;
            node = edge.node;
        }
        Some((node, spelled))
    }

    /// The instances an occurrence of `node` places, in the order its master
    /// places them, each with the context it resolved in. A name placed
    /// twice is the first placement of it, as a path finds it.
    pub fn placed(&self, node: usize) -> impl Iterator<Item = (&str, usize)> + '_ {
        self.nodes[node]
            .children
            .iter()
            .enumerate()
            .filter(move |(at, edge)| self.child_at(node, &edge.folded) == Some(*at))
            .map(|(_, edge)| (edge.name.as_str(), edge.node))
    }

    /// The grouped row an occurrence of `node` counts in.
    pub fn own_row(&self, node: usize) -> Option<usize> {
        self.nodes[node].rows.first().copied()
    }

    /// How many occurrences resolved in `node`.
    pub fn occurrences_of(&self, node: usize) -> usize {
        self.nodes[node].multiplicity
    }

    /// How many occurrences the design places, the root among them.
    pub fn len(&self) -> usize {
        self.nodes.iter().fold(0_usize, |total, node| {
            total.saturating_add(node.multiplicity)
        })
    }

    /// Whether the walk reached no occurrence at all.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Paths a configuration names that the design does not place, or a
    /// refusal of the whole design at its root, each with its grouped row.
    pub fn unplaced(&self) -> impl Iterator<Item = (usize, &InstancePath)> + '_ {
        self.listed.iter().map(|(row, path)| (*row, path))
    }

    /// A hierarchy named by its occurrences' paths, each with its grouped
    /// row and its own context, in the order the design places them: what a
    /// test or an importer states. A path whose parent is not among them is
    /// one the design does not place.
    pub fn from_paths(paths: impl IntoIterator<Item = (usize, InstancePath)>) -> Self {
        let mut graph = Self::default();
        let mut at = std::collections::HashMap::new();
        for (row, path) in paths {
            if path.is_root() && graph.nodes.is_empty() {
                graph.nodes.push(OccurrenceNode {
                    rows: vec![row],
                    ..OccurrenceNode::default()
                });
                at.insert(path.fold_key(), 0);
                continue;
            }
            let parent = path
                .parent()
                .and_then(|parent| at.get(&parent.fold_key()).copied());
            let (Some(parent), Some(name)) = (parent, path.segments().last()) else {
                graph.listed.push((row, path));
                continue;
            };
            let node = graph.nodes.len();
            graph.nodes.push(OccurrenceNode {
                rows: vec![row],
                ..OccurrenceNode::default()
            });
            graph.nodes[parent].children.push(OccurrenceEdge {
                name: name.clone(),
                folded: fold(name),
                node,
            });
            at.insert(path.fold_key(), node);
        }
        graph.seal();
        graph
    }

    /// Order each node's children by name and count the occurrences that
    /// reach each node: one reaches the root, and each edge carries every
    /// occurrence of its parent to its child.
    pub(super) fn seal(&mut self) {
        for node in &mut self.nodes {
            let children = &node.children;
            let mut by_name = (0..children.len()).collect::<Vec<_>>();
            by_name.sort_by(|left, right| children[*left].folded.cmp(&children[*right].folded));
            // Two spellings of one name are one instance: the first placed
            // is the one a path finds.
            by_name.dedup_by(|right, left| children[*right].folded == children[*left].folded);
            node.by_name = by_name;
        }
        let order = self.topological_order();
        for node in &mut self.nodes {
            node.multiplicity = 0;
        }
        if let Some(root) = self.nodes.first_mut() {
            root.multiplicity = 1;
        }
        for node in order {
            let multiplicity = self.nodes[node].multiplicity;
            for at in 0..self.nodes[node].children.len() {
                let child = self.nodes[node].children[at].node;
                self.nodes[child].multiplicity =
                    self.nodes[child].multiplicity.saturating_add(multiplicity);
            }
        }
    }

    /// Every node reachable from the root, each before the nodes under it.
    pub(super) fn topological_order(&self) -> Vec<usize> {
        let mut finished = Vec::with_capacity(self.nodes.len());
        if self.nodes.is_empty() {
            return finished;
        }
        let mut seen = vec![false; self.nodes.len()];
        let mut stack = vec![(0_usize, 0_usize)];
        seen[0] = true;
        while let Some((node, next)) = stack.last_mut() {
            let children = &self.nodes[*node].children;
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

    /// Nodes in the order a walk of every occurrence first reaches them.
    pub(super) fn first_reached(&self) -> Vec<usize> {
        let mut order = Vec::with_capacity(self.nodes.len());
        if self.nodes.is_empty() {
            return order;
        }
        let mut seen = vec![false; self.nodes.len()];
        let mut stack = vec![(0_usize, 0_usize)];
        seen[0] = true;
        order.push(0);
        while let Some((node, next)) = stack.last_mut() {
            let children = &self.nodes[*node].children;
            if *next < children.len() {
                let child = children[*next].node;
                *next += 1;
                if !seen[child] {
                    seen[child] = true;
                    order.push(child);
                    stack.push((child, 0));
                }
            } else {
                stack.pop();
            }
        }
        order
    }

    /// For each node, whether it or a node under it is `wanted`.
    fn reaching(&self, wanted: &[bool]) -> Vec<bool> {
        let mut reaches = wanted.to_vec();
        let mut order = self.topological_order();
        order.reverse();
        for node in order {
            if !reaches[node] {
                reaches[node] = self.nodes[node]
                    .children
                    .iter()
                    .any(|edge| reaches[edge.node]);
            }
        }
        reaches
    }

    /// The paths of every occurrence of a `wanted` node, in walk order.
    pub(super) fn walk(&self, wanted: Vec<bool>) -> OccurrenceWalk<'_> {
        let reaches = self.reaching(&wanted);
        OccurrenceWalk {
            graph: self,
            wanted,
            reaches,
            stack: Vec::new(),
            started: false,
        }
    }

    /// How many occurrences count in `row`.
    pub(super) fn row_count(&self, row: usize) -> usize {
        let held = self.nodes.iter().fold(0_usize, |total, node| {
            let times = node
                .rows
                .iter()
                .filter(|candidate| **candidate == row)
                .count();
            total.saturating_add(node.multiplicity.saturating_mul(times))
        });
        held.saturating_add(self.listed.iter().filter(|(at, _)| *at == row).count())
    }

    /// The paths that count in `row`: the graph's in walk order, then the
    /// listed ones.
    pub(super) fn row_paths(&self, row: usize) -> impl Iterator<Item = InstancePath> + '_ {
        let wanted = self
            .nodes
            .iter()
            .map(|node| node.rows.contains(&row))
            .collect();
        self.walk(wanted).map(|(path, _)| path).chain(
            self.listed
                .iter()
                .filter(move |(at, _)| *at == row)
                .map(|(_, path)| path.clone()),
        )
    }
}

/// A depth-first walk of the occurrences of some nodes, which never enters
/// a subtree none of them is in.
pub(super) struct OccurrenceWalk<'a> {
    graph: &'a HierarchyOccurrences,
    wanted: Vec<bool>,
    reaches: Vec<bool>,
    stack: Vec<(usize, InstancePath, usize)>,
    started: bool,
}

impl Iterator for OccurrenceWalk<'_> {
    type Item = (InstancePath, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            if self.reaches.first().copied().unwrap_or(false) {
                self.stack.push((0, InstancePath::root(), 0));
                if self.wanted[0] {
                    return Some((InstancePath::root(), 0));
                }
            }
        }
        loop {
            let (node, path, next) = self.stack.last_mut()?;
            let Some(edge) = self.graph.nodes[*node].children.get(*next) else {
                self.stack.pop();
                continue;
            };
            *next += 1;
            if !self.reaches[edge.node] {
                continue;
            }
            let Ok(child) = path.child(&edge.name) else {
                continue;
            };
            let found = self.wanted[edge.node].then(|| (child.clone(), edge.node));
            self.stack.push((edge.node, child, 0));
            if found.is_some() {
                return found;
            }
        }
    }
}

/// The occurrences one grouped row counts: how many, and their paths when
/// they are asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowOccurrences {
    graph: Arc<HierarchyOccurrences>,
    row: usize,
}

impl RowOccurrences {
    pub(super) const fn new(graph: Arc<HierarchyOccurrences>, row: usize) -> Self {
        Self { graph, row }
    }

    /// Occurrences named by their paths alone, as a row a test or an
    /// importer states.
    pub fn listed(paths: impl IntoIterator<Item = InstancePath>) -> Self {
        Self {
            graph: Arc::new(HierarchyOccurrences {
                nodes: Vec::new(),
                listed: paths.into_iter().map(|path| (0, path)).collect(),
            }),
            row: 0,
        }
    }

    /// The occurrences' paths, in the order the design places them.
    pub fn iter(&self) -> impl Iterator<Item = InstancePath> + '_ {
        self.graph.row_paths(self.row)
    }

    /// The first occurrence the design places.
    pub fn first(&self) -> Option<InstancePath> {
        self.iter().next()
    }

    /// Every occurrence's path as written, ordered and without repeats, the
    /// way a list of them reads.
    pub fn paths(&self) -> Vec<String> {
        let mut paths = self
            .iter()
            .map(|path| (path.fold_key(), path.to_string()))
            .collect::<Vec<_>>();
        paths.sort();
        paths.dedup_by(|right, left| right.0 == left.0);
        paths.into_iter().map(|(_, path)| path).collect()
    }

    /// How many occurrences there are.
    pub fn len(&self) -> usize {
        self.graph.row_count(self.row)
    }

    /// Whether there are none.
    pub fn is_empty(&self) -> bool {
        self.first().is_none()
    }

    /// The first occurrence, then how many more there are: what a message
    /// that names the occurrences says, however many there are.
    pub fn summary(&self) -> String {
        let Some(first) = self.first() else {
            return String::new();
        };
        match self.len().saturating_sub(1) {
            0 => first.to_string(),
            1 => format!("{first} and 1 more"),
            more => format!("{first} and {more} more"),
        }
    }
}

/// One occurrence and what runs there.
#[derive(Debug, Clone)]
pub struct OccurrenceBinding<'a> {
    path: InstancePath,
    binding: &'a ConfigurationExecutionBinding,
}

impl<'a> OccurrenceBinding<'a> {
    /// The occurrence's own path.
    pub const fn instance_path(&self) -> &InstancePath {
        &self.path
    }

    /// What runs there, which every occurrence of its context shares.
    pub const fn binding(&self) -> &'a ConfigurationExecutionBinding {
        self.binding
    }
}

impl std::ops::Deref for OccurrenceBinding<'_> {
    type Target = ConfigurationExecutionBinding;

    fn deref(&self) -> &Self::Target {
        self.binding
    }
}

/// Every occurrence that runs something, with what it runs.
pub struct PlanBindings<'a> {
    graph: &'a HierarchyOccurrences,
    walk: OccurrenceWalk<'a>,
    remaining: usize,
}

impl<'a> PlanBindings<'a> {
    pub(super) fn new(graph: &'a HierarchyOccurrences) -> Self {
        let wanted = graph
            .nodes
            .iter()
            .map(|node| node.binding.is_some())
            .collect::<Vec<_>>();
        let remaining = graph
            .nodes
            .iter()
            .filter(|node| node.binding.is_some())
            .fold(0_usize, |total, node| {
                total.saturating_add(node.multiplicity)
            });
        Self {
            graph,
            walk: graph.walk(wanted),
            remaining,
        }
    }
}

impl<'a> Iterator for PlanBindings<'a> {
    type Item = OccurrenceBinding<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (path, node) = self.walk.next()?;
        self.remaining = self.remaining.saturating_sub(1);
        let binding = self.graph.nodes[node].binding.as_ref()?;
        Some(OccurrenceBinding { path, binding })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for PlanBindings<'_> {}
