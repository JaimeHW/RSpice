//! The per-run state one concrete deck is executed against, and the
//! dispatcher that turns each authored card into a call.
//!
//! [`RunContext`] is what a card's runner reads: the engine, the parsed deck,
//! the resolved output destination and format, the canonical identities the
//! planner assigned to every artifact this run may publish, and the results
//! and measurements it has published so far. It decides nothing about what a
//! directive means -- that is `rspice-core`'s -- and nothing about how an
//! artifact is named, which is [`super::naming`]'s.

// This module was split out of `run.rs` and still works against the run
// command's own context, errors, and helpers, so it takes the parent's
// imports rather than restating them.
use super::naming::{
    analysis_output_tag, analysis_output_tag_multiplicities, command_line_analysis_identity,
    output_tag_analysis_kind, planned_fft_ids, planned_fft_instances,
};
use super::*;

pub(super) struct RunContext<'a> {
    pub(super) engine: &'a Engine,
    pub(super) netlist: &'a Netlist,
    pub(super) args: &'a RunArgs,
    /// Resolved output format: CLI flag, else config `output.format`, else raw.
    pub(super) format: OutputFormat,
    /// Resolved output path; relative paths land in config `output.output_directory`.
    pub(super) output: Option<std::path::PathBuf>,
    /// Coordinate-local checkpoint output path. Multi-run labels are inserted
    /// before the extension so independent runs can never overwrite state.
    pub(super) checkpoint: Option<std::path::PathBuf>,
    /// Coordinate-local checkpoint input path, using the same namespace rule
    /// as checkpoint output.
    pub(super) resume: Option<std::path::PathBuf>,
    /// CLI `--progress` or config `output.show_progress`.
    pub(super) show_progress: bool,
    /// CLI `--compress` or config `simulation.compress_waveforms`.
    pub(super) compress: bool,
    /// CLI `--compress-tol`, else config `simulation.compression_tolerance`.
    pub(super) compress_tol: f64,
    /// More than one analysis card runs; output files get per-analysis tags.
    multi_analysis: bool,
    /// Canonical identity of the concrete STEP/TEMP coordinate, when this is
    /// an axis-expanded run. Scalar runs deliberately retain `None`.
    pub(super) coordinate: Option<ArtifactCoordinate>,
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
    /// The upstream periodic instance the canonical plan bound each dependent
    /// analysis to, in planned order.
    ///
    /// A `.PAC`, `.PNOISE` or `.ENVELOPE` card linearizes around the periodic
    /// operating point of a specific `.PSS`/`.HB` instance. The plan decides
    /// which one; reading that binding here is what keeps a deck with two
    /// carriers attaching each dependent card to the same carrier the browser
    /// runner and the engine adapter attach it to.
    planned_upstreams: Vec<(AnalysisInstanceId, AnalysisInstanceId)>,
    /// Every planned `.FOUR` operand and `.FFT` card, named by the canonical
    /// plan and bound to the transient it post-processes.
    planned_post_processes: Vec<PlannedPostProcess>,
    /// Canonical `.FFT` identities in authored order, one per authored
    /// transient FFT request, projected from `planned_post_processes`.
    planned_fft_ids: Vec<String>,
    pub(super) planned_namespace_error: std::cell::RefCell<Option<String>>,
    /// The canonical coordinate this concrete deck run belongs to, retained so
    /// a typed result document can name it. Scalar decks have none.
    pub(super) run_coordinate: Option<RunCoordinate>,
    /// Structural fingerprint of the elaborated circuit. An axis coordinate
    /// arrives with the one its materialization computed; a scalar deck
    /// computes it on demand, and only when a format that carries it is
    /// requested.
    pub(super) topology: std::cell::RefCell<Option<rspice_core::execution::TopologyFingerprint>>,
    /// Typed results this run published, in publication order.
    pub(super) published: std::cell::RefCell<Vec<PublishedResult>>,
    pub(super) verbose: bool,
    pub(super) quiet: bool,
    /// .MEAS results collected while analyses run, for CI/CD reporting
    /// (`--report-file` / `--meas-file`) and the process exit status.
    pub(super) measurements: std::cell::RefCell<Vec<MeasurementReport>>,
    /// Analysis tags (upper-case) whose .MEAS statements were evaluated,
    /// so leftover measurements can fail loudly instead of being skipped.
    pub(super) evaluated_meas: std::cell::RefCell<std::collections::HashSet<String>>,
    /// Result files this run resolved for export, for the `--summary`
    /// manifest.
    pub(super) outputs: std::cell::RefCell<Vec<std::path::PathBuf>>,
    /// Completed authored transients a planned `.FOUR` card post-processes,
    /// in authored order. `.FOUR` cards run after every physical analysis —
    /// ngspice accepts a `.FOUR` card above the `.TRAN` it belongs to — so a
    /// deck with several transients must still be able to reach the one the
    /// plan bound each card to, not merely the one that ran last. A transient
    /// no planned card names is not retained at all.
    retained_transients: std::cell::RefCell<Vec<RetainedTransient>>,
    /// Periodic large-signal operating points this run has solved.
    ///
    /// `.PAC`, `.PNOISE` and `.ENVELOPE` linearize around an upstream `.PSS`
    /// or `.HB`. Core exposes that operating point alongside the upstream
    /// analysis's own result, so the deck route retains it instead of solving
    /// the large-signal problem a second time for every dependent card.
    periodic: std::cell::RefCell<PeriodicOperatingPoints>,
    /// Zero-based ordinal assigned to authored transient cards as they enter
    /// the physical-analysis dispatcher.
    next_transient_ordinal: std::cell::Cell<u32>,
    /// Zero-based ordinal assigned to source-authored Fourier cards.
    next_fourier_ordinal: std::cell::Cell<u32>,
}

