//! Transactional mutations for the measurement-correlation workspace.
//!
//! Every operation builds a complete candidate aggregate against the exact
//! current project-owned model source, validates it through the correlation
//! domain, and publishes it through project history once. Runtime dialog
//! drafts are never authoritative project state.

use csv::{Terminator, WriterBuilder};
use sha2::{Digest as _, Sha256};

use crate::product::{ContentDigest, ModelSourceId, ObjectRevision, RunId};
use crate::state::model_library::{
    CorrelationAggregation, CorrelationAlignmentPolicy, CorrelationCalculation,
    CorrelationDatasetClass, CorrelationDatasetRevision, CorrelationEvaluation,
    CorrelationEvidence, CorrelationExtrapolationPolicy, CorrelationMetricDefinition,
    CorrelationMetricDomain, CorrelationOutlierDecision, CorrelationOutlierDisposition,
    CorrelationReleaseRole, CorrelationReviewDecision, CorrelationSimulationProvenance,
    CorrelationSuite, FiniteValue, MAX_CORRELATION_ROWS, MAX_CORRELATION_TEXT_BYTES,
    ModelCorrelationState, ModelSourceEvidenceBinding, NonNegativeFinite,
};
use crate::state::{AnalysisType, SimulationRunLifecycle, WaveformData};
use crate::workbench::RSpiceApp;

use crate::workbench::documents::model_correlation::{
    CorrelationAggregationDraft, CorrelationAlignmentDraft, CorrelationCalculationDraft,
    CorrelationDatasetClassDraft, CorrelationDatasetDraft, CorrelationExtrapolationDraft,
    CorrelationMetricDraft, CorrelationOutlierDecisionDraft, CorrelationOutlierDraft,
    CorrelationReleaseRoleDraft, CorrelationReviewDecisionDraft, CorrelationReviewDraft,
};
use crate::workbench::documents::model_editor::resolve_project_model_for_editor;

#[derive(Debug, Clone)]
struct CorrelationContext {
    library_name: String,
    model_name: String,
    source_id: ModelSourceId,
    library_revision: ObjectRevision,
    model_revision: ObjectRevision,
    model_digest: ContentDigest,
    source: ModelSourceEvidenceBinding,
    correlation: ModelCorrelationState,
}

#[derive(Debug, Clone)]
struct PublicationSelection {
    suite_id: String,
    dataset_id: Option<String>,
    metric_id: Option<String>,
    notice: String,
}

/// Resolve the exact project-owned source identity currently selected by the
/// correlation workspace.
pub(super) fn current_source_binding(
    app: &RSpiceApp,
) -> Result<ModelSourceEvidenceBinding, String> {
    current_context(app).map(|context| context.source)
}

/// Append one immutable dataset revision by publishing a new suite revision.
pub(super) fn append_dataset(
    app: &mut RSpiceApp,
    draft: &CorrelationDatasetDraft,
) -> Result<(), String> {
    preflight_mutation(app)?;
    let context = current_context(app)?;
    require_dialog_source(app, &context.source)?;
    let selected_suite_id = app
        .state
        .workbench
        .model_correlation
        .selected_suite_id
        .as_deref();
    let base = select_current_suite(&context, selected_suite_id, true)?.cloned();

    let dataset_id = required_text("Dataset ID", &draft.id)?;
    let dataset_revision = base
        .as_ref()
        .and_then(|suite| {
            suite
                .datasets
                .iter()
                .filter(|dataset| dataset.id.eq_ignore_ascii_case(&dataset_id))
                .max_by_key(|dataset| dataset.revision)
        })
        .map_or(Ok(ObjectRevision::INITIAL), |dataset| {
            dataset
                .revision
                .next()
                .map_err(|error| format!("Dataset revision cannot advance: {error}"))
        })?;
    let dataset = dataset_from_draft(
        app,
        draft,
        dataset_id.clone(),
        dataset_revision,
        &context.source,
    )?;

    let suite = if let Some(base) = base {
        require_exact_suite_source(&base, &context.source)?;
        let revision = next_suite_revision(&base)?;
        let mut datasets = base.datasets.clone();
        datasets.push(dataset);
        CorrelationSuite::try_new(
            base.id.clone(),
            revision,
            base.name.clone(),
            base.owner_id.clone(),
            context.source.clone(),
            datasets,
            base.metrics.clone(),
            base.dispositions.clone(),
        )
        .map_err(|error| format!("Dataset cannot be appended: {error}"))?
    } else {
        let suite_id = default_suite_id(&context)?;
        let owner_id = required_text("Suite owner identity", &draft.suite_owner_id)?;
        CorrelationSuite::try_new(
            suite_id,
            ObjectRevision::INITIAL,
            format!("{} correlation suite", context.model_name),
            owner_id,
            context.source.clone(),
            vec![dataset],
            Vec::new(),
            Vec::new(),
        )
        .map_err(|error| format!("Correlation suite cannot be created: {error}"))?
    };
    let suite_id = suite.id.clone();
    let suite_revision = suite.revision;

    let mut correlation = context.correlation.clone();
    correlation
        .add_suite_revision(suite)
        .map_err(|error| format!("Dataset cannot be appended: {error}"))?;
    publish_correlation(
        app,
        context,
        correlation,
        PublicationSelection {
            suite_id,
            dataset_id: Some(dataset_id.clone()),
            metric_id: None,
            notice: format!(
                "Dataset '{dataset_id}' was retained in correlation suite revision {}.",
                suite_revision.get()
            ),
        },
        format!("append correlation dataset {dataset_id}"),
    )
}

