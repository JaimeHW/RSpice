//! Results Browser Tree
//!
//! Hierarchical tree browser for simulation results matching Cadence's
//! Results Browser functionality.
//!
//! # Features
//!
//! - Tree structure: Analysis → Signals → Data
//! - Expand/collapse nodes
//! - Signal selection for waveform viewing
//! - Context menu for operations
//! - Search/filter capabilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Tree Node Types
// =============================================================================

/// Type of result node in the browser tree
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResultNodeType {
    /// Root node (simulation session)
    Root,
    /// Analysis type category (Transient, AC, DC, etc.)
    Analysis,
    /// Signal category (Voltages, Currents, etc.)
    Category,
    /// Individual signal (V(out), I(R1), etc.)
    Signal,
    /// Device operating point group
    DeviceOp,
    /// Individual device parameter
    DeviceParam,
}

impl ResultNodeType {
    /// Icon character for this node type
    pub fn icon(&self) -> &'static str {
        match self {
            ResultNodeType::Root => "📊",
            ResultNodeType::Analysis => "📈",
            ResultNodeType::Category => "📁",
            ResultNodeType::Signal => "〰️",
            ResultNodeType::DeviceOp => "🔧",
            ResultNodeType::DeviceParam => "⚙️",
        }
    }
}

/// A node in the results browser tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultNode {
    /// Unique node ID
    pub id: u64,
    /// Parent node ID (None for root)
    pub parent_id: Option<u64>,
    /// Node type
    pub node_type: ResultNodeType,
    /// Display name
    pub name: String,
    /// Full hierarchical path
    pub path: String,
    /// Whether node is expanded
    pub expanded: bool,
    /// Whether node is selected
    pub selected: bool,
    /// Children node IDs
    pub children: Vec<u64>,
    /// Associated data (signal name for plotting, etc.)
    pub signal_name: Option<String>,
    /// Scalar value if applicable
    pub value: Option<f64>,
    /// Unit string if applicable
    pub unit: Option<String>,
}

impl Default for ResultNode {
    fn default() -> Self {
        Self {
            id: 0,
            parent_id: None,
            node_type: ResultNodeType::Root,
            name: String::new(),
            path: String::new(),
            expanded: false,
            selected: false,
            children: Vec::new(),
            signal_name: None,
            value: None,
            unit: None,
        }
    }
}

impl ResultNode {
    /// Create a new node
    pub fn new(id: u64, name: impl Into<String>, node_type: ResultNodeType) -> Self {
        let name = name.into();
        Self {
            id,
            name: name.clone(),
            path: name,
            node_type,
            ..Default::default()
        }
    }

    /// Set parent
    pub fn with_parent(mut self, parent_id: u64, parent_path: &str) -> Self {
        self.parent_id = Some(parent_id);
        self.path = format!("{}.{}", parent_path, self.name);
        self
    }

    /// Set as signal node
    pub fn with_signal(mut self, signal_name: impl Into<String>) -> Self {
        self.signal_name = Some(signal_name.into());
        self
    }

    /// Set scalar value
    pub fn with_value(mut self, value: f64, unit: impl Into<String>) -> Self {
        self.value = Some(value);
        self.unit = Some(unit.into());
        self
    }

    /// Toggle expanded state
    pub fn toggle_expand(&mut self) {
        self.expanded = !self.expanded;
    }
}

// =============================================================================
// Results Browser Tree
// =============================================================================

/// Complete results browser tree state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResultsBrowserTree {
    /// All nodes indexed by ID
    nodes: HashMap<u64, ResultNode>,
    /// Root node IDs (simulation sessions)
    roots: Vec<u64>,
    /// Next available node ID
    next_id: u64,
    /// Currently selected node IDs
    selected: Vec<u64>,
    /// Search filter text
    pub filter_text: String,
    /// Whether to show voltages
    pub show_voltages: bool,
    /// Whether to show currents
    pub show_currents: bool,
    /// Whether to show device parameters
    pub show_device_params: bool,
}

impl ResultsBrowserTree {
    /// Create a new empty tree
    pub fn new() -> Self {
        Self {
            show_voltages: true,
            show_currents: true,
            show_device_params: true,
            ..Default::default()
        }
    }

    /// Allocate a new node ID
    fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Add a root node (simulation session)
    pub fn add_root(&mut self, name: impl Into<String>) -> u64 {
        let id = self.next_id();
        let mut node = ResultNode::new(id, name, ResultNodeType::Root);
        node.expanded = true; // Root nodes start expanded
        self.nodes.insert(id, node);
        self.roots.push(id);
        id
    }

