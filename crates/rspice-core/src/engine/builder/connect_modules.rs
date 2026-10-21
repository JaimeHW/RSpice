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
use rspice_veriloga::connect::library::BuiltinSignature;
use rspice_veriloga::connect::{
    ConnectDirection, ConnectRuleTable, ConnectValueKind, NetSegment, PortLink, ResolutionMode,
    Signal, plan_connect_modules, resolve_disciplines,
};
use rspice_veriloga::disciplines::DisciplineDb;

use crate::{ElaborationError, ElaborationErrorKind, SimulationError};

mod parameters;
pub(super) mod supply_ports;
pub(super) use crate::circuit::{ConverterSource as ParameterSource, ConverterSupply};
pub(super) use parameters::{ConverterLaw, RealConverterLaw, module_name};
use supply_ports::{DeclaredSupplyPort, SupplyPortBinding};

/// A clause-7 refusal about the deck's connect rules.
///
/// Several of these run before any X-card is reached at all — choosing the
/// deck's `connectrules` block is a design-level decision — so their subject
/// is the source rather than an instance. `ElaborationError` carries no
/// instance for those, which is what tells a workbench it has nothing to mark
/// on the schematic and should report against the deck instead.
///
/// A refusal about one boundary is different: the boundary is one instance's
/// port, and the rules that cannot settle it are refused by that instance, as
/// a port that would bring a second event domain onto a node is. Those name
/// it (see [`connect_error`] and [`check_execution`]).
fn connect_refusal(detail: impl Into<String>) -> ElaborationError {
    ElaborationError::new(ElaborationErrorKind::ConnectRule, detail)
}

/// The original coordinates of a connect source the design retains only as
/// preprocessed text, prepared again from its file when a refusal needs to
/// point into it. A file that no longer prepares to the same text is not
/// mapped, so a location is never taken from a different snapshot.
pub(super) fn original_coordinates<'a>(
    path: &'a std::path::Path,
    source: &str,
) -> Option<OriginalCoordinates<'a>> {
    let prepared = super::veriloga_cache::prepare_veriloga_source(
        path,
        crate::ResourceLimits::default(),
        &crate::abort_signal::NoAbort,
    )
    .ok()?;
    (prepared.preprocessed_source() == source).then_some(OriginalCoordinates { path, prepared })
}

/// A connect source's preprocessed text mapped back to the files it was
/// written in. The deck's own file is named as the deck names it.
pub(super) struct OriginalCoordinates<'a> {
    path: &'a std::path::Path,
    prepared: rspice_veriloga::PreparedRuntimeSource,
}

