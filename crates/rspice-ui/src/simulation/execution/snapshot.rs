//! The prepared execution snapshot.
//!
//! An immutable capture of everything a run executes against, taken at the
//! moment the run is authorized. Preparation errors are reported here rather
//! than discovered mid-run.

use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::product::ProcessCorner;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
use crate::simulation::controller::{QueuedAnalysis, splice_before_terminal_end_card};
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::netlist_gen::EmissionRow;
use crate::simulation::output_contract::{
    PreparedSavedOutput, output_kind_tag, policy_tag, precision_tag, streaming_tag,
};
use crate::simulation::plan::AnalysisNumericOverride;
use crate::simulation::run_set::{RunSetDimensionKind, RunSetState};
use crate::state::{
    AnalysisResultSourceDomain, Point, PreparedModelSourceIdentity, PreparedRunReceipt,
    PreparedRunTaskReceipt, PreparedSourceCheckReceipt, PreparedSpecification,
    PreparedSpecificationPolicy, SimulationRunIntent,
};

use super::artifact::{
    ExecutionArtifactEnvelope, ExecutionArtifactError, ExecutionArtifactKind,
    PreparedDependencyBinding, ResolvedExecutionDependencies,
    validate_prepared_dependency_contract_with_options,
};
use super::canonical::{
    CanonicalWriter, analysis_config_digest, analysis_kind_tag, content_digest,
};
use super::permit::ConsumedExecutionPermit;

mod declared_points;

use declared_points::{expand_corner_run_point_tasks, expand_temperature_run_point_tasks};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreparationStage {
    DesignChecks,
    SourceChecks,
    AnalysisPlan,
    ModelBindings,
    Netlist,
    Authorization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparationError {
    stage: PreparationStage,
    message: String,
}

impl PreparationError {
    pub(crate) fn new(stage: PreparationStage, message: impl Into<String>) -> Self {
        Self {
            stage,
            message: message.into(),
        }
    }

    pub(crate) const fn stage(&self) -> PreparationStage {
        self.stage
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for PreparationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::simulation) enum SavePolicy {
    /// Retain every dataset and signal produced by the selected engine
    /// analyses. RSpice currently exposes no narrower output-selection UI.
    RetainEngineProducedResults,
    /// Simulation-plan-owned controls authenticated with the prepared run.
    PlanOwned {
        output_selection_mode: crate::state::OutputSelectionMode,
        retained_dataset_limit: usize,
        maximum_storage_bytes: u64,
        live_streaming_enabled: bool,
        retain_failure_diagnostics: bool,
    },
}

impl SavePolicy {
    pub(in crate::simulation) const fn label(self) -> &'static str {
        match self {
            Self::RetainEngineProducedResults => "Retain engine-produced results",
            Self::PlanOwned { .. } => "Plan-owned save and streaming policy",
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::RetainEngineProducedResults => 0,
            Self::PlanOwned { .. } => 1,
        }
    }

    pub(in crate::simulation) const fn retained_dataset_limit(self) -> Option<usize> {
        match self {
            Self::RetainEngineProducedResults => None,
            Self::PlanOwned {
                retained_dataset_limit,
                ..
            } => Some(retained_dataset_limit),
        }
    }

    pub(in crate::simulation) const fn output_selection_mode(
        self,
    ) -> crate::state::OutputSelectionMode {
        match self {
            Self::RetainEngineProducedResults => crate::state::OutputSelectionMode::SaveAll,
            Self::PlanOwned {
                output_selection_mode,
                ..
            } => output_selection_mode,
        }
    }

    /// Hard ceiling for the retained evidence owned by one prepared run.
    ///
    /// Manual-deck execution intentionally has no plan-owned ceiling. Studio
    /// runs authenticate this value in the prepared snapshot and enforce it a
    /// second time when terminal analysis evidence is about to enter the run.
    pub(in crate::simulation) const fn maximum_storage_bytes(self) -> Option<u64> {
        match self {
            Self::RetainEngineProducedResults => None,
            Self::PlanOwned {
                maximum_storage_bytes,
                ..
            } => Some(maximum_storage_bytes),
        }
    }

    pub(in crate::simulation) const fn live_streaming_enabled(self) -> bool {
        match self {
            Self::RetainEngineProducedResults => true,
            Self::PlanOwned {
                live_streaming_enabled,
                ..
            } => live_streaming_enabled,
        }
    }

    pub(in crate::simulation) const fn retain_failure_diagnostics(self) -> bool {
        match self {
            Self::RetainEngineProducedResults => true,
            Self::PlanOwned {
                retain_failure_diagnostics,
                ..
            } => retain_failure_diagnostics,
        }
    }

    fn encode(self, writer: &mut CanonicalWriter) {
        writer.u8(self.tag());
        if let Self::PlanOwned {
            output_selection_mode,
            retained_dataset_limit,
            maximum_storage_bytes,
            live_streaming_enabled,
            retain_failure_diagnostics,
        } = self
        {
            writer.u8(match output_selection_mode {
                crate::state::OutputSelectionMode::Automatic => 0,
                crate::state::OutputSelectionMode::ExplicitOnly => 1,
                crate::state::OutputSelectionMode::SaveAll => 2,
            });
            writer.usize(retained_dataset_limit);
            writer.u64(maximum_storage_bytes);
            writer.bool(live_streaming_enabled);
            writer.bool(retain_failure_diagnostics);
        }
    }
}

/// Immutable automatic Touchstone-export policy authenticated by preflight.
///
/// The output prefix is captured before execution so edits to the live
/// schematic path or S-parameter dialog cannot redirect a completed run. The
/// digest uses the platform-exact path identity; display conversion is never
/// used as persistence authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::simulation) enum TouchstoneExportPolicy {
    Disabled,
    Enabled {
        version: u32,
        output_directory: PathBuf,
        output_stem: OsString,
        output_identity: ContentDigest,
    },
}

impl TouchstoneExportPolicy {
    pub(in crate::simulation) const fn disabled() -> Self {
        Self::Disabled
    }

    pub(in crate::simulation) fn enabled(
        version: u32,
        output_directory: PathBuf,
        output_stem: OsString,
    ) -> Result<Self, PreparationError> {
        if !(1..=2).contains(&version) {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Touchstone export version must be 1 or 2, got {version}"),
            ));
        }
        if output_directory.as_os_str().is_empty() || output_stem.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Touchstone export requires a non-empty captured output prefix",
            ));
        }
        let stem_path = Path::new(&output_stem);
        if stem_path.file_name() != Some(output_stem.as_os_str())
            || stem_path.components().count() != 1
        {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Touchstone export stem must be one path component",
            ));
        }
        let output_identity = exact_path_digest(&output_directory.join(&output_stem));
        Ok(Self::Enabled {
            version,
            output_directory,
            output_stem,
            output_identity,
        })
    }

    pub(in crate::simulation) const fn version(&self) -> Option<u32> {
        match self {
            Self::Disabled => None,
            Self::Enabled { version, .. } => Some(*version),
        }
    }

    pub(in crate::simulation) fn output_path(
        &self,
        run_id: u64,
        analysis_idx: usize,
        num_ports: usize,
    ) -> Option<PathBuf> {
        let Self::Enabled {
            output_directory,
            output_stem,
            ..
        } = self
        else {
            return None;
        };
        let mut file_name = output_stem.clone();
        file_name.push(format!(
            "_run{run_id:04}_sp{:02}.s{}p",
            analysis_idx.max(1),
            num_ports.max(2)
        ));
        Some(output_directory.join(file_name))
    }

    fn encode(&self, writer: &mut CanonicalWriter) {
        writer.domain("touchstone-export-policy");
        match self {
            Self::Disabled => writer.u8(0),
            Self::Enabled {
                version,
                output_identity,
                ..
            } => {
                writer.u8(1);
                writer.u64(u64::from(*version));
                writer.digest(*output_identity);
            }
        }
    }
}

fn exact_path_digest(path: &Path) -> ContentDigest {
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;

        let mut bytes = Vec::new();
        for unit in path.as_os_str().encode_wide() {
            bytes.extend_from_slice(&unit.to_be_bytes());
        }
        content_digest("rspice.touchstone-output-prefix/windows-utf16be/v1", &bytes)
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;

        content_digest(
            "rspice.touchstone-output-prefix/unix-bytes/v1",
            path.as_os_str().as_bytes(),
        )
    }
    #[cfg(not(any(windows, unix)))]
    {
        content_digest(
            "rspice.touchstone-output-prefix/utf8/v1",
            path.as_os_str().to_string_lossy().as_bytes(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::simulation) struct ExecutionTargetCapabilities {
    label: &'static str,
    background_execution: bool,
    cancellable: bool,
    sealed_netlist_input: bool,
    max_parallel_tasks: u64,
}

impl ExecutionTargetCapabilities {
    pub(in crate::simulation) const fn current() -> Self {
        #[cfg(target_arch = "wasm32")]
        let label = "Browser module worker";
        #[cfg(not(target_arch = "wasm32"))]
        let label = "Desktop background thread";

        Self {
            label,
            background_execution: true,
            // Browser workers are terminated synchronously. Native execution
            // cooperatively polls the same typed abort signal through every
            // analysis service and core long-running loop.
            cancellable: execution_target_supports_cancellation(),
            sealed_netlist_input: true,
            // The controller intentionally preserves ordered task semantics.
            max_parallel_tasks: 1,
        }
    }

    pub(in crate::simulation) const fn label(&self) -> &'static str {
        self.label
    }

    fn encode(&self, writer: &mut CanonicalWriter) {
        writer.domain("target-capabilities");
        writer.string(self.label);
        writer.bool(self.background_execution);
        writer.bool(self.cancellable);
        writer.bool(self.sealed_netlist_input);
        writer.u64(self.max_parallel_tasks);
    }
}

pub(crate) const fn execution_target_supports_cancellation() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::simulation) struct ModelSourceIdentity {
    label: String,
    digest: ContentDigest,
}

impl ModelSourceIdentity {
    pub(in crate::simulation) fn new(label: impl Into<String>, digest: ContentDigest) -> Self {
        Self {
            label: label.into(),
            digest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::simulation) enum RunSourceReceipt {
    SchematicDrc(ContentDigest),
    ManualSourceCheck(ContentDigest),
}

impl RunSourceReceipt {
    pub(in crate::simulation) const fn digest(self) -> ContentDigest {
        match self {
            Self::SchematicDrc(digest) | Self::ManualSourceCheck(digest) => digest,
        }
    }

    pub(in crate::simulation) const fn label(self) -> &'static str {
        match self {
            Self::SchematicDrc(_) => "Schematic DRC receipt",
            Self::ManualSourceCheck(_) => "Manual source-check receipt",
        }
    }

    const fn durable(self) -> PreparedSourceCheckReceipt {
        match self {
            Self::SchematicDrc(digest) => PreparedSourceCheckReceipt::SchematicDrc(digest),
            Self::ManualSourceCheck(digest) => {
                PreparedSourceCheckReceipt::ManualSourceCheck(digest)
            }
        }
    }

    fn encode(self, writer: &mut CanonicalWriter) {
        writer.domain("source-check-receipt");
        match self {
            Self::SchematicDrc(digest) => {
                writer.u8(0);
                writer.digest(digest);
            }
            Self::ManualSourceCheck(digest) => {
                writer.u8(1);
                writer.digest(digest);
            }
        }
    }
}

/// One immutable analysis task frozen before snapshot authorization.
///
/// Identity, source revision, graph edges, display label, authenticated engine
/// payload, and payload digest deliberately live in one record. Keeping these
/// fields together prevents parallel-vector drift and makes it impossible for
/// dispatch authorization to shed graph provenance accidentally.
#[derive(Debug, Clone)]
pub(in crate::simulation) struct PreparedTask {
    instance_id: AnalysisInstanceId,
    authored_instance_id: AnalysisInstanceId,
    source_revision: ObjectRevision,
    dependencies: Vec<AnalysisInstanceId>,
    dependency_bindings: Vec<PreparedDependencyBinding>,
    label: String,
    config_digest: ContentDigest,
    task: QueuedAnalysis,
    saved_output_contracts: Vec<PreparedSavedOutput>,
    /// Explicit per-instance override. Stable plan tasks always set this for
    /// S-parameter analyses; manual-deck tasks inherit the run-level policy.
    touchstone_export: Option<TouchstoneExportPolicy>,
    /// Exact per-point source after process-model binding. `None` selects the
    /// run-level executable source verbatim.
    executable_netlist_override: Option<String>,
    /// The PVT point this task was expanded to. Only per-point expansion sets
    /// it; a task that runs once for the whole declared space has no point,
    /// and naming one would attribute its results to a corner it never solved.
    pvt_point: Option<crate::state::AnalysisResultPvtPoint>,
    /// The PVT declaration this task solves one point of. The declaration is
    /// never dispatched, so this is the only record of which base analysis it
    /// ran and where this point sits in the space it declared.
    declared_point: Option<crate::simulation::point_family::DeclaredRunPoint>,
    /// Temperature and supply overrides applied to the parsed task deck.
    /// Process selection is already present in `executable_netlist_override`.
    execution_environment: Option<crate::simulation::runner::AnalysisExecutionEnvironment>,
}

impl PreparedTask {
    pub(in crate::simulation) fn new(
        instance_id: AnalysisInstanceId,
        source_revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        label: impl Into<String>,
        task: QueuedAnalysis,
    ) -> Self {
        let config_digest = analysis_config_digest(
            &task.analysis_line,
            &task.spec,
            task.config.as_ref(),
            &task.spec_options,
            task.numeric_override.as_ref(),
        );
        Self {
            instance_id,
            authored_instance_id: instance_id,
            source_revision,
            dependencies,
            dependency_bindings: Vec::new(),
            label: label.into(),
            config_digest,
            task,
            saved_output_contracts: Vec::new(),
            touchstone_export: None,
            executable_netlist_override: None,
            pvt_point: None,
            declared_point: None,
            execution_environment: None,
        }
    }

