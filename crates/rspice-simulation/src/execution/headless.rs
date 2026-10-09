//! Capture literal circuit sources and prepare an explicit headless task graph.

use std::path::{Path, PathBuf};

use rspice_app_types::product::{ObjectRevision, ProcessCorner};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::netlist::{
    IncludeProcessor, NetlistParseOptions, ParseError, ParseWithAbortError, SealedSourceBundle,
    StatisticalParamMode,
};
use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits};
use rspice_simulation_contract::saved_output::SavedOutput;

use super::{
    ExecutionTargetCapabilities, HeadlessTaskRequest, PreparedRunSnapshot, PreparedTask,
    RunSourceReceipt, SavePolicy, SimulationRunIntent, SnapshotParts, TaskSourcePolicy,
    TouchstoneExportPolicy, prepare_headless_tasks,
};
use crate::error::ServiceRunError;
use crate::execution_identity::{manual_source_receipt_digest, sealed_dependency_closure_digest};
use crate::measurement_references::PreparedMeasurementReferences;
#[cfg(not(target_arch = "wasm32"))]
use crate::netlist_preparation::IncludeSearchChain;
use crate::netlist_preparation::{
    reject_deferred_external_sources_with_project_runtimes,
    validated_parsed_hierarchy_with_limits_and_abort,
};
use crate::preparation::{PreparationError, PreparationStage};
use crate::veriloga::PreparedVerilogARuntimeSet;

/// Where preparation may resolve include/library contents. Workers always
/// receive the captured closure, never this resolver or a host source path.
#[derive(Debug, Clone)]
pub enum HeadlessSourceResolver {
    /// Resolve only through the supplied portable source bundle.
    Sealed(SealedSourceBundle),
    /// Read native dependencies once during preparation. Relative search paths
    /// are interpreted against the root circuit's directory.
    #[cfg(not(target_arch = "wasm32"))]
    Native { include_search_paths: Vec<PathBuf> },
}

/// Inputs for a headless run over circuit text and explicit analysis requests.
///
/// The circuit may contain models, options, measurements and save directives.
/// Analysis cards and control scripts belong to their own authored-deck route;
/// they are rejected here so a study cannot accidentally execute or inherit a
/// second, conflicting analysis plan. No editor or fabricated schematic is used.
pub struct HeadlessRunInput<'a> {
    pub source: &'a str,
    pub origin: &'a Path,
    pub resolver: HeadlessSourceResolver,
    pub source_revision: ObjectRevision,
    pub tasks: Vec<HeadlessTaskRequest>,
    pub reference_process: ProcessCorner,
    pub reference_temperature_celsius: f64,
    pub veriloga_runtimes: PreparedVerilogARuntimeSet,
    pub measurement_references: PreparedMeasurementReferences,
    pub saved_outputs: Vec<SavedOutput>,
    pub save_policy: SavePolicy,
    pub touchstone_export: TouchstoneExportPolicy,
    /// Input expansion/parsing and graph limits. Execution policy is a separate
    /// contract; this field does not claim to configure a worker's solver limits.
    pub preparation_limits: ResourceLimits,
}

impl<'a> HeadlessRunInput<'a> {
    /// Start with a self-contained circuit and the shared nominal environment.
    /// An include requires an explicit sealed bundle or native resolver.
    pub fn new(source: &'a str, origin: &'a Path, tasks: Vec<HeadlessTaskRequest>) -> Self {
        Self {
            source,
            origin,
            resolver: HeadlessSourceResolver::Sealed(SealedSourceBundle::default()),
            source_revision: ObjectRevision::INITIAL,
            tasks,
            reference_process: ProcessCorner::TT,
            reference_temperature_celsius: 27.0,
            veriloga_runtimes: PreparedVerilogARuntimeSet::default(),
            measurement_references: PreparedMeasurementReferences::default(),
            saved_outputs: Vec::new(),
            save_policy: SavePolicy::RetainEngineProducedResults,
            touchstone_export: TouchstoneExportPolicy::disabled(),
            preparation_limits: ResourceLimits::default(),
        }
    }
}

