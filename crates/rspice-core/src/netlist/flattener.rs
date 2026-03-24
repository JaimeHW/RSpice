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
use super::{Element, ElementKind, Netlist, ParamContext, ParametricValue, ParseError, SubcircuitDef};
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
    /// Counter for generating unique node names (when not preserving hierarchy)
    #[allow(dead_code)]
    node_counter: usize,
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
            node_counter: 0,
            config,
            param_resolver,
            instance_metadata: Vec::new(),
            external_subckts: HashSet::new(),
            global_nodes: HashSet::new(),
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

        // Build node map: subcircuit port -> instance connection
        let mut node_map = HashMap::new();

        // Map ports to external connections
        for (i, port) in subckt.ports.iter().enumerate() {
            if i < instance.nodes.len() {
                let external_node = self.remap_node(&instance.nodes[i], prefix, parent_node_map);
                node_map.insert(port.clone(), external_node);
            }
        }

        let param_map =
            build_subcircuit_param_scope(subckt, caller_scope_params, instance_params)?;

        // Expand each element in the subcircuit
        for sub_element in &subckt.elements {
            // Apply parameter substitution to element values
            let substituted = self.substitute_params(sub_element, &param_map)?;
            self.flatten_element(
                &substituted,
                &new_prefix,
                &node_map,
                &param_map,
                depth + 1,
                output,
            )?;
        }

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
                if is_probe && ws_idx < chars.len() && chars[ws_idx] == '(' {
                    if let Some((inner, end_idx)) = extract_parenthesized(&chars, ws_idx) {
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
            } => {
                let resolved_value = if let Some(expr) = value_expr {
                    resolve_parametric_value(&ParametricValue::Expression(expr.clone()), param_map)?
                } else {
                    *value
                };
                ElementKind::Resistor {
                    value: resolved_value,
                    value_expr: None,
                    model: model.clone(),
                    instance_params: instance_params.clone(),
                }
            }
            ElementKind::Capacitor {
                value,
                initial_voltage,
            } => ElementKind::Capacitor {
                value: *value,
                initial_voltage: *initial_voltage,
            },
            ElementKind::Inductor {
                value,
                initial_current,
            } => ElementKind::Inductor {
                value: *value,
                initial_current: *initial_current,
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
                        ParametricValue::Resolved(resolve_parametric_value(value, param_map)?),
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

    fn resolve_external_subcircuit_params(
        &self,
        mut element: Element,
        scope_params: &HashMap<String, Value>,
    ) -> Result<Element, ParseError> {
        if let ElementKind::Subcircuit { params, .. } = &mut element.kind {
            for (_, value) in params.iter_mut() {
                let resolved = resolve_parametric_value(value, scope_params)?;
                *value = ParametricValue::Resolved(resolved);
            }
        }
        Ok(element)
    }
}

fn resolve_parametric_value(
    value: &ParametricValue,
    param_map: &HashMap<String, Value>,
) -> Result<Value, ParseError> {
    match value {
        ParametricValue::Resolved(v) => Ok(*v),
        ParametricValue::Expression(expr) => {
            let mut ctx = ParamContext::new();
            for (name, value) in param_map {
                ctx.set(name, *value);
            }
            super::expr::eval_expression(expr, &ctx).map_err(|e| ParseError::InvalidValue(e.to_string()))
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
) -> Result<HashMap<String, Value>, ParseError> {
    let mut param_map: HashMap<String, Value> = caller_scope_params
        .iter()
        .map(|(name, value)| (canonical_param_name(name), *value))
        .collect();

    for (name, value) in &subckt.params {
        param_map.insert(canonical_param_name(name), *value);
    }

    for (name, value) in instance_params {
        let resolved = resolve_parametric_value(value, caller_scope_params)?;
        param_map.insert(canonical_param_name(name), resolved);
    }

    Ok(param_map)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::parse_netlist;

    #[test]
    fn test_flatten_simple_subcircuit() {
        let netlist_str = r#"Subcircuit Test
.SUBCKT RESISTOR_DIV IN OUT
R1 IN MID 1k
R2 MID OUT 1k
.ENDS
X1 1 2 RESISTOR_DIV
V1 1 0 10
R3 2 0 1k
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();

        // Should have: V1, R3, X1.R1, X1.R2
        assert_eq!(flat.len(), 4, "Expected 4 elements, got {}", flat.len());

        // Check that internal node was renamed
        let r1 = flat
            .iter()
            .find(|e| e.name == "X1.R1")
            .expect("Missing X1.R1");
        assert!(
            r1.nodes.contains(&"X1.MID".to_string()),
            "Expected X1.MID node"
        );
    }

    #[test]
    fn test_flatten_nested_subcircuit() {
        let netlist_str = r#"Nested Subcircuit Test
.SUBCKT RESISTOR A B
R1 A B 1k
.ENDS
.SUBCKT TWO_RESISTORS IN OUT
X1 IN MID RESISTOR
X2 MID OUT RESISTOR
.ENDS
X1 1 2 TWO_RESISTORS
V1 1 0 5
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();

        // Should have: V1, X1.X1.R1, X1.X2.R1
        assert_eq!(flat.len(), 3, "Expected 3 elements, got {}", flat.len());

        // Check nested naming
        assert!(
            flat.iter().any(|e| e.name == "X1.X1.R1"),
            "Missing X1.X1.R1"
        );
        assert!(
            flat.iter().any(|e| e.name == "X1.X2.R1"),
            "Missing X1.X2.R1"
        );
    }

    #[test]
    fn test_ground_not_renamed() {
        let netlist_str = r#"Ground Test
.SUBCKT GROUNDED_R IN
R1 IN 0 1k
.ENDS
X1 1 GROUNDED_R
V1 1 0 5
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();

        let r1 = flat
            .iter()
            .find(|e| e.name == "X1.R1")
            .expect("Missing X1.R1");
        assert!(
            r1.nodes.contains(&"0".to_string()),
            "Ground should remain as 0"
        );
    }

    #[test]
    fn test_subcircuit_with_default_params() {
        let netlist_str = r#"Subcircuit Params Test
.SUBCKT RESISTOR_PAR IN OUT R=1k
R1 IN OUT 1k
.ENDS
X1 1 2 RESISTOR_PAR
V1 1 0 10
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();

        // Check that subcircuit definition has default params
        let subckt = netlist
            .subcircuits
            .iter()
            .find(|s| s.name == "RESISTOR_PAR")
            .expect("Missing subcircuit definition");
        assert!(!subckt.params.is_empty(), "Expected default parameters");

        // Check R parameter
        let r_param = subckt.params.iter().find(|(n, _)| n == "R");
        assert!(r_param.is_some(), "Expected R parameter");
        assert!(
            (r_param.unwrap().1 - 1000.0).abs() < 1e-10,
            "R should be 1k"
        );
    }

    #[test]
    fn test_subcircuit_instance_params() {
        let netlist_str = r#"Instance Params Test
.SUBCKT RES IN OUT R=1k
R1 IN OUT 1k
.ENDS
X1 1 2 RES R=2k
V1 1 0 10
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();

        // Find the subcircuit instance
        let x1 = netlist
            .elements
            .iter()
            .find(|e| e.name == "X1")
            .expect("Missing X1 instance");

        // Check it has instance parameters
        if let ElementKind::Subcircuit { params, .. } = &x1.kind {
            assert!(!params.is_empty(), "Expected instance parameters");
            let r_param = params.iter().find(|(n, _)| n == "R");
            assert!(r_param.is_some(), "Expected R parameter");
            assert!(
                matches!(
                    r_param.unwrap().1,
                    ParametricValue::Resolved(v) if (v - 2000.0).abs() < 1e-10
                ),
                "R should be 2k"
            );
        } else {
            panic!("Expected Subcircuit element");
        }
    }

    #[test]
    fn test_subcircuit_params_colon_syntax() {
        // PARAMS: syntax
        let netlist_str = r#"Params Colon Test
.SUBCKT RES IN OUT PARAMS: R=1k C=1n
R1 IN OUT 1k
.ENDS
X1 1 2 RES
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();

        let subckt = netlist
            .subcircuits
            .iter()
            .find(|s| s.name == "RES")
            .expect("Missing subcircuit definition");
        assert_eq!(subckt.params.len(), 2, "Expected 2 default parameters");
    }

    #[test]
    fn test_flatten_with_params() {
        // Test that parameter maps are passed during flattening
        let netlist_str = r#"Flatten Params Test
.SUBCKT DIV IN OUT
R1 IN MID 1k
R2 MID OUT 1k  
.ENDS
X1 1 2 DIV
V1 1 0 5
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();

        // Verify flattening still works: V1 + X1.R1 + X1.R2 = 3 elements
        assert_eq!(flat.len(), 3);
    }

    #[test]
    fn test_flatten_preserves_veriloga_external_instance() {
        let netlist_str = r#"External VerilogA Test
.VERILOGA custom_model.va custom_model
V1 in 0 1
X1 in 0 custom_model g=2m
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();

        assert_eq!(flat.len(), 2, "Expected V1 and X1 to remain");
        let x1 = flat
            .iter()
            .find(|e| e.name == "X1")
            .expect("Missing external X1 instance");
        match &x1.kind {
            ElementKind::Subcircuit {
                subckt_name,
                params,
            } => {
                assert!(subckt_name.eq_ignore_ascii_case("custom_model"));
                assert!(
                    params.iter().any(|(name, value)| {
                        name.eq_ignore_ascii_case("g")
                            && matches!(value, ParametricValue::Resolved(v) if v.is_finite())
                    })
                );
            }
            other => panic!(
                "Expected Subcircuit kind for external instance, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn test_flatten_unknown_external_without_include_errors() {
        let netlist_str = r#"Unknown External
V1 in 0 1
X1 in 0 missing_model
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let err = flatten_netlist(&netlist).expect_err("missing external model should error");
        let msg = err.to_string();
        assert!(
            msg.contains("Undefined subcircuit")
                && msg.to_ascii_uppercase().contains("MISSING_MODEL"),
            "Unexpected error: {}",
            err
        );
    }

    #[test]
    fn test_flatten_preserves_global_nodes_across_subcircuits() {
        let netlist_str = r#"Global Nodes
.global VDD
.subckt child out
R1 out 0 1k
R2 VDD 0 2k
.ends
V1 VDD 0 5
X1 out child
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        assert!(netlist.is_global("VDD"));

        let flat = flatten_netlist(&netlist).unwrap();

        let r1 = flat
            .iter()
            .find(|element| element.name == "X1.R1")
            .expect("expected flattened local resistor");
        assert!(r1.nodes[0].eq_ignore_ascii_case("OUT"));
        assert_eq!(r1.nodes[1], "0");

        let r2 = flat
            .iter()
            .find(|element| element.name == "X1.R2")
            .expect("expected flattened global resistor");
        assert!(r2.nodes[0].eq_ignore_ascii_case("VDD"));
        assert_eq!(r2.nodes[1], "0");
    }

    #[test]
    fn test_flatten_subcircuits_inherit_global_parameter_scope() {
        let netlist_str = r#"Inherited Parameters
.param rval=2k
.subckt child in out
R1 in out {rval}
.ends
V1 in 0 1
X1 in out child
R2 out 0 1k
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();

        let r1 = flat
            .iter()
            .find(|element| element.name == "X1.R1")
            .expect("expected flattened subcircuit resistor");
        match &r1.kind {
            ElementKind::Resistor { value, value_expr, .. } => {
                assert_eq!(value_expr, &None);
                assert!((*value - 2_000.0).abs() < 1e-9, "expected 2k, got {value}");
            }
            other => panic!("expected resistor, got {:?}", other),
        }
    }

    #[test]
    fn test_flatten_nested_numeric_port_subcircuits_keep_port_order_and_model_scope() {
        let netlist_str = r#"Model Scope
I2 n1002_t 0 DC=-1
I3 n1003_t 0 DC=-1
I4 n1004_t 0 DC=-1
I5 n1005_t 0 DC=-1
I6 n1006_t 0 DC=-1
I7 n1007_t 0 DC=-1
X2 n1002_t n1003_t n1004_t n1005_t n1006_t n1007_t sub2

.subckt sub2 3 41a 41b 42a 42b 5
  R2 3 0 my
  X31 41a 41b sub3
  X32 42a 42b sub3
  .subckt sub3 4 5
    .model my r r=8k
    R5 4 0 1k
    R6 5 0 my
  .ends
  .model just r r=43
  R7 5 0 just
.ends

.model my r r=4k
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let x2 = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("X2"))
            .expect("expected top-level subckt instance");
        assert_eq!(
            x2.nodes,
            vec![
                "N1002_T".to_string(),
                "N1003_T".to_string(),
                "N1004_T".to_string(),
                "N1005_T".to_string(),
                "N1006_T".to_string(),
                "N1007_T".to_string()
            ]
        );

        let sub2 = netlist
            .subcircuits
            .iter()
            .find(|subckt| subckt.name == "sub2")
            .expect("expected parent subckt");
        assert_eq!(
            sub2.ports,
            vec![
                "3".to_string(),
                "41A".to_string(),
                "41B".to_string(),
                "42A".to_string(),
                "42B".to_string(),
                "5".to_string()
            ]
        );
        let x31 = sub2
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("X31"))
            .expect("expected nested subckt instance");
        assert_eq!(x31.nodes, vec!["41A".to_string(), "41B".to_string()]);

        let sub3 = netlist
            .subcircuits
            .iter()
            .find(|subckt| subckt.name == "sub2.sub3")
            .expect("expected qualified nested subckt");
        assert_eq!(sub3.ports, vec!["4".to_string(), "5".to_string()]);

        let flat = flatten_netlist(&netlist).unwrap();
        let x2_r2 = flat
            .iter()
            .find(|element| element.name == "X2.R2")
            .expect("expected X2.R2");
        match &x2_r2.kind {
            ElementKind::Resistor { model, .. } => assert!(
                model.as_deref().is_some_and(|name| name.eq_ignore_ascii_case("my")),
                "parent resistor should bind to the top-level model, got {:?}",
                model
            ),
            other => panic!("expected resistor, got {:?}", other),
        }

        let x31_r5 = flat
            .iter()
            .find(|element| element.name == "X2.X31.R5")
            .expect("expected X2.X31.R5");
        assert_eq!(x31_r5.nodes, vec!["N1003_T".to_string(), "0".to_string()]);

        let x31_r6 = flat
            .iter()
            .find(|element| element.name == "X2.X31.R6")
            .expect("expected X2.X31.R6");
        assert_eq!(x31_r6.nodes, vec!["N1004_T".to_string(), "0".to_string()]);
        match &x31_r6.kind {
            ElementKind::Resistor { model, .. } => assert!(
                model
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case("sub2.sub3::my")),
                "nested resistor should bind to nested local model, got {:?}",
                model
            ),
            other => panic!("expected resistor, got {:?}", other),
        }

        let x32_r5 = flat
            .iter()
            .find(|element| element.name == "X2.X32.R5")
            .expect("expected X2.X32.R5");
        assert_eq!(x32_r5.nodes, vec!["N1005_T".to_string(), "0".to_string()]);

        let x32_r6 = flat
            .iter()
            .find(|element| element.name == "X2.X32.R6")
            .expect("expected X2.X32.R6");
        assert_eq!(x32_r6.nodes, vec!["N1006_T".to_string(), "0".to_string()]);
        match &x32_r6.kind {
            ElementKind::Resistor { model, .. } => assert!(
                model
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case("sub2.sub3::my")),
                "nested resistor should bind to nested local model, got {:?}",
                model
            ),
            other => panic!("expected resistor, got {:?}", other),
        }

        let x2_r7 = flat
            .iter()
            .find(|element| element.name == "X2.R7")
            .expect("expected X2.R7");
        assert_eq!(x2_r7.nodes, vec!["N1007_T".to_string(), "0".to_string()]);
        match &x2_r7.kind {
            ElementKind::Resistor { model, .. } => assert!(
                model
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case("sub2::just")),
                "parent-local resistor should bind to parent-local model, got {:?}",
                model
            ),
            other => panic!("expected resistor, got {:?}", other),
        }
    }

    #[test]
    fn test_flatten_resolves_nested_resistor_parameter_expressions_per_instance_scope() {
        let netlist_str = r#"Scope Expressions
.param foo=2k
.subckt sub1 n1 n2 foo=5k
.subckt sub n1 n2 foo=10k
R1 n1 n2 'foo'
.ends
X1 n1 n2 sub foo='3*foo'
R2 n1 n2 '5*foo'
.ends
XTOP n1 0 sub1 foo='foo*3'
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();

        let nested = flat
            .iter()
            .find(|element| element.name == "XTOP.X1.R1")
            .expect("expected nested resistor");
        match &nested.kind {
            ElementKind::Resistor { value, value_expr, .. } => {
                assert_eq!(value_expr, &None);
                assert!((*value - 18_000.0).abs() < 1e-9, "expected 18k, got {value}");
            }
            other => panic!("expected resistor, got {:?}", other),
        }

        let outer = flat
            .iter()
            .find(|element| element.name == "XTOP.R2")
            .expect("expected outer resistor");
        match &outer.kind {
            ElementKind::Resistor { value, value_expr, .. } => {
                assert_eq!(value_expr, &None);
                assert!((*value - 30_000.0).abs() < 1e-9, "expected 30k, got {value}");
            }
            other => panic!("expected resistor, got {:?}", other),
        }
    }

    #[test]
    fn test_flatten_remaps_behavioral_internal_voltage_probe_nodes() {
        let netlist_str = r#"Behavioral Probe Remap
.SUBCKT BTEST OUT
V1 1 0 1
B1 OUT 0 V=nint(v(1))
.ENDS
X1 out BTEST
RLOAD out 0 1k
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();
        let b1 = flat
            .iter()
            .find(|e| e.name == "X1.B1")
            .expect("expected flattened behavioral source");
        let expression = match &b1.kind {
            ElementKind::BehavioralVoltage { expression, .. } => expression,
            other => panic!("expected behavioral voltage source, got {:?}", other),
        };

        let upper = expression.to_ascii_uppercase();
        assert!(
            upper.contains("V(X1.1)"),
            "expected hierarchical node remap in expression, got '{}'",
            expression
        );
        assert!(
            !upper.contains("V(1)"),
            "unremapped local node probe should not remain, got '{}'",
            expression
        );
    }

    #[test]
    fn test_flatten_remaps_behavioral_branch_current_probes() {
        let netlist_str = r#"Behavioral Branch Probe Remap
.SUBCKT BTEST OUT
VS local 0 1
B1 OUT 0 V=I(VS)
.ENDS
X1 out BTEST
RLOAD out 0 1k
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();
        let b1 = flat
            .iter()
            .find(|e| e.name == "X1.B1")
            .expect("expected flattened behavioral source");
        let expression = match &b1.kind {
            ElementKind::BehavioralVoltage { expression, .. } => expression,
            other => panic!("expected behavioral voltage source, got {:?}", other),
        };
        assert!(
            expression.to_ascii_uppercase().contains("I(X1.VS)"),
            "expected hierarchical branch remap in expression, got '{}'",
            expression
        );
    }

    #[test]
    fn test_flatten_remaps_behavioral_subckt_ports_to_instance_nodes() {
        let netlist_str = r#"Behavioral Port Probe Remap
.SUBCKT DIFFBUF P N OUT
B1 OUT 0 V=V(P,N)
.ENDS
X1 a b out DIFFBUF
RLOAD out 0 1k
.end
"#;
        let netlist = parse_netlist(netlist_str).unwrap();
        let flat = flatten_netlist(&netlist).unwrap();
        let b1 = flat
            .iter()
            .find(|e| e.name == "X1.B1")
            .expect("expected flattened behavioral source");
        let expression = match &b1.kind {
            ElementKind::BehavioralVoltage { expression, .. } => expression,
            other => panic!("expected behavioral voltage source, got {:?}", other),
        };
        assert!(
            expression.to_ascii_uppercase().contains("V(A, B)"),
            "expected external-port node remap in expression, got '{}'",
            expression
        );
    }
}
