//! Selection Management
//!
//! Tracks selected components and wires in the schematic.

use serde::{Deserialize, Serialize};

// =============================================================================
// Selection
// =============================================================================

/// Selection state for schematic elements
///
/// Tracks which components and wires are currently selected.
/// Supports multi-selection for batch operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Selection {
    /// Selected component IDs
    pub components: Vec<u64>,

    /// Selected wire IDs
    pub wires: Vec<u64>,
}

impl Selection {
    /// Create an empty selection
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if selection is empty (no items selected)
    pub fn is_empty(&self) -> bool {
        self.components.is_empty() && self.wires.is_empty()
    }

    /// Get total number of selected items
    pub fn count(&self) -> usize {
        self.components.len() + self.wires.len()
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.components.clear();
        self.wires.clear();
    }

    /// Check if a component is selected
    pub fn has_component(&self, id: u64) -> bool {
        self.components.contains(&id)
    }

    /// Check if a wire is selected
    pub fn has_wire(&self, id: u64) -> bool {
        self.wires.contains(&id)
    }

    /// Select a component (if not already selected)
    pub fn select_component(&mut self, id: u64) {
        if !self.has_component(id) {
            self.components.push(id);
        }
    }

    /// Select a wire (if not already selected)
    pub fn select_wire(&mut self, id: u64) {
        if !self.has_wire(id) {
            self.wires.push(id);
        }
    }

    /// Deselect a component
    pub fn deselect_component(&mut self, id: u64) {
        self.components.retain(|&c| c != id);
    }

    /// Deselect a wire
    pub fn deselect_wire(&mut self, id: u64) {
        self.wires.retain(|&w| w != id);
    }

    /// Toggle component selection
    pub fn toggle_component(&mut self, id: u64) {
        if self.has_component(id) {
            self.deselect_component(id);
        } else {
            self.select_component(id);
        }
    }

    /// Toggle wire selection
    pub fn toggle_wire(&mut self, id: u64) {
        if self.has_wire(id) {
            self.deselect_wire(id);
        } else {
            self.select_wire(id);
        }
    }

    /// Select only a single component (clears other selections)
    pub fn select_only_component(&mut self, id: u64) {
        self.clear();
        self.components.push(id);
    }

    /// Select only a single wire (clears other selections)
    pub fn select_only_wire(&mut self, id: u64) {
        self.clear();
        self.wires.push(id);
    }

    /// Get the single selected component ID (if exactly one is selected)
    pub fn single_component(&self) -> Option<u64> {
        if self.components.len() == 1 && self.wires.is_empty() {
            Some(self.components[0])
        } else {
            None
        }
    }

    /// Get the single selected wire ID (if exactly one is selected)
    pub fn single_wire(&self) -> Option<u64> {
        if self.wires.len() == 1 && self.components.is_empty() {
            Some(self.wires[0])
        } else {
            None
        }
    }

    /// Check if selection contains multiple items
    pub fn is_multi_selection(&self) -> bool {
        self.count() > 1
    }

    /// Check if only components are selected (no wires)
    pub fn only_components(&self) -> bool {
        !self.components.is_empty() && self.wires.is_empty()
    }

    /// Check if only wires are selected (no components)
    pub fn only_wires(&self) -> bool {
        self.components.is_empty() && !self.wires.is_empty()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_new() {
        let sel = Selection::new();
        assert!(sel.is_empty());
        assert_eq!(sel.count(), 0);
    }

    #[test]
    fn test_selection_add_component() {
        let mut sel = Selection::new();
        sel.select_component(1);
        assert!(!sel.is_empty());
        assert!(sel.has_component(1));
        assert_eq!(sel.count(), 1);
    }

    #[test]
    fn test_selection_add_wire() {
        let mut sel = Selection::new();
        sel.select_wire(5);
        assert!(!sel.is_empty());
        assert!(sel.has_wire(5));
        assert_eq!(sel.count(), 1);
    }

    #[test]
    fn test_selection_no_duplicates() {
        let mut sel = Selection::new();
        sel.select_component(1);
        sel.select_component(1); // Duplicate
        assert_eq!(sel.components.len(), 1);
    }

    #[test]
    fn test_selection_clear() {
        let mut sel = Selection::new();
        sel.select_component(1);
        sel.select_wire(2);
        sel.clear();
        assert!(sel.is_empty());
    }

    #[test]
    fn test_selection_deselect() {
        let mut sel = Selection::new();
        sel.select_component(1);
        sel.select_component(2);
        sel.deselect_component(1);
        assert!(!sel.has_component(1));
        assert!(sel.has_component(2));
    }

    #[test]
    fn test_selection_toggle() {
        let mut sel = Selection::new();
        sel.toggle_component(1);
        assert!(sel.has_component(1));
        sel.toggle_component(1);
        assert!(!sel.has_component(1));
    }

    #[test]
    fn test_selection_select_only() {
        let mut sel = Selection::new();
        sel.select_component(1);
        sel.select_component(2);
        sel.select_wire(3);
        sel.select_only_component(5);
        assert_eq!(sel.count(), 1);
        assert!(sel.has_component(5));
    }

    #[test]
    fn test_selection_single_component() {
        let mut sel = Selection::new();
        sel.select_component(7);
        assert_eq!(sel.single_component(), Some(7));

        sel.select_component(8);
        assert_eq!(sel.single_component(), None); // Multiple selected
    }

    #[test]
    fn test_selection_single_wire() {
        let mut sel = Selection::new();
        sel.select_wire(3);
        assert_eq!(sel.single_wire(), Some(3));

        sel.select_component(1);
        assert_eq!(sel.single_wire(), None); // Mixed selection
    }

    #[test]
    fn test_selection_is_multi() {
        let mut sel = Selection::new();
        sel.select_component(1);
        assert!(!sel.is_multi_selection());

        sel.select_wire(2);
        assert!(sel.is_multi_selection());
    }

    #[test]
    fn test_selection_only_types() {
        let mut sel = Selection::new();
        sel.select_component(1);
        assert!(sel.only_components());
        assert!(!sel.only_wires());

        sel.select_wire(2);
        assert!(!sel.only_components());
        assert!(!sel.only_wires());
    }
}
