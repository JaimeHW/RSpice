//! What a prepared run was prepared from, so dispatch can tell whether it
//! still is.
//!
//! A Run press prepares its run on a worker from inputs frozen at the press,
//! has the engine build every deck, and retains the run once nothing refuses
//! it; Validate prepares a manual deck and retains it. Dispatch used to
//! prepare the whole run again from the live session and dispatch only if the
//! two digests agreed: a second preparation, in the frame that starts the
//! run, twelve seconds on a 14-family PDK design and five and a half on a
//! half-megabyte deck in a debug build. The preparation that authorized the
//! run is now the authority. Dispatch runs that snapshot, and in its frame
//! checks only that nothing the snapshot was prepared from has moved since;
//! when anything has, the run is refused and nothing is prepared or
//! dispatched.
//!
//! What a run is prepared from is the session state that
//! `run_preparation_inputs` lends; for a run set, the configured root's
//! projection; for a manual deck, the netlist document; and the host: the
//! model files an external library is sealed from, the files a deck's
//! includes splice in, the file a manual deck came from, and the data files a
//! file-backed source is routed to. [`RunInputIdentity`] names the session
//! part, in the cheapest exact form each input has, and only what the run's
//! kind reads; the host part is read again from what the preparation recorded
//! about it. Automation and tuning prepare in their own frame from the live
//! session, and retain the identity of the state they prepared from, so their
//! runs are held to the same check.
//!
//! Automation also publishes evidence from the run it dispatched, and only
//! while the session still holds what that run was prepared from. It used to
//! decide so by preparing the whole run again and comparing digests, seconds
//! of the frame on a large design, each frame it waited on the dispatch and
//! again when it published, and a second judgement of currency that could
//! disagree with the dispatch's. It now keeps [`PreparedRunSetInputs`], the
//! very inputs its run was retained with, past the dispatch that spends them,
//! and asks of them the question the dispatch asks.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use sha2::{Digest as _, Sha256};

use super::*;
use crate::product::ContentDigest;
use crate::simulation::execution::{PreparationError, PreparationStage, PreparedRunSnapshot};
use crate::simulation::plan::AnalysisDraft;
use crate::state::stimulus_library::definition::RetainedPwlFile;
use crate::state::{AnalysisResultFamilyMetadata, MonteCarloCheckpointEvidence};
use rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointValidationRef;
use rspice_results::result_import::ResultImportSource;
use rspice_results::transient_checkpoint::TransientCheckpointEvidence;
use rspice_simulation::monte_carlo_checkpoint::preparation::{
    SelectedCheckpoint, selected_checkpoints,
};
use rspice_simulation::netlist_gen::{NetlistSourceHost, TableRoute};

/// The status a run set's dispatch refused for moved inputs leaves, and the
/// start of what it says on the Console.
pub(crate) const DESIGN_CHANGED_SINCE_CHECK: &str =
    "The design changed after it was checked; run again";

/// The same, for a manual deck, whose check is its validation.
pub(crate) const NETLIST_CHANGED_SINCE_VALIDATION: &str =
    "The netlist changed after it was validated; validate it again";

/// State a refused dispatch: in full on the status bar, as the refusal of a
/// gesture, and in the run status as the refusal for moved inputs it is, or
/// as a blocked run.
pub(super) fn state_refused_dispatch(state: &mut AppState, error: &PreparationError) {
    state.prompt(error.to_string());
    state.simulation.execution.status =
        [DESIGN_CHANGED_SINCE_CHECK, NETLIST_CHANGED_SINCE_VALIDATION]
            .into_iter()
            .find(|status| error.message().starts_with(status))
            .unwrap_or("Run blocked")
            .to_owned();
}

/// The refusal of a retained run held without what it was prepared from.
pub(super) fn unaccounted_run() -> PreparationError {
    PreparationError::new(
        PreparationStage::Authorization,
        "The prepared run does not say what it was prepared from; run again",
    )
}

/// The identity of everything preparing a run reads from the session.
///
/// Captured in the frame whose state a run is prepared from, and compared,
/// input by input, with the live session in the frame that dispatches it.
/// Only what the run's kind reads is held, so an edit the run never reads
/// does not expire it. Each input is held in the cheapest exact form it has:
///
/// * A run set's design is the projection's own key: the content of every
///   cell view the configured root can reach, with the open schematic
///   overlaid, the configuration, design-management and connectivity
///   authorities, the cell library's revision and the project's. The
///   topology revisions ride beside it, because the snapshot records them and
///   the key does not.
/// * A manual deck's text is the snapshot's own, and is compared with it.
///   Beside it are held the document it was taken from, the file that
///   anchors its includes as the host resolves it, and what it reads of the
///   open schematic: the topology revision its run is recorded against and,
///   when it solves S-parameters, the RF ports its Touchstone export names.
/// * Inputs that compare by value are held by value. Most are shared or
///   small; the model catalogue is the largest, and comparing it costs a walk
///   over memory, never an encoding.
/// * The setup, the project descriptor and the design-check configuration
///   cannot be compared by value, and are digested through `Debug`, which
///   names every field. Their serialized form would not: the setup's drafts
///   skip an `initialized` flag their lowering reads.
/// * The selection, the active run and the retained runs are read only by a
///   run set's analyses that report on them: an operating point names the
///   selected devices, and may start from a retained solution or report
///   against the active run's violations; a Monte Carlo or transient that
///   resumes from a checkpoint reads the results that hold it. Those reads
///   are what is held, so selecting something or opening an older run while a
///   check runs does not expire a run that never reads it. Neither does a
///   run added to the history that changes nothing a checkpoint resumes
///   from: the run's own, which joins the history from its dispatch on,
///   among them.
///
/// Held on the heap: the controller holds one beside its retained run and
/// another beside a press's pipeline, and the application holds the
/// controller by value.
pub(super) struct RunInputIdentity(Box<CapturedInputs>);

