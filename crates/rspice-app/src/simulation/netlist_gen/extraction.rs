//! The one connectivity extraction: geometry, names, and ground, resolved once.
//!
//! Wires, junctions, terminals, net labels, interface ports, typed bus taps,
//! vector projection and the ground symbol become nets here and nowhere else.
//! The emitted deck, the net summary the navigator shows, and the electrical
//! rule check all read this result rather than tracing the drawing again.
//!
//! That single ownership is the invariant. Two extractions of one drawing
//! disagree — over a terminal that sits mid-segment, over which label wins a
//! net, over whether a ground symbol reached the circuit — and a checker that
//! disagrees with the netlister signs off on a circuit the engine never runs.
//! Anything that needs to know what is connected to what asks [`extract`].

use std::collections::{HashMap, HashSet};

use super::{HierarchySource, Net, deck_bit_name};
use crate::schematic::bus_connectivity::{
    BusConnectivityAnalysis, BusDiagnosticKind, analyze_bus_connectivity,
};
use crate::state::{
    BusNotation, Component, ComponentType, NetLabel, NetNamingPolicy, Point, SchematicState,
    VectorConnectivity, WireSegment, vector_connectivity,
};

/// One placed terminal, bound to the node the deck emits for it.
#[derive(Debug, Clone)]
pub struct ExtractedTerminal {
    /// Owning component, for selection and cross-probing.
    pub component_id: u64,
    /// Terminal name on that instance.
    pub pin: String,
    /// Where the terminal sits in schematic space.
    pub point: Point,
    /// The node name the deck writes for that net.
    pub net_name: String,
    /// Whether anything else in the drawing meets this terminal: a conductor,
    /// another terminal, a net label, or a typed bus tap. A terminal that meets
    /// nothing is an open pin, and this is the only place that judgement is
    /// made.
    pub attached: bool,
}

/// What a connectivity diagnostic is about.
///
/// The kinds a design check has a rule for are named individually so it can
/// route each to its own finding; everything else the naming pass refuses is
/// [`ConnectivityDiagnosticKind::NetNaming`], which blocks the deck and has no
/// electrical rule of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectivityDiagnosticKind {
    /// Bus geometry or declaration is structurally invalid.
    MalformedBus,
    /// A bus carries no typed declaration.
    UnnamedBus,
    /// Connected declarations, a tap selector, or a label and a typed member
    /// disagree about which conductors a net carries.
    BusRangeConflict,
    /// A bus tap does not terminate on its required target kind.
    DanglingBusTap,
    /// Scalar and vector connectivity are mixed at a bus tap.
    MixedBusTap,
    /// A net label stands on neither a conductor nor a terminal.
    OrphanNetLabel,
    /// A vector join whose two ends declare different conductors.
    VectorWidthMismatch,
    /// A name the deck cannot carry: an anonymous interface port, an invalid
    /// label, or two labels claiming one net.
    NetNaming,
}

/// Where a connectivity diagnostic points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectivityAnchor {
    /// A position in schematic space.
    Point(Point),
    /// A drawn bus.
    Bus(u64),
    /// A drawn bus tap.
    BusTap(u64),
    /// A drawn net label, by the name it carries.
    NetLabel(String),
    /// An electrical node, by name.
    Net(String),
}

/// One thing the extraction found wrong with the drawing's connectivity.
#[derive(Debug, Clone)]
pub struct ConnectivityDiagnostic {
    /// What the diagnostic is about.
    pub kind: ConnectivityDiagnosticKind,
    /// Whether the deck refuses to emit while this stands.
    pub blocking: bool,
    /// The one rendering of this finding, quoting the spellings the drawing
    /// uses.
    pub message: String,
    /// Where it points.
    pub anchor: ConnectivityAnchor,
}