    pub(in crate::simulation) fn with_saved_output_contracts(
        mut self,
        contracts: Vec<PreparedSavedOutput>,
    ) -> Self {
        self.saved_output_contracts = contracts;
        self
    }

    pub(in crate::simulation) fn with_touchstone_export_policy(
        mut self,
        policy: TouchstoneExportPolicy,
    ) -> Self {
        self.touchstone_export = Some(policy);
        self
    }

    pub(in crate::simulation) fn set_dependency_bindings(
        &mut self,
        bindings: Vec<PreparedDependencyBinding>,
    ) {
        self.dependency_bindings = bindings;
    }

    pub(in crate::simulation) fn set_dependencies(
        &mut self,
        dependencies: Vec<AnalysisInstanceId>,
    ) {
        self.dependencies = dependencies;
    }

    pub(in crate::simulation) const fn instance_id(&self) -> AnalysisInstanceId {
        self.instance_id
    }

    pub(in crate::simulation) const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    pub(in crate::simulation) fn dependencies(&self) -> &[AnalysisInstanceId] {
        &self.dependencies
    }

    #[cfg(test)]
    pub(in crate::simulation) fn touchstone_export_policy(
        &self,
    ) -> Option<&TouchstoneExportPolicy> {
        self.touchstone_export.as_ref()
    }

    pub(in crate::simulation) const fn config_digest(&self) -> ContentDigest {
        self.config_digest
    }

    pub(in crate::simulation) const fn queued_analysis(&self) -> &QueuedAnalysis {
        &self.task
    }

    #[cfg(test)]
    pub(in crate::simulation) const fn pvt_point(
        &self,
    ) -> Option<&crate::state::AnalysisResultPvtPoint> {
        self.pvt_point.as_ref()
    }

    fn payload_digest(&self) -> ContentDigest {
        let analysis_digest = analysis_config_digest(
            &self.task.analysis_line,
            &self.task.spec,
            self.task.config.as_ref(),
            &self.task.spec_options,
            self.task.numeric_override.as_ref(),
        );
        let Some(environment) = self.execution_environment.as_ref() else {
            return analysis_digest;
        };
        let mut writer = CanonicalWriter::new("rspice.analysis-run-set-environment/v1");
        writer.digest(analysis_digest);
        writer.f64(environment.temperature_celsius);
        writer.option(environment.supply_voltage.as_ref(), |writer, voltage| {
            writer.f64(*voltage);
        });
        writer.option(
            environment.nominal_supply_voltage.as_ref(),
            |writer, voltage| writer.f64(*voltage),
        );
        writer.usize(environment.supply_source_names.len());
        for source in &environment.supply_source_names {
            writer.string(source);
        }
        writer.finish()
    }
}

#[derive(Debug, Clone)]
struct PreparedPvtPoint {
    process: ProcessCorner,
    voltage: Option<f64>,
    supply_source_names: Vec<String>,
    temperature_celsius: f64,
    /// Exact point-specific design-variable values, in declaration order.
    /// These are materialized as `.param` cards in the authenticated task
    /// source before the task digest is frozen.
    parameter_overrides: Vec<(String, String)>,
    /// Exact independent source values materialized into the task deck.
    source_overrides: Vec<(String, String)>,
    /// Exact corner contract that owns process-model and nominal-voltage
    /// semantics for this point. Temperature-only axes do not carry one.
    corner_contract: Option<crate::services::simulation_runner::CornerRunConfig>,
}

/// What one generation pass learned about the design it netlisted. The one
/// site that builds this names its fields: four of them are maps over the same
/// handful of types, and a positional list of those swaps silently.
#[derive(Debug, Clone)]
pub(in crate::simulation) struct CrossProbeSnapshot {
    pub(in crate::simulation) source_reference: crate::state::CellViewRef,
    pub(in crate::simulation) point_to_net: HashMap<Point, String>,
    pub(in crate::simulation) nets: HashMap<String, Vec<Point>>,
    pub(in crate::simulation) net_segments: HashMap<String, Vec<(Point, Point)>>,
    pub(in crate::simulation) topology_version: u64,
    /// The master each occurrence was emitted against. Cross probing never
    /// reads it; it is captured here because the same pass produces it.
    pub(in crate::simulation) emission_map: Vec<EmissionRow>,
}

impl CrossProbeSnapshot {
    pub(in crate::simulation) fn apply(self, state: &mut crate::workbench::app_state::AppState) {
        let source_is_active = state
            .workspace
            .active_view
            .key()
            .eq_ignore_ascii_case(&self.source_reference.key());
        if source_is_active {
            state.schematic.net_mapping = self.point_to_net.clone();
        }
        state.simulation.cross_probe.update(
            self.source_reference,
            self.point_to_net,
            self.nets,
            self.net_segments,
            self.topology_version,
        );
    }
}

/// Read-only fields rendered by the existing mockup preflight surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedRunMetadata {
    pub(crate) snapshot_digest: ContentDigest,
    pub(crate) source_digest: ContentDigest,
    pub(crate) receipt_digest: ContentDigest,
    pub(crate) receipt_label: &'static str,
    pub(crate) project_revision: u64,
    pub(crate) topology_revision: u64,
    pub(crate) analysis_ids: Vec<ContentDigest>,
    pub(crate) task_count: usize,
    pub(crate) saved_output_contract_count: usize,
    pub(crate) pvt_point_count: usize,
    pub(crate) target: &'static str,
    pub(crate) save_policy: &'static str,
    pub(crate) model_identity_count: usize,
    pub(crate) advisories: Vec<String>,
    /// Exact complete source files read while sealing a manual deck. Empty for
    /// schematic-generated runs and manual decks without external sources.
    pub(crate) sealed_source_dependencies: Vec<rspice_core::netlist::ResolvedIncludeDependency>,
}

/// Whole-run dispatch object that can only be created with a successfully
/// consumed generation permit.
#[derive(Debug)]
pub(in crate::simulation) struct AuthorizedRunDispatch {
    digest: ContentDigest,
    intent: SimulationRunIntent,
    simulation_plan_id: Option<SimulationPlanId>,
    project_revision: u64,
    source_digest: ContentDigest,
    source_receipt: RunSourceReceipt,
    save_policy: SavePolicy,
    project_model_sources: Vec<PreparedModelSourceIdentity>,
    specifications: Vec<PreparedSpecification>,
    specification_policy: PreparedSpecificationPolicy,
    tasks: VecDeque<AuthorizedTaskDispatch>,
    executable_netlist: Arc<str>,
    advisories: Vec<String>,
    manual_source: Option<String>,
    cross_probe: Option<CrossProbeSnapshot>,
}

/// Opaque task/netlist pair accepted by the runner's sole production start.
/// Its constructor and fields are private to the execution boundary.
#[derive(Debug)]
pub(in crate::simulation) struct AuthorizedTaskDispatch {
    snapshot_digest: ContentDigest,
    instance_id: AnalysisInstanceId,
    authored_instance_id: AnalysisInstanceId,
    source_revision: ObjectRevision,
    dependencies: Vec<AnalysisInstanceId>,
    dependency_bindings: Vec<PreparedDependencyBinding>,
    label: String,
    config_digest: ContentDigest,
    task: QueuedAnalysis,
    saved_output_contracts: Vec<PreparedSavedOutput>,
    executable_netlist: Arc<str>,
    project_veriloga_runtimes: crate::simulation::veriloga::PreparedVerilogARuntimeSet,
    touchstone_export: TouchstoneExportPolicy,
    pvt_point: Option<crate::state::AnalysisResultPvtPoint>,
    declared_point: Option<crate::simulation::point_family::DeclaredRunPoint>,
    execution_environment: Option<crate::simulation::runner::AnalysisExecutionEnvironment>,
}

/// A prepared task paired with the exact batch-local artifacts required by
/// its authenticated dependency contract.
#[derive(Debug)]
pub(in crate::simulation) struct ResolvedTaskDispatch {
    dispatch: AuthorizedTaskDispatch,
    dependencies: ResolvedExecutionDependencies,
}

impl AuthorizedRunDispatch {
    pub(in crate::simulation) const fn digest(&self) -> ContentDigest {
        self.digest
    }

    pub(in crate::simulation) const fn intent(&self) -> SimulationRunIntent {
        self.intent
    }

    pub(in crate::simulation) const fn simulation_plan_id(&self) -> Option<SimulationPlanId> {
        self.simulation_plan_id
    }

    pub(in crate::simulation) const fn save_policy(&self) -> SavePolicy {
        self.save_policy
    }

    pub(in crate::simulation) fn prepared_run_receipt(
        &self,
        source_domain: AnalysisResultSourceDomain,
    ) -> Result<PreparedRunReceipt, PreparationError> {
        let project_revision = ObjectRevision::new(self.project_revision).map_err(|error| {
            PreparationError::new(
                PreparationStage::Authorization,
                format!("Authorized run has invalid project revision: {error}"),
            )
        })?;
        let tasks = self
            .tasks
            .iter()
            .map(|task| {
                PreparedRunTaskReceipt::new(
                    task.instance_id,
                    task.source_revision,
                    task.dependencies.clone(),
                    analysis_kind_tag(&task.task.spec),
                    task.config_digest,
                )
                .map_err(|error| PreparationError::new(PreparationStage::Authorization, error))
            })
            .collect::<Result<Vec<_>, _>>()?;
        PreparedRunReceipt::new_with_project_model_sources_specifications_and_policy(
            source_domain,
            self.simulation_plan_id,
            project_revision,
            self.digest,
            self.source_digest,
            self.source_receipt.durable(),
            self.project_model_sources.clone(),
            self.specifications.clone(),
            self.specification_policy.clone(),
            tasks,
        )
        .map_err(|error| PreparationError::new(PreparationStage::Authorization, error))
    }

    pub(in crate::simulation) fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub(in crate::simulation) fn tasks(&self) -> impl Iterator<Item = &AuthorizedTaskDispatch> {
        self.tasks.iter()
    }

    pub(in crate::simulation) fn into_tasks(self) -> VecDeque<AuthorizedTaskDispatch> {
        self.tasks
    }

    pub(in crate::simulation) fn executable_netlist(&self) -> &str {
        &self.executable_netlist
    }

    pub(in crate::simulation) fn advisories(&self) -> &[String] {
        &self.advisories
    }

    pub(in crate::simulation) fn manual_source(&self) -> Option<&str> {
        self.manual_source.as_deref()
    }

    pub(in crate::simulation) fn take_cross_probe(&mut self) -> Option<CrossProbeSnapshot> {
        self.cross_probe.take()
    }
}

impl AuthorizedTaskDispatch {
    pub(in crate::simulation) const fn snapshot_digest(&self) -> ContentDigest {
        self.snapshot_digest
    }

    pub(in crate::simulation) const fn instance_id(&self) -> AnalysisInstanceId {
        self.instance_id
    }

    pub(in crate::simulation) const fn authored_instance_id(&self) -> AnalysisInstanceId {
        self.authored_instance_id
    }

    pub(in crate::simulation) const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    pub(in crate::simulation) fn dependencies(&self) -> &[AnalysisInstanceId] {
        &self.dependencies
    }

    /// The PVT point this task was expanded to, or `None` when the task is not
    /// point-specific.
    pub(in crate::simulation) const fn pvt_point(
        &self,
    ) -> Option<&crate::state::AnalysisResultPvtPoint> {
        self.pvt_point.as_ref()
    }

    /// The PVT declaration this task solves one point of, when it is one.
    pub(in crate::simulation) const fn declared_point(
        &self,
    ) -> Option<&crate::simulation::point_family::DeclaredRunPoint> {
        self.declared_point.as_ref()
    }

