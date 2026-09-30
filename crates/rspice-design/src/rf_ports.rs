//! RF-port declarations shared by schematic inspection and simulation preparation.

use crate::connectivity::summary::{net_names_by_terminal, terminal_nets};
use crate::parameters::{carries_source_value as carries_value, parse_params_string};
use crate::schematic::{
    component::Component, component_type::ComponentType, document::SchematicDocument,
};
use std::collections::HashMap;

/// What a placed RF port does in the design it sits in.
///
/// One port element covers the whole span from a passive load to a
/// large-signal generator, and which of those a row is decides whether an
/// unread port is a finding or the ordinary state. The order is the netlist
/// generator's own precedence (`netlist_gen::instances`, the `RfPort` arm): a
/// power drive outranks an AC magnitude, which outranks a DC bias, and a port
/// carrying none of the three is a termination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RfPortMode {
    /// `PWR=` — an available-power generator behind Z0.
    PowerDrive,
    /// `AC` — a small-signal magnitude behind Z0.
    AcDrive,
    /// `DC` — a bias behind Z0, with no signal of its own.
    DcBias,
    /// No source spec at all: the port is a Z0 load.
    Termination,
}

impl RfPortMode {
    /// The word a column with room for one has to carry.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PowerDrive => "drive",
            Self::AcDrive => "AC drive",
            Self::DcBias => "DC bias",
            Self::Termination => "term",
        }
    }
}

/// Electrical declaration of one RF port in a schematic.
#[derive(Debug, Clone, PartialEq)]
pub struct RfPort {
    pub component_id: u64,
    pub reference: String,
    pub port_number: u32,
    pub z0: String,
    pub mode: RfPortMode,
    pub nets: Vec<String>,
}

/// One component read as an RF port. The caller has already established that
/// it is one.
pub fn rf_port(component: &Component, nets: &HashMap<(u64, String), String>) -> RfPort {
    let params = parse_params_string(&component.params);
    RfPort {
        component_id: component.id,
        reference: component.spice_instance_name(),
        port_number: port_number(&params),
        z0: reference_impedance(&params),
        mode: rf_port_mode(component, &params),
        nets: terminal_nets(component, nets),
    }
}

/// Resolve ports in matrix-index order, then by case-folded reference.
pub fn rf_ports(schematic: &impl AsRef<SchematicDocument>) -> Vec<RfPort> {
    if !schematic
        .as_ref()
        .components
        .iter()
        .any(|component| component.kind == ComponentType::RfPort)
    {
        return Vec::new();
    }
    let nets = net_names_by_terminal(schematic);
    let mut ports: Vec<RfPort> = schematic
        .as_ref()
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::RfPort)
        .map(|component| rf_port(component, &nets))
        .collect();
    ports.sort_by_key(|port| (port.port_number, port.reference.to_ascii_uppercase()));
    ports
}

/// The index an S-parameter run addresses this port by.
///
/// One, when the parameter is absent or holds something a port number cannot
/// be. That is the registry's own default and its own floor (the `port`
/// property is bounded 1–64), and a row reporting port 0 for a field the
/// property sheet will not accept would state a port the run can never
/// address.
fn port_number(params: &HashMap<String, String>) -> u32 {
    params
        .get("port")
        .and_then(|raw| rspice_app_types::quantity::parse_engineering_value(raw).ok())
        .filter(|number| *number >= 1.0)
        .map_or(1, |number| number.round() as u32)
}

/// The port's reference impedance, as the deck would carry it.
///
/// Re-formatted through the same parse-and-format path a source's key figure
/// takes, so `5e1` and `50` print identically. An impedance authored as an
/// expression parses as neither, and is echoed rather than dropped: the field
/// is what the port was given, and a blank cell would read as a port with no
/// reference impedance at all.
fn reference_impedance(params: &HashMap<String, String>) -> String {
    let raw = params
        .get("z0")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .unwrap_or("50");
    rspice_app_types::quantity::parse_engineering_value(raw)
        .map(rspice_app_types::property::format_engineering)
        .unwrap_or_else(|_| raw.to_owned())
}

/// What this port does in the design.
///
/// The precedence is the netlist generator's own (`netlist_gen::instances`,
/// the `RfPort` arm): `PWR` outranks `AC`, which outranks `DC`. The `dc`
/// parameter falls back to the component's value exactly as the emitted card
/// does, so a port biased through its value field is not reported as a
/// termination the deck then biases.
fn rf_port_mode(component: &Component, params: &HashMap<String, String>) -> RfPortMode {
    let param = |key: &str| params.get(key).map(String::as_str);
    if carries_value(param("pwr")) {
        RfPortMode::PowerDrive
    } else if carries_value(param("ac_mag")) {
        RfPortMode::AcDrive
    } else if carries_value(
        param("dc")
            .filter(|dc| !dc.trim().is_empty())
            .or(Some(component.value.as_str())),
    ) {
        RfPortMode::DcBias
    } else {
        RfPortMode::Termination
    }
}
