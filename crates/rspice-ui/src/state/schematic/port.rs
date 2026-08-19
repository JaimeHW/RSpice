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

/// Signal semantics declared by an interface port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PortSignalType {
    Logic,
    #[default]
    Analog,
    Power,
}

impl PortSignalType {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Logic => "logic",
            Self::Analog => "analog",
            Self::Power => "power",
        }
    }

    fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "logic" | "digital" => Self::Logic,
            "power" | "supply" | "rail" => Self::Power,
            _ => Self::Analog,
        }
    }
}

/// Physical or behavioral discipline carried by an interface port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PortDiscipline {
    #[default]
    Electrical,
    Logic,
    Wreal,
    Thermal,
}

impl PortDiscipline {
    pub const ALL: [Self; 4] = [Self::Electrical, Self::Logic, Self::Wreal, Self::Thermal];

    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Electrical => "electrical",
            Self::Logic => "logic",
            Self::Wreal => "wreal",
            Self::Thermal => "thermal",
        }
    }

    fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "logic" | "digital" => Self::Logic,
            "wreal" | "real" => Self::Wreal,
            "thermal" | "temperature" | "heat" => Self::Thermal,
            _ => Self::Electrical,
        }
    }
}

/// The four exact direction/type combinations specified by the workbench
/// mockup. Keeping the pair typed prevents a dialog index or translated label
/// from silently producing a different electrical contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PortDirectionType {
    #[default]
    InputLogic,
    InputAnalog,
    OutputAnalog,
    InOutPower,
}

impl PortDirectionType {
    pub const ALL: [Self; 4] = [
        Self::InputLogic,
        Self::InputAnalog,
        Self::OutputAnalog,
        Self::InOutPower,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::InputLogic => "input \u{00b7} logic",
            Self::InputAnalog => "input \u{00b7} analog",
            Self::OutputAnalog => "output \u{00b7} analog",
            Self::InOutPower => "inout \u{00b7} power",
        }
    }

    pub const fn direction(self) -> PortDirection {
        match self {
            Self::InputLogic | Self::InputAnalog => PortDirection::In,
            Self::OutputAnalog => PortDirection::Out,
            Self::InOutPower => PortDirection::InOut,
        }
    }

    pub const fn signal_type(self) -> PortSignalType {
        match self {
            Self::InputLogic => PortSignalType::Logic,
            Self::InputAnalog | Self::OutputAnalog => PortSignalType::Analog,
            Self::InOutPower => PortSignalType::Power,
        }
    }
}

/// Durable, typed interface metadata encoded in the component parameter
/// string. SPICE itself consumes the positional port list; hierarchy,
/// generated symbols, documentation, AMS tooling and future connect rules
/// consume this richer contract without maintaining a second source of truth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortContract {
    pub direction: PortDirection,
    pub signal_type: PortSignalType,
    pub discipline: PortDiscipline,
    /// One-based positional interface order. Legacy ports derive this from
    /// document order until they are rewritten through the typed editor.
    pub netlist_order: Option<usize>,
    pub documentation: String,
}

impl PortContract {
    fn from_component(component: &Component, name: &str) -> Self {
        let params = crate::state::parse_params_string(&component.params);
        let direction = params
            .get("dir")
            .map(|raw| PortDirection::parse(raw))
            .unwrap_or_default();
        let signal_type = params
            .get("signal_type")
            .map(|raw| PortSignalType::parse(raw))
            .unwrap_or_else(|| {
                if direction == PortDirection::Supply {
                    PortSignalType::Power
                } else {
                    PortSignalType::Analog
                }
            });
        let discipline = params
            .get("discipline")
            .map(|raw| PortDiscipline::parse(raw))
            .unwrap_or_default();
        let netlist_order = params
            .get("interface_order")
            .and_then(|raw| parse_interface_order(raw));
        let documentation = params.get("documentation").cloned().unwrap_or_else(|| {
            format!(
                "{name} {} {} interface port",
                direction.keyword(),
                discipline.keyword()
            )
        });
        Self {
            direction,
            signal_type,
            discipline,
            netlist_order,
            documentation,
        }
    }

