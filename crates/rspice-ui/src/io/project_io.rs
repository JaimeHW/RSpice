//! Project file I/O.
//!
//! `.rspiceproj` stores the product-level workspace: project identity,
//! libraries/cells/views, open documents, and schematic buffers. Individual
//! schematic export remains available through `.rsch`; project files are the
//! native professional workflow container.

use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::product::{DatasetId, RunId};
use crate::state::{
    AnalysisResult, AnalysisType, CellViewRef, DcOpResult, LibraryManager, NoiseContributorRow,
    NoiseSummary, OperatingPointValue, ProjectWorkspace, SimulationRun, SimulationState, ViewType,
    WaveformData,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ProjectVersion {
    pub const fn current() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }

    pub fn is_compatible(self) -> bool {
        self.major == Self::current().major
    }
}

impl Default for ProjectVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl std::fmt::Display for ProjectVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub version: ProjectVersion,
    pub workspace: ProjectWorkspace,
    pub libraries: LibraryManager,
    #[serde(default, skip_serializing_if = "ProjectSimulationResults::is_empty")]
    pub simulation_results: ProjectSimulationResults,
    #[serde(skip)]
    pub simulation_results_warning: Option<String>,
}

impl ProjectFile {
    pub fn new(workspace: ProjectWorkspace, libraries: LibraryManager) -> Self {
        Self {
            version: ProjectVersion::current(),
            workspace,
            libraries,
            simulation_results: ProjectSimulationResults::default(),
            simulation_results_warning: None,
        }
    }

    pub fn new_with_simulation_results(
        workspace: ProjectWorkspace,
        libraries: LibraryManager,
        simulation_results: ProjectSimulationResults,
    ) -> Self {
        Self {
            version: ProjectVersion::current(),
            workspace,
            libraries,
            simulation_results,
            simulation_results_warning: None,
        }
    }

    pub fn validate(&self) -> Result<(), ProjectIoError> {
        if !self.version.is_compatible() {
            return Err(ProjectIoError::IncompatibleVersion {
                file_version: self.version.to_string(),
                app_version: ProjectVersion::current().to_string(),
            });
        }
        self.workspace
            .project
            .validate()
            .map_err(|error| ProjectIoError::InvalidData(error.to_string()))?;
        self.validate_library_tree_keys()?;
        self.validate_workspace_references()?;
        Ok(())
    }

    fn validate_library_tree_keys(&self) -> Result<(), ProjectIoError> {
        for (library_key, library) in self.libraries.libraries_by_key() {
            if library.name != library_key {
                return Err(ProjectIoError::InvalidData(format!(
                    "library map key '{library_key}' does not match embedded library name '{}'",
                    library.name
                )));
            }
            for (cell_key, cell) in &library.cells {
                if cell.name != *cell_key {
                    return Err(ProjectIoError::InvalidData(format!(
                        "cell map key '{library_key}/{cell_key}' does not match embedded cell name '{}'",
                        cell.name
                    )));
                }
                for (view_key, view) in &cell.views {
                    if view.name != *view_key {
                        return Err(ProjectIoError::InvalidData(format!(
                            "view map key '{library_key}/{cell_key}/{view_key}' does not match embedded view name '{}'",
                            view.name
                        )));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_workspace_references(&self) -> Result<(), ProjectIoError> {
        let mut required_schematic_buffers = HashSet::new();

        let active_view_type =
            self.validate_library_reference("workspace.active_view", &self.workspace.active_view)?;
        if !self
            .workspace
            .open_views
            .iter()
            .any(|open_view| open_view.reference == self.workspace.active_view)
        {
            return Err(ProjectIoError::InvalidData(format!(
                "workspace.active_view references '{}', but workspace.open_views does not contain it",
                self.workspace.active_key()
            )));
        }
        if project_view_requires_schematic_buffer(active_view_type) {
            required_schematic_buffers.insert(self.workspace.active_key());
        }
        for (index, open_view) in self.workspace.open_views.iter().enumerate() {
            let library_view_type = self.validate_library_reference(
                &format!("workspace.open_views[{index}]"),
                &open_view.reference,
            )?;
            if library_view_type != open_view.view_type {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace.open_views[{index}] references '{}', but persisted view type '{}' does not match library view type '{}'",
                    open_view.reference.key(),
                    open_view.view_type.display_name(),
                    library_view_type.display_name()
                )));
            }
            if project_view_requires_schematic_buffer(open_view.view_type) {
                required_schematic_buffers.insert(open_view.reference.key());
            }
        }
        for (index, reference) in self.workspace.hierarchy_stack.iter().enumerate() {
            self.validate_library_reference(
                &format!("workspace.hierarchy_stack[{index}]"),
                reference,
            )?;
        }

        for key in required_schematic_buffers {
            if !self.workspace.schematic_buffers.contains_key(&key) {
                return Err(ProjectIoError::InvalidData(format!(
                    "workspace references schematic/testbench buffer '{key}', but no backing buffer was found"
                )));
            }
        }
        Ok(())
    }

    fn validate_library_reference(
        &self,
        context: &str,
        reference: &CellViewRef,
    ) -> Result<ViewType, ProjectIoError> {
        let key = reference.key();
        let library = self
            .libraries
            .get_library(&reference.library)
            .ok_or_else(|| {
                ProjectIoError::InvalidData(format!(
                    "{context} references '{key}', but library '{}' was not found",
                    reference.library
                ))
            })?;
        let cell = library.get_cell(&reference.cell).ok_or_else(|| {
            ProjectIoError::InvalidData(format!(
                "{context} references '{key}', but cell '{}' was not found in library '{}'",
                reference.cell, reference.library
            ))
        })?;
        let view = cell.get_view(&reference.view).ok_or_else(|| {
            ProjectIoError::InvalidData(format!(
                "{context} references '{key}', but view '{}' was not found in cell '{}'",
                reference.view, reference.cell
            ))
        })?;
        Ok(view.view_type)
    }
}

fn project_view_requires_schematic_buffer(view_type: ViewType) -> bool {
    matches!(view_type, ViewType::Schematic | ViewType::Testbench)
}

const LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION: u32 = 1;
const PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION: u32 = 2;

fn default_simulation_results_schema_version() -> u32 {
    // A present result-history object without a version predates the stable-ID
    // schema. New objects set v2 explicitly through `Default`/`from_state`.
    LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION
}

fn analysis_type_key(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::DcOp => "DcOp",
        AnalysisType::DcSweep => "DcSweep",
        AnalysisType::Ac => "Ac",
        AnalysisType::Disto => "Disto",
        AnalysisType::Transient => "Transient",
        AnalysisType::Noise => "Noise",
        AnalysisType::PoleZero => "PoleZero",
        AnalysisType::Tf => "Tf",
        AnalysisType::Sensitivity => "Sensitivity",
        AnalysisType::Pac => "Pac",
        AnalysisType::Pnoise => "Pnoise",
        AnalysisType::Pxf => "Pxf",
        AnalysisType::Pstb => "Pstb",
        AnalysisType::Stb => "Stb",
        AnalysisType::MonteCarlo => "MonteCarlo",
        AnalysisType::Parametric => "Parametric",
        AnalysisType::Corner => "Corner",
        AnalysisType::Reliability => "Reliability",
        AnalysisType::Optimization => "Optimization",
        AnalysisType::Soa => "Soa",
        AnalysisType::SParameter => "SParameter",
        AnalysisType::Envelope => "Envelope",
        AnalysisType::Fourier => "Fourier",
        AnalysisType::HarmonicBalance => "HarmonicBalance",
        AnalysisType::Pss => "Pss",
    }
}

fn analysis_type_from_key(key: &str) -> Option<AnalysisType> {
    match key {
        "DcOp" | ".op" => Some(AnalysisType::DcOp),
        "DcSweep" | ".dc" => Some(AnalysisType::DcSweep),
        "Ac" | ".ac" => Some(AnalysisType::Ac),
        "Disto" | ".disto" => Some(AnalysisType::Disto),
        "Transient" | ".tran" => Some(AnalysisType::Transient),
        "Noise" | ".noise" => Some(AnalysisType::Noise),
        "PoleZero" | ".pz" => Some(AnalysisType::PoleZero),
        "Tf" | ".tf" => Some(AnalysisType::Tf),
        "Sensitivity" | ".sens" => Some(AnalysisType::Sensitivity),
        "Pac" | ".pac" => Some(AnalysisType::Pac),
        "Pnoise" | ".pnoise" => Some(AnalysisType::Pnoise),
        "Pxf" | ".pxf" => Some(AnalysisType::Pxf),
        "Pstb" | ".pstb" => Some(AnalysisType::Pstb),
        "Stb" | ".stb" => Some(AnalysisType::Stb),
        "MonteCarlo" | ".mc" => Some(AnalysisType::MonteCarlo),
        "Parametric" | ".step" => Some(AnalysisType::Parametric),
        "Corner" => Some(AnalysisType::Corner),
        "Reliability" | ".reliability" => Some(AnalysisType::Reliability),
        "Optimization" | ".opt" => Some(AnalysisType::Optimization),
        "Soa" | ".soa" => Some(AnalysisType::Soa),
        "SParameter" | ".sp" => Some(AnalysisType::SParameter),
        "Envelope" | ".envlp" => Some(AnalysisType::Envelope),
        "Fourier" | ".four" => Some(AnalysisType::Fourier),
        "HarmonicBalance" | ".hb" => Some(AnalysisType::HarmonicBalance),
        "Pss" | ".pss" => Some(AnalysisType::Pss),
        _ => None,
    }
}

/// Stable project-file representation of result history.
///
/// This is intentionally separate from `SimulationState`: project files should
/// persist user-visible result data, not transient runner flags, progress text,
/// or UI trigger bits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSimulationResults {
    #[serde(default = "default_simulation_results_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub runs: Vec<ProjectSimulationRun>,
    #[serde(default)]
    pub next_run_id: u64,
    /// Stable v2 selection identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_run_stable_id: Option<RunId>,
    /// Stable v2 selected immutable dataset identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_dataset_id: Option<DatasetId>,
    /// Run-local analysis sequence within the selected immutable dataset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_analysis_sequence: Option<u64>,
    /// Stable v2 dataset overlay identities.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overlay_dataset_ids: Vec<DatasetId>,
    /// Legacy v1 display-sequence selection; consumed during migration and
    /// never written by a current project.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_run_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_analysis_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overlay_run_ids: Vec<u64>,
}

