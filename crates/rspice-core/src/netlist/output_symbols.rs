//! Typed output-request provenance and pre-construction symbol validation.
//!
//! Output directives are intentionally retained in their existing execution
//! forms (`SaveSet`, `MeasureStatement`, and `AnalysisCommand::Four`).  This
//! module is the semantic sidecar: it records where each request came from,
//! extracts every circuit-symbol dependency in authored order, and validates
//! those dependencies against the flattened circuit namespace before any
//! topology reduction or device stamping occurs.

use super::{
    AnalysisCommand, Element, ElementKind, Flattener, FlattenerConfig, MeasureStatement, Netlist,
    NetlistSourceLocation, ParseError, ParseWithAbortError, ensure_parse_not_aborted,
    poll_parse_abort, poll_parse_text,
};
use crate::abort_signal::AbortSignal;
use std::collections::{HashMap, HashSet};

/// The directive family that owns an output request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputDirectiveKind {
    Save,
    Probe,
    Print,
    Plot,
    Measure,
    Four,
}

impl OutputDirectiveKind {
    fn is_direct_output(self) -> bool {
        matches!(self, Self::Save | Self::Probe | Self::Print | Self::Plot)
    }
}

impl std::fmt::Display for OutputDirectiveKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Save => ".SAVE",
            Self::Probe => ".PROBE",
            Self::Print => ".PRINT",
            Self::Plot => ".PLOT",
            Self::Measure => ".MEASURE",
            Self::Four => ".FOUR",
        })
    }
}

/// Analysis domain selected by a direct output request.
///
/// `.PRINT` and `.PLOT` carry this qualifier explicitly. Measurement
/// requests retain it from their parsed statement, while analysis-agnostic
/// `.SAVE` and `.PROBE` requests leave it unset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputAnalysisKind {
    /// Transient analysis.
    Tran,
    /// Small-signal AC analysis.
    Ac,
    /// DC sweep analysis.
    Dc,
    /// Noise spectral-density analysis.
    Noise,
    /// Small-signal distortion analysis.
    Disto,
    /// DC operating-point analysis.
    Op,
    /// Small-signal transfer-function analysis.
    Tf,
    /// Scattering-parameter analysis.
    Sp,
    /// Periodic steady-state analysis.
    Pss,
}

impl OutputAnalysisKind {
    pub(crate) fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword.trim().to_ascii_uppercase().as_str() {
            "TRAN" | "TRAN_CONT" => Some(Self::Tran),
            "AC" | "AC_CONT" => Some(Self::Ac),
            "DC" | "DC_CONT" => Some(Self::Dc),
            "NOISE" | "NOISE_CONT" => Some(Self::Noise),
            "DISTO" => Some(Self::Disto),
            "OP" => Some(Self::Op),
            "TF" => Some(Self::Tf),
            "SP" => Some(Self::Sp),
            "PSS" => Some(Self::Pss),
            _ => None,
        }
    }
}

/// Namespace searched by one output dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputSymbolKind {
    Node,
    Device,
}

impl std::fmt::Display for OutputSymbolKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Node => "node",
            Self::Device => "device",
        })
    }
}

/// One circuit-symbol occurrence referenced by an output request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputSymbolDependency {
    /// Authored accessor (`V`, `VM`, `I`, `@`, ...), retained for adapters.
    pub operator: String,
    /// Authored symbol spelling, including hierarchy separators or wildcards.
    pub symbol: String,
    pub kind: OutputSymbolKind,
    /// Whether this accessor occurred inside a braced/quoted expression.
    /// Direct-output expression occurrences retain duplicates exactly.
    pub expression: bool,
}

/// Provenance sidecar for one source-level output request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputRequest {
    pub directive: OutputDirectiveKind,
    pub origin: NetlistSourceLocation,
    /// Explicit analysis qualifier, when the directive carries one.
    pub analysis: Option<OutputAnalysisKind>,
    /// Optional semantic name used to replace redefined measurements.
    pub name: Option<String>,
    /// Dependencies in their original occurrence order. Repetitions are
    /// meaningful and are therefore not collapsed here.
    pub dependencies: Vec<OutputSymbolDependency>,
}

impl OutputRequest {
    /// Whether this request needs any transient device-current operand.
    pub(crate) fn requires_transient_device_current_operand(&self) -> bool {
        self.analysis
            .is_none_or(|analysis| analysis == OutputAnalysisKind::Tran)
            && self.dependencies.iter().any(|dependency| {
                dependency.kind == OutputSymbolKind::Device
                    && is_transient_device_current_operator(&dependency.operator)
            })
    }

    /// Whether this request needs a derived transient current for `device`.
    ///
    /// Direct `I(device)` probes already appear in [`SaveSet`](super::SaveSet),
    /// but current and power accessors nested inside output expressions are
    /// represented only by this typed dependency sidecar. Result retention
    /// must honor both representations before integration starts.
    pub(crate) fn selects_transient_device_current(&self, device: &str) -> bool {
        if !self.requires_transient_device_current_operand() {
            return false;
        }
        let device = canonical_symbol(device);
        self.dependencies.iter().any(|dependency| {
            dependency.kind == OutputSymbolKind::Device
                && is_transient_device_current_operator(&dependency.operator)
                && hierarchy_pattern_matches(&canonical_symbol(&dependency.symbol), &device)
        })
    }

    /// Whether this request retains a transient node-voltage operand for the
    /// named node.  Node dependencies are recorded for direct voltage probes
    /// and for voltage operands nested inside expressions; device-current
    /// dependencies are intentionally excluded.
    pub(crate) fn selects_transient_node_voltage(&self, node: &str) -> bool {
        if self
            .analysis
            .is_some_and(|analysis| analysis != OutputAnalysisKind::Tran)
        {
            return false;
        }
        let node = canonical_symbol(node);
        self.dependencies.iter().any(|dependency| {
            dependency.kind == OutputSymbolKind::Node
                && hierarchy_pattern_matches(&canonical_symbol(&dependency.symbol), &node)
        })
    }

    pub(crate) fn from_source(
        directive: OutputDirectiveKind,
        origin: NetlistSourceLocation,
        source: &str,
    ) -> Self {
        let analysis = if matches!(
            directive,
            OutputDirectiveKind::Print | OutputDirectiveKind::Plot
        ) {
            source
                .split_whitespace()
                .next()
                .and_then(OutputAnalysisKind::from_keyword)
        } else {
            None
        };
        Self {
            directive,
            origin,
            analysis,
            name: None,
            dependencies: extract_output_dependencies(source),
        }
    }

    /// Build the semantic request corresponding to one frontend output
    /// override, such as a command-line `--save` value.
    ///
    /// The same accessor extractor used for source-authored output cards owns
    /// dependency recognition here. Bare vector names follow
    /// [`parse_save_probe`](super::parse_save_probe) semantics and are treated
    /// as node-voltage shorthand. Device-parameter probes remain outside
    /// symbol-existence validation because their validity belongs to device
    /// metadata validation.
    pub fn from_save_override(origin: NetlistSourceLocation, source: &str) -> Self {
        let mut dependencies = extract_output_dependencies(source);
        if dependencies.is_empty()
            && !source.trim_start().starts_with('@')
            && let Some(super::SaveSignal::Raw(node)) = super::parse_save_probe(source)
            && !node.eq_ignore_ascii_case("all")
        {
            dependencies = extract_output_dependencies(&format!("V({})", source.trim()));
        }
        Self {
            directive: OutputDirectiveKind::Save,
            origin,
            analysis: None,
            name: None,
            dependencies,
        }
    }

    pub(crate) fn from_measure(
        statement: &MeasureStatement,
        origin: NetlistSourceLocation,
        authored_source: &str,
    ) -> Self {
        let mut sources = Vec::new();
        collect_measure_sources(statement, &mut sources);
        let mut dependencies = Vec::new();
        for source in sources {
            let (source, expression_context) = match source {
                MeasureDependencySource::Direct(source) => (source, false),
                MeasureDependencySource::Expression(source) => (source, true),
            };
            dependencies.extend(extract_output_dependencies_with_context(
                source,
                expression_context,
            ));
        }
        let dependencies = retain_authored_dependency_spelling(
            dependencies,
            extract_output_dependencies(authored_source),
        );
        Self {
            directive: OutputDirectiveKind::Measure,
            origin,
            analysis: OutputAnalysisKind::from_keyword(&statement.analysis),
            name: Some(statement.name.clone()),
            dependencies,
        }
    }

    pub(crate) fn from_four(
        outputs: &[String],
        origin: NetlistSourceLocation,
        authored_source: &str,
    ) -> Self {
        let dependencies = outputs
            .iter()
            .flat_map(|output| extract_output_dependencies(output))
            .collect();
        let dependencies = retain_authored_dependency_spelling(
            dependencies,
            extract_output_dependencies(authored_source),
        );
        Self {
            directive: OutputDirectiveKind::Four,
            origin,
            analysis: None,
            name: None,
            dependencies,
        }
    }
}

