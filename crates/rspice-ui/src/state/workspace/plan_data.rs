//! Per-plan payload data: specs, design variables, and saved outputs.
//!
//! Each simulation plan owns its own payload, and these operations keep that
//! ownership exact — reading or editing one plan's data never reaches into
//! another's, and cloning a plan deep-copies the payload so the two cannot
//! drift into sharing state. Older projects are migrated on access rather than
//! at load, so a plan that was never opened keeps its stored form untouched.

use super::*;

impl ProjectWorkspace {
    pub fn active_plan_data(&self, plan_id: SimulationPlanId) -> Option<&SimulationPlanPayload> {
        self.simulation_plan_payloads
            .iter()
            .find(|record| record.plan_id == plan_id)
            .map(|record| &record.payload)
    }

    pub fn active_plan_data_mut(
        &mut self,
        plan_id: SimulationPlanId,
    ) -> Option<&mut SimulationPlanPayload> {
        self.simulation_plan_payloads
            .iter_mut()
            .find(|record| record.plan_id == plan_id)
            .map(|record| &mut record.payload)
    }

    /// Deterministically seed a plan payload from the legacy active specs
    /// projection. Existing plan-owned data always wins.
    pub fn migrate_active_plan_data(&mut self, plan_id: SimulationPlanId) {
        if self.active_plan_data(plan_id).is_none() {
            self.simulation_plan_payloads
                .push(SimulationPlanPayloadRecord {
                    plan_id,
                    payload: SimulationPlanPayload {
                        specs: self.specs.clone(),
                        ..SimulationPlanPayload::default()
                    },
                });
        }
        self.sync_legacy_specs_projection(plan_id);
    }

    /// Seed a retained inactive plan without copying the active plan's legacy
    /// specification projection into a different ownership boundary.
    pub fn migrate_inactive_plan_data(&mut self, plan_id: SimulationPlanId) {
        if self.active_plan_data(plan_id).is_none() {
            self.simulation_plan_payloads
                .push(SimulationPlanPayloadRecord {
                    plan_id,
                    payload: SimulationPlanPayload::default(),
                });
        }
    }

    pub fn ensure_active_plan_data(
        &mut self,
        plan_id: SimulationPlanId,
    ) -> &mut SimulationPlanPayload {
        self.migrate_active_plan_data(plan_id);
        self.active_plan_data_mut(plan_id)
            .expect("migration inserts the requested plan payload")
    }

    pub fn active_specs(&self, plan_id: SimulationPlanId) -> &[SpecEntry] {
        self.active_plan_data(plan_id)
            .map_or(self.specs.as_slice(), |payload| payload.specs.as_slice())
    }

    pub fn replace_active_specs(&mut self, plan_id: SimulationPlanId, specs: Vec<SpecEntry>) {
        self.ensure_active_plan_data(plan_id).specs = specs.clone();
        self.specs = specs;
    }

    pub fn sync_legacy_specs_projection(&mut self, plan_id: SimulationPlanId) {
        if let Some(specs) = self
            .active_plan_data(plan_id)
            .map(|payload| payload.specs.clone())
        {
            self.specs = specs;
        }
    }

    pub fn add_design_variable(
        &mut self,
        plan_id: SimulationPlanId,
        variable: DesignVariable,
    ) -> Result<(), SimulationConfigurationError> {
        variable.validate().map_err(|message| {
            SimulationConfigurationError::InvalidDesignVariable {
                plan_id,
                index: self
                    .active_plan_data(plan_id)
                    .map_or(0, |payload| payload.design_variables.len()),
                message,
            }
        })?;
        let payload = self.ensure_active_plan_data(plan_id);
        if payload
            .design_variables
            .iter()
            .any(|existing| existing.name.eq_ignore_ascii_case(&variable.name))
        {
            return Err(SimulationConfigurationError::DesignVariableNameConflict {
                plan_id,
                name: variable.name,
            });
        }
        payload.design_variables.push(variable);
        Ok(())
    }

    /// Replace the expression of one plan-owned design variable as a single
    /// validated workspace transaction.
    ///
    /// The stable identity and all engineering metadata are retained. The
    /// variable revision advances exactly once for every committed update.
    /// Validation runs against a complete cloned workspace before assignment,
    /// so a malformed expression, range violation, revision exhaustion, or
    /// unrelated configuration invariant leaves the source workspace intact.
    pub fn update_design_variable_expression(
        &mut self,
        plan_id: SimulationPlanId,
        variable_id: DesignVariableId,
        expression: impl Into<String>,
    ) -> Result<ObjectRevision, SimulationConfigurationError> {
        let revisions =
            self.update_design_variable_expressions(plan_id, &[(variable_id, expression.into())])?;
        Ok(revisions[0].1)
    }

