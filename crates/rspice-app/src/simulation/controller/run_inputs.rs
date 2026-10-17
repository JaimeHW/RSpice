//! What a prepared run was prepared from, so dispatch can tell whether it
//! still is.
//!
//! A Run press prepares its run on a worker from inputs frozen at the press,
//! has the engine build every deck, and retains the run once nothing refuses
//! it. Dispatch used to prepare the whole run again from the live session and
//! dispatch only if the two digests agreed: a second preparation, in the frame
//! that starts the run, twelve seconds on a 14-family PDK design in a debug
//! build. The preparation the press authorized is now the authority. Dispatch
//! runs that snapshot, and in its frame checks only that nothing the snapshot
//! was prepared from has moved since the inputs were frozen; when anything has,
//! the run is refused and nothing is prepared or dispatched.
//!
//! What a run is prepared from is the session state that
//! `run_preparation_inputs` lends, the configured root's projection, and the
//! host: the model files an external library is sealed from, the files a
//! deck's includes splice in, and the data files a file-backed source is
//! routed to. [`RunInputIdentity`] names the session part, in the cheapest
//! exact form each input has; the host part is read again from what the
//! preparation recorded about it. Automation and tuning prepare in their own
//! frame from the live session, and retain the identity of the state they
//! prepared from, so their runs are held to the same check.

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
use rspice_simulation::netlist_gen::{NetlistSourceHost, TableRoute};

/// The status a dispatch refused for moved inputs leaves, and the start of
/// what it says on the Console.
pub(crate) const DESIGN_CHANGED_SINCE_CHECK: &str =
    "The design changed after it was checked; run again";

/// The identity of everything preparing the run set reads from the session.
///
/// Captured in the frame whose state a run is prepared from, and compared,
/// input by input, with the live session in the frame that dispatches it.
/// Each input is held in the cheapest exact form it has:
///
/// * The design is the projection's own key: the content of every cell view
///   the configured root can reach, with the open schematic overlaid, the
///   configuration, design-management and connectivity authorities, the cell
///   library's revision and the project's. The topology revisions ride beside
///   it, because the snapshot records them and the key does not.
/// * Inputs that compare by value are held by value. Most are shared or
///   small; the model catalogue is the largest, and comparing it costs a walk
///   over memory, never an encoding.
/// * The setup, the project descriptor and the design-check configuration
///   cannot be compared by value, and are digested through `Debug`, which
///   names every field. Their serialized form would not: the setup's drafts
///   skip an `initialized` flag their lowering reads.
/// * The selection, the active run and the retained runs are read only by the
///   analyses that report on them: an operating point names the selected
///   devices, and may start from a retained solution or report against the
///   active run's violations; a Monte Carlo or transient that resumes from a
///   checkpoint reads the runs that hold it. Those reads are what is held, so
///   selecting something or opening an older run while a check runs does not
///   expire a run that never reads it.
///
/// Held on the heap: the controller holds one beside its retained run and
/// another beside a press's pipeline, and the application holds the
/// controller by value.
pub(super) struct RunInputIdentity(Box<CapturedInputs>);

struct CapturedInputs {
    projection: Option<rspice_design::projection::DesignProjectionKey>,
    topology: Vec<(String, u64)>,
    setup: ContentDigest,
    plan_payloads: Vec<rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord>,
    project: ContentDigest,
    models: rspice_model_library::ModelCatalog,
    resolutions: BTreeMap<String, rspice_model_library::ModelResolutionRecord>,
    registry: rspice_simulation::pdk::PdkTechnologyRegistry,
    layouts: BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>,
    project_sources: rspice_design::project_sources::ProjectSourceRegistry,
    design_management: rspice_design_model::design_management::DesignManagementCatalog,
    configuration_sets: rspice_design::configuration_set::ConfigurationSetCatalog,
    stimulus_library: Option<rspice_design::stimulus_library::library::StimulusLibrary>,
    imported_checkpoints: rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    imported_transient_checkpoints:
        rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    operating_points: Vec<Result<AnalysisSpec, String>>,
    checkpoint_runs: Option<rspice_results::run_history::RunHistoryRevision>,
    display_waveform_cache_samples: usize,
    max_parallel_workers: Option<usize>,
    compilation: Option<rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt>,
    design_checks: ContentDigest,
}

