//! Interface ports — the pins a schematic cell presents to its parents.
//!
//! A `ComponentType::Port` placed in a schematic does two things at once:
//! it names the net its terminal touches (like a net label), and it declares
//! that net as a pin of the containing cell. The set of ports, in document
//! order, IS the cell's interface: it defines the `.SUBCKT` port list, the
//! node order of every `X` instance of the cell, and the pin layout of the
//! cell's generated symbol.

use serde::{Deserialize, Serialize};

use super::component::Component;
use super::component_type::ComponentType;
use super::state::SchematicState;

/// Electrical direction of an interface port.
///
/// Direction drives generated-symbol pin placement (inputs left, outputs
/// right, supplies top/bottom) and is advisory for netlisting — SPICE port
/// lists are positional and direction-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PortDirection {
    /// Signal input — generated symbols place these on the left edge.
    In,
    /// Signal output — right edge.
    Out,
    /// Bidirectional or unclassified — right edge, after outputs.
    #[default]
    InOut,
    /// Power/ground rail — top edge for the first, bottom for the second.
    Supply,
}

impl PortDirection {
    /// Parse the `dir=` instance parameter, tolerant of common synonyms.
    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "in" | "input" => PortDirection::In,
            "out" | "output" => PortDirection::Out,
            "supply" | "power" | "rail" | "global" => PortDirection::Supply,
            _ => PortDirection::InOut,
        }
    }

    /// Canonical keyword for the `dir=` parameter.
    pub fn keyword(&self) -> &'static str {
        match self {
            PortDirection::In => "in",
            PortDirection::Out => "out",
            PortDirection::InOut => "inout",
            PortDirection::Supply => "supply",
        }
    }
}

/// One pin of a cell's interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortSpec {
    /// Port (and net) name.
    pub name: String,
    /// Declared direction.
    pub direction: PortDirection,
}

impl Component {
    /// The interface pin this component declares, when it is a named port.
    ///
    /// The port's name lives in `value` (it doubles as the net name); the
    /// direction in the `dir=` entry of `params`. An unnamed port declares
    /// nothing — netlist generation reports it instead of guessing.
    pub fn port_spec(&self) -> Option<PortSpec> {
        if self.kind != ComponentType::Port {
            return None;
        }
        let name = self.value.trim();
        if name.is_empty() {
            return None;
        }
        let params = crate::properties::parse_params_string(&self.params);
        let direction = params
            .get("dir")
            .map(|raw| PortDirection::parse(raw))
            .unwrap_or_default();
        Some(PortSpec {
            name: name.to_string(),
            direction,
        })
    }
}

impl SchematicState {
    /// The cell's interface: named ports in document order.
    ///
    /// Document order is the contract — it defines `.SUBCKT` port order and
    /// the node order of every instance, so reordering components reorders
    /// the interface. Duplicate names collapse to their first occurrence
    /// (several port flags may pin the same net on different sheets/edges).
    pub fn interface_ports(&self) -> Vec<PortSpec> {
        let mut seen = std::collections::HashSet::new();
        self.components
            .iter()
            .filter_map(Component::port_spec)
            .filter(|spec| seen.insert(spec.name.to_ascii_lowercase()))
            .collect()
    }

    /// `true` when the schematic declares at least one interface port —
    /// i.e. it is a reusable cell rather than a top-level testbench.
    pub fn has_interface(&self) -> bool {
        self.components
            .iter()
            .any(|component| component.port_spec().is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    fn port(state: &mut SchematicState, name: &str, params: &str) -> u64 {
        let id = state.add_component(ComponentType::Port, Point::new(0, 0));
        let component = state
            .components
            .iter_mut()
            .find(|c| c.id == id)
            .expect("component exists");
        component.value = name.to_string();
        component.params = params.to_string();
        id
    }

    #[test]
    fn interface_follows_document_order_and_dedupes() {
        let mut state = SchematicState::default();
        port(&mut state, "inp", "dir=in");
        port(&mut state, "out", "dir=out");
        port(&mut state, "vdd", "dir=supply");
        port(&mut state, "INP", "dir=in"); // duplicate, different case

        let ports = state.interface_ports();
        let names: Vec<&str> = ports.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["inp", "out", "vdd"]);
        assert_eq!(ports[0].direction, PortDirection::In);
        assert_eq!(ports[2].direction, PortDirection::Supply);
    }

    #[test]
    fn unnamed_ports_declare_nothing() {
        let mut state = SchematicState::default();
        let id = state.add_component(ComponentType::Port, Point::new(0, 0));
        state
            .components
            .iter_mut()
            .find(|c| c.id == id)
            .expect("component exists")
            .value
            .clear();
        assert!(state.interface_ports().is_empty());
        assert!(!state.has_interface());
    }

    #[test]
    fn placed_ports_autoname_uniquely() {
        let mut state = SchematicState::default();
        state.add_component(ComponentType::Port, Point::new(0, 0));
        state.add_component(ComponentType::Port, Point::new(10, 0));
        let names: Vec<String> = state
            .components
            .iter()
            .map(|c| c.value.clone())
            .collect();
        assert_eq!(names.len(), 2);
        assert_ne!(names[0], names[1]);
        assert!(names.iter().all(|n| !n.is_empty()));
    }

    #[test]
    fn direction_parsing_tolerates_synonyms() {
        assert_eq!(PortDirection::parse("Input"), PortDirection::In);
        assert_eq!(PortDirection::parse("OUTPUT"), PortDirection::Out);
        assert_eq!(PortDirection::parse("power"), PortDirection::Supply);
        assert_eq!(PortDirection::parse("weird"), PortDirection::InOut);
    }

    /// A bound instance's electrical terminals follow the generated,
    /// direction-aware symbol layout, in interface order.
    #[test]
    fn bound_instance_terminals_follow_the_generated_layout() {
        use crate::state::LibraryCellInstance;

        let specs = vec![
            PortSpec { name: "inp".into(), direction: PortDirection::In },
            PortSpec { name: "inn".into(), direction: PortDirection::In },
            PortSpec { name: "out".into(), direction: PortDirection::Out },
            PortSpec { name: "vdd".into(), direction: PortDirection::Supply },
            PortSpec { name: "vss".into(), direction: PortDirection::Supply },
        ];
        let mut binding = LibraryCellInstance::new("work", "ota_5t", "schematic");
        binding.bind_interface(&specs);

        let mut state = SchematicState::default();
        let id = state.add_library_cell_component(Point::new(100, 100), binding);
        let component = state
            .components
            .iter()
            .find(|c| c.id == id)
            .expect("instance exists");

        let terminals = component.terminal_positions();
        assert_eq!(terminals.len(), 5);
        assert_eq!(terminals[0].1, Point::new(70, 90)); // inp — left, first
        assert_eq!(terminals[1].1, Point::new(70, 110)); // inn — left, second
        assert_eq!(terminals[2].1, Point::new(130, 100)); // out — right
        assert_eq!(terminals[3].1, Point::new(100, 80)); // vdd — top rail
        assert_eq!(terminals[4].1, Point::new(100, 120)); // vss — bottom rail
    }
}
