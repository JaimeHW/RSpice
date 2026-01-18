//! Document Model for Multi-Document Interface
//!
//! Provides abstraction for individual documents, each containing
//! a schematic, simulation results, and file metadata.

use std::path::PathBuf;
use uuid::Uuid;

use crate::state::{SchematicState, SimulationState};

/// A single document representing one schematic file
#[derive(Debug, Clone)]
pub struct Document {
    /// Unique identifier for this document
    pub id: Uuid,

    /// Display name (filename or "Untitled N")
    pub name: String,

    /// File path if saved, None for new unsaved documents
    pub file_path: Option<PathBuf>,

    /// The schematic state for this document
    pub schematic: SchematicState,

    /// Simulation results for this document
    pub simulation: SimulationState,

    /// Whether document has unsaved changes
    pub is_dirty: bool,
}

impl Document {
    /// Create a new untitled document
    pub fn new_untitled(number: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: if number == 1 {
                "Untitled".to_string()
            } else {
                format!("Untitled {}", number)
            },
            file_path: None,
            schematic: SchematicState::default(),
            simulation: SimulationState::default(),
            is_dirty: false,
        }
    }

    /// Create a document from a file path
    pub fn from_path(path: PathBuf, schematic: SchematicState) -> Self {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Self {
            id: Uuid::new_v4(),
            name,
            file_path: Some(path),
            schematic,
            simulation: SimulationState::default(),
            is_dirty: false,
        }
    }

    /// Mark document as modified
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    /// Mark document as saved
    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }

    /// Get display title (with * if dirty)
    pub fn display_title(&self) -> String {
        if self.is_dirty {
            format!("{}*", self.name)
        } else {
            self.name.clone()
        }
    }
}

/// Manager for multiple open documents
#[derive(Debug, Clone)]
pub struct DocumentManager {
    /// List of open documents
    pub documents: Vec<Document>,

    /// Index of the currently active document
    pub active_index: usize,

    /// Counter for generating unique untitled names
    untitled_counter: usize,
}

impl Default for DocumentManager {
    fn default() -> Self {
        let mut mgr = Self {
            documents: Vec::new(),
            active_index: 0,
            untitled_counter: 0,
        };
        // Start with one untitled document
        mgr.new_document();
        mgr
    }
}

impl DocumentManager {
    /// Create a new document and add it to the list
    /// Returns the index of the new document
    pub fn new_document(&mut self) -> usize {
        self.untitled_counter += 1;
        let doc = Document::new_untitled(self.untitled_counter);
        self.documents.push(doc);
        let index = self.documents.len() - 1;
        self.active_index = index;
        index
    }

    /// Open a document from a file
    pub fn open_document(&mut self, path: PathBuf, schematic: SchematicState) -> usize {
        // Check if already open
        if let Some(idx) = self
            .documents
            .iter()
            .position(|d| d.file_path.as_ref() == Some(&path))
        {
            self.active_index = idx;
            return idx;
        }

        let doc = Document::from_path(path, schematic);
        self.documents.push(doc);
        let index = self.documents.len() - 1;
        self.active_index = index;
        index
    }

    /// Close a document by index
    /// Returns true if closed, false if cancelled (e.g., unsaved)
    pub fn close_document(&mut self, index: usize) -> bool {
        if index >= self.documents.len() {
            return false;
        }

        self.documents.remove(index);

        // Ensure we always have at least one document
        if self.documents.is_empty() {
            self.new_document();
            return true;
        }

        // Adjust active index if needed
        if self.active_index >= self.documents.len() {
            self.active_index = self.documents.len() - 1;
        } else if self.active_index > index {
            self.active_index -= 1;
        }

        true
    }

    /// Get reference to active document
    pub fn active(&self) -> &Document {
        &self.documents[self.active_index]
    }

    /// Get mutable reference to active document
    pub fn active_mut(&mut self) -> &mut Document {
        &mut self.documents[self.active_index]
    }

    /// Set active document by index
    pub fn set_active(&mut self, index: usize) {
        if index < self.documents.len() {
            self.active_index = index;
        }
    }

    /// Get document count
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    /// Check if empty (should never be true)
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    /// Check if any document has unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.documents.iter().any(|d| d.is_dirty)
    }

    /// Get list of documents with unsaved changes
    pub fn unsaved_documents(&self) -> Vec<usize> {
        self.documents
            .iter()
            .enumerate()
            .filter(|(_, d)| d.is_dirty)
            .map(|(i, _)| i)
            .collect()
    }

    /// Save the active document to its file path
    /// Returns Ok(path) if saved, Err with message if failed
    pub fn save_active_document(&mut self) -> Result<PathBuf, String> {
        let idx = self.active_index;

        // Clone path first to avoid borrow conflict
        let path = self.documents[idx]
            .file_path
            .clone()
            .ok_or_else(|| "No file path set. Use Save As.".to_string())?;

        Self::write_schematic_to_file(&self.documents[idx].schematic, &path)?;
        self.documents[idx].mark_clean();
        Ok(path)
    }

    /// Save the active document to a new path
    pub fn save_active_document_as(&mut self, path: PathBuf) -> Result<(), String> {
        let doc = &mut self.documents[self.active_index];

        Self::write_schematic_to_file(&doc.schematic, &path)?;

        // Update document with new path and name
        doc.file_path = Some(path.clone());
        doc.name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();
        doc.mark_clean();

        Ok(())
    }

    /// Load a document from a file path and add it to the document list
    pub fn load_document_from_file(&mut self, path: PathBuf) -> Result<usize, String> {
        // Check if already open
        if let Some(idx) = self
            .documents
            .iter()
            .position(|d| d.file_path.as_ref() == Some(&path))
        {
            self.active_index = idx;
            return Ok(idx);
        }

        let schematic = Self::read_schematic_from_file(&path)?;
        let idx = self.open_document(path, schematic);
        Ok(idx)
    }

    /// Write schematic state to a file
    fn write_schematic_to_file(schematic: &SchematicState, path: &PathBuf) -> Result<(), String> {
        let json = serde_json::to_string_pretty(schematic)
            .map_err(|e| format!("Serialization error: {}", e))?;

        std::fs::write(path, json).map_err(|e| format!("File write error: {}", e))?;

        Ok(())
    }

    /// Read schematic state from a file
    fn read_schematic_from_file(path: &PathBuf) -> Result<SchematicState, String> {
        let content =
            std::fs::read_to_string(path).map_err(|e| format!("File read error: {}", e))?;

        serde_json::from_str(&content).map_err(|e| format!("Parse error: {}", e))
    }
}
