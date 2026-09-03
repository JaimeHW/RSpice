//! Run Command - Execute SPICE simulations
//!
//! This is the main simulation execution command, supporting:
//! - All analysis types (DC, AC, Transient, Noise, etc.)
//! - Multiple output formats
//! - Progress reporting
//! - Waveform compression

#![allow(clippy::too_many_arguments)]

mod advanced;
mod basic;
mod document;
mod fft_document;
mod fourier_document;
mod frequency;
mod restart;
mod shared;

pub(crate) use document::PublishedResult;

pub(crate) use crate::commands::export_table as export;
pub(crate) use fft_document::read_fft_raw_artifact;

use crate::report::{
    CsvMeasReporter, JUnitReporter, JsonMeasReporter, MeasurementReport, SimulationReport,
    TapReporter,
};

use crate::cli::{
    CliError, Config, MeasFormat, OutputFormat, PzTransferMode, RunArgs, map_atomic_output_error,
};
use crate::commands::publish;
use rspice_core::execution::{
    AnalysisInstanceId, AxisAssignment, AxisKind, DeckPlan, DeckPlanError, DeckPlanMaterializer,
    MaterializedAnalysis, MaterializedRunError, PlannedPostProcess, PostProcessSource,
    RunAxisValue, RunCoordinate, StepAxisTarget,
};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{
    ConvergencePreset, Engine, Netlist, SimulationConfig, SimulationConfigOverrides,
    resolve_simulation_config,
};
use std::path::PathBuf;
use std::time::Instant;

fn parse_options_for_run(
    args: &RunArgs,
    resource_limits: rspice_core::ResourceLimits,
) -> rspice_core::netlist::NetlistParseOptions {
    let mut options = rspice_core::netlist::NetlistParseOptions {
        resource_limits,
        ..rspice_core::netlist::NetlistParseOptions::default()
    };
    if let Some(mode) = args.redefined_params {
        let (selection, diagnostic) = mode.parse_policies();
        options.parameter_redefinition_policy = selection;
        options.parameter_redefinition_diagnostic_policy = diagnostic;
    }
    if let Some(dialect) = args.spice_dialect {
        options.expression_dialect = dialect.expression_dialect();
    }
    options
}

struct RunContext<'a> {
    engine: &'a Engine,
    netlist: &'a Netlist,
    args: &'a RunArgs,
    /// Resolved output format: CLI flag, else config `output.format`, else raw.
    format: OutputFormat,
    /// Resolved output path; relative paths land in config `output.output_directory`.
    output: Option<std::path::PathBuf>,
    /// Coordinate-local checkpoint output path. Multi-run labels are inserted
    /// before the extension so independent runs can never overwrite state.
    checkpoint: Option<std::path::PathBuf>,
    /// Coordinate-local checkpoint input path, using the same namespace rule
    /// as checkpoint output.
    resume: Option<std::path::PathBuf>,
    /// CLI `--progress` or config `output.show_progress`.
    show_progress: bool,
    /// CLI `--compress` or config `simulation.compress_waveforms`.
    compress: bool,
    /// CLI `--compress-tol`, else config `simulation.compression_tolerance`.
    compress_tol: f64,
    /// More than one analysis card runs; output files get per-analysis tags.
    multi_analysis: bool,
    /// Canonical identity of the concrete STEP/TEMP coordinate, when this is
    /// an axis-expanded run. Scalar runs deliberately retain `None`.
    coordinate: Option<ArtifactCoordinate>,
    /// Number of authored analysis instances that publish under each output
    /// tag. A tag registered here must resolve to a planned identity.
    output_tag_multiplicities: std::collections::HashMap<&'static str, usize>,
    /// Canonical plan-owned output identities, consumed in authored analysis
    /// order by the per-analysis exporters.
    planned_output_ids: std::cell::RefCell<
        std::collections::HashMap<
            &'static str,
            std::collections::VecDeque<rspice_core::execution::AnalysisInstanceId>,
        >,
    >,
    /// Canonical transient identities used by checkpoint and post-processing
    /// namespaces, indexed by zero-based authored transient ordinal.
    planned_transient_ids: Vec<rspice_core::execution::AnalysisInstanceId>,
    /// Every planned `.FOUR` operand and `.FFT` card, named by the canonical
    /// plan and bound to the transient it post-processes.
    planned_post_processes: Vec<PlannedPostProcess>,
    /// Canonical `.FFT` identities in authored order, one per authored
    /// transient FFT request, projected from `planned_post_processes`.
    planned_fft_ids: Vec<String>,
    planned_namespace_error: std::cell::RefCell<Option<String>>,
    /// The canonical coordinate this concrete deck run belongs to, retained so
    /// a typed result document can name it. Scalar decks have none.
    run_coordinate: Option<RunCoordinate>,
    /// Structural fingerprint of the elaborated circuit. An axis coordinate
    /// arrives with the one its materialization computed; a scalar deck
    /// computes it on demand, and only when a format that carries it is
    /// requested.
    topology: std::cell::RefCell<Option<rspice_core::execution::TopologyFingerprint>>,
    /// Typed results this run published, in publication order.
    published: std::cell::RefCell<Vec<PublishedResult>>,
    verbose: bool,
    quiet: bool,
    /// .MEAS results collected while analyses run, for CI/CD reporting
    /// (`--report-file` / `--meas-file`) and the process exit status.
    measurements: std::cell::RefCell<Vec<MeasurementReport>>,
    /// Analysis tags (upper-case) whose .MEAS statements were evaluated,
    /// so leftover measurements can fail loudly instead of being skipped.
    evaluated_meas: std::cell::RefCell<std::collections::HashSet<String>>,
    /// Result files this run resolved for export, for the `--summary`
    /// manifest.
    outputs: std::cell::RefCell<Vec<std::path::PathBuf>>,
    /// Completed authored transients a planned `.FOUR` card post-processes,
    /// in authored order. `.FOUR` cards run after every physical analysis —
    /// ngspice accepts a `.FOUR` card above the `.TRAN` it belongs to — so a
    /// deck with several transients must still be able to reach the one the
    /// plan bound each card to, not merely the one that ran last. A transient
    /// no planned card names is not retained at all.
    retained_transients: std::cell::RefCell<Vec<RetainedTransient>>,
    /// Zero-based ordinal assigned to authored transient cards as they enter
    /// the physical-analysis dispatcher.
    next_transient_ordinal: std::cell::Cell<u32>,
    /// Zero-based ordinal assigned to source-authored Fourier cards.
    next_fourier_ordinal: std::cell::Cell<u32>,
}

/// The canonical identity of one planned `.FOUR` operand and the authored
/// output spelling that operand names.
type PlannedFourierOperand<'plan> = (AnalysisInstanceId, &'plan str);

struct RetainedTransient {
    analysis_id: String,
    /// The same identity, typed, so a post-process document can name its
    /// parent.
    analysis: rspice_core::execution::AnalysisInstanceId,
    result: rspice_core::engine::TransientResult,
    /// Typed post-process products the core evaluated on the exact accepted
    /// trajectory. Present only for a compressed run, whose retained waveform
    /// is decimated and therefore cannot reproduce them.
    post_results: Option<rspice_core::engine::TransientPostResults>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArtifactCoordinate {
    id: String,
    ordinal: usize,
    tag: String,
    assignment: String,
}

impl ArtifactCoordinate {
    fn from_run_coordinate(coordinate: &RunCoordinate) -> Self {
        Self {
            id: coordinate.stable_id().to_string(),
            ordinal: coordinate.ordinal().saturating_add(1),
            tag: coordinate.stable_tag(),
            assignment: canonical_coordinate_description(coordinate),
        }
    }
}

impl<'a> RunContext<'a> {
    fn new(
        engine: &'a Engine,
        netlist: &'a Netlist,
        args: &'a RunArgs,
        config: &Config,
        verbose: bool,
        quiet: bool,
        run_label: Option<&str>,
        identity: RunIdentity<'_>,
    ) -> Result<Self, CliError> {
        let RunIdentity {
            coordinate,
            topology,
            analyses: planned,
        } = identity;
        let format = match args.format {
            Some(format) => format,
            None => parse_format_name(&config.output.format)?,
        };
        let mut output = resolve_output_path(args.output.clone(), config)?;
        let mut checkpoint = args.checkpoint.clone();
        let mut resume = args.resume.clone();
        // Multi-run decks tag each run's output so later runs cannot
        // silently overwrite earlier ones: `out.csv` -> `out.hot.csv`.
        if let Some(label) = run_label {
            let tag = sanitize_run_tag(label);
            if let Some(path) = output.as_mut() {
                *path = tag_output_path(path, &tag);
            }
            if let Some(path) = checkpoint.as_mut() {
                *path = tag_output_path(path, &tag);
            }
            if let Some(path) = resume.as_mut() {
                *path = tag_output_path(path, &tag);
            }
        }

        Ok(Self {
            engine,
            netlist,
            args,
            format,
            output,
            checkpoint,
            resume,
            show_progress: args.progress || config.output.show_progress,
            compress: args.compress || config.simulation.compress_waveforms,
            compress_tol: args
                .compress_tol
                .unwrap_or(config.simulation.compression_tolerance),
            // `.FFT` is retained outside `Netlist::analyses`, but publishing
            // its typed result adds an independent artifact beside the
            // parent transient and therefore also requires tagged paths.
            multi_analysis: coordinate.is_some()
                || netlist.analyses.len() > 1
                || !netlist.fft_analyses.is_empty(),
            coordinate: coordinate.map(ArtifactCoordinate::from_run_coordinate),
            output_tag_multiplicities: analysis_output_tag_multiplicities(netlist),
            planned_output_ids: std::cell::RefCell::new(planned.output_ids),
            planned_transient_ids: planned.transient_ids,
            planned_fft_ids: planned_fft_ids(&planned.post_processes, netlist)?,
            planned_post_processes: planned.post_processes,
            planned_namespace_error: std::cell::RefCell::new(None),
            run_coordinate: coordinate.cloned(),
            topology: std::cell::RefCell::new(topology),
            published: std::cell::RefCell::new(Vec::new()),
            verbose,
            quiet,
            measurements: std::cell::RefCell::new(Vec::new()),
            evaluated_meas: std::cell::RefCell::new(std::collections::HashSet::new()),
            outputs: std::cell::RefCell::new(Vec::new()),
            retained_transients: std::cell::RefCell::new(Vec::new()),
            next_transient_ordinal: std::cell::Cell::new(0),
            next_fourier_ordinal: std::cell::Cell::new(0),
        })
    }

    /// Context for a deck this process elaborated itself rather than received
    /// from the deck planner: one `--corners` variant, whose artifact paths
    /// the caller has already namespaced by corner.
    ///
    /// The corner deck is re-parsed, so it gets its own canonical plan and
    /// therefore names its analyses exactly as the nominal deck does.
    pub(super) fn for_elaborated_deck(
        engine: &'a Engine,
        netlist: &'a Netlist,
        args: &'a RunArgs,
        format: OutputFormat,
        paths: ElaboratedDeckPaths,
        source: &RunContextSettings,
    ) -> Result<Self, CliError> {
        let plan = DeckPlan::from_netlist_with_abort(
            netlist,
            &engine.config().resource_limits,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| CliError::InternalError {
            message: format!("re-elaborated deck cannot be planned: {error}"),
        })?;
        let planned = PlannedAnalysisIdentities::from_plan(&plan, netlist);
        Ok(Self {
            engine,
            netlist,
            args,
            format,
            output: paths.output,
            checkpoint: paths.checkpoint,
            resume: paths.resume,
            show_progress: source.show_progress,
            compress: source.compress,
            compress_tol: source.compress_tol,
            multi_analysis: source.coordinate.is_some()
                || netlist.analyses.len() > 1
                || !netlist.fft_analyses.is_empty(),
            coordinate: source.coordinate.clone(),
            output_tag_multiplicities: analysis_output_tag_multiplicities(netlist),
            planned_output_ids: std::cell::RefCell::new(planned.output_ids),
            planned_transient_ids: planned.transient_ids,
            planned_fft_ids: planned_fft_ids(&planned.post_processes, netlist)?,
            planned_post_processes: planned.post_processes,
            planned_namespace_error: std::cell::RefCell::new(None),
            // A corner is a re-elaborated deck: it owns no plan coordinate,
            // and its topology is its own, computed on demand.
            run_coordinate: None,
            topology: std::cell::RefCell::new(None),
            published: std::cell::RefCell::new(Vec::new()),
            verbose: source.verbose,
            quiet: source.quiet,
            measurements: std::cell::RefCell::new(Vec::new()),
            evaluated_meas: std::cell::RefCell::new(std::collections::HashSet::new()),
            outputs: std::cell::RefCell::new(Vec::new()),
            retained_transients: std::cell::RefCell::new(Vec::new()),
            next_transient_ordinal: std::cell::Cell::new(0),
            next_fourier_ordinal: std::cell::Cell::new(0),
        })
    }

    /// Record evaluated .MEAS results: print them under `--meas` and keep
    /// them for report files and the exit status.
    fn record_measurements(&self, analysis: &str, results: Vec<rspice_core::MeasureResult>) {
        if results.is_empty() {
            return;
        }
        self.evaluated_meas
            .borrow_mut()
            .insert(analysis.to_ascii_uppercase());

        if self.args.meas && !self.quiet {
            println!("  Measurement Results ({}, {}):", analysis, results.len());
            for mr in &results {
                match (mr.value, mr.passed) {
                    (Some(value), true) => println!(
                        "    {} = {}",
                        mr.name,
                        crate::report::format_spice_exponent(value)
                    ),
                    // Evaluated, but failed an authored verification contract.
                    (Some(value), false) => println!(
                        "    {} = {} FAILED ({})",
                        mr.name,
                        crate::report::format_spice_exponent(value),
                        mr.error
                            .as_deref()
                            .unwrap_or("verification contract failed")
                    ),
                    (None, _) => println!(
                        "    {} = FAILED ({})",
                        mr.name,
                        mr.error.as_deref().unwrap_or("not evaluated")
                    ),
                }
            }
        }

        self.measurements
            .borrow_mut()
            .extend(results.into_iter().map(|mr| MeasurementReport {
                name: mr.name,
                value: mr.value,
                raw_value: mr.raw_value,
                expected: mr.expected,
                tolerance: mr.tolerance,
                failure_limit: mr.failure_limit,
                failure_limit_exceeded: mr.failure_limit_exceeded,
                passed: mr.passed,
                error: mr.error,
                record_index: None,
                event_axis: mr.event_axis,
                trigger_axis: None,
                target_axis: None,
                aggregate_policy: None,
            }));
    }

    /// Convert .MEAS statements whose analysis never evaluated them into
    /// explicit failures, so automation cannot mistake a skipped check for
    /// a passing one.
    fn record_unevaluated_measurements(&self) {
        let mut analyses: Vec<String> = self
            .netlist
            .measurements
            .iter()
            .map(|m| m.analysis.to_ascii_uppercase())
            .collect();
        analyses.sort();
        analyses.dedup();
        analyses.retain(|analysis| !self.evaluated_meas.borrow().contains(analysis));

        for analysis in analyses {
            let reason = match analysis.as_str() {
                "TRAN" | "DC" | "AC" | "NOISE" => format!("{analysis} analysis did not run"),
                "TRAN_CONT" => "TRAN analysis did not run".to_string(),
                "DC_CONT" => "DC analysis did not run".to_string(),
                "AC_CONT" => "AC analysis did not run".to_string(),
                "NOISE_CONT" => "NOISE analysis did not run".to_string(),
                other => format!("unknown .MEAS analysis kind '{other}'"),
            };
            let results =
                rspice_core::analysis::unevaluated_measurements(self.netlist, &analysis, &reason);
            self.record_measurements(&analysis, results);
        }
    }

    /// Output path for one analysis.
    ///
    /// When the deck runs several analyses, each gets its own file so later
    /// analyses cannot silently overwrite earlier results:
    /// `out.csv` becomes `out.op-001.csv`, `out.tran-001.csv`, ...
    ///
    /// The namespace component is always the canonical
    /// `AnalysisInstanceId::tag()` the planner minted for the authored card.
    /// A tag the deck did not author belongs to a command-line analysis mode
    /// (`--monte-carlo`, `--sparam`, ...), which is single by construction and
    /// therefore publishes under the bare tag.
    ///
    /// Every resolved path is remembered for the `--summary` manifest.
    fn output_path_for(&self, tag: &str) -> Option<std::path::PathBuf> {
        self.resolve_output(tag).map(|output| output.path)
    }

    /// Resolve one analysis artifact: where it goes and which canonical
    /// analysis instance it publishes under.
    ///
    /// A deck-authored card takes the identity the planner minted for it. A
    /// command-line analysis mode has no authored card, so its identity is
    /// minted the same canonical way: it is single by construction, and the
    /// first instance of its family.
    fn resolve_output(&self, tag: &str) -> Option<ResolvedOutput> {
        let path = self.output.clone()?;
        let planned_id = self
            .planned_output_ids
            .borrow_mut()
            .get_mut(tag)
            .and_then(std::collections::VecDeque::pop_front);
        let (qualified_tag, analysis) = match planned_id {
            Some(id) => (id.tag(), Some(id)),
            None => {
                if self.output_tag_multiplicities.contains_key(tag) {
                    self.planned_namespace_error.replace(Some(format!(
                        "planned analysis namespace queue has no remaining '{tag}' identity"
                    )));
                    return None;
                }
                let minted = output_tag_analysis_kind(tag)
                    .map(command_line_analysis_identity)
                    .transpose()
                    .unwrap_or_else(|error| {
                        self.planned_namespace_error.replace(Some(format!(
                            "cannot mint a canonical '{tag}' identity: {error}"
                        )));
                        None
                    });
                (tag.to_string(), minted)
            }
        };
        let resolved = if !self.multi_analysis {
            path
        } else {
            let mut file_name = path
                .file_stem()
                .map(|stem| stem.to_os_string())
                .unwrap_or_default();
            file_name.push(format!(".{qualified_tag}"));
            if let Some(ext) = path.extension() {
                file_name.push(".");
                file_name.push(ext);
            }
            path.with_file_name(file_name)
        };
        self.outputs.borrow_mut().push(resolved.clone());
        Some(ResolvedOutput {
            path: resolved,
            analysis,
        })
    }

