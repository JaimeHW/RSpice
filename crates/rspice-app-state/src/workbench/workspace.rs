//! The canonical workspaces, and the model catalog a session starts from.
//!
//! Both are vocabulary every tier reads: the session persists the active
//! workspace, and the platform services, the workbench and each feature key
//! their work by it.

use serde::{Deserialize, Serialize};

use crate::state::model_library::ModelLibraryManager;

/// The eight canonical workspaces from the product contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Workspace {
    Project,
    #[default]
    Design,
    Simulate,
    /// The stimulus definitions this project owns. It reads beside the studio
    /// that consumes them rather than at the end of the rail, which is where
    /// its chord — kept at Alt+8 so the seven older chords do not move — would
    /// otherwise put it.
    Stimulus,
    Results,
    Verify,
    Models,
    Netlist,
}

impl Workspace {
    pub const ALL: [Self; 8] = [
        Self::Project,
        Self::Design,
        Self::Simulate,
        Self::Stimulus,
        Self::Results,
        Self::Verify,
        Self::Models,
        Self::Netlist,
    ];

    pub const PHONE_PRIMARY: [Self; 4] =
        [Self::Design, Self::Simulate, Self::Results, Self::Verify];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::Design => "Design",
            Self::Simulate => "Simulate",
            Self::Stimulus => "Stimulus Library",
            Self::Results => "Results",
            Self::Verify => "Verify",
            Self::Models => "Models",
            Self::Netlist => "Netlist & Script Editor",
        }
    }

    pub const fn owner_label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::Design => "Design",
            Self::Simulate => "Simulation Studio",
            // The product name is already the owner's name, exactly as the
            // Netlist & Script Editor's is. Coining a second spelling here
            // would give one thing two names in one shell.
            Self::Stimulus => "Stimulus Library",
            Self::Results => "Result document",
            Self::Verify => "Verification results",
            Self::Models => "Model binding",
            Self::Netlist => "Netlist & Script Editor",
        }
    }

    pub const fn inspector_title(self) -> &'static str {
        match self {
            Self::Project => "Project details",
            Self::Design => "Inspector",
            Self::Simulate => "Analysis",
            Self::Stimulus => "Definition details",
            Self::Results => "Data inspector",
            Self::Verify => "Verification",
            Self::Models => "Model details",
            Self::Netlist => "Source",
        }
    }
}

/// The built-in models and the SPICE packs shipped beside the binary.
pub fn default_model_library_manager() -> ModelLibraryManager {
    let mut manager = ModelLibraryManager::new();
    manager.load_builtin_models();
    // The packs ship beside the binary; absence is normal (browser build).
    manager.discover_spice_packs();
    manager
}
