//! Cross-Probe Registry
//!
//! Central signal selection registry for cross-probing coordination.

use std::collections::{HashMap, HashSet};

use super::event::{ProbeEvent, ProbeEventBus, ProbeSource};
use super::signal::{ProbeableSignal, SignalId, SignalPath, SignalType};

// =============================================================================
// Probe Target
// =============================================================================

/// Target for probing in a view
#[derive(Debug, Clone, PartialEq)]
pub enum ProbeTarget {
    /// A specific signal
    Signal(SignalId),
    /// A hierarchical node in schematic
    SchematicNode { path: SignalPath, node_name: String },
    /// A trace in waveform viewer
    WaveformTrace {
        signal_id: SignalId,
        trace_index: usize,
    },
    /// A time point
    TimePoint(f64),
}

// =============================================================================
// Registry
// =============================================================================

/// Central cross-probe registry
pub struct CrossProbeRegistry {
    /// All known signals
    signals: HashMap<SignalId, ProbeableSignal>,
    /// Currently selected signals
    selected: HashSet<SignalId>,
    /// Currently highlighted signal
    highlighted: Option<SignalId>,
    /// Event bus for broadcasting changes
    event_bus: ProbeEventBus,
    /// Current hierarchy path
    current_path: SignalPath,
    /// Cursor time position
    cursor_time: Option<f64>,
    /// Selection history (for undo)
    selection_history: Vec<HashSet<SignalId>>,
    /// Maximum history size
    max_history: usize,
}

impl Default for CrossProbeRegistry {
    fn default() -> Self {
        Self {
            signals: HashMap::new(),
            selected: HashSet::new(),
            highlighted: None,
            event_bus: ProbeEventBus::new(),
            current_path: SignalPath::new(),
            cursor_time: None,
            selection_history: Vec::new(),
            max_history: 50,
        }
    }
}

