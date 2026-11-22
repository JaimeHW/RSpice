//! The canonical workspaces, and the model catalog a session starts from.
//!
//! Both are vocabulary every tier reads: the session persists the active
//! workspace, and the platform services, the workbench and each feature key
//! their work by it.

use serde::{Deserialize, Serialize};

use crate::state::model_library::ModelLibraryManager;
use crate::state::pdk_config::PdkConfig;

/// The eight canonical workspaces from the product contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Workspace {
    Project,
    #[default]
    Design,
    Simulate,
    /// The stimulus definitions this project owns. It reads beside the studio
    /// that consumes them.
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

    /// Where the workspace stands on the rail, from 0: its place in
    /// [`Self::ALL`], which its keyboard chord is numbered by.
    pub const fn rail_index(self) -> usize {
        let mut index = 0;
        while index < Self::ALL.len() {
            if Self::ALL[index] as u8 == self as u8 {
                return index;
            }
            index += 1;
        }
        unreachable!()
    }

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
            Self::Netlist => "Netlist",
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

/// The model catalog a PDK configuration describes: the built-in models and
/// shipped packs, then the configuration's model sources, with the count of
/// models those sources loaded. On success the configuration records the
/// sources the catalog now manages; on failure it is unchanged. Nothing is
/// persisted: keeping the recorded sources is the caller's decision.
pub fn restore_model_library_manager(
    config: &mut PdkConfig,
) -> Result<(ModelLibraryManager, usize), Vec<String>> {
    let previous = config.clone();
    let mut next = previous.clone();
    let mut manager = default_model_library_manager();
    let loaded = manager.replace_from_pdk_config(Some(&previous), &mut next)?;
    *config = next;
    Ok((manager, loaded))
}

/// Rebuild the application-scoped model catalog from durable PDK settings.
///
/// Project-owned libraries never participate in this reconstruction. The
/// caller receives a completely staged catalog, and both the live
/// configuration and catalog remain unchanged unless discovery, parsing, and
/// native ownership persistence all succeed.
pub fn restore_session_model_library_manager(
    config: &mut crate::state::pdk_config::PdkConfig,
) -> Result<(crate::state::model_library::ModelLibraryManager, usize), Vec<String>> {
    let mut next = config.clone();
    let restored = restore_model_library_manager(&mut next)?;

    #[cfg(all(not(any(test, feature = "test-support")), not(target_arch = "wasm32")))]
    if next.managed_model_sources != config.managed_model_sources {
        crate::state::pdk_config::save(&next).map_err(|error| {
            vec![format!(
                "Configured PDK sources were validated but their ownership could not be persisted: {error}"
            )]
        })?;
    }

    *config = next;
    Ok(restored)
}