/// Headless preparation retains parser locations and typed abort/budget failures.
#[derive(Debug, thiserror::Error)]
pub enum HeadlessPreparationError {
    #[error("Headless preparation aborted")]
    Aborted,
    #[error(transparent)]
    ResourceLimit(#[from] ResourceLimitError),
    #[error(transparent)]
    Parse(ParseError),
    #[error("{0}")]
    Preparation(PreparationError),
    #[error(transparent)]
    TaskGraph(ServiceRunError),
}

impl From<PreparationError> for HeadlessPreparationError {
    fn from(error: PreparationError) -> Self {
        if error.is_aborted() {
            Self::Aborted
        } else if let Some(limit) = error.resource_limit() {
            Self::ResourceLimit(*limit)
        } else {
            Self::Preparation(error)
        }
    }
}

impl From<ParseWithAbortError> for HeadlessPreparationError {
    fn from(error: ParseWithAbortError) -> Self {
        match error {
            ParseWithAbortError::Aborted => Self::Aborted,
            ParseWithAbortError::Parse(ParseError::ResourceLimit(error)) => {
                Self::ResourceLimit(error)
            }
            ParseWithAbortError::Parse(error) => Self::Parse(error),
        }
    }
}

impl From<ServiceRunError> for HeadlessPreparationError {
    fn from(error: ServiceRunError) -> Self {
        match error {
            ServiceRunError::Aborted => Self::Aborted,
            ServiceRunError::ResourceLimit(error) => Self::ResourceLimit(error),
            other => Self::TaskGraph(other),
        }
    }
}

/// Capture sources, bind tasks and validate the same immutable snapshot used
/// by application dispatch. The result still needs one-use authorization.
pub fn prepare_headless_run(
    input: HeadlessRunInput<'_>,
    abort: &dyn AbortSignal,
) -> Result<PreparedRunSnapshot, HeadlessPreparationError> {
    check_abort(abort)?;
    let limits = input.preparation_limits;
    if input.source.len() > limits.max_netlist_bytes {
        return Err(ResourceLimitError {
            resource: ResourceKind::NetlistBytes,
            requested: input.source.len(),
            limit: limits.max_netlist_bytes,
        }
        .into());
    }
    if input.source.trim().is_empty() {
        return Err(source_error("Headless circuit source is empty"));
    }
    if !input.reference_temperature_celsius.is_finite()
        || input.reference_temperature_celsius <= -273.15
    {
        return Err(source_error(
            "Reference temperature must be finite and above absolute zero",
        ));
    }
    let (origin, mut processor) = source_processor(input.origin, input.resolver)?;
    processor = processor.with_resource_limits(limits);
    let expanded = processor.expand_content_with_abort(input.source, &origin, abort)?;
    let sealed_source_dependencies = processor.resolved_dependencies().to_vec();
    check_abort(abort)?;
    // Root admission happened before expansion. The captured closure is now
    // an executable deck governed by the expanded-source budget.
    let mut expanded_limits = limits;
    expanded_limits.max_netlist_bytes = limits.max_expanded_source_bytes;
    let parsed = rspice_core::Netlist::parse_with_options_and_abort(
        &expanded,
        NetlistParseOptions {
            retain_control_script: true,
            statistical_mode: StatisticalParamMode::Nominal,
            resource_limits: expanded_limits,
            ..Default::default()
        },
        abort,
    )?;
    if !parsed.analyses.is_empty()
        || !parsed.fft_analyses.is_empty()
        || parsed.lin_analysis.is_some()
        || parsed.control_script.is_some()
    {
        return Err(source_error(
            "Headless explicit studies require circuit-only source; move analysis cards into the task graph and run control scripts through their authored-deck host",
        ));
    }
    // Reject hierarchy amplification under the caller's policy before any
    // compatibility check can repeat flattening under default limits.
    validated_parsed_hierarchy_with_limits_and_abort(&parsed, expanded_limits, abort)?;
    super::preparation::reject_unresolved_device_models(&expanded, false)?;
    reject_deferred_external_sources_with_project_runtimes(
        &expanded,
        &input.veriloga_runtimes,
        &input.measurement_references,
    )?;
    check_abort(abort)?;

    for request in &input.tasks {
        check_abort(abort)?;
        let bytes = expanded
            .len()
            .saturating_add(request.analysis.analysis_line.len());
        if bytes > limits.max_expanded_source_bytes {
            return Err(ResourceLimitError {
                resource: ResourceKind::ExpandedSourceBytes,
                requested: bytes,
                limit: limits.max_expanded_source_bytes,
            }
            .into());
        }
        for (_, line) in
            crate::netlist_preparation::executable_logical_lines(&request.analysis.analysis_line)
        {
            let word = line.split_whitespace().next().unwrap_or_default();
            if [".end", ".control", ".endc", ".alter"]
                .iter()
                .any(|blocked| word.eq_ignore_ascii_case(blocked))
            {
                return Err(source_error(format!(
                    "Analysis {} contains a source/session boundary card: {word}",
                    request.instance_id
                )));
            }
        }
    }

    let tasks = prepare_headless_tasks(input.tasks, input.source_revision, limits, abort)?;
    let tasks =
        super::task_preparation::attach_saved_output_contracts(tasks, &input.saved_outputs)?;
    super::preparation::reject_deferred_corner_model_sources(
        tasks.iter().map(PreparedTask::queued_analysis),
        &expanded,
    )?;
    crate::preparation::validate_prepared_periodic_sources(
        tasks
            .iter()
            .map(|task| (task.instance_id(), &task.queued_analysis().spec)),
        &expanded,
    )?;
    let source_digest = crate::sealed_source::manual_executable_source_digest(&expanded);
    let config_digests = tasks
        .iter()
        .map(PreparedTask::config_digest)
        .collect::<Vec<_>>();
    let origin_identity = origin.to_string_lossy().replace('\\', "/");
    let receipt_digest = manual_source_receipt_digest(
        input.source,
        &expanded,
        Some(&origin_identity),
        sealed_dependency_closure_digest(&sealed_source_dependencies),
        &config_digests,
    );
    check_abort(abort)?;
    let snapshot = PreparedRunSnapshot::new_with_preparation_policy(
        SnapshotParts {
            task_source_policy: TaskSourcePolicy::PreparedAnalyses,
            // Source domain means literal netlist versus schematic generation; it
            // does not identify which frontend supplied the explicit task graph.
            intent: SimulationRunIntent::ManualDeck,
            simulation_plan_id: None,
            project_revision: input.source_revision.get(),
            topology_revision: 0,
            source_digest,
            reference_process: input.reference_process,
            reference_temperature_celsius: input.reference_temperature_celsius,
            run_set: None,
            tasks,
            executable_netlist: expanded,
            save_policy: input.save_policy,
            model_identities: Vec::new(),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy: Default::default(),
            project_veriloga_runtimes: input.veriloga_runtimes,
            measurement_references: input.measurement_references,
            target: ExecutionTargetCapabilities::current(),
            receipt: RunSourceReceipt::ManualSourceCheck(receipt_digest),
            advisories: parsed
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.message.clone())
                .collect(),
            manual_source: Some(input.source.to_owned()),
            cross_probe: None,
            touchstone_export: input.touchstone_export,
            sealed_source_dependencies,
        },
        expanded_limits,
        abort,
    )?;
    check_abort(abort)?;
    Ok(snapshot)
}

