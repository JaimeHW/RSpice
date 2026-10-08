//! The first producer of the bridge planner's connect-module seam.
//!
//! Verilog-AMS LRM 2.4 clause 7 decides which connect module bridges a
//! mixed-discipline connection. Those decisions are `rspice_veriloga::connect`'s
//! and are made there; this module is the two ends of the wire:
//! it hands that planner a boundary the engine found, and it turns the module
//! the planner names into a built-in bridge or an executable authored body.
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
//! Selected authored declarations compile through the ordinary mixed module
//! pipeline. Delegation is restricted to token-equivalent shipped signatures;
//! a matching module name never substitutes for its authored body.

use rspice_veriloga::ast::PortDirection;
use rspice_veriloga::connect::{
    ConnectDirection, ConnectRuleTable, ConnectValueKind, NetSegment, PortLink, ResolutionMode,
    Signal, plan_connect_modules, resolve_disciplines,
};
use rspice_veriloga::disciplines::DisciplineDb;

use crate::{ElaborationError, ElaborationErrorKind, SimulationError};

/// A clause-7 refusal about one boundary node.
///
/// Selection happens per boundary rather than per instance, and several of
/// these run before any X-card is reached at all — choosing the deck's
/// `connectrules` block is a design-level decision — so the subject is the
/// node or the source rather than an instance. `ElaborationError` carries no
/// instance for those, which is what tells a workbench it has nothing to mark
/// on the schematic and should report against the deck instead.
fn connect_refusal(detail: impl Into<String>) -> ElaborationError {
    ElaborationError::new(ElaborationErrorKind::ConnectRule, detail)
}

/// The continuous discipline a SPICE deck node has.
const DECK_DISCIPLINE: &str = "electrical";
/// The discrete discipline an XSPICE event net has.
const EVENT_DISCIPLINE: &str = "logic";

/// A selected boundary declaration, its execution binding and its parameters.
/// The parameters are section 7.7.3's, already folded — see
/// [`rspice_veriloga::connect::InsertionRule::numeric_parameters`], which folds
/// them in the crate that owns the expression.
#[derive(Debug, Clone)]
pub(super) struct PlannedConnectModule {
    pub(super) name: String,
    /// Section 7.8.5's generated instance name, kept for diagnostics: it names
    /// the boundary in the vocabulary the deck author wrote, not the engine's.
    pub(super) instance: String,
    pub(super) parameters: Vec<(String, f64)>,
    pub(super) execution: Option<std::sync::Arc<super::connect_execution::ConnectExecution>>,
}

