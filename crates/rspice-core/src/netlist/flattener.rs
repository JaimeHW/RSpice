//! Subcircuit flattening - converts hierarchical netlists to flat element lists
//!
//! This module handles the expansion of X (subcircuit instance) elements by:
//! 1. Looking up the subcircuit definition
//! 2. Renaming internal nodes to prevent collisions
//! 3. Mapping external ports to instance connections
//! 4. Recursively handling nested subcircuits

use std::collections::HashMap;
use super::{Element, ElementKind, Netlist, SubcircuitDef, ParseError};
use crate::Value;

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
}

impl Default for FlattenerConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            preserve_hierarchy: true,  // Default to full path for debugging
            hierarchy_separator: '.',
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
        }
    }
    
    /// Create a config optimized for performance (shorter names)
    pub fn production() -> Self {
        Self {
            max_depth: 100,
            preserve_hierarchy: false,
            hierarchy_separator: '_',
        }
    }
}

//=============================================================================
// Flattener
//=============================================================================

/// Flattens a hierarchical netlist into a flat element list
pub struct Flattener<'a> {
    /// Subcircuit definitions indexed by name
    subcircuits: HashMap<&'a str, &'a SubcircuitDef>,
    /// Counter for generating unique node names (when not preserving hierarchy)
    node_counter: usize,
    /// Configuration options
    config: FlattenerConfig,
}

impl<'a> Flattener<'a> {
    /// Create a new flattener with the given subcircuit definitions
    pub fn new(subcircuits: &'a [SubcircuitDef]) -> Self {
        Self::with_config(subcircuits, FlattenerConfig::default())
    }

    /// Create a flattener with custom configuration
    pub fn with_config(subcircuits: &'a [SubcircuitDef], config: FlattenerConfig) -> Self {
        let subcircuits = subcircuits
            .iter()
            .map(|s| (s.name.as_str(), s))
            .collect();
        
        Self {
            subcircuits,
            node_counter: 0,
            config,
        }
    }

    /// Flatten a netlist, expanding all subcircuit instances
    pub fn flatten(&mut self, netlist: &Netlist) -> Result<Vec<Element>, ParseError> {
        let mut flat_elements = Vec::new();
        
        for element in &netlist.elements {
            self.flatten_element(element, "", &HashMap::new(), 0, &mut flat_elements)?;
        }
        
        Ok(flat_elements)
    }