/// Append one metric definition by publishing a new immutable suite revision.
pub(super) fn append_metric(
    app: &mut RSpiceApp,
    draft: &CorrelationMetricDraft,
) -> Result<(), String> {
    preflight_mutation(app)?;
    let context = current_context(app)?;
    require_dialog_source(app, &context.source)?;
    let selected_suite_id = app
        .state
        .workbench
        .model_correlation
        .selected_suite_id
        .as_deref();
    let base = required_current_suite(&context, selected_suite_id)?.clone();
    require_exact_suite_source(&base, &context.source)?;

    let metric = metric_from_draft(draft)?;
    let metric_id = metric.id.clone();
    let revision = next_suite_revision(&base)?;
    let mut metrics = base.metrics.clone();
    if let Some(existing_index) = metrics
        .iter()
        .position(|existing| existing.id.eq_ignore_ascii_case(&metric.id))
    {
        if metrics[existing_index] == metric {
            return Err(format!(
                "Metric '{}' is unchanged; revise at least one declared contract field",
                metric.id
            ));
        }
        // The prior definition remains immutable in the preceding suite
        // revision. This new revision explicitly supersedes it by stable ID.
        metrics[existing_index] = metric;
    } else {
        metrics.push(metric);
    }
    let suite = CorrelationSuite::try_new(
        base.id.clone(),
        revision,
        base.name.clone(),
        base.owner_id.clone(),
        context.source.clone(),
        base.datasets.clone(),
        metrics,
        base.dispositions.clone(),
    )
    .map_err(|error| format!("Metric cannot be appended: {error}"))?;
    let suite_id = suite.id.clone();

    let mut correlation = context.correlation.clone();
    correlation
        .add_suite_revision(suite)
        .map_err(|error| format!("Metric cannot be appended: {error}"))?;
    publish_correlation(
        app,
        context,
        correlation,
        PublicationSelection {
            suite_id,
            dataset_id: None,
            metric_id: Some(metric_id.clone()),
            notice: format!(
                "Metric '{metric_id}' was retained in correlation suite revision {}.",
                revision.get()
            ),
        },
        format!("append correlation metric {metric_id}"),
    )
}

/// Append one governed outlier decision by publishing a new immutable suite
/// revision. A later decision for the same subject explicitly supersedes the
/// immediately preceding event.
pub(super) fn append_disposition(
    app: &mut RSpiceApp,
    draft: &CorrelationOutlierDraft,
) -> Result<(), String> {
    preflight_mutation(app)?;
    let context = current_context(app)?;
    require_dialog_source(app, &context.source)?;
    let selected_suite_id = app
        .state
        .workbench
        .model_correlation
        .selected_suite_id
        .as_deref();
    let base = required_current_suite(&context, selected_suite_id)?.clone();
    require_exact_suite_source(&base, &context.source)?;

    let metric_id = required_text("Metric ID", &draft.metric_id)?;
    let observation_id =
        required_text("Reference observation ID", &draft.reference_observation_id)?;
    let previous = base
        .dispositions
        .iter()
        .filter(|event| {
            event.metric_id.eq_ignore_ascii_case(&metric_id)
                && event
                    .reference_observation_id
                    .eq_ignore_ascii_case(&observation_id)
        })
        .max_by_key(|event| event.decided_at_unix_ms);
    let timestamp = timestamp_after(previous.map(|event| event.decided_at_unix_ms))?;
    let supersedes = previous.map(|event| event.id.clone());
    let revision = next_suite_revision(&base)?;
    let decision = map_outlier_decision(draft.decision);
    let disposition_id = stable_event_id(
        "cor-disposition",
        &[
            base.id.as_str(),
            &revision.get().to_string(),
            metric_id.as_str(),
            observation_id.as_str(),
            &timestamp.to_string(),
            outlier_decision_key(decision),
            draft.owner_id.trim(),
            draft.reviewer_id.trim(),
            draft.reason.trim(),
        ],
    );
    let disposition = CorrelationOutlierDisposition {
        id: disposition_id.clone(),
        metric_id: metric_id.clone(),
        reference_observation_id: observation_id,
        decision,
        reason: required_text("Disposition reason", &draft.reason)?,
        owner_id: required_text("Disposition owner", &draft.owner_id)?,
        reviewer_id: required_text("Independent reviewer", &draft.reviewer_id)?,
        decided_at_unix_ms: timestamp,
        supersedes,
    };
    let mut dispositions = base.dispositions.clone();
    dispositions.push(disposition);
    let suite = CorrelationSuite::try_new(
        base.id.clone(),
        revision,
        base.name.clone(),
        base.owner_id.clone(),
        context.source.clone(),
        base.datasets.clone(),
        base.metrics.clone(),
        dispositions,
    )
    .map_err(|error| format!("Outlier disposition cannot be appended: {error}"))?;
    let suite_id = suite.id.clone();

    let mut correlation = context.correlation.clone();
    correlation
        .add_suite_revision(suite)
        .map_err(|error| format!("Outlier disposition cannot be appended: {error}"))?;
    publish_correlation(
        app,
        context,
        correlation,
        PublicationSelection {
            suite_id,
            dataset_id: None,
            metric_id: Some(metric_id),
            notice: format!(
                "Outlier disposition '{disposition_id}' was retained in suite revision {}.",
                revision.get()
            ),
        },
        format!("append correlation disposition {disposition_id}"),
    )
}