    /// The canonical coordinate this concrete deck run belongs to.
    const fn run_coordinate(&self) -> Option<&RunCoordinate> {
        self.run_coordinate.as_ref()
    }

    /// Structural fingerprint of this run's elaborated circuit.
    ///
    /// An axis coordinate arrives with the fingerprint its materialization
    /// already computed. A scalar deck computes it once, on the first request,
    /// so a run that publishes no typed artifact never pays for a circuit
    /// build it does not use.
    fn topology_fingerprint(
        &self,
    ) -> Result<Option<rspice_core::execution::TopologyFingerprint>, CliError> {
        if let Some(fingerprint) = *self.topology.borrow() {
            return Ok(Some(fingerprint));
        }
        let fingerprint = rspice_core::execution::topology_fingerprint_with_abort(
            self.engine,
            self.netlist,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| {
            if matches!(source, rspice_core::SimulationError::Aborted) {
                cancellation_cli_error(self.args.timeout)
            } else {
                CliError::CoreSimulationError {
                    source,
                    analysis: Some("Result topology fingerprint".to_string()),
                }
            }
        })?;
        self.topology.replace(Some(fingerprint));
        Ok(Some(fingerprint))
    }

    /// Record one artifact this run wrote outside `resolve_output`, so the
    /// `--summary` manifest names it too.
    fn record_output(&self, path: std::path::PathBuf) {
        self.outputs.borrow_mut().push(path);
    }

    /// Record one typed result this run published.
    fn record_published(&self, published: PublishedResult) {
        self.published.borrow_mut().push(published);
    }

    fn ensure_planned_namespaces_consumed(&self) -> Result<(), CliError> {
        if let Some(message) = self.planned_namespace_error.borrow_mut().take() {
            return Err(CliError::InternalError { message });
        }
        if self.output.is_none() {
            return Ok(());
        }
        let unconsumed = self
            .planned_output_ids
            .borrow()
            .iter()
            .filter_map(|(tag, ids)| (!ids.is_empty()).then_some((*tag, ids.len())))
            .collect::<Vec<_>>();
        if unconsumed.is_empty() {
            Ok(())
        } else {
            Err(CliError::InternalError {
                message: format!(
                    "planned analysis namespace queue retained unconsumed identities: {unconsumed:?}"
                ),
            })
        }
    }

    /// Resolve the checkpoint namespace of the transient currently entering
    /// the dispatcher. The dispatcher advances `next_transient_ordinal`
    /// before calling the transient executor, so its value is the one-based
    /// external ordinal used here.
    fn transient_checkpoint_path(
        &self,
        path: Option<&std::path::Path>,
    ) -> Option<std::path::PathBuf> {
        let path = path?.to_path_buf();
        let analysis_id = match self.current_transient_analysis_id() {
            Ok(analysis_id) => analysis_id,
            Err(CliError::InternalError { message }) => {
                self.planned_namespace_error.replace(Some(message));
                return None;
            }
            Err(_) => return None,
        };
        // A deck with one authored transient keeps the requested checkpoint
        // path so `--checkpoint state.chk` round-trips under its own name.
        if self
            .output_tag_multiplicities
            .get("tran")
            .is_none_or(|count| *count <= 1)
            && self.coordinate.is_none()
        {
            return Some(path);
        }
        Some(tag_output_path(&path, &analysis_id))
    }

    fn current_transient_analysis_id(&self) -> Result<String, CliError> {
        self.current_transient_instance().map(|id| id.tag())
    }

    /// Canonical identity of the transient currently entering the dispatcher.
    fn current_transient_instance(
        &self,
    ) -> Result<rspice_core::execution::AnalysisInstanceId, CliError> {
        let ordinal = self.next_transient_ordinal.get();
        let index = ordinal
            .checked_sub(1)
            .ok_or_else(|| CliError::InternalError {
                message: "transient execution entered without an assigned analysis ordinal"
                    .to_string(),
            })?;
        self.planned_transient_ids
            .get(index as usize)
            .copied()
            .ok_or_else(|| CliError::InternalError {
                message: format!(
                    "planned transient namespace has no identity for ordinal {ordinal}"
                ),
            })
    }

    /// One FFT artifact contains every source-authored directive for one
    /// parent transient. Repeated transients compose the parent identity into
    /// the artifact tag so no result can overwrite another.
    fn fft_output_path_for(&self, parent_analysis_id: &str) -> Option<std::path::PathBuf> {
        let tag = if self
            .output_tag_multiplicities
            .get("tran")
            .is_some_and(|count| *count > 1)
        {
            format!("{parent_analysis_id}.fft")
        } else {
            "fft".to_string()
        };
        self.output_path_for(&tag)
    }

    /// Every planned `.FOUR` operand of one authored card.
    fn planned_fourier_card(&self, card_index: usize) -> impl Iterator<Item = &PlannedPostProcess> {
        self.planned_post_processes.iter().filter(move |post| {
            matches!(
                post.source(),
                PostProcessSource::FourierOperand { card_index: card, .. } if *card == card_index
            )
        })
    }

    /// Whether any planned `.FOUR` operand post-processes this transient.
    fn plans_fourier_for(&self, transient: AnalysisInstanceId) -> bool {
        self.planned_post_processes.iter().any(|post| {
            matches!(post.source(), PostProcessSource::FourierOperand { .. })
                && post.parent() == transient
        })
    }

    /// The transient one authored `.FOUR` card post-processes, and the planned
    /// identity and authored spelling of each of its operands.
    ///
    /// The core evaluates one Fourier spectrum per resolved operand and the
    /// shared result document names one spectrum, so an operand — not a card —
    /// is the analysis instance. The plan already assigned those identities
    /// and already bound the card to its transient, so the CLI reads both off
    /// it instead of counting operands and assuming the transient that ran
    /// last is the one the card meant.
    fn planned_fourier_operands(
        &self,
        card_index: usize,
    ) -> Result<(AnalysisInstanceId, Vec<PlannedFourierOperand<'_>>), CliError> {
        let parent = self
            .planned_fourier_card(card_index)
            .next()
            .map(PlannedPostProcess::parent)
            .ok_or_else(|| CliError::InternalError {
                message: format!(
                    "the canonical plan names no operand for .FOUR card {}",
                    card_index.saturating_add(1)
                ),
            })?;
        let operands = self
            .planned_fourier_card(card_index)
            .map(|post| match post.source() {
                PostProcessSource::FourierOperand { output, .. } => {
                    Ok((post.id(), output.as_str()))
                }
                _ => Err(CliError::InternalError {
                    message: format!("{} is not a planned .FOUR operand", post.id()),
                }),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok((parent, operands))
    }

    /// The completed transient result one `.FOUR` card post-processes.
    fn retained_transient(
        &self,
        parent: AnalysisInstanceId,
    ) -> Result<std::cell::Ref<'_, RetainedTransient>, CliError> {
        std::cell::Ref::filter_map(self.retained_transients.borrow(), |retained| {
            retained
                .iter()
                .find(|candidate| candidate.analysis == parent)
        })
        .map_err(|_| {
            CliError::simulation_error_in(
                format!(
                    ".FOUR requires a completed authored .TRAN to post-process; the plan binds this card to {parent}, which this run did not complete"
                ),
                "Fourier",
            )
        })
    }

    /// Canonical identities of the deck's authored `.FFT` requests, in source
    /// order. One transient publishes a spectrum for each of them.
    fn fft_analysis_ids(&self) -> &[String] {
        &self.planned_fft_ids
    }

    /// The same identities as typed instances, for a document that names its
    /// attached spectra as children.
    fn fft_analysis_instances(&self) -> Vec<AnalysisInstanceId> {
        planned_fft_instances(&self.planned_post_processes).collect()
    }

    fn run_analysis(&self, analysis: &AnalysisCommand) -> Result<(), CliError> {
        match analysis {
            AnalysisCommand::Op => basic::run_dc_op(self)?,
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                sweep2,
                mode: _,
            } => basic::run_dc_sweep(self, source, *start, *stop, *step, sweep2.as_ref())?,
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
            } => {
                let ordinal = self.next_transient_ordinal.get();
                let next = ordinal
                    .checked_add(1)
                    .ok_or_else(|| CliError::InternalError {
                        message: "authored transient ordinal overflowed u32".to_string(),
                    })?;
                self.next_transient_ordinal.set(next);
                let outcome = basic::run_transient(
                    self,
                    *stop,
                    *step,
                    start.unwrap_or(0.0),
                    *max_step,
                    *uic,
                )?;
                let analysis = self.current_transient_instance()?;
                // Only a transient a planned `.FOUR` card names is kept:
                // holding every transient's waveform for a deck that
                // post-processes none of them would be an unbounded cost for
                // nothing.
                if self.plans_fourier_for(analysis) {
                    self.retained_transients
                        .borrow_mut()
                        .push(RetainedTransient {
                            analysis_id: analysis.tag(),
                            analysis,
                            result: outcome.result,
                            post_results: outcome.post_results,
                        });
                }
            }
            AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            } => frequency::run_ac(self, *variation, *points, *start_freq, *stop_freq)?,
            AnalysisCommand::AcData { table_name } => frequency::run_ac_data(self, table_name)?,
            AnalysisCommand::Hb { frequencies } => {
                advanced::run_hb_from_command(self, frequencies)?
            }
            AnalysisCommand::Sp { .. } => advanced::run_sparam_from_command(self, analysis)?,
            AnalysisCommand::Stb {
                variation,
                points,
                start_freq,
                stop_freq,
                probe,
            } => frequency::run_stb(self, *variation, *points, *start_freq, *stop_freq, probe)?,
            AnalysisCommand::Disto {
                variation,
                points,
                start_freq,
                stop_freq,
                f2_over_f1,
            } => frequency::run_disto(
                self,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                *f2_over_f1,
            )?,
            AnalysisCommand::Noise {
                output_node,
                reference_node,
                input_source,
                variation,
                points,
                start_freq,
                stop_freq,
            } => frequency::run_noise(
                self,
                output_node,
                reference_node.as_deref(),
                input_source,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
            )?,
            AnalysisCommand::NoiseData {
                output_node,
                reference_node,
                input_source,
                table_name,
            } => frequency::run_noise_data(
                self,
                output_node,
                reference_node.as_deref(),
                input_source,
                table_name,
            )?,
            AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                output_is_current,
                filters,
                ac_sweep,
            } => frequency::run_sensitivity_from_command(
                self,
                output_node,
                reference_node.as_deref(),
                *output_is_current,
                filters,
                *ac_sweep,
            )?,
            AnalysisCommand::Tf {
                output_node,
                reference_node,
                output_is_current,
                input_source,
            } => frequency::run_tf_from_command(
                self,
                output_node,
                reference_node.as_deref(),
                *output_is_current,
                input_source,
            )?,
            AnalysisCommand::PoleZero {
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                transfer_type,
                analysis_type,
            } => frequency::run_pz_from_command(
                self,
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                *transfer_type,
                *analysis_type,
            )?,
            // `.STEP` and `.TEMP` are run axes. `DeckPlan` turns both into
            // Cartesian coordinates and `MaterializedRun` strips them from the
            // coordinate-local netlist, so neither can reach a dispatcher that
            // executes one concrete deck.
            AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. } => {
                return Err(CliError::InternalError {
                    message: format!(
                        "{} reached the physical-analysis dispatcher; Cartesian planning was bypassed",
                        if matches!(analysis, AnalysisCommand::Step(_)) {
                            ".STEP"
                        } else {
                            ".TEMP"
                        }
                    ),
                });
            }
            AnalysisCommand::Four {
                fundamental,
                outputs: _,
                num_harmonics,
            } => {
                let ordinal = self.next_fourier_ordinal.get();
                let next = ordinal
                    .checked_add(1)
                    .ok_or_else(|| CliError::InternalError {
                        message: "authored Fourier ordinal overflowed u32".to_string(),
                    })?;
                self.next_fourier_ordinal.set(next);
                fourier_document::run_fourier(
                    self,
                    ordinal as usize,
                    *fundamental,
                    *num_harmonics,
                )?;
            }
            AnalysisCommand::MonteCarlo(mc_cmd) => {
                advanced::run_monte_carlo_from_command(self, mc_cmd)?
            }
            AnalysisCommand::Pss(_)
            | AnalysisCommand::Pac(_)
            | AnalysisCommand::Pnoise(_)
            | AnalysisCommand::Envelope(_) => {
                // `refuse_unsupported_deck_analyses` rejects these before any
                // solver or artifact work; reaching the dispatcher means the
                // preflight was bypassed.
                return Err(unsupported_deck_analysis_error(analysis, None));
            }
        }

        Ok(())
    }
}

/// Dot-command spelling of a card the CLI has no execution route for.
fn unsupported_deck_analysis_card(analysis: &AnalysisCommand) -> Option<&'static str> {
    match analysis {
        AnalysisCommand::Pss(_) => Some(".PSS"),
        AnalysisCommand::Pac(_) => Some(".PAC"),
        AnalysisCommand::Pnoise(_) => Some(".PNOISE"),
        AnalysisCommand::Envelope(_) => Some(".ENVELOPE"),
        _ => None,
    }
}

/// Typed refusal for an authored periodic-family card.
fn unsupported_deck_analysis_error(
    analysis: &AnalysisCommand,
    analysis_id: Option<String>,
) -> CliError {
    let card = unsupported_deck_analysis_card(analysis).unwrap_or(".<analysis>");
    CliError::UnsupportedDeckAnalysis {
        card,
        analysis_id,
        reason: "the CLI has no execution route or result artifact for the periodic \
                 large-signal analysis family; run it through the Python API",
    }
}

/// Refuse authored cards this frontend cannot execute before any solver work
/// runs, any run-axis compatibility check reports a lesser problem, or any
/// artifact is written.
fn refuse_unsupported_deck_analyses(
    netlist: &Netlist,
    config: &Config,
    args: &RunArgs,
) -> Result<(), CliError> {
    if !netlist
        .analyses
        .iter()
        .any(|analysis| unsupported_deck_analysis_card(analysis).is_some())
    {
        return Ok(());
    }
    let plan = DeckPlan::from_netlist_with_abort(
        netlist,
        &config.resources.limits(),
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| map_deck_plan_error(error, args))?;
    refuse_planned_unsupported_analyses(netlist, &plan)
}

/// Name every refused card with the identity the canonical plan assigned it.
fn refuse_planned_unsupported_analyses(netlist: &Netlist, plan: &DeckPlan) -> Result<(), CliError> {
    for (analysis, id) in plan.authored_analyses(netlist) {
        if unsupported_deck_analysis_card(analysis).is_some() {
            return Err(unsupported_deck_analysis_error(
                analysis,
                id.map(|id| id.tag()),
            ));
        }
    }
    Ok(())
}

/// `--pss-freq` and an authored `.PSS` both name a periodic steady state.
/// Executing one and dropping the other would silently discard what the deck
/// or the command line asked for.
fn validate_pss_flag_conflict(netlist: &Netlist, args: &RunArgs) -> Result<(), CliError> {
    if args.pss_freq.is_none() {
        return Ok(());
    }
    if !netlist
        .analyses
        .iter()
        .any(|analysis| matches!(analysis, AnalysisCommand::Pss(_)))
    {
        return Ok(());
    }
    Err(CliError::InvalidArgument {
        message: "--pss-freq cannot be combined with an authored .PSS card".to_string(),
        suggestion: Some(
            "drop --pss-freq and author the whole periodic steady state on the .PSS card"
                .to_string(),
        ),
    })
}

fn analysis_output_tag(analysis: &AnalysisCommand) -> Option<&'static str> {
    match analysis {
        AnalysisCommand::Op => Some("op"),
        AnalysisCommand::Dc { .. } => Some("dc"),
        AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => Some("ac"),
        AnalysisCommand::Tran { .. } => Some("tran"),
        AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => Some("noise"),
        AnalysisCommand::Sp { .. } => Some("sp"),
        AnalysisCommand::Stb { .. } => Some("stb"),
        AnalysisCommand::Disto { .. } => Some("disto"),
        AnalysisCommand::PoleZero { .. } => Some("pz"),
        AnalysisCommand::Sensitivity { .. } => Some("sens"),
        AnalysisCommand::Tf { .. } => Some("tf"),
        AnalysisCommand::Hb { .. } => Some("hb"),
        AnalysisCommand::MonteCarlo(_) => Some("mc"),
        // `.STEP` and `.TEMP` are run axes whose coordinates own the artifact
        // namespace; `.FOUR` publishes under its own post-process instance
        // identity. The CLI publishes no artifact for the periodic
        // large-signal family and refuses those cards before execution, so
        // none of these own a physical output namespace.
        AnalysisCommand::Step(_)
        | AnalysisCommand::Temp { .. }
        | AnalysisCommand::Four { .. }
        | AnalysisCommand::Pss(_)
        | AnalysisCommand::Pac(_)
        | AnalysisCommand::Pnoise(_)
        | AnalysisCommand::Envelope(_) => None,
    }
}

fn analysis_output_tag_multiplicities(
    netlist: &Netlist,
) -> std::collections::HashMap<&'static str, usize> {
    let mut counts = std::collections::HashMap::new();
    for tag in netlist.analyses.iter().filter_map(analysis_output_tag) {
        let count = counts.entry(tag).or_insert(0usize);
        *count = count.saturating_add(1);
    }
    counts
}