    /// Atomically replace multiple expressions in one active-plan payload.
    /// Every target is resolved and validated in the same candidate workspace,
    /// which avoids partial commits and scales independently of update count.
    pub fn update_design_variable_expressions(
        &mut self,
        plan_id: SimulationPlanId,
        updates: &[(DesignVariableId, String)],
    ) -> Result<Vec<(DesignVariableId, ObjectRevision)>, SimulationConfigurationError> {
        let mut candidate = self.clone();
        let payload = candidate
            .active_plan_data_mut(plan_id)
            .ok_or(SimulationConfigurationError::PlanPayloadMissing { plan_id })?;
        let mut seen = std::collections::HashSet::with_capacity(updates.len());
        let mut revisions = Vec::with_capacity(updates.len());
        for (variable_id, expression) in updates {
            if !seen.insert(*variable_id) {
                return Err(
                    SimulationConfigurationError::DuplicateDesignVariableUpdate {
                        plan_id,
                        variable_id: *variable_id,
                    },
                );
            }
            let index = payload
                .design_variables
                .iter()
                .position(|variable| variable.id == *variable_id)
                .ok_or(SimulationConfigurationError::DesignVariableNotFound {
                    plan_id,
                    variable_id: *variable_id,
                })?;
            let variable = &mut payload.design_variables[index];
            let next_revision = variable.revision.next().map_err(|source| {
                SimulationConfigurationError::DesignVariableRevision {
                    plan_id,
                    variable_id: *variable_id,
                    source,
                }
            })?;
            variable.expression.clone_from(expression);
            variable.revision = next_revision;
            variable.validate().map_err(|message| {
                SimulationConfigurationError::InvalidDesignVariable {
                    plan_id,
                    index,
                    message,
                }
            })?;
            revisions.push((*variable_id, next_revision));
        }
        candidate.validate_simulation_configuration()?;

        *self = candidate;
        Ok(revisions)
    }

    pub fn add_saved_output(
        &mut self,
        plan_id: SimulationPlanId,
        output: SavedOutput,
    ) -> Result<(), SimulationConfigurationError> {
        output
            .validate()
            .map_err(|message| SimulationConfigurationError::InvalidSavedOutput {
                plan_id,
                index: self
                    .active_plan_data(plan_id)
                    .map_or(0, |payload| payload.saved_outputs.len()),
                message,
            })?;
        let payload = self.ensure_active_plan_data(plan_id);
        if payload
            .saved_outputs
            .iter()
            .any(|existing| existing.name.eq_ignore_ascii_case(&output.name))
        {
            return Err(SimulationConfigurationError::SavedOutputNameConflict {
                plan_id,
                name: output.name,
            });
        }
        payload.saved_outputs.push(output);
        Ok(())
    }

    /// Copy or initialize the payload for a newly cloned plan. The insertion
    /// is atomic with respect to validation: no target record is created on
    /// any error.
    pub fn clone_plan_data(
        &mut self,
        source_plan_id: SimulationPlanId,
        cloned_plan_id: SimulationPlanId,
        copy_variables_outputs_specs: bool,
        copy_regression_baseline: bool,
        analysis_identity_map: &[(AnalysisInstanceId, AnalysisInstanceId)],
    ) -> Result<(), SimulationConfigurationError> {
        if self.active_plan_data(cloned_plan_id).is_some() {
            return Err(SimulationConfigurationError::PlanPayloadAlreadyExists {
                plan_id: cloned_plan_id,
            });
        }
        let source = (copy_variables_outputs_specs || copy_regression_baseline)
            .then(|| {
                self.active_plan_data(source_plan_id).cloned().ok_or(
                    SimulationConfigurationError::PlanPayloadMissing {
                        plan_id: source_plan_id,
                    },
                )
            })
            .transpose()?;
        let remap = analysis_identity_map
            .iter()
            .copied()
            .collect::<HashMap<_, _>>();
        let (design_variables, saved_outputs, specs) = if copy_variables_outputs_specs {
            let source = source.as_ref().expect("copy request resolves source above");
            let design_variables = source
                .design_variables
                .iter()
                .map(|variable| variable.cloned_for_new_plan(&remap))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|analysis_id| {
                    SimulationConfigurationError::MissingClonedAnalysisMapping { analysis_id }
                })?;
            let saved_outputs = source
                .saved_outputs
                .iter()
                .map(|output| output.cloned_for_new_plan(&remap))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|analysis_id| {
                    SimulationConfigurationError::MissingClonedAnalysisMapping { analysis_id }
                })?;
            (design_variables, saved_outputs, source.specs.clone())
        } else {
            (Vec::new(), Vec::new(), Vec::new())
        };
        let (regression_baseline_run, regression_tolerances) = if copy_regression_baseline {
            let source = source.as_ref().expect("copy request resolves source above");
            let tolerances = source
                .regression_tolerances
                .iter()
                .map(|rule| rule.cloned_for_new_plan(&remap))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|analysis_id| {
                    SimulationConfigurationError::MissingClonedAnalysisMapping { analysis_id }
                })?;
            (source.regression_baseline_run, tolerances)
        } else {
            (None, Vec::new())
        };
        let payload = SimulationPlanPayload {
            design_variables,
            saved_outputs,
            specs,
            regression_baseline_run,
            regression_tolerances,
        };
        self.simulation_plan_payloads
            .push(SimulationPlanPayloadRecord {
                plan_id: cloned_plan_id,
                payload,
            });
        self.sync_legacy_specs_projection(cloned_plan_id);
        Ok(())
    }
}
