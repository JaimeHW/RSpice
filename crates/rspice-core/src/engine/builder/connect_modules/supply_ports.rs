//! The nets an authored connect module's supply ports are wired to.
//!
//! A supply port is a further `electrical` port beside the conversion pair
//! (`connectmodule d2a_sup(d, a, vsup)`). Its body reads `V(vsup)` as any
//! analog module reads a port, so binding the port to a net is the whole of
//! supply tracking: the solve does the rest, in every analysis.
//!
//! One precedence per port, highest first:
//!
//! 1. the connect statement (`connect d2a_sup #(.vsup(vdd));`), a deck net or
//!    `0` for ground;
//! 2. the bridged HDL port's attribute: `supplySensitivity` for the module's
//!    first supply port, `groundSensitivity` for a port whose
//!    `inh_conn_prop_name` names ground; its net is looked up in the X-card's
//!    scope, the enclosing subcircuit first and then the deck;
//! 3. the port's own `inh_conn_def_value`;
//! 4. the one constant rail that reaches the boundary, for the first supply
//!    port only;
//! 5. otherwise a refusal naming the module, the port, the boundary and the
//!    instance. A port needs a net, so no level is ever assumed.
//!
//! A port whose `inh_conn_prop_name` asks for a Cadence netSet property, with
//! no default and no binding above it, is refused naming the property: a
//! SPICE deck has no netSet to read it from.

use rspice_veriloga::connect::InsertionRule;

use super::super::boundary_supply::{BoundarySupplies, SupplyDerivation};
use super::{ParameterSource, PlannedConnectModule, connect_refusal};
use crate::{CircuitData, SimulationError};

/// One supply port as the selected declaration and its statement give it.
#[derive(Debug, Clone)]
pub(in crate::engine::builder) struct DeclaredSupplyPort {
    pub(in crate::engine::builder) name: String,
    /// The net the connect statement binds it to.
    statement: Option<String>,
    /// Its `inh_conn_prop_name`.
    property: Option<String>,
    /// The net its `inh_conn_def_value` names.
    default: Option<String>,
}

impl DeclaredSupplyPort {
    /// The supply ports of the module `rule` selects.
    pub(super) fn of_rule(rule: &InsertionRule) -> Vec<Self> {
        rule.supplies
            .iter()
            .map(|port| Self {
                name: port.name.to_string(),
                statement: rule.supply_port_net(&port.name).map(|net| net.to_string()),
                property: port.inherited_property.as_deref().map(str::to_owned),
                default: port.inherited_default.as_deref().map(str::to_owned),
            })
            .collect()
    }

    /// Whether `inh_conn_prop_name` names this port as the module's ground.
    fn is_ground(&self) -> bool {
        self.property.as_deref().is_some_and(|property| {
            let property = property.trim_end_matches('!').to_ascii_lowercase();
            matches!(property.as_str(), "gnd" | "vss" | "ground")
        })
    }
}

/// The net one supply port is wired to, and what bound it.
#[derive(Debug, Clone)]
pub(in crate::engine::builder) struct SupplyPortBinding {
    pub(in crate::engine::builder) port: String,
    /// The circuit node; `0` is ground.
    pub(in crate::engine::builder) node: usize,
    /// The circuit's own name for it.
    pub(in crate::engine::builder) net: String,
    /// Volts: the level of the rail that holds the net, else the deck's
    /// supply for the boundary. The record's nominal value; the body reads
    /// the net itself.
    pub(in crate::engine::builder) level: f64,
    pub(in crate::engine::builder) source: ParameterSource,
    /// The rail inferred for it, when inference bound it.
    pub(in crate::engine::builder) inferred: Option<SupplyDerivation>,
}

/// The boundary a module is inserted on: its node, the instance whose port it
/// bridges, and that port's supply attributes, when it is an HDL port.
pub(in crate::engine::builder) struct Boundary<'a> {
    pub(in crate::engine::builder) node_label: &'a str,
    pub(in crate::engine::builder) instance: &'a str,
    pub(in crate::engine::builder) port: &'a str,
    /// The deck's supply for the boundary, the nominal level of a net no
    /// rail holds.
    pub(in crate::engine::builder) level: f64,
}