fn source_processor(
    origin: &Path,
    resolver: HeadlessSourceResolver,
) -> Result<(PathBuf, IncludeProcessor), HeadlessPreparationError> {
    match resolver {
        HeadlessSourceResolver::Sealed(bundle) => {
            if !rspice_model_library::is_portable_absolute_path(origin) {
                return Err(source_error(
                    "A sealed headless source requires an absolute portable root identity",
                ));
            }
            let directory = origin.parent().unwrap_or(origin);
            Ok((
                origin.to_owned(),
                IncludeProcessor::new_sealed(directory, bundle),
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        HeadlessSourceResolver::Native {
            include_search_paths,
        } => {
            let origin =
                crate::netlist_preparation::dependencies::absolute_source_identity(origin)?;
            let directory = origin.parent().unwrap_or(&origin);
            let mut processor = IncludeProcessor::new(directory);
            IncludeSearchChain::resolve(&include_search_paths, Some(directory))
                .apply_to(&mut processor);
            Ok((origin, processor))
        }
    }
}

fn source_error(message: impl Into<String>) -> HeadlessPreparationError {
    PreparationError::new(PreparationStage::SourceChecks, message).into()
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), HeadlessPreparationError> {
    if abort.is_aborted() {
        Err(HeadlessPreparationError::Aborted)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