/// Everything one cell view's drawing says about what is connected to what.
#[derive(Debug, Clone, Default)]
pub struct ExtractedConnectivity {
    /// Every electrical net, named as the deck names it.
    pub nets: Vec<Net>,
    /// Node points to net identity. Points between nodes resolve through the
    /// conductor they lie on rather than appearing here.
    pub point_to_net: HashMap<Point, usize>,
    /// Every placed terminal, bound to its net.
    pub terminals: Vec<ExtractedTerminal>,
    /// The vector nets the drawing declares, and the joins that disagree about
    /// width. The projection onto scalar nodes has already happened in `nets`;
    /// this is the declaration-level view the width policy judges.
    pub vector_nets: VectorConnectivity,
    /// What the extraction found wrong, in the order it found it.
    pub diagnostics: Vec<ConnectivityDiagnostic>,
    /// The net the deck emits as node 0, when a ground symbol binds one.
    pub ground_net: Option<usize>,
    /// Nets a typed bus tap binds. A tap is a real electrical endpoint that no
    /// placed terminal represents, so a conductor that reaches one is not
    /// dangling even with no instance on it.
    pub tapped_nets: HashSet<usize>,
    /// Deck node name to the spelling the drawing shows, for the nodes where
    /// the two differ — the projected bits of a vector, whose `#` separator no
    /// one can author. A message that quoted the deck spelling would be naming
    /// a net the drawing does not contain.
    pub display_names: HashMap<String, String>,
}

impl ExtractedConnectivity {
    /// The net a drawn point belongs to.
    ///
    /// Every point the drawing gives meaning to — a wire vertex, a terminal, a
    /// junction, a net-label anchor, a typed bus tap — is a node of the traced
    /// graph, so this is a lookup rather than a second traversal. A coordinate
    /// between nodes belongs to the conductor it lies on, and the caller that
    /// has that conductor asks [`ExtractedConnectivity::net_of_wire`] instead.
    pub fn net_at(&self, point: Point) -> Option<&Net> {
        let id = *self.point_to_net.get(&point)?;
        self.nets.iter().find(|net| net.id == id)
    }

    /// The net a drawn conductor belongs to.
    pub fn net_of_wire(&self, wire_id: u64) -> Option<&Net> {
        self.nets.iter().find(|net| net.wires.contains(&wire_id))
    }
}

/// Resolve one cell view's connectivity.
///
/// `hierarchy` supplies authored symbol geometry and the global-net promotion
/// contract; without one the placed geometry is used, which is the same rule
/// applied to less information.
pub fn extract(
    schematic: &SchematicState,
    hierarchy: Option<&HierarchySource<'_>>,
) -> ExtractedConnectivity {
    let mut pass = Pass::new(schematic, hierarchy);
    pass.trace_conductors();
    pass.project_vector_nets();
    pass.apply_interface_ports();
    pass.apply_net_labels();
    pass.identify_ground();
    pass.finish()
}

/// One component's terminal names and positions, resolved through the authored
/// symbol when the hierarchy carries one.
pub(super) fn terminal_positions(
    component: &Component,
    hierarchy: Option<&HierarchySource<'_>>,
) -> Vec<(String, Point)> {
    let resolved_symbol = component
        .library_cell
        .as_ref()
        .and_then(|binding| hierarchy?.resolved_symbol_for(binding));
    component.terminal_positions_resolved(resolved_symbol.as_ref())
}

/// One net name as the drawing spells it.
///
/// A projected bit carries its deck spelling, and `#` is a character no one can
/// author, so a diagnostic that quoted it back would be describing a name the
/// drawing does not contain. The bit is rendered through the notation its own
/// declaration used — the declaration is the authority for the delimiters
/// exactly as it is for the width — and every other name is already the
/// authored form.
pub(super) fn display_net_name(schematic: &SchematicState, name: &str) -> String {
    super::vector_names::display_bit_name(name, declared_notation(schematic, name))
        .unwrap_or_else(|| name.to_owned())
}

