//! Which conductors the canvas is lighting, and nothing about why.
//!
//! This module holds a selection, not a theory of connectivity. It records the
//! wire IDs a highlight covers and the exact net name that authorized it; it
//! never decides which conductors belong together. That question has one owner
//! — `rspice_design::connectivity` — and the canvas, the netlister, the
//! electrical rule check and the navigator all read that one answer. A second
//! graph built here would light a different net than the deck solves, which is
//! the failure this module is deliberately incapable of.
//!
//! A named highlight is held as that net, not as the wires it covered when it
//! was lit: after an edit [`NetHighlightState::follow`] re-reads the net from
//! connectivity's answer, so a wire drawn onto it lights and a cut leaves only
//! the part that is still that net lit.

use rspice_design::connectivity::summary::DesignNet;
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
    /// The document version the lit wires were last read against, as the
    /// caller counts it; `None` until [`Self::follow`] has read them.
    pub followed_version: Option<(u64, u64)>,
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
        self.followed_version = None;
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
        self.followed_version = None;
    }

    /// Highlight one exact live net, including semantic nets that currently
    /// have no drawn conductor geometry.
    pub fn highlight_named_wires(&mut self, name: impl Into<String>, wire_ids: HashSet<u64>) {
        self.highlighted_wires = wire_ids;
        self.selected_net_name = Some(name.into());
        self.active = true;
        self.followed_version = None;
    }

    /// Whether the lit wires were last read against `version` of the
    /// document, so there is nothing for [`Self::follow`] to do.
    pub fn is_followed_at(&self, version: (u64, u64)) -> bool {
        !self.active || self.selected_net_name.is_none() || self.followed_version == Some(version)
    }

    /// Re-read the named net from `nets`, the document's nets at `version`.
    ///
    /// The net keeps its identity through the edit: a net named by a label, a
    /// port or a ground keeps that name; an automatically named net, whose
    /// name the netlister may hand to another net after an edit, is the net
    /// that kept most of the wires that were lit; a net with no wire is known
    /// by its name alone. A net the edit removed is no longer lit. A highlight
    /// of loose wires names no net and is left as it is.
    pub fn follow(&mut self, nets: &[DesignNet], version: (u64, u64)) {
        self.followed_version = Some(version);
        let Some(name) = self.selected_net_name.as_deref() else {
            return;
        };
        let named = nets.iter().find(|net| net.name.eq_ignore_ascii_case(name));
        let net = match named {
            Some(net) if net.authored_name => Some(net),
            _ if self.highlighted_wires.is_empty() => named,
            _ => nets
                .iter()
                .map(|net| {
                    let kept = net
                        .wire_ids
                        .iter()
                        .filter(|id| self.highlighted_wires.contains(id))
                        .count();
                    (net, kept)
                })
                .filter(|(_, kept)| *kept > 0)
                .max_by_key(|(_, kept)| *kept)
                .map(|(net, _)| net),
        };
        match net {
            Some(net) => {
                if !net.name.eq_ignore_ascii_case(name) {
                    self.selected_net_name = Some(net.name.clone());
                }
                if self.highlighted_wires.len() != net.wire_ids.len()
                    || net
                        .wire_ids
                        .iter()
                        .any(|id| !self.highlighted_wires.contains(id))
                {
                    self.highlighted_wires = net.wire_ids.iter().copied().collect();
                }
            }
            None => self.clear(),
        }
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
