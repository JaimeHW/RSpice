use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Cell;

/// A library is a collection of cells, typically representing a project or IP.
///
/// Libraries can be:
/// - Project libraries (user's own designs)
/// - Reference libraries (vendor IP, standard cells)
/// - Technology libraries (primitives for a process)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    /// Unique library name
    pub name: String,
    /// Library path on disk
    pub path: Option<String>,
    /// Cells in this library
    pub cells: HashMap<String, Cell>,
    /// Library type/category
    pub library_type: LibraryType,
    /// Library metadata
    pub metadata: LibraryMetadata,
    /// Whether library is read-only
    pub read_only: bool,
}

/// Library type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LibraryType {
    /// User's project library
    #[default]
    Project,
    /// Reference/IP library
    Reference,
    /// Technology primitives
    Technology,
    /// Standard cell library
    StdCell,
}

/// Library metadata
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LibraryMetadata {
    /// Description
    pub description: String,
    /// Version string
    pub version: String,
    /// Author/vendor
    pub author: String,
    /// Creation timestamp
    pub created: String,
    /// Last modified timestamp
    pub modified: String,
    /// Technology/process name
    pub technology: Option<String>,
}

impl Library {
    /// Create a new library
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            path: None,
            cells: HashMap::new(),
            library_type: LibraryType::Project,
            metadata: LibraryMetadata::default(),
            read_only: false,
        }
    }

    /// Create library with path
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    /// Set library type
    pub fn with_type(mut self, lib_type: LibraryType) -> Self {
        self.library_type = lib_type;
        self
    }

    /// Create a new cell in this library
    pub fn create_cell(&mut self, name: &str) -> &mut Cell {
        self.cells
            .entry(name.to_string())
            .and_modify(|cell| *cell = Cell::new(name, &self.name))
            .or_insert_with(|| Cell::new(name, &self.name))
    }

    /// Get cell by name
    pub fn get_cell(&self, name: &str) -> Option<&Cell> {
        self.cells.get(name)
    }

    /// Get mutable cell by name
    pub fn get_cell_mut(&mut self, name: &str) -> Option<&mut Cell> {
        self.cells.get_mut(name)
    }

    /// List all cell names
    pub fn cell_names(&self) -> Vec<&str> {
        self.cells.keys().map(|s| s.as_str()).collect()
    }

    /// Number of cells
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Check if cell exists
    pub fn has_cell(&self, name: &str) -> bool {
        self.cells.contains_key(name)
    }

    /// Delete a cell
    pub fn delete_cell(&mut self, name: &str) -> bool {
        self.cells.remove(name).is_some()
    }
}

impl Library {
    /// Constant for primitives library name (for use in inner functions)
    pub const PRIMITIVES_LIBRARY_NAME: &'static str = "primitives";
}