impl CrossProbeRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Get event bus for handler registration
    pub fn event_bus_mut(&mut self) -> &mut ProbeEventBus {
        &mut self.event_bus
    }

    // =========================================================================
    // Signal Registration
    // =========================================================================

    /// Register a signal
    pub fn register_signal(&mut self, signal: ProbeableSignal) {
        self.signals.insert(signal.id.clone(), signal);
    }

    /// Register multiple signals
    pub fn register_signals(&mut self, signals: Vec<ProbeableSignal>) {
        for signal in signals {
            self.register_signal(signal);
        }
    }

    /// Unregister a signal
    pub fn unregister_signal(&mut self, signal_id: &SignalId) {
        self.signals.remove(signal_id);
        self.selected.remove(signal_id);
        if self.highlighted.as_ref() == Some(signal_id) {
            self.highlighted = None;
        }
    }

    /// Get signal by ID
    pub fn get_signal(&self, signal_id: &SignalId) -> Option<&ProbeableSignal> {
        self.signals.get(signal_id)
    }

    /// Get mutable signal by ID
    pub fn get_signal_mut(&mut self, signal_id: &SignalId) -> Option<&mut ProbeableSignal> {
        self.signals.get_mut(signal_id)
    }

    /// Get signal by name
    pub fn find_signal(&self, name: &str) -> Option<&ProbeableSignal> {
        self.signals.values().find(|s| s.display_name == name)
    }

    /// Get all signals
    pub fn signals(&self) -> impl Iterator<Item = &ProbeableSignal> {
        self.signals.values()
    }

    /// Get signals matching pattern
    pub fn find_signals(&self, pattern: &str) -> Vec<&ProbeableSignal> {
        self.signals
            .values()
            .filter(|s| s.id.matches(pattern))
            .collect()
    }

    /// Get signals of a specific type
    pub fn signals_of_type(&self, signal_type: SignalType) -> Vec<&ProbeableSignal> {
        self.signals
            .values()
            .filter(|s| s.signal_type == signal_type)
            .collect()
    }

    /// Get signals under a path
    pub fn signals_in_hierarchy(&self, path: &SignalPath) -> Vec<&ProbeableSignal> {
        self.signals
            .values()
            .filter(|s| path.is_ancestor_of(&s.path) || &s.path == path)
            .collect()
    }

    /// Number of registered signals
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }

    /// Clear all signals
    pub fn clear_signals(&mut self) {
        self.signals.clear();
        self.selected.clear();
        self.highlighted = None;
    }

    // =========================================================================
    // Selection Management
    // =========================================================================

    /// Select a signal
    pub fn select(&mut self, signal_id: &SignalId, source: ProbeSource) {
        self.save_selection_history();

        if let Some(signal) = self.signals.get_mut(signal_id) {
            signal.is_selected = true;
            self.selected.insert(signal_id.clone());

            self.event_bus.broadcast(
                &ProbeEvent::SignalSelected {
                    signal_id: signal_id.clone(),
                    source,
                },
                Some(source),
            );
        }
    }

    /// Deselect a signal
    pub fn deselect(&mut self, signal_id: &SignalId, source: ProbeSource) {
        self.save_selection_history();

        if let Some(signal) = self.signals.get_mut(signal_id) {
            signal.is_selected = false;
            self.selected.remove(signal_id);

            self.event_bus.broadcast(
                &ProbeEvent::SignalDeselected {
                    signal_id: signal_id.clone(),
                    source,
                },
                Some(source),
            );
        }
    }

    /// Toggle signal selection
    pub fn toggle_selection(&mut self, signal_id: &SignalId, source: ProbeSource) {
        if self.selected.contains(signal_id) {
            self.deselect(signal_id, source);
        } else {
            self.select(signal_id, source);
        }
    }

    /// Select multiple signals
    pub fn select_multiple(&mut self, signal_ids: &[SignalId], source: ProbeSource) {
        self.save_selection_history();

        for signal_id in signal_ids {
            if let Some(signal) = self.signals.get_mut(signal_id) {
                signal.is_selected = true;
                self.selected.insert(signal_id.clone());

                self.event_bus.broadcast(
                    &ProbeEvent::SignalSelected {
                        signal_id: signal_id.clone(),
                        source,
                    },
                    Some(source),
                );
            }
        }
    }

    /// Clear all selections
    pub fn clear_selection(&mut self, source: ProbeSource) {
        self.save_selection_history();

        for signal_id in self.selected.drain().collect::<Vec<_>>() {
            if let Some(signal) = self.signals.get_mut(&signal_id) {
                signal.is_selected = false;
            }
        }

        self.event_bus
            .broadcast(&ProbeEvent::AllCleared, Some(source));
    }

    /// Get selected signals
    pub fn selected_signals(&self) -> Vec<&ProbeableSignal> {
        self.selected
            .iter()
            .filter_map(|id| self.signals.get(id))
            .collect()
    }

    /// Get selected signal IDs
    pub fn selected_ids(&self) -> &HashSet<SignalId> {
        &self.selected
    }

    /// Is signal selected?
    pub fn is_selected(&self, signal_id: &SignalId) -> bool {
        self.selected.contains(signal_id)
    }

    /// Number of selected signals
    pub fn selection_count(&self) -> usize {
        self.selected.len()
    }

    // =========================================================================
    // Highlighting
    // =========================================================================

    /// Highlight a signal (hover)
    pub fn highlight(&mut self, signal_id: &SignalId, source: ProbeSource) {
        // Clear previous highlight
        if let Some(ref prev) = self.highlighted {
            if let Some(signal) = self.signals.get_mut(prev) {
                signal.is_highlighted = false;
            }
        }

        // Set new highlight
        if let Some(signal) = self.signals.get_mut(signal_id) {
            signal.is_highlighted = true;
            self.highlighted = Some(signal_id.clone());

            self.event_bus.broadcast(
                &ProbeEvent::SignalHighlighted {
                    signal_id: signal_id.clone(),
                    source,
                },
                Some(source),
            );
        }
    }

    /// Clear highlight
    pub fn clear_highlight(&mut self, source: ProbeSource) {
        if let Some(ref signal_id) = self.highlighted.take() {
            if let Some(signal) = self.signals.get_mut(signal_id) {
                signal.is_highlighted = false;
            }

            self.event_bus.broadcast(
                &ProbeEvent::SignalUnhighlighted {
                    signal_id: signal_id.clone(),
                    source,
                },
                Some(source),
            );
        }
    }

    /// Get highlighted signal
    pub fn highlighted(&self) -> Option<&ProbeableSignal> {
        self.highlighted
            .as_ref()
            .and_then(|id| self.signals.get(id))
    }

    // =========================================================================
    // Cursor Management
    // =========================================================================

    /// Set cursor time position
    pub fn set_cursor_time(&mut self, time: f64, source: ProbeSource) {
        self.cursor_time = Some(time);
        self.event_bus
            .broadcast(&ProbeEvent::CursorMoved { time, source }, Some(source));
    }

    /// Clear cursor
    pub fn clear_cursor(&mut self) {
        self.cursor_time = None;
    }

    /// Get cursor time
    pub fn cursor_time(&self) -> Option<f64> {
        self.cursor_time
    }

    // =========================================================================
    // Hierarchy Navigation
    // =========================================================================

    /// Navigate to path
    pub fn navigate_to(&mut self, path: SignalPath) {
        self.current_path = path.clone();
        self.event_bus
            .broadcast(&ProbeEvent::HierarchyNavigated { path }, None);
    }

    /// Go up one level
    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.navigate_to(parent);
        }
    }

    /// Get current path
    pub fn current_path(&self) -> &SignalPath {
        &self.current_path
    }

    // =========================================================================
    // History Management
    // =========================================================================

    fn save_selection_history(&mut self) {
        if self.selection_history.len() >= self.max_history {
            self.selection_history.remove(0);
        }
        self.selection_history.push(self.selected.clone());
    }

    /// Undo last selection change
    pub fn undo_selection(&mut self, source: ProbeSource) {
        if let Some(prev) = self.selection_history.pop() {
            // Clear current selection
            for signal_id in &self.selected {
                if let Some(signal) = self.signals.get_mut(signal_id) {
                    signal.is_selected = false;
                }
            }

            // Restore previous selection
            self.selected = prev;
            for signal_id in &self.selected {
                if let Some(signal) = self.signals.get_mut(signal_id) {
                    signal.is_selected = true;
                    self.event_bus.broadcast(
                        &ProbeEvent::SignalSelected {
                            signal_id: signal_id.clone(),
                            source,
                        },
                        Some(source),
                    );
                }
            }
        }
    }

    /// Can undo?
    pub fn can_undo(&self) -> bool {
        !self.selection_history.is_empty()
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.selection_history.clear();
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_signal(name: &str) -> ProbeableSignal {
        ProbeableSignal::new(name)
    }

    // =========================================================================
    // ProbeTarget Tests
    // =========================================================================

    #[test]
    fn test_probe_target_signal() {
        let target = ProbeTarget::Signal(SignalId::new("v(out)"));
        match target {
            ProbeTarget::Signal(id) => assert_eq!(id.name(), "v(out)"),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_probe_target_schematic_node() {
        let target = ProbeTarget::SchematicNode {
            path: SignalPath::from_str("top.amp"),
            node_name: "out".to_string(),
        };
        match target {
            ProbeTarget::SchematicNode { path, node_name } => {
                assert_eq!(path.full_path(), "top.amp");
                assert_eq!(node_name, "out");
            }
            _ => panic!("Wrong variant"),
        }
    }

    // =========================================================================
    // Registry Creation Tests
    // =========================================================================

    #[test]
    fn test_registry_new() {
        let reg = CrossProbeRegistry::new();
        assert_eq!(reg.signal_count(), 0);
        assert_eq!(reg.selection_count(), 0);
    }

    #[test]
    fn test_registry_default() {
        let reg = CrossProbeRegistry::default();
        assert!(reg.current_path().is_root());
        assert!(reg.cursor_time().is_none());
    }

    // =========================================================================
    // Signal Registration Tests
    // =========================================================================

    #[test]
    fn test_register_signal() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");

        reg.register_signal(sig);

        assert_eq!(reg.signal_count(), 1);
    }

    #[test]
    fn test_register_multiple() {
        let mut reg = CrossProbeRegistry::new();
        reg.register_signals(vec![
            create_test_signal("v(out)"),
            create_test_signal("v(in)"),
        ]);

        assert_eq!(reg.signal_count(), 2);
    }

    #[test]
    fn test_unregister_signal() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();

        reg.register_signal(sig);
        reg.unregister_signal(&id);

        assert_eq!(reg.signal_count(), 0);
    }

    #[test]
    fn test_get_signal() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();

        reg.register_signal(sig);

        let found = reg.get_signal(&id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().display_name, "v(out)");
    }

    #[test]
    fn test_find_signal_by_name() {
        let mut reg = CrossProbeRegistry::new();
        reg.register_signal(create_test_signal("v(out)"));

        let found = reg.find_signal("v(out)");
        assert!(found.is_some());
    }

    #[test]
    fn test_find_signals_by_pattern() {
        let mut reg = CrossProbeRegistry::new();
        reg.register_signal(create_test_signal("v(out)"));
        reg.register_signal(create_test_signal("v(in)"));
        reg.register_signal(create_test_signal("i(R1)"));

        let found = reg.find_signals("v(*");
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn test_signals_of_type() {
        let mut reg = CrossProbeRegistry::new();
        reg.register_signal(create_test_signal("v(out)"));
        reg.register_signal(create_test_signal("i(R1)"));

        let voltages = reg.signals_of_type(SignalType::Voltage);
        assert_eq!(voltages.len(), 1);
    }

    // =========================================================================
    // Selection Tests
    // =========================================================================

    #[test]
    fn test_select() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();
        reg.register_signal(sig);

        reg.select(&id, ProbeSource::Browser);

        assert!(reg.is_selected(&id));
        assert_eq!(reg.selection_count(), 1);
    }

    #[test]
    fn test_deselect() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();
        reg.register_signal(sig);

        reg.select(&id, ProbeSource::Browser);
        reg.deselect(&id, ProbeSource::Browser);

        assert!(!reg.is_selected(&id));
        assert_eq!(reg.selection_count(), 0);
    }

    #[test]
    fn test_toggle_selection() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();
        reg.register_signal(sig);

        reg.toggle_selection(&id, ProbeSource::Browser);
        assert!(reg.is_selected(&id));

        reg.toggle_selection(&id, ProbeSource::Browser);
        assert!(!reg.is_selected(&id));
    }

    #[test]
    fn test_select_multiple() {
        let mut reg = CrossProbeRegistry::new();
        let sig1 = create_test_signal("v(out)");
        let sig2 = create_test_signal("v(in)");
        let id1 = sig1.id.clone();
        let id2 = sig2.id.clone();
        reg.register_signals(vec![sig1, sig2]);

        reg.select_multiple(&[id1.clone(), id2.clone()], ProbeSource::Browser);

        assert_eq!(reg.selection_count(), 2);
    }

    #[test]
    fn test_clear_selection() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();
        reg.register_signal(sig);
        reg.select(&id, ProbeSource::Browser);

        reg.clear_selection(ProbeSource::Browser);

        assert_eq!(reg.selection_count(), 0);
    }

    #[test]
    fn test_selected_signals() {
        let mut reg = CrossProbeRegistry::new();
        let sig1 = create_test_signal("v(out)");
        let sig2 = create_test_signal("v(in)");
        let id1 = sig1.id.clone();
        reg.register_signals(vec![sig1, sig2]);
        reg.select(&id1, ProbeSource::Browser);

        let selected = reg.selected_signals();
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].display_name, "v(out)");
    }

    // =========================================================================
    // Highlighting Tests
    // =========================================================================

    #[test]
    fn test_highlight() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();
        reg.register_signal(sig);

        reg.highlight(&id, ProbeSource::Schematic);

        assert!(reg.highlighted().is_some());
        assert_eq!(reg.highlighted().unwrap().display_name, "v(out)");
    }

    #[test]
    fn test_highlight_replaces_previous() {
        let mut reg = CrossProbeRegistry::new();
        let sig1 = create_test_signal("v(out)");
        let sig2 = create_test_signal("v(in)");
        let id1 = sig1.id.clone();
        let id2 = sig2.id.clone();
        reg.register_signals(vec![sig1, sig2]);

        reg.highlight(&id1, ProbeSource::Schematic);
        reg.highlight(&id2, ProbeSource::Schematic);

        assert_eq!(reg.highlighted().unwrap().display_name, "v(in)");
        assert!(!reg.get_signal(&id1).unwrap().is_highlighted);
    }

    #[test]
    fn test_clear_highlight() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();
        reg.register_signal(sig);

        reg.highlight(&id, ProbeSource::Schematic);
        reg.clear_highlight(ProbeSource::Schematic);

        assert!(reg.highlighted().is_none());
    }

    // =========================================================================
    // Cursor Tests
    // =========================================================================

    #[test]
    fn test_set_cursor_time() {
        let mut reg = CrossProbeRegistry::new();
        reg.set_cursor_time(1.5e-9, ProbeSource::Waveform);

        assert!((reg.cursor_time().unwrap() - 1.5e-9).abs() < 1e-15);
    }

    #[test]
    fn test_clear_cursor() {
        let mut reg = CrossProbeRegistry::new();
        reg.set_cursor_time(1.5e-9, ProbeSource::Waveform);
        reg.clear_cursor();

        assert!(reg.cursor_time().is_none());
    }

    // =========================================================================
    // Hierarchy Navigation Tests
    // =========================================================================

    #[test]
    fn test_navigate_to() {
        let mut reg = CrossProbeRegistry::new();
        let path = SignalPath::from_str("top.amp");

        reg.navigate_to(path);

        assert_eq!(reg.current_path().full_path(), "top.amp");
    }

    #[test]
    fn test_navigate_up() {
        let mut reg = CrossProbeRegistry::new();
        reg.navigate_to(SignalPath::from_str("top.amp.stage1"));

        reg.navigate_up();

        assert_eq!(reg.current_path().full_path(), "top.amp");
    }

    // =========================================================================
    // History Tests
    // =========================================================================

    #[test]
    fn test_undo_selection() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();
        reg.register_signal(sig);

        reg.select(&id, ProbeSource::Browser);
        assert!(reg.can_undo());

        reg.undo_selection(ProbeSource::Browser);
        // After undo, should be back to empty selection
        // Note: undo restores previous state, which was empty
    }

    #[test]
    fn test_can_undo() {
        let reg = CrossProbeRegistry::new();
        assert!(!reg.can_undo());
    }

    #[test]
    fn test_clear_history() {
        let mut reg = CrossProbeRegistry::new();
        let sig = create_test_signal("v(out)");
        let id = sig.id.clone();
        reg.register_signal(sig);
        reg.select(&id, ProbeSource::Browser);

        reg.clear_history();

        assert!(!reg.can_undo());
    }
}
