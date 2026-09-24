//! Direction-aware generated symbols for hierarchical cell instances.
//!
//! The schematic-from-ports analog of Virtuoso's "Create Cellview From
//! Cellview": a cell's interface ports produce a block symbol whose pins
//! land by electrical role — inputs on the left edge, outputs and
//! bidirectional pins on the right, supply rails on top and bottom. Every
//! pin sits on the routing grid, and the pin sequence stays in interface
//! order so terminal index N is always `.SUBCKT` port N.
//!
//! The block is sized to its own contents: the body is wide enough to print
//! every pin name inside it without the two name columns meeting, and tall
//! enough to keep the busier edge on the pin pitch. Everything a renderer
//! needs to draw one — body extents, which edge a pin belongs to, where its
//! lead ends, where its name goes — is published here, so the canvas, the
//! SVG export, and the printed sheet cannot drift apart.

use crate::state::SymbolPinSide;

use super::point::Point;
use super::port::{PortDirection, PortSpec};

/// Minimum generated block width in grid units (terminals at ±width/2).
pub const GENERATED_WIDTH: i32 = 60;
/// Lead length between a terminal and the body outline.
pub const GENERATED_STUB: i32 = 10;
/// Pin-name type size in symbol units; renderers scale it to the viewport.
pub const GENERATED_PIN_LABEL_SIZE: f32 = 5.0;
/// Clear space between the body outline and a pin name.
pub const GENERATED_LABEL_GUTTER: i32 = 3;
/// Monospace advance per character at [`GENERATED_PIN_LABEL_SIZE`]. IBM Plex
/// Mono advances 600/1000 em, so a 5-unit glyph is exactly 3 units wide.
const LABEL_ADVANCE: i32 = 3;
/// Clear space kept between the left and right pin-name columns.
const LABEL_COLUMN_GAP: i32 = 4;
/// Longest pin name printed in full. Imported interfaces may carry names far
/// past anything readable; the block stops growing for them and the renderer
/// elides instead, so one pathological port cannot produce a symbol nobody
/// can place.
pub const GENERATED_PIN_LABEL_MAX_CHARS: usize = 24;
/// Vertical pitch between side pins, one routing-grid pair.
const PIN_PITCH: i32 = 20;
/// Horizontal pitch between multiple supply pins on one edge.
const RAIL_PITCH: i32 = 20;

/// One pin of a generated symbol, in interface order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedPin {
    /// Port name (drawn inside the body, beside its lead).
    pub name: String,
    /// Declared direction (decides the edge).
    pub direction: PortDirection,
    /// Body edge this pin belongs to.
    pub side: SymbolPinSide,
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

    /// Half-width of the drawn body: one stub inside the side terminals.
    pub fn body_half_width(&self) -> i32 {
        body_half_width(self.width)
    }

    /// Half-height of the drawn body: the rail terminals sit on this edge.
    pub fn body_half_height(&self) -> i32 {
        body_half_height(self.height)
    }

    /// Corner-to-corner extents of the drawn body.
    pub fn body_bounds(&self) -> (Point, Point) {
        let (half_width, half_height) = (self.body_half_width(), self.body_half_height());
        (
            Point::new(-half_width, -half_height),
            Point::new(half_width, half_height),
        )
    }
}

/// Half-width of the body drawn inside a block of the given full width.
pub fn body_half_width(width: i32) -> i32 {
    (width / 2 - GENERATED_STUB).max(GENERATED_STUB)
}

/// Half-height of the body drawn inside a block of the given full height.
pub fn body_half_height(height: i32) -> i32 {
    (height / 2).max(GENERATED_STUB * 2)
}

/// Unit step from a terminal toward the body interior.
///
/// Renderers transform this through the instance orientation to decide how
/// a pin name is aligned, so a rotated block still reads outward-in.
pub fn inward_step(side: SymbolPinSide) -> Point {
    match side {
        SymbolPinSide::Left => Point::new(1, 0),
        SymbolPinSide::Right => Point::new(-1, 0),
        SymbolPinSide::Top => Point::new(0, 1),
        SymbolPinSide::Bottom => Point::new(0, -1),
    }
}

