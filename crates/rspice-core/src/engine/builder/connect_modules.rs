//! The first producer of the bridge planner's connect-module seam.
//!
//! Verilog-AMS LRM 2.4 clause 7 decides which connect module bridges a
//! mixed-discipline connection. Those decisions are `rspice_veriloga::connect`'s
//! and are made there; this module is the two ends of the wire:
//! it hands that planner a boundary the engine found, and it turns the module
//! the planner names into parameters the engine's existing bridge stamps.
//!
//! # Why the boundary is the auto-bridge planner's and not a second pass
//!
//! `super::plan_xspice_auto_bridges` is the one place that answers "is this
//! node a boundary, and which way does it face". A connect module changes
//! *which model* bridges a node, never whether the node is a boundary, so this
//! runs after that planner and reads its answers. There is no second search
//! for boundaries, which is what keeps the two from ever disagreeing.
//!
//! # What a flat deck's boundary looks like to clause 7
//!
//! Clause 7 resolves disciplines over a *signal*: net segments joined by
//! ports. A SPICE deck has no Verilog-AMS hierarchy to elaborate, but a mixed
//! node in one is not therefore outside clause 7 — it is the smallest signal
//! clause 7 describes, and it is built here rather than approximated:
//!
//! * the upper (actual) segment is the deck node, declared `electrical`,
//!   because a node an analog element attaches to is a node with a potential
//!   and a flow;
//! * the lower (formal) segment is the event-side net inside the XSPICE
//!   instance, declared `logic` and marked as used in digital behavioural code
//!   — Annex F.2.1 step 4a's own words for why it is discrete;
//! * one [`PortLink`] joins them, whose direction is the direction the
//!   instance declares for that port.
//!
//! Section 7.4.4's resolution then runs unchanged, and
//! [`plan_connect_modules`] selects unchanged. Nothing here re-implements
//! either. The one thing this asserts about the result is that the direction
//! clause 7 derives from the port matches the direction the bridge planner
//! derived from the port types — two independent routes to one answer, checked
//! rather than assumed.
//!
//! # What is refused
//!
//! A connect module this engine cannot execute. Delegation is what makes a
//! connect module runnable at all: `a2d`, `d2a` and `bidir` become the XSPICE
//! bridge code models that already implement them. A connect module named by a
//! `connectrules` block and *not* in that library has a body only a
//! Verilog-AMS mixed host could run, and the one this crate has
//! (`crate::xspice::verilog::MixedSignalHost`) is not wired to the engine.
//! It no longer refuses an off-grid trial time — it floors an analog timepoint
//! onto its tick grid and keeps the unquantized time — so what is missing is
//! the elaboration that would give a resolved connect module a `CircuitData`
//! instance to be planned onto, not the host's own time base. Until that
//! exists it is refused by name, with that reason, rather than silently
//! bridged as if the deck had asked for nothing.

use rspice_veriloga::ast::PortDirection;
use rspice_veriloga::connect::{
    ConnectDirection, ConnectRuleTable, NetSegment, PortLink, ResolutionMode, Signal,
    plan_connect_modules, resolve_disciplines,
};
use rspice_veriloga::disciplines::DisciplineDb;

use crate::SimulationError;

/// The continuous discipline a SPICE deck node has.
const DECK_DISCIPLINE: &str = "electrical";
/// The discrete discipline an XSPICE event net has.
const EVENT_DISCIPLINE: &str = "logic";

/// A connect module selected for one boundary, reduced to what the engine
/// stamps.
///
/// The name is kept so materialization can say which module it is delegating,
/// and so a module outside the library can be refused by that name. The
/// parameters are section 7.7.3's, already folded — see
/// [`rspice_veriloga::connect::InsertionRule::numeric_parameters`], which folds
/// them in the crate that owns the expression.
#[derive(Debug, Clone)]
pub(super) struct PlannedConnectModule {
    pub(super) name: String,
    /// Section 7.8.5's generated instance name, kept for diagnostics: it names
    /// the boundary in the vocabulary the deck author wrote, not the engine's.
    pub(super) instance: String,
    pub(super) parameters: Vec<(String, f64)>,
}

