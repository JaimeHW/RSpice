use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::product::ContentDigest;
use crate::simulation::controller::QueuedAnalysis;
use crate::simulation::dialog::corner::ProcessCorner;
use crate::simulation::multi_run::AnalysisSpec;
use crate::state::{Point, SimulationRunIntent};

use super::canonical::{
    CanonicalWriter, analysis_config_digest, analysis_instance_id, analysis_kind_tag,
    content_digest,
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

#[derive(Debug, Clone)]
struct PreparedAnalysisIdentity {
    instance_id: ContentDigest,
    config_digest: ContentDigest,
    label: String,
}

#[derive(Debug, Clone)]
struct PreparedPvtPoint {
    process: ProcessCorner,
    voltage: Option<f64>,
    temperature_celsius: f64,
}

#[derive(Debug, Clone)]
struct PreparedTaskNode {
    instance_id: ContentDigest,
    config_digest: ContentDigest,
    dependencies: Vec<ContentDigest>,
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
    tasks: VecDeque<AuthorizedTaskDispatch>,
    executable_netlist: Arc<str>,
    advisories: Vec<String>,
    manual_source: Option<String>,
    cross_probe: Option<CrossProbeSnapshot>,
    touchstone_export: TouchstoneExportPolicy,
}

/// Opaque task/netlist pair accepted by the runner's sole production start.
/// Its constructor and fields are private to the execution boundary.
#[derive(Debug)]
pub(in crate::simulation) struct AuthorizedTaskDispatch {
    task: QueuedAnalysis,
    executable_netlist: Arc<str>,
}

impl AuthorizedRunDispatch {
    pub(in crate::simulation) const fn digest(&self) -> ContentDigest {
        self.digest
    }

    pub(in crate::simulation) const fn intent(&self) -> SimulationRunIntent {
        self.intent
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

    pub(in crate::simulation) fn take_touchstone_export_policy(
        &mut self,
    ) -> TouchstoneExportPolicy {
        std::mem::replace(
            &mut self.touchstone_export,
            TouchstoneExportPolicy::disabled(),
        )
    }
}

impl AuthorizedTaskDispatch {
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
    pub(in crate::simulation) project_revision: u64,
    pub(in crate::simulation) topology_revision: u64,
    pub(in crate::simulation) source_digest: ContentDigest,
    pub(in crate::simulation) reference_process: ProcessCorner,
    pub(in crate::simulation) reference_temperature_celsius: f64,
    pub(in crate::simulation) analysis_instance_ids: Vec<ContentDigest>,
    pub(in crate::simulation) analysis_config_digests: Vec<ContentDigest>,
    pub(in crate::simulation) analysis_labels: Vec<String>,
    pub(in crate::simulation) tasks: Vec<QueuedAnalysis>,
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
    project_revision: u64,
    topology_revision: u64,
    source_digest: ContentDigest,
    pvt_points: Vec<PreparedPvtPoint>,
    analyses: Vec<PreparedAnalysisIdentity>,
    task_graph: Vec<PreparedTaskNode>,
    tasks: Vec<QueuedAnalysis>,
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
            .field("project_revision", &self.project_revision)
            .field("topology_revision", &self.topology_revision)
            .field("source_digest", &self.source_digest)
            .field("analyses", &self.analyses.len())
            .field("tasks", &self.tasks.len())
            .field("executable_netlist_bytes", &self.executable_netlist.len())
            .finish_non_exhaustive()
    }
}

impl PreparedRunSnapshot {
    pub(in crate::simulation) fn new(mut parts: SnapshotParts) -> Result<Self, PreparationError> {
        let count = parts.tasks.len();
        if count == 0
            || parts.analysis_instance_ids.len() != count
            || parts.analysis_config_digests.len() != count
            || parts.analysis_labels.len() != count
        {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Prepared analysis identities, configurations, labels, and tasks must be non-empty and have identical lengths",
            ));
        }
        if parts.executable_netlist.trim().is_empty() {
            return Err(PreparationError::new(
                PreparationStage::Netlist,
                "Prepared executable netlist is empty",
            ));
        }

        let mut kind_occurrences = [0usize; 26];
        for (index, task) in parts.tasks.iter().enumerate() {
            let kind = usize::from(analysis_kind_tag(&task.spec));
            let occurrence = kind_occurrences[kind];
            kind_occurrences[kind] += 1;
            let expected_instance = analysis_instance_id(parts.intent, &task.spec, occurrence);
            let expected_config = analysis_config_digest(
                &task.analysis_line,
                &task.spec,
                task.config.as_ref(),
                &task.spec_options,
            );
            if parts.analysis_instance_ids[index] != expected_instance
                || parts.analysis_config_digests[index] != expected_config
            {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "Prepared task {} identity/configuration digest does not authenticate its actual dispatch payload",
                        index + 1
                    ),
                ));
            }
        }

        // Model identities describe a set. Sort by canonical label/digest so
        // discovery or map insertion order cannot perturb snapshot identity;
        // executable model precedence remains bound by the netlist bytes.
        parts.model_identities.sort_unstable();
        parts.model_identities.dedup();

        let analyses = parts
            .analysis_instance_ids
            .iter()
            .copied()
            .zip(parts.analysis_config_digests.iter().copied())
            .zip(parts.analysis_labels.iter().cloned())
            .map(
                |((instance_id, config_digest), label)| PreparedAnalysisIdentity {
                    instance_id,
                    config_digest,
                    label,
                },
            )
            .collect::<Vec<_>>();
        let task_graph = analyses
            .iter()
            .enumerate()
            .map(|(index, analysis)| PreparedTaskNode {
                instance_id: analysis.instance_id,
                config_digest: analysis.config_digest,
                dependencies: index
                    .checked_sub(1)
                    .map(|previous| vec![analyses[previous].instance_id])
                    .unwrap_or_default(),
            })
            .collect::<Vec<_>>();
        let pvt_points = derive_pvt_points(
            &parts.tasks,
            parts.reference_process,
            parts.reference_temperature_celsius,
        )?;

        let digest = snapshot_digest(
            parts.intent,
            parts.project_revision,
            parts.topology_revision,
            parts.source_digest,
            &pvt_points,
            &analyses,
            &task_graph,
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
            project_revision: parts.project_revision,
            topology_revision: parts.topology_revision,
            source_digest: parts.source_digest,
            pvt_points,
            analyses,
            task_graph,
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
                .analyses
                .iter()
                .map(|analysis| analysis.instance_id)
                .collect(),
            task_count: self.task_graph.len(),
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
        let tasks = self
            .tasks
            .into_iter()
            .map(|task| AuthorizedTaskDispatch {
                task,
                executable_netlist: Arc::clone(&executable_netlist),
            })
            .collect();
        Ok(AuthorizedRunDispatch {
            digest: self.digest,
            intent: self.intent,
            tasks,
            executable_netlist,
            advisories: self.advisories,
            manual_source: self.manual_source,
            cross_probe: self.cross_probe,
            touchstone_export: self.touchstone_export,
        })
    }
}