/// Inner end of a pin's lead: the point on the body outline that the pin's
/// own edge puts it against.
///
/// The edge decides the axis — never the terminal's own coordinates, which
/// say the wrong thing the moment a body is taller than it is wide, turning
/// an outer side pin's lead through ninety degrees so it never arrives. The
/// body decides the length, so a lead always lands on the outline instead of
/// stopping a fixed distance short of it. Without a body to reach, the lead
/// falls back to one stub length.
pub fn lead_inner(terminal: Point, side: SymbolPinSide, body: Option<(Point, Point)>) -> Point {
    let step = inward_step(side);
    let Some((min, max)) = body else {
        return Point::new(
            terminal.x + step.x * GENERATED_STUB,
            terminal.y + step.y * GENERATED_STUB,
        );
    };
    match side {
        SymbolPinSide::Left => Point::new(min.x.max(terminal.x), terminal.y),
        SymbolPinSide::Right => Point::new(max.x.min(terminal.x), terminal.y),
        SymbolPinSide::Top => Point::new(terminal.x, min.y.max(terminal.y)),
        SymbolPinSide::Bottom => Point::new(terminal.x, max.y.min(terminal.y)),
    }
}

/// Where a pin's name is drawn: just inside the body outline, on the pin's
/// own lead axis. The renderer aligns the text away from this point, so the
/// name reads into the body and never crosses the outline it belongs to.
pub fn pin_label_anchor(
    terminal: Point,
    side: SymbolPinSide,
    body: Option<(Point, Point)>,
) -> Point {
    let inner = lead_inner(terminal, side, body);
    let step = inward_step(side);
    Point::new(
        inner.x + step.x * GENERATED_LABEL_GUTTER,
        inner.y + step.y * GENERATED_LABEL_GUTTER,
    )
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

/// The pin name as it is printed: the name itself, or an elided form that
/// fits the width the block reserved for it.
pub fn fit_pin_name(name: &str) -> String {
    if name.chars().count() <= GENERATED_PIN_LABEL_MAX_CHARS {
        return name.to_owned();
    }
    let mut fitted: String = name
        .chars()
        .take(GENERATED_PIN_LABEL_MAX_CHARS - 1)
        .collect();
    fitted.push('…');
    fitted
}

/// Width of a printed pin name in symbol units.
fn name_span(name: &str) -> i32 {
    let chars = name.chars().count().min(GENERATED_PIN_LABEL_MAX_CHARS);
    i32::try_from(chars).unwrap_or(i32::MAX) * LABEL_ADVANCE
}

/// Widest printed name among a set of interface ports.
fn column_span(ports: &[PortSpec], slots: &[usize]) -> i32 {
    slots
        .iter()
        .filter_map(|&index| ports.get(index))
        .map(|port| name_span(&port.name))
        .max()
        .unwrap_or(0)
}

/// Block width that prints every pin name inside the body, with both name
/// columns clear of the outline and of each other, rounded so the terminals
/// stay on the routing grid.
fn fitted_width(ports: &[PortSpec], left: &[usize], right: &[usize], rail_reach: i32) -> i32 {
    let left_span = column_span(ports, left);
    let right_span = column_span(ports, right);
    let gap = if left_span > 0 && right_span > 0 {
        LABEL_COLUMN_GAP
    } else {
        0
    };
    let names = 2 * GENERATED_LABEL_GUTTER + left_span + right_span + gap;
    // A supply rail must land on the body edge it is drawn against, with
    // room on both sides of its own lead.
    let rails = 2 * (rail_reach + RAIL_PITCH / 2);
    let width = names
        .max(rails)
        .saturating_add(2 * GENERATED_STUB)
        .max(GENERATED_WIDTH);
    // Round up so half the width — where the terminals land — stays on the
    // routing grid.
    let step = 2 * GENERATED_STUB;
    width.saturating_add(step - 1) / step * step
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
    let rail_reach = (0..rails.len())
        .map(|slot| rail_x(slot / 2).abs())
        .max()
        .unwrap_or(0);
    let width = fitted_width(ports, &left, &right, rail_reach);
    let hw = width / 2;
    let hh = body_half_height(height);

    let mut placement: Vec<(SymbolPinSide, Point)> =
        vec![(SymbolPinSide::Left, Point::new(0, 0)); ports.len()];
    for (slot, &index) in left.iter().enumerate() {
        placement[index] = (
            SymbolPinSide::Left,
            Point::new(-hw, side_axis(left.len())[slot]),
        );
    }
    for (slot, &index) in right.iter().enumerate() {
        placement[index] = (
            SymbolPinSide::Right,
            Point::new(hw, side_axis(right.len())[slot]),
        );
    }
    // Rails alternate top, bottom, fanning out from the center pairwise so
    // vdd/vss stay vertically aligned.
    for (slot, &index) in rails.iter().enumerate() {
        let (side, edge) = if slot % 2 == 0 {
            (SymbolPinSide::Top, -hh)
        } else {
            (SymbolPinSide::Bottom, hh)
        };
        placement[index] = (side, Point::new(rail_x(slot / 2), edge));
    }

    GeneratedSymbol {
        pins: ports
            .iter()
            .zip(placement)
            .map(|(port, (side, offset))| GeneratedPin {
                name: port.name.clone(),
                direction: port.direction,
                side,
                offset,
            })
            .collect(),
        width,
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
        let hw = symbol.width / 2;
        let hh = symbol.body_half_height();
        assert_eq!(symbol.pins[0].offset, Point::new(-hw, -10)); // inp
        assert_eq!(symbol.pins[1].offset, Point::new(-hw, 10)); // inn
        assert_eq!(symbol.pins[2].offset, Point::new(hw, 0)); // out
        assert_eq!(symbol.pins[3].offset, Point::new(0, -hh)); // vdd up
        assert_eq!(symbol.pins[4].offset, Point::new(0, hh)); // vss down
        let sides: Vec<SymbolPinSide> = symbol.pins.iter().map(|pin| pin.side).collect();
        assert_eq!(
            sides,
            [
                SymbolPinSide::Left,
                SymbolPinSide::Left,
                SymbolPinSide::Right,
                SymbolPinSide::Top,
                SymbolPinSide::Bottom,
            ]
        );
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
    fn every_rail_lands_on_the_body_edge_it_is_drawn_against() {
        let ports: Vec<PortSpec> = (0..6)
            .map(|index| spec(&format!("v{index}"), PortDirection::Supply))
            .collect();
        let symbol = generate_symbol(&ports);
        let half_width = symbol.body_half_width();
        for pin in &symbol.pins {
            assert!(
                pin.offset.x.abs() <= half_width,
                "rail {pin:?} hangs off a body of half-width {half_width}"
            );
        }
    }

    #[test]
    fn minimum_body_for_tiny_interfaces() {
        let symbol = generate_symbol(&[spec("io", PortDirection::InOut)]);
        assert_eq!(symbol.dimensions(), (GENERATED_WIDTH, 40));
        assert_eq!(symbol.pins[0].offset, Point::new(GENERATED_WIDTH / 2, 0));
    }

    #[test]
    fn short_interfaces_keep_the_nominal_block_width() {
        for ports in [
            vec![
                spec("in", PortDirection::In),
                spec("out", PortDirection::Out),
            ],
            vec![
                spec("in[0]", PortDirection::In),
                spec("in[1]", PortDirection::In),
                spec("out", PortDirection::Out),
            ],
            vec![
                spec("cntl_in", PortDirection::In),
                spec("out", PortDirection::Out),
            ],
        ] {
            assert_eq!(
                generate_symbol(&ports).width,
                GENERATED_WIDTH,
                "{ports:?} must not widen the nominal block"
            );
        }
    }

    #[test]
    fn long_pin_names_are_printed_inside_the_body_without_meeting() {
        let ports = vec![
            spec("data_in[0]", PortDirection::In),
            spec("write_en", PortDirection::In),
            spec("data_out[0]", PortDirection::Out),
        ];
        let symbol = generate_symbol(&ports);
        let half_width = symbol.body_half_width();
        let body = Some(symbol.body_bounds());

        let left = &symbol.pins[0];
        let right = &symbol.pins[2];
        let left_end = pin_label_anchor(left.offset, left.side, body).x + name_span(&left.name);
        let right_end = pin_label_anchor(right.offset, right.side, body).x - name_span(&right.name);

        assert!(
            left_end <= half_width - GENERATED_LABEL_GUTTER,
            "left names must stay inside the body: {left_end} vs {half_width}"
        );
        assert!(
            right_end >= -half_width + GENERATED_LABEL_GUTTER,
            "right names must stay inside the body: {right_end} vs {half_width}"
        );
        assert!(
            left_end < right_end,
            "the two name columns must not meet: {left_end} vs {right_end}"
        );
    }

    #[test]
    fn every_lead_reaches_the_body_from_its_own_edge() {
        let ports = vec![
            spec("data_in[0]", PortDirection::In),
            spec("data_in[1]", PortDirection::In),
            spec("data_in[2]", PortDirection::In),
            spec("data_in[3]", PortDirection::In),
            spec("write_en", PortDirection::In),
            spec("data_out[0]", PortDirection::Out),
            spec("vdd", PortDirection::Supply),
        ];
        let symbol = generate_symbol(&ports);
        let half_width = symbol.body_half_width();
        let half_height = symbol.body_half_height();

        for pin in &symbol.pins {
            let inner = lead_inner(pin.offset, pin.side, Some(symbol.body_bounds()));
            let on_outline = match pin.side {
                SymbolPinSide::Left => inner.x == -half_width,
                SymbolPinSide::Right => inner.x == half_width,
                SymbolPinSide::Top => inner.y == -half_height,
                SymbolPinSide::Bottom => inner.y == half_height,
            };
            assert!(
                on_outline,
                "{pin:?} lead ends at {inner:?}, not on a body ±({half_width}, {half_height})"
            );
        }
    }

    #[test]
    fn a_name_too_long_for_the_nominal_body_widens_the_block_onto_the_grid() {
        let symbol = generate_symbol(&[
            spec("freq_in", PortDirection::In),
            spec("freq_out", PortDirection::Out),
        ]);

        assert!(symbol.width > GENERATED_WIDTH);
        assert_eq!(symbol.width % (2 * GENERATED_STUB), 0);
        assert_eq!(symbol.pins[0].offset, Point::new(-symbol.width / 2, 0));
        assert_eq!(symbol.pins[1].offset, Point::new(symbol.width / 2, 0));
    }

    #[test]
    fn a_pathological_name_stops_the_block_growing_and_is_elided_to_fit() {
        let long = "d".repeat(GENERATED_PIN_LABEL_MAX_CHARS * 8);
        let capped = generate_symbol(&[spec(&long, PortDirection::In)]);
        let at_limit = generate_symbol(&[spec(
            &"d".repeat(GENERATED_PIN_LABEL_MAX_CHARS),
            PortDirection::In,
        )]);

        assert_eq!(capped.width, at_limit.width);
        assert_eq!(
            fit_pin_name(&long).chars().count(),
            GENERATED_PIN_LABEL_MAX_CHARS
        );
        assert!(fit_pin_name(&long).ends_with('…'));
        assert_eq!(fit_pin_name("data_in[0]"), "data_in[0]");
    }
}