/// What both kinds of run read, and what only this run's kind reads.
struct CapturedInputs {
    setup: ContentDigest,
    plan_payloads: Vec<rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord>,
    project: ContentDigest,
    models: rspice_model_library::ModelCatalog,
    resolutions: BTreeMap<String, rspice_model_library::ModelResolutionRecord>,
    registry: rspice_simulation::pdk::PdkTechnologyRegistry,
    layouts: BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>,
    project_sources: rspice_design::project_sources::ProjectSourceRegistry,
    max_parallel_workers: Option<usize>,
    library_folders: Vec<PathBuf>,
    compilation: Option<rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt>,
    reads: KindReads,
}

enum KindReads {
    RunSet(Box<RunSetReads>),
    ManualDeck(Box<ManualDeckReads>),
}

/// What only preparing a run set reads.
struct RunSetReads {
    projection: Option<rspice_design::projection::DesignProjectionKey>,
    topology: Vec<(String, u64)>,
    design_management: rspice_design_model::design_management::DesignManagementCatalog,
    configuration_sets: rspice_design::configuration_set::ConfigurationSetCatalog,
    stimulus_library: Option<rspice_design::stimulus_library::library::StimulusLibrary>,
    imported_checkpoints: rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    imported_transient_checkpoints:
        rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    operating_points: Vec<Result<AnalysisSpec, String>>,
    checkpoint_resumes: Vec<CheckpointResume>,
    display_waveform_cache_samples: usize,
    design_checks: ContentDigest,
}

/// What only preparing a manual deck reads, beside its text, which the
/// snapshot holds.
struct ManualDeckReads {
    descriptor: Option<rspice_design::owned_netlist::OwnedNetlistDescriptor>,
    generated_artifact: Option<rspice_design::netlist_document::GeneratedArtifact>,
    owned: bool,
    origin: Option<PathBuf>,
    origin_identity: Option<PathBuf>,
    topology_revision: u64,
    rf_ports: Option<Vec<rspice_design::rf_ports::RfPort>>,
}

/// The session's inputs, borrowed from the live session.
struct LiveInputs<'a> {
    setup: ContentDigest,
    plan_payloads: &'a [rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord],
    project: ContentDigest,
    models: &'a rspice_model_library::ModelCatalog,
    resolutions: &'a BTreeMap<String, rspice_model_library::ModelResolutionRecord>,
    registry: &'a rspice_simulation::pdk::PdkTechnologyRegistry,
    layouts: &'a BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>,
    project_sources: &'a rspice_design::project_sources::ProjectSourceRegistry,
    max_parallel_workers: Option<usize>,
    library_folders: &'a [PathBuf],
    compilation: Option<&'a rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt>,
    design_management: &'a rspice_design_model::design_management::DesignManagementCatalog,
    configuration_sets: &'a rspice_design::configuration_set::ConfigurationSetCatalog,
    stimulus_library: Option<&'a rspice_design::stimulus_library::library::StimulusLibrary>,
    imported_checkpoints: &'a rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    imported_transient_checkpoints:
        &'a rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    display_waveform_cache_samples: usize,
    design_checks: rspice_design::drc::DrcConfig,
    schematic: &'a rspice_design::schematic::document::SchematicDocument,
    schematic_topology_revision: u64,
}

