//! Subcircuit flattening - converts hierarchical netlists to flat element lists
//!
//! This module handles the expansion of X (subcircuit instance) elements by:
//! 1. Looking up the subcircuit definition
//! 2. Renaming internal nodes to prevent collisions
//! 3. Mapping external ports to instance connections
//! 4. Recursively handling nested subcircuits
//!
//! # Hierarchy Path Tracking
//!
//! The flattener now uses `HierarchyPath` to track the current position in
//! the design hierarchy. This enables:
//! - Fully-qualified element names (X1.X2.R1)
//! - Hierarchical node naming for waveform access
//! - Proper parameter scoping with precedence resolution

use super::expr::{
    behavioral_expression_references_runtime_quantity, prepare_behavioral_expression,
    prepare_behavioral_expression_preserving_parameters,
    prepare_behavioral_expression_preserving_spelling,
    validate_prepared_behavioral_runtime_expression,
};
use super::hierarchy_path::HierarchyPath;
use super::param_scope::ParamResolver;
use super::parser::grouped_source_expression;
use super::remove_unused::filter_elements_with_abort as filter_removeunused_elements_with_abort;
use super::{
    DeviceInitialConditionDirective, DeviceInitialConditionError, DeviceInitialConditionSource,
    DuplicateSubcircuitPortBindingError, Element, ElementKind, GlobalSubcircuitPortBindingError,
    InitialCondition, ModelDef, Netlist, NodeSet, ParamContext, ParameterRedefinitionPolicy,
    ParametricValue, ParseError, ParseWithAbortError, RandomState, SourceMultiplicity, SourceSpec,
    StartupDirectiveDisposition, StartupDirectiveRecord, StartupDirectiveScope, SubcircuitDef,
    UndefinedSubcircuitError, ensure_parse_not_aborted, finish_non_aborting_parse,
    poll_parse_abort, validate_mutual_inductor_references,
};
use super::{ElementParameterDirection, ParameterDirectionCapture};
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::config::ExpressionDialect;
use crate::expr::Derivative;
use crate::resource::{ResourceKind, ResourceLimitError};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

fn format_xspice_complex_component(value: Value) -> String {
    let formatted = value.to_string();
    formatted
        .strip_suffix(".0")
        .unwrap_or(formatted.as_str())
        .to_string()
}

//=============================================================================
// Flattener Configuration
//=============================================================================

/// Configuration options for subcircuit flattening
#[derive(Debug, Clone)]
pub struct FlattenerConfig {
    /// Maximum recursion depth to prevent infinite loops
    pub max_depth: usize,
    /// Maximum number of leaf device records emitted after expansion.
    pub max_elements: usize,
    /// Preserve hierarchical node names for debugging (X1.X2.node format)
    /// When true, internal nodes keep the full hierarchical path
    /// When false, uses shorter hash-based names for efficiency
    pub preserve_hierarchy: bool,
    /// Separator character for hierarchical names (default: '.')
    pub hierarchy_separator: char,
    /// Whether to collect hierarchy metadata during flattening
    pub collect_metadata: bool,
}

impl Default for FlattenerConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            max_elements: crate::resource::ResourceLimits::default().max_flattened_elements,
            preserve_hierarchy: true, // Default to full path for debugging
            hierarchy_separator: '.',
            collect_metadata: false,
        }
    }
}

impl FlattenerConfig {
    /// Create a config optimized for debugging (full hierarchical names)
    pub fn debug() -> Self {
        Self {
            max_depth: 100,
            max_elements: crate::resource::ResourceLimits::default().max_flattened_elements,
            preserve_hierarchy: true,
            hierarchy_separator: '.',
            collect_metadata: true,
        }
    }

    /// Create a config optimized for performance (shorter names)
    pub fn production() -> Self {
        Self {
            max_depth: 100,
            max_elements: crate::resource::ResourceLimits::default().max_flattened_elements,
            preserve_hierarchy: false,
            hierarchy_separator: '_',
            collect_metadata: false,
        }
    }

    /// Create a standard config
    pub fn spectre() -> Self {
        Self {
            max_depth: 256,
            max_elements: crate::resource::ResourceLimits::default().max_flattened_elements,
            preserve_hierarchy: true,
            hierarchy_separator: '.',
            collect_metadata: true,
        }
    }
}

//=============================================================================
// Hierarchy Metadata
//=============================================================================

/// Metadata about a flattened instance for hierarchy navigation
#[derive(Debug, Clone)]
pub struct InstanceMetadata {
    /// Full hierarchical path to this instance
    pub path: HierarchyPath,
    /// Subcircuit definition name
    pub subcircuit_name: String,
    /// Instance parameters (overrides)
    pub instance_params: Vec<(String, Value)>,
    /// Child instances within this instance
    pub children: Vec<String>,
}

/// Result of flattening a netlist, including any instance-scoped model cards
/// needed by parameterized subcircuits.
#[derive(Debug, Clone)]
pub struct FlattenedNetlist {
    pub elements: Vec<Element>,
    pub scoped_models: Vec<ModelDef>,
    pub scoped_initial_conditions: Vec<InitialCondition>,
    pub scoped_node_sets: Vec<NodeSet>,
    /// Per-instance startup provenance with concrete qualified nodes.
    pub scoped_startup_directives: Vec<StartupDirectiveRecord>,
    pub xspice_auto_bridge_node_hints: Vec<XspiceAutoBridgeNodeHint>,
}

/// Scope-derived parameter hint for one flattened XSPICE digital node.
#[derive(Debug, Clone)]
pub struct XspiceAutoBridgeNodeHint {
    pub node: String,
    pub depth: usize,
    pub vcc: Option<Value>,
    pub family: Option<String>,
}

//=============================================================================
// Flattener
//=============================================================================

/// Validate one subcircuit invocation after its actual nodes have been mapped
/// through the parent scope. Both hierarchy flattening and selective interface
/// alias projection use this routine so duplicate/global/arity diagnostics
/// retain identical ordering and typed error payloads.
pub(super) fn validate_subcircuit_port_bindings(
    subcircuit: &SubcircuitDef,
    names: SubcircuitInstanceNames<'_>,
    actual_count: usize,
    mapped_ports: &[(&String, String)],
    global_nodes: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let SubcircuitInstanceNames {
        invoked_subcircuit_name,
        instance_name,
        qualified_instance_name,
    } = names;
    let mut first_bindings = HashMap::<String, (usize, &str)>::new();
    for (index, (formal, actual)) in mapped_ports.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        let canonical_formal = formal.to_ascii_uppercase();
        if let Some((first_index, first_actual)) = first_bindings.get(&canonical_formal) {
            if !first_actual.eq_ignore_ascii_case(actual) {
                return Err(ParseError::DuplicateSubcircuitPortBinding(Box::new(
                    DuplicateSubcircuitPortBindingError {
                        subcircuit_name: subcircuit.name.clone(),
                        canonical_subcircuit_name: subcircuit.name.to_ascii_uppercase(),
                        instance_name: instance_name.to_string(),
                        canonical_instance_name: instance_name.to_ascii_uppercase(),
                        qualified_instance_name: qualified_instance_name.to_string(),
                        formal_port: formal.to_string(),
                        first_position: first_index + 1,
                        conflicting_position: index + 1,
                        first_actual_node: (*first_actual).to_string(),
                        conflicting_actual_node: actual.clone(),
                    },
                ))
                .into());
            }
        } else {
            let formal_is_global = global_nodes.contains(&canonical_formal)
                || formal
                    .get(..2)
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("$G"));
            if formal_is_global && !formal.eq_ignore_ascii_case(actual) {
                return Err(ParseError::GlobalSubcircuitPortBinding(Box::new(
                    GlobalSubcircuitPortBindingError {
                        subcircuit_name: subcircuit.name.clone(),
                        canonical_subcircuit_name: subcircuit.name.to_ascii_uppercase(),
                        instance_name: instance_name.to_string(),
                        canonical_instance_name: instance_name.to_ascii_uppercase(),
                        qualified_instance_name: qualified_instance_name.to_string(),
                        formal_port: formal.to_string(),
                        position: index + 1,
                        actual_node: actual.clone(),
                    },
                ))
                .into());
            }
            first_bindings.insert(canonical_formal, (index, actual.as_str()));
        }
    }

    // Xyce validates every available duplicate/global binding before
    // connection count and recursion.
    if actual_count != subcircuit.ports.len() {
        return Err(ParseError::Syntax {
            line: 0,
            message: format!(
                "Subcircuit instance '{}' connects {} node(s) but '{}' declares {} port(s): {}",
                qualified_instance_name,
                actual_count,
                invoked_subcircuit_name,
                subcircuit.ports.len(),
                subcircuit.ports.join(" ")
            ),
        }
        .into());
    }
    Ok(())
}

/// The three names one subcircuit instantiation carries: the `.SUBCKT` it
/// invokes, the instance's own name, and the fully qualified name it takes in
/// the flattened deck. Diagnostics read all three, and passing them as bare
/// `&str`s made a swapped pair invisible.
#[derive(Clone, Copy)]
pub(super) struct SubcircuitInstanceNames<'a> {
    pub invoked_subcircuit_name: &'a str,
    pub instance_name: &'a str,
    pub qualified_instance_name: &'a str,
}

/// Where the flattener currently is: the name prefix it prepends, the node
/// renaming in force, the parameter scope, and how deep the instantiation
/// stack is.
#[derive(Clone, Copy)]
struct FlattenScope<'a> {
    prefix: &'a str,
    node_map: &'a HashMap<String, String>,
    scope: &'a ParamContext,
    depth: usize,
}

/// Flattens a hierarchical netlist into a flat element list
///
/// This is the core hierarchy processor that converts subcircuit instances
/// into their constituent elements while managing:
/// - Unique node naming
/// - Parameter inheritance and override
/// - Local options propagation
pub struct Flattener<'a> {
    /// Subcircuit definitions indexed by name
    subcircuits: HashMap<String, &'a SubcircuitDef>,
    /// Original model definitions, used to clone parameterized local models
    /// when subcircuits are expanded with instance-specific parameter scopes.
    models: &'a [ModelDef],
    /// Configuration options
    config: FlattenerConfig,
    /// Parameter resolver for scoped parameter lookup
    param_resolver: ParamResolver,
    /// Collected instance metadata (if collect_metadata is enabled)
    instance_metadata: Vec<InstanceMetadata>,
    /// External subcircuit/model names backed by out-of-line implementations
    /// (for example `.VERILOGA` includes).
    external_subckts: HashSet<String>,
    /// Unselected Verilog-A imports may supply additional names at binding.
    defer_external_module_binding: bool,
    /// Global nodes that must not be renamed while flattening hierarchy.
    global_nodes: HashSet<String>,
    /// Xyce's explicit ground-synonym preprocessing policy.
    ground_policy: super::GroundPolicy,
    /// Definitions currently being expanded, outermost first. A definition
    /// re-entered while still on this stack is a recursive instantiation,
    /// reported with the full cycle instead of running into `max_depth`.
    expansion_stack: Vec<String>,
    /// Netlist-wide statistical-function stream (shared draw counter), so
    /// per-instance expression draws are distinct yet reproducible.
    random: RandomState,
    /// Model cards cloned while flattening parameterized subcircuit instances.
    scoped_models: Vec<ModelDef>,
    /// Exact source-model index for each scoped clone, kept in lockstep with
    /// `scoped_models` so diagnostics never infer provenance from a generated
    /// hierarchy suffix or conflate duplicate names.
    scoped_model_sources: Vec<usize>,
    /// Diagnostic-only mode that isolates ordinary hierarchy failures between
    /// sibling instances while still propagating cancellation and resource
    /// limits. Normal circuit flattening always leaves this disabled.
    diagnostic_best_effort: bool,
    /// Startup directives scoped while flattening subcircuit instances.
    scoped_initial_conditions: Vec<InitialCondition>,
    scoped_node_sets: Vec<NodeSet>,
    startup_directives: Vec<StartupDirectiveRecord>,
    scoped_startup_directives: Vec<StartupDirectiveRecord>,
    /// Digital XSPICE nodes with the effective scope-local VCC value.
    xspice_auto_bridge_node_hints: Vec<XspiceAutoBridgeNodeHint>,
    /// Scope parameter used for digital auto-bridge voltage levels.
    xspice_auto_bridge_digital_param_name: String,
    /// Directory of the parsed deck, used to resolve scoped XSPICE file params.
    source_base_dir: Option<PathBuf>,
    /// Case-insensitive element identities already emitted during the current
    /// flatten operation. This also protects names generated by deferred
    /// per-instance lowering from colliding with authored elements.
    flattened_element_names: HashSet<String>,
    parameter_direction: Option<Box<ParameterDirectionCapture>>,
}

impl<'a> Flattener<'a> {
    /// Create a new flattener with the given subcircuit definitions
    pub fn new(subcircuits: &'a [SubcircuitDef]) -> Self {
        Self::with_config(subcircuits, FlattenerConfig::default())
    }

    /// Create a flattener with custom configuration
    pub fn with_config(subcircuits: &'a [SubcircuitDef], config: FlattenerConfig) -> Self {
        Self::with_models_config(subcircuits, &[], config)
    }

    /// Create a flattener with model definitions available for scoped cloning.
    pub fn with_models_config(
        subcircuits: &'a [SubcircuitDef],
        models: &'a [ModelDef],
        config: FlattenerConfig,
    ) -> Self {
        let foundation_subcircuits = super::foundation_subcircuits();
        let subcircuit_map: HashMap<String, &SubcircuitDef> = foundation_subcircuits
            .iter()
            .chain(subcircuits)
            .map(|s| (s.name.to_ascii_uppercase(), s))
            .collect();

        // Initialize param resolver with subcircuit defaults
        let mut param_resolver = ParamResolver::new();
        for subckt in foundation_subcircuits.iter().filter(|foundation| {
            !subcircuits
                .iter()
                .any(|declared| declared.name.eq_ignore_ascii_case(&foundation.name))
        }) {
            param_resolver.add_subcircuit_defaults(&subckt.name, &subckt.params);
        }
        for subckt in subcircuits {
            param_resolver.add_subcircuit_defaults(&subckt.name, &subckt.params);
        }

        Self {
            subcircuits: subcircuit_map,
            models,
            config,
            param_resolver,
            instance_metadata: Vec::new(),
            external_subckts: HashSet::new(),
            defer_external_module_binding: false,
            global_nodes: HashSet::new(),
            ground_policy: super::GroundPolicy::OnlyZero,
            expansion_stack: Vec::new(),
            random: RandomState::default(),
            scoped_models: Vec::new(),
            scoped_model_sources: Vec::new(),
            diagnostic_best_effort: false,
            scoped_initial_conditions: Vec::new(),
            scoped_node_sets: Vec::new(),
            startup_directives: Vec::new(),
            scoped_startup_directives: Vec::new(),
            xspice_auto_bridge_node_hints: Vec::new(),
            xspice_auto_bridge_digital_param_name: "vcc".to_string(),
            source_base_dir: None,
            flattened_element_names: HashSet::new(),
            parameter_direction: None,
        }
    }

    /// Get the collected instance metadata (after flattening)
    pub fn instance_metadata(&self) -> &[InstanceMetadata] {
        &self.instance_metadata
    }

    /// Get a reference to the parameter resolver
    pub fn param_resolver(&self) -> &ParamResolver {
        &self.param_resolver
    }

    /// Resolved instance-scoped models paired with their source definitions.
    pub(crate) fn scoped_models_with_sources(&self) -> impl Iterator<Item = (&ModelDef, usize)> {
        debug_assert_eq!(self.scoped_models.len(), self.scoped_model_sources.len());
        self.scoped_models
            .iter()
            .zip(&self.scoped_model_sources)
            .map(|(model, source)| (model, *source))
    }

    /// Collect instance-scoped model resolutions for parser diagnostics.
    ///
    /// Ordinary hierarchy errors are isolated to the failing branch so a
    /// broken sibling cannot hide diagnostics from a concrete, valid model
    /// route. Cancellation and configured resource limits remain hard errors.
    /// This mode deliberately reuses the production scope/substitution logic;
    /// it does not alter the behavior of normal flattening.
    pub(crate) fn collect_scoped_models_for_diagnostics_with_abort(
        &mut self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let previous_mode = self.diagnostic_best_effort;
        self.diagnostic_best_effort = true;
        let result = self.flatten_with_abort(netlist, abort).map(|_| ());
        self.diagnostic_best_effort = previous_mode;
        result
    }

    fn handle_branch_error(
        &self,
        error: ParseWithAbortError,
        context: &str,
    ) -> Result<(), ParseWithAbortError> {
        match error {
            error @ ParseWithAbortError::Aborted
            | error @ ParseWithAbortError::Parse(ParseError::ResourceLimit(_)) => Err(error),
            ParseWithAbortError::Parse(error) if self.diagnostic_best_effort => {
                log::debug!(
                    "skipping invalid hierarchy branch while collecting scoped model diagnostics ({context}): {error}"
                );
                Ok(())
            }
            error => Err(error),
        }
    }

    /// Flatten a netlist, expanding all subcircuit instances
    pub fn flatten(&mut self, netlist: &Netlist) -> Result<Vec<Element>, ParseError> {
        finish_non_aborting_parse(self.flatten_with_abort(netlist, &NoAbort))
    }

    /// Flatten a netlist with cooperative cancellation between hierarchy
    /// records and recursive expansion steps.
    pub(crate) fn flatten_with_abort(
        &mut self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Element>, ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        if let Err(error) = validate_mutual_inductor_references(netlist) {
            self.handle_branch_error(error.into(), "mutual-inductor validation")?;
        }
        let mut flat_elements = Vec::new();
        self.parameter_direction = netlist.parameter_direction.clone();
        self.external_subckts = Self::collect_external_subckts(netlist);
        self.defer_external_module_binding = netlist.needs_veriloga_module_discovery();
        self.global_nodes = netlist
            .global_nodes
            .iter()
            .map(|node| node.to_ascii_uppercase())
            .collect();
        self.ground_policy = netlist.ground_policy();
        self.expansion_stack.clear();
        self.scoped_models.clear();
        self.scoped_model_sources.clear();
        self.scoped_initial_conditions.clear();
        self.scoped_node_sets.clear();
        self.startup_directives = netlist.startup_directives.clone();
        self.scoped_startup_directives.clear();
        self.xspice_auto_bridge_node_hints.clear();
        self.flattened_element_names.clear();
        self.xspice_auto_bridge_digital_param_name = netlist
            .options
            .auto_bridge_param_name("d")
            .unwrap_or("vcc")
            .to_string();
        self.source_base_dir = netlist
            .source_path
            .as_ref()
            .and_then(|path| path.parent())
            .map(Path::to_path_buf);
        // Continue the netlist's statistical draw sequence (seeded at parse
        // time) so flatten-time draws are distinct per instance.
        self.random = netlist.params.random().clone();
        let mut global_scope = netlist.params.clone();
        global_scope.adopt_random(&self.random);

        // Set global parameters from netlist
        for (name, value) in netlist.params.all_params() {
            self.param_resolver.set_global(&name, value);
        }

        for (element_index, element) in netlist.elements.iter().enumerate() {
            poll_parse_abort(abort, element_index)?;
            let stack_len = self.expansion_stack.len();
            if let Err(error) = self.flatten_element(
                element,
                FlattenScope {
                    prefix: "",
                    node_map: &HashMap::new(),
                    scope: &global_scope,
                    depth: 0,
                },
                &mut flat_elements,
                abort,
            ) {
                self.expansion_stack.truncate(stack_len);
                self.handle_branch_error(error, &format!("top-level element {}", element.name))?;
            }
        }

        if self.diagnostic_best_effort {
            return Ok(flat_elements);
        }

        self.validate_generated_internal_node_collisions(&flat_elements, abort)?;

        if let Some(policy) = netlist.options.remove_unused.as_ref() {
            flat_elements = filter_removeunused_elements_with_abort(&flat_elements, policy, abort)?;
        }

        apply_device_initial_conditions(
            netlist.device_initial_conditions.as_ref(),
            &mut flat_elements,
        )?;
        Ok(flat_elements)
    }

