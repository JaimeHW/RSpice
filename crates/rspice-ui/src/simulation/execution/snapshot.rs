use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
use crate::simulation::controller::QueuedAnalysis;
use crate::simulation::dialog::corner::ProcessCorner;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::output_contract::{
    PreparedSavedOutput, output_kind_tag, policy_tag, precision_tag, streaming_tag,
};
use crate::state::{
    AnalysisResultSourceDomain, Point, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, SimulationRunIntent,
};

use super::artifact::{
    ExecutionArtifactEnvelope, ExecutionArtifactError, ExecutionArtifactKind,
    PreparedDependencyBinding, ResolvedExecutionDependencies,
    validate_prepared_dependency_contract,
};
use super::canonical::{
    CanonicalWriter, analysis_config_digest, analysis_kind_tag, content_digest,
};
use super::permit::ConsumedExecutionPermit;

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
}

impl SavePolicy {
    pub(in crate::simulation) const fn label(self) -> &'static str {
        match self {
            Self::RetainEngineProducedResults => "Retain engine-produced results",
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::RetainEngineProducedResults => 0,
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

    fn payload_digest(&self) -> ContentDigest {
        analysis_config_digest(
            &self.task.analysis_line,
            &self.task.spec,
            self.task.config.as_ref(),
            &self.task.spec_options,
        )
    }
}

#[derive(Debug, Clone)]
struct PreparedPvtPoint {
    process: ProcessCorner,
    voltage: Option<f64>,
    temperature_celsius: f64,
    /// Exact corner contract that owns process-model and nominal-voltage
    /// semantics for this point. Temperature-only axes do not carry one.
    corner_contract: Option<crate::services::simulation_runner::CornerRunConfig>,
}

#[derive(Debug, Clone)]
pub(in crate::simulation) struct CrossProbeSnapshot {
    source_reference: crate::state::CellViewRef,
    point_to_net: HashMap<Point, String>,
    nets: HashMap<String, Vec<Point>>,
    net_segments: HashMap<String, Vec<(Point, Point)>>,
    topology_version: u64,
}

impl CrossProbeSnapshot {
    pub(in crate::simulation) fn new(
        source_reference: crate::state::CellViewRef,
        point_to_net: HashMap<Point, String>,
        nets: HashMap<String, Vec<Point>>,
        net_segments: HashMap<String, Vec<(Point, Point)>>,
        topology_version: u64,
    ) -> Self {
        Self {
            source_reference,
            point_to_net,
            nets,
            net_segments,
            topology_version,
        }
    }

    pub(in crate::simulation) fn apply(self, state: &mut crate::common::app::AppState) {
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
    project_veriloga_runtimes: crate::workbench::code_workspace::PreparedVerilogARuntimeSet,
    touchstone_export: TouchstoneExportPolicy,
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
        PreparedRunReceipt::new(
            source_domain,
            self.simulation_plan_id,
            project_revision,
            self.digest,
            self.source_digest,
            self.source_receipt.durable(),
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
        crate::workbench::code_workspace::PreparedVerilogARuntimeSet,
        ResolvedExecutionDependencies,
    ) {
        (
            self.dispatch.task,
            self.dispatch.executable_netlist,
            self.dispatch.project_veriloga_runtimes,
            self.dependencies,
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
    pub(in crate::simulation) tasks: Vec<PreparedTask>,
    pub(in crate::simulation) executable_netlist: String,
    pub(in crate::simulation) save_policy: SavePolicy,
    pub(in crate::simulation) model_identities: Vec<ModelSourceIdentity>,
    pub(in crate::simulation) project_veriloga_runtimes:
        crate::workbench::code_workspace::PreparedVerilogARuntimeSet,
    pub(in crate::simulation) target: ExecutionTargetCapabilities,
    pub(in crate::simulation) receipt: RunSourceReceipt,
    pub(in crate::simulation) advisories: Vec<String>,
    pub(in crate::simulation) manual_source: Option<String>,
    pub(in crate::simulation) cross_probe: Option<CrossProbeSnapshot>,
    pub(in crate::simulation) touchstone_export: TouchstoneExportPolicy,
    pub(in crate::simulation) sealed_source_dependencies:
        Vec<rspice_core::netlist::ResolvedIncludeDependency>,
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
    project_veriloga_runtimes: crate::workbench::code_workspace::PreparedVerilogARuntimeSet,
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
        )?;
        parts.tasks =
            expand_operating_point_tasks(parts.tasks, &pvt_points, &parts.executable_netlist)?;

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
                AnalysisSpec::Pss {
                    method: crate::simulation::multi_run::PssMethod::Shooting,
                    ..
                } => Some(ExecutionArtifactKind::DcOperatingPointSeed),
                AnalysisSpec::Pac
                | AnalysisSpec::Pxf
                | AnalysisSpec::Pnoise
                | AnalysisSpec::Pstb => Some(ExecutionArtifactKind::PeriodicState),
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
                validate_prepared_dependency_contract(&task.task.spec, &producer.task.spec)
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
                    format!("Project Verilog-A runtime set is invalid: {error}"),
                )
            })?;
        let parsed_netlist =
            rspice_core::Netlist::parse(&parts.executable_netlist).map_err(|error| {
                PreparationError::new(
                    PreparationStage::Netlist,
                    format!("Prepared executable netlist is invalid: {error}"),
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
                "Every executable Verilog-A directive must match exactly one sealed project runtime",
            ));
        }
        for runtime in parts.project_veriloga_runtimes.iter() {
            runtime.validate().map_err(|error| {
                PreparationError::new(
                    PreparationStage::ModelBindings,
                    format!("Project Verilog-A runtime is invalid: {error}"),
                )
            })?;
            let directive = crate::workbench::code_workspace::project_veriloga_directive(
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
                        "Prepared project Verilog-A runtime '{}' must be referenced exactly once by the executable netlist (found {directive_count})",
                        runtime.netlist_alias()
                    ),
                ));
            }
            parts.model_identities.push(ModelSourceIdentity::new(
                format!("project-veriloga:{}", runtime.source_key()),
                runtime.artifact_digest(),
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
        crate::workbench::netlist_document::source_content_digest(exact_source)
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

fn expand_operating_point_tasks(
    tasks: Vec<PreparedTask>,
    pvt_points: &[PreparedPvtPoint],
    executable_netlist: &str,
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
            };
            point_task.executable_netlist_override = source_override;
            point_task.task.spec = operating_point_spec(&config);
            point_task.task.config = Some(AnalysisConfig::DcOp(config));
            if pvt_points.len() > 1 {
                point_task.label = format!(
                    "{original_label} \u{00b7} point {}/{} \u{00b7} {} \u{00b0}C",
                    index + 1,
                    pvt_points.len(),
                    point.temperature_celsius
                );
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
                "Operating-point PVT voltage is missing its authenticated corner contract",
            ));
        }
        return Ok((None, None));
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
                "Failed to materialize {} operating-point process corner: {error}",
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
                    "Operating-point PVT voltage axis requires a non-zero independent DC supply or an explicit nominal voltage",
                )
            })?,
        })
    } else {
        None
    };

    let source_override = (source != executable_netlist).then_some(source);
    Ok((source_override, nominal_supply_voltage))
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
            run_point: *run_point,
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
        run_point: config.run_point,
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
        spec_config.run_point,
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
    if let Some(crate::simulation::AnalysisConfig::DcOp(config)) = task.config.as_ref() {
        if config != &spec_config {
            return Err(
                "the operating-point spec and engine configuration carry different contracts"
                    .to_owned(),
            );
        }
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
) -> Result<Vec<PreparedPvtPoint>, PreparationError> {
    if !reference_temperature_celsius.is_finite() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Reference PVT temperature must be finite",
        ));
    }

    let max_pvt_points = rspice_core::ResourceLimits::default().max_batch_runs;
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
                        temperature_celsius,
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
                            temperature_celsius,
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
        temperature_celsius,
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