    pub(in crate::simulation) fn resolve_dependency_artifacts(
        self,
        artifacts: &HashMap<AnalysisInstanceId, ExecutionArtifactEnvelope>,
    ) -> Result<ResolvedTaskDispatch, ExecutionArtifactError> {
        let dependencies = ResolvedExecutionDependencies::resolve(
            self.snapshot_digest,
            self.dependency_bindings.clone(),
            artifacts,
        )?;
        dependencies.validate_for_spec(&self.task.spec)?;
        Ok(ResolvedTaskDispatch {
            dispatch: self,
            dependencies,
        })
    }

    pub(in crate::simulation) fn label(&self) -> &str {
        &self.label
    }

    pub(in crate::simulation) const fn config_digest(&self) -> ContentDigest {
        self.config_digest
    }

    pub(in crate::simulation) fn touchstone_export_policy(&self) -> &TouchstoneExportPolicy {
        &self.touchstone_export
    }

    pub(in crate::simulation) fn spec(&self) -> &crate::simulation::multi_run::AnalysisSpec {
        &self.task.spec
    }

    pub(in crate::simulation) fn config(&self) -> Option<&crate::simulation::AnalysisConfig> {
        self.task.config.as_ref()
    }

    pub(in crate::simulation) fn spec_options(
        &self,
    ) -> &crate::simulation::runner::SpecExecutionOptions {
        &self.task.spec_options
    }

    pub(in crate::simulation) fn saved_output_contracts(&self) -> &[PreparedSavedOutput] {
        &self.saved_output_contracts
    }

    pub(in crate::simulation) fn executable_netlist(&self) -> &str {
        &self.executable_netlist
    }
}

impl ResolvedTaskDispatch {
    pub(in crate::simulation) fn into_runner_parts(
        self,
    ) -> (
        QueuedAnalysis,
        Arc<str>,
        crate::simulation::veriloga::PreparedVerilogARuntimeSet,
        ResolvedExecutionDependencies,
        Option<crate::simulation::runner::AnalysisExecutionEnvironment>,
    ) {
        (
            self.dispatch.task,
            self.dispatch.executable_netlist,
            self.dispatch.project_veriloga_runtimes,
            self.dependencies,
            self.dispatch.execution_environment,
        )
    }
}

/// Constructor input kept separate so the snapshot's authoritative fields
/// remain private after construction.
pub(in crate::simulation) struct SnapshotParts {
    pub(in crate::simulation) intent: SimulationRunIntent,
    pub(in crate::simulation) simulation_plan_id: Option<SimulationPlanId>,
    pub(in crate::simulation) project_revision: u64,
    pub(in crate::simulation) topology_revision: u64,
    pub(in crate::simulation) source_digest: ContentDigest,
    pub(in crate::simulation) reference_process: ProcessCorner,
    pub(in crate::simulation) reference_temperature_celsius: f64,
    /// Global Studio Run Set. Manual decks retain their authored `.step`,
    /// `.temp`, and corner directives and therefore leave this absent.
    pub(in crate::simulation) run_set: Option<PreparedRunSet>,
    pub(in crate::simulation) tasks: Vec<PreparedTask>,
    pub(in crate::simulation) executable_netlist: String,
    pub(in crate::simulation) save_policy: SavePolicy,
    pub(in crate::simulation) model_identities: Vec<ModelSourceIdentity>,
    pub(in crate::simulation) project_model_sources: Vec<PreparedModelSourceIdentity>,
    pub(in crate::simulation) specifications: Vec<PreparedSpecification>,
    pub(in crate::simulation) specification_policy: PreparedSpecificationPolicy,
    pub(in crate::simulation) project_veriloga_runtimes:
        crate::simulation::veriloga::PreparedVerilogARuntimeSet,
    pub(in crate::simulation) target: ExecutionTargetCapabilities,
    pub(in crate::simulation) receipt: RunSourceReceipt,
    pub(in crate::simulation) advisories: Vec<String>,
    pub(in crate::simulation) manual_source: Option<String>,
    pub(in crate::simulation) cross_probe: Option<CrossProbeSnapshot>,
    pub(in crate::simulation) touchstone_export: TouchstoneExportPolicy,
    pub(in crate::simulation) sealed_source_dependencies:
        Vec<rspice_core::netlist::ResolvedIncludeDependency>,
}

/// Validated inputs needed to turn the Studio's global Run Set into exact
/// point-specific tasks. The authored state preserves disabled axes and
/// filtered point identities; the runner configuration supplies sealed model
/// bindings and nominal-voltage behavior.
#[derive(Debug, Clone)]
pub(in crate::simulation) struct PreparedRunSet {
    state: RunSetState,
    corner_contract: crate::services::simulation_runner::CornerRunConfig,
}

impl PreparedRunSet {
    pub(in crate::simulation) fn new(
        state: RunSetState,
        corner_contract: crate::services::simulation_runner::CornerRunConfig,
    ) -> Self {
        Self {
            state,
            corner_contract,
        }
    }
}

/// Complete immutable execution tuple produced by preflight.
///
/// No field is public and no mutator exists. Dispatch accessors expose only
/// clones or consuming ancillary data, never a way to alter canonical state.
#[derive(Clone)]
pub(in crate::simulation) struct PreparedRunSnapshot {
    digest: ContentDigest,
    intent: SimulationRunIntent,
    simulation_plan_id: Option<SimulationPlanId>,
    project_revision: u64,
    topology_revision: u64,
    source_digest: ContentDigest,
    pvt_points: Vec<PreparedPvtPoint>,
    tasks: Vec<PreparedTask>,
    executable_netlist: String,
    save_policy: SavePolicy,
    model_identities: Vec<ModelSourceIdentity>,
    project_model_sources: Vec<PreparedModelSourceIdentity>,
    specifications: Vec<PreparedSpecification>,
    specification_policy: PreparedSpecificationPolicy,
    project_veriloga_runtimes: crate::simulation::veriloga::PreparedVerilogARuntimeSet,
    target: ExecutionTargetCapabilities,
    receipt: RunSourceReceipt,
    advisories: Vec<String>,
    manual_source: Option<String>,
    cross_probe: Option<CrossProbeSnapshot>,
    touchstone_export: TouchstoneExportPolicy,
    sealed_source_dependencies: Vec<rspice_core::netlist::ResolvedIncludeDependency>,
}

impl std::fmt::Debug for PreparedRunSnapshot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedRunSnapshot")
            .field("digest", &self.digest)
            .field("intent", &self.intent)
            .field("simulation_plan_id", &self.simulation_plan_id)
            .field("project_revision", &self.project_revision)
            .field("topology_revision", &self.topology_revision)
            .field("source_digest", &self.source_digest)
            .field("tasks", &self.tasks.len())
            .field("executable_netlist_bytes", &self.executable_netlist.len())
            .field(
                "sealed_source_dependencies",
                &self.sealed_source_dependencies.len(),
            )
            .finish_non_exhaustive()
    }
}

impl PreparedRunSnapshot {
    pub(in crate::simulation) const fn simulation_plan_id(&self) -> Option<SimulationPlanId> {
        self.simulation_plan_id
    }

