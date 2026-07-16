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

#![allow(clippy::too_many_arguments)]
use super::expr::{
    behavioral_expression_references_runtime_quantity, prepare_behavioral_expression,
};
use super::hierarchy_path::HierarchyPath;
use super::param_scope::ParamResolver;
use super::parser::parse_source_spec_text;
use super::{
    DeviceInitialConditionDirective, DeviceInitialConditionError, DeviceInitialConditionSource,
    DuplicateSubcircuitPortBindingError, Element, ElementKind, GlobalSubcircuitPortBindingError,
    InitialCondition, ModelDef, Netlist, NodeSet, ParamContext, ParameterRedefinitionPolicy,
    ParametricValue, ParseError, RandomState, SourceSpec, SubcircuitDef,
};
use crate::Value;
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
            preserve_hierarchy: true,
            hierarchy_separator: '.',
            collect_metadata: true,
        }
    }

    /// Create a config optimized for performance (shorter names)
    pub fn production() -> Self {
        Self {
            max_depth: 100,
            preserve_hierarchy: false,
            hierarchy_separator: '_',
            collect_metadata: false,
        }
    }

    /// Create a standard config
    pub fn spectre() -> Self {
        Self {
            max_depth: 256,
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
    /// Global nodes that must not be renamed while flattening hierarchy.
    global_nodes: HashSet<String>,
    /// Definitions currently being expanded, outermost first. A definition
    /// re-entered while still on this stack is a recursive instantiation,
    /// reported with the full cycle instead of running into `max_depth`.
    expansion_stack: Vec<String>,
    /// Netlist-wide statistical-function stream (shared draw counter), so
    /// per-instance expression draws are distinct yet reproducible.
    random: RandomState,
    /// Model cards cloned while flattening parameterized subcircuit instances.
    scoped_models: Vec<ModelDef>,
    /// Startup directives scoped while flattening subcircuit instances.
    scoped_initial_conditions: Vec<InitialCondition>,
    scoped_node_sets: Vec<NodeSet>,
    /// Digital XSPICE nodes with the effective scope-local VCC value.
    xspice_auto_bridge_node_hints: Vec<XspiceAutoBridgeNodeHint>,
    /// Scope parameter used for digital auto-bridge voltage levels.
    xspice_auto_bridge_digital_param_name: String,
    /// Directory of the parsed deck, used to resolve scoped XSPICE file params.
    source_base_dir: Option<PathBuf>,
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
        let subcircuit_map: HashMap<String, &SubcircuitDef> = subcircuits
            .iter()
            .map(|s| (s.name.to_ascii_uppercase(), s))
            .collect();

        // Initialize param resolver with subcircuit defaults
        let mut param_resolver = ParamResolver::new();
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
            global_nodes: HashSet::new(),
            expansion_stack: Vec::new(),
            random: RandomState::default(),
            scoped_models: Vec::new(),
            scoped_initial_conditions: Vec::new(),
            scoped_node_sets: Vec::new(),
            xspice_auto_bridge_node_hints: Vec::new(),
            xspice_auto_bridge_digital_param_name: "vcc".to_string(),
            source_base_dir: None,
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

    /// Flatten a netlist, expanding all subcircuit instances
    pub fn flatten(&mut self, netlist: &Netlist) -> Result<Vec<Element>, ParseError> {
        let mut flat_elements = Vec::new();
        self.external_subckts = Self::collect_external_subckts(netlist);
        self.global_nodes = netlist.global_nodes.clone();
        self.expansion_stack.clear();
        self.scoped_models.clear();
        self.scoped_initial_conditions.clear();
        self.scoped_node_sets.clear();
        self.xspice_auto_bridge_node_hints.clear();
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

        for element in &netlist.elements {
            self.flatten_element(
                element,
                "",
                &HashMap::new(),
                &global_scope,
                0,
                &mut flat_elements,
            )?;
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
        prefix: &str,
        node_map: &HashMap<String, String>,
        scope: &ParamContext,
        depth: usize,
        output: &mut Vec<Element>,
    ) -> Result<(), ParseError> {
        if depth > self.config.max_depth {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "Subcircuit recursion depth exceeded (max {})",
                    self.config.max_depth
                ),
            });
        }

        match &element.kind {
            ElementKind::Subcircuit {
                subckt_name,
                params,
            } => {
                if self.find_subcircuit(subckt_name).is_some() {
                    self.expand_subcircuit(
                        element,
                        subckt_name,
                        params,
                        prefix,
                        node_map,
                        scope,
                        depth,
                        output,
                    )?;
                } else if self.is_external_subckt(subckt_name) {
                    // Preserve external instance (e.g. Verilog-A model) as a leaf.
                    let new_element = self.resolve_external_subcircuit_params(
                        self.remap_element(element, prefix, node_map),
                        scope,
                    )?;
                    output.push(new_element);
                } else {
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!("Undefined subcircuit: {}", subckt_name),
                    });
                }
            }
            _ => {
                // Regular element - remap nodes and add to output
                let new_element = self.remap_element(element, prefix, node_map);
                self.record_xspice_auto_bridge_node_hints(&new_element, scope, depth);
                output.push(new_element);
            }
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
            if !is_ground_node_name(node) {
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

    fn collect_external_subckts(netlist: &Netlist) -> HashSet<String> {
        let mut names = HashSet::new();
        for include in &netlist.veriloga_includes {
            if let Some(model_name) = &include.model_name {
                names.insert(model_name.to_ascii_uppercase());
            }
            if let Some(stem) = include.file_path.file_stem().and_then(|s| s.to_str()) {
                names.insert(stem.to_ascii_uppercase());
            }
        }
        #[cfg(feature = "veriloga-builtins")]
        {
            for name in crate::device::veriloga_generated::builtins::builtin_names() {
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
        prefix: &str,
        parent_node_map: &HashMap<String, String>,
        caller_scope: &ParamContext,
        depth: usize,
        output: &mut Vec<Element>,
    ) -> Result<(), ParseError> {
        // Look up subcircuit definition
        let subckt = self
            .find_subcircuit(subckt_name)
            .ok_or_else(|| ParseError::Syntax {
                line: 0,
                message: format!("Undefined subcircuit: {}", subckt_name),
            })?;

        // Build new prefix for this instance
        let new_prefix = if prefix.is_empty() {
            instance.name.clone()
        } else {
            format!("{}.{}", prefix, instance.name)
        };

        // Resolve every actual through the parent context before validating
        // repeated formal ports. Duplicate formals are legal only when every
        // occurrence maps to the same effective node for this invocation.
        let mapped_ports = subckt
            .ports
            .iter()
            .zip(&instance.nodes)
            .map(|(port, actual)| (port, self.remap_node(actual, prefix, parent_node_map)))
            .collect::<Vec<_>>();
        let mut first_bindings = HashMap::<String, (usize, &str)>::new();
        for (index, (formal, actual)) in mapped_ports.iter().enumerate() {
            let canonical_formal = formal.to_ascii_uppercase();
            if let Some((first_index, first_actual)) = first_bindings.get(&canonical_formal) {
                if !first_actual.eq_ignore_ascii_case(actual) {
                    return Err(ParseError::DuplicateSubcircuitPortBinding(Box::new(
                        DuplicateSubcircuitPortBindingError {
                            subcircuit_name: subckt.name.clone(),
                            canonical_subcircuit_name: subckt.name.to_ascii_uppercase(),
                            instance_name: instance.name.clone(),
                            canonical_instance_name: instance.name.to_ascii_uppercase(),
                            qualified_instance_name: new_prefix.clone(),
                            formal_port: formal.to_string(),
                            first_position: first_index + 1,
                            conflicting_position: index + 1,
                            first_actual_node: (*first_actual).to_string(),
                            conflicting_actual_node: actual.clone(),
                        },
                    )));
                }
            } else {
                let formal_is_global = self.global_nodes.contains(&canonical_formal)
                    || formal
                        .get(..2)
                        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("$G"));
                if formal_is_global && !formal.eq_ignore_ascii_case(actual) {
                    return Err(ParseError::GlobalSubcircuitPortBinding(Box::new(
                        GlobalSubcircuitPortBindingError {
                            subcircuit_name: subckt.name.clone(),
                            canonical_subcircuit_name: subckt.name.to_ascii_uppercase(),
                            instance_name: instance.name.clone(),
                            canonical_instance_name: instance.name.to_ascii_uppercase(),
                            qualified_instance_name: new_prefix.clone(),
                            formal_port: formal.to_string(),
                            position: index + 1,
                            actual_node: actual.clone(),
                        },
                    )));
                }
                first_bindings.insert(canonical_formal, (index, actual.as_str()));
            }
        }

        // Xyce validates every available duplicate/global binding before
        // connection count and recursion.
        if instance.nodes.len() != subckt.ports.len() {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "Subcircuit instance '{}' connects {} node(s) but '{}' declares {} port(s): {}",
                    new_prefix,
                    instance.nodes.len(),
                    subckt_name,
                    subckt.ports.len(),
                    subckt.ports.join(" ")
                ),
            });
        }

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
            });
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

        let param_scope =
            build_subcircuit_param_scope(subckt, caller_scope, instance_params, &self.random)?;

        // X-line multiplicity: `M=` on a subcircuit instance multiplies the
        // parallel multiplicity of every device it expands to (HSPICE/ngspice
        // semantics), composing multiplicatively through nested hierarchy.
        // A subcircuit that declares its own formal `M` parameter keeps
        // ordinary parameter behavior instead — the author owns the name.
        let formal_declares_m = subckt
            .params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("M"));
        let mut multiplicity = 1.0;
        if !formal_declares_m {
            for (name, value) in instance_params {
                if name.eq_ignore_ascii_case("M") {
                    let resolved = resolve_parametric_value(value, caller_scope, &self.random)?;
                    if !resolved.is_finite() || resolved <= 0.0 {
                        return Err(ParseError::Syntax {
                            line: 0,
                            message: format!(
                                "Subcircuit instance '{}' has invalid multiplicity M={}",
                                instance.name, resolved
                            ),
                        });
                    }
                    multiplicity = resolved;
                }
            }
        }

        // Expand each element in the subcircuit
        self.expansion_stack.push(subckt_name.to_owned());
        for sub_element in &subckt.elements {
            // Apply parameter substitution to element values
            let element_path = if new_prefix.is_empty() {
                sub_element.name.clone()
            } else {
                format!("{}.{}", new_prefix, sub_element.name)
            };
            let mut substituted =
                self.substitute_params(sub_element, &param_scope, &element_path, &new_prefix)?;
            if multiplicity != 1.0 {
                apply_element_multiplicity(&mut substituted, multiplicity);
            }
            self.flatten_element(
                &substituted,
                &new_prefix,
                &node_map,
                &param_scope,
                depth + 1,
                output,
            )?;
        }
        self.expansion_stack.pop();
        self.collect_scoped_startup_directives(subckt, &new_prefix, &node_map, &param_scope)?;

        Ok(())
    }

    fn collect_scoped_startup_directives(
        &mut self,
        subckt: &SubcircuitDef,
        prefix: &str,
        node_map: &HashMap<String, String>,
        scope: &ParamContext,
    ) -> Result<(), ParseError> {
        for ic in &subckt.initial_conditions {
            self.scoped_initial_conditions.push(InitialCondition {
                node: self.remap_node(&ic.node, prefix, node_map),
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

        for nodeset in &subckt.node_sets {
            self.scoped_node_sets.push(NodeSet {
                node: self.remap_node(&nodeset.node, prefix, node_map),
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
        let Some(expr) = voltage_expr else {
            return Ok(voltage);
        };
        resolve_parametric_value(
            &ParametricValue::Expression(expr.to_string()),
            scope,
            &self.random,
        )
        .map_err(|err| {
            ParseError::InvalidValue(format!(
                "{directive} for node '{node}' could not resolve expression '{expr}': {err}"
            ))
        })
    }

    /// Remap an element's nodes using the current prefix and node map
    /// Also remaps CCCS/CCVS control element names
    fn remap_element(
        &self,
        element: &Element,
        prefix: &str,
        node_map: &HashMap<String, String>,
    ) -> Element {
        let new_name = if prefix.is_empty() {
            element.name.clone()
        } else {
            format!("{}.{}", prefix, element.name)
        };

        let new_nodes: Vec<String> = element
            .nodes
            .iter()
            .map(|n| self.remap_node(n, prefix, node_map))
            .collect();

        // Remap the element kind, handling CCCS/CCVS control element names
        let new_kind = match &element.kind {
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
            ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
            } => ElementKind::BehavioralVoltage {
                expression: self.remap_behavioral_expression(expression, prefix, node_map),
                tc1: *tc1,
                tc2: *tc2,
            },
            ElementKind::BehavioralCurrent {
                expression,
                tc1,
                tc2,
            } => ElementKind::BehavioralCurrent {
                expression: self.remap_behavioral_expression(expression, prefix, node_map),
                tc1: *tc1,
                tc2: *tc2,
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
                control_nodes,
            } => ElementKind::Vccs {
                transconductance: *transconductance,
                transconductance_expr: transconductance_expr.clone(),
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
                let new_ctrl = Self::remap_local_element_reference(control_element, prefix);
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
                let new_ctrl = Self::remap_local_element_reference(control_element, prefix);
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
                control_element: Self::remap_local_element_reference(control_element, prefix),
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
            } => ElementKind::Coupling {
                inductors: inductors
                    .iter()
                    .map(|name| Self::remap_local_element_reference(name, prefix))
                    .collect(),
                coefficient: *coefficient,
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
                model: model.clone(),
                pspice_u_timing: pspice_u_timing.clone(),
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
            },
            // All other kinds - clone as-is
            other => other.clone(),
        };

        Element {
            name: new_name,
            kind: new_kind,
            nodes: new_nodes,
        }
    }

    fn remap_local_element_reference(name: &str, prefix: &str) -> String {
        if prefix.is_empty() {
            name.to_string()
        } else {
            format!("{}.{}", prefix, name)
        }
    }

    /// Remap a single node name
    fn remap_node(&self, node: &str, prefix: &str, node_map: &HashMap<String, String>) -> String {
        // Ground is never renamed
        if node == "0" || node.eq_ignore_ascii_case("gnd") {
            return "0".to_string();
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
            node.to_string()
        } else {
            format!("{}{}{}", prefix, self.config.hierarchy_separator, node)
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
                        remap_current_probe_arg(prefix, &inner)
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

    /// Substitute parameters in element values
    ///
    /// Replaces parameter references in element values with values from param_map.
    /// Parameter values can be:
    /// - Direct numeric values (already resolved, no substitution needed)
    /// - Referenced as {PARAM_NAME} in future expression support
    ///
    /// Since our current AST stores resolved f64 values, we substitute
    /// by scaling/replacing values based on parameter lookups.
    fn substitute_params(
        &mut self,
        element: &Element,
        scope: &ParamContext,
        element_path: &str,
        model_scope_path: &str,
    ) -> Result<Element, ParseError> {
        let new_kind = match &element.kind {
            // Passive components
            ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } => {
                let (value, value_expr) =
                    self.resolve_passive_value_expr(*value, value_expr, scope, element_path)?;
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
            } => ElementKind::Capacitor {
                value: self.resolve_optional_value_expr(*value, value_expr, scope)?,
                value_expr: None,
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
            },
            ElementKind::Inductor {
                value,
                value_expr,
                initial_current,
                model,
                instance_params,
                deferred_params,
            } => ElementKind::Inductor {
                value: self.resolve_optional_value_expr(*value, value_expr, scope)?,
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

            // Nested subcircuit - propagate parameters
            ElementKind::Subcircuit {
                subckt_name,
                params: instance_params,
            } => {
                let mut merged_params = Vec::with_capacity(instance_params.len());
                for (name, value) in instance_params {
                    let resolved = if parametric_value_is_string(value) {
                        ParametricValue::String(resolve_string_parametric_value(value, scope)?)
                    } else {
                        ParametricValue::Resolved(resolve_parametric_value(
                            value,
                            scope,
                            &self.random,
                        )?)
                    };
                    merged_params.push((name.clone(), resolved));
                }

                ElementKind::Subcircuit {
                    subckt_name: subckt_name.clone(),
                    params: merged_params,
                }
            }

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

            ElementKind::VoltageSourceDeferred(raw_spec) => {
                self.resolve_deferred_source_kind(raw_spec, scope, element_path, true)?
            }
            ElementKind::CurrentSourceDeferred(raw_spec) => {
                self.resolve_deferred_source_kind(raw_spec, scope, element_path, false)?
            }

            ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
            } => ElementKind::BehavioralVoltage {
                expression: self.prepare_scoped_behavioral_expression(
                    expression,
                    scope,
                    element_path,
                )?,
                tc1: *tc1,
                tc2: *tc2,
            },
            ElementKind::BehavioralCurrent {
                expression,
                tc1,
                tc2,
            } => ElementKind::BehavioralCurrent {
                expression: self.prepare_scoped_behavioral_expression(
                    expression,
                    scope,
                    element_path,
                )?,
                tc1: *tc1,
                tc2: *tc2,
            },

            // Controlled sources
            ElementKind::Vcvs {
                gain,
                gain_expr,
                control_nodes,
            } => ElementKind::Vcvs {
                gain: self.resolve_optional_value_expr(*gain, gain_expr, scope)?,
                gain_expr: None,
                control_nodes: control_nodes.clone(),
            },
            ElementKind::Vccs {
                transconductance,
                transconductance_expr,
                control_nodes,
            } => ElementKind::Vccs {
                transconductance: self.resolve_optional_value_expr(
                    *transconductance,
                    transconductance_expr,
                    scope,
                )?,
                transconductance_expr: None,
                control_nodes: control_nodes.clone(),
            },
            ElementKind::Cccs {
                gain,
                gain_expr,
                control_element,
            } => ElementKind::Cccs {
                gain: self.resolve_optional_value_expr(*gain, gain_expr, scope)?,
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
                control_expression: control_expression.clone(),
                initial_state: *initial_state,
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

        Ok(Element {
            name: element.name.clone(),
            kind: new_kind,
            nodes: element.nodes.clone(),
        })
    }

    /// Resolve a deferred value expression, or keep the parse-time value.
    fn resolve_optional_value_expr(
        &self,
        value: Value,
        value_expr: &Option<String>,
        scope: &ParamContext,
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
                resolve_parametric_value(
                    &ParametricValue::Expression(prepared),
                    scope,
                    &self.random,
                )
            }
            None => Ok(value),
        }
    }

    fn resolve_passive_value_expr(
        &self,
        value: Value,
        value_expr: &Option<String>,
        scope: &ParamContext,
        element_path: &str,
    ) -> Result<(Value, Option<String>), ParseError> {
        match value_expr {
            Some(expr) => {
                let prepared =
                    self.prepare_scoped_behavioral_expression(expr, scope, element_path)?;
                if behavioral_expression_references_runtime_quantity(&prepared) {
                    Ok((Value::NAN, Some(prepared)))
                } else {
                    Ok((
                        resolve_parametric_value(
                            &ParametricValue::Expression(prepared),
                            scope,
                            &self.random,
                        )?,
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
            let value = resolve_parametric_value(
                &ParametricValue::Expression(expr.clone()),
                scope,
                &self.random,
            )?;
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
    ) -> Result<ElementKind, ParseError> {
        match parse_source_spec_text(raw_spec, 0, scope) {
            Ok(spec) if voltage_source => Ok(ElementKind::VoltageSource(spec)),
            Ok(spec) => Ok(ElementKind::CurrentSource(spec)),
            Err(source_error) => {
                let trimmed = raw_spec.trim();
                let expression = trimmed
                    .strip_prefix('{')
                    .and_then(|inner| inner.strip_suffix('}'))
                    .map(str::trim)
                    .filter(|inner| !inner.is_empty())
                    .ok_or_else(|| {
                        ParseError::InvalidValue(format!(
                            "source specification for element '{}' could not be resolved: {}",
                            element_path, source_error
                        ))
                    })?;
                let expression =
                    self.prepare_scoped_behavioral_expression(expression, scope, element_path)?;
                if voltage_source {
                    Ok(ElementKind::BehavioralVoltage {
                        expression,
                        tc1: 0.0,
                        tc2: 0.0,
                    })
                } else {
                    Ok(ElementKind::BehavioralCurrent {
                        expression,
                        tc1: 0.0,
                        tc2: 0.0,
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
        let Some(model_def) = self
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(model_name))
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
        Ok(scoped_name)
    }
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

fn apply_device_initial_condition_entry(
    element: &mut Element,
    entry: &super::DeviceInitialConditionEntry,
) -> Result<(), ParseError> {
    match &mut element.kind {
        ElementKind::Capacitor {
            initial_voltage, ..
        } => {
            require_device_initial_condition_arity(entry, "exactly 1 value", 1, 1)?;
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

fn resolve_parametric_value(
    value: &ParametricValue,
    scope: &ParamContext,
    random: &RandomState,
) -> Result<Value, ParseError> {
    match value {
        ParametricValue::Resolved(v) => Ok(*v),
        ParametricValue::Expression(expr) => {
            let mut ctx = scope.clone();
            // Join the netlist-wide stream: each instance expression that
            // calls gauss/agauss/unif/aunif/limit advances one shared,
            // reproducible sequence instead of replaying the same draws.
            ctx.adopt_random(random);
            super::expr::eval_expression(expr, &ctx).map_err(|error| match error {
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
        ParametricValue::Resolved(_) => Err(ParseError::InvalidValue(format!(
            "numeric parameter value cannot be used as a string value"
        ))),
    }
}

fn build_subcircuit_param_scope(
    subckt: &SubcircuitDef,
    caller_scope: &ParamContext,
    instance_params: &[(String, ParametricValue)],
    random: &RandomState,
) -> Result<ParamContext, ParseError> {
    let (instance_numeric, instance_strings) =
        resolve_subcircuit_instance_params(subckt, caller_scope, instance_params, random)?;
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
    for (name, value) in instance_numeric {
        scope.set(&name, value);
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
    resolve_deferred_param_expressions(&formal_expr_params, &mut scope, random, &instance_names)?;
    resolve_deferred_param_expressions(&body_expr_params, &mut scope, random, &instance_names)?;

    Ok(scope)
}

fn resolve_deferred_param_expressions(
    expr_params: &[(String, String)],
    scope: &mut ParamContext,
    random: &RandomState,
    skip_names: &[String],
) -> Result<(), ParseError> {
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
        let mut progress = false;
        let mut unresolved = Vec::new();
        let mut first_error = None;

        for (name, expr) in pending {
            match resolve_parametric_value(
                &ParametricValue::Expression(expr.clone()),
                scope,
                random,
            ) {
                Ok(value) => {
                    scope.set(&name, value);
                    progress = true;
                }
                Err(err) => {
                    first_error.get_or_insert(err);
                    unresolved.push((name, expr));
                }
            }
        }

        if !progress {
            return Err(first_error.unwrap_or_else(|| {
                ParseError::InvalidValue(
                    "subcircuit deferred parameters could not be resolved".to_string(),
                )
            }));
        }
        pending = unresolved;
    }

    Ok(())
}

fn resolve_subcircuit_instance_params(
    subckt: &SubcircuitDef,
    caller_scope: &ParamContext,
    instance_params: &[(String, ParametricValue)],
    random: &RandomState,
) -> Result<(Vec<(String, Value)>, Vec<(String, String)>), ParseError> {
    let mut instance_scope = caller_scope.clone();
    instance_scope.adopt_random(random);
    let mut pending = instance_params.to_vec();
    let mut numeric = Vec::<(String, Value)>::new();
    let mut strings = Vec::<(String, String)>::new();

    while !pending.is_empty() {
        let mut progress = false;
        let mut unresolved = Vec::new();
        let mut first_error = None;

        for (name, value) in pending {
            if subcircuit_instance_param_is_string(subckt, &name, &value) {
                match resolve_string_parametric_value(&value, &instance_scope) {
                    Ok(resolved) => {
                        instance_scope.set_string(&name, resolved.clone());
                        upsert_string_param_value(&mut strings, name, resolved);
                        progress = true;
                    }
                    Err(err) => {
                        first_error.get_or_insert(err);
                        unresolved.push((name, value));
                    }
                }
            } else {
                match resolve_parametric_value(&value, &instance_scope, random) {
                    Ok(resolved) => {
                        instance_scope.set(&name, resolved);
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
            return Err(first_error.unwrap_or_else(|| {
                ParseError::InvalidValue(
                    "subcircuit instance parameters could not be resolved".to_string(),
                )
            }));
        }
        pending = unresolved;
    }

    Ok((numeric, strings))
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

fn upsert_numeric_param_value(items: &mut Vec<(String, Value)>, name: String, value: Value) {
    if let Some((_, existing_value)) = items
        .iter_mut()
        .find(|(existing, _)| existing.eq_ignore_ascii_case(&name))
    {
        *existing_value = value;
    } else {
        items.push((name, value));
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
        match super::lexer::parse_spice_value(field) {
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
        SourceSpec::TrNoise { na, namp, .. } => {
            // Deterministic multiplicity convention: parallel copies share
            // one sample train, so amplitudes scale linearly. Physically
            // uncorrelated devices would scale as sqrt(m); model that by
            // adjusting NA/NAMP explicitly in the deck.
            *na *= m;
            *namp *= m;
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
    let mut flattener = Flattener::with_models_config(
        &netlist.subcircuits,
        &netlist.models,
        FlattenerConfig::default(),
    );
    let elements = flattener.flatten(netlist)?;
    Ok(FlattenedNetlist {
        elements,
        scoped_models: flattener.scoped_models,
        scoped_initial_conditions: flattener.scoped_initial_conditions,
        scoped_node_sets: flattener.scoped_node_sets,
        xspice_auto_bridge_node_hints: flattener.xspice_auto_bridge_node_hints,
    })
}

fn is_ground_node_name(node: &str) -> bool {
    node == "0" || node.eq_ignore_ascii_case("gnd")
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

fn remap_current_probe_arg(prefix: &str, arg: &str) -> String {
    let trimmed = arg.trim();
    if trimmed.is_empty() || !is_simple_probe_name(trimmed) {
        return trimmed.to_string();
    }
    if prefix.is_empty() {
        trimmed.to_string()
    } else {
        format!("{}.{}", prefix, trimmed)
    }
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
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '#' || c == ':')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn duplicate_binding_error(source: &str) -> Box<DuplicateSubcircuitPortBindingError> {
        let netlist = Netlist::parse(source).expect("duplicate-formal deck parses");
        let error = flatten_netlist(&netlist).expect_err("conflicting invocation must fail");
        let ParseError::DuplicateSubcircuitPortBinding(error) = error else {
            panic!("expected typed duplicate subcircuit-port binding error");
        };
        error
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
}