impl PlannedConnectModule {
    fn parameter(&self, name: &str) -> Option<f64> {
        self.parameters
            .iter()
            .find(|(parameter, _)| parameter.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
    }
}

/// Which of Table 7-2's kinds a planned bridge needs, and the port direction
/// that produces it.
///
/// The bridge planner reads XSPICE port *types* and the connect planner reads
/// port *directions*; this is the one place the two vocabularies meet, and it
/// is a total function so neither can grow a case the other has not.
pub(super) fn boundary_direction(
    kind: super::XspiceAutoBridgeKind,
) -> Option<(PortDirection, ConnectDirection)> {
    use super::XspiceAutoBridgeKind as Kind;
    match kind {
        // The digital device reads the node, so its port is an input and the
        // analog side drives.
        Kind::Adc => Some((PortDirection::Input, ConnectDirection::AnalogToDiscrete)),
        Kind::Dac => Some((PortDirection::Output, ConnectDirection::DiscreteToAnalog)),
        Kind::Bidi => Some((PortDirection::Inout, ConnectDirection::Bidirectional)),
        // Real-valued event traffic is not a discipline boundary: clause 7
        // resolves disciplines, and a `wreal` net carries a real number rather
        // than a discipline's potential and flow.
        Kind::RealToV | Kind::VToReal => None,
    }
}

/// Select the connect module for one boundary node.
///
/// `node_label` is the deck's own name for the node, which becomes section
/// 7.8.5's `SigName` in the generated instance name.
pub(super) fn select_for_boundary(
    table: &ConnectRuleTable,
    db: &DisciplineDb,
    kind: super::XspiceAutoBridgeKind,
    node_label: &str,
    instance_name: &str,
    port_name: &str,
) -> Result<Option<PlannedConnectModule>, SimulationError> {
    let Some((port_direction, expected)) = boundary_direction(kind) else {
        return Ok(None);
    };

    let mut signal = Signal::default();
    let lower = signal.push(
        NetSegment::new(node_label)
            .declared(EVENT_DISCIPLINE)
            .digital_behavioral(),
    );
    signal.push(
        NetSegment::new(node_label)
            .declared(DECK_DISCIPLINE)
            .with_child(PortLink::new(
                lower,
                port_direction,
                instance_name,
                port_name,
            )),
    );

    let resolved = resolve_disciplines(&signal, table, db, None, ResolutionMode::Basic)
        .map_err(|error| connect_error(node_label, &error))?;
    let plan = plan_connect_modules(&signal, &resolved, table, db)
        .map_err(|error| connect_error(node_label, &error))?;

    let Some(insertion) = plan.insertions.into_iter().next() else {
        return Ok(None);
    };
    if insertion.direction != expected {
        return Err(SimulationError::Circuit(format!(
            "connect module selection for node '{node_label}' derived a {} bridge from the \
             port direction while the bridge planner derived a {}; the two disagree about \
             which side drives",
            insertion.direction.label(),
            expected.label()
        )));
    }

    let parameters = table
        .select(DECK_DISCIPLINE, EVENT_DISCIPLINE, expected, db)
        .and_then(|rule| rule.numeric_parameters())
        .map_err(|error| connect_error(node_label, &error))?
        .into_iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect();

    Ok(Some(PlannedConnectModule {
        name: insertion.connect_module.to_string(),
        instance: insertion.instance,
        parameters,
    }))
}

fn connect_error(
    node_label: &str,
    error: &rspice_veriloga::connect::ConnectError,
) -> SimulationError {
    SimulationError::Circuit(format!(
        "node '{node_label}' is a mixed-discipline connection and its connect rules do not \
         settle it: {error}"
    ))
}

/// What the delegation stamps on the XSPICE bridge code model for one selected
/// connect module.
///
/// Nothing is stamped that the connect statement did not ask for. That is the
/// whole of why a deck which names a connect module gets the same numbers it
/// would have got from the auto-bridge alone: the supply is the deck's, the
/// thresholds derive from it exactly as `super::add_planned_xspice_auto_bridge`
/// derives them, and a code-model parameter with no section 7.7.3 override
/// keeps the code model's own default.
///
/// The supply is the *deck's* `vcc` rather than the connect module's declared
/// `vsup` default, because a node's supply is a property of the deck and the
/// module's default exists so the module is well formed standing alone. The
/// two are the same number by construction — both 3.3 V — which
/// `rspice_veriloga::connect::library`'s parameter pin holds still.
pub(super) fn delegated_parameters(
    selected: &PlannedConnectModule,
    kind: super::XspiceAutoBridgeKind,
    vcc: crate::Value,
) -> Result<Vec<(String, crate::Value)>, SimulationError> {
    use super::XspiceAutoBridgeKind as Kind;

    let supply = selected.parameter("vsup").unwrap_or(vcc);
    let half_supply = supply / 2.0;
    let mut parameters: Vec<(String, crate::Value)> = match kind {
        Kind::Adc => vec![
            ("in_low".to_string(), half_supply),
            ("in_high".to_string(), half_supply),
        ],
        Kind::Dac => vec![
            ("out_low".to_string(), 0.0),
            ("out_high".to_string(), supply),
        ],
        Kind::Bidi => vec![
            ("out_high".to_string(), supply),
            ("in_low".to_string(), half_supply),
            ("in_high".to_string(), half_supply),
        ],
        Kind::RealToV | Kind::VToReal => Vec::new(),
    };

    // `dac_bridge` reads `out_undef` as the midpoint of the two levels exactly
    // when they are given and it is not, so the midpoint is obtained by
    // leaving it out. Stamping a midpoint here would be a second statement of
    // what half is.
    for (connect_parameter, model_parameter) in delegated_timing(kind) {
        if let Some(value) = selected.parameter(connect_parameter) {
            parameters.push((model_parameter.to_string(), value));
        }
    }

    for (name, _) in &selected.parameters {
        let known = name.eq_ignore_ascii_case("vsup")
            || delegated_timing(kind)
                .iter()
                .any(|(connect_parameter, _)| name.eq_ignore_ascii_case(connect_parameter));
        if !known {
            return Err(SimulationError::Circuit(format!(
                "the connect statement for '{}' passes parameter '{name}', which this \
                 delegation does not carry to the {} bridge; the built-in connect modules \
                 take a supply and their transition times",
                selected.name,
                super::xspice_auto_bridge_kind_label(kind)
            )));
        }
    }

    Ok(parameters)
}

/// The timing parameters this kind's connect module carries, paired with the
/// code-model parameter each becomes.
fn delegated_timing(kind: super::XspiceAutoBridgeKind) -> &'static [(&'static str, &'static str)] {
    use super::XspiceAutoBridgeKind as Kind;
    match kind {
        Kind::Adc => &[("tdrise", "rise_delay"), ("tdfall", "fall_delay")],
        Kind::Dac | Kind::Bidi => &[("trise", "t_rise"), ("tfall", "t_fall")],
        Kind::RealToV | Kind::VToReal => &[],
    }
}

