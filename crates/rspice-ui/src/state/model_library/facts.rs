//! Facts about a model closure that more than one surface reports.
//!
//! Every fact here used to be derived twice — once by a Models & PDKs page and
//! once by the inspector beside it — and two of those pairs disagreed. A
//! fixed-geometry bin (`l_min == l_max`) read valid on the page and invalid in
//! the inspector; a source included by two libraries was counted once by the
//! page and twice by the inspector. A fact with two derivations has no owner,
//! so both derivations live here now and the surfaces read them.
//!
//! Geometry selection and overlap belong exclusively to the simulator. The
//! catalog asks the simulator's public axis primitive about reversed metadata;
//! the Bins workspace consumes the simulator's complete inspection receipt.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

use super::{DeviceModel, ModelLibrary};

/// Whether a model's declared L/W envelope is reversed.
///
/// The catalog reads a card at a time, where lifting the envelope out first
/// reads worse than asking the card.
#[must_use]
pub fn envelope_is_invalid(model: &DeviceModel) -> bool {
    rspice_core::engine::ModelBinAxisRange {
        min: model.l_min,
        max: model.l_max,
    }
    .is_reversed()
        || rspice_core::engine::ModelBinAxisRange {
            min: model.w_min,
            max: model.w_max,
        }
        .is_reversed()
}

impl ModelLibrary {
    /// Whether the library names an external root a pin could be taken from.
    ///
    /// The root is *where* a closure would be pinned from; whether one has
    /// been taken is [`has_retained_closure`](Self::has_retained_closure). A
    /// pane offering to pin asks this one, because the offer means nothing
    /// without a root to read.
    #[must_use]
    pub fn has_external_root(&self) -> bool {
        self.root_path.is_some()
    }

    /// Whether the library holds the retained closure a run is reproduced from.
    #[must_use]
    pub fn has_retained_closure(&self) -> bool {
        !self.source_closure.is_empty()
    }

    /// Whether the library names an external root it has taken no pin from.
    ///
    /// This is one finding a catalog row, a detail pane, and the include-graph
    /// diagnostics all report about the same library, and the conjunction is
    /// why it lives here: spelled out at each site, "has a root" and "has no
    /// closure" are two conditions a later edit can drift apart, and a library
    /// that reads unpinned in one pane and fine in the next is worse than
    /// either answer alone. An unpinned external binding is never runnable, so
    /// every reader of this is asking the same question — whether the library
    /// can execute at all.
    #[must_use]
    pub fn is_unpinned_root(&self) -> bool {
        self.has_external_root() && !self.has_retained_closure()
    }
}

/// A digest, shortened for display.
///
/// One rendering, because a digest shown two ways in two panes reads as two
/// digests. Head and tail are both kept so a shared prefix stays visible.
#[must_use]
pub fn short_digest(digest: &str) -> String {
    if digest.len() <= 12 {
        digest.to_owned()
    } else {
        format!("{}…{}", &digest[..8], &digest[digest.len() - 4..])
    }
}

/// What the retained include closure is, over a set of libraries.
///
/// `files` counts each source once however many libraries include it. Summing
/// `source_closure.len()` instead double-counts a shared include, which is how
/// the Include Graph page and its inspector used to disagree.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ClosureFacts {
    /// Distinct source files across every retained closure.
    pub files: usize,
    /// Retained dependency edges.
    pub edges: usize,
    /// Sources left over after a topological sort, so members of a cycle.
    pub cyclic_nodes: usize,
    /// Libraries that have an external root but no retained closure.
    pub unpinned_roots: usize,
}

impl ClosureFacts {
    /// Findings a reader should act on.
    #[must_use]
    pub const fn diagnostics(&self) -> usize {
        self.unpinned_roots + self.cyclic_nodes
    }
}

/// Derive the closure facts for a set of libraries.
pub fn closure_facts<'a>(libraries: impl IntoIterator<Item = &'a ModelLibrary>) -> ClosureFacts {
    let mut sources = HashSet::<&PathBuf>::new();
    let mut edges = Vec::<(&PathBuf, &PathBuf)>::new();
    let mut unpinned_roots = 0usize;
    for library in libraries {
        if library.is_unpinned_root() {
            unpinned_roots += 1;
        }
        sources.extend(library.source_closure.iter().map(|source| &source.path));
        edges.extend(
            library
                .source_edges
                .iter()
                .map(|edge| (&edge.owner, &edge.target)),
        );
    }
    ClosureFacts {
        files: sources.len(),
        cyclic_nodes: cyclic_node_count(&edges),
        edges: edges.len(),
        unpinned_roots,
    }
}

/// Sources that remain after a topological sort, so members of a cycle.
fn cyclic_node_count(edges: &[(&PathBuf, &PathBuf)]) -> usize {
    let mut nodes = HashSet::<&PathBuf>::new();
    let mut indegree = HashMap::<&PathBuf, usize>::new();
    let mut outgoing = HashMap::<&PathBuf, Vec<&PathBuf>>::new();
    for (owner, target) in edges {
        nodes.insert(owner);
        nodes.insert(target);
        outgoing.entry(owner).or_default().push(target);
        *indegree.entry(target).or_default() += 1;
        indegree.entry(owner).or_default();
    }
    let mut queue = indegree
        .iter()
        .filter_map(|(node, degree)| (*degree == 0).then_some(*node))
        .collect::<Vec<_>>();
    let mut visited = 0usize;
    while let Some(node) = queue.pop() {
        visited += 1;
        for target in outgoing.get(node).into_iter().flatten() {
            if let Some(degree) = indegree.get_mut(target) {
                *degree = degree.saturating_sub(1);
                if *degree == 0 {
                    queue.push(target);
                }
            }
        }
    }
    nodes.len().saturating_sub(visited)
}

impl ModelLibrary {
    /// Section names the retained closure actually defines, lower-cased.
    ///
    /// Derived from what parsing produced — every model and subcircuit records
    /// the section it came from — rather than by searching the retained bytes
    /// for a `.lib` line. A text search reports a section missing whenever the
    /// file spells the directive with a tab or extra spacing, and reports one
    /// present when the string appears in a comment.
    ///
    /// A section that defines nothing is not in this index. That is the
    /// intended reading: a corner cannot bind to a section with no content.
    #[must_use]
    pub fn section_index(&self) -> BTreeSet<String> {
        self.models
            .values()
            .filter_map(|model| model.section.as_deref())
            .chain(
                self.subcircuits
                    .values()
                    .filter_map(|subcircuit| subcircuit.section.as_deref()),
            )
            .map(str::to_ascii_lowercase)
            .collect()
    }

    /// Whether the retained closure defines a section, case-insensitively.
    #[must_use]
    pub fn defines_section(&self, section: &str) -> bool {
        let section = section.to_ascii_lowercase();
        self.models
            .values()
            .filter_map(|model| model.section.as_deref())
            .chain(
                self.subcircuits
                    .values()
                    .filter_map(|subcircuit| subcircuit.section.as_deref()),
            )
            .any(|candidate| candidate.eq_ignore_ascii_case(&section))
    }
}

#[cfg(test)]
mod tests;