    pub(crate) fn encoded_params(&self) -> String {
        let mut values = std::collections::HashMap::from([
            ("dir".to_owned(), self.direction.keyword().to_owned()),
            (
                "signal_type".to_owned(),
                self.signal_type.keyword().to_owned(),
            ),
            (
                "discipline".to_owned(),
                self.discipline.keyword().to_owned(),
            ),
            ("documentation".to_owned(), self.documentation.clone()),
        ]);
        if let Some(order) = self.netlist_order {
            values.insert("interface_order".to_owned(), order.to_string());
        }
        crate::state::format_params_string(&values)
    }
}

/// Read a durable interface position.
///
/// The numeric property editor writes a whole number as `2`, but a value that
/// arrived from a hand-edited file or a future schema may not be a position at
/// all. Anything that is not a positive whole number is no position, and the
/// port falls back to document order — `interface_order_is_well_formed` is what
/// stops that fallback from being silent on an edit.
fn parse_interface_order(raw: &str) -> Option<usize> {
    let value = raw.trim().parse::<f64>().ok()?;
    (value.is_finite() && value > 0.0 && value.fract() == 0.0).then_some(value as usize)
}

/// `true` when an `interface_order=` entry expresses either a position or the
/// explicit absence of one. Zero and empty both mean "follow document order".
fn interface_order_is_well_formed(raw: &str) -> bool {
    let trimmed = raw.trim();
    trimmed.is_empty()
        || trimmed
            .parse::<f64>()
            .is_ok_and(|value| value.is_finite() && value >= 0.0 && value.fract() == 0.0)
}

/// Validated one-shot configuration owned by the armed port tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingPortPlacement {
    pub name: String,
    pub contract: PortContract,
    pub expected_topology_version: u64,
    pub expected_netlist_order: usize,
    pub document_authority: Option<PortPlacementAuthority>,
}

/// Application-document authority captured when a validated port draft arms
/// the one-shot canvas tool. Schematic-only callers can leave it absent, but
/// the interactive placement boundary requires an exact match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortPlacementAuthority {
    pub design_execution_epoch: u64,
    pub active_schematic_epoch: u64,
    pub view_path: String,
}

impl PendingPortPlacement {
    pub fn new(
        name: impl Into<String>,
        direction_type: PortDirectionType,
        discipline: PortDiscipline,
        expected_topology_version: u64,
        expected_netlist_order: usize,
    ) -> Self {
        let name = name.into();
        let contract = PortContract {
            direction: direction_type.direction(),
            signal_type: direction_type.signal_type(),
            discipline,
            netlist_order: Some(expected_netlist_order),
            documentation: format!(
                "{name} {} {} interface port",
                direction_type.label(),
                discipline.keyword()
            ),
        };
        Self {
            name,
            contract,
            expected_topology_version,
            expected_netlist_order,
            document_authority: None,
        }
    }

    pub fn with_document_authority(
        mut self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: impl Into<String>,
    ) -> Self {
        self.document_authority = Some(PortPlacementAuthority {
            design_execution_epoch,
            active_schematic_epoch,
            view_path: view_path.into(),
        });
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortPlacementError {
    ReadOnly,
    EmptyName,
    NameTooLong,
    InvalidName(&'static str),
    /// A name the engine reads as node `0`, or a global net. Neither can be a
    /// pin of one cell.
    ReservedGroundName {
        name: String,
        reason: &'static str,
    },
    DuplicateName(String),
    InvalidContract(&'static str),
    StaleTopology,
    StaleOrder,
}

impl std::fmt::Display for PortPlacementError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadOnly => formatter.write_str("the active schematic is read-only"),
            Self::EmptyName => formatter.write_str("enter a port name"),
            Self::NameTooLong => formatter.write_str("port names are limited to 128 characters"),
            Self::InvalidName(reason) => write!(formatter, "port name: {reason}"),
            Self::ReservedGroundName { name, reason } => {
                write!(formatter, "port name `{name}` is {reason}")
            }
            Self::DuplicateName(name) => {
                write!(formatter, "an interface port named '{name}' already exists")
            }
            Self::InvalidContract(reason) => write!(formatter, "interface contract: {reason}"),
            Self::StaleTopology => formatter.write_str(
                "the schematic changed after this port contract was armed; reopen the dialog",
            ),
            Self::StaleOrder => formatter.write_str(
                "the interface order changed after this port contract was armed; reopen the dialog",
            ),
        }
    }
}

/// One pin of a cell's interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortSpec {
    /// Port (and net) name.
    pub name: String,
    /// Declared direction.
    pub direction: PortDirection,
}

