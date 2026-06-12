//! Direction-aware generated symbols for hierarchical cell instances.
//!
//! The schematic-from-ports analog of Virtuoso's "Create Cellview From
//! Cellview": a cell's interface ports produce a block symbol whose pins
//! land by electrical role — inputs on the left edge, outputs and
//! bidirectional pins on the right, supply rails on top and bottom. Every
//! pin sits on the routing grid, and the pin sequence stays in interface
//! order so terminal index N is always `.SUBCKT` port N.

use super::point::Point;
use super::port::{PortDirection, PortSpec};

/// Generated block width in grid units (terminals at ±width/2).
pub const GENERATED_WIDTH: i32 = 60;
/// Vertical pitch between side pins, one routing-grid pair.
const PIN_PITCH: i32 = 20;
/// Horizontal pitch between multiple supply pins on one edge.
const RAIL_PITCH: i32 = 20;

/// One pin of a generated symbol, in interface order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedPin {
    /// Port name (drawn beside the stub).
    pub name: String,
    /// Declared direction (decides the edge).
    pub direction: PortDirection,
    /// Terminal offset from the symbol origin, unrotated grid units.
    pub offset: Point,
}

/// A complete generated symbol: pins plus body extents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedSymbol {
    /// Pins in interface order (terminal index == port index).
    pub pins: Vec<GeneratedPin>,
    /// Full symbol width (terminal to terminal).
    pub width: i32,
    /// Full symbol height (rail terminal to rail terminal).
    pub height: i32,
}

impl GeneratedSymbol {
    /// Symbol extents as `(width, height)` grid units.
    pub fn dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}

/// Evenly spaced side-pin Y coordinates: `n` pins at `PIN_PITCH`, centered
/// on the origin, always on the 10-unit grid.
fn side_axis(n: usize) -> Vec<i32> {
    (0..n as i32)
        .map(|idx| idx * PIN_PITCH - (n as i32 - 1) * (PIN_PITCH / 2))
        .collect()
}

/// X coordinate of the `k`-th supply pin sharing one edge: 0, then fanning
/// out by `RAIL_PITCH` alternating right/left of center.
fn rail_x(k: usize) -> i32 {
    let step = k.div_ceil(2) as i32 * RAIL_PITCH;
    if k % 2 == 1 { step } else { -step }
}

/// Generate the block symbol for an interface.
///
/// Empty interfaces produce an empty pin list with the minimum body —
/// callers treat that as "nothing to place yet".
pub fn generate_symbol(ports: &[PortSpec]) -> GeneratedSymbol {
    // Bucket by edge, preserving interface order within each edge.
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut rails = Vec::new();
    for (index, port) in ports.iter().enumerate() {
        match port.direction {
            PortDirection::In => left.push(index),
            PortDirection::Out | PortDirection::InOut => right.push(index),
            PortDirection::Supply => rails.push(index),
        }
    }

    let rows = left.len().max(right.len()).max(1);
    let height = (rows as i32 * PIN_PITCH).max(40);
    let hw = GENERATED_WIDTH / 2;
    let hh = height / 2;

    let mut offsets: Vec<Point> = vec![Point::new(0, 0); ports.len()];
    for (slot, &index) in left.iter().enumerate() {
        offsets[index] = Point::new(-hw, side_axis(left.len())[slot]);
    }
    for (slot, &index) in right.iter().enumerate() {
        offsets[index] = Point::new(hw, side_axis(right.len())[slot]);
    }
    // Rails alternate top, bottom, fanning out from the center pairwise so
    // vdd/vss stay vertically aligned.
    for (slot, &index) in rails.iter().enumerate() {
        let edge = if slot % 2 == 0 { -hh } else { hh };
        offsets[index] = Point::new(rail_x(slot / 2), edge);
    }

    GeneratedSymbol {
        pins: ports
            .iter()
            .zip(offsets)
            .map(|(port, offset)| GeneratedPin {
                name: port.name.clone(),
                direction: port.direction,
                offset,
            })
            .collect(),
        width: GENERATED_WIDTH,
        height,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_string(),
            direction,
        }
    }

    /// The 5T OTA interface — the canonical shape this generator serves.
    fn ota_ports() -> Vec<PortSpec> {
        vec![
            spec("inp", PortDirection::In),
            spec("inn", PortDirection::In),
            spec("out", PortDirection::Out),
            spec("vdd", PortDirection::Supply),
            spec("vss", PortDirection::Supply),
        ]
    }

    #[test]
    fn pins_stay_in_interface_order() {
        let symbol = generate_symbol(&ota_ports());
        let names: Vec<&str> = symbol.pins.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["inp", "inn", "out", "vdd", "vss"]);
    }

    #[test]
    fn directions_pick_the_edges() {
        let symbol = generate_symbol(&ota_ports());
        let hw = GENERATED_WIDTH / 2;
        let hh = symbol.height / 2;
        assert_eq!(symbol.pins[0].offset, Point::new(-hw, -10)); // inp
        assert_eq!(symbol.pins[1].offset, Point::new(-hw, 10)); // inn
        assert_eq!(symbol.pins[2].offset, Point::new(hw, 0)); // out
        assert_eq!(symbol.pins[3].offset, Point::new(0, -hh)); // vdd up
        assert_eq!(symbol.pins[4].offset, Point::new(0, hh)); // vss down
    }

    #[test]
    fn every_pin_lands_on_the_grid() {
        let mut ports = ota_ports();
        for i in 0..5 {
            ports.push(spec(&format!("d{i}"), PortDirection::In));
            ports.push(spec(&format!("q{i}"), PortDirection::Out));
        }
        ports.push(spec("vddio", PortDirection::Supply));
        let symbol = generate_symbol(&ports);
        for pin in &symbol.pins {
            assert_eq!(pin.offset.x % 10, 0, "{pin:?}");
            assert_eq!(pin.offset.y % 10, 0, "{pin:?}");
        }
    }

    #[test]
    fn body_grows_with_the_busier_edge() {
        let mut ports = vec![spec("out", PortDirection::Out)];
        for i in 0..6 {
            ports.push(spec(&format!("in{i}"), PortDirection::In));
        }
        let symbol = generate_symbol(&ports);
        assert_eq!(symbol.height, 120); // six rows of PIN_PITCH
        // All six inputs distinct and within the body.
        let ys: std::collections::HashSet<i32> = symbol
            .pins
            .iter()
            .filter(|p| p.direction == PortDirection::In)
            .map(|p| p.offset.y)
            .collect();
        assert_eq!(ys.len(), 6);
        assert!(ys.iter().all(|y| y.abs() <= symbol.height / 2 - 10));
    }

    #[test]
    fn extra_rails_fan_out_without_collisions() {
        let ports = vec![
            spec("vdd", PortDirection::Supply),
            spec("vss", PortDirection::Supply),
            spec("vdda", PortDirection::Supply),
            spec("vssa", PortDirection::Supply),
        ];
        let symbol = generate_symbol(&ports);
        let unique: std::collections::HashSet<(i32, i32)> = symbol
            .pins
            .iter()
            .map(|p| (p.offset.x, p.offset.y))
            .collect();
        assert_eq!(unique.len(), 4, "{:?}", symbol.pins);
    }

    #[test]
    fn minimum_body_for_tiny_interfaces() {
        let symbol = generate_symbol(&[spec("io", PortDirection::InOut)]);
        assert_eq!(symbol.dimensions(), (GENERATED_WIDTH, 40));
        assert_eq!(symbol.pins[0].offset, Point::new(GENERATED_WIDTH / 2, 0));
    }
}