/// Evaluate the exact current suite and append immutable reviewer evidence.
pub(super) fn append_review_evidence(
    app: &mut RSpiceApp,
    draft: &CorrelationReviewDraft,
) -> Result<(), String> {
    preflight_mutation(app)?;
    let context = current_context(app)?;
    require_dialog_source(app, &context.source)?;
    let selected_suite_id = app
        .state
        .workbench
        .model_correlation
        .selected_suite_id
        .as_deref();
    let suite = required_current_suite(&context, selected_suite_id)?.clone();
    require_exact_suite_source(&suite, &context.source)?;

    let reviewer_id = required_text("Reviewer identity", &draft.reviewer_id)?;
    if reviewer_id.eq_ignore_ascii_case(suite.owner_id.trim()) {
        return Err(
            "Correlation review requires a reviewer identity independent from the suite owner"
                .to_owned(),
        );
    }
    let decision = match draft.decision {
        CorrelationReviewDecisionDraft::Approve => CorrelationReviewDecision::Accept,
        CorrelationReviewDecisionDraft::Reject => CorrelationReviewDecision::Reject,
    };
    let conclusion = required_text("Review conclusion", &draft.conclusion)?;
    let evaluation = CorrelationEvaluation::evaluate(&suite)
        .map_err(|error| format!("Correlation evidence cannot be evaluated: {error}"))?;
    let latest_timestamp = context
        .correlation
        .evidence
        .iter()
        .map(|evidence| evidence.reviewed_at_unix_ms)
        .max();
    let timestamp = timestamp_after(latest_timestamp)?;
    let suite_digest = suite
        .content_digest()
        .map_err(|error| format!("Correlation suite digest cannot be computed: {error}"))?;
    let evidence_id = stable_event_id(
        "cor-evidence",
        &[
            suite.id.as_str(),
            &suite.revision.get().to_string(),
            &suite_digest.to_string(),
            reviewer_id.as_str(),
            match decision {
                CorrelationReviewDecision::Accept => "accept",
                CorrelationReviewDecision::Reject => "reject",
            },
            conclusion.as_str(),
            &timestamp.to_string(),
        ],
    );
    let evidence = CorrelationEvidence::try_new(
        evidence_id.clone(),
        &evaluation,
        reviewer_id,
        decision,
        conclusion,
        timestamp,
    )
    .map_err(|error| format!("Correlation evidence cannot be created: {error}"))?;

    let suite_id = suite.id.clone();
    let suite_revision = suite.revision;
    let mut correlation = context.correlation.clone();
    correlation
        .add_evidence(evidence)
        .map_err(|error| format!("Correlation evidence cannot be appended: {error}"))?;
    publish_correlation(
        app,
        context,
        correlation,
        PublicationSelection {
            suite_id,
            dataset_id: None,
            metric_id: None,
            notice: format!(
                "Review evidence '{evidence_id}' was retained for suite revision {}.",
                suite_revision.get()
            ),
        },
        format!("append correlation review evidence {evidence_id}"),
    )
}

fn preflight_mutation(app: &RSpiceApp) -> Result<(), String> {
    if !app.state.project_lifecycle.project_open {
        return Err("Measurement correlation requires an open project".to_owned());
    }
    if app.state.workbench.safe_mode.project_read_only() {
        return Err(
            "Measurement correlation cannot be changed while safe mode keeps the project read-only"
                .to_owned(),
        );
    }
    if app
        .state
        .workbench
        .model_editor
        .qualification_execution
        .is_some()
    {
        return Err(
            "Finish or cancel the active model qualification run before changing correlation records"
                .to_owned(),
        );
    }
    if let Some(draft) = app.state.workbench.model_editor.draft.as_ref()
        && draft.is_dirty()
    {
        return Err(format!(
            "Save or discard unsaved model candidate '{}/{}' before changing correlation records",
            draft.library_name, draft.model_name
        ));
    }
    Ok(())
}

fn current_context(app: &RSpiceApp) -> Result<CorrelationContext, String> {
    let library_name = app
        .state
        .model_library_manager
        .selected_library
        .clone()
        .ok_or_else(|| "Select a project-owned model library first".to_owned())?;
    let requested_model = app
        .state
        .workbench
        .selected_model
        .clone()
        .ok_or_else(|| "Select a project-owned model family first".to_owned())?;
    let resolved = resolve_project_model_for_editor(
        &app.state.model_library_manager,
        &library_name,
        &requested_model,
    )?;
    let model_name = resolved.definition.base.name.clone();
    let source = ModelSourceEvidenceBinding::try_new_project_bound(
        model_name.clone(),
        resolved.source_id,
        resolved.model_digest,
        resolved.model_revision,
    )
    .map_err(|error| format!("Current model source identity is invalid: {error}"))?;
    let correlation = app
        .state
        .model_library_manager
        .get_library(&library_name)
        .and_then(|library| library.model_correlation.get(&model_name))
        .cloned()
        .unwrap_or_default();
    correlation
        .validate_for_model(&model_name)
        .map_err(|error| format!("Retained model correlation state is invalid: {error}"))?;

    Ok(CorrelationContext {
        library_name,
        model_name,
        source_id: resolved.source_id,
        library_revision: resolved.library_revision,
        model_revision: resolved.model_revision,
        model_digest: resolved.model_digest,
        source,
        correlation,
    })
}

