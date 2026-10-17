//! Rebuild terminal-to-wire connections from intrinsic or resolved geometry.

use super::{document::SchematicDocument, wire::WireConnection};
use rspice_design_model::Point;

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

    // Exactly on the pin, as the extractor joins them: a wire end one unit
    // away leaves the pin open, so it is no connection to carry either.
    for (wire_id, pos, point_index) in wire_endpoints {
        if let Some((comp_id, term_name, _)) =
            terminals.iter().find(|(_, _, terminal)| *terminal == pos)
        {
            document.connections.push(WireConnection::new(
                wire_id,
                point_index,
                *comp_id,
                term_name.clone(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::wire::Wire;

    /// A wire end one unit off a pin is not on it: the extractor leaves the
    /// pin open, so the record that rubber-bands it must not claim it either.
    #[test]
    fn only_a_wire_end_exactly_on_a_pin_is_recorded_as_connected() {
        let pin = Point::new(40, 0);
        let mut document = SchematicDocument {
            wires: vec![
                Wire::segment(1, Point::new(41, 0), Point::new(100, 0)),
                Wire::segment(2, Point::new(40, 1), Point::new(40, 100)),
                Wire::segment(3, pin, Point::new(-100, 0)),
            ],
            ..SchematicDocument::default()
        };

        rebuild_connections_from_terminals(&mut document, &[(7, "1".to_owned(), pin)]);

        assert_eq!(
            document.connections,
            vec![WireConnection::new(3, 0, 7, "1".to_owned())]
        );
    }

    /// Wire ends at the far corners of the coordinate range are compared,
    /// not subtracted.
    #[test]
    fn extreme_coordinates_compare_without_overflow() {
        let mut document = SchematicDocument {
            wires: vec![Wire::segment(
                1,
                Point::new(i32::MIN, i32::MIN),
                Point::new(i32::MAX, i32::MIN),
            )],
            ..SchematicDocument::default()
        };

        rebuild_connections_from_terminals(
            &mut document,
            &[(7, "1".to_owned(), Point::new(i32::MAX, i32::MAX))],
        );

        assert!(document.connections.is_empty());
    }
}
