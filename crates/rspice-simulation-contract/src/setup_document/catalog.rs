//! Atomic operations on the persisted active and inactive plan catalog.

use super::SimulationSetupDocument;
use crate::options::SimulationOptions;
use crate::plan_catalog::{
    SimulationPlanCatalogError, SimulationPlanCloneOptions, SimulationPlanCloneOutcome,
    SimulationPlanImportDocument, SimulationPlanLineage, SimulationPlanName, StoredSimulationPlan,
    validate_model_binding_list,
};
use crate::plan_model::SimulationPlan;
use crate::run_set::{ReferencePoint as ReferencePvtPoint, RunSetState};
use rspice_app_types::product::SimulationPlanId;
use std::collections::HashSet;

impl SimulationSetupDocument {
    #[must_use]
    pub fn active_plan_name(&self) -> &SimulationPlanName {
        &self.active_plan_name
    }

    #[must_use]
    pub const fn active_plan_lineage(&self) -> SimulationPlanLineage {
        self.active_plan_lineage
    }

    #[must_use]
    pub fn inactive_plans(&self) -> &[StoredSimulationPlan] {
        &self.inactive_plans
    }

    #[must_use]
    pub fn plan_count(&self) -> usize {
        usize::from(self.analysis_plan.is_some()) + self.inactive_plans.len()
    }

    /// Snapshot one catalog entry as a portable plan document without
    /// changing the active editor.
    pub fn export_plan(
        &self,
        id: SimulationPlanId,
    ) -> Result<SimulationPlanImportDocument, SimulationPlanCatalogError> {
        self.validate_plan_catalog()?;
        if let Some(plan) = self.analysis_plan.as_ref().filter(|plan| plan.id() == id) {
            return Ok(SimulationPlanImportDocument {
                source_plan_id: plan.id(),
                source_revision: plan.revision(),
                name: self.active_plan_name.clone(),
                analysis_plan: plan.clone(),
                reference_pvt: self.reference_pvt,
                run_set: self.run_set.clone(),
                model_bindings: self.model_bindings.clone(),
                save_policy: self.save_policy,
                options: self.options.clone(),
            });
        }
        let stored = self
            .inactive_plans
            .iter()
            .find(|plan| plan.id() == id)
            .ok_or(SimulationPlanCatalogError::PlanNotFound(id))?;
        Ok(stored.to_document())
    }

    /// Create and activate a fresh root plan while retaining the current plan
    /// unchanged in the catalog.
    pub fn create_plan(
        &mut self,
        name: impl Into<String>,
    ) -> Result<SimulationPlanId, SimulationPlanCatalogError> {
        let name = SimulationPlanName::new(name)?;
        self.ensure_plan_name_available(&name, None)?;
        self.validate_plan_catalog()?;
        let active = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        if active.has_executing_instances() {
            return Err(SimulationPlanCatalogError::PlanExecuting(active.id()));
        }

        let mut candidate = self.clone();
        let stored = take_active_plan_for_storage(&mut candidate)?;
        candidate.inactive_plans.push(stored);
        let plan = SimulationPlan::new();
        let id = plan.id();
        candidate.active_plan_name = name;
        candidate.active_plan_lineage = SimulationPlanLineage::default();
        candidate.analysis_plan = Some(plan);
        candidate.reference_pvt = ReferencePvtPoint::default();
        candidate.run_set = RunSetState::reference_only();
        candidate.model_bindings.clear();
        candidate.save_policy = crate::output_policy::SimulationSavePolicy::default();
        candidate.options = SimulationOptions::default();
        candidate.options.temp = candidate.reference_pvt.temperature_celsius;
        candidate.validate_plan_catalog()?;
        *self = candidate;
        Ok(id)
    }

    /// Rename one plan without changing its stable identity or revision.
    pub fn rename_plan(
        &mut self,
        id: SimulationPlanId,
        name: impl Into<String>,
    ) -> Result<(), SimulationPlanCatalogError> {
        self.validate_plan_catalog()?;
        let name = SimulationPlanName::new(name)?;
        self.ensure_plan_name_available(&name, Some(id))?;
        let active = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        if active.id() == id {
            if active.has_executing_instances() {
                return Err(SimulationPlanCatalogError::PlanExecuting(id));
            }
            self.active_plan_name = name;
            return Ok(());
        }
        let plan = self
            .inactive_plans
            .iter_mut()
            .find(|plan| plan.id() == id)
            .ok_or(SimulationPlanCatalogError::PlanNotFound(id))?;
        if plan.analysis_plan().has_executing_instances() {
            return Err(SimulationPlanCatalogError::PlanExecuting(id));
        }
        plan.rename(name);
        Ok(())
    }

