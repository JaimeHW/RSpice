//! Which conductors the canvas is lighting, and nothing about why.
//!
//! This module holds a selection, not a theory of connectivity. It records the
//! wire IDs a highlight covers and the exact net name that authorized it; it
//! never decides which conductors belong together. That question has one owner
//! — `simulation::netlist_gen::extraction` — and the canvas, the netlister, the
//! electrical rule check and the navigator all read that one answer. A second
//! graph built here would light a different net than the deck solves, which is
//! the failure this module is deliberately incapable of.

use std::collections::HashSet;

/// State for tracking highlighted net
#[derive(Debug, Clone, Default)]
pub struct NetHighlightState {
    /// IDs of wires in the currently highlighted net
    pub highlighted_wires: HashSet<u64>,
    /// Exact live semantic net selected by a Navigator/Inspector net row.
    /// This remains meaningful for a legal net with no drawn wires.
    pub selected_net_name: Option<String>,
    /// Whether highlighting is active
    pub active: bool,
}

impl NetHighlightState {
    /// Create a new empty state
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear the highlight
    pub fn clear(&mut self) {
        self.highlighted_wires.clear();
        self.selected_net_name = None;
        self.active = false;
    }

    /// Check if a wire is highlighted
    pub fn is_wire_highlighted(&self, wire_id: u64) -> bool {
        self.active && self.highlighted_wires.contains(&wire_id)
    }

    /// Highlight a set of wires directly
    pub fn highlight_wires(&mut self, wire_ids: HashSet<u64>) {
        self.highlighted_wires = wire_ids;
        self.selected_net_name = None;
        self.active = !self.highlighted_wires.is_empty();
    }

    /// Highlight one exact live net, including semantic nets that currently
    /// have no drawn conductor geometry.
    pub fn highlight_named_wires(&mut self, name: impl Into<String>, wire_ids: HashSet<u64>) {
        self.highlighted_wires = wire_ids;
        self.selected_net_name = Some(name.into());
        self.active = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(values: &[u64]) -> HashSet<u64> {
        values.iter().copied().collect()
    }

    #[test]
    fn named_wireless_net_remains_exact_until_any_ordinary_highlight_change() {
        let mut highlight = NetHighlightState::default();
        highlight.highlight_named_wires("PORT_OUT", HashSet::new());
        assert!(highlight.active);
        assert_eq!(highlight.selected_net_name.as_deref(), Some("PORT_OUT"));

        highlight.highlight_wires(ids(&[7]));
        assert!(highlight.selected_net_name.is_none());
        highlight.clear();
        assert!(!highlight.active);
        assert!(highlight.selected_net_name.is_none());
    }
}
