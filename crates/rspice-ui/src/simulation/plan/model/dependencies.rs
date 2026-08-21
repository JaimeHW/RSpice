//! Binding, auto-binding, and repairing analysis prerequisites.
//!
//! A dependency edge names an exact prerequisite instance, never a kind, so a
//! binding cannot silently re-point at a different analysis that happens to
//! match. Auto-binding only fills a role it can satisfy exactly and leaves the
//! rest unbound, and a repair moves the whole dependency closure together so
//! the plan is never left with an edge that points forward in run order.

use super::*;

impl SimulationPlan {
    /// Bind or replace one exact prerequisite role.
    pub fn bind_dependency(
        &mut self,
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        target: AnalysisInstanceId,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let dependent_instance = self
            .instance(dependent)
            .ok_or(AnalysisPlanError::InstanceNotFound(dependent))?;
        let kind = dependent_instance.kind();
        let outcome = if dependent_instance.enabled() {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Dependency,
            dependent,
            Some(target),
            outcome,
            format!(
                "{} analysis {dependent} bound {target} as its {} prerequisite.",
                kind.label(),
                prerequisite.label()
            ),
            move |candidate, revision| {
                let dependent_index = candidate.index_of(dependent)?;
                candidate.ensure_editable(dependent_index)?;
                if !candidate.instances[dependent_index]
                    .prerequisite_roles()
                    .contains(&prerequisite)
                {
                    return Err(AnalysisPlanError::UnexpectedDependencyRole {
                        dependent,
                        prerequisite,
                    });
                }
                if dependent == target {
                    return Err(AnalysisPlanError::SelfDependency { dependent });
                }
                let target_index = candidate.index_of(target).map_err(|_| {
                    AnalysisPlanError::DependencyTargetMissing { dependent, target }
                })?;
                let target_instance = &candidate.instances[target_index];
                if target_instance.kind != prerequisite {
                    return Err(AnalysisPlanError::DependencyTargetWrongKind {
                        dependent,
                        target,
                        expected: prerequisite,
                        actual: target_instance.kind,
                    });
                }
                if !target_instance.enabled {
                    return Err(AnalysisPlanError::DependencyTargetDisabled { dependent, target });
                }
                if target_index >= dependent_index {
                    return Err(AnalysisPlanError::DependencyTargetNotEarlier {
                        dependent,
                        target,
                    });
                }
                if let Some(issue) = dependency_configuration_issue(
                    &candidate.instances[dependent_index].draft,
                    &target_instance.draft,
                ) {
                    return Err(AnalysisPlanError::DependencyConfigurationInvalid {
                        dependent,
                        prerequisite,
                        detail: issue.detail().to_owned(),
                    });
                }
                let instance = &mut candidate.instances[dependent_index];
                instance
                    .dependencies
                    .retain(|dependency| dependency.prerequisite != prerequisite);
                instance
                    .dependencies
                    .push(AnalysisDependency::new(prerequisite, target));
                instance
                    .dependencies
                    .sort_by_key(|dependency| dependency.prerequisite.legacy_index());
                instance.modified_revision = revision;
                instance.lifecycle = if instance.enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
                Ok(())
            },
        )?;
        Ok(receipt)
    }

    /// Deterministically bind every required role to the latest matching,
    /// enabled instance that appears earlier. Unresolved roles remain explicit
    /// validation issues rather than making this convenience command partial.
    #[cfg(test)]
    pub fn auto_bind_dependencies(
        &mut self,
        dependent: AnalysisInstanceId,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        self.auto_bind_dependencies_with_context(
            dependent,
            &AnalysisDependencyRepairContext::default(),
        )
    }