/// Bind every supply port of `selected` at `boundary`.
pub(in crate::engine::builder) fn bind(
    circuit: &CircuitData,
    supplies: &BoundarySupplies,
    selected: &PlannedConnectModule,
    boundary: &Boundary<'_>,
) -> Result<Vec<SupplyPortBinding>, SimulationError> {
    let attributes = circuit
        .port_sensitivities
        .iter()
        .find(|port| port.instance == boundary.instance && port.port == boundary.port);
    let supply_port = selected
        .supply_ports
        .iter()
        .position(|port| !port.is_ground());
    let mut bindings = Vec::with_capacity(selected.supply_ports.len());
    for (index, port) in selected.supply_ports.iter().enumerate() {
        let attribute = attributes.and_then(|attributes| {
            if port.is_ground() {
                attributes
                    .ground
                    .as_deref()
                    .map(|net| (net, "groundSensitivity"))
            } else if Some(index) == supply_port {
                attributes
                    .supply
                    .as_deref()
                    .map(|net| (net, "supplySensitivity"))
            } else {
                None
            }
        });
        let (net, scoped, source) = if let Some(net) = &port.statement {
            (net.as_str(), false, ParameterSource::ConnectStatement)
        } else if let Some((net, name)) = attribute {
            (net, true, ParameterSource::PortAttribute(name.into()))
        } else if let Some(net) = &port.default {
            (net.as_str(), true, ParameterSource::ModuleDefault)
        } else if let Some(property) = &port.property {
            return Err(refusal(
                selected,
                boundary,
                format!(
                    "supply port '{}' takes its net from the inherited-connection property \
                     '{property}' (inh_conn_prop_name), which only a Cadence netSet sets, and \
                     RSpice does not read netSet properties. Give the port an \
                     inh_conn_def_value, bind it in the connect statement (connect {} #(.{}(<net>));), \
                     or give the HDL port a supplySensitivity attribute",
                    port.name, selected.name, port.name
                ),
            ));
        } else if let Some(SupplyDerivation::Derived {
            source,
            level,
            node,
        }) = (index == 0)
            .then(|| supplies.inferred_rail(boundary.node_label))
            .flatten()
        {
            let Some(node_id) = circuit.get_node_by_name(&node).filter(|id| *id > 0) else {
                return Err(missing_net(
                    selected,
                    boundary,
                    &port.name,
                    &node,
                    "inferred rail",
                ));
            };
            bindings.push(SupplyPortBinding {
                port: port.name.clone(),
                node: node_id,
                net: circuit.node_name_by_id(node_id).unwrap_or(&node).to_owned(),
                level,
                source: ParameterSource::Inferred {
                    source: source.clone(),
                    level,
                },
                inferred: Some(SupplyDerivation::Derived {
                    source,
                    level,
                    node,
                }),
            });
            continue;
        } else {
            return Err(refusal(
                selected,
                boundary,
                format!(
                    "supply port '{}' is bound to no net{}. Bind it in the connect statement \
                     (connect {} #(.{}(<net>));), with a supplySensitivity attribute on the HDL \
                     port it bridges, or with an inh_conn_def_value attribute on the port; a \
                     supply port needs a net, so no level is assumed",
                    port.name,
                    if index == 0 {
                        ", and no single constant supply rail reaches the boundary"
                    } else {
                        ""
                    },
                    selected.name,
                    port.name
                ),
            ));
        };
        let Some((node, label)) = resolve(circuit, boundary.instance, net, scoped) else {
            return Err(missing_net(
                selected,
                boundary,
                &port.name,
                net,
                &source.to_string(),
            ));
        };
        bindings.push(SupplyPortBinding {
            port: port.name.clone(),
            node,
            level: supplies.rail_level(&label).unwrap_or(boundary.level),
            net: label,
            source,
            inferred: None,
        });
    }
    Ok(bindings)
}

/// The net the bridged HDL port's `supplySensitivity` attribute names, for a
/// built-in converter at `node_label`: the circuit's own name for it, looked
/// up in `instance`'s enclosing subcircuit first. `None` when the port has no
/// such attribute. A net the circuit does not have, or ground, is refused
/// naming the instance, the port and the net.
pub(in crate::engine::builder) fn sensitivity_net(
    circuit: &CircuitData,
    node_label: &str,
    instance: &str,
    port: &str,
) -> Result<Option<String>, SimulationError> {
    let Some(net) = circuit
        .port_sensitivities
        .iter()
        .find(|attributes| attributes.instance == instance && attributes.port == port)
        .and_then(|attributes| attributes.supply.as_deref())
    else {
        return Ok(None);
    };
    match resolve(circuit, instance, net, true) {
        Some((node, label)) if node > 0 => Ok(Some(label)),
        _ => Err(connect_refusal(format!(
            "node '{node_label}' at port '{port}' of instance '{instance}' converts against the \
             net its supplySensitivity attribute names, '{net}', which is not a net of this \
             circuit (or is ground); name the net that powers the port"
        ))
        .instance(instance)
        .into()),
    }
}

/// The node `net` names: ground for `0` or `gnd`, else in `instance`'s
/// enclosing subcircuit when `scoped`, else in the deck.
fn resolve(
    circuit: &CircuitData,
    instance: &str,
    net: &str,
    scoped: bool,
) -> Option<(usize, String)> {
    if net == "0" || net.eq_ignore_ascii_case("gnd") {
        return Some((0, "0".into()));
    }
    let local = scoped
        .then(|| instance.rsplit_once('.'))
        .flatten()
        .and_then(|(scope, _)| circuit.get_node_by_name(&format!("{scope}.{net}")));
    let node = local.or_else(|| circuit.get_node_by_name(net))?;
    Some((
        node,
        circuit.node_name_by_id(node).unwrap_or(net).to_owned(),
    ))
}

fn refusal(
    selected: &PlannedConnectModule,
    boundary: &Boundary<'_>,
    detail: String,
) -> SimulationError {
    connect_refusal(format!(
        "node '{}' at port '{}' of instance '{}' selects connect module '{}' (instance '{}'), \
         whose {detail}",
        boundary.node_label, boundary.port, boundary.instance, selected.name, selected.instance
    ))
    .module(selected.name.as_str())
    .instance(boundary.instance)
    .into()
}

fn missing_net(
    selected: &PlannedConnectModule,
    boundary: &Boundary<'_>,
    port: &str,
    net: &str,
    source: &str,
) -> SimulationError {
    refusal(
        selected,
        boundary,
        format!(
            "supply port '{port}' is bound by its {source} to net '{net}', which is not a net of \
             this circuit"
        ),
    )
}