/// The canonical plan identity of every authored analysis one concrete deck
/// run publishes under.
///
/// Both sources resolve to the same thing — the `AnalysisInstanceId` the
/// canonical planner minted for each authored card, in source order. A scalar
/// deck reads them straight off its `DeckPlan`; an axis coordinate reads them
/// off that coordinate's `MaterializedAnalysis` list, which additionally binds
/// each identity to the coordinate. The CLI never formats an analysis
/// namespace of its own, so a repeated `.AC` pair is `ac-001`/`ac-002` for
/// exactly one reason everywhere.
#[derive(Debug, Default)]
struct PlannedAnalysisIdentities {
    /// Output tag to the queue of canonical analysis identities, consumed in
    /// authored order by the per-analysis exporters.
    output_ids: std::collections::HashMap<
        &'static str,
        std::collections::VecDeque<rspice_core::execution::AnalysisInstanceId>,
    >,
    /// Transient identities in authored order, indexed by zero-based
    /// transient ordinal for checkpoint and post-processing namespaces.
    transient_ids: Vec<rspice_core::execution::AnalysisInstanceId>,
    /// Every planned `.FOUR` operand and `.FFT` card, each already named and
    /// bound to the transient it post-processes.
    ///
    /// These are not analyses the CLI dispatches; they are the identities the
    /// transient's post-processing artifacts publish under. Taking them from
    /// the plan is what keeps `four-002` meaning the same operand here, in the
    /// browser runner, and in the engine adapter.
    post_processes: Vec<PlannedPostProcess>,
}

impl PlannedAnalysisIdentities {
    fn from_pairs<'a>(
        pairs: impl IntoIterator<
            Item = (
                &'a AnalysisCommand,
                rspice_core::execution::AnalysisInstanceId,
            ),
        >,
        post_processes: &[PlannedPostProcess],
    ) -> Self {
        let mut identities = Self {
            post_processes: post_processes.to_vec(),
            ..Self::default()
        };
        for (analysis, id) in pairs {
            let Some(tag) = analysis_output_tag(analysis) else {
                continue;
            };
            if tag == "tran" {
                identities.transient_ids.push(id);
            }
            identities.output_ids.entry(tag).or_default().push_back(id);
        }
        identities
    }

    /// Identities for a deck with no run axis, read off its canonical plan.
    ///
    /// Run axes and `.FOUR` pair with `None` in the plan: an axis owns no
    /// analysis namespace, and a Fourier card publishes under its own
    /// post-process identity instead.
    fn from_plan(plan: &DeckPlan, netlist: &Netlist) -> Self {
        Self::from_pairs(
            plan.authored_analyses(netlist)
                .filter_map(|(analysis, id)| id.map(|id| (analysis, id))),
            plan.post_process_analyses(),
        )
    }

    /// Identities for one materialized axis coordinate.
    ///
    /// The coordinate binding is checked here rather than trusted: an output
    /// or checkpoint namespace that disagrees with the coordinate it claims to
    /// belong to would let one coordinate overwrite another's artifact.
    fn from_materialized(
        plan: &DeckPlan,
        coordinate: &RunCoordinate,
        analyses: &[MaterializedAnalysis],
    ) -> Result<Self, CliError> {
        for analysis in analyses {
            let output = analysis.output_namespace();
            let checkpoint = analysis.checkpoint_namespace();
            if output.coordinate_id() != coordinate.stable_id()
                || checkpoint.coordinate_id() != coordinate.stable_id()
                || output.analysis_id() != analysis.id()
                || checkpoint.analysis_id() != analysis.id()
            {
                return Err(CliError::InternalError {
                    message: format!(
                        "materialized output/checkpoint namespace disagrees with coordinate {} and analysis {}",
                        coordinate.stable_id(),
                        analysis.id()
                    ),
                });
            }
        }
        // Post-processes are planned per deck, not per coordinate: a `.FOUR`
        // operand keeps one identity across the whole sweep, and the
        // coordinate that separates two artifacts is already in their paths.
        Ok(Self::from_pairs(
            analyses
                .iter()
                .filter_map(|analysis| analysis.command().map(|command| (command, analysis.id()))),
            plan.post_process_analyses(),
        ))
    }
}

/// One resolved artifact destination and the canonical analysis instance it
/// publishes under.
///
/// `analysis` is `None` only for the aggregated axis sweep table, which is a
/// cross-coordinate aggregation rather than one analysis result.
struct ResolvedOutput {
    path: PathBuf,
    analysis: Option<rspice_core::execution::AnalysisInstanceId>,
}

impl ResolvedOutput {
    /// The canonical identity this artifact publishes under.
    fn analysis(&self, tag: &str) -> Result<rspice_core::execution::AnalysisInstanceId, CliError> {
        self.analysis.ok_or_else(|| CliError::InternalError {
            message: format!("'{tag}' resolved an artifact with no canonical analysis identity"),
        })
    }
}

/// The core analysis family one output tag publishes under.
///
/// Every tag `analysis_output_tag` can return appears here, plus the two the
/// command line can request without an authored card. A tag with no family
/// owns no analysis identity and therefore no typed result document.
fn output_tag_analysis_kind(tag: &str) -> Option<rspice_core::execution::AnalysisKind> {
    use rspice_core::execution::AnalysisKind;
    match tag {
        "op" => Some(AnalysisKind::Op),
        "dc" => Some(AnalysisKind::Dc),
        "ac" => Some(AnalysisKind::Ac),
        "tran" => Some(AnalysisKind::Tran),
        "noise" => Some(AnalysisKind::Noise),
        // `--sparam` is the command-line spelling of the `.SP` family.
        "sp" | "sparam" => Some(AnalysisKind::Sp),
        "stb" => Some(AnalysisKind::Stb),
        "disto" => Some(AnalysisKind::Distortion),
        "pz" => Some(AnalysisKind::PoleZero),
        "sens" => Some(AnalysisKind::Sensitivity),
        "tf" => Some(AnalysisKind::TransferFunction),
        "hb" => Some(AnalysisKind::HarmonicBalance),
        "mc" => Some(AnalysisKind::MonteCarlo),
        "pss" => Some(AnalysisKind::Pss),
        // The aggregated axis sweep table spans coordinates, so it is not one
        // analysis instance and publishes no typed document of its own.
        _ => None,
    }
}

/// Artifact paths one re-elaborated deck publishes under, already namespaced
/// by the caller.
pub(crate) struct ElaboratedDeckPaths {
    pub(crate) output: Option<PathBuf>,
    pub(crate) checkpoint: Option<PathBuf>,
    pub(crate) resume: Option<PathBuf>,
}

/// Reporting and compression settings a re-elaborated deck inherits from the
/// run that spawned it.
pub(crate) struct RunContextSettings {
    pub(crate) show_progress: bool,
    pub(crate) compress: bool,
    pub(crate) compress_tol: f64,
    pub(crate) coordinate: Option<ArtifactCoordinate>,
    pub(crate) verbose: bool,
    pub(crate) quiet: bool,
}

/// The first `count` canonical identities of one analysis family, minted by
/// the planner exactly as it would mint them for `count` authored cards.
///
/// Every artifact this process publishes for a card the deck authored takes
/// its identity from that deck's own `DeckPlan`. Two callers have no such plan
/// to read and must still name a family instance canonically, and both go
/// through here rather than formatting `sp-001` or `fft-002` by hand:
///
/// - a command-line analysis mode (`--sparam`, `--monte-carlo`) publishes an
///   analysis the deck never authored, so there is no planned card for it. It
///   is single by construction and is therefore planned on its own.
/// - the FFT RAW artifact decoder validates a file this process did not
///   necessarily write, so it has only the artifact's own declared result
///   count to mint the identities it checks against.
///
/// `AnalysisInstanceId` is deliberately not constructible outside
/// `rspice-core`; going through the planner is what keeps the tag spelling,
/// the ordinal base, and the family name decided in exactly one place.
pub(crate) fn canonical_analysis_identities(
    kind: rspice_core::execution::AnalysisKind,
    count: usize,
) -> Result<Vec<AnalysisInstanceId>, DeckPlanError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let requests = std::iter::repeat_with(|| rspice_core::execution::AnalysisRequest::new(kind))
        .take(count)
        .collect::<Vec<_>>();
    Ok(DeckPlan::new(Vec::new(), requests)?
        .analyses()
        .iter()
        .map(rspice_core::execution::PlannedAnalysis::id)
        .collect())
}

/// Canonical identity of an analysis the command line requested and the deck
/// never authored, such as `--sparam` or `--monte-carlo`.
fn command_line_analysis_identity(
    kind: rspice_core::execution::AnalysisKind,
) -> Result<AnalysisInstanceId, DeckPlanError> {
    canonical_analysis_identities(kind, 1)?
        .first()
        .copied()
        .ok_or(DeckPlanError::MissingUpstreamAnalysis {
            card: "command-line analysis mode",
            required: "one planned analysis instance",
        })
}

/// The planned `.FFT` identities of one deck, in authored card order.
fn planned_fft_instances(
    post_processes: &[PlannedPostProcess],
) -> impl Iterator<Item = AnalysisInstanceId> + '_ {
    post_processes
        .iter()
        .filter(|post| matches!(post.source(), PostProcessSource::Fft { .. }))
        .map(PlannedPostProcess::id)
}

/// Canonical artifact tags of the deck's authored `.FFT` cards.
///
/// The plan is the only source: an authored `.FFT` card the plan did not name
/// would otherwise publish under a tag this process invented, and the same
/// spectrum would carry two identities across surfaces.
fn planned_fft_ids(
    post_processes: &[PlannedPostProcess],
    netlist: &Netlist,
) -> Result<Vec<String>, CliError> {
    let ids = planned_fft_instances(post_processes)
        .map(|id| id.tag())
        .collect::<Vec<_>>();
    if ids.len() != netlist.fft_analyses.len() {
        return Err(CliError::InternalError {
            message: format!(
                "the deck authors {} .FFT card(s) but the canonical plan named {}",
                netlist.fft_analyses.len(),
                ids.len()
            ),
        });
    }
    Ok(ids)
}

/// What one concrete deck run publishes under: its canonical analysis
/// identities and, for an axis-expanded run, its coordinate.
struct RunIdentity<'a> {
    /// Coordinate of an axis-expanded run. A scalar deck's single trivial
    /// coordinate deliberately does not namespace artifact paths, so this
    /// stays `None` there.
    coordinate: Option<&'a RunCoordinate>,
    /// Structural fingerprint the coordinate's materialization computed. A
    /// scalar deck computes its own on demand.
    topology: Option<rspice_core::execution::TopologyFingerprint>,
    analyses: PlannedAnalysisIdentities,
}

fn cancellation_cli_error(timeout_seconds: Option<f64>) -> CliError {
    match crate::abort::reason() {
        Some(crate::abort::AbortReason::Interrupt) => CliError::Interrupted,
        Some(crate::abort::AbortReason::Timeout) => CliError::TimedOut {
            seconds: timeout_seconds.unwrap_or(0.0),
        },
        None => CliError::InternalError {
            message: "operation was cancelled without a recorded process abort reason".to_string(),
        },
    }
}

fn map_cancellable_parse_error(
    error: rspice_core::netlist::ParseWithAbortError,
    timeout_seconds: Option<f64>,
) -> CliError {
    match error {
        rspice_core::netlist::ParseWithAbortError::Aborted => {
            cancellation_cli_error(timeout_seconds)
        }
        rspice_core::netlist::ParseWithAbortError::Parse(error) => {
            crate::commands::map_parse_error(error)
        }
    }
}

fn map_multi_run_error(
    error: rspice_core::netlist::multi_run::MultiRunError,
    timeout_seconds: Option<f64>,
) -> CliError {
    if error.is_aborted() {
        return cancellation_cli_error(timeout_seconds);
    }
    let suggestion = error.resource_limit_error().map_or_else(
        || Some("fix the .DATA table or its DATA=<name> reference".to_string()),
        |limit| {
            Some(format!(
                "reduce the workload or raise resources.max_{} above {}",
                limit.resource.as_str(),
                limit.requested
            ))
        },
    );
    CliError::ParseError {
        message: error.to_string(),
        line: None,
        suggestion,
    }
}

struct DeckOutcome {
    reports: Vec<SimulationReport>,
    outputs: Vec<PathBuf>,
}

pub fn execute(args: RunArgs, config: &Config, verbose: bool, quiet: bool) -> Result<(), CliError> {
    let from_stdin = crate::commands::is_stdin(&args.input);
    if !from_stdin && !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    crate::abort::install_interrupt_handler();
    validate_run_numeric_args(&args)?;
    // Held for the whole cancellable region. Dropping it on any exit path
    // closes the completion latch, so a deadline that expires after the run
    // is already over cannot announce a cancellation that never happened.
    let _timeout = args.timeout.map(crate::abort::arm_timeout);

    // A run that was killed (rather than cancelled) cannot clean up after
    // itself, so its staging files stay in the output directory. Reclaim the
    // ones whose writer is gone before this run starts staging its own.
    if let Some(output) = resolve_output_path(args.output.clone(), config)? {
        let directory = output
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf);
        publish::recover_stale_artifacts(&directory, quiet);
    }

    let resource_limits = config.resources.limits();
    let parse_options = parse_options_for_run(&args, resource_limits);
    log::info!("Loading netlist: {}", args.input.display());
    let source = if from_stdin {
        crate::commands::read_stdin_source_with_limits_and_abort(
            resource_limits,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_cancellable_parse_error(error, args.timeout))?
    } else {
        Netlist::read_source_with_options_and_abort(
            &args.input,
            parse_options,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_cancellable_parse_error(error, args.timeout))?
    };

    // HSPICE `.ALTER` / `.DATA` constructs expand into several concrete
    // runs; a plain deck passes through as a single unlabeled run.
    let plan = rspice_core::netlist::multi_run::try_expand_multi_run_with_limits_and_abort(
        &source,
        resource_limits,
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| map_multi_run_error(error, args.timeout))?;
    let multi_run = plan.len() > 1;
    if multi_run {
        // One Xyce-compatible sibling name cannot safely represent several
        // rewritten .ALTER/.DATA decks. Preflight every outer variant and its
        // complete Cartesian child-run count before workers start, so a late
        // materialization error or aggregate-budget failure cannot leave an
        // earlier variant's artifact behind.
        let mut concrete_runs = 0usize;
        for deck in &plan {
            let netlist = load_netlist_from_source(&deck.source, &args, config, false)?;
            let deck_runs = preflight_deck_run_count(&netlist, &args, config)?;
            concrete_runs =
                concrete_runs
                    .checked_add(deck_runs)
                    .ok_or_else(|| CliError::ResourceLimit {
                        path: args.input.clone(),
                        source: rspice_core::ResourceLimitError {
                            resource: rspice_core::ResourceKind::BatchRuns,
                            requested: usize::MAX,
                            limit: resource_limits.max_batch_runs,
                        },
                    })?;
            if netlist
                .options
                .add_resistors
                .as_ref()
                .is_some_and(|policy| !policy.is_empty())
            {
                return Err(CliError::InvalidArgument {
                    message: ".PREPROCESS ADDRESISTORS is ambiguous in a multi-run .ALTER/.DATA deck"
                        .to_string(),
                    suggestion: Some(
                        "run each expanded deck separately so each has its own <input>_xyce.cir artifact"
                            .to_string(),
                    ),
                });
            }
        }
        if concrete_runs > resource_limits.max_batch_runs {
            return Err(CliError::ResourceLimit {
                path: args.input.clone(),
                source: rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::BatchRuns,
                    requested: concrete_runs,
                    limit: resource_limits.max_batch_runs,
                },
            });
        }
    }
    if multi_run && !quiet {
        println!(
            "Multi-run deck: {} runs (.alter/.data expansion)",
            plan.len()
        );
    }

    let start_time = Instant::now();
    let mut reports = Vec::with_capacity(plan.len());
    let mut outputs: Vec<PathBuf> = Vec::new();
    // Both the message and the typed details a failing report published, so
    // the plan-level exit status keeps the category a single-deck run would
    // have produced.
    let mut first_error: Option<(String, Option<crate::cli::ErrorDetails>)> = None;

    let workers = effective_jobs(args.jobs, plan.len(), config.resources.max_parallel_workers)?;
    if workers > 1 {
        // Parallel multi-run execution: every run is independent (own
        // parse, own engine, tagged output files). Per-run console
        // output is silenced — interleaved analysis chatter from N
        // workers is noise — and replaced by ordered status lines.
        if !quiet {
            println!("Running {} runs on {workers} workers", plan.len());
        }
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(|e| CliError::InternalError {
                message: format!("failed to build the multi-run worker pool: {e}"),
            })?;
        let outcomes: Vec<Result<DeckOutcome, CliError>> = pool.install(|| {
            use rayon::prelude::*;
            plan.par_iter()
                .map(|deck| {
                    let netlist = load_netlist_from_source(&deck.source, &args, config, false)?;
                    run_deck(&netlist, &args, config, false, true, deck.label.as_deref())
                })
                .collect()
        });
        for (deck, outcome) in plan.iter().zip(outcomes) {
            let label = deck.label.as_deref().unwrap_or("base");
            let outcome = outcome?;
            if !quiet {
                if outcome.reports.iter().all(|report| report.passed) {
                    let duration: f64 = outcome
                        .reports
                        .iter()
                        .map(|report| report.duration_secs)
                        .sum();
                    println!("  ✓ {label} ({duration:.3}s)");
                } else {
                    let failure = outcome
                        .reports
                        .iter()
                        .find(|report| !report.passed)
                        .ok_or_else(|| CliError::InternalError {
                            message: format!(
                                "multi-run aggregate for '{label}' reported failure without a failed child report"
                            ),
                        })?;
                    println!("  ✗ {label}: {}", status_failure_summary(failure));
                }
            }
            if first_error.is_none() {
                first_error = first_reported_failure(&outcome.reports);
            }
            reports.extend(outcome.reports);
            outputs.extend(outcome.outputs);
        }
    } else {
        for deck in &plan {
            if multi_run && !quiet {
                println!("\n=== run: {} ===", deck.label.as_deref().unwrap_or("base"));
            }
            let netlist = load_netlist_from_source(&deck.source, &args, config, !quiet)?;
            validate_pss_flag_conflict(&netlist, &args)?;
            refuse_unsupported_deck_analyses(&netlist, config, &args)?;
            validate_step_frontend_compatibility(&netlist, &args)?;
            let addresistors_artifact =
                materialize_addresistors_artifact(&netlist, &args.input, from_stdin, args.timeout)?;
            let outcome = run_deck(
                &netlist,
                &args,
                config,
                verbose,
                quiet,
                deck.label.as_deref(),
            )?;
            if first_error.is_none() {
                first_error = first_reported_failure(&outcome.reports);
            }
            reports.extend(outcome.reports);
            outputs.extend(outcome.outputs);
            if let Some(path) = addresistors_artifact {
                outputs.push(path);
            }
        }
    }

    let duration = start_time.elapsed().as_secs_f64();
    let abort_reason = crate::abort::reason();
    if let Some(reason) = abort_reason {
        ensure_cancellation_report(&mut reports, &args.input, args.timeout, reason);
    }
    write_report_files(&reports, &args, verbose)?;

    let failed_measurements: Vec<&str> = reports
        .iter()
        .flat_map(|report| &report.measurements)
        .filter(|meas| !meas.passed)
        .map(|meas| meas.name.as_str())
        .collect();
    let passed = first_error.is_none()
        && abort_reason.is_none()
        && (failed_measurements.is_empty() || args.allow_failed_meas);

    if let Some(ref summary_path) = args.summary {
        outputs.dedup();
        write_run_summary(
            summary_path,
            &args,
            &reports,
            &outputs,
            duration,
            passed,
            abort_reason,
            resource_limits,
            workers,
        )?;
    }

    // An abort outranks the per-analysis errors it caused: report files are
    // already on disk, but the exit status says interrupted/timed out.
    match abort_reason {
        Some(crate::abort::AbortReason::Interrupt) => return Err(CliError::Interrupted),
        Some(crate::abort::AbortReason::Timeout) => {
            return Err(CliError::TimedOut {
                seconds: args.timeout.unwrap_or(0.0),
            });
        }
        None => {}
    }

    if !quiet {
        println!("\nSimulation complete in {:.3}s.", duration);
    }

    if let Some((message, details)) = first_error {
        return Err(CliError::reported(message, details));
    }

    // The simulation itself succeeded; failed .MEAS checks still fail the
    // process so automation can trust the exit status.
    if !failed_measurements.is_empty() && !args.allow_failed_meas {
        return Err(CliError::VerificationFailed {
            message: format!(
                "{} measurement(s) failed: {}",
                failed_measurements.len(),
                failed_measurements.join(", ")
            ),
        });
    }

    Ok(())
}

