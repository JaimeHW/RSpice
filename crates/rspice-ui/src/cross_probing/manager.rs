//! Cross-Probing Coordination
//!
//! Implements bi-directional cross-probing between schematic and waveform views.
//!
//! # Overview
//!
//! Cross-probing allows users to:
//! - Click a node/component in the schematic → highlight corresponding waveform trace
//! - Click a waveform trace → highlight corresponding net/component in schematic
//! - Hover over schematic elements → preview waveform tooltip
//! - Hover over waveform traces → preview schematic node location
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐      CrossProbeEvent       ┌─────────────────┐
//! │   Schematic     │◄──────────────────────────►│   Waveform      │
//! │   Editor        │    ProbeLink mapping       │   Viewer        │
//! └─────────────────┘                            └─────────────────┘
//!          │                    ▲                        │
//!          └──────────┬─────────┴────────────────────────┘
//!                     │
//!              CrossProbeManager
//!              (state coordination)
//! ```
//!
//! # Usage
//!
//! ```ignore
//! let mut cross_probe = CrossProbeManager::new();
//!
//! // Register a link between schematic node and waveform
//! cross_probe.register_node_signal("net1", "V(net1)");
//!
//! // When user clicks in schematic
//! cross_probe.probe_from_schematic(ProbeTarget::Node("net1".to_string()));
//!
//! // Get highlighted waveforms
//! let highlighted = cross_probe.highlighted_waveforms();
//! ```

use std::collections::{HashMap, HashSet};

//=============================================================================
// Probe Targets
//=============================================================================

/// A target that can be probed in the schematic
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchematicTarget {
    /// A net/node in the schematic
    Node(String),
    /// A component instance
    Component(String),
    /// A specific terminal on a component
    Terminal { component: String, terminal: String },
    /// A wire segment (by ID)
    Wire(u64),
}

impl SchematicTarget {
    /// Get the primary name for this target
    pub fn name(&self) -> &str {
        match self {
            SchematicTarget::Node(n) => n,
            SchematicTarget::Component(c) => c,
            SchematicTarget::Terminal { component, .. } => component,
            SchematicTarget::Wire(_) => "wire",
        }
    }
}

/// A target that can be probed in the waveform viewer
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WaveformTarget {
    /// A voltage signal (e.g., "V(net1)")
    Voltage(String),
    /// A current signal (e.g., "I(R1)")
    Current(String),
    /// A device parameter (e.g., "Id(M1)")
    DeviceParam { device: String, param: String },
    /// An expression result (e.g., "V(out)-V(in)")
    Expression(String),
}

impl WaveformTarget {
    /// Get the signal name for display
    pub fn signal_name(&self) -> String {
        match self {
            WaveformTarget::Voltage(n) => format!("V({})", n),
            WaveformTarget::Current(n) => format!("I({})", n),
            WaveformTarget::DeviceParam { device, param } => format!("{}({})", param, device),
            WaveformTarget::Expression(e) => e.clone(),
        }
    }

    /// Parse a signal name into a WaveformTarget
    pub fn parse(signal: &str) -> Option<Self> {
        let signal = signal.trim();

        // Check for V(node) format
        if signal.to_lowercase().starts_with("v(") && signal.ends_with(')') {
            let node = &signal[2..signal.len() - 1];
            return Some(WaveformTarget::Voltage(node.to_string()));
        }

        // Check for I(device) format
        if signal.to_lowercase().starts_with("i(") && signal.ends_with(')') {
            let device = &signal[2..signal.len() - 1];
            return Some(WaveformTarget::Current(device.to_string()));
        }

        // Check for device parameter format (e.g., Id(M1), Vgs(M1))
        if let Some(paren_pos) = signal.find('(')
            && signal.ends_with(')')
        {
            let param = &signal[..paren_pos];
            let device = &signal[paren_pos + 1..signal.len() - 1];
            return Some(WaveformTarget::DeviceParam {
                device: device.to_string(),
                param: param.to_string(),
            });
        }

        // Treat as expression
        Some(WaveformTarget::Expression(signal.to_string()))
    }
}