impl<'a> LiveInputs<'a> {
    /// Read every input from the session, through the very view preparation
    /// borrows. The view is taken apart field by field, so an input added to
    /// it later does not compile until it is given an identity here.
    fn of(state: &'a AppState) -> Self {
        let crate::simulation::execution::preparation::RunPreparationInputs {
            analysis,
            technology,
            // The cell library and the file the open schematic was loaded
            // from are inputs of a run set's projection, and its key names
            // them. A manual deck reads neither.
            libraries: _,
            project_sources,
            design_management,
            configuration_sets,
            source_data,
            schematic_file: _,
            schematic_topology_revision,
            imported_checkpoints,
            imported_transient_checkpoints,
            display_waveform_cache_samples,
            max_parallel_workers,
            library_folders,
            compilation,
            design_checks,
        } = super::prepared_run::run_preparation_inputs(state);
        // A run set reads the selection, the runs and the project's revision
        // through its operating points and checkpoints, and holds what they
        // read; a manual deck reads none of them.
        let rspice_simulation::analysis_preparation::AnalysisInputs {
            sim_setup,
            schematic,
            selected_components: _,
            selection_occurrence: _,
            runs: _,
            active_run: _,
            project_revision: _,
            plan_payloads,
        } = analysis;
        let rspice_simulation::project_technology::ProjectTechnologyInputs {
            project,
            sim_setup: _,
            models,
            resolutions,
            registry,
            layouts,
        } = technology;
        // The data root is the project's; the files are the host's, and
        // their answers are recorded by the preparation itself.
        let rspice_simulation::netlist_gen::NetlistSourceData {
            files: _,
            data_root: _,
            stimulus_library,
        } = source_data;

        Self {
            setup: debug_digest("rspice.run-input.setup/v1", sim_setup),
            plan_payloads,
            project: debug_digest("rspice.run-input.project/v1", project),
            models,
            resolutions: resolutions.as_map(),
            registry,
            layouts,
            project_sources,
            max_parallel_workers,
            library_folders,
            compilation,
            design_management,
            configuration_sets,
            stimulus_library,
            imported_checkpoints,
            imported_transient_checkpoints,
            display_waveform_cache_samples,
            design_checks,
            schematic,
            schematic_topology_revision,
        }
    }
}

impl RunInputIdentity {
    /// The identity of what preparing a run set reads, as the session holds
    /// it now.
    pub(super) fn of_run_set(state: &AppState) -> Self {
        let live = LiveInputs::of(state);
        let reads = KindReads::RunSet(Box::new(RunSetReads {
            projection: projection_key(state),
            topology: topology_revisions(state, live.schematic_topology_revision),
            design_management: live.design_management.clone(),
            configuration_sets: live.configuration_sets.clone(),
            stimulus_library: live.stimulus_library.cloned(),
            imported_checkpoints: live.imported_checkpoints.clone(),
            imported_transient_checkpoints: live.imported_transient_checkpoints.clone(),
            operating_points: operating_point_reads(state),
            checkpoint_resumes: checkpoint_resume_reads(state, &live)
                .into_iter()
                .map(CheckpointResume::held)
                .collect(),
            display_waveform_cache_samples: live.display_waveform_cache_samples,
            design_checks: design_check_digest(&live.design_checks),
        }));
        Self::holding(live, reads)
    }

    /// The identity of what preparing the manual deck `snapshot` read, as the
    /// session holds it now: in the frame that prepared it.
    pub(super) fn of_manual_deck(
        state: &AppState,
        snapshot: &PreparedRunSnapshot,
    ) -> Result<Self, PreparationError> {
        let live = LiveInputs::of(state);
        let deck = super::prepared_run::manual_source_inputs(state)?;
        let reads = KindReads::ManualDeck(Box::new(ManualDeckReads {
            descriptor: deck.descriptor.cloned(),
            generated_artifact: deck.generated_artifact.cloned(),
            owned: deck.owned,
            origin: deck.origin.map(Path::to_path_buf),
            origin_identity: origin_identity(deck.origin),
            topology_revision: live.schematic_topology_revision,
            rf_ports: snapshot
                .solves_s_parameters()
                .then(|| rspice_design::rf_ports::rf_ports(live.schematic)),
        }));
        Ok(Self::holding(live, reads))
    }

    /// `reads`, beside what both kinds of run read of `live`.
    fn holding(live: LiveInputs<'_>, reads: KindReads) -> Self {
        Self(Box::new(CapturedInputs {
            setup: live.setup,
            plan_payloads: live.plan_payloads.to_vec(),
            project: live.project,
            models: live.models.clone(),
            resolutions: live.resolutions.clone(),
            registry: live.registry.clone(),
            layouts: live.layouts.clone(),
            project_sources: live.project_sources.clone(),
            max_parallel_workers: live.max_parallel_workers,
            library_folders: live.library_folders.to_vec(),
            compilation: live.compilation.cloned(),
            reads,
        }))
    }

    /// Whether `snapshot` is of the kind whose reads are held.
    fn describes(&self, snapshot: &PreparedRunSnapshot) -> bool {
        matches!(
            (&self.0.reads, snapshot.intent()),
            (KindReads::RunSet(_), SimulationRunIntent::SimulateRunSet)
                | (KindReads::ManualDeck(_), SimulationRunIntent::ManualDeck)
        )
    }