/// Which built-in connect module a bridge kind delegates to.
///
/// A `connectrules` block that names anything else is refused: see this
/// module's documentation for why there is no route to execute one.
pub(super) fn expected_library_module(kind: super::XspiceAutoBridgeKind) -> Option<&'static str> {
    use super::XspiceAutoBridgeKind as Kind;
    match kind {
        Kind::Adc => Some("a2d"),
        Kind::Dac => Some("d2a"),
        Kind::Bidi => Some("bidir"),
        Kind::RealToV | Kind::VToReal => None,
    }
}

/// Refuse a connect module that is not one the delegation implements.
pub(super) fn check_delegable(
    selected: &PlannedConnectModule,
    kind: super::XspiceAutoBridgeKind,
    node_label: &str,
) -> Result<(), SimulationError> {
    let expected = expected_library_module(kind);
    if expected.is_some_and(|expected| selected.name.eq_ignore_ascii_case(expected)) {
        return Ok(());
    }
    Err(SimulationError::Circuit(format!(
        "node '{node_label}' selects connect module '{}' (instance '{}'), which RSpice cannot \
         execute: a connect module runs here by delegating to the XSPICE bridge code model \
         that implements it, and only the built-in library — a2d, d2a and bidir — has such a \
         delegation. Executing an arbitrary connect module's body needs the Verilog-AMS mixed \
         host, which is not wired to the engine and refuses any trial time off its \
         integer-nanosecond grid",
        selected.name, selected.instance
    )))
}