impl OriginalCoordinates<'_> {
    fn shown(&self, file: &str) -> String {
        let canonical = super::veriloga_cache::canonicalize_for_cache;
        if canonical(std::path::Path::new(file)) == canonical(self.path) {
            self.path.display().to_string()
        } else {
            file.to_owned()
        }
    }

    /// The file, line and column one byte of the prepared text was written at.
    pub(super) fn locate(&self, offset: usize) -> Option<(String, usize, usize)> {
        let (file, line, column) = self.prepared.source_location(offset)?;
        Some((self.shown(&file), line, column))
    }

    /// A compiler error on the prepared text, as findings in original files.
    pub(super) fn diagnostics_for_error(
        &self,
        error: &rspice_veriloga::CompileError,
    ) -> Vec<rspice_veriloga::SourceCompileDiagnostic> {
        let mut diagnostics = self.prepared.diagnostics_for_error(error);
        for diagnostic in &mut diagnostics {
            if let Some(path) = &mut diagnostic.path {
                *path = self.shown(path);
            }
        }
        diagnostics
    }
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
    /// The deck net a `vsup` override names instead of a level
    /// (`#(.vsup(vdd))`): the boundary converts against that net's solved
    /// voltage, live, rather than against a number.
    pub(super) supply_net: Option<String>,
    pub(super) execution: Option<std::sync::Arc<super::connect_execution::ConnectExecution>>,
    /// The `connectrules` block the selecting statement is in.
    pub(super) block: String,
    /// Where that statement is written, `file:line`, when its source still
    /// prepares to the text the rules were read from.
    pub(super) location: Option<String>,
    /// The selected module's supply ports, as its declaration and the
    /// statement give them.
    pub(super) supply_ports: Vec<DeclaredSupplyPort>,
    /// The net each supply port is wired to at this boundary; see
    /// [`supply_ports::bind`].
    pub(super) supply_bindings: Vec<SupplyPortBinding>,
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
    /// when it is present.
    pub(super) fn stated_supply(&self) -> Option<f64> {
        self.parameter("vsup")
    }

    /// The shipped signature this module's declaration matches, when the
    /// built-in converter executes it.
    pub(super) fn signature(&self) -> Option<BuiltinSignature> {
        match self.execution.as_deref()? {
            super::connect_execution::ConnectExecution::Delegated(signature) => Some(*signature),
            super::connect_execution::ConnectExecution::Authored(_) => None,
        }
    }

    /// Whether this connect statement answers the supply question itself.
    pub(super) fn states_supply(&self) -> bool {
        self.stated_supply().is_some()
    }

    /// The net this connect statement binds the boundary's supply to.
    pub(super) fn supply_net(&self) -> Option<&str> {
        self.supply_net.as_deref()
    }

    /// What chose the converter, as its insertion record says it.
    pub(super) fn rules(&self) -> ParameterSource {
        ParameterSource::ConnectRule {
            block: self.block.clone(),
            earlier_signature: self.signature() == Some(BuiltinSignature::Legacy),
        }
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
        .map_err(|error| connect_error(node_label, instance_name, &error))?;
    let plan = plan_connect_modules(&signal, &resolved, table, db)
        .map_err(|error| connect_error(node_label, instance_name, &error))?;

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

    let rule = table
        .select_typed(
            DECK_DISCIPLINE,
            EVENT_DISCIPLINE,
            expected,
            Some(value_kind),
            db,
        )
        .map_err(|error| connect_error(node_label, instance_name, &error))?;
    let parameters = rule
        .numeric_parameters()
        .map_err(|error| connect_error(node_label, instance_name, &error))?
        .into_iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect();

    Ok(Some(PlannedConnectModule {
        name: insertion.connect_module.to_string(),
        instance: insertion.instance,
        parameters,
        supply_net: rule.supply_net().map(ToString::to_string),
        execution: None,
        block: table
            .blocks()
            .first()
            .map(|block| block.name.to_string())
            .unwrap_or_default(),
        location: None,
        supply_ports: DeclaredSupplyPort::of_rule(rule),
        supply_bindings: Vec::new(),
    }))
}

/// The connect rules cannot settle the boundary `instance_name`'s port makes
/// at `node_label`.
fn connect_error(
    node_label: &str,
    instance_name: &str,
    error: &rspice_veriloga::connect::ConnectError,
) -> SimulationError {
    connect_refusal(format!(
        "node '{node_label}' is a mixed-discipline connection and its connect rules do not \
         settle it: {error}"
    ))
    .instance(instance_name)
    .into()
}

/// The built-in converter's law for one boundary: every parameter, checked,
/// with where it came from.
///
/// `selected` is the delegating connect module, or `None` for an HDL port no
/// connect rule reaches, which converts with the rule set's defaults.
///
/// The supply is the *deck's* rather than the connect module's declared
/// `vsup` default, unless the statement gives one: a node's supply is a
/// property of the deck, and the module's default exists so the module is
/// well formed standing alone.
pub(super) fn converter_law(
    selected: Option<&PlannedConnectModule>,
    kind: super::XspiceAutoBridgeKind,
    supply: ConverterSupply,
) -> Result<ConverterLaw, SimulationError> {
    let (selected, signature) = match selected {
        Some(selected) => {
            let signature = selected.signature().ok_or_else(|| {
                SimulationError::from(ElaborationError::new(
                    ElaborationErrorKind::Internal,
                    format!(
                        "connect module '{}' reached the built-in converter without delegating",
                        selected.name
                    ),
                ))
            })?;
            (Some(selected), signature)
        }
        None => (None, BuiltinSignature::Legacy),
    };
    ConverterLaw::resolve(
        kind,
        selected.map(|selected| {
            (
                selected.name.as_str(),
                signature,
                selected.parameters.as_slice(),
            )
        }),
        supply,
    )
    .map_err(|message| {
        let refusal = connect_refusal(message);
        match selected {
            Some(selected) => refusal.module(selected.name.as_str()),
            None => refusal,
        }
        .into()
    })
}