/// The same inputs, borrowed from the live session.
struct LiveInputs<'a> {
    projection: Option<rspice_design::projection::DesignProjectionKey>,
    topology: Vec<(String, u64)>,
    setup: ContentDigest,
    plan_payloads: &'a [rspice_simulation_contract::plan_payload::SimulationPlanPayloadRecord],
    project: ContentDigest,
    models: &'a rspice_model_library::ModelCatalog,
    resolutions: &'a BTreeMap<String, rspice_model_library::ModelResolutionRecord>,
    registry: &'a rspice_simulation::pdk::PdkTechnologyRegistry,
    layouts: &'a BTreeMap<String, rspice_design::physical_layout::PhysicalLayoutDocument>,
    project_sources: &'a rspice_design::project_sources::ProjectSourceRegistry,
    design_management: &'a rspice_design_model::design_management::DesignManagementCatalog,
    configuration_sets: &'a rspice_design::configuration_set::ConfigurationSetCatalog,
    stimulus_library: Option<&'a rspice_design::stimulus_library::library::StimulusLibrary>,
    imported_checkpoints: &'a rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    imported_transient_checkpoints:
        &'a rspice_results::transient_checkpoint::TransientCheckpointLibrary,
    operating_points: Vec<Result<AnalysisSpec, String>>,
    checkpoint_runs: Option<rspice_results::run_history::RunHistoryRevision>,
    display_waveform_cache_samples: usize,
    max_parallel_workers: Option<usize>,
    compilation: Option<&'a rspice_simulation::project_veriloga::receipt::ProjectCompileReceipt>,
    design_checks: ContentDigest,
}

impl<'a> LiveInputs<'a> {
    /// Read every input from the session, through the very view preparation
    /// borrows. The view is taken apart field by field, so an input added to
    /// it later does not compile until it is given an identity here.
    fn of(state: &'a AppState) -> Self {
        let crate::simulation::execution::preparation::RunPreparationInputs {
            analysis,
            technology,
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
            compilation,
            design_checks,
        } = super::prepared_run::run_preparation_inputs(state);
        // The open schematic, the file it was loaded from and the cell
        // library are inputs of the projection, and its key names them.
        let rspice_simulation::analysis_preparation::AnalysisInputs {
            sim_setup,
            schematic: _,
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

        let mut topology = state
            .workspace
            .content
            .schematic_buffers
            .iter()
            .map(|(key, schematic)| (key.clone(), schematic.topology_version()))
            .collect::<Vec<_>>();
        topology.sort();
        topology.push((String::new(), schematic_topology_revision));

        Self {
            projection: state.workspace.design_projection_key(
                &state.library_manager,
                &state.workspace.content.active_view,
                &state.schematic,
            ),
            topology,
            setup: debug_digest("rspice.run-input.setup/v1", sim_setup),
            plan_payloads,
            project: debug_digest("rspice.run-input.project/v1", project),
            models,
            resolutions: resolutions.as_map(),
            registry,
            layouts,
            project_sources,
            design_management,
            configuration_sets,
            stimulus_library,
            imported_checkpoints,
            imported_transient_checkpoints,
            operating_points: operating_point_reads(state),
            checkpoint_runs: checkpoint_run_reads(state),
            display_waveform_cache_samples,
            max_parallel_workers,
            compilation,
            design_checks: debug_digest("rspice.run-input.design-checks/v1", &design_checks),
        }
    }
}

impl RunInputIdentity {
    /// The identity of the inputs the session holds now.
    pub(super) fn capture(state: &AppState) -> Self {
        let live = LiveInputs::of(state);
        Self(Box::new(CapturedInputs {
            projection: live.projection,
            topology: live.topology,
            setup: live.setup,
            plan_payloads: live.plan_payloads.to_vec(),
            project: live.project,
            models: live.models.clone(),
            resolutions: live.resolutions.clone(),
            registry: live.registry.clone(),
            layouts: live.layouts.clone(),
            project_sources: live.project_sources.clone(),
            design_management: live.design_management.clone(),
            configuration_sets: live.configuration_sets.clone(),
            stimulus_library: live.stimulus_library.cloned(),
            imported_checkpoints: live.imported_checkpoints.clone(),
            imported_transient_checkpoints: live.imported_transient_checkpoints.clone(),
            operating_points: live.operating_points,
            checkpoint_runs: live.checkpoint_runs,
            display_waveform_cache_samples: live.display_waveform_cache_samples,
            max_parallel_workers: live.max_parallel_workers,
            compilation: live.compilation.cloned(),
            design_checks: live.design_checks,
        }))
    }

