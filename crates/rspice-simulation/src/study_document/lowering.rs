use std::collections::HashMap;
use std::path::Path;

use rspice_app_types::product::AnalysisInstanceId;
use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits};

use super::*;
use crate::execution::{HeadlessRunInput, HeadlessSourceResolver, HeadlessTaskRequest, SavePolicy};
use crate::execution_options::SpecExecutionOptions;
use crate::preparation::QueuedAnalysis;

impl StudyDocument {
    /// Lower named requests without filesystem access. The host can then bind
    /// captured model runtimes and measurement files before calling
    /// [`crate::execution::prepare_headless_run`]. Graph and source validation
    /// still precede authorization; this value is not a dispatch token.
    pub fn run_input<'a>(
        &self,
        source: &'a str,
        origin: &'a Path,
        resolver: HeadlessSourceResolver,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<HeadlessRunInput<'a>, StudyDocumentError> {
        let tasks = self.lower_tasks(limits, abort)?;
        let mut input = HeadlessRunInput::new(source, origin, tasks);
        input.resolver = resolver;
        input.preparation_limits = limits;
        input.saved_outputs = self.saved_outputs.clone();
        if let Some(policy) = self.save_policy {
            input.save_policy = SavePolicy::PlanOwned {
                output_selection_mode: policy.output_selection_mode,
                retained_dataset_limit: policy.retained_dataset_limit,
                maximum_storage_bytes: policy.maximum_storage_bytes,
                live_streaming_enabled: policy.live_streaming_enabled,
                retain_failure_diagnostics: policy.retain_failure_diagnostics,
            };
        }
        Ok(input)
    }

    pub fn lower_tasks(
        &self,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<HeadlessTaskRequest>, StudyDocumentError> {
        check_abort(abort)?;
        self.validate_header()?;
        check_limit(
            ResourceKind::BatchRuns,
            self.tasks.len(),
            limits.max_batch_runs,
        )?;
        if self.tasks.is_empty() {
            return Err(invalid("tasks must contain at least one analysis"));
        }
        let mut identities = HashMap::with_capacity(self.tasks.len());
        let mut edges = 0usize;
        for task in &self.tasks {
            check_abort(abort)?;
            validate_task_name(&task.id)?;
            let name = format!("rspice.study-task/v1\0{}", task.id);
            let identity = AnalysisInstanceId::from_namespace(self.id.as_uuid(), name.as_bytes());
            if identities.insert(task.id.as_str(), identity).is_some() {
                return Err(invalid(format!("duplicate task name {:?}", task.id)));
            }
            edges = edges.saturating_add(task.depends_on.len());
            check_limit(ResourceKind::ResultValues, edges, limits.max_result_values)?;
        }
        self.tasks
            .iter()
            .map(|task| {
                check_abort(abort)?;
                let context = |message: String| invalid(format!("task {:?}: {message}", task.id));
                task.analysis.validate().map_err(&context)?;
                let options = task.execution_options().map_err(&context)?;
                let line = analysis_cards(&task.analysis).map_err(&context)?;
                let numeric = task.effective_numeric(&self.numeric).map_err(&context)?;
                let dependencies = task
                    .depends_on
                    .iter()
                    .map(|name| {
                        check_abort(abort)?;
                        identities
                            .get(name.as_str())
                            .copied()
                            .ok_or_else(|| context(format!("depends on missing task {name:?}")))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let label = task.label.clone().unwrap_or_else(|| task.id.clone());
                if label.trim().is_empty() {
                    return Err(context("label must not be empty".into()));
                }
                Ok(HeadlessTaskRequest {
                    instance_id: identities[task.id.as_str()],
                    label,
                    dependencies,
                    analysis: QueuedAnalysis {
                        spec: task.analysis.clone(),
                        config: None,
                        spec_options: options,
                        analysis_line: line,
                        numeric_override: (!numeric.is_empty()).then_some(numeric),
                    },
                })
            })
            .collect()
    }
}

impl StudyTask {
    fn effective_numeric(
        &self,
        global: &AnalysisNumericOverride,
    ) -> Result<AnalysisNumericOverride, String> {
        use rspice_simulation_contract::numeric_override::NumericOverrideOption;
        let merged = global.clone().with_base_options(&self.numeric);
        let ownership = solver_ownership(&self.analysis, &merged);
        let canonical = crate::execution_identity::canonical_analysis_kind(&self.analysis);
        let kind = rspice_simulation_contract::analysis_kind::AnalysisKind::ALL
            .into_iter()
            .find(|kind| kind.canonical_kind() == canonical);
        let Some(kind) = kind else {
            // A retained PSS spectrum performs no solve of its own.
            if !self.numeric.is_empty() {
                return Err("this result projection does not accept numerical overrides".into());
            }
            return Ok(AnalysisNumericOverride::default());
        };
        if let Some((option, reason)) = self.numeric.first_refusal_for_instance(kind, ownership) {
            return Err(format!(
                "{} cannot be set for this analysis: {reason}",
                option.key()
            ));
        }
        let mut checked = AnalysisNumericOverride::default();
        for option in NumericOverrideOption::all() {
            let Some(value) = merged.stated(option) else {
                continue;
            };
            // Global defaults apply only to analyses that consume the option.
            // In contrast, an explicit task override must be applicable.
            if option.refusal_for_instance(kind, ownership).is_some() {
                continue;
            }
            let spelling = value.to_deck_text().ok_or_else(|| {
                format!(
                    "{} has no explicit solver value; omit it to inherit",
                    option.key()
                )
            })?;
            checked.set_for_instance(kind, ownership, option, &spelling)?;
        }
        Ok(checked)
    }

    fn execution_options(&self) -> Result<SpecExecutionOptions, String> {
        let mut options = SpecExecutionOptions::default();
        match (&self.analysis, &self.periodic) {
            (AnalysisSpec::Pac, Some(StudyPeriodicSettings::Pac(config))) => {
                config.validate()?;
                options.pac = Some(config.clone());
            }
            (AnalysisSpec::Pxf, Some(StudyPeriodicSettings::Pxf(config))) => {
                config.validate()?;
                options.pxf = Some(config.clone());
            }
            (AnalysisSpec::Pnoise, Some(StudyPeriodicSettings::Pnoise(config))) => {
                config.validate().map_err(|error| error.to_string())?;
                options.pnoise = Some(config.clone());
            }
            (AnalysisSpec::Pstb, Some(StudyPeriodicSettings::Pstb(config))) => {
                config.validate()?;
                options.pstb = Some(config.clone());
            }
            (
                AnalysisSpec::Pac | AnalysisSpec::Pxf | AnalysisSpec::Pnoise | AnalysisSpec::Pstb,
                _,
            ) => {
                return Err("periodic settings must explicitly match the analysis kind".into());
            }
            (_, Some(_)) => return Err("this analysis does not accept periodic settings".into()),
            (_, None) => {}
        }
        Ok(options)
    }
}

fn solver_ownership(
    spec: &AnalysisSpec,
    numeric: &AnalysisNumericOverride,
) -> rspice_simulation_contract::numeric_override::SolverOwnership {
    use rspice_simulation_contract::analysis_spec::EnvelopeInitialPeriodicSolve;
    use rspice_simulation_contract::numeric_override::{
        NumericOverrideOption, OverrideValue, SolverOwnership,
    };
    use rspice_simulation_contract::options::HbTimeDomainMode;
    match spec {
        AnalysisSpec::LegacyDcOp => {
            let config = rspice_simulation_contract::config::OpConfig::default();
            SolverOwnership {
                accuracy: Some(config.accuracy),
                homotopy: Some(config.homotopy),
                ..SolverOwnership::NONE
            }
        }
        AnalysisSpec::DcOp {
            accuracy, homotopy, ..
        } => SolverOwnership {
            accuracy: Some(*accuracy),
            homotopy: Some(*homotopy),
            ..SolverOwnership::NONE
        },
        AnalysisSpec::Tf { accuracy, .. } => SolverOwnership {
            accuracy: Some(*accuracy),
            ..SolverOwnership::NONE
        },
        AnalysisSpec::Optimization { .. } | AnalysisSpec::MonteCarlo { .. } => SolverOwnership {
            time_integration: Some(false),
            ..SolverOwnership::NONE
        },
        AnalysisSpec::HarmonicBalance { .. } => SolverOwnership {
            time_integration: Some(matches!(
                numeric.stated(NumericOverrideOption::HbInitialState),
                Some(OverrideValue::TimeDomainMode(
                    HbTimeDomainMode::TransientAssisted
                ))
            )),
            ..SolverOwnership::NONE
        },
        AnalysisSpec::Envelope {
            multirate,
            initial_periodic_solve,
            ..
        } => SolverOwnership {
            multirate_envelope_dc: multirate.as_ref().map(|config| config.dc_initialization),
            hb_initializer: Some(
                multirate.is_none()
                    && matches!(
                        initial_periodic_solve,
                        EnvelopeInitialPeriodicSolve::HarmonicBalance
                    ),
            ),
            ..SolverOwnership::NONE
        },
        _ => SolverOwnership::NONE,
    }
}

fn analysis_cards(spec: &AnalysisSpec) -> Result<String, String> {
    use crate::analysis_preparation::{
        analysis_spec_to_config, build_ac_data_command, build_transient_noise_command,
    };
    match spec {
        AnalysisSpec::Fft { request } => Ok(request.to_card()),
        AnalysisSpec::TransientNoise { .. } => build_transient_noise_command(spec),
        AnalysisSpec::AcData { .. } => build_ac_data_command(spec),
        AnalysisSpec::LegacyDcOp
        | AnalysisSpec::DcOp { .. }
        | AnalysisSpec::DcSweep { .. }
        | AnalysisSpec::Ac { .. }
        | AnalysisSpec::Transient { .. }
        | AnalysisSpec::Noise { .. }
        | AnalysisSpec::PoleZero { .. }
        | AnalysisSpec::Sensitivity { .. } => {
            analysis_spec_to_config(spec).map(|config| config.to_spice())
        }
        AnalysisSpec::MonteCarlo { .. } | AnalysisSpec::Parametric | AnalysisSpec::Corner => {
            Err("structured sweep settings are not yet supported by study documents".into())
        }
        // These runners consume their complete typed specification. No second
        // source-level request is necessary, and no user-authored snippet can
        // introduce an option or competing analysis after the frozen settings.
        _ => Ok(String::new()),
    }
}

fn validate_task_name(name: &str) -> Result<(), StudyDocumentError> {
    if name.is_empty()
        || name.len() > 128
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
    {
        return Err(invalid(format!(
            "task name {name:?} must contain 1–128 ASCII letters, digits, dots, underscores or hyphens"
        )));
    }
    Ok(())
}

fn check_limit(
    resource: ResourceKind,
    requested: usize,
    limit: usize,
) -> Result<(), StudyDocumentError> {
    if requested > limit {
        Err(ResourceLimitError {
            resource,
            requested,
            limit,
        }
        .into())
    } else {
        Ok(())
    }
}
