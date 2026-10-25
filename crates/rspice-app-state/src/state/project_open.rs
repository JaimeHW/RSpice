//! Whether a project is open, as the model reads it.
//!
//! The project lifecycle opens and closes projects, and it sits above the
//! model. Model code that refuses an edit while no project is open reads this
//! copy of the lifecycle's answer instead of asking the lifecycle, so the model
//! never reaches up. The copy is runtime state and is never persisted: the
//! session records whether a project is open through the lifecycle, and a
//! restored lifecycle publishes its answer here when it is rebuilt.

/// The model's read-only copy of whether a project is open.
///
/// [`ProjectOpenFact::set_from_lifecycle`] is its one writer. The project
/// lifecycle calls it from one function, in the same call that opens, resets
/// or closes the project, and a source guard refuses any other caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectOpenFact {
    open: bool,
}

impl Default for ProjectOpenFact {
    /// A fresh lifecycle holds an open, empty project, so a fresh fact says
    /// open too.
    fn default() -> Self {
        Self { open: true }
    }
}

impl ProjectOpenFact {
    /// Whether a project is open.
    pub const fn is_open(self) -> bool {
        self.open
    }

    /// Record the project lifecycle's answer. Only the lifecycle's one
    /// publishing function calls this; nothing else writes the fact.
    pub fn set_from_lifecycle(&mut self, open: bool) {
        self.open = open;
    }
}
