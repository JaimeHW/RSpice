//! History/Undo-Redo System
//!
//! Implements command pattern for schematic state mutations
//! with undo/redo stack support.

use std::collections::VecDeque;

/// Generic history state with undo/redo capability
#[derive(Debug, Clone)]
pub struct History<T: Clone> {
    /// Past states (undo stack)
    past: VecDeque<T>,
    /// Current state
    present: T,
    /// Future states (redo stack)
    future: Vec<T>,
    /// Maximum number of past states to keep
    max_size: usize,
}

impl<T: Clone> History<T> {
    /// Create a new history with initial state
    pub fn new(initial: T, max_size: usize) -> Self {
        Self {
            past: VecDeque::new(),
            present: initial,
            future: Vec::new(),
            max_size,
        }
    }

    /// Get current state
    pub fn current(&self) -> &T {
        &self.present
    }

    /// Get mutable current state (for direct mutations without history)
    pub fn current_mut(&mut self) -> &mut T {
        &mut self.present
    }

    /// Push a new state, clearing redo stack
    pub fn push(&mut self, new_state: T) {
        // Move current to past
        self.past.push_back(self.present.clone());

        // Enforce max size
        while self.past.len() > self.max_size {
            self.past.pop_front();
        }

        // Set new present
        self.present = new_state;

        // Clear future (can't redo after new action)
        self.future.clear();
    }

    /// Undo: move current to future, pop past to current
    pub fn undo(&mut self) -> bool {
        if let Some(past_state) = self.past.pop_back() {
            self.future
                .push(std::mem::replace(&mut self.present, past_state));
            true
        } else {
            false
        }
    }

    /// Redo: move current to past, pop future to current
    pub fn redo(&mut self) -> bool {
        if let Some(future_state) = self.future.pop() {
            self.past
                .push_back(std::mem::replace(&mut self.present, future_state));
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.past.is_empty()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.future.is_empty()
    }

    /// Get number of undo steps available
    pub fn undo_count(&self) -> usize {
        self.past.len()
    }

    /// Get number of redo steps available
    pub fn redo_count(&self) -> usize {
        self.future.len()
    }

    /// Clear all history, keeping only current state
    pub fn clear(&mut self) {
        self.past.clear();
        self.future.clear();
    }

    /// Replace current state without affecting history
    /// Useful for non-undoable changes like view state
    pub fn replace_current(&mut self, new_state: T) {
        self.present = new_state;
    }
}

/// Schematic-specific history that tracks operation descriptions
#[derive(Debug, Clone)]
pub struct SchematicHistory<T: Clone> {
    history: History<T>,
    /// Descriptions of past operations (for UI display)
    past_descriptions: VecDeque<String>,
    /// Descriptions of future operations (for UI display)
    future_descriptions: Vec<String>,
}

impl<T: Clone> SchematicHistory<T> {
    pub fn new(initial: T, max_size: usize) -> Self {
        Self {
            history: History::new(initial, max_size),
            past_descriptions: VecDeque::new(),
            future_descriptions: Vec::new(),
        }
    }

    pub fn current(&self) -> &T {
        self.history.current()
    }

    pub fn current_mut(&mut self) -> &mut T {
        self.history.current_mut()
    }

    /// Push a new state with description
    pub fn push(&mut self, new_state: T, description: impl Into<String>) {
        let desc = description.into();

        // Push state
        self.history.push(new_state);

        // Track description
        self.past_descriptions.push_back(desc);
        while self.past_descriptions.len() > self.history.max_size {
            self.past_descriptions.pop_front();
        }

        self.future_descriptions.clear();
    }

    pub fn undo(&mut self) -> bool {
        if self.history.undo() {
            if let Some(desc) = self.past_descriptions.pop_back() {
                self.future_descriptions.push(desc);
            }
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if self.history.redo() {
            if let Some(desc) = self.future_descriptions.pop() {
                self.past_descriptions.push_back(desc);
            }
            true
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Get description of last undoable operation
    pub fn undo_description(&self) -> Option<&str> {
        self.past_descriptions.back().map(|s| s.as_str())
    }

    /// Get description of next redoable operation
    pub fn redo_description(&self) -> Option<&str> {
        self.future_descriptions.last().map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_undo_redo() {
        let mut h = History::new(0, 10);

        h.push(1);
        h.push(2);
        h.push(3);

        assert_eq!(*h.current(), 3);

        assert!(h.undo());
        assert_eq!(*h.current(), 2);

        assert!(h.undo());
        assert_eq!(*h.current(), 1);

        assert!(h.redo());
        assert_eq!(*h.current(), 2);

        // New action clears redo stack
        h.push(10);
        assert_eq!(*h.current(), 10);
        assert!(!h.can_redo());
    }

    #[test]
    fn test_max_size() {
        let mut h = History::new(0, 3);

        h.push(1);
        h.push(2);
        h.push(3);
        h.push(4);

        // Only 3 undo steps should be available
        assert_eq!(h.undo_count(), 3);
    }

    #[test]
    fn test_schematic_history() {
        let mut h = SchematicHistory::new("initial", 10);

        h.push("after add R1", "Add Resistor R1");
        h.push("after add C1", "Add Capacitor C1");

        assert_eq!(h.undo_description(), Some("Add Capacitor C1"));

        h.undo();
        assert_eq!(h.redo_description(), Some("Add Capacitor C1"));
        assert_eq!(h.undo_description(), Some("Add Resistor R1"));
    }
}
