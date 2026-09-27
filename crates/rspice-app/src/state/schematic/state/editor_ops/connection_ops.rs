//! Terminal-to-wire connectivity.
//!
//! Rebuilding the connection set after geometry moves. Connectivity is derived, never stored by hand: any
//! operation that changes where a terminal or a wire end sits rebuilds from
//! the terminals rather than patching individual connections.

use super::super::terminal_connection;
use super::super::*;

impl SchematicState {
    // =========================================================================
    // Wire Connection Management (for rubber-banding)
    // =========================================================================

    /// Rebuild all wire connections based on current positions
    pub fn rebuild_connections(&mut self) {
        terminal_connection::rebuild_connections(&mut self.document);
    }

    /// Rebuild the rubber-band cache from authoritative resolved terminal
    /// geometry. Authored library symbols must use this path because their pin
    /// positions can differ from intrinsic fallback geometry.
    pub fn rebuild_connections_from_terminals(&mut self, terminals: &[(u64, String, Point)]) {
        terminal_connection::rebuild_connections_from_terminals(&mut self.document, terminals);
    }
}