/// Which built-in connect module a bridge kind delegates to.
pub(super) fn expected_library_module(kind: super::XspiceAutoBridgeKind) -> Option<&'static str> {
    use super::XspiceAutoBridgeKind as Kind;
    match kind {
        Kind::Adc => Some("a2d"),
        Kind::Dac => Some("d2a"),
        Kind::Bidi => Some("bidir"),
        Kind::RealToV => Some("r2e"),
        Kind::VToReal => Some("e2r"),
        Kind::RealBidi => Some("rbidir"),
    }
}

/// The law of the real-valued converter a delegating module selects: every
/// parameter, checked, with where it came from.
pub(super) fn real_converter_law(
    selected: &PlannedConnectModule,
    kind: super::XspiceAutoBridgeKind,
) -> Result<RealConverterLaw, SimulationError> {
    RealConverterLaw::resolve(kind, &selected.name, &selected.parameters).map_err(|message| {
        connect_refusal(message)
            .module(selected.name.as_str())
            .into()
    })
}

/// Whether a planned boundary converts through a shipped real-valued
/// converter: a real port whose connect rule selects `r2e`, `e2r` or
/// `rbidir`. A real port no connect rule reaches keeps the stock converters.
pub(super) fn converts_real_built_in(bridge: &super::PlannedXspiceAutoBridge) -> bool {
    use super::XspiceAutoBridgeKind as Kind;
    matches!(bridge.kind, Kind::RealToV | Kind::VToReal | Kind::RealBidi)
        && bridge
            .connect_module
            .as_ref()
            .is_some_and(|selected| selected.signature().is_some())
}

/// Selection must carry an authenticated execution decision from its source.
/// A refusal names `instance_name`, whose port the boundary is.
pub(super) fn check_execution(
    selected: &PlannedConnectModule,
    kind: super::XspiceAutoBridgeKind,
    node_label: &str,
    instance_name: &str,
) -> Result<(), SimulationError> {
    if let Some(net) = selected.supply_net()
        && !matches!(
            kind,
            super::XspiceAutoBridgeKind::Adc
                | super::XspiceAutoBridgeKind::Dac
                | super::XspiceAutoBridgeKind::Bidi
        )
    {
        return Err(connect_refusal(format!(
            "node '{node_label}' selects connect module '{}' with vsup naming net '{net}'; only \
             the digital/analog converters track a supply net, so give this {} boundary's \
             vsup a level",
            selected.name,
            super::xspice_auto_bridge_kind_label(kind)
        ))
        .instance(instance_name)
        .module(selected.name.as_str())
        .into());
    }
    if let (Some(net), Some(execution)) = (selected.supply_net(), selected.execution.as_deref())
        && matches!(
            execution,
            super::connect_execution::ConnectExecution::Authored(_)
        )
    {
        // An authored module receives its parameters as numbers; a net binds
        // to a supply port, and this module has none named vsup.
        return Err(connect_refusal(format!(
            "node '{node_label}' selects the authored connect module '{}' with vsup naming net \
             '{net}', but the module has no supply port named vsup; give vsup a level, or \
             declare vsup as an inout electrical port of the module so the net can be wired \
             to it",
            selected.name
        ))
        .instance(instance_name)
        .module(selected.name.as_str())
        .into());
    }
    if selected.execution.is_some() {
        return Ok(());
    }
    Err(connect_refusal(format!(
        "node '{node_label}' selects connect module '{}' (instance '{}') without its prepared source execution binding",
        selected.name, selected.instance
    )).instance(instance_name).module(selected.name.as_str()).into())
}