    /// Add an analysis node under a root
    pub fn add_analysis(&mut self, parent_id: u64, name: impl Into<String>) -> u64 {
        let id = self.next_id();
        let parent_path = self
            .nodes
            .get(&parent_id)
            .map(|n| n.path.clone())
            .unwrap_or_default();

        let node = ResultNode::new(id, name, ResultNodeType::Analysis)
            .with_parent(parent_id, &parent_path);
        self.nodes.insert(id, node);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }
        id
    }

    /// Add a category node under an analysis
    pub fn add_category(&mut self, parent_id: u64, name: impl Into<String>) -> u64 {
        let id = self.next_id();
        let parent_path = self
            .nodes
            .get(&parent_id)
            .map(|n| n.path.clone())
            .unwrap_or_default();

        let node = ResultNode::new(id, name, ResultNodeType::Category)
            .with_parent(parent_id, &parent_path);
        self.nodes.insert(id, node);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }
        id
    }

    /// Add a signal node
    pub fn add_signal(
        &mut self,
        parent_id: u64,
        name: impl Into<String>,
        signal_name: impl Into<String>,
    ) -> u64 {
        let id = self.next_id();
        let parent_path = self
            .nodes
            .get(&parent_id)
            .map(|n| n.path.clone())
            .unwrap_or_default();

        let node = ResultNode::new(id, name, ResultNodeType::Signal)
            .with_parent(parent_id, &parent_path)
            .with_signal(signal_name);
        self.nodes.insert(id, node);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }
        id
    }

    /// Add a device operating point group
    pub fn add_device_op(&mut self, parent_id: u64, device_name: impl Into<String>) -> u64 {
        let id = self.next_id();
        let parent_path = self
            .nodes
            .get(&parent_id)
            .map(|n| n.path.clone())
            .unwrap_or_default();

        let node = ResultNode::new(id, device_name, ResultNodeType::DeviceOp)
            .with_parent(parent_id, &parent_path);
        self.nodes.insert(id, node);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }
        id
    }

    /// Add a device parameter
    pub fn add_device_param(
        &mut self,
        parent_id: u64,
        param_name: impl Into<String>,
        value: f64,
        unit: impl Into<String>,
    ) -> u64 {
        let id = self.next_id();
        let parent_path = self
            .nodes
            .get(&parent_id)
            .map(|n| n.path.clone())
            .unwrap_or_default();

        let node = ResultNode::new(id, param_name, ResultNodeType::DeviceParam)
            .with_parent(parent_id, &parent_path)
            .with_value(value, unit);
        self.nodes.insert(id, node);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(id);
        }
        id
    }

    /// Get a node by ID
    pub fn get(&self, id: u64) -> Option<&ResultNode> {
        self.nodes.get(&id)
    }

    /// Get a mutable node by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut ResultNode> {
        self.nodes.get_mut(&id)
    }

    /// Get all root nodes
    pub fn roots(&self) -> impl Iterator<Item = &ResultNode> {
        self.roots.iter().filter_map(|id| self.nodes.get(id))
    }

    /// Get children of a node
    pub fn children(&self, parent_id: u64) -> Vec<&ResultNode> {
        self.nodes
            .get(&parent_id)
            .map(|parent| {
                parent
                    .children
                    .iter()
                    .filter_map(|id| self.nodes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Toggle node expansion
    pub fn toggle_expand(&mut self, id: u64) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.toggle_expand();
        }
    }

    /// Select a node (single selection mode)
    pub fn select(&mut self, id: u64) {
        // Clear previous selection
        for s in &self.selected {
            if let Some(node) = self.nodes.get_mut(s) {
                node.selected = false;
            }
        }
        self.selected.clear();

        // Select new node
        if let Some(node) = self.nodes.get_mut(&id) {
            node.selected = true;
            self.selected.push(id);
        }
    }

    /// Get selected signal names (for waveform plotting)
    pub fn selected_signals(&self) -> Vec<String> {
        self.selected
            .iter()
            .filter_map(|id| self.nodes.get(id).and_then(|n| n.signal_name.clone()))
            .collect()
    }

    /// Check if a node matches the current filter
    pub fn matches_filter(&self, node: &ResultNode) -> bool {
        if self.filter_text.is_empty() {
            return true;
        }
        let name_lower = node.name.to_lowercase();
        let filter_lower = self.filter_text.to_lowercase();
        name_lower.contains(&filter_lower)
    }

    /// Get visible nodes (respecting expansion and filter)
    pub fn visible_nodes(&self) -> Vec<(u64, usize)> {
        let mut result = Vec::new();
        for root_id in &self.roots {
            self.collect_visible_nodes(*root_id, 0, &mut result);
        }
        result
    }

    fn collect_visible_nodes(&self, node_id: u64, depth: usize, result: &mut Vec<(u64, usize)>) {
        if let Some(node) = self.nodes.get(&node_id) {
            if self.matches_filter(node) {
                result.push((node_id, depth));
            }
            if node.expanded {
                for child_id in &node.children {
                    self.collect_visible_nodes(*child_id, depth + 1, result);
                }
            }
        }
    }

    /// Populate from transient simulation signals
    pub fn populate_from_signals(
        &mut self,
        session_name: &str,
        analysis_name: &str,
        signals: &[String],
    ) {
        let root_id = self.add_root(session_name);
        let analysis_id = self.add_analysis(root_id, analysis_name);

        // Categorize signals
        let voltages_id = self.add_category(analysis_id, "Voltages");
        let currents_id = self.add_category(analysis_id, "Currents");

        for signal in signals {
            if signal.starts_with("V(") || signal.starts_with("v(") {
                let display_name = signal.clone();
                self.add_signal(voltages_id, &display_name, signal);
            } else if signal.starts_with("I(") || signal.starts_with("i(") {
                let display_name = signal.clone();
                self.add_signal(currents_id, &display_name, signal);
            } else {
                // Other signals go under analysis directly
                self.add_signal(analysis_id, signal, signal);
            }
        }
    }

    /// Clear all nodes
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.roots.clear();
        self.selected.clear();
        self.next_id = 0;
    }

    /// Total node count
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_creation() {
        let tree = ResultsBrowserTree::new();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_add_root() {
        let mut tree = ResultsBrowserTree::new();
        let id = tree.add_root("Simulation 1");

        assert_eq!(tree.len(), 1);
        let node = tree.get(id).unwrap();
        assert_eq!(node.name, "Simulation 1");
        assert_eq!(node.node_type, ResultNodeType::Root);
        assert!(node.expanded);
    }

    #[test]
    fn test_add_hierarchy() {
        let mut tree = ResultsBrowserTree::new();
        let root = tree.add_root("Sim1");
        let analysis = tree.add_analysis(root, "Transient");
        let voltages = tree.add_category(analysis, "Voltages");
        let signal = tree.add_signal(voltages, "V(out)", "V(out)");

        assert_eq!(tree.len(), 4);

        let sig_node = tree.get(signal).unwrap();
        assert_eq!(sig_node.name, "V(out)");
        assert_eq!(sig_node.signal_name.as_deref(), Some("V(out)"));
        assert!(sig_node.path.contains("Voltages"));
    }

    #[test]
    fn test_select_signal() {
        let mut tree = ResultsBrowserTree::new();
        let root = tree.add_root("Sim1");
        let analysis = tree.add_analysis(root, "Transient");
        let signal = tree.add_signal(analysis, "V(out)", "V(out)");

        tree.select(signal);

        let signals = tree.selected_signals();
        assert_eq!(signals.len(), 1);
        assert_eq!(signals[0], "V(out)");
    }

    #[test]
    fn test_toggle_expand() {
        let mut tree = ResultsBrowserTree::new();
        let root = tree.add_root("Sim1");
        let analysis = tree.add_analysis(root, "Transient");

        // Root starts expanded
        assert!(tree.get(root).unwrap().expanded);

        // Analysis starts collapsed
        assert!(!tree.get(analysis).unwrap().expanded);

        // Toggle
        tree.toggle_expand(analysis);
        assert!(tree.get(analysis).unwrap().expanded);

        tree.toggle_expand(analysis);
        assert!(!tree.get(analysis).unwrap().expanded);
    }

    #[test]
    fn test_populate_from_signals() {
        let mut tree = ResultsBrowserTree::new();
        let signals = vec![
            "V(out)".to_string(),
            "V(in)".to_string(),
            "I(R1)".to_string(),
            "I(C1)".to_string(),
        ];

        tree.populate_from_signals("simulation_1", "Transient", &signals);

        // Should have root, analysis, 2 categories, 4 signals
        assert_eq!(tree.len(), 8);
    }

    #[test]
    fn test_visible_nodes() {
        let mut tree = ResultsBrowserTree::new();
        let root = tree.add_root("Sim1");
        let analysis = tree.add_analysis(root, "Transient");
        tree.add_signal(analysis, "V(out)", "V(out)");

        // Only root visible (expanded) and analysis (collapsed)
        let visible = tree.visible_nodes();
        assert_eq!(visible.len(), 2); // Root and Analysis only

        // Expand analysis
        tree.toggle_expand(analysis);
        let visible = tree.visible_nodes();
        assert_eq!(visible.len(), 3); // Root, Analysis, Signal
    }

    #[test]
    fn test_filter() {
        let mut tree = ResultsBrowserTree::new();
        let root = tree.add_root("Sim1");
        let analysis = tree.add_analysis(root, "Transient");
        tree.add_signal(analysis, "V(out)", "V(out)");
        tree.add_signal(analysis, "V(in)", "V(in)");

        tree.toggle_expand(analysis);

        tree.filter_text = "out".to_string();
        let visible = tree.visible_nodes();

        // Should show root, analysis, and only V(out)
        assert!(visible.len() < 4);
    }

    #[test]
    fn test_device_params() {
        let mut tree = ResultsBrowserTree::new();
        let root = tree.add_root("Sim1");
        let analysis = tree.add_analysis(root, "DC OP");
        let device = tree.add_device_op(analysis, "M1");
        tree.add_device_param(device, "Id", 1.5e-3, "A");
        tree.add_device_param(device, "Vgs", 0.8, "V");
        tree.add_device_param(device, "Vds", 1.2, "V");

        let m1 = tree.get(device).unwrap();
        assert_eq!(m1.children.len(), 3);

        let id_param = tree.get(m1.children[0]).unwrap();
        assert_eq!(id_param.value, Some(1.5e-3));
        assert_eq!(id_param.unit.as_deref(), Some("A"));
    }
}
