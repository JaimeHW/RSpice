use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{View, ViewType};

// Cell
// =============================================================================

/// A design cell containing multiple views
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cell {
    /// Cell name (e.g., "opamp", "bandgap")
    pub name: String,
    /// Views within this cell
    pub views: HashMap<String, View>,
    /// Cell description/documentation
    pub description: String,
    /// Cell category (e.g., "analog", "digital", "mixed")
    pub category: String,
    /// Whether cell is expanded in browser
    pub expanded: bool,
    /// Cell-level metadata
    pub metadata: HashMap<String, String>,
}

impl Cell {
    /// Create a new cell
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Add a view to this cell
    pub fn add_view(&mut self, view: View) {
        self.views.insert(view.name.clone(), view);
    }

    /// Get a view by name
    pub fn get_view(&self, name: &str) -> Option<&View> {
        self.views.get(name)
    }

    /// Get mutable view by name
    pub fn get_view_mut(&mut self, name: &str) -> Option<&mut View> {
        self.views.get_mut(name)
    }

    /// Check if cell has a specific view type
    pub fn has_view_type(&self, view_type: ViewType) -> bool {
        self.views.values().any(|v| v.view_type == view_type)
    }

    /// Get views sorted by name
    pub fn views_sorted(&self) -> Vec<&View> {
        let mut views: Vec<_> = self.views.values().collect();
        views.sort_by(|a, b| a.name.cmp(&b.name));
        views
    }

    /// Count of views
    pub fn view_count(&self) -> usize {
        self.views.len()
    }

    /// Remove a view by name
    pub fn remove_view(&mut self, name: &str) -> bool {
        self.views.remove(name).is_some()
    }
}

// =============================================================================
