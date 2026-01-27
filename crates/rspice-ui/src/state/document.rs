//! Document Model for Multi-Document Interface
//!
//! Provides abstraction for individual documents, each containing
//! a schematic, simulation results, and file metadata.

use std::path::PathBuf;
use uuid::Uuid;

use crate::state::hierarchy::HierarchyManager;
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

    /// Hierarchical navigation state
    pub hierarchy_nav: HierarchyManager,
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
            hierarchy_nav: HierarchyManager::new(),
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
            hierarchy_nav: HierarchyManager::new(),
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

    /// Get a document by index
    pub fn get(&self, index: usize) -> Option<&Document> {
        self.documents.get(index)
    }

    /// Get a mutable document by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Document> {
        self.documents.get_mut(index)
    }

    /// Find document index by file path
    pub fn find_by_path(&self, path: &PathBuf) -> Option<usize> {
        self.documents
            .iter()
            .position(|d| d.file_path.as_ref() == Some(path))
    }

    /// Get the active document index
    pub fn active_index(&self) -> usize {
        self.active_index
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Document Tests
    // =========================================================================

    #[test]
    fn test_document_new_untitled_first() {
        let doc = Document::new_untitled(1);
        assert_eq!(doc.name, "Untitled");
        assert!(doc.file_path.is_none());
        assert!(!doc.is_dirty);
    }

    #[test]
    fn test_document_new_untitled_subsequent() {
        let doc = Document::new_untitled(2);
        assert_eq!(doc.name, "Untitled 2");

        let doc3 = Document::new_untitled(5);
        assert_eq!(doc3.name, "Untitled 5");
    }

    #[test]
    fn test_document_unique_ids() {
        let doc1 = Document::new_untitled(1);
        let doc2 = Document::new_untitled(2);
        assert_ne!(doc1.id, doc2.id);
    }

    #[test]
    fn test_document_from_path() {
        let path = PathBuf::from("/projects/my_circuit.rspice");
        let schematic = SchematicState::default();
        let doc = Document::from_path(path.clone(), schematic);

        assert_eq!(doc.name, "my_circuit");
        assert_eq!(doc.file_path, Some(path));
        assert!(!doc.is_dirty);
    }

    #[test]
    fn test_document_from_path_no_extension() {
        let path = PathBuf::from("/projects/test_schematic");
        let doc = Document::from_path(path.clone(), SchematicState::default());
        assert_eq!(doc.name, "test_schematic");
    }

    #[test]
    fn test_document_from_path_complex_name() {
        let path = PathBuf::from("/home/user/designs/opamp_v2.1.rspice");
        let doc = Document::from_path(path, SchematicState::default());
        assert_eq!(doc.name, "opamp_v2.1");
    }

    #[test]
    fn test_document_mark_dirty() {
        let mut doc = Document::new_untitled(1);
        assert!(!doc.is_dirty);

        doc.mark_dirty();
        assert!(doc.is_dirty);
    }

    #[test]
    fn test_document_mark_clean() {
        let mut doc = Document::new_untitled(1);
        doc.mark_dirty();
        assert!(doc.is_dirty);

        doc.mark_clean();
        assert!(!doc.is_dirty);
    }

    #[test]
    fn test_document_display_title_clean() {
        let doc = Document::new_untitled(1);
        assert_eq!(doc.display_title(), "Untitled");
    }

    #[test]
    fn test_document_display_title_dirty() {
        let mut doc = Document::new_untitled(1);
        doc.mark_dirty();
        assert_eq!(doc.display_title(), "Untitled*");
    }

    #[test]
    fn test_document_display_title_with_path_dirty() {
        let path = PathBuf::from("/test/circuit.rspice");
        let mut doc = Document::from_path(path, SchematicState::default());
        doc.mark_dirty();
        assert_eq!(doc.display_title(), "circuit*");
    }

    #[test]
    fn test_document_has_hierarchy_manager() {
        let doc = Document::new_untitled(1);
        assert!(doc.hierarchy_nav.libraries.is_empty());
    }

    // =========================================================================
    // DocumentManager Creation Tests
    // =========================================================================

    #[test]
    fn test_document_manager_default() {
        let mgr = DocumentManager::default();
        assert_eq!(mgr.len(), 1);
        assert!(!mgr.is_empty());
        assert_eq!(mgr.active().name, "Untitled");
    }

    #[test]
    fn test_document_manager_new_document() {
        let mut mgr = DocumentManager::default();
        let initial_count = mgr.len();

        let idx = mgr.new_document();
        assert_eq!(mgr.len(), initial_count + 1);
        assert_eq!(mgr.active_index, idx);
        assert_eq!(mgr.active().name, "Untitled 2");
    }

    #[test]
    fn test_document_manager_multiple_new_documents() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();
        mgr.new_document();
        mgr.new_document();

        assert_eq!(mgr.len(), 4);
        assert_eq!(mgr.documents[0].name, "Untitled");
        assert_eq!(mgr.documents[1].name, "Untitled 2");
        assert_eq!(mgr.documents[2].name, "Untitled 3");
        assert_eq!(mgr.documents[3].name, "Untitled 4");
    }

    // =========================================================================
    // DocumentManager Open/Close Tests
    // =========================================================================

    #[test]
    fn test_document_manager_open_document() {
        let mut mgr = DocumentManager::default();
        let path = PathBuf::from("/test/circuit.rspice");

        let idx = mgr.open_document(path.clone(), SchematicState::default());
        assert_eq!(mgr.len(), 2);
        assert_eq!(mgr.active_index, idx);
        assert_eq!(mgr.active().file_path, Some(path));
    }

    #[test]
    fn test_document_manager_open_same_path_twice() {
        let mut mgr = DocumentManager::default();
        let path = PathBuf::from("/test/circuit.rspice");

        let idx1 = mgr.open_document(path.clone(), SchematicState::default());
        let idx2 = mgr.open_document(path.clone(), SchematicState::default());

        assert_eq!(idx1, idx2);
        assert_eq!(mgr.len(), 2);
    }

    #[test]
    fn test_document_manager_open_different_paths() {
        let mut mgr = DocumentManager::default();
        let path1 = PathBuf::from("/test/circuit1.rspice");
        let path2 = PathBuf::from("/test/circuit2.rspice");

        mgr.open_document(path1, SchematicState::default());
        mgr.open_document(path2, SchematicState::default());

        assert_eq!(mgr.len(), 3);
    }

    #[test]
    fn test_document_manager_close_document() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();

        assert!(mgr.close_document(0));
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_document_manager_close_invalid_index() {
        let mut mgr = DocumentManager::default();
        assert!(!mgr.close_document(999));
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_document_manager_close_last_document_creates_new() {
        let mut mgr = DocumentManager::default();
        assert_eq!(mgr.len(), 1);

        mgr.close_document(0);
        assert_eq!(mgr.len(), 1);
        assert_eq!(mgr.active().name, "Untitled 2");
    }

    #[test]
    fn test_document_manager_close_adjusts_active_index_when_after() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();
        mgr.new_document();
        mgr.set_active(2);

        mgr.close_document(0);
        assert_eq!(mgr.active_index, 1);
    }

    #[test]
    fn test_document_manager_close_adjusts_active_index_at_end() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();
        mgr.set_active(1);

        mgr.close_document(1);
        assert_eq!(mgr.active_index, 0);
    }

    // =========================================================================
    // DocumentManager Active Document Tests
    // =========================================================================

    #[test]
    fn test_document_manager_set_active() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();
        mgr.new_document();

        mgr.set_active(0);
        assert_eq!(mgr.active_index, 0);

        mgr.set_active(2);
        assert_eq!(mgr.active_index, 2);
    }

    #[test]
    fn test_document_manager_set_active_invalid_ignored() {
        let mut mgr = DocumentManager::default();
        mgr.set_active(999);
        assert_eq!(mgr.active_index, 0);
    }

    #[test]
    fn test_document_manager_active_mut() {
        let mut mgr = DocumentManager::default();
        mgr.active_mut().mark_dirty();
        assert!(mgr.active().is_dirty);
    }

    #[test]
    fn test_document_manager_get_by_index() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();

        assert!(mgr.get(0).is_some());
        assert!(mgr.get(1).is_some());
        assert!(mgr.get(999).is_none());
    }

    #[test]
    fn test_document_manager_get_mut_by_index() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();

        if let Some(doc) = mgr.get_mut(1) {
            doc.mark_dirty();
        }
        assert!(mgr.get(1).unwrap().is_dirty);
    }

    // =========================================================================
    // DocumentManager Dirty State Tests
    // =========================================================================

    #[test]
    fn test_document_manager_has_unsaved_changes_none() {
        let mgr = DocumentManager::default();
        assert!(!mgr.has_unsaved_changes());
    }

    #[test]
    fn test_document_manager_has_unsaved_changes_one() {
        let mut mgr = DocumentManager::default();
        mgr.active_mut().mark_dirty();
        assert!(mgr.has_unsaved_changes());
    }

    #[test]
    fn test_document_manager_has_unsaved_changes_multiple() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();
        mgr.new_document();

        mgr.get_mut(0).unwrap().mark_dirty();
        mgr.get_mut(2).unwrap().mark_dirty();

        assert!(mgr.has_unsaved_changes());
    }

    #[test]
    fn test_document_manager_unsaved_documents_empty() {
        let mgr = DocumentManager::default();
        assert!(mgr.unsaved_documents().is_empty());
    }

    #[test]
    fn test_document_manager_unsaved_documents_some() {
        let mut mgr = DocumentManager::default();
        mgr.new_document();
        mgr.new_document();

        mgr.get_mut(0).unwrap().mark_dirty();
        mgr.get_mut(2).unwrap().mark_dirty();

        let unsaved = mgr.unsaved_documents();
        assert_eq!(unsaved.len(), 2);
        assert!(unsaved.contains(&0));
        assert!(unsaved.contains(&2));
    }

    // =========================================================================
    // DocumentManager Find/Query Tests
    // =========================================================================

    #[test]
    fn test_document_manager_find_by_path() {
        let mut mgr = DocumentManager::default();
        let path = PathBuf::from("/test/circuit.rspice");
        mgr.open_document(path.clone(), SchematicState::default());

        assert_eq!(mgr.find_by_path(&path), Some(1));
        assert_eq!(mgr.find_by_path(&PathBuf::from("/nonexistent")), None);
    }

    #[test]
    fn test_document_manager_active_index_getter() {
        let mut mgr = DocumentManager::default();
        assert_eq!(mgr.active_index(), 0);

        mgr.new_document();
        assert_eq!(mgr.active_index(), 1);
    }

    // =========================================================================
    // DocumentManager Save Tests
    // =========================================================================

    #[test]
    fn test_document_manager_save_active_no_path_fails() {
        let mut mgr = DocumentManager::default();
        let result = mgr.save_active_document();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No file path set"));
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_document_manager_never_empty() {
        let mut mgr = DocumentManager::default();

        while mgr.len() > 1 {
            mgr.close_document(0);
        }
        mgr.close_document(0);

        assert!(!mgr.is_empty());
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_document_manager_close_middle_document() {
        let mut mgr = DocumentManager::default();
        let path1 = PathBuf::from("/test/a.rspice");
        let path2 = PathBuf::from("/test/b.rspice");
        let path3 = PathBuf::from("/test/c.rspice");

        mgr.open_document(path1, SchematicState::default());
        mgr.open_document(path2, SchematicState::default());
        mgr.open_document(path3, SchematicState::default());

        mgr.set_active(1);
        mgr.close_document(2);

        assert_eq!(mgr.len(), 3);
        assert_eq!(mgr.active_index, 1);
    }

    #[test]
    fn test_document_manager_rapid_create_close() {
        let mut mgr = DocumentManager::default();

        for _ in 0..10 {
            mgr.new_document();
        }
        assert_eq!(mgr.len(), 11);

        for _ in 0..10 {
            mgr.close_document(1);
        }
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_document_dirty_state_preserved_across_operations() {
        let mut mgr = DocumentManager::default();
        mgr.active_mut().mark_dirty();

        mgr.new_document();
        mgr.set_active(0);

        assert!(mgr.active().is_dirty);
    }
}
