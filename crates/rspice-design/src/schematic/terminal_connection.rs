//! Rebuild terminal-to-wire connections from intrinsic or resolved geometry.

use super::{document::SchematicDocument, wire::WireConnection};
use rspice_design_model::Point;

/// Snap distance in grid units for terminal connections.
const SNAP_DISTANCE: i32 = 1;

/// Rebuild all wire connections based on current positions
pub fn rebuild_connections(document: &mut SchematicDocument) {
    let terminals = document
        .components
        .iter()
        .flat_map(|component| {
            component
                .terminal_positions()
                .into_iter()
                .map(move |(name, point)| (component.id, name.to_owned(), point))
        })
        .collect::<Vec<_>>();
    rebuild_connections_from_terminals(document, &terminals);
}

/// Rebuild the rubber-band cache from authoritative resolved terminal
/// geometry. Authored library symbols must use this path because their pin
/// positions can differ from intrinsic fallback geometry.
pub fn rebuild_connections_from_terminals(
    document: &mut SchematicDocument,
    terminals: &[(u64, String, Point)],
) {
    document.connections.clear();

    let wire_endpoints: Vec<(u64, Point, usize)> = document
        .wires
        .iter()
        .filter(|w| !w.points.is_empty())
        .flat_map(|w| {
            let mut endpoints = vec![(w.id, w.points[0], 0usize)];
            let end_idx = w.points.len() - 1;
            if end_idx > 0 {
                endpoints.push((w.id, w.points[end_idx], end_idx));
            }
            endpoints
        })
        .collect();

    for (wire_id, pos, point_index) in wire_endpoints {
        if let Some((comp_id, term_name, _)) = terminals.iter().find(|(_, _, terminal)| {
            (pos.x - terminal.x).abs() <= SNAP_DISTANCE
                && (pos.y - terminal.y).abs() <= SNAP_DISTANCE
        }) {
            document.connections.push(WireConnection::new(
                wire_id,
                point_index,
                *comp_id,
                term_name.clone(),
            ));
        }
    }
}