    /// Bind every role to the latest compatible earlier instance while also
    /// enforcing circuit-derived prerequisite identity constraints.
    pub fn auto_bind_dependencies_with_context(
        &mut self,
        dependent: AnalysisInstanceId,
        context: &AnalysisDependencyRepairContext,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let dependent_instance = self
            .instance(dependent)
            .ok_or(AnalysisPlanError::InstanceNotFound(dependent))?;
        let kind = dependent_instance.kind();
        if dependent_instance
            .prerequisite_roles()
            .contains(&AnalysisKind::Pss)
            && let Some(detail) = context.availability_error()
        {
            return Err(AnalysisPlanError::DependencyConfigurationInvalid {
                dependent,
                prerequisite: AnalysisKind::Pss,
                detail: format!(
                    "automatic binding requires the authenticated periodic-source circuit: {detail}"
                ),
            });
        }
        let outcome = if dependent_instance.enabled() {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        let context = context.clone();
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Dependency,
            dependent,
            None,
            outcome,
            format!(
                "Dependency bindings for {} analysis {dependent} were refreshed from enabled earlier instances.",
                kind.label()
            ),
            move |candidate, revision| {
                let dependent_index = candidate.index_of(dependent)?;
                candidate.ensure_editable(dependent_index)?;
                let prerequisites = candidate.instances[dependent_index]
                    .prerequisite_roles()
                    .to_vec();
                let dependencies = prerequisites
                    .into_iter()
                    .filter_map(|prerequisite| {
                        candidate.instances[..dependent_index]
                            .iter()
                            .rev()
                            .find(|instance| {
                                instance.enabled
                                    && Self::dependency_candidate_compatibility(
                                        &candidate.instances[dependent_index],
                                        prerequisite,
                                        instance,
                                    )
                                    .is_ok_and(|compatible| compatible)
                                    && dependency_candidate_context_issue(
                                        prerequisite,
                                        &instance.draft,
                                        &context,
                                    )
                                    .is_none()
                            })
                            .map(|target| AnalysisDependency::new(prerequisite, target.id))
                    })
                    .collect();
                let instance = &mut candidate.instances[dependent_index];
                instance.dependencies = dependencies;
                instance.modified_revision = revision;
                instance.lifecycle = if instance.enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
                Ok(())
            },
        )?;
        Ok(receipt)
    }

    /// Atomically make every prerequisite in one instance's transitive closure
    /// executable. Existing matching identities are preferred over insertion;
    /// disabled matches are enabled, later matches and their dependency closure
    /// are moved before their consumer, absent kinds are inserted immediately
    /// before their consumer, and every exact role is rebound.
    ///
    /// The complete repair commits as one revision and one lifecycle receipt.
    /// Any identity, lifecycle, revision, or graph failure leaves the original
    /// plan unchanged.
    #[cfg(test)]
    pub fn repair_dependencies(
        &mut self,
        dependent: AnalysisInstanceId,
    ) -> Result<(AnalysisDependencyRepair, AnalysisLifecycleReceipt), AnalysisPlanError> {
        self.repair_dependencies_with_context(
            dependent,
            &AnalysisDependencyRepairContext::default(),
        )
    }

    /// Repair a complete prerequisite closure using an authenticated circuit
    /// context for any synthesized or reused circuit-specific prerequisite.
    pub fn repair_dependencies_with_context(
        &mut self,
        dependent: AnalysisInstanceId,
        context: &AnalysisDependencyRepairContext,
    ) -> Result<(AnalysisDependencyRepair, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let dependent_instance = self
            .instance(dependent)
            .ok_or(AnalysisPlanError::InstanceNotFound(dependent))?;
        let kind = dependent_instance.kind();
        let outcome = if dependent_instance.enabled() {
            AnalysisLifecycleState::Draft
        } else {
            AnalysisLifecycleState::Disabled
        };
        let context = context.clone();
        self.transact(
            AnalysisLifecycleCommand::Dependency,
            dependent,
            None,
            outcome,
            format!(
                "The complete prerequisite closure for {} analysis {dependent} was repaired atomically.",
                kind.label()
            ),
            move |candidate, revision| {
                let dependent_index = candidate.index_of(dependent)?;
                candidate.ensure_editable(dependent_index)?;
                let mut repair = AnalysisDependencyRepair::new(dependent);
                let mut visiting = Vec::new();
                candidate.repair_dependency_closure(
                    dependent,
                    revision,
                    &mut repair,
                    &mut visiting,
                    &context,
                )?;
                repair.sort_in_final_order(&candidate.instances);
                Ok(repair)
            },
        )
    }