impl PortSpec {
    /// The vector this pin declares, when its name declares one.
    ///
    /// The name is the declaration — see [`super::bus::declared_vector`] — so a
    /// port drawn `DATA[7:0]` is one interface pin carrying eight conductors,
    /// and no second field can drift away from the name the drawing shows.
    pub fn vector(&self) -> Option<super::BusDeclaration> {
        super::declared_vector(&self.name)
    }

    /// Conductors this pin carries: the declared width, or one.
    pub fn width(&self) -> usize {
        super::declared_width(&self.name)
    }
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
        let direction = PortContract::from_component(self, name).direction;
        Some(PortSpec {
            name: name.to_string(),
            direction,
        })
    }

    /// Typed interface metadata for a named port.
    pub fn port_contract(&self) -> Option<PortContract> {
        let spec = self.port_spec()?;
        Some(PortContract::from_component(self, &spec.name))
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
        let mut ports = self
            .components
            .iter()
            .enumerate()
            .filter_map(|(document_index, component)| {
                let spec = component.port_spec()?;
                let order = component
                    .port_contract()
                    .and_then(|contract| contract.netlist_order)
                    .unwrap_or(document_index + 1);
                Some((order, document_index, spec))
            })
            .filter(|(_, _, spec)| seen.insert(spec.name.to_ascii_lowercase()))
            .collect::<Vec<_>>();
        ports.sort_by_key(|(order, document_index, _)| (*order, *document_index));
        ports.into_iter().map(|(_, _, spec)| spec).collect()
    }

    /// `true` when the schematic declares at least one interface port —
    /// i.e. it is a reusable cell rather than a top-level testbench.
    pub fn has_interface(&self) -> bool {
        self.components
            .iter()
            .any(|component| component.port_spec().is_some())
    }

    pub fn next_interface_order(&self) -> usize {
        self.components
            .iter()
            .enumerate()
            .filter_map(|(document_index, component)| {
                component.port_spec()?;
                Some(
                    component
                        .port_contract()
                        .and_then(|contract| contract.netlist_order)
                        .unwrap_or(document_index + 1),
                )
            })
            .max()
            .unwrap_or(0)
            + 1
    }

    pub fn suggested_port_name(&self, base: &str) -> String {
        let trimmed = base.trim();
        let mut stem = trimmed.chars().take(128).collect::<String>();
        if validate_port_name_syntax(&stem, self.document_policy.net_naming).is_err() {
            stem = "PORT".to_owned();
        }
        if self.validate_new_port_name(&stem).is_ok() {
            return stem;
        }
        // A vector pin's name IS its declaration, so a uniquifying suffix goes
        // on the base and the declared range is carried through untouched — a
        // suffix appended after the range would name nothing the interface can
        // declare.
        let (head, range) = match super::declared_vector(&stem) {
            Some(declaration) => {
                let range = stem[declaration.name.len()..].to_owned();
                (declaration.name, range)
            }
            None => (stem, String::new()),
        };
        (2..)
            .map(|index| {
                let suffix = format!("_{index}");
                let head_budget = 128usize
                    .saturating_sub(suffix.chars().count())
                    .saturating_sub(range.chars().count());
                format!(
                    "{}{suffix}{range}",
                    head.chars().take(head_budget).collect::<String>()
                )
            })
            .find(|candidate| self.validate_new_port_name(candidate).is_ok())
            .expect("the valid bounded stem has an unbounded numeric suffix space")
    }

    /// Validate the exact name accepted by the dialog and netlister.
    pub fn validate_new_port_name(&self, name: &str) -> Result<(), PortPlacementError> {
        self.validate_port_name_except(name, None)
    }

    /// Validate an edited port name while excluding the component being
    /// edited from the case-insensitive uniqueness check.
    pub fn validate_edited_port_name(
        &self,
        component_id: u64,
        name: &str,
    ) -> Result<(), PortPlacementError> {
        self.validate_port_name_except(name, Some(component_id))
    }

    fn validate_port_name_except(
        &self,
        name: &str,
        excluded_component_id: Option<u64>,
    ) -> Result<(), PortPlacementError> {
        let name = name.trim();
        validate_port_name_syntax(name, self.document_policy.net_naming)?;
        // A net LABEL may name the ground net; an interface port may not. The
        // port list is the cell's contract, and a formal named `0` is illegal
        // in every dialect while one named `GND` silently shorts to global
        // ground under ngspice.
        if let Some(reason) = super::ground_names::reserved_ground_name(name) {
            return Err(PortPlacementError::ReservedGroundName {
                name: name.to_owned(),
                reason,
            });
        }
        if self.components.iter().any(|component| {
            Some(component.id) != excluded_component_id
                && component
                    .port_spec()
                    .is_some_and(|port| port.name.eq_ignore_ascii_case(name))
        }) {
            return Err(PortPlacementError::DuplicateName(name.to_owned()));
        }
        Ok(())
    }

    pub fn validate_pending_port(
        &self,
        pending: &PendingPortPlacement,
    ) -> Result<(), PortPlacementError> {
        if self.read_only {
            return Err(PortPlacementError::ReadOnly);
        }
        self.validate_new_port_name(&pending.name)?;
        validate_contract_fields(&pending.contract)?;
        if pending.contract.netlist_order != Some(pending.expected_netlist_order) {
            return Err(PortPlacementError::InvalidContract(
                "the durable netlist order does not match the armed placement",
            ));
        }
        if self.topology_version() != pending.expected_topology_version {
            return Err(PortPlacementError::StaleTopology);
        }
        if self.next_interface_order() != pending.expected_netlist_order {
            return Err(PortPlacementError::StaleOrder);
        }
        Ok(())
    }

    /// Validate a complete edited port candidate against the live interface,
    /// excluding its own stable object from uniqueness checks.
    pub fn validate_edited_port_contract(
        &self,
        component_id: u64,
        candidate: &Component,
    ) -> Result<(), PortPlacementError> {
        if candidate.kind != ComponentType::Port {
            return Err(PortPlacementError::InvalidContract(
                "the edited object is not an interface port",
            ));
        }
        self.validate_edited_port_name(component_id, candidate.value.trim())?;
        let encoded = crate::state::parse_params_string(&candidate.params);
        if encoded
            .get("documentation")
            .is_none_or(|documentation| documentation.trim().is_empty())
        {
            return Err(PortPlacementError::InvalidContract(
                "documentation is empty",
            ));
        }
        if encoded
            .get("interface_order")
            .is_some_and(|raw| !interface_order_is_well_formed(raw))
        {
            return Err(PortPlacementError::InvalidContract(
                "interface order must be a whole number",
            ));
        }
        let contract = candidate
            .port_contract()
            .ok_or(PortPlacementError::EmptyName)?;
        validate_contract_fields(&contract)?;
        if let Some(order) = contract.netlist_order
            && self.components.iter().any(|component| {
                component.id != component_id
                    && component
                        .port_contract()
                        .and_then(|other| other.netlist_order)
                        == Some(order)
            })
        {
            return Err(PortPlacementError::InvalidContract(
                "netlist order conflicts with another interface port",
            ));
        }
        Ok(())
    }

    /// Place one named typed port as a single undoable document transaction.
    pub fn place_pending_port(
        &mut self,
        pos: super::Point,
        pending: PendingPortPlacement,
    ) -> Result<u64, PortPlacementError> {
        self.validate_pending_port(&pending)?;
        let mut placed_id = None;
        let name = pending.name.trim().to_owned();
        let params = pending.contract.encoded_params();
        let changed = self.with_undo("place interface port", |schematic| {
            let id = schematic.add_component(ComponentType::Port, pos);
            let component = schematic
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .expect("newly allocated port exists");
            component.value.clone_from(&name);
            component.params.clone_from(&params);
            placed_id = Some(id);
        });
        if !changed {
            return Err(PortPlacementError::ReadOnly);
        }
        Ok(placed_id.expect("a changed placement records its stable identifier"))
    }
}