    /// Flatten a single element, recursively expanding subcircuits
    fn flatten_element(
        &mut self,
        element: &Element,
        scope: FlattenScope<'_>,
        output: &mut Vec<Element>,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let FlattenScope {
            prefix,
            node_map,
            scope,
            depth,
        } = scope;
        ensure_parse_not_aborted(abort)?;
        ResourceLimitError::ensure(ResourceKind::HierarchyDepth, depth, self.config.max_depth)
            .map_err(ParseError::from)?;

        match &element.kind {
            ElementKind::RfPortDeferred {
                source,
                line,
                multiplicity,
            } => {
                let generated =
                    super::parser::lower_deferred_rf_port(element, source, *line, scope)?;
                for mut generated in generated {
                    ensure_parse_not_aborted(abort)?;
                    apply_element_multiplicity(&mut generated, *multiplicity);
                    let flattened = self.remap_element(&generated, prefix, node_map);
                    self.push_flattened_element(output, flattened)?;
                }
            }
            ElementKind::PspiceChebyshev {
                source_line,
                input_expression,
                filter_kind,
                frequencies_hz,
                ripple_db,
                stop_db,
                voltage_output,
                multiplicity,
            } => {
                let [node_pos, node_neg] = element.nodes.as_slice() else {
                    return Err(ParseError::Syntax {
                        line: *source_line,
                        message: format!(
                            "CHEBYSHEV source '{}' requires exactly two output nodes",
                            element.name
                        ),
                    }
                    .into());
                };
                let resolve =
                    |value: &ParametricValue| resolve_parametric_value(value, scope, &self.random);
                let spec = super::parser::ChebyshevSpec {
                    kind: *filter_kind,
                    frequencies_hz: frequencies_hz
                        .iter()
                        .map(resolve)
                        .collect::<Result<Vec<_>, _>>()?,
                    ripple_db: resolve(ripple_db)?,
                    stop_db: resolve(stop_db)?,
                };
                let scoped_input = self.prepare_scoped_behavioral_expression(
                    input_expression,
                    scope,
                    &self.qualify_hierarchy_name(prefix, &element.name),
                )?;
                let multiplicity = self.resolve_source_multiplicity(
                    multiplicity,
                    scope,
                    &self.qualify_hierarchy_name(prefix, &element.name),
                    None,
                )?;
                let mut synthesized = super::parser::synthesize_chebyshev(
                    &element.name,
                    node_pos,
                    node_neg,
                    &scoped_input,
                    &spec,
                    *voltage_output,
                    *source_line,
                )?;
                for generated in &mut synthesized {
                    if generated.name.eq_ignore_ascii_case(&element.name) {
                        match &mut generated.kind {
                            ElementKind::BehavioralVoltage {
                                multiplicity: target,
                                ..
                            }
                            | ElementKind::BehavioralCurrent {
                                multiplicity: target,
                                ..
                            } => *target = multiplicity.clone(),
                            _ => {}
                        }
                    }
                }
                for generated in synthesized {
                    let flattened = self.remap_element(&generated, prefix, node_map);
                    self.push_flattened_element(output, flattened)?;
                }
            }
            ElementKind::Subcircuit {
                subckt_name,
                params,
            } => {
                if self.find_subcircuit(subckt_name).is_some() {
                    self.expand_subcircuit(
                        element,
                        subckt_name,
                        params,
                        FlattenScope {
                            prefix,
                            node_map,
                            scope,
                            depth,
                        },
                        output,
                        abort,
                    )?;
                } else if self.is_external_subckt(subckt_name) || self.defer_external_module_binding
                {
                    // External modules are linked after hierarchy expansion.
                    // An unselected import may declare a name absent from the
                    // deck; the builder must resolve every preserved leaf.
                    let new_element = self.resolve_external_subcircuit_params(
                        self.remap_element(element, prefix, node_map),
                        scope,
                    )?;
                    self.push_flattened_element(output, new_element)?;
                } else {
                    return Err(ParseError::UndefinedSubcircuit(Box::new(
                        UndefinedSubcircuitError {
                            subcircuit_name: subckt_name.clone(),
                            canonical_subcircuit_name: subckt_name.to_ascii_uppercase(),
                            instance_name: element.name.clone(),
                            canonical_instance_name: element.name.to_ascii_uppercase(),
                            qualified_instance_name: self
                                .qualify_hierarchy_name(prefix, &element.name),
                        },
                    ))
                    .into());
                }
            }
            _ => {
                // Regular element - remap nodes and add to output
                let new_element = self.remap_element(element, prefix, node_map);
                self.record_xspice_auto_bridge_node_hints(&new_element, scope, depth);
                self.push_flattened_element(output, new_element)?;
            }
        }

        Ok(())
    }