fn select_current_suite<'a>(
    context: &'a CorrelationContext,
    selected_suite_id: Option<&str>,
    allow_create: bool,
) -> Result<Option<&'a CorrelationSuite>, String> {
    if let Some(selected_suite_id) = selected_suite_id {
        let selected_suite_id = required_text("Correlation suite ID", selected_suite_id)?;
        let suite = context
            .correlation
            .latest_suite(&selected_suite_id)
            .ok_or_else(|| {
                format!("Correlation suite '{selected_suite_id}' is no longer retained")
            })?;
        require_exact_suite_source(suite, &context.source)?;
        return Ok(Some(suite));
    }

    let current = context
        .correlation
        .suite_lineages()
        .into_iter()
        .filter(|suite| suite.source == context.source)
        .collect::<Vec<_>>();
    match current.as_slice() {
        [] if allow_create => Ok(None),
        [] => Err(
            "Create a dataset-backed correlation suite for the exact current model source first"
                .to_owned(),
        ),
        [suite] => Ok(Some(*suite)),
        _ => Err("Select one current correlation suite before changing it".to_owned()),
    }
}

fn require_exact_suite_source(
    suite: &CorrelationSuite,
    current: &ModelSourceEvidenceBinding,
) -> Result<(), String> {
    if suite.source == *current {
        Ok(())
    } else {
        Err(format!(
            "Correlation suite '{}@{}' is bound to a different model source revision; select or create a suite for the exact current source",
            suite.id,
            suite.revision.get()
        ))
    }
}

fn required_current_suite<'a>(
    context: &'a CorrelationContext,
    selected_suite_id: Option<&str>,
) -> Result<&'a CorrelationSuite, String> {
    select_current_suite(context, selected_suite_id, false)?.ok_or_else(|| {
        "Create a dataset-backed correlation suite for the exact current model source first"
            .to_owned()
    })
}

fn dataset_from_draft(
    app: &RSpiceApp,
    draft: &CorrelationDatasetDraft,
    id: String,
    revision: ObjectRevision,
    current_source: &ModelSourceEvidenceBinding,
) -> Result<CorrelationDatasetRevision, String> {
    let class = map_dataset_class(draft.class);
    let (
        raw_source,
        authority,
        device_or_lot,
        fixture,
        calibration,
        source_name,
        model_source,
        provenance,
    ) = if class.is_simulation() {
        retained_simulation_dataset(app, draft, current_source)?
    } else {
        (
            draft.csv.as_bytes().to_vec(),
            required_text("Dataset authority", &draft.authority)?,
            required_text("Device or lot identity", &draft.device_or_lot)?,
            required_text("Fixture identity", &draft.fixture)?,
            required_text("Calibration identity", &draft.calibration)?,
            required_text("Dataset source name", &draft.source_name)?,
            None,
            None,
        )
    };

    CorrelationDatasetRevision::try_from_csv_with_provenance(
        id,
        revision,
        required_text("Dataset name", &draft.name)?,
        class,
        authority,
        device_or_lot,
        fixture,
        calibration,
        source_name,
        raw_source,
        model_source,
        provenance,
    )
    .map_err(|error| format!("Dataset is invalid: {error}"))
}

type RetainedSimulationDataset = (
    Vec<u8>,
    String,
    String,
    String,
    String,
    String,
    Option<ModelSourceEvidenceBinding>,
    Option<CorrelationSimulationProvenance>,
);

