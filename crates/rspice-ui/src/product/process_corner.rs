//! The named process corner a run, a PDK section, or a model card is taken at.
//!
//! This is a vocabulary of five names, not a simulator concept. It was
//! declared inside the corner *dialog*, which put a term the persisted model
//! needs above the model that needs it: typing
//! `ModelLibraryManager::reference_process` meant `state` reaching up into
//! `simulation` 38 times for an enum with no behaviour. A vocabulary is data,
//! so it sits with the other product contracts, where persistence, services,
//! the runner, and presentation can all read down into it.
//!
//! The variants are the SPICE-conventional spellings and are serialized by
//! name, so renaming one breaks saved projects.

/// Standard process corner types.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ProcessCorner {
    /// Typical-Typical (nominal)
    #[default]
    TT,
    /// Slow-Slow (worst delay)
    SS,
    /// Fast-Fast (worst power)
    FF,
    /// Slow-Fast (skewed)
    SF,
    /// Fast-Slow (skewed)
    FS,
}

impl ProcessCorner {
    /// Short name
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::TT => "TT",
            Self::SS => "SS",
            Self::FF => "FF",
            Self::SF => "SF",
            Self::FS => "FS",
        }
    }

    /// Speed corners only (SS, TT, FF)
    pub fn speed_corners() -> Vec<ProcessCorner> {
        vec![Self::SS, Self::TT, Self::FF]
    }
}