/// Canonical current-projection output operators supported by Xyce.
pub(crate) fn is_current_projection_accessor(operator: &str) -> bool {
    matches!(operator, "I" | "IR" | "II" | "IM" | "IP" | "IDB")
}

/// Xyce's device-lead syntax is `I?`, where `?` is an arbitrary one-byte
/// terminal designator (`ID`, `IT`, `I1`, and so on). The caller must already
/// have canonicalized the operator.
pub(crate) fn is_device_lead_current_accessor(operator: &str) -> bool {
    operator.len() == 2 && operator.starts_with('I')
}

/// Direct/raw Xyce current-output grammar. Expression callers must reserve
/// builtin function names such as `IF` using their expression context.
pub(crate) fn is_current_output_accessor(operator: &str) -> bool {
    is_current_projection_accessor(operator) || is_device_lead_current_accessor(operator)
}

fn is_transient_device_current_operator(operator: &str) -> bool {
    matches!(operator, "P" | "W") || is_current_output_accessor(operator)
}

fn retain_authored_dependency_spelling(
    semantic: Vec<OutputSymbolDependency>,
    authored: Vec<OutputSymbolDependency>,
) -> Vec<OutputSymbolDependency> {
    let mut authored_index = 0;
    semantic
        .into_iter()
        .map(|dependency| {
            let matched = authored[authored_index..]
                .iter()
                .position(|candidate| {
                    candidate.kind == dependency.kind
                        && candidate
                            .operator
                            .eq_ignore_ascii_case(&dependency.operator)
                        && canonical_symbol(&candidate.symbol)
                            == canonical_symbol(&dependency.symbol)
                })
                .map(|offset| authored_index + offset);
            let Some(index) = matched else {
                return dependency;
            };
            authored_index = index + 1;
            let authored = &authored[index];
            OutputSymbolDependency {
                operator: authored.operator.clone(),
                symbol: authored.symbol.clone(),
                kind: dependency.kind,
                expression: dependency.expression,
            }
        })
        .collect()
}

/// One unresolved occurrence in a validated output request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedOutputSymbol {
    pub directive: OutputDirectiveKind,
    pub origin: NetlistSourceLocation,
    pub operator: String,
    pub symbol: String,
    pub kind: OutputSymbolKind,
}

/// Typed aggregate returned when output requests reference absent symbols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputSymbolValidationError {
    pub unresolved: Vec<UnresolvedOutputSymbol>,
}

impl std::fmt::Display for OutputSymbolValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("undefined symbol(s) in output request(s): ")?;
        for (index, item) in self.unresolved.iter().enumerate() {
            if index != 0 {
                formatter.write_str(", ")?;
            }
            write!(
                formatter,
                "{} {} '{}' via {} at {}",
                item.directive, item.kind, item.symbol, item.operator, item.origin
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for OutputSymbolValidationError {}

/// Validate every typed output dependency against the flattened namespace.
///
/// The operation is transactional: all dependencies are scanned into a local
/// error vector, and the netlist is never mutated. Cancellation is polled while
/// traversing requests and flattened symbols. If flattening itself is invalid,
/// that existing error remains owned by the ordinary elaboration stage rather
/// than being reordered behind output validation.
pub fn validate_output_symbols_with_abort(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    if netlist.output_requests.is_empty() {
        return Ok(());
    }
    ensure_parse_not_aborted(abort)?;

    let mut flattener = Flattener::with_models_config(
        &netlist.subcircuits,
        &netlist.models,
        FlattenerConfig::debug(),
    );
    let elements = match flattener.flatten_with_abort(netlist, abort) {
        Ok(elements) => elements,
        Err(ParseWithAbortError::Aborted) => return Err(ParseWithAbortError::Aborted),
        Err(ParseWithAbortError::Parse(_)) => {
            // Do not change established flattening-error precedence. The same
            // elaboration will report its typed failure before topology/stamping.
            return Ok(());
        }
    };

    let mut nodes = HashSet::new();
    let mut devices = HashSet::new();
    nodes.insert("0".to_string());
    for (index, element) in elements.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        devices.insert(canonical_symbol(&element.name));
        if let ElementKind::Coupling {
            model: Some(_),
            inductors,
            ..
        } = &element.kind
            && !inductors.is_empty()
        {
            // Xyce's nonlinear Core device owns internal vectors under the
            // generated YMIN!KNAME namespace (for example
            // N(YMIN!KTRANS1_H)).
            devices.insert(canonical_symbol(&format!("YMIN!{}", element.name)));
        }
        for node in &element.nodes {
            nodes.insert(canonical_symbol(node));
        }
        collect_embedded_element_nodes(&element.kind, &mut nodes);
    }
    let node_aliases = collect_interface_node_aliases_with_abort(netlist, abort)?;
    ensure_parse_not_aborted(abort)?;

    let mut unresolved = Vec::new();
    for (request_index, request) in netlist.output_requests.iter().enumerate() {
        poll_parse_abort(abort, request_index)?;
        let ordered = validation_order(request);
        let mut seen = HashSet::new();
        for dependency in ordered {
            poll_parse_text(abort, &dependency.symbol)?;
            let canonical = canonical_dependency_symbol(
                &dependency.symbol,
                dependency.kind,
                netlist.ground_policy(),
            );
            let dedup_key = (dependency.kind, canonical.clone());
            if request.directive.is_direct_output()
                && !dependency.expression
                && !seen.insert(dedup_key)
            {
                continue;
            }
            let matched =
                if analysis_owned_output_vector_exists(netlist, request, dependency, &canonical) {
                    true
                } else {
                    let namespace = match dependency.kind {
                        OutputSymbolKind::Node => &nodes,
                        OutputSymbolKind::Device => &devices,
                    };
                    if dependency.kind == OutputSymbolKind::Node {
                        let node_match =
                            namespace_matches_with_aliases(namespace, &node_aliases, &canonical);
                        if node_match {
                            true
                        } else if dependency.operator.eq_ignore_ascii_case("N") {
                            n_operator_device_vector_exists(&devices, &canonical)
                        } else {
                            false
                        }
                    } else {
                        namespace_matches(namespace, &canonical)
                    }
                };
            if !matched {
                unresolved.push(UnresolvedOutputSymbol {
                    directive: request.directive,
                    origin: request.origin.clone(),
                    operator: dependency.operator.clone(),
                    symbol: dependency.symbol.clone(),
                    kind: dependency.kind,
                });
            }
        }
    }
    ensure_parse_not_aborted(abort)?;
    if unresolved.is_empty() {
        Ok(())
    } else {
        Err(
            ParseError::OutputSymbolValidation(Box::new(OutputSymbolValidationError {
                unresolved,
            }))
            .into(),
        )
    }
}

fn analysis_owned_output_vector_exists(
    netlist: &Netlist,
    request: &OutputRequest,
    dependency: &OutputSymbolDependency,
    canonical: &str,
) -> bool {
    if dependency.kind != OutputSymbolKind::Node
        || !matches!(
            canonical,
            "INOISE_SPECTRUM" | "ONOISE_SPECTRUM" | "INOISE" | "ONOISE"
        )
        || !netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. }
            )
        })
    {
        return false;
    }

    match request.directive {
        OutputDirectiveKind::Print | OutputDirectiveKind::Plot | OutputDirectiveKind::Measure => {
            request.analysis == Some(OutputAnalysisKind::Noise)
        }
        OutputDirectiveKind::Save | OutputDirectiveKind::Probe => true,
        OutputDirectiveKind::Four => false,
    }
}