//=============================================================================
// Probe Link
//=============================================================================

/// A link between schematic and waveform elements
#[derive(Debug, Clone)]
pub struct ProbeLink {
    /// Schematic target
    pub schematic: SchematicTarget,
    /// Associated waveform targets
    pub waveforms: Vec<WaveformTarget>,
    /// Optional color for highlighting
    pub color: Option<String>,
    /// Whether this link is currently highlighted
    pub highlighted: bool,
}

impl ProbeLink {
    /// Create a new probe link
    pub fn new(schematic: SchematicTarget) -> Self {
        Self {
            schematic,
            waveforms: Vec::new(),
            color: None,
            highlighted: false,
        }
    }

    /// Add a waveform target to this link
    pub fn with_waveform(mut self, wf: WaveformTarget) -> Self {
        self.waveforms.push(wf);
        self
    }

    /// Set the highlight color
    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
}

//=============================================================================
// Cross-Probe Event
//=============================================================================

/// Events generated by cross-probing actions
#[derive(Debug, Clone)]
pub enum CrossProbeEvent {
    /// Probe initiated from schematic view
    FromSchematic {
        target: SchematicTarget,
        /// Waveforms to highlight
        waveforms: Vec<String>,
    },
    /// Probe initiated from waveform view
    FromWaveform {
        /// Signal name that was selected
        signal: String,
        /// Schematic nodes to highlight
        nodes: Vec<String>,
        /// Components to highlight
        components: Vec<String>,
    },
    /// Clear all cross-probe highlights
    Clear,
    /// Hover preview (temporary highlight)
    HoverPreview {
        /// Source of the hover
        source: ProbeSource,
        /// Target to preview
        target: String,
    },
}

/// Source of a probe action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeSource {
    Schematic,
    Waveform,
}

//=============================================================================
// Cross-Probe Manager
//=============================================================================

/// Manages cross-probing state and coordination
#[derive(Debug, Clone, Default)]
pub struct CrossProbeManager {
    /// Map from node names to signal names
    node_to_signals: HashMap<String, Vec<String>>,
    /// Map from signal names to node names
    signal_to_nodes: HashMap<String, Vec<String>>,
    /// Map from component names to current signals
    component_to_currents: HashMap<String, Vec<String>>,
    /// Currently highlighted schematic elements
    highlighted_schematic: HashSet<String>,
    /// Currently highlighted waveform signals
    highlighted_waveforms: HashSet<String>,
    /// Pending events (to be consumed by views)
    pending_events: Vec<CrossProbeEvent>,
    /// Whether cross-probing is enabled
    enabled: bool,
}

impl CrossProbeManager {
    /// Create a new cross-probe manager
    pub fn new() -> Self {
        Self {
            node_to_signals: HashMap::new(),
            signal_to_nodes: HashMap::new(),
            component_to_currents: HashMap::new(),
            highlighted_schematic: HashSet::new(),
            highlighted_waveforms: HashSet::new(),
            pending_events: Vec::new(),
            enabled: true,
        }
    }