    /// Atomically insert a disabled prerequisite draft immediately before its
    /// consumer and prepare every machine-inferable dependency beneath it.
    /// This is the guided path for
    /// prerequisites that require user-owned circuit data which cannot be
    /// inferred safely (for example, an autonomous PSS oscillator node).
    /// The returned prerequisite is selected for configuration by the caller.
    /// Its consumer role remains deliberately unbound until the draft is valid
    /// and enabled, after which ordinary atomic dependency repair reuses it.
    pub fn prepare_prerequisite_for_configuration(
        &mut self,
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        draft: AnalysisDraft,
        context: &AnalysisDependencyRepairContext,
    ) -> Result<(AnalysisInstanceId, AnalysisLifecycleReceipt), AnalysisPlanError> {
        let actual = draft.kind();
        if actual != prerequisite {
            return Err(AnalysisPlanError::DraftKindMismatch {
                expected: prerequisite,
                actual,
            });
        }
        let dependent_instance = self
            .instance(dependent)
            .ok_or(AnalysisPlanError::InstanceNotFound(dependent))?;
        if !dependent_instance
            .prerequisite_roles()
            .contains(&prerequisite)
        {
            return Err(AnalysisPlanError::UnexpectedDependencyRole {
                dependent,
                prerequisite,
            });
        }
        let id = self.fresh_identity();
        let context = context.clone();
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Insert,
            id,
            Some(dependent),
            AnalysisLifecycleState::Disabled,
            format!(
                "Disabled {} prerequisite {id} was inserted before analysis {dependent} with its inferable dependency closure and is ready for configuration.",
                prerequisite.label()
            ),
            move |candidate, revision| {
                candidate.ensure_identity_available(id)?;
                let dependent_index = candidate.index_of(dependent)?;
                candidate.ensure_editable(dependent_index)?;
                candidate.instances.insert(
                    dependent_index,
                    AnalysisInstance::fresh(id, None, draft, false, Vec::new(), None, revision),
                );
                let dependent_index = candidate.index_of(dependent)?;
                let dependent_instance = &mut candidate.instances[dependent_index];
                dependent_instance
                    .dependencies
                    .retain(|dependency| dependency.prerequisite != prerequisite);
                dependent_instance.modified_revision = revision;
                dependent_instance.lifecycle = if dependent_instance.enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
                let mut repair = AnalysisDependencyRepair::new(id);
                let mut visiting = Vec::new();
                candidate.repair_dependency_closure(
                    id,
                    revision,
                    &mut repair,
                    &mut visiting,
                    &context,
                )?;
                Ok(())
            },
        )?;
        Ok((id, receipt))
    }

    fn repair_dependency_closure(
        &mut self,
        dependent: AnalysisInstanceId,
        revision: ObjectRevision,
        repair: &mut AnalysisDependencyRepair,
        visiting: &mut Vec<AnalysisInstanceId>,
        context: &AnalysisDependencyRepairContext,
    ) -> Result<(), AnalysisPlanError> {
        if let Some(cycle_start) = visiting.iter().position(|id| *id == dependent) {
            let mut members = visiting[cycle_start..].to_vec();
            members.push(dependent);
            return Err(AnalysisPlanError::InvalidPlan(vec![
                AnalysisPlanIssue::DependencyCycle { members },
            ]));
        }
        let dependent_index = self.index_of(dependent)?;
        self.ensure_editable(dependent_index)?;
        visiting.push(dependent);
        let prerequisites = self.instances[dependent_index]
            .prerequisite_roles()
            .to_vec();
        let removed = self.instances[dependent_index]
            .dependencies
            .iter()
            .copied()
            .filter(|dependency| !prerequisites.contains(&dependency.prerequisite))
            .collect::<Vec<_>>();
        if !removed.is_empty() {
            let instance = &mut self.instances[dependent_index];
            instance
                .dependencies
                .retain(|dependency| prerequisites.contains(&dependency.prerequisite));
            instance.modified_revision = revision;
            instance.lifecycle = if instance.enabled {
                AnalysisLifecycleState::Draft
            } else {
                AnalysisLifecycleState::Disabled
            };
            repair.removed.extend(removed);
        }

        for prerequisite in prerequisites {
            let target = self.repair_target(dependent, prerequisite, revision, repair, context)?;
            self.repair_dependency_closure(target, revision, repair, visiting, context)?;
            self.move_dependency_closure_before(target, dependent, revision, repair)?;

            let dependent_index = self.index_of(dependent)?;
            let role_bindings = self.instances[dependent_index]
                .dependencies
                .iter()
                .filter(|dependency| dependency.prerequisite == prerequisite)
                .collect::<Vec<_>>();
            let exactly_bound = role_bindings.len() == 1 && role_bindings[0].target == target;
            if !exactly_bound {
                let instance = &mut self.instances[dependent_index];
                instance
                    .dependencies
                    .retain(|dependency| dependency.prerequisite != prerequisite);
                let dependency = AnalysisDependency::new(prerequisite, target);
                instance.dependencies.push(dependency);
                instance
                    .dependencies
                    .sort_by_key(|dependency| dependency.prerequisite.legacy_index());
                instance.modified_revision = revision;
                instance.lifecycle = if instance.enabled {
                    AnalysisLifecycleState::Draft
                } else {
                    AnalysisLifecycleState::Disabled
                };
                repair.bound.push(dependency);
            }
        }
        visiting.pop();
        Ok(())
    }

    fn repair_target(
        &mut self,
        dependent: AnalysisInstanceId,
        prerequisite: AnalysisKind,
        revision: ObjectRevision,
        repair: &mut AnalysisDependencyRepair,
        context: &AnalysisDependencyRepairContext,
    ) -> Result<AnalysisInstanceId, AnalysisPlanError> {
        let dependent_index = self.index_of(dependent)?;
        let dependent_draft = self.instances[dependent_index].draft.clone();
        let is_compatible = |candidate: &AnalysisInstance| {
            let dependent_instance = &self.instances[dependent_index];
            Self::dependency_candidate_compatibility(dependent_instance, prerequisite, candidate)
                .is_ok_and(|compatible| compatible)
                && dependency_candidate_context_issue(prerequisite, &candidate.draft, context)
                    .is_none()
        };
        let explicit_target = self.instances[dependent_index]
            .dependencies
            .iter()
            .find(|dependency| dependency.prerequisite == prerequisite)
            .and_then(|dependency| {
                self.instances
                    .iter()
                    .find(|candidate| candidate.id == dependency.target && is_compatible(candidate))
                    .map(|candidate| candidate.id)
            });
        let target = explicit_target.or_else(|| {
            self.instances[..dependent_index]
                .iter()
                .rev()
                .find(|candidate| candidate.enabled && is_compatible(candidate))
                .or_else(|| {
                    self.instances[..dependent_index]
                        .iter()
                        .rev()
                        .find(|candidate| is_compatible(candidate))
                })
                .or_else(|| {
                    self.instances[dependent_index + 1..]
                        .iter()
                        .find(|candidate| candidate.enabled && is_compatible(candidate))
                })
                .or_else(|| {
                    self.instances[dependent_index + 1..]
                        .iter()
                        .find(|candidate| is_compatible(candidate))
                })
                .map(|candidate| candidate.id)
        });

        let target = match target {
            Some(target) => target,
            None => {
                let target = self.fresh_identity();
                let draft = prerequisite_draft_for(&dependent_draft, prerequisite, context)
                    .map_err(|detail| AnalysisPlanError::DependencyConfigurationInvalid {
                        dependent,
                        prerequisite,
                        detail,
                    })?;
                self.instances.insert(
                    dependent_index,
                    AnalysisInstance::fresh(target, None, draft, true, Vec::new(), None, revision),
                );
                repair.inserted.push(target);
                target
            }
        };

        let target_index = self.index_of(target)?;
        if !self.instances[target_index].enabled {
            self.ensure_editable(target_index)?;
            let instance = &mut self.instances[target_index];
            instance.enabled = true;
            instance.lifecycle = AnalysisLifecycleState::Draft;
            instance.modified_revision = revision;
            if !repair.enabled.contains(&target) {
                repair.enabled.push(target);
            }
        }
        Ok(target)
    }

    fn move_dependency_closure_before(
        &mut self,
        target: AnalysisInstanceId,
        dependent: AnalysisInstanceId,
        revision: ObjectRevision,
        repair: &mut AnalysisDependencyRepair,
    ) -> Result<(), AnalysisPlanError> {
        let dependencies = self
            .instance(target)
            .ok_or(AnalysisPlanError::InstanceNotFound(target))?
            .dependencies
            .iter()
            .map(|dependency| dependency.target)
            .collect::<Vec<_>>();
        for prerequisite in dependencies {
            self.move_dependency_closure_before(prerequisite, dependent, revision, repair)?;
        }

        let target_index = self.index_of(target)?;
        let dependent_index = self.index_of(dependent)?;
        if target_index > dependent_index {
            self.ensure_editable(target_index)?;
            let mut instance = self.instances.remove(target_index);
            let dependent_index = self.index_of(dependent)?;
            instance.modified_revision = revision;
            self.instances.insert(dependent_index, instance);
            if !repair.moved.contains(&target) {
                repair.moved.push(target);
            }
        }
        Ok(())
    }
}