fn retained_simulation_dataset(
    app: &RSpiceApp,
    draft: &CorrelationDatasetDraft,
    current_source: &ModelSourceEvidenceBinding,
) -> Result<RetainedSimulationDataset, String> {
    let run_id = draft
        .retained_run_id
        .trim()
        .parse::<RunId>()
        .map_err(|error| format!("Select a valid retained simulation run: {error}"))?;
    let run = app
        .state
        .simulation
        .run_by_stable_id(run_id)
        .ok_or_else(|| "The selected retained simulation run no longer exists".to_owned())?;
    if run.lifecycle != SimulationRunLifecycle::Completed || !run.success {
        return Err(
            "Model-correlation evidence requires a successfully completed retained run".to_owned(),
        );
    }
    run.validate_provenance()
        .map_err(|error| format!("Retained run provenance is invalid: {error}"))?;
    let receipt = run.prepared_receipt().ok_or_else(|| {
        "The selected run predates authenticated prepared-run receipts and cannot authorize new correlation evidence"
            .to_owned()
    })?;
    let plan_id = receipt.simulation_plan_id().ok_or_else(|| {
        "Model-correlation evidence must come from a retained simulation-plan run".to_owned()
    })?;
    let source_id = current_source.source_id.ok_or_else(|| {
        "The current model is not bound to an exact project-owned source identity".to_owned()
    })?;
    let source_is_authenticated = receipt.project_model_sources().iter().any(|source| {
        source.source_id() == source_id
            && source
                .model_name()
                .eq_ignore_ascii_case(&current_source.model_id)
            && source.revision() == current_source.source_revision
            && source.content_digest() == current_source.source_digest
    });
    if !source_is_authenticated {
        return Err(
            "The selected run was not prepared against this exact project model revision; run the current simulation plan again before retaining correlation evidence"
                .to_owned(),
        );
    }

    let analysis_id =
        parse_positive_u64("Retained analysis identity", &draft.retained_analysis_id)?;
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.id == analysis_id)
        .ok_or_else(|| "The selected retained analysis no longer exists in this run".to_owned())?;
    if !analysis.success {
        return Err("A failed analysis cannot produce correlation evidence".to_owned());
    }
    let analysis_provenance = analysis.provenance().ok_or_else(|| {
        "The selected analysis predates authenticated prepared-task provenance".to_owned()
    })?;
    let task = receipt
        .tasks()
        .iter()
        .find(|task| task.instance_id() == analysis_provenance.source_instance_id())
        .ok_or_else(|| {
            "The selected analysis does not match any task in its authenticated run receipt"
                .to_owned()
        })?;
    let trace_name = required_text("Retained waveform", &draft.retained_trace_name)?;
    let trace = analysis
        .waveforms
        .iter()
        .find(|trace| trace.name == trace_name)
        .ok_or_else(|| "The selected retained waveform no longer exists".to_owned())?;
    let raw_source = canonical_simulation_csv(analysis.analysis_type, trace)?;
    let export_digest = digest(&raw_source);
    let executed_at_unix_ms =
        unix_timestamp_ms("Analysis completion timestamp", analysis.timestamp)?;
    let execution_target = run
        .execution_target
        .ok_or_else(|| {
            "The selected run predates retained execution-target identity and cannot authorize new correlation evidence"
                .to_owned()
        })?
        .label()
        .to_owned();
    let provenance = CorrelationSimulationProvenance {
        run_id: run.run_id.to_string(),
        run_dataset_id: run.dataset_id.to_string(),
        analysis_id,
        analysis_result_digest: analysis.result_data_digest(),
        plan_id: plan_id.to_string(),
        project_revision: receipt.project_revision(),
        prepared_snapshot_digest: receipt.prepared_snapshot_digest(),
        source_content_digest: receipt.source_content_digest(),
        task_config_digest: task.config_digest(),
        execution_target,
        export_digest,
        model_source: current_source.clone(),
        executed_at_unix_ms,
    };
    let source_name = format!(
        "retained-run://{}/{}/{}",
        run.run_id, analysis_id, trace.name
    );
    Ok((
        raw_source,
        "RSpice retained simulation history".to_owned(),
        current_source.model_id.clone(),
        format!("simulation plan {plan_id}"),
        format!("prepared snapshot {}", receipt.prepared_snapshot_digest()),
        source_name,
        Some(current_source.clone()),
        Some(provenance),
    ))
}

fn canonical_simulation_csv(
    analysis_type: AnalysisType,
    trace: &WaveformData,
) -> Result<Vec<u8>, String> {
    if trace.x.is_empty() || trace.x.len() != trace.y.len() {
        return Err(
            "The selected waveform must retain equally sized, non-empty X and Y vectors".to_owned(),
        );
    }
    if trace.x.len() > MAX_CORRELATION_ROWS {
        return Err(format!(
            "The selected waveform has {} samples; correlation datasets are limited to {MAX_CORRELATION_ROWS}",
            trace.x.len()
        ));
    }
    if trace
        .x
        .iter()
        .chain(trace.y.iter())
        .any(|value| !value.is_finite())
    {
        return Err("The selected waveform contains a non-finite sample".to_owned());
    }
    let (axis, axis_unit) = waveform_axis(analysis_type)?;
    let value_unit = waveform_value_unit(&trace.name);
    let condition_header = format!("condition:{axis}[{axis_unit}]");
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .terminator(Terminator::Any(b'\n'))
        .from_writer(Vec::new());
    writer
        .write_record([
            "id",
            "quantity",
            "value",
            "unit",
            "uncertainty",
            "weight",
            condition_header.as_str(),
        ])
        .map_err(|error| format!("Canonical correlation CSV header cannot be written: {error}"))?;
    for (index, (&x, &y)) in trace.x.iter().zip(trace.y.iter()).enumerate() {
        let id = format!("sample-{index:08}");
        let value = y.to_string();
        let coordinate = x.to_string();
        writer
            .write_record([
                id.as_str(),
                trace.name.as_str(),
                value.as_str(),
                value_unit,
                "0",
                "1",
                coordinate.as_str(),
            ])
            .map_err(|error| {
                format!(
                    "Canonical correlation CSV row {} cannot be written: {error}",
                    index + 1
                )
            })?;
    }
    writer
        .into_inner()
        .map_err(|error| format!("Canonical correlation CSV cannot be finalized: {error}"))
}