/// The one syntax rule for an interface pin's name.
///
/// A pin carries either one conductor or a declared vector, and the name says
/// which: `DATA[7:0]` parses as a declaration through
/// [`super::BusDeclaration::parse`] — the single authority for range syntax —
/// and a name with no bus delimiters is one conductor. Nothing between the two
/// is a pin. `DATA[3]` selects one member of a bus, and the bus is the pin
/// rather than the bit; the deck cannot carry that spelling either, because a
/// probe written `V(DATA[3])` reaches the engine as `v(data3)`.
fn validate_port_name_syntax(
    name: &str,
    policy: super::NetNamingPolicy,
) -> Result<(), PortPlacementError> {
    if name.is_empty() {
        return Err(PortPlacementError::EmptyName);
    }
    if name.chars().count() > 128 {
        return Err(PortPlacementError::NameTooLong);
    }
    super::NetLabel::validate_name(name, policy).map_err(PortPlacementError::InvalidName)?;
    if name.contains(['[', ']', '<', '>']) && super::declared_vector(name).is_none() {
        return Err(PortPlacementError::InvalidName(
            "a pin carries one conductor or a declared range such as DATA[7:0]; \
             a single member such as DATA[3] is a bit of a bus, not a pin",
        ));
    }
    Ok(())
}

