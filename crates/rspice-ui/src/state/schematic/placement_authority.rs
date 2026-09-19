//! Which document an armed placement was configured for.
//!
//! An armed tool outlives the form that filled it: the reader types a pin name,
//! presses Enter, and the click that places it happens some frames later, in
//! whatever document is active then. The three values here are what identify
//! that document — the design's execution epoch, the active schematic buffer's
//! epoch, and the path of the cell/view on screen — and the click compares them
//! before it touches anything.
//!
//! What this deliberately does *not* carry is the schematic's topology version.
//! A form that asks for a name is not invalidated by someone else moving a
//! wire, and a placement that bumped the version would invalidate its own
//! successor, which is what made every one of these tools single-shot. Whatever
//! genuinely conflicts — a name now taken, an interface order now used — is
//! re-validated by the model at the click, in the model's own words.

/// The document an armed placement belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementAuthority {
    pub design_execution_epoch: u64,
    pub active_schematic_epoch: u64,
    pub view_path: String,
}

impl PlacementAuthority {
    pub fn new(
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: impl Into<String>,
    ) -> Self {
        Self {
            design_execution_epoch,
            active_schematic_epoch,
            view_path: view_path.into(),
        }
    }

    /// `true` when the live application values name the same document.
    pub fn matches(
        &self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: &str,
    ) -> bool {
        self.design_execution_epoch == design_execution_epoch
            && self.active_schematic_epoch == active_schematic_epoch
            && self.view_path == view_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_authority_matches_only_its_own_document() {
        let authority = PlacementAuthority::new(7, 3, "work/ota_5t/schematic");
        assert!(authority.matches(7, 3, "work/ota_5t/schematic"));
        assert!(!authority.matches(8, 3, "work/ota_5t/schematic"));
        assert!(!authority.matches(7, 4, "work/ota_5t/schematic"));
        assert!(!authority.matches(7, 3, "work/ota_5t/symbol"));
    }
}
