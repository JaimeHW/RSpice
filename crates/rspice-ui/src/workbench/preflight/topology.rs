//! The topology refusals the engine makes, decided before a run is queued.
//!
//! Two of the engine's refusals are structural rather than numerical: a node
//! that no chain of DC-conducting elements ties to the reference has no
//! operating point the circuit determines, and a cycle built only from voltage
//! sources states two voltages for one node pair. The engine raises both while
//! elaborating the deck — after the author has already committed to a run — so
//! preflight decides them here and names the offending nodes and instances
//! while the drawing is still on screen.
//!
//! Connectivity is read, never re-derived: the caller passes the design's one
//! [`ExtractedConnectivity`], the same graph the netlister and the design-rule
//! check read. A second tracing pass here could refuse a topology the emitted
//! deck does not have.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::simulation::netlist_gen::extraction::ExtractedConnectivity;
use crate::state::{Component, ComponentType, PortDirection, SchematicState, is_ground_reference};
use crate::workbench::state::{PreflightIssue, PreflightRemediation};

/// Row labels the Solver surface states as enforced contract. The refusal and
/// the statement of the rule use one vocabulary.
const NO_DC_PATH_CHECK: &str = "Nodes without a DC path";
const VOLTAGE_LOOP_CHECK: &str = "Voltage-source loops";

/// How many offending nodes one refusal quotes before it counts the rest, as
/// the engine's own message does.
const QUOTED_NODE_LIMIT: usize = 8;

/// The control terminal of a field-effect device, which draws no DC current.
const CONTROL_GATE_PIN: &str = "G";

/// Every topology refusal this design carries, in reporting order.
pub(super) fn topology_blockers(
    schematic: &SchematicState,
    connectivity: &ExtractedConnectivity,
) -> Vec<PreflightIssue> {
    let mut nodes = Nodes::default();
    let mut terminals: HashMap<u64, Vec<TerminalNode>> = HashMap::new();
    for terminal in &connectivity.terminals {
        let node = nodes.intern(&terminal.net_name);
        terminals
            .entry(terminal.component_id)
            .or_default()
            .push(TerminalNode {
                pin: terminal.pin.clone(),
                node,
            });
    }

    let mut issues = Vec::new();
    issues.extend(no_dc_path_issue(schematic, &nodes, &terminals));
    issues.extend(voltage_source_loop_issues(schematic, &nodes, &terminals));
    issues
}

/// One placed terminal, bound to the deck node it sits on.
struct TerminalNode {
    pin: String,
    node: usize,
}

/// The deck's nodes, in first-seen order, keyed the way the engine keys them.
///
/// SPICE node names are case-insensitive, so `Out` and `out` are one node in
/// the simulation whatever the document's naming policy allows; the first
/// spelling the drawing used is the one a refusal quotes back.
#[derive(Default)]
struct Nodes {
    index: HashMap<String, usize>,
    names: Vec<String>,
}

impl Nodes {
    fn intern(&mut self, name: &str) -> usize {
        let key = name.to_ascii_lowercase();
        if let Some(&node) = self.index.get(&key) {
            return node;
        }
        let node = self.names.len();
        self.index.insert(key, node);
        self.names.push(name.to_owned());
        node
    }

    fn len(&self) -> usize {
        self.names.len()
    }

    fn name(&self, node: usize) -> &str {
        &self.names[node]
    }
}

/// Disjoint sets over node indices.
struct DisjointSets {
    parent: Vec<usize>,
}

impl DisjointSets {
    fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
        }
    }

    fn find(&mut self, node: usize) -> usize {
        let mut root = node;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        let mut cursor = node;
        while self.parent[cursor] != root {
            let next = self.parent[cursor];
            self.parent[cursor] = root;
            cursor = next;
        }
        root
    }

    /// Join two sets, reporting `false` when they were already one — which is
    /// exactly the condition that makes a new edge close a cycle.
    fn union(&mut self, left: usize, right: usize) -> bool {
        let (left, right) = (self.find(left), self.find(right));
        if left == right {
            return false;
        }
        self.parent[right] = left;
        true
    }
}