    /// What a dispatch of this kind of run says, first, when it is refused
    /// for moved inputs.
    fn refusal(&self) -> &'static str {
        match self.0.reads {
            KindReads::RunSet(_) => DESIGN_CHANGED_SINCE_CHECK,
            KindReads::ManualDeck(_) => NETLIST_CHANGED_SINCE_VALIDATION,
        }
    }

    /// The first input the session no longer holds as it was captured, named
    /// for the reader, or `None` when every one is unchanged.
    fn moved_in(&self, state: &AppState, snapshot: &PreparedRunSnapshot) -> Option<&'static str> {
        self.first_moved(state, |live| match &self.0.reads {
            KindReads::RunSet(reads) => reads.compared(state, live),
            KindReads::ManualDeck(reads) => reads.compared(state, live, snapshot),
        })
    }

    /// The first input a run set's preparation froze that the session no
    /// longer holds as it was, before any run is prepared from them: what a
    /// Run press asks of the pass in flight, by the comparison its dispatch
    /// will make. A manual deck's identity is taken once its deck is
    /// prepared, so it names nothing here.
    pub(super) fn run_set_moved_in(&self, state: &AppState) -> Option<&'static str> {
        let KindReads::RunSet(reads) = &self.0.reads else {
            return None;
        };
        self.first_moved(state, |live| reads.compared(state, live))
    }

    fn first_moved(
        &self,
        state: &AppState,
        kind_reads: impl FnOnce(&LiveInputs<'_>) -> Compared,
    ) -> Option<&'static str> {
        let captured = &*self.0;
        let live = LiveInputs::of(state);
        let compilation_unchanged = match (&captured.compilation, live.compilation) {
            (None, None) => true,
            (Some(held), Some(now)) => {
                held.token() == now.token() && Arc::ptr_eq(held.report(), now.report())
            }
            _ => false,
        };
        let shared = [
            (captured.setup == live.setup, "the simulation setup"),
            (
                captured.plan_payloads == live.plan_payloads,
                "the plan's variables, outputs or specifications",
            ),
            (captured.project == live.project, "the project"),
            (
                captured.models == *live.models && captured.resolutions == *live.resolutions,
                "the model library",
            ),
            (captured.registry == *live.registry, "the PDK"),
            (captured.layouts == *live.layouts, "the physical layout"),
            (
                captured.project_sources == *live.project_sources,
                "the project sources",
            ),
            (
                captured.max_parallel_workers == live.max_parallel_workers,
                "the solver limits",
            ),
            (
                captured.library_folders == live.library_folders,
                "the library folders",
            ),
            (compilation_unchanged, "the Verilog-A compilation"),
        ];
        let (first, last) = kind_reads(&live);
        first
            .into_iter()
            .chain(shared)
            .chain(last)
            .find_map(|(unchanged, input)| (!unchanged).then_some(input))
    }
}

/// Whether each input one kind of run reads is unchanged, with its name:
/// those named before the inputs both kinds read, and those named after.
type Compared = (Vec<(bool, &'static str)>, Vec<(bool, &'static str)>);

impl RunSetReads {
    /// A design whose key could not be taken is never confirmed unchanged.
    fn compared(&self, state: &AppState, live: &LiveInputs<'_>) -> Compared {
        let first = vec![
            (
                self.projection.is_some() && self.projection == projection_key(state),
                "the design",
            ),
            (
                self.topology == topology_revisions(state, live.schematic_topology_revision),
                "the design",
            ),
        ];
        let last = vec![
            (
                self.design_management == *live.design_management
                    && self.configuration_sets == *live.configuration_sets,
                "the design configuration",
            ),
            (
                self.stimulus_library.as_ref() == live.stimulus_library,
                "the stimulus library",
            ),
            (
                self.imported_checkpoints == *live.imported_checkpoints
                    && self.imported_transient_checkpoints == *live.imported_transient_checkpoints,
                "the imported checkpoints",
            ),
            (
                self.operating_points == operating_point_reads(state),
                "the devices or results the operating point reports",
            ),
            (
                CheckpointResume::all_resume_as(
                    &self.checkpoint_resumes,
                    &checkpoint_resume_reads(state, live),
                ),
                "the retained runs a checkpoint resumes from",
            ),
            (
                self.display_waveform_cache_samples == live.display_waveform_cache_samples,
                "the solver limits",
            ),
            (
                self.design_checks == design_check_digest(&live.design_checks),
                "the design-check rules",
            ),
        ];
        (first, last)
    }
}

impl ManualDeckReads {
    /// The deck's text is compared with the text `snapshot` was prepared
    /// from.
    fn compared(
        &self,
        state: &AppState,
        live: &LiveInputs<'_>,
        snapshot: &PreparedRunSnapshot,
    ) -> Compared {
        let document_unchanged =
            super::prepared_run::manual_source_inputs(state).is_ok_and(|deck| {
                Some(deck.source) == snapshot.manual_source()
                    && deck.descriptor == self.descriptor.as_ref()
                    && deck.generated_artifact == self.generated_artifact.as_ref()
                    && deck.owned == self.owned
                    && deck.origin == self.origin.as_deref()
            });
        let first = vec![
            (document_unchanged, "the netlist"),
            (
                self.origin_identity == origin_identity(self.origin.as_deref()),
                "the netlist's file",
            ),
        ];
        let last = vec![
            (
                self.topology_revision == live.schematic_topology_revision,
                "the schematic",
            ),
            (
                self.rf_ports.as_ref().is_none_or(|ports| {
                    *ports == rspice_design::rf_ports::rf_ports(live.schematic)
                }),
                "the schematic's RF ports",
            ),
        ];
        (first, last)
    }
}