/// The notation the vector that owns this deck bit was declared in.
///
/// Only declared bus geometry mints a projected bit, so the declaring bus is
/// where the delimiters come from. A name that no declaration claims is not a
/// bit and the answer is never used.
fn declared_notation(schematic: &SchematicState, deck_name: &str) -> BusNotation {
    let base = deck_name
        .rsplit_once('#')
        .map_or(deck_name, |(base, _)| base);
    schematic
        .buses
        .iter()
        .filter_map(|bus| bus.declaration.as_ref())
        .find(|declaration| declaration.name == base)
        .map_or(BusNotation::default(), |declaration| declaration.notation)
}

/// Net identity folds ASCII case because the emitted deck does: SPICE node
/// names are case-insensitive, so `Out` and `out` are one node in the
/// simulation whatever the document's naming policy says. That policy is an
/// authoring-syntax rule — which characters a name may contain — and never a
/// statement about which nets are the same net.
fn net_name_key(name: &str) -> String {
    name.to_ascii_lowercase()
}

/// Identity again, for a direct comparison: see [`net_name_key`].
fn net_names_equal(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

fn validate_net_name(name: &str, policy: NetNamingPolicy) -> Result<(), &'static str> {
    NetLabel::validate_name(name, policy)
}

/// The extraction in progress.
///
/// The phases run in one fixed order, and the order is the label-winner rule:
/// an interface port names its net first because the port list is the cell's
/// contract, then a typed bus member because a tap is an electrical
/// declaration, then the lowest-numbered ordinary label, and finally the
/// ground symbol, which overrides every name with node 0.
struct Pass<'a> {
    schematic: &'a SchematicState,
    hierarchy: Option<&'a HierarchySource<'a>>,
    buses: BusConnectivityAnalysis,
    nets: Vec<Net>,
    point_to_net: HashMap<Point, usize>,
    ground_net: Option<usize>,
    vector_nets: VectorConnectivity,
    diagnostics: Vec<ConnectivityDiagnostic>,
}

impl<'a> Pass<'a> {
    fn new(schematic: &'a SchematicState, hierarchy: Option<&'a HierarchySource<'a>>) -> Self {
        let mut buses = analyze_bus_connectivity(schematic);
        buses.scalar_taps.sort_by_key(|binding| binding.tap_id);
        Self {
            schematic,
            hierarchy,
            buses,
            nets: Vec::new(),
            point_to_net: HashMap::new(),
            ground_net: None,
            vector_nets: VectorConnectivity::default(),
            diagnostics: Vec::new(),
        }
    }

    fn report(
        &mut self,
        kind: ConnectivityDiagnosticKind,
        blocking: bool,
        message: String,
        anchor: ConnectivityAnchor,
    ) {
        self.diagnostics.push(ConnectivityDiagnostic {
            kind,
            blocking,
            message,
            anchor,
        });
    }

    fn naming_error(&mut self, message: String, anchor: ConnectivityAnchor) {
        self.report(ConnectivityDiagnosticKind::NetNaming, true, message, anchor);
    }

    fn net(&self, id: usize) -> Option<&Net> {
        self.nets.iter().find(|net| net.id == id)
    }