/// Build the complete flattened node namespace used by semantic validators.
///
/// This includes element terminals, control/embedded nodes, and one-hop
/// hierarchy-interface aliases. `None` preserves ordinary elaboration-error
/// precedence when a deck cannot yet be flattened.
pub(crate) fn collect_output_node_namespace_from_elements_with_abort(
    netlist: &Netlist,
    elements: &[Element],
    abort: &dyn AbortSignal,
) -> Result<HashSet<String>, ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let mut nodes = HashSet::new();
    nodes.insert("0".to_string());
    for (index, element) in elements.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        for node in &element.nodes {
            nodes.insert(canonical_symbol(node));
        }
        collect_embedded_element_nodes(&element.kind, &mut nodes);
    }
    let aliases = collect_interface_node_aliases_with_abort(netlist, abort)?;
    for (alias, target) in aliases.iter() {
        if nodes.contains(target) {
            nodes.insert(alias.to_string());
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(nodes)
}

fn n_operator_device_vector_exists(devices: &HashSet<String>, canonical: &str) -> bool {
    if canonical
        .rsplit_once('.')
        .is_some_and(|(device, _)| namespace_matches(devices, device))
    {
        return true;
    }
    // Xyce exposes model-owned internal-node and branch solution vectors as
    // N(<device>_<vector>). Device names may themselves contain underscores,
    // so test every separator and accept only a prefix in the actual flattened
    // device namespace; the generated model remains the authority for whether
    // that vector is present at execution time.
    canonical
        .match_indices('_')
        .any(|(separator, _)| separator != 0 && namespace_matches(devices, &canonical[..separator]))
}

/// Validate output dependencies without cancellation.
///
/// Simulation engines invoke this transactionally before circuit
/// construction. Tooling may call it earlier to provide editor diagnostics
/// without making syntactic parsing depend on a fully constructed circuit.
pub fn validate_output_symbols(netlist: &Netlist) -> Result<(), ParseError> {
    super::finish_non_aborting_parse(validate_output_symbols_with_abort(
        netlist,
        &crate::abort_signal::NoAbort,
    ))
}

fn validation_order(request: &OutputRequest) -> Vec<&OutputSymbolDependency> {
    if !request.directive.is_direct_output() {
        return request.dependencies.iter().collect();
    }
    // Xyce creates direct lead-current operators before solution-vector node
    // operators. Keep lexical order within each namespace.
    let mut devices = request
        .dependencies
        .iter()
        .filter(|dependency| !dependency.expression && dependency.kind == OutputSymbolKind::Device)
        .collect::<Vec<_>>();
    let mut nodes = request
        .dependencies
        .iter()
        .filter(|dependency| !dependency.expression && dependency.kind == OutputSymbolKind::Node)
        .collect::<Vec<_>>();
    devices.sort_by_key(|dependency| canonical_symbol(&dependency.symbol));
    nodes.sort_by_key(|dependency| canonical_symbol(&dependency.symbol));
    devices.extend(nodes);
    devices.extend(
        request
            .dependencies
            .iter()
            .filter(|dependency| dependency.expression),
    );
    devices
}

fn collect_embedded_element_nodes(kind: &ElementKind, nodes: &mut HashSet<String>) {
    let mut insert = |node: &str| {
        nodes.insert(canonical_symbol(node));
    };
    match kind {
        ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. } => {
            insert(&control_nodes.0);
            insert(&control_nodes.1);
        }
        ElementKind::VSwitch {
            control_pos,
            control_neg,
            ..
        } => {
            insert(control_pos);
            insert(control_neg);
        }
        ElementKind::Xspice { ports, .. } => {
            for port in ports {
                use super::XspicePort;
                match port {
                    XspicePort::Analog(node)
                    | XspicePort::Digital(node)
                    | XspicePort::ExplicitDigital(node)
                    | XspicePort::DigitalInverted(node)
                    | XspicePort::Conductance(node)
                    | XspicePort::Current(node)
                    | XspicePort::VoltageName(node)
                    | XspicePort::Hybrid(node) => insert(node),
                    XspicePort::AnalogVector(nodes) | XspicePort::DigitalVector(nodes) => {
                        for node in nodes {
                            insert(node);
                        }
                    }
                    XspicePort::DigitalVectorMixed(nodes) => {
                        for node in nodes {
                            insert(&node.name);
                        }
                    }
                    XspicePort::DifferentialVoltage { pos, neg }
                    | XspicePort::DifferentialCurrent { pos, neg }
                    | XspicePort::DifferentialConductance { pos, neg }
                    | XspicePort::DifferentialHybrid { pos, neg } => {
                        insert(pos);
                        insert(neg);
                    }
                    XspicePort::Null => {}
                }
            }
        }
        _ => {}
    }
}

fn canonical_dependency_symbol(
    symbol: &str,
    kind: OutputSymbolKind,
    ground_policy: super::GroundPolicy,
) -> String {
    let canonical = canonical_symbol(symbol);
    if kind == OutputSymbolKind::Node && ground_policy.is_ground(&canonical) {
        "0".to_string()
    } else {
        canonical
    }
}

pub(crate) fn canonical_symbol(symbol: &str) -> String {
    symbol
        .trim()
        .chars()
        .map(|ch| {
            if ch == ':' {
                '.'
            } else {
                ch.to_ascii_uppercase()
            }
        })
        .collect()
}

/// Canonical map from subcircuit interface names to flattened solution nodes.
///
/// The representation stays opaque so validation and runtime consumers share
/// the same case and hierarchy-separator rules. Targets remain one-hop: a
/// target can itself be a physical node whose spelling also happens to be an
/// interface alias, and that physical node must win.
#[derive(Debug, Clone, Default)]
pub(crate) struct InterfaceNodeAliases {
    targets: HashMap<String, String>,
}

impl InterfaceNodeAliases {
    /// Resolve an authored alias spelling to its direct flattened target.
    pub(crate) fn resolve(&self, authored: &str) -> Option<&str> {
        let canonical = canonical_symbol(authored);
        self.targets.get(&canonical).map(String::as_str)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.targets
            .iter()
            .map(|(alias, target)| (alias.as_str(), target.as_str()))
    }

    fn keys(&self) -> impl Iterator<Item = &String> {
        self.targets.keys()
    }
}

/// Collect interface aliases for non-abortable analysis post-processing.
#[cfg(test)]
pub(crate) fn collect_interface_node_aliases(
    netlist: &Netlist,
) -> Result<InterfaceNodeAliases, ParseError> {
    super::finish_non_aborting_parse(collect_interface_node_aliases_with_abort(
        netlist,
        &crate::abort_signal::NoAbort,
    ))
}

/// Collect only interface aliases referenced by one analysis adapter.
///
/// Only hierarchy paths that can own a requested alias are traversed, and only
/// requested formal ports are retained. Exhaustive output-symbol validation
/// remains owned by the full collector. This projection matches Xyce's
/// requested-alias materialization policy without scaling measurement setup
/// with unrelated hierarchy.
pub(crate) fn collect_requested_interface_node_aliases(
    netlist: &Netlist,
    requested: &HashSet<String>,
) -> Result<InterfaceNodeAliases, ParseError> {
    let requested = requested
        .iter()
        .map(|alias| canonical_symbol(alias))
        .collect::<HashSet<_>>();
    if requested.is_empty() {
        return Ok(InterfaceNodeAliases::default());
    }
    super::finish_non_aborting_parse(collect_interface_node_aliases_impl(
        netlist,
        Some(&requested),
        &crate::abort_signal::NoAbort,
    ))
}

