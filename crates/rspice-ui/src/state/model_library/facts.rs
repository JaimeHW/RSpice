//! Facts about a model closure that more than one surface reports.
//!
//! Every fact here used to be derived twice — once by a Models & PDKs page and
//! once by the inspector beside it — and two of those pairs disagreed. A
//! fixed-geometry bin (`l_min == l_max`) read valid on the page and invalid in
//! the inspector; a source included by two libraries was counted once by the
//! page and twice by the inspector. A fact with two derivations has no owner,
//! so both derivations live here now and the surfaces read them.
//!
//! The geometry rules restate the engine's rather than approximating them: a
//! finding on the Bins page has to mean the solver would reject or ambiguate
//! the same card, otherwise the page is inventing faults. The rules mirrored
//! here are `rspice_core`'s `engine::builder::model_resolution` —
//! `validate_model_bin_range` for a reversed range,
//! `bin_ranges_overlap_with_positive_area` for overlap, and
//! `BIN_BOUND_TOLERANCE` for why a shared boundary is not an overlap. They are
//! private to core today; if they are ever widened, delete the copies below
//! and call them instead.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;

use super::{DeviceModel, ModelLibrary};

/// Slack on a bin-boundary comparison, in metres.
///
/// Mirrors core's `BIN_BOUND_TOLERANCE` (ngspice's `is_equal`). The tolerance
/// does real work: a deck writing `W=0.22u` produces a value one ULP below a
/// card's `wmin = 2.2e-007`, so comparing exactly reports boundary faults the
/// engine does not have.
const BIN_BOUND_TOLERANCE: f64 = 1e-9;

fn bin_bound_equal(left: f64, right: f64) -> bool {
    (left - right).abs() < BIN_BOUND_TOLERANCE
}

/// A card's declared geometry bounds, detached from the card.
///
/// The Bins page compares every card in a family against every other one. It
/// used to carry whole [`DeviceModel`]s — two hash maps each — through that
/// comparison, so a corpus-sized page cloned the corpus to read eight numbers
/// from it.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GeometryEnvelope {
    pub l_min: Option<f64>,
    pub l_max: Option<f64>,
    pub w_min: Option<f64>,
    pub w_max: Option<f64>,
}

impl GeometryEnvelope {
    #[must_use]
    pub const fn of(model: &DeviceModel) -> Self {
        Self {
            l_min: model.l_min,
            l_max: model.l_max,
            w_min: model.w_min,
            w_max: model.w_max,
        }
    }

    /// Whether the card declares any geometry envelope at all.
    ///
    /// The Bins page and its inspector both decide which cards belong to a
    /// geometry family, and they have to decide it the same way.
    #[must_use]
    pub const fn is_declared(self) -> bool {
        self.l_min.is_some() || self.l_max.is_some() || self.w_min.is_some() || self.w_max.is_some()
    }

    /// Whether the envelope is reversed on either axis.
    ///
    /// `min == max` is a legal point bin — the engine's range test is inclusive
    /// at both ends — so only a reversal wider than the boundary tolerance
    /// counts.
    #[must_use]
    pub fn is_invalid(self) -> bool {
        axis_is_reversed(self.l_min, self.l_max) || axis_is_reversed(self.w_min, self.w_max)
    }

    /// Whether two declared envelopes overlap over a positive area.
    ///
    /// Adjacent bins share a boundary by construction — a PDK writes one card's
    /// `lmax` as the next card's `lmin` — and the engine resolves that by
    /// taking the first match. A shared edge is therefore not a finding; only
    /// an overlap with area is. An axis left unbounded is open on that side,
    /// exactly as the engine treats it.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        axes_overlap(self.l_min, self.l_max, other.l_min, other.l_max)
            && axes_overlap(self.w_min, self.w_max, other.w_min, other.w_max)
    }
}

fn axis_is_reversed(min: Option<f64>, max: Option<f64>) -> bool {
    min.zip(max)
        .is_some_and(|(min, max)| min > max && !bin_bound_equal(min, max))
}

/// Whether a model's declared L/W envelope is reversed.
///
/// The catalog reads a card at a time, where lifting the envelope out first
/// reads worse than asking the card.
#[must_use]
pub fn envelope_is_invalid(model: &DeviceModel) -> bool {
    GeometryEnvelope::of(model).is_invalid()
}

fn axes_overlap(
    left_min: Option<f64>,
    left_max: Option<f64>,
    right_min: Option<f64>,
    right_max: Option<f64>,
) -> bool {
    let minimum = match (left_min, right_min) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (left, right) => left.or(right),
    };
    let maximum = match (left_max, right_max) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (left, right) => left.or(right),
    };
    match (minimum, maximum) {
        (Some(minimum), Some(maximum)) => maximum - minimum > BIN_BOUND_TOLERANCE,
        _ => true,
    }
}

/// The bin family a card belongs to: everything before its last `.`.
///
/// Mirrors core's `model_bin_family_name`. A binned PDK writes `nch.1`,
/// `nch.2`, … and an instance references `nch`; the engine collects candidates
/// by that prefix and compares only those against each other.
///
/// Which cards share a family is not cosmetic. The Bins page used to group by
/// device type, so two unrelated NMOS cards in two different foundry libraries
/// were reported as overlapping bins — a fault the engine does not have, since
/// it never considers them for the same instance.
#[must_use]
pub fn bin_family_name(model_name: &str) -> &str {
    model_name
        .rsplit_once('.')
        .map(|(family, _)| family)
        .filter(|family| !family.is_empty())
        .unwrap_or(model_name)
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
        if library.root_path.is_some() && library.source_closure.is_empty() {
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
