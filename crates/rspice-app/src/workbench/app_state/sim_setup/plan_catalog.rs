//! Editor restoration after persisted simulation-plan catalog transactions.

use super::SimSetupState;
use crate::product::SimulationPlanId;
#[cfg(test)]
use crate::simulation::dialog::SimulationOptions;
pub use rspice_simulation_contract::plan_catalog::{
    SimulationPlanCatalogError, SimulationPlanCloneOptions, SimulationPlanCloneOutcome,
    SimulationPlanImportDocument, SimulationPlanLineage, SimulationPlanName,
};

impl SimSetupState {
    pub fn create_plan(
        &mut self,
        name: impl Into<String>,
    ) -> Result<SimulationPlanId, SimulationPlanCatalogError> {
        let id = self.document.create_plan(name)?;
        self.reset_plan_editor_transients();
        self.refresh_legacy_analysis_projections();
        Ok(id)
    }

    pub fn import_plan(
        &mut self,
        document: SimulationPlanImportDocument,
    ) -> Result<SimulationPlanCloneOutcome, SimulationPlanCatalogError> {
        let outcome = self.document.import_plan(document)?;
        self.reset_plan_editor_transients();
        self.refresh_legacy_analysis_projections();
        Ok(outcome)
    }

    pub fn clone_active_plan(
        &mut self,
        new_name: impl Into<String>,
        contents: SimulationPlanCloneOptions,
    ) -> Result<SimulationPlanCloneOutcome, SimulationPlanCatalogError> {
        let outcome = self.document.clone_active_plan(new_name, contents)?;
        self.reset_plan_editor_transients();
        self.refresh_legacy_analysis_projections();
        Ok(outcome)
    }

    pub fn activate_plan(
        &mut self,
        id: SimulationPlanId,
    ) -> Result<(), SimulationPlanCatalogError> {
        let previous = self.document.analysis_plan.as_ref().map(|plan| plan.id());
        self.document.activate_plan(id)?;
        if previous != Some(id) {
            self.reset_plan_editor_transients();
            self.refresh_legacy_analysis_projections();
        }
        Ok(())
    }

    fn reset_plan_editor_transients(&mut self) {
        self.session.options_draft =
            crate::simulation::dialog::OptionsDialogState::from_options(&self.options);
        self.session.palette_open = false;
        self.session.palette_query.clear();
        self.session.palette_active = 0;
        self.session.palette_scroll_to_active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::AnalysisKind;

    #[test]
    fn clone_content_flags_create_an_empty_plan_with_default_advanced_options() {
        let mut setup = SimSetupState::new();
        setup
            .set_reference_pvt(crate::product::ProcessCorner::SS, 125.0)
            .unwrap();
        setup.options.reltol = 9.0e-7;

        let contents = SimulationPlanCloneOptions {
            copy_analyses: false,
            copy_advanced_options: false,
            ..SimulationPlanCloneOptions::default()
        };
        let outcome = setup
            .clone_active_plan("Empty diagnostic", contents)
            .expect("empty clone commits");

        assert!(setup.analysis_plan.as_ref().unwrap().instances().is_empty());
        assert_eq!(setup.options.reltol, SimulationOptions::default().reltol);
        assert_eq!(setup.options.temp, 125.0);
        assert_eq!(
            setup.reference_pvt.process,
            crate::product::ProcessCorner::SS
        );
        assert_eq!(setup.reference_pvt.temperature_celsius, 125.0);
        assert_eq!(outcome.contents, contents);
        assert!(outcome.analysis_identity_map.is_empty());
        assert_eq!(setup.active_plan_lineage().clone_contents(), Some(contents));
        assert_eq!(setup.enabled_analysis_instance_count(), 0);
    }

    #[test]
    fn plan_switch_is_atomic_when_the_target_is_missing() {
        let mut setup = SimSetupState::new();
        setup.session.palette_open = true;
        setup.session.palette_query = "keep this draft".to_owned();
        setup.session.palette_active = 3;
        setup.session.palette_scroll_to_active = true;
        setup.ac.points = "uncommitted draft".to_owned();
        let before = serde_json::to_value(&setup).unwrap();
        let active_id = setup.analysis_plan.as_ref().unwrap().id();
        setup
            .activate_plan(active_id)
            .expect("active plan is unchanged");
        assert!(matches!(
            setup.activate_plan(SimulationPlanId::new()),
            Err(SimulationPlanCatalogError::PlanNotFound(_))
        ));
        assert_eq!(serde_json::to_value(&setup).unwrap(), before);
        assert!(setup.session.palette_open);
        assert_eq!(setup.session.palette_query, "keep this draft");
        assert_eq!(setup.session.palette_active, 3);
        assert!(setup.session.palette_scroll_to_active);
        assert_eq!(setup.ac.points, "uncommitted draft");
    }

    #[test]
    fn stored_plan_round_trip_preserves_catalog_identity_and_lineage() {
        let mut setup = SimSetupState::new();
        let source = setup.analysis_plan.as_ref().unwrap().id();
        let clone = setup
            .clone_active_plan("AC characterization", SimulationPlanCloneOptions::default())
            .unwrap()
            .cloned_plan_id;
        setup
            .analysis_plan
            .as_mut()
            .unwrap()
            .insert(AnalysisKind::OperatingPoint)
            .unwrap();

        let json = serde_json::to_string(&setup).unwrap();
        let mut restored: SimSetupState = serde_json::from_str(&json).unwrap();
        restored.prepare_after_restore();

        assert_eq!(restored.analysis_plan.as_ref().unwrap().id(), clone);
        assert_eq!(restored.inactive_plans()[0].id(), source);
        assert_eq!(
            restored.active_plan_lineage().source_plan_id(),
            Some(source)
        );
        restored.validate_plan_catalog().unwrap();
    }
}
