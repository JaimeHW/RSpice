//! Typed bus and bus-tap design data.
//!
//! Bus declarations are deliberately parsed into a durable semantic model.
//! Downstream connectivity and netlisting code never has to reinterpret an
//! arbitrary display string, which keeps range direction, delimiter style,
//! and scalar-versus-slice intent unambiguous across persistence boundaries.

use serde::{Deserialize, Serialize};

pub use rspice_design_model::bus::{
    BusDeclaration, BusDirection, BusMember, BusNotation, BusParseError, BusSlice, BusTargetKind,
    MAX_BUS_MEMBER_INDEX, declared_vector, declared_width,
};

use super::component::Component;
use super::component_type::ComponentType;
use super::state::SchematicState;
use super::{Point, WireRoutingMode};

/// Exact mutation scope resolved for one bus-property transaction.
///
/// The GUI uses the same domain plan that will commit the edit, so a
/// connected-network refactor is never presented as a single-object change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BusPropertyImpact {
    pub connected_buses: usize,
    pub buses_changed: usize,
    pub taps_changed: usize,
}

impl BusPropertyImpact {
    pub const fn has_changes(self) -> bool {
        self.buses_changed != 0 || self.taps_changed != 0
    }
}

/// User-controlled orientation of a bus tap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BusTapOrientation {
    #[default]
    Automatic,
    Left,
    Right,
    Up,
    Down,
}

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

/// A durable polyline carrying an optional typed declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bus {
    pub id: u64,
    pub points: Vec<Point>,
    pub declaration: Option<BusDeclaration>,
}

impl Bus {
    pub fn new(
        id: u64,
        points: Vec<Point>,
        declaration: Option<BusDeclaration>,
    ) -> Result<Self, BusParseError> {
        let bus = Self {
            id,
            points,
            declaration,
        };
        bus.validate()?;
        Ok(bus)
    }

    pub fn segment(
        id: u64,
        start: Point,
        end: Point,
        declaration: Option<BusDeclaration>,
    ) -> Result<Self, BusParseError> {
        Self::new(id, vec![start, end], declaration)
    }

    pub fn validate(&self) -> Result<(), BusParseError> {
        if self.points.len() < 2 || self.points.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(BusParseError::InvalidGeometry);
        }
        if let Some(declaration) = &self.declaration {
            declaration.validate()?;
        }
        Ok(())
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.points
            .windows(2)
            .any(|pair| point_on_segment(point, pair[0], pair[1]))
    }

    pub fn translate(&mut self, delta: Point) {
        for point in &mut self.points {
            point.x = point.x.saturating_add(delta.x);
            point.y = point.y.saturating_add(delta.y);
        }
    }
}

/// A typed scalar or slice connection emerging from a declared bus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusTap {
    pub id: u64,
    pub bus_id: u64,
    pub bus_point: Point,
    pub connection_point: Point,
    pub slice: BusSlice,
    pub orientation: BusTapOrientation,
}

impl BusTap {
    pub fn new(
        id: u64,
        bus: &Bus,
        bus_point: Point,
        connection_point: Point,
        slice: BusSlice,
        orientation: BusTapOrientation,
    ) -> Result<Self, BusParseError> {
        let tap = Self {
            id,
            bus_id: bus.id,
            bus_point,
            connection_point,
            slice,
            orientation,
        };
        tap.validate_against_bus(bus)?;
        Ok(tap)
    }

    pub fn validate_against_bus(&self, bus: &Bus) -> Result<(), BusParseError> {
        if self.bus_id != bus.id || !bus.contains_point(self.bus_point) {
            return Err(BusParseError::InvalidBusReference);
        }
        if self.bus_point == self.connection_point {
            return Err(BusParseError::InvalidGeometry);
        }
        let declaration = bus
            .declaration
            .as_ref()
            .ok_or(BusParseError::UndeclaredBus)?;
        declaration.validate_slice(&self.slice)
    }

    pub fn width(&self) -> usize {
        self.slice.width()
    }

    pub fn members(&self) -> Vec<BusMember> {
        self.slice.members()
    }

    pub fn target_kind(&self) -> BusTargetKind {
        self.slice.target_kind()
    }