    /// The first input the session no longer holds as it was captured, named
    /// for the reader, or `None` when every one is unchanged. A design whose
    /// key could not be taken is never confirmed unchanged.
    fn moved_in(&self, state: &AppState) -> Option<&'static str> {
        let captured = &*self.0;
        let live = LiveInputs::of(state);
        let compilation_unchanged = match (&captured.compilation, live.compilation) {
            (None, None) => true,
            (Some(held), Some(now)) => {
                held.token() == now.token() && Arc::ptr_eq(held.report(), now.report())
            }
            _ => false,
        };
        [
            (
                captured.projection.is_some() && captured.projection == live.projection,
                "the design",
            ),
            (captured.topology == live.topology, "the design"),
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
                captured.design_management == *live.design_management
                    && captured.configuration_sets == *live.configuration_sets,
                "the design configuration",
            ),
            (
                captured.stimulus_library.as_ref() == live.stimulus_library,
                "the stimulus library",
            ),
            (
                captured.imported_checkpoints == *live.imported_checkpoints
                    && captured.imported_transient_checkpoints
                        == *live.imported_transient_checkpoints,
                "the imported checkpoints",
            ),
            (
                captured.operating_points == live.operating_points,
                "the devices or results the operating point reports",
            ),
            (
                captured.checkpoint_runs == live.checkpoint_runs,
                "the retained runs a checkpoint resumes from",
            ),
            (
                captured.display_waveform_cache_samples == live.display_waveform_cache_samples
                    && captured.max_parallel_workers == live.max_parallel_workers,
                "the solver limits",
            ),
            (compilation_unchanged, "the Verilog-A compilation"),
            (
                captured.design_checks == live.design_checks,
                "the design-check rules",
            ),
        ]
        .into_iter()
        .find_map(|(unchanged, input)| (!unchanged).then_some(input))
    }
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

/// The revision of the retained runs, when an analysis in the plan resumes
/// from a checkpoint they may hold; `None` when none does, because then
/// nothing but an operating point reads them.
fn checkpoint_run_reads(state: &AppState) -> Option<rspice_results::run_history::RunHistoryRevision> {
    let plan = state.sim_setup.analysis_plan.as_ref()?;
    plan.instances()
        .iter()
        .any(|instance| match instance.draft() {
            // A draft that does not lower prepares no run at all.
            AnalysisDraft::MonteCarlo(draft) => {
                !matches!(draft.to_config(), Ok(config) if config.checkpoint.is_none())
            }
            AnalysisDraft::Transient(draft) => draft.checkpoint.is_some(),
            _ => false,
        })
        .then(|| state.simulation.retained.runs.revision())
}

/// SHA-256 over `value`'s `Debug` text, streamed into the hash rather than
/// held. `Debug` prints every field, a float as the shortest text that reads
/// back as the same value, and a string escaped, so two values that print
/// alike hold alike.
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
        let answer = crate::simulation::table_route::SourceFiles.route_retaining(
            stored, data_root, table,
        );
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

/// What a retained run set was prepared from, held beside it until it is
/// dispatched or dropped.
pub(super) struct RetainedRunInputs {
    pub(super) identity: RunInputIdentity,
    pub(super) source_files: SourceFileAnswers,
}

impl RetainedRunInputs {
    /// Refuse unless nothing `snapshot` was prepared from has moved: the
    /// session's inputs, the external model files its seal read, the files
    /// its includes spliced in, and the data files its sources were routed
    /// to. Nothing is prepared.
    pub(super) fn still_current(
        &self,
        state: &AppState,
        snapshot: &PreparedRunSnapshot,
    ) -> Result<(), PreparationError> {
        let refuse = |stage: PreparationStage, what: &str| {
            Err(PreparationError::new(
                stage,
                format!("{DESIGN_CHANGED_SINCE_CHECK} · {what}"),
            ))
        };
        if let Some(input) = self.identity.moved_in(state) {
            return refuse(
                PreparationStage::Authorization,
                &format!("{input} changed"),
            );
        }
        if let Err(error) = rspice_simulation::model_sources::verify_plan_external_model_sources(
            state.model_library_manager.catalog(),
            &state.sim_setup.model_bindings,
        ) {
            return refuse(PreparationStage::ModelBindings, &error);
        }
        if let Err(error) =
            rspice_simulation::netlist_preparation::dependencies::verify_host_dependencies(
                snapshot.sealed_source_dependencies(),
            )
        {
            return refuse(PreparationStage::SourceChecks, &error);
        }
        if let Some(path) = self.source_files.moved() {
            return refuse(
                PreparationStage::SourceChecks,
                &format!("the data file '{path}' changed"),
            );
        }
        Ok(())
    }
}