fn waveform_axis(analysis_type: AnalysisType) -> Result<(&'static str, &'static str), String> {
    use AnalysisType as Kind;
    match analysis_type {
        Kind::Transient | Kind::TransientNoise | Kind::Envelope | Kind::Pss | Kind::Qpss => {
            Ok(("time", "s"))
        }
        Kind::Ac
        | Kind::Disto
        | Kind::Noise
        | Kind::Pac
        | Kind::Pnoise
        | Kind::Pxf
        | Kind::Pstb
        | Kind::Stb
        | Kind::SParameter
        | Kind::Fourier
        | Kind::HarmonicBalance
        | Kind::Hbsp
        | Kind::Hbnoise
        | Kind::Psp
        | Kind::Qpac
        | Kind::Qpnoise
        | Kind::Qpxf => Ok(("frequency", "Hz")),
        Kind::DcSweep
        | Kind::Sensitivity
        | Kind::MonteCarlo
        | Kind::Parametric
        | Kind::Corner
        | Kind::Reliability
        | Kind::Optimization
        | Kind::Soa
        | Kind::DcMismatch => Ok(("sweep", "1")),
        Kind::DcOp | Kind::PoleZero | Kind::Tf => Err(format!(
            "{} does not expose a correlation-compatible waveform axis",
            analysis_type.display_name()
        )),
    }
}

fn waveform_value_unit(trace_name: &str) -> &'static str {
    let normalized = trace_name.trim().to_ascii_lowercase();
    if normalized.starts_with("phase(") {
        "deg"
    } else if normalized.starts_with("db(") {
        "dB"
    } else {
        let unwrapped = normalized.trim_matches('|');
        if unwrapped.starts_with("v(") {
            "V"
        } else if unwrapped.starts_with("i(") {
            "A"
        } else {
            "1"
        }
    }
}

fn unix_timestamp_ms(label: &str, seconds: f64) -> Result<u64, String> {
    let milliseconds = seconds * 1_000.0;
    if !milliseconds.is_finite() || milliseconds < 1.0 || milliseconds > u64::MAX as f64 {
        return Err(format!("{label} is outside the supported range"));
    }
    Ok(milliseconds.round() as u64)
}

fn metric_from_draft(
    draft: &CorrelationMetricDraft,
) -> Result<CorrelationMetricDefinition, String> {
    let domain = if draft.domain_enabled {
        Some(CorrelationMetricDomain {
            axis: required_text("Metric domain axis", &draft.domain_axis)?,
            unit: required_text("Metric domain unit", &draft.domain_unit)?,
            minimum: FiniteValue::new(parse_number(
                "Metric domain minimum",
                &draft.domain_minimum,
            )?)
            .map_err(|error| format!("Metric domain minimum is invalid: {error}"))?,
            maximum: FiniteValue::new(parse_number(
                "Metric domain maximum",
                &draft.domain_maximum,
            )?)
            .map_err(|error| format!("Metric domain maximum is invalid: {error}"))?,
        })
    } else {
        None
    };
    let alignment = match draft.alignment {
        CorrelationAlignmentDraft::Exact => CorrelationAlignmentPolicy::ExactOnly,
        CorrelationAlignmentDraft::MonotoneInterpolation => {
            let extrapolation = match draft.extrapolation {
                CorrelationExtrapolationDraft::Forbid => CorrelationExtrapolationPolicy::Forbid,
                CorrelationExtrapolationDraft::Limited => CorrelationExtrapolationPolicy::Limited {
                    max_axis_span_fraction: NonNegativeFinite::new(parse_number(
                        "Maximum extrapolation span fraction",
                        &draft.extrapolation_fraction,
                    )?)
                    .map_err(|error| {
                        format!("Maximum extrapolation span fraction is invalid: {error}")
                    })?,
                },
            };
            CorrelationAlignmentPolicy::MonotoneInterpolation {
                axis: required_text("Alignment axis", &draft.alignment_axis)?,
                extrapolation,
            }
        }
    };

    CorrelationMetricDefinition::try_new(
        required_text("Metric ID", &draft.id)?,
        required_text("Metric name", &draft.name)?,
        required_text("Reference dataset ID", &draft.reference_dataset_id)?,
        required_text("Simulation dataset ID", &draft.simulation_dataset_id)?,
        required_text("Metric quantity", &draft.quantity)?,
        map_calculation(draft.calculation),
        domain,
        parse_number("Metric limit", &draft.limit)?,
        parse_number(
            "Metric uncertainty multiplier",
            &draft.uncertainty_multiplier,
        )?,
        parse_number("Metric minimum coverage", &draft.minimum_coverage)?,
        map_aggregation(draft.aggregation),
        alignment,
        map_release_role(draft.release_role),
    )
    .map_err(|error| format!("Metric is invalid: {error}"))
}