    pub(in crate::simulation) fn new(mut parts: SnapshotParts) -> Result<Self, PreparationError> {
        ObjectRevision::new(parts.project_revision).map_err(|error| {
            PreparationError::new(
                PreparationStage::Authorization,
                format!("Prepared run has invalid project revision: {error}"),
            )
        })?;
        match (parts.intent, parts.simulation_plan_id, parts.receipt) {
            (SimulationRunIntent::SimulateRunSet, Some(_), RunSourceReceipt::SchematicDrc(_))
            | (SimulationRunIntent::ManualDeck, None, RunSourceReceipt::ManualSourceCheck(_)) => {}
            _ => {
                return Err(PreparationError::new(
                    PreparationStage::Authorization,
                    "Prepared run intent, simulation plan identity, and source-check receipt disagree",
                ));
            }
        }
        if parts.tasks.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Prepared task graph must contain at least one analysis",
            ));
        }
        if parts.intent == SimulationRunIntent::ManualDeck && !parts.specifications.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Manual-deck runs cannot claim simulation-plan specifications",
            ));
        }
        let mut specification_names = HashSet::with_capacity(parts.specifications.len());
        for specification in &parts.specifications {
            specification.entry().validate().map_err(|error| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!("Prepared specification is invalid: {error}"),
                )
            })?;
            if !specification_names.insert(specification.entry().measurement.to_ascii_lowercase()) {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Prepared specification measurement '{}' is duplicated",
                        specification.entry().measurement
                    ),
                ));
            }
        }
        if parts.executable_netlist.trim().is_empty() {
            return Err(PreparationError::new(
                PreparationStage::Netlist,
                "Prepared executable netlist is empty",
            ));
        }
        let source_revision = parts.tasks[0].source_revision;
        if parts
            .tasks
            .iter()
            .any(|task| task.source_revision != source_revision)
        {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Prepared task graph cannot mix source revisions in one frozen run",
            ));
        }
        for (index, task) in parts.tasks.iter().enumerate() {
            validate_prepared_task_integrity(
                task,
                index,
                parts.source_digest,
                &parts.executable_netlist,
            )?;
        }

        // Resolve the run-level PVT set before authenticating the task graph.
        // An OP configured for the PVT/run-set axis is one solve per exact
        // point, not one nominal solve accompanied by descriptive metadata.
        // Expansion also rewires downstream ordering edges to the final OP
        // point so dispatch cannot observe a partially completed sequence.
        let pvt_points = derive_pvt_points(
            &parts.tasks,
            parts.reference_process,
            parts.reference_temperature_celsius,
            parts.run_set.as_ref(),
        )?;
        let global_run_set_has_axes = parts
            .run_set
            .as_ref()
            .is_some_and(|run_set| run_set.state.enabled_dimensions().next().is_some());
        parts.tasks = if global_run_set_has_axes {
            expand_global_run_set_tasks(
                parts.tasks,
                &pvt_points,
                &parts.executable_netlist,
                parts.reference_process,
                parts.reference_temperature_celsius,
            )?
        } else {
            // A Run Set object exists even when all of its axes are disabled.
            // That reference-only state must not hide an analysis's own
            // Temperature or Corner declaration; the declaration-aware path
            // expands those exact points and keeps its family assembly task.
            expand_pvt_point_tasks(
                parts.tasks,
                &pvt_points,
                &parts.executable_netlist,
                parts.reference_process,
                parts.reference_temperature_celsius,
            )?
        };

        // A per-analysis solver override reaches the engine exactly the way the
        // plan's own policy does: as an `.OPTIONS` card in the deck. Splicing
        // it here, after per-point expansion has chosen each task's deck, is
        // what lets a PVT point keep its process binding and still resolve
        // under the analysis's own bounds. The parser takes the last card per
        // key, so the analysis wins over the plan without either block having
        // to know the other exists.
        for task in &mut parts.tasks {
            let Some(block) = task
                .task
                .numeric_override
                .as_ref()
                .map(AnalysisNumericOverride::to_spice_options)
                .filter(|block| !block.is_empty())
            else {
                continue;
            };
            let deck = task
                .executable_netlist_override
                .as_deref()
                .unwrap_or(&parts.executable_netlist);
            task.executable_netlist_override = Some(splice_before_terminal_end_card(deck, &block));
        }

        let mut positions = HashMap::with_capacity(parts.tasks.len());
        for (index, task) in parts.tasks.iter().enumerate() {
            if positions.insert(task.instance_id, index).is_some() {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Prepared task graph contains duplicate analysis instance identity {}",
                        task.instance_id
                    ),
                ));
            }
            validate_prepared_task_integrity(
                task,
                index,
                parts.source_digest,
                &parts.executable_netlist,
            )?;
        }

        for task in &parts.tasks {
            let mut dependencies = HashSet::with_capacity(task.dependencies.len());
            for dependency in &task.dependencies {
                if !dependencies.insert(*dependency) {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Prepared analysis {} contains duplicate dependency {}",
                            task.instance_id, dependency
                        ),
                    ));
                }
                if *dependency == task.instance_id {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Prepared analysis {} cannot depend on itself",
                            task.instance_id
                        ),
                    ));
                }
                if !positions.contains_key(dependency) {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Prepared analysis {} references missing dependency {}",
                            task.instance_id, dependency
                        ),
                    ));
                }
            }
        }

        if let Some(cycle) = dependency_cycle(&parts.tasks, &positions) {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Prepared task graph contains a dependency cycle involving {cycle:?}"),
            ));
        }
        for (index, task) in parts.tasks.iter().enumerate() {
            for dependency in &task.dependencies {
                if positions[dependency] >= index {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Prepared analysis {} dependency {} must appear earlier in the frozen task order",
                            task.instance_id, dependency
                        ),
                    ));
                }
            }

            let expected_artifact_kind = match task.task.spec {
                AnalysisSpec::Fourier { .. } => Some(ExecutionArtifactKind::TransientTrajectory),
                AnalysisSpec::Hbsp { .. } | AnalysisSpec::Hbnoise { .. } => {
                    Some(ExecutionArtifactKind::HbState)
                }
                AnalysisSpec::Pss {
                    method: crate::simulation::multi_run::PssMethod::Shooting,
                    ..
                } => Some(ExecutionArtifactKind::DcOperatingPointSeed),
                AnalysisSpec::PssSpectrum { .. }
                | AnalysisSpec::Pac
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pstb
                | AnalysisSpec::Psp { .. } => Some(ExecutionArtifactKind::PeriodicState),
                _ => None,
            };
            let expected_binding_count = usize::from(expected_artifact_kind.is_some());
            if task.dependency_bindings.len() != expected_binding_count {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Prepared {} task {} requires {expected_binding_count} typed artifact binding(s), received {}",
                        task.task.spec.run_type().display_name(),
                        task.instance_id,
                        task.dependency_bindings.len()
                    ),
                ));
            }
            let mut bound_producers = HashSet::with_capacity(task.dependency_bindings.len());
            for binding in &task.dependency_bindings {
                let producer_id = binding.producer_instance_id();
                if !bound_producers.insert(producer_id) {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Prepared analysis {} repeats typed artifact producer {}",
                            task.instance_id, producer_id
                        ),
                    ));
                }
                if !task.dependencies.contains(&producer_id) {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Prepared analysis {} artifact producer {} is not an authenticated dependency edge",
                            task.instance_id, producer_id
                        ),
                    ));
                }
                let producer = &parts.tasks[positions[&producer_id]];
                let producer_kind_matches = match binding.kind() {
                    ExecutionArtifactKind::TransientTrajectory => {
                        matches!(producer.task.spec, AnalysisSpec::Transient { .. })
                    }
                    ExecutionArtifactKind::PeriodicState => {
                        matches!(producer.task.spec, AnalysisSpec::Pss { .. })
                    }
                    ExecutionArtifactKind::HbState => {
                        matches!(producer.task.spec, AnalysisSpec::HarmonicBalance { .. })
                    }
                    ExecutionArtifactKind::DcOperatingPointSeed => matches!(
                        producer.task.spec,
                        AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }
                    ),
                };
                if Some(binding.kind()) != expected_artifact_kind
                    || !producer_kind_matches
                    || binding.producer_source_revision() != producer.source_revision
                    || binding.producer_config_digest() != producer.config_digest
                {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Prepared analysis {} has a mismatched typed artifact binding for producer {}",
                            task.instance_id, producer_id
                        ),
                    ));
                }
                validate_prepared_dependency_contract_with_options(
                    &task.task.spec,
                    &task.task.spec_options,
                    &producer.task.spec,
                )
                .map_err(|error| {
                        PreparationError::new(
                            PreparationStage::AnalysisPlan,
                            format!(
                                "Prepared analysis {} dependency contract with producer {} is invalid: {error}",
                                task.instance_id, producer_id
                            ),
                        )
                    })?;
            }
        }

        // Model identities describe a set. Sort by canonical label/digest so
        // discovery or map insertion order cannot perturb snapshot identity;
        // executable model precedence remains bound by the netlist bytes.
        parts
            .project_veriloga_runtimes
            .validate()
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!("Sealed Verilog-A runtime set is invalid: {error}"),
                )
            })?;
        let parsed_netlist =
            rspice_core::Netlist::parse(&parts.executable_netlist).map_err(|error| {
                PreparationError::new(
                    PreparationStage::Netlist,
                    format!("Prepared executable netlist is invalid: {error}"),
                )
            })?;
        let model_bin_config = rspice_core::SimulationConfig {
            temperature: rspice_core::constants::celsius_to_kelvin(
                parts.reference_temperature_celsius,
            ),
            ..Default::default()
        };
        rspice_core::Engine::new(model_bin_config)
            .validate_model_bin_contracts(&parsed_netlist)
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!("Prepared model-bin resolution is invalid: {error}"),
                )
            })?;
        let expected_veriloga_bindings = parts
            .project_veriloga_runtimes
            .iter()
            .map(|runtime| {
                (
                    runtime.source_key().to_owned(),
                    runtime.netlist_alias().to_ascii_uppercase(),
                )
            })
            .collect::<HashSet<_>>();
        let mut observed_veriloga_bindings = HashSet::new();
        for include in &parsed_netlist.veriloga_includes {
            let Some(source_key) = include.file_path.to_str() else {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    "Prepared Verilog-A source identity is not valid UTF-8",
                ));
            };
            let Some(alias) = include.model_name.as_deref() else {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Prepared Verilog-A source '{source_key}' must declare its sealed model alias"
                    ),
                ));
            };
            if !observed_veriloga_bindings
                .insert((source_key.to_owned(), alias.to_ascii_uppercase()))
            {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Prepared Verilog-A binding '{source_key}' with alias '{alias}' is duplicated"
                    ),
                ));
            }
        }
        if observed_veriloga_bindings != expected_veriloga_bindings {
            return Err(PreparationError::new(
                PreparationStage::ModelBindings,
                "Every executable Verilog-A directive must match exactly one sealed runtime",
            ));
        }
        for runtime in parts.project_veriloga_runtimes.iter() {
            runtime.validate().map_err(|error| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!("Sealed Verilog-A runtime is invalid: {error}"),
                )
            })?;
            let directive = crate::simulation::veriloga::project_veriloga_directive(
                runtime.source_key(),
                runtime.netlist_alias(),
            );
            let directive_count = parts
                .executable_netlist
                .lines()
                .filter(|line| line.trim().eq_ignore_ascii_case(&directive))
                .count();
            if directive_count != 1 {
                return Err(PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!(
                        "Prepared sealed Verilog-A runtime '{}' must be referenced exactly once by the executable netlist (found {directive_count})",
                        runtime.netlist_alias()
                    ),
                ));
            }
            parts.model_identities.push(ModelSourceIdentity::new(
                runtime.provenance_label(),
                runtime.artifact_digest(),
            ));
        }
        parts.project_model_sources.sort_by(|left, right| {
            left.source_id()
                .as_uuid()
                .cmp(&right.source_id().as_uuid())
                .then_with(|| {
                    left.model_name()
                        .to_ascii_lowercase()
                        .cmp(&right.model_name().to_ascii_lowercase())
                })
                .then_with(|| left.revision().cmp(&right.revision()))
                .then_with(|| left.content_digest().cmp(&right.content_digest()))
        });
        parts.project_model_sources.dedup();
        for model in &parts.project_model_sources {
            parts.model_identities.push(ModelSourceIdentity::new(
                format!(
                    "project-model:{}:{}:{}",
                    model.source_id(),
                    model.model_name(),
                    model.revision().get()
                ),
                model.content_digest(),
            ));
        }
        parts.model_identities.sort_unstable();
        parts.model_identities.dedup();

        let digest = snapshot_digest(
            parts.intent,
            parts.simulation_plan_id,
            parts.project_revision,
            parts.topology_revision,
            parts.source_digest,
            &pvt_points,
            &parts.tasks,
            &parts.specifications,
            &parts.specification_policy,
            parts.save_policy,
            &parts.model_identities,
            &parts.target,
            parts.receipt,
            &parts.touchstone_export,
            &parts.executable_netlist,
        );

        Ok(Self {
            digest,
            intent: parts.intent,
            simulation_plan_id: parts.simulation_plan_id,
            project_revision: parts.project_revision,
            topology_revision: parts.topology_revision,
            source_digest: parts.source_digest,
            pvt_points,
            tasks: parts.tasks,
            executable_netlist: parts.executable_netlist,
            save_policy: parts.save_policy,
            model_identities: parts.model_identities,
            project_model_sources: parts.project_model_sources,
            specifications: parts.specifications,
            specification_policy: parts.specification_policy,
            project_veriloga_runtimes: parts.project_veriloga_runtimes,
            target: parts.target,
            receipt: parts.receipt,
            advisories: parts.advisories,
            manual_source: parts.manual_source,
            cross_probe: parts.cross_probe,
            touchstone_export: parts.touchstone_export,
            sealed_source_dependencies: parts.sealed_source_dependencies,
        })
    }

    pub(in crate::simulation) const fn digest(&self) -> ContentDigest {
        self.digest
    }

    /// The master each occurrence was emitted against; a manual deck has none.
    pub(in crate::simulation) fn emission_map(&self) -> &[EmissionRow] {
        self.cross_probe
            .as_ref()
            .map_or(&[], |design| design.emission_map.as_slice())
    }

    #[cfg(test)]
    pub(in crate::simulation) fn executable_netlist(&self) -> &str {
        &self.executable_netlist
    }

    pub(in crate::simulation) const fn intent(&self) -> SimulationRunIntent {
        self.intent
    }

    pub(in crate::simulation) fn metadata(&self) -> PreparedRunMetadata {
        PreparedRunMetadata {
            snapshot_digest: self.digest,
            source_digest: self.source_digest,
            receipt_digest: self.receipt.digest(),
            receipt_label: self.receipt.label(),
            project_revision: self.project_revision,
            topology_revision: self.topology_revision,
            analysis_ids: self
                .tasks
                .iter()
                .map(|task| analysis_id_metadata_digest(task.instance_id))
                .collect(),
            task_count: self.tasks.len(),
            saved_output_contract_count: self
                .tasks
                .iter()
                .map(|task| task.saved_output_contracts.len())
                .sum(),
            pvt_point_count: self.pvt_points.len(),
            target: self.target.label(),
            save_policy: self.save_policy.label(),
            model_identity_count: self.model_identities.len(),
            advisories: self.advisories.clone(),
            sealed_source_dependencies: self.sealed_source_dependencies.clone(),
        }
    }

    /// Convert canonical inputs into runner-visible task objects only after a
    /// successful one-use permit CAS for this exact snapshot digest.
    pub(in crate::simulation) fn authorize_dispatch(
        self,
        proof: ConsumedExecutionPermit,
    ) -> Result<AuthorizedRunDispatch, PreparationError> {
        if proof.snapshot_digest() != self.digest {
            return Err(PreparationError::new(
                PreparationStage::Authorization,
                "Consumed execution permit does not match the retained snapshot",
            ));
        }
        let executable_netlist: Arc<str> = Arc::from(self.executable_netlist);
        let default_touchstone_export = self.touchstone_export.clone();
        let project_veriloga_runtimes = self.project_veriloga_runtimes.clone();
        let tasks = self
            .tasks
            .into_iter()
            .map(|prepared| {
                let touchstone_export = if matches!(
                    &prepared.task.spec,
                    crate::simulation::multi_run::AnalysisSpec::SParameter { .. }
                ) {
                    prepared
                        .touchstone_export
                        .unwrap_or_else(|| default_touchstone_export.clone())
                } else {
                    TouchstoneExportPolicy::disabled()
                };
                AuthorizedTaskDispatch {
                    snapshot_digest: self.digest,
                    instance_id: prepared.instance_id,
                    authored_instance_id: prepared.authored_instance_id,
                    source_revision: prepared.source_revision,
                    dependencies: prepared.dependencies,
                    dependency_bindings: prepared.dependency_bindings,
                    label: prepared.label,
                    config_digest: prepared.config_digest,
                    task: prepared.task,
                    saved_output_contracts: prepared.saved_output_contracts,
                    executable_netlist: prepared
                        .executable_netlist_override
                        .map(Arc::<str>::from)
                        .unwrap_or_else(|| Arc::clone(&executable_netlist)),
                    project_veriloga_runtimes: project_veriloga_runtimes.clone(),
                    touchstone_export,
                    pvt_point: prepared.pvt_point,
                    declared_point: prepared.declared_point,
                    execution_environment: prepared.execution_environment,
                }
            })
            .collect();
        Ok(AuthorizedRunDispatch {
            digest: self.digest,
            intent: self.intent,
            simulation_plan_id: self.simulation_plan_id,
            project_revision: self.project_revision,
            source_digest: self.source_digest,
            source_receipt: self.receipt,
            save_policy: self.save_policy,
            project_model_sources: self.project_model_sources,
            specifications: self.specifications,
            specification_policy: self.specification_policy,
            tasks,
            executable_netlist,
            advisories: self.advisories,
            manual_source: self.manual_source,
            cross_probe: self.cross_probe,
        })
    }
}

