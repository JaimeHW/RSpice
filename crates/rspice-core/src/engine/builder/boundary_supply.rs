//! Which supply a mixed boundary converts against.
//!
//! Both boundary routes need one number: the level a discrete `1` reaches on
//! an analog node, and the level whose half is the threshold a discrete input
//! compares against. The deck can state it (`.param vcc`, or the parameter
//! `.options auto_bridge_parm_d` renames), a `connectrules` block can state it
//! for one boundary (`vsup`), and otherwise it has to be derived.
//!
//! # Why a derived supply is said out loud
//!
//! It used to be 3.3 V whenever the deck did not say, whatever the deck's
//! rails were. A 1 V core and a 5 V I/O ring both converted at 3.3 V, and
//! nothing in the run said so: the number was not so much wrong as unrelated
//! to the circuit. So the rail that reaches the boundary is used when there is
//! exactly one, with a warning naming it and the parameter that would make it
//! explicit; 3.3 V survives as the answer when no rail reaches the boundary at
//! all, also with a warning; and two rails is a refusal, because picking one
//! of two supplies on the author's behalf is the silent default with extra
//! steps.
//!
//! # One resolver, both boundaries
//!
//! The XSPICE auto-bridge planner and the Verilog-AMS mixed host ask the same
//! question of the same deck, so they ask it here. A deck carrying both gets
//! one answer per *net*, not one per route: the supply is a property of where
//! the net sits in the circuit, which is why the reachability walk is over the
//! netlist rather than over either route's own vocabulary.
//!
//! What counts as a rail and what counts as reaching are stated by
//! [`crate::netlist::analyze_supply_reachability`], which owns both rules.

use std::cell::RefCell;
use std::collections::BTreeSet;

use crate::SimulationError;
use crate::netlist::{Element, Netlist, analyze_supply_reachability};

/// The level a boundary converts against when the deck names no supply and no
/// rail reaches it.
///
/// This is the number every boundary used to get unconditionally, kept as the
/// answer for the case it was always right for: a deck with no power supply in
/// it at all, which is most small mixed-signal test decks.
pub(super) const DEFAULT_BOUNDARY_SUPPLY: crate::Value = 3.3;

/// Two rails are the same supply when their levels agree this closely,
/// relative to the larger of the two.
///
/// The tolerance exists for rails written as expressions that evaluate to the
/// same volt through different arithmetic, not to merge nearby supplies: 1.8 V
/// and 1.85 V are two supplies and the refusal should say so.
const SUPPLY_LEVEL_TOLERANCE: crate::Value = 1e-9;

/// Where one boundary net's supply came from.
#[derive(Debug, Clone)]
pub(super) enum SupplyDerivation {
    /// The deck named it: `.param vcc`, the parameter
    /// `.options auto_bridge_parm_d` renames, or a subcircuit-scoped value for
    /// this net. Authoritative and silent, as it has always been.
    Declared,
    /// Exactly one rail reaches this net. The boundary binds to the rail's
    /// node live: see [`live_supply_net`].
    Derived {
        source: String,
        level: crate::Value,
        node: String,
    },
    /// No rail reaches this net.
    Defaulted,
    /// More than one rail reaches this net, at different levels.
    Ambiguous { rails: Vec<(String, crate::Value)> },
}

/// One boundary net's supply, and where it came from.
#[derive(Debug, Clone)]
pub(super) struct ResolvedSupply {
    pub(super) level: crate::Value,
    pub(super) derivation: SupplyDerivation,
}

/// The per-design answer to "what supply does this boundary net convert
/// against", shared by both boundary routes.
pub(super) struct BoundarySupplies {
    declared: Option<crate::Value>,
    parameter: String,
    reachability: crate::netlist::SupplyReachability,
    /// Nets already spoken about, so a design with one boundary net and two
    /// bridges on it says its sentence once.
    reported: RefCell<BTreeSet<String>>,
}