fn publish_correlation(
    app: &mut RSpiceApp,
    context: CorrelationContext,
    correlation: ModelCorrelationState,
    selection: PublicationSelection,
    description: String,
) -> Result<(), String> {
    let mut candidate = app.state.model_library_manager.clone();
    let commit = candidate.replace_project_model_correlation(
        &context.library_name,
        context.source_id,
        context.library_revision,
        context.model_revision,
        context.model_digest,
        &context.model_name,
        &correlation,
    )?;
    let committed_revision =
        app.state
            .publish_project_model_candidate(candidate, commit, description)?;

    app.state.select_model_library(&context.library_name);
    app.state.workbench.selected_model = Some(context.model_name);
    if let Some(editor_draft) = app.state.workbench.model_editor.draft.as_mut() {
        editor_draft.base_project_revision = committed_revision;
    }
    app.state
        .workbench
        .model_editor
        .invalidate_candidate_evidence();

    let workspace = &mut app.state.workbench.model_correlation;
    workspace.selected_suite_id = Some(selection.suite_id);
    if let Some(dataset_id) = selection.dataset_id {
        workspace.selected_dataset_id = Some(dataset_id);
    }
    if let Some(metric_id) = selection.metric_id {
        workspace.selected_metric_id = Some(metric_id);
    }
    workspace.dialog = None;
    workspace.dialog_source = None;
    workspace.notice = Some(selection.notice);
    Ok(())
}

fn require_dialog_source(
    app: &RSpiceApp,
    current_source: &ModelSourceEvidenceBinding,
) -> Result<(), String> {
    let opening_source = app
        .state
        .workbench
        .model_correlation
        .dialog_source
        .as_ref()
        .ok_or_else(|| {
            "The correlation transaction has no opening model-source identity; cancel and reopen it"
                .to_owned()
        })?;
    if opening_source == current_source {
        Ok(())
    } else {
        Err(
            "The selected model source changed after this correlation transaction opened; cancel and reopen it"
                .to_owned(),
        )
    }
}

fn default_suite_id(context: &CorrelationContext) -> Result<String, String> {
    let stem = safe_id_fragment(&context.model_name);
    let base = format!("{stem}-correlation");
    if context
        .correlation
        .suites
        .iter()
        .all(|suite| !suite.id.eq_ignore_ascii_case(&base))
    {
        return Ok(base);
    }

    let revision_base = format!("{base}-source-r{}", context.model_revision.get());
    if context
        .correlation
        .suites
        .iter()
        .all(|suite| !suite.id.eq_ignore_ascii_case(&revision_base))
    {
        return Ok(revision_base);
    }
    let mut suffix = 2_u64;
    loop {
        let candidate = format!("{revision_base}-{suffix}");
        if context
            .correlation
            .suites
            .iter()
            .all(|suite| !suite.id.eq_ignore_ascii_case(&candidate))
        {
            return Ok(candidate);
        }
        suffix = suffix.checked_add(1).ok_or_else(|| {
            "Correlation suite identity space is exhausted for the current model source".to_owned()
        })?;
    }
}

fn safe_id_fragment(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut separator = false;
    for character in value.trim().chars() {
        if character.is_ascii_alphanumeric() || matches!(character, '_' | '-') {
            result.push(character.to_ascii_lowercase());
            separator = false;
        } else if !separator && !result.is_empty() {
            result.push('-');
            separator = true;
        }
    }
    while result.ends_with('-') {
        result.pop();
    }
    if result.is_empty() {
        "model".to_owned()
    } else {
        result
    }
}

fn next_suite_revision(suite: &CorrelationSuite) -> Result<ObjectRevision, String> {
    suite
        .revision
        .next()
        .map_err(|error| format!("Correlation suite revision cannot advance: {error}"))
}

fn required_text(label: &str, value: &str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("{label} is required"));
    }
    if value.len() > MAX_CORRELATION_TEXT_BYTES {
        return Err(format!(
            "{label} is limited to {MAX_CORRELATION_TEXT_BYTES} UTF-8 bytes"
        ));
    }
    if value.contains('\0') {
        return Err(format!("{label} cannot contain a NUL character"));
    }
    Ok(value.to_owned())
}

fn parse_number(label: &str, value: &str) -> Result<f64, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("{label} is required"));
    }
    let parsed = value
        .parse::<f64>()
        .map_err(|error| format!("{label} must be a decimal number: {error}"))?;
    if !parsed.is_finite() {
        return Err(format!("{label} must be finite"));
    }
    Ok(parsed)
}

fn parse_positive_u64(label: &str, value: &str) -> Result<u64, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("{label} is required"));
    }
    let parsed = value
        .parse::<u64>()
        .map_err(|error| format!("{label} must be a positive integer: {error}"))?;
    if parsed == 0 {
        Err(format!("{label} must be greater than zero"))
    } else {
        Ok(parsed)
    }
}

fn unix_time_ms() -> Result<u64, String> {
    let millis = crate::time_compat::unix_epoch().as_millis();
    let millis = u64::try_from(millis)
        .map_err(|_| "Current wall-clock timestamp exceeds the supported range".to_owned())?;
    if millis == 0 {
        Err("Current wall-clock timestamp is unavailable".to_owned())
    } else {
        Ok(millis)
    }
}

fn timestamp_after(previous: Option<u64>) -> Result<u64, String> {
    let now = unix_time_ms()?;
    match previous {
        Some(previous) if now <= previous => previous
            .checked_add(1)
            .ok_or_else(|| "Correlation event timestamp space is exhausted".to_owned()),
        _ => Ok(now),
    }
}