fn validate_prepared_task_integrity(
    task: &PreparedTask,
    index: usize,
    source_digest: ContentDigest,
    executable_netlist: &str,
) -> Result<(), PreparationError> {
    if task.config_digest != task.payload_digest() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "Prepared task {} configuration digest does not authenticate its actual dispatch payload",
                index + 1
            ),
        ));
    }
    let exact_source = task
        .executable_netlist_override
        .as_deref()
        .unwrap_or(executable_netlist);
    let exact_source_digest = if task.executable_netlist_override.is_some() {
        crate::workbench::documents::netlist_document::source_content_digest(exact_source)
    } else {
        source_digest
    };
    validate_retained_operating_point_contract(&task.task, exact_source_digest, exact_source)
        .map_err(|message| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Prepared operating-point task {} is invalid: {message}",
                    task.instance_id
                ),
            )
        })?;

    let mut output_ids = HashSet::with_capacity(task.saved_output_contracts.len());
    let mut output_names = HashSet::with_capacity(task.saved_output_contracts.len());
    let mut output_digests = HashSet::with_capacity(task.saved_output_contracts.len());
    for contract in &task.saved_output_contracts {
        if contract.analysis_id() != task.instance_id {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Prepared saved output {} targets analysis {}, not its owning task {}",
                    contract.output_id(),
                    contract.analysis_id(),
                    task.instance_id
                ),
            ));
        }
        if !output_ids.insert(contract.output_id())
            || !output_names.insert(contract.name())
            || !output_digests.insert(contract.digest())
        {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Prepared task {} contains duplicate saved-output identity, name, or contract digest",
                    task.instance_id
                ),
            ));
        }
    }
    Ok(())
}

/// Whether a point is the run's own reference point.
///
/// Exact comparison, not a tolerance: the reference process and temperature
/// are the same values the axes were resolved from, so a point that sits on
/// the reference carries its bits unchanged. A supply axis is nominal only at
/// the resolved nominal supply; a run with no supply axis left the deck's own
/// supply standing, which is nominal by construction.
fn point_is_nominal(
    point: &PreparedPvtPoint,
    nominal_supply_voltage: Option<f64>,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> bool {
    if point.process != reference_process
        || point.temperature_celsius != reference_temperature_celsius
    {
        return false;
    }
    match (point.voltage, nominal_supply_voltage) {
        (None, _) => true,
        (Some(voltage), Some(nominal)) => voltage == nominal,
        (Some(_), None) => false,
    }
}

fn run_set_point_task_label(
    original_label: &str,
    point: &PreparedPvtPoint,
    point_index: usize,
    point_count: usize,
) -> String {
    let mut label = format!(
        "{original_label} \u{00b7} point {}/{} \u{00b7} {}",
        point_index + 1,
        point_count,
        point.process.short_name(),
    );
    if let Some(voltage) = point.voltage {
        label.push_str(&format!(" \u{00b7} {voltage} V"));
    }
    label.push_str(&format!(
        " \u{00b7} {} \u{00b0}C",
        point.temperature_celsius
    ));
    for (name, value) in &point.parameter_overrides {
        label.push_str(&format!(" \u{00b7} param {name}={value}"));
    }
    for (name, value) in &point.source_overrides {
        label.push_str(&format!(" \u{00b7} source {name}={value}"));
    }
    label
}

/// Expand every ordinary plan analysis across the Studio's global Run Set.
///
/// The point identity is part of the task identity, configuration digest,
/// prepared source, result attribution, and dependency mapping. This keeps the
/// forecasted matrix and the authorized worker queue as one actual expansion
/// instead of a nominal task accompanied by descriptive PVT metadata.
fn expand_global_run_set_tasks(
    tasks: Vec<PreparedTask>,
    pvt_points: &[PreparedPvtPoint],
    executable_netlist: &str,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> Result<Vec<PreparedTask>, PreparationError> {
    if pvt_points.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Global Run Set did not produce any execution points",
        ));
    }
    let requested = tasks.len().checked_mul(pvt_points.len()).ok_or_else(|| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Global Run Set task count overflowed the platform task capacity",
        )
    })?;
    ensure_pvt_point_capacity(
        0,
        requested,
        rspice_core::ResourceLimits::default().max_batch_runs,
    )?;

    // A Temperature or Corner analysis owns another point declaration. Its
    // implicit cross-product with this global declaration would make both the
    // forecast and result-family authority ambiguous, so it remains an
    // explicit refusal. Ordinary spec-driven analyses receive their global
    // point through an authenticated deck below.
    if let Some(task) = tasks.iter().find(|task| {
        matches!(task.task.spec, AnalysisSpec::Corner)
            || matches!(task.task.spec, AnalysisSpec::Parametric)
                && task.task.spec_options.temp.is_some()
    }) {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "{} owns an internal point declaration and cannot execute inside the global multi-point Run Set; disable the global axes or remove the nested declaration",
                task.label
            ),
        ));
    }

    let mut prepared_sources = Vec::with_capacity(pvt_points.len());
    for point in pvt_points {
        prepared_sources.push(prepare_pvt_point_source(executable_netlist, point)?);
    }

    let mut identities = HashMap::with_capacity(requested);
    for task in &tasks {
        for (point_index, point) in pvt_points.iter().enumerate() {
            let identity = if pvt_points.len() == 1 {
                task.instance_id
            } else {
                let corner_digest = point
                    .corner_contract
                    .as_ref()
                    .map(corner_contract_digest)
                    .map_or_else(|| "none".to_owned(), |digest| digest.to_string());
                let mut identity_material = format!(
                    "rspice-global-run-set/v2/{point_index}/{}/{:016x}/{:016x}/{corner_digest}",
                    process_tag(point.process),
                    point.voltage.map(f64::to_bits).unwrap_or_default(),
                    point.temperature_celsius.to_bits(),
                );
                for (name, value) in &point.parameter_overrides {
                    identity_material.push('/');
                    identity_material.push_str(name);
                    identity_material.push('=');
                    identity_material.push_str(value);
                }
                for (name, value) in &point.source_overrides {
                    identity_material.push('/');
                    identity_material.push_str(name);
                    identity_material.push('=');
                    identity_material.push_str(value);
                }
                AnalysisInstanceId::from_namespace(
                    task.instance_id.as_uuid(),
                    identity_material.as_bytes(),
                )
            };
            identities.insert((task.instance_id, point_index), identity);
        }
    }

    let mut expanded = Vec::with_capacity(requested);
    let mut task_points = Vec::with_capacity(requested);
    for task in tasks {
        let original_identity = task.instance_id;
        let original_label = task.label.clone();
        for (point_index, point) in pvt_points.iter().enumerate() {
            let instance_id = identities[&(original_identity, point_index)];
            let mut point_task = task.clone();
            point_task.instance_id = instance_id;
            point_task.dependencies = task
                .dependencies
                .iter()
                .map(|dependency| {
                    identities
                        .get(&(*dependency, point_index))
                        .copied()
                        .ok_or_else(|| {
                            PreparationError::new(
                                PreparationStage::AnalysisPlan,
                                format!(
                                    "Run Set task {} references missing dependency {} at point {}",
                                    original_identity,
                                    dependency,
                                    point_index + 1
                                ),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;

            let (source_override, nominal_supply_voltage) = &prepared_sources[point_index];
            point_task.executable_netlist_override = source_override.clone();
            point_task.pvt_point = Some(
                crate::state::AnalysisResultPvtPoint::new(
                    point.process.short_name(),
                    point.voltage,
                    point.temperature_celsius,
                    point.corner_contract.as_ref().map(corner_contract_digest),
                    point_is_nominal(
                        point,
                        *nominal_supply_voltage,
                        reference_process,
                        reference_temperature_celsius,
                    ),
                )
                .map_err(|error| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Run Set point {}/{} cannot be attributed: {error}",
                            point_index + 1,
                            pvt_points.len()
                        ),
                    )
                })?,
            );

            if let Some(mut config) = operating_point_config(&point_task.task.spec) {
                use crate::simulation::dialog::OpRunPointContext;
                config.temperature_celsius = point.temperature_celsius;
                config.run_point = OpRunPointContext {
                    index: point_index,
                    count: pvt_points.len(),
                    process: point.process,
                    supply_voltage: point.voltage,
                    nominal_supply_voltage: *nominal_supply_voltage,
                    supply_source_names: point.supply_source_names.clone(),
                };
                point_task.task.spec = operating_point_spec(&config);
                point_task.task.config = Some(crate::simulation::AnalysisConfig::DcOp(config));
                point_task.execution_environment = None;
            } else {
                if let Some(crate::simulation::AnalysisConfig::Noise(config)) =
                    point_task.task.config.as_mut()
                {
                    config.temperature_kelvin =
                        rspice_core::constants::celsius_to_kelvin(point.temperature_celsius);
                }
                if let AnalysisSpec::Noise { temperature, .. } = &mut point_task.task.spec {
                    *temperature =
                        rspice_core::constants::celsius_to_kelvin(point.temperature_celsius);
                }
                let environment = crate::simulation::runner::AnalysisExecutionEnvironment {
                    temperature_celsius: point.temperature_celsius,
                    supply_voltage: point.voltage,
                    nominal_supply_voltage: *nominal_supply_voltage,
                    supply_source_names: point.supply_source_names.clone(),
                };

                // Configuration-backed analyses and Monte Carlo apply this
                // environment to their parsed deck at dispatch. A shooting
                // PSS applies the identical values from its same-point DC seed
                // artifact. Every other spec-driven service consumes normal
                // deck options, so freeze temperature and scaled supplies into
                // that task's authenticated source now.
                let materialize_environment = point_task.task.config.is_none()
                    && !matches!(point_task.task.spec, AnalysisSpec::MonteCarlo { .. })
                    && !matches!(
                        point_task.task.spec,
                        AnalysisSpec::Pss {
                            method: crate::simulation::multi_run::PssMethod::Shooting,
                            ..
                        }
                    );
                if materialize_environment {
                    let deck = point_task
                        .executable_netlist_override
                        .as_deref()
                        .unwrap_or(executable_netlist);
                    point_task.executable_netlist_override =
                        Some(materialize_spec_run_environment_source(
                            deck,
                            &environment,
                            point_index,
                            pvt_points.len(),
                        )?);
                }
                point_task.execution_environment = Some(environment);
            }

            point_task.label =
                run_set_point_task_label(&original_label, point, point_index, pvt_points.len());
            point_task.saved_output_contracts = task
                .saved_output_contracts
                .iter()
                .map(|contract| {
                    contract
                        .rebind_analysis(instance_id, &point_task.task.spec)
                        .map_err(|error| {
                            PreparationError::new(
                                PreparationStage::AnalysisPlan,
                                format!(
                                    "Failed to bind saved output to Run Set point {}/{}: {error}",
                                    point_index + 1,
                                    pvt_points.len()
                                ),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            point_task.config_digest = point_task.payload_digest();
            task_points.push((original_identity, point_index));
            expanded.push(point_task);
        }
    }

    let producer_details = expanded
        .iter()
        .map(|task| (task.instance_id, (task.source_revision, task.config_digest)))
        .collect::<HashMap<_, _>>();
    for (task, (_, point_index)) in expanded.iter_mut().zip(task_points) {
        for binding in &mut task.dependency_bindings {
            let producer = identities
                .get(&(binding.producer_instance_id(), point_index))
                .copied()
                .ok_or_else(|| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Run Set dependency binding references missing producer {} at point {}",
                            binding.producer_instance_id(),
                            point_index + 1
                        ),
                    )
                })?;
            let (revision, digest) = producer_details[&producer];
            binding.rebind_producer(producer, revision, digest);
        }
    }

    Ok(expanded)
}

/// Expand every task that is declared over the run set's PVT points into one
/// prepared task per point.
///
/// Three declarations reach this: an operating point bound to the run-set
/// axis, and the corner run and the temperature step, whose base analysis *is*
/// the thing declared over the points. All expand through the same per-point
/// ingredients — the process corner materialized into the point's own deck,
/// the point recorded on the task so its result can be attributed, and an
/// instance identity derived from the point so two points can never share one.
fn expand_pvt_point_tasks(
    tasks: Vec<PreparedTask>,
    pvt_points: &[PreparedPvtPoint],
    executable_netlist: &str,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> Result<Vec<PreparedTask>, PreparationError> {
    use crate::simulation::AnalysisConfig;
    use crate::simulation::dialog::{OpRunPointContext, OpTemperatureMode};

    let mut expanded = Vec::with_capacity(tasks.len().saturating_add(pvt_points.len()));
    let mut final_task = HashMap::<
        AnalysisInstanceId,
        (
            AnalysisInstanceId,
            ObjectRevision,
            ContentDigest,
            Option<String>,
        ),
    >::new();

    for mut prepared in tasks {
        let original_identity = prepared.instance_id;
        prepared.dependencies = prepared
            .dependencies
            .iter()
            .map(|dependency| {
                final_task
                    .get(dependency)
                    .map_or(*dependency, |(identity, _, _, _)| *identity)
            })
            .collect();
        let inherited_op_source_override = prepared
            .dependency_bindings
            .iter()
            .find(|binding| binding.kind() == ExecutionArtifactKind::DcOperatingPointSeed)
            .and_then(|binding| final_task.get(&binding.producer_instance_id()))
            .and_then(|(_, _, _, source)| source.clone());
        for binding in &mut prepared.dependency_bindings {
            if let Some((identity, revision, digest, _)) =
                final_task.get(&binding.producer_instance_id())
            {
                binding.rebind_producer(*identity, *revision, *digest);
            }
        }
        if matches!(
            prepared.task.spec,
            AnalysisSpec::Pss {
                method: crate::simulation::multi_run::PssMethod::Shooting,
                ..
            }
        ) {
            prepared.executable_netlist_override = inherited_op_source_override;
        }

        // A corner run and a temperature step are not one analysis swept along
        // an axis: each is a base analysis solved once per declared point, and
        // collapsing those solves into one scalar per node is what threw the
        // waveforms and the `.MEAS` results away. Each point earns its own task
        // so its result carries its own evidence and the point that produced
        // it.
        //
        // The declaration is no longer solved. Solving it would solve every
        // point a second time, and the family its plot reads is one scalar per
        // node per point — a view over the point results. It stays a task
        // because the run's authenticated receipt is built from this list and
        // the retained results must line up with it one for one; its turn in
        // the queue assembles the family instead of reaching the engine.
        //
        // Its position after the points is load-bearing. Results must remain an
        // exact ordered prefix of the receipt's tasks even when a run is
        // aborted or a task is blocked, and the assembly cannot produce a
        // result until its points have. Any earlier position would leave a hole
        // at the declaration's own index in every partial run.
        //
        // Its points are deliberately not its dependencies. The queue is
        // strictly vector order — a `VecDeque` popped from the front with one
        // task in flight — so the position alone orders it, whereas a
        // dependency edge would skip the family outright the moment a single
        // point failed to converge, which is the point a plot most needs.
        //
        // A dependent of the declaration binds to the last *point*: the
        // assembly produces no execution artifact, and the last point is the
        // task whose completion means the declared space has been solved. That
        // is the same rule the operating-point expansion below applies.
        let declared_point_tasks = match &prepared.task.spec {
            AnalysisSpec::Corner => Some(expand_corner_run_point_tasks(
                &prepared,
                executable_netlist,
                reference_process,
                reference_temperature_celsius,
            )?),
            // A parametric run declares a PVT space only when it carries a
            // temperature contract. Without one it steps a design parameter,
            // which the engine sweeps for itself inside a single solve and
            // which is not a condition a specification can be scoped to.
            AnalysisSpec::Parametric => match prepared.task.spec_options.temp.as_ref() {
                Some(contract) => Some(expand_temperature_run_point_tasks(
                    &prepared,
                    contract,
                    executable_netlist,
                    reference_process,
                    reference_temperature_celsius,
                )?),
                None => None,
            },
            _ => None,
        };
        if let Some(points) = declared_point_tasks {
            let last = points
                .last()
                .expect("an expansion refuses an empty declared space");
            final_task.insert(
                original_identity,
                (
                    last.instance_id,
                    last.source_revision,
                    last.config_digest,
                    last.executable_netlist_override.clone(),
                ),
            );
            expanded.extend(points);
            expanded.push(prepared);
            continue;
        }

        let Some(base_config) = operating_point_config(&prepared.task.spec) else {
            final_task.insert(
                original_identity,
                (
                    original_identity,
                    prepared.source_revision,
                    prepared.config_digest,
                    prepared.executable_netlist_override.clone(),
                ),
            );
            expanded.push(prepared);
            continue;
        };
        if !matches!(
            base_config.temperature_mode,
            OpTemperatureMode::PvtRunSet | OpTemperatureMode::ActiveRunSetAxis
        ) {
            final_task.insert(
                original_identity,
                (
                    original_identity,
                    prepared.source_revision,
                    prepared.config_digest,
                    prepared.executable_netlist_override.clone(),
                ),
            );
            expanded.push(prepared);
            continue;
        }

        let base_dependencies = prepared.dependencies.clone();
        let original_label = prepared.label.clone();
        let mut previous_point_identity = None;
        for (index, point) in pvt_points.iter().enumerate() {
            let instance_id = if pvt_points.len() == 1 {
                original_identity
            } else {
                let corner_digest = point
                    .corner_contract
                    .as_ref()
                    .map(corner_contract_digest)
                    .map_or_else(|| "none".to_owned(), |digest| digest.to_string());
                AnalysisInstanceId::from_namespace(
                    original_identity.as_uuid(),
                    format!(
                        "rspice-op-run-point/v2/{index}/{}/{}/{:016x}/{:016x}/{corner_digest}",
                        pvt_points.len(),
                        process_tag(point.process),
                        point.voltage.map(f64::to_bits).unwrap_or_default(),
                        point.temperature_celsius.to_bits(),
                    )
                    .as_bytes(),
                )
            };
            let mut point_task = prepared.clone();
            point_task.instance_id = instance_id;
            point_task.dependencies = base_dependencies.clone();
            if let Some(previous) = previous_point_identity {
                point_task.dependencies.push(previous);
            }

            let mut config = base_config.clone();
            config.temperature_celsius = point.temperature_celsius;
            let (source_override, nominal_supply_voltage) =
                prepare_pvt_point_source(executable_netlist, point)?;
            config.run_point = OpRunPointContext {
                index,
                count: pvt_points.len(),
                process: point.process,
                supply_voltage: point.voltage,
                nominal_supply_voltage,
                supply_source_names: point.supply_source_names.clone(),
            };
            point_task.pvt_point = Some(
                crate::state::AnalysisResultPvtPoint::new(
                    point.process.short_name(),
                    point.voltage,
                    point.temperature_celsius,
                    point.corner_contract.as_ref().map(corner_contract_digest),
                    point_is_nominal(
                        point,
                        nominal_supply_voltage,
                        reference_process,
                        reference_temperature_celsius,
                    ),
                )
                .map_err(|error| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Operating-point run point {}/{} cannot be attributed: {error}",
                            index + 1,
                            pvt_points.len()
                        ),
                    )
                })?,
            );
            point_task.executable_netlist_override = source_override;
            point_task.task.spec = operating_point_spec(&config);
            point_task.task.config = Some(AnalysisConfig::DcOp(config));
            if pvt_points.len() > 1 {
                point_task.label =
                    run_set_point_task_label(&original_label, point, index, pvt_points.len());
            }
            point_task.saved_output_contracts = prepared
                .saved_output_contracts
                .iter()
                .map(|contract| {
                    contract
                        .rebind_analysis(instance_id, &point_task.task.spec)
                        .map_err(|error| {
                            PreparationError::new(
                                PreparationStage::AnalysisPlan,
                                format!(
                                    "Failed to bind saved output to operating-point run point {}/{}: {error}",
                                    index + 1,
                                    pvt_points.len()
                                ),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            point_task.config_digest = point_task.payload_digest();
            previous_point_identity = Some(instance_id);
            expanded.push(point_task);
        }
        let final_identity = previous_point_identity.expect("validated PVT point set is non-empty");
        let final_prepared = expanded
            .last()
            .expect("expanded operating-point task retains its final point");
        final_task.insert(
            original_identity,
            (
                final_identity,
                final_prepared.source_revision,
                final_prepared.config_digest,
                final_prepared.executable_netlist_override.clone(),
            ),
        );
    }

    Ok(expanded)
}

fn prepare_pvt_point_source(
    executable_netlist: &str,
    point: &PreparedPvtPoint,
) -> Result<(Option<String>, Option<f64>), PreparationError> {
    let Some(contract) = point.corner_contract.as_ref() else {
        if point.voltage.is_some() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "PVT run point voltage is missing its authenticated corner contract",
            ));
        }
        let source = materialize_source_overrides(executable_netlist, &point.source_overrides)?;
        let source = materialize_parameter_overrides(&source, &point.parameter_overrides);
        return Ok(((source != executable_netlist).then_some(source), None));
    };

    let source = crate::services::simulation_runner::materialize_corner_process_source(
        executable_netlist,
        contract,
        process_to_corner_runner(point.process),
        &rspice_core::NoAbort,
    )
    .map_err(|error| {
        PreparationError::new(
            PreparationStage::ModelBindings,
            format!(
                "Failed to materialize the {} PVT process corner: {error}",
                point.process.short_name()
            ),
        )
    })?;

    let nominal_supply_voltage = if point.voltage.is_some() {
        let parsed = rspice_core::Netlist::parse(&source).map_err(|error| {
            PreparationError::new(
                PreparationStage::Netlist,
                format!("Prepared operating-point corner source is invalid: {error}"),
            )
        })?;
        Some(match contract.nominal_voltage {
            Some(voltage) => voltage,
            None => crate::services::simulation_runner::infer_nominal_supply_voltage(
                &parsed,
                &point.supply_source_names,
                &rspice_core::NoAbort,
            )
            .map_err(|error| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!("Failed to resolve nominal PVT supply voltage: {error}"),
                )
            })?
            .ok_or_else(|| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    "A PVT supply axis requires a non-zero independent DC supply or an explicit nominal voltage",
                )
            })?,
        })
    } else {
        None
    };

    let source = materialize_source_overrides(&source, &point.source_overrides)?;
    let source = materialize_parameter_overrides(&source, &point.parameter_overrides);
    let source_override = (source != executable_netlist).then_some(source);
    Ok((source_override, nominal_supply_voltage))
}

