//! Solver defaults for the actual stages of a configured study.
use super::*;
use crate::simulation::dialog::HbTimeDomainMode;
use crate::simulation::plan::{OverrideValue, SolverOwnership};

impl SimulationPlan {
    pub fn solver_ownership(&self, id: AnalysisInstanceId) -> SolverOwnership {
        self.instance(id).map_or(SolverOwnership::NONE, |instance| {
            self.solver_ownership_for_draft(instance.draft(), instance.numeric_override())
        })
    }

    pub fn solver_ownership_for_draft(
        &self,
        draft: &AnalysisDraft,
        defaults: Option<&AnalysisNumericOverride>,
    ) -> SolverOwnership {
        self.study_solver_ownership(draft, defaults)
            .unwrap_or_else(|| draft.solver_ownership_with_options(defaults))
    }

    fn study_solver_ownership(
        &self,
        draft: &AnalysisDraft,
        defaults: Option<&AnalysisNumericOverride>,
    ) -> Option<SolverOwnership> {
        let base_id = match draft {
            AnalysisDraft::MonteCarlo(state) => state.base_analysis?,
            AnalysisDraft::Optimization(state) => state.base_analysis?,
            AnalysisDraft::Temperature(state) => state.base_analysis?,
            AnalysisDraft::Corner(state) => state.base_analysis?,
            _ => return None,
        };
        let base = self.instance(base_id)?;
        if draft.pvt_base_analysis().is_some() && !base.kind().supports_pvt_base() {
            return None;
        }
        if !base.kind().supports_study_base() {
            return None;
        }
        // Match compile_study_base: ordinary AC/noise/DC studies run their
        // own bias solve; only periodic stages execute a configured OP seed.
        let producer = match base.kind() {
            AnalysisKind::Fourier | AnalysisKind::Fft => {
                self.study_bound_stage(base, &[AnalysisKind::Transient])?
            }
            kind if kind.inherits_periodic_solver_options() => self.study_bound_stage(
                base,
                &[
                    AnalysisKind::Pss,
                    AnalysisKind::HarmonicBalance,
                    AnalysisKind::Qpss,
                ],
            )?,
            _ => base,
        };
        let mut stages = vec![producer];
        let hb_startup = producer
            .numeric_override()
            .and_then(|record| record.stated(NumericOverrideOption::HbInitialState))
            .or_else(|| {
                defaults.and_then(|record| record.stated(NumericOverrideOption::HbInitialState))
            });
        let uses_seed = match producer.draft() {
            AnalysisDraft::Pss(_) => true,
            AnalysisDraft::Qpss(state) => state.dc_initialization,
            AnalysisDraft::HarmonicBalance(_) => !matches!(
                hb_startup,
                Some(OverrideValue::TimeDomainMode(HbTimeDomainMode::Direct))
            ),
            _ => false,
        };
        if uses_seed {
            stages.push(self.study_bound_stage(producer, &[AnalysisKind::OperatingPoint])?);
        }
        Some(SolverOwnership::for_study(
            NumericOverrideOption::all().filter(|&option| {
                let conflicting_schedule = match option {
                    NumericOverrideOption::StrobeInterval => {
                        Some(NumericOverrideOption::OutputTimePoints)
                    }
                    NumericOverrideOption::OutputTimePoints => {
                        Some(NumericOverrideOption::StrobeInterval)
                    }
                    _ => None,
                };
                // Unlike an ordinary same-key override, opposite schedule kinds
                // cannot coexist anywhere in the accumulated .OPTIONS record.
                if conflicting_schedule.is_some_and(|conflict| {
                    stages.iter().any(|stage| {
                        stage
                            .numeric_override()
                            .is_some_and(|record| record.stated(conflict).is_some())
                    })
                }) {
                    return false;
                }
                stages.iter().any(|stage| {
                    // OUTPUTTIMEPOINTS accumulates across .OPTIONS cards. Other
                    // per-stage values replace the corresponding study default.
                    if option != NumericOverrideOption::OutputTimePoints
                        && stage
                            .numeric_override()
                            .is_some_and(|record| record.stated(option).is_some())
                    {
                        return false;
                    }
                    // With no local startup selection, the source deck may
                    // select TAHB=1. Preserve those conditional defaults.
                    let assisted_hb = stage.kind() == AnalysisKind::HarmonicBalance
                        && !matches!(
                            hb_startup,
                            Some(OverrideValue::TimeDomainMode(
                                HbTimeDomainMode::Direct | HbTimeDomainMode::DcOperatingPoint
                            ))
                        );
                    if assisted_hb
                        && !matches!(
                            option,
                            NumericOverrideOption::StrobeInterval
                                | NumericOverrideOption::OutputTimePoints
                                | NumericOverrideOption::RetainEverySignal
                        )
                        && (option == NumericOverrideOption::MaximumTimestep
                            || option.refusal_for(AnalysisKind::Transient).is_none())
                    {
                        // TAHB=1 integrates a complete internal trajectory; its
                        // numerical controls apply, but reporting switches do not.
                        return true;
                    }
                    // A study ceiling tightens the transient form's max step via
                    // min; the local-editor duplicate rule must not discard it.
                    (option == NumericOverrideOption::MaximumTimestep
                        && stage.kind() == AnalysisKind::Transient)
                        || option
                            .refusal_for_instance(stage.kind(), stage.draft().solver_ownership())
                            .is_none()
                })
            }),
        ))
    }

    fn study_bound_stage(
        &self,
        consumer: &AnalysisInstance,
        kinds: &[AnalysisKind],
    ) -> Option<&AnalysisInstance> {
        let mut stages = consumer.dependencies().iter().filter_map(|edge| {
            let stage = self.instance(edge.target())?;
            (kinds.contains(&edge.prerequisite()) && stage.kind() == edge.prerequisite())
                .then_some(stage)
        });
        let stage = stages.next()?;
        stages.next().is_none().then_some(stage)
    }

    pub(super) fn validate_study_solver_options(&self) -> Result<(), AnalysisPlanError> {
        for instance in &self.instances {
            if let Some(record) = instance.numeric_override()
                && let Some(ownership) = self.study_solver_ownership(instance.draft(), Some(record))
                && let Some((option, reason)) =
                    record.first_refusal_for_instance(instance.kind(), ownership)
            {
                return Err(AnalysisPlanError::NumericOverrideNotApplicable {
                    id: instance.id(),
                    kind: instance.kind(),
                    option,
                    reason,
                });
            }
        }
        Ok(())
    }

    pub(super) fn restore_study_solver_options(&mut self) {
        // Resolve after every prerequisite draft has completed its migration.
        let ownership: Vec<_> = self
            .instances
            .iter()
            .map(|instance| {
                self.study_solver_ownership(instance.draft(), instance.numeric_override())
            })
            .collect();
        for (instance, ownership) in self.instances.iter_mut().zip(ownership) {
            if let Some(ownership) = ownership
                && let Some(record) = instance.numeric_override.as_mut()
            {
                for option in NumericOverrideOption::all() {
                    if option
                        .refusal_for_instance(instance.kind, ownership)
                        .is_some()
                    {
                        record.clear(option);
                    }
                }
                if record.is_empty() {
                    instance.numeric_override = None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