// ---------------------------------------------------------------------------
// Where the rules come from, and the pass that runs over the planner's answers
// ---------------------------------------------------------------------------

/// The design's clause 7 connect specification, accumulated one `.veriloga`
/// file at a time as the include loop reads them.
///
/// Two rules govern what gets in, both stated rather than discovered:
///
/// * **A file is read only if its text contains `connectrules`.** Compiling a
///   Verilog-A model is cached and reading its connect rules is not, so the
///   filter is what keeps a deck that has no connect rules — which is every
///   deck that had none before this existed — from paying a preprocess per
///   build. The consequence is that a `connectrules` block reached through an
///   `` `include `` is not seen, and that is the stated rule: clause 7's block
///   is a design-level statement and RSpice requires it in the file the deck
///   names.
/// * **At most one file may declare connect rules.** Clause 7 names a
///   `connectrules` block and gives no way to select among several. Merging
///   the blocks *within* one file is the only reading available and is what
///   `rspice_veriloga::connect::build_connect_rule_table` does; extending that
///   across files would be this crate inventing a rule the language does not
///   have, so a second file is refused and both are named.
#[derive(Debug, Default)]
pub(super) struct DesignConnectRules {
    declared_in: Option<std::path::PathBuf>,
    table: ConnectRuleTable,
}

impl DesignConnectRules {
    /// Whether a file is worth reading for connect rules at all.
    ///
    /// The cheapest filter there is, and the reason the rule about `` `include
    /// `` above holds: compiling a Verilog-A model is cached and reading its
    /// connect rules is not, so a deck that has no connect rules must not pay
    /// a preprocess per build to discover that.
    pub(super) fn may_declare(path: &std::path::Path) -> bool {
        std::fs::read_to_string(path).is_ok_and(|text| text.contains("connectrules"))
    }

    /// Read one file's specification, refusing a second file that declares
    /// rules.
    pub(super) fn read(
        &mut self,
        path: &std::path::Path,
    ) -> Result<rspice_veriloga::ConnectSpecification, SimulationError> {
        let specification = rspice_veriloga::VerilogACompiler::default()
            .connect_specification_from_file(path)
            .map_err(|error| {
                SimulationError::Circuit(format!(
                    "connect rules in '{}' could not be read: {error}",
                    path.display()
                ))
            })?;
        if specification.rules.insertions().is_empty()
            && specification.rules.resolutions().is_empty()
        {
            return Ok(specification);
        }
        if let Some(first) = self.declared_in.as_ref() {
            return Err(SimulationError::Circuit(format!(
                "'{}' and '{}' both declare connect rules; Verilog-AMS LRM 2.4 clause 7 names \
                 one connectrules specification for a design and gives no way to select among \
                 several",
                first.display(),
                path.display()
            )));
        }
        self.declared_in = Some(path.to_path_buf());
        self.table = specification.rules.clone();
        Ok(specification)
    }