/// The configured root's projection key; `None` when it cannot be taken.
fn projection_key(state: &AppState) -> Option<rspice_design::projection::DesignProjectionKey> {
    state.workspace.design_projection_key(
        &state.library_manager,
        &state.workspace.content.active_view,
        &state.schematic,
    )
}

/// The topology revision of every open schematic buffer and, last, of the
/// open schematic: a run set's snapshot records them, and the projection key
/// does not.
fn topology_revisions(state: &AppState, open: u64) -> Vec<(String, u64)> {
    let mut topology = state
        .workspace
        .content
        .schematic_buffers
        .iter()
        .map(|(key, schematic)| (key.clone(), schematic.topology_version()))
        .collect::<Vec<_>>();
    topology.sort();
    topology.push((String::new(), open));
    topology
}

fn design_check_digest(config: &rspice_design::drc::DrcConfig) -> ContentDigest {
    debug_digest("rspice.run-input.design-checks/v1", config)
}

/// What preparing a manual deck resolves its file to: the path its receipt
/// names and its includes are searched from.
fn origin_identity(origin: Option<&Path>) -> Option<PathBuf> {
    origin.and_then(|origin| {
        rspice_simulation::netlist_preparation::dependencies::absolute_source_identity(origin).ok()
    })
}

/// What every operating point in the plan reads of the selection and the
/// retained runs, as it reads them: the spec its lowering produces.
///
/// Lowering an operating point is the only reader of the selection and of
/// the active run, and one of the readers of the retained runs. Everything
/// else it reads is setup, held by the setup's own identity, so two equal
/// lowerings under one setup read the same devices and the same evidence.
fn operating_point_reads(state: &AppState) -> Vec<Result<AnalysisSpec, String>> {
    let Some(plan) = state.sim_setup.analysis_plan.as_ref() else {
        return Vec::new();
    };
    let inputs = analysis_inputs(state);
    plan.instances()
        .iter()
        .filter_map(|instance| match instance.draft() {
            AnalysisDraft::OperatingPoint(draft) => Some(
                rspice_simulation::analysis_preparation::build_op_spec(&inputs, draft),
            ),
            _ => None,
        })
        .collect()
}

/// What every analysis in the plan that resumes from checkpoints resumes
/// from, in plan order, read as its preparation reads it.
///
/// Preparation looks retained results up only for the checkpoints a draft
/// resumes from, and asks of them only what is read here: for a transient,
/// the images its selection resolves, each checked against every result that
/// holds it; for a Monte Carlo, each checkpoint beside the result its
/// selection takes it from, which preparation checks it against. Capturing
/// checkpoints reads no retained result. A run added to the history moves
/// nothing here unless it holds a checkpoint a draft resumes from, so a run
/// prepared to resume is not expired by the run its own dispatch starts,
/// though that run captures checkpoints of its own.
fn checkpoint_resume_reads<'a>(state: &'a AppState, live: &LiveInputs<'a>) -> Vec<ResumeRead<'a>> {
    let Some(plan) = state.sim_setup.analysis_plan.as_ref() else {
        return Vec::new();
    };
    // Every retained result, newest run first, as preparation offers them.
    let runs = &state.simulation.retained.runs;
    let results = || {
        runs.iter()
            .flat_map(|run| run.analyses.iter().map(|analysis| &analysis.data))
    };
    plan.instances()
        .iter()
        .filter_map(|instance| match instance.draft() {
            AnalysisDraft::Transient(draft) => {
                let checkpoint = draft
                    .checkpoint
                    .as_ref()
                    .filter(|checkpoint| !checkpoint.resume.is_empty())?;
                Some(ResumeRead::Transient(
                    rspice_simulation::transient_checkpoint::selected_images(
                        results(),
                        live.imported_transient_checkpoints,
                        Some(checkpoint),
                    )
                    .map(|images| {
                        images
                            .iter()
                            .map(TransientCheckpointEvidence::digest)
                            .collect()
                    }),
                ))
            }
            // A draft that does not lower prepares no run, and reads no
            // retained result to find so.
            AnalysisDraft::MonteCarlo(draft) => {
                let checkpoint = draft
                    .to_config()
                    .ok()?
                    .checkpoint
                    .filter(|checkpoint| !checkpoint.resume.is_empty())?;
                Some(ResumeRead::MonteCarlo(selected_checkpoints(
                    results(),
                    live.imported_checkpoints,
                    Some(&checkpoint),
                )))
            }
            _ => None,
        })
        .collect()
}

/// What one analysis that resumes from checkpoints resumes from, read from
/// the session.
enum ResumeRead<'a> {
    /// A transient's images, by digest, or why they do not resolve.
    Transient(Result<Vec<ContentDigest>, String>),
    /// A Monte Carlo's checkpoints, each beside the result it is taken from,
    /// or why they do not resolve.
    MonteCarlo(Result<Vec<SelectedCheckpoint<'a, WaveformData>>, String>),
}

/// What one analysis that resumes from checkpoints resumed from, held past
/// the frame it was read in.
enum CheckpointResume {
    Transient(Result<Vec<ContentDigest>, String>),
    MonteCarlo(Result<Vec<MonteCarloResume>, String>),
}