    /// Recoverably archive one inactive plan.
    pub fn archive_plan(&mut self, id: SimulationPlanId) -> Result<(), SimulationPlanCatalogError> {
        self.validate_plan_catalog()?;
        if self
            .analysis_plan
            .as_ref()
            .is_some_and(|plan| plan.id() == id)
        {
            return Err(SimulationPlanCatalogError::ActivePlanCannotBeArchived(id));
        }
        let plan = self
            .inactive_plans
            .iter_mut()
            .find(|plan| plan.id() == id)
            .ok_or(SimulationPlanCatalogError::PlanNotFound(id))?;
        if plan.archived() {
            return Err(SimulationPlanCatalogError::PlanAlreadyArchived(id));
        }
        if plan.analysis_plan().has_executing_instances() {
            return Err(SimulationPlanCatalogError::PlanExecuting(id));
        }
        plan.set_archived(true);
        Ok(())
    }

    /// Restore an archived plan to the selectable catalog.
    pub fn restore_plan(&mut self, id: SimulationPlanId) -> Result<(), SimulationPlanCatalogError> {
        self.validate_plan_catalog()?;
        let plan = self
            .inactive_plans
            .iter_mut()
            .find(|plan| plan.id() == id)
            .ok_or(SimulationPlanCatalogError::PlanNotFound(id))?;
        if !plan.archived() {
            return Err(SimulationPlanCatalogError::PlanNotArchived(id));
        }
        plan.set_archived(false);
        Ok(())
    }

    /// Import a portable plan as a fresh local identity and activate it.
    /// Source lineage is retained, while analysis identities are remapped so
    /// imported references cannot collide with this project.
    pub fn import_plan(
        &mut self,
        mut document: SimulationPlanImportDocument,
    ) -> Result<SimulationPlanCloneOutcome, SimulationPlanCatalogError> {
        self.validate_plan_catalog()?;
        self.ensure_plan_name_available(&document.name, None)?;
        document.analysis_plan.prepare_after_restore();
        document.analysis_plan.validate_structure()?;
        validate_model_binding_list(&document.model_bindings)
            .map_err(SimulationPlanCatalogError::InvalidModelBindings)?;
        let current = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        if current.has_executing_instances() {
            return Err(SimulationPlanCatalogError::PlanExecuting(current.id()));
        }
        let imported = document.analysis_plan.clone_as_new()?;
        let analysis_identity_map = document
            .analysis_plan
            .instances()
            .iter()
            .zip(imported.instances())
            .map(|(source, destination)| (source.id(), destination.id()))
            .collect::<Vec<_>>();
        let imported_id = imported.id();

        let mut candidate = self.clone();
        let stored = take_active_plan_for_storage(&mut candidate)?;
        candidate.inactive_plans.push(stored);
        candidate.active_plan_name = document.name;
        candidate.active_plan_lineage = SimulationPlanLineage::cloned_from_with_contents(
            document.source_plan_id,
            document.source_revision,
            SimulationPlanCloneOptions::ALL_PLAN_CONTENTS,
        );
        candidate.analysis_plan = Some(imported);
        candidate.reference_pvt = document.reference_pvt;
        candidate.run_set = document.run_set;
        candidate.model_bindings = document.model_bindings;
        candidate.save_policy = document.save_policy;
        candidate.options = document.options;
        candidate.validate_plan_catalog()?;
        *self = candidate;
        Ok(SimulationPlanCloneOutcome {
            source_plan_id: document.source_plan_id,
            source_revision: document.source_revision,
            cloned_plan_id: imported_id,
            contents: SimulationPlanCloneOptions::ALL_PLAN_CONTENTS,
            analysis_identity_map,
        })
    }