    /// Extract electrical nets from wire connectivity.
    ///
    /// The graph holds only meaningful nodes — wire vertices, component
    /// terminals, junctions, and net-label anchors — and chains each wire
    /// segment through the candidates that lie on it. Connectivity is
    /// identical to enumerating every coordinate along every segment (a point
    /// matters only if some other geometry references it), at a fraction of
    /// the cost and memory.
    fn trace_conductors(&mut self) {
        let schematic = self.schematic;
        let hierarchy = self.hierarchy;
        for index in 0..self.buses.diagnostics.len() {
            let diagnostic = self.buses.diagnostics[index].clone();
            let kind = match diagnostic.kind {
                BusDiagnosticKind::MalformedBus => ConnectivityDiagnosticKind::MalformedBus,
                BusDiagnosticKind::UnnamedBus => ConnectivityDiagnosticKind::UnnamedBus,
                BusDiagnosticKind::RangeConflict => ConnectivityDiagnosticKind::BusRangeConflict,
                BusDiagnosticKind::DanglingTap => ConnectivityDiagnosticKind::DanglingBusTap,
                BusDiagnosticKind::MixedTap => ConnectivityDiagnosticKind::MixedBusTap,
            };
            let anchor = if let Some(tap_id) = diagnostic.tap_id {
                ConnectivityAnchor::BusTap(tap_id)
            } else if let Some(bus_id) = diagnostic.bus_id {
                ConnectivityAnchor::Bus(bus_id)
            } else {
                ConnectivityAnchor::Point(diagnostic.point)
            };
            let blocking = kind != ConnectivityDiagnosticKind::UnnamedBus;
            self.report(kind, blocking, diagnostic.message, anchor);
        }

        // Seeded nodes: always nets, even when isolated (a floating terminal
        // must still get its own SPICE node).
        let mut point_graph: HashMap<Point, HashSet<Point>> = HashMap::new();
        for wire in &schematic.wires {
            for point in &wire.points {
                point_graph.entry(*point).or_default();
            }
        }
        for component in &schematic.components {
            for (_, terminal_pos) in terminal_positions(component, hierarchy) {
                point_graph.entry(terminal_pos).or_default();
            }
        }

        // Chain candidates: points that join a segment when they lie on it.
        // Labels and junctions are candidates but not seeds — a label floating
        // in empty space must NOT mint a net (it warns instead).
        let mut by_row: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut by_col: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut candidate_points: HashSet<Point> = HashSet::new();
        {
            let mut add_candidate = |p: Point| {
                by_row.entry(p.y).or_default().push(p.x);
                by_col.entry(p.x).or_default().push(p.y);
                candidate_points.insert(p);
            };
            for point in point_graph.keys() {
                add_candidate(*point);
            }
            for junction in &schematic.junctions {
                add_candidate(junction.pos);
            }
            for label in &schematic.net_labels {
                add_candidate(label.pos);
            }
            for binding in &self.buses.scalar_taps {
                add_candidate(binding.point);
            }
        }
        for xs in by_row.values_mut() {
            xs.sort_unstable();
            xs.dedup();
        }
        for ys in by_col.values_mut() {
            ys.sort_unstable();
            ys.dedup();
        }

        fn link(graph: &mut HashMap<Point, HashSet<Point>>, a: Point, b: Point) {
            if a != b {
                graph.entry(a).or_default().insert(b);
                graph.entry(b).or_default().insert(a);
            }
        }

        for wire in &schematic.wires {
            for seg in wire.points.windows(2) {
                let (a, b) = (seg[0], seg[1]);
                if a.y == b.y {
                    // Horizontal: chain through candidates on this row.
                    let (x0, x1) = (a.x.min(b.x), a.x.max(b.x));
                    let mut prev = Point::new(x0, a.y);
                    if let Some(xs) = by_row.get(&a.y) {
                        let lo = xs.partition_point(|&x| x < x0);
                        let hi = xs.partition_point(|&x| x <= x1);
                        for &x in &xs[lo..hi] {
                            let p = Point::new(x, a.y);
                            link(&mut point_graph, prev, p);
                            prev = p;
                        }
                    }
                    link(&mut point_graph, prev, Point::new(x1, a.y));
                } else if a.x == b.x {
                    // Vertical: chain through candidates on this column.
                    let (y0, y1) = (a.y.min(b.y), a.y.max(b.y));
                    let mut prev = Point::new(a.x, y0);
                    if let Some(ys) = by_col.get(&a.x) {
                        let lo = ys.partition_point(|&y| y < y0);
                        let hi = ys.partition_point(|&y| y <= y1);
                        for &y in &ys[lo..hi] {
                            let p = Point::new(a.x, y);
                            link(&mut point_graph, prev, p);
                            prev = p;
                        }
                    }
                    link(&mut point_graph, prev, Point::new(a.x, y1));
                } else {
                    // Any-angle conductors have the same exact attachment
                    // semantics as Manhattan conductors. Terminals, labels,
                    // junctions, bus-tap endpoints, and another wire's
                    // endpoint on this segment all split its electrical
                    // chain. Interior/interior crossings stay disconnected
                    // unless an explicit junction is authored there.
                    let segment = WireSegment::new(a, b);
                    let mut points = candidate_points
                        .iter()
                        .copied()
                        .filter(|point| segment.contains_point(*point))
                        .collect::<Vec<_>>();
                    points.sort_unstable_by_key(|point| {
                        let dx = i128::from(point.x) - i128::from(a.x);
                        let dy = i128::from(point.y) - i128::from(a.y);
                        dx * dx + dy * dy
                    });
                    points.dedup();
                    for pair in points.windows(2) {
                        link(&mut point_graph, pair[0], pair[1]);
                    }
                }
            }
        }

        // Flood-fill to find connected components (nets).
        let mut visited: HashSet<Point> = HashSet::new();
        let mut net_id = 1;

        // Deterministic traversal is critical because auto-generated node
        // labels (net1, net2, ...) depend on discovery order. HashMap key
        // iteration is intentionally randomized, so sort points first to keep
        // names stable across repeated runs of the same schematic.
        let mut all_points: Vec<Point> = point_graph.keys().copied().collect();
        all_points.sort_by_key(|point| (point.x, point.y));

        for start_point in all_points {
            if visited.contains(&start_point) {
                continue;
            }

            let mut net = Net::new(net_id);
            let mut stack = vec![start_point];

            while let Some(point) = stack.pop() {
                if visited.insert(point) {
                    net.add_point(point);
                    self.point_to_net.insert(point, net_id);

                    if let Some(neighbors) = point_graph.get(&point) {
                        for neighbor in neighbors {
                            if !visited.contains(neighbor) {
                                stack.push(*neighbor);
                            }
                        }
                    }
                }
            }

            if !net.points.is_empty() {
                self.nets.push(net);
                net_id += 1;
            }
        }
    }

