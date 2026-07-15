use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
use crate::simulation::controller::QueuedAnalysis;
use crate::simulation::dialog::corner::ProcessCorner;
use crate::simulation::multi_run::AnalysisSpec;
use crate::state::{
    AnalysisResultSourceDomain, Point, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, SimulationRunIntent,
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
    source_revision: ObjectRevision,
    dependencies: Vec<AnalysisInstanceId>,
    label: String,
    config_digest: ContentDigest,
    task: QueuedAnalysis,
    /// Explicit per-instance override. Stable plan tasks always set this for
    /// S-parameter analyses; manual-deck tasks inherit the run-level policy.
    touchstone_export: Option<TouchstoneExportPolicy>,
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
            source_revision,
            dependencies,
            label: label.into(),
            config_digest,
            task,
            touchstone_export: None,
        }
    }

    pub(in crate::simulation) fn with_touchstone_export_policy(
        mut self,
        policy: TouchstoneExportPolicy,
    ) -> Self {
        self.touchstone_export = Some(policy);
        self
    }

    #[cfg(test)]
    pub(in crate::simulation) const fn instance_id(&self) -> AnalysisInstanceId {
        self.instance_id
    }

    #[cfg(test)]
    pub(in crate::simulation) const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    #[cfg(test)]
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
}

#[derive(Debug, Clone, Default)]
pub(in crate::simulation) struct CrossProbeSnapshot {
    point_to_net: HashMap<Point, String>,
    nets: HashMap<String, Vec<Point>>,
    net_segments: HashMap<String, Vec<(Point, Point)>>,
}

impl CrossProbeSnapshot {
    pub(in crate::simulation) fn new(
        point_to_net: HashMap<Point, String>,
        nets: HashMap<String, Vec<Point>>,
        net_segments: HashMap<String, Vec<(Point, Point)>>,
    ) -> Self {
        Self {
            point_to_net,
            nets,
            net_segments,
        }
    }

