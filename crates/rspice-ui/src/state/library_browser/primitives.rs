use super::{Cell, Library, LibraryManager};

impl LibraryManager {
    // =========================================================================
    // Built-in Library Initialization
    // =========================================================================

    /// Name of the legacy seeded primitives library (purged on load).
    pub const PRIMITIVES_LIBRARY: &'static str = "primitives";

    /// User library name constant
    pub const USER_LIBRARY: &'static str = "user";

    /// Create a LibraryManager with the built-in libraries.
    ///
    /// Placeable primitives come from the component palette
    /// (`crate::schematic::component_palette`), not from seeded library
    /// cells — the old "primitives" library of empty placeholder cells
    /// only duplicated the palette as dead blocks in the browser.
    pub fn with_primitives() -> Self {
        let mut mgr = Self::new();
        mgr.create_user_library();
        mgr
    }

    /// Create an empty user library for custom cells
    pub fn create_user_library(&mut self) {
        let lib = Library::new(Self::USER_LIBRARY);
        self.add_library(lib);
    }

    /// Remove the legacy seeded "primitives" library from persisted
    /// sessions: its empty placeholder cells shadow the component palette
    /// in the browser. User-authored cells are never touched (the seeded
    /// library was read-only).
    pub fn purge_legacy_primitives(&mut self) {
        let is_seeded = self
            .get_library(Self::PRIMITIVES_LIBRARY)
            .is_some_and(|lib| lib.read_only);
        if is_seeded {
            self.remove_library(Self::PRIMITIVES_LIBRARY);
        }
    }

    /// Get all cells in a category
    pub fn cells_in_category(&self, library: &str, category: &str) -> Vec<&Cell> {
        self.get_library(library)
            .map(|lib| {
                lib.cells
                    .values()
                    .filter(|c| c.category == category)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get unique categories from a library
    pub fn categories(&self, library: &str) -> Vec<String> {
        self.get_library(library)
            .map(|lib| {
                let mut cats: Vec<String> =
                    lib.cells.values().map(|c| c.category.clone()).collect();
                cats.sort();
                cats.dedup();
                cats
            })
            .unwrap_or_default()
    }
}

// =============================================================================