    /// Clone the active plan and activate the clone in one atomic operation.
    ///
    /// The source becomes an inactive named plan. Result data is not part of
    /// `SimulationSetupDocument`, so no result dataset or manifest can be duplicated by
    /// this transaction.
    pub fn clone_active_plan(
        &mut self,
        new_name: impl Into<String>,
        contents: SimulationPlanCloneOptions,
    ) -> Result<SimulationPlanCloneOutcome, SimulationPlanCatalogError> {
        let new_name = SimulationPlanName::new(new_name)?;
        self.ensure_plan_name_available(&new_name, None)?;
        self.validate_plan_catalog()?;

        let source = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        if source.has_executing_instances() {
            return Err(SimulationPlanCatalogError::PlanExecuting(source.id()));
        }

        let existing_ids = self.plan_identities();
        let cloned_plan = loop {
            let candidate = if contents.copy_analyses {
                source.clone_as_new()?
            } else {
                SimulationPlan::empty()
            };
            if !existing_ids.contains(&candidate.id()) {
                break candidate;
            }
        };
        let source_id = source.id();
        let source_revision = source.revision();
        let cloned_id = cloned_plan.id();
        let analysis_identity_map = source
            .instances()
            .iter()
            .zip(cloned_plan.instances())
            .map(|(source, cloned)| (source.id(), cloned.id()))
            .collect();
        let cloned_lineage =
            SimulationPlanLineage::cloned_from_with_contents(source_id, source_revision, contents);
        let cloned_reference_pvt = if contents.copy_pvt_and_model_bindings {
            self.reference_pvt
        } else {
            ReferencePvtPoint::default()
        };
        let cloned_run_set = if contents.copy_pvt_and_model_bindings {
            self.run_set.clone()
        } else {
            RunSetState::reference_only()
        };
        let cloned_model_bindings = if contents.copy_pvt_and_model_bindings {
            self.model_bindings.clone()
        } else {
            Vec::new()
        };
        let cloned_save_policy = if contents.copy_advanced_options {
            self.save_policy
        } else {
            crate::output_policy::SimulationSavePolicy::default()
        };
        let mut cloned_options = if contents.copy_advanced_options {
            self.options.clone()
        } else {
            SimulationOptions::default()
        };
        // Reference temperature is owned by the PVT domain and must agree
        // with the exact value the solver consumes, regardless of whether the
        // rest of the advanced options were copied.
        cloned_options.temp = cloned_reference_pvt.temperature_celsius;

        let mut candidate = self.clone();
        let stored = take_active_plan_for_storage(&mut candidate)?;
        candidate.inactive_plans.push(stored);
        candidate.active_plan_name = new_name;
        candidate.active_plan_lineage = cloned_lineage;
        candidate.analysis_plan = Some(cloned_plan);
        candidate.reference_pvt = cloned_reference_pvt;
        candidate.run_set = cloned_run_set;
        candidate.model_bindings = cloned_model_bindings;
        candidate.save_policy = cloned_save_policy;
        candidate.options = cloned_options;
        candidate.validate_plan_catalog()?;
        *self = candidate;
        Ok(SimulationPlanCloneOutcome {
            source_plan_id: source_id,
            source_revision,
            cloned_plan_id: cloned_id,
            contents,
            analysis_identity_map,
        })
    }

    /// Activate an existing plan without changing either plan's durable
    /// identity or revision. The replaced active plan is retained in the same
    /// catalog slot, preserving deterministic presentation order.
    pub fn activate_plan(
        &mut self,
        id: SimulationPlanId,
    ) -> Result<(), SimulationPlanCatalogError> {
        self.validate_plan_catalog()?;
        let active = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        if active.id() == id {
            return Ok(());
        }
        if active.has_executing_instances() {
            return Err(SimulationPlanCatalogError::PlanExecuting(active.id()));
        }
        let index = self
            .inactive_plans
            .iter()
            .position(|plan| plan.id() == id)
            .ok_or(SimulationPlanCatalogError::PlanNotFound(id))?;
        if self.inactive_plans[index]
            .analysis_plan()
            .has_executing_instances()
        {
            return Err(SimulationPlanCatalogError::PlanExecuting(id));
        }
        if self.inactive_plans[index].archived() {
            return Err(SimulationPlanCatalogError::PlanArchived(id));
        }

        let mut candidate = self.clone();
        let target = candidate.inactive_plans.remove(index);
        let current = take_active_plan_for_storage(&mut candidate)?;
        candidate.inactive_plans.insert(index, current);
        candidate.active_plan_lineage = target.lineage();
        let target = target.into_document();
        candidate.active_plan_name = target.name;
        candidate.analysis_plan = Some(target.analysis_plan);
        candidate.reference_pvt = target.reference_pvt;
        candidate.run_set = target.run_set;
        candidate.model_bindings = target.model_bindings;
        candidate.save_policy = target.save_policy;
        candidate.options = target.options;
        candidate.validate_plan_catalog()?;
        *self = candidate;
        Ok(())
    }

    /// Advance the active plan revision for a committed variables, outputs,
    /// specifications, PVT, model-binding, or other plan-owned configuration
    /// change. This invalidates revision-pinned preflight evidence without
    /// pretending an analysis instance was edited.
    pub fn commit_active_plan_configuration_change(
        &mut self,
        detail: impl Into<String>,
    ) -> Result<crate::plan_model::SimulationPlanConfigurationReceipt, SimulationPlanCatalogError>
    {
        self.validate_plan_catalog()?;
        let plan = self
            .analysis_plan
            .as_mut()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        let receipt = plan.commit_configuration_change(detail)?;
        Ok(receipt)
    }

