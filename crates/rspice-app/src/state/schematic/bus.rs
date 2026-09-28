//! Bus routing gestures and schematic integration.

pub use rspice_design::schematic::bus::{
    Bus, BusDeclaration, BusDirection, BusNotation, BusParseError, BusPropertyImpact, BusSlice,
    BusTap, BusTapOrientation, BusTargetKind, declared_vector, declared_width,
    nearest_lattice_point_on_segment,
};

use super::{Point, WireRoutingMode};

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
}

/// Runtime state machine for interactive bus routing.
#[derive(Debug, Clone, Default)]
pub struct BusDrawing {
    pub points: Vec<Point>,
    pub active: bool,
    pub preview_pos: Option<Point>,
    pub routing_mode: WireRoutingMode,
    pub declaration: Option<BusDeclaration>,
}

impl BusDrawing {
    pub fn start(&mut self, position: Point, declaration: Option<BusDeclaration>) {
        self.points.clear();
        self.points.push(position);
        self.active = true;
        self.preview_pos = Some(position);
        self.declaration = declaration;
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
        self.active = false;
        self.preview_pos = None;
        self.declaration = None;
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ComponentType, SchematicState};
    use super::*;
    use rspice_design::schematic::bus::MAX_BUS_MEMBER_INDEX;
    use rspice_design::schematic::bus::{VectorConnectivity, vector_connectivity};

    /// The member budget is exact at both ends, and the refusal says which
    /// number it read and which one it holds authors to.
    ///
    /// A budget stated only in a constant is a budget nobody can see: the
    /// message this asserts is the whole of what an author is told when a
    /// declaration is refused, and "out of range" without either number left
    /// them to guess the range. The boundary pair is here because an
    /// off-by-one in the comparison is invisible to a test that only refuses
    /// something far above it.
    #[test]
    fn the_member_budget_is_exact_and_the_refusal_names_both_numbers() {
        assert_eq!(MAX_BUS_MEMBER_INDEX, 4_095, "a 4,096-member budget");
        // The drawn bus and the retained one are the same budget. A schematic
        // that could declare a wider bus than a result can carry would draw
        // something no run could ever report, and the two numbers are stated
        // in different crates, so nothing but this holds them together.
        assert_eq!(
            u64::from(MAX_BUS_MEMBER_INDEX) + 1,
            u64::from(rspice_core::engine::MAX_DIGITAL_BUS_WIDTH),
            "a drawn bus and a retained bus admit the same number of members"
        );

        let widest = BusDeclaration::parse("DATA[4095:0]").expect("the whole budget is declarable");
        assert_eq!(widest.width(), 4_096);

        assert_eq!(
            BusDeclaration::parse("DATA[4096:0]"),
            Err(BusParseError::IndexOutOfRange("4096".to_owned()))
        );
        assert_eq!(
            BusParseError::IndexOutOfRange("4096".to_owned()).to_string(),
            "bus member index 4096 is out of range: the highest member a bus declares is 4095"
        );

        // A literal too wide for `u32` is refused by the same rule, and the
        // refusal quotes it rather than a number it could not hold.
        assert_eq!(
            BusSlice::parse("DATA[99999999999]"),
            Err(BusParseError::IndexOutOfRange("99999999999".to_owned()))
        );
    }

    fn declared(id: u64, name: &str, start: Point, end: Point) -> Bus {
        Bus::segment(id, start, end, Some(BusDeclaration::parse(name).unwrap())).unwrap()
    }

    fn placed_port(state: &mut SchematicState, name: &str, pos: Point) {
        let id = state.add_component(ComponentType::Port, pos);
        state
            .design
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("placed port")
            .value = name.to_owned();
    }

    fn connectivity(state: &SchematicState) -> VectorConnectivity {
        vector_connectivity(
            &state.design.document().buses,
            &state.design.document().components,
            |component| component.terminal_positions_resolved(None),
        )
    }

    #[test]
    fn identical_declarations_that_touch_are_one_vector_net() {
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().buses = vec![
            declared(1, "DATA[7:0]", Point::new(0, 0), Point::new(40, 0)),
            declared(2, "DATA[7:0]", Point::new(40, 0), Point::new(40, 40)),
            // Same declaration, nowhere near the other two: a separate net in
            // the graph, and the same eight conductors once projected.
            declared(3, "DATA[7:0]", Point::new(200, 0), Point::new(240, 0)),
            declared(4, "ADDR[3:0]", Point::new(0, 80), Point::new(40, 80)),
        ];

        let vectors = connectivity(&schematic);

        assert_eq!(vectors.nets.len(), 3);
        assert!(vectors.mismatches.is_empty());
        let merged = vectors
            .nets
            .iter()
            .find(|net| net.attachments.contains(&Point::new(0, 0)))
            .expect("the first bus joined a net");
        assert!(merged.attachments.contains(&Point::new(40, 40)));
    }

    #[test]
    fn a_vector_port_joins_the_bus_that_declares_the_same_range() {
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .buses
            .push(declared(
                1,
                "DATA[3:0]",
                Point::new(90, 0),
                Point::new(150, 0),
            ));
        placed_port(&mut schematic, "DATA[3:0]", Point::new(100, 0));

        let vectors = connectivity(&schematic);

        assert_eq!(vectors.nets.len(), 1);
        assert!(vectors.mismatches.is_empty());
        assert!(
            vectors.nets[0].attachments.contains(&Point::new(90, 0)),
            "the port terminal joins the bus it stands on"
        );
    }

    #[test]
    fn a_vector_port_on_a_bus_of_another_range_states_both_widths() {
        let mut schematic = SchematicState::default();
        schematic
            .design
            .document_mut_for_test()
            .buses
            .push(declared(
                1,
                "DATA[1:0]",
                Point::new(90, 0),
                Point::new(150, 0),
            ));
        placed_port(&mut schematic, "DATA[3:0]", Point::new(100, 0));

        let vectors = connectivity(&schematic);

        assert_eq!(vectors.mismatches.len(), 1);
        let mismatch = &vectors.mismatches[0];
        assert_eq!(mismatch.declared_width, 4);
        assert_eq!(mismatch.found_width, 2);
        let message = mismatch.message();
        assert!(
            message.contains("DATA[3:0]")
                && message.contains("4 bits")
                && message.contains("DATA[1:0]")
                && message.contains("2 bits"),
            "{message}"
        );
    }
}