    pub fn translate(&mut self, delta: Point) {
        self.bus_point.x = self.bus_point.x.saturating_add(delta.x);
        self.bus_point.y = self.bus_point.y.saturating_add(delta.y);
        self.connection_point.x = self.connection_point.x.saturating_add(delta.x);
        self.connection_point.y = self.connection_point.y.saturating_add(delta.y);
    }
}

fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
    let px = i128::from(point.x);
    let py = i128::from(point.y);
    let ax = i128::from(start.x);
    let ay = i128::from(start.y);
    let bx = i128::from(end.x);
    let by = i128::from(end.y);
    let cross = (px - ax) * (by - ay) - (py - ay) * (bx - ax);
    cross == 0 && px >= ax.min(bx) && px <= ax.max(bx) && py >= ay.min(by) && py <= ay.max(by)
}

pub(crate) fn nearest_lattice_point_on_segment(point: Point, start: Point, end: Point) -> Point {
    let dx = i64::from(end.x) - i64::from(start.x);
    let dy = i64::from(end.y) - i64::from(start.y);
    let steps = gcd_u64(dx.unsigned_abs(), dy.unsigned_abs());
    if steps == 0 {
        return start;
    }
    let dx128 = i128::from(dx);
    let dy128 = i128::from(dy);
    let denominator = dx128 * dx128 + dy128 * dy128;
    let numerator = (i128::from(point.x) - i128::from(start.x)) * dx128
        + (i128::from(point.y) - i128::from(start.y)) * dy128;
    let steps128 = i128::from(steps);
    let step = if numerator <= 0 {
        0
    } else if numerator >= denominator {
        steps128
    } else {
        (numerator * steps128 + denominator / 2) / denominator
    };
    let x = i128::from(start.x) + dx128 * step / steps128;
    let y = i128::from(start.y) + dy128 * step / steps128;
    Point::new(
        i32::try_from(x).expect("lattice projection remains inside its i32 segment"),
        i32::try_from(y).expect("lattice projection remains inside its i32 segment"),
    )
}

fn gcd_u64(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

/// One vector net: a declaration and every point that joins it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorNet {
    pub declaration: BusDeclaration,
    /// Bus vertices and vector terminals that resolve to this net. A point is
    /// listed once, and every point of one net carries the same declaration.
    pub attachments: Vec<Point>,
}

/// A vector connection whose two ends declare different conductors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorWidthMismatch {
    pub point: Point,
    /// What owns the attachment, in the words the drawing uses.
    pub owner: String,
    /// Declaration the attachment carries, and its width.
    pub declared: String,
    pub declared_width: usize,
    /// Declarations the bus geometry under that point carries, joined for
    /// display, and the widest of them.
    pub found: String,
    pub found_width: usize,
}

impl VectorWidthMismatch {
    /// One sentence naming both sums, so a reader never has to count bits.
    pub fn message(&self) -> String {
        format!(
            "{} declares {} ({} bits) but the bus it meets declares {} ({} bits)",
            self.owner, self.declared, self.declared_width, self.found, self.found_width
        )
    }
}

/// Vector connectivity of one schematic: its vector nets and the attachments
/// that disagree about width.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VectorConnectivity {
    pub nets: Vec<VectorNet>,
    pub mismatches: Vec<VectorWidthMismatch>,
}