/// Ensure every machine-readable report family carries the same cancellation
/// verdict as the process exit and JSON summary. Completed coordinate reports
/// remain available, while a distinct failed record prevents partial work
/// from being misclassified as a passing CI run.
fn ensure_cancellation_report(
    reports: &mut Vec<SimulationReport>,
    input: &std::path::Path,
    timeout_seconds: Option<f64>,
    reason: crate::abort::AbortReason,
) {
    let error = match reason {
        crate::abort::AbortReason::Interrupt => CliError::Interrupted,
        crate::abort::AbortReason::Timeout => CliError::TimedOut {
            seconds: timeout_seconds.unwrap_or(0.0),
        },
    };
    let error_message = error.to_string();
    let run_status_measurement = || MeasurementReport {
        name: "__rspice_run_status__".to_string(),
        value: None,
        raw_value: None,
        expected: None,
        tolerance: None,
        failure_limit: None,
        failure_limit_exceeded: false,
        passed: false,
        error: Some(error_message.clone()),
        record_index: None,
        event_axis: None,
        trigger_axis: None,
        target_axis: None,
        aggregate_policy: None,
    };
    if let Some(report) = reports.iter_mut().find(|report| {
        report
            .error_details
            .as_ref()
            .is_some_and(|details| details.category == "cancellation")
    }) {
        report.passed = false;
        if !report
            .measurements
            .iter()
            .any(|measurement| measurement.name == "__rspice_run_status__")
        {
            report.measurements.push(run_status_measurement());
        }
        return;
    }

    let base_name = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| *stem != "-")
        .unwrap_or("stdin");
    let label = match reason {
        crate::abort::AbortReason::Interrupt => "interrupted",
        crate::abort::AbortReason::Timeout => "timed-out",
    };
    reports.push(SimulationReport {
        name: format!("{base_name} [{label}]"),
        netlist: input.display().to_string(),
        passed: false,
        duration_secs: 0.0,
        error: Some(error_message.clone()),
        error_details: Some(error.details()),
        // Measurement-only JSON/CSV artifacts do not serialize report-level
        // failures. A reserved run-status record keeps cancellation visible
        // and failed in those formats as well as JUnit/TAP and summaries.
        measurements: vec![run_status_measurement()],
    });
}

fn materialize_addresistors_artifact(
    netlist: &Netlist,
    input: &std::path::Path,
    from_stdin: bool,
    timeout_seconds: Option<f64>,
) -> Result<Option<PathBuf>, CliError> {
    if netlist
        .options
        .add_resistors
        .as_ref()
        .is_none_or(|policy| policy.is_empty())
    {
        return Ok(None);
    }
    if from_stdin {
        return Err(CliError::InvalidArgument {
            message: ".PREPROCESS ADDRESISTORS requires a file-backed input; stdin has no unambiguous sibling artifact path"
                .to_string(),
            suggestion: Some(
                "save the deck to a file and run `rspice run <file>` to create <file>_xyce.cir"
                    .to_string(),
            ),
        });
    }

    let materialized = netlist
        .materialize_xyce_add_resistors_with_abort(&crate::abort::ProcessAbort)
        .map_err(|source| {
            if matches!(
                source,
                rspice_core::netlist::XyceAddResistorsMaterializationError::Aborted
            ) {
                match crate::abort::reason() {
                    Some(crate::abort::AbortReason::Interrupt) => return CliError::Interrupted,
                    Some(crate::abort::AbortReason::Timeout) => {
                        return CliError::TimedOut {
                            seconds: timeout_seconds.unwrap_or(0.0),
                        };
                    }
                    None => {}
                }
            }
            CliError::AddResistorsMaterialization { source }
        })?;
    let path = xyce_addresistors_artifact_path(input);
    publish::artifact(&path, |writer| {
        writer.write_all(materialized.derived_source.as_bytes())
    })
    .map_err(|source| CliError::AddResistorsArtifactIo {
        path: path.clone(),
        source,
    })?;
    Ok(Some(path))
}

fn xyce_addresistors_artifact_path(input: &std::path::Path) -> PathBuf {
    let mut name = input.as_os_str().to_os_string();
    name.push("_xyce.cir");
    PathBuf::from(name)
}

fn validate_run_numeric_args(args: &RunArgs) -> Result<(), CliError> {
    if matches!(args.maxiter, Some(0)) {
        return Err(invalid_run_arg(
            "--maxiter",
            "must be at least 1",
            "e.g. --maxiter 100",
        ));
    }

    require_celsius_arg("--temp", args.temp)?;
    require_positive_arg("--timeout", args.timeout)?;
    require_positive_arg("--tran-stop", args.tran_stop)?;
    require_positive_arg("--compress-tol", args.compress_tol)?;
    require_positive_arg("--abstol", args.abstol)?;
    require_positive_arg("--reltol", args.reltol)?;
    require_positive_arg("--residual-reltol", args.residual_reltol)?;
    require_positive_arg("--min-step", args.min_step)?;
    require_positive_arg("--max-step", args.max_step)?;
    if let (Some(min_step), Some(max_step)) = (args.min_step, args.max_step)
        && min_step > max_step
    {
        return Err(invalid_run_arg(
            "--min-step/--max-step",
            &format!("must satisfy --min-step <= --max-step, got {min_step} > {max_step}"),
            "set --min-step less than or equal to --max-step",
        ));
    }
    require_positive_arg("--trtol", args.trtol)?;
    require_non_negative_arg("--gmin", args.gmin)?;
    require_positive_arg("--voltage-abstol", args.voltage_abstol)?;
    require_positive_arg("--current-abstol", args.current_abstol)?;
    require_positive_arg("--charge-abstol", args.charge_abstol)?;
    require_non_negative_arg("--mc-spread", args.mc_spread)?;

    Ok(())
}

fn require_positive_arg(name: &str, value: Option<f64>) -> Result<(), CliError> {
    if let Some(value) = value
        && (!value.is_finite() || value <= 0.0)
    {
        return Err(invalid_run_arg(
            name,
            &format!("must be a positive finite number, got {value}"),
            "use a positive SPICE value",
        ));
    }
    Ok(())
}

fn require_non_negative_arg(name: &str, value: Option<f64>) -> Result<(), CliError> {
    if let Some(value) = value
        && (!value.is_finite() || value < 0.0)
    {
        return Err(invalid_run_arg(
            name,
            &format!("must be a finite non-negative number, got {value}"),
            "use 0 or a positive SPICE value",
        ));
    }
    Ok(())
}

fn require_celsius_arg(name: &str, value: Option<f64>) -> Result<(), CliError> {
    if let Some(value) = value
        && (!value.is_finite() || value <= -273.15)
    {
        return Err(invalid_run_arg(
            name,
            &format!("must be finite and above absolute zero, got {value} C"),
            "use a Celsius value greater than -273.15",
        ));
    }
    Ok(())
}

fn invalid_run_arg(name: &str, message: &str, suggestion: &str) -> CliError {
    CliError::InvalidArgument {
        message: format!("{name} {message}"),
        suggestion: Some(suggestion.to_string()),
    }
}