    /// Project every vector net of this schematic into named scalar nodes.
    ///
    /// Runs between net extraction and interface-port naming: the bits exist
    /// before any name is resolved, so a scalar tap and a vector port both
    /// resolve onto the same projected node instead of racing to create it.
    fn project_vector_nets(&mut self) {
        let schematic = self.schematic;
        let hierarchy = self.hierarchy;
        let connectivity = vector_connectivity(schematic, |component| {
            terminal_positions(component, hierarchy)
        });

        for mismatch in &connectivity.mismatches {
            self.diagnostics.push(ConnectivityDiagnostic {
                kind: ConnectivityDiagnosticKind::VectorWidthMismatch,
                blocking: true,
                message: mismatch.message(),
                anchor: ConnectivityAnchor::Point(mismatch.point),
            });
        }

        let mut taken: HashSet<String> = self
            .nets
            .iter()
            .filter_map(|net| net.label.as_ref())
            .map(|label| label.to_ascii_lowercase())
            .collect();
        let mut next_id = self.nets.iter().map(|net| net.id).max().unwrap_or(0) + 1;
        for vector in &connectivity.nets {
            for member in vector.declaration.members() {
                let name = deck_bit_name(&vector.declaration.name, member.index);
                if !taken.insert(name.to_ascii_lowercase()) {
                    continue;
                }
                let mut projected = Net::new(next_id);
                projected.label = Some(name);
                self.nets.push(projected);
                next_id += 1;
            }
        }
        self.vector_nets = connectivity;
    }

