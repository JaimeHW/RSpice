//! Runtime consumers for user-level engineering preferences.
//!
//! Preference pages never own execution semantics. This module applies the
//! resolved user defaults only at the runtime boundaries that can safely
//! consume them without mutating stored project data.

use crate::workbench::TogglePreference;

use super::RSpiceApp;

impl RSpiceApp {
    /// Refresh connectivity evidence once per authored topology version when
    /// incremental checking is enabled. The same production checker backs the
    /// explicit command, so enabling this preference cannot create a weaker
    /// or differently classified result.
    pub(super) fn refresh_incremental_connectivity_checks(&mut self) {
        if !self
            .state
            .ui
            .preferences
            .toggle(TogglePreference::IncrementalConnectivityChecks)
        {
            return;
        }
        let topology_version = self.state.schematic.topology_version();
        if self.state.dialogs.drc_checked_version == topology_version {
            return;
        }
        let result = crate::services::drc::run_drc_check(&self.state.schematic);
        self.state.dialogs.drc_results = Some(result);
        self.state.dialogs.drc_checked_version = topology_version;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ComponentType, Point};

    #[test]
    fn incremental_connectivity_consumes_each_new_topology_version_once() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 10));
        let version = app.state.schematic.topology_version();

        app.refresh_incremental_connectivity_checks();

        assert_eq!(app.state.dialogs.drc_checked_version, version);
        assert!(app.state.dialogs.drc_results.is_some());
        app.refresh_incremental_connectivity_checks();
        assert_eq!(app.state.dialogs.drc_checked_version, version);
    }

    #[test]
    fn disabled_incremental_connectivity_leaves_manual_evidence_untouched() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .ui
            .preferences
            .set_toggle(TogglePreference::IncrementalConnectivityChecks, false);
        app.state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 10));

        app.refresh_incremental_connectivity_checks();

        assert!(app.state.dialogs.drc_results.is_none());
        assert_ne!(
            app.state.dialogs.drc_checked_version,
            app.state.schematic.topology_version()
        );
    }
}