impl PlannedConnectModule {
    fn parameter(&self, name: &str) -> Option<f64> {
        self.parameters
            .iter()
            .find(|(parameter, _)| parameter.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
    }

    /// The supply this connect statement states for the boundary, if it states
    /// one.
    ///
    /// Section 7.7.3's `vsup` is authoritative for the boundary it is written
    /// on, which is why the deck's supply is neither derived nor spoken about
    /// when it is present: see [`delegated_parameters`], which prefers it.
    pub(super) fn stated_supply(&self) -> Option<f64> {
        self.parameter("vsup")
    }

    /// Whether this connect statement answers the supply question itself.
    pub(super) fn states_supply(&self) -> bool {
        self.stated_supply().is_some()
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
) -> (PortDirection, ConnectDirection) {
    use super::XspiceAutoBridgeKind as Kind;
    match kind {
        Kind::Adc | Kind::VToReal => (PortDirection::Input, ConnectDirection::AnalogToDiscrete),
        Kind::Dac | Kind::RealToV => (PortDirection::Output, ConnectDirection::DiscreteToAnalog),
        Kind::Bidi | Kind::RealBidi => (PortDirection::Inout, ConnectDirection::Bidirectional),
    }
}

fn boundary_value_kind(kind: super::XspiceAutoBridgeKind) -> ConnectValueKind {
    use super::XspiceAutoBridgeKind as Kind;
    match kind {
        Kind::Adc | Kind::Dac | Kind::Bidi => ConnectValueKind::FourState,
        Kind::RealToV | Kind::VToReal | Kind::RealBidi => ConnectValueKind::Real,
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
    let (port_direction, expected) = boundary_direction(kind);
    let value_kind = boundary_value_kind(kind);
    // A logic-only configuration leaves existing real conversion policy alone.
    if value_kind == ConnectValueKind::Real
        && !table
            .insertions()
            .iter()
            .any(|rule| rule.discrete.value_kind == Some(value_kind))
    {
        return Ok(None);
    }

    let mut signal = Signal::default();
    let lower = signal.push(
        NetSegment::new(node_label)
            .declared(EVENT_DISCIPLINE)
            .digital_behavioral()
            .with_value_kind(value_kind),
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
        return Err(ElaborationError::new(
            ElaborationErrorKind::Internal,
            format!(
                "connect module selection for node '{node_label}' derived a {} bridge from the \
                 port direction while the bridge planner derived a {}; the two disagree about \
                 which side drives",
                insertion.direction.label(),
                expected.label()
            ),
        )
        .instance(instance_name)
        .into());
    }

    let parameters = table
        .select_typed(
            DECK_DISCIPLINE,
            EVENT_DISCIPLINE,
            expected,
            Some(value_kind),
            db,
        )
        .and_then(|rule| rule.numeric_parameters())
        .map_err(|error| connect_error(node_label, &error))?
        .into_iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect();

    Ok(Some(PlannedConnectModule {
        name: insertion.connect_module.to_string(),
        instance: insertion.instance,
        parameters,
        execution: None,
    }))
}

fn connect_error(
    node_label: &str,
    error: &rspice_veriloga::connect::ConnectError,
) -> SimulationError {
    connect_refusal(format!(
        "node '{node_label}' is a mixed-discipline connection and its connect rules do not \
         settle it: {error}"
    ))
    .into()
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
/// The supply is the *deck's* rather than the connect module's declared
/// `vsup` default, because a node's supply is a property of the deck and the
/// module's default exists so the module is well formed standing alone. The
/// library's default is 3.3 V, which `rspice_veriloga::connect::library`'s
/// parameter pin holds still, and which is also what the deck's supply falls
/// back to when nothing in the deck says otherwise.
pub(super) fn delegated_parameters(
    selected: &PlannedConnectModule,
    kind: super::XspiceAutoBridgeKind,
    vcc: crate::Value,
) -> Result<Vec<(String, crate::Value)>, SimulationError> {
    use super::XspiceAutoBridgeKind as Kind;

    let supply = selected.stated_supply().unwrap_or(vcc);
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
        Kind::RealToV | Kind::VToReal | Kind::RealBidi => Vec::new(),
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
            return Err(connect_refusal(format!(
                "the connect statement for '{}' passes parameter '{name}', which this \
                 delegation does not carry to the {} bridge; the built-in connect modules \
                 take a supply and their transition times",
                selected.name,
                super::xspice_auto_bridge_kind_label(kind)
            ))
            .module(selected.name.as_str())
            .into());
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
        Kind::RealToV | Kind::VToReal | Kind::RealBidi => &[],
    }
}

/// Which built-in connect module a bridge kind delegates to.
pub(super) fn expected_library_module(kind: super::XspiceAutoBridgeKind) -> Option<&'static str> {
    use super::XspiceAutoBridgeKind as Kind;
    match kind {
        Kind::Adc => Some("a2d"),
        Kind::Dac => Some("d2a"),
        Kind::Bidi => Some("bidir"),
        Kind::RealToV | Kind::VToReal | Kind::RealBidi => None,
    }
}

/// Selection must carry an authenticated execution decision from its source.
pub(super) fn check_execution(
    selected: &PlannedConnectModule,
    _kind: super::XspiceAutoBridgeKind,
    node_label: &str,
) -> Result<(), SimulationError> {
    if selected.execution.is_some() {
        return Ok(());
    }
    Err(connect_refusal(format!(
        "node '{node_label}' selects connect module '{}' (instance '{}') without its prepared source execution binding",
        selected.name, selected.instance
    )).module(selected.name.as_str()).into())
}

// ---------------------------------------------------------------------------
// Where the rules come from, and the pass that runs over the planner's answers
// ---------------------------------------------------------------------------

/// A design's active connect specification. Compiled devices supply the exact
/// preprocessed closure retained in their artifact, including virtual sources.
/// Standalone libraries use the same preparation path. Repeated modules from
/// one closure register once. Selection happens after all sources are known,
/// so include order cannot select a different named configuration.
#[derive(Debug, Default)]
pub(super) struct DesignConnectRules {
    declared_in: Option<std::path::PathBuf>,
    requested: Option<String>,
    requested_source: Option<(String, std::path::PathBuf)>,
    registered_sources: std::collections::HashSet<String>,
    available: Vec<(String, std::path::PathBuf, rspice_veriloga::source::Span)>,
    matches: usize,
    table: ConnectRuleTable,
    disciplines: DisciplineDb,
    source: Option<std::sync::Arc<str>>,
    builtin_delegations: std::collections::BTreeSet<String>,
    executions: std::cell::RefCell<
        std::collections::HashMap<
            String,
            std::sync::Arc<super::connect_execution::ConnectExecution>,
        >,
    >,
}

impl DesignConnectRules {
    /// Verify both the library and named block already used by executable
    /// internal connections. A later deck selection must re-elaborate first.
    pub(super) fn validate_hierarchical_context(
        &self,
        artifact: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
        instance: &str,
    ) -> Result<(), SimulationError> {
        let same_selection = match artifact.connections.configuration() {
            Some(configuration) => {
                Some(configuration.library().preprocessed_source()) == self.source.as_deref()
                    && self.table.blocks().first()
                        .is_some_and(|block| block.name == configuration.block())
            }
            None => artifact.connections.source() == self.source.as_deref(),
        };
        if artifact.connections.is_elaborated() && !same_selection {
            return Err(connect_refusal(
                "hierarchical connect instances were elaborated with a different source configuration; compile the retained source with the deck-selected connection library and rule block",
            )
            .instance(instance)
            .module(artifact.hir.module_name.as_str())
            .into());
        }
        Ok(())
    }

    pub(super) fn for_netlist(netlist: &crate::Netlist) -> Result<Self, SimulationError> {
        let mut selected = Self {
            requested: netlist.options.connect_rules.clone(),
            ..Default::default()
        };
        if let Some(alias) = netlist.options.connect_rules_source.as_deref() {
            let mut paths = std::collections::BTreeSet::new();
            for include in &netlist.veriloga_includes {
                if include
                    .model_name
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(alias))
                {
                    paths.insert(super::veriloga_cache::canonicalize_for_cache(
                        &include.file_path,
                    ));
                }
            }
            if paths.len() != 1 {
                let available = netlist
                    .veriloga_includes
                    .iter()
                    .filter_map(|include| {
                        include
                            .model_name
                            .as_deref()
                            .map(|name| format!("'{name}' from '{}'", include.file_path.display()))
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(connect_refusal(format!(
                    "CONNECTRULES_SOURCE '{alias}' must identify exactly one explicitly aliased .VERILOGA source; found {} sources. Available imports: {available}",
                    paths.len()
                ))
                .into());
            }
            selected.requested_source = Some((
                alias.to_owned(),
                paths.into_iter().next().expect("one source"),
            ));
        }
        Ok(selected)
    }

    fn source_is_selected(&self, path: &std::path::Path) -> bool {
        self.requested_source.as_ref().is_none_or(|(_, selected)| {
            *selected == super::veriloga_cache::canonicalize_for_cache(path)
        })
    }

    pub(super) fn register(
        &mut self,
        path: &std::path::Path,
        specification: rspice_veriloga::ConnectSpecification,
    ) -> Result<(), SimulationError> {
        // Filter by source before deduplicating a content identity. Two imports
        // may have identical bytes; an earlier unselected import must not hide
        // the explicitly selected source from the design.
        if !self.source_is_selected(path) {
            return Ok(());
        }
        if !self
            .registered_sources
            .insert(specification.source_identity)
        {
            return Ok(());
        }
        for block in specification.rules.blocks() {
            self.available
                .push((block.name.to_string(), path.to_path_buf(), block.span));
            if self
                .requested
                .as_deref()
                .is_some_and(|name| name != block.name.as_str())
            {
                continue;
            }
            self.matches += 1;
            if self.matches == 1 {
                self.table = specification
                    .rules
                    .select_block(&block.name)
                    .map_err(|error| {
                        SimulationError::from(
                            connect_refusal(format!("connectrules block: {error}")).in_source(path),
                        )
                    })?;
                self.declared_in = Some(path.to_path_buf());
                self.disciplines = specification.disciplines.clone();
                self.source = specification.source.clone();
                self.builtin_delegations = specification.builtin_delegations.clone();
            }
        }
        Ok(())
    }

    pub(super) fn finish_selection(&self) -> Result<(), SimulationError> {
        if self.matches == 1
            || (self.matches == 0 && self.requested.is_none() && self.requested_source.is_none())
        {
            return Ok(());
        }
        let available = self
            .available
            .iter()
            .map(|(name, path, span)| {
                format!(
                    "'{name}' in '{}' (preprocessed bytes {}..{})",
                    path.display(),
                    span.start,
                    span.end
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        let mut message = match self.requested.as_deref() {
            Some(name) if self.matches == 0 => {
                format!("Unknown connectrules '{name}'; available configurations: {available}")
            }
            Some(name) => format!(
                "Connectrules '{name}' is ambiguous across distinct source closures: {available}"
            ),
            None if self.matches == 0 => {
                "The selected Verilog-A source declares no connectrules configuration".to_owned()
            }
            None => format!(
                "Multiple connectrules configurations are available; select one with .options connectrules=NAME: {available}"
            ),
        };
        if let Some((alias, path)) = &self.requested_source {
            message.push_str(&format!(
                "; CONNECTRULES_SOURCE '{alias}' selects '{}'",
                path.display()
            ));
        }
        Err(connect_refusal(message).into())
    }

    pub(super) fn register_artifact(
        &mut self,
        path: &std::path::Path,
        artifact: &rspice_veriloga::canonical_ir::CanonicalIrArtifact,
    ) -> Result<(), SimulationError> {
        if !self.source_is_selected(path) {
            return Ok(());
        }
        let Some(source) = artifact.connections.source() else {
            return Ok(());
        };
        if self
            .registered_sources
            .contains(artifact.metadata.source_identity.as_str())
        {
            return Ok(());
        }
        let specification = rspice_veriloga::VerilogACompiler::default()
            .connect_specification_from_preprocessed(source)
            .map_err(|error| {
                SimulationError::from(
                    ElaborationError::new(
                        ElaborationErrorKind::CacheCorrupt,
                        format!("connect rules in this compiled source could not be read: {error}"),
                    )
                    .in_source(path),
                )
            })?;
        self.register(path, specification)
    }

    fn selected(&self) -> Option<(&ConnectRuleTable, &DisciplineDb)> {
        self.declared_in
            .as_ref()
            .map(|_| (&self.table, &self.disciplines))
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
        let Some(mut selected) =
            select_for_boundary(table, db, kind, node_label, instance_name, port_name)?
        else {
            return Ok(None);
        };
        let rule = table
            .select_typed(
                DECK_DISCIPLINE,
                EVENT_DISCIPLINE,
                boundary_direction(kind).1,
                Some(boundary_value_kind(kind)),
                db,
            )
            .map_err(|error| connect_error(node_label, &error))?;
        let key = format!(
            "{}:{:?}",
            selected.name,
            selected
                .parameters
                .iter()
                .map(|(name, value)| (name, value.to_bits()))
                .collect::<Vec<_>>()
        );
        let mut executions = self.executions.borrow_mut();
        let execution = executions.entry(key).or_insert_with(|| {
            std::sync::Arc::new(
                if self.builtin_delegations.contains(&selected.name)
                    && expected_library_module(kind) == Some(selected.name.as_str())
                {
                    super::connect_execution::ConnectExecution::Delegated
                } else {
                    super::connect_execution::ConnectExecution::Authored(
                        super::connect_execution::AuthoredConnectBody::new(
                            self.source.clone().expect("selected source retained"),
                            self.declared_in
                                .as_ref()
                                .expect("selected source path")
                                .to_string_lossy()
                                .into_owned(),
                            rule.continuous.name.to_string(),
                            rule.discrete.name.to_string(),
                        ),
                    )
                },
            )
        });
        selected.execution = Some(std::sync::Arc::clone(execution));
        Ok(Some(selected))
    }
}

/// The XSPICE instance and port that made each node discrete.
///
/// Clause 7's [`PortLink`] names them, and this is where a flat deck keeps
/// them. It is a second walk over the instances rather than a field on the
/// planner's own traversal so that a deck with no connect rules pays nothing —
/// the caller runs this only after finding rules.
fn event_port_owners(
    circuit: &crate::CircuitData,
) -> std::collections::BTreeMap<usize, (String, String)> {
    let mut owners: std::collections::BTreeMap<usize, (String, String)> = Default::default();
    for instance in &circuit.xspice_instances {
        for (port_idx, port) in instance.ports().iter().enumerate() {
            let Some(connection) = instance.connection_at(port_idx) else {
                continue;
            };
            let mut nodes = std::collections::BTreeMap::new();
            super::register_digital_connection_nodes(&mut nodes, connection, port.direction);
            let mut real_nodes = std::collections::BTreeMap::new();
            super::register_real_connection_nodes(&mut real_nodes, connection, port.direction);
            for node in nodes.keys().chain(real_nodes.keys()) {
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
    if rules.selected().is_none() {
        return Ok(());
    }

    let node_names = circuit.node_names_sorted();
    let owners = event_port_owners(circuit);

    for bridge in bridges.iter_mut() {
        let node_label = super::xspice_auto_bridge_node_label(Some(&node_names), bridge.node);
        let (instance_name, port_name) = owners
            .get(&bridge.node)
            .cloned()
            .unwrap_or_else(|| (node_label.clone(), "d".to_string()));
        let Some(selected) =
            rules.select_for_boundary_node(bridge.kind, &node_label, &instance_name, &port_name)?
        else {
            continue;
        };
        check_execution(&selected, bridge.kind, &node_label)?;
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