/// What a placed instance ties together at DC.
///
/// The two open cases are the ones the engine's own refusal names — capacitors
/// and current sources do not conduct at DC — and the isolated gate is the
/// third structural one: a field-effect control terminal draws no DC current,
/// so a gate reached only through a capacitor is a node no operating point
/// determines. Everything else is read as conducting, because the reading that
/// errs towards conduction can miss a refusal but can never invent one.
enum DcConduction {
    Through,
    Open,
    ExceptControlGate,
}

fn dc_conduction(component: &Component) -> DcConduction {
    match component.kind {
        ComponentType::Capacitor
        | ComponentType::CurrentSource
        | ComponentType::CurrentSourceAc
        | ComponentType::CurrentSourcePulse
        | ComponentType::CurrentSourceSin
        | ComponentType::CurrentSourceExp
        | ComponentType::CurrentSourceSffm
        | ComponentType::CurrentSourceAm
        | ComponentType::CurrentSourcePat
        | ComponentType::CurrentSourceNoise
        | ComponentType::CurrentSourceRandom
        | ComponentType::CurrentSourcePwl
        | ComponentType::CurrentSourcePwlFile => DcConduction::Open,
        ComponentType::Nmos
        | ComponentType::Pmos
        | ComponentType::NVdmos
        | ComponentType::PVdmos
        | ComponentType::NmosSoi
        | ComponentType::PmosSoi
        | ComponentType::Njfet
        | ComponentType::Pjfet
        | ComponentType::Nmesfet
        | ComponentType::Pmesfet => DcConduction::ExceptControlGate,
        _ => DcConduction::Through,
    }
}

fn is_independent_voltage_source(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::VoltageSourceAm
            | ComponentType::VoltageSourcePat
            | ComponentType::VoltageSourceNoise
            | ComponentType::VoltageSourceRandom
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourcePwlFile
    )
}

/// How a refusal names one instance, in the spelling the checker already uses.
fn designator(component: &Component) -> String {
    if component.name.trim().is_empty() {
        format!("component #{}", component.id)
    } else {
        component.name.clone()
    }
}

/// The nodes no conducting chain ties to the reference, as one ordered
/// refusal.
///
/// A design with no reference node at all is not reported here: every node
/// would be listed, burying the missing-ground finding that owns that state.
fn no_dc_path_issue(
    schematic: &SchematicState,
    nodes: &Nodes,
    terminals: &HashMap<u64, Vec<TerminalNode>>,
) -> Option<PreflightIssue> {
    let ground = (0..nodes.len()).find(|&node| is_ground_reference(nodes.name(node)))?;
    let mut sets = DisjointSets::new(nodes.len());
    for alias in (ground + 1)..nodes.len() {
        if is_ground_reference(nodes.name(alias)) {
            sets.union(ground, alias);
        }
    }

    for component in &schematic.components {
        let Some(pins) = terminals.get(&component.id) else {
            continue;
        };
        match dc_conduction(component) {
            DcConduction::Open => {}
            DcConduction::Through => join(&mut sets, pins.iter().map(|pin| pin.node)),
            DcConduction::ExceptControlGate => join(
                &mut sets,
                pins.iter()
                    .filter(|pin| pin.pin != CONTROL_GATE_PIN)
                    .map(|pin| pin.node),
            ),
        }
        // A supply pin declares that the parent feeds this net. That is a
        // source, so the net it names reaches one with nothing else on it —
        // which is what separates a rail from an undriven node.
        if component
            .port_spec()
            .is_some_and(|port| port.direction == PortDirection::Supply)
        {
            join(
                &mut sets,
                std::iter::once(ground).chain(pins.iter().map(|pin| pin.node)),
            );
        }
    }

    let reference = sets.find(ground);
    let mut floating = Vec::new();
    for node in 0..nodes.len() {
        if sets.find(node) != reference {
            floating.push(nodes.name(node));
        }
    }
    if floating.is_empty() {
        return None;
    }
    floating.sort_unstable();

    Some(PreflightIssue {
        check: NO_DC_PATH_CHECK.to_owned(),
        observed: format!(
            "No DC path to ground from node(s) {}: capacitors and field-effect control \
             terminals do not conduct at DC.",
            quoted_list(&floating)
        ),
        required: "A conducting path from every node to the reference node".to_owned(),
        remediation: PreflightRemediation::DesignChecks,
    })
}

