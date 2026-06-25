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
use super::hierarchy_path::HierarchyPath;
use super::param_scope::ParamResolver;
use super::{
    Element, ElementKind, Netlist, ParamContext, ParametricValue, ParseError, RandomState,
    SourceSpec, SubcircuitDef,
};
use crate::Value;
use std::collections::{HashMap, HashSet};

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
    subcircuits: HashMap<&'a str, &'a SubcircuitDef>,
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
}

impl<'a> Flattener<'a> {
    /// Create a new flattener with the given subcircuit definitions
    pub fn new(subcircuits: &'a [SubcircuitDef]) -> Self {
        Self::with_config(subcircuits, FlattenerConfig::default())
    }

    /// Create a flattener with custom configuration
    pub fn with_config(subcircuits: &'a [SubcircuitDef], config: FlattenerConfig) -> Self {
        let subcircuit_map: HashMap<&str, &SubcircuitDef> =
            subcircuits.iter().map(|s| (s.name.as_str(), s)).collect();

        // Initialize param resolver with subcircuit defaults
        let mut param_resolver = ParamResolver::new();
        for subckt in subcircuits {
            param_resolver.add_subcircuit_defaults(&subckt.name, &subckt.params);
        }

        Self {
            subcircuits: subcircuit_map,
            config,
            param_resolver,
            instance_metadata: Vec::new(),
            external_subckts: HashSet::new(),
            global_nodes: HashSet::new(),
            expansion_stack: Vec::new(),
            random: RandomState::default(),
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
        // Continue the netlist's statistical draw sequence (seeded at parse
        // time) so flatten-time draws are distinct per instance.
        self.random = netlist.params.random().clone();
        let global_params: HashMap<String, Value> = netlist
            .params
            .all_params()
            .into_iter()
            .map(|(name, value)| (canonical_param_name(&name), value))
            .collect();

        // Set global parameters from netlist
        for (name, value) in netlist.params.all_params() {
            self.param_resolver.set_global(&name, value);
        }

        for element in &netlist.elements {
            self.flatten_element(
                element,
                "",
                &HashMap::new(),
                &global_params,
                0,
                &mut flat_elements,
            )?;
        }

        Ok(flat_elements)
    }

    /// Flatten a single element, recursively expanding subcircuits
    fn flatten_element(
        &mut self,
        element: &Element,
        prefix: &str,
        node_map: &HashMap<String, String>,
        scope_params: &HashMap<String, Value>,
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
                if self.subcircuits.contains_key(subckt_name.as_str()) {
                    self.expand_subcircuit(
                        element,
                        subckt_name,
                        params,
                        prefix,
                        node_map,
                        scope_params,
                        depth,
                        output,
                    )?;
                } else if self.is_external_subckt(subckt_name) {
                    // Preserve external instance (e.g. Verilog-A model) as a leaf.
                    let new_element = self.resolve_external_subcircuit_params(
                        self.remap_element(element, prefix, node_map),
                        scope_params,
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
                output.push(new_element);
            }
        }

        Ok(())
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
        names
    }

    fn is_external_subckt(&self, name: &str) -> bool {
        self.external_subckts.contains(&name.to_ascii_uppercase())
    }

    /// Expand a subcircuit instance into its constituent elements
    fn expand_subcircuit(
        &mut self,
        instance: &Element,
        subckt_name: &str,
        instance_params: &[(String, ParametricValue)],
        prefix: &str,
        parent_node_map: &HashMap<String, String>,
        caller_scope_params: &HashMap<String, Value>,
        depth: usize,
        output: &mut Vec<Element>,
    ) -> Result<(), ParseError> {
        // Look up subcircuit definition
        let subckt = self
            .subcircuits
            .get(subckt_name)
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

        // A connection-count mismatch silently mis-wires the circuit if the
        // ports are truncated or nodes are dropped, so it is a hard error —
        // the same contract Spectre and HSPICE enforce.
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

        // A definition that is still being expanded cannot be instantiated
        // again below itself; report the cycle by name instead of expanding
        // until max_depth trips ~100 levels later.
        if self.expansion_stack.iter().any(|name| name == subckt_name) {
            let start = self
                .expansion_stack
                .iter()
                .position(|name| name == subckt_name)
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

        // Build node map: subcircuit port -> instance connection
        let mut node_map = HashMap::new();

        // Map ports to external connections
        for (i, port) in subckt.ports.iter().enumerate() {
            if i < instance.nodes.len() {
                let external_node = self.remap_node(&instance.nodes[i], prefix, parent_node_map);
                node_map.insert(port.clone(), external_node);
            }
        }

        let param_map = build_subcircuit_param_scope(
            subckt,
            caller_scope_params,
            instance_params,
            &self.random,
        )?;

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
                    let resolved =
                        resolve_parametric_value(value, caller_scope_params, &self.random)?;
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
            let mut substituted = self.substitute_params(sub_element, &param_map)?;
            if multiplicity != 1.0 {
                apply_element_multiplicity(&mut substituted, multiplicity);
            }
            self.flatten_element(
                &substituted,
                &new_prefix,
                &node_map,
                &param_map,
                depth + 1,
                output,
            )?;
        }
        self.expansion_stack.pop();

        Ok(())
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
                control_nodes,
            } => ElementKind::Vcvs {
                gain: *gain,
                control_nodes: (
                    self.remap_node(&control_nodes.0, prefix, node_map),
                    self.remap_node(&control_nodes.1, prefix, node_map),
                ),
            },
            ElementKind::Vccs {
                transconductance,
                control_nodes,
            } => ElementKind::Vccs {
                transconductance: *transconductance,
                control_nodes: (
                    self.remap_node(&control_nodes.0, prefix, node_map),
                    self.remap_node(&control_nodes.1, prefix, node_map),
                ),
            },
            ElementKind::Cccs {
                gain,
                control_element,
            } => {
                // Remap control element name with prefix (like element names)
                let new_ctrl = if prefix.is_empty() {
                    control_element.clone()
                } else {
                    format!("{}.{}", prefix, control_element)
                };
                ElementKind::Cccs {
                    gain: *gain,
                    control_element: new_ctrl,
                }
            }
            ElementKind::Ccvs {
                transresistance,
                control_element,
            } => {
                let new_ctrl = if prefix.is_empty() {
                    control_element.clone()
                } else {
                    format!("{}.{}", prefix, control_element)
                };
                ElementKind::Ccvs {
                    transresistance: *transresistance,
                    control_element: new_ctrl,
                }
            }
            // All other kinds - clone as-is
            other => other.clone(),
        };