/// The canonical identity of one planned `.FOUR` operand and the authored
/// output spelling that operand names.
type PlannedFourierOperand<'plan> = (AnalysisInstanceId, &'plan str);

/// Periodic large-signal operating points one concrete deck run retained.
///
/// The harmonic-balance configuration is retained beside its operating point
/// because `.ENVELOPE` continues the exact carrier its upstream `.HB` solved,
/// and re-deriving that configuration from the card would let a deck-level
/// `.OPTIONS HBINT` change turn one carrier into two.
#[derive(Default)]
pub(super) struct PeriodicOperatingPoints {
    pss: Vec<(AnalysisInstanceId, rspice_core::engine::PssOperatingPoint)>,
    hb: Vec<(
        AnalysisInstanceId,
        rspice_core::engine::HbOperatingPoint,
        rspice_core::analysis::HbConfig,
    )>,
}

impl PeriodicOperatingPoints {
    /// The retained shooting-`.PSS` state of one instance.
    pub(super) fn pss(
        &self,
        id: AnalysisInstanceId,
    ) -> Option<&rspice_core::engine::PssOperatingPoint> {
        self.pss
            .iter()
            .find(|(candidate, _)| *candidate == id)
            .map(|(_, point)| point)
    }

    /// The retained harmonic-balance state of one instance, with the exact
    /// configuration that produced it.
    pub(super) fn hb(
        &self,
        id: AnalysisInstanceId,
    ) -> Option<(
        &rspice_core::engine::HbOperatingPoint,
        &rspice_core::analysis::HbConfig,
    )> {
        self.hb
            .iter()
            .find(|(candidate, _, _)| *candidate == id)
            .map(|(_, point, config)| (point, config))
    }
}