/// Every cycle whose edges are all independent voltage sources, one refusal
/// each, naming the sources the cycle runs through.
///
/// Only independent voltage sources are edges. An inductor is a DC short and a
/// dependent source can be one too, but a cycle through either is a refusal
/// this preflight leaves to the engine rather than a run it stops on a reading
/// the deck may not share.
fn voltage_source_loop_issues(
    schematic: &SchematicState,
    nodes: &Nodes,
    terminals: &HashMap<u64, Vec<TerminalNode>>,
) -> Vec<PreflightIssue> {
    let mut sets = DisjointSets::new(nodes.len());
    let mut adjacency: Vec<Vec<(usize, usize)>> = vec![Vec::new(); nodes.len()];
    let mut placed: Vec<String> = Vec::new();
    let mut issues = Vec::new();

    for component in &schematic.components {
        if !is_independent_voltage_source(component.kind) {
            continue;
        }
        let Some(pins) = terminals.get(&component.id) else {
            continue;
        };
        let (Some(plus), Some(minus)) = (pin_node(pins, "+"), pin_node(pins, "-")) else {
            continue;
        };
        let designator = designator(component);
        if plus == minus {
            issues.push(loop_issue(&format!(
                "Voltage source {designator} has both terminals on node '{}'",
                nodes.name(plus)
            )));
            continue;
        }
        if sets.union(plus, minus) {
            adjacency[plus].push((minus, placed.len()));
            adjacency[minus].push((plus, placed.len()));
            placed.push(designator);
            continue;
        }
        let mut members = tree_path(&adjacency, plus, minus)
            .into_iter()
            .map(|edge| placed[edge].clone())
            .collect::<Vec<_>>();
        members.push(designator);
        issues.push(loop_issue(&format!(
            "Voltage sources {} form a closed loop",
            members.join(", ")
        )));
    }
    issues
}

fn loop_issue(observed: &str) -> PreflightIssue {
    PreflightIssue {
        check: VOLTAGE_LOOP_CHECK.to_owned(),
        observed: format!("{observed}: the loop states two voltages for one node pair."),
        required: "A series element inside the loop, or one fewer source across the pair"
            .to_owned(),
        remediation: PreflightRemediation::DesignChecks,
    }
}

fn pin_node(pins: &[TerminalNode], name: &str) -> Option<usize> {
    pins.iter()
        .find(|terminal| terminal.pin == name)
        .map(|terminal| terminal.node)
}

fn join(sets: &mut DisjointSets, mut terminals: impl Iterator<Item = usize>) {
    let Some(first) = terminals.next() else {
        return;
    };
    for node in terminals {
        sets.union(first, node);
    }
}

/// The edges of the spanning-forest path between two nodes already in one set.
fn tree_path(adjacency: &[Vec<(usize, usize)>], from: usize, to: usize) -> Vec<usize> {
    let mut arrival: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut seen: HashSet<usize> = HashSet::from([from]);
    let mut queue: VecDeque<usize> = VecDeque::from([from]);
    while let Some(node) = queue.pop_front() {
        if node == to {
            break;
        }
        for &(next, edge) in &adjacency[node] {
            if seen.insert(next) {
                arrival.insert(next, (node, edge));
                queue.push_back(next);
            }
        }
    }
    let mut edges = Vec::new();
    let mut cursor = to;
    while let Some(&(previous, edge)) = arrival.get(&cursor) {
        edges.push(edge);
        cursor = previous;
    }
    edges.reverse();
    edges
}