        Element {
            name: new_name,
            kind: new_kind,
            nodes: new_nodes,
        }
    }

    /// Remap a single node name
    fn remap_node(&self, node: &str, prefix: &str, node_map: &HashMap<String, String>) -> String {
        // Ground is never renamed
        if node == "0" || node.eq_ignore_ascii_case("gnd") {
            return "0".to_string();
        }

        // .GLOBAL nodes retain their original names across hierarchy levels.
        if self.global_nodes.contains(&node.to_ascii_uppercase()) {
            return node.to_string();
        }

        // Check if this is a port that maps to an external node
        if let Some(mapped) = node_map.get(node) {
            return mapped.clone();
        }

        // Internal node - prefix with instance path
        if prefix.is_empty() {
            node.to_string()
        } else {
            format!("{}{}{}", prefix, self.config.hierarchy_separator, node)
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
        &self,
        element: &Element,
        param_map: &HashMap<String, Value>,
    ) -> Result<Element, ParseError> {
        let new_kind = match &element.kind {
            // Passive components
            ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } => ElementKind::Resistor {
                value: self.resolve_optional_value_expr(*value, value_expr, param_map)?,
                value_expr: None,
                model: model.clone(),
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    param_map,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::Capacitor {
                value,
                value_expr,
                initial_voltage,
                model,
                instance_params,
                deferred_params,
            } => ElementKind::Capacitor {
                value: self.resolve_optional_value_expr(*value, value_expr, param_map)?,
                value_expr: None,
                initial_voltage: *initial_voltage,
                model: model.clone(),
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    param_map,
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
                value: self.resolve_optional_value_expr(*value, value_expr, param_map)?,
                value_expr: None,
                initial_current: *initial_current,
                model: model.clone(),
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    param_map,
                )?,
                deferred_params: Vec::new(),
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
                model: model.clone(),
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    param_map,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::Bjt {
                model,
                bjt_type,
                instance_params,
                deferred_params,
            } => ElementKind::Bjt {
                model: model.clone(),
                bjt_type: *bjt_type,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    param_map,
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
                model: model.clone(),
                mos_type: *mos_type,
                compact_syntax: *compact_syntax,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    param_map,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::Jfet {
                model,
                jfet_type,
                instance_params,
                deferred_params,
            } => ElementKind::Jfet {
                model: model.clone(),
                jfet_type: *jfet_type,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    param_map,
                )?,
                deferred_params: Vec::new(),
            },
            ElementKind::Mesfet {
                model,
                mesfet_type,
                instance_params,
                deferred_params,
            } => ElementKind::Mesfet {
                model: model.clone(),
                mesfet_type: *mesfet_type,
                instance_params: self.merge_deferred_params(
                    instance_params,
                    deferred_params,
                    param_map,
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
                    merged_params.push((
                        name.clone(),
                        ParametricValue::Resolved(resolve_parametric_value(
                            value,
                            param_map,
                            &self.random,
                        )?),
                    ));
                }

                ElementKind::Subcircuit {
                    subckt_name: subckt_name.clone(),
                    params: merged_params,
                }
            }

            // Controlled sources
            ElementKind::Vcvs {
                gain,
                control_nodes,
            } => ElementKind::Vcvs {
                gain: *gain,
                control_nodes: control_nodes.clone(),
            },
            ElementKind::Vccs {
                transconductance,
                control_nodes,
            } => ElementKind::Vccs {
                transconductance: *transconductance,
                control_nodes: control_nodes.clone(),
            },
            ElementKind::Cccs {
                gain,
                control_element,
            } => ElementKind::Cccs {
                gain: *gain,
                control_element: control_element.clone(),
            },
            ElementKind::Ccvs {
                transresistance,
                control_element,
            } => ElementKind::Ccvs {
                transresistance: *transresistance,
                control_element: control_element.clone(),
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
        param_map: &HashMap<String, Value>,
    ) -> Result<Value, ParseError> {
        match value_expr {
            Some(expr) => resolve_parametric_value(
                &ParametricValue::Expression(expr.clone()),
                param_map,
                &self.random,
            ),
            None => Ok(value),
        }
    }

    /// Merge deferred (expression-valued) instance parameters over the
    /// parse-time-resolved set, evaluating each against this instance's
    /// parameter scope. A deferred entry overrides a same-named resolved one.
    fn merge_deferred_params(
        &self,
        instance_params: &[(String, Value)],
        deferred_params: &[(String, String)],
        param_map: &HashMap<String, Value>,
    ) -> Result<Vec<(String, Value)>, ParseError> {
        if deferred_params.is_empty() {
            return Ok(instance_params.to_vec());
        }
        let mut merged = instance_params.to_vec();
        for (name, expr) in deferred_params {
            let value = resolve_parametric_value(
                &ParametricValue::Expression(expr.clone()),
                param_map,
                &self.random,
            )?;
            match merged.iter_mut().find(|(existing, _)| existing == name) {
                Some(slot) => slot.1 = value,
                None => merged.push((name.clone(), value)),
            }
        }
        Ok(merged)
    }

    fn resolve_external_subcircuit_params(
        &self,
        mut element: Element,
        scope_params: &HashMap<String, Value>,
    ) -> Result<Element, ParseError> {
        if let ElementKind::Subcircuit { params, .. } = &mut element.kind {
            for (_, value) in params.iter_mut() {
                let resolved = resolve_parametric_value(value, scope_params, &self.random)?;
                *value = ParametricValue::Resolved(resolved);
            }
        }
        Ok(element)
    }
}

fn resolve_parametric_value(
    value: &ParametricValue,
    param_map: &HashMap<String, Value>,
    random: &RandomState,
) -> Result<Value, ParseError> {
    match value {
        ParametricValue::Resolved(v) => Ok(*v),
        ParametricValue::Expression(expr) => {
            let mut ctx = ParamContext::new();
            for (name, value) in param_map {
                ctx.set(name, *value);
            }
            // Join the netlist-wide stream: each instance expression that
            // calls gauss/agauss/unif/aunif/limit advances one shared,
            // reproducible sequence instead of replaying the same draws.
            ctx.adopt_random(random);
            super::expr::eval_expression(expr, &ctx)
                .map_err(|e| ParseError::InvalidValue(e.to_string()))
        }
    }
}

fn canonical_param_name(name: &str) -> String {
    name.to_ascii_uppercase()
}

fn build_subcircuit_param_scope(
    subckt: &SubcircuitDef,
    caller_scope_params: &HashMap<String, Value>,
    instance_params: &[(String, ParametricValue)],
    random: &RandomState,
) -> Result<HashMap<String, Value>, ParseError> {
    let mut param_map: HashMap<String, Value> = caller_scope_params
        .iter()
        .map(|(name, value)| (canonical_param_name(name), *value))
        .collect();

    for (name, value) in &subckt.params {
        param_map.insert(canonical_param_name(name), *value);
    }

    for (name, value) in instance_params {
        let resolved = resolve_parametric_value(value, caller_scope_params, random)?;
        param_map.insert(canonical_param_name(name), resolved);
    }

    Ok(param_map)
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
        SourceSpec::Pwl { points } => {
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