/// Wire an authored module's supply ports at the boundary it is inserted on:
/// `node_label`, bridging `instance`'s port `port`. `level` is the deck's
/// supply for the boundary, a bound net's nominal level where no rail holds
/// it. A module with no supply ports, or one the built-in converter executes,
/// is left as it is.
pub(super) fn bind_supply_ports(
    circuit: &crate::CircuitData,
    supplies: &super::boundary_supply::BoundarySupplies,
    selected: &mut PlannedConnectModule,
    boundary: &supply_ports::Boundary<'_>,
) -> Result<(), SimulationError> {
    if selected.supply_ports.is_empty() || selected.signature().is_some() {
        return Ok(());
    }
    selected.supply_bindings = supply_ports::bind(circuit, supplies, selected, boundary)?;
    Ok(())
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
    /// Each configuration's name, file, and span in that file's prepared
    /// source, which the last field retains for locating it.
    available: Vec<(
        String,
        std::path::PathBuf,
        rspice_veriloga::source::Span,
        Option<std::sync::Arc<str>>,
    )>,
    matches: usize,
    table: ConnectRuleTable,
    disciplines: DisciplineDb,
    source: Option<std::sync::Arc<str>>,
    builtin_delegations:
        std::collections::BTreeMap<String, rspice_veriloga::connect::library::BuiltinSignature>,
    configuration: std::sync::OnceLock<std::sync::Arc<rspice_veriloga::ConnectionConfiguration>>,
    executions: std::cell::RefCell<
        std::collections::HashMap<
            String,
            std::sync::Arc<super::connect_execution::ConnectExecution>,
        >,
    >,
    /// Where each selected statement is written, by its offset in the
    /// prepared source, so the source is prepared again once per statement
    /// rather than once per boundary.
    locations: std::cell::RefCell<std::collections::HashMap<usize, Option<String>>>,
    /// The deck's `.options wreal_resolution`, which an authored body's plain
    /// `wreal` ports take as the deck's own modules do.
    wreal_resolution: rspice_veriloga::canonical_ir::digital::DigitalRealResolution,
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
                    && self
                        .table
                        .blocks()
                        .first()
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
            wreal_resolution: super::veriloga_sources::wreal_resolution(
                netlist.options.wreal_resolution,
            ),
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
            self.available.push((
                block.name.to_string(),
                path.to_path_buf(),
                block.span,
                specification.source.clone(),
            ));
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

    pub(super) fn configuration(
        &self,
    ) -> Result<Option<std::sync::Arc<rspice_veriloga::ConnectionConfiguration>>, SimulationError>
    {
        self.finish_selection()?;
        let Some(block) = self.table.blocks().first() else {
            return Ok(None);
        };
        if let Some(configuration) = self.configuration.get() {
            return Ok(Some(configuration.clone()));
        }
        let configuration = rspice_veriloga::ConnectionConfiguration::from_preprocessed(
            &self
                .declared_in
                .as_ref()
                .expect("selected library")
                .to_string_lossy(),
            self.source.as_deref().expect("selected source"),
            &block.name,
        )
        .map_err(|error| SimulationError::from(connect_refusal(error)))?;
        let _ = self.configuration.set(std::sync::Arc::new(configuration));
        Ok(self.configuration.get().cloned())
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
            .map(|(name, path, span, source)| {
                match source
                    .as_deref()
                    .and_then(|source| original_coordinates(path, source))
                    .and_then(|coordinates| coordinates.locate(span.start as usize))
                {
                    Some((file, line, column)) => format!("'{name}' at {file}:{line}:{column}"),
                    None => format!("'{name}' in '{}'", path.display()),
                }
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
            .map_err(|error| connect_error(node_label, instance_name, &error))?;
        selected.location = self
            .locations
            .borrow_mut()
            .entry(rule.span.start as usize)
            .or_insert_with(|| {
                let path = self.declared_in.as_deref()?;
                let (file, line, _) = original_coordinates(path, self.source.as_deref()?)?
                    .locate(rule.span.start as usize)?;
                Some(format!("{file}:{line}"))
            })
            .clone();
        let key = format!(
            "{}:{:?}",
            selected.name,
            selected
                .parameters
                .iter()
                .map(|(name, value)| (name, value.to_bits()))
                .collect::<Vec<_>>()
        );
        let configuration = self.configuration()?.expect("selected configuration");
        let mut executions = self.executions.borrow_mut();
        let execution = executions.entry(key).or_insert_with(|| {
            std::sync::Arc::new(
                if let Some(signature) = self
                    .builtin_delegations
                    .get(&selected.name)
                    .filter(|_| expected_library_module(kind) == Some(selected.name.as_str()))
                {
                    super::connect_execution::ConnectExecution::Delegated(*signature)
                } else {
                    super::connect_execution::ConnectExecution::Authored(
                        super::connect_execution::AuthoredConnectBody::new(
                            configuration.clone(),
                            self.source.clone().expect("selected source retained"),
                            self.declared_in
                                .as_ref()
                                .expect("selected source path")
                                .to_string_lossy()
                                .into_owned(),
                            rule.continuous.name.to_string(),
                            rule.discrete.name.to_string(),
                            rule.span,
                            self.wreal_resolution,
                        ),
                    )
                },
            )
        });
        selected.execution = Some(std::sync::Arc::clone(execution));
        Ok(Some(selected))
    }
}

/// Select a connect module for every boundary the bridge planner found.
///
/// A no-op for a design with no connect rules, which is the path every
/// existing deck takes: the planner's answers are left exactly as it produced
/// them and materialization is the one it has always been.
pub(super) fn attach_to_planned_bridges(
    circuit: &crate::CircuitData,
    rules: &DesignConnectRules,
    supplies: &super::boundary_supply::BoundarySupplies,
    bridges: &mut [super::PlannedXspiceAutoBridge],
) -> Result<(), SimulationError> {
    if bridges.is_empty() {
        return Ok(());
    }
    if rules.selected().is_none() {
        return Ok(());
    }

    let node_names = circuit.node_names_sorted();

    for bridge in bridges.iter_mut() {
        let node_label = super::xspice_auto_bridge_node_label(Some(&node_names), bridge.node);
        let (instance_name, port_name) = bridge
            .port
            .clone()
            .unwrap_or_else(|| (node_label.clone(), "d".to_string()));
        let Some(mut selected) =
            rules.select_for_boundary_node(bridge.kind, &node_label, &instance_name, &port_name)?
        else {
            continue;
        };
        check_execution(&selected, bridge.kind, &node_label, &instance_name)?;
        bind_supply_ports(
            circuit,
            supplies,
            &mut selected,
            &supply_ports::Boundary {
                node_label: &node_label,
                instance: &instance_name,
                port: &port_name,
                level: bridge.vcc,
            },
        )?;
        bridge.connect_module = Some(selected);
    }

    Ok(())
}

/// Whether a planned boundary converts through the built-in connect-rule
/// converter: a four-state HDL port, or an XSPICE boundary whose connect rule
/// selects a shipped signature. An XSPICE boundary no connect rule reaches
/// keeps ngspice's own bridge, and an authored module executes its body.
pub(super) fn converts_built_in(bridge: &super::PlannedXspiceAutoBridge) -> bool {
    use super::XspiceAutoBridgeKind as Kind;
    matches!(bridge.kind, Kind::Adc | Kind::Dac | Kind::Bidi)
        && match &bridge.connect_module {
            Some(selected) => selected.signature().is_some(),
            None => bridge.event_node.is_some(),
        }
}

/// Every XSPICE output driving the event node `node`, in instance order: the
/// drivers a driver-sensitive D/A converts one by one. Each is the event
/// bank's driver key: instance, port and element.
pub(super) fn event_node_drivers(
    circuit: &crate::CircuitData,
    node: usize,
) -> Vec<(String, String, usize)> {
    use crate::xspice::{PortConnection, PortDirection};
    let mut drivers = Vec::new();
    for instance in &circuit.xspice_instances {
        for (port_index, port) in instance.ports().iter().enumerate() {
            if !matches!(port.direction, PortDirection::Out | PortDirection::InOut) {
                continue;
            }
            let nodes: Vec<usize> = match instance.connection_at(port_index) {
                Some(PortConnection::Digital(n) | PortConnection::DigitalInverted(n)) => vec![*n],
                Some(PortConnection::DigitalVector(nodes)) => nodes.clone(),
                Some(PortConnection::DigitalVectorMapped(nodes)) => {
                    nodes.iter().map(|mapped| mapped.node).collect()
                }
                _ => continue,
            };
            for (element, connected) in nodes.into_iter().enumerate() {
                if connected == node {
                    drivers.push((instance.name.clone(), port.name.clone(), element));
                }
            }
        }
    }
    drivers
}

#[cfg(test)]
mod tests;