/// The offending nodes, quoted up to the limit and counted past it.
fn quoted_list(nodes: &[&str]) -> String {
    let quoted = nodes
        .iter()
        .take(QUOTED_NODE_LIMIT)
        .map(|name| format!("'{name}'"))
        .collect::<Vec<_>>()
        .join(", ");
    match nodes.len().saturating_sub(QUOTED_NODE_LIMIT) {
        0 => quoted,
        remaining => format!("{quoted} (and {remaining} more)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::netlist_gen::extraction::extract;
    use crate::state::{NetLabel, Point, Wire};

    fn place<'a>(
        schematic: &'a mut SchematicState,
        kind: ComponentType,
        at: Point,
        name: &str,
    ) -> &'a mut Component {
        let id = schematic.add_component(kind, at);
        let component = schematic
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the placed component is retained");
        component.name = name.to_owned();
        component
    }

    fn port(schematic: &mut SchematicState, at: Point, name: &str, direction: &str) {
        let component = place(schematic, ComponentType::Port, at, name);
        component.value = name.to_owned();
        component.params = format!("dir={direction}");
    }

    fn wire(schematic: &mut SchematicState, from: Point, to: Point) {
        let id = schematic.wires.len() as u64 + 1;
        schematic.wires.push(Wire::segment(id, from, to));
    }

    /// A biased divider with a grounded source, plus two capacitor-isolated
    /// nodes: `FLOATB` reaches nothing, and `VDDA` is named by an interface
    /// port whose direction the caller chooses.
    fn isolated_nodes_design(supply_direction: &str) -> SchematicState {
        let mut schematic = SchematicState::default();
        place(&mut schematic, ComponentType::Ground, Point::new(0, 40), "");
        place(
            &mut schematic,
            ComponentType::VoltageSource,
            Point::new(0, 10),
            "V1",
        );
        place(
            &mut schematic,
            ComponentType::Resistor,
            Point::new(100, -10),
            "R1",
        );
        wire(&mut schematic, Point::new(0, -10), Point::new(80, -10));
        wire(&mut schematic, Point::new(120, -10), Point::new(120, 30));
        wire(&mut schematic, Point::new(120, 30), Point::new(0, 30));

        place(
            &mut schematic,
            ComponentType::Capacitor,
            Point::new(200, -10),
            "C1",
        );
        wire(&mut schematic, Point::new(120, -10), Point::new(180, -10));
        schematic
            .net_labels
            .push(NetLabel::new(1, Point::new(220, -10), "FLOATB"));

        place(
            &mut schematic,
            ComponentType::Capacitor,
            Point::new(200, 60),
            "C2",
        );
        wire(&mut schematic, Point::new(120, 60), Point::new(180, 60));
        wire(&mut schematic, Point::new(120, 30), Point::new(120, 60));
        port(
            &mut schematic,
            Point::new(230, 60),
            "VDDA",
            supply_direction,
        );
        schematic
    }

    fn blockers(schematic: &SchematicState) -> Vec<PreflightIssue> {
        topology_blockers(schematic, &extract(schematic, None))
    }

    /// A supply pin is a source, so the net it names reaches one. A node that
    /// only capacitors touch reaches nothing, and the two must not be reported
    /// the same way — a rail refused as floating would block every design that
    /// declares its supplies as pins.
    #[test]
    fn a_supply_only_net_reaches_a_source_while_an_undriven_net_does_not() {
        let issues = blockers(&isolated_nodes_design("supply"));

        assert_eq!(issues.len(), 1, "{issues:?}");
        assert!(issues[0].observed.contains("'FLOATB'"), "{issues:?}");
        assert!(
            !issues[0].observed.contains("VDDA"),
            "a supply pin feeds the net it names: {issues:?}"
        );

        // The same drawing with a signal pin instead: nothing feeds that net,
        // so it is reported exactly like the undriven one.
        let signal = blockers(&isolated_nodes_design("in"));
        assert_eq!(signal.len(), 1, "{signal:?}");
        assert!(signal[0].observed.contains("'VDDA'"), "{signal:?}");
        assert!(signal[0].observed.contains("'FLOATB'"), "{signal:?}");
    }

    /// The refusal states the rule the Solver surface publishes, names the
    /// node, and routes to the design checks that fix it. A sound topology
    /// produces no row at all.
    #[test]
    fn a_node_with_no_dc_path_blocks_the_run_by_name() {
        let issues = blockers(&isolated_nodes_design("supply"));

        assert_eq!(issues[0].check, NO_DC_PATH_CHECK);
        assert_eq!(issues[0].remediation, PreflightRemediation::DesignChecks);
        assert!(
            issues[0].required.contains("reference node"),
            "{:?}",
            issues[0]
        );

        let mut sound = SchematicState::default();
        place(&mut sound, ComponentType::Ground, Point::new(0, 40), "");
        place(
            &mut sound,
            ComponentType::VoltageSource,
            Point::new(0, 10),
            "V1",
        );
        place(
            &mut sound,
            ComponentType::Resistor,
            Point::new(100, -10),
            "R1",
        );
        wire(&mut sound, Point::new(0, -10), Point::new(80, -10));
        wire(&mut sound, Point::new(120, -10), Point::new(120, 30));
        wire(&mut sound, Point::new(120, 30), Point::new(0, 30));
        let sound_issues = blockers(&sound);
        assert!(sound_issues.is_empty(), "{sound_issues:?}");
    }

    /// Two sources across one node pair have no solution, and the refusal has
    /// to name both of them: naming only the one that closed the cycle would
    /// point at whichever instance happened to be drawn second.
    #[test]
    fn a_voltage_source_loop_blocks_with_every_source_named() {
        let mut schematic = SchematicState::default();
        place(&mut schematic, ComponentType::Ground, Point::new(0, 40), "");
        place(
            &mut schematic,
            ComponentType::VoltageSource,
            Point::new(0, 10),
            "V1",
        );
        place(
            &mut schematic,
            ComponentType::VoltageSource,
            Point::new(60, 10),
            "V2",
        );
        wire(&mut schematic, Point::new(0, -10), Point::new(60, -10));
        wire(&mut schematic, Point::new(0, 30), Point::new(60, 30));

        let issues = blockers(&schematic);

        let loops = issues
            .iter()
            .filter(|issue| issue.check == VOLTAGE_LOOP_CHECK)
            .collect::<Vec<_>>();
        assert_eq!(loops.len(), 1, "{issues:?}");
        assert!(loops[0].observed.contains("V1"), "{loops:?}");
        assert!(loops[0].observed.contains("V2"), "{loops:?}");
        assert_eq!(loops[0].remediation, PreflightRemediation::DesignChecks);

        // One source across the same pair is an ordinary drive, not a loop.
        schematic
            .components
            .retain(|component| component.name != "V2");
        assert!(
            !blockers(&schematic)
                .iter()
                .any(|issue| issue.check == VOLTAGE_LOOP_CHECK),
            "a single source states one voltage"
        );
    }

    /// Without a reference node every node is unreachable, and listing them
    /// all would bury the missing-ground finding that owns that state.
    #[test]
    fn a_design_with_no_reference_node_defers_to_the_missing_ground_finding() {
        let mut schematic = SchematicState::default();
        place(
            &mut schematic,
            ComponentType::Capacitor,
            Point::new(0, 0),
            "C1",
        );

        assert!(
            blockers(&schematic)
                .iter()
                .all(|issue| issue.check != NO_DC_PATH_CHECK)
        );
    }
}