/// Extract the schematic's vector nets in one union-find pass.
///
/// Declared buses are the seeds. Two buses are one vector net when their
/// geometry touches AND their declarations are identical — a touching pair
/// that disagrees is a range conflict, reported by the bus-tap projection
/// rather than silently fused here. A vector terminal joins the net of the bus
/// under it when the two declarations agree, and is reported as a width
/// mismatch when they do not.
///
/// `terminals_of` supplies each component's terminal names and positions, so a
/// caller that can resolve authored symbol geometry uses it and one that
/// cannot still sees the same rule applied to the placed geometry.
pub fn vector_connectivity(
    schematic: &SchematicState,
    mut terminals_of: impl FnMut(&Component) -> Vec<(String, Point)>,
) -> VectorConnectivity {
    let declared: Vec<(&Bus, &BusDeclaration)> = schematic
        .buses
        .iter()
        .filter(|bus| bus.validate().is_ok())
        .filter_map(|bus| {
            bus.declaration
                .as_ref()
                .map(|declaration| (bus, declaration))
        })
        .collect();

    let mut parents: Vec<usize> = (0..declared.len()).collect();
    for index in 0..declared.len() {
        for other in (index + 1)..declared.len() {
            if declared[index].1 == declared[other].1
                && buses_touch(declared[index].0, declared[other].0)
            {
                union(&mut parents, index, other);
            }
        }
    }

    let mut nets: Vec<VectorNet> = Vec::new();
    let mut net_of_root: Vec<Option<usize>> = vec![None; declared.len()];
    for (index, &(bus, declaration)) in declared.iter().enumerate() {
        let root = find(&mut parents, index);
        let net_index = match net_of_root[root] {
            Some(net_index) => net_index,
            None => {
                nets.push(VectorNet {
                    declaration: declaration.clone(),
                    attachments: Vec::new(),
                });
                net_of_root[root] = Some(nets.len() - 1);
                nets.len() - 1
            }
        };
        for point in &bus.points {
            attach(&mut nets[net_index], *point);
        }
    }

    let mut mismatches = Vec::new();
    for component in &schematic.components {
        for (owner, declaration, point) in vector_terminals(component, &mut terminals_of) {
            let candidates: Vec<usize> = (0..declared.len())
                .filter(|index| declared[*index].0.contains_point(point))
                .collect();
            if candidates.is_empty() {
                continue;
            }
            let matching = candidates
                .iter()
                .copied()
                .find(|index| *declared[*index].1 == declaration);
            match matching {
                Some(index) => {
                    let root = find(&mut parents, index);
                    if let Some(net_index) = net_of_root[root] {
                        attach(&mut nets[net_index], point);
                    }
                }
                None => {
                    let found = candidates
                        .iter()
                        .map(|index| declared[*index].1.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    let found_width = candidates
                        .iter()
                        .map(|index| declared[*index].1.width())
                        .max()
                        .unwrap_or(0);
                    mismatches.push(VectorWidthMismatch {
                        point,
                        owner,
                        declared: declaration.to_string(),
                        declared_width: declaration.width(),
                        found,
                        found_width,
                    });
                }
            }
        }
    }

    VectorConnectivity { nets, mismatches }
}

/// The vector-declaring terminals of one component, named as the drawing names
/// them.
///
/// An interface port declares through the name it carries as a net. A placed
/// instance declares through its bound interface: the frozen terminal order is
/// what holds the port names, because the generic terminal labels a placement
/// falls back to when no authored symbol resolves would declare nothing at all.
fn vector_terminals(
    component: &Component,
    terminals_of: &mut impl FnMut(&Component) -> Vec<(String, Point)>,
) -> Vec<(String, BusDeclaration, Point)> {
    if component.kind == ComponentType::Port {
        let Some(spec) = component.port_spec() else {
            return Vec::new();
        };
        let Some(declaration) = declared_vector(&spec.name) else {
            return Vec::new();
        };
        return terminals_of(component)
            .into_iter()
            .next()
            .map(|(_, point)| vec![(format!("Interface port {}", spec.name), declaration, point)])
            .unwrap_or_default();
    }
    let drawn = terminals_of(component);
    let bound = component
        .library_cell
        .as_ref()
        .map(|binding| binding.terminal_order.as_slice())
        .filter(|order| order.len() == drawn.len());
    drawn
        .iter()
        .enumerate()
        .filter_map(|(index, (drawn_name, point))| {
            let name = bound.map_or(drawn_name.as_str(), |order| order[index].as_str());
            let declaration = declared_vector(name)?;
            Some((
                format!("Terminal {name} of {}", component.spice_instance_name()),
                declaration,
                *point,
            ))
        })
        .collect()
}

fn attach(net: &mut VectorNet, point: Point) {
    if !net.attachments.contains(&point) {
        net.attachments.push(point);
    }
}

/// Endpoint contact, exactly as scalar wires join: a shared endpoint or a
/// T-contact connects, and two strokes that merely cross do not.
fn buses_touch(left: &Bus, right: &Bus) -> bool {
    left.points.iter().any(|point| right.contains_point(*point))
        || right.points.iter().any(|point| left.contains_point(*point))
}

fn find(parents: &mut [usize], mut node: usize) -> usize {
    while parents[node] != node {
        parents[node] = parents[parents[node]];
        node = parents[node];
    }
    node
}

fn union(parents: &mut [usize], left: usize, right: usize) {
    let left_root = find(parents, left);
    let right_root = find(parents, right);
    if left_root != right_root {
        parents[right_root.max(left_root)] = right_root.min(left_root);
    }
}

impl Bus {
    /// Nearest grid point on the polyline and its squared distance.
    pub fn nearest_point(&self, point: Point) -> Option<(Point, i128)> {
        self.points
            .windows(2)
            .map(|pair| nearest_lattice_point_on_segment(point, pair[0], pair[1]))
            .map(|candidate| {
                let dx = i128::from(candidate.x) - i128::from(point.x);
                let dy = i128::from(candidate.y) - i128::from(point.y);
                (candidate, dx * dx + dy * dy)
            })
            .min_by_key(|(_, distance)| *distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn tap_validation_checks_source_geometry_and_type() {
        let declaration = BusDeclaration::parse("DATA[7:0]").unwrap();
        let bus = Bus::segment(4, Point::new(0, 0), Point::new(20, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            5,
            &bus,
            Point::new(10, 0),
            Point::new(10, 5),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        assert_eq!(tap.target_kind(), BusTargetKind::Wire);
        assert_eq!(tap.members()[0].to_string(), "DATA[3]");
        assert!(
            BusTap::new(
                6,
                &bus,
                Point::new(10, 1),
                Point::new(10, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Automatic,
            )
            .is_err()
        );
    }

    #[test]
    fn durable_models_preserve_stable_ids_and_types_through_serde() {
        let bus = Bus::segment(
            41,
            Point::new(-5, 2),
            Point::new(15, 2),
            Some(BusDeclaration::parse("CTRL<0:3>").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            42,
            &bus,
            Point::new(5, 2),
            Point::new(5, 8),
            BusSlice::parse("CTRL<2>").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let encoded = serde_json::to_string(&(bus.clone(), tap.clone())).unwrap();
        let decoded: (Bus, BusTap) = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, (bus, tap));
    }

    #[test]
    fn extreme_coordinate_geometry_does_not_overflow() {
        let diagonal = Bus::segment(
            1,
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MAX),
            None,
        )
        .unwrap();
        assert!(diagonal.contains_point(Point::origin()));
        let bus = Bus::segment(
            2,
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MIN),
            None,
        )
        .unwrap();
        let (nearest, distance) = bus.nearest_point(Point::new(i32::MIN, i32::MAX)).unwrap();
        assert!(distance > i128::from(i64::MAX));
        assert_eq!(nearest, Point::new(i32::MIN, i32::MIN));
    }

    fn declared(id: u64, name: &str, start: Point, end: Point) -> Bus {
        Bus::segment(id, start, end, Some(BusDeclaration::parse(name).unwrap())).unwrap()
    }

    fn placed_port(state: &mut SchematicState, name: &str, pos: Point) {
        let id = state.add_component(ComponentType::Port, pos);
        state
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("placed port")
            .value = name.to_owned();
    }

    fn connectivity(state: &SchematicState) -> VectorConnectivity {
        vector_connectivity(state, |component| {
            component.terminal_positions_resolved(None)
        })
    }

    #[test]
    fn identical_declarations_that_touch_are_one_vector_net() {
        let mut schematic = SchematicState::default();
        schematic.buses = vec![
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
        schematic.buses.push(declared(
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
        schematic.buses.push(declared(
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

    #[test]
    fn nearest_point_on_any_angle_bus_is_always_an_exact_lattice_member() {
        let sparse = Bus::segment(20, Point::new(0, 0), Point::new(10, 3), None).unwrap();
        let (sparse_hit, _) = sparse.nearest_point(Point::new(5, 2)).unwrap();
        assert!(sparse.contains_point(sparse_hit));
        assert!(matches!(
            sparse_hit,
            Point { x: 0, y: 0 } | Point { x: 10, y: 3 }
        ));

        let dense = Bus::segment(21, Point::new(0, 0), Point::new(10, 4), None).unwrap();
        let (dense_hit, _) = dense.nearest_point(Point::new(6, 3)).unwrap();
        assert_eq!(dense_hit, Point::new(5, 2));
        assert!(dense.contains_point(dense_hit));
    }
}