/// Freeze a global Run Set point into a spec-driven task's exact deck.
///
/// Service runners resolve `.OPTIONS TEMP` through their shared engine-config
/// builder. Supply scaling is materialized as exact DC card values because the
/// parsed-netlist environment hook is intentionally owned by configuration
/// requests and Monte Carlo. Selection and validation mirror the core supply
/// scaler: only explicitly bound independent voltage sources with a `DC` or
/// `DC ... AC ...` value are eligible.
fn materialize_spec_run_environment_source(
    executable_netlist: &str,
    environment: &crate::simulation::runner::AnalysisExecutionEnvironment,
    point_index: usize,
    point_count: usize,
) -> Result<String, PreparationError> {
    if !environment.temperature_celsius.is_finite() || environment.temperature_celsius <= -273.15 {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "Run Set point {}/{} has an invalid temperature",
                point_index + 1,
                point_count
            ),
        ));
    }

    let mut source = executable_netlist.to_owned();
    match (
        environment.supply_voltage,
        environment.nominal_supply_voltage,
    ) {
        (None, None) => {}
        (Some(supply), Some(nominal)) => {
            if !supply.is_finite() || supply <= 0.0 || !nominal.is_finite() || nominal <= 0.0 {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Run Set point {}/{} has an invalid supply or nominal voltage",
                        point_index + 1,
                        point_count
                    ),
                ));
            }
            if environment.supply_source_names.is_empty() {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Run Set point {}/{} has a supply value without an explicitly bound source",
                        point_index + 1,
                        point_count
                    ),
                ));
            }
            let scale = supply / nominal;
            for source_name in &environment.supply_source_names {
                let parsed = rspice_core::Netlist::parse(&source).map_err(|error| {
                    PreparationError::new(
                        PreparationStage::Netlist,
                        format!(
                            "Cannot materialize Run Set supply {source_name:?} at point {}/{}: {error}",
                            point_index + 1,
                            point_count
                        ),
                    )
                })?;
                let element = parsed
                    .elements
                    .iter()
                    .find(|element| element.name.eq_ignore_ascii_case(source_name))
                    .ok_or_else(|| {
                        PreparationError::new(
                            PreparationStage::AnalysisPlan,
                            format!(
                                "Run Set supply source {source_name:?} is absent at point {}/{}",
                                point_index + 1,
                                point_count
                            ),
                        )
                    })?;
                let rspice_core::netlist::ElementKind::VoltageSource(spec) = &element.kind else {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Run Set supply binding {source_name:?} is not an independent voltage source"
                        ),
                    ));
                };
                let dc = scalable_supply_dc_value(spec).ok_or_else(|| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!("Run Set supply source {source_name:?} has no scalable DC value"),
                    )
                })?;
                let scaled = dc * scale;
                if !scaled.is_finite() {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Run Set supply source {source_name:?} overflowed at point {}/{}",
                            point_index + 1,
                            point_count
                        ),
                    ));
                }
                source = replace_source_dc_card(&source, source_name, &scaled.to_string())
                    .ok_or_else(|| {
                        PreparationError::new(
                            PreparationStage::Netlist,
                            format!(
                                "Run Set could not locate the authored card for supply source {source_name:?}"
                            ),
                        )
                    })?;
            }
        }
        _ => {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Run Set point {}/{} must provide supply and nominal voltage together",
                    point_index + 1,
                    point_count
                ),
            ));
        }
    }

    Ok(splice_before_terminal_end_card(
        &source,
        &format!(".OPTIONS TEMP={}", environment.temperature_celsius),
    ))
}

fn scalable_supply_dc_value(spec: &rspice_core::netlist::SourceSpec) -> Option<f64> {
    match spec {
        rspice_core::netlist::SourceSpec::Dc(value) => Some(*value),
        rspice_core::netlist::SourceSpec::DcAc { dc_value, .. } => Some(*dc_value),
        _ => None,
    }
}

fn materialize_parameter_overrides(
    executable_netlist: &str,
    overrides: &[(String, String)],
) -> String {
    if overrides.is_empty() {
        return executable_netlist.to_owned();
    }
    let mut block = String::new();
    for (name, value) in overrides {
        block.push_str(".param ");
        block.push_str(name);
        block.push('=');
        block.push_str(value);
        block.push('\n');
    }
    splice_before_terminal_end_card(executable_netlist, block.trim_end())
}