    /// Enable or disable cross-probing
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.clear_highlights();
        }
    }

    /// Check if cross-probing is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Register a mapping from node name to voltage signal
    pub fn register_node_signal(&mut self, node: &str, signal: &str) {
        self.node_to_signals
            .entry(node.to_string())
            .or_default()
            .push(signal.to_string());

        self.signal_to_nodes
            .entry(signal.to_string())
            .or_default()
            .push(node.to_string());
    }

    /// Register a mapping from component to current signal
    pub fn register_component_current(&mut self, component: &str, signal: &str) {
        self.component_to_currents
            .entry(component.to_string())
            .or_default()
            .push(signal.to_string());

        self.signal_to_nodes
            .entry(signal.to_string())
            .or_default()
            .push(component.to_string());
    }

    /// Auto-register signals from simulation results
    ///
    /// Parses signal names and creates appropriate mappings
    pub fn register_from_signals(&mut self, signals: &[String]) {
        for signal in signals {
            if let Some(target) = WaveformTarget::parse(signal) {
                match target {
                    WaveformTarget::Voltage(node) => {
                        self.register_node_signal(&node, signal);
                    }
                    WaveformTarget::Current(device) => {
                        self.register_component_current(&device, signal);
                    }
                    WaveformTarget::DeviceParam { device, .. } => {
                        self.register_component_current(&device, signal);
                    }
                    WaveformTarget::Expression(_) => {
                        // Expressions don't map directly to schematic elements
                    }
                }
            }
        }
    }

    /// Probe from schematic - highlight corresponding waveforms
    pub fn probe_from_schematic(&mut self, target: &SchematicTarget) {
        if !self.enabled {
            return;
        }

        // Clear previous highlights
        self.highlighted_schematic.clear();
        self.highlighted_waveforms.clear();

        match target {
            SchematicTarget::Node(node) => {
                self.highlighted_schematic.insert(node.clone());
                if let Some(signals) = self.node_to_signals.get(node) {
                    for sig in signals {
                        self.highlighted_waveforms.insert(sig.clone());
                    }
                }
            }
            SchematicTarget::Component(comp) => {
                self.highlighted_schematic.insert(comp.clone());
                if let Some(signals) = self.component_to_currents.get(comp) {
                    for sig in signals {
                        self.highlighted_waveforms.insert(sig.clone());
                    }
                }
            }
            SchematicTarget::Terminal {
                component,
                terminal,
            } => {
                // Highlight both the component and any associated signals
                self.highlighted_schematic.insert(component.clone());
                let term_node = format!("{}:{}", component, terminal);
                if let Some(signals) = self.node_to_signals.get(&term_node) {
                    for sig in signals {
                        self.highlighted_waveforms.insert(sig.clone());
                    }
                }
            }
            SchematicTarget::Wire(id) => {
                self.highlighted_schematic.insert(format!("wire_{}", id));
            }
        }

        // Generate event
        let waveforms: Vec<String> = self.highlighted_waveforms.iter().cloned().collect();
        self.pending_events.push(CrossProbeEvent::FromSchematic {
            target: target.clone(),
            waveforms,
        });
    }

    /// Probe from waveform - highlight corresponding schematic elements
    ///
    /// Toggle behavior: if the signal is already highlighted, this clears the highlight.
    /// This provides seamless UX where right-clicking a highlighted trace deselects it.
    pub fn probe_from_waveform(&mut self, signal: &str) {
        if !self.enabled {
            return;
        }

        // Toggle behavior - if this signal is already highlighted, clear all highlights
        if self.highlighted_waveforms.contains(signal) {
            self.clear_highlights();
            return;
        }

        // Clear previous highlights (select new signal)
        self.highlighted_schematic.clear();
        self.highlighted_waveforms.clear();

        // Add waveform highlight
        self.highlighted_waveforms.insert(signal.to_string());

        // Find corresponding schematic elements
        let mut nodes = Vec::new();
        let mut components = Vec::new();

        if let Some(schematic_elements) = self.signal_to_nodes.get(signal) {
            for elem in schematic_elements {
                self.highlighted_schematic.insert(elem.clone());

                // Categorize as node or component
                if elem.starts_with(|c: char| c.is_ascii_uppercase()) {
                    components.push(elem.clone());
                } else {
                    nodes.push(elem.clone());
                }
            }
        }

        // Parse signal to extract target
        if let Some(target) = WaveformTarget::parse(signal) {
            match target {
                WaveformTarget::Voltage(node) => {
                    self.highlighted_schematic.insert(node.clone());
                    nodes.push(node);
                }
                WaveformTarget::Current(device) => {
                    self.highlighted_schematic.insert(device.clone());
                    components.push(device);
                }
                WaveformTarget::DeviceParam { device, .. } => {
                    self.highlighted_schematic.insert(device.clone());
                    components.push(device);
                }
                _ => {}
            }
        }

        // Generate event
        self.pending_events.push(CrossProbeEvent::FromWaveform {
            signal: signal.to_string(),
            nodes,
            components,
        });
    }

    /// Clear all highlights
    pub fn clear_highlights(&mut self) {
        self.highlighted_schematic.clear();
        self.highlighted_waveforms.clear();
        self.pending_events.push(CrossProbeEvent::Clear);
    }

    /// Get currently highlighted schematic elements
    pub fn highlighted_schematic(&self) -> &HashSet<String> {
        &self.highlighted_schematic
    }

    /// Get currently highlighted waveform signals
    pub fn highlighted_waveforms(&self) -> &HashSet<String> {
        &self.highlighted_waveforms
    }

    /// Check if a schematic element is highlighted
    pub fn is_schematic_highlighted(&self, name: &str) -> bool {
        self.highlighted_schematic.contains(name)
    }

    /// Check if a waveform signal is highlighted
    pub fn is_waveform_highlighted(&self, signal: &str) -> bool {
        self.highlighted_waveforms.contains(signal)
    }

    /// Take pending events (consumed by views)
    pub fn take_events(&mut self) -> Vec<CrossProbeEvent> {
        std::mem::take(&mut self.pending_events)
    }

    /// Check if there are pending events
    pub fn has_pending_events(&self) -> bool {
        !self.pending_events.is_empty()
    }

    /// Clear all registered mappings
    pub fn clear_mappings(&mut self) {
        self.node_to_signals.clear();
        self.signal_to_nodes.clear();
        self.component_to_currents.clear();
        self.clear_highlights();
    }

    /// Get signals associated with a node
    pub fn signals_for_node(&self, node: &str) -> Option<&Vec<String>> {
        self.node_to_signals.get(node)
    }

    /// Get nodes associated with a signal
    pub fn nodes_for_signal(&self, signal: &str) -> Option<&Vec<String>> {
        self.signal_to_nodes.get(signal)
    }

    /// Get number of registered mappings
    pub fn mapping_count(&self) -> usize {
        self.node_to_signals.len() + self.component_to_currents.len()
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========== WaveformTarget Tests ===========

    #[test]
    fn test_parse_voltage_signal() {
        let target = WaveformTarget::parse("V(net1)").unwrap();
        assert!(matches!(target, WaveformTarget::Voltage(n) if n == "net1"));

        let target = WaveformTarget::parse("v(OUT)").unwrap();
        assert!(matches!(target, WaveformTarget::Voltage(n) if n == "OUT"));
    }

    #[test]
    fn test_parse_current_signal() {
        let target = WaveformTarget::parse("I(R1)").unwrap();
        assert!(matches!(target, WaveformTarget::Current(d) if d == "R1"));

        let target = WaveformTarget::parse("i(M1)").unwrap();
        assert!(matches!(target, WaveformTarget::Current(d) if d == "M1"));
    }

    #[test]
    fn test_parse_device_param() {
        let target = WaveformTarget::parse("Id(M1)").unwrap();
        if let WaveformTarget::DeviceParam { device, param } = target {
            assert_eq!(device, "M1");
            assert_eq!(param, "Id");
        } else {
            panic!("Expected DeviceParam");
        }
    }

    #[test]
    fn test_signal_name() {
        let v = WaveformTarget::Voltage("net1".to_string());
        assert_eq!(v.signal_name(), "V(net1)");

        let i = WaveformTarget::Current("R1".to_string());
        assert_eq!(i.signal_name(), "I(R1)");
    }

    // =========== SchematicTarget Tests ===========

    #[test]
    fn test_schematic_target_name() {
        let node = SchematicTarget::Node("net1".to_string());
        assert_eq!(node.name(), "net1");

        let comp = SchematicTarget::Component("R1".to_string());
        assert_eq!(comp.name(), "R1");
    }

    // =========== ProbeLink Tests ===========

    #[test]
    fn test_probe_link_creation() {
        let link = ProbeLink::new(SchematicTarget::Node("net1".to_string()))
            .with_waveform(WaveformTarget::Voltage("net1".to_string()))
            .with_color("#ff0000".to_string());

        assert_eq!(link.waveforms.len(), 1);
        assert_eq!(link.color, Some("#ff0000".to_string()));
        assert!(!link.highlighted);
    }

    // =========== CrossProbeManager Tests ===========

    #[test]
    fn test_manager_creation() {
        let manager = CrossProbeManager::new();
        assert!(manager.is_enabled());
        assert!(manager.highlighted_schematic().is_empty());
        assert!(manager.highlighted_waveforms().is_empty());
    }

    #[test]
    fn test_register_node_signal() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("net1", "V(net1)");

        assert!(manager.signals_for_node("net1").is_some());
        assert_eq!(
            manager.signals_for_node("net1").unwrap(),
            &vec!["V(net1)".to_string()]
        );
    }

    #[test]
    fn test_probe_from_schematic() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("out", "V(out)");

        manager.probe_from_schematic(&SchematicTarget::Node("out".to_string()));

        assert!(manager.is_schematic_highlighted("out"));
        assert!(manager.is_waveform_highlighted("V(out)"));
        assert!(manager.has_pending_events());
    }

    #[test]
    fn test_probe_from_waveform() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("in", "V(in)");

        manager.probe_from_waveform("V(in)");

        assert!(manager.is_waveform_highlighted("V(in)"));
        assert!(manager.is_schematic_highlighted("in"));
    }

    #[test]
    fn test_clear_highlights() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("test", "V(test)");
        manager.probe_from_schematic(&SchematicTarget::Node("test".to_string()));

        assert!(!manager.highlighted_schematic().is_empty());

        manager.clear_highlights();

        assert!(manager.highlighted_schematic().is_empty());
        assert!(manager.highlighted_waveforms().is_empty());
    }

    #[test]
    fn test_disabled_manager() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("test", "V(test)");
        manager.set_enabled(false);

        manager.probe_from_schematic(&SchematicTarget::Node("test".to_string()));

        // Should not highlight when disabled
        assert!(manager.highlighted_schematic().is_empty());
    }

    #[test]
    fn test_auto_register_signals() {
        let mut manager = CrossProbeManager::new();
        let signals = vec![
            "V(net1)".to_string(),
            "V(net2)".to_string(),
            "I(R1)".to_string(),
            "Id(M1)".to_string(),
        ];

        manager.register_from_signals(&signals);

        assert!(manager.signals_for_node("net1").is_some());
        assert!(manager.signals_for_node("net2").is_some());
        assert_eq!(manager.mapping_count(), 4); // 2 nodes + 2 components
    }

    #[test]
    fn test_component_current_mapping() {
        let mut manager = CrossProbeManager::new();
        manager.register_component_current("R1", "I(R1)");

        manager.probe_from_schematic(&SchematicTarget::Component("R1".to_string()));

        assert!(manager.is_waveform_highlighted("I(R1)"));
    }

    #[test]
    fn test_take_events() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("x", "V(x)");

        manager.probe_from_schematic(&SchematicTarget::Node("x".to_string()));

        let events = manager.take_events();
        assert_eq!(events.len(), 1);

        // Events should be cleared after take
        assert!(!manager.has_pending_events());
    }

    #[test]
    fn test_clear_mappings() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("a", "V(a)");
        manager.register_node_signal("b", "V(b)");

        assert_eq!(manager.mapping_count(), 2);

        manager.clear_mappings();

        assert_eq!(manager.mapping_count(), 0);
    }

    #[test]
    fn test_multiple_signals_per_node() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("out", "V(out)");
        manager.register_node_signal("out", "V(out)-V(in)");

        manager.probe_from_schematic(&SchematicTarget::Node("out".to_string()));

        assert!(manager.is_waveform_highlighted("V(out)"));
        assert!(manager.is_waveform_highlighted("V(out)-V(in)"));
    }

    #[test]
    fn test_terminal_probing() {
        let mut manager = CrossProbeManager::new();
        manager.register_node_signal("M1:gate", "V(gate)");

        manager.probe_from_schematic(&SchematicTarget::Terminal {
            component: "M1".to_string(),
            terminal: "gate".to_string(),
        });

        assert!(manager.is_schematic_highlighted("M1"));
    }
}