fn collect_interface_node_aliases_with_abort(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<InterfaceNodeAliases, ParseWithAbortError> {
    collect_interface_node_aliases_impl(netlist, None, abort)
}

fn collect_interface_node_aliases_impl(
    netlist: &Netlist,
    requested_aliases: Option<&HashSet<String>>,
    abort: &dyn AbortSignal,
) -> Result<InterfaceNodeAliases, ParseWithAbortError> {
    let config = FlattenerConfig::default();
    collect_interface_node_aliases_impl_with_limits(
        netlist,
        requested_aliases,
        abort,
        config.max_depth,
        config.max_elements,
    )
}

fn collect_interface_node_aliases_impl_with_limits(
    netlist: &Netlist,
    requested_aliases: Option<&HashSet<String>>,
    abort: &dyn AbortSignal,
    max_depth: usize,
    max_elements: usize,
) -> Result<InterfaceNodeAliases, ParseWithAbortError> {
    fn requested_instance_paths(
        requested_aliases: Option<&HashSet<String>>,
    ) -> (Option<HashSet<String>>, Option<HashSet<String>>) {
        let Some(requested_aliases) = requested_aliases else {
            return (None, None);
        };
        let mut visit = HashSet::new();
        let mut descend = HashSet::new();
        for alias in requested_aliases {
            let Some((deepest_instance, _)) = alias.rsplit_once('.') else {
                continue;
            };
            visit.insert(deepest_instance.to_string());
            let mut child = deepest_instance;
            while let Some((parent, _)) = child.rsplit_once('.') {
                visit.insert(parent.to_string());
                descend.insert(parent.to_string());
                child = parent;
            }
        }
        (Some(visit), Some(descend))
    }

    struct AliasCollector<'a> {
        definitions: HashMap<String, &'a super::SubcircuitDef>,
        external_subcircuits: HashSet<String>,
        globals: &'a HashSet<String>,
        ground_policy: super::GroundPolicy,
        aliases: HashMap<String, String>,
        requested_aliases: Option<&'a HashSet<String>>,
        instances_to_visit: Option<HashSet<String>>,
        instances_to_descend: Option<HashSet<String>>,
        max_depth: usize,
        max_elements: usize,
        emitted_elements: usize,
        traversal_steps: usize,
    }

    impl<'a> AliasCollector<'a> {
        fn charge_emitted_element(&mut self) -> Result<(), ParseWithAbortError> {
            let requested = self.emitted_elements.saturating_add(1);
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::FlattenedElements,
                requested,
                self.max_elements,
            )
            .map_err(ParseError::from)?;
            self.emitted_elements = requested;
            Ok(())
        }

        fn remap_node(&self, node: &str, prefix: &str, ports: &HashMap<String, String>) -> String {
            let canonical =
                canonical_dependency_symbol(node, OutputSymbolKind::Node, self.ground_policy);
            if canonical == "0" || self.globals.contains(&canonical) || canonical.starts_with("$G")
            {
                return canonical;
            }
            if let Some(mapped) = ports.get(&canonical) {
                return mapped.clone();
            }
            if prefix.is_empty() {
                canonical
            } else {
                format!("{prefix}.{canonical}")
            }
        }

        fn collect(
            &mut self,
            root_elements: &'a [super::Element],
            abort: &dyn AbortSignal,
        ) -> Result<(), ParseWithAbortError> {
            struct Frame<'a> {
                elements: &'a [super::Element],
                next_index: usize,
                prefix: String,
                parent_ports: HashMap<String, String>,
                active_definitions: Vec<String>,
                depth: usize,
            }

            let mut frames = vec![Frame {
                elements: root_elements,
                next_index: 0,
                prefix: String::new(),
                parent_ports: HashMap::new(),
                active_definitions: Vec::new(),
                depth: 0,
            }];
            while !frames.is_empty() {
                let frame_index = frames.len() - 1;
                if frames[frame_index].next_index >= frames[frame_index].elements.len() {
                    frames.pop();
                    continue;
                }
                let element_index = frames[frame_index].next_index;
                frames[frame_index].next_index += 1;
                poll_parse_abort(abort, self.traversal_steps)?;
                self.traversal_steps = self.traversal_steps.saturating_add(1);
                let frame = &frames[frame_index];
                let element = &frame.elements[element_index];
                crate::resource::ResourceLimitError::ensure(
                    crate::resource::ResourceKind::HierarchyDepth,
                    frame.depth,
                    self.max_depth,
                )
                .map_err(ParseError::from)?;
                let ElementKind::Subcircuit { subckt_name, .. } = &element.kind else {
                    if self.requested_aliases.is_none() {
                        self.charge_emitted_element()?;
                    }
                    continue;
                };
                let instance = if frame.prefix.is_empty() {
                    canonical_symbol(&element.name)
                } else {
                    format!("{}.{}", frame.prefix, canonical_symbol(&element.name))
                };
                if self
                    .instances_to_visit
                    .as_ref()
                    .is_some_and(|instances| !instances.contains(&instance))
                {
                    continue;
                }
                let canonical_subcircuit = subckt_name.to_ascii_uppercase();
                let Some(definition) = self.definitions.get(&canonical_subcircuit).copied() else {
                    if self.external_subcircuits.contains(&canonical_subcircuit) {
                        self.charge_emitted_element()?;
                        continue;
                    }
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!("Undefined subcircuit: {subckt_name}"),
                    }
                    .into());
                };
                let mapped_ports = definition
                    .ports
                    .iter()
                    .zip(&element.nodes)
                    .map(|(formal, actual)| {
                        (
                            formal,
                            self.remap_node(actual, &frame.prefix, &frame.parent_ports),
                        )
                    })
                    .collect::<Vec<_>>();
                super::flattener::validate_subcircuit_port_bindings(
                    definition,
                    subckt_name,
                    &element.name,
                    &instance,
                    element.nodes.len(),
                    &mapped_ports,
                    self.globals,
                    abort,
                )?;
                if let Some(start) = frame
                    .active_definitions
                    .iter()
                    .position(|name| name == &canonical_subcircuit)
                {
                    let mut chain = frame.active_definitions[start..].to_vec();
                    chain.push(canonical_subcircuit);
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!(
                            "Recursive subcircuit instantiation at '{instance}': {}",
                            chain.join(" -> ")
                        ),
                    }
                    .into());
                }
                let mut child_ports = HashMap::new();
                for (formal, target) in mapped_ports {
                    let formal = canonical_symbol(formal);
                    let alias = format!("{instance}.{formal}");
                    if self
                        .requested_aliases
                        .is_none_or(|requested| requested.contains(&alias))
                    {
                        if let Some(existing) = self.aliases.get(&alias) {
                            if existing != &target {
                                return Err(ParseError::Syntax {
                                    line: 0,
                                    message: format!(
                                        "Interface alias '{alias}' resolves ambiguously to '{existing}' and '{target}'"
                                    ),
                                }
                                .into());
                            }
                        } else {
                            self.aliases.insert(alias, target.clone());
                        }
                    }
                    child_ports.entry(formal).or_insert(target);
                }
                if self
                    .instances_to_descend
                    .as_ref()
                    .is_some_and(|instances| !instances.contains(&instance))
                {
                    continue;
                }
                let mut active_definitions = frame.active_definitions.clone();
                active_definitions.push(canonical_subcircuit);
                frames.push(Frame {
                    elements: &definition.elements,
                    next_index: 0,
                    prefix: instance,
                    parent_ports: child_ports,
                    active_definitions,
                    depth: frame.depth.saturating_add(1),
                });
            }
            Ok(())
        }
    }

    let definitions = netlist
        .subcircuits
        .iter()
        .map(|definition| (definition.name.to_ascii_uppercase(), definition))
        .collect();
    let globals = netlist
        .global_nodes
        .iter()
        .map(|node| canonical_symbol(node))
        .collect::<HashSet<_>>();
    let (instances_to_visit, instances_to_descend) = requested_instance_paths(requested_aliases);
    let mut collector = AliasCollector {
        definitions,
        external_subcircuits: Flattener::collect_external_subckts(netlist),
        globals: &globals,
        ground_policy: netlist.ground_policy(),
        aliases: HashMap::new(),
        requested_aliases,
        instances_to_visit,
        instances_to_descend,
        max_depth,
        max_elements,
        emitted_elements: 0,
        traversal_steps: 0,
    };
    collector.collect(&netlist.elements, abort)?;
    Ok(InterfaceNodeAliases {
        targets: collector.aliases,
    })
}

fn namespace_matches(namespace: &HashSet<String>, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') && !pattern.contains('?') {
        return namespace.contains(pattern);
    }
    namespace
        .iter()
        .any(|candidate| hierarchy_pattern_matches(pattern, candidate))
}

fn namespace_matches_with_aliases(
    namespace: &HashSet<String>,
    aliases: &InterfaceNodeAliases,
    pattern: &str,
) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern.contains('*') || pattern.contains('?') {
        return namespace
            .iter()
            .any(|candidate| hierarchy_pattern_matches(pattern, candidate))
            || aliases.keys().any(|alias| {
                hierarchy_pattern_matches(pattern, alias)
                    && resolved_alias_exists(namespace, aliases, alias)
            });
    }
    namespace.contains(pattern) || resolved_alias_exists(namespace, aliases, pattern)
}

fn resolved_alias_exists(
    namespace: &HashSet<String>,
    aliases: &InterfaceNodeAliases,
    alias: &str,
) -> bool {
    aliases
        .resolve(alias)
        .is_some_and(|target| namespace.contains(target))
}

fn hierarchy_pattern_matches(pattern: &str, candidate: &str) -> bool {
    let pattern = pattern.chars().collect::<Vec<_>>();
    let candidate = candidate.chars().collect::<Vec<_>>();
    let mut reachable = vec![false; candidate.len() + 1];
    reachable[0] = true;
    for token in pattern {
        let mut next = vec![false; candidate.len() + 1];
        if token == '*' {
            let mut active = false;
            for index in 0..=candidate.len() {
                active |= reachable[index];
                next[index] = active;
            }
        } else {
            for index in 0..candidate.len() {
                if reachable[index] && (token == '?' || token == candidate[index]) {
                    next[index + 1] = true;
                }
            }
        }
        reachable = next;
    }
    reachable[candidate.len()]
}

/// Extract recognized circuit accessors without interpreting arithmetic.
/// Unknown operators are deliberately excluded: operator support and symbol
/// existence are independent diagnostics.
pub(crate) fn extract_output_dependencies(source: &str) -> Vec<OutputSymbolDependency> {
    extract_output_dependencies_with_context(source, false)
}