fn validate_contract_fields(contract: &PortContract) -> Result<(), PortPlacementError> {
    let valid_direction_type = matches!(
        (contract.direction, contract.signal_type),
        (PortDirection::In, PortSignalType::Logic)
            | (PortDirection::In, PortSignalType::Analog)
            | (PortDirection::Out, PortSignalType::Analog)
            | (PortDirection::InOut, PortSignalType::Analog)
            | (PortDirection::InOut, PortSignalType::Power)
            | (PortDirection::Supply, PortSignalType::Power)
    );
    if !valid_direction_type {
        return Err(PortPlacementError::InvalidContract(
            "direction and signal type are not one of the supported typed combinations",
        ));
    }
    if contract.documentation.trim().is_empty() {
        return Err(PortPlacementError::InvalidContract(
            "documentation is empty",
        ));
    }
    Ok(())
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
    fn a_ports_name_declares_how_many_conductors_it_carries() {
        let mut state = SchematicState::default();
        port(&mut state, "DATA[3:0]", "dir=inout");
        port(&mut state, "EN", "dir=in");

        let ports = state.interface_ports();
        // The contract still has one entry per drawn pin: a vector is one pin
        // of the interface, and expanding it is the deck's business.
        assert_eq!(ports.len(), 2);
        assert_eq!(
            ports[0].vector().map(|declaration| declaration.width()),
            Some(4)
        );
        assert_eq!(ports[0].width(), 4);
        assert!(ports[1].vector().is_none());
        assert_eq!(ports[1].width(), 1);
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
        let names: Vec<String> = state.components.iter().map(|c| c.value.clone()).collect();
        assert_eq!(names.len(), 2);
        assert_ne!(names[0], names[1]);
        assert!(names.iter().all(|n| !n.is_empty()));
    }

    #[test]
    fn suggested_name_is_valid_and_bounded_for_invalid_or_max_length_bases() {
        let mut state = SchematicState::default();
        port(&mut state, "PORT", "dir=in");
        assert_eq!(state.suggested_port_name("invalid name"), "PORT_2");

        let long = "A".repeat(128);
        port(&mut state, &long, "dir=in");
        let suggested = state.suggested_port_name(&long);
        assert_eq!(suggested.chars().count(), 128);
        assert!(suggested.ends_with("_2"));
        assert!(state.validate_new_port_name(&suggested).is_ok());
    }

    #[test]
    fn a_port_cannot_be_named_ground() {
        let mut state = SchematicState::default();
        for reserved in ["0", "gnd", "GND!", "Ground", "vdd!"] {
            let error = state
                .validate_new_port_name(reserved)
                .expect_err("a ground alias or global net is not an interface pin");
            assert!(
                matches!(&error, PortPlacementError::ReservedGroundName { name, .. } if name == reserved),
                "`{reserved}` was rejected as {error:?}"
            );
            assert!(
                error.to_string().contains(reserved),
                "the message names the port: {error}"
            );
        }
        // Ordinary supply pins keep working: no dialect ties them to node 0.
        for supply in ["GNDA", "VSS", "AGND", "VEE"] {
            assert_eq!(state.validate_new_port_name(supply), Ok(()));
        }

        // Every placement and rename path funnels through the same rejection.
        let pending = PendingPortPlacement::new(
            "GND",
            PortDirectionType::InOutPower,
            PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        assert!(matches!(
            state.place_pending_port(Point::origin(), pending),
            Err(PortPlacementError::ReservedGroundName { .. })
        ));
        assert!(state.components.is_empty());

        let placed_id = port(&mut state, "BIAS", "dir=in");
        assert!(matches!(
            state.validate_edited_port_name(placed_id, "GROUND"),
            Err(PortPlacementError::ReservedGroundName { .. })
        ));

        // A net label may still mark the ground net.
        assert!(
            crate::state::NetLabel::validate_name("0", state.document_policy.net_naming).is_ok()
        );
    }

    /// The pin-name rule, at every gate that admits a name: a conductor or a
    /// declared range, never one member of one.
    #[test]
    fn a_pin_name_is_a_conductor_or_a_declared_range_and_never_one_member() {
        let mut state = SchematicState::default();
        for pin in ["DATA[7:0]", "ADDR<0:3>", "EN", "bias_1"] {
            assert_eq!(state.validate_new_port_name(pin), Ok(()), "{pin}");
        }
        for member in ["DATA[3]", "DATA<3>", "DATA[7:0]_2", "DATA[]", "DATA[3:3]"] {
            let error = state
                .validate_new_port_name(member)
                .expect_err("one bus member is not a pin");
            assert!(
                matches!(error, PortPlacementError::InvalidName(_)),
                "`{member}` was rejected as {error:?}"
            );
            // The message states the rule rather than only the verdict.
            assert!(
                error.to_string().contains("DATA[7:0]"),
                "the message states the rule: {error}"
            );
        }

        // The same rule guards a rename and an armed placement.
        let placed = port(&mut state, "DATA[7:0]", "dir=inout");
        assert!(matches!(
            state.validate_edited_port_name(placed, "DATA[3]"),
            Err(PortPlacementError::InvalidName(_))
        ));
        let pending = PendingPortPlacement::new(
            "DATA[3]",
            PortDirectionType::InOutPower,
            PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        assert!(matches!(
            state.place_pending_port(Point::origin(), pending),
            Err(PortPlacementError::InvalidName(_))
        ));

        // Autonaming stays inside the rule: the suffix lands on the base so the
        // declared range survives, and the candidate it produces is accepted.
        let suggested = state.suggested_port_name("DATA[7:0]");
        assert_eq!(suggested, "DATA_2[7:0]");
        assert_eq!(state.validate_new_port_name(&suggested), Ok(()));
        assert_eq!(
            PortSpec {
                name: suggested,
                direction: PortDirection::InOut,
            }
            .width(),
            8
        );
    }

    #[test]
    fn direction_parsing_tolerates_synonyms() {
        assert_eq!(PortDirection::parse("Input"), PortDirection::In);
        assert_eq!(PortDirection::parse("OUTPUT"), PortDirection::Out);
        assert_eq!(PortDirection::parse("power"), PortDirection::Supply);
        assert_eq!(PortDirection::parse("weird"), PortDirection::InOut);
    }

    #[test]
    fn every_typed_contract_places_and_round_trips_losslessly() {
        for direction_type in PortDirectionType::ALL {
            for discipline in PortDiscipline::ALL {
                let mut state = SchematicState::default();
                let pending = PendingPortPlacement::new(
                    "PORT_A",
                    direction_type,
                    discipline,
                    state.topology_version(),
                    state.next_interface_order(),
                );
                let expected = pending.contract.clone();
                let stable_id = state
                    .place_pending_port(Point::new(10, 20), pending)
                    .expect("valid typed contract places atomically");
                let placed = state
                    .components
                    .iter()
                    .find(|component| component.id == stable_id)
                    .expect("stable identity is retained");
                assert_eq!(placed.port_contract(), Some(expected.clone()));

                let encoded = serde_json::to_string(&state).expect("schematic serializes");
                let restored: SchematicState =
                    serde_json::from_str(&encoded).expect("schematic restores");
                let restored_port = restored
                    .components
                    .iter()
                    .find(|component| component.id == stable_id)
                    .expect("stable identity survives persistence");
                assert_eq!(restored_port.port_contract(), Some(expected));
                assert!(restored.pending_port.is_none());
            }
        }
    }

    #[test]
    fn typed_interface_order_survives_component_storage_reordering() {
        let mut state = SchematicState::default();
        for name in ["IN", "OUT"] {
            let pending = PendingPortPlacement::new(
                name,
                PortDirectionType::InputAnalog,
                PortDiscipline::Electrical,
                state.topology_version(),
                state.next_interface_order(),
            );
            state
                .place_pending_port(Point::origin(), pending)
                .expect("port places");
        }
        state.components.reverse();

        let names = state
            .interface_ports()
            .into_iter()
            .map(|port| port.name)
            .collect::<Vec<_>>();
        assert_eq!(names, ["IN", "OUT"]);
        assert_eq!(state.next_interface_order(), 3);
    }

    #[test]
    fn duplicate_and_stale_pending_ports_make_no_document_change() {
        let mut state = SchematicState::default();
        port(&mut state, "BIAS_EN", "dir=in");
        let baseline = state.components.clone();
        let duplicate = PendingPortPlacement::new(
            "bias_en",
            PortDirectionType::InputLogic,
            PortDiscipline::Logic,
            state.topology_version(),
            state.next_interface_order(),
        );
        assert!(matches!(
            state.place_pending_port(Point::origin(), duplicate),
            Err(PortPlacementError::DuplicateName(_))
        ));
        assert_eq!(state.components, baseline);

        let stale = PendingPortPlacement::new(
            "UNIQUE",
            PortDirectionType::OutputAnalog,
            PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        state.bump_topology_version();
        assert_eq!(
            state.place_pending_port(Point::origin(), stale),
            Err(PortPlacementError::StaleTopology)
        );
        assert_eq!(state.components, baseline);

        let mut malformed = PendingPortPlacement::new(
            "UNIQUE",
            PortDirectionType::OutputAnalog,
            PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        malformed.contract.signal_type = PortSignalType::Logic;
        assert!(matches!(
            state.place_pending_port(Point::origin(), malformed),
            Err(PortPlacementError::InvalidContract(_))
        ));
        assert_eq!(state.components, baseline);
    }

    #[test]
    fn edited_contract_rejects_invalid_type_pair_empty_docs_and_duplicate_order() {
        let mut state = SchematicState::default();
        let first = PendingPortPlacement::new(
            "IN",
            PortDirectionType::InputAnalog,
            PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        let first_id = state
            .place_pending_port(Point::origin(), first)
            .expect("first port places");
        let second = PendingPortPlacement::new(
            "OUT",
            PortDirectionType::OutputAnalog,
            PortDiscipline::Electrical,
            state.topology_version(),
            state.next_interface_order(),
        );
        let second_id = state
            .place_pending_port(Point::origin(), second)
            .expect("second port places");
        let baseline = state
            .components
            .iter()
            .find(|component| component.id == second_id)
            .expect("second port exists")
            .clone();

        let rewrite = |component: &Component, key: &str, value: String| {
            let mut candidate = component.clone();
            let mut params = crate::state::parse_params_string(&candidate.params);
            params.insert(key.to_owned(), value);
            candidate.params = crate::state::format_params_string(&params);
            candidate
        };
        let invalid_pair = rewrite(&baseline, "signal_type", "logic".to_owned());
        assert!(matches!(
            state.validate_edited_port_contract(second_id, &invalid_pair),
            Err(PortPlacementError::InvalidContract(_))
        ));

        let empty_docs = rewrite(&baseline, "documentation", String::new());
        assert!(matches!(
            state.validate_edited_port_contract(second_id, &empty_docs),
            Err(PortPlacementError::InvalidContract(_))
        ));

        let first_order = state
            .components
            .iter()
            .find(|component| component.id == first_id)
            .and_then(Component::port_contract)
            .and_then(|contract| contract.netlist_order)
            .expect("first order");
        let duplicate_order = rewrite(&baseline, "interface_order", first_order.to_string());
        assert!(matches!(
            state.validate_edited_port_contract(second_id, &duplicate_order),
            Err(PortPlacementError::InvalidContract(_))
        ));
    }

    /// A bound instance's electrical terminals follow the generated,
    /// direction-aware symbol layout, in interface order.
    #[test]
    fn bound_instance_terminals_follow_the_generated_layout() {
        use crate::state::LibraryCellInstance;

        let specs = vec![
            PortSpec {
                name: "inp".into(),
                direction: PortDirection::In,
            },
            PortSpec {
                name: "inn".into(),
                direction: PortDirection::In,
            },
            PortSpec {
                name: "out".into(),
                direction: PortDirection::Out,
            },
            PortSpec {
                name: "vdd".into(),
                direction: PortDirection::Supply,
            },
            PortSpec {
                name: "vss".into(),
                direction: PortDirection::Supply,
            },
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