    /// Fold interface ports into the extracted nets.
    ///
    /// A port names its net exactly like a label, and same-name ports connect
    /// their nets. This runs before [`Pass::apply_net_labels`] so the interface
    /// name wins label conflicts — the port list is the cell's contract. An
    /// unnamed port is an error: the interface cannot contain an anonymous pin.
    fn apply_interface_ports(&mut self) {
        let schematic = self.schematic;
        let hierarchy = self.hierarchy;
        let policy = schematic.document_policy.net_naming;
        let mut name_to_net: HashMap<String, usize> = HashMap::new();
        for component in &schematic.components {
            if component.kind != ComponentType::Port {
                continue;
            }
            let Some(spec) = component.port_spec() else {
                self.naming_error(
                    format!(
                        "Unnamed interface port at ({}, {}) — name it or remove it",
                        component.pos.x, component.pos.y
                    ),
                    ConnectivityAnchor::Point(component.pos),
                );
                continue;
            };
            let Some((_, terminal)) = terminal_positions(component, hierarchy).into_iter().next()
            else {
                continue;
            };
            let Some(&net_id) = self.point_to_net.get(&terminal) else {
                continue;
            };
            // A vector port names no scalar node. Its conductors are the
            // projected bits of the vector net it joins, which already carry
            // the deck names the header declares; writing `DATA[7:0]` onto the
            // terminal's own net would put the authored delimiters — which no
            // probe survives — into the emitted deck.
            if spec.vector().is_some() {
                continue;
            }

            if let Err(error) = validate_net_name(&spec.name, policy) {
                self.naming_error(
                    format!("Invalid interface port name '{}': {error}", spec.name),
                    ConnectivityAnchor::Point(component.pos),
                );
                continue;
            }
            let key = net_name_key(&spec.name);
            match name_to_net.get(&key) {
                // Same port name elsewhere: one interface pin, one net.
                Some(&primary) if primary != net_id => self.merge_nets(primary, net_id),
                _ => {
                    name_to_net.insert(key, net_id);
                    if let Some(net) = self.nets.iter_mut().find(|net| net.id == net_id) {
                        net.label = Some(spec.name.clone());
                    }
                }
            }
        }
    }