impl CheckpointResume {
    fn held(read: ResumeRead<'_>) -> Self {
        match read {
            ResumeRead::Transient(images) => Self::Transient(images),
            ResumeRead::MonteCarlo(selected) => Self::MonteCarlo(selected.map(|selected| {
                selected
                    .into_iter()
                    .map(|(holder, image)| MonteCarloResume {
                        image: image.clone(),
                        holder: holder.map(|holder| CheckpointHolder::of(holder.into())),
                    })
                    .collect()
            })),
        }
    }

    /// Whether every analysis `live` reads resumes from what the one held
    /// beside it resumed from.
    fn all_resume_as(held: &[Self], live: &[ResumeRead<'_>]) -> bool {
        held.len() == live.len()
            && held
                .iter()
                .zip(live)
                .all(|(held, live)| held.resumes_as(live))
    }

    fn resumes_as(&self, live: &ResumeRead<'_>) -> bool {
        match (self, live) {
            (Self::Transient(held), ResumeRead::Transient(live)) => held == live,
            (Self::MonteCarlo(Ok(held)), ResumeRead::MonteCarlo(Ok(live))) => {
                held.len() == live.len()
                    && held
                        .iter()
                        .zip(live)
                        .all(|(held, &live)| held.resumes_as(live))
            }
            (Self::MonteCarlo(Err(held)), ResumeRead::MonteCarlo(Err(live))) => held == live,
            _ => false,
        }
    }
}

/// A checkpoint a Monte Carlo resumed from, and what checking it read of the
/// result it was taken from; `None` when it was taken from the imported
/// library, where it is not checked.
struct MonteCarloResume {
    image: MonteCarloCheckpointEvidence,
    holder: Option<CheckpointHolder>,
}

impl MonteCarloResume {
    /// Whether `live`, the same checkpoint selected again, resumes alike: the
    /// same image, checked alike against what it is taken from.
    ///
    /// Mostly it is taken from the same result, read alike, and nothing is
    /// checked. Taken from another result, as when a newer run holds the same
    /// image, or from the library instead of a result, it resumes alike when
    /// it checks alike against each, which reads the image.
    fn resumes_as(&self, (holder, image): SelectedCheckpoint<'_, WaveformData>) -> bool {
        if self.image.digest() != image.digest() {
            return false;
        }
        match (&self.holder, holder) {
            (None, None) => true,
            (Some(held), Some(live)) if held.reads_as(live.into()) => true,
            (held, live) => {
                held.as_ref()
                    .map_or(Ok(()), |held| self.image.validate_for(held.read()))
                    == live.map_or(Ok(()), |live| image.validate_for(live.into()))
            }
        }
    }
}

/// What checking a Monte Carlo checkpoint reads of the retained result it is
/// taken from, held by value.
struct CheckpointHolder {
    analysis_type: AnalysisType,
    provenance: Option<AnalysisResultProvenance>,
    import_source: Option<ResultImportSource>,
    success: bool,
    family_metadata: Option<AnalysisResultFamilyMetadata>,
}

impl CheckpointHolder {
    fn of(result: MonteCarloCheckpointValidationRef<'_>) -> Self {
        let MonteCarloCheckpointValidationRef {
            analysis_type,
            provenance,
            import_source,
            success,
            family_metadata,
        } = result;
        Self {
            analysis_type,
            provenance: provenance.cloned(),
            import_source: import_source.cloned(),
            success,
            family_metadata: family_metadata.cloned(),
        }
    }

    fn read(&self) -> MonteCarloCheckpointValidationRef<'_> {
        MonteCarloCheckpointValidationRef {
            analysis_type: self.analysis_type,
            provenance: self.provenance.as_ref(),
            import_source: self.import_source.as_ref(),
            success: self.success,
            family_metadata: self.family_metadata.as_ref(),
        }
    }

    /// Whether checking a checkpoint against `result` reads what it read of
    /// this one.
    fn reads_as(&self, result: MonteCarloCheckpointValidationRef<'_>) -> bool {
        let MonteCarloCheckpointValidationRef {
            analysis_type,
            provenance,
            import_source,
            success,
            family_metadata,
        } = result;
        self.analysis_type == analysis_type
            && self.provenance.as_ref() == provenance
            && self.import_source.as_ref() == import_source
            && self.success == success
            && self.family_metadata.as_ref() == family_metadata
    }
}

/// SHA-256 over `value`'s `Debug` text, streamed into the hash rather than
/// held. `Debug` prints every field, a float as the shortest text that reads
/// back as the same value, and a string escaped, so two values that print
/// alike hold alike.
///
/// The converse holds only for a value whose every collection is ordered: a
/// hashed map or set prints in its hasher's order, which differs between two
/// equal collections built apart, and a run would then be refused at dispatch
/// with nothing edited. Each value digested here holds ordered collections
/// only.
fn debug_digest(domain: &str, value: &impl std::fmt::Debug) -> ContentDigest {
    struct Sink(Sha256);

    impl std::fmt::Write for Sink {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            self.0.update(text.as_bytes());
            Ok(())
        }
    }

    let mut sink = Sink(Sha256::new());
    sink.0.update(domain.as_bytes());
    sink.0.update([0]);
    // Writing into a hash cannot fail.
    let _ = write!(sink, "{value:?}");
    ContentDigest::from_bytes(sink.0.finalize().into())
}