impl BoundarySupplies {
    pub(super) fn new(netlist: &Netlist, flat_elements: &[Element]) -> Self {
        let parameter = netlist
            .options
            .auto_bridge_param_name("d")
            .unwrap_or("vcc")
            .to_string();
        let declared = netlist
            .params
            .get(&parameter)
            .filter(|value| value.is_finite());
        Self {
            declared,
            parameter,
            reachability: analyze_supply_reachability(flat_elements),
            reported: RefCell::new(BTreeSet::new()),
        }
    }

    /// One boundary net's supply.
    ///
    /// `scoped` is a value the deck stated for this net specifically -- the
    /// subcircuit-scoped `vcc` the flattener records for a hierarchical
    /// instance -- and outranks the design-wide parameter, as it always has.
    pub(super) fn resolve(&self, node_label: &str, scoped: Option<crate::Value>) -> ResolvedSupply {
        if let Some(level) = scoped.filter(|value| value.is_finite()).or(self.declared) {
            return ResolvedSupply {
                level,
                derivation: SupplyDerivation::Declared,
            };
        }

        let mut distinct: Vec<&crate::netlist::SupplyRail> = Vec::new();
        for rail in self.reachability.rails_reaching(node_label) {
            if !distinct
                .iter()
                .any(|kept| same_level(kept.level, rail.level))
            {
                distinct.push(rail);
            }
        }

        match distinct.as_slice() {
            [] => ResolvedSupply {
                level: DEFAULT_BOUNDARY_SUPPLY,
                derivation: SupplyDerivation::Defaulted,
            },
            [only] => ResolvedSupply {
                level: only.level,
                derivation: SupplyDerivation::Derived {
                    source: only.source.clone(),
                    level: only.level,
                    node: only.node.clone(),
                },
            },
            many => ResolvedSupply {
                level: DEFAULT_BOUNDARY_SUPPLY,
                derivation: SupplyDerivation::Ambiguous {
                    rails: many
                        .iter()
                        .map(|rail| (rail.source.clone(), rail.level))
                        .collect(),
                },
            },
        }
    }

    /// Say what a boundary net's supply was derived from, or refuse to guess.
    ///
    /// Call this only where the derived number is actually about to be used: a
    /// boundary whose `connectrules` block states a `vsup` takes that instead,
    /// and warning about a rail nothing consulted would be noise.
    pub(super) fn report(
        &self,
        node_label: &str,
        derivation: &SupplyDerivation,
        live: bool,
    ) -> Result<(), SimulationError> {
        if let SupplyDerivation::Ambiguous { rails } = derivation {
            let named = rails
                .iter()
                .map(|(source, level)| format!("{source} = {level} V"))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(SimulationError::Circuit(format!(
                "boundary net '{node_label}' is reached by {} supply rails at different levels \
                 ({named}); RSpice will not choose one of them for you. Set '.param {}=<volts>' \
                 for the design, or give this boundary a connectrules block with an explicit \
                 vsup.",
                rails.len(),
                self.parameter
            )));
        }

        if !self
            .reported
            .borrow_mut()
            .insert(node_label.to_ascii_uppercase())
        {
            return Ok(());
        }

        match derivation {
            SupplyDerivation::Declared | SupplyDerivation::Ambiguous { .. } => {}
            SupplyDerivation::Derived {
                source,
                level,
                node,
            } if live => log::warn!(
                "supply for boundary net '{node_label}' derived from {source} = {level} V, bound \
                 live to net '{node}' (inferred); set '.param {}={level}' to make it explicit \
                 and static",
                self.parameter
            ),
            SupplyDerivation::Derived { source, level, .. } => log::warn!(
                "supply for boundary net '{node_label}' derived from {source} = {level} V; set \
                 '.param {}={level}' to make it explicit",
                self.parameter
            ),
            SupplyDerivation::Defaulted => log::warn!(
                "no supply reaches boundary net '{node_label}'; defaulting to \
                 {DEFAULT_BOUNDARY_SUPPLY} V. Set '.param {}=<volts>' to say what it is",
                self.parameter
            ),
        }
        Ok(())
    }
}