    fn selected(&self) -> Option<(&ConnectRuleTable, DisciplineDb)> {
        self.declared_in
            .as_ref()
            .map(|_| (&self.table, DisciplineDb::with_standard()))
    }

    /// Select a connect module for one boundary named directly rather than
    /// found by the XSPICE bridge planner.
    ///
    /// A mixed Verilog-AMS module's discrete port *is* the boundary — the
    /// instance and port clause 7's [`PortLink`] wants are the deck's X-card
    /// and the module's own port name — so there is nothing for
    /// [`attach_to_planned_bridges`] to search for. What must not differ is
    /// the selection itself, so this is the same
    /// [`select_for_boundary`] call with the two names supplied instead of
    /// recovered.
    ///
    /// `None` for a design with no connect rules, which is every design that
    /// had none before this existed.
    pub(super) fn select_for_boundary_node(
        &self,
        kind: super::XspiceAutoBridgeKind,
        node_label: &str,
        instance_name: &str,
        port_name: &str,
    ) -> Result<Option<PlannedConnectModule>, SimulationError> {
        let Some((table, db)) = self.selected() else {
            return Ok(None);
        };
        select_for_boundary(table, &db, kind, node_label, instance_name, port_name)
    }
}

/// The XSPICE instance and port that made each node discrete.
///
/// Clause 7's [`PortLink`] names them, and this is where a flat deck keeps
/// them. It is a second walk over the instances rather than a field on the
/// planner's own traversal so that a deck with no connect rules pays nothing —
/// the caller runs this only after finding rules.
fn digital_port_owners(
    circuit: &crate::CircuitData,
) -> std::collections::BTreeMap<usize, (String, String)> {
    let mut owners: std::collections::BTreeMap<usize, (String, String)> = Default::default();
    for instance in &circuit.xspice_instances {
        for (port_idx, port) in instance.ports().iter().enumerate() {
            if port.default_type != crate::xspice::PortType::Digital {
                continue;
            }
            let Some(connection) = instance.connection_at(port_idx) else {
                continue;
            };
            let mut nodes = std::collections::BTreeMap::new();
            super::register_digital_connection_nodes(&mut nodes, connection, port.direction);
            for node in nodes.keys() {
                owners
                    .entry(*node)
                    .or_insert_with(|| (instance.name.clone(), port.name.clone()));
            }
        }
    }
    owners
}

/// Select a connect module for every boundary the bridge planner found.
///
/// A no-op for a design with no connect rules, which is the path every
/// existing deck takes: the planner's answers are left exactly as it produced
/// them and materialization is the one it has always been.
pub(super) fn attach_to_planned_bridges(
    circuit: &crate::CircuitData,
    rules: &DesignConnectRules,
    bridges: &mut [super::PlannedXspiceAutoBridge],
) -> Result<(), SimulationError> {
    if bridges.is_empty() {
        return Ok(());
    }
    let Some((table, db)) = rules.selected() else {
        return Ok(());
    };

    let node_names = circuit.node_names_sorted();
    let owners = digital_port_owners(circuit);

    for bridge in bridges.iter_mut() {
        if boundary_direction(bridge.kind).is_none() {
            continue;
        }
        let node_label = super::xspice_auto_bridge_node_label(Some(&node_names), bridge.node);
        let (instance_name, port_name) = owners
            .get(&bridge.node)
            .cloned()
            .unwrap_or_else(|| (node_label.clone(), "d".to_string()));
        let Some(selected) = select_for_boundary(
            table,
            &db,
            bridge.kind,
            &node_label,
            &instance_name,
            &port_name,
        )?
        else {
            continue;
        };
        check_delegable(&selected, bridge.kind, &node_label)?;
        log::info!(
            "Node '{}' bridges through connect module '{}' as instance '{}'",
            node_label,
            selected.name,
            selected.instance
        );
        bridge.connect_module = Some(selected);
    }

    Ok(())
}

#[cfg(test)]
mod tests;
