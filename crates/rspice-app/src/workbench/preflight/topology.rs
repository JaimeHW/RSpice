//! The topology refusals the engine makes, decided before a run is queued.
//!
//! A node that no chain of DC-conducting elements ties to the reference has no
//! operating point the circuit determines. The engine raises that while
//! elaborating the deck — after the author has already committed to a run — so
//! preflight decides it here and names the offending nodes while the drawing is
//! still on screen.
//!
//! A cycle built only from voltage sources is not decided here. The design
//! rules own that fact (`SourceToSource`), across the hierarchy and without a
//! deck, and an error-level rule finding already blocks the run.
//!
//! Connectivity is read, never re-derived: the caller passes the design's one
//! [`ExtractedConnectivity`], the same graph the netlister and the design-rule
//! check read. A second tracing pass here could refuse a topology the emitted
//! deck does not have.

#[cfg(test)]
use crate::state::SchematicState;
use rspice_design::schematic::document::SchematicDocument;

use std::collections::HashMap;

use crate::state::{Component, ComponentType, PortDirection, is_ground_reference};
use crate::workbench::state::{PreflightIssue, PreflightRemediation};
use rspice_design::connectivity::ExtractedConnectivity;

/// Row label the Solver surface states as enforced contract. The refusal and
/// the statement of the rule use one vocabulary.
const NO_DC_PATH_CHECK: &str = "Nodes without a DC path";

/// How many offending nodes one refusal quotes before it counts the rest, as
/// the engine's own message does.
const QUOTED_NODE_LIMIT: usize = 8;

/// The control terminal of a field-effect device, which draws no DC current.
const CONTROL_GATE_PIN: &str = "G";

/// Every topology refusal this design carries, in reporting order.
pub(super) fn topology_blockers(
    schematic: &impl AsRef<SchematicDocument>,
    connectivity: &ExtractedConnectivity,
) -> Vec<PreflightIssue> {
    let schematic = schematic.as_ref();
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

    no_dc_path_issue(schematic, &nodes, &terminals)
        .into_iter()
        .collect()
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

    fn union(&mut self, left: usize, right: usize) {
        let (left, right) = (self.find(left), self.find(right));
        if left != right {
            self.parent[right] = left;
        }
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

/// The nodes no conducting chain ties to the reference, as one ordered
/// refusal.
///
/// A design with no reference node at all is not reported here: every node
/// would be listed, burying the missing-ground finding that owns that state.
fn no_dc_path_issue(
    schematic: &SchematicDocument,
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

fn join(sets: &mut DisjointSets, mut terminals: impl Iterator<Item = usize>) {
    let Some(first) = terminals.next() else {
        return;
    };
    for node in terminals {
        sets.union(first, node);
    }
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
    use crate::state::{NetLabel, Point, Wire};
    use rspice_design::connectivity::extract_with_hierarchy as extract;

    fn place<'a>(
        schematic: &'a mut SchematicState,
        kind: ComponentType,
        at: Point,
        name: &str,
    ) -> &'a mut Component {
        let id = schematic.add_component(kind, at);
        let component = schematic
            .document_mut_for_test()
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
        let id = schematic.document().wires.len() as u64 + 1;
        schematic
            .document_mut_for_test()
            .wires
            .push(Wire::segment(id, from, to));
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
            .document_mut_for_test()
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