fn digest(bytes: &[u8]) -> ContentDigest {
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn stable_event_id(prefix: &str, fields: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    for field in fields {
        hasher.update(Sha256::digest(field.as_bytes()));
    }
    format!(
        "{prefix}-{}",
        ContentDigest::from_bytes(hasher.finalize().into())
    )
}

const fn map_dataset_class(value: CorrelationDatasetClassDraft) -> CorrelationDatasetClass {
    match value {
        CorrelationDatasetClassDraft::Bench => CorrelationDatasetClass::BenchMeasurement,
        CorrelationDatasetClassDraft::Silicon => CorrelationDatasetClass::SiliconCharacterization,
        CorrelationDatasetClassDraft::Vendor => CorrelationDatasetClass::VendorReference,
        CorrelationDatasetClassDraft::IndependentOracle => {
            CorrelationDatasetClass::IndependentOracle
        }
        CorrelationDatasetClassDraft::Simulated => CorrelationDatasetClass::ModelSimulation,
    }
}

const fn map_calculation(value: CorrelationCalculationDraft) -> CorrelationCalculation {
    match value {
        CorrelationCalculationDraft::AbsoluteLinear => CorrelationCalculation::AbsoluteLinear,
        CorrelationCalculationDraft::AbsoluteDecibels => CorrelationCalculation::AbsoluteDecibels,
        CorrelationCalculationDraft::Relative => CorrelationCalculation::Relative,
        CorrelationCalculationDraft::WeightedRelative => CorrelationCalculation::WeightedRelative,
        CorrelationCalculationDraft::PhaseWrapped => CorrelationCalculation::PhaseWrappedDegrees,
    }
}

const fn map_aggregation(value: CorrelationAggregationDraft) -> CorrelationAggregation {
    match value {
        CorrelationAggregationDraft::EveryPoint => CorrelationAggregation::EveryPoint,
        CorrelationAggregationDraft::WorstCase => CorrelationAggregation::WorstCondition,
        CorrelationAggregationDraft::Percentile95 => CorrelationAggregation::Percentile95,
        CorrelationAggregationDraft::RootMeanSquare => CorrelationAggregation::RootMeanSquare,
    }
}

const fn map_release_role(value: CorrelationReleaseRoleDraft) -> CorrelationReleaseRole {
    match value {
        CorrelationReleaseRoleDraft::Review => CorrelationReleaseRole::Review,
        CorrelationReleaseRoleDraft::Advisory => CorrelationReleaseRole::Advisory,
    }
}

const fn map_outlier_decision(
    value: CorrelationOutlierDecisionDraft,
) -> CorrelationOutlierDecision {
    match value {
        CorrelationOutlierDecisionDraft::Retain => CorrelationOutlierDecision::Retain,
        CorrelationOutlierDecisionDraft::ExcludeFixtureFault => {
            CorrelationOutlierDecision::ExcludeFixtureFault
        }
        CorrelationOutlierDecisionDraft::LimitOnlyEvidence => {
            CorrelationOutlierDecision::LimitOnlyEvidence
        }
    }
}

const fn outlier_decision_key(value: CorrelationOutlierDecision) -> &'static str {
    match value {
        CorrelationOutlierDecision::Retain => "retain",
        CorrelationOutlierDecision::ExcludeFixtureFault => "exclude-fixture-fault",
        CorrelationOutlierDecision::LimitOnlyEvidence => "limit-only-evidence",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retained_waveform_export_is_canonical_parseable_and_unit_aware() {
        let waveform = WaveformData::new(
            "phase(V(out))",
            vec![1.0, 10.0],
            vec![-12.5, 42.25],
            "#ffffff",
        );
        let source = canonical_simulation_csv(AnalysisType::Ac, &waveform).unwrap();
        assert!(
            String::from_utf8_lossy(&source)
                .starts_with("id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz]\n")
        );
        let dataset = CorrelationDatasetRevision::try_from_csv(
            "simulation-export",
            ObjectRevision::INITIAL,
            "Simulation export",
            CorrelationDatasetClass::BenchMeasurement,
            "test",
            "device",
            "fixture",
            "calibration",
            "generated.csv",
            source,
            None,
        )
        .unwrap();
        assert_eq!(dataset.observations.len(), 2);
        assert_eq!(dataset.observations[0].quantity, "phase(V(out))");
        assert_eq!(dataset.observations[0].unit, "deg");
        assert_eq!(
            dataset.observations[1].coordinates[0].dimension,
            "frequency"
        );
        assert_eq!(dataset.observations[1].coordinates[0].unit, "Hz");
    }

    #[test]
    fn retained_waveform_export_rejects_unusable_or_unbounded_sources() {
        let mismatched = WaveformData::new("V(out)", vec![0.0], Vec::<f64>::new(), "#ffffff");
        assert!(
            canonical_simulation_csv(AnalysisType::Transient, &mismatched)
                .unwrap_err()
                .contains("equally sized")
        );
        let non_finite = WaveformData::new("V(out)", vec![0.0], vec![f64::NAN], "#ffffff");
        assert!(
            canonical_simulation_csv(AnalysisType::Transient, &non_finite)
                .unwrap_err()
                .contains("non-finite")
        );
        let scalar = WaveformData::new("V(out)", vec![0.0], vec![1.0], "#ffffff");
        assert!(
            canonical_simulation_csv(AnalysisType::DcOp, &scalar)
                .unwrap_err()
                .contains("does not expose")
        );
    }
}