/// Write the one-artifact JSON contract for automation: tool identity,
/// per-run status with every measurement, and the overall verdict that the
/// exit code will reflect. `-` writes to stdout.
fn write_run_summary(
    path: &std::path::Path,
    args: &RunArgs,
    reports: &[SimulationReport],
    outputs: &[PathBuf],
    duration: f64,
    passed: bool,
    abort_reason: Option<crate::abort::AbortReason>,
    resource_limits: rspice_core::ResourceLimits,
    workers: usize,
) -> Result<(), CliError> {
    let status = match abort_reason {
        Some(crate::abort::AbortReason::Interrupt) => "interrupted",
        Some(crate::abort::AbortReason::Timeout) => "timed_out",
        None if passed => "passed",
        None => "failed",
    };
    let passed_runs = reports.iter().filter(|report| report.passed).count();
    let measurement_count = reports
        .iter()
        .map(|report| report.measurements.len())
        .sum::<usize>();
    let passed_measurements = reports
        .iter()
        .flat_map(|report| &report.measurements)
        .filter(|measurement| measurement.passed)
        .count();
    let json = serde_json::json!({
        "schema_version": 1,
        "tool": {
            "name": "rspice",
            "version": env!("CARGO_PKG_VERSION"),
            "target": env!("RSPICE_BUILD_TARGET"),
            "profile": env!("RSPICE_BUILD_PROFILE"),
            "commit": env!("RSPICE_BUILD_COMMIT"),
        },
        "run_id": crate::observability::run_id(),
        "netlist": args.input.display().to_string(),
        "status": status,
        "duration_secs": duration,
        "passed": passed,
        "execution": {
            "requested_jobs": args.jobs,
            "workers": workers,
            "parallel": workers > 1,
        },
        "counts": {
            "runs": reports.len(),
            "passed_runs": passed_runs,
            "failed_runs": reports.len().saturating_sub(passed_runs),
            "measurements": measurement_count,
            "passed_measurements": passed_measurements,
            "failed_measurements": measurement_count.saturating_sub(passed_measurements),
            "outputs": outputs.len(),
        },
        "resource_limits": resource_limits_summary(resource_limits),
        "outputs": outputs.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        "aborted": abort_reason.map(|reason| match reason {
            crate::abort::AbortReason::Interrupt => "interrupt",
            crate::abort::AbortReason::Timeout => "timeout",
        }),
        "runs": reports.iter().map(|report| {
            serde_json::json!({
                "name": report.name,
                "passed": report.passed,
                "error": report.error,
                "error_details": report.error_details,
                "duration_secs": report.duration_secs,
                "measurements": report.measurements.iter().map(|meas| {
                    serde_json::json!({
                        "name": meas.name,
                        "value": meas.value,
                        "expected": meas.expected,
                        "tolerance": meas.tolerance,
                        "passed": meas.passed,
                        "error": meas.error,
                    })
                }).collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>(),
    });

    if path.as_os_str() == "-" {
        match serde_json::to_string_pretty(&json) {
            Ok(text) => println!("{text}"),
            Err(e) => eprintln!("Error: failed to serialize run summary: {e}"),
        }
        return Ok(());
    }

    let text =
        serde_json::to_string_pretty(&json).map_err(|e| CliError::output_json_error(path, e))?;
    let document = text + "\n";
    publish::artifact(path, |writer| {
        writer
            .write_all(document.as_bytes())
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))?;
    Ok(())
}

fn resource_limits_summary(limits: rspice_core::ResourceLimits) -> serde_json::Value {
    serde_json::json!({
        "max_netlist_bytes": limits.max_netlist_bytes,
        "max_netlist_lines": limits.max_netlist_lines,
        "max_expanded_source_bytes": limits.max_expanded_source_bytes,
        "max_dependency_source_bytes": limits.max_dependency_source_bytes,
        "max_external_data_bytes": limits.max_external_data_bytes,
        "max_external_data_values": limits.max_external_data_values,
        "max_shared_cache_bytes": limits.max_shared_cache_bytes,
        "max_include_depth": limits.max_include_depth,
        "max_hierarchy_depth": limits.max_hierarchy_depth,
        "max_flattened_elements": limits.max_flattened_elements,
        "max_circuit_nodes": limits.max_circuit_nodes,
        "max_matrix_unknowns": limits.max_matrix_unknowns,
        "max_analysis_points": limits.max_analysis_points,
        "max_result_values": limits.max_result_values,
        "max_parallel_workers": limits.max_parallel_workers,
        "max_batch_runs": limits.max_batch_runs,
    })
}

fn requested_mode_name(args: &RunArgs) -> Option<&'static str> {
    if args.monte_carlo.is_some() {
        Some("--monte-carlo")
    } else if args.pss_freq.is_some() {
        Some("--pss-freq")
    } else if args.hb_freq.is_some() {
        Some("--hb-freq")
    } else if args.pz_input.is_some() || args.pz_output.is_some() {
        Some("--pz-input/--pz-output")
    } else if args.sens_output.is_some() || args.sens_param.is_some() {
        Some("--sens-output/--sens-param")
    } else if args.sparam.is_some() {
        Some("--sparam")
    } else if args.corners.is_some() {
        Some("--corners")
    } else {
        None
    }
}

/// Whether one authored card post-processes an already completed transient
/// instead of driving the solver itself.
pub(crate) const fn is_transient_post_process(analysis: &AnalysisCommand) -> bool {
    matches!(analysis, AnalysisCommand::Four { .. })
}

/// Authored cards in execution order: every physical analysis first, then the
/// transient post-processors.
///
/// A `.FOUR` card is source-order independent in SPICE decks, so it must
/// consume the deck's final authored transient even when the card precedes
/// `.TRAN`. This is the one place that ordering is decided; every executor
/// walks the deck through it.
pub(crate) fn analyses_in_execution_order(
    netlist: &Netlist,
) -> impl Iterator<Item = &AnalysisCommand> {
    netlist
        .analyses
        .iter()
        .filter(|analysis| !is_transient_post_process(analysis))
        .chain(
            netlist
                .analyses
                .iter()
                .filter(|analysis| is_transient_post_process(analysis)),
        )
}

/// Signature symbol of one authored card under a run axis.
///
/// A run axis contributes nothing: it decorates the deck rather than naming a
/// child analysis. `.FOUR` does contribute even though `DeckPlan` mints no
/// planned slot for it and it owns no physical output namespace, because a
/// conditional that adds or drops a Fourier card changes what a coordinate
/// publishes; it is marked as a post-process entry so it cannot be mistaken
/// for a planned physical analysis.
fn step_analysis_signature_kind(
    analysis: &AnalysisCommand,
) -> Result<Option<&'static str>, CliError> {
    if unsupported_deck_analysis_card(analysis).is_some() {
        return Err(unsupported_deck_analysis_error(analysis, None));
    }
    if matches!(analysis, AnalysisCommand::MonteCarlo(_)) {
        return Err(CliError::InvalidArgument {
            message: ".STEP cannot wrap authored Monte Carlo until deterministic nested seed/substream derivation is configured"
                .to_string(),
            suggestion: Some(
                "run the parameter coordinates or Monte Carlo campaign as the outer experiment, but not both in one deck"
                    .to_string(),
            ),
        });
    }
    Ok(match analysis {
        AnalysisCommand::Four { .. } => Some(POST_PROCESS_FOURIER_SIGNATURE),
        other => analysis_output_tag(other),
    })
}

/// Signature symbol of a `.FOUR` card. It is deliberately not an output tag:
/// `.FOUR` publishes under the post-process instance identity of the transient
/// it consumes, never under a physical analysis namespace.
const POST_PROCESS_FOURIER_SIGNATURE: &str = "post-process:four";

fn step_commands(netlist: &Netlist) -> Vec<rspice_core::netlist::StepCommand> {
    netlist
        .analyses
        .iter()
        .filter_map(|analysis| match analysis {
            AnalysisCommand::Step(step) => Some(step.clone()),
            _ => None,
        })
        .collect()
}

fn step_analysis_signature(netlist: &Netlist) -> Result<Vec<&'static str>, CliError> {
    let mut signature = Vec::new();
    for analysis in &netlist.analyses {
        let Some(kind) = step_analysis_signature_kind(analysis)? else {
            continue;
        };
        signature.push(kind);
    }
    Ok(signature)
}

fn validate_step_frontend_compatibility(netlist: &Netlist, args: &RunArgs) -> Result<(), CliError> {
    let steps = step_commands(netlist);
    let has_temperature_axis = netlist
        .analyses
        .iter()
        .any(|analysis| matches!(analysis, AnalysisCommand::Temp { .. }));
    if steps.is_empty() && !has_temperature_axis {
        return Ok(());
    }
    if let Some(mode) = requested_mode_name(args) {
        return Err(CliError::InvalidArgument {
            message: format!("{mode} cannot be combined with authored .STEP/.TEMP run axes"),
            suggestion: Some(
                "encode the desired supported physical child analysis in the deck".to_string(),
            ),
        });
    }
    if netlist
        .options
        .add_resistors
        .as_ref()
        .is_some_and(|policy| !policy.is_empty())
    {
        return Err(CliError::InvalidArgument {
            message: ".PREPROCESS ADDRESISTORS cannot emit one canonical artifact for a .STEP deck"
                .to_string(),
            suggestion: Some(
                "materialize each coordinate separately when generating ADDRESISTORS artifacts"
                    .to_string(),
            ),
        });
    }

    for step in &steps {
        shared::validate_step_sweep(&step.sweep)?;
    }
    let signature = step_analysis_signature(netlist)?;
    if (args.checkpoint.is_some() || args.resume.is_some()) && !signature.contains(&"tran") {
        return Err(CliError::InvalidArgument {
            message: ".STEP --checkpoint/--resume requires an authored .TRAN child analysis"
                .to_string(),
            suggestion: Some(
                "add at least one .TRAN card or remove the transient checkpoint option".to_string(),
            ),
        });
    }
    Ok(())
}

fn map_step_core_error(
    error: rspice_core::SimulationError,
    timeout_seconds: Option<f64>,
    analysis: impl Into<String>,
) -> CliError {
    if matches!(error, rspice_core::SimulationError::Aborted) {
        cancellation_cli_error(timeout_seconds)
    } else {
        CliError::CoreSimulationError {
            source: error,
            analysis: Some(analysis.into()),
        }
    }
}

fn map_deck_plan_error(error: DeckPlanError, args: &RunArgs) -> CliError {
    match error {
        DeckPlanError::Aborted => cancellation_cli_error(args.timeout),
        DeckPlanError::ResourceLimit(source) => map_step_core_error(
            rspice_core::SimulationError::ResourceLimit(source),
            args.timeout,
            "Run-axis planning",
        ),
        error => CliError::InvalidArgument {
            message: format!("canonical .STEP/.TEMP planning failed: {error}"),
            suggestion: Some(
                "fix the run-axis definition before any coordinate is simulated".to_string(),
            ),
        },
    }
}

fn map_materialized_run_error(
    error: MaterializedRunError,
    args: &RunArgs,
    analysis: impl Into<String>,
) -> CliError {
    match error {
        MaterializedRunError::Aborted => cancellation_cli_error(args.timeout),
        MaterializedRunError::DeckPlan(error) => map_deck_plan_error(error, args),
        MaterializedRunError::Simulation(error) => {
            map_step_core_error(error, args.timeout, analysis)
        }
        error => CliError::InternalError {
            message: format!("canonical deck materialization failed: {error}"),
        },
    }
}

fn canonical_coordinate_description(coordinate: &RunCoordinate) -> String {
    coordinate
        .assignments()
        .iter()
        .map(|assignment| match assignment.value() {
            RunAxisValue::Numeric(value) => {
                format!("{} = {value}", canonical_assignment_target(assignment))
            }
            RunAxisValue::DataRow(bindings) => format!(
                "DATA {} row {} ({})",
                assignment.name(),
                assignment.value_index() + 1,
                bindings
                    .iter()
                    .map(|binding| format!("{}={}", binding.name(), binding.value()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            RunAxisValue::AlterVariant { label, .. } => format!("ALTER {label}"),
            unsupported => format!(
                "{} = {unsupported:?}",
                canonical_assignment_target(assignment)
            ),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn canonical_assignment_target(assignment: &AxisAssignment) -> String {
    match assignment.step_target() {
        Some(StepAxisTarget::Parameter { name }) => format!("PARAM {name}"),
        Some(StepAxisTarget::Device {
            name,
            parameter: Some(parameter),
        }) => format!("DEVICE {name}.{parameter}"),
        Some(StepAxisTarget::Device {
            name,
            parameter: None,
        }) => format!("DEVICE {name}"),
        Some(StepAxisTarget::Model { name, parameter }) => {
            format!("MODEL {name}.{parameter}")
        }
        Some(StepAxisTarget::Temperature) | None if assignment.kind() == AxisKind::Temperature => {
            "TEMP".to_string()
        }
        None | Some(_) => assignment.name().to_string(),
    }
}

/// The contract one axis coordinate declared before any solver work began.
///
/// The preflight materializes every coordinate once, so this is where its
/// child-analysis signature and its structural fingerprint are captured. The
/// execution pass re-materializes to run, and checks what it gets against this
/// rather than re-deriving it, so a coordinate whose topology moved between the
/// two passes fails instead of publishing under a contract it no longer meets.
struct StepCoordinateContract {
    signature: Vec<&'static str>,
    topology: rspice_core::execution::TopologyFingerprint,
}

fn preflight_step_coordinates(
    engine: &Engine,
    materializer: &DeckPlanMaterializer<'_>,
    base_signature: &[&'static str],
    aggregate_report_values: Option<usize>,
    args: &RunArgs,
) -> Result<Vec<StepCoordinateContract>, CliError> {
    let mut contracts = Vec::with_capacity(materializer.len());
    let mut retained_report_values = aggregate_report_values.unwrap_or(0);
    let retained_limit = engine.config().resource_limits.max_result_values;
    if retained_report_values > retained_limit {
        return Err(map_step_core_error(
            rspice_core::SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::ResultValues,
                requested: retained_report_values,
                limit: retained_limit,
            }),
            args.timeout,
            "Step reporting preflight",
        ));
    }
    for (run_index, canonical_coordinate) in materializer.coordinates().iter().enumerate() {
        let coordinate = canonical_coordinate_description(canonical_coordinate);
        let materialized = materializer
            .materialize_run_with_abort(run_index, &crate::abort::ProcessAbort)
            .map_err(|error| {
                map_materialized_run_error(
                    error,
                    args,
                    format!(
                        ".STEP {} ({coordinate}) preflight",
                        canonical_coordinate.stable_tag()
                    ),
                )
            })?;
        let signature = step_analysis_signature(materialized.netlist())?;
        if signature != base_signature {
            return Err(CliError::InvalidArgument {
                message: format!(
                    ".STEP coordinate {} ({coordinate}) conditionally changes the child-analysis signature from {:?} to {:?}",
                    run_index + 1,
                    base_signature,
                    signature
                ),
                suggestion: Some(
                    "keep the authored physical-analysis and post-processing card set unconditional across every coordinate"
                        .to_string(),
                ),
            });
        }
        contracts.push(StepCoordinateContract {
            signature,
            topology: materialized.topology_fingerprint(),
        });

        // Each report retains one duration plus up to value/goal/tolerance
        // for every measurement. Bound the numeric reporting payload before
        // any solver or output file starts.
        if aggregate_report_values.is_none() {
            retained_report_values = retained_report_values
                .saturating_add(1)
                .saturating_add(materialized.netlist().measurements.len().saturating_mul(3));
            if retained_report_values > retained_limit {
                return Err(map_step_core_error(
                    rspice_core::SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
                        resource: rspice_core::ResourceKind::ResultValues,
                        requested: retained_report_values,
                        limit: retained_limit,
                    }),
                    args.timeout,
                    "Step reporting preflight",
                ));
            }
        }
    }
    Ok(contracts)
}

/// Preflight one already-expanded `.ALTER`/textual-`.DATA` variant without
/// solving it or publishing output. The returned count is the number of
/// concrete Cartesian coordinates this outer variant will execute.
fn preflight_deck_run_count(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
) -> Result<usize, CliError> {
    validate_step_frontend_compatibility(netlist, args)?;

    let resource_limits = config.resources.limits();
    let canonical_plan =
        DeckPlan::from_netlist_with_abort(netlist, &resource_limits, &crate::abort::ProcessAbort)
            .map_err(|error| map_deck_plan_error(error, args))?;
    if canonical_plan.axes().is_empty() {
        return Ok(1);
    }

    let base_signature = step_analysis_signature(netlist)?;
    let engine = Engine::try_new(build_sim_config(args, config, netlist))?;
    let materializer = engine
        .prepare_deck_plan_materializer_with_abort(
            netlist,
            &canonical_plan,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_materialized_run_error(error, args, "Step planning preflight"))?;
    let aggregate_report_values = base_signature
        .is_empty()
        .then(|| 1usize.saturating_add(netlist.measurements.len().saturating_mul(3)));
    preflight_step_coordinates(
        &engine,
        &materializer,
        &base_signature,
        aggregate_report_values,
        args,
    )?;
    Ok(materializer.len())
}

fn compose_run_label(outer: Option<&str>, inner: Option<&str>) -> Option<String> {
    match (outer, inner) {
        (Some(outer), Some(inner)) => Some(format!("{outer} · {inner}")),
        (Some(label), None) | (None, Some(label)) => Some(label.to_string()),
        (None, None) => None,
    }
}

fn implicit_axis_assignment(coordinate: &RunCoordinate) -> Result<&AxisAssignment, CliError> {
    let [assignment] = coordinate.assignments() else {
        return Err(CliError::InternalError {
            message: format!(
                "implicit operating-point aggregation requires exactly one run axis, but coordinate {} has {}",
                coordinate.stable_tag(),
                coordinate.assignments().len()
            ),
        });
    };
    Ok(assignment)
}

fn implicit_axis_value(assignment: &AxisAssignment) -> Result<f64, CliError> {
    match assignment.value() {
        RunAxisValue::Numeric(value) => Ok(*value),
        RunAxisValue::DataRow(_) => Ok(assignment.value_index() as f64),
        RunAxisValue::AlterVariant { .. } => Err(CliError::InvalidArgument {
            message: "textual .ALTER cannot drive an implicit operating-point table".to_string(),
            suggestion: Some("expand each .ALTER variant before execution".to_string()),
        }),
        value => Err(CliError::InternalError {
            message: format!("unsupported implicit run-axis value {value:?}"),
        }),
    }
}

fn implicit_axis_scale_name(assignment: &AxisAssignment) -> String {
    match assignment.value() {
        RunAxisValue::DataRow(_) => format!("DATA({})", assignment.name()),
        _ => match assignment.step_target() {
            Some(StepAxisTarget::Parameter { name }) => name.clone(),
            Some(StepAxisTarget::Device {
                name,
                parameter: Some(parameter),
            })
            | Some(StepAxisTarget::Model { name, parameter }) => {
                format!("{name}:{parameter}")
            }
            Some(StepAxisTarget::Device {
                name,
                parameter: None,
            }) => name.clone(),
            Some(StepAxisTarget::Temperature) | None
                if assignment.kind() == AxisKind::Temperature =>
            {
                "TEMP".to_string()
            }
            None | Some(_) => assignment.name().to_string(),
        },
    }
}

fn run_implicit_step_op_table(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    engine: &Engine,
    plan: &DeckPlan,
    materializer: &DeckPlanMaterializer<'_>,
    run_label: Option<&str>,
) -> Result<DeckOutcome, CliError> {
    let first_coordinate =
        materializer
            .coordinates()
            .first()
            .ok_or_else(|| CliError::InternalError {
                message: "implicit deck materializer has no coordinate".to_string(),
            })?;
    let one_dimensional = first_coordinate.assignments().len() == 1;
    let (target, scale_name) = if one_dimensional {
        let first_assignment = implicit_axis_assignment(first_coordinate)?;
        (
            canonical_assignment_target(first_assignment),
            Some(implicit_axis_scale_name(first_assignment)),
        )
    } else {
        ("Cartesian run axes".to_string(), None)
    };
    // This path publishes under the implicit-OP identity each coordinate
    // carries, resolved per coordinate below. The shared context owns no
    // authored analysis namespace of its own.
    let ctx = RunContext::new(
        engine,
        netlist,
        args,
        config,
        verbose,
        quiet,
        run_label,
        RunIdentity {
            coordinate: None,
            topology: None,
            analyses: PlannedAnalysisIdentities::default(),
        },
    )?;
    let start_time = Instant::now();
    let mut retained_values = 0usize;
    let mut preflight = Vec::with_capacity(materializer.len());

    for run_index in 0..materializer.len() {
        if crate::abort::reason().is_some() {
            break;
        }
        let materialized =
            match materializer.materialize_run_with_abort(run_index, &crate::abort::ProcessAbort) {
                Ok(materialized) => materialized,
                Err(MaterializedRunError::Aborted) if crate::abort::reason().is_some() => {
                    break;
                }
                Err(error) => {
                    return Err(map_materialized_run_error(error, args, "Step"));
                }
            };
        let canonical_coordinate = materialized.coordinate();
        let value = one_dimensional
            .then(|| implicit_axis_assignment(canonical_coordinate))
            .transpose()?
            .map(implicit_axis_value)
            .transpose()?;
        let [implicit_analysis] = materialized.analyses() else {
            return Err(CliError::InternalError {
                message: format!(
                    "implicit coordinate {} materialized {} analysis identities",
                    canonical_coordinate.stable_id(),
                    materialized.analyses().len()
                ),
            });
        };
        if implicit_analysis.command().is_some() {
            return Err(CliError::InternalError {
                message: format!(
                    "implicit coordinate {} unexpectedly owns an authored analysis command",
                    canonical_coordinate.stable_id()
                ),
            });
        }
        let coordinate_engine =
            Engine::try_new(build_sim_config(args, config, materialized.netlist()))?;
        let topology = materialized.topology_fingerprint();
        let result = match coordinate_engine
            .run_dc_op_with_abort(materialized.netlist(), &crate::abort::ProcessAbort)
        {
            Ok(result) => result,
            Err(rspice_core::SimulationError::Aborted) if crate::abort::reason().is_some() => {
                break;
            }
            Err(error) => {
                let coordinate_context = value.map_or_else(
                    || canonical_coordinate_description(canonical_coordinate),
                    |value| format!("{target} = {value}"),
                );
                return Err(CliError::simulation_error_in(
                    format!(
                        ".STEP {} ({coordinate_context}): {error}",
                        canonical_coordinate.stable_tag()
                    ),
                    "Step",
                ));
            }
        };
        let signals = crate::commands::run_signals::dc_operating_point_export_signals(
            materialized.netlist(),
            &result,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| CliError::CoreSimulationError {
            source,
            analysis: Some(format!(
                ".STEP {} output-schema preflight",
                canonical_coordinate.stable_tag()
            )),
        })?;
        let schema =
            crate::commands::run_signals::scalar_signal_schema(&signals).map_err(|error| {
                CliError::CoreSimulationError {
                    source: rspice_core::SimulationError::Circuit(format!(
                        ".STEP {} has an invalid coordinate-local signal schema: {error}",
                        canonical_coordinate.stable_tag()
                    )),
                    analysis: Some("Step output-schema preflight".to_string()),
                }
            })?;
        shared::ensure_finite_series(
            args.allow_nonfinite,
            "Step",
            signals
                .iter()
                .map(|signal| (signal.display_name.as_str(), signal.values.as_slice())),
        )?;
        retained_values = retained_values
            .saturating_add(result.retained_value_count())
            .saturating_add(1);
        let retained_limit = engine.config().resource_limits.max_result_values;
        if retained_values > retained_limit {
            return Err(map_step_core_error(
                rspice_core::SimulationError::ResourceLimit(rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::ResultValues,
                    requested: retained_values,
                    limit: retained_limit,
                }),
                args.timeout,
                "Step result aggregation",
            ));
        }
        preflight.push(ImplicitStepCoordinate {
            value,
            coordinate_id: canonical_coordinate.stable_id(),
            coordinate_tag: canonical_coordinate.stable_tag(),
            canonical: canonical_coordinate.clone(),
            analysis: implicit_analysis.id(),
            analysis_id: implicit_analysis.output_namespace().analysis_component(),
            topology,
            result,
            signals,
            schema,
            validity: Vec::new(),
        });
    }
    if preflight.len() != materializer.len() {
        return Err(cancellation_cli_error(args.timeout));
    }

    let schema_union =
        rspice_core::execution::SignalSchema::union(preflight.iter().map(|run| {
            rspice_core::execution::CoordinateSchema::new(run.coordinate_id, &run.schema)
        }))
        .map_err(|error| CliError::CoreSimulationError {
            source: rspice_core::SimulationError::Circuit(format!(
                ".STEP coordinate schemas cannot form a typed union: {error}"
            )),
            analysis: Some("Step output-schema preflight".to_string()),
        })?;
    for run in &mut preflight {
        let values = run
            .signals
            .iter()
            .map(|signal| {
                signal
                    .values
                    .first()
                    .copied()
                    .ok_or_else(|| CliError::CoreSimulationError {
                        source: rspice_core::SimulationError::Circuit(format!(
                            ".STEP {} signal '{}' has no operating-point value",
                            run.coordinate_id, signal.display_name
                        )),
                        analysis: Some("Step output-schema preflight".to_string()),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let aligned = schema_union
            .align_values(run.coordinate_id, &values)
            .map_err(|error| CliError::CoreSimulationError {
                source: rspice_core::SimulationError::Circuit(format!(
                    ".STEP {} cannot align its coordinate-local values to the union schema: {error}",
                    run.coordinate_id
                )),
                analysis: Some("Step output-schema preflight".to_string()),
            })?;
        run.validity = aligned.iter().map(Option::is_some).collect();
    }

    let stable_topology_and_schema = preflight.first().is_none_or(|first| {
        preflight
            .iter()
            .all(|run| run.topology == first.topology && run.schema == first.schema)
    });
    let mut outputs = Vec::new();
    if one_dimensional && stable_topology_and_schema {
        let scale_name = scale_name
            .as_deref()
            .ok_or_else(|| CliError::InternalError {
                message: "one-dimensional implicit plan has no scale name".to_string(),
            })?;
        let results = preflight
            .iter()
            .map(|run| {
                run.value
                    .map(|value| (value, run.result.clone()))
                    .ok_or_else(|| CliError::InternalError {
                        message: format!(
                            "one-dimensional implicit coordinate {} has no scalar axis value",
                            run.coordinate_id
                        ),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        advanced::export_step_sweep(&ctx, scale_name, &results)?;
        outputs.extend(ctx.outputs.borrow().iter().cloned());
    } else if let Some(base_output) = ctx.output.clone() {
        // Flat artifacts stay coordinate-local when topology changes.  Every
        // coordinate was solved and schema-checked above, before this first
        // write, so coordinate order can neither select columns nor leave a
        // partial batch due to a late schema mismatch. The set is published
        // as one transaction, so the schema manifest can never name a
        // coordinate artifact that a cancellation left unwritten.
        let transaction = publish::begin()?;
        let mut coordinate_publications = Vec::with_capacity(preflight.len());
        let mut set_coordinates = Vec::with_capacity(preflight.len());
        for run in &preflight {
            let coordinate_path =
                tag_output_path(&base_output, &sanitize_run_tag(&run.coordinate_tag));
            let path = tag_output_path(&coordinate_path, &run.analysis_id);
            if ctx.format == OutputFormat::Json {
                // A coordinate-local implicit operating point is a result like
                // any other: it publishes the shared typed document, naming the
                // coordinate and topology that produced it.
                let builder = rspice_core::execution::AnalysisResultDocument::from_operating_point(
                    run.analysis,
                    &run.result,
                    None,
                )
                .map_err(|error| document::document_error(&ctx, run.analysis, error))?;
                let built = document::finish_at_coordinate(
                    &ctx,
                    run.analysis,
                    &run.canonical,
                    run.topology,
                    builder,
                )?;
                document::write_document(&ctx, &path, &built)?;
            } else {
                basic::write_dc_op_output(
                    &path,
                    &run.signals,
                    ctx.format,
                    Some(&crate::hdf5::Hdf5ResultIdentity {
                        analysis_id: run.analysis_id.clone(),
                        coordinate_id: Some(run.coordinate_id.to_string()),
                        coordinate_tag: Some(run.coordinate_tag.clone()),
                        coordinate_assignment: Some(canonical_coordinate_description(
                            &run.canonical,
                        )),
                        topology_fingerprint: Some(run.topology.to_string()),
                    }),
                )?;
            }
            coordinate_publications.push(CoordinatePublication {
                coordinate: run.canonical.clone(),
                topology: run.topology,
                results: vec![PublishedResult {
                    analysis_id: run.analysis_id.clone(),
                    schema: run.schema.clone(),
                    artifact: path.clone(),
                }],
            });
            set_coordinates.push(AxisSetCoordinate {
                identity: ArtifactCoordinate::from_run_coordinate(&run.canonical),
                artifacts: vec![path.clone()],
            });
            outputs.push(path);
        }
        let manifest_path = conditional_step_schema_path(&base_output);
        write_step_schema_manifest(&manifest_path, &coordinate_publications)?;
        // This is a coordinate set like any other, so it commits the same
        // manifest declaring the set complete. Without it a reader could not
        // tell a finished implicit-operating-point set from one a cancellation
        // stopped part-way, and would have to re-derive the artifact names it
        // expected instead of reading the ones the run published.
        let set_manifest_path = axis_set_manifest_path(args, config, run_label)?;
        if let Some(set_manifest_path) = &set_manifest_path {
            write_axis_set_manifest(set_manifest_path, args, plan, &set_coordinates)?;
        }
        if crate::abort::reason().is_some() {
            drop(transaction);
            return Err(cancellation_cli_error(args.timeout));
        }
        transaction.commit()?;
        outputs.push(manifest_path);
        outputs.extend(set_manifest_path);
    }
    ctx.record_unevaluated_measurements();
    let measurements = ctx.measurements.borrow().clone();
    let base_name = args
        .input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| *stem != "-")
        .unwrap_or("stdin")
        .to_string();
    let report_name = match run_label {
        Some(label) => format!("{base_name} [{label}]"),
        None => base_name,
    };
    Ok(DeckOutcome {
        reports: vec![SimulationReport {
            name: report_name,
            netlist: args.input.display().to_string(),
            passed: measurements.iter().all(|measurement| measurement.passed),
            duration_secs: start_time.elapsed().as_secs_f64(),
            error: None,
            error_details: None,
            measurements,
        }],
        outputs,
    })
}

struct ImplicitStepCoordinate {
    value: Option<f64>,
    coordinate_id: rspice_core::execution::RunCoordinateId,
    coordinate_tag: String,
    /// The canonical coordinate itself, so a typed coordinate document can
    /// name every axis assignment rather than only its identity string.
    canonical: RunCoordinate,
    analysis: rspice_core::execution::AnalysisInstanceId,
    analysis_id: String,
    topology: rspice_core::execution::TopologyFingerprint,
    result: rspice_core::solver::SimulationResult,
    signals: Vec<crate::commands::run_signals::ScalarSignal>,
    schema: rspice_core::execution::SignalSchema,
    validity: Vec<bool>,
}

fn conditional_step_schema_path(base: &std::path::Path) -> PathBuf {
    let mut path = tag_output_path(base, "step_schema");
    path.set_extension("json");
    path
}

/// Version of the coordinate schema manifest.
///
/// Version 2 groups the union by analysis instance. Version 1 described one
/// implicit operating point, which could not name the several analyses a
/// stepped physical deck publishes at each coordinate.
const STEP_SCHEMA_MANIFEST_VERSION: u32 = 2;

/// The union schema and per-coordinate validity of one analysis instance
/// across an axis deck's coordinates.
struct AnalysisSchemaUnion {
    analysis_id: String,
    union: rspice_core::execution::SchemaUnion,
    coordinates: Vec<CoordinateValidity>,
}

/// What one coordinate published for one analysis, and which of the union's
/// columns it carried.
struct CoordinateValidity {
    coordinate_id: rspice_core::execution::RunCoordinateId,
    assignment: String,
    topology: rspice_core::execution::TopologyFingerprint,
    artifact: PathBuf,
    validity: Vec<bool>,
}

/// Union each analysis instance's coordinate-local schemas and record, per
/// coordinate, which union columns that coordinate actually carried.
///
/// An analysis that only some coordinates published — a conditional that adds
/// or drops a card — is still named, with its own coordinate list. Nothing is
/// inferred from a coordinate that did not publish it.
fn analysis_schema_unions(
    published: &[CoordinatePublication],
) -> Result<Vec<AnalysisSchemaUnion>, CliError> {
    let mut order: Vec<String> = Vec::new();
    let mut grouped: std::collections::HashMap<
        String,
        Vec<(&CoordinatePublication, &PublishedResult)>,
    > = std::collections::HashMap::new();
    for coordinate in published {
        for result in &coordinate.results {
            let entry = grouped.entry(result.analysis_id.clone()).or_default();
            if entry.is_empty() {
                order.push(result.analysis_id.clone());
            }
            entry.push((coordinate, result));
        }
    }

    let mut unions = Vec::with_capacity(order.len());
    for analysis_id in order {
        let entries = grouped.remove(&analysis_id).unwrap_or_default();
        let union = rspice_core::execution::SignalSchema::union(entries.iter().map(
            |(coordinate, result)| {
                rspice_core::execution::CoordinateSchema::new(
                    coordinate.coordinate.stable_id(),
                    &result.schema,
                )
            },
        ))
        .map_err(|error| CliError::CoreSimulationError {
            source: rspice_core::SimulationError::Circuit(format!(
                "coordinate schemas of {analysis_id} cannot form a typed union: {error}"
            )),
            analysis: Some("Step output-schema union".to_string()),
        })?;
        let mut coordinates = Vec::with_capacity(entries.len());
        for (coordinate, result) in entries {
            let indices = union
                .source_indices()
                .get(&coordinate.coordinate.stable_id())
                .ok_or_else(|| CliError::InternalError {
                    message: format!(
                        "coordinate {} vanished from the {analysis_id} schema union",
                        coordinate.coordinate.stable_id()
                    ),
                })?;
            coordinates.push(CoordinateValidity {
                coordinate_id: coordinate.coordinate.stable_id(),
                assignment: canonical_coordinate_description(&coordinate.coordinate),
                topology: coordinate.topology,
                artifact: result.artifact.clone(),
                validity: indices.iter().map(Option::is_some).collect(),
            });
        }
        unions.push(AnalysisSchemaUnion {
            analysis_id,
            union,
            coordinates,
        });
    }
    Ok(unions)
}

/// Publish the manifest that says what each coordinate of an axis deck
/// carried.
///
/// Flat formats have no representation for an absent column, so this is where
/// a consumer learns which coordinate published which signal: the union names
/// every column any coordinate had, and each coordinate's validity bitmap says
/// which of them its own artifact contains.
fn write_step_schema_manifest(
    path: &std::path::Path,
    published: &[CoordinatePublication],
) -> Result<(), CliError> {
    let unions = analysis_schema_unions(published)?;
    let analyses = unions
        .iter()
        .map(|entry| {
            let coordinates = entry
                .coordinates
                .iter()
                .map(|coordinate| {
                    let artifact = coordinate
                        .artifact
                        .file_name()
                        .and_then(std::ffi::OsStr::to_str)
                        .ok_or_else(|| CliError::InternalError {
                            message: format!(
                                "coordinate artifact '{}' has no portable UTF-8 filename",
                                coordinate.artifact.display()
                            ),
                        })?;
                    Ok(serde_json::json!({
                        "coordinate_id": coordinate.coordinate_id.to_string(),
                        "assignment": coordinate.assignment,
                        "topology_fingerprint": coordinate.topology.to_string(),
                        "validity": coordinate.validity,
                        "artifact": artifact,
                    }))
                })
                .collect::<Result<Vec<_>, CliError>>()?;
            Ok(serde_json::json!({
                "analysis_id": entry.analysis_id,
                "union_schema": entry
                    .union
                    .schema()
                    .descriptors()
                    .iter()
                    .map(signal_descriptor_json)
                    .collect::<Vec<_>>(),
                "coordinates": coordinates,
            }))
        })
        .collect::<Result<Vec<_>, CliError>>()?;
    let document = serde_json::json!({
        "schema_version": STEP_SCHEMA_MANIFEST_VERSION,
        "aggregation": "coordinate_local",
        "missingness": "union_validity_bitmap",
        "analyses": analyses,
    });
    let text = serde_json::to_string_pretty(&document)
        .map_err(|error| CliError::output_json_error(path, error))?
        + "\n";
    publish::artifact(path, |writer: &mut dyn std::io::Write| {
        writer
            .write_all(text.as_bytes())
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

fn signal_descriptor_json(
    descriptor: &rspice_core::execution::SignalDescriptor,
) -> serde_json::Value {
    serde_json::json!({
        "canonical_name": descriptor.canonical_name(),
        "display_name": descriptor.display_name(),
        "kind": execution_signal_kind_name(descriptor.kind()),
        "unit": execution_signal_unit_name(descriptor.unit()),
        "value_type": execution_signal_value_type_name(descriptor.value_type()),
        "shape": execution_signal_shape_name(descriptor.shape()),
        "owner": execution_signal_owner_json(descriptor.owner()),
    })
}

fn execution_signal_kind_name(kind: rspice_core::execution::SignalKind) -> &'static str {
    use rspice_core::execution::SignalKind;
    match kind {
        SignalKind::Voltage => "voltage",
        SignalKind::Current => "current",
        SignalKind::DeviceObservable => "device_observable",
        SignalKind::Scalar => "scalar",
        SignalKind::Digital => "digital",
        _ => "unknown",
    }
}

fn execution_signal_unit_name(unit: &rspice_core::execution::SignalUnit) -> String {
    use rspice_core::execution::SignalUnit;
    match unit {
        SignalUnit::Volt => "volt".to_string(),
        SignalUnit::Ampere => "ampere".to_string(),
        SignalUnit::Ohm => "ohm".to_string(),
        SignalUnit::Siemens => "siemens".to_string(),
        SignalUnit::Watt => "watt".to_string(),
        SignalUnit::Hertz => "hertz".to_string(),
        SignalUnit::Second => "second".to_string(),
        SignalUnit::Degree => "degree".to_string(),
        SignalUnit::Radian => "radian".to_string(),
        SignalUnit::Dimensionless => "dimensionless".to_string(),
        SignalUnit::Logic => "logic".to_string(),
        SignalUnit::Custom(name) => format!("custom:{name}"),
        _ => "unknown".to_string(),
    }
}

fn execution_signal_value_type_name(
    value_type: rspice_core::execution::SignalValueType,
) -> &'static str {
    use rspice_core::execution::SignalValueType;
    match value_type {
        SignalValueType::Real => "real",
        SignalValueType::Complex => "complex",
        SignalValueType::Logic => "logic",
        _ => "unknown",
    }
}

fn execution_signal_shape_name(shape: rspice_core::execution::SignalShape) -> &'static str {
    use rspice_core::execution::SignalShape;
    match shape {
        SignalShape::Scalar => "scalar",
        SignalShape::Vector => "vector",
        SignalShape::Matrix => "matrix",
        _ => "unknown",
    }
}

fn execution_signal_owner_json(owner: &rspice_core::execution::SignalOwner) -> serde_json::Value {
    use rspice_core::execution::SignalOwner;
    match owner {
        SignalOwner::Node(name) => serde_json::json!({"kind": "node", "name": name}),
        SignalOwner::Branch(name) => serde_json::json!({"kind": "branch", "name": name}),
        SignalOwner::Device(name) => serde_json::json!({"kind": "device", "name": name}),
        SignalOwner::Analysis => serde_json::json!({"kind": "analysis"}),
        _ => serde_json::json!({"kind": "unknown"}),
    }
}

fn run_deck(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    run_label: Option<&str>,
) -> Result<DeckOutcome, CliError> {
    validate_pss_flag_conflict(netlist, args)?;
    refuse_unsupported_deck_analyses(netlist, config, args)?;
    validate_step_frontend_compatibility(netlist, args)?;

    let resource_limits = config.resources.limits();
    let canonical_plan =
        DeckPlan::from_netlist_with_abort(netlist, &resource_limits, &crate::abort::ProcessAbort)
            .map_err(|error| map_deck_plan_error(error, args))?;
    if canonical_plan.axes().is_empty() {
        // An axis-free deck still takes its artifact namespaces from the
        // canonical plan. Reading the authored identities straight off the
        // plan costs no materialization, so a scalar run does not pay for a
        // second elaboration to learn what it already planned.
        let outcome = run_concrete_deck(
            netlist,
            args,
            config,
            verbose,
            quiet,
            run_label,
            RunIdentity {
                coordinate: None,
                topology: None,
                analyses: PlannedAnalysisIdentities::from_plan(&canonical_plan, netlist),
            },
        )?;
        return Ok(DeckOutcome {
            reports: vec![outcome.report],
            outputs: outcome.outputs,
        });
    }

    let base_signature = step_analysis_signature(netlist)?;
    let sim_config = build_sim_config(args, config, netlist);
    let engine = Engine::try_new(sim_config)?;
    let materializer = engine
        .prepare_deck_plan_materializer_with_abort(
            netlist,
            &canonical_plan,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_materialized_run_error(error, args, "Step planning"))?;
    let aggregate_report_values = base_signature
        .is_empty()
        .then(|| 1usize.saturating_add(netlist.measurements.len().saturating_mul(3)));
    let coordinate_contracts = preflight_step_coordinates(
        &engine,
        &materializer,
        &base_signature,
        aggregate_report_values,
        args,
    )?;

    if base_signature.is_empty() {
        return run_implicit_step_op_table(
            netlist,
            args,
            config,
            verbose,
            quiet,
            &engine,
            &canonical_plan,
            &materializer,
            run_label,
        );
    }

    if !quiet {
        println!(
            "Canonical Cartesian run plan: {} dimension(s), {} run(s); first canonical dimension varies fastest",
            canonical_plan.axes().len(),
            materializer.len()
        );
    }
    // Every coordinate of one axis deck is one result. The transaction holds
    // each coordinate's complete artifact in a staging file beside its
    // destination, so a cancellation or a failure at coordinate k publishes
    // nothing at all instead of a directory that looks like a shorter sweep.
    let transaction = publish::begin()?;
    let mut reports = Vec::with_capacity(materializer.len());
    let mut outputs = Vec::new();
    let mut coordinates = Vec::new();
    let mut published = Vec::new();
    for (run_index, expected) in coordinate_contracts.iter().enumerate() {
        if crate::abort::reason().is_some() {
            break;
        }
        let materialized =
            match materializer.materialize_run_with_abort(run_index, &crate::abort::ProcessAbort) {
                Ok(materialized) => materialized,
                Err(MaterializedRunError::Aborted) if crate::abort::reason().is_some() => {
                    break;
                }
                Err(error) => {
                    return Err(map_materialized_run_error(
                        error,
                        args,
                        format!("Step coordinate {}", run_index + 1),
                    ));
                }
            };
        let canonical_coordinate = materialized.coordinate();
        let materialized_signature = step_analysis_signature(materialized.netlist())?;
        if materialized_signature != expected.signature {
            return Err(CliError::InternalError {
                message: format!(
                    ".STEP coordinate {} changed its preflight physical-analysis signature from {:?} to {:?}",
                    run_index + 1,
                    expected.signature,
                    materialized_signature
                ),
            });
        }
        // The preflight captured this coordinate's structural fingerprint
        // before any solver work. Executing it a second time must reproduce the
        // same circuit, or the artifact would be published under a topology
        // contract it no longer meets.
        let topology = materialized.topology_fingerprint();
        if topology != expected.topology {
            return Err(CliError::InternalError {
                message: format!(
                    ".STEP coordinate {} changed its preflight topology fingerprint from {} to {topology}",
                    run_index + 1,
                    expected.topology
                ),
            });
        }
        let coordinate_label = canonical_coordinate.stable_tag();
        let label = compose_run_label(run_label, Some(&coordinate_label)).ok_or_else(|| {
            CliError::InternalError {
                message: "STEP coordinate unexpectedly has no output namespace".to_string(),
            }
        })?;
        if verbose && !quiet {
            println!(
                "\n=== {label} ({}): {} ===",
                canonical_coordinate.stable_tag(),
                canonical_coordinate_description(canonical_coordinate)
            );
        }
        let outcome = match run_concrete_deck(
            materialized.netlist(),
            args,
            config,
            verbose,
            quiet,
            Some(&label),
            RunIdentity {
                coordinate: Some(canonical_coordinate),
                topology: Some(topology),
                analyses: PlannedAnalysisIdentities::from_materialized(
                    &canonical_plan,
                    canonical_coordinate,
                    materialized.analyses(),
                )?,
            },
        ) {
            Ok(outcome) => outcome,
            Err(_) if crate::abort::reason().is_some() => break,
            Err(error) => return Err(error),
        };
        reports.push(outcome.report);
        published.push(CoordinatePublication {
            coordinate: canonical_coordinate.clone(),
            topology,
            results: outcome.published,
        });
        coordinates.push(AxisSetCoordinate {
            identity: ArtifactCoordinate::from_run_coordinate(canonical_coordinate),
            artifacts: outcome.outputs.clone(),
        });
        outputs.extend(outcome.outputs);
        if crate::abort::reason().is_some() {
            break;
        }
    }
    if crate::abort::reason().is_some() {
        // Dropping the transaction removes every staged coordinate, so the
        // destination directory keeps exactly the artifacts it had before.
        drop(transaction);
        if !quiet {
            println!(
                "Cancelled after {} of {} coordinates: the incomplete set was discarded and no coordinate artifact was published",
                reports.len(),
                materializer.len()
            );
        }
        return Ok(DeckOutcome {
            reports,
            outputs: Vec::new(),
        });
    }
    if reports.len() != materializer.len() {
        return Err(CliError::InternalError {
            message: format!(
                ".STEP completed {} of {} planned coordinates without a cancellation or error",
                reports.len(),
                materializer.len()
            ),
        });
    }
    // A coordinate that failed published nothing, so the set is not the
    // complete one the manifests would declare. Committing them anyway would
    // leave a reader a `run_set` naming a set it can never load: dropping the
    // transaction leaves the destination exactly as it was.
    if reports.iter().any(|report| report.error.is_some()) {
        drop(transaction);
        if !quiet {
            println!(
                "{} of {} coordinates failed: the incomplete set was discarded and no coordinate artifact was published",
                reports
                    .iter()
                    .filter(|report| report.error.is_some())
                    .count(),
                materializer.len()
            );
        }
        return Ok(DeckOutcome {
            reports,
            outputs: Vec::new(),
        });
    }

    // Every coordinate published under its own schema, so the set declares the
    // union of those schemas and, per coordinate, which of its columns the
    // coordinate actually carried. A signal a conditional removed is absent
    // from that coordinate's artifact and invalid in the bitmap; it is never
    // inferred from another coordinate or written as zero.
    if let Some(base_output) = resolve_output_path(args.output.clone(), config)? {
        let base_output = match run_label {
            Some(label) => tag_output_path(&base_output, &sanitize_run_tag(label)),
            None => base_output,
        };
        let manifest_path = conditional_step_schema_path(&base_output);
        write_step_schema_manifest(&manifest_path, &published)?;
        outputs.push(manifest_path);
    }

    if let Some(manifest_path) = axis_set_manifest_path(args, config, run_label)? {
        write_axis_set_manifest(&manifest_path, args, &canonical_plan, &coordinates)?;
        outputs.push(manifest_path);
    }
    transaction.commit()?;
    Ok(DeckOutcome { reports, outputs })
}

/// Everything one axis coordinate published, with the identity it published
/// under.
struct CoordinatePublication {
    coordinate: RunCoordinate,
    topology: rspice_core::execution::TopologyFingerprint,
    results: Vec<PublishedResult>,
}

/// What one concrete deck run produced: its report, the artifacts it staged,
/// and the typed contract each of those artifacts published under.
struct ConcreteDeckOutcome {
    report: SimulationReport,
    outputs: Vec<PathBuf>,
    published: Vec<PublishedResult>,
}

/// One coordinate's identity and the artifacts it staged, for the set
/// manifest published with the coordinate set.
struct AxisSetCoordinate {
    identity: ArtifactCoordinate,
    artifacts: Vec<PathBuf>,
}

/// Path of the manifest that names a complete axis coordinate set.
///
/// A deck without a resolved output path publishes no artifacts, so it has no
/// set to describe.
fn axis_set_manifest_path(
    args: &RunArgs,
    config: &Config,
    run_label: Option<&str>,
) -> Result<Option<PathBuf>, CliError> {
    let Some(mut base) = resolve_output_path(args.output.clone(), config)? else {
        return Ok(None);
    };
    if let Some(label) = run_label {
        base = tag_output_path(&base, &sanitize_run_tag(label));
    }
    let mut path = tag_output_path(&base, "run_set");
    path.set_extension("json");
    Ok(Some(path))
}

/// Describe the complete coordinate set as the last member of its own
/// transaction.
///
/// The manifest is committed after every coordinate artifact, so a reader
/// that finds it knows every artifact it names is present and complete.
fn write_axis_set_manifest(
    path: &std::path::Path,
    args: &RunArgs,
    plan: &DeckPlan,
    coordinates: &[AxisSetCoordinate],
) -> Result<(), CliError> {
    let coordinate_documents = coordinates
        .iter()
        .map(|coordinate| {
            let artifacts = coordinate
                .artifacts
                .iter()
                .map(|artifact| {
                    artifact
                        .file_name()
                        .and_then(std::ffi::OsStr::to_str)
                        .map(str::to_owned)
                        .ok_or_else(|| CliError::InternalError {
                            message: format!(
                                "coordinate artifact '{}' has no portable UTF-8 filename",
                                artifact.display()
                            ),
                        })
                })
                .collect::<Result<Vec<_>, CliError>>()?;
            Ok(serde_json::json!({
                "coordinate_id": coordinate.identity.id,
                "ordinal": coordinate.identity.ordinal,
                "tag": coordinate.identity.tag,
                "assignment": coordinate.identity.assignment,
                "artifacts": artifacts,
            }))
        })
        .collect::<Result<Vec<_>, CliError>>()?;
    let document = serde_json::json!({
        "schema_version": 1,
        "kind": "axis_coordinate_set",
        "deck": args.input.display().to_string(),
        "axes": plan
            .axes()
            .iter()
            .map(|axis| match axis.kind() {
                AxisKind::Temperature => "temperature".to_string(),
                AxisKind::Step => "step".to_string(),
                other => format!("{other:?}").to_ascii_lowercase(),
            })
            .collect::<Vec<_>>(),
        "coordinate_count": coordinates.len(),
        "coordinates": coordinate_documents,
    });
    let text = serde_json::to_string_pretty(&document)
        .map_err(|error| CliError::output_json_error(path, error))?
        + "\n";
    publish::set_manifest(path, |writer| {
        writer
            .write_all(text.as_bytes())
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

/// Run one concrete deck (all of its analyses) and assemble its report.
/// Multi-run failures don't abort the remaining runs — HSPICE semantics —
/// so errors land in the report instead of bubbling, except for setup
/// errors (bad output paths, alternate-mode failures).
fn run_concrete_deck(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    run_label: Option<&str>,
    identity: RunIdentity<'_>,
) -> Result<ConcreteDeckOutcome, CliError> {
    if verbose {
        println!("Title: {}", netlist.title);
        println!("Elements: {}", netlist.elements.len());
        println!("Analyses: {}", netlist.analyses.len());
    }

    let sim_config = build_sim_config(args, config, netlist);
    let engine = Engine::try_new(sim_config)?;
    let ctx = RunContext::new(
        &engine, netlist, args, config, verbose, quiet, run_label, identity,
    )?;

    let base_name = args
        .input
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|stem| *stem != "-")
        .unwrap_or("stdin")
        .to_string();
    let name = match run_label {
        Some(label) => format!("{base_name} [{label}]"),
        None => base_name,
    };

    let start_time = Instant::now();
    let requested_mode = run_requested_mode(&ctx, config)?;
    if requested_mode.ran() {
        if requested_mode.needs_measurement_finalization() {
            ctx.record_unevaluated_measurements();
        }
        // A command-line analysis mode deliberately supersedes the deck's
        // authored cards, so their planned identities stay unconsumed. Only
        // the deferred namespace failure is still a defect here.
        if let Some(message) = ctx.planned_namespace_error.borrow_mut().take() {
            return Err(CliError::InternalError { message });
        }
        let measurements = ctx.measurements.borrow().clone();
        let passed = measurements.iter().all(|meas| meas.passed);
        return Ok(ConcreteDeckOutcome {
            report: SimulationReport {
                name,
                netlist: args.input.display().to_string(),
                passed,
                duration_secs: start_time.elapsed().as_secs_f64(),
                error: None,
                error_details: None,
                measurements,
            },
            published: ctx.published.into_inner(),
            outputs: ctx.outputs.into_inner(),
        });
    }

    let mut ran_analysis = false;
    let mut simulation_error: Option<String> = None;
    let mut simulation_error_details: Option<crate::cli::ErrorDetails> = None;

    for (idx, analysis) in analyses_in_execution_order(netlist).enumerate() {
        if verbose {
            println!(
                "\nRunning analysis {}/{}: {:?}",
                idx + 1,
                netlist.analyses.len(),
                analysis
            );
        }

        ran_analysis = true;
        if let Err(e) = ctx.run_analysis(analysis) {
            if is_run_setup_or_output_error(&e) {
                return Err(e);
            }
            simulation_error_details = Some(e.details());
            simulation_error = Some(simulation_error_message(&e));
            break;
        }
    }

    if !ran_analysis && simulation_error.is_none() {
        if !quiet {
            println!("No analysis commands - running default DC OP...");
        }
        if let Err(e) = basic::run_dc_op(&ctx) {
            if is_run_setup_or_output_error(&e) {
                return Err(e);
            }
            simulation_error_details = Some(e.details());
            simulation_error = Some(simulation_error_message(&e));
        }
    }

    if args.meas && !quiet && netlist.measurements.is_empty() {
        println!("  No .MEAS statements found in netlist");
    }
    ctx.record_unevaluated_measurements();
    if simulation_error.is_none() {
        ctx.ensure_planned_namespaces_consumed()?;
    }

    let duration = start_time.elapsed().as_secs_f64();
    let measurements = ctx.measurements.borrow().clone();
    let passed = simulation_error.is_none() && measurements.iter().all(|meas| meas.passed);

    Ok(ConcreteDeckOutcome {
        report: SimulationReport {
            name,
            netlist: args.input.display().to_string(),
            passed,
            duration_secs: duration,
            error: simulation_error,
            error_details: simulation_error_details,
            measurements,
        },
        published: ctx.published.into_inner(),
        outputs: ctx.outputs.into_inner(),
    })
}

/// Failure text for the run report: simulation errors carry their bare
/// message (re-wrapping at exit would otherwise print
/// "Simulation failed: Simulation failed: ...").
fn simulation_error_message(e: &CliError) -> String {
    match e {
        CliError::SimulationError { message, .. } => message.clone(),
        CliError::CoreSimulationError { source, .. } => source.to_string(),
        other => other.to_string(),
    }
}

/// The first recorded failure in a plan's reports, with the typed details it
/// published.
///
/// The message alone used to be all that survived, which turned every deck
/// failure — a capability refusal, a convergence failure, an exceeded budget —
/// into one undifferentiated simulation error at the process boundary.
fn first_reported_failure(
    reports: &[SimulationReport],
) -> Option<(String, Option<crate::cli::ErrorDetails>)> {
    reports.iter().find_map(|report| {
        report
            .error
            .clone()
            .map(|message| (message, report.error_details.clone()))
    })
}

fn is_run_setup_or_output_error(error: &CliError) -> bool {
    matches!(
        error,
        CliError::InvalidArgument { .. }
            | CliError::OutputError { .. }
            | CliError::OutputSerializationError { .. }
    ) || matches!(
        error.category(),
        // A failed publication is an output failure however it was typed:
        // demoting it into a run report would report a successful simulation
        // whose results never reached the disk.
        crate::cli::FailureCategory::Engine(rspice_core::SimulationErrorCategory::OutputCommit)
    )
}

fn status_failure_summary(report: &SimulationReport) -> String {
    if let Some(error) = &report.error {
        return error.clone();
    }

    let failed_measurements: Vec<&str> = report
        .measurements
        .iter()
        .filter(|meas| !meas.passed)
        .map(|meas| meas.name.as_str())
        .collect();
    if failed_measurements.is_empty() {
        "run failed".to_string()
    } else {
        format!(
            "{} measurement(s) failed: {}",
            failed_measurements.len(),
            failed_measurements.join(", ")
        )
    }
}

/// Write the CI/CD report artifacts covering every run.
fn write_report_files(
    reports: &[SimulationReport],
    args: &RunArgs,
    verbose: bool,
) -> Result<(), CliError> {
    if let Some(ref report_file) = args.report_file {
        match args.report_format {
            Some(crate::cli::ReportFormat::Junit) | None => {
                JUnitReporter::write(reports, report_file)?;
                if verbose {
                    println!("JUnit report written to: {}", report_file.display());
                }
            }
            Some(crate::cli::ReportFormat::Tap) => {
                TapReporter::write(reports, report_file)?;
                if verbose {
                    println!("TAP report written to: {}", report_file.display());
                }
            }
        }
    }

    if let Some(ref meas_file) = args.meas_file {
        match args.meas_format {
            Some(MeasFormat::Csv) => CsvMeasReporter::write(reports, meas_file)?,
            Some(MeasFormat::Json) | None => JsonMeasReporter::write(reports, meas_file)?,
        }
        if verbose {
            println!("Measurement report written to: {}", meas_file.display());
        }
    }
    Ok(())
}

/// Worker count for a multi-run plan: `--jobs 0` = all available cores up to
/// the configured worker budget, never more workers than runs, and single-run
/// plans stay serial. Explicit requests that would exceed the budget fail
/// instead of silently changing operator intent.
fn effective_jobs(
    requested: usize,
    runs: usize,
    max_parallel_workers: usize,
) -> Result<usize, CliError> {
    if max_parallel_workers == 0 {
        return Err(rspice_core::SimulationConfigError::ResourceLimit(
            rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::ParallelWorkers,
                requested: 1,
                limit: 0,
            },
        )
        .into());
    }
    if runs <= 1 {
        return Ok(1);
    }
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    if requested == 0 {
        return Ok(cores.min(runs).min(max_parallel_workers).max(1));
    }

    let workers = requested.min(runs).max(1);
    if workers > max_parallel_workers {
        return Err(rspice_core::SimulationConfigError::ResourceLimit(
            rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::ParallelWorkers,
                requested: workers,
                limit: max_parallel_workers,
            },
        )
        .into());
    }
    Ok(workers)
}

/// `out.csv` + `hot` -> `out.hot.csv` (run-level analog of the
/// per-analysis tagging in `output_path_for`).
/// Where a second document published beside an analysis artifact goes.
///
/// A card whose result is two documents — `.SP DONOISE` publishes the
/// scattering sweep and the port-noise sweep — keeps them under one analysis
/// identity and separates them by file name. Composing the child's name into
/// the parent artifact's own path is what guarantees they differ even for a
/// deck that authors nothing else and therefore publishes under the bare
/// requested path.
pub(super) fn sibling_output_path(path: &std::path::Path, child: &str) -> PathBuf {
    tag_output_path(path, child)
}

fn tag_output_path(path: &std::path::Path, tag: &str) -> PathBuf {
    let mut file_name = path
        .file_stem()
        .map(|stem| stem.to_os_string())
        .unwrap_or_default();
    file_name.push(format!(".{tag}"));
    if let Some(ext) = path.extension() {
        file_name.push(".");
        file_name.push(ext);
    }
    path.with_file_name(file_name)
}

/// Reduce a run label to a file-name-safe tag.
fn sanitize_run_tag(label: &str) -> String {
    let mut tag: String = label
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    while tag.contains("__") {
        tag = tag.replace("__", "_");
    }
    tag.trim_matches('_').to_string()
}

fn load_netlist_from_source(
    source: &str,
    args: &RunArgs,
    config: &Config,
    emit_diagnostics: bool,
) -> Result<Netlist, CliError> {
    let mut search_paths: Vec<PathBuf> = args.includes.clone();
    search_paths.extend(config.paths.include_paths.iter().cloned());
    search_paths.extend(config.paths.library_paths.iter().cloned());
    let parse_options = parse_options_for_run(args, config.resources.limits());

    let parsed = if search_paths.is_empty() {
        Netlist::parse_with_path_and_options_and_abort(
            source,
            &args.input,
            parse_options,
            &crate::abort::ProcessAbort,
        )
    } else {
        Netlist::parse_with_search_paths_and_options_and_abort(
            source,
            &args.input,
            &search_paths,
            parse_options,
            &crate::abort::ProcessAbort,
        )
    };
    let mut netlist = parsed.map_err(|error| map_cancellable_parse_error(error, args.timeout))?;

    if !args.defines.is_empty() {
        let defines: Vec<(String, f64)> = args
            .defines
            .iter()
            .map(|define| parse_define(define))
            .collect::<Result<_, _>>()?;

        // Parameter overrides must win over the deck's own .PARAM assignments
        // and must be visible at parse time (simple `{param}` references are
        // resolved while parsing). Rewrite the include-expanded source so the
        // override is indistinguishable from a deck edit, then re-parse.
        let source = netlist
            .source_text
            .clone()
            .ok_or_else(|| CliError::InternalError {
                message: "netlist source unavailable for --define substitution".to_string(),
            })?;
        let rewritten = apply_defines_to_source(&source, &defines);
        netlist = Netlist::parse_with_path_and_options_and_abort(
            &rewritten,
            &args.input,
            parse_options,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_cancellable_parse_error(error, args.timeout))?;
        for (name, value) in &defines {
            netlist.params.set(name, *value);
        }
    }

    // --save replaces the deck's output selection outright: the caller is
    // asking for exactly these signals. Applied after any -D re-parse so the
    // override always wins.
    if !args.saves.is_empty() {
        let mut saves = rspice_core::netlist::SaveSet::default();
        let mut override_requests = Vec::with_capacity(args.saves.len());
        for (index, spec) in args.saves.iter().enumerate() {
            // The netlist parser falls back to a bare vector name for
            // anything unrecognized; a spec with parentheses that didn't
            // parse as V(...)/I(...) is a typo, not a vector name.
            let parsed = rspice_core::netlist::parse_save_probe(spec).ok_or_else(|| {
                CliError::InvalidArgument {
                    message: format!("invalid --save probe '{spec}'"),
                    suggestion: Some(
                        "use forms like V(out), V(a,b), I(v1), @m1[id], or all".to_string(),
                    ),
                }
            })?;
            let malformed = match &parsed {
                rspice_core::netlist::SaveSignal::Raw(_) => {
                    spec.contains('(') || spec.contains(')')
                }
                _ => false,
            };
            if malformed {
                return Err(CliError::InvalidArgument {
                    message: format!("invalid --save probe '{spec}'"),
                    suggestion: Some(
                        "use forms like V(out), V(a,b), I(v1), @m1[id], or all".to_string(),
                    ),
                });
            }
            saves.signals.push(parsed);
            override_requests.push(rspice_core::netlist::OutputRequest::from_save_override(
                rspice_core::netlist::NetlistSourceLocation::in_file(
                    "<command line --save>",
                    index + 1,
                ),
                spec,
            ));
        }
        saves.apply_ground_policy(netlist.ground_policy());
        netlist.saves = saves;
        netlist.output_requests.retain(|request| {
            !matches!(
                request.directive,
                rspice_core::netlist::OutputDirectiveKind::Save
                    | rspice_core::netlist::OutputDirectiveKind::Probe
                    | rspice_core::netlist::OutputDirectiveKind::Print
                    | rspice_core::netlist::OutputDirectiveKind::Plot
            )
        });
        netlist.output_requests.extend(override_requests);
    }

    rspice_core::netlist::validate_output_symbols_with_abort(&netlist, &crate::abort::ProcessAbort)
        .map_err(|error| map_cancellable_parse_error(error, args.timeout))?;

    if emit_diagnostics {
        crate::commands::emit_netlist_diagnostics(&netlist, false);
    }

    Ok(netlist)
}

/// Parse a config `output.format` name.
fn parse_format_name(name: &str) -> Result<OutputFormat, CliError> {
    use clap::ValueEnum;
    OutputFormat::from_str(name, true).map_err(|_| CliError::ConfigError {
        message: format!(
            "invalid output.format '{}'; expected one of: raw, ascii, csv, json, tsv, hdf5",
            name
        ),
    })
}

/// Resolve `-o` against config `output.output_directory`.
///
/// Relative output paths are placed inside the configured directory (created
/// on demand); absolute paths are used as given.
fn resolve_output_path(
    output: Option<PathBuf>,
    config: &Config,
) -> Result<Option<PathBuf>, CliError> {
    let Some(path) = output else {
        return Ok(None);
    };
    let Some(dir) = config.output.output_directory.as_ref() else {
        return Ok(Some(path));
    };
    if path.is_absolute() {
        return Ok(Some(path));
    }

    std::fs::create_dir_all(dir).map_err(|e| CliError::OutputError {
        path: dir.clone(),
        source: e,
    })?;
    Ok(Some(dir.join(path)))
}

/// Parse a `-D NAME=VALUE` override; the value accepts SPICE suffixes (4.7k, 1u, ...).
fn parse_define(define: &str) -> Result<(String, f64), CliError> {
    let invalid = |message: String| CliError::InvalidArgument {
        message,
        suggestion: Some("use -D NAME=VALUE, e.g. -D RLOAD=4.7k".to_string()),
    };

    let (name, value) = define
        .split_once('=')
        .ok_or_else(|| invalid(format!("malformed --define '{}'", define)))?;
    let name = name.trim();
    if name.is_empty() {
        return Err(invalid(format!(
            "missing parameter name in --define '{}'",
            define
        )));
    }

    let value = rspice_core::netlist::lexer::parse_spice_value(value.trim())
        .map_err(|e| invalid(format!("invalid value in --define '{}': {}", define, e)))?;
    Ok((name.to_string(), value))
}

/// Apply `-D` overrides to include-expanded netlist source.
///
/// Each override is injected as a `.param` right after the title so it exists
/// before first use, and every top-level `.param` assignment of the same name
/// is rewritten in place so the deck cannot re-assign it. Assignments inside
/// `.subckt` bodies are left alone — overrides target global parameters only.
fn apply_defines_to_source(source: &str, defines: &[(String, f64)]) -> String {
    let mut out = String::with_capacity(source.len() + defines.len() * 32);
    let mut lines = source.lines();

    if let Some(title) = lines.next() {
        out.push_str(title);
        out.push('\n');
    }
    for (name, value) in defines {
        out.push_str(&format!(".param {}={:e}\n", name, value));
    }

    let mut subckt_depth = 0usize;
    let mut in_param_statement = false;
    for line in lines {
        let trimmed = line.trim_start();
        let keyword = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();

        if keyword == ".subckt" {
            subckt_depth += 1;
        } else if keyword == ".ends" {
            subckt_depth = subckt_depth.saturating_sub(1);
        }

        let continues_param = in_param_statement && trimmed.starts_with('+');
        if keyword == ".param" || continues_param {
            in_param_statement = true;
            if subckt_depth == 0 {
                out.push_str(&rewrite_param_assignments(line, defines));
            } else {
                out.push_str(line);
            }
        } else {
            if !trimmed.is_empty() && !trimmed.starts_with('*') {
                in_param_statement = false;
            }
            out.push_str(line);
        }
        out.push('\n');
    }

    out
}

/// Rewrite `name=value` assignments in a `.param` line for overridden names.
///
/// Values are scanned with brace/paren depth tracking so expressions like
/// `{a + b}` or `max(1, 2)` stay intact.
fn rewrite_param_assignments(text: &str, defines: &[(String, f64)]) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();

            let mut j = i;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && chars[j] == '=' {
                j += 1;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                let value_start = j;
                let mut depth = 0i32;
                while j < chars.len() {
                    match chars[j] {
                        '(' | '{' | '[' => depth += 1,
                        ')' | '}' | ']' => depth -= 1,
                        ch if depth == 0 && (ch.is_whitespace() || ch == ',') => break,
                        _ => {}
                    }
                    j += 1;
                }

                out.push_str(&ident);
                out.push('=');
                match defines
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(&ident))
                {
                    Some((_, value)) => out.push_str(&format!("{:e}", value)),
                    None => out.extend(&chars[value_start..j]),
                }
                i = j;
                continue;
            }

            out.push_str(&ident);
            continue;
        }

        out.push(c);
        i += 1;
    }

    out
}

fn build_sim_config(args: &RunArgs, config: &Config, netlist: &Netlist) -> SimulationConfig {
    let base = config.core_simulation_config();

    let convergence_mode = args
        .convergence
        .as_deref()
        .unwrap_or(&config.simulation.convergence_mode);
    let convergence_preset = ConvergencePreset::from_mode_name(convergence_mode);

    let integration_method = args.integration_method.as_deref().map(|method| {
        use rspice_core::numerics::integration::IntegrationMethod;
        match method {
            "euler" => IntegrationMethod::BackwardEuler,
            "trap" => IntegrationMethod::Trapezoidal,
            "gear" => IntegrationMethod::Gear2,
            _ => IntegrationMethod::TrapGear,
        }
    });

    let overrides = SimulationConfigOverrides {
        temperature_kelvin: args.temp.map(|temp_c| temp_c + 273.15),
        max_iterations: args.maxiter,
        min_timestep: args.min_step,
        max_timestep: args.max_step,
        integration_method,
        transient_trtol: args.trtol,
        transient_lte_reltol: None,
        transient_lte_abstol: None,
        transient_timeint_max_timestep: None,
        transient_use_device_max_timestep: None,
        transient_error_control: None,
        transient_min_steps_between_breakpoints: None,
        transient_timeint_nlmin: None,
        transient_timeint_nlmax: None,
        transient_timeint_min_order: None,
        transient_timeint_max_order: None,
        transient_timesteps_reversal: None,
        transient_nonlinear_reltol: None,
        transient_nonlinear_abstol: None,
        transient_nonlinear_deltaxtol: None,
        transient_nonlinear_rhstol: None,
        transient_nonlinear_max_iterations: None,
        transient_nonlinear_nox: None,
        transient_enforce_device_convergence: None,
        transient_lte_reference: None,
        transient_new_bp_stepping: None,
        ramptime: None,
        convergence_preset,
        reltol: args.reltol,
        abstol: args.abstol,
        voltage_abstol: args.voltage_abstol,
        current_abstol: args.current_abstol,
        charge_abstol: args.charge_abstol,
        residual_reltol: args.residual_reltol,
        gmin_initial: args.gmin,
        device_voltage_limiting: None,
        digital_delay_type: None,
        spice_dialect: args
            .spice_dialect
            .map(crate::cli::SpiceDialectArg::simulation_dialect),
        jfet_level2_model: None,
    };

    resolve_simulation_config(&base, Some(&netlist.options), &overrides)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestedModeOutcome {
    NotRequested,
    RanNeedsMeasurementFinalization,
    RanManagedMeasurements,
}

impl RequestedModeOutcome {
    fn ran(self) -> bool {
        !matches!(self, Self::NotRequested)
    }

    fn needs_measurement_finalization(self) -> bool {
        matches!(self, Self::RanNeedsMeasurementFinalization)
    }
}

fn run_requested_mode(
    ctx: &RunContext<'_>,
    _config: &Config,
) -> Result<RequestedModeOutcome, CliError> {
    if let Some(num_runs) = ctx.args.monte_carlo {
        let spread = ctx.args.mc_spread.unwrap_or(0.01);
        if !spread.is_finite() || spread < 0.0 {
            return Err(CliError::InvalidArgument {
                message: format!("--mc-spread must be a finite non-negative value, got {spread}"),
                suggestion: Some(
                    "Use 0 for deterministic samples, or e.g. --mc-spread 0.05 for 5% variation"
                        .to_string(),
                ),
            });
        }
        let distribution = match ctx.args.mc_distribution.as_deref() {
            Some("uniform") => rspice_core::analysis::Distribution::Uniform { tolerance: spread },
            Some("worst-case") => {
                rspice_core::analysis::Distribution::WorstCase { tolerance: spread }
            }
            _ => rspice_core::analysis::Distribution::Gaussian { sigma: spread },
        };
        let parameter_filter = if ctx.args.mc_params.is_empty() {
            None
        } else {
            Some(ctx.args.mc_params.as_slice())
        };
        advanced::run_monte_carlo(
            ctx,
            num_runs,
            ctx.args.seed.unwrap_or(1),
            distribution,
            parameter_filter,
        )?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let Some(freq) = ctx.args.pss_freq {
        advanced::run_pss(ctx, freq, ctx.args.pss_harmonics, ctx.args.pss_tstab)?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let Some(freq) = ctx.args.hb_freq {
        advanced::run_hb(ctx, freq, ctx.args.hb_harmonics)?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let (Some(input), Some(output)) =
        (ctx.args.pz_input.as_deref(), ctx.args.pz_output.as_deref())
    {
        let (input, output) = resolve_node_pair(ctx, input, output, "--pz-input/--pz-output")?;
        let input_is_current = matches!(ctx.args.pz_transfer, Some(PzTransferMode::Current));
        frequency::run_pz(ctx, input, output, input_is_current)?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let (Some(output_node), Some(param)) = (
        ctx.args.sens_output.as_deref(),
        ctx.args.sens_param.as_deref(),
    ) {
        let output_name = output_node;
        let output_node = resolve_node(ctx, output_node, "--sens-output")?;
        frequency::run_sensitivity(
            ctx,
            output_node,
            output_name,
            param,
            ctx.args.sens_value.unwrap_or(1.0),
        )?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let Some(ports) = ctx.args.sparam.as_deref() {
        advanced::run_sparam(ctx, ports, ctx.args.sparam_z0.unwrap_or(50.0))?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let Some(corners_str) = ctx.args.corners.as_deref() {
        advanced::run_corner_sweep(ctx, corners_str)?;
        return Ok(RequestedModeOutcome::RanManagedMeasurements);
    }

    Ok(RequestedModeOutcome::NotRequested)
}

/// Resolve a node given by name or index for analysis flags.
fn resolve_node(ctx: &RunContext<'_>, node: &str, flag: &str) -> Result<usize, CliError> {
    let resolver = shared::NodeResolver::from_netlist(ctx.engine, ctx.netlist)?;
    resolver
        .resolve_node(node)
        .ok_or_else(|| CliError::InvalidArgument {
            message: format!("unknown node '{node}' for {flag}"),
            suggestion: Some("pass a node name from the netlist or a node index".to_string()),
        })
}

fn resolve_node_pair(
    ctx: &RunContext<'_>,
    first: &str,
    second: &str,
    flag: &str,
) -> Result<(usize, usize), CliError> {
    Ok((
        resolve_node(ctx, first, flag)?,
        resolve_node(ctx, second, flag)?,
    ))
}

#[cfg(test)]
mod step_cancellation_report_tests {
    use super::*;

    fn passing_report() -> SimulationReport {
        SimulationReport {
            name: "deck [step-000001]".to_string(),
            netlist: "deck.cir".to_string(),
            passed: true,
            duration_secs: 0.1,
            error: None,
            error_details: None,
            measurements: Vec::new(),
        }
    }

    #[test]
    fn timeout_appends_one_failed_typed_ci_report_after_partial_success() {
        let mut reports = vec![passing_report()];
        ensure_cancellation_report(
            &mut reports,
            std::path::Path::new("deck.cir"),
            Some(2.5),
            crate::abort::AbortReason::Timeout,
        );
        assert_eq!(reports.len(), 2);
        assert!(reports[0].passed);
        assert!(!reports[1].passed);
        assert_eq!(reports[1].name, "deck [timed-out]");
        assert!(
            reports[1]
                .error
                .as_deref()
                .is_some_and(|error| error.contains("2.5s"))
        );
        assert_eq!(
            reports[1]
                .error_details
                .as_ref()
                .map(|details| (details.code, details.category)),
            Some(("timed_out", "timeout"))
        );
        assert_eq!(reports[1].measurements.len(), 1);
        assert_eq!(reports[1].measurements[0].name, "__rspice_run_status__");
        assert!(!reports[1].measurements[0].passed);
    }

    #[test]
    fn existing_typed_cancellation_report_is_not_duplicated() {
        let interrupted = CliError::Interrupted;
        let mut reports = vec![SimulationReport {
            name: "deck [step-000002]".to_string(),
            netlist: "deck.cir".to_string(),
            passed: false,
            duration_secs: 0.2,
            error: Some(interrupted.to_string()),
            error_details: Some(interrupted.details()),
            measurements: Vec::new(),
        }];
        ensure_cancellation_report(
            &mut reports,
            std::path::Path::new("deck.cir"),
            None,
            crate::abort::AbortReason::Interrupt,
        );
        ensure_cancellation_report(
            &mut reports,
            std::path::Path::new("deck.cir"),
            None,
            crate::abort::AbortReason::Interrupt,
        );
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].measurements.len(), 1);
        assert_eq!(reports[0].measurements[0].name, "__rspice_run_status__");
        assert!(!reports[0].measurements[0].passed);
        assert_eq!(
            reports[0]
                .error_details
                .as_ref()
                .map(|details| (details.code, details.category)),
            Some(("interrupted", "cancellation"))
        );

        let suffix = format!("{}_{}", std::process::id(), reports.len());
        let json_path = std::env::temp_dir().join(format!("rspice_cancel_{suffix}.json"));
        let csv_path = std::env::temp_dir().join(format!("rspice_cancel_{suffix}.csv"));
        JsonMeasReporter::write(&reports, &json_path).expect("write cancellation JSON");
        CsvMeasReporter::write(&reports, &csv_path).expect("write cancellation CSV");
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&json_path).expect("read cancellation JSON"))
                .expect("parse cancellation JSON");
        assert_eq!(json["failed"], 1);
        let csv = std::fs::read_to_string(&csv_path).expect("read cancellation CSV");
        assert!(csv.contains("__rspice_run_status__"), "{csv}");
        assert!(csv.contains(",false,Simulation interrupted,"), "{csv}");
        let _ = std::fs::remove_file(json_path);
        let _ = std::fs::remove_file(csv_path);
    }
}