    fn push_flattened_element(
        &mut self,
        output: &mut Vec<Element>,
        mut element: Element,
    ) -> Result<(), ParseWithAbortError> {
        let requested = output.len().saturating_add(1);
        ResourceLimitError::ensure(
            ResourceKind::FlattenedElements,
            requested,
            self.config.max_elements,
        )
        .map_err(ParseError::from)?;
        materialize_passive_initial_condition(&mut element)?;
        let canonical_name = element.name.to_ascii_uppercase();
        if !self.flattened_element_names.insert(canonical_name.clone()) {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "flattened element name '{}' collides with an existing element in the same hierarchical expansion",
                    element.name
                ),
            }
            .into());
        }
        output.push(element);
        Ok(())
    }

    /// Validate the private node namespace created by dynamic-source
    /// lowering after every hierarchy has been expanded. Generated state
    /// storage and derivative elements may legitimately share their owner's
    /// private nodes; no authored element (or different generated owner) may
    /// use the same case-insensitive flattened identity.
    fn validate_generated_internal_node_collisions(
        &self,
        elements: &[Element],
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        #[derive(Debug)]
        struct Reservation {
            spelling: String,
            owner: String,
        }

        let mut reservations = HashMap::<String, Reservation>::new();
        let mut reserve = |node: &str, owner: &str| -> Result<(), ParseWithAbortError> {
            if self.ground_policy.is_ground(node) {
                return Err(ParseError::Syntax {
                    line: 0,
                    message: format!(
                        "generated dynamic source '{}' attempted to reserve ground as an internal node",
                        owner
                    ),
                }
                .into());
            }
            let canonical = node.to_ascii_uppercase();
            if let Some(existing) = reservations.get(&canonical) {
                if !existing.owner.eq_ignore_ascii_case(owner) {
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!(
                            "generated internal node '{}' owned by '{}' collides with node '{}' owned by '{}'",
                            node, owner, existing.spelling, existing.owner
                        ),
                    }
                    .into());
                }
            } else {
                reservations.insert(
                    canonical,
                    Reservation {
                        spelling: node.to_string(),
                        owner: owner.to_string(),
                    },
                );
            }
            Ok(())
        };

        for (index, element) in elements.iter().enumerate() {
            poll_parse_abort(abort, index)?;
            match &element.provenance {
                super::ElementProvenance::GeneratedDynamicInternalNode { owner, node, .. } => {
                    if !element
                        .nodes
                        .iter()
                        .any(|terminal| terminal.eq_ignore_ascii_case(node))
                    {
                        return Err(ParseError::Syntax {
                            line: 0,
                            message: format!(
                                "generated internal-node metadata for element '{}' does not identify one of its terminals",
                                element.name
                            ),
                        }
                        .into());
                    }
                    reserve(node, owner)?;
                }
                super::ElementProvenance::GeneratedDynamicStateDerivative { owner, .. } => {
                    for node in &element.nodes {
                        if !self.ground_policy.is_ground(node) {
                            reserve(node, owner)?;
                        }
                    }
                }
                _ => {}
            }
        }

        for (element_index, element) in elements.iter().enumerate() {
            poll_parse_abort(abort, elements.len().saturating_add(element_index))?;
            let generated_owner = match &element.provenance {
                super::ElementProvenance::GeneratedDynamicInternalNode { owner, .. }
                | super::ElementProvenance::GeneratedDynamicStateDerivative { owner, .. } => {
                    Some(owner.as_str())
                }
                _ => None,
            };
            Self::visit_element_electrical_nodes(element, &mut |node| -> Result<
                (),
                ParseWithAbortError,
            > {
                let Some(reservation) = reservations.get(&node.to_ascii_uppercase()) else {
                    return Ok(());
                };
                if generated_owner
                    .is_some_and(|owner| owner.eq_ignore_ascii_case(&reservation.owner))
                {
                    return Ok(());
                }
                Err(ParseError::Syntax {
                    line: 0,
                    message: format!(
                        "flattened node '{}' on element '{}' collides with generated internal node '{}' owned by '{}'",
                        node, element.name, reservation.spelling, reservation.owner
                    ),
                }
                .into())
            })?;
        }
        ensure_parse_not_aborted(abort)?;
        Ok(())
    }

    /// Visit the same electrical node-bearing fields used by topology
    /// construction. XSPICE ports and explicit controlled-source terminals
    /// are not stored in `Element::nodes`, but can still drive or constrain a
    /// generated private node and therefore participate in collision checks.
    fn visit_element_electrical_nodes<E>(
        element: &Element,
        visit: &mut impl FnMut(&str) -> Result<(), E>,
    ) -> Result<(), E> {
        for node in &element.nodes {
            visit(node)?;
        }
        match &element.kind {
            ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. } => {
                visit(&control_nodes.0)?;
                visit(&control_nodes.1)?;
            }
            ElementKind::VSwitch {
                control_pos,
                control_neg,
                ..
            } => {
                visit(control_pos)?;
                visit(control_neg)?;
            }
            ElementKind::Xspice { ports, .. } => {
                for port in ports {
                    for node in port.node_names() {
                        visit(node)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn record_xspice_auto_bridge_node_hints(
        &mut self,
        element: &Element,
        scope: &ParamContext,
        depth: usize,
    ) {
        let ElementKind::Xspice {
            ports,
            string_params,
            ..
        } = &element.kind
        else {
            return;
        };
        let vcc = scope
            .get(&self.xspice_auto_bridge_digital_param_name)
            .filter(|value| value.is_finite());
        let family = xspice_auto_bridge_family(string_params, scope);
        if vcc.is_none() && family.is_none() {
            return;
        }

        let mut push_node = |node: &str| {
            if !self.ground_policy.is_ground(node) {
                self.xspice_auto_bridge_node_hints
                    .push(XspiceAutoBridgeNodeHint {
                        node: node.to_string(),
                        depth,
                        vcc,
                        family: family.clone(),
                    });
            }
        };

        for port in ports {
            match port {
                super::XspicePort::Digital(node)
                | super::XspicePort::ExplicitDigital(node)
                | super::XspicePort::DigitalInverted(node) => push_node(node),
                super::XspicePort::DigitalVector(nodes) => {
                    for node in nodes {
                        push_node(node);
                    }
                }
                super::XspicePort::DigitalVectorMixed(nodes) => {
                    for node in nodes {
                        push_node(&node.name);
                    }
                }
                _ => {}
            }
        }
    }

    pub(super) fn collect_external_subckts(netlist: &Netlist) -> HashSet<String> {
        let mut names = HashSet::new();
        for include in &netlist.veriloga_includes {
            names.extend(include.declared_model_names().map(str::to_ascii_uppercase));
        }
        #[cfg(feature = "veriloga-builtins-base")]
        {
            // The generated registry is an engine-neutral catalog. Reading
            // its names directly keeps netlist flattening below the device
            // adapter layer instead of reaching upward through it.
            for name in rspice_veriloga_models::registry::builtin_names() {
                names.insert(name.to_ascii_uppercase());
            }
        }
        names
    }

    fn is_external_subckt(&self, name: &str) -> bool {
        self.external_subckts.contains(&name.to_ascii_uppercase())
    }

    fn find_subcircuit(&self, name: &str) -> Option<&'a SubcircuitDef> {
        self.subcircuits.get(&name.to_ascii_uppercase()).copied()
    }

    /// Expand a subcircuit instance into its constituent elements
    fn expand_subcircuit(
        &mut self,
        instance: &Element,
        subckt_name: &str,
        instance_params: &[(String, ParametricValue)],
        scope: FlattenScope<'_>,
        output: &mut Vec<Element>,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let FlattenScope {
            prefix,
            node_map: parent_node_map,
            scope: caller_scope,
            depth,
        } = scope;
        ensure_parse_not_aborted(abort)?;
        // Look up subcircuit definition
        let subckt = self.find_subcircuit(subckt_name).ok_or_else(|| {
            ParseError::UndefinedSubcircuit(Box::new(UndefinedSubcircuitError {
                subcircuit_name: subckt_name.to_string(),
                canonical_subcircuit_name: subckt_name.to_ascii_uppercase(),
                instance_name: instance.name.clone(),
                canonical_instance_name: instance.name.to_ascii_uppercase(),
                qualified_instance_name: self.qualify_hierarchy_name(prefix, &instance.name),
            }))
        })?;

        // Build new prefix for this instance
        let new_prefix = self.qualify_hierarchy_name(prefix, &instance.name);

        // Resolve every actual through the parent context before validating
        // repeated formal ports. Duplicate formals are legal only when every
        // occurrence maps to the same effective node for this invocation.
        let mapped_ports = subckt
            .ports
            .iter()
            .zip(&instance.nodes)
            .map(|(port, actual)| (port, self.remap_node(actual, prefix, parent_node_map)))
            .collect::<Vec<_>>();
        validate_subcircuit_port_bindings(
            subckt,
            SubcircuitInstanceNames {
                invoked_subcircuit_name: subckt_name,
                instance_name: &instance.name,
                qualified_instance_name: &new_prefix,
            },
            instance.nodes.len(),
            &mapped_ports,
            &self.global_nodes,
            abort,
        )?;

        if self
            .expansion_stack
            .iter()
            .any(|name| name.eq_ignore_ascii_case(subckt_name))
        {
            let start = self
                .expansion_stack
                .iter()
                .position(|name| name.eq_ignore_ascii_case(subckt_name))
                .unwrap_or(0);
            let mut chain: Vec<&str> = self.expansion_stack[start..]
                .iter()
                .map(String::as_str)
                .collect();
            chain.push(subckt_name);
            return Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "Recursive subcircuit instantiation at '{}': {}",
                    new_prefix,
                    chain.join(" -> ")
                ),
            }
            .into());
        }

        // Preserve the first mapping, matching Xyce. Repeated identical
        // bindings are compatibility aliases and never replace it.
        let mut node_map = HashMap::new();
        let mut mapped_formals = HashSet::new();
        for (formal, actual) in mapped_ports {
            if mapped_formals.insert(formal.to_ascii_uppercase()) {
                node_map.insert(formal.clone(), actual);
            }
        }

        // X-line multiplicity composes through nested hierarchy. Xyce treats
        // `M=` as a reserved physical multiplier even when the subcircuit has
        // an ordinary formal parameter with the same name.
        // Other dialects keep the existing policy in which a declared formal
        // `M` owns the name and remains an ordinary parameter.
        let formal_declares_m = subckt
            .params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("M"));
        let xyce_implicit_multiplier = caller_scope.expression_dialect() == ExpressionDialect::Xyce;
        let applies_as_physical_multiplier = xyce_implicit_multiplier || !formal_declares_m;
        let mut multiplicity = 1.0;
        if applies_as_physical_multiplier {
            for (param_index, (name, value)) in instance_params.iter().enumerate() {
                poll_parse_abort(abort, param_index)?;
                if name.eq_ignore_ascii_case("M") {
                    if let Some(capture) = &mut self.parameter_direction {
                        capture.has_uncaptured_dependencies = true;
                    }
                    let resolved = resolve_parametric_value(value, caller_scope, &self.random)?;
                    if !resolved.is_finite() || resolved <= 0.0 {
                        return Err(ParseError::Syntax {
                            line: 0,
                            message: format!(
                                "Subcircuit instance '{}' has invalid multiplicity M={}",
                                instance.name, resolved
                            ),
                        }
                        .into());
                    }
                    multiplicity = resolved;
                }
            }
        }

        let scoped_instance_params = if xyce_implicit_multiplier {
            instance_params
                .iter()
                .filter(|(name, _)| !name.eq_ignore_ascii_case("M"))
                .cloned()
                .collect::<Vec<_>>()
        } else {
            instance_params.to_vec()
        };
        let param_scope = build_subcircuit_param_scope(
            subckt,
            &instance.name,
            &new_prefix,
            caller_scope,
            &scoped_instance_params,
            &self.random,
            abort,
        )?;

        // Expand each element in the subcircuit
        self.expansion_stack.push(subckt_name.to_owned());
        for (element_index, sub_element) in subckt.elements.iter().enumerate() {
            poll_parse_abort(abort, element_index)?;
            let stack_len = self.expansion_stack.len();
            let branch_result = (|| -> Result<(), ParseWithAbortError> {
                // Apply parameter substitution to element values.
                let element_path = self.qualify_hierarchy_name(&new_prefix, &sub_element.name);
                let mut substituted =
                    self.substitute_params(sub_element, &param_scope, &element_path, &new_prefix)?;
                if multiplicity != 1.0 {
                    apply_element_multiplicity(&mut substituted, multiplicity);
                }
                self.flatten_element(
                    &substituted,
                    FlattenScope {
                        prefix: &new_prefix,
                        node_map: &node_map,
                        scope: &param_scope,
                        depth: depth + 1,
                    },
                    output,
                    abort,
                )
            })();
            if let Err(error) = branch_result {
                self.expansion_stack.truncate(stack_len);
                self.handle_branch_error(
                    error,
                    &format!(
                        "element {element_path}",
                        element_path = self.qualify_hierarchy_name(&new_prefix, &sub_element.name)
                    ),
                )?;
            }
        }
        self.expansion_stack.pop();
        self.collect_scoped_startup_directives(
            subckt,
            &new_prefix,
            &node_map,
            &param_scope,
            abort,
        )?;

        Ok(())
    }

    fn collect_scoped_startup_directives(
        &mut self,
        subckt: &SubcircuitDef,
        prefix: &str,
        node_map: &HashMap<String, String>,
        scope: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        for (index, ic) in subckt.initial_conditions.iter().enumerate() {
            poll_parse_abort(abort, index)?;
            self.scoped_initial_conditions.push(InitialCondition {
                node: self.remap_node(&ic.node, prefix, node_map),
                reference: ic
                    .reference
                    .as_ref()
                    .map(|reference| self.remap_node(reference, prefix, node_map)),
                voltage: self.resolve_startup_voltage(
                    ic.voltage,
                    ic.voltage_expr.as_deref(),
                    scope,
                    ".IC",
                    &ic.node,
                )?,
                voltage_expr: None,
            });
        }

        for (index, nodeset) in subckt.node_sets.iter().enumerate() {
            poll_parse_abort(abort, index)?;
            self.scoped_node_sets.push(NodeSet {
                node: self.remap_node(&nodeset.node, prefix, node_map),
                reference: nodeset
                    .reference
                    .as_ref()
                    .map(|reference| self.remap_node(reference, prefix, node_map)),
                voltage: self.resolve_startup_voltage(
                    nodeset.voltage,
                    nodeset.voltage_expr.as_deref(),
                    scope,
                    ".NODESET",
                    &nodeset.node,
                )?,
                voltage_expr: None,
            });
        }

        for (index, record) in self
            .startup_directives
            .iter()
            .filter(|record| {
                matches!(
                    &record.scope,
                    StartupDirectiveScope::Subcircuit {
                        qualified_definition,
                        ..
                    } if qualified_definition.eq_ignore_ascii_case(&subckt.name)
                ) && !matches!(record.disposition, StartupDirectiveDisposition::Ignored(_))
            })
            .enumerate()
        {
            poll_parse_abort(abort, index)?;
            let mut elaborated = record.clone();
            elaborated.scope = StartupDirectiveScope::Subcircuit {
                qualified_definition: subckt.name.clone(),
                qualified_instances: vec![prefix.replace(':', ".")],
            };
            for entry in &mut elaborated.entries {
                entry.voltage = self.resolve_startup_voltage(
                    entry.voltage,
                    entry.voltage_expr.as_deref(),
                    scope,
                    record.kind.as_spice_directive(),
                    &entry.execution_node,
                )?;
                entry.voltage_expr = None;
                entry.qualified_nodes = vec![
                    self.remap_node(&entry.execution_node, prefix, node_map)
                        .replace(':', "."),
                ];
                entry.qualified_references =
                    vec![entry.execution_reference.as_ref().map(|reference| {
                        self.remap_node(reference, prefix, node_map)
                            .replace(':', ".")
                    })];
            }
            self.scoped_startup_directives.push(elaborated);
        }

        Ok(())
    }

    fn resolve_startup_voltage(
        &self,
        voltage: Value,
        voltage_expr: Option<&str>,
        scope: &ParamContext,
        directive: &str,
        node: &str,
    ) -> Result<Value, ParseError> {
        let value = match voltage_expr {
            Some(expr) => resolve_parametric_value(
                &ParametricValue::Expression(expr.to_string()),
                scope,
                &self.random,
            )
            .map_err(|err| {
                ParseError::InvalidValue(format!(
                    "{directive} for node '{node}' could not resolve expression '{expr}': {err}"
                ))
            })?,
            None => voltage,
        };
        if value.is_finite() {
            Ok(value)
        } else {
            Err(ParseError::InvalidValue(format!(
                "{directive} for node '{node}' resolved to non-finite voltage {value}"
            )))
        }
    }

    /// Remap an element's nodes using the current prefix and node map
    /// Also remaps CCCS/CCVS control element names
    fn remap_element(
        &self,
        element: &Element,
        prefix: &str,
        node_map: &HashMap<String, String>,
    ) -> Element {
        let new_name = self.qualify_hierarchy_name(prefix, &element.name);

        let new_nodes: Vec<String> = element
            .nodes
            .iter()
            .map(|n| self.remap_node(n, prefix, node_map))
            .collect();

        // Remap the element kind, handling CCCS/CCVS control element names
        let new_kind = match &element.kind {
            ElementKind::VoltageSource(spec) => {
                let mut spec = spec.clone();
                if let Some(port) = spec.rf_port_mut()
                    && let Some(plane) = &mut port.reference_plane
                {
                    *plane = self.remap_node(plane, prefix, node_map);
                }
                ElementKind::VoltageSource(spec)
            }
            ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } => ElementKind::Resistor {
                value: *value,
                value_expr: value_expr
                    .as_ref()
                    .map(|expr| self.remap_behavioral_expression(expr, prefix, node_map)),
                model: model.clone(),
                instance_params: instance_params.clone(),
                deferred_params: deferred_params.clone(),
            },
            ElementKind::Capacitor {
                value,
                value_expr,
                initial_voltage,
                model,
                instance_params,
                deferred_params,
            } => ElementKind::Capacitor {
                value: *value,
                value_expr: value_expr
                    .as_ref()
                    .map(|expr| self.remap_behavioral_expression(expr, prefix, node_map)),
                initial_voltage: *initial_voltage,
                model: model.clone(),
                instance_params: instance_params.clone(),
                deferred_params: deferred_params.clone(),
            },
            ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
                multiplicity,
            } => ElementKind::BehavioralVoltage {
                expression: self.remap_behavioral_expression(expression, prefix, node_map),
                tc1: *tc1,
                tc2: *tc2,
                multiplicity: multiplicity.clone(),
            },
            ElementKind::BehavioralCurrent {
                expression,
                tc1,
                tc2,
                multiplicity,
            } => ElementKind::BehavioralCurrent {
                expression: self.remap_behavioral_expression(expression, prefix, node_map),
                tc1: *tc1,
                tc2: *tc2,
                multiplicity: multiplicity.clone(),
            },
            ElementKind::Vcvs {
                gain,
                gain_expr,
                control_nodes,
            } => ElementKind::Vcvs {
                gain: *gain,
                gain_expr: gain_expr.clone(),
                control_nodes: (
                    self.remap_node(&control_nodes.0, prefix, node_map),
                    self.remap_node(&control_nodes.1, prefix, node_map),
                ),
            },
            ElementKind::Vccs {
                transconductance,
                transconductance_expr,
                multiplicity,
                control_nodes,
            } => ElementKind::Vccs {
                transconductance: *transconductance,
                transconductance_expr: transconductance_expr.clone(),
                multiplicity: multiplicity.clone(),
                control_nodes: (
                    self.remap_node(&control_nodes.0, prefix, node_map),
                    self.remap_node(&control_nodes.1, prefix, node_map),
                ),
            },
            ElementKind::Cccs {
                gain,
                gain_expr,
                control_element,
            } => {
                // Remap control element name with prefix (like element names)
                let new_ctrl = self.remap_local_element_reference(control_element, prefix);
                ElementKind::Cccs {
                    gain: *gain,
                    gain_expr: gain_expr.clone(),
                    control_element: new_ctrl,
                }
            }
            ElementKind::Ccvs {
                transresistance,
                transresistance_expr,
                control_element,
            } => {
                let new_ctrl = self.remap_local_element_reference(control_element, prefix);
                ElementKind::Ccvs {
                    transresistance: *transresistance,
                    transresistance_expr: transresistance_expr.clone(),
                    control_element: new_ctrl,
                }
            }
            ElementKind::VSwitch {
                control_pos,
                control_neg,
                model,
                initial_state,
            } => ElementKind::VSwitch {
                control_pos: self.remap_node(control_pos, prefix, node_map),
                control_neg: self.remap_node(control_neg, prefix, node_map),
                model: model.clone(),
                initial_state: *initial_state,
            },
            ElementKind::ISwitch {
                control_element,
                model,
                initial_state,
            } => ElementKind::ISwitch {
                control_element: self.remap_local_element_reference(control_element, prefix),
                model: model.clone(),
                initial_state: *initial_state,
            },
            ElementKind::GenericSwitch {
                model,
                control_expression,
                initial_state,
            } => ElementKind::GenericSwitch {
                model: model.clone(),
                control_expression: self.remap_behavioral_expression(
                    control_expression,
                    prefix,
                    node_map,
                ),
                initial_state: *initial_state,
            },
            ElementKind::Coupling {
                inductors,
                coefficient,
                model,
            } => ElementKind::Coupling {
                inductors: inductors
                    .iter()
                    .map(|name| self.remap_local_element_reference(name, prefix))
                    .collect(),
                coefficient: *coefficient,
                model: model.clone(),
            },
            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ports,
                params,
                expr_params,
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                real_vector_params,
                real_vector_expr_params,
            } => {
                let pspice_u_timing = pspice_u_timing.as_ref().map(|timing| {
                    let mut timing = timing.clone();
                    if let Some((dpwr, dgnd)) = timing.power_pins.as_mut() {
                        *dpwr = self.remap_node(dpwr, prefix, node_map);
                        *dgnd = self.remap_node(dgnd, prefix, node_map);
                    }
                    timing
                });
                ElementKind::Xspice {
                    model: model.clone(),
                    pspice_u_timing,
                    ports: ports
                        .iter()
                        .map(|port| self.remap_xspice_port(port, prefix, node_map))
                        .collect(),
                    params: params.clone(),
                    expr_params: expr_params.clone(),
                    string_params: string_params
                        .iter()
                        .map(|(name, value)| {
                            (
                                name.clone(),
                                super::normalize_model_string_path_value(
                                    name,
                                    value,
                                    self.source_base_dir.as_deref(),
                                ),
                            )
                        })
                        .collect(),
                    string_expr_params: string_expr_params.clone(),
                    string_vector_params: string_vector_params.clone(),
                    string_vector_expr_params: string_vector_expr_params.clone(),
                    real_vector_params: real_vector_params.clone(),
                    real_vector_expr_params: real_vector_expr_params.clone(),
                }
            }
            // All other kinds - clone as-is
            other => other.clone(),
        };

        Element {
            name: new_name,
            kind: new_kind,
            nodes: new_nodes,
            provenance: match &element.provenance {
                super::ElementProvenance::Authored => super::ElementProvenance::Authored,
                super::ElementProvenance::ImportedSpef {
                    net,
                    record_id,
                    line,
                } => super::ElementProvenance::ImportedSpef {
                    net: net.clone(),
                    record_id: *record_id,
                    line: *line,
                },
                super::ElementProvenance::GeneratedPassiveHelper { owner, role } => {
                    super::ElementProvenance::GeneratedPassiveHelper {
                        owner: self.remap_local_element_reference(owner, prefix),
                        role: *role,
                    }
                }
                super::ElementProvenance::GeneratedDynamicStateDerivative {
                    owner,
                    form,
                    dc_determined,
                } => super::ElementProvenance::GeneratedDynamicStateDerivative {
                    owner: self.remap_local_element_reference(owner, prefix),
                    form: *form,
                    dc_determined: *dc_determined,
                },
                super::ElementProvenance::GeneratedDynamicInternalNode { owner, form, node } => {
                    super::ElementProvenance::GeneratedDynamicInternalNode {
                        owner: self.remap_local_element_reference(owner, prefix),
                        form: *form,
                        // This is the private identity promised by lowering,
                        // not an ordinary authored node reference. Keeping it
                        // independent of the port/global map lets final
                        // validation detect a formal or `.GLOBAL` name that
                        // attempts to capture the generated node.
                        node: self.qualify_hierarchy_name(prefix, node),
                    }
                }
                super::ElementProvenance::GeneratedXyceAddResistor { mode } => {
                    super::ElementProvenance::GeneratedXyceAddResistor { mode: *mode }
                }
                super::ElementProvenance::SynthesizedTransferState { owner, form } => {
                    super::ElementProvenance::SynthesizedTransferState {
                        owner: self.remap_local_element_reference(owner, prefix),
                        form: *form,
                    }
                }
            },
        }
    }

    fn remap_local_element_reference(&self, name: &str, prefix: &str) -> String {
        self.qualify_hierarchy_name(prefix, name)
    }

    fn canonicalize_hierarchy_name(&self, name: &str) -> String {
        if self.config.hierarchy_separator == ':' || !name.contains(':') {
            name.to_string()
        } else {
            name.replace(':', &self.config.hierarchy_separator.to_string())
        }
    }

    fn qualify_hierarchy_name(&self, prefix: &str, local_name: &str) -> String {
        let local_name = self.canonicalize_hierarchy_name(local_name);
        if prefix.is_empty() {
            local_name
        } else {
            format!("{prefix}{}{local_name}", self.config.hierarchy_separator)
        }
    }

    /// Remap a single node name
    fn remap_node(&self, node: &str, prefix: &str, node_map: &HashMap<String, String>) -> String {
        // Ground is never renamed
        let canonical = self.ground_policy.canonical_node(node);
        if canonical == "0" {
            return canonical.to_string();
        }

        // Explicit .GLOBAL nodes and Xyce's implicit $G* global-node names
        // retain their original names across hierarchy levels. Xyce applies
        // the $G prefix rule during subcircuit expansion, so recognizing it
        // here is essential before connectivity and device construction.
        if self.global_nodes.contains(&node.to_ascii_uppercase())
            || node
                .get(..2)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("$G"))
        {
            return node.to_string();
        }

        // Check if this is a port that maps to an external node
        if let Some(mapped) = node_map.get(node) {
            return mapped.clone();
        }
        if let Some((_, mapped)) = node_map
            .iter()
            .find(|(port, _)| port.eq_ignore_ascii_case(node))
        {
            return mapped.clone();
        }

        // Internal node - prefix with instance path
        if prefix.is_empty() {
            self.canonicalize_hierarchy_name(node)
        } else {
            self.qualify_hierarchy_name(prefix, node)
        }
    }

    fn remap_xspice_port(
        &self,
        port: &super::XspicePort,
        prefix: &str,
        node_map: &HashMap<String, String>,
    ) -> super::XspicePort {
        use super::XspicePort;

        let remap = |name: &str| self.remap_node(name, prefix, node_map);
        match port {
            XspicePort::Analog(name) => XspicePort::Analog(remap(name)),
            XspicePort::Digital(name) => XspicePort::Digital(remap(name)),
            XspicePort::ExplicitDigital(name) => XspicePort::ExplicitDigital(remap(name)),
            XspicePort::DigitalInverted(name) => XspicePort::DigitalInverted(remap(name)),
            XspicePort::AnalogVector(names) => {
                XspicePort::AnalogVector(names.iter().map(|name| remap(name)).collect())
            }
            XspicePort::DigitalVector(names) => {
                XspicePort::DigitalVector(names.iter().map(|name| remap(name)).collect())
            }
            XspicePort::DigitalVectorMixed(nodes) => XspicePort::DigitalVectorMixed(
                nodes
                    .iter()
                    .map(|node| super::XspiceDigitalNode::new(remap(&node.name), node.inverted))
                    .collect(),
            ),
            XspicePort::Conductance(name) => XspicePort::Conductance(remap(name)),
            XspicePort::Current(name) => XspicePort::Current(remap(name)),
            XspicePort::VoltageName(name) => XspicePort::VoltageName(remap(name)),
            XspicePort::DifferentialVoltage { pos, neg } => XspicePort::DifferentialVoltage {
                pos: remap(pos),
                neg: remap(neg),
            },
            XspicePort::DifferentialCurrent { pos, neg } => XspicePort::DifferentialCurrent {
                pos: remap(pos),
                neg: remap(neg),
            },
            XspicePort::DifferentialConductance { pos, neg } => {
                XspicePort::DifferentialConductance {
                    pos: remap(pos),
                    neg: remap(neg),
                }
            }
            XspicePort::Hybrid(name) => XspicePort::Hybrid(remap(name)),
            XspicePort::DifferentialHybrid { pos, neg } => XspicePort::DifferentialHybrid {
                pos: remap(pos),
                neg: remap(neg),
            },
            XspicePort::Null => XspicePort::Null,
        }
    }

    /// Remap V(...) and I(...) probe references inside behavioral expressions.
    ///
    /// This keeps behavioral source references consistent with flattened names:
    /// - `V(internal)` -> `V(X1.internal)`
    /// - `V(port)` -> `V(parent_mapped_node)`
    /// - `I(vsrc)` -> `I(X1.vsrc)` for local branch probes
    fn remap_behavioral_expression(
        &self,
        expression: &str,
        prefix: &str,
        node_map: &HashMap<String, String>,
    ) -> String {
        let chars: Vec<char> = expression.chars().collect();
        let mut out = String::with_capacity(expression.len() + prefix.len());
        let mut i = 0usize;

        while i < chars.len() {
            let c = chars[i];
            if is_ident_start(c) {
                let ident_start = i;
                i += 1;
                while i < chars.len() && is_ident_continue(chars[i]) {
                    i += 1;
                }
                let ident: String = chars[ident_start..i].iter().collect();

                let mut ws_idx = i;
                while ws_idx < chars.len() && chars[ws_idx].is_whitespace() {
                    ws_idx += 1;
                }

                let is_probe = ident.eq_ignore_ascii_case("V") || ident.eq_ignore_ascii_case("I");
                if is_probe
                    && ws_idx < chars.len()
                    && chars[ws_idx] == '('
                    && let Some((inner, end_idx)) = extract_parenthesized(&chars, ws_idx)
                {
                    let remapped = if ident.eq_ignore_ascii_case("V") {
                        remap_voltage_probe_args(self, &inner, prefix, node_map)
                    } else {
                        remap_current_probe_arg(self, prefix, &inner)
                    };
                    out.push_str(&ident);
                    out.push('(');
                    out.push_str(&remapped);
                    out.push(')');
                    i = end_idx + 1;
                    continue;
                }

                out.push_str(&ident);
                continue;
            }

            out.push(c);
            i += 1;
        }

        out
    }

    /// Resolve fields once in their instance scope, retaining selected numeric
    /// directions before substitution discards the authored parameter leaves.
    fn substitute_params(
        &mut self,
        element: &Element,
        scope: &ParamContext,
        element_path: &str,
        model_scope_path: &str,
    ) -> Result<Element, ParseError> {
        let capture_fields =
            self.parameter_direction.is_some() && !scope.has_retained_parameter_expressions();
        let mut scalar_direction = capture_fields.then(|| Derivative::from(0.0));
        let mut multiplicity_direction = scalar_direction;
        let source_directions =
            capture_fields.then(|| std::cell::RefCell::new([Derivative::from(0.0); 3]));
        let mut new_kind = match &element.kind {
            // Passive components
            ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } => {
                let (value, value_expr) = self.resolve_passive_value_expr(
                    *value,
                    value_expr,
                    scope,
                    element_path,
                    Some(&mut scalar_direction),
                )?;
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model: self.resolve_optional_scoped_model(
                        model,
                        scope,
                        element_path,
                        model_scope_path,
                    )?,
                    instance_params: self.merge_deferred_params(
                        instance_params,
                        deferred_params,
                        scope,
                    )?,
                    deferred_params: Vec::new(),
                }
            }
            ElementKind::Capacitor {
                value,
                value_expr,
                initial_voltage,
                model,
                instance_params,
                deferred_params,
            } => {
                let (value, value_expr) = self.resolve_passive_value_expr(
                    *value,
                    value_expr,
                    scope,
                    element_path,
                    Some(&mut scalar_direction),
                )?;
                ElementKind::Capacitor {
                    value,
                    value_expr,
                    initial_voltage: *initial_voltage,
                    model: self.resolve_optional_scoped_model(
                        model,
                        scope,
                        element_path,
                        model_scope_path,
                    )?,
                    instance_params: self.merge_deferred_params(
                        instance_params,
                        deferred_params,
                        scope,
                    )?,
                    deferred_params: Vec::new(),
                }
            }
            ElementKind::Inductor {
                value,
                value_expr,
                initial_current,
                model,
                instance_params,
                deferred_params,
            } => ElementKind::Inductor {
                value: self.resolve_optional_value_expr(
                    *value,
                    value_expr,
                    scope,
                    Some(&mut scalar_direction),
                )?,
                value_expr: None,
                initial_current: *initial_current,
                model: self.resolve_optional_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    scope,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::JilesAthertonInductor {
                value,
                model,
                initial_current,
            } => ElementKind::JilesAthertonInductor {
                value: *value,
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                initial_current: *initial_current,
            },

            // Semiconductor devices: instance parameters captured as
            // expressions inside the subcircuit body resolve against this
            // instance's parameter scope, so overrides like `x1 ... wn=4u`
            // reach device geometry instead of being shadowed by the
            // definition-time defaults.
            ElementKind::Diode {
                model,
                instance_params,
                deferred_params,
            } => ElementKind::Diode {
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    scope,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::Bjt {
                model,
                bjt_type,
                instance_params,
                deferred_params,
            } => ElementKind::Bjt {
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                bjt_type: *bjt_type,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    scope,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::Mosfet {
                model,
                mos_type,
                compact_syntax,
                instance_params,
                deferred_params,
            } => ElementKind::Mosfet {
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                mos_type: *mos_type,
                compact_syntax: *compact_syntax,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    scope,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::Jfet {
                model,
                jfet_type,
                instance_params,
                deferred_params,
            } => ElementKind::Jfet {
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                jfet_type: *jfet_type,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    scope,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::Mesfet {
                model,
                mesfet_type,
                instance_params,
                deferred_params,
            } => ElementKind::Mesfet {
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                mesfet_type: *mesfet_type,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    scope,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::XyceMemristor {
                model,
                instance_params,
                deferred_params,
            } => ElementKind::XyceMemristor {
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    scope,
                )?,
                deferred_params: Vec::new(),
            },

            // The child resolver binds arguments sequentially in this caller
            // scope. Eager projection here loses complex values and makes later
            // arguments miss earlier bindings on the same invocation.
            ElementKind::Subcircuit {
                subckt_name,
                params,
            } => ElementKind::Subcircuit {
                subckt_name: subckt_name.clone(),
                params: params.clone(),
            },

            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ports,
                params,
                expr_params,
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                real_vector_params,
                real_vector_expr_params,
            } => ElementKind::Xspice {
                model: self.resolve_xspice_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                pspice_u_timing: pspice_u_timing.clone(),
                ports: ports.clone(),
                params: self.merge_deferred_params(params, expr_params, scope)?,
                expr_params: Vec::new(),
                string_params: self.merge_deferred_string_params(
                    string_params,
                    string_expr_params,
                    scope,
                    element_path,
                )?,
                string_expr_params: Vec::new(),
                string_vector_params: self.merge_deferred_string_vector_params(
                    string_vector_params,
                    string_vector_expr_params,
                    scope,
                    element_path,
                )?,
                string_vector_expr_params: Vec::new(),
                real_vector_params: self.merge_deferred_real_vector_params(
                    real_vector_params,
                    real_vector_expr_params,
                    scope,
                )?,
                real_vector_expr_params: Vec::new(),
            },

            ElementKind::VoltageSourceDeferred(raw_spec) => self.resolve_deferred_source_kind(
                raw_spec,
                scope,
                element_path,
                true,
                source_directions.as_ref(),
            )?,
            ElementKind::CurrentSourceDeferred(raw_spec) => self.resolve_deferred_source_kind(
                raw_spec,
                scope,
                element_path,
                false,
                source_directions.as_ref(),
            )?,

            ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
                multiplicity,
            } => ElementKind::BehavioralVoltage {
                expression: self.prepare_scoped_behavioral_expression(
                    expression,
                    scope,
                    element_path,
                )?,
                tc1: *tc1,
                tc2: *tc2,
                multiplicity: self.resolve_source_multiplicity(
                    multiplicity,
                    scope,
                    element_path,
                    None,
                )?,
            },
            ElementKind::BehavioralCurrent {
                expression,
                tc1,
                tc2,
                multiplicity,
            } => ElementKind::BehavioralCurrent {
                expression: self.prepare_scoped_behavioral_expression(
                    expression,
                    scope,
                    element_path,
                )?,
                tc1: *tc1,
                tc2: *tc2,
                multiplicity: self.resolve_source_multiplicity(
                    multiplicity,
                    scope,
                    element_path,
                    None,
                )?,
            },

            // Controlled sources
            ElementKind::Vcvs {
                gain,
                gain_expr,
                control_nodes,
            } => ElementKind::Vcvs {
                gain: self.resolve_optional_value_expr(
                    *gain,
                    gain_expr,
                    scope,
                    Some(&mut scalar_direction),
                )?,
                gain_expr: None,
                control_nodes: control_nodes.clone(),
            },
            ElementKind::Vccs {
                transconductance,
                transconductance_expr,
                multiplicity,
                control_nodes,
            } => ElementKind::Vccs {
                transconductance: self.resolve_optional_value_expr(
                    *transconductance,
                    transconductance_expr,
                    scope,
                    Some(&mut scalar_direction),
                )?,
                transconductance_expr: None,
                multiplicity: self.resolve_source_multiplicity(
                    multiplicity,
                    scope,
                    element_path,
                    Some(&mut multiplicity_direction),
                )?,
                control_nodes: control_nodes.clone(),
            },
            ElementKind::Cccs {
                gain,
                gain_expr,
                control_element,
            } => ElementKind::Cccs {
                gain: self.resolve_optional_value_expr(
                    *gain,
                    gain_expr,
                    scope,
                    Some(&mut scalar_direction),
                )?,
                gain_expr: None,
                control_element: control_element.clone(),
            },
            ElementKind::Ccvs {
                transresistance,
                transresistance_expr,
                control_element,
            } => ElementKind::Ccvs {
                transresistance: self.resolve_optional_value_expr(
                    *transresistance,
                    transresistance_expr,
                    scope,
                    Some(&mut scalar_direction),
                )?,
                transresistance_expr: None,
                control_element: control_element.clone(),
            },
            ElementKind::VSwitch {
                control_pos,
                control_neg,
                model,
                initial_state,
            } => ElementKind::VSwitch {
                control_pos: control_pos.clone(),
                control_neg: control_neg.clone(),
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                initial_state: *initial_state,
            },
            ElementKind::ISwitch {
                control_element,
                model,
                initial_state,
            } => ElementKind::ISwitch {
                control_element: control_element.clone(),
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                initial_state: *initial_state,
            },
            ElementKind::GenericSwitch {
                model,
                control_expression,
                initial_state,
            } => ElementKind::GenericSwitch {
                model: self.resolve_native_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
                control_expression: prepare_behavioral_expression_preserving_spelling(
                    control_expression,
                    scope,
                )
                .map_err(|err| {
                    ParseError::InvalidValue(format!(
                        "behavioral expression for element '{}' could not be prepared: {}",
                        element_path, err
                    ))
                })?,
                initial_state: *initial_state,
            },
            ElementKind::Coupling {
                inductors,
                coefficient,
                model,
            } => ElementKind::Coupling {
                inductors: inductors.clone(),
                coefficient: *coefficient,
                model: self.resolve_optional_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
            },
            ElementKind::TransmissionLine {
                z0,
                td,
                freq,
                nl,
                model,
            } => ElementKind::TransmissionLine {
                z0: *z0,
                td: *td,
                freq: *freq,
                nl: *nl,
                model: self.resolve_optional_scoped_model(
                    model,
                    scope,
                    element_path,
                    model_scope_path,
                )?,
            },

            // All other element types - clone as-is
            other => other.clone(),
        };
        self.restore_spectre_statistical_expressions(
            &element.kind,
            &mut new_kind,
            scope,
            element_path,
        )?;

        if capture_fields && let Some(capture) = &mut self.parameter_direction {
            let direction = match &new_kind {
                ElementKind::Resistor { value, .. }
                | ElementKind::Capacitor { value, .. }
                | ElementKind::Inductor { value, .. } => {
                    scalar_direction.map(|direction| ElementParameterDirection::Passive {
                        value: *value,
                        direction,
                    })
                }
                ElementKind::Vcvs { .. } | ElementKind::Cccs { .. } | ElementKind::Ccvs { .. } => {
                    scalar_direction.map(ElementParameterDirection::Gain)
                }
                ElementKind::Vccs {
                    transconductance,
                    multiplicity,
                    ..
                } => scalar_direction
                    .zip(multiplicity_direction)
                    .map(|(gain, mult)| {
                        ElementParameterDirection::Gain(
                            gain * multiplicity.value + mult * *transconductance,
                        )
                    }),
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec)
                    if matches!(
                        spec,
                        SourceSpec::Dc(_) | SourceSpec::Ac { .. } | SourceSpec::DcAc { .. }
                    ) =>
                {
                    let [dc, magnitude, phase] = source_directions
                        .expect("selected field capture")
                        .into_inner();
                    Some(ElementParameterDirection::Source {
                        dc,
                        magnitude,
                        phase,
                    })
                }
                _ => None,
            };
            if let Some(direction) = direction {
                capture.elements.insert(element_path.to_string(), direction);
            }
        }
        Ok(Element {
            name: element.name.clone(),
            kind: new_kind,
            nodes: element.nodes.clone(),
            provenance: element.provenance.clone(),
        })
    }

    fn restore_spectre_statistical_expressions(
        &self,
        authored: &ElementKind,
        lowered: &mut ElementKind,
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<(), ParseError> {
        let prepare = |expression: &str| {
            self.prepare_spectre_statistical_expression(expression, scope, element_path)
        };
        match (authored, lowered) {
            (
                ElementKind::Resistor {
                    value_expr: authored_value,
                    deferred_params: authored_deferred,
                    ..
                },
                ElementKind::Resistor {
                    value_expr,
                    instance_params,
                    deferred_params,
                    ..
                },
            )
            | (
                ElementKind::Capacitor {
                    value_expr: authored_value,
                    deferred_params: authored_deferred,
                    ..
                },
                ElementKind::Capacitor {
                    value_expr,
                    instance_params,
                    deferred_params,
                    ..
                },
            )
            | (
                ElementKind::Inductor {
                    value_expr: authored_value,
                    deferred_params: authored_deferred,
                    ..
                },
                ElementKind::Inductor {
                    value_expr,
                    instance_params,
                    deferred_params,
                    ..
                },
            ) => {
                if let Some(expression) = authored_value
                    && scope.expression_references_spectre_statistics(expression)
                {
                    *value_expr = Some(prepare(expression)?);
                }
                restore_statistical_deferred_params(
                    authored_deferred,
                    instance_params,
                    deferred_params,
                    scope,
                    &prepare,
                )?;
            }
            (
                ElementKind::Diode {
                    deferred_params: authored_deferred,
                    ..
                }
                | ElementKind::Bjt {
                    deferred_params: authored_deferred,
                    ..
                }
                | ElementKind::Mosfet {
                    deferred_params: authored_deferred,
                    ..
                }
                | ElementKind::Jfet {
                    deferred_params: authored_deferred,
                    ..
                }
                | ElementKind::Mesfet {
                    deferred_params: authored_deferred,
                    ..
                }
                | ElementKind::XyceMemristor {
                    deferred_params: authored_deferred,
                    ..
                },
                ElementKind::Diode {
                    instance_params,
                    deferred_params,
                    ..
                }
                | ElementKind::Bjt {
                    instance_params,
                    deferred_params,
                    ..
                }
                | ElementKind::Mosfet {
                    instance_params,
                    deferred_params,
                    ..
                }
                | ElementKind::Jfet {
                    instance_params,
                    deferred_params,
                    ..
                }
                | ElementKind::Mesfet {
                    instance_params,
                    deferred_params,
                    ..
                }
                | ElementKind::XyceMemristor {
                    instance_params,
                    deferred_params,
                    ..
                },
            ) => restore_statistical_deferred_params(
                authored_deferred,
                instance_params,
                deferred_params,
                scope,
                &prepare,
            )?,
            (
                ElementKind::Vcvs {
                    gain_expr: Some(expression),
                    ..
                }
                | ElementKind::Cccs {
                    gain_expr: Some(expression),
                    ..
                },
                ElementKind::Vcvs { gain_expr, .. } | ElementKind::Cccs { gain_expr, .. },
            ) if scope.expression_references_spectre_statistics(expression) => {
                *gain_expr = Some(prepare(expression)?);
            }
            (
                ElementKind::Vccs {
                    transconductance_expr: Some(expression),
                    ..
                },
                ElementKind::Vccs {
                    transconductance_expr,
                    ..
                },
            ) if scope.expression_references_spectre_statistics(expression) => {
                *transconductance_expr = Some(prepare(expression)?);
            }
            (
                ElementKind::Ccvs {
                    transresistance_expr: Some(expression),
                    ..
                },
                ElementKind::Ccvs {
                    transresistance_expr,
                    ..
                },
            ) if scope.expression_references_spectre_statistics(expression) => {
                *transresistance_expr = Some(prepare(expression)?);
            }
            (
                ElementKind::BehavioralVoltage { expression, .. },
                ElementKind::BehavioralVoltage {
                    expression: lowered,
                    ..
                },
            )
            | (
                ElementKind::BehavioralCurrent { expression, .. },
                ElementKind::BehavioralCurrent {
                    expression: lowered,
                    ..
                },
            ) if scope.expression_references_spectre_statistics(expression) => {
                *lowered = prepare(expression)?;
            }
            (
                ElementKind::GenericSwitch {
                    control_expression, ..
                },
                ElementKind::GenericSwitch {
                    control_expression: lowered,
                    ..
                },
            ) if scope.expression_references_spectre_statistics(control_expression) => {
                *lowered = prepare(control_expression)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn prepare_spectre_statistical_expression(
        &self,
        expression: &str,
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<String, ParseError> {
        let preserved_parameters = scope
            .spectre_statistical_parameter_names()
            .into_iter()
            .filter(|name| scope.get_parameter_expression(name).is_none())
            .collect();
        prepare_behavioral_expression_preserving_parameters(
            expression,
            scope,
            &preserved_parameters,
        )
        .map_err(|error| {
            ParseError::InvalidValue(format!(
                "statistical expression for element '{element_path}' could not be prepared: {error}"
            ))
        })
    }

    /// Ready parameter leaves retain complex values and function bindings.
    /// Symbolic definitions and behavioral-only syntax still need expansion.
    fn resolve_prepared_scalar_value(
        &self,
        expression: &str,
        prepared: String,
        scope: &ParamContext,
        direction: Option<&mut Option<Derivative>>,
    ) -> Result<Value, ParseError> {
        let preserves_bindings = !scope.has_retained_parameter_expressions()
            && super::expr::parse_expression(expression).is_ok();
        let expression = if preserves_bindings {
            expression.to_string()
        } else {
            prepared
        };
        let binding = resolve_numeric_parameter_binding(
            &ParametricValue::Expression(expression),
            scope,
            &self.random,
        )?;
        if let Some(direction) = direction {
            *direction = if preserves_bindings && direction.is_some() {
                binding
                    .direction
                    .transpose()
                    .map_err(|error| ParseError::InvalidValue(error.to_string()))?
                    .map(|direction| direction.re)
            } else {
                None
            };
        }
        Ok(binding.value.re)
    }

    /// Resolve a deferred value expression, or keep the parse-time value.
    fn resolve_optional_value_expr(
        &self,
        value: Value,
        value_expr: &Option<String>,
        scope: &ParamContext,
        direction: Option<&mut Option<Derivative>>,
    ) -> Result<Value, ParseError> {
        match value_expr {
            Some(expr) => {
                let prepared = prepare_behavioral_expression(expr, scope).map_err(|error| {
                    ParseError::InvalidValue(format!(
                        "element value expression could not be prepared: {error}"
                    ))
                })?;
                if behavioral_expression_references_runtime_quantity(&prepared) {
                    return Err(ParseError::InvalidValue(
                        "runtime-dependent value expressions are not supported for this element"
                            .to_string(),
                    ));
                }
                self.resolve_prepared_scalar_value(expr, prepared, scope, direction)
            }
            None => Ok(value),
        }
    }

    fn resolve_source_multiplicity(
        &self,
        multiplicity: &SourceMultiplicity,
        scope: &ParamContext,
        element_path: &str,
        direction: Option<&mut Option<Derivative>>,
    ) -> Result<SourceMultiplicity, ParseError> {
        let value = self.resolve_optional_value_expr(
            multiplicity.value,
            &multiplicity.value_expr,
            scope,
            direction,
        )?;
        if !value.is_finite()
            || scope.expression_dialect() == ExpressionDialect::Xyce && value <= 0.0
        {
            return Err(ParseError::InvalidValue(format!(
                "source element '{}' has invalid multiplicity M={}",
                element_path, value
            )));
        }
        Ok(SourceMultiplicity {
            value,
            value_expr: None,
            given: multiplicity.given,
        })
    }

    fn resolve_passive_value_expr(
        &self,
        value: Value,
        value_expr: &Option<String>,
        scope: &ParamContext,
        element_path: &str,
        direction: Option<&mut Option<Derivative>>,
    ) -> Result<(Value, Option<String>), ParseError> {
        match value_expr {
            Some(expr) => {
                let prepared =
                    self.prepare_scoped_behavioral_expression(expr, scope, element_path)?;
                if behavioral_expression_references_runtime_quantity(&prepared) {
                    if let Some(direction) = direction {
                        *direction = None;
                    }
                    Ok((Value::NAN, Some(prepared)))
                } else {
                    Ok((
                        self.resolve_prepared_scalar_value(expr, prepared, scope, direction)?,
                        None,
                    ))
                }
            }
            None => Ok((value, None)),
        }
    }

    /// Merge deferred (expression-valued) instance parameters over the
    /// parse-time-resolved set, evaluating each against this instance's
    /// parameter scope. A deferred entry overrides a same-named resolved one.
    fn merge_deferred_params(
        &self,
        instance_params: &[(String, Value)],
        deferred_params: &[(String, String)],
        scope: &ParamContext,
    ) -> Result<Vec<(String, Value)>, ParseError> {
        if deferred_params.is_empty() {
            return Ok(instance_params.to_vec());
        }
        let mut merged = instance_params.to_vec();
        for (name, expr) in deferred_params {
            let prepared = prepare_behavioral_expression(expr, scope).map_err(|error| {
                ParseError::InvalidValue(format!(
                    "instance parameter '{}' could not be prepared: {}",
                    name, error
                ))
            })?;
            if behavioral_expression_references_runtime_quantity(&prepared) {
                return Err(ParseError::InvalidValue(format!(
                    "runtime-dependent instance/model parameter '{}' is not supported by this device target",
                    name
                )));
            }
            let value = self.resolve_prepared_scalar_value(expr, prepared, scope, None)?;
            match merged
                .iter_mut()
                .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
            {
                Some(slot) => slot.1 = value,
                None => merged.push((name.clone(), value)),
            }
        }
        Ok(merged)
    }

    fn resolve_deferred_source_kind(
        &self,
        raw_spec: &str,
        scope: &ParamContext,
        element_path: &str,
        voltage_source: bool,
        direction: Option<&std::cell::RefCell<[Derivative; 3]>>,
    ) -> Result<ElementKind, ParseError> {
        if scope.expression_references_spectre_statistics(raw_spec) {
            let prepared =
                super::parser::map_source_spec_values(raw_spec, scope, &|expression, nominal| {
                    if scope.expression_references_spectre_statistics(expression) {
                        self.prepare_spectre_statistical_expression(expression, scope, element_path)
                    } else {
                        Ok(nominal.to_string())
                    }
                })
                .map_err(|error| {
                    ParseError::InvalidValue(format!(
                        "statistical source specification for element '{element_path}' could not be prepared: {error}"
                    ))
                })?;
            return Ok(if voltage_source {
                ElementKind::VoltageSourceDeferred(prepared)
            } else {
                ElementKind::CurrentSourceDeferred(prepared)
            });
        }
        match super::parser::parse_source_spec_text_with_direction(raw_spec, 0, scope, direction) {
            Ok(spec) if voltage_source => Ok(ElementKind::VoltageSource(spec)),
            Ok(spec) => Ok(ElementKind::CurrentSource(spec)),
            Err(source_error) => {
                let resolution_error = || {
                    ParseError::InvalidValue(format!(
                        "source specification for element '{}' could not be resolved: {}",
                        element_path, source_error
                    ))
                };
                let expression =
                    grouped_source_expression(raw_spec).ok_or_else(resolution_error)?;
                let expression =
                    self.prepare_scoped_behavioral_expression(expression, scope, element_path)?;
                // Preserve constant-evaluation failures such as division by zero
                // instead of handing them to the runtime evaluator's domain rules.
                if !behavioral_expression_references_runtime_quantity(&expression) {
                    return Err(resolution_error());
                }
                if voltage_source {
                    Ok(ElementKind::BehavioralVoltage {
                        expression,
                        tc1: 0.0,
                        tc2: 0.0,
                        multiplicity: SourceMultiplicity::default(),
                    })
                } else {
                    Ok(ElementKind::BehavioralCurrent {
                        expression,
                        tc1: 0.0,
                        tc2: 0.0,
                        multiplicity: SourceMultiplicity::default(),
                    })
                }
            }
        }
    }

    fn prepare_scoped_behavioral_expression(
        &self,
        expression: &str,
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<String, ParseError> {
        prepare_behavioral_expression(expression, scope).map_err(|err| {
            ParseError::InvalidValue(format!(
                "behavioral expression for element '{}' could not be prepared: {}",
                element_path, err
            ))
        })
    }

    fn merge_deferred_string_params(
        &self,
        instance_params: &[(String, String)],
        deferred_params: &[(String, String)],
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<Vec<(String, String)>, ParseError> {
        let mut merged: Vec<(String, String)> = instance_params
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    super::normalize_model_string_path_value(
                        name,
                        value,
                        self.source_base_dir.as_deref(),
                    ),
                )
            })
            .collect();
        if deferred_params.is_empty() {
            return Ok(merged);
        }

        for (name, expr) in deferred_params {
            let value = if let Some((real_expr, imag_expr)) =
                super::parse_deferred_xspice_complex(expr)
            {
                self.resolve_deferred_xspice_complex_string(
                    name,
                    &real_expr,
                    &imag_expr,
                    scope,
                    element_path,
                )?
            } else {
                let raw_value = scope
                    .get_string(expr)
                    .map(ToString::to_string)
                    .ok_or_else(|| {
                        ParseError::InvalidValue(format!(
                            "XSPICE instance string parameter '{}' for element '{}' could not resolve string parameter '{}'",
                            name, element_path, expr
                        ))
                    })?;
                super::normalize_model_string_path_value(
                    name,
                    &raw_value,
                    self.source_base_dir.as_deref(),
                )
            };
            match merged
                .iter_mut()
                .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
            {
                Some(slot) => slot.1 = value,
                None => merged.push((name.clone(), value)),
            }
        }
        Ok(merged)
    }

    fn merge_deferred_real_vector_params(
        &self,
        instance_params: &[(String, Vec<Value>)],
        deferred_params: &[(String, Vec<String>)],
        scope: &ParamContext,
    ) -> Result<Vec<(String, Vec<Value>)>, ParseError> {
        if deferred_params.is_empty() {
            return Ok(instance_params.to_vec());
        }

        let mut merged = instance_params.to_vec();
        for (name, exprs) in deferred_params {
            let values = exprs
                .iter()
                .map(|expr| {
                    resolve_parametric_value(
                        &ParametricValue::Expression(expr.clone()),
                        scope,
                        &self.random,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            match merged
                .iter_mut()
                .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
            {
                Some(slot) => slot.1 = values,
                None => merged.push((name.clone(), values)),
            }
        }
        Ok(merged)
    }

    fn merge_deferred_string_vector_params(
        &self,
        instance_params: &[(String, Vec<String>)],
        deferred_params: &[(String, String)],
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<Vec<(String, Vec<String>)>, ParseError> {
        if deferred_params.is_empty() {
            return Ok(instance_params.to_vec());
        }

        let mut merged = instance_params.to_vec();
        for (name, expr) in deferred_params {
            let values = if let Some(entries) = super::parse_deferred_xspice_complex_vector(expr) {
                self.resolve_deferred_xspice_complex_vector(name, entries, scope, element_path)?
            } else {
                let value = scope
                        .get_string(expr)
                        .map(ToString::to_string)
                        .ok_or_else(|| {
                            ParseError::InvalidValue(format!(
                                "XSPICE instance string-vector parameter '{}' for element '{}' could not resolve string parameter '{}'",
                                name, element_path, expr
                            ))
                        })?;
                super::parse_xspice_string_vector_literal(&value, 1, name).map_err(|err| {
                        ParseError::InvalidValue(format!(
                            "XSPICE instance string-vector parameter '{}' for element '{}' could not parse string parameter '{}': {}",
                            name, element_path, expr, err
                        ))
                    })?
            };
            match merged
                .iter_mut()
                .find(|(existing, _)| existing.eq_ignore_ascii_case(name))
            {
                Some(slot) => slot.1 = values,
                None => merged.push((name.clone(), values)),
            }
        }
        Ok(merged)
    }

    fn resolve_deferred_xspice_complex_vector(
        &self,
        param_name: &str,
        entries: Vec<super::DeferredXspiceStringVectorEntry>,
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<Vec<String>, ParseError> {
        entries
            .into_iter()
            .map(|entry| match entry {
                super::DeferredXspiceStringVectorEntry::Resolved(value) => Ok(value),
                super::DeferredXspiceStringVectorEntry::Complex { real, imag } => self
                    .resolve_deferred_xspice_complex_string(
                        param_name,
                        &real,
                        &imag,
                        scope,
                        element_path,
                    ),
            })
            .collect()
    }

    fn resolve_deferred_xspice_complex_string(
        &self,
        param_name: &str,
        real_expr: &str,
        imag_expr: &str,
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<String, ParseError> {
        let real = self.resolve_deferred_xspice_complex_component(
            param_name,
            real_expr,
            scope,
            element_path,
            "real",
        )?;
        let imag = self.resolve_deferred_xspice_complex_component(
            param_name,
            imag_expr,
            scope,
            element_path,
            "imaginary",
        )?;

        Ok(format!(
            "<{} {}>",
            format_xspice_complex_component(real),
            format_xspice_complex_component(imag)
        ))
    }

    fn resolve_deferred_xspice_complex_component(
        &self,
        param_name: &str,
        expr: &str,
        scope: &ParamContext,
        element_path: &str,
        component: &str,
    ) -> Result<Value, ParseError> {
        resolve_parametric_value(
            &ParametricValue::Expression(expr.to_string()),
            scope,
            &self.random,
        )
        .map_err(|err| {
            ParseError::InvalidValue(format!(
                "XSPICE instance complex parameter '{}' for element '{}' could not resolve {} expression '{}': {}",
                param_name, element_path, component, expr, err
            ))
        })
    }

    fn resolve_deferred_xspice_model_complex_vector(
        &self,
        model_name: &str,
        param_name: &str,
        entries: Vec<super::DeferredXspiceStringVectorEntry>,
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<Vec<String>, ParseError> {
        entries
            .into_iter()
            .map(|entry| match entry {
                super::DeferredXspiceStringVectorEntry::Resolved(value) => Ok(value),
                super::DeferredXspiceStringVectorEntry::Complex { real, imag } => self
                    .resolve_deferred_xspice_model_complex_string(
                        model_name,
                        param_name,
                        &real,
                        &imag,
                        scope,
                        element_path,
                    ),
            })
            .collect()
    }

    fn resolve_deferred_xspice_model_complex_string(
        &self,
        model_name: &str,
        param_name: &str,
        real_expr: &str,
        imag_expr: &str,
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<String, ParseError> {
        let real = self.resolve_deferred_xspice_model_complex_component(
            model_name,
            param_name,
            real_expr,
            scope,
            element_path,
            "real",
        )?;
        let imag = self.resolve_deferred_xspice_model_complex_component(
            model_name,
            param_name,
            imag_expr,
            scope,
            element_path,
            "imaginary",
        )?;

        Ok(format!(
            "<{} {}>",
            format_xspice_complex_component(real),
            format_xspice_complex_component(imag)
        ))
    }

    fn resolve_deferred_xspice_model_complex_component(
        &self,
        model_name: &str,
        param_name: &str,
        expr: &str,
        scope: &ParamContext,
        element_path: &str,
        component: &str,
    ) -> Result<Value, ParseError> {
        resolve_parametric_value(
            &ParametricValue::Expression(expr.to_string()),
            scope,
            &self.random,
        )
        .map_err(|err| {
            ParseError::InvalidValue(format!(
                "XSPICE model '{}' complex parameter '{}' for scoped instance '{}' could not resolve {} expression '{}': {}",
                model_name, param_name, element_path, component, expr, err
            ))
        })
    }

    fn resolve_external_subcircuit_params(
        &self,
        mut element: Element,
        scope: &ParamContext,
    ) -> Result<Element, ParseError> {
        if let ElementKind::Subcircuit { params, .. } = &mut element.kind {
            for (_, value) in params.iter_mut() {
                let resolved = resolve_parametric_value(value, scope, &self.random)?;
                *value = ParametricValue::Resolved(resolved);
            }
        }
        Ok(element)
    }

    fn resolve_optional_scoped_model(
        &mut self,
        model_name: &Option<String>,
        scope: &ParamContext,
        element_path: &str,
        model_scope_path: &str,
    ) -> Result<Option<String>, ParseError> {
        model_name
            .as_deref()
            .map(|model| {
                self.resolve_native_scoped_model(model, scope, element_path, model_scope_path)
            })
            .transpose()
    }

    fn resolve_native_scoped_model(
        &mut self,
        model_name: &str,
        scope: &ParamContext,
        element_path: &str,
        model_scope_path: &str,
    ) -> Result<String, ParseError> {
        self.resolve_scoped_model(model_name, scope, element_path, model_scope_path, false)
    }

    fn resolve_xspice_scoped_model(
        &mut self,
        model_name: &str,
        scope: &ParamContext,
        element_path: &str,
        model_scope_path: &str,
    ) -> Result<String, ParseError> {
        self.resolve_scoped_model(model_name, scope, element_path, model_scope_path, true)
    }

    fn resolve_scoped_model(
        &mut self,
        model_name: &str,
        scope: &ParamContext,
        element_path: &str,
        model_scope_path: &str,
        preserve_unresolved: bool,
    ) -> Result<String, ParseError> {
        let Some((source_model_index, model_def)) = self
            .models
            .iter()
            .enumerate()
            .find(|(_, model)| model.name.eq_ignore_ascii_case(model_name))
        else {
            return Ok(model_name.to_string());
        };

        if model_def.expr_params.is_empty() && model_def.real_vector_expr_params.is_empty() {
            return Ok(model_name.to_string());
        }

        let scoped_name = scoped_model_name(model_name, model_scope_path);
        if self
            .scoped_models
            .iter()
            .any(|model| model.name.eq_ignore_ascii_case(&scoped_name))
        {
            return Ok(scoped_name);
        }

        let mut scoped_model = model_def.clone();
        scoped_model.name = scoped_name.clone();
        scoped_model.expr_params.clear();
        scoped_model.real_vector_expr_params.clear();

        for (name, expr) in &model_def.expr_params {
            if scope.expression_references_spectre_statistics(expr) {
                replace_model_param(&mut scoped_model, name);
                scoped_model.expr_params.push((
                    name.clone(),
                    self.prepare_spectre_statistical_expression(expr, scope, element_path)?,
                ));
                continue;
            }
            if let Some((real_expr, imag_expr)) = super::parse_deferred_xspice_complex(expr) {
                let value = self.resolve_deferred_xspice_model_complex_string(
                    model_name,
                    name,
                    &real_expr,
                    &imag_expr,
                    scope,
                    element_path,
                )?;
                replace_model_param(&mut scoped_model, name);
                scoped_model.string_params.push((name.clone(), value));
                continue;
            }

            if let Some(entries) = super::parse_deferred_xspice_complex_vector(expr) {
                let values = self.resolve_deferred_xspice_model_complex_vector(
                    model_name,
                    name,
                    entries,
                    scope,
                    element_path,
                )?;
                replace_model_param(&mut scoped_model, name);
                scoped_model
                    .string_vector_params
                    .push((name.clone(), values));
                continue;
            }

            if let Some(value) = scope.get_string(expr) {
                push_scoped_model_string_value(
                    &mut scoped_model,
                    name,
                    value,
                    element_path,
                    self.source_base_dir.as_deref(),
                )?;
                continue;
            }

            match super::expr::eval_expression(expr, scope) {
                Ok(value) if value.is_finite() => {
                    replace_model_param(&mut scoped_model, name);
                    scoped_model.params.push((name.clone(), value));
                }
                Ok(value) => {
                    return Err(ParseError::InvalidValue(format!(
                        "model parameter '{}' for scoped model '{}' resolved to non-finite value {}",
                        name, model_name, value
                    )));
                }
                Err(err) if preserve_unresolved => {
                    scoped_model.expr_params.push((name.clone(), expr.clone()));
                    log::debug!(
                        "Preserved unresolved expression parameter '{}'='{}' for scoped model '{}': {}",
                        name,
                        expr,
                        model_name,
                        err
                    );
                }
                Err(err) => {
                    return Err(ParseError::InvalidValue(format!(
                        "model parameter '{}' for scoped model '{}' could not be resolved against subcircuit instance '{}': {}",
                        name, model_name, element_path, err
                    )));
                }
            }
        }

        for (name, exprs) in &model_def.real_vector_expr_params {
            let mut values = Vec::with_capacity(exprs.len());
            let mut first_error = None;

            for expr in exprs {
                match super::expr::eval_expression(expr, scope) {
                    Ok(value) if value.is_finite() => values.push(value),
                    Ok(value) => {
                        return Err(ParseError::InvalidValue(format!(
                            "model vector parameter '{}' for scoped model '{}' expression '{}' resolved to non-finite value {}",
                            name, model_name, expr, value
                        )));
                    }
                    Err(err) => {
                        first_error.get_or_insert_with(|| (expr.clone(), err));
                        break;
                    }
                }
            }

            if let Some((expr, err)) = first_error {
                if preserve_unresolved {
                    scoped_model
                        .real_vector_expr_params
                        .push((name.clone(), exprs.clone()));
                    log::debug!(
                        "Preserved unresolved vector expression parameter '{}' entry '{}' for scoped model '{}': {}",
                        name,
                        expr,
                        model_name,
                        err
                    );
                } else {
                    return Err(ParseError::InvalidValue(format!(
                        "model vector parameter '{}' for scoped model '{}' could not resolve expression '{}' against subcircuit instance '{}': {}",
                        name, model_name, expr, element_path, err
                    )));
                }
            } else {
                replace_model_param(&mut scoped_model, name);
                scoped_model.real_vector_params.push((name.clone(), values));
            }
        }

        self.scoped_models.push(scoped_model);
        self.scoped_model_sources.push(source_model_index);
        Ok(scoped_name)
    }
}

fn restore_statistical_deferred_params(
    authored: &[(String, String)],
    numeric: &mut Vec<(String, Value)>,
    deferred: &mut Vec<(String, String)>,
    scope: &ParamContext,
    prepare: &impl Fn(&str) -> Result<String, ParseError>,
) -> Result<(), ParseError> {
    for (name, expression) in authored {
        if !scope.expression_references_spectre_statistics(expression) {
            continue;
        }
        numeric.retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
        deferred.retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
        deferred.push((name.clone(), prepare(expression)?));
    }
    Ok(())
}

fn apply_device_initial_conditions(
    directive: Option<&DeviceInitialConditionDirective>,
    elements: &mut [Element],
) -> Result<(), ParseError> {
    let Some(directive) = directive else {
        return Ok(());
    };
    if let DeviceInitialConditionSource::File {
        requested_path,
        resolved_path: None,
        ..
    } = &directive.source
    {
        return Err(ParseError::DeviceInitialCondition(Box::new(
            DeviceInitialConditionError::UnresolvedSource {
                origin: directive.origin.clone(),
                requested_path: requested_path.clone(),
            },
        )));
    }

    let element_indices = elements
        .iter()
        .enumerate()
        .map(|(index, element)| {
            (
                canonical_device_initial_condition_name(&element.name),
                index,
            )
        })
        .collect::<HashMap<_, _>>();

    for entry in &directive.entries {
        let canonical = canonical_device_initial_condition_name(&entry.device);
        let Some(index) = element_indices.get(&canonical).copied() else {
            continue;
        };
        apply_device_initial_condition_entry(&mut elements[index], entry)?;
    }
    Ok(())
}

fn canonical_device_initial_condition_name(name: &str) -> String {
    name.trim().replace(':', ".").to_ascii_uppercase()
}

/// Move a resolved deferred IC into the physical passive's canonical field.
/// Statistical fields stay deferred until their instance draw is materialized.
pub(crate) fn materialize_passive_initial_condition(
    element: &mut Element,
) -> Result<(), ParseError> {
    let (initial, numeric, deferred) = match &mut element.kind {
        ElementKind::Capacitor {
            initial_voltage,
            instance_params,
            deferred_params,
            ..
        } => (initial_voltage, instance_params, deferred_params),
        ElementKind::Inductor {
            initial_current,
            instance_params,
            deferred_params,
            ..
        } => (initial_current, instance_params, deferred_params),
        _ => return Ok(()),
    };
    if deferred
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("IC"))
    {
        *initial = None;
    } else {
        numeric.retain(|(name, value)| {
            if name.eq_ignore_ascii_case("IC") {
                *initial = Some(*value);
                false
            } else {
                true
            }
        });
    }
    if let Some(value) = initial
        && !value.is_finite()
    {
        return Err(ParseError::InvalidValue(format!(
            "Element '{}' IC requires a finite value, got {value}",
            element.name
        )));
    }
    Ok(())
}

fn apply_device_initial_condition_entry(
    element: &mut Element,
    entry: &super::DeviceInitialConditionEntry,
) -> Result<(), ParseError> {
    match &mut element.kind {
        ElementKind::Capacitor {
            initial_voltage,
            instance_params,
            deferred_params,
            ..
        } => {
            require_device_initial_condition_arity(entry, "exactly 1 value", 1, 1)?;
            instance_params.retain(|(name, _)| !name.eq_ignore_ascii_case("IC"));
            deferred_params.retain(|(name, _)| !name.eq_ignore_ascii_case("IC"));
            *initial_voltage = Some(entry.values[0]);
        }
        ElementKind::Mosfet {
            instance_params,
            deferred_params,
            ..
        } => {
            require_device_initial_condition_arity(entry, "between 1 and 5 values", 1, 5)?;
            const LABELS: [&str; 5] = ["IC_VDS", "IC_VGS", "IC_VBS", "IC_VES", "IC_VPS"];
            instance_params
                .retain(|(name, _)| !LABELS.iter().any(|label| name.eq_ignore_ascii_case(label)));
            deferred_params
                .retain(|(name, _)| !LABELS.iter().any(|label| name.eq_ignore_ascii_case(label)));
            instance_params.extend(
                LABELS
                    .iter()
                    .zip(&entry.values)
                    .map(|(label, value)| ((*label).to_string(), *value)),
            );
        }
        ElementKind::Inductor { .. }
        | ElementKind::JilesAthertonInductor { .. }
        | ElementKind::Coupling { .. }
        | ElementKind::Subcircuit { .. } => {}
        kind => {
            return Err(ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::UnsupportedTarget {
                    origin: entry.origin.clone(),
                    device: entry.device.clone(),
                    device_type: device_initial_condition_element_type(kind).to_string(),
                },
            )));
        }
    }
    Ok(())
}

fn device_initial_condition_element_type(kind: &ElementKind) -> &'static str {
    match kind {
        ElementKind::Resistor { .. } => "resistor",
        ElementKind::VoltageSource(_) | ElementKind::VoltageSourceDeferred(_) => "voltage source",
        ElementKind::CurrentSource(_) | ElementKind::CurrentSourceDeferred(_) => "current source",
        ElementKind::Diode { .. } => "diode",
        ElementKind::Bjt { .. } => "BJT",
        ElementKind::Jfet { .. } => "JFET",
        ElementKind::Mesfet { .. } => "MESFET",
        ElementKind::XyceMemristor { .. } => "Xyce memristor",
        ElementKind::Vcvs { .. }
        | ElementKind::Cccs { .. }
        | ElementKind::Vccs { .. }
        | ElementKind::Ccvs { .. } => "controlled source",
        ElementKind::BehavioralVoltage { .. } | ElementKind::BehavioralCurrent { .. } => {
            "behavioral source"
        }
        ElementKind::TransmissionLine { .. } => "transmission line",
        ElementKind::Xspice { .. } => "XSPICE instance",
        _ => "device",
    }
}

fn require_device_initial_condition_arity(
    entry: &super::DeviceInitialConditionEntry,
    expected: &str,
    minimum: usize,
    maximum: usize,
) -> Result<(), ParseError> {
    if (minimum..=maximum).contains(&entry.values.len()) {
        return Ok(());
    }
    Err(ParseError::DeviceInitialCondition(Box::new(
        DeviceInitialConditionError::InvalidArity {
            origin: entry.origin.clone(),
            device: entry.device.clone(),
            expected: expected.to_string(),
            actual: entry.values.len(),
        },
    )))
}

fn parametric_value_is_string(value: &ParametricValue) -> bool {
    matches!(
        value,
        ParametricValue::String(_) | ParametricValue::StringExpression(_)
    )
}

/// A parameter remains complex until an electrical scalar field consumes it.
#[derive(Clone)]
struct NumericParameterBinding {
    value: crate::ComplexValue,
    direction: Option<Result<super::expr::ComplexDirection, super::expr::ExprError>>,
}

impl NumericParameterBinding {
    fn bind(&self, name: &str, scope: &mut ParamContext) {
        scope.set_complex(name, self.value);
        scope.retain_parameter_direction(name, false, self.direction.clone());
    }
}

fn resolve_parametric_value(
    value: &ParametricValue,
    scope: &ParamContext,
    random: &RandomState,
) -> Result<Value, ParseError> {
    resolve_numeric_parameter_binding(value, scope, random).map(|binding| binding.value.re)
}

fn resolve_numeric_parameter_binding(
    value: &ParametricValue,
    scope: &ParamContext,
    random: &RandomState,
) -> Result<NumericParameterBinding, ParseError> {
    match value {
        ParametricValue::Resolved(value) => Ok(NumericParameterBinding {
            value: (*value).into(),
            direction: None,
        }),
        ParametricValue::Expression(expr) => {
            let mut context = scope.clone();
            // All derived scopes consume the same netlist-wide sequence.
            context.adopt_random(random);
            context
                .evaluate_parameter_binding(expr)
                .map(|(value, direction)| NumericParameterBinding { value, direction })
                .map_err(|error| match error {
                    super::expr::ExprError::UndefinedParam(name) => {
                        ParseError::UndefinedParameter(name)
                    }
                    other => ParseError::InvalidValue(other.to_string()),
                })
        }
        ParametricValue::String(value) => Err(ParseError::InvalidValue(format!(
            "string parameter value '{}' cannot be used as a numeric value",
            value
        ))),
        ParametricValue::StringExpression(expr) => Err(ParseError::InvalidValue(format!(
            "string parameter expression '{}' cannot be used as a numeric value",
            expr
        ))),
    }
}

fn resolve_string_parametric_value(
    value: &ParametricValue,
    scope: &ParamContext,
) -> Result<String, ParseError> {
    match value {
        ParametricValue::String(value) => Ok(value.clone()),
        ParametricValue::StringExpression(expr) | ParametricValue::Expression(expr) => scope
            .get_string(expr)
            .map(ToString::to_string)
            .ok_or_else(|| {
                ParseError::InvalidValue(format!(
                    "string parameter expression '{}' could not be resolved",
                    expr
                ))
            }),
        ParametricValue::Resolved(_) => Err(ParseError::InvalidValue(
            "numeric parameter value cannot be used as a string value".to_string(),
        )),
    }
}

fn build_subcircuit_param_scope(
    subckt: &SubcircuitDef,
    instance_name: &str,
    qualified_instance_name: &str,
    caller_scope: &ParamContext,
    instance_params: &[(String, ParametricValue)],
    random: &RandomState,
    abort: &dyn AbortSignal,
) -> Result<ParamContext, ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let (instance_numeric, instance_strings, instance_expressions) =
        resolve_subcircuit_instance_params(subckt, caller_scope, instance_params, random, abort)?;
    let instance_names = instance_params
        .iter()
        .map(|(name, _)| name.to_ascii_uppercase())
        .collect::<Vec<_>>();

    let mut scope = caller_scope.clone();
    scope.adopt_random(random);

    let formal_names = subckt
        .params
        .iter()
        .map(|(name, _)| name)
        .chain(subckt.expr_params.iter().map(|(name, _)| name))
        .chain(subckt.string_params.iter().map(|(name, _)| name))
        .map(|name| name.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let body_names = subckt
        .body_params
        .iter()
        .map(|(name, _)| name)
        .chain(subckt.body_expr_params.iter().map(|(name, _)| name))
        .chain(subckt.body_string_params.iter().map(|(name, _)| name))
        .map(|name| name.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let use_first =
        caller_scope.parameter_redefinition_policy() == ParameterRedefinitionPolicy::UseFirst;
    let formal_is_authoritative =
        |name: &str| use_first || !body_names.contains(&name.to_ascii_uppercase());
    let body_is_authoritative =
        |name: &str| !use_first || !formal_names.contains(&name.to_ascii_uppercase());

    // Defaults, body definitions and instance arguments introduce local
    // bindings. Re-mark symbolic dependencies as they are resolved below.
    for name in formal_names
        .iter()
        .chain(&body_names)
        .chain(&instance_names)
    {
        scope.shadow_spectre_statistical_parameter(name);
    }

    for (name, value) in &subckt.params {
        scope.set(name, *value);
    }
    for (name, value) in &subckt.string_params {
        scope.set_string(name, value.clone());
    }
    for (name, value) in &subckt.body_params {
        if body_is_authoritative(name) {
            scope.set(name, *value);
        }
    }
    for (name, value) in &subckt.body_string_params {
        if body_is_authoritative(name) {
            scope.set_string(name, value.clone());
        }
    }
    for function in &subckt.body_functions {
        scope.import_function(function.clone());
    }

    for (name, value) in instance_strings {
        scope.set_string(&name, value);
    }
    for (name, binding) in instance_numeric {
        binding.bind(&name, &mut scope);
    }
    for (name, expression) in instance_expressions {
        if scope.expression_references_spectre_statistics(&expression) {
            scope.mark_spectre_statistical_parameter(&name);
        }
        scope.define_parameter_expression(&name, expression, None);
    }
    let formal_expr_params = subckt
        .expr_params
        .iter()
        .filter(|(name, _)| formal_is_authoritative(name))
        .cloned()
        .collect::<Vec<_>>();
    let body_expr_params = subckt
        .body_expr_params
        .iter()
        .filter(|(name, _)| body_is_authoritative(name))
        .cloned()
        .collect::<Vec<_>>();
    let resolution_context = SubcircuitParameterResolutionContext {
        subcircuit_name: &subckt.name,
        instance_name,
        qualified_instance_name,
    };
    resolve_deferred_param_expressions(
        &formal_expr_params,
        &mut scope,
        random,
        &instance_names,
        resolution_context,
        abort,
    )?;
    resolve_deferred_param_expressions(
        &body_expr_params,
        &mut scope,
        random,
        &instance_names,
        resolution_context,
        abort,
    )?;

    Ok(scope)
}

#[derive(Clone, Copy)]
struct SubcircuitParameterResolutionContext<'a> {
    subcircuit_name: &'a str,
    instance_name: &'a str,
    qualified_instance_name: &'a str,
}

/// Runtime quantities use the behavioral compiler; static parameter functions
/// are validated by the parameter evaluator that actually implements them.
fn prepare_runtime_parameter_expression(
    name: &str,
    expression: &str,
    scope: &ParamContext,
) -> Result<Option<String>, ParseError> {
    let prepared = prepare_behavioral_expression(expression, scope).map_err(|error| {
        ParseError::InvalidValue(format!(
            "parameter expression '{name}' could not be prepared: {error}"
        ))
    })?;
    if !behavioral_expression_references_runtime_quantity(&prepared) {
        return Ok(None);
    }
    match validate_prepared_behavioral_runtime_expression(&prepared) {
        Ok(Some(identifier)) => Err(ParseError::UndefinedParameter(identifier)),
        Err(error) => Err(ParseError::InvalidValue(format!(
            "parameter expression '{name}' is invalid: {error}"
        ))),
        Ok(None) => Ok(Some(prepared)),
    }
}

fn resolve_deferred_param_expressions(
    expr_params: &[(String, String)],
    scope: &mut ParamContext,
    random: &RandomState,
    skip_names: &[String],
    context: SubcircuitParameterResolutionContext<'_>,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let mut pending = expr_params
        .iter()
        .filter(|(name, _)| {
            !skip_names
                .iter()
                .any(|skip| skip.eq_ignore_ascii_case(name))
        })
        .cloned()
        .collect::<Vec<_>>();

    while !pending.is_empty() {
        ensure_parse_not_aborted(abort)?;
        let mut progress = false;
        let mut unresolved = Vec::new();
        let mut first_error = None;

        for (definition_index, (name, expr)) in pending.into_iter().enumerate() {
            poll_parse_abort(abort, definition_index)?;
            if scope.expression_references_spectre_statistics(&expr) {
                scope.define_parameter_expression(&name, expr, None);
                scope.mark_spectre_statistical_parameter(&name);
                progress = true;
                continue;
            }
            if scope.expression_dialect() == ExpressionDialect::Xyce {
                match prepare_runtime_parameter_expression(&name, &expr, scope) {
                    Ok(Some(_)) => {
                        scope.define_parameter_expression(&name, expr, None);
                        progress = true;
                        continue;
                    }
                    Ok(None) => {}
                    Err(error) => {
                        first_error.get_or_insert(error);
                        unresolved.push((name, expr));
                        continue;
                    }
                }
            }
            match resolve_numeric_parameter_binding(
                &ParametricValue::Expression(expr.clone()),
                scope,
                random,
            ) {
                Ok(binding) => {
                    binding.bind(&name, scope);
                    progress = true;
                }
                Err(err) => {
                    first_error.get_or_insert(err);
                    unresolved.push((name, expr));
                }
            }
        }

        if !progress {
            let (parameter_name, expression) = unresolved
                .first()
                .cloned()
                .ok_or_else(|| {
                    ParseError::InvalidValue(
                        "subcircuit deferred parameter resolver made no progress without retaining an unresolved definition"
                            .to_string(),
                    )
                })?;
            let cause = first_error.unwrap_or_else(|| {
                ParseError::InvalidValue(
                    "subcircuit deferred parameters could not be resolved".to_string(),
                )
            });
            let missing_dependency = match &cause {
                ParseError::UndefinedParameter(name) => Some(name.to_ascii_uppercase()),
                _ => None,
            };
            return Err(ParseError::UnresolvedSubcircuitParameter(Box::new(
                super::UnresolvedSubcircuitParameterError {
                    subcircuit_name: context.subcircuit_name.to_string(),
                    canonical_subcircuit_name: context.subcircuit_name.to_ascii_uppercase(),
                    instance_name: context.instance_name.to_string(),
                    canonical_instance_name: context.instance_name.to_ascii_uppercase(),
                    qualified_instance_name: context.qualified_instance_name.to_string(),
                    canonical_parameter_name: parameter_name.to_ascii_uppercase(),
                    parameter_name,
                    expression,
                    missing_dependency,
                    reason: cause.to_string(),
                },
            ))
            .into());
        }
        pending = unresolved;
    }

    Ok(())
}

/// Parameters after resolution, split by what each one resolved to: numeric
/// values, string literals, and the assignments that stayed symbolic.
type ResolvedParams = (
    Vec<(String, NumericParameterBinding)>,
    Vec<(String, String)>,
    Vec<(String, String)>,
);

fn resolve_subcircuit_instance_params(
    subckt: &SubcircuitDef,
    caller_scope: &ParamContext,
    instance_params: &[(String, ParametricValue)],
    random: &RandomState,
    abort: &dyn AbortSignal,
) -> Result<ResolvedParams, ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let mut instance_scope = caller_scope.clone();
    instance_scope.adopt_random(random);
    let mut pending = instance_params.to_vec();
    let mut numeric = Vec::<(String, NumericParameterBinding)>::new();
    let mut strings = Vec::<(String, String)>::new();
    let mut expressions = Vec::<(String, String)>::new();

    while !pending.is_empty() {
        ensure_parse_not_aborted(abort)?;
        let mut progress = false;
        let mut unresolved = Vec::new();
        let mut first_error = None;

        for (definition_index, (name, value)) in pending.into_iter().enumerate() {
            poll_parse_abort(abort, definition_index)?;
            if subcircuit_instance_param_is_string(subckt, &name, &value) {
                match resolve_string_parametric_value(&value, &instance_scope) {
                    Ok(resolved) => {
                        instance_scope.shadow_spectre_statistical_parameter(&name);
                        instance_scope.set_string(&name, resolved.clone());
                        numeric.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                        expressions.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                        upsert_string_param_value(&mut strings, name, resolved);
                        progress = true;
                    }
                    Err(err) => {
                        first_error.get_or_insert(err);
                        unresolved.push((name, value));
                    }
                }
            } else {
                if let ParametricValue::Expression(expression) = &value
                    && instance_scope.expression_references_spectre_statistics(expression)
                {
                    instance_scope.define_parameter_expression(&name, expression.clone(), None);
                    instance_scope.mark_spectre_statistical_parameter(&name);
                    numeric.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                    strings.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                    upsert_expression_param_value(&mut expressions, name, expression.clone());
                    progress = true;
                    continue;
                }
                if instance_scope.expression_dialect() == ExpressionDialect::Xyce
                    && let ParametricValue::Expression(expression) = &value
                {
                    match prepare_runtime_parameter_expression(&name, expression, &instance_scope) {
                        Ok(Some(prepared)) => {
                            instance_scope.shadow_spectre_statistical_parameter(&name);
                            instance_scope.define_parameter_expression(
                                &name,
                                prepared.clone(),
                                None,
                            );
                            numeric.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                            strings.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                            upsert_expression_param_value(&mut expressions, name, prepared);
                            progress = true;
                            continue;
                        }
                        Ok(None) => {}
                        Err(error) => {
                            first_error.get_or_insert(error);
                            unresolved.push((name, value));
                            continue;
                        }
                    }
                }
                match resolve_numeric_parameter_binding(&value, &instance_scope, random) {
                    Ok(resolved) => {
                        instance_scope.shadow_spectre_statistical_parameter(&name);
                        resolved.bind(&name, &mut instance_scope);
                        strings.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                        expressions.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                        upsert_numeric_param_value(&mut numeric, name, resolved);
                        progress = true;
                    }
                    Err(err) => {
                        first_error.get_or_insert(err);
                        unresolved.push((name, value));
                    }
                }
            }
        }

        if !progress {
            return Err(first_error
                .unwrap_or_else(|| {
                    ParseError::InvalidValue(
                        "subcircuit instance parameters could not be resolved".to_string(),
                    )
                })
                .into());
        }
        pending = unresolved;
    }

    Ok((numeric, strings, expressions))
}

fn subcircuit_instance_param_is_string(
    subckt: &SubcircuitDef,
    name: &str,
    value: &ParametricValue,
) -> bool {
    subckt
        .string_params
        .iter()
        .any(|(formal, _)| formal.eq_ignore_ascii_case(name))
        || parametric_value_is_string(value)
}

fn upsert_numeric_param_value(
    items: &mut Vec<(String, NumericParameterBinding)>,
    name: String,
    value: NumericParameterBinding,
) {
    if let Some((_, existing_value)) = items
        .iter_mut()
        .find(|(existing, _)| existing.eq_ignore_ascii_case(&name))
    {
        *existing_value = value;
    } else {
        items.push((name, value));
    }
}

fn upsert_expression_param_value(
    items: &mut Vec<(String, String)>,
    name: String,
    expression: String,
) {
    if let Some((_, existing)) = items
        .iter_mut()
        .find(|(existing, _)| existing.eq_ignore_ascii_case(&name))
    {
        *existing = expression;
    } else {
        items.push((name, expression));
    }
}

fn upsert_string_param_value(items: &mut Vec<(String, String)>, name: String, value: String) {
    if let Some((_, existing_value)) = items
        .iter_mut()
        .find(|(existing, _)| existing.eq_ignore_ascii_case(&name))
    {
        *existing_value = value;
    } else {
        items.push((name, value));
    }
}

fn replace_model_param(model: &mut ModelDef, name: &str) {
    model
        .params
        .retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
    model
        .expr_params
        .retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
    model
        .string_params
        .retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
    model
        .string_vector_params
        .retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
    model
        .real_vector_params
        .retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
    model
        .real_vector_expr_params
        .retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
    model
        .integer_vector_params
        .retain(|(existing, _)| !existing.eq_ignore_ascii_case(name));
}

fn push_scoped_model_string_value(
    model: &mut ModelDef,
    name: &str,
    value: &str,
    element_path: &str,
    source_base_dir: Option<&Path>,
) -> Result<(), ParseError> {
    replace_model_param(model, name);
    let value = super::normalize_model_string_path_value(name, value, source_base_dir);
    if value.trim_start().starts_with('[') {
        match parse_scoped_model_vector_string(&value, element_path, name)? {
            ScopedModelVector::Real(values) => {
                model.real_vector_params.push((name.to_string(), values));
            }
            ScopedModelVector::String(values) => {
                model.string_vector_params.push((name.to_string(), values));
            }
        }
    } else {
        model.string_params.push((name.to_string(), value));
    }
    Ok(())
}

enum ScopedModelVector {
    Real(Vec<Value>),
    String(Vec<String>),
}

fn parse_scoped_model_vector_string(
    value: &str,
    element_path: &str,
    name: &str,
) -> Result<ScopedModelVector, ParseError> {
    let trimmed = value.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err(ParseError::InvalidValue(format!(
            "XSPICE scoped model parameter '{}' for element '{}' has malformed vector string '{}'",
            name, element_path, value
        )));
    }

    let inner = &trimmed[1..trimmed.len() - 1];
    let fields = split_vector_fields(inner);
    if fields.is_empty() {
        return Err(ParseError::InvalidValue(format!(
            "XSPICE scoped model parameter '{}' for element '{}' has an empty vector",
            name, element_path
        )));
    }

    let mut numeric_values = Vec::with_capacity(fields.len());
    let mut all_numeric = true;
    for field in &fields {
        match super::lexer::parse_spice_value_complete(field) {
            Ok(value) if value.is_finite() => numeric_values.push(value),
            _ => {
                all_numeric = false;
                break;
            }
        }
    }

    if all_numeric {
        return Ok(ScopedModelVector::Real(numeric_values));
    }

    Ok(ScopedModelVector::String(
        fields
            .into_iter()
            .map(|field| strip_local_string_literal(&field).to_string())
            .collect(),
    ))
}

fn strip_local_string_literal(value: &str) -> &str {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.as_bytes()[0] as char;
        let last = trimmed.as_bytes()[trimmed.len() - 1] as char;
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return &trimmed[1..trimmed.len() - 1];
        }
    }
    trimmed
}

fn split_vector_fields(input: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut single_quote = false;
    let mut double_quote = false;

    for ch in input.chars() {
        match ch {
            '\'' if !double_quote => {
                single_quote = !single_quote;
                current.push(ch);
            }
            '"' if !single_quote => {
                double_quote = !double_quote;
                current.push(ch);
            }
            ',' | ' ' | '\t' if !single_quote && !double_quote => {
                if !current.is_empty() {
                    fields.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        fields.push(current);
    }

    fields
}

fn scoped_model_name(model_name: &str, element_path: &str) -> String {
    let suffix: String = element_path
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect();
    format!("{model_name}__{suffix}")
}

/// Multiply an element's effective parallel multiplicity by `m`.
///
/// Composes an inherited X-line multiplicity into the element's own `M`
/// instance parameter (devices), the child instance's `M` binding (nested
/// subcircuits, so the next expansion level applies it recursively), or the
/// source amplitudes (current sources). Voltage-like elements are left
/// untouched: parallel copies of an ideal voltage source are electrically
/// identical to a single one.
fn apply_element_multiplicity(element: &mut Element, m: Value) {
    match &mut element.kind {
        ElementKind::RfPortDeferred { multiplicity, .. } => *multiplicity *= m,
        ElementKind::Resistor {
            instance_params, ..
        }
        | ElementKind::Capacitor {
            instance_params, ..
        }
        | ElementKind::Inductor {
            instance_params, ..
        }
        | ElementKind::Diode {
            instance_params, ..
        }
        | ElementKind::Bjt {
            instance_params, ..
        }
        | ElementKind::Mosfet {
            instance_params, ..
        }
        | ElementKind::Jfet {
            instance_params, ..
        }
        | ElementKind::Mesfet {
            instance_params, ..
        } => {
            scale_multiplicity_param(instance_params, m);
        }
        ElementKind::Subcircuit { params, .. } => {
            if let Some((_, value)) = params
                .iter_mut()
                .find(|(name, _)| name.eq_ignore_ascii_case("M"))
            {
                let composed = match &*value {
                    ParametricValue::Resolved(v) => ParametricValue::Resolved(*v * m),
                    ParametricValue::Expression(expr) => {
                        ParametricValue::Expression(format!("({})*({})", expr, m))
                    }
                    ParametricValue::String(_) | ParametricValue::StringExpression(_) => {
                        value.clone()
                    }
                };
                *value = composed;
            } else {
                params.push(("M".to_string(), ParametricValue::Resolved(m)));
            }
        }
        ElementKind::CurrentSource(spec) => scale_source_amplitudes(spec, m),
        ElementKind::Vccs { multiplicity, .. }
        | ElementKind::BehavioralCurrent { multiplicity, .. } => {
            multiplicity.value *= m;
        }
        ElementKind::PspiceChebyshev {
            voltage_output: false,
            multiplicity,
            ..
        } => {
            if let Some(expression) = &mut multiplicity.value_expr {
                *expression = format!("({expression})*({m})");
            } else {
                multiplicity.value *= m;
            }
        }
        _ => {}
    }
}

/// Fold a multiplicity factor into an instance-parameter list, composing
/// with any `M`/`MULT` the instance already carries.
fn scale_multiplicity_param(instance_params: &mut Vec<(String, Value)>, m: Value) {
    if let Some((_, value)) = instance_params
        .iter_mut()
        .find(|(name, _)| name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT"))
    {
        *value *= m;
    } else {
        instance_params.push(("M".to_string(), m));
    }
}

/// Scale every amplitude-like quantity of a source specification by `m`,
/// recursing through combined DC/AC/transient forms. Time-like quantities
/// (delays, frequencies, time constants) are never touched.
fn scale_source_amplitudes(spec: &mut SourceSpec, m: Value) {
    match spec {
        SourceSpec::Distortion { inner, f1, f2 } => {
            if let Some(tone) = f1 {
                tone.magnitude *= m;
            }
            if let Some(tone) = f2 {
                tone.magnitude *= m;
            }
            scale_source_amplitudes(inner, m);
        }
        SourceSpec::RfPort { inner, .. } => scale_source_amplitudes(inner, m),
        SourceSpec::Dc(v) => *v *= m,
        SourceSpec::Ac { magnitude, .. } => *magnitude *= m,
        SourceSpec::DcAc {
            dc_value,
            ac_magnitude,
            ..
        } => {
            *dc_value *= m;
            *ac_magnitude *= m;
        }
        SourceSpec::DcTransient {
            dc_value,
            transient,
        } => {
            *dc_value *= m;
            scale_source_amplitudes(transient, m);
        }
        SourceSpec::AcTransient {
            ac_magnitude,
            transient,
            ..
        } => {
            *ac_magnitude *= m;
            scale_source_amplitudes(transient, m);
        }
        SourceSpec::DcAcTransient {
            dc_value,
            ac_magnitude,
            transient,
            ..
        } => {
            *dc_value *= m;
            *ac_magnitude *= m;
            scale_source_amplitudes(transient, m);
        }
        SourceSpec::Pulse { v1, v2, .. } => {
            *v1 *= m;
            *v2 *= m;
        }
        SourceSpec::Sin {
            offset, amplitude, ..
        } => {
            *offset *= m;
            *amplitude *= m;
        }
        SourceSpec::Pwl { points, .. } => {
            for (_, value) in points {
                *value *= m;
            }
        }
        SourceSpec::PwlFile {
            value_scale,
            value_offset,
            ..
        } => {
            *value_scale *= m;
            *value_offset *= m;
        }
        SourceSpec::Pat { vhi, vlo, .. } => {
            *vhi *= m;
            *vlo *= m;
        }
        SourceSpec::Exp { v1, v2, .. } => {
            *v1 *= m;
            *v2 *= m;
        }
        SourceSpec::Sffm {
            offset, amplitude, ..
        } => {
            *offset *= m;
            *amplitude *= m;
        }
        SourceSpec::Am {
            offset,
            modulation_offset,
            modulation_amplitude,
            ..
        } => {
            *offset *= m;
            *modulation_offset *= m;
            *modulation_amplitude *= m;
        }
        SourceSpec::TrNoise {
            na,
            namp,
            rts_amplitude,
            ..
        } => {
            // Deterministic multiplicity convention: parallel copies share
            // one sample train, so amplitudes scale linearly. Physically
            // uncorrelated devices would scale as sqrt(m); model that by
            // adjusting NA/NAMP explicitly in the deck.
            *na *= m;
            *namp *= m;
            *rts_amplitude *= m;
        }
        SourceSpec::TrRandom {
            parameter1,
            parameter2,
            ..
        } => {
            *parameter1 *= m;
            *parameter2 *= m;
        }
    }
}

/// Convenience function to flatten a netlist
pub fn flatten_netlist(netlist: &Netlist) -> Result<Vec<Element>, ParseError> {
    let mut flattener = Flattener::new(&netlist.subcircuits);
    flattener.flatten(netlist)
}

/// Convenience function to flatten a netlist and return instance-scoped model
/// definitions created during subcircuit expansion.
pub fn flatten_netlist_with_models(netlist: &Netlist) -> Result<FlattenedNetlist, ParseError> {
    finish_non_aborting_parse(flatten_netlist_with_models_with_abort(netlist, &NoAbort))
}

/// Flatten a netlist while cooperatively observing `abort` throughout
/// hierarchy expansion and deferred parameter resolution.
pub fn flatten_netlist_with_models_with_abort(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<FlattenedNetlist, ParseWithAbortError> {
    flatten_netlist_with_models_config_with_abort(netlist, FlattenerConfig::default(), abort)
}

pub(crate) fn flatten_netlist_with_models_config_with_abort(
    netlist: &Netlist,
    config: FlattenerConfig,
    abort: &dyn AbortSignal,
) -> Result<FlattenedNetlist, ParseWithAbortError> {
    flatten_netlist_with_parameter_direction(netlist, config, abort).map(|(flattened, _)| flattened)
}

pub(crate) fn flatten_netlist_with_parameter_direction(
    netlist: &Netlist,
    config: FlattenerConfig,
    abort: &dyn AbortSignal,
) -> Result<(FlattenedNetlist, Option<Box<ParameterDirectionCapture>>), ParseWithAbortError> {
    let mut flattener =
        Flattener::with_models_config(&netlist.subcircuits, &netlist.models, config);
    let elements = flattener.flatten_with_abort(netlist, abort)?;
    Ok((
        FlattenedNetlist {
            elements,
            scoped_models: flattener.scoped_models,
            scoped_initial_conditions: flattener.scoped_initial_conditions,
            scoped_node_sets: flattener.scoped_node_sets,
            scoped_startup_directives: flattener.scoped_startup_directives,
            xspice_auto_bridge_node_hints: flattener.xspice_auto_bridge_node_hints,
        },
        flattener.parameter_direction,
    ))
}

fn xspice_auto_bridge_family(
    string_params: &[(String, String)],
    scope: &ParamContext,
) -> Option<String> {
    string_params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("family"))
        .map(|(_, value)| value.trim())
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .or_else(|| {
            scope
                .get_string("family")
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToString::to_string)
        })
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '#' || c == ':'
}

fn extract_parenthesized(chars: &[char], lparen_idx: usize) -> Option<(String, usize)> {
    if chars.get(lparen_idx).copied() != Some('(') {
        return None;
    }

    let mut depth = 0usize;
    let mut i = lparen_idx;
    while i < chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    let inner: String = chars[lparen_idx + 1..i].iter().collect();
                    return Some((inner, i));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn split_top_level_commas(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0usize;
    for c in input.chars() {
        match c {
            '(' => {
                depth += 1;
                current.push(c);
            }
            ')' => {
                depth = depth.saturating_sub(1);
                current.push(c);
            }
            ',' if depth == 0 => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    parts.push(current.trim().to_string());
    parts
}

fn remap_voltage_probe_args(
    flattener: &Flattener<'_>,
    args: &str,
    prefix: &str,
    node_map: &HashMap<String, String>,
) -> String {
    let parts = split_top_level_commas(args);
    if parts.len() == 1 {
        return remap_probe_node(flattener, &parts[0], prefix, node_map);
    }
    if parts.len() == 2 {
        let a = remap_probe_node(flattener, &parts[0], prefix, node_map);
        let b = remap_probe_node(flattener, &parts[1], prefix, node_map);
        return format!("{}, {}", a, b);
    }
    args.trim().to_string()
}

fn remap_current_probe_arg(flattener: &Flattener<'_>, prefix: &str, arg: &str) -> String {
    let trimmed = arg.trim();
    if trimmed.is_empty() || !is_simple_probe_name(trimmed) {
        return trimmed.to_string();
    }
    flattener.qualify_hierarchy_name(prefix, trimmed)
}

fn remap_probe_node(
    flattener: &Flattener<'_>,
    arg: &str,
    prefix: &str,
    node_map: &HashMap<String, String>,
) -> String {
    let trimmed = arg.trim();
    if trimmed.is_empty() || !is_simple_probe_name(trimmed) {
        return trimmed.to_string();
    }
    flattener.remap_node(trimmed, prefix, node_map)
}

fn is_simple_probe_name(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '#' | ':' | '!'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::CountingAbort;

    #[test]
    fn statistical_parameter_expansion_preserves_arithmetic_and_branches() {
        let mut scope = ParamContext::new();
        scope.set("rv", 100.0);
        scope.mark_spectre_statistical_parameter("rv");
        scope.define_parameter_expression("derived", "2*rv", Some(200.0.into()));
        scope.mark_spectre_statistical_parameter("derived");
        scope.define_global_expression("offset", "7", Some(7.0.into()));
        scope.set("offset", 5.0);
        scope.define_function("choose", vec!["x".into()], "if(x<100,3,7)");
        let flattener = Flattener::new(&[]);
        let mut failures = Vec::new();
        for function_count in [1, 65] {
            for index in 1..function_count {
                scope.define_function(&format!("unused{index}"), vec!["x".into()], "x");
            }
            for (expression, below, above) in [
                ("rv*2", 180.0, 220.0),
                ("derived+rv", 270.0, 330.0),
                ("rv+offset", 95.0, 115.0),
                ("choose(rv)", 3.0, 7.0),
                ("choose(110)", 7.0, 7.0),
                ("if(rv<100,rv,2*rv)", 90.0, 220.0),
            ] {
                let prepared = flattener
                    .prepare_spectre_statistical_expression(expression, &scope, "X1.R1")
                    .expect("statistical expression prepares");
                for (sample, expected) in [(90.0, below), (110.0, above)] {
                    let mut sampled = scope.clone();
                    sampled.set("rv", sample);
                    sampled.set("offset", 999.0);
                    let actual = super::super::expr::eval_expression(&prepared, &sampled)
                        .expect("prepared expression evaluates at the sampled coordinate");
                    if actual != expected {
                        failures.push(format!(
                            "{function_count} functions, {expression} at rv={sample}: {prepared} gives {actual}, expected {expected}"
                        ));
                    }
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn subcircuit_bindings_preserve_complex_scopes_and_physical_values() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let netlist = Netlist::parse_with_options(
                "Complex instance bindings
.param z={2+3j}
Xdirect d 0 cell z={z+1j} next={z+1j}
Xnested n 0 wrapper z={z}
Xdefault f 0 cell
.subckt wrapper a b z=100
.param local={z+2j}
Xchild a b cell z={local} next={z+1j}
.ends
.subckt cell a b z={1+2j} next={z+1j}
.param squared={z*z}
.func part(x) {img(x)}
R1 a b {1+part(z)}
R2 a b {1+img(squared)}
R3 a b 1 M={1+img(next)}
V1 inside b DC {1+img(next)} AC {1+img(next)}
.ends
.end",
                super::super::parser::NetlistParseOptions {
                    expression_dialect: dialect,
                    ..Default::default()
                },
            )
            .unwrap();
            let circuit = crate::engine::Engine::default()
                .build_circuit(&netlist)
                .unwrap();
            for (scope, r1, r2, magnitude) in [
                ("Xdirect", 5.0, 17.0, 6.0),
                ("Xnested.Xchild", 6.0, 21.0, 7.0),
                ("Xdefault", 3.0, 5.0, 4.0),
            ] {
                for (name, conductance) in [("R1", 1.0 / r1), ("R2", 1.0 / r2), ("R3", magnitude)] {
                    let name = format!("{scope}.{name}");
                    let index = circuit
                        .resistors
                        .names
                        .iter()
                        .position(|actual| actual.eq_ignore_ascii_case(&name))
                        .unwrap();
                    assert_eq!(
                        circuit.resistors.conductances[index], conductance,
                        "{dialect:?} {name}"
                    );
                }
                let name = format!("{scope}.V1");
                let index = circuit
                    .voltage_sources
                    .names
                    .iter()
                    .position(|actual| actual.eq_ignore_ascii_case(&name))
                    .unwrap();
                assert_eq!(
                    circuit.voltage_sources.dc_values[index], magnitude,
                    "{dialect:?} {name}"
                );
                assert_eq!(
                    circuit.voltage_sources.ac_magnitudes[index], magnitude,
                    "{dialect:?} {name}"
                );
            }
        }
    }

    #[test]
    fn nested_instance_arguments_preserve_sequential_random_bindings() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            for mode in [
                super::super::StatisticalParamMode::Sample,
                super::super::StatisticalParamMode::Nominal,
            ] {
                let make = |nested| {
                    let instances = if nested {
                        "Xtop out 0 wrapper\n.subckt wrapper a b z=999\nX1 a b cell z={aunif(2,0.25)} next={z+1}\nX2 a b cell z={aunif(3,0.5)} next={z+1}\n.ends"
                    } else {
                        "X1 out 0 cell z={aunif(2,0.25)} next={z+1}\nX2 out 0 cell z={aunif(3,0.5)} next={z+1}"
                    };
                    Netlist::parse_with_options(&format!(
                        "Sequential argument draws\n{instances}\n.subckt cell a b z=99 next=99\n.param sampled={{aunif(5,0.5)}}\nR1 a b {{next}}\nR2 a b {{sampled}}\n.ends\n.end"
                    ), super::super::parser::NetlistParseOptions {
                        expression_dialect: dialect,
                        statistical_mode: mode,
                        statistical_seed: Some(793),
                        ..Default::default()
                    }).unwrap()
                };
                let direct = make(false);
                let nested = make(true);
                let engine = crate::engine::Engine::default();
                let direct_circuit = engine.build_circuit(&direct).unwrap();
                let next_draw = direct.params.random().next_uniform();
                let nested_circuit = engine.build_circuit(&nested).unwrap();
                assert_eq!(
                    nested_circuit.resistors.reported_resistances,
                    direct_circuit.resistors.reported_resistances,
                    "{dialect:?} {mode:?}"
                );
                assert_eq!(nested.params.random().next_uniform(), next_draw);
                let expected_stream = RandomState::new(793);
                let sample = |mean, spread| match mode {
                    super::super::StatisticalParamMode::Sample => {
                        mean + spread * expected_stream.next_symmetric()
                    }
                    super::super::StatisticalParamMode::Nominal => mean,
                };
                let expected = [
                    sample(2.0, 0.25) + 1.0,
                    sample(5.0, 0.5),
                    sample(3.0, 0.5) + 1.0,
                    sample(5.0, 0.5),
                ];
                assert_eq!(nested_circuit.resistors.reported_resistances, expected);
                assert_eq!(next_draw, expected_stream.next_uniform());
                assert!((2.75..=3.25).contains(&nested_circuit.resistors.reported_resistances[0]));
                assert!((3.5..=4.5).contains(&nested_circuit.resistors.reported_resistances[2]));
            }
        }
    }

    #[test]
    fn numeric_subcircuit_bindings_retain_complex_parameter_directions() {
        for dialect in [ExpressionDialect::Ngspice, ExpressionDialect::Xyce] {
            let netlist = Netlist::parse_with_options(
                "Parameter directions through local bindings\n.param p=0\n.subckt cell a b z=99 next=99\n.param squared={z*z} cusp={abs(p)} alias=cusp\nR1 a b 1\n.ends\n.end",
                super::super::parser::NetlistParseOptions {
                    expression_dialect: dialect,
                    ..Default::default()
                },
            ).unwrap();
            let mut caller = netlist.params.clone();
            caller.seed_parameter_direction("p", false);
            let scope = build_subcircuit_param_scope(
                &netlist.subcircuits[0],
                "X1",
                "Xtop.X1",
                &caller,
                &[
                    (
                        "z".into(),
                        ParametricValue::Expression("1+1e-8*p+2j".into()),
                    ),
                    ("next".into(), ParametricValue::Expression("z+3j".into())),
                    ("label".into(), ParametricValue::Resolved(1.0)),
                    ("label".into(), ParametricValue::String("new".into())),
                    ("number".into(), ParametricValue::String("old".into())),
                    ("number".into(), ParametricValue::Resolved(7.0)),
                ],
                caller.random(),
                &NoAbort,
            )
            .unwrap();
            assert_eq!(
                scope.get_complex("z"),
                Some(crate::ComplexValue::new(1.0, 2.0))
            );
            assert_eq!(
                scope.get_complex("next"),
                Some(crate::ComplexValue::new(1.0, 5.0))
            );
            assert_eq!(
                scope.get_complex("squared"),
                Some(crate::ComplexValue::new(-3.0, 4.0))
            );
            assert_eq!(
                scope.parameter_direction("next").unwrap().binary64(),
                crate::ComplexValue::new(1e-8, 0.0)
            );
            assert_eq!(
                scope.parameter_direction("squared").unwrap().binary64(),
                crate::ComplexValue::new(2e-8, 4e-8)
            );
            assert_eq!(scope.get_string("label"), Some("new"));
            assert_eq!(scope.get("number"), Some(7.0));
            assert_eq!(scope.get("alias"), Some(0.0));
            assert!(scope.parameter_direction("alias").is_err());
        }
    }

    #[test]
    fn statistical_parameter_expansion_reaches_the_built_circuit() {
        use crate::netlist::{
            SpectreDistribution, SpectreSpread, SpectreStatisticalCoordinate,
            SpectreStatisticsPlan, SpectreVariation, SpectreVariationScope,
        };
        let mut plan = SpectreStatisticsPlan {
            variations: vec![SpectreVariation {
                line: 3,
                scope: SpectreVariationScope::Process,
                parameter: "rv".into(),
                distribution: SpectreDistribution::Gaussian,
                spread: SpectreSpread::StandardDeviation("5".into()),
                percent: false,
            }],
            correlations: vec![],
        };
        let coordinate = SpectreStatisticalCoordinate {
            seed: 41,
            monte_carlo_run: 9,
            temperature_celsius: 27.0,
            axes: vec![],
        };
        for (variation_scope, function_count) in [
            (SpectreVariationScope::Process, 0),
            (SpectreVariationScope::Process, 65),
            (SpectreVariationScope::Mismatch, 0),
            (SpectreVariationScope::Mismatch, 65),
        ] {
            plan.variations[0].scope = variation_scope;
            let mut definitions = String::new();
            for index in 0..function_count {
                definitions.push_str(&format!(".func unused{index}(x) {{x}}\n"));
            }
            let mut netlist = Netlist::parse(&format!(
                "statistical expansion\n.param rv=100\n.RSPICE_SPECTRE_STAT {}\n{definitions}\
                 .func hidden(x) {{rv*x}}\n.func nested(x) {{hidden(x)+1}}\n\
                 .param scale=2 top_alias={{scale*rv}} bare_alias=top_alias\n\
                 .param z={{3+4J}}\n.func captured(x) {{scale*x+img(z)}}\n\
                 .param captured_alias={{captured(rv)}} noise_capture={{rv+agauss(0,1,1)}}\n\
                 .param scale=9 top_alias=123 root_nested={{bare_alias+rv}}\n\
                 .param z=0\n.func captured(x) {{9*x}}\n\
                 RROOT in 0 {{hidden(2)}}\nVSTAT stat 0 DC {{nested(1)}}\n\
                 .subckt unit a b\n.param derived={{2*rv}}\n.param indirect={{nested(3)}}\n\
                 R1 a b {{if(rv<1e6,derived,3*rv)}}\n\
                 R2 a b {{hidden(2)}}\nR3 a b {{nested(3)}}\nR4 a b {{indirect}}\n\
                 .model RSTAT R(R={{hidden(2)}})\nR5 a b 1 RSTAT\n\
                 R6 a b {{bare_alias}}\nR7 a b {{root_nested}}\nR8 a b {{top_alias}}\n\
                 R9 a b {{captured_alias}}\nR10 a b {{noise_capture}}\n.ends\n\
                 .subckt shadow a b rv=50\n.param local={{2*rv}}\n\
                 R1 a b {{rv}}\nR2 a b {{local}}\nR3 a b {{bare_alias}}\n\
                 R4 a b {{rv+bare_alias}}\n.model LOCAL R(R={{rv}})\nR5 a b 1 LOCAL\n\
                 VS sv b DC {{rv+bare_alias}} AC rv local SIN(rv {{bare_alias}} 1k)\n\
                 IS a b DC {{rv+bare_alias}} AC rv local PWL(0 rv 1m {{rv+bare_alias}})\n.ends\n\
                 .subckt body_shadow a b\n.param rv=60\n\
                 R1 a b {{rv}}\nR2 a b {{bare_alias}}\n\
                 XCH a b shadow rv=80\n.ends\n\
                 X1 in 0 unit\nX2 in 0 shadow\nX3 in 0 shadow rv=70\n\
                 X4 in 0 body_shadow\nV1 in 0 1\n.end\n",
                plan.encode_internal()
            ))
            .expect("statistical deck parses");
            let process = plan.sample_process(&netlist.params, &coordinate).unwrap();
            let sample_for = |identity| {
                plan.sample_mismatch(&netlist.params, &process, identity, &coordinate)
                    .unwrap()
                    .get("RV")
                    .or_else(|| process.get("RV"))
                    .copied()
                    .unwrap()
            };
            let sample = sample_for("X1");
            let root_sample = sample_for("RROOT");
            let source_sample = sample_for("VSTAT");
            let captured_noise = netlist.params.get("noise_capture").unwrap() - 100.0;
            let expected = if sample < 1e6 {
                2.0 * sample
            } else {
                3.0 * sample
            };
            netlist.spectre_statistical_coordinate = Some(coordinate.clone());
            let circuit = crate::Engine::new(crate::SimulationConfig::default())
                .build_circuit(&netlist)
                .expect("sampled hierarchical circuit builds");
            let mut failures = Vec::new();
            for (name, expected) in [
                ("RROOT", 2.0 * root_sample),
                ("X1.R1", expected),
                ("X1.R2", 2.0 * sample),
                ("X1.R3", 3.0 * sample + 1.0),
                ("X1.R4", 3.0 * sample + 1.0),
                ("X1.R5", 2.0 * sample),
                ("X1.R6", 2.0 * sample),
                ("X1.R7", 3.0 * sample),
                ("X1.R8", 123.0),
                ("X1.R9", 2.0 * sample + 4.0),
                ("X1.R10", sample + captured_noise),
                ("X2.R1", 50.0),
                ("X2.R2", 100.0),
                ("X2.R3", 2.0 * sample_for("X2")),
                ("X2.R4", 50.0 + 2.0 * sample_for("X2")),
                ("X2.R5", 50.0),
                ("X3.R1", 70.0),
                ("X3.R2", 140.0),
                ("X3.R3", 2.0 * sample_for("X3")),
                ("X3.R4", 70.0 + 2.0 * sample_for("X3")),
                ("X3.R5", 70.0),
                ("X4.R1", 60.0),
                ("X4.R2", 2.0 * sample_for("X4")),
                ("X4.XCH.R1", 80.0),
                ("X4.XCH.R2", 160.0),
                ("X4.XCH.R3", 2.0 * sample_for("X4.XCH")),
                ("X4.XCH.R4", 80.0 + 2.0 * sample_for("X4.XCH")),
                ("X4.XCH.R5", 80.0),
            ] {
                let index = circuit
                    .resistors
                    .names
                    .iter()
                    .position(|actual| actual.eq_ignore_ascii_case(name))
                    .expect("resistor exists");
                let actual = circuit.resistors.conductances[index].recip();
                let relative = (actual / expected - 1.0).abs();
                if !relative.is_finite() || relative >= 1e-14 {
                    failures.push(format!(
                        "{variation_scope:?}, {function_count} extra functions, {name}: sampled resistance {actual}, expected {expected}"
                    ));
                }
            }
            let source_index = circuit
                .voltage_sources
                .names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("VSTAT"))
                .expect("source exists");
            let actual = circuit.voltage_sources.dc_values[source_index];
            let expected = source_sample + 1.0;
            let relative = (actual / expected - 1.0).abs();
            if !relative.is_finite() || relative >= 1e-14 {
                failures.push(format!(
                    "{variation_scope:?}: sampled source {actual}, expected {expected}"
                ));
            }
            for (scope, local) in [("X2", 50.0), ("X3", 70.0), ("X4.XCH", 80.0)] {
                let sampled = sample_for(scope);
                for (kind, names, dc, magnitudes, phases, specs, peak_time) in [
                    (
                        "VS",
                        &circuit.voltage_sources.names,
                        &circuit.voltage_sources.dc_values,
                        &circuit.voltage_sources.ac_magnitudes,
                        &circuit.voltage_sources.ac_phases,
                        &circuit.voltage_sources.source_specs,
                        0.25e-3,
                    ),
                    (
                        "IS",
                        &circuit.current_sources.names,
                        &circuit.current_sources.dc_values,
                        &circuit.current_sources.ac_magnitudes,
                        &circuit.current_sources.ac_phases,
                        &circuit.current_sources.source_specs,
                        1e-3,
                    ),
                ] {
                    let name = format!("{scope}.{kind}");
                    let index = names
                        .iter()
                        .position(|actual| actual.eq_ignore_ascii_case(&name))
                        .unwrap();
                    let spec = specs[index].as_ref().unwrap();
                    let at_time = |time| {
                        crate::circuit::VoltageSources::evaluate_source_spec_at_time_with_dialect(
                            spec,
                            time,
                            1e-9,
                            1.0,
                            crate::config::SpiceDialect::Ngspice,
                        )
                    };
                    for (field, actual, expected) in [
                        ("DC", dc[index], local + 2.0 * sampled),
                        ("AC magnitude", magnitudes[index], local),
                        ("AC phase", phases[index], (2.0 * local).to_radians()),
                        ("waveform start", at_time(0.0), local),
                        ("waveform peak", at_time(peak_time), local + 2.0 * sampled),
                    ] {
                        let relative = (actual / expected - 1.0).abs();
                        if !relative.is_finite() || relative >= 1e-14 {
                            failures.push(format!(
                                "{variation_scope:?}, {name} {field}: {actual}, expected {expected}"
                            ));
                        }
                    }
                }
            }
            assert!(failures.is_empty(), "{}", failures.join("\n"));
        }
    }

    fn duplicate_binding_error(source: &str) -> Box<DuplicateSubcircuitPortBindingError> {
        let netlist = Netlist::parse(source).expect("duplicate-formal deck parses");
        let error = flatten_netlist(&netlist).expect_err("conflicting invocation must fail");
        let ParseError::DuplicateSubcircuitPortBinding(error) = error else {
            panic!("expected typed duplicate subcircuit-port binding error");
        };
        error
    }

    #[test]
    fn undefined_subcircuit_is_a_typed_hierarchy_error() {
        let netlist = Netlist::parse(
            "undefined subcircuit\n\
             X1 in 0 missing\n\
             .end\n",
        )
        .expect("the X-line is syntactically valid");
        let error = flatten_netlist(&netlist).expect_err("MISSING is undefined");
        let ParseError::UndefinedSubcircuit(error) = error else {
            panic!("expected typed undefined-subcircuit error");
        };
        assert_eq!(error.subcircuit_name, "missing");
        assert_eq!(error.canonical_subcircuit_name, "MISSING");
        assert_eq!(error.instance_name, "X1");
        assert_eq!(error.canonical_instance_name, "X1");
        assert_eq!(error.qualified_instance_name, "X1");
    }

    #[test]
    fn unselected_veriloga_imports_defer_binding_only_for_the_current_deck() {
        let mut flattener = Flattener::new(&[]);
        for (include, deferred) in [
            (".va source.va", cfg!(feature = "veriloga")),
            (".va source.va module=Selected", false),
            (".va source.va alias", cfg!(feature = "veriloga")),
            ("", false),
        ] {
            let netlist = Netlist::parse(&format!(
                "external discovery\n{include}\nX1 a 0 Missing\nD1 a 0 MissingDiode\n.end\n"
            ))
            .unwrap();
            let diagnostics = netlist.lint_unknown_references();
            assert!(diagnostics.iter().any(|item| item.element == "D1"));
            assert_eq!(
                diagnostics.iter().any(|item| item.element == "X1"),
                !deferred
            );
            let result = flattener.flatten(&netlist);
            if deferred {
                assert_eq!(result.unwrap().len(), 2);
            } else {
                assert!(matches!(result, Err(ParseError::UndefinedSubcircuit(_))));
            }
        }
    }

    #[test]
    fn subcircuit_body_parameter_assignment_spacing_survives_flattening() {
        let netlist = Netlist::parse(
            "subcircuit parameter assignment spacing\n\
             .subckt CELL a b\n\
             .param A=1 B =2 C= 3 D = 4\n\
             L1 a n1 A\n\
             L2 n1 n2 B\n\
             L3 n2 n3 C\n\
             L4 n3 b D\n\
             .ends\n\
             X1 in 0 CELL\n\
             .end\n",
        )
        .expect("all assignment spacings parse");
        let cell = netlist
            .subcircuits
            .iter()
            .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case("CELL"))
            .expect("CELL definition retained");
        for (name, expected) in [("A", 1.0f64), ("B", 2.0f64), ("C", 3.0f64), ("D", 4.0f64)] {
            assert!(cell.body_params.iter().any(|(actual_name, actual_value)| {
                actual_name.eq_ignore_ascii_case(name)
                    && actual_value.to_bits() == expected.to_bits()
            }));
        }

        let flattened = flatten_netlist(&netlist).expect("parameterized inductors flatten");
        for (name, expected) in [
            ("X1.L1", 1.0f64),
            ("X1.L2", 2.0f64),
            ("X1.L3", 3.0f64),
            ("X1.L4", 4.0f64),
        ] {
            assert!(flattened.iter().any(|element| {
                element.name.eq_ignore_ascii_case(name)
                    && matches!(&element.kind, ElementKind::Inductor {
                        value,
                        value_expr: None,
                        ..
                    } if value.to_bits() == expected.to_bits())
            }));
        }
    }

    #[test]
    fn repeated_formals_are_legal_when_effective_actual_nodes_match() {
        let netlist = Netlist::parse(
            "compatible duplicate formals\n\
             .SUBCKT DUP a b A g G\n\
             R1 a b 1\n\
             .ENDS\n\
             X1 Input Out input 0 GND DUP\n\
             .END\n",
        )
        .expect("parser must not reject duplicate formal ports");
        let duplicate = netlist
            .subcircuits
            .iter()
            .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case("DUP"))
            .expect("duplicate-port definition retained");
        assert_eq!(duplicate.ports, ["A", "B", "A", "G", "G"]);

        let flattened = flatten_netlist(&netlist)
            .expect("case-equivalent and ground-equivalent bindings are legal");
        let resistor = flattened
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("X1.R1"))
            .expect("subcircuit resistor expands");
        assert!(resistor.nodes[0].eq_ignore_ascii_case("Input"));
        assert!(resistor.nodes[1].eq_ignore_ascii_case("Out"));
    }

    #[test]
    fn repeated_formals_compare_nodes_after_parent_hierarchy_remapping() {
        let netlist = Netlist::parse(
            "parent-remapped duplicate formals\n\
             .SUBCKT OUTER p q\n\
             XINNER p q P DUP\n\
             .ENDS\n\
             .SUBCKT DUP a b A\n\
             R1 a b 1\n\
             .ENDS\n\
             XTOP N1 N2 OUTER\n\
             .END\n",
        )
        .expect("nested duplicate-formal deck parses");

        let flattened =
            flatten_netlist(&netlist).expect("parent-remapped identical nodes are legal");
        let resistor = flattened
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("XTOP.XINNER.R1"))
            .expect("nested resistor expands");
        assert!(resistor.nodes[0].eq_ignore_ascii_case("N1"));
        assert!(resistor.nodes[1].eq_ignore_ascii_case("N2"));
    }

    #[test]
    fn third_repeated_formal_reports_first_binding_and_deterministic_conflict() {
        let error = duplicate_binding_error(
            "third duplicate conflict\n\
             .SUBCKT DUP a A a\n\
             R1 a 0 1\n\
             .ENDS\n\
             X1 SAME same DIFFERENT DUP\n\
             .END\n",
        );

        assert_eq!(error.subcircuit_name, "DUP");
        assert_eq!(error.canonical_subcircuit_name, "DUP");
        assert_eq!(error.instance_name, "X1");
        assert_eq!(error.canonical_instance_name, "X1");
        assert_eq!(error.qualified_instance_name, "X1");
        assert_eq!(error.formal_port, "A");
        assert_eq!(error.first_position, 1);
        assert_eq!(error.conflicting_position, 3);
        assert_eq!(error.first_actual_node, "SAME");
        assert_eq!(error.conflicting_actual_node, "DIFFERENT");
    }

    #[test]
    fn duplicate_binding_validation_precedes_connection_count_validation() {
        let error = duplicate_binding_error(
            "duplicate binding before arity\n\
             .SUBCKT DUP a A\n\
             R1 a 0 1\n\
             .ENDS\n\
             X1 FIRST SECOND EXTRA DUP\n\
             .END\n",
        );
        assert_eq!(error.first_position, 1);
        assert_eq!(error.conflicting_position, 2);
        assert_eq!(error.first_actual_node, "FIRST");
        assert_eq!(error.conflicting_actual_node, "SECOND");
    }

    #[test]
    fn duplicate_binding_validation_precedes_recursion_validation() {
        let error = duplicate_binding_error(
            "duplicate binding before recursion\n\
             .SUBCKT REC a A\n\
             XSELF a internal REC\n\
             .ENDS\n\
             XTOP TOP top REC\n\
             .END\n",
        );
        assert_eq!(error.instance_name, "XSELF");
        assert_eq!(error.qualified_instance_name, "XTOP.XSELF");
        assert_eq!(error.first_actual_node, "TOP");
        assert_eq!(error.conflicting_actual_node, "XTOP.INTERNAL");
    }

    #[test]
    fn global_formal_ports_require_the_same_effective_node_name() {
        let explicit = Netlist::parse(
            "explicit global binding\n\
             .GLOBAL VDD\n\
             .SUBCKT cell VDD p\n\
             R1 VDD p 1\n\
             .ENDS\n\
             X1 OTHER out cell\n\
             .END\n",
        )
        .expect("explicit-global fixture parses");
        let error = flatten_netlist(&explicit).expect_err("global binding mismatch must fail");
        let ParseError::GlobalSubcircuitPortBinding(error) = error else {
            panic!("expected typed global subcircuit-port binding error");
        };
        assert_eq!(error.subcircuit_name, "cell");
        assert_eq!(error.canonical_subcircuit_name, "CELL");
        assert_eq!(error.instance_name, "X1");
        assert_eq!(error.canonical_instance_name, "X1");
        assert_eq!(error.formal_port, "VDD");
        assert_eq!(error.position, 1);
        assert_eq!(error.actual_node, "OTHER");

        let implicit = Netlist::parse(
            "implicit global binding\n\
             .SUBCKT CELL $G_SHARED p\n\
             R1 $G_SHARED p 1\n\
             .ENDS\n\
             X1 local out CELL\n\
             .END\n",
        )
        .expect("implicit-global fixture parses");
        assert!(matches!(
            flatten_netlist(&implicit),
            Err(ParseError::GlobalSubcircuitPortBinding(_))
        ));

        let valid_explicit = Netlist::parse(
            "valid explicit global binding\n\
             .GLOBAL VDD\n\
             .SUBCKT CELL VDD p\n\
             R1 VDD p 1\n\
             .ENDS\n\
             X1 vdd out CELL\n\
             .END\n",
        )
        .expect("valid explicit-global fixture parses");
        flatten_netlist(&valid_explicit).expect("case-equivalent explicit global binding is legal");

        let valid_implicit = Netlist::parse(
            "valid implicit global binding\n\
             .SUBCKT CELL $G_SHARED p\n\
             R1 $G_SHARED p 1\n\
             .ENDS\n\
             X1 $g_shared out CELL\n\
             .END\n",
        )
        .expect("valid implicit-global fixture parses");
        flatten_netlist(&valid_implicit).expect("case-equivalent $G binding is legal");
    }

    #[test]
    fn later_duplicate_global_conflict_precedes_global_name_error() {
        let netlist = Netlist::parse(
            "duplicate global precedence\n\
             .GLOBAL VDD\n\
             .SUBCKT CELL VDD vdd\n\
             R1 VDD 0 1\n\
             .ENDS\n\
             X1 VDD OTHER CELL\n\
             .END\n",
        )
        .expect("duplicate-global fixture parses");
        assert!(matches!(
            flatten_netlist(&netlist),
            Err(ParseError::DuplicateSubcircuitPortBinding(_))
        ));
    }

    #[test]
    fn xyce_duplicate_formal_oracle_decks_report_exact_conflicts() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/xyce/Netlists/Message/Subcircuit");
        for (
            file_name,
            subcircuit_name,
            instance_name,
            formal_port,
            first_position,
            conflicting_position,
            first_actual,
            conflicting_actual,
        ) in [
            (
                "subckt_a2_dup_error.cir",
                "INV1",
                "Xinv1",
                "GND",
                4,
                8,
                "0",
                "VDD",
            ),
            (
                "subckt_j1_dup_error.cir",
                "ONEBIT",
                "X1",
                "6",
                6,
                8,
                "99",
                "1",
            ),
        ] {
            let netlist =
                Netlist::parse_file(&root.join(file_name)).expect("oracle deck must parse");
            let error =
                flatten_netlist(&netlist).expect_err("oracle invocation conflict must fail");
            let ParseError::DuplicateSubcircuitPortBinding(error) = error else {
                panic!("{file_name}: expected typed duplicate binding error");
            };
            assert_eq!(error.subcircuit_name, subcircuit_name, "{file_name}");
            assert_eq!(
                error.canonical_subcircuit_name, subcircuit_name,
                "{file_name}"
            );
            assert_eq!(error.instance_name, instance_name, "{file_name}");
            assert_eq!(
                error.canonical_instance_name,
                instance_name.to_ascii_uppercase(),
                "{file_name}"
            );
            assert_eq!(error.qualified_instance_name, instance_name, "{file_name}");
            assert_eq!(error.formal_port, formal_port, "{file_name}");
            assert_eq!(error.first_position, first_position, "{file_name}");
            assert_eq!(
                error.conflicting_position, conflicting_position,
                "{file_name}"
            );
            assert_eq!(error.first_actual_node, first_actual, "{file_name}");
            assert_eq!(
                error.conflicting_actual_node, conflicting_actual,
                "{file_name}"
            );
            assert!(
                error.to_string().contains(&format!(
                    "Duplicate nodes in .subckt {subcircuit_name} point to different nodes in X line invocation"
                ))
            );
            assert!(error.to_string().contains(&format!(
                "Error invoking subcircuit {subcircuit_name} instance {}",
                instance_name.to_ascii_uppercase()
            )));
        }
    }

    #[test]
    fn xyce_bug784_duplicate_formal_deck_reports_invocation_conflict() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/xyce/Netlists/Certification_Tests/BUG_784/bug_784.cir");
        let netlist = Netlist::parse_file(&path)
            .expect("BUG784 duplicate formal definition remains parser-legal");
        let subcircuit = netlist
            .subcircuits
            .iter()
            .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case("SUBA"))
            .expect("SUBA definition retained");
        assert_eq!(subcircuit.ports, ["B", "B"]);

        let error = flatten_netlist(&netlist).expect_err("BUG784 invocation must fail");
        let ParseError::DuplicateSubcircuitPortBinding(error) = error else {
            panic!("BUG784 must expose the typed duplicate binding error");
        };
        assert_eq!(error.subcircuit_name, "suba");
        assert_eq!(error.canonical_subcircuit_name, "SUBA");
        assert_eq!(error.instance_name, "X1");
        assert_eq!(error.canonical_instance_name, "X1");
        assert_eq!(error.formal_port, "B");
        assert_eq!(error.first_position, 1);
        assert_eq!(error.conflicting_position, 2);
        assert_eq!(error.first_actual_node, "1");
        assert_eq!(error.conflicting_actual_node, "2");
        assert!(error.to_string().contains(".subckt SUBA"));
        assert!(
            error
                .to_string()
                .contains("Error invoking subcircuit SUBA instance X1")
        );
    }

    #[test]
    fn xyce_dollar_g_nodes_remain_global_across_subcircuits() {
        let netlist = Netlist::parse(
            "implicit global node\n\
             X1 a CELL\n\
             X2 b CELL\n\
             .subckt CELL p\n\
             C1 p $G_SHARED 1p\n\
             .ends\n\
             .end\n",
        )
        .expect("deck parses");

        let flattened = flatten_netlist_with_models(&netlist).expect("netlist flattens");
        let global_attachments = flattened
            .elements
            .iter()
            .filter(|element| {
                element
                    .nodes
                    .iter()
                    .any(|node| node.eq_ignore_ascii_case("$G_SHARED"))
            })
            .count();
        assert_eq!(global_attachments, 2);
        assert!(flattened.elements.iter().all(|element| {
            element
                .nodes
                .iter()
                .all(|node| !node.to_ascii_uppercase().contains(".$G_SHARED"))
        }));
    }

    #[test]
    fn sibling_subcircuit_instance_overrides_do_not_consume_shared_defaults() {
        let netlist = Netlist::parse(
            "independent subcircuit parameter scopes\n\
             XTOP in 0 OUTER\n\
             .subckt OUTER a b\n\
             X1 a mid CELL bogus=2 rvalue=1\n\
             X2 mid b CELL bogus=3\n\
             .ends\n\
             .subckt CELL a b\n\
             .param rvalue=10 bogus=1\n\
             R1 a b {rvalue}\n\
             .ends\n\
             V1 in 0 1\n\
             .end\n",
        )
        .expect("deck parses");

        let flattened = flatten_netlist_with_models(&netlist).expect("netlist flattens");
        let mut resistances = flattened
            .elements
            .iter()
            .filter_map(|element| match element.kind {
                ElementKind::Resistor { value, .. } => Some(value),
                _ => None,
            })
            .collect::<Vec<_>>();
        resistances.sort_by(f64::total_cmp);
        assert_eq!(resistances, vec![1.0, 10.0]);
    }

    #[test]
    fn unresolved_subcircuit_values_preserve_typed_parameter_identity() {
        let netlist = Netlist::parse(
            "typed undefined subcircuit parameter\n\
             X1 out 0 CELL\n\
             V1 out 0 1\n\
             .subckt CELL p n\n\
             R1 p n {missing_value}\n\
             .ends\n\
             .end\n",
        )
        .expect("deck parses before hierarchical parameter resolution");

        assert!(matches!(
            flatten_netlist_with_models(&netlist),
            Err(ParseError::UndefinedParameter(name))
                if name.eq_ignore_ascii_case("missing_value")
        ));
    }

    #[test]
    fn unresolved_subcircuit_param_preserves_definition_and_dependency_identity() {
        let netlist = Netlist::parse_with_options(
            "typed unresolved local parameter\n\
             X1 d g 0 0 MYMOS w=0.4u l=3.57u\n\
             V1 d 0 1.8\n\
             V2 g 0 0\n\
             .subckt MYMOS 1 2 3 4 w=0 l=0\n\
             .param foo = '(meh != 1)'\n\
             R1 1 3 1k\n\
             .ends MYMOS\n\
             .end\n",
            super::super::parser::NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("top-level parsing retains the unresolved local expression");
        assert_eq!(
            netlist.subcircuits[0].body_expr_params,
            [("FOO".to_string(), "(meh != 1)".to_string())]
        );

        let error = flatten_netlist_with_models(&netlist)
            .expect_err("every retained local parameter must resolve before expansion");
        let ParseError::UnresolvedSubcircuitParameter(error) = error else {
            panic!("expected typed unresolved subcircuit-parameter error, got {error:?}");
        };
        assert_eq!(error.subcircuit_name, "MYMOS");
        assert_eq!(error.canonical_subcircuit_name, "MYMOS");
        assert_eq!(error.instance_name, "X1");
        assert_eq!(error.canonical_instance_name, "X1");
        assert_eq!(error.qualified_instance_name, "X1");
        assert_eq!(error.parameter_name, "FOO");
        assert_eq!(error.canonical_parameter_name, "FOO");
        assert_eq!(error.expression, "(meh != 1)");
        assert_eq!(error.missing_dependency.as_deref(), Some("MEH"));
        assert!(error.reason.contains("Undefined parameter: MEH"));
        assert!(
            error
                .to_string()
                .contains("Unable to resolve parameter FOO found in .PARAM statement")
        );
    }

    #[test]
    fn runtime_subcircuit_param_does_not_mask_an_undefined_dependency() {
        let parse = |definition: &str| {
            Netlist::parse_with_options(
                &format!(
                    "runtime local parameter\n\
                     X1 in out CELL\n\
                     V1 in 0 1\n\
                     .subckt CELL in out\n\
                     .param foo = '{definition}'\n\
                     R1 in out 1k\n\
                     .ends CELL\n\
                     .end\n"
                ),
                super::super::parser::NetlistParseOptions {
                    expression_dialect: ExpressionDialect::Xyce,
                    ..Default::default()
                },
            )
            .expect("runtime local parameter parses")
        };

        let error = flatten_netlist_with_models(&parse("TIME + MEH"))
            .expect_err("runtime quantity must not mask an undefined parameter");
        let ParseError::UnresolvedSubcircuitParameter(error) = error else {
            panic!("expected typed unresolved subcircuit-parameter error, got {error:?}");
        };
        assert_eq!(error.canonical_parameter_name, "FOO");
        assert_eq!(error.missing_dependency.as_deref(), Some("MEH"));

        flatten_netlist_with_models(&parse("TIME + 1"))
            .expect("a fully bound runtime parameter remains deferred");

        let error = flatten_netlist_with_models(&parse("BOGUS(TIME)"))
            .expect_err("an unknown function must not be retained as a runtime expression");
        let ParseError::UnresolvedSubcircuitParameter(error) = error else {
            panic!("expected typed unresolved subcircuit-parameter error, got {error:?}");
        };
        assert_eq!(error.canonical_parameter_name, "FOO");
        assert!(error.missing_dependency.is_none());
        assert!(error.reason.contains("Unknown function"));
    }

    #[test]
    fn runtime_instance_param_does_not_mask_invalid_dependencies_or_functions() {
        let parse = |expression: &str| {
            Netlist::parse_with_options(
                &format!(
                    "runtime instance parameter\n\
                     X1 in out CELL p='{expression}'\n\
                     V1 in 0 1\n\
                     .subckt CELL in out p=0\n\
                     R1 in out 1k\n\
                     .ends CELL\n\
                     .end\n"
                ),
                super::super::parser::NetlistParseOptions {
                    expression_dialect: ExpressionDialect::Xyce,
                    ..Default::default()
                },
            )
            .expect("runtime instance parameter parses")
        };

        let missing = flatten_netlist_with_models(&parse("TIME + MEH"))
            .expect_err("runtime quantity must not mask an undefined instance dependency");
        assert!(matches!(missing, ParseError::UndefinedParameter(ref name) if name == "MEH"));

        flatten_netlist_with_models(&parse("TIME + 1"))
            .expect("a fully bound runtime instance parameter remains deferred");

        let unknown = flatten_netlist_with_models(&parse("BOGUS(TIME)"))
            .expect_err("an unknown instance function must not be retained at runtime");
        assert!(
            matches!(unknown, ParseError::InvalidValue(ref reason) if reason.contains("Unknown function"))
        );
    }

    #[test]
    fn deferred_subcircuit_parameter_resolution_is_cooperatively_abortable() {
        let netlist = Netlist::parse_with_options(
            "abort deferred local parameter\n\
             X1 in out CELL\n\
             V1 in 0 1\n\
             .subckt CELL in out\n\
             .param foo='TIME + 1'\n\
             R1 in out 1k\n\
             .ends CELL\n\
             .end\n",
            super::super::parser::NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("abortable runtime parameter deck parses");
        let abort = CountingAbort::new(4);
        let error = build_subcircuit_param_scope(
            &netlist.subcircuits[0],
            "X1",
            "X1",
            &netlist.params,
            &[],
            &RandomState::default(),
            &abort,
        )
        .expect_err("deferred local-parameter resolution must poll cancellation");
        assert!(matches!(error, ParseWithAbortError::Aborted));
        assert!(abort.count() >= 5);
    }

    #[test]
    fn public_abort_api_polls_inside_body_and_instance_parameter_worklists() {
        fn completed_abort_checks(netlist: &Netlist) -> usize {
            let counter = CountingAbort::new(usize::MAX);
            flatten_netlist_with_models_with_abort(netlist, &counter)
                .expect("non-cancelling count run flattens");
            counter.count()
        }

        fn parse(source: &str) -> Netlist {
            Netlist::parse_with_options(
                source,
                super::super::parser::NetlistParseOptions {
                    expression_dialect: ExpressionDialect::Xyce,
                    ..Default::default()
                },
            )
            .expect("abort worklist fixture parses")
        }

        let baseline = parse(
            "abort worklist baseline\n\
             X1 in out CELL\n\
             V1 in 0 1\n\
             .subckt CELL in out\n\
             R1 in out 1k\n\
             .ends CELL\n\
             .end\n",
        );
        let baseline_checks = completed_abort_checks(&baseline);

        let mut body_source =
            String::from("abort body worklist\nX1 in out CELL\nV1 in 0 1\n.subckt CELL in out\n");
        for index in 0..256 {
            body_source.push_str(&format!(".param p{index}='TIME+{index}'\n"));
        }
        body_source.push_str("R1 in out 1k\n.ends CELL\n.end\n");
        let body = parse(&body_source);
        let error =
            flatten_netlist_with_models_with_abort(&body, &CountingAbort::new(baseline_checks))
                .expect_err("body-expression worklist must add observable abort polls");
        assert!(matches!(error, ParseWithAbortError::Aborted));

        let formals = (0..256)
            .map(|index| format!(" p{index}=0"))
            .collect::<String>();
        let overrides = (0..256)
            .map(|index| format!(" p{index}='TIME+{index}'"))
            .collect::<String>();
        let instance_baseline = parse(&format!(
            "abort instance baseline\nX1 in out CELL\nV1 in 0 1\n.subckt CELL in out{formals}\nR1 in out 1k\n.ends CELL\n.end\n"
        ));
        let instance_baseline_checks = completed_abort_checks(&instance_baseline);
        let instance = parse(&format!(
            "abort instance worklist\nX1 in out CELL{overrides}\nV1 in 0 1\n.subckt CELL in out{formals}\nR1 in out 1k\n.ends CELL\n.end\n"
        ));
        let error = flatten_netlist_with_models_with_abort(
            &instance,
            &CountingAbort::new(instance_baseline_checks),
        )
        .expect_err("instance-expression worklist must add observable abort polls");
        assert!(matches!(error, ParseWithAbortError::Aborted));
    }

    #[test]
    fn behavioral_voltage_probe_remaps_subcircuit_port_case_insensitively() {
        let netlist = Netlist::parse(
            "\
behavioral port remap
X1 5 ABM_SUB
.SUBCKT ABM_SUB IN_1
E1 out 0 VALUE={V(in_1)}
R1 out 0 1k
.ENDS
.END
",
        )
        .expect("deck parses");

        let flattened = flatten_netlist_with_models(&netlist).expect("netlist flattens");
        let expression = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::BehavioralVoltage { expression, .. }
                    if element.name.eq_ignore_ascii_case("X1.E1") =>
                {
                    Some(expression.as_str())
                }
                _ => None,
            })
            .expect("flattened behavioral source exists");

        assert!(
            expression.contains("V(5)"),
            "subcircuit port probe should map to instance connection, got {expression}"
        );
        assert!(
            !expression.to_ascii_lowercase().contains("x1.in_1"),
            "subcircuit port probe must not be prefixed as an internal node: {expression}"
        );
    }

    #[test]
    fn top_level_dynamic_capacitor_value_expression_is_preserved() {
        let netlist = Netlist::parse(
            "top-level solution-dependent capacitor\n\
             VCTRL ctrl 0 2.0\n\
             C1 out 0 C={1p+V(ctrl)}\n\
             .end\n",
        )
        .expect("top-level dynamic capacitor deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("top-level dynamic capacitor deck flattens");
        let (value, value_expr) = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Capacitor {
                    value, value_expr, ..
                } if element.name.eq_ignore_ascii_case("C1") => Some((*value, value_expr)),
                _ => None,
            })
            .expect("flattened top-level capacitor exists");

        assert!(
            value.is_nan(),
            "runtime-dependent capacitor values stay unresolved until runtime"
        );
        let expression = value_expr
            .as_deref()
            .expect("runtime-dependent capacitor expression is preserved");
        assert!(
            expression.to_ascii_lowercase().contains("v(ctrl)"),
            "top-level capacitor probe should remain addressable, got {expression}"
        );
    }

    #[test]
    fn subckt_dynamic_capacitor_value_expression_remaps_voltage_probe() {
        let netlist = Netlist::parse(
            "subckt solution dependent capacitor\n\
             X1 out 0 soldepcap\n\
             .subckt soldepcap p n\n\
             Vcontrol cntl n 2.0\n\
             C1 p n C={1p+V(cntl)}\n\
             .ends\n\
             .end\n",
        )
        .expect("solution-dependent capacitor subcircuit parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("solution-dependent capacitor subcircuit flattens");
        let (value, value_expr) = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Capacitor {
                    value, value_expr, ..
                } if element.name.eq_ignore_ascii_case("X1.C1") => Some((*value, value_expr)),
                _ => None,
            })
            .expect("flattened subcircuit capacitor exists");

        assert!(
            value.is_nan(),
            "runtime-dependent capacitor values stay unresolved until runtime"
        );
        let expression = value_expr
            .as_deref()
            .expect("runtime-dependent capacitor expression is preserved");
        assert!(
            expression.to_ascii_lowercase().contains("v(x1.cntl)"),
            "subcircuit-local capacitor probe should follow flattened hierarchy, got {expression}"
        );
    }

    #[test]
    fn subcircuit_nodeset_uses_instance_parameter_scope_and_remaps_internal_node() {
        let scoped = Netlist::parse(
            "\
scoped nodeset
X_X1 in out NODESET_Subckt params: vmid=0.5
.SUBCKT NODESET_Subckt in out params: vmid=5.0
R1 in mid 10
C1 mid out 1u
.NODESET V(mid)={vmid}
.ENDS
.END
",
        )
        .expect("scoped NODESET deck parses");
        let subcircuit = scoped
            .subcircuits
            .iter()
            .find(|subcircuit| subcircuit.name.eq_ignore_ascii_case("NODESET_Subckt"))
            .expect("subcircuit exists");
        assert_eq!(subcircuit.node_sets.len(), 1);
        assert!(subcircuit.node_sets[0].node.eq_ignore_ascii_case("mid"));
        assert!(
            subcircuit.node_sets[0].voltage_expr.is_some(),
            "subcircuit-scoped NODESET must remain deferred for instance overrides"
        );

        let flattened = flatten_netlist_with_models(&scoped).expect("scoped NODESET flattens");
        assert_eq!(flattened.scoped_node_sets.len(), 1);
        assert!(
            flattened.scoped_node_sets[0]
                .node
                .eq_ignore_ascii_case("X_X1.mid")
        );
        assert_eq!(
            flattened.scoped_node_sets[0].voltage.to_bits(),
            0.5f64.to_bits()
        );
        assert!(flattened.scoped_node_sets[0].voltage_expr.is_none());

        let explicit = Netlist::parse(
            "\
explicit hierarchical nodeset
X_X1 in out NODESET_Subckt
.SUBCKT NODESET_Subckt in out
R1 in mid 10
C1 mid out 1u
.ENDS
.NODESET V(X_X1:mid)=0.5
.END
",
        )
        .expect("explicit hierarchical NODESET deck parses");
        assert_eq!(explicit.node_sets.len(), 1);
        assert!(explicit.node_sets[0].node.eq_ignore_ascii_case("X_X1:mid"));
        assert_eq!(explicit.node_sets[0].voltage.to_bits(), 0.5f64.to_bits());
        assert!(explicit.node_sets[0].voltage_expr.is_none());
        assert!(
            flatten_netlist_with_models(&explicit)
                .expect("explicit NODESET deck flattens")
                .scoped_node_sets
                .is_empty()
        );
    }

    #[test]
    fn nested_abort_is_distinct_and_the_same_flattener_retries_cleanly() {
        let mut source = String::from("abortable hierarchy\nXTOP 1 0 S0\n");
        for depth in 0..8 {
            source.push_str(&format!(
                ".SUBCKT S{depth} a b\nX{depth} a b S{}\n.ENDS\n",
                depth + 1
            ));
        }
        source.push_str(".SUBCKT S8 a b\nR1 a b 1\n.ENDS\n.END\n");
        let netlist = Netlist::parse(&source).expect("nested abort deck parses");
        let mut flattener = Flattener::new(&netlist.subcircuits);

        let error = flattener
            .flatten_with_abort(&netlist, &CountingAbort::new(4))
            .expect_err("nested expansion must observe cancellation");
        assert!(matches!(error, ParseWithAbortError::Aborted));

        let elements = flattener
            .flatten(&netlist)
            .expect("retry clears partial recursion state");
        assert_eq!(elements.len(), 1);
        assert_eq!(elements[0].name, "XTOP.X0.X1.X2.X3.X4.X5.X6.X7.R1");
    }
}