/// The net a planned boundary's supply binds to live, if it binds to one.
///
/// A connect statement naming a net (`#(.vsup(vdd))`) binds to it, declared.
/// A connect statement stating a level, or the deck's `.param`, is static, as
/// Spectre's default connect rules are. A rail inferred from the netlist binds
/// to the rail's node, because that inference is a statement about which net
/// the boundary is powered from, and Spectre's supply-sensitive connect
/// modules read that net's voltage rather than a snapshot of it.
///
/// Only the A/D and D/A converters track a supply; a bidirectional boundary
/// converts against its static level.
pub(super) fn live_supply_net(bridge: &super::PlannedXspiceAutoBridge) -> Option<&str> {
    if !matches!(
        bridge.kind,
        super::XspiceAutoBridgeKind::Adc | super::XspiceAutoBridgeKind::Dac
    ) {
        return None;
    }
    #[cfg(feature = "veriloga")]
    if let Some(selected) = &bridge.connect_module {
        if let Some(net) = selected.supply_net() {
            return Some(net);
        }
        if selected.states_supply() {
            return None;
        }
    }
    match bridge.supply.as_ref()? {
        SupplyDerivation::Derived { node, .. } => Some(node),
        SupplyDerivation::Declared
        | SupplyDerivation::Defaulted
        | SupplyDerivation::Ambiguous { .. } => None,
    }
}

/// The circuit node a planned converter's live supply binds to.
///
/// A connect statement may name a net the deck does not have, which is a
/// refusal naming both rather than a converter silently left static.
pub(super) fn live_supply_node(
    circuit: &crate::CircuitData,
    bridge: &super::PlannedXspiceAutoBridge,
) -> Result<Option<usize>, SimulationError> {
    let Some(net) = live_supply_net(bridge) else {
        return Ok(None);
    };
    match circuit.get_node_by_name(net) {
        Some(node) if node > 0 => Ok(Some(node)),
        _ => {
            let names = circuit.node_names_sorted();
            Err(SimulationError::Circuit(format!(
                "boundary net '{}' converts against supply net '{net}', which is not a net of \
                 this circuit (or is ground); name the net that powers the boundary",
                super::xspice_auto_bridge_node_label(Some(&names), bridge.node)
            )))
        }
    }
}

/// The supply-tracking form of a built-in converter: the model reads its
/// supply through a `sup` port wired to `supply`. The Verilog-AMS route's
/// converters choose their tracking form themselves; the XSPICE auto-bridge's
/// stock `adc_bridge`/`dac_bridge` are swapped for their tracking twins, so a
/// deck's explicit `adc_bridge` cards keep the stock model untouched.
pub(super) fn with_live_supply(
    bridge: &super::PlannedXspiceAutoBridge,
    supply: Option<usize>,
    code_model: std::sync::Arc<dyn crate::xspice::CodeModel>,
    mut connections: Vec<crate::xspice::PortConnection>,
) -> (
    std::sync::Arc<dyn crate::xspice::CodeModel>,
    Vec<crate::xspice::PortConnection>,
) {
    let Some(supply) = supply else {
        return (code_model, connections);
    };
    connections.push(crate::xspice::PortConnection::Analog(supply));
    if bridge.event_node.is_some() {
        return (code_model, connections);
    }
    let code_model: std::sync::Arc<dyn crate::xspice::CodeModel> = match bridge.kind {
        super::XspiceAutoBridgeKind::Adc => {
            std::sync::Arc::new(crate::xspice::models::TrackingAdcBridge)
        }
        super::XspiceAutoBridgeKind::Dac => {
            std::sync::Arc::new(crate::xspice::models::TrackingDacBridge)
        }
        _ => code_model,
    };
    (code_model, connections)
}

/// Say that a boundary converts against a net its connect statement named.
#[cfg(feature = "veriloga")]
pub(super) fn report_declared_live(node_label: &str, net: &str) {
    log::info!(
        "supply for boundary net '{node_label}' is net '{net}', bound live (declared by its \
         connect statement)"
    );
}

fn same_level(left: crate::Value, right: crate::Value) -> bool {
    (left - right).abs() <= SUPPLY_LEVEL_TOLERANCE * left.abs().max(right.abs()).max(1.0)
}