fn derive_pvt_points(
    tasks: &[QueuedAnalysis],
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
    for task in tasks {
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
    project_revision: u64,
    topology_revision: u64,
    source_digest: ContentDigest,
    pvt_points: &[PreparedPvtPoint],
    analyses: &[PreparedAnalysisIdentity],
    task_graph: &[PreparedTaskNode],
    save_policy: SavePolicy,
    model_identities: &[ModelSourceIdentity],
    target: &ExecutionTargetCapabilities,
    receipt: RunSourceReceipt,
    touchstone_export: &TouchstoneExportPolicy,
    executable_netlist: &str,
) -> ContentDigest {
    let mut writer = CanonicalWriter::new("rspice.prepared-run-snapshot/v2");
    writer.domain("run-intent");
    writer.u8(match intent {
        SimulationRunIntent::SimulateRunSet => 0,
        SimulationRunIntent::ManualDeck => 1,
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

    writer.domain("analysis-instances-and-configurations");
    writer.sequence(analyses.len());
    for analysis in analyses {
        writer.digest(analysis.instance_id);
        writer.digest(analysis.config_digest);
        writer.string(&analysis.label);
    }

    writer.domain("ordered-task-graph");
    writer.sequence(task_graph.len());
    for task in task_graph {
        writer.digest(task.instance_id);
        writer.digest(task.config_digest);
        writer.sequence(task.dependencies.len());
        for dependency in &task.dependencies {
            writer.digest(*dependency);
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

    fn authenticate_tasks(parts: &mut SnapshotParts) {
        let mut kind_occurrences = [0usize; 26];
        parts.analysis_instance_ids.clear();
        parts.analysis_config_digests.clear();
        for task in &parts.tasks {
            let kind = usize::from(analysis_kind_tag(&task.spec));
            let occurrence = kind_occurrences[kind];
            kind_occurrences[kind] += 1;
            parts.analysis_instance_ids.push(analysis_instance_id(
                parts.intent,
                &task.spec,
                occurrence,
            ));
            parts.analysis_config_digests.push(analysis_config_digest(
                &task.analysis_line,
                &task.spec,
                task.config.as_ref(),
                &task.spec_options,
            ));
        }
    }

    fn parts() -> SnapshotParts {
        let mut parts = SnapshotParts {
            intent: SimulationRunIntent::SimulateRunSet,
            project_revision: 3,
            topology_revision: 4,
            source_digest: ContentDigest::from_bytes([1; 32]),
            reference_process: ProcessCorner::TT,
            reference_temperature_celsius: 27.0,
            analysis_instance_ids: Vec::new(),
            analysis_config_digests: Vec::new(),
            analysis_labels: vec!["DC Operating Point".to_owned()],
            tasks: vec![task()],
            executable_netlist: "deck\n.op\n.end\n".to_owned(),
            save_policy: SavePolicy::RetainEngineProducedResults,
            model_identities: Vec::new(),
            target: ExecutionTargetCapabilities::current(),
            receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([4; 32])),
            advisories: Vec::new(),
            manual_source: None,
            cross_probe: None,
            touchstone_export: TouchstoneExportPolicy::disabled(),
        };
        authenticate_tasks(&mut parts);
        parts
    }

    #[test]
    fn task_order_changes_snapshot_identity() {
        let mut two = parts();
        two.analysis_labels.push("Transient".to_owned());
        two.tasks.push(transient_task());
        authenticate_tasks(&mut two);
        let ordered = PreparedRunSnapshot::new(two).expect("ordered snapshot");

        let mut reversed = parts();
        reversed.analysis_labels = vec!["Transient".to_owned(), "DC Operating Point".to_owned()];
        reversed.tasks = vec![transient_task(), task()];
        authenticate_tasks(&mut reversed);
        let reversed = PreparedRunSnapshot::new(reversed).expect("reversed snapshot");

        assert_ne!(ordered.digest(), reversed.digest());
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
        matrix.tasks = vec![corner_task(
            vec![CornerProcess::TT, CornerProcess::FF],
            vec![0.9, 1.1],
            vec![-40.0, 125.0],
            true,
        )];
        matrix.analysis_labels = vec!["Corner".to_owned()];
        authenticate_tasks(&mut matrix);

        let snapshot = PreparedRunSnapshot::new(matrix).expect("full corner matrix snapshot");
        assert_eq!(snapshot.pvt_points.len(), 8);
        assert_eq!(snapshot.task_graph.len(), 1);
        assert_eq!(snapshot.metadata().pvt_point_count, 8);
    }

    #[test]
    fn pvt_metadata_uses_the_runners_diagonal_corner_expansion_order() {
        use crate::services::simulation_runner::CornerProcess;
        let mut diagonal = parts();
        diagonal.tasks = vec![corner_task(
            vec![CornerProcess::SS, CornerProcess::FF],
            vec![0.9, 1.0, 1.1],
            vec![-40.0, 125.0],
            false,
        )];
        diagonal.analysis_labels = vec!["Corner".to_owned()];
        authenticate_tasks(&mut diagonal);

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
        swept.tasks = vec![task(), temperature_task(vec![27.0, 85.0, 85.0])];
        swept.analysis_labels = vec!["DC Operating Point".to_owned(), "Temperature".to_owned()];
        authenticate_tasks(&mut swept);

        let snapshot = PreparedRunSnapshot::new(swept).expect("temperature snapshot");
        assert_eq!(snapshot.pvt_points.len(), 2);
        assert_eq!(snapshot.pvt_points[0].temperature_celsius, 27.0);
        assert_eq!(snapshot.pvt_points[1].temperature_celsius, 85.0);
        assert_eq!(snapshot.task_graph.len(), 2);
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
        let (_, netlist) = tasks
            .pop_front()
            .expect("authorized task")
            .into_runner_parts();
        assert_eq!(&*netlist, "deck\n.op\n.end\n");
        assert!(tasks.is_empty());
    }

    #[test]
    fn snapshot_rejects_parallel_digest_that_does_not_match_actual_task() {
        let mut mismatched = parts();
        mismatched.analysis_config_digests[0] = ContentDigest::from_bytes([0xff; 32]);
        let error = PreparedRunSnapshot::new(mismatched)
            .expect_err("parallel digest cannot authenticate a different task");
        assert_eq!(error.stage(), PreparationStage::AnalysisPlan);
        assert!(error.message().contains("actual dispatch payload"));
    }
}