impl Default for ProjectSimulationResults {
    fn default() -> Self {
        Self {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: Vec::new(),
            next_run_id: 0,
            active_run_stable_id: None,
            active_dataset_id: None,
            active_analysis_sequence: None,
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        }
    }
}

impl ProjectSimulationResults {
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
            && self.next_run_id == 0
            && self.active_run_stable_id.is_none()
            && self.active_dataset_id.is_none()
            && self.active_analysis_sequence.is_none()
            && self.overlay_dataset_ids.is_empty()
            && self.active_run_id.is_none()
            && self.active_analysis_id.is_none()
            && self.overlay_run_ids.is_empty()
    }

    pub fn from_state(state: &SimulationState) -> Self {
        if state.runs.is_empty() {
            return Self::default();
        }

        let runs: Vec<_> = state.runs.iter().map(ProjectSimulationRun::from).collect();
        let max_run_id = state.runs.iter().map(|run| run.id).max().unwrap_or(0);
        Self {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs,
            next_run_id: state.next_run_id.max(max_run_id),
            active_run_stable_id: state.active_run().map(|run| run.run_id),
            active_dataset_id: state.active_run().map(|run| run.dataset_id),
            active_analysis_sequence: state.active_analysis().map(|analysis| analysis.id),
            overlay_dataset_ids: state.overlay_dataset_ids.clone(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        }
    }

    pub fn into_simulation_state(self) -> Result<SimulationState, String> {
        let mut state = SimulationState::default();
        self.apply_to_state(&mut state)?;
        Ok(state)
    }

    pub fn apply_to_state(mut self, state: &mut SimulationState) -> Result<(), String> {
        self.migrate_to_current()?;
        self.validate()?;
        let runs = self
            .runs
            .into_iter()
            .map(ProjectSimulationRun::into_run)
            .collect::<Result<Vec<_>, _>>()?;
        state.restore_run_history(
            runs,
            self.next_run_id,
            self.active_run_stable_id,
            self.active_dataset_id,
            self.active_analysis_sequence,
            self.overlay_dataset_ids,
        );
        Ok(())
    }

    /// Upgrade v1 display-sequence references to stable run and dataset IDs.
    /// Missing identities remain explicit after deserialization and are minted
    /// only in this v1 branch. The next save persists them under schema v2.
    pub(crate) fn migrate_to_current(&mut self) -> Result<(), String> {
        if self.schema_version != LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION {
            return Ok(());
        }

        for run in &mut self.runs {
            if run.run_id.is_none() {
                run.run_id = Some(RunId::new());
            }
            if run.dataset_id.is_none() {
                run.dataset_id = Some(DatasetId::new());
            }
        }

        if let Some(run_sequence) = self.active_run_id {
            let active_run = self
                .runs
                .iter()
                .find(|run| run.id == run_sequence)
                .ok_or_else(|| {
                    format!("legacy active simulation run sequence {run_sequence} does not exist")
                })?;
            self.active_run_stable_id = Some(active_run.run_id.ok_or_else(|| {
                format!(
                    "legacy simulation run sequence {} has no stable id",
                    active_run.id
                )
            })?);
            self.active_dataset_id = Some(active_run.dataset_id.ok_or_else(|| {
                format!(
                    "legacy simulation run sequence {} has no dataset id",
                    active_run.id
                )
            })?);
            self.active_analysis_sequence = if let Some(sequence) = self.active_analysis_id {
                if !active_run
                    .analyses
                    .iter()
                    .any(|analysis| analysis.id == sequence)
                {
                    return Err(format!(
                        "legacy active analysis sequence {sequence} does not exist in run {}",
                        active_run.id
                    ));
                }
                Some(sequence)
            } else {
                None
            };
        } else if self.active_analysis_id.is_some() {
            return Err("legacy active analysis has no active simulation run".to_owned());
        }

        if !self.overlay_run_ids.is_empty() {
            self.overlay_dataset_ids.clear();
            for sequence in &self.overlay_run_ids {
                let dataset_id = self
                    .runs
                    .iter()
                    .find(|run| run.id == *sequence)
                    .map(|run| {
                        run.dataset_id.ok_or_else(|| {
                            format!("legacy overlay run sequence {sequence} has no dataset id")
                        })
                    })
                    .transpose()?
                    .ok_or_else(|| {
                        format!("legacy overlay run sequence {sequence} does not exist")
                    })?;
                if Some(dataset_id) != self.active_dataset_id
                    && !self.overlay_dataset_ids.contains(&dataset_id)
                {
                    self.overlay_dataset_ids.push(dataset_id);
                }
            }
        }

        self.active_run_id = None;
        self.active_analysis_id = None;
        self.overlay_run_ids.clear();
        self.schema_version = PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION;
        Ok(())
    }

    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.schema_version != PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION {
            return Err(format!(
                "unsupported simulation results schema version {}",
                self.schema_version
            ));
        }
        if self.active_run_id.is_some()
            || self.active_analysis_id.is_some()
            || !self.overlay_run_ids.is_empty()
        {
            return Err("current simulation results retain unmigrated v1 references".to_owned());
        }

        let mut run_sequences = HashSet::new();
        let mut run_ids = HashSet::new();
        let mut dataset_ids = HashSet::new();
        for (run_idx, run) in self.runs.iter().enumerate() {
            if run.id == 0 {
                return Err(format!("runs[{run_idx}].id must be greater than zero"));
            }
            if !run_sequences.insert(run.id) {
                return Err(format!("duplicate simulation run id {}", run.id));
            }
            let run_id = run.run_id.ok_or_else(|| {
                format!("runs[{run_idx}].run_id is required by simulation results schema v2")
            })?;
            if !run_ids.insert(run_id) {
                return Err(format!("duplicate stable simulation run id {run_id}"));
            }
            let dataset_id = run.dataset_id.ok_or_else(|| {
                format!("runs[{run_idx}].dataset_id is required by simulation results schema v2")
            })?;
            if !dataset_ids.insert(dataset_id) {
                return Err(format!("duplicate immutable dataset id {dataset_id}"));
            }
            run.validate(run_idx)?;
        }
        if let Some(active_run_id) = self.active_run_stable_id
            && !run_ids.contains(&active_run_id)
        {
            return Err(format!(
                "active simulation run id {} does not exist in persisted history",
                active_run_id
            ));
        }
        match (self.active_run_stable_id, self.active_dataset_id) {
            (Some(active_run_id), Some(active_dataset_id)) => {
                let Some(active_run) = self
                    .runs
                    .iter()
                    .find(|run| run.run_id == Some(active_run_id))
                else {
                    return Err(format!(
                        "active simulation run id {} does not exist in persisted history",
                        active_run_id
                    ));
                };
                if active_run.dataset_id != Some(active_dataset_id) {
                    return Err(format!(
                        "active dataset id {} does not belong to active run {}",
                        active_dataset_id, active_run_id
                    ));
                }
                if let Some(sequence) = self.active_analysis_sequence
                    && !active_run
                        .analyses
                        .iter()
                        .any(|analysis| analysis.id == sequence)
                {
                    return Err(format!(
                        "active analysis sequence {} does not exist in active dataset {}",
                        sequence, active_dataset_id
                    ));
                }
            }
            (Some(active_run_id), None) => {
                return Err(format!(
                    "active simulation run id {} has no active dataset id",
                    active_run_id
                ));
            }
            (None, Some(active_dataset_id)) => {
                return Err(format!(
                    "active dataset id {} has no active run id",
                    active_dataset_id
                ));
            }
            (None, None) if self.active_analysis_sequence.is_some() => {
                return Err("active analysis sequence has no active dataset".to_owned());
            }
            (None, None) => {}
        }
        let mut overlay_ids = HashSet::new();
        for overlay_id in &self.overlay_dataset_ids {
            if !dataset_ids.contains(overlay_id) {
                return Err(format!(
                    "overlay dataset id {} does not exist in persisted history",
                    overlay_id
                ));
            }
            if !overlay_ids.insert(*overlay_id) {
                return Err(format!("duplicate overlay dataset id {}", overlay_id));
            }
            if Some(*overlay_id) == self.active_dataset_id {
                return Err(format!(
                    "active dataset id {} cannot also be an overlay",
                    overlay_id
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSimulationRun {
    #[serde(default)]
    pub run_id: Option<RunId>,
    #[serde(default)]
    pub dataset_id: Option<DatasetId>,
    /// Display sequence retained for labels and v1 migration.
    pub id: u64,
    pub label: String,
    pub timestamp: f64,
    #[serde(default)]
    pub analyses: Vec<ProjectAnalysisResult>,
    #[serde(default)]
    pub elapsed_time: f64,
    #[serde(default = "default_true")]
    pub success: bool,
}

impl ProjectSimulationRun {
    fn into_run(self) -> Result<SimulationRun, String> {
        let run_id = self
            .run_id
            .ok_or_else(|| format!("simulation run sequence {} has no stable id", self.id))?;
        let dataset_id = self
            .dataset_id
            .ok_or_else(|| format!("simulation run sequence {} has no dataset id", self.id))?;
        let analyses = self
            .analyses
            .into_iter()
            .map(ProjectAnalysisResult::into_analysis)
            .collect();
        Ok(SimulationRun {
            run_id,
            dataset_id,
            id: self.id,
            label: self.label,
            timestamp: self.timestamp,
            analyses,
            elapsed_time: self.elapsed_time,
            success: self.success,
        })
    }

    fn validate(&self, run_idx: usize) -> Result<(), String> {
        require_finite(self.timestamp, &format!("runs[{run_idx}].timestamp"))?;
        require_finite(self.elapsed_time, &format!("runs[{run_idx}].elapsed_time"))?;
        let mut analysis_ids = HashSet::new();
        for (analysis_idx, analysis) in self.analyses.iter().enumerate() {
            if analysis.id == 0 {
                return Err(format!(
                    "runs[{run_idx}].analyses[{analysis_idx}].id must be greater than zero"
                ));
            }
            if !analysis_ids.insert(analysis.id) {
                return Err(format!(
                    "duplicate analysis id {} in runs[{run_idx}]",
                    analysis.id
                ));
            }
            analysis.validate(run_idx, analysis_idx)?;
        }
        Ok(())
    }
}

impl From<&SimulationRun> for ProjectSimulationRun {
    fn from(run: &SimulationRun) -> Self {
        Self {
            run_id: Some(run.run_id),
            dataset_id: Some(run.dataset_id),
            id: run.id,
            label: run.label.clone(),
            timestamp: run.timestamp,
            analyses: run
                .analyses
                .iter()
                .map(ProjectAnalysisResult::from)
                .collect(),
            elapsed_time: run.elapsed_time,
            success: run.success,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectAnalysisResult {
    /// Run-local display sequence retained for labels and v1 migration.
    pub id: u64,
    pub analysis_type: String,
    pub label: String,
    pub timestamp: f64,
    #[serde(default)]
    pub waveforms: Vec<ProjectWaveformData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dc_op: Option<ProjectDcOpResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_op: Option<ProjectDeviceOpReport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_summary: Option<ProjectNoiseSummary>,
    #[serde(default)]
    pub measurements: Vec<ProjectMeasurement>,
    #[serde(default = "default_true")]
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ProjectAnalysisResult {
    fn into_analysis(self) -> AnalysisResult {
        AnalysisResult {
            id: self.id,
            analysis_type: analysis_type_from_key(&self.analysis_type)
                .unwrap_or(AnalysisType::Transient),
            label: self.label,
            timestamp: self.timestamp,
            waveforms: self
                .waveforms
                .into_iter()
                .map(ProjectWaveformData::into_waveform)
                .collect(),
            dc_op: self.dc_op.map(ProjectDcOpResult::into_dc_op),
            device_op: self.device_op.map(ProjectDeviceOpReport::into_report),
            noise_summary: self
                .noise_summary
                .map(ProjectNoiseSummary::into_noise_summary),
            measurements: self
                .measurements
                .into_iter()
                .map(ProjectMeasurement::into_measurement)
                .collect(),
            success: self.success,
            error_message: self.error_message,
        }
    }

    fn validate(&self, run_idx: usize, analysis_idx: usize) -> Result<(), String> {
        let prefix = format!("runs[{run_idx}].analyses[{analysis_idx}]");
        if analysis_type_from_key(&self.analysis_type).is_none() {
            return Err(format!(
                "{prefix}.analysis_type has unknown analysis type '{}'",
                self.analysis_type
            ));
        }
        require_finite(self.timestamp, &format!("{prefix}.timestamp"))?;
        let mut waveform_names = HashSet::new();
        for (waveform_idx, waveform) in self.waveforms.iter().enumerate() {
            if !waveform_names.insert(waveform.name.as_str()) {
                return Err(format!(
                    "{prefix} has duplicate waveform name '{}'",
                    waveform.name
                ));
            }
            waveform.validate(&format!("{prefix}.waveforms[{waveform_idx}]"))?;
        }
        if let Some(dc_op) = &self.dc_op {
            dc_op.validate(&format!("{prefix}.dc_op"))?;
        }
        if let Some(device_op) = &self.device_op {
            device_op.validate(&format!("{prefix}.device_op"))?;
        }
        if let Some(noise_summary) = &self.noise_summary {
            noise_summary.validate(&format!("{prefix}.noise_summary"))?;
        }
        for (measurement_idx, measurement) in self.measurements.iter().enumerate() {
            measurement.validate(&format!("{prefix}.measurements[{measurement_idx}]"))?;
        }
        Ok(())
    }
}

impl From<&AnalysisResult> for ProjectAnalysisResult {
    fn from(analysis: &AnalysisResult) -> Self {
        Self {
            id: analysis.id,
            analysis_type: analysis_type_key(analysis.analysis_type).to_string(),
            label: analysis.label.clone(),
            timestamp: analysis.timestamp,
            waveforms: analysis
                .waveforms
                .iter()
                .map(ProjectWaveformData::from)
                .collect(),
            dc_op: analysis.dc_op.as_ref().map(ProjectDcOpResult::from),
            device_op: analysis.device_op.as_ref().map(ProjectDeviceOpReport::from),
            noise_summary: analysis
                .noise_summary
                .as_ref()
                .map(ProjectNoiseSummary::from),
            measurements: analysis
                .measurements
                .iter()
                .map(ProjectMeasurement::from)
                .collect(),
            success: analysis.success,
            error_message: analysis.error_message.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectWaveformData {
    pub name: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complex: Option<ProjectComplexWaveformComponents>,
}

impl ProjectWaveformData {
    fn into_waveform(self) -> WaveformData {
        let mut waveform = WaveformData::new(self.name, self.x, self.y, self.color);
        waveform.visible = self.visible;
        if let Some(complex) = self.complex {
            waveform =
                waveform.with_complex_components(complex.source_name, complex.real, complex.imag);
        }
        waveform
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        if self.x.len() != self.y.len() {
            return Err(format!(
                "{prefix} has mismatched x/y sample counts ({} vs {})",
                self.x.len(),
                self.y.len()
            ));
        }
        require_finite_values(&self.x, &format!("{prefix}.x"))?;
        require_monotonic_non_decreasing(&self.x, &format!("{prefix}.x"))?;
        require_finite_values(&self.y, &format!("{prefix}.y"))?;
        if let Some(complex) = &self.complex {
            complex.validate(prefix, self.y.len())?;
        }
        Ok(())
    }
}

impl From<&WaveformData> for ProjectWaveformData {
    fn from(waveform: &WaveformData) -> Self {
        Self {
            name: waveform.name.clone(),
            x: waveform.x.iter().copied().collect(),
            y: waveform.y.iter().copied().collect(),
            color: waveform.color.clone(),
            visible: waveform.visible,
            complex: waveform
                .complex
                .as_ref()
                .map(|complex| ProjectComplexWaveformComponents {
                    source_name: complex.source_name.clone(),
                    real: complex.real.iter().copied().collect(),
                    imag: complex.imag.iter().copied().collect(),
                }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectComplexWaveformComponents {
    pub source_name: String,
    pub real: Vec<f64>,
    pub imag: Vec<f64>,
}

impl ProjectComplexWaveformComponents {
    fn validate(&self, prefix: &str, expected_len: usize) -> Result<(), String> {
        if self.real.len() != self.imag.len() || self.real.len() != expected_len {
            return Err(format!(
                "{prefix}.complex has mismatched real/imag/display sample counts"
            ));
        }
        require_finite_values(&self.real, &format!("{prefix}.complex.real"))?;
        require_finite_values(&self.imag, &format!("{prefix}.complex.imag"))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDcOpResult {
    #[serde(default)]
    pub node_voltages: Vec<ProjectOperatingPointValue>,
    #[serde(default)]
    pub branch_currents: Vec<ProjectOperatingPointValue>,
    #[serde(default)]
    pub power_dissipation: Vec<ProjectOperatingPointValue>,
}

impl ProjectDcOpResult {
    fn into_dc_op(self) -> DcOpResult {
        DcOpResult {
            node_voltages: self
                .node_voltages
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
            branch_currents: self
                .branch_currents
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
            power_dissipation: self
                .power_dissipation
                .into_iter()
                .map(ProjectOperatingPointValue::into_op_value)
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        for (idx, value) in self.node_voltages.iter().enumerate() {
            value.validate(&format!("{prefix}.node_voltages[{idx}]"))?;
        }
        for (idx, value) in self.branch_currents.iter().enumerate() {
            value.validate(&format!("{prefix}.branch_currents[{idx}]"))?;
        }
        for (idx, value) in self.power_dissipation.iter().enumerate() {
            value.validate(&format!("{prefix}.power_dissipation[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&DcOpResult> for ProjectDcOpResult {
    fn from(dc_op: &DcOpResult) -> Self {
        Self {
            node_voltages: dc_op
                .node_voltages
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
            branch_currents: dc_op
                .branch_currents
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
            power_dissipation: dc_op
                .power_dissipation
                .iter()
                .map(ProjectOperatingPointValue::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectOperatingPointValue {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

impl ProjectOperatingPointValue {
    fn into_op_value(self) -> OperatingPointValue {
        OperatingPointValue {
            name: self.name,
            value: self.value,
            unit: self.unit,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_finite(self.value, &format!("{prefix}.value"))
    }
}

impl From<&OperatingPointValue> for ProjectOperatingPointValue {
    fn from(value: &OperatingPointValue) -> Self {
        Self {
            name: value.name.clone(),
            value: value.value,
            unit: value.unit.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNoiseSummary {
    #[serde(default)]
    pub rows: Vec<ProjectNoiseContributorRow>,
    pub total_rms: f64,
    pub band: (f64, f64),
}

impl ProjectNoiseSummary {
    fn into_noise_summary(self) -> NoiseSummary {
        NoiseSummary {
            rows: self
                .rows
                .into_iter()
                .map(ProjectNoiseContributorRow::into_row)
                .collect(),
            total_rms: self.total_rms,
            band: self.band,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_finite(self.total_rms, &format!("{prefix}.total_rms"))?;
        require_finite(self.band.0, &format!("{prefix}.band.0"))?;
        require_finite(self.band.1, &format!("{prefix}.band.1"))?;
        for (idx, row) in self.rows.iter().enumerate() {
            row.validate(&format!("{prefix}.rows[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&NoiseSummary> for ProjectNoiseSummary {
    fn from(summary: &NoiseSummary) -> Self {
        Self {
            rows: summary
                .rows
                .iter()
                .map(ProjectNoiseContributorRow::from)
                .collect(),
            total_rms: summary.total_rms,
            band: summary.band,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNoiseContributorRow {
    pub device: String,
    pub mechanism: String,
    pub power: f64,
    pub share_pct: f64,
}

impl ProjectNoiseContributorRow {
    fn into_row(self) -> NoiseContributorRow {
        NoiseContributorRow {
            device: self.device,
            mechanism: intern_static_label(self.mechanism),
            power: self.power,
            share_pct: self.share_pct,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.mechanism, &format!("{prefix}.mechanism"))?;
        require_finite(self.power, &format!("{prefix}.power"))?;
        require_finite(self.share_pct, &format!("{prefix}.share_pct"))
    }
}

impl From<&NoiseContributorRow> for ProjectNoiseContributorRow {
    fn from(row: &NoiseContributorRow) -> Self {
        Self {
            device: row.device.clone(),
            mechanism: row.mechanism.to_string(),
            power: row.power,
            share_pct: row.share_pct,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeviceOpReport {
    #[serde(default)]
    pub entries: Vec<ProjectDeviceOpEntry>,
}

impl ProjectDeviceOpReport {
    fn into_report(self) -> rspice_core::circuit::DeviceOpReport {
        rspice_core::circuit::DeviceOpReport {
            entries: self
                .entries
                .into_iter()
                .map(ProjectDeviceOpEntry::into_entry)
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        for (idx, entry) in self.entries.iter().enumerate() {
            entry.validate(&format!("{prefix}.entries[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&rspice_core::circuit::DeviceOpReport> for ProjectDeviceOpReport {
    fn from(report: &rspice_core::circuit::DeviceOpReport) -> Self {
        Self {
            entries: report
                .entries
                .iter()
                .map(ProjectDeviceOpEntry::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectDeviceOpEntry {
    pub name: String,
    pub device_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    pub params: Vec<ProjectNamedValue>,
}

impl ProjectDeviceOpEntry {
    fn into_entry(self) -> rspice_core::circuit::DeviceOpEntry {
        rspice_core::circuit::DeviceOpEntry {
            name: self.name,
            device_kind: intern_static_label(self.device_kind),
            region: self.region.map(intern_static_label),
            params: self
                .params
                .into_iter()
                .map(|param| (intern_static_label(param.name), param.value))
                .collect(),
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.device_kind, &format!("{prefix}.device_kind"))?;
        if let Some(region) = &self.region {
            require_static_label(region, &format!("{prefix}.region"))?;
        }
        for (idx, param) in self.params.iter().enumerate() {
            param.validate(&format!("{prefix}.params[{idx}]"))?;
        }
        Ok(())
    }
}

impl From<&rspice_core::circuit::DeviceOpEntry> for ProjectDeviceOpEntry {
    fn from(entry: &rspice_core::circuit::DeviceOpEntry) -> Self {
        Self {
            name: entry.name.clone(),
            device_kind: entry.device_kind.to_string(),
            region: entry.region.map(str::to_string),
            params: entry
                .params
                .iter()
                .map(|(name, value)| ProjectNamedValue {
                    name: (*name).to_string(),
                    value: *value,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectNamedValue {
    pub name: String,
    pub value: f64,
}

impl ProjectNamedValue {
    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_static_label(&self.name, &format!("{prefix}.name"))?;
        require_finite(self.value, &format!("{prefix}.value"))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectMeasurement {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<f64>,
}

impl ProjectMeasurement {
    fn into_measurement(self) -> rspice_core::MeasureResult {
        rspice_core::MeasureResult {
            name: self.name,
            value: self.value,
            error: self.error,
            passed: self.passed,
            expected: self.expected,
            tolerance: self.tolerance,
        }
    }

    fn validate(&self, prefix: &str) -> Result<(), String> {
        require_optional_finite(self.value, &format!("{prefix}.value"))?;
        require_optional_finite(self.expected, &format!("{prefix}.expected"))?;
        require_optional_finite(self.tolerance, &format!("{prefix}.tolerance"))
    }
}

impl From<&rspice_core::MeasureResult> for ProjectMeasurement {
    fn from(measurement: &rspice_core::MeasureResult) -> Self {
        Self {
            name: measurement.name.clone(),
            value: measurement.value,
            error: measurement.error.clone(),
            passed: measurement.passed,
            expected: measurement.expected,
            tolerance: measurement.tolerance,
        }
    }
}

fn default_true() -> bool {
    true
}

fn require_finite(value: f64, field: &str) -> Result<(), String> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(format!("{field} is not finite"))
    }
}

fn require_optional_finite(value: Option<f64>, field: &str) -> Result<(), String> {
    if let Some(value) = value {
        require_finite(value, field)?;
    }
    Ok(())
}

fn require_finite_values(values: &[f64], field: &str) -> Result<(), String> {
    for (idx, value) in values.iter().enumerate() {
        if !value.is_finite() {
            return Err(format!("{field}[{idx}] is not finite"));
        }
    }
    Ok(())
}

fn require_monotonic_non_decreasing(values: &[f64], field: &str) -> Result<(), String> {
    for (idx, pair) in values.windows(2).enumerate() {
        if pair[1] < pair[0] {
            return Err(format!(
                "{field} must be monotonic non-decreasing; sample {} ({}) is less than sample {} ({})",
                idx + 1,
                pair[1],
                idx,
                pair[0]
            ));
        }
    }
    Ok(())
}

fn require_static_label(value: &str, field: &str) -> Result<(), String> {
    if known_static_label(value).is_some() {
        Ok(())
    } else {
        Err(format!("{field} has unknown static label '{value}'"))
    }
}

fn intern_static_label(value: String) -> &'static str {
    known_static_label(&value).unwrap_or("unknown")
}

fn known_static_label(value: &str) -> Option<&'static str> {
    match value {
        "MOSFET" => "MOSFET",
        "BSIM3" => "BSIM3",
        "BSIM4" => "BSIM4",
        "BJT" => "BJT",
        "DIODE" => "DIODE",
        "JFET" => "JFET",
        "MESFET" => "MESFET",
        "id" => "id",
        "vgs" => "vgs",
        "vds" => "vds",
        "vbs" => "vbs",
        "vth" => "vth",
        "vdsat" => "vdsat",
        "gm" => "gm",
        "gds" => "gds",
        "gmb" => "gmb",
        "gmbs" => "gmbs",
        "ic" => "ic",
        "ib" => "ib",
        "vbe" => "vbe",
        "vce" => "vce",
        "beta" => "beta",
        "vd" => "vd",
        "gd" => "gd",
        "igs" => "igs",
        "igd" => "igd",
        "saturation" => "saturation",
        "linear" => "linear",
        "cutoff" => "cutoff",
        "triode" => "triode",
        "subthreshold" => "subthreshold",
        "active" => "active",
        "reverse" => "reverse",
        "forward" => "forward",
        "thermal" => "thermal",
        "flicker" => "flicker",
        "shot" => "shot",
        "burst" => "burst",
        "white" => "white",
        "table" => "table",
        _ => return None,
    }
    .into()
}

#[derive(Debug, Clone)]
pub enum ProjectIoError {
    Cancelled,
    NotFound(PathBuf),
    IncompatibleVersion {
        file_version: String,
        app_version: String,
    },
    ParseError(String),
    SerializeError(String),
    InvalidData(String),
    Io(String),
}

impl std::fmt::Display for ProjectIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => f.write_str("Operation cancelled"),
            Self::NotFound(path) => write!(f, "Project file not found: {}", path.display()),
            Self::IncompatibleVersion {
                file_version,
                app_version,
            } => write!(
                f,
                "Project version {} is not compatible with app version {}",
                file_version, app_version
            ),
            Self::ParseError(error) => write!(f, "Project parse error: {}", error),
            Self::SerializeError(error) => write!(f, "Project serialize error: {}", error),
            Self::InvalidData(error) => write!(f, "Project data error: {}", error),
            Self::Io(error) => write!(f, "Project I/O error: {}", error),
        }
    }
}

impl std::error::Error for ProjectIoError {}

impl From<std::io::Error> for ProjectIoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

pub const PROJECT_FILTER: (&str, &[&str]) = ("RSpice Project", &["rspiceproj", "json"]);

#[cfg(not(target_arch = "wasm32"))]
pub fn show_open_project_dialog() -> Result<PathBuf, ProjectIoError> {
    rfd::FileDialog::new()
        .add_filter(PROJECT_FILTER.0, PROJECT_FILTER.1)
        .add_filter("All Files", &["*"])
        .set_title("Open RSpice Project")
        .pick_file()
        .ok_or(ProjectIoError::Cancelled)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn show_save_project_dialog(default_name: Option<&str>) -> Result<PathBuf, ProjectIoError> {
    let mut dialog = rfd::FileDialog::new()
        .add_filter(PROJECT_FILTER.0, PROJECT_FILTER.1)
        .set_title("Save RSpice Project");

    dialog = dialog.set_file_name(default_name.unwrap_or("untitled.rspiceproj"));

    let mut path = dialog.save_file().ok_or(ProjectIoError::Cancelled)?;
    let has_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rspiceproj"));
    if !has_extension {
        path.set_extension("rspiceproj");
    }
    Ok(path)
}

#[cfg(target_arch = "wasm32")]
pub fn show_open_project_dialog() -> Result<PathBuf, ProjectIoError> {
    Err(ProjectIoError::Io(
        "Use the browser project import workflow for web file selection".to_string(),
    ))
}

#[cfg(target_arch = "wasm32")]
pub fn show_save_project_dialog(default_name: Option<&str>) -> Result<PathBuf, ProjectIoError> {
    Ok(suggested_project_save_path(default_name))
}

pub fn save_project_file(project: &ProjectFile, path: &Path) -> Result<(), ProjectIoError> {
    project.validate()?;
    project
        .simulation_results
        .validate()
        .map_err(ProjectIoError::InvalidData)?;

    #[cfg(target_arch = "wasm32")]
    {
        let contents = serialize_project_file(project)?;
        crate::common::browser_download::download_text_file(path, &contents)
            .map_err(ProjectIoError::Io)?;
        return Ok(());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        if path.exists() {
            let backup = path.with_extension("rspiceproj.bak");
            if let Err(error) = fs::copy(path, backup) {
                log::warn!("Failed to create project backup: {}", error);
            }
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let temp_path = path.with_extension("rspiceproj.tmp");
        let file = File::create(&temp_path)?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, project)
            .map_err(|error| ProjectIoError::SerializeError(error.to_string()))?;
        writer.flush()?;
        fs::rename(temp_path, path)?;

        Ok(())
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn suggested_project_save_path(default_name: Option<&str>) -> PathBuf {
    let mut path = PathBuf::from(
        default_name
            .filter(|name| !name.trim().is_empty())
            .unwrap_or("untitled.rspiceproj"),
    );
    crate::common::file_actions::ensure_file_extension(&mut path, "rspiceproj");
    path
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn serialize_project_file(project: &ProjectFile) -> Result<String, ProjectIoError> {
    project.validate()?;
    project
        .simulation_results
        .validate()
        .map_err(ProjectIoError::InvalidData)?;
    let mut contents = serde_json::to_string_pretty(project)
        .map_err(|error| ProjectIoError::SerializeError(error.to_string()))?;
    contents.push('\n');
    Ok(contents)
}

pub(crate) fn load_project_text(
    contents: &str,
    source_path: Option<&Path>,
) -> Result<ProjectFile, ProjectIoError> {
    let mut project: ProjectFile =
        serde_json::from_str(contents).map_err(|e| ProjectIoError::ParseError(e.to_string()))?;
    project.validate()?;
    let simulation_results_error = project
        .simulation_results
        .migrate_to_current()
        .err()
        .or_else(|| project.simulation_results.validate().err());
    if let Some(error) = simulation_results_error {
        project.simulation_results = ProjectSimulationResults::default();
        project.simulation_results_warning = Some(format!(
            "Simulation results were not restored because their persisted data is invalid: {error}"
        ));
    }
    match source_path {
        Some(path) => project.workspace.project.set_path(path.to_path_buf()),
        None => project.workspace.project.path = None,
    }
    Ok(project)
}

pub fn load_project_file(path: &Path) -> Result<ProjectFile, ProjectIoError> {
    if !path.exists() {
        return Err(ProjectIoError::NotFound(path.to_path_buf()));
    }

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    load_project_text(&contents, Some(path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisType, CellViewRef, OpenCellView, OperatingPointValue,
        SimulationRun, SimulationState, WaveformData,
    };

    #[test]
    fn suggested_project_save_path_defaults_and_enforces_extension() {
        assert_eq!(
            suggested_project_save_path(None),
            PathBuf::from("untitled.rspiceproj")
        );
        assert_eq!(
            suggested_project_save_path(Some("amp")),
            PathBuf::from("amp.rspiceproj")
        );
        assert_eq!(
            suggested_project_save_path(Some("amp.rspiceproj")),
            PathBuf::from("amp.rspiceproj")
        );
    }

    #[test]
    fn project_file_serializes_to_versioned_json() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);

        let json = serialize_project_file(&project).expect("project serializes");

        assert!(json.contains("\"version\""));
        assert!(json.contains("\"workspace\""));
        assert!(json.contains("\"libraries\""));
        assert!(json.ends_with('\n'));
    }

    #[test]
    fn project_file_round_trips_persisted_simulation_results() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut simulation = SimulationState::default();
        let waveform = WaveformData::new(
            "|V(out)|",
            vec![1.0, 10.0, 100.0],
            vec![2.0, 3.0, 4.0],
            "#00aaff",
        )
        .with_complex_components("V(out)", vec![2.0, 3.0, 4.0], vec![0.1, 0.2, 0.3]);
        let mut run = SimulationRun::new(12);
        run.timestamp = 1234.5;
        run.label = "Run 12 (fixture)".to_string();
        run.set_elapsed_time(0.125);
        run.add_analysis(
            AnalysisResult::new(7, AnalysisType::Ac, "AC fixture")
                .with_waveforms(vec![waveform])
                .with_dc_op(crate::state::DcOpResult {
                    node_voltages: vec![OperatingPointValue {
                        name: "V(out)".to_string(),
                        value: 1.25,
                        unit: "V".to_string(),
                    }],
                    branch_currents: Vec::new(),
                    power_dissipation: Vec::new(),
                })
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", 3.0)]),
        );
        let expected_run_id = run.run_id;
        let expected_dataset_id = run.dataset_id;
        simulation.runs = vec![run];
        simulation.next_run_id = 12;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);

        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let json = serialize_project_file(&project).expect("project serializes with results");

        assert!(json.contains("\"simulation_results\""));
        let loaded = load_project_text(&json, None).expect("project reloads");
        let restored = loaded
            .simulation_results
            .into_simulation_state()
            .expect("validated project results restore");

        assert_eq!(restored.run_count(), 1);
        assert_eq!(
            restored.active_run().map(|run| run.run_id),
            Some(expected_run_id)
        );
        assert_eq!(
            restored.active_run().map(|run| run.dataset_id),
            Some(expected_dataset_id)
        );
        assert_eq!(
            restored.active_run().expect("active run").label,
            "Run 12 (fixture)"
        );
        let analysis = restored.active_analysis().expect("active analysis");
        assert_eq!(analysis.id, 7);
        assert_eq!(analysis.analysis_type, AnalysisType::Ac);
        assert_eq!(analysis.measurements[0].name, "gain");
        assert_eq!(analysis.waveforms[0].complex.as_ref().unwrap().imag[2], 0.3);
        assert_eq!(restored.waveforms[0].name, "|V(out)|");

        let mut unversioned_value: serde_json::Value =
            serde_json::from_str(&json).expect("current project parses as JSON");
        unversioned_value["simulation_results"]
            .as_object_mut()
            .expect("simulation result object")
            .remove("schema_version");
        let unversioned_json =
            serde_json::to_string(&unversioned_value).expect("unversioned project serializes");
        let unversioned =
            load_project_text(&unversioned_json, None).expect("unversioned project migrates");
        assert_eq!(
            unversioned.simulation_results.active_run_stable_id,
            Some(expected_run_id)
        );
        assert_eq!(
            unversioned.simulation_results.active_dataset_id,
            Some(expected_dataset_id)
        );
        assert_eq!(
            unversioned.simulation_results.active_analysis_sequence,
            Some(7)
        );
    }

    #[test]
    fn project_simulation_results_omits_empty_history_after_cleared_runs() {
        let mut simulation = SimulationState::default();
        simulation.start_run();
        simulation.clear_runs();

        let results = ProjectSimulationResults::from_state(&simulation);

        assert!(results.is_empty());
    }

    #[test]
    fn project_text_load_drops_invalid_simulation_results_without_rejecting_workspace() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut simulation = SimulationState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
        simulation.runs = vec![run];
        simulation.next_run_id = 1;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let mut value = serde_json::to_value(project).expect("project converts to JSON");
        value["simulation_results"]["schema_version"] = serde_json::Value::from(999);
        let json = serde_json::to_string_pretty(&value).expect("fixture serializes");

        let loaded = load_project_text(&json, None).expect("workspace still loads");

        assert!(loaded.simulation_results.is_empty());
        assert!(
            loaded
                .simulation_results_warning
                .as_deref()
                .unwrap_or_default()
                .contains("unsupported simulation results schema version")
        );
    }

    #[test]
    fn project_text_load_drops_unknown_analysis_type_results_without_parse_failure() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut simulation = SimulationState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "AC"));
        simulation.runs = vec![run];
        simulation.next_run_id = 1;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let json = serialize_project_file(&project)
            .expect("project serializes")
            .replace(
                "\"analysis_type\": \"Ac\"",
                "\"analysis_type\": \"FutureAnalysis\"",
            );

        let loaded = load_project_text(&json, None).expect("workspace still loads");

        assert!(loaded.simulation_results.is_empty());
        assert!(
            loaded
                .simulation_results_warning
                .as_deref()
                .unwrap_or_default()
                .contains("unknown analysis type")
        );
    }

    #[test]
    fn project_results_restore_rejects_invalid_overlay_references() {
        let run_one = SimulationRun::new(1);
        let run_two = SimulationRun::new(2);
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![
                ProjectSimulationRun::from(&run_one),
                ProjectSimulationRun::from(&run_two),
            ],
            next_run_id: 2,
            active_run_stable_id: Some(run_one.run_id),
            active_dataset_id: Some(run_one.dataset_id),
            active_analysis_sequence: None,
            overlay_dataset_ids: vec![
                run_two.dataset_id,
                run_two.dataset_id,
                run_one.dataset_id,
                DatasetId::new(),
            ],
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        let error = results
            .into_simulation_state()
            .expect_err("invalid overlay references fail closed");

        assert!(error.contains("duplicate overlay dataset id"));
    }

    #[test]
    fn project_results_validation_rejects_duplicate_run_ids() {
        let run_one = SimulationRun::new(1);
        let run_duplicate = SimulationRun::new(1);
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![
                ProjectSimulationRun::from(&run_one),
                ProjectSimulationRun::from(&run_duplicate),
            ],
            next_run_id: 1,
            active_run_stable_id: Some(run_one.run_id),
            active_dataset_id: Some(run_one.dataset_id),
            active_analysis_sequence: None,
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        let error = results.validate().expect_err("duplicate run ids fail");

        assert!(error.contains("duplicate simulation run id 1"));
    }

    #[test]
    fn project_results_v2_requires_unique_stable_run_and_dataset_ids() {
        let mut run_one = SimulationRun::new(1);
        run_one.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN one"));
        let mut run_two = SimulationRun::new(2);
        run_two.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN two"));
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run_one, run_two];
        simulation.next_run_id = 2;

        let baseline = ProjectSimulationResults::from_state(&simulation);

        let mut missing_run_identity = baseline.clone();
        missing_run_identity.runs[0].run_id = None;
        let error = missing_run_identity
            .validate()
            .expect_err("schema v2 must not regenerate a missing run id");
        assert!(error.contains("run_id is required"));

        let mut missing_dataset_identity = baseline.clone();
        missing_dataset_identity.runs[0].dataset_id = None;
        let error = missing_dataset_identity
            .validate()
            .expect_err("schema v2 must not regenerate a missing dataset id");
        assert!(error.contains("dataset_id is required"));

        let mut duplicate_run_identity = baseline.clone();
        duplicate_run_identity.runs[1].run_id = duplicate_run_identity.runs[0].run_id;
        let error = duplicate_run_identity
            .validate()
            .expect_err("stable run ids are globally unique");
        assert!(error.contains("duplicate stable simulation run id"));

        let mut duplicate_dataset_identity = baseline;
        duplicate_dataset_identity.runs[1].dataset_id =
            duplicate_dataset_identity.runs[0].dataset_id;
        let error = duplicate_dataset_identity
            .validate()
            .expect_err("dataset ids are globally unique");
        assert!(error.contains("duplicate immutable dataset id"));
    }

    #[test]
    fn project_results_v2_rejects_cross_bound_selection_and_active_overlay() {
        let mut run_one = SimulationRun::new(1);
        run_one.add_analysis(AnalysisResult::new(3, AnalysisType::Transient, "TRAN one"));
        let mut run_two = SimulationRun::new(2);
        run_two.add_analysis(AnalysisResult::new(8, AnalysisType::Ac, "AC two"));
        let run_one_dataset_id = run_one.dataset_id;
        let run_two_dataset_id = run_two.dataset_id;
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run_one, run_two];
        simulation.next_run_id = 2;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        let baseline = ProjectSimulationResults::from_state(&simulation);

        let mut cross_bound = baseline.clone();
        cross_bound.active_dataset_id = Some(run_two_dataset_id);
        let error = cross_bound
            .validate()
            .expect_err("a dataset cannot be rebound to a different active run");
        assert!(error.contains("does not belong to active run"));

        let mut missing_analysis = baseline.clone();
        missing_analysis.active_analysis_sequence = Some(999);
        let error = missing_analysis
            .validate()
            .expect_err("the selected analysis must belong to the active dataset");
        assert!(error.contains("does not exist in active dataset"));

        let mut active_overlay = baseline;
        active_overlay.overlay_dataset_ids = vec![run_one_dataset_id];
        let error = active_overlay
            .validate()
            .expect_err("the active dataset cannot also be an overlay");
        assert!(error.contains("cannot also be an overlay"));
    }

    #[test]
    fn project_text_migrates_v1_result_sequences_once_to_stable_identities() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut run_one = SimulationRun::new(1);
        run_one.add_analysis(AnalysisResult::new(
            4,
            AnalysisType::Transient,
            "TRAN legacy one",
        ));
        let mut run_two = SimulationRun::new(2);
        run_two.add_analysis(AnalysisResult::new(9, AnalysisType::Ac, "AC legacy two"));
        let overlay_dataset_id = run_one.dataset_id;
        let mut simulation = SimulationState::default();
        simulation.runs = vec![run_one, run_two];
        simulation.next_run_id = 2;
        simulation.active_run_idx = Some(1);
        simulation.active_analysis_idx = Some(0);
        simulation.overlay_dataset_ids = vec![overlay_dataset_id];
        let project = ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            ProjectSimulationResults::from_state(&simulation),
        );
        let mut value = serde_json::to_value(project).expect("project converts to JSON");
        let results = value["simulation_results"]
            .as_object_mut()
            .expect("simulation result object");
        results.insert(
            "schema_version".to_owned(),
            serde_json::Value::from(LEGACY_SIMULATION_RESULTS_SCHEMA_VERSION),
        );
        results.remove("active_run_stable_id");
        results.remove("active_dataset_id");
        results.remove("active_analysis_sequence");
        results.remove("overlay_dataset_ids");
        results.insert("active_run_id".to_owned(), serde_json::Value::from(2));
        results.insert("active_analysis_id".to_owned(), serde_json::Value::from(9));
        results.insert("overlay_run_ids".to_owned(), serde_json::json!([1]));
        for run in results["runs"].as_array_mut().expect("legacy run array") {
            let run = run.as_object_mut().expect("legacy run object");
            run.remove("run_id");
            run.remove("dataset_id");
        }

        let mut unversioned_value = value.clone();
        unversioned_value["simulation_results"]
            .as_object_mut()
            .expect("unversioned result object")
            .remove("schema_version");
        let unversioned_json = serde_json::to_string_pretty(&unversioned_value)
            .expect("unversioned legacy fixture serializes");
        let unversioned = load_project_text(&unversioned_json, None)
            .expect("unversioned legacy project migrates as v1");
        assert!(unversioned.simulation_results_warning.is_none());
        assert_eq!(
            unversioned.simulation_results.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );

        let legacy_json = serde_json::to_string_pretty(&value).expect("legacy fixture serializes");
        let migrated = load_project_text(&legacy_json, None).expect("legacy project migrates");

        assert!(migrated.simulation_results_warning.is_none());
        assert_eq!(
            migrated.simulation_results.schema_version,
            PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
        );
        assert!(
            migrated
                .simulation_results
                .runs
                .iter()
                .all(|run| run.run_id.is_some() && run.dataset_id.is_some())
        );
        let active_run_id = migrated.simulation_results.runs[1]
            .run_id
            .expect("migrated active run id");
        let active_dataset_id = migrated.simulation_results.runs[1]
            .dataset_id
            .expect("migrated active dataset id");
        let migrated_overlay_id = migrated.simulation_results.runs[0]
            .dataset_id
            .expect("migrated overlay id");
        assert_eq!(
            migrated.simulation_results.active_run_stable_id,
            Some(active_run_id)
        );
        assert_eq!(
            migrated.simulation_results.active_dataset_id,
            Some(active_dataset_id)
        );
        assert_eq!(
            migrated.simulation_results.active_analysis_sequence,
            Some(9)
        );
        assert_eq!(
            migrated.simulation_results.overlay_dataset_ids,
            vec![migrated_overlay_id]
        );
        assert!(migrated.simulation_results.active_run_id.is_none());
        assert!(migrated.simulation_results.active_analysis_id.is_none());
        assert!(migrated.simulation_results.overlay_run_ids.is_empty());

        let current_json = serialize_project_file(&migrated).expect("migration persists");
        let reloaded = load_project_text(&current_json, None).expect("migrated project reloads");
        assert_eq!(
            reloaded.simulation_results.active_run_stable_id,
            Some(active_run_id)
        );
        assert_eq!(
            reloaded.simulation_results.active_dataset_id,
            Some(active_dataset_id)
        );
        assert_eq!(
            reloaded.simulation_results.active_analysis_sequence,
            Some(9)
        );
        assert_eq!(
            reloaded.simulation_results.overlay_dataset_ids,
            vec![migrated_overlay_id]
        );
    }

    #[test]
    fn project_results_validation_rejects_duplicate_waveform_names_in_analysis() {
        let run_id = RunId::new();
        let dataset_id = DatasetId::new();
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![ProjectSimulationRun {
                run_id: Some(run_id),
                dataset_id: Some(dataset_id),
                id: 1,
                label: "Run 1".to_string(),
                timestamp: 1.0,
                analyses: vec![ProjectAnalysisResult {
                    id: 1,
                    analysis_type: "Transient".to_string(),
                    label: "TRAN".to_string(),
                    timestamp: 1.0,
                    waveforms: vec![
                        ProjectWaveformData {
                            name: "V(out)".to_string(),
                            x: vec![0.0],
                            y: vec![1.0],
                            color: "#00aaff".to_string(),
                            visible: true,
                            complex: None,
                        },
                        ProjectWaveformData {
                            name: "V(out)".to_string(),
                            x: vec![0.0],
                            y: vec![2.0],
                            color: "#ffaa00".to_string(),
                            visible: true,
                            complex: None,
                        },
                    ],
                    dc_op: None,
                    device_op: None,
                    noise_summary: None,
                    measurements: Vec::new(),
                    success: true,
                    error_message: None,
                }],
                elapsed_time: 0.1,
                success: true,
            }],
            next_run_id: 1,
            active_run_stable_id: Some(run_id),
            active_dataset_id: Some(dataset_id),
            active_analysis_sequence: Some(1),
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        let error = results
            .validate()
            .expect_err("duplicate waveform names in an analysis fail");

        assert!(error.contains("runs[0].analyses[0]"));
        assert!(error.contains("duplicate waveform name 'V(out)'"));
    }

    #[test]
    fn project_results_validation_rejects_non_monotonic_waveform_x() {
        let run_id = RunId::new();
        let dataset_id = DatasetId::new();
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![ProjectSimulationRun {
                run_id: Some(run_id),
                dataset_id: Some(dataset_id),
                id: 1,
                label: "Run 1".to_string(),
                timestamp: 1.0,
                analyses: vec![ProjectAnalysisResult {
                    id: 1,
                    analysis_type: "Transient".to_string(),
                    label: "TRAN".to_string(),
                    timestamp: 1.0,
                    waveforms: vec![ProjectWaveformData {
                        name: "V(out)".to_string(),
                        x: vec![0.0, 2.0, 1.0, 3.0],
                        y: vec![0.0, 1.0, 2.0, 3.0],
                        color: "#00aaff".to_string(),
                        visible: true,
                        complex: None,
                    }],
                    dc_op: None,
                    device_op: None,
                    noise_summary: None,
                    measurements: Vec::new(),
                    success: true,
                    error_message: None,
                }],
                elapsed_time: 0.1,
                success: true,
            }],
            next_run_id: 1,
            active_run_stable_id: Some(run_id),
            active_dataset_id: Some(dataset_id),
            active_analysis_sequence: Some(1),
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        let error = results
            .validate()
            .expect_err("non-monotonic waveform x data must fail");

        assert!(error.contains("runs[0].analyses[0].waveforms[0].x"));
        assert!(error.contains("monotonic"));
    }

    #[test]
    fn project_results_preserve_core_noise_mechanism_labels() {
        let run_id = RunId::new();
        let dataset_id = DatasetId::new();
        let results = ProjectSimulationResults {
            schema_version: PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION,
            runs: vec![ProjectSimulationRun {
                run_id: Some(run_id),
                dataset_id: Some(dataset_id),
                id: 1,
                label: "Run 1".to_string(),
                timestamp: 1.0,
                analyses: vec![ProjectAnalysisResult {
                    id: 1,
                    analysis_type: "Noise".to_string(),
                    label: "NOISE".to_string(),
                    timestamp: 1.0,
                    waveforms: Vec::new(),
                    dc_op: None,
                    device_op: None,
                    noise_summary: Some(ProjectNoiseSummary {
                        rows: vec![
                            ProjectNoiseContributorRow {
                                device: "BNOISE1".to_string(),
                                mechanism: "white".to_string(),
                                power: 1.0e-18,
                                share_pct: 60.0,
                            },
                            ProjectNoiseContributorRow {
                                device: "ATABLE1".to_string(),
                                mechanism: "table".to_string(),
                                power: 2.0e-18,
                                share_pct: 40.0,
                            },
                        ],
                        total_rms: 1.0e-6,
                        band: (1.0, 1.0e6),
                    }),
                    measurements: Vec::new(),
                    success: true,
                    error_message: None,
                }],
                elapsed_time: 0.1,
                success: true,
            }],
            next_run_id: 1,
            active_run_stable_id: Some(run_id),
            active_dataset_id: Some(dataset_id),
            active_analysis_sequence: Some(1),
            overlay_dataset_ids: Vec::new(),
            active_run_id: None,
            active_analysis_id: None,
            overlay_run_ids: Vec::new(),
        };

        results.validate().expect("core noise labels are valid");

        let restored = results
            .into_simulation_state()
            .expect("valid result history restores");
        let summary = restored
            .active_analysis()
            .and_then(|analysis| analysis.noise_summary.as_ref())
            .expect("noise summary restores");

        assert_eq!(summary.rows[0].mechanism, "white");
        assert_eq!(summary.rows[1].mechanism, "table");
    }

    #[test]
    fn project_text_load_updates_source_path_without_renaming_identity() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace
            .project
            .set_path(PathBuf::from("stale-native-path.rspiceproj"));
        let project = ProjectFile::new(workspace, libraries);
        let json = serialize_project_file(&project).expect("project serializes");

        let loaded = load_project_text(&json, Some(Path::new("browser-import.rspiceproj")))
            .expect("project text loads");

        assert_eq!(
            loaded.workspace.project.path.as_deref(),
            Some(Path::new("browser-import.rspiceproj"))
        );
        assert_eq!(
            loaded.workspace.project.display_name(),
            "stale-native-path",
            "moving a project file must not silently rename its logical identity"
        );
    }

    #[test]
    fn project_text_load_without_source_path_clears_stale_file_identity() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace
            .project
            .set_path(PathBuf::from("stale-native-path.rspiceproj"));
        let project = ProjectFile::new(workspace, libraries);
        let json = serialize_project_file(&project).expect("project serializes");

        let loaded = load_project_text(&json, None).expect("project text loads");

        assert!(loaded.workspace.project.path.is_none());
        assert_eq!(loaded.workspace.project.display_name(), "stale-native-path");
    }

    #[test]
    fn legacy_project_migration_assigns_stable_identity_metadata() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);
        let mut value = serde_json::to_value(project).expect("project converts to JSON");
        let descriptor = value["workspace"]["project"]
            .as_object_mut()
            .expect("project descriptor is an object");
        descriptor.remove("id");
        descriptor.remove("schema_version");
        descriptor.remove("revision");
        let legacy = serde_json::to_string_pretty(&value).expect("legacy fixture serializes");

        let migrated = load_project_text(&legacy, None).expect("legacy project migrates");

        assert!(!migrated.workspace.project.id().as_uuid().is_nil());
        assert_eq!(
            migrated.workspace.project.schema_version(),
            crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION
        );
        assert_eq!(migrated.workspace.project.revision().get(), 1);

        let migrated_json =
            serialize_project_file(&migrated).expect("migrated identity persists on save");
        let reloaded = load_project_text(&migrated_json, None).expect("migrated project reloads");
        assert_eq!(
            reloaded.workspace.project.id(),
            migrated.workspace.project.id()
        );
        assert_eq!(
            reloaded.workspace.project.revision(),
            migrated.workspace.project.revision()
        );
    }

    #[test]
    fn project_load_rejects_unsupported_descriptor_schema() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let project = ProjectFile::new(workspace, libraries);
        let mut value = serde_json::to_value(project).expect("project converts to JSON");
        value["workspace"]["project"]["schema_version"] =
            serde_json::Value::from(crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION + 1);
        let contents = serde_json::to_string_pretty(&value).expect("fixture serializes");

        let error = load_project_text(&contents, None)
            .expect_err("future project descriptor schema must fail closed");

        assert!(matches!(error, ProjectIoError::InvalidData(_)));
        assert!(error.to_string().contains("project schema version"));
    }

    #[test]
    fn project_text_load_rejects_missing_active_schematic_buffer() {
        let mut libraries = LibraryManager::with_primitives();
        let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let mut project = ProjectFile::new(workspace, libraries);
        let active_key = project.workspace.active_key();
        project.workspace.schematic_buffers.remove(&active_key);
        let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("missing active schematic buffer must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(&active_key),
            "error should name the missing buffer key"
        );
    }

    #[test]
    fn project_text_load_rejects_workspace_references_missing_from_libraries() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let ghost = CellViewRef::new("ghost", "amp", "schematic");
        let ghost_key = ghost.key();
        let schematic = workspace
            .schematic_buffers
            .values()
            .next()
            .cloned()
            .expect("default project has a schematic buffer");
        workspace.active_view = ghost.clone();
        workspace.open_views = vec![OpenCellView::new(ghost.clone(), ViewType::Schematic)];
        workspace.hierarchy_stack = vec![ghost.clone()];
        workspace.schematic_buffers.clear();
        workspace
            .schematic_buffers
            .insert(ghost_key.clone(), schematic);
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("workspace references absent from libraries must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(&ghost_key),
            "error should name the missing workspace reference"
        );
    }

    #[test]
    fn project_text_load_rejects_workspace_view_type_mismatch() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        let active = workspace.active_view.clone();
        workspace.open_views = vec![OpenCellView::new(active.clone(), ViewType::Symbol)];
        let active_key = active.key();
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("workspace view type mismatch must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(&active_key) && err.to_string().contains("view type"),
            "error should name the mismatched view reference"
        );
    }

    #[test]
    fn project_text_load_rejects_active_view_missing_from_open_views() {
        let mut libraries = LibraryManager::with_primitives();
        let mut workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
        assert!(libraries.create_view("user", "top", "symbol", ViewType::Symbol));
        let active = CellViewRef::new("user", "top", "symbol");
        workspace.active_view = active.clone();
        workspace.hierarchy_stack = vec![active.clone()];
        let project = ProjectFile::new(workspace, libraries);
        let json = serde_json::to_string_pretty(&project).expect("corrupt fixture serializes");

        let err = load_project_text(&json, None)
            .expect_err("active view absent from open views must fail load");

        assert!(matches!(err, ProjectIoError::InvalidData(_)));
        assert!(
            err.to_string().contains(&active.key()) && err.to_string().contains("open_views"),
            "error should name the missing active open view"
        );
    }

    #[test]
    fn project_text_load_rejects_library_tree_key_name_mismatch() {
        type ProjectJsonMutation = fn(&mut serde_json::Value, &CellViewRef);

        let cases: [(&str, ProjectJsonMutation); 3] = [
            (
                "library",
                |value: &mut serde_json::Value, active: &CellViewRef| {
                    value["libraries"]["libraries"][&active.library]["name"] =
                        serde_json::Value::String("ghost".to_string());
                },
            ),
            (
                "cell",
                |value: &mut serde_json::Value, active: &CellViewRef| {
                    value["libraries"]["libraries"][&active.library]["cells"][&active.cell]["name"] =
                        serde_json::Value::String("ghost".to_string());
                },
            ),
            (
                "view",
                |value: &mut serde_json::Value, active: &CellViewRef| {
                    value["libraries"]["libraries"][&active.library]["cells"][&active.cell]["views"]
                        [&active.view]["name"] = serde_json::Value::String("ghost".to_string());
                },
            ),
        ];

        for (case, mutate) in cases {
            let mut libraries = LibraryManager::with_primitives();
            let workspace = ProjectWorkspace::new_bootstrapped(&mut libraries);
            let active = workspace.active_view.clone();
            let project = ProjectFile::new(workspace, libraries);
            let mut value = serde_json::to_value(&project).expect("project converts to json value");
            mutate(&mut value, &active);
            let json = serde_json::to_string_pretty(&value).expect("corrupt fixture serializes");

            let err = load_project_text(&json, None)
                .expect_err("library tree key/name mismatches must fail load");

            assert!(matches!(err, ProjectIoError::InvalidData(_)));
            assert!(
                err.to_string().contains(case) && err.to_string().contains("map key"),
                "unexpected {case} mismatch error: {err}"
            );
        }
    }

    #[test]
    fn project_text_load_reports_parse_errors_without_filesystem() {
        let err = load_project_text("{not valid json", Some(Path::new("bad.rspiceproj")))
            .expect_err("invalid project text fails");

        assert!(matches!(err, ProjectIoError::ParseError(_)));
    }
}
