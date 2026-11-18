//! The Netlist workspace's documents as tabs, and what Run runs there.
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
