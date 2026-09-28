//! Initial user library and removal of obsolete seeded primitives.

use super::ProjectLibraries;
use rspice_design::library::Library;

impl ProjectLibraries {
    /// Name of the legacy seeded primitives library (purged on load).
    pub const PRIMITIVES_LIBRARY: &'static str = "primitives";

    /// User library name constant
    pub const USER_LIBRARY: &'static str = "user";

    /// Create the initial user library.
    ///
    /// Placeable primitives come from the component palette. Legacy seeded
    /// primitive cells were empty placeholders that duplicated the palette.
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
    /// in the browser. Callers migrate any workspace content out of it
    /// first, so removal is unconditional. Old sessions may carry it without
    /// the read-only flag.
    pub fn purge_legacy_primitives(&mut self) {
        if self.get_library(Self::PRIMITIVES_LIBRARY).is_some() {
            self.remove_library(Self::PRIMITIVES_LIBRARY);
        }
    }
}