fn materialize_source_overrides(
    executable_netlist: &str,
    overrides: &[(String, String)],
) -> Result<String, PreparationError> {
    let mut source = executable_netlist.to_owned();
    for (name, value) in overrides {
        let parsed = rspice_core::Netlist::parse(&source).map_err(|error| {
            PreparationError::new(
                PreparationStage::Netlist,
                format!("Cannot validate Run Set source binding {name:?}: {error}"),
            )
        })?;
        let element = parsed
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!("Run Set source binding {name:?} is absent from the executable deck"),
                )
            })?;
        let spec = match &element.kind {
            rspice_core::netlist::ElementKind::VoltageSource(spec)
            | rspice_core::netlist::ElementKind::CurrentSource(spec) => spec,
            _ => {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!("Run Set source binding {name:?} is not an independent source"),
                ));
            }
        };
        if !source_spec_has_explicit_dc(spec) {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Run Set source binding {name:?} has no explicit DC value that can be replaced"
                ),
            ));
        }
        source = replace_source_dc_card(&source, name, value).ok_or_else(|| {
            PreparationError::new(
                PreparationStage::Netlist,
                format!("Run Set could not locate the authored card for source {name:?}"),
            )
        })?;
    }
    Ok(source)
}

fn source_spec_has_explicit_dc(spec: &rspice_core::netlist::SourceSpec) -> bool {
    use rspice_core::netlist::SourceSpec;
    match spec {
        SourceSpec::Dc(_)
        | SourceSpec::DcAc { .. }
        | SourceSpec::DcTransient { .. }
        | SourceSpec::DcAcTransient { .. } => true,
        SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
            source_spec_has_explicit_dc(inner)
        }
        _ => false,
    }
}

fn replace_source_dc_card(source: &str, source_name: &str, value: &str) -> Option<String> {
    let mut replaced = false;
    let mut inside_subcircuit = false;
    let mut output = String::with_capacity(source.len() + value.len());
    for line in source.split_inclusive('\n') {
        let newline = line.ends_with('\n');
        let content = line.trim_end_matches(['\r', '\n']);
        let mut tokens = content.split_whitespace().collect::<Vec<_>>();
        let directive = tokens.first().copied().unwrap_or_default();
        if directive.eq_ignore_ascii_case(".subckt") {
            inside_subcircuit = true;
        }
        if !replaced
            && !inside_subcircuit
            && tokens
                .first()
                .is_some_and(|token| token.eq_ignore_ascii_case(source_name))
            && tokens.len() >= 4
        {
            let value_index = tokens
                .iter()
                .position(|token| token.eq_ignore_ascii_case("DC"))
                .and_then(|index| (index + 1 < tokens.len()).then_some(index + 1))
                .unwrap_or(3);
            tokens[value_index] = value;
            output.push_str(&tokens.join(" "));
            replaced = true;
        } else {
            output.push_str(content);
        }
        if directive.eq_ignore_ascii_case(".ends") {
            inside_subcircuit = false;
        }
        if newline {
            output.push('\n');
        }
    }
    replaced.then_some(output)
}

fn operating_point_config(spec: &AnalysisSpec) -> Option<crate::simulation::dialog::OpConfig> {
    use crate::simulation::dialog::OpConfig;

    match spec {
        AnalysisSpec::LegacyDcOp => Some(OpConfig::default()),
        AnalysisSpec::DcOp {
            temperature_mode,
            temperature_celsius,
            initial_guess,
            node_initialization,
            homotopy,
            annotation,
            device_detail,
            save_device_op,
            accuracy,
            selected_devices,
            previous_state,
            violation_devices,
            violation_source_content_digest,
            run_point,
        } => Some(OpConfig {
            temperature_mode: *temperature_mode,
            temperature_celsius: *temperature_celsius,
            initial_guess: *initial_guess,
            node_initialization: *node_initialization,
            homotopy: *homotopy,
            annotation: *annotation,
            device_detail: *device_detail,
            save_device_op: *save_device_op,
            accuracy: *accuracy,
            selected_devices: selected_devices.clone(),
            previous_state: previous_state.clone(),
            violation_devices: violation_devices.clone(),
            violation_source_content_digest: *violation_source_content_digest,
            run_point: run_point.clone(),
        }),
        _ => None,
    }
}

fn operating_point_spec(config: &crate::simulation::dialog::OpConfig) -> AnalysisSpec {
    AnalysisSpec::DcOp {
        temperature_mode: config.temperature_mode,
        temperature_celsius: config.temperature_celsius,
        initial_guess: config.initial_guess,
        node_initialization: config.node_initialization,
        homotopy: config.homotopy,
        annotation: config.annotation,
        device_detail: config.device_detail,
        save_device_op: config.save_device_op,
        accuracy: config.accuracy,
        selected_devices: config.selected_devices.clone(),
        previous_state: config.previous_state.clone(),
        violation_devices: config.violation_devices.clone(),
        violation_source_content_digest: config.violation_source_content_digest,
        run_point: config.run_point.clone(),
    }
}

fn validate_retained_operating_point_contract(
    task: &QueuedAnalysis,
    source_digest: ContentDigest,
    executable_source: &str,
) -> Result<(), String> {
    let spec_config = operating_point_config(&task.spec);
    let Some(spec_config) = spec_config else {
        return Ok(());
    };
    spec_config.validate_for_execution()?;
    let effective_source_digest = super::canonical::operating_point_effective_source_digest(
        executable_source,
        spec_config.run_point.clone(),
    );
    if let Some(previous) = spec_config.previous_state.as_ref()
        && previous.source_content_digest != effective_source_digest
    {
        return Err(
            "the retained previous solution belongs to different executable source content"
                .to_owned(),
        );
    }
    if let Some(soa_source_digest) = spec_config.violation_source_content_digest
        && soa_source_digest != source_digest
    {
        return Err(
            "the retained SOA violation evidence belongs to different executable source content"
                .to_owned(),
        );
    }
    if let Some(crate::simulation::AnalysisConfig::DcOp(config)) = task.config.as_ref()
        && config != &spec_config
    {
        return Err(
            "the operating-point spec and engine configuration carry different contracts"
                .to_owned(),
        );
    }
    Ok(())
}

fn dependency_cycle(
    tasks: &[PreparedTask],
    positions: &HashMap<AnalysisInstanceId, usize>,
) -> Option<Vec<AnalysisInstanceId>> {
    let mut indegree = tasks
        .iter()
        .map(|task| task.dependencies.len())
        .collect::<Vec<_>>();
    let mut dependents = vec![Vec::new(); tasks.len()];
    for (dependent_index, task) in tasks.iter().enumerate() {
        for dependency in &task.dependencies {
            dependents[positions[dependency]].push(dependent_index);
        }
    }
    let mut ready = VecDeque::new();
    for (index, degree) in indegree.iter().enumerate() {
        if *degree == 0 {
            ready.push_back(index);
        }
    }
    let mut visited = 0usize;
    while let Some(index) = ready.pop_front() {
        visited += 1;
        for dependent in &dependents[index] {
            indegree[*dependent] -= 1;
            if indegree[*dependent] == 0 {
                ready.push_back(*dependent);
            }
        }
    }
    if visited == tasks.len() {
        return None;
    }
    Some(
        tasks
            .iter()
            .enumerate()
            .filter_map(|(index, task)| (indegree[index] != 0).then_some(task.instance_id))
            .collect(),
    )
}

fn analysis_id_metadata_digest(instance_id: AnalysisInstanceId) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.analysis-instance-metadata-id/v1");
    writer.uuid(instance_id.as_uuid());
    writer.finish()
}

fn derive_pvt_points(
    tasks: &[PreparedTask],
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
    run_set: Option<&PreparedRunSet>,
) -> Result<Vec<PreparedPvtPoint>, PreparationError> {
    if !reference_temperature_celsius.is_finite() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Reference PVT temperature must be finite",
        ));
    }

    let max_pvt_points = rspice_core::ResourceLimits::default().max_batch_runs;
    if let Some(run_set) = run_set {
        let validation = crate::simulation::run_set::validate(&run_set.state, tasks.len());
        if let Some(error) = validation.errors.first() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!("Run Set is not executable: {}", error.message),
            ));
        }
        let points = crate::simulation::run_set::resolve(&run_set.state).ok_or_else(|| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Run Set could not be expanded into exact execution points",
            )
        })?;
        ensure_pvt_point_capacity(0, points.len(), max_pvt_points)?;

        let mut prepared = Vec::with_capacity(points.len());
        for point in points {
            let mut process = reference_process;
            let mut voltage = None;
            let mut supply_source_names = Vec::new();
            let mut temperature_celsius = reference_temperature_celsius;
            let mut parameter_overrides = Vec::new();
            let mut source_overrides = Vec::new();
            for (dimension, value) in point.coordinates {
                let canonical = value.canonical.ok_or_else(|| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!("Run Set value {:?} is not executable", value.lexical),
                    )
                })?;
                match dimension.kind {
                    RunSetDimensionKind::ProcessSection => {
                        process = match canonical as usize {
                            0 => ProcessCorner::TT,
                            1 => ProcessCorner::SS,
                            2 => ProcessCorner::FF,
                            3 => ProcessCorner::SF,
                            4 => ProcessCorner::FS,
                            _ => {
                                return Err(PreparationError::new(
                                    PreparationStage::AnalysisPlan,
                                    format!(
                                        "Run Set process section {:?} is not supported",
                                        value.lexical
                                    ),
                                ));
                            }
                        };
                    }
                    RunSetDimensionKind::Supply => {
                        voltage = Some(canonical);
                        supply_source_names =
                            crate::simulation::run_set::parse_supply_source_authority(
                                &dimension.source,
                            )
                            .map_err(|error| {
                                PreparationError::new(
                                    PreparationStage::AnalysisPlan,
                                    format!("Run Set supply binding is not executable: {error}"),
                                )
                            })?;
                    }
                    RunSetDimensionKind::Temperature => temperature_celsius = canonical,
                    RunSetDimensionKind::Parameter => {
                        let name = crate::simulation::run_set::parse_parameter_source_authority(
                            &dimension.source,
                        )
                        .map_err(|error| {
                            PreparationError::new(
                                PreparationStage::AnalysisPlan,
                                format!("Run Set parameter binding is not executable: {error}"),
                            )
                        })?;
                        parameter_overrides.push((name, value.lexical.trim().to_owned()));
                    }
                    RunSetDimensionKind::Source => {
                        let name = crate::simulation::run_set::parse_source_value_authority(
                            &dimension.source,
                        )
                        .map_err(|error| {
                            PreparationError::new(
                                PreparationStage::AnalysisPlan,
                                format!("Run Set source binding is not executable: {error}"),
                            )
                        })?;
                        source_overrides.push((name, value.lexical.trim().to_owned()));
                    }
                    RunSetDimensionKind::Model
                    | RunSetDimensionKind::Frequency
                    | RunSetDimensionKind::Time
                    | RunSetDimensionKind::Seed
                    | RunSetDimensionKind::Sample
                    | RunSetDimensionKind::AnalysisSelection
                    | RunSetDimensionKind::DigitalConfiguration
                    | RunSetDimensionKind::ExternalDataset => {
                        return Err(PreparationError::new(
                            PreparationStage::AnalysisPlan,
                            format!(
                                "Run Set {} binding reached preparation without an executor",
                                dimension.kind.as_str()
                            ),
                        ));
                    }
                }
            }
            prepared.push(PreparedPvtPoint {
                process,
                voltage,
                supply_source_names,
                temperature_celsius,
                parameter_overrides,
                source_overrides,
                corner_contract: Some(run_set.corner_contract.clone()),
            });
        }
        if prepared.is_empty() {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Run Set did not produce any execution points",
            ));
        }
        return Ok(prepared);
    }

    let mut expanded = Vec::new();
    let mut found_explicit_axis = false;
    for prepared in tasks {
        let task = prepared.queued_analysis();
        match &task.spec {
            AnalysisSpec::Corner => {
                found_explicit_axis = true;
                let default_corner;
                let corner = match task.spec_options.corner.as_ref() {
                    Some(corner) => corner,
                    None => {
                        default_corner =
                            crate::services::simulation_runner::CornerRunConfig::default();
                        &default_corner
                    }
                };
                corner.validate().map_err(|error| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!("Corner PVT contract is invalid: {error}"),
                    )
                })?;
                let expanded_corner =
                    crate::services::simulation_runner::expand_corner_pvt_points(corner);
                let points = expanded_corner.map_err(|error| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!("Corner PVT expansion failed: {error}"),
                    )
                })?;
                ensure_pvt_point_capacity(expanded.len(), points.len(), max_pvt_points)?;
                expanded.extend(points.into_iter().map(
                    |(process, voltage, temperature_celsius)| PreparedPvtPoint {
                        process: process_from_corner_runner(process),
                        voltage: Some(voltage),
                        supply_source_names: corner.supply_source_names.clone(),
                        temperature_celsius,
                        parameter_overrides: Vec::new(),
                        source_overrides: Vec::new(),
                        corner_contract: Some(corner.clone()),
                    },
                ));
            }
            AnalysisSpec::Parametric => {
                if let Some(temp) = task.spec_options.temp.as_ref() {
                    found_explicit_axis = true;
                    if temp.temperatures_c.is_empty()
                        || temp
                            .temperatures_c
                            .iter()
                            .any(|temperature| !temperature.is_finite())
                    {
                        return Err(PreparationError::new(
                            PreparationStage::AnalysisPlan,
                            "Temperature PVT expansion requires at least one finite temperature",
                        ));
                    }
                    ensure_pvt_point_capacity(
                        expanded.len(),
                        temp.temperatures_c.len(),
                        max_pvt_points,
                    )?;
                    expanded.extend(temp.temperatures_c.iter().copied().map(
                        |temperature_celsius| PreparedPvtPoint {
                            process: reference_process,
                            voltage: None,
                            supply_source_names: Vec::new(),
                            temperature_celsius,
                            parameter_overrides: Vec::new(),
                            source_overrides: Vec::new(),
                            corner_contract: None,
                        },
                    ));
                }
            }
            _ => {}
        }
    }

    // Ordinary analyses do not create an axis point. The reference point is
    // the fallback only when the run has no explicit corner/temperature axis;
    // otherwise it would silently add a nominal solve the authored run set
    // never requested.
    if !found_explicit_axis {
        ensure_pvt_point_capacity(expanded.len(), 1, max_pvt_points)?;
        expanded.push(reference_pvt_point(
            reference_process,
            reference_temperature_celsius,
        ));
    }

    let mut seen = HashSet::new();
    expanded.retain(|point| {
        seen.insert((
            process_tag(point.process),
            point.voltage.map(f64::to_bits),
            point.temperature_celsius.to_bits(),
            point.corner_contract.as_ref().map(corner_contract_digest),
        ))
    });
    if expanded.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Prepared run did not produce any PVT execution points",
        ));
    }
    Ok(expanded)
}

