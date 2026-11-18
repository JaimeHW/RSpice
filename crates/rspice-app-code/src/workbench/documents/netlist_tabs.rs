//! The Netlist workspace's documents as tabs, and what Run runs there.
//!
//! The page shows one document at a time, the tab in front: the generated
//! netlist, the plan's Added lines, a deck, an include open beside its deck
//! or beside the generated netlist, the input one run executed, or a
//! comparison of two of these. [`NetlistTab`] says which, and is the only
//! record of it.
//!
//! Run (F5, the toolbar's Run, the palette) on the Netlist workspace runs one
//! target: the plan, as Simulate runs it, or one deck as written. Bringing a
//! deck's tab to the front makes that deck the target; bringing the generated
//! netlist or the Added lines makes the plan the target. An include, a run's
//! input and a comparison leave the target as it was, and closing the target
//! deck's tab makes the plan the target again. The toolbar names the target,
//! so a change is never silent.

use crate::state::DeckId;

/// What Run runs on the Netlist workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RunTarget {
    /// The active simulation plan, exactly as Simulate's Run runs it.
    #[default]
    Plan,
    /// The deck as written.
    Deck(DeckId),
}

/// The document in front of the Netlist page.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum NetlistTab {
    /// The deck the active plan's root design netlists to.
    #[default]
    Generated,
    /// The lines the plan's root design adds to its generated netlist.
    AddedLines,
    /// A deck, as written.
    Deck(DeckId),
    /// A file a deck reads (`deck` names it), or the generated netlist reads
    /// (`deck` is `None`), by the path or name its root reads it by.
    Include { deck: Option<DeckId>, path: String },
    /// The text one run executed: for `point` of a sweep, or the deck a
    /// manual run read when `point` is `None`.
    RunInput { run: u64, point: Option<usize> },
    /// Two documents side by side, as a unified diff of `left` to `right`.
    Compare {
        left: CompareSide,
        right: CompareSide,
    },
}

/// One side of a comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CompareSide {
    /// The generated netlist as it is now.
    Generated,
    /// An earlier build of the generated netlist, by its place in the
    /// builds kept this session, newest first.
    PreviousGenerated(usize),
    /// A deck as the editor holds it.
    Deck(DeckId),
    /// A deck's file as it is on disk.
    DiskFile(DeckId),
    /// The input a run executed.
    RunInput(u64),
}

impl NetlistTab {
    /// Whether the generated netlist, or a file it reads, is in front.
    #[must_use]
    pub fn shows_generated(&self) -> bool {
        matches!(self, Self::Generated | Self::Include { deck: None, .. })
    }

    /// Whether a deck, or a file it reads, is in front.
    #[must_use]
    pub fn shows_deck(&self) -> bool {
        matches!(self, Self::Deck(_) | Self::Include { deck: Some(_), .. })
    }

    /// Whether a comparison is in front.
    #[must_use]
    pub fn is_comparison(&self) -> bool {
        matches!(self, Self::Compare { .. })
    }

    /// Whether a run's input is in front.
    #[must_use]
    pub fn is_run_input(&self) -> bool {
        matches!(self, Self::RunInput { .. })
    }

    /// The include in front, by the path or name its root reads it by.
    #[must_use]
    pub fn include(&self) -> Option<&str> {
        match self {
            Self::Include { path, .. } => Some(path),
            _ => None,
        }
    }

    /// The tab an include belongs to, and any other tab itself.
    #[must_use]
    pub fn root(&self) -> Self {
        match self {
            Self::Include {
                deck: Some(deck), ..
            } => Self::Deck(*deck),
            Self::Include { deck: None, .. } => Self::Generated,
            other => other.clone(),
        }
    }

    /// An include of the document this tab's root is, by `path`; `None` for
    /// a root no include belongs to.
    #[must_use]
    pub fn include_of(&self, path: &str) -> Option<Self> {
        match self.root() {
            Self::Generated => Some(Self::Include {
                deck: None,
                path: path.to_owned(),
            }),
            Self::Deck(deck) => Some(Self::Include {
                deck: Some(deck),
                path: path.to_owned(),
            }),
            _ => None,
        }
    }
}

/// What kind of document a tab is, an include counting as its root: the key
/// the editor files its per-document state and indexes under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NetlistRoot {
    #[default]
    Generated,
    AddedLines,
    Deck,
    Comparison,
    RunInput,
}

impl NetlistTab {
    /// What kind of document this tab is, an include counting as its root.
    #[must_use]
    pub fn root_kind(&self) -> NetlistRoot {
        match self {
            Self::Generated | Self::Include { deck: None, .. } => NetlistRoot::Generated,
            Self::AddedLines => NetlistRoot::AddedLines,
            Self::Deck(_) | Self::Include { deck: Some(_), .. } => NetlistRoot::Deck,
            Self::RunInput { .. } => NetlistRoot::RunInput,
            Self::Compare { .. } => NetlistRoot::Comparison,
        }
    }
}