    /// Validate names, identities, lineage, and every active/inactive analysis
    /// graph. Editable incompleteness is permitted; structural corruption is
    /// rejected.
    pub fn validate_plan_catalog(&self) -> Result<(), SimulationPlanCatalogError> {
        let active = self
            .analysis_plan
            .as_ref()
            .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
        active.validate_structure()?;

        let mut names = HashSet::with_capacity(self.plan_count());
        names.insert(self.active_plan_name.uniqueness_key());
        let mut ids = HashSet::with_capacity(self.plan_count());
        ids.insert(active.id());
        if !self.active_plan_lineage.is_valid() {
            return Err(SimulationPlanCatalogError::InvalidLineage(active.id()));
        }
        validate_model_binding_list(&self.model_bindings)
            .map_err(SimulationPlanCatalogError::InvalidModelBindings)?;
        self.save_policy
            .validate()
            .map_err(SimulationPlanCatalogError::InvalidSavePolicy)?;

        for stored in &self.inactive_plans {
            if !names.insert(stored.name().uniqueness_key()) {
                return Err(SimulationPlanCatalogError::DuplicateName(
                    stored.name().to_string(),
                ));
            }
            if !ids.insert(stored.id()) {
                return Err(SimulationPlanCatalogError::DuplicatePlanIdentity(
                    stored.id(),
                ));
            }
            if !stored.lineage().is_valid() {
                return Err(SimulationPlanCatalogError::InvalidLineage(stored.id()));
            }
            validate_model_binding_list(stored.model_bindings())
                .map_err(SimulationPlanCatalogError::InvalidModelBindings)?;
            stored
                .save_policy()
                .validate()
                .map_err(SimulationPlanCatalogError::InvalidSavePolicy)?;
            stored.analysis_plan().validate_structure()?;
        }
        Ok(())
    }

    pub fn prepare_plan_catalog_after_restore(&mut self) {
        for plan in &mut self.inactive_plans {
            plan.prepare_after_restore();
        }
    }

    /// Migrate the former project-global model selection into every plan.
    /// Called only by the execution-context schema migration that predates
    /// per-plan bindings; a current explicit empty closure remains empty.
    pub fn migrate_legacy_model_bindings(
        &mut self,
        bindings: &[rspice_model_library::SimulationPlanModelBinding],
    ) {
        self.model_bindings = bindings.to_vec();
        for plan in &mut self.inactive_plans {
            plan.replace_model_bindings(bindings.to_vec());
        }
    }

    fn plan_identities(&self) -> HashSet<SimulationPlanId> {
        self.analysis_plan
            .iter()
            .map(SimulationPlan::id)
            .chain(self.inactive_plans.iter().map(StoredSimulationPlan::id))
            .collect()
    }

    fn ensure_plan_name_available(
        &self,
        name: &SimulationPlanName,
        except_id: Option<SimulationPlanId>,
    ) -> Result<(), SimulationPlanCatalogError> {
        let key = name.uniqueness_key();
        let active_conflicts = self
            .analysis_plan
            .as_ref()
            .is_some_and(|plan| Some(plan.id()) != except_id)
            && self.active_plan_name.uniqueness_key() == key;
        let inactive_conflicts = self
            .inactive_plans
            .iter()
            .any(|plan| Some(plan.id()) != except_id && plan.name().uniqueness_key() == key);
        if active_conflicts || inactive_conflicts {
            Err(SimulationPlanCatalogError::DuplicateName(name.to_string()))
        } else {
            Ok(())
        }
    }
}

fn take_active_plan_for_storage(
    setup: &mut SimulationSetupDocument,
) -> Result<StoredSimulationPlan, SimulationPlanCatalogError> {
    let plan = setup
        .analysis_plan
        .take()
        .ok_or(SimulationPlanCatalogError::ActivePlanUnavailable)?;
    let document = SimulationPlanImportDocument {
        source_plan_id: plan.id(),
        source_revision: plan.revision(),
        name: setup.active_plan_name.clone(),
        analysis_plan: plan,
        reference_pvt: setup.reference_pvt,
        run_set: setup.run_set.clone(),
        model_bindings: setup.model_bindings.clone(),
        save_policy: setup.save_policy,
        options: setup.options.clone(),
    };
    Ok(StoredSimulationPlan::from_document(
        document,
        setup.active_plan_lineage,
    ))
}

#[cfg(test)]
mod tests;