#[allow(clippy::too_many_arguments)]
fn snapshot_digest(
    intent: SimulationRunIntent,
    simulation_plan_id: Option<SimulationPlanId>,
    project_revision: u64,
    topology_revision: u64,
    source_digest: ContentDigest,
    pvt_points: &[PreparedPvtPoint],
    tasks: &[PreparedTask],
    save_policy: SavePolicy,
    model_identities: &[ModelSourceIdentity],
    target: &ExecutionTargetCapabilities,
    receipt: RunSourceReceipt,
    touchstone_export: &TouchstoneExportPolicy,
    executable_netlist: &str,
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.prepared-run-snapshot/v7");
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

    writer.domain("save-policy");
    writer.u8(save_policy.tag());
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
mod tests {
    use super::*;
    use crate::simulation::AnalysisConfig;
    use crate::simulation::multi_run::AnalysisSpec;
    use crate::simulation::runner::SpecExecutionOptions;

    fn task() -> QueuedAnalysis {
        QueuedAnalysis {
            spec: AnalysisSpec::dc_op(),
            config: Some(AnalysisConfig::dc_op()),
            spec_options: SpecExecutionOptions::default(),
            analysis_line: ".op".to_owned(),
        }
    }

    fn configured_op_task(config: crate::simulation::dialog::OpConfig) -> QueuedAnalysis {
        QueuedAnalysis {
            spec: operating_point_spec(&config),
            config: Some(AnalysisConfig::DcOp(config)),
            spec_options: SpecExecutionOptions::default(),
            analysis_line: ".op".to_owned(),
        }
    }

    fn transient_task() -> QueuedAnalysis {
        let config = crate::simulation::config::TransientAnalysisConfig {
            stop_time: 1.0e-6,
            step_time: 1.0e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        };
        QueuedAnalysis {
            spec: AnalysisSpec::Transient {
                stop_time: config.stop_time,
                step_time: config.step_time,
                start_time: config.start_time,
                max_timestep: config.max_timestep,
                uic: config.uic,
            },
            config: Some(AnalysisConfig::Transient(config)),
            spec_options: SpecExecutionOptions::default(),
            analysis_line: ".tran 1n 1u".to_owned(),
        }
    }

    fn temperature_task(temperatures_c: Vec<f64>) -> QueuedAnalysis {
        QueuedAnalysis {
            spec: AnalysisSpec::Parametric,
            config: None,
            spec_options: SpecExecutionOptions {
                temp: Some(crate::services::simulation_runner::TempRunConfig {
                    temperatures_c,
                    base_mode: crate::services::simulation_runner::CornerBaseMode::Op,
                }),
                ..SpecExecutionOptions::default()
            },
            analysis_line: ".temp".to_owned(),
        }
    }

    fn corner_task(
        process_corners: Vec<crate::services::simulation_runner::CornerProcess>,
        voltages: Vec<f64>,
        temperatures_c: Vec<f64>,
        full_matrix: bool,
    ) -> QueuedAnalysis {
        QueuedAnalysis {
            spec: AnalysisSpec::Corner,
            config: None,
            spec_options: SpecExecutionOptions {
                corner: Some(crate::services::simulation_runner::CornerRunConfig {
                    process_corners,
                    voltages,
                    temperatures_c,
                    full_matrix,
                    nominal_voltage: Some(1.0),
                    base_mode: crate::services::simulation_runner::CornerBaseMode::Op,
                    model_bindings: Vec::new(),
                }),
                ..SpecExecutionOptions::default()
            },
            analysis_line: ".corner".to_owned(),
        }
    }

    fn corner_binding(
        process: crate::services::simulation_runner::CornerProcess,
        source_label: &str,
        saturation_current: &str,
    ) -> crate::services::simulation_runner::CornerModelBinding {
        crate::services::simulation_runner::CornerModelBinding {
            process,
            source_label: source_label.to_owned(),
            section: Some(process.as_keyword().to_owned()),
            materialized_model_cards: format!(".model DPROCESS D (IS={saturation_current})"),
        }
    }

    fn instance_id(name: &str) -> AnalysisInstanceId {
        const TEST_NAMESPACE: uuid::Uuid =
            uuid::Uuid::from_u128(0x3ce2_b258_0c75_55f0_96d4_8fc1_cadf_1384);
        AnalysisInstanceId::from_namespace(TEST_NAMESPACE, name.as_bytes())
    }

    fn prepared(name: &str, label: &str, task: QueuedAnalysis) -> PreparedTask {
        PreparedTask::new(
            instance_id(name),
            ObjectRevision::INITIAL,
            Vec::new(),
            label,
            task,
        )
    }

    fn prepared_with(
        name: &str,
        revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        label: &str,
        task: QueuedAnalysis,
    ) -> PreparedTask {
        PreparedTask::new(instance_id(name), revision, dependencies, label, task)
    }

    fn parts() -> SnapshotParts {
        const TEST_NAMESPACE: uuid::Uuid =
            uuid::Uuid::from_u128(0xe6bc_c27a_6103_5327_b2ec_c759_b58a_8598);
        SnapshotParts {
            intent: SimulationRunIntent::SimulateRunSet,
            simulation_plan_id: Some(SimulationPlanId::from_namespace(
                TEST_NAMESPACE,
                b"snapshot-test-plan",
            )),
            project_revision: 3,
            topology_revision: 4,
            source_digest: ContentDigest::from_bytes([1; 32]),
            reference_process: ProcessCorner::TT,
            reference_temperature_celsius: 27.0,
            tasks: vec![prepared("op", "DC Operating Point", task())],
            executable_netlist: "deck\n.op\n.end\n".to_owned(),
            save_policy: SavePolicy::RetainEngineProducedResults,
            model_identities: Vec::new(),
            project_veriloga_runtimes: Default::default(),
            target: ExecutionTargetCapabilities::current(),
            receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([4; 32])),
            advisories: Vec::new(),
            manual_source: None,
            cross_probe: None,
            touchstone_export: TouchstoneExportPolicy::disabled(),
            sealed_source_dependencies: Vec::new(),
        }
    }

    fn project_runtime() -> crate::workbench::code_workspace::PreparedVerilogARuntime {
        let project_id = crate::product::ProjectId::new();
        let bundle = crate::state::ProjectSourceBundle::try_new(
            crate::state::ProjectSourceOwner::code_workspace(
                crate::state::ProjectSourceLanguage::VerilogA,
            ),
            crate::state::ProjectSourceLanguage::VerilogA,
            "file_name_differs.va",
            "module snapshot_owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
            [],
            [],
        )
        .unwrap();
        let receipt = crate::workbench::code_workspace::compile_project_bundle_receipt(
            project_id,
            &bundle,
            Some("snapshot_owned"),
        )
        .unwrap();
        crate::workbench::code_workspace::PreparedVerilogARuntime::try_from_current_bundle_receipt(
            project_id, &bundle, &receipt,
        )
        .unwrap()
    }

    #[test]
    fn snapshot_requires_the_exact_aliased_project_runtime_directive() {
        let runtime = project_runtime();
        let mut missing = parts();
        missing.project_veriloga_runtimes =
            crate::workbench::code_workspace::PreparedVerilogARuntimeSet::try_new(vec![
                runtime.clone(),
            ])
            .unwrap();
        assert!(matches!(
            PreparedRunSnapshot::new(missing),
            Err(PreparationError {
                stage: PreparationStage::ModelBindings,
                ..
            })
        ));

        let directive = crate::workbench::code_workspace::project_veriloga_directive(
            runtime.source_key(),
            runtime.netlist_alias(),
        );
        let mut suffixed = parts();
        suffixed.executable_netlist = format!("deck\n{directive} unexpected\n.op\n.end\n");
        suffixed.project_veriloga_runtimes =
            crate::workbench::code_workspace::PreparedVerilogARuntimeSet::try_new(vec![
                runtime.clone(),
            ])
            .unwrap();
        assert!(PreparedRunSnapshot::new(suffixed).is_err());

        let mut exact = parts();
        exact.executable_netlist = format!("deck\n{directive}\n.op\n.end\n");
        exact.project_veriloga_runtimes =
            crate::workbench::code_workspace::PreparedVerilogARuntimeSet::try_new(vec![runtime])
                .unwrap();
        assert!(PreparedRunSnapshot::new(exact).is_ok());
    }

    #[test]
    fn snapshot_rejects_unsealed_or_duplicate_veriloga_directives() {
        let runtime = project_runtime();
        let directive = crate::workbench::code_workspace::project_veriloga_directive(
            runtime.source_key(),
            runtime.netlist_alias(),
        );
        let runtime_set =
            crate::workbench::code_workspace::PreparedVerilogARuntimeSet::try_new(vec![runtime])
                .unwrap();

        let mut unsealed = parts();
        unsealed.executable_netlist = format!(
            "deck\n{directive}\n.veriloga \"__rspice_project__/foreign.va\" FOREIGN\n.op\n.end\n"
        );
        unsealed.project_veriloga_runtimes = runtime_set.clone();
        assert!(matches!(
            PreparedRunSnapshot::new(unsealed),
            Err(PreparationError {
                stage: PreparationStage::ModelBindings,
                ..
            })
        ));

        let mut duplicated = parts();
        duplicated.executable_netlist = format!("deck\n{directive}\n{directive}\n.op\n.end\n");
        duplicated.project_veriloga_runtimes = runtime_set;
        assert!(matches!(
            PreparedRunSnapshot::new(duplicated),
            Err(PreparationError {
                stage: PreparationStage::ModelBindings,
                ..
            })
        ));
    }

    #[test]
    fn task_order_changes_snapshot_identity() {
        let mut two = parts();
        two.tasks
            .push(prepared("tran", "Transient", transient_task()));
        let ordered = PreparedRunSnapshot::new(two).expect("ordered snapshot");

        let mut reversed = parts();
        reversed.tasks = vec![
            prepared("tran", "Transient", transient_task()),
            prepared("op", "DC Operating Point", task()),
        ];
        let reversed = PreparedRunSnapshot::new(reversed).expect("reversed snapshot");

        assert_ne!(ordered.digest(), reversed.digest());
    }

    #[test]
    fn owning_plan_identity_is_authenticated_and_required_for_plan_dispatch() {
        const OTHER_PLAN_NAMESPACE: uuid::Uuid =
            uuid::Uuid::from_u128(0xb77a_31fd_d31e_54f4_9507_1b1a_26ce_53fa);
        let first = PreparedRunSnapshot::new(parts()).expect("first plan snapshot");
        let mut changed = parts();
        changed.simulation_plan_id = Some(SimulationPlanId::from_namespace(
            OTHER_PLAN_NAMESPACE,
            b"other-plan",
        ));
        let changed = PreparedRunSnapshot::new(changed).expect("changed plan snapshot");
        assert_ne!(first.digest(), changed.digest());

        let mut missing = parts();
        missing.simulation_plan_id = None;
        let error = PreparedRunSnapshot::new(missing)
            .expect_err("plan dispatch without an owning plan must fail closed");
        assert_eq!(error.stage(), PreparationStage::Authorization);
        assert!(error.message().contains("simulation plan identity"));
    }

    #[test]
    fn task_revision_and_exact_dependency_graph_change_snapshot_identity() {
        let mut independent = parts();
        independent
            .tasks
            .push(prepared("tran", "Transient", transient_task()));
        let independent = PreparedRunSnapshot::new(independent).expect("independent task snapshot");

        let mut dependent = parts();
        dependent.tasks.push(prepared_with(
            "tran",
            ObjectRevision::INITIAL,
            vec![instance_id("op")],
            "Transient",
            transient_task(),
        ));
        let dependent = PreparedRunSnapshot::new(dependent).expect("dependent task snapshot");
        assert_ne!(independent.digest(), dependent.digest());

        let base_revision = PreparedRunSnapshot::new(parts()).expect("base revision snapshot");
        let mut revised = parts();
        revised.tasks[0] = prepared_with(
            "op",
            ObjectRevision::new(2).expect("revision two"),
            Vec::new(),
            "DC Operating Point",
            task(),
        );
        let revised = PreparedRunSnapshot::new(revised).expect("revised task snapshot");
        assert_ne!(base_revision.digest(), revised.digest());
    }

    #[test]
    fn snapshot_rejects_mixed_task_source_revisions() {
        let mut mixed = parts();
        mixed.tasks.push(prepared_with(
            "tran",
            ObjectRevision::new(2).expect("revision two"),
            Vec::new(),
            "Transient",
            transient_task(),
        ));

        let error = PreparedRunSnapshot::new(mixed)
            .expect_err("one frozen run cannot mix source revisions");
        assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
        assert!(error.message().contains("cannot mix source revisions"));
    }

    #[test]
    fn snapshot_rejects_duplicate_ids_and_duplicate_dependency_edges() {
        let mut empty = parts();
        empty.tasks.clear();
        let error = PreparedRunSnapshot::new(empty).expect_err("empty graph must fail");
        assert!(error.message().contains("at least one analysis"));

        let mut duplicate_id = parts();
        duplicate_id
            .tasks
            .push(prepared("op", "Duplicate OP", task()));
        let error = PreparedRunSnapshot::new(duplicate_id).expect_err("duplicate ID must fail");
        assert!(error.message().contains("duplicate analysis instance"));

        let mut duplicate_edge = parts();
        duplicate_edge.tasks.push(prepared_with(
            "tran",
            ObjectRevision::INITIAL,
            vec![instance_id("op"), instance_id("op")],
            "Transient",
            transient_task(),
        ));
        let error = PreparedRunSnapshot::new(duplicate_edge).expect_err("duplicate edge must fail");
        assert!(error.message().contains("duplicate dependency"));
    }

    #[test]
    fn snapshot_rejects_self_dangling_later_and_cyclic_dependencies() {
        let mut self_edge = parts();
        self_edge.tasks[0].dependencies = vec![instance_id("op")];
        let error = PreparedRunSnapshot::new(self_edge).expect_err("self edge must fail");
        assert!(error.message().contains("depend on itself"));

        let mut dangling = parts();
        dangling.tasks[0].dependencies = vec![instance_id("missing")];
        let error = PreparedRunSnapshot::new(dangling).expect_err("dangling edge must fail");
        assert!(error.message().contains("missing dependency"));

        let mut later = parts();
        later.tasks = vec![
            prepared_with(
                "tran",
                ObjectRevision::INITIAL,
                vec![instance_id("op")],
                "Transient",
                transient_task(),
            ),
            prepared("op", "DC Operating Point", task()),
        ];
        let error = PreparedRunSnapshot::new(later).expect_err("later edge must fail");
        assert!(error.message().contains("must appear earlier"));

        let mut cycle = parts();
        cycle.tasks = vec![
            prepared_with(
                "op",
                ObjectRevision::INITIAL,
                vec![instance_id("tran")],
                "DC Operating Point",
                task(),
            ),
            prepared_with(
                "tran",
                ObjectRevision::INITIAL,
                vec![instance_id("op")],
                "Transient",
                transient_task(),
            ),
        ];
        let error = PreparedRunSnapshot::new(cycle).expect_err("cycle must fail");
        assert!(error.message().contains("dependency cycle"));
    }

    #[test]
    fn source_content_change_changes_snapshot_identity_without_revision_change() {
        let first = PreparedRunSnapshot::new(parts()).expect("first snapshot");
        let mut changed = parts();
        changed.executable_netlist = "deck\n.op\nR1 out 0 1k\n.end\n".to_owned();
        changed.source_digest = ContentDigest::from_bytes([9; 32]);
        let changed = PreparedRunSnapshot::new(changed).expect("changed snapshot");
        assert_ne!(first.digest(), changed.digest());
    }

    #[test]
    fn retained_op_state_must_match_the_prepared_executable_source() {
        use crate::simulation::dialog::{OpInitialGuess, OpNodeInitialization, OpPreviousState};

        let mut matching = parts();
        let previous = OpPreviousState {
            source_content_digest: super::super::canonical::operating_point_effective_source_digest(
                &matching.executable_netlist,
                crate::simulation::dialog::OpRunPointContext::default(),
            ),
            producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
            producer_result_digest: ContentDigest::from_bytes([3; 32]),
            node_names: vec!["out".to_owned()],
            branch_names: Vec::new(),
            solution: vec![1.25],
        };
        let mut config = crate::simulation::dialog::OpConfig::default();
        config.initial_guess = OpInitialGuess::PreviousConverged;
        config.node_initialization = OpNodeInitialization::IgnoreIcAndNodeset;
        config.previous_state = Some(previous.clone());
        let mut spec = AnalysisSpec::dc_op();
        let AnalysisSpec::DcOp {
            initial_guess,
            node_initialization,
            previous_state,
            ..
        } = &mut spec
        else {
            unreachable!("current OP constructor returns the configured variant");
        };
        *initial_guess = OpInitialGuess::PreviousConverged;
        *node_initialization = OpNodeInitialization::IgnoreIcAndNodeset;
        *previous_state = Some(previous);
        let retained_task = QueuedAnalysis {
            spec,
            config: Some(AnalysisConfig::DcOp(config)),
            spec_options: SpecExecutionOptions::default(),
            analysis_line: ".op".to_owned(),
        };

        matching.tasks = vec![prepared("op", "DC Operating Point", retained_task.clone())];
        PreparedRunSnapshot::new(matching).expect("matching source-bound state");

        let mut changed = parts();
        changed.executable_netlist = "deck\nR1 out 0 1k\n.op\n.end\n".to_owned();
        changed.source_digest = ContentDigest::from_bytes([9; 32]);
        changed.tasks = vec![prepared("op", "DC Operating Point", retained_task)];
        let error = PreparedRunSnapshot::new(changed)
            .expect_err("stale retained state must fail before dispatch");
        assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
        assert!(error.message().contains("different executable source"));
    }

    #[test]
    fn retained_soa_context_must_match_the_prepared_executable_source() {
        use crate::simulation::dialog::OpDeviceDetail;

        let soa_source = ContentDigest::from_bytes([1; 32]);
        let mut config = crate::simulation::dialog::OpConfig::default();
        config.device_detail = OpDeviceDetail::ViolationsOnly;
        config.violation_devices = vec!["M1".to_owned()];
        config.violation_source_content_digest = Some(soa_source);
        let mut spec = AnalysisSpec::dc_op();
        let AnalysisSpec::DcOp {
            device_detail,
            violation_devices,
            violation_source_content_digest,
            ..
        } = &mut spec
        else {
            unreachable!("current OP constructor returns the configured variant");
        };
        *device_detail = OpDeviceDetail::ViolationsOnly;
        *violation_devices = vec!["M1".to_owned()];
        *violation_source_content_digest = Some(soa_source);
        let retained_task = QueuedAnalysis {
            spec,
            config: Some(AnalysisConfig::DcOp(config)),
            spec_options: SpecExecutionOptions::default(),
            analysis_line: ".op".to_owned(),
        };

        let mut matching = parts();
        matching.tasks = vec![prepared("op", "DC Operating Point", retained_task.clone())];
        PreparedRunSnapshot::new(matching).expect("matching source-bound SOA context");

        let mut changed = parts();
        changed.source_digest = ContentDigest::from_bytes([9; 32]);
        changed.tasks = vec![prepared("op", "DC Operating Point", retained_task)];
        let error = PreparedRunSnapshot::new(changed)
            .expect_err("stale retained SOA evidence must fail before dispatch");
        assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
        assert!(error.message().contains("SOA violation evidence"));
    }

    #[test]
    fn target_capability_change_changes_snapshot_identity() {
        let first = PreparedRunSnapshot::new(parts()).expect("first snapshot");
        let mut changed = parts();
        changed.target.cancellable = !changed.target.cancellable;
        let changed = PreparedRunSnapshot::new(changed).expect("changed target snapshot");
        assert_ne!(first.digest(), changed.digest());
    }

    #[test]
    fn every_execution_target_advertises_verified_cancellation() {
        let target = ExecutionTargetCapabilities::current();
        assert!(target.cancellable);
        assert_eq!(execution_target_supports_cancellation(), target.cancellable);
    }

    #[test]
    fn automatic_export_policy_is_authenticated_by_snapshot_identity() {
        let first = PreparedRunSnapshot::new(parts()).expect("disabled export snapshot");
        let mut changed = parts();
        changed.touchstone_export = TouchstoneExportPolicy::enabled(
            2,
            PathBuf::from("sealed-output"),
            OsString::from("amp"),
        )
        .expect("valid export policy");
        let changed = PreparedRunSnapshot::new(changed).expect("enabled export snapshot");

        assert_ne!(first.digest(), changed.digest());
        assert_eq!(
            changed
                .touchstone_export
                .output_path(12, 3, 4)
                .expect("enabled output path"),
            PathBuf::from("sealed-output").join("amp_run0012_sp03.s4p")
        );
    }

    #[test]
    fn automatic_export_identity_is_derived_from_the_exact_output_prefix() {
        let first = TouchstoneExportPolicy::enabled(
            2,
            PathBuf::from("sealed-output-a"),
            OsString::from("amp"),
        )
        .expect("first output policy");
        let second = TouchstoneExportPolicy::enabled(
            2,
            PathBuf::from("sealed-output-b"),
            OsString::from("amp"),
        )
        .expect("second output policy");
        assert_ne!(first, second);

        assert!(
            TouchstoneExportPolicy::enabled(
                2,
                PathBuf::from("sealed-output"),
                OsString::from("../redirect"),
            )
            .is_err(),
            "a stem must not be able to redirect the captured directory"
        );
    }

    #[test]
    fn model_identity_set_order_does_not_change_snapshot_identity() {
        let first_identity =
            ModelSourceIdentity::new("first", ContentDigest::from_bytes([0x11; 32]));
        let second_identity =
            ModelSourceIdentity::new("second", ContentDigest::from_bytes([0x22; 32]));
        let mut forward = parts();
        forward.model_identities = vec![first_identity.clone(), second_identity.clone()];
        let forward = PreparedRunSnapshot::new(forward).expect("forward snapshot");
        let mut reverse = parts();
        reverse.model_identities = vec![second_identity, first_identity];
        let reverse = PreparedRunSnapshot::new(reverse).expect("reverse snapshot");
        assert_eq!(forward.digest(), reverse.digest());
    }

    #[test]
    fn pvt_metadata_counts_the_exact_full_corner_matrix_inside_one_task() {
        use crate::services::simulation_runner::CornerProcess;
        let mut matrix = parts();
        let mut corner = corner_task(
            vec![CornerProcess::TT, CornerProcess::FF],
            vec![0.9, 1.1],
            vec![-40.0, 125.0],
            true,
        );
        corner
            .spec_options
            .corner
            .as_mut()
            .expect("corner config")
            .model_bindings = vec![corner_binding(CornerProcess::FF, "ff.lib", "1e-11")];
        matrix.tasks = vec![prepared("corner", "Corner", corner)];

        let snapshot = PreparedRunSnapshot::new(matrix).expect("full corner matrix snapshot");
        assert_eq!(snapshot.pvt_points.len(), 8);
        assert_eq!(snapshot.tasks.len(), 1);
        assert_eq!(snapshot.metadata().pvt_point_count, 8);
    }

    #[test]
    fn pvt_metadata_uses_the_runners_diagonal_corner_expansion_order() {
        use crate::services::simulation_runner::CornerProcess;
        let mut diagonal = parts();
        let mut corner = corner_task(
            vec![CornerProcess::SS, CornerProcess::FF],
            vec![0.9, 1.0, 1.1],
            vec![-40.0, 125.0],
            false,
        );
        corner
            .spec_options
            .corner
            .as_mut()
            .expect("corner config")
            .model_bindings = vec![
            corner_binding(CornerProcess::SS, "ss.lib", "1e-13"),
            corner_binding(CornerProcess::FF, "ff.lib", "1e-11"),
        ];
        diagonal.tasks = vec![prepared("corner", "Corner", corner)];

        let snapshot = PreparedRunSnapshot::new(diagonal).expect("diagonal corner snapshot");
        assert_eq!(snapshot.pvt_points.len(), 3);
        assert_eq!(snapshot.pvt_points[0].process, ProcessCorner::SS);
        assert_eq!(snapshot.pvt_points[0].voltage, Some(0.9));
        assert_eq!(snapshot.pvt_points[0].temperature_celsius, -40.0);
        assert_eq!(snapshot.pvt_points[2].process, ProcessCorner::SS);
        assert_eq!(snapshot.pvt_points[2].voltage, Some(1.1));
        assert_eq!(snapshot.pvt_points[2].temperature_celsius, -40.0);
    }

    #[test]
    fn pvt_point_set_is_ordered_and_deduplicated_across_tasks() {
        let mut swept = parts();
        swept.tasks = vec![
            prepared("op", "DC Operating Point", task()),
            prepared(
                "temperature",
                "Temperature",
                temperature_task(vec![27.0, 85.0, 85.0]),
            ),
        ];

        let snapshot = PreparedRunSnapshot::new(swept).expect("temperature snapshot");
        assert_eq!(snapshot.pvt_points.len(), 2);
        assert_eq!(snapshot.pvt_points[0].temperature_celsius, 27.0);
        assert_eq!(snapshot.pvt_points[1].temperature_celsius, 85.0);
        assert_eq!(snapshot.tasks.len(), 3);
        assert_eq!(snapshot.metadata().pvt_point_count, 2);
    }

    #[test]
    fn pvt_operating_point_dispatches_three_exact_temperatures_and_retains_only_final_report() {
        use crate::simulation::dialog::{OpConfig, OpSaveDevice};

        let mut swept = parts();
        swept.executable_netlist =
            "diode\nV1 in 0 0.7\nD1 in 0 DTEST\n.model DTEST D\n.op\n.end\n".to_owned();
        swept.tasks = vec![
            prepared(
                "op",
                "DC Operating Point",
                configured_op_task(OpConfig {
                    save_device_op: OpSaveDevice::FinalPointOnly,
                    ..OpConfig::default()
                }),
            ),
            prepared(
                "temperature",
                "Temperature",
                temperature_task(vec![-40.0, 27.0, 85.0]),
            ),
        ];

        let snapshot = PreparedRunSnapshot::new(swept).expect("three-point OP snapshot");
        let op_configs = snapshot
            .tasks
            .iter()
            .filter_map(|task| match task.queued_analysis().config.as_ref() {
                Some(AnalysisConfig::DcOp(config)) => Some(config.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(op_configs.len(), 3);
        assert_eq!(
            op_configs
                .iter()
                .map(|config| config.temperature_celsius)
                .collect::<Vec<_>>(),
            vec![-40.0, 27.0, 85.0]
        );

        for (index, config) in op_configs.into_iter().enumerate() {
            assert_eq!(config.run_point.index, index);
            assert_eq!(config.run_point.count, 3);
            let result = crate::simulation::EngineBridge::new()
                .run(
                    &AnalysisConfig::DcOp(config),
                    "diode\nV1 in 0 0.7\nD1 in 0 DTEST\n.model DTEST D\n.op\n.end\n",
                )
                .expect("exact PVT OP point solves");
            let crate::simulation::SimulationResult::DcOp(result) = result else {
                panic!("OP result")
            };
            assert_eq!(result.device_report.is_some(), index == 2);
        }
    }

    #[test]
    fn op_plus_ss_corner_has_no_unrequested_reference_point() {
        use crate::services::simulation_runner::CornerProcess;

        let mut ss_corner = corner_task(vec![CornerProcess::SS], vec![0.9], vec![125.0], true);
        ss_corner
            .spec_options
            .corner
            .as_mut()
            .expect("corner config")
            .model_bindings = vec![corner_binding(CornerProcess::SS, "ss.lib", "1e-13")];
        let mut ss_only = parts();
        ss_only.tasks.push(prepared("ss", "SS Corner", ss_corner));

        let snapshot = PreparedRunSnapshot::new(ss_only).expect("SS-only PVT snapshot");
        assert_eq!(snapshot.pvt_points.len(), 1);
        assert_eq!(snapshot.pvt_points[0].process, ProcessCorner::SS);
        assert_eq!(snapshot.pvt_points[0].temperature_celsius, 125.0);
        let config = snapshot.tasks[0]
            .queued_analysis()
            .config
            .as_ref()
            .and_then(|config| match config {
                AnalysisConfig::DcOp(config) => Some(config),
                _ => None,
            })
            .expect("expanded OP config");
        assert_eq!(config.run_point.process, ProcessCorner::SS);
        assert_eq!(config.run_point.count, 1);
    }

    #[test]
    fn process_and_voltage_axes_change_the_authorized_op_execution_contract() {
        use crate::services::simulation_runner::CornerProcess;

        let mut corner = corner_task(
            vec![CornerProcess::TT, CornerProcess::SS],
            vec![1.0, 1.2],
            vec![27.0],
            true,
        );
        corner
            .spec_options
            .corner
            .as_mut()
            .expect("corner config")
            .model_bindings = vec![
            corner_binding(CornerProcess::TT, "tt.lib", "1e-12"),
            corner_binding(CornerProcess::SS, "ss.lib", "1e-13"),
        ];
        let mut pvt = parts();
        pvt.executable_netlist = "pvt\nVDD in 0 1\nR1 in 0 1k\n.op\n.end\n".to_owned();
        pvt.tasks.push(prepared("corner", "Corner", corner));
        let snapshot = PreparedRunSnapshot::new(pvt).expect("PVT snapshot");
        assert_eq!(snapshot.pvt_points.len(), 4);
        assert_eq!(
            snapshot
                .tasks
                .iter()
                .filter(|task| {
                    matches!(task.task.config.as_ref(), Some(AnalysisConfig::DcOp(_)))
                })
                .count(),
            4
        );

        let digest = snapshot.digest();
        let permit = crate::simulation::execution::ExecutionPermitIssuer::default()
            .issue(digest)
            .expect("permit");
        let proof = permit.consume(digest, digest).expect("consume permit");
        let dispatch = snapshot.authorize_dispatch(proof).expect("authorize PVT");
        let mut seen_contracts = HashSet::new();
        let mut seen_process_sources = HashMap::new();
        let mut seen_supplies = HashSet::new();
        for task in dispatch.into_tasks().into_iter().take(4) {
            assert_eq!(task.authored_instance_id(), instance_id("op"));
            let config_digest = task.config_digest();
            let resolved = task
                .resolve_dependency_artifacts(&HashMap::new())
                .expect("OP has no typed dependencies");
            let (queued, source, _, _) = resolved.into_runner_parts();
            let Some(AnalysisConfig::DcOp(config)) = queued.config else {
                panic!("OP config")
            };
            seen_contracts.insert(config_digest);
            seen_supplies.insert(config.run_point.supply_voltage.unwrap().to_bits());
            seen_process_sources
                .entry(config.run_point.process)
                .or_insert_with(|| source.to_string());
            let result = crate::simulation::EngineBridge::new()
                .run(&AnalysisConfig::DcOp(config.clone()), &source)
                .expect("corner OP solve");
            let crate::simulation::SimulationResult::DcOp(result) = result else {
                panic!("OP result")
            };
            assert!(
                (result.voltage("in").expect("supply node")
                    - config.run_point.supply_voltage.unwrap())
                .abs()
                    <= 1.0e-10
            );
        }
        assert_eq!(seen_contracts.len(), 4);
        assert_eq!(seen_supplies.len(), 2);
        assert!(seen_process_sources[&ProcessCorner::TT].contains("tt.lib"));
        assert!(seen_process_sources[&ProcessCorner::SS].contains("ss.lib"));
        assert_ne!(
            seen_process_sources[&ProcessCorner::TT],
            seen_process_sources[&ProcessCorner::SS]
        );
    }

    #[test]
    fn downstream_ordering_dependency_targets_the_final_expanded_op_point() {
        let mut graph = parts();
        let original_op = instance_id("op");
        graph.tasks = vec![
            prepared("op", "DC Operating Point", task()),
            prepared(
                "temperature",
                "Temperature",
                temperature_task(vec![-40.0, 85.0]),
            ),
            prepared_with(
                "tran",
                ObjectRevision::INITIAL,
                vec![original_op],
                "Transient",
                transient_task(),
            ),
        ];

        let snapshot = PreparedRunSnapshot::new(graph).expect("expanded dependency graph");
        let op_tasks = snapshot
            .tasks
            .iter()
            .filter(|task| matches!(task.task.config.as_ref(), Some(AnalysisConfig::DcOp(_))))
            .collect::<Vec<_>>();
        assert_eq!(op_tasks.len(), 2);
        assert_eq!(op_tasks[1].dependencies(), &[op_tasks[0].instance_id()]);
        let transient = snapshot
            .tasks
            .iter()
            .find(|task| matches!(task.task.spec, AnalysisSpec::Transient { .. }))
            .expect("transient task");
        assert_eq!(transient.dependencies(), &[op_tasks[1].instance_id()]);
    }

    #[test]
    fn identical_coordinates_with_different_model_contracts_do_not_deduplicate() {
        use crate::services::simulation_runner::CornerProcess;

        let corner_with = |label: &str, saturation_current: &str| {
            let mut corner = corner_task(vec![CornerProcess::TT], vec![1.0], vec![27.0], true);
            corner
                .spec_options
                .corner
                .as_mut()
                .expect("corner config")
                .model_bindings =
                vec![corner_binding(CornerProcess::TT, label, saturation_current)];
            corner
        };
        let mut ambiguous_coordinates = parts();
        ambiguous_coordinates.tasks.push(prepared(
            "corner-a",
            "Corner A",
            corner_with("a.lib", "1e-12"),
        ));
        ambiguous_coordinates.tasks.push(prepared(
            "corner-b",
            "Corner B",
            corner_with("b.lib", "2e-12"),
        ));

        let snapshot = PreparedRunSnapshot::new(ambiguous_coordinates)
            .expect("distinct source contracts remain distinct points");
        assert_eq!(snapshot.pvt_points.len(), 2);
        let op_sources = snapshot
            .tasks
            .iter()
            .filter_map(|task| {
                matches!(task.task.config.as_ref(), Some(AnalysisConfig::DcOp(_)))
                    .then(|| task.executable_netlist_override.as_deref())
                    .flatten()
            })
            .collect::<Vec<_>>();
        assert_eq!(op_sources.len(), 2);
        assert_ne!(op_sources[0], op_sources[1]);
    }

    #[test]
    fn pvt_change_changes_snapshot_identity_without_revision_change() {
        let first = PreparedRunSnapshot::new(parts()).expect("reference snapshot");
        let mut changed = parts();
        changed.reference_temperature_celsius = 125.0;
        let changed = PreparedRunSnapshot::new(changed).expect("changed PVT snapshot");
        assert_ne!(first.digest(), changed.digest());
    }

    #[test]
    fn authorized_tasks_own_the_exact_snapshot_netlist_after_permit_consumption() {
        let snapshot = PreparedRunSnapshot::new(parts()).expect("prepared snapshot");
        let digest = snapshot.digest();
        let issuer = crate::simulation::execution::ExecutionPermitIssuer::default();
        let permit = issuer.issue(digest).expect("issue permit");
        let proof = permit
            .consume(digest, digest)
            .expect("consume exact permit");
        let dispatch = snapshot
            .authorize_dispatch(proof)
            .expect("authorize exact snapshot");

        assert_eq!(dispatch.executable_netlist(), "deck\n.op\n.end\n");
        assert_eq!(dispatch.task_count(), 1);
        let mut tasks = dispatch.into_tasks();
        let authorized = tasks.pop_front().expect("authorized task");
        assert_eq!(authorized.snapshot_digest(), digest);
        assert_eq!(authorized.instance_id(), instance_id("op"));
        assert_eq!(authorized.source_revision(), ObjectRevision::INITIAL);
        assert!(authorized.dependencies().is_empty());
        assert_eq!(authorized.label(), "DC Operating Point");
        assert_eq!(authorized.config_digest(), parts().tasks[0].config_digest());
        let resolved = authorized
            .resolve_dependency_artifacts(&HashMap::new())
            .expect("artifact-free task resolves");
        let (_, netlist, runtimes, dependencies) = resolved.into_runner_parts();
        assert_eq!(&*netlist, "deck\n.op\n.end\n");
        assert!(runtimes.is_empty());
        dependencies
            .validate_for_config()
            .expect("OP has no typed dependencies");
        assert!(tasks.is_empty());
    }

    #[test]
    fn authorization_preserves_exact_task_graph_and_source_revision() {
        let revision = ObjectRevision::new(7).expect("revision seven");
        let mut frozen = parts();
        frozen.tasks[0] = prepared_with("op", revision, Vec::new(), "DC Operating Point", task());
        frozen.tasks.push(prepared_with(
            "tran",
            revision,
            vec![instance_id("op")],
            "Transient",
            transient_task(),
        ));
        let snapshot = PreparedRunSnapshot::new(frozen).expect("prepared graph");
        let digest = snapshot.digest();
        let issuer = crate::simulation::execution::ExecutionPermitIssuer::default();
        let proof = issuer
            .issue(digest)
            .expect("issue permit")
            .consume(digest, digest)
            .expect("consume permit");
        let dispatch = snapshot.authorize_dispatch(proof).expect("authorize graph");
        let tasks = dispatch.tasks().collect::<Vec<_>>();

        assert_eq!(tasks[0].instance_id(), instance_id("op"));
        assert!(tasks[0].dependencies().is_empty());
        assert_eq!(tasks[1].instance_id(), instance_id("tran"));
        assert_eq!(tasks[1].dependencies(), &[instance_id("op")]);
        assert_eq!(tasks[1].source_revision(), revision);
        assert_eq!(tasks[1].snapshot_digest(), digest);
    }

    #[test]
    fn snapshot_rejects_digest_that_does_not_match_actual_task() {
        let mut mismatched = parts();
        mismatched.tasks[0].config_digest = ContentDigest::from_bytes([0xff; 32]);
        let error = PreparedRunSnapshot::new(mismatched)
            .expect_err("digest cannot authenticate a different task");
        assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
        assert!(error.message().contains("actual dispatch payload"));
    }

    #[test]
    fn pvt_plan_capacity_check_handles_overflow_and_limit() {
        let error = ensure_pvt_point_capacity(9, 2, 10).expect_err("eleven runs exceed ten");
        assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
        assert!(error.message().contains("11 runs"));

        let overflow = ensure_pvt_point_capacity(usize::MAX, 1, usize::MAX - 1)
            .expect_err("overflow must fail closed");
        assert!(overflow.message().contains(&usize::MAX.to_string()));
    }
}