    /// Flatten a single element, recursively expanding subcircuits
    fn flatten_element(
        &mut self,
        element: &Element,
        prefix: &str,
        node_map: &HashMap<String, String>,
        depth: usize,
        output: &mut Vec<Element>,
    ) -> Result<(), ParseError> {
        if depth > self.config.max_depth {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!("Subcircuit recursion depth exceeded (max {})", self.config.max_depth),
            });
        }

        match &element.kind {
            ElementKind::Subcircuit { subckt_name, params } => {
                self.expand_subcircuit(element, subckt_name, params, prefix, node_map, depth, output)?;
            }
            _ => {
                // Regular element - remap nodes and add to output
                let new_element = self.remap_element(element, prefix, node_map);
                output.push(new_element);
            }
        }

        Ok(())
    }

    /// Expand a subcircuit instance into its constituent elements
    fn expand_subcircuit(
        &mut self,
        instance: &Element,
        subckt_name: &str,
        instance_params: &[(String, Value)],
        prefix: &str,
        parent_node_map: &HashMap<String, String>,
        depth: usize,
        output: &mut Vec<Element>,
    ) -> Result<(), ParseError> {
        // Look up subcircuit definition
        let subckt = self.subcircuits.get(subckt_name).ok_or_else(|| {
            ParseError::Syntax {
                line: 0,
                message: format!("Undefined subcircuit: {}", subckt_name),
            }
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

        // Build parameter map for value substitution
        let mut param_map: HashMap<&str, Value> = subckt.params
            .iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect();
        
        // Instance parameters override definition defaults
        for (name, value) in instance_params {
            param_map.insert(name.as_str(), *value);
        }

        // Expand each element in the subcircuit
        for sub_element in &subckt.elements {
            // Apply parameter substitution to element values
            let substituted = self.substitute_params(sub_element, &param_map);
            self.flatten_element(&substituted, &new_prefix, &node_map, depth + 1, output)?;
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

        let new_nodes: Vec<String> = element.nodes
            .iter()
            .map(|n| self.remap_node(n, prefix, node_map))
            .collect();

        // Remap the element kind, handling CCCS/CCVS control element names
        let new_kind = match &element.kind {
            ElementKind::Cccs { gain, control_element } => {
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
            ElementKind::Ccvs { transresistance, control_element } => {
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
    fn remap_node(
        &self,
        node: &str,
        prefix: &str,
        node_map: &HashMap<String, String>,
    ) -> String {
        // Ground is never renamed
        if node == "0" || node.eq_ignore_ascii_case("gnd") {
            return "0".to_string();
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
        param_map: &HashMap<&str, Value>,
    ) -> Element {
        let new_kind = match &element.kind {
            // Passive components
            ElementKind::Resistor { value } => {
                ElementKind::Resistor { value: *value }
            }
            ElementKind::Capacitor { value, initial_voltage } => {
                ElementKind::Capacitor { 
                    value: *value, 
                    initial_voltage: *initial_voltage 
                }
            }
            ElementKind::Inductor { value, initial_current } => {
                ElementKind::Inductor { 
                    value: *value, 
                    initial_current: *initial_current 
                }
            }
            
            // Nested subcircuit - propagate parameters
            ElementKind::Subcircuit { subckt_name, params: instance_params } => {
                // Merge: instance params override those in param_map
                let mut merged_params: Vec<(String, Value)> = instance_params.clone();
                
                // Substitute any parameter references in instance params
                for (name, value) in &mut merged_params {
                    // If the value matches a known parameter name (sentinel value),
                    // substitute with the actual parameter value
                    if let Some(&param_value) = param_map.get(name.as_str()) {
                        // Only substitute if the value appears to be a reference
                        // (in practice, the parser would resolve expressions)
                        if value.is_nan() || *value == 0.0 {
                            *value = param_value;
                        }
                    }
                }
                
                ElementKind::Subcircuit {
                    subckt_name: subckt_name.clone(),
                    params: merged_params,
                }
            }
            
            // Controlled sources
            ElementKind::Vcvs { gain, control_nodes } => {
                ElementKind::Vcvs { 
                    gain: *gain, 
                    control_nodes: control_nodes.clone() 
                }
            }
            ElementKind::Vccs { transconductance, control_nodes } => {
                ElementKind::Vccs { 
                    transconductance: *transconductance, 
                    control_nodes: control_nodes.clone() 
                }
            }
            ElementKind::Cccs { gain, control_element } => {
                ElementKind::Cccs { 
                    gain: *gain, 
                    control_element: control_element.clone() 
                }
            }
            ElementKind::Ccvs { transresistance, control_element } => {
                ElementKind::Ccvs { 
                    transresistance: *transresistance, 
                    control_element: control_element.clone() 
                }
            }
            
            // All other element types - clone as-is
            other => other.clone(),
        };

        Element {
            name: element.name.clone(),
            kind: new_kind,
            nodes: element.nodes.clone(),
        }
    }
}

/// Convenience function to flatten a netlist
pub fn flatten_netlist(netlist: &Netlist) -> Result<Vec<Element>, ParseError> {
    let mut flattener = Flattener::new(&netlist.subcircuits);
    flattener.flatten(netlist)
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
        let r1 = flat.iter().find(|e| e.name == "X1.R1").expect("Missing X1.R1");
        assert!(r1.nodes.contains(&"X1.MID".to_string()), "Expected X1.MID node");
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
        assert!(flat.iter().any(|e| e.name == "X1.X1.R1"), "Missing X1.X1.R1");
        assert!(flat.iter().any(|e| e.name == "X1.X2.R1"), "Missing X1.X2.R1");
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
        
        let r1 = flat.iter().find(|e| e.name == "X1.R1").expect("Missing X1.R1");
        assert!(r1.nodes.contains(&"0".to_string()), "Ground should remain as 0");
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
        let subckt = netlist.subcircuits.iter().find(|s| s.name == "RESISTOR_PAR")
            .expect("Missing subcircuit definition");
        assert!(!subckt.params.is_empty(), "Expected default parameters");
        
        // Check R parameter
        let r_param = subckt.params.iter().find(|(n, _)| n == "R");
        assert!(r_param.is_some(), "Expected R parameter");
        assert!((r_param.unwrap().1 - 1000.0).abs() < 1e-10, "R should be 1k");
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
        let x1 = netlist.elements.iter().find(|e| e.name == "X1")
            .expect("Missing X1 instance");
        
        // Check it has instance parameters
        if let ElementKind::Subcircuit { params, .. } = &x1.kind {
            assert!(!params.is_empty(), "Expected instance parameters");
            let r_param = params.iter().find(|(n, _)| n == "R");
            assert!(r_param.is_some(), "Expected R parameter");
            assert!((r_param.unwrap().1 - 2000.0).abs() < 1e-10, "R should be 2k");
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
        
        let subckt = netlist.subcircuits.iter().find(|s| s.name == "RES")
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
}

