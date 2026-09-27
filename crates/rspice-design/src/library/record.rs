//! A library: a named collection of cells.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::Cell;

// Library
// =============================================================================

/// A design library containing multiple cells
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Library {
    /// Library name (e.g., "my_designs", "analog_lib")
    pub name: String,
    /// Path to library directory on disk
    pub path: Option<PathBuf>,
    /// Cells within this library
    pub cells: HashMap<String, Cell>,
    /// Library technology/PDK name
    pub technology: String,
    /// Whether this is a read-only library
    pub read_only: bool,
    /// Whether library is expanded in browser
    pub expanded: bool,
    /// Library-level metadata
    pub metadata: HashMap<String, String>,
}

impl Library {
    /// Create a new library
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Add a cell to this library
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.insert(cell.name.clone(), cell);
    }

    /// Get a cell by name
    pub fn get_cell(&self, name: &str) -> Option<&Cell> {
        self.cells.get(name)
    }

    /// Get mutable cell by name
    pub fn get_cell_mut(&mut self, name: &str) -> Option<&mut Cell> {
        self.cells.get_mut(name)
    }

    /// Get or create cell
    pub fn get_or_create_cell(&mut self, name: &str) -> &mut Cell {
        self.cells
            .entry(name.to_string())
            .or_insert_with(|| Cell::new(name))
    }

    /// Get cells sorted by name
    pub fn cells_sorted(&self) -> Vec<&Cell> {
        let mut cells: Vec<_> = self.cells.values().collect();
        cells.sort_by(|a, b| a.name.cmp(&b.name));
        cells
    }

    /// Count of cells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Total view count across all cells
    pub fn total_view_count(&self) -> usize {
        self.cells.values().map(|c| c.view_count()).sum()
    }

    /// Remove a cell by name
    pub fn remove_cell(&mut self, name: &str) -> bool {
        self.cells.remove(name).is_some()
    }
}

// =============================================================================