/// The data-file host a run is prepared against, recording what it answers.
///
/// Generating the deck asks the host where each file-backed source's table
/// is read from: whether the named file is there, whether it is the retained
/// table, whether the engine can load it. The answers decide the path the
/// deck names. Recorded, they can be asked again at dispatch.
#[derive(Default)]
pub(super) struct RecordingSourceFiles {
    answers: Mutex<Vec<SourceFileAnswer>>,
}

/// One question the data-file host was asked, and what it answered.
#[derive(Debug, Clone, PartialEq)]
enum SourceFileAnswer {
    Route {
        stored: String,
        data_root: Option<PathBuf>,
        table: Option<RetainedPwlFile>,
        answer: (TableRoute, Option<String>),
    },
    IsFile {
        path: String,
        answer: Result<bool, std::io::ErrorKind>,
    },
    EngineRefusal {
        path: String,
        answer: Option<String>,
    },
    NamedFileMatches {
        path: String,
        table: RetainedPwlFile,
        answer: Option<bool>,
    },
}

impl SourceFileAnswer {
    /// Ask the question again, of the host as it is now.
    fn asked_again(&self) -> Self {
        let host = crate::simulation::table_route::SourceFiles;
        match self {
            Self::Route {
                stored,
                data_root,
                table,
                ..
            } => Self::Route {
                stored: stored.clone(),
                data_root: data_root.clone(),
                table: table.clone(),
                answer: host.route_retaining(stored, data_root.as_deref(), table.as_ref()),
            },
            Self::IsFile { path, .. } => Self::IsFile {
                path: path.clone(),
                answer: host.is_file(path).map_err(|error| error.kind()),
            },
            Self::EngineRefusal { path, .. } => Self::EngineRefusal {
                path: path.clone(),
                answer: host.engine_refusal(path),
            },
            Self::NamedFileMatches { path, table, .. } => Self::NamedFileMatches {
                path: path.clone(),
                table: table.clone(),
                answer: host.named_file_matches(path, table),
            },
        }
    }

    fn path(&self) -> &str {
        match self {
            Self::Route { stored, .. } => stored,
            Self::IsFile { path, .. }
            | Self::EngineRefusal { path, .. }
            | Self::NamedFileMatches { path, .. } => path,
        }
    }
}

impl RecordingSourceFiles {
    fn record(&self, answer: SourceFileAnswer) {
        if let Ok(mut answers) = self.answers.lock() {
            answers.push(answer);
        }
    }

    /// What the host answered, in the order it was asked.
    pub(super) fn into_answers(self) -> SourceFileAnswers {
        SourceFileAnswers(
            self.answers
                .into_inner()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        )
    }
}

impl NetlistSourceHost for RecordingSourceFiles {
    fn route_retaining(
        &self,
        stored: &str,
        data_root: Option<&Path>,
        table: Option<&RetainedPwlFile>,
    ) -> (TableRoute, Option<String>) {
        let answer =
            crate::simulation::table_route::SourceFiles.route_retaining(stored, data_root, table);
        self.record(SourceFileAnswer::Route {
            stored: stored.to_owned(),
            data_root: data_root.map(Path::to_path_buf),
            table: table.cloned(),
            answer: answer.clone(),
        });
        answer
    }

    fn is_file(&self, path: &str) -> std::io::Result<bool> {
        let answer = crate::simulation::table_route::SourceFiles.is_file(path);
        self.record(SourceFileAnswer::IsFile {
            path: path.to_owned(),
            answer: answer.as_ref().copied().map_err(std::io::Error::kind),
        });
        answer
    }

    fn engine_refusal(&self, path: &str) -> Option<String> {
        let answer = crate::simulation::table_route::SourceFiles.engine_refusal(path);
        self.record(SourceFileAnswer::EngineRefusal {
            path: path.to_owned(),
            answer: answer.clone(),
        });
        answer
    }

    fn named_file_matches(&self, path: &str, table: &RetainedPwlFile) -> Option<bool> {
        let answer = crate::simulation::table_route::SourceFiles.named_file_matches(path, table);
        self.record(SourceFileAnswer::NamedFileMatches {
            path: path.to_owned(),
            table: table.clone(),
            answer,
        });
        answer
    }
}

/// What the data-file host answered while a run was prepared.
#[derive(Debug, Default)]
pub(super) struct SourceFileAnswers(Vec<SourceFileAnswer>);

impl SourceFileAnswers {
    /// The first data file the host now answers for differently.
    fn moved(&self) -> Option<&str> {
        self.0
            .iter()
            .find(|answer| answer.asked_again() != **answer)
            .map(SourceFileAnswer::path)
    }
}