    /// Fold user net labels into the extracted nets.
    ///
    /// A label names the net beneath it, and labels sharing a name connect
    /// their nets (standard schematic-label semantics). Conflicts keep the
    /// lowest-numbered label and warn; a label off any wire or terminal is
    /// reported as an orphan.
    fn apply_net_labels(&mut self) {
        let schematic = self.schematic;
        let policy = schematic.document_policy.net_naming;
        // Typed scalar aliases are the electrical contract, so resolve them
        // before ordinary labels. A conflicting free-form label is a blocking
        // error rather than a warning that silently changes DATA[n] into an
        // unrelated scalar node name.
        let typed_bindings = self.buses.scalar_taps.clone();

        // Seed with names already assigned (interface ports run first), so a
        // label matching a port name connects to the port's net instead of
        // minting a same-named twin.
        let mut name_to_net: HashMap<String, usize> = self
            .nets
            .iter()
            .filter_map(|net| {
                net.label
                    .as_ref()
                    .map(|label| (net_name_key(label), net.id))
            })
            .collect();
        // The authored spelling is what a user could have written, so it is
        // what gets validated, keyed and reported; the deck spelling is what
        // the net is actually named, because `<3>` does not survive a probe.
        for binding in &typed_bindings {
            let name = binding.member_name.as_str();
            let deck_name = binding.deck_name.as_str();
            if let Err(error) = validate_net_name(name, policy) {
                self.naming_error(
                    format!("Invalid typed bus member \"{name}\": {error}"),
                    ConnectivityAnchor::Point(binding.point),
                );
                continue;
            }
            let bound = self.point_to_net.get(&binding.point).copied();
            let Some(net_id) = bound else {
                self.naming_error(
                    format!(
                        "Typed bus member \"{name}\" at ({}, {}) is not on a scalar net",
                        binding.point.x, binding.point.y
                    ),
                    ConnectivityAnchor::BusTap(binding.tap_id),
                );
                continue;
            };
            let assigned = self.net(net_id).and_then(|net| net.label.clone());
            if let Some(existing) = assigned
                && !net_names_equal(&existing, name)
                && !net_names_equal(&existing, deck_name)
            {
                let existing = display_net_name(schematic, &existing);
                self.report(
                    ConnectivityDiagnosticKind::BusRangeConflict,
                    true,
                    format!("Typed bus member \"{name}\" conflicts with net name \"{existing}\""),
                    ConnectivityAnchor::Net(name.to_owned()),
                );
                continue;
            }

            // The bit this tap selects may already exist as a projected node of
            // its bus. Both spellings are looked up so the tap merges into that
            // node instead of creating a second one under the same deck name.
            let key = net_name_key(name);
            let deck_key = net_name_key(deck_name);
            let effective_net_id = match name_to_net
                .get(&key)
                .or_else(|| name_to_net.get(&deck_key))
                .copied()
            {
                Some(primary) if primary != net_id => {
                    self.merge_nets(primary, net_id);
                    primary
                }
                _ => net_id,
            };
            name_to_net.insert(key, effective_net_id);
            name_to_net.insert(deck_key, effective_net_id);
            if let Some(net) = self.nets.iter_mut().find(|net| net.id == effective_net_id) {
                net.label = Some(deck_name.to_owned());
            }
        }

        let mut labels: Vec<&NetLabel> = schematic.net_labels.iter().collect();
        labels.sort_by_key(|label| label.id);
        for label in labels {
            let authored_name = label.name.trim();
            if authored_name.is_empty() {
                continue;
            }
            if let Err(error) = validate_net_name(authored_name, policy) {
                self.naming_error(
                    format!("Invalid net label \"{authored_name}\": {error}"),
                    ConnectivityAnchor::NetLabel(label.name.clone()),
                );
                continue;
            }
            let promoted_name = self
                .hierarchy
                .and_then(|hierarchy| hierarchy.canonical_global_label(authored_name));
            let name = promoted_name.as_deref().unwrap_or(authored_name);
            let bound = self.point_to_net.get(&label.pos).copied();
            let Some(net_id) = bound else {
                self.report(
                    ConnectivityDiagnosticKind::OrphanNetLabel,
                    false,
                    format!(
                        "Net label \"{authored_name}\" at ({}, {}) is not on a wire or terminal",
                        label.pos.x, label.pos.y
                    ),
                    ConnectivityAnchor::NetLabel(label.name.clone()),
                );
                continue;
            };

            let typed_conflict = typed_bindings.iter().find_map(|binding| {
                (self.point_to_net.get(&binding.point) == Some(&net_id)
                    && !net_names_equal(&binding.member_name, name))
                .then(|| binding.member_name.clone())
            });
            if let Some(typed_name) = typed_conflict {
                let message = format!(
                    "Net label \"{name}\" conflicts with typed bus member \"{typed_name}\""
                );
                self.report(
                    ConnectivityDiagnosticKind::BusRangeConflict,
                    true,
                    message,
                    ConnectivityAnchor::Net(typed_name),
                );
                continue;
            }

            let key = net_name_key(name);
            match name_to_net.get(&key) {
                // Same name on another net: connect them.
                Some(&primary) if primary != net_id => self.merge_nets(primary, net_id),
                _ => {
                    name_to_net.insert(key, net_id);
                    // A tap already named this net in the deck spelling, and a
                    // label that disagreed with the member errored above. This
                    // one agrees, so it must not rewrite the node's identity
                    // back into the authored delimiters.
                    if typed_bindings
                        .iter()
                        .any(|binding| self.point_to_net.get(&binding.point) == Some(&net_id))
                    {
                        continue;
                    }
                    let existing = self.net(net_id).and_then(|net| net.label.clone());
                    match existing {
                        Some(existing) if !net_names_equal(&existing, name) => {
                            let existing = display_net_name(schematic, &existing);
                            let message = format!(
                                "Net carries conflicting labels \"{existing}\" and \"{name}\"; \
                                 keeping \"{existing}\""
                            );
                            self.report(
                                ConnectivityDiagnosticKind::NetNaming,
                                false,
                                message,
                                ConnectivityAnchor::Net(existing),
                            );
                        }
                        _ => {
                            if let Some(net) = self.nets.iter_mut().find(|net| net.id == net_id) {
                                net.label = Some(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Merge net `loser` into `winner`: fold its points, repoint the lookup,
    /// and drop it from the net list.
    fn merge_nets(&mut self, winner: usize, loser: usize) {
        if winner == loser {
            return;
        }
        let Some(index) = self.nets.iter().position(|net| net.id == loser) else {
            return;
        };
        let loser_net = self.nets.remove(index);
        for net_id in self.point_to_net.values_mut() {
            if *net_id == loser {
                *net_id = winner;
            }
        }
        if let Some(net) = self.nets.iter_mut().find(|net| net.id == winner) {
            net.merge(&loser_net);
        }
    }

    /// Bind every ground symbol's net to node 0.
    ///
    /// Standard schematic semantics: separate ground symbols are the same
    /// electrical node even with no drawn wire between them.
    fn identify_ground(&mut self) {
        let schematic = self.schematic;
        let hierarchy = self.hierarchy;
        for component in &schematic.components {
            if component.kind != ComponentType::Ground {
                continue;
            }
            let terminals = terminal_positions(component, hierarchy);
            if let Some((_, terminal_pos)) = terminals.first()
                && let Some(&net_id) = self.point_to_net.get(terminal_pos)
            {
                if self.ground_net.is_none() {
                    self.ground_net = Some(net_id);
                }
                if let Some(net) = self.nets.iter_mut().find(|net| net.id == net_id) {
                    net.label = Some("0".to_string());
                }
            }
        }
    }

    fn finish(mut self) -> ExtractedConnectivity {
        let schematic = self.schematic;
        let hierarchy = self.hierarchy;

        // A wire belongs to exactly one net; index it under that net so a
        // conductor-level finding can name the drawn object.
        for wire in &schematic.wires {
            let Some(&net_id) = wire.points.first().and_then(|p| self.point_to_net.get(p)) else {
                continue;
            };
            if let Some(net) = self.nets.iter_mut().find(|net| net.id == net_id) {
                net.wires.push(wire.id);
            }
        }

        let tapped_nets: HashSet<usize> = self
            .buses
            .scalar_taps
            .iter()
            .filter_map(|binding| self.point_to_net.get(&binding.point).copied())
            .collect();

        // An annotation is a name written onto a point: it holds a terminal
        // that meets no conductor into the drawing rather than leaving it open.
        let annotations: HashSet<Point> = schematic
            .net_labels
            .iter()
            .map(|label| label.pos)
            .chain(self.buses.scalar_taps.iter().map(|binding| binding.point))
            .collect();

        let mut bound: Vec<(u64, String, Point, usize)> = Vec::new();
        let mut terminal_counts: HashMap<usize, usize> = HashMap::new();
        for component in &schematic.components {
            for (pin, point) in terminal_positions(component, hierarchy) {
                let Some(&net_id) = self.point_to_net.get(&point) else {
                    continue;
                };
                *terminal_counts.entry(net_id).or_default() += 1;
                bound.push((component.id, pin, point, net_id));
            }
        }

        let names: HashMap<usize, String> = self
            .nets
            .iter()
            .map(|net| (net.id, net.spice_name()))
            .collect();
        let display_names: HashMap<String, String> = names
            .values()
            .filter_map(|name| {
                let shown = display_net_name(schematic, name);
                (shown != *name).then(|| (name.clone(), shown))
            })
            .collect();
        let spans: HashMap<usize, usize> = self
            .nets
            .iter()
            .map(|net| (net.id, net.points.len()))
            .collect();
        let terminals = bound
            .into_iter()
            .map(|(component_id, pin, point, net)| ExtractedTerminal {
                component_id,
                pin,
                point,
                net_name: names
                    .get(&net)
                    .cloned()
                    .unwrap_or_else(|| format!("net{net}")),
                attached: spans.get(&net).copied().unwrap_or_default() > 1
                    || terminal_counts.get(&net).copied().unwrap_or_default() > 1
                    || annotations.contains(&point),
            })
            .collect();

        ExtractedConnectivity {
            nets: self.nets,
            point_to_net: self.point_to_net,
            terminals,
            vector_nets: self.vector_nets,
            diagnostics: self.diagnostics,
            ground_net: self.ground_net,
            tapped_nets,
            display_names,
        }
    }
}

#[cfg(test)]
mod tests;