pub(super) struct RetainedTransient {
    pub(super) analysis_id: String,
    /// The same identity, typed, so a post-process document can name its
    /// parent.
    pub(super) analysis: rspice_core::execution::AnalysisInstanceId,
    pub(super) result: rspice_core::engine::TransientResult,
    /// Typed post-process products the core evaluated on the exact accepted
    /// trajectory. Present only for a compressed run, whose retained waveform
    /// is decimated and therefore cannot reproduce them.
    pub(super) post_results: Option<rspice_core::engine::TransientPostResults>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArtifactCoordinate {
    pub(super) id: String,
    pub(super) ordinal: usize,
    pub(super) tag: String,
    pub(super) assignment: String,
}

impl ArtifactCoordinate {
    pub(super) fn from_run_coordinate(coordinate: &RunCoordinate) -> Self {
        Self {
            id: coordinate.stable_id().to_string(),
            ordinal: coordinate.ordinal().saturating_add(1),
            tag: coordinate.stable_tag(),
            assignment: canonical_coordinate_description(coordinate),
        }
    }
}

impl<'a> RunContext<'a> {
    pub(super) fn new(
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
            planned_upstreams: planned.upstreams,
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
            periodic: std::cell::RefCell::default(),
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
            planned_upstreams: planned.upstreams,
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
            periodic: std::cell::RefCell::default(),
            next_transient_ordinal: std::cell::Cell::new(0),
            next_fourier_ordinal: std::cell::Cell::new(0),
        })
    }

    /// Record evaluated .MEAS results: print them under `--meas` and keep
    /// them for report files and the exit status.
    pub(super) fn record_measurements(
        &self,
        analysis: &str,
        results: Vec<rspice_core::MeasureResult>,
    ) {
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
    pub(super) fn record_unevaluated_measurements(&self) {
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
    pub(super) fn output_path_for(&self, tag: &str) -> Option<std::path::PathBuf> {
        self.resolve_output(tag).map(|output| output.path)
    }

    /// Resolve one analysis artifact: where it goes and which canonical
    /// analysis instance it publishes under.
    ///
    /// A deck-authored card takes the identity the planner minted for it. A
    /// command-line analysis mode has no authored card, so its identity is
    /// minted the same canonical way: it is single by construction, and the
    /// first instance of its family.
    pub(super) fn resolve_output(&self, tag: &str) -> Option<ResolvedOutput> {
        let path = self.output.clone()?;
        let (qualified_tag, analysis) = self.take_planned_identity(tag)?;
        let resolved = self.namespaced_artifact_path(path, &qualified_tag);
        self.outputs.borrow_mut().push(resolved.clone());
        Some(ResolvedOutput {
            path: resolved,
            analysis,
        })
    }

    /// The canonical identity the next artifact under `tag` publishes with,
    /// and the namespace component its path takes.
    ///
    /// `None` means the planned queue disagrees with the deck, which is
    /// recorded as a deferred namespace error rather than resolved here.
    fn take_planned_identity(&self, tag: &str) -> Option<(String, Option<AnalysisInstanceId>)> {
        let planned_id = self
            .planned_output_ids
            .borrow_mut()
            .get_mut(tag)
            .and_then(std::collections::VecDeque::pop_front);
        match planned_id {
            Some(id) => Some((id.tag(), Some(id))),
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
                Some((tag.to_string(), minted))
            }
        }
    }

    fn namespaced_artifact_path(&self, path: PathBuf, qualified_tag: &str) -> PathBuf {
        if !self.multi_analysis {
            return path;
        }
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
    }

    /// The canonical identity of the periodic large-signal card entering the
    /// dispatcher, with its artifact destination when the run resolved one.
    ///
    /// `.PSS`, `.HB`, `.PAC`, `.PNOISE` and `.ENVELOPE` need their identity
    /// before they publish, and on a run that resolved no output path at all:
    /// a carrier is retained under its identity so the dependent cards the
    /// plan bound to it can find it, and a dependent card looks its carrier up
    /// by that same identity. Every other family resolves its identity
    /// together with its destination, because it needs the identity only to
    /// publish. The destination is recorded in the `--summary` manifest by the
    /// caller when it publishes, so a card that fails names no artifact.
    pub(super) fn resolve_periodic_analysis(
        &self,
        tag: &str,
    ) -> Result<PeriodicArtifact, CliError> {
        let namespace_error = || {
            self.planned_namespace_error
                .borrow_mut()
                .take()
                .map_or_else(
                    || CliError::InternalError {
                        message: format!("'{tag}' has no canonical analysis identity"),
                    },
                    |message| CliError::InternalError { message },
                )
        };
        let (qualified_tag, analysis) = self
            .take_planned_identity(tag)
            .ok_or_else(namespace_error)?;
        let analysis = analysis.ok_or_else(namespace_error)?;
        let path = self
            .output
            .clone()
            .map(|path| self.namespaced_artifact_path(path, &qualified_tag));
        Ok(PeriodicArtifact { analysis, path })
    }

    /// The upstream periodic carrier the canonical plan bound `analysis` to.
    pub(super) fn planned_upstream(
        &self,
        analysis: AnalysisInstanceId,
        card: &'static str,
    ) -> Result<AnalysisInstanceId, CliError> {
        self.planned_upstreams
            .iter()
            .find(|(dependent, _)| *dependent == analysis)
            .map(|(_, upstream)| *upstream)
            .ok_or_else(|| {
                CliError::simulation_error_in(
                    format!(
                        "{card} linearizes around an upstream periodic analysis and the canonical plan bound {analysis} to none"
                    ),
                    card,
                )
            })
    }

    /// Borrow the periodic operating points this run has retained.
    pub(super) fn periodic(&self) -> std::cell::Ref<'_, PeriodicOperatingPoints> {
        self.periodic.borrow()
    }

    /// Retain one converged shooting-`.PSS` operating point under its identity.
    pub(super) fn retain_pss(
        &self,
        analysis: AnalysisInstanceId,
        operating_point: rspice_core::engine::PssOperatingPoint,
    ) {
        self.periodic
            .borrow_mut()
            .pss
            .push((analysis, operating_point));
    }

    /// Retain one converged harmonic-balance operating point, with the exact
    /// configuration that produced it, under its identity.
    pub(super) fn retain_hb(
        &self,
        analysis: AnalysisInstanceId,
        operating_point: rspice_core::engine::HbOperatingPoint,
        config: rspice_core::analysis::HbConfig,
    ) {
        self.periodic
            .borrow_mut()
            .hb
            .push((analysis, operating_point, config));
    }

    /// The canonical coordinate this concrete deck run belongs to.
    pub(super) const fn run_coordinate(&self) -> Option<&RunCoordinate> {
        self.run_coordinate.as_ref()
    }

    /// Structural fingerprint of this run's elaborated circuit.
    ///
    /// An axis coordinate arrives with the fingerprint its materialization
    /// already computed. A scalar deck computes it once, on the first request,
    /// so a run that publishes no typed artifact never pays for a circuit
    /// build it does not use.
    pub(super) fn topology_fingerprint(
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
    pub(super) fn record_output(&self, path: std::path::PathBuf) {
        self.outputs.borrow_mut().push(path);
    }

    /// Record one typed result this run published.
    pub(super) fn record_published(&self, published: PublishedResult) {
        self.published.borrow_mut().push(published);
    }

    pub(super) fn ensure_planned_namespaces_consumed(&self) -> Result<(), CliError> {
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
    pub(super) fn transient_checkpoint_path(
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

    pub(super) fn current_transient_analysis_id(&self) -> Result<String, CliError> {
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
    pub(super) fn fft_output_path_for(
        &self,
        parent_analysis_id: &str,
    ) -> Option<std::path::PathBuf> {
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
    pub(super) fn planned_fourier_operands(
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
    pub(super) fn retained_transient(
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
    pub(super) fn fft_analysis_ids(&self) -> &[String] {
        &self.planned_fft_ids
    }

    /// The same identities as typed instances, for a document that names its
    /// attached spectra as children.
    pub(super) fn fft_analysis_instances(&self) -> Vec<AnalysisInstanceId> {
        planned_fft_instances(&self.planned_post_processes).collect()
    }

    pub(super) fn run_analysis(&self, analysis: &AnalysisCommand) -> Result<(), CliError> {
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
            AnalysisCommand::Pss(card) => periodic::run_pss_card(self, card)?,
            AnalysisCommand::Pac(card) => periodic::run_pac_card(self, card)?,
            AnalysisCommand::Pnoise(card) => periodic::run_pnoise_card(self, card)?,
            AnalysisCommand::Envelope(card) => periodic::run_envelope_card(self, card)?,
        }

        Ok(())
    }
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
pub(super) struct PlannedAnalysisIdentities {
    /// Output tag to the queue of canonical analysis identities, consumed in
    /// authored order by the per-analysis exporters.
    output_ids: std::collections::HashMap<
        &'static str,
        std::collections::VecDeque<rspice_core::execution::AnalysisInstanceId>,
    >,
    /// Transient identities in authored order, indexed by zero-based
    /// transient ordinal for checkpoint and post-processing namespaces.
    transient_ids: Vec<rspice_core::execution::AnalysisInstanceId>,
    /// Every planned analysis the plan bound to an upstream periodic carrier,
    /// as `(dependent, carrier)`.
    upstreams: Vec<(AnalysisInstanceId, AnalysisInstanceId)>,
    /// Every planned `.FOUR` operand and `.FFT` card, each already named and
    /// bound to the transient it post-processes.
    ///
    /// These are not analyses the CLI dispatches; they are the identities the
    /// transient's post-processing artifacts publish under. Taking them from
    /// the plan is what keeps `four-002` meaning the same operand here, in the
    /// browser runner, and in the engine adapter.
    pub(super) post_processes: Vec<PlannedPostProcess>,
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

    /// The carrier binding of every planned analysis that has one.
    fn planned_upstreams<'plan>(
        planned: impl IntoIterator<Item = &'plan rspice_core::execution::PlannedAnalysis>,
    ) -> Vec<(AnalysisInstanceId, AnalysisInstanceId)> {
        planned
            .into_iter()
            .filter_map(|analysis| {
                analysis
                    .request()
                    .upstream()
                    .map(|upstream| (analysis.id(), upstream))
            })
            .collect()
    }

    /// Identities for a deck with no run axis, read off its canonical plan.
    ///
    /// Run axes and `.FOUR` pair with `None` in the plan: an axis owns no
    /// analysis namespace, and a Fourier card publishes under its own
    /// post-process identity instead.
    pub(super) fn from_plan(plan: &DeckPlan, netlist: &Netlist) -> Self {
        Self {
            upstreams: Self::planned_upstreams(plan.analyses()),
            ..Self::from_pairs(
                plan.authored_analyses(netlist)
                    .filter_map(|(analysis, id)| id.map(|id| (analysis, id))),
                plan.post_process_analyses(),
            )
        }
    }

    /// Identities for one materialized axis coordinate.
    ///
    /// The coordinate binding is checked here rather than trusted: an output
    /// or checkpoint namespace that disagrees with the coordinate it claims to
    /// belong to would let one coordinate overwrite another's artifact.
    pub(super) fn from_materialized(
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
        Ok(Self {
            upstreams: Self::planned_upstreams(analyses.iter().map(MaterializedAnalysis::planned)),
            ..Self::from_pairs(
                analyses.iter().filter_map(|analysis| {
                    analysis.command().map(|command| (command, analysis.id()))
                }),
                plan.post_process_analyses(),
            )
        })
    }
}

/// One resolved artifact destination and the canonical analysis instance it
/// publishes under.
///
/// `analysis` is `None` only for the aggregated axis sweep table, which is a
/// cross-coordinate aggregation rather than one analysis result.
pub(super) struct ResolvedOutput {
    pub(super) path: PathBuf,
    pub(super) analysis: Option<rspice_core::execution::AnalysisInstanceId>,
}

impl ResolvedOutput {
    /// The canonical identity this artifact publishes under.
    pub(super) fn analysis(
        &self,
        tag: &str,
    ) -> Result<rspice_core::execution::AnalysisInstanceId, CliError> {
        self.analysis.ok_or_else(|| CliError::InternalError {
            message: format!("'{tag}' resolved an artifact with no canonical analysis identity"),
        })
    }
}

/// What one periodic-family card publishes under.
///
/// The destination is optional because a run without `-o` still executes the
/// card: a carrier must be solved and retained for the dependent cards bound
/// to it whether or not anything is written.
pub(super) struct PeriodicArtifact {
    pub(super) analysis: AnalysisInstanceId,
    pub(super) path: Option<PathBuf>,
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

/// What one concrete deck run publishes under: its canonical analysis
/// identities and, for an axis-expanded run, its coordinate.
pub(super) struct RunIdentity<'a> {
    /// Coordinate of an axis-expanded run. A scalar deck's single trivial
    /// coordinate deliberately does not namespace artifact paths, so this
    /// stays `None` there.
    pub(super) coordinate: Option<&'a RunCoordinate>,
    /// Structural fingerprint the coordinate's materialization computed. A
    /// scalar deck computes its own on demand.
    pub(super) topology: Option<rspice_core::execution::TopologyFingerprint>,
    pub(super) analyses: PlannedAnalysisIdentities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RequestedModeOutcome {
    NotRequested,
    RanNeedsMeasurementFinalization,
    RanManagedMeasurements,
}

impl RequestedModeOutcome {
    pub(super) fn ran(self) -> bool {
        !matches!(self, Self::NotRequested)
    }

    pub(super) fn needs_measurement_finalization(self) -> bool {
        matches!(self, Self::RanNeedsMeasurementFinalization)
    }
}

pub(super) fn run_requested_mode(
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
pub(super) fn resolve_node(
    ctx: &RunContext<'_>,
    node: &str,
    flag: &str,
) -> Result<usize, CliError> {
    let resolver = shared::NodeResolver::from_netlist(ctx.engine, ctx.netlist, ctx.args.timeout)?;
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