/// What a retained run was prepared from, held beside it until it is
/// dispatched or dropped, and by a workflow that keeps it longer; see
/// [`PreparedRunSetInputs`].
pub(super) struct RetainedRunInputs {
    pub(super) identity: RunInputIdentity,
    pub(super) source_files: SourceFileAnswers,
}

impl RetainedRunInputs {
    /// Refuse unless nothing `snapshot` was prepared from has moved: the
    /// session's inputs, the external model files its seal read, the files
    /// its includes spliced in, and the data files its sources were routed
    /// to. Nothing is prepared.
    ///
    /// A model file that no longer seals is refused at the stage that seals
    /// it, as preparing the run again would refuse it; anything else that
    /// moved leaves a run whose authorization has expired.
    pub(super) fn still_current(
        &self,
        state: &AppState,
        snapshot: &PreparedRunSnapshot,
    ) -> Result<(), PreparationError> {
        if !self.identity.describes(snapshot) {
            return Err(unaccounted_run());
        }
        self.refused_if_moved(
            state,
            self.identity.moved_in(state, snapshot),
            snapshot.sealed_source_dependencies(),
        )
    }

    /// Refuse when `moved` names a session input that moved, or when a host
    /// file the run was prepared from has: the external model files its seal
    /// read, `dependencies`, the files its includes spliced in, and the data
    /// files its sources were routed to.
    fn refused_if_moved(
        &self,
        state: &AppState,
        moved: Option<&'static str>,
        dependencies: &[rspice_core::netlist::ResolvedIncludeDependency],
    ) -> Result<(), PreparationError> {
        let refused = self.identity.refusal();
        let refuse = |stage: PreparationStage, what: &str| {
            Err(PreparationError::new(stage, format!("{refused} · {what}")))
        };
        if let Some(input) = moved {
            return refuse(PreparationStage::Authorization, &format!("{input} changed"));
        }
        if let Err(error) = rspice_simulation::model_sources::verify_plan_external_model_sources(
            state.model_library_manager.catalog(),
            &state.sim_setup.model_bindings,
        ) {
            return refuse(PreparationStage::ModelBindings, &error);
        }
        if let Err(error) =
            rspice_simulation::netlist_preparation::dependencies::verify_host_dependencies(
                dependencies,
            )
        {
            return refuse(PreparationStage::Authorization, &error);
        }
        if let Some(path) = self.source_files.moved() {
            return refuse(
                PreparationStage::Authorization,
                &format!("the data file '{path}' changed"),
            );
        }
        Ok(())
    }
}

/// What a run set was prepared from, kept past its dispatch by the workflow
/// that publishes the run's results.
///
/// Dispatch spends the retained run and what it was prepared from. Automation
/// publishes the evidence of the run it dispatched only while the session
/// still holds what that run was prepared from: it asks on each frame it
/// waits for the dispatch, and again when it publishes. These are the inputs
/// the run was retained with, shared, not a second capture, and the question
/// asked of them is the one its dispatch asks, refused in the same words. The
/// two cannot disagree, and neither prepares anything.
#[derive(Clone)]
pub(crate) struct PreparedRunSetInputs {
    inputs: Arc<RetainedRunInputs>,
    /// The host sources the run's includes were read from, as they were read.
    sealed_source_dependencies: Arc<[rspice_core::netlist::ResolvedIncludeDependency]>,
}

impl PreparedRunSetInputs {
    /// What the run set `snapshot` was retained with; `None` when `inputs`
    /// are not a run set's, or not `snapshot`'s.
    pub(super) fn of(
        inputs: &Arc<RetainedRunInputs>,
        snapshot: &PreparedRunSnapshot,
    ) -> Option<Self> {
        (snapshot.intent() == SimulationRunIntent::SimulateRunSet
            && inputs.identity.describes(snapshot))
        .then(|| Self {
            inputs: Arc::clone(inputs),
            sealed_source_dependencies: snapshot.sealed_source_dependencies().into(),
        })
    }

    /// Refuse unless nothing the run set was prepared from has moved, as its
    /// dispatch refuses it, whether or not it has been dispatched since.
    /// Nothing is prepared.
    pub(crate) fn still_current(&self, state: &AppState) -> Result<(), PreparationError> {
        self.inputs.refused_if_moved(
            state,
            self.inputs.identity.run_set_moved_in(state),
            &self.sealed_source_dependencies,
        )
    }
}

impl std::fmt::Debug for PreparedRunSetInputs {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedRunSetInputs")
            .field(
                "sealed_source_dependencies",
                &self.sealed_source_dependencies.len(),
            )
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
impl PreparedRunSetInputs {
    /// What a run set prepared from `state` would be retained with, for a
    /// test that builds a workflow's dispatch record and prepares no run. It
    /// holds no host file.
    pub(crate) fn of_unprepared(state: &AppState) -> Self {
        Self {
            inputs: Arc::new(RetainedRunInputs {
                identity: RunInputIdentity::of_run_set(state),
                source_files: SourceFileAnswers::default(),
            }),
            sealed_source_dependencies: Arc::from([]),
        }
    }
}

#[cfg(test)]
mod tests;