fn extract_output_dependencies_with_context(
    source: &str,
    inherited_expression_context: bool,
) -> Vec<OutputSymbolDependency> {
    let bytes = source.as_bytes();
    // Determine expression membership once for the whole source. The previous
    // implementation rescanned `source[..operator_start]` for every accessor,
    // making a long `.PRINT` list quadratic. Real Xyce certification decks can
    // request tens of thousands of node voltages and device currents on one
    // line, so that repeated prefix scan looked like an indefinite hang.
    let expression_context = output_expression_context_by_byte(source);
    let mut dependencies = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b'@' {
            let start = index + 1;
            let mut end = start;
            while end < bytes.len() && is_symbol_char(bytes[end] as char) {
                end += 1;
            }
            // Device-parameter probes belong to parameter metadata
            // validation, not circuit symbol-existence validation.
            index = end.max(index + 1);
            continue;
        }
        if !(bytes[index] as char).is_ascii_alphabetic() {
            index += 1;
            continue;
        }
        let operator_start = index;
        index += 1;
        while index < bytes.len()
            && ((bytes[index] as char).is_ascii_alphanumeric() || bytes[index] == b'_')
        {
            index += 1;
        }
        let operator = source[operator_start..index].to_ascii_uppercase();
        let mut open = index;
        while open < bytes.len() && (bytes[open] as char).is_ascii_whitespace() {
            open += 1;
        }
        if open >= bytes.len() || bytes[open] != b'(' {
            continue;
        }
        let Some(close) = matching_parenthesis(bytes, open) else {
            continue;
        };
        let args = split_top_level_args(&source[open + 1..close]);
        let first_new_dependency = dependencies.len();
        let expression = inherited_expression_context || expression_context[operator_start];
        // `IF(device)` is a valid direct Xyce lead-current request, while
        // `IF(condition, then, else)` inside an authored expression is the
        // expression builtin and must be traversed recursively.
        let current_accessor =
            is_current_output_accessor(&operator) && !(expression && operator.as_str() == "IF");
        match operator.as_str() {
            "V" | "VR" | "VI" | "VM" | "VP" | "VDB" => {
                for arg in args.into_iter().take(2) {
                    push_dependency(&mut dependencies, &operator, arg, OutputSymbolKind::Node);
                }
            }
            "N" => {
                if let Some(arg) = args.first() {
                    push_dependency(&mut dependencies, &operator, arg, OutputSymbolKind::Node);
                }
            }
            "DNO" | "DNI" | "P" | "W" => {
                if let Some(arg) = args.first() {
                    push_dependency(&mut dependencies, &operator, arg, OutputSymbolKind::Device);
                }
            }
            _ if current_accessor => {
                if let Some(arg) = args.first() {
                    push_dependency(&mut dependencies, &operator, arg, OutputSymbolKind::Device);
                }
            }
            _ => {}
        }
        for dependency in &mut dependencies[first_new_dependency..] {
            dependency.expression = expression;
        }
        // Continue inside the argument list as well, so expressions nested in
        // ordinary arithmetic functions retain their circuit dependencies.
        if !matches!(
            operator.as_str(),
            "V" | "VR" | "VI" | "VM" | "VP" | "VDB" | "N" | "DNO" | "DNI" | "P" | "W"
        ) && !current_accessor
        {
            dependencies.extend(extract_output_dependencies_with_context(
                &source[open + 1..close],
                expression,
            ));
        }
        index = close + 1;
    }
    dependencies
}

fn push_dependency(
    dependencies: &mut Vec<OutputSymbolDependency>,
    operator: &str,
    symbol: &str,
    kind: OutputSymbolKind,
) {
    // Whitespace around hierarchy separators and wildcards is lexical, not
    // part of a circuit symbol. Xyce's legacy `I(YDEVICE BRANCH)` spelling is
    // the sole exception: its two tokens form one logical branch-current name.
    let authored = symbol.trim();
    let mut parts = authored.split_whitespace();
    let first = parts.next();
    let second = parts.next();
    let preserve_legacy_branch = operator == "I"
        && first.is_some()
        && second.is_some_and(|part| part.eq_ignore_ascii_case("BRANCH"))
        && parts.next().is_none();
    let symbol = if preserve_legacy_branch {
        format!(
            "{} {}",
            first.expect("checked above"),
            second.expect("checked above")
        )
    } else {
        authored.split_whitespace().collect::<String>()
    };
    if !symbol.is_empty() {
        dependencies.push(OutputSymbolDependency {
            operator: operator.to_string(),
            symbol,
            kind,
            expression: false,
        });
    }
}

fn output_expression_context_by_byte(source: &str) -> Vec<bool> {
    let mut context = vec![false; source.len() + 1];
    let mut braces = 0usize;
    let mut single_quote = false;
    let mut double_quote = false;
    for (index, byte) in source.bytes().enumerate() {
        context[index] = braces != 0 || single_quote || double_quote;
        match byte {
            b'{' if !single_quote && !double_quote => braces += 1,
            b'}' if !single_quote && !double_quote => braces = braces.saturating_sub(1),
            b'\'' if !double_quote => single_quote = !single_quote,
            b'"' if !single_quote => double_quote = !double_quote,
            _ => {}
        }
    }
    context[source.len()] = braces != 0 || single_quote || double_quote;
    context
}