    pub(in crate::simulation) fn apply(self, state: &mut crate::common::app::AppState) {
        state.schematic.net_mapping = self.point_to_net.clone();
        state
            .simulation
            .cross_probe
            .update(self.point_to_net, self.nets, self.net_segments);
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
    pub(crate) pvt_point_count: usize,
    pub(crate) target: &'static str,
    pub(crate) save_policy: &'static str,
    pub(crate) model_identity_count: usize,
    pub(crate) advisories: Vec<String>,
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
    source_revision: ObjectRevision,
    dependencies: Vec<AnalysisInstanceId>,
    label: String,
    config_digest: ContentDigest,
    task: QueuedAnalysis,
    executable_netlist: Arc<str>,
    touchstone_export: TouchstoneExportPolicy,
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

    pub(in crate::simulation) const fn source_revision(&self) -> ObjectRevision {
        self.source_revision
    }

    pub(in crate::simulation) fn dependencies(&self) -> &[AnalysisInstanceId] {
        &self.dependencies
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

    pub(in crate::simulation) fn into_runner_parts(self) -> (QueuedAnalysis, Arc<str>) {
        (self.task, self.executable_netlist)
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
    pub(in crate::simulation) target: ExecutionTargetCapabilities,
    pub(in crate::simulation) receipt: RunSourceReceipt,
    pub(in crate::simulation) advisories: Vec<String>,
    pub(in crate::simulation) manual_source: Option<String>,
    pub(in crate::simulation) cross_probe: Option<CrossProbeSnapshot>,
    pub(in crate::simulation) touchstone_export: TouchstoneExportPolicy,
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
    target: ExecutionTargetCapabilities,
    receipt: RunSourceReceipt,
    advisories: Vec<String>,
    manual_source: Option<String>,
    cross_probe: Option<CrossProbeSnapshot>,
    touchstone_export: TouchstoneExportPolicy,
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
            if task.config_digest != task.payload_digest() {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Prepared task {} configuration digest does not authenticate its actual dispatch payload",
                        index + 1
                    ),
                ));
            }
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
        }

        // Model identities describe a set. Sort by canonical label/digest so
        // discovery or map insertion order cannot perturb snapshot identity;
        // executable model precedence remains bound by the netlist bytes.
        parts.model_identities.sort_unstable();
        parts.model_identities.dedup();

        let pvt_points = derive_pvt_points(
            &parts.tasks,
            parts.reference_process,
            parts.reference_temperature_celsius,
        )?;

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
            target: parts.target,
            receipt: parts.receipt,
            advisories: parts.advisories,
            manual_source: parts.manual_source,
            cross_probe: parts.cross_probe,
            touchstone_export: parts.touchstone_export,
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
            pvt_point_count: self.pvt_points.len(),
            target: self.target.label(),
            save_policy: self.save_policy.label(),
            model_identity_count: self.model_identities.len(),
            advisories: self.advisories.clone(),
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
                    source_revision: prepared.source_revision,
                    dependencies: prepared.dependencies,
                    label: prepared.label,
                    config_digest: prepared.config_digest,
                    task: prepared.task,
                    executable_netlist: Arc::clone(&executable_netlist),
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

    let mut expanded = Vec::new();
    for prepared in tasks {
        let task = prepared.queued_analysis();
        match &task.spec {
            AnalysisSpec::Corner => {
                let default_corner;
                let corner = match task.spec_options.corner.as_ref() {
                    Some(corner) => corner,
                    None => {
                        default_corner =
                            crate::services::simulation_runner::CornerRunConfig::default();
                        &default_corner
                    }
                };
                if corner.process_corners.is_empty()
                    || corner.voltages.is_empty()
                    || corner.temperatures_c.is_empty()
                    || corner
                        .voltages
                        .iter()
                        .any(|voltage| !voltage.is_finite() || *voltage <= 0.0)
                    || corner
                        .temperatures_c
                        .iter()
                        .any(|temperature| !temperature.is_finite())
                {
                    return Err(PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        "Corner PVT expansion requires non-empty process, positive finite voltage, and finite temperature inputs",
                    ));
                }
                expanded.extend(
                    crate::services::simulation_runner::expand_corner_pvt_points(corner)
                        .into_iter()
                        .map(|(process, voltage, temperature_celsius)| PreparedPvtPoint {
                            process: process_from_corner_runner(process),
                            voltage: Some(voltage),
                            temperature_celsius,
                        }),
                );
            }
            AnalysisSpec::Parametric => {
                if let Some(temp) = task.spec_options.temp.as_ref() {
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
                    expanded.extend(temp.temperatures_c.iter().copied().map(
                        |temperature_celsius| PreparedPvtPoint {
                            process: reference_process,
                            voltage: None,
                            temperature_celsius,
                        },
                    ));
                } else {
                    expanded.push(reference_pvt_point(
                        reference_process,
                        reference_temperature_celsius,
                    ));
                }
            }
            _ => expanded.push(reference_pvt_point(
                reference_process,
                reference_temperature_celsius,
            )),
        }
    }

    let mut seen = HashSet::new();
    expanded.retain(|point| {
        seen.insert((
            process_tag(point.process),
            point.voltage.map(f64::to_bits),
            point.temperature_celsius.to_bits(),
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

fn reference_pvt_point(process: ProcessCorner, temperature_celsius: f64) -> PreparedPvtPoint {
    PreparedPvtPoint {
        process,
        voltage: None,
        temperature_celsius,
    }
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
    let mut writer = CanonicalWriter::new("rspice.prepared-run-snapshot/v5");
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
        writer.u64(task.source_revision.get());
        writer.digest(task.config_digest);
        writer.option(task.touchstone_export.as_ref(), |writer, policy| {
            policy.encode(writer);
        });
        writer.string(&task.label);
        writer.sequence(task.dependencies.len());
        for dependency in &task.dependencies {
            writer.uuid(dependency.as_uuid());
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
            spec: AnalysisSpec::DcOp,
            config: Some(AnalysisConfig::DcOp),
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
            target: ExecutionTargetCapabilities::current(),
            receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([4; 32])),
            advisories: Vec::new(),
            manual_source: None,
            cross_probe: None,
            touchstone_export: TouchstoneExportPolicy::disabled(),
        }
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
        matrix.tasks = vec![prepared(
            "corner",
            "Corner",
            corner_task(
                vec![CornerProcess::TT, CornerProcess::FF],
                vec![0.9, 1.1],
                vec![-40.0, 125.0],
                true,
            ),
        )];

        let snapshot = PreparedRunSnapshot::new(matrix).expect("full corner matrix snapshot");
        assert_eq!(snapshot.pvt_points.len(), 8);
        assert_eq!(snapshot.tasks.len(), 1);
        assert_eq!(snapshot.metadata().pvt_point_count, 8);
    }

    #[test]
    fn pvt_metadata_uses_the_runners_diagonal_corner_expansion_order() {
        use crate::services::simulation_runner::CornerProcess;
        let mut diagonal = parts();
        diagonal.tasks = vec![prepared(
            "corner",
            "Corner",
            corner_task(
                vec![CornerProcess::SS, CornerProcess::FF],
                vec![0.9, 1.0, 1.1],
                vec![-40.0, 125.0],
                false,
            ),
        )];

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
        assert_eq!(snapshot.tasks.len(), 2);
        assert_eq!(snapshot.metadata().pvt_point_count, 2);
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
        let (_, netlist) = authorized.into_runner_parts();
        assert_eq!(&*netlist, "deck\n.op\n.end\n");
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
}
