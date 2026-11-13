//! Initial user library.

use super::ProjectLibraries;
use rspice_design::library::Library;

impl ProjectLibraries {
    /// User library name constant
    pub const USER_LIBRARY: &'static str = "user";

    /// Create the initial user library. Placeable primitives come from the
    /// component palette.
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
}
