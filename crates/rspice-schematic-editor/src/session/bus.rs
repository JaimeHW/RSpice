//! Uncommitted bus routing and tap placement state.

use super::placement_authority::PlacementAuthority;
use super::wire::WireRoutingMode;
use crate::requests::EditorRequestSource;
use rspice_design::schematic::bus::{BusDeclaration, BusParseError, BusSlice, BusTapOrientation};
use rspice_design_model::Point;

/// Validated configuration retained while the bus-tap placement tool is armed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingBusTap {
    pub bus_declaration: BusDeclaration,
    pub slice: BusSlice,
    pub orientation: BusTapOrientation,
}

impl PendingBusTap {
    pub fn new(
        bus_declaration: BusDeclaration,
        slice: BusSlice,
        orientation: BusTapOrientation,
    ) -> Result<Self, BusParseError> {
        bus_declaration.validate_slice(&slice)?;
        Ok(Self {
            bus_declaration,
            slice,
            orientation,
        })
    }

    /// The tap the next click places once this one is down: a single bit
    /// steps to the bus's next member in its declared order (`DATA[7]`, then
    /// `DATA[6]` on `DATA[7:0]`), as Virtuoso's expanded wire names drop one
    /// member per click. A range taps the same bits again, and the last bit
    /// has no next one.
    pub fn following(&self) -> Option<Self> {
        if !self.slice.is_scalar() {
            return None;
        }
        let (msb, lsb, bit) = (
            self.bus_declaration.msb,
            self.bus_declaration.lsb,
            self.slice.msb,
        );
        let next = if msb >= lsb {
            (bit > lsb).then(|| bit - 1)
        } else {
            (bit < lsb).then(|| bit + 1)
        }?;
        let slice = BusSlice::new(self.slice.name.clone(), next, next, self.slice.notation).ok()?;
        Some(Self {
            slice,
            ..self.clone()
        })
    }
}

/// A tap configuration and the document context that owns its placement batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingBusTapPlacement {
    pub configuration: PendingBusTap,
    pub authority: PlacementAuthority,
}

impl PendingBusTapPlacement {
    pub fn new(configuration: PendingBusTap, source: EditorRequestSource) -> Self {
        Self {
            configuration,
            authority: PlacementAuthority::new(source),
        }
    }
}

/// Runtime state machine for interactive bus routing.
#[derive(Debug, Clone, Default)]
pub struct BusDrawing {
    pub points: Vec<Point>,
    pub active: bool,
    pub preview_pos: Option<Point>,
    pub routing_mode: WireRoutingMode,
    pub declaration: Option<BusDeclaration>,
    /// What the Bus strip's Name field holds. Every bus started while it
    /// holds a declaration takes it, and it outlasts the route, so a run of
    /// strokes draws one named bus after another.
    pub name: String,
    /// How many of `points` stood after each vertex the user placed, so
    /// taking one back also takes the corner routed to reach it.
    placed: Vec<usize>,
}

impl BusDrawing {
    pub fn start(&mut self, position: Point, declaration: Option<BusDeclaration>) {
        self.points.clear();
        self.points.push(position);
        self.placed.clear();
        self.placed.push(self.points.len());
        self.active = true;
        self.preview_pos = Some(position);
        self.declaration = declaration;
    }

    /// Take back the last vertex the user placed, with any corner routed to
    /// reach it, as a wire route does. Taking back the first point ends the
    /// route.
    pub fn remove_last_point(&mut self) {
        if !self.active {
            return;
        }
        self.placed.pop();
        let keep = match self.placed.last() {
            Some(&placed) if placed < self.points.len() => placed,
            _ => self.points.len().saturating_sub(1),
        };
        self.points.truncate(keep);
        if self.points.is_empty() {
            self.cancel();
        }
    }

    /// The declaration the Name field holds: `None` while it is empty, and
    /// why it is refused when it does not parse.
    pub fn named(&self) -> Result<Option<BusDeclaration>, BusParseError> {
        let name = self.name.trim();
        if name.is_empty() {
            return Ok(None);
        }
        BusDeclaration::parse(name).map(Some)
    }

    pub fn update_preview(&mut self, position: Point) {
        if self.active {
            self.preview_pos = Some(position);
        }
    }

    pub fn add_point(&mut self, position: Point) {
        let Some(start) = self.points.last().copied() else {
            return;
        };
        if !self.active || start == position {
            return;
        }
        for point in self.routing_mode.suggest_route(start, position) {
            if self.points.last() != Some(&point) {
                self.points.push(point);
            }
        }
        self.placed.push(self.points.len());
        self.preview_pos = Some(position);
    }

    pub fn preview_path(&self) -> Vec<Point> {
        let (Some(start), Some(end)) = (self.points.last().copied(), self.preview_pos) else {
            return Vec::new();
        };
        let mut path = vec![start];
        if start != end {
            path.extend(self.routing_mode.suggest_route(start, end));
        }
        path
    }

    pub fn cancel(&mut self) {
        self.points.clear();
        self.placed.clear();
        self.active = false;
        self.preview_pos = None;
        self.declaration = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tap(bus: &str, bits: &str) -> PendingBusTap {
        let bus = BusDeclaration::parse(bus).unwrap();
        let slice = bus.slice(bits).unwrap();
        PendingBusTap::new(bus, slice, BusTapOrientation::Automatic).unwrap()
    }

    /// One bit steps to the next in the bus's own order, both ways round;
    /// the last bit and a range have no next tap.
    #[test]
    fn a_single_bit_tap_steps_to_the_next_member_in_declared_order() {
        let next = |bus: &str, bits: &str| {
            tap(bus, bits)
                .following()
                .map(|following| following.slice.to_string())
        };
        assert_eq!(next("DATA[7:0]", "7").as_deref(), Some("DATA[6]"));
        assert_eq!(next("DATA[7:0]", "1").as_deref(), Some("DATA[0]"));
        assert_eq!(next("DATA[7:0]", "0"), None);
        assert_eq!(next("ADDR<0:3>", "2").as_deref(), Some("ADDR<3>"));
        assert_eq!(next("ADDR<0:3>", "3"), None);
        assert_eq!(next("DATA[7:0]", "7:4"), None);
    }

    /// Backspace takes back the last bus point the user placed, with the
    /// corner routed to reach it, as it does on a wire; the first point
    /// going ends the route.
    #[test]
    fn taking_back_a_bus_point_takes_its_corner_with_it() {
        let mut drawing = BusDrawing::default();
        drawing.start(Point::new(0, 0), None);
        drawing.add_point(Point::new(100, 100));
        drawing.add_point(Point::new(200, 100));
        assert_eq!(
            drawing.points,
            vec![
                Point::new(0, 0),
                Point::new(100, 0),
                Point::new(100, 100),
                Point::new(200, 100)
            ]
        );

        drawing.remove_last_point();
        assert_eq!(
            drawing.points,
            vec![Point::new(0, 0), Point::new(100, 0), Point::new(100, 100)]
        );
        drawing.remove_last_point();
        assert_eq!(drawing.points, vec![Point::new(0, 0)]);
        assert!(drawing.active);
        drawing.remove_last_point();
        assert!(!drawing.active);
        assert!(drawing.points.is_empty());
    }
}
