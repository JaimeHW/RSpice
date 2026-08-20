//! Where a navigator tree is unfolded, and which node it is pointing at.
//!
//! Expansion is a reading position rather than project data, so it is keyed by
//! what the design calls the node — a folded instance path, or a folded master
//! reference — and never by a row index. That is what lets it survive a
//! re-render, a re-resolve, and a design edit that inserts a row above it.
//!
//! One state per workspace. A navigator serves whichever workspace is showing,
//! and each of them reads a different tree, so a shared unfold would move a
//! reader's position in a workspace they are not looking at. The filter box
//! above the tree is the same kind of reading position and is kept here for
//! the same reason: one shared query would carry a search for a net into the
//! workspace that lists models.

use std::collections::{BTreeSet, HashMap};

use crate::state::InstancePath;

use super::Workspace;

/// One node a navigator tree can unfold.
///
/// Masters and occurrences key two different spaces over the same design — one
/// cell view is one master and many occurrences — so the variant is what keeps
/// a folded master reference from colliding with a folded instance path.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NavigatorTreeNode {
    /// A master group, keyed by its folded `library` or `library/cell`.
    Master(String),
    /// An occurrence, keyed by [`InstancePath::fold_key`].
    Occurrence(String),
}

/// What one navigator's tree has unfolded, selected, and been asked to reveal.
#[derive(Debug, Clone, Default)]
pub struct NavigatorTreeState {
    expanded: BTreeSet<NavigatorTreeNode>,
    selection: Option<InstancePath>,
    scroll_to: Option<InstancePath>,
}

impl NavigatorTreeState {
    pub fn is_expanded(&self, node: &NavigatorTreeNode) -> bool {
        self.expanded.contains(node)
    }

    pub fn toggle(&mut self, node: NavigatorTreeNode) {
        if !self.expanded.remove(&node) {
            self.expanded.insert(node);
        }
    }

    pub fn expand(&mut self, node: NavigatorTreeNode) {
        self.expanded.insert(node);
    }

    /// The occurrence the tree is pointing at.
    pub fn selection(&self) -> Option<&InstancePath> {
        self.selection.as_ref()
    }

    pub fn select(&mut self, occurrence: InstancePath) {
        self.selection = Some(occurrence);
    }

    /// Point at one occurrence and ask the tree to bring it into view.
    pub fn reveal(&mut self, occurrence: InstancePath) {
        self.scroll_to = Some(occurrence.clone());
        self.selection = Some(occurrence);
    }

    /// The occurrence to bring into view, taken by the frame that does it: a
    /// reveal that stayed pending would fight every later scroll.
    pub fn take_scroll_to(&mut self) -> Option<InstancePath> {
        self.scroll_to.take()
    }
}

/// One navigator tree state and one filter query per workspace.
///
/// The two are separate maps because a tree state is taken out and put back
/// while its rows are built, and the filter has to stay readable throughout.
#[derive(Debug, Clone, Default)]
pub struct NavigatorTrees {
    trees: HashMap<Workspace, NavigatorTreeState>,
    filters: HashMap<Workspace, String>,
}

impl NavigatorTrees {
    pub fn for_workspace(&mut self, workspace: Workspace) -> &mut NavigatorTreeState {
        self.trees.entry(workspace).or_default()
    }

    /// What this workspace's navigator is filtering by. A workspace nobody has
    /// typed a query into filters nothing.
    pub fn filter(&self, workspace: Workspace) -> &str {
        self.filters
            .get(&workspace)
            .map_or("", |filter| filter.as_str())
    }

    /// The buffer the workspace's filter box edits in place.
    pub fn filter_mut(&mut self, workspace: Workspace) -> &mut String {
        self.filters.entry(workspace).or_default()
    }
}