fn matching_parenthesis(bytes: &[u8], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, byte) in bytes.iter().enumerate().skip(open) {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn split_top_level_args(source: &str) -> Vec<&str> {
    let bytes = source.as_bytes();
    let mut depth = 0usize;
    let mut start = 0usize;
    let mut args = Vec::new();
    for (index, byte) in bytes.iter().enumerate() {
        match byte {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => {
                args.push(source[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    args.push(source[start..].trim());
    args
}

fn is_symbol_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$' | '.' | ':' | '!' | '*' | '?')
}

#[derive(Clone, Copy)]
enum MeasureDependencySource<'a> {
    Direct(&'a str),
    Expression(&'a str),
}

fn collect_measure_sources<'a>(
    statement: &'a MeasureStatement,
    output: &mut Vec<MeasureDependencySource<'a>>,
) {
    use crate::netlist::measure::{MeasureOperand, MeasureType, TriggerEvent};

    fn condition<'a>(
        condition: &'a crate::netlist::measure::WhenCondition,
        output: &mut Vec<MeasureDependencySource<'a>>,
    ) {
        output.push(MeasureDependencySource::Direct(condition.left.as_str()));
        if let MeasureOperand::Waveform(source) = &condition.right {
            output.push(MeasureDependencySource::Direct(source.as_str()));
        }
    }
    fn trigger<'a>(
        trigger: &'a crate::netlist::measure::TrigSpec,
        output: &mut Vec<MeasureDependencySource<'a>>,
    ) {
        if let TriggerEvent::When(when) = &trigger.event {
            condition(when, output);
        }
    }
    match &statement.measure_type {
        MeasureType::Delay { trig, targ, .. } => {
            trigger(trig, output);
            trigger(targ, output);
        }
        MeasureType::Find { signal, when, .. } | MeasureType::Derivative { signal, when, .. } => {
            output.push(MeasureDependencySource::Direct(signal));
            if let Some(when) = when {
                condition(when, output);
            }
        }
        MeasureType::When {
            condition: when, ..
        } => condition(when, output),
        MeasureType::Param { expression } | MeasureType::Equation { expression, .. } => output
            .push(match expression.kind {
                crate::netlist::measure::MeasureExpressionKind::Expression => {
                    MeasureDependencySource::Expression(&expression.text)
                }
                crate::netlist::measure::MeasureExpressionKind::RawReference
                | crate::netlist::measure::MeasureExpressionKind::RawOutputOperator => {
                    MeasureDependencySource::Direct(&expression.text)
                }
            }),
        MeasureType::ErrorFunction {
            measured,
            comparison,
            ..
        } => {
            output.push(MeasureDependencySource::Direct(measured));
            output.push(MeasureDependencySource::Direct(comparison));
        }
        MeasureType::FileError { signal, .. }
        | MeasureType::Min { signal, .. }
        | MeasureType::Max { signal, .. }
        | MeasureType::PeakToPeak { signal, .. }
        | MeasureType::Avg { signal, .. }
        | MeasureType::Rms { signal, .. }
        | MeasureType::RiseTime { signal, .. }
        | MeasureType::FallTime { signal, .. }
        | MeasureType::Integ { signal, .. } => output.push(MeasureDependencySource::Direct(signal)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntactic_parse_remains_permissive_while_strict_phase_is_typed() {
        let source = "semantic phase\nV1 1 0 1\n.PRINT OP V(MISSING)\n.OP\n.END\n";
        let netlist = Netlist::parse(source).expect("syntactic parse accepts unresolved output");
        assert!(matches!(
            validate_output_symbols(&netlist),
            Err(ParseError::OutputSymbolValidation(_))
        ));
        assert!(matches!(
            Netlist::parse_validated(source),
            Err(ParseError::OutputSymbolValidation(_))
        ));
    }

    #[test]
    fn noise_analysis_owned_vectors_validate_only_in_the_noise_domain() {
        let source = "noise-owned output vectors\n\
                      V1 in 0 AC 1\n\
                      R1 in out 1k\n\
                      R2 out 0 1k\n\
                      .NOISE V(out) V1 DEC 1 1 10\n\
                      .PRINT NOISE V(inoise_spectrum) V(onoise)\n\
                      .SAVE V(inoise) V(onoise_spectrum)\n\
                      .END\n";
        let netlist = Netlist::parse_validated(source)
            .expect("noise-generated vectors are valid noise outputs");
        let print = netlist
            .output_requests
            .iter()
            .find(|request| request.directive == OutputDirectiveKind::Print)
            .expect("typed PRINT provenance exists");
        assert_eq!(print.analysis, Some(OutputAnalysisKind::Noise));

        let wrong_domain = source.replace(
            ".PRINT NOISE V(inoise_spectrum) V(onoise)",
            ".PRINT OP V(inoise_spectrum)",
        );
        let error = Netlist::parse_validated(&wrong_domain)
            .expect_err("noise-generated vectors are not OP topology nodes");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));

        let wrong_namespace = source.replace(
            ".PRINT NOISE V(inoise_spectrum) V(onoise)",
            ".PRINT NOISE I(inoise)",
        );
        let error = Netlist::parse_validated(&wrong_namespace)
            .expect_err("noise-generated vectors are not circuit devices");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));

        let no_noise_analysis = "missing noise producer\n\
                                 V1 in 0 1\n\
                                 R1 in 0 1k\n\
                                 .PRINT NOISE V(inoise_spectrum)\n\
                                 .END\n";
        let error = Netlist::parse_validated(no_noise_analysis)
            .expect_err("a qualifier alone does not create noise vectors");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));
    }

    #[test]
    fn dependency_extraction_preserves_repetitions_and_skips_unknown_operators() {
        let dependencies = extract_output_dependencies(
            "{V(bogo1)} VP(bogo9) VM(bogo9) VQ(a,b) @x1:m1[id] N(x2:m2:id)",
        );
        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| (
                    dependency.operator.as_str(),
                    dependency.symbol.as_str(),
                    dependency.kind,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("V", "bogo1", OutputSymbolKind::Node),
                ("VP", "bogo9", OutputSymbolKind::Node),
                ("VM", "bogo9", OutputSymbolKind::Node),
                ("N", "x2:m2:id", OutputSymbolKind::Node),
            ]
        );
    }

    #[test]
    fn continuous_measurements_retain_their_base_analysis_domain() {
        for (keyword, expected) in [
            ("TRAN_CONT", OutputAnalysisKind::Tran),
            ("DC_CONT", OutputAnalysisKind::Dc),
            ("AC_CONT", OutputAnalysisKind::Ac),
            ("NOISE_CONT", OutputAnalysisKind::Noise),
        ] {
            assert_eq!(OutputAnalysisKind::from_keyword(keyword), Some(expected));
        }
    }

    #[test]
    fn derived_current_and_noise_probes_retain_device_dependencies() {
        let dependencies = extract_output_dependencies(
            "IM(X1:R1) IR(X2:R2) II(X3:R3) IP(X4:R4) IDB(X5:R5) \
             DNO(X6:R6,thermal) DNI(X7:R7)",
        );
        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| (
                    dependency.operator.as_str(),
                    dependency.symbol.as_str(),
                    dependency.kind,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("IM", "X1:R1", OutputSymbolKind::Device),
                ("IR", "X2:R2", OutputSymbolKind::Device),
                ("II", "X3:R3", OutputSymbolKind::Device),
                ("IP", "X4:R4", OutputSymbolKind::Device),
                ("IDB", "X5:R5", OutputSymbolKind::Device),
                ("DNO", "X6:R6", OutputSymbolKind::Device),
                ("DNI", "X7:R7", OutputSymbolKind::Device),
            ]
        );
    }

    #[test]
    fn direct_current_accessors_accept_arbitrary_leads_without_capturing_expression_builtins() {
        for operator in [
            "I", "IR", "II", "IM", "IP", "IDB", "ID", "IG", "IS", "IB", "IC", "IE", "IA", "IT",
            "I1", "IF",
        ] {
            assert!(is_current_output_accessor(operator), "{operator}");
        }
        for function in ["IMG", "INT"] {
            assert!(!is_current_output_accessor(function), "{function}");
        }

        let dependencies =
            extract_output_dependencies("IA(XA) I1(X1) IF(XF) {IF(V(A), IMG(V(B)), INT(V(C)))}");
        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| (
                    dependency.operator.as_str(),
                    dependency.symbol.as_str(),
                    dependency.kind,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("IA", "XA", OutputSymbolKind::Device),
                ("I1", "X1", OutputSymbolKind::Device),
                ("IF", "XF", OutputSymbolKind::Device),
                ("V", "A", OutputSymbolKind::Node),
                ("V", "B", OutputSymbolKind::Node),
                ("V", "C", OutputSymbolKind::Node),
            ]
        );
    }

    #[test]
    fn parsed_measure_expressions_preserve_builtin_context_in_output_requests() {
        let netlist = Netlist::parse_with_options(
            "typed expression dependencies\n\
             V1 out 0 0\n\
             .TRAN 1 1\n\
             .MEASURE TRAN collision EQN {IF(V(out)>0,INT(V(out)),IMG(V(out)))}\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("typed expression measure parses");
        let request = netlist
            .output_requests
            .iter()
            .find(|request| {
                request
                    .name
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case("collision"))
            })
            .expect("measure output request retained");
        assert!(request.dependencies.iter().any(|dependency| {
            dependency.operator == "V"
                && dependency.symbol.eq_ignore_ascii_case("out")
                && dependency.kind == OutputSymbolKind::Node
        }));
        assert!(request.dependencies.iter().all(|dependency| {
            !(dependency.operator == "IF" && dependency.kind == OutputSymbolKind::Device)
        }));
    }

    #[test]
    fn dependency_symbols_compact_lexical_whitespace_except_legacy_branch_names() {
        let dependencies = extract_output_dependencies("V(XTOP. *) I(XTOP:R ?) I(YDEVICE BRANCH)");
        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| dependency.symbol.as_str())
                .collect::<Vec<_>>(),
            vec!["XTOP.*", "XTOP:R?", "YDEVICE BRANCH"]
        );
    }

    #[test]
    fn transient_capture_predicate_covers_current_projections_only() {
        for operator in ["I", "IR", "II", "IM", "IP", "IDB", "P", "W"] {
            let source = format!(
                "transient current selection\nR1 1 0 1\n.TRAN 1 1\n.PRINT TRAN {operator}(R1)\n.END\n"
            );
            let netlist = Netlist::parse(&source).expect("transient current request parses");
            let request = netlist
                .output_requests
                .last()
                .expect("PRINT request retained");
            assert!(
                request.requires_transient_device_current_operand(),
                "{operator}"
            );
            assert!(request.selects_transient_device_current("r1"), "{operator}");
        }

        for source in [
            "AC current is not transient\nR1 1 0 1\n.AC LIN 1 1 1\n.PRINT AC IR(R1)\n.END\n",
            "noise contribution is not a branch current\nR1 1 0 1\n.NOISE V(1) V1 LIN 1 1 1\n.PRINT NOISE DNO(R1,thermal)\n.END\n",
        ] {
            let netlist = Netlist::parse(source).expect("non-transient request parses");
            let request = netlist
                .output_requests
                .last()
                .expect("output request retained");
            assert!(!request.requires_transient_device_current_operand());
            assert!(!request.selects_transient_device_current("R1"));
        }
    }

    #[test]
    fn dependency_extraction_scales_to_certification_sized_probe_lists() {
        let source = (0..20_000)
            .map(|index| {
                if index % 2 == 0 {
                    format!("V(n{index})")
                } else {
                    format!("I(d{index})")
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        let dependencies = extract_output_dependencies(&source);

        assert_eq!(dependencies.len(), 20_000);
        assert!(dependencies.iter().all(|dependency| !dependency.expression));
        assert_eq!(dependencies[0].symbol, "n0");
        assert_eq!(dependencies[19_999].symbol, "d19999");
    }

    #[test]
    fn expression_context_is_preserved_by_linear_dependency_scan() {
        let dependencies =
            extract_output_dependencies("V(out) {V(expr)} 'I(quoted)' \"V(double_quoted)\"");

        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| (dependency.symbol.as_str(), dependency.expression))
                .collect::<Vec<_>>(),
            vec![
                ("out", false),
                ("expr", true),
                ("quoted", true),
                ("double_quoted", true),
            ]
        );
    }

    #[test]
    fn measure_and_four_requests_retain_authored_dependency_spelling() {
        let netlist = Netlist::parse(
            "authored output spelling\n\
             V1 1 0 1\n\
             .TRAN 0.1 1\n\
             .MEASURE TRAN mixedCase MAX V(bogoNode)\n\
             .FOUR 1k I(BogoDevice1) V(MixedNode)\n\
             .END\n",
        )
        .expect("syntactic parse accepts unresolved output dependencies");
        let authored = netlist
            .output_requests
            .iter()
            .flat_map(|request| request.dependencies.iter())
            .map(|dependency| (dependency.operator.as_str(), dependency.symbol.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            authored,
            vec![("V", "bogoNode"), ("I", "BogoDevice1"), ("V", "MixedNode")]
        );
    }

    #[test]
    fn save_override_requests_reuse_accessor_and_bare_vector_semantics() {
        let origin = NetlistSourceLocation::in_file("<command line --save>", 2);
        let current = OutputRequest::from_save_override(origin.clone(), "I(MissingDevice)");
        assert_eq!(current.directive, OutputDirectiveKind::Save);
        assert_eq!(current.origin, origin);
        assert_eq!(current.dependencies.len(), 1);
        assert_eq!(current.dependencies[0].operator, "I");
        assert_eq!(current.dependencies[0].symbol, "MissingDevice");
        assert_eq!(current.dependencies[0].kind, OutputSymbolKind::Device);

        let bare = OutputRequest::from_save_override(
            NetlistSourceLocation::in_file("<command line --save>", 3),
            "MissingNode",
        );
        assert_eq!(bare.dependencies.len(), 1);
        assert_eq!(bare.dependencies[0].operator, "V");
        assert_eq!(bare.dependencies[0].symbol, "MissingNode");
        assert_eq!(bare.dependencies[0].kind, OutputSymbolKind::Node);

        let parameter = OutputRequest::from_save_override(
            NetlistSourceLocation::in_file("<command line --save>", 4),
            "@m1[id]",
        );
        assert!(parameter.dependencies.is_empty());
        assert!(matches!(
            super::super::parse_save_probe("ALL"),
            Some(super::super::SaveSignal::All)
        ));
    }

    #[test]
    fn xyce_wildcards_cross_hierarchy_and_question_matches_one_character() {
        assert!(hierarchy_pattern_matches("X1.*", "X1.N1"));
        assert!(hierarchy_pattern_matches("X1.*", "X1.X2.N1"));
        assert!(hierarchy_pattern_matches("X1.*.*", "X1.X2.N1"));
        assert!(hierarchy_pattern_matches("X?.N1", "X1.N1"));
        assert!(!hierarchy_pattern_matches("X?.N1", "X12.N1"));
    }

    #[test]
    fn bug718_voltage_difference_and_ground_replacement_validate() {
        let netlist = Netlist::parse_validated(
            "BUG718 positive\n\
             .PREPROCESS REPLACEGROUND TRUE\n\
             .TRAN 0.1 1\n\
             .PRINT TRAN V(1,0) V(GND,1) {V(GROUND,1)} N(GND)\n\
             V1 1 0 1\n\
             R1 1 0 1\n\
             .END\n",
        )
        .expect("all Xyce ground synonyms resolve under REPLACEGROUND");
        assert_eq!(netlist.options.replace_ground, Some(true));
        assert!(netlist.saves.signals.iter().any(
            |signal| matches!(signal, super::super::SaveSignal::VoltageDiff(pos, _) if pos == "0")
        ));
    }

    #[test]
    fn expression_occurrences_remain_ordered_and_repeated() {
        let error = Netlist::parse_validated(
            "ordered output failures\n\
             V1 1 0 1\n\
             .AC DEC 1 1 10\n\
             .PRINT AC {VP(BOGO9)} {VM(BOGO9)}\n\
             .END\n",
        )
        .expect_err("both authored expression occurrences are invalid");
        let ParseError::OutputSymbolValidation(error) = error else {
            panic!("unexpected error: {error}");
        };
        assert_eq!(
            error
                .unresolved
                .iter()
                .map(|item| (item.operator.as_str(), item.symbol.as_str()))
                .collect::<Vec<_>>(),
            vec![("VP", "BOGO9"), ("VM", "BOGO9")]
        );
    }

    #[test]
    fn direct_requests_sort_devices_then_nodes_and_deduplicate() {
        let error = Netlist::parse_validated(
            "direct output failures\n\
             V1 1 0 1\n\
             .TRAN 0.1 1\n\
             .PRINT TRAN V(D) V(C) V(D) I(RBOGO) I(ABOGO)\n\
             .END\n",
        )
        .expect_err("direct symbols are absent");
        let ParseError::OutputSymbolValidation(error) = error else {
            panic!("unexpected error: {error}");
        };
        assert_eq!(
            error
                .unresolved
                .iter()
                .map(|item| item.symbol.as_str())
                .collect::<Vec<_>>(),
            vec!["ABOGO", "RBOGO", "C", "D"]
        );
    }

    #[test]
    fn qualified_formal_aliases_and_cross_hierarchy_wildcards_resolve() {
        Netlist::parse_validated(
            "alias outputs\n\
             V1 1 0 1\n\
             XTOP 1 0 DIV\n\
             .SUBCKT DIV A B\n\
             R1 A MID 1\n\
             R2 MID B 1\n\
             .ENDS\n\
             .PRINT DC V(XTOP:A) V(XTOP.*) I(XTOP:R?)\n\
             .DC V1 0 1 1\n\
             .END\n",
        )
        .expect("formal aliases, wildcard nodes, and wildcard devices resolve");
    }

    #[test]
    fn interface_aliases_resolve_nested_ports_case_separators_and_ground() {
        let netlist = Netlist::parse(
            "nested interface aliases\n\
             V1 1 0 1\n\
             XTOP 1 0 OUTER\n\
             .SUBCKT INNER P N\n\
             R1 P N 1\n\
             .ENDS\n\
             .SUBCKT OUTER A G\n\
             XINNER A G INNER\n\
             .ENDS\n\
             .END\n",
        )
        .expect("nested alias deck parses");

        let aliases = collect_interface_node_aliases(&netlist).expect("aliases collect");
        assert_eq!(aliases.resolve("xtop:a"), Some("1"));
        assert_eq!(aliases.resolve("XTOP.A"), Some("1"));
        assert_eq!(aliases.resolve("XTOP:XINNER:P"), Some("1"));
        assert_eq!(aliases.resolve("xtop.xinner.n"), Some("0"));
        assert_eq!(aliases.resolve("XTOP:UNKNOWN"), None);
    }

    #[test]
    fn interface_alias_resolution_is_one_hop() {
        let aliases = InterfaceNodeAliases {
            targets: HashMap::from([
                ("A".to_string(), "B".to_string()),
                ("B".to_string(), "C".to_string()),
            ]),
        };
        assert_eq!(aliases.resolve("a"), Some("B"));
        assert_eq!(aliases.resolve("b"), Some("C"));

        let physical_name_collision = InterfaceNodeAliases {
            targets: HashMap::from([("A".to_string(), "A".to_string())]),
        };
        assert_eq!(physical_name_collision.resolve("A"), Some("A"));
    }

    #[test]
    fn interface_alias_collection_rejects_recursive_hierarchy() {
        let netlist = Netlist::parse(
            "recursive aliases\n\
             X1 1 0 LOOP\n\
             .SUBCKT LOOP A B\n\
             XSELF A B LOOP\n\
             .ENDS\n\
             .END\n",
        )
        .expect("syntactic parser retains recursive hierarchy");

        let error = collect_interface_node_aliases(&netlist)
            .expect_err("runtime alias collection must reject recursion");
        assert!(
            matches!(&error, ParseError::Syntax { message, .. } if message.contains("Recursive subcircuit instantiation")),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn requested_interface_alias_collection_omits_unreferenced_ports_and_instances() {
        let netlist = Netlist::parse(
            "selected aliases\n\
             X1 1 0 CELL\n\
             X2 2 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .END\n",
        )
        .expect("selection deck parses");
        let requested = HashSet::from(["X1:A".to_string()]);

        let aliases = collect_requested_interface_node_aliases(&netlist, &requested)
            .expect("selected aliases collect");

        assert_eq!(aliases.iter().count(), 1);
        assert_eq!(aliases.resolve("X1.A"), Some("1"));
        assert_eq!(aliases.resolve("X1.B"), None);
        assert_eq!(aliases.resolve("X2.A"), None);
    }

    #[test]
    fn requested_interface_alias_collection_prunes_empty_and_unrelated_paths() {
        let recursive = Netlist::parse(
            "unrequested recursive hierarchy\n\
             X1 1 LOOP\n\
             .SUBCKT LOOP A\n\
             XSELF A LOOP\n\
             .ENDS\n\
             .END\n",
        )
        .expect("recursive hierarchy parses syntactically");
        let empty = collect_requested_interface_node_aliases(&recursive, &HashSet::new())
            .expect("empty projection does not elaborate hierarchy");
        assert_eq!(empty.iter().count(), 0);

        let netlist = Netlist::parse(
            "prefix-exact selected aliases\n\
             X1 1 MISSING\n\
             X10 10 CELL\n\
             .SUBCKT CELL A\n\
             R1 A 0 1\n\
             .ENDS\n\
             .END\n",
        )
        .expect("selected hierarchy parses syntactically");
        let selected = HashSet::from(["X10:A".to_string()]);
        let aliases = collect_requested_interface_node_aliases(&netlist, &selected)
            .expect("X10 selection must not traverse X1");
        assert_eq!(aliases.resolve("X10.A"), Some("10"));

        let invalid = HashSet::from(["X1:A".to_string()]);
        assert!(matches!(
            collect_requested_interface_node_aliases(&netlist, &invalid),
            Err(ParseError::Syntax { message, .. }) if message.contains("Undefined subcircuit")
        ));
    }

    #[test]
    fn requested_interface_aliases_share_typed_port_binding_validation() {
        let netlist = Netlist::parse(
            "duplicate formal binding\n\
             X1 1 2 CELL\n\
             .SUBCKT CELL A A\n\
             R1 A 0 1\n\
             .ENDS\n\
             .END\n",
        )
        .expect("duplicate formal hierarchy parses syntactically");
        let requested = HashSet::from(["X1:A".to_string()]);
        assert!(matches!(
            collect_requested_interface_node_aliases(&netlist, &requested),
            Err(ParseError::DuplicateSubcircuitPortBinding(_))
        ));
    }

    #[test]
    fn exhaustive_alias_resource_accounting_matches_flattened_leaf_count() {
        let one_leaf = Netlist::parse(
            "one flattened leaf\n\
             X1 1 OUTER\n\
             .SUBCKT INNER A\n\
             R1 A 0 1\n\
             .ENDS\n\
             .SUBCKT OUTER A\n\
             X2 A INNER\n\
             .ENDS\n\
             .END\n",
        )
        .expect("one-leaf hierarchy parses");
        crate::netlist::finish_non_aborting_parse(collect_interface_node_aliases_impl_with_limits(
            &one_leaf,
            None,
            &crate::abort_signal::NoAbort,
            100,
            1,
        ))
        .expect("two X containers plus one resistor emit one leaf");

        let two_leaves = Netlist::parse(
            "two flattened leaves\n\
             X1 1 CELL\n\
             .SUBCKT CELL A\n\
             R1 A 0 1\n\
             R2 A 0 2\n\
             .ENDS\n\
             .END\n",
        )
        .expect("two-leaf hierarchy parses");
        let error = crate::netlist::finish_non_aborting_parse(
            collect_interface_node_aliases_impl_with_limits(
                &two_leaves,
                None,
                &crate::abort_signal::NoAbort,
                100,
                1,
            ),
        )
        .expect_err("second emitted resistor exceeds the one-leaf limit");
        assert!(matches!(
            error,
            ParseError::ResourceLimit(crate::resource::ResourceLimitError {
                resource: crate::resource::ResourceKind::FlattenedElements,
                requested: 2,
                limit: 1,
            })
        ));
    }

    #[test]
    fn interface_alias_collection_accepts_the_exact_hierarchy_depth_limit() {
        let mut source = String::from("allowed alias depth\nXROOT 1 0 S0\n");
        for depth in 0..=99 {
            source.push_str(&format!(".SUBCKT S{depth} A B\n"));
            if depth < 99 {
                source.push_str(&format!("XNEXT A B S{}\n", depth + 1));
            } else {
                source.push_str("R1 A B 1\n");
            }
            source.push_str(".ENDS\n");
        }
        source.push_str(".END\n");
        let netlist = Netlist::parse(&source).expect("boundary hierarchy parses");

        collect_interface_node_aliases(&netlist)
            .expect("an element at hierarchy depth 100 is permitted");
    }

    #[test]
    fn interface_alias_collection_enforces_hierarchy_depth_limit() {
        let mut source = String::from("deep aliases\nXROOT 1 0 S0\n");
        for depth in 0..=100 {
            source.push_str(&format!(".SUBCKT S{depth} A B\n"));
            if depth < 100 {
                source.push_str(&format!("XNEXT A B S{}\n", depth + 1));
            } else {
                source.push_str("R1 A B 1\n");
            }
            source.push_str(".ENDS\n");
        }
        source.push_str(".END\n");
        let netlist = Netlist::parse(&source).expect("deep hierarchy parses syntactically");

        let error = collect_interface_node_aliases(&netlist)
            .expect_err("runtime alias collection must enforce depth limits");
        assert!(
            matches!(
                &error,
                ParseError::ResourceLimit(crate::resource::ResourceLimitError {
                    resource: crate::resource::ResourceKind::HierarchyDepth,
                    requested: 101,
                    limit: 100,
                })
            ),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn n_operator_resolves_interface_aliases_before_excluding_device_parameters() {
        Netlist::parse_validated(
            "N operator ambiguity\n\
             V1 1 0 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             M1 A A B B NM\n\
             .ENDS\n\
             .MODEL NM NMOS LEVEL=1\n\
             .PRINT OP N(X1:A) N(X1:M1:id)\n\
             .OP\n\
             .END\n",
        )
        .expect("formal node aliases resolve while hierarchical device parameters are excluded");
    }

    #[test]
    fn n_operator_defers_existing_device_internal_and_branch_vectors_to_execution() {
        Netlist::parse_validated(
            "device-owned vectors\n\
             VSRC_1 1 0 1\n\
             M1 1 1 0 0 NM\n\
             .MODEL NM NMOS LEVEL=1\n\
             .PRINT OP N(M1_t) N(VSRC_1_BRANCH)\n\
             .OP\n\
             .END\n",
        )
        .expect("generated internal-node and branch-vector metadata remains execution-owned");

        let error = Netlist::parse_validated(
            "unknown device vector\n\
             V1 1 0 1\n\
             .PRINT OP N(BOGO_BRANCH)\n\
             .OP\n\
             .END\n",
        )
        .expect_err("unknown device prefixes remain unresolved");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));
    }

    #[test]
    fn n_operator_does_not_exclude_parameters_for_unknown_hierarchical_devices() {
        let error = Netlist::parse_validated(
            "N operator unknown device\n\
             V1 1 0 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             M1 A A B B NM\n\
             .ENDS\n\
             .MODEL NM NMOS LEVEL=1\n\
             .PRINT OP N(X1:BOGO:id)\n\
             .OP\n\
             .END\n",
        )
        .expect_err("an unknown device prefix remains an unresolved node dependency");
        let ParseError::OutputSymbolValidation(error) = error else {
            panic!("unexpected error: {error}");
        };
        assert_eq!(error.unresolved.len(), 1);
        assert_eq!(error.unresolved[0].operator, "N");
        assert_eq!(error.unresolved[0].symbol, "X1:BOGO:id");
        assert_eq!(error.unresolved[0].kind, OutputSymbolKind::Node);
    }

    #[test]
    fn gnd_is_an_ordinary_node_without_replaceground() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..Default::default()
        };
        Netlist::parse_validated_with_options(
            "defined GND\nV1 GND 0 1\n.PRINT OP V(GND)\n.OP\n.END\n",
            options,
        )
        .expect("an authored GND node is valid without replacement");
        let error = Netlist::parse_validated_with_options(
            "undefined GND\nV1 1 0 1\n.PRINT OP V(GND)\n.OP\n.END\n",
            options,
        )
        .expect_err("GND is not implicit without replacement");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));
    }

    #[test]
    fn ngspice_default_aliases_only_exact_gnd() {
        Netlist::parse_validated("ngspice GND\nV1 1 0 1\n.PRINT OP V(GND)\n.OP\n.END\n")
            .expect("ngspice's exact GND alias resolves to zero");
        for alias in ["GND!", "GROUND"] {
            let source =
                format!("ngspice ordinary alias\nV1 1 0 1\n.PRINT OP V({alias})\n.OP\n.END\n");
            assert!(
                matches!(
                    Netlist::parse_validated(&source),
                    Err(ParseError::OutputSymbolValidation(_))
                ),
                "{alias} must remain an ordinary undefined node in ngspice mode"
            );
        }
    }

    #[test]
    fn replaceground_preserves_output_provenance_and_normalizes_typed_execution_fields() {
        let source = "execution output ground aliases\n\
                      V1 out 0 1\n\
                      .PRINT DC {V(GROUND)+V(GND!)}\n\
                      .MEAS DC M PARAM='{V(GROUND)+V(GND!)}'\n\
                      .FOUR 1k V(GROUND) V(GND!)\n\
                      .PREPROCESS REPLACEGROUND TRUE\n\
                      .END\n";
        let netlist =
            Netlist::parse_validated(source).expect("all ground-alias output forms parse");

        assert!(
            netlist
                .source_text
                .as_deref()
                .is_some_and(|text| { text.contains("V(GROUND)") && text.contains("V(GND!)") })
        );
        assert!(netlist.output_requests.iter().any(|request| {
            request
                .dependencies
                .iter()
                .any(|dependency| dependency.symbol.eq_ignore_ascii_case("GROUND"))
        }));
        assert!(netlist.output_requests.iter().any(|request| {
            request
                .dependencies
                .iter()
                .any(|dependency| dependency.symbol.eq_ignore_ascii_case("GND!"))
        }));
        let crate::netlist::measure::MeasureType::Param { expression } =
            &netlist.measurements[0].measure_type
        else {
            panic!("expected PARAM measurement");
        };
        assert_eq!(expression.text, "{V(0)+V(0)}");
        let outputs = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                super::super::AnalysisCommand::Four { outputs, .. } => Some(outputs),
                _ => None,
            })
            .expect("FOUR analysis exists");
        assert_eq!(outputs, &["V(0)", "V(0)"]);
    }

    #[test]
    fn late_root_replaceground_applies_to_xline_actuals_before_flattening() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..Default::default()
        };
        let netlist = Netlist::parse_validated_with_options(
            "late replacement\n\
             X1 1 GND DIV\n\
             .SUBCKT DIV A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .END\n\
             .PREPROCESS REPLACEGROUND TRUE\n",
            options,
        )
        .expect("root prescan observes a late card after END");
        let flattened = super::super::flatten_netlist(&netlist).expect("subcircuit flattens");
        assert_eq!(flattened[0].nodes, vec!["1", "0"]);
    }

    #[test]
    fn explicit_false_keeps_xline_gnd_as_an_ordinary_node() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..Default::default()
        };
        let netlist = Netlist::parse_validated_with_options(
            "false replacement\n\
             .PREPROCESS REPLACEGROUND FALSE\n\
             X1 1 GND DIV\n\
             .SUBCKT DIV A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .END\n",
            options,
        )
        .expect("FALSE is accepted");
        assert_eq!(netlist.options.replace_ground, None);
        let flattened = super::super::flatten_netlist(&netlist).expect("subcircuit flattens");
        assert_eq!(flattened[0].nodes, vec!["1", "GND"]);
    }
}