fn ensure_pvt_point_capacity(
    retained: usize,
    additional: usize,
    limit: usize,
) -> Result<(), PreparationError> {
    let requested = retained.saturating_add(additional);
    if requested <= limit {
        Ok(())
    } else {
        Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "PVT plan requests {requested} runs, exceeding the configured batch limit of {limit}"
            ),
        ))
    }
}

fn reference_pvt_point(process: ProcessCorner, temperature_celsius: f64) -> PreparedPvtPoint {
    PreparedPvtPoint {
        process,
        voltage: None,
        supply_source_names: Vec::new(),
        temperature_celsius,
        parameter_overrides: Vec::new(),
        source_overrides: Vec::new(),
        corner_contract: None,
    }
}

fn corner_contract_digest(
    contract: &crate::services::simulation_runner::CornerRunConfig,
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.op-pvt-corner-contract/v1");
    writer.option(contract.nominal_voltage.as_ref(), |writer, voltage| {
        writer.f64(*voltage);
    });
    writer.sequence(contract.supply_source_names.len());
    for source in &contract.supply_source_names {
        writer.string(source);
    }
    writer.sequence(contract.model_bindings.len());
    for binding in &contract.model_bindings {
        writer.u8(match binding.process {
            crate::services::simulation_runner::CornerProcess::TT => 0,
            crate::services::simulation_runner::CornerProcess::SS => 1,
            crate::services::simulation_runner::CornerProcess::FF => 2,
            crate::services::simulation_runner::CornerProcess::SF => 3,
            crate::services::simulation_runner::CornerProcess::FS => 4,
        });
        writer.string(&binding.source_label);
        writer.option(binding.section.as_ref(), |writer, section| {
            writer.string(section);
        });
        writer.string(&binding.materialized_model_cards);
    }
    writer.finish()
}

fn process_from_corner_runner(
    process: crate::services::simulation_runner::CornerProcess,
) -> ProcessCorner {
    use crate::services::simulation_runner::CornerProcess;
    match process {
        CornerProcess::TT => ProcessCorner::TT,
        CornerProcess::SS => ProcessCorner::SS,
        CornerProcess::FF => ProcessCorner::FF,
        CornerProcess::SF => ProcessCorner::SF,
        CornerProcess::FS => ProcessCorner::FS,
    }
}

fn process_to_corner_runner(
    process: ProcessCorner,
) -> crate::services::simulation_runner::CornerProcess {
    use crate::services::simulation_runner::CornerProcess;
    match process {
        ProcessCorner::TT => CornerProcess::TT,
        ProcessCorner::SS => CornerProcess::SS,
        ProcessCorner::FF => CornerProcess::FF,
        ProcessCorner::SF => CornerProcess::SF,
        ProcessCorner::FS => CornerProcess::FS,
    }
}

fn snapshot_digest(
    intent: SimulationRunIntent,
    simulation_plan_id: Option<SimulationPlanId>,
    project_revision: u64,
    topology_revision: u64,
    source_digest: ContentDigest,
    pvt_points: &[PreparedPvtPoint],
    tasks: &[PreparedTask],
    specifications: &[PreparedSpecification],
    specification_policy: &PreparedSpecificationPolicy,
    save_policy: SavePolicy,
    model_identities: &[ModelSourceIdentity],
    target: &ExecutionTargetCapabilities,
    receipt: RunSourceReceipt,
    touchstone_export: &TouchstoneExportPolicy,
    executable_netlist: &str,
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.prepared-run-snapshot/v9");
    writer.domain("run-intent");
    writer.u8(match intent {
        SimulationRunIntent::SimulateRunSet => 0,
        SimulationRunIntent::ManualDeck => 1,
    });
    writer.domain("owning-simulation-plan");
    writer.option(simulation_plan_id.as_ref(), |writer, plan_id| {
        writer.uuid(plan_id.as_uuid());
    });
    writer.domain("project-revision");
    writer.u64(project_revision);
    writer.domain("topology-revision");
    writer.u64(topology_revision);
    writer.domain("source-content-identity");
    writer.digest(source_digest);

    writer.domain("pvt-points");
    writer.sequence(pvt_points.len());
    for point in pvt_points {
        writer.u8(process_tag(point.process));
        writer.option(point.voltage.as_ref(), |writer, voltage| {
            writer.f64(*voltage);
        });
        writer.f64(point.temperature_celsius);
        writer.sequence(point.parameter_overrides.len());
        for (name, value) in &point.parameter_overrides {
            writer.string(name);
            writer.string(value);
        }
        writer.sequence(point.source_overrides.len());
        for (name, value) in &point.source_overrides {
            writer.string(name);
            writer.string(value);
        }
    }

    writer.domain("ordered-prepared-task-graph");
    writer.sequence(tasks.len());
    for task in tasks {
        writer.uuid(task.instance_id.as_uuid());
        writer.uuid(task.authored_instance_id.as_uuid());
        writer.u64(task.source_revision.get());
        writer.digest(task.config_digest);
        writer.option(task.touchstone_export.as_ref(), |writer, policy| {
            policy.encode(writer);
        });
        writer.option(
            task.executable_netlist_override.as_ref(),
            |writer, source| writer.string(source),
        );
        writer.string(&task.label);
        writer.sequence(task.dependencies.len());
        for dependency in &task.dependencies {
            writer.uuid(dependency.as_uuid());
        }
        writer.domain("typed-dependency-bindings");
        writer.sequence(task.dependency_bindings.len());
        for binding in &task.dependency_bindings {
            binding.encode(&mut writer);
        }
        writer.domain("saved-output-contracts");
        writer.sequence(task.saved_output_contracts.len());
        for contract in &task.saved_output_contracts {
            writer.uuid(contract.output_id().as_uuid());
            writer.u64(contract.output_revision().get());
            writer.uuid(contract.analysis_id().as_uuid());
            writer.digest(contract.digest());
            writer.u8(output_kind_tag(contract.kind()));
            writer.string(contract.name());
            writer.string(contract.source_expression());
            writer.u8(policy_tag(contract.policy()));
            writer.u8(precision_tag(contract.precision()));
            writer.u8(streaming_tag(contract.streaming()));
            writer.option(contract.selection_grid().as_ref(), |writer, grid| {
                writer.f64(grid.start);
                writer.f64(grid.step);
                writer.f64(grid.stop);
            });
        }
    }

    writer.domain("frozen-specifications");
    writer.sequence(specifications.len());
    for specification in specifications {
        let entry = specification.entry();
        writer.string(&entry.measurement);
        writer.string(&entry.expression);
        writer.option(entry.min.as_ref(), |writer, value| writer.f64(*value));
        writer.option(entry.max.as_ref(), |writer, value| writer.f64(*value));
        writer.string(&entry.unit);
        match &entry.scope {
            crate::state::SpecPointScope::AllPoints => writer.u8(0),
            crate::state::SpecPointScope::Nominal => writer.u8(1),
            crate::state::SpecPointScope::SelectedCorners { corners } => {
                writer.u8(2);
                writer.sequence(corners.len());
                for corner in corners {
                    writer.string(corner);
                }
            }
        }
        writer.option(specification.definition(), |writer, definition| {
            writer.uuid(definition.id.as_uuid());
            writer.string(&definition.requirement_key);
            writer.string(&definition.requirement_name);
            writer.string(&definition.measurement);
            writer.string(&definition.expression);
            writer.option(
                definition.producing_analysis.as_ref(),
                |writer, analysis_id| {
                    writer.uuid(analysis_id.as_uuid());
                },
            );
            match &definition.comparison {
                crate::state::SpecificationComparison::Tracked => writer.u8(0),
                crate::state::SpecificationComparison::Minimum { limit } => {
                    writer.u8(1);
                    writer.f64(*limit);
                }
                crate::state::SpecificationComparison::Maximum { limit } => {
                    writer.u8(2);
                    writer.f64(*limit);
                }
                crate::state::SpecificationComparison::Range { minimum, maximum } => {
                    writer.u8(3);
                    writer.f64(*minimum);
                    writer.f64(*maximum);
                }
                crate::state::SpecificationComparison::EqualWithin { target, tolerance } => {
                    writer.u8(4);
                    writer.f64(*target);
                    writer.f64(*tolerance);
                }
            }
            writer.option(definition.guard_band.as_ref(), |writer, value| {
                writer.f64(*value);
            });
            writer.u8(match definition.role {
                crate::state::SpecificationRole::Blocking => 0,
                crate::state::SpecificationRole::Review => 1,
                crate::state::SpecificationRole::Informational => 2,
            });
            writer.option(definition.source.as_ref(), |writer, source| {
                writer.string(&source.logical_path);
                writer.u64(source.row);
                writer.string(&source.imported_revision);
                writer.digest(source.source_digest);
            });
            writer.option(definition.waiver.as_ref(), |writer, waiver| {
                writer.string(&waiver.reference);
                writer.string(&waiver.owner);
                writer.string(&waiver.rationale);
            });
        });
    }

    writer.domain("specification-policy");
    let policy = specification_policy.policy();
    writer.u8(match policy.nominal_failure {
        crate::state::NominalFailurePolicy::Block => 0,
        crate::state::NominalFailurePolicy::RecordDisposition => 1,
    });
    match policy.monte_carlo {
        crate::state::MonteCarloSpecificationGate::NotGated => writer.u8(0),
        crate::state::MonteCarloSpecificationGate::YieldAtLeast { percent } => {
            writer.u8(1);
            writer.f64(percent);
        }
    }
    writer.u8(match policy.regression {
        crate::state::RegressionSpecificationPolicy::LimitAndWaveform => 0,
        crate::state::RegressionSpecificationPolicy::LimitOnly => 1,
    });
    writer.u8(match policy.missing_measurement {
        crate::state::MissingMeasurementPolicy::FailClosed => 0,
        crate::state::MissingMeasurementPolicy::ReportUnmapped => 1,
    });

    writer.domain("save-policy");
    save_policy.encode(&mut writer);
    writer.domain("model-and-library-identities");
    writer.sequence(model_identities.len());
    for identity in model_identities {
        writer.string(&identity.label);
        writer.digest(identity.digest);
    }
    target.encode(&mut writer);
    receipt.encode(&mut writer);
    touchstone_export.encode(&mut writer);
    writer.domain("deterministic-executable-netlist");
    writer.string(executable_netlist);
    writer.finish()
}

fn process_tag(process: ProcessCorner) -> u8 {
    match process {
        ProcessCorner::TT => 0,
        ProcessCorner::SS => 1,
        ProcessCorner::FF => 2,
        ProcessCorner::SF => 3,
        ProcessCorner::FS => 4,
    }
}

#[cfg(test)]
mod tests;
