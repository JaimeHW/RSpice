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
mod frequency;
mod shared;

pub(crate) use crate::commands::export_table as export;
pub(crate) use basic::read_fft_raw_artifact;

use crate::report::{
    CsvMeasReporter, JUnitReporter, JsonMeasReporter, MeasurementReport, SimulationReport,
    TapReporter,
};

use crate::cli::{
    CliError, Config, MeasFormat, OutputFormat, PzTransferMode, RunArgs, map_atomic_output_error,
};
use rspice_core::execution::{
    AxisAssignment, AxisKind, DeckPlan, DeckPlanError, DeckPlanMaterializer, MaterializedAnalysis,
    MaterializedRunError, RunAxisValue, RunCoordinate, StepAxisTarget,
};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{
    ConvergencePreset, Engine, Netlist, SimulationConfig, SimulationConfigOverrides,
    resolve_simulation_config,
};
use rspice_output::{AtomicArtifactOptions, Durability, write_atomic};
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
    /// tag. Repeated kinds receive stable one-based ordinal suffixes.
    output_tag_multiplicities: std::collections::HashMap<&'static str, usize>,
    /// Next output ordinal for every repeated analysis tag.
    next_output_tag_ordinal: std::cell::RefCell<std::collections::HashMap<&'static str, usize>>,
    /// Stable materializer-owned output identities, consumed in authored
    /// analysis order by the legacy per-analysis exporters.
    materialized_output_ids: std::cell::RefCell<
        std::collections::HashMap<&'static str, std::collections::VecDeque<String>>,
    >,
    /// Stable transient identities used by checkpoint and post-processing
    /// namespaces. Empty for scalar runs outside a materialized deck plan.
    materialized_transient_ids: Vec<String>,
    materialized_namespace_required: bool,
    materialized_namespace_error: std::cell::RefCell<Option<String>>,
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
    /// Most recently completed authored transient. Transient post-processors
    /// consume this exact result instead of launching an independent run.
    last_transient: std::cell::RefCell<Option<RetainedTransient>>,
    /// Zero-based ordinal assigned to authored transient cards as they enter
    /// the physical-analysis dispatcher.
    next_transient_ordinal: std::cell::Cell<u32>,
    /// Zero-based ordinal assigned to source-authored Fourier cards.
    next_fourier_ordinal: std::cell::Cell<u32>,
}

struct RetainedTransient {
    analysis_id: String,
    result: rspice_core::engine::TransientResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArtifactCoordinate {
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
        coordinate: Option<&RunCoordinate>,
        materialized_analyses: Option<&[MaterializedAnalysis]>,
    ) -> Result<Self, CliError> {
        validate_materialized_namespaces(coordinate, materialized_analyses)?;
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
            next_output_tag_ordinal: std::cell::RefCell::new(std::collections::HashMap::new()),
            materialized_output_ids: std::cell::RefCell::new(materialized_output_id_queues(
                materialized_analyses,
            )),
            materialized_transient_ids: materialized_analysis_ids(materialized_analyses, "tran"),
            materialized_namespace_required: materialized_analyses.is_some(),
            materialized_namespace_error: std::cell::RefCell::new(None),
            verbose,
            quiet,
            measurements: std::cell::RefCell::new(Vec::new()),
            evaluated_meas: std::cell::RefCell::new(std::collections::HashSet::new()),
            outputs: std::cell::RefCell::new(Vec::new()),
            last_transient: std::cell::RefCell::new(None),
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
    /// `out.csv` becomes `out.op.csv`, `out.tran.csv`, ...
    ///
    /// Every resolved path is remembered for the `--summary` manifest.
    fn output_path_for(&self, tag: &str) -> Option<std::path::PathBuf> {
        let path = self.output.clone()?;
        let materialized_id = self
            .materialized_output_ids
            .borrow_mut()
            .get_mut(tag)
            .and_then(std::collections::VecDeque::pop_front);
        if materialized_id.is_none()
            && self.materialized_namespace_required
            && is_physical_output_tag(tag)
        {
            self.materialized_namespace_error.replace(Some(format!(
                "materialized analysis namespace queue has no remaining '{tag}' identity"
            )));
            return None;
        }
        let repeated_tag = self
            .output_tag_multiplicities
            .get_key_value(tag)
            .and_then(|(registered_tag, count)| (*count > 1).then_some(*registered_tag));
        let qualified_tag = materialized_id.unwrap_or_else(|| {
            repeated_tag.map_or_else(
                || tag.to_string(),
                |registered_tag| {
                    let mut ordinals = self.next_output_tag_ordinal.borrow_mut();
                    let ordinal = ordinals.entry(registered_tag).or_default();
                    *ordinal = ordinal.saturating_add(1);
                    format!("{tag}-{:03}", *ordinal)
                },
            )
        });
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
        Some(resolved)
    }

    fn ensure_materialized_namespaces_consumed(&self) -> Result<(), CliError> {
        if let Some(message) = self.materialized_namespace_error.borrow_mut().take() {
            return Err(CliError::InternalError { message });
        }
        if !self.materialized_namespace_required || self.output.is_none() {
            return Ok(());
        }
        let unconsumed = self
            .materialized_output_ids
            .borrow()
            .iter()
            .filter_map(|(tag, ids)| (!ids.is_empty()).then_some((*tag, ids.len())))
            .collect::<Vec<_>>();
        if unconsumed.is_empty() {
            Ok(())
        } else {
            Err(CliError::InternalError {
                message: format!(
                    "materialized analysis namespace queue retained unconsumed identities: {unconsumed:?}"
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
        let ordinal = self.next_transient_ordinal.get();
        if let Some(analysis_id) = ordinal
            .checked_sub(1)
            .and_then(|index| self.materialized_transient_ids.get(index as usize))
        {
            return Some(tag_output_path(&path, analysis_id));
        }
        if self.materialized_namespace_required {
            self.materialized_namespace_error.replace(Some(format!(
                "materialized transient checkpoint namespace has no identity for ordinal {ordinal}"
            )));
            return None;
        }
        if self
            .output_tag_multiplicities
            .get("tran")
            .is_none_or(|count| *count <= 1)
        {
            return Some(path);
        }
        Some(tag_output_path(&path, &format!("tran-{ordinal:03}")))
    }

    fn current_transient_analysis_id(&self) -> Result<String, CliError> {
        let ordinal = self.next_transient_ordinal.get();
        if ordinal == 0 {
            return Err(CliError::InternalError {
                message: "transient execution entered without an assigned analysis ordinal"
                    .to_string(),
            });
        }
        if let Some(analysis_id) = ordinal
            .checked_sub(1)
            .and_then(|index| self.materialized_transient_ids.get(index as usize))
        {
            Ok(analysis_id.clone())
        } else if self.materialized_namespace_required {
            Err(CliError::InternalError {
                message: format!(
                    "materialized transient namespace has no identity for ordinal {ordinal}"
                ),
            })
        } else {
            Ok(format!("tran-{ordinal:03}"))
        }
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
                let result = basic::run_transient(
                    self,
                    *stop,
                    *step,
                    start.unwrap_or(0.0),
                    *max_step,
                    *uic,
                )?;
                self.last_transient.replace(Some(RetainedTransient {
                    analysis_id: self.current_transient_analysis_id()?,
                    result,
                }));
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
            AnalysisCommand::Sp {
                variation,
                points,
                start_freq,
                stop_freq,
                do_noise,
            } => advanced::run_sparam_from_command(
                self,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                *do_noise,
            )?,
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
            AnalysisCommand::Step(_) => {
                return Err(CliError::InternalError {
                    message: ".STEP reached the physical-analysis dispatcher; Cartesian planning was bypassed"
                        .to_string(),
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
                basic::run_fourier(self, ordinal as usize, *fundamental, *num_harmonics)?;
            }
            AnalysisCommand::Temp { temperatures } => basic::run_temp(self, temperatures)?,
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
        AnalysisCommand::Temp { .. } => Some("temp"),
        // The CLI publishes no artifact for the periodic large-signal family
        // and refuses those cards before execution, so they own no namespace.
        AnalysisCommand::Step(_)
        | AnalysisCommand::Four { .. }
        | AnalysisCommand::Pss(_)
        | AnalysisCommand::Pac(_)
        | AnalysisCommand::Pnoise(_)
        | AnalysisCommand::Envelope(_) => None,
    }
}

fn is_physical_output_tag(tag: &str) -> bool {
    matches!(
        tag,
        "op" | "dc"
            | "ac"
            | "tran"
            | "noise"
            | "sp"
            | "stb"
            | "disto"
            | "pz"
            | "sens"
            | "tf"
            | "hb"
            | "mc"
            | "temp"
    )
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

fn materialized_output_id_queues(
    analyses: Option<&[MaterializedAnalysis]>,
) -> std::collections::HashMap<&'static str, std::collections::VecDeque<String>> {
    let mut ids = std::collections::HashMap::new();
    for analysis in analyses.into_iter().flatten() {
        let Some(tag) = analysis.command().and_then(analysis_output_tag) else {
            continue;
        };
        ids.entry(tag)
            .or_insert_with(std::collections::VecDeque::new)
            .push_back(analysis.output_namespace().analysis_component());
    }
    ids
}

fn validate_materialized_namespaces(
    coordinate: Option<&RunCoordinate>,
    analyses: Option<&[MaterializedAnalysis]>,
) -> Result<(), CliError> {
    let Some(analyses) = analyses else {
        return Ok(());
    };
    let coordinate = coordinate.ok_or_else(|| CliError::InternalError {
        message: "materialized analyses were supplied without their run coordinate".to_string(),
    })?;
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
    Ok(())
}

fn materialized_analysis_ids(
    analyses: Option<&[MaterializedAnalysis]>,
    requested_tag: &str,
) -> Vec<String> {
    analyses
        .into_iter()
        .flatten()
        .filter(|analysis| {
            analysis
                .command()
                .and_then(analysis_output_tag)
                .is_some_and(|tag| tag == requested_tag)
        })
        .map(|analysis| analysis.checkpoint_namespace().analysis_component())
        .collect()
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
    if let Some(seconds) = args.timeout {
        crate::abort::arm_timeout(seconds);
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
    let mut first_error: Option<String> = None;

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
                first_error = outcome
                    .reports
                    .iter()
                    .find_map(|report| report.error.clone());
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
                first_error = outcome
                    .reports
                    .iter()
                    .find_map(|report| report.error.clone());
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

    if let Some(err_msg) = first_error {
        return Err(CliError::simulation_error(err_msg));
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
    write_atomic(
        &path,
        AtomicArtifactOptions::new(Durability::SyncFileAndParent),
        |writer| writer.write_all(materialized.derived_source.as_bytes()),
    )
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
    write_atomic(
        path,
        AtomicArtifactOptions::new(Durability::SyncFileAndParent),
        |writer| {
            writer
                .write_all(document.as_bytes())
                .map_err(|error| CliError::output_error(path, error))
        },
    )
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

fn physical_step_analysis_kind(
    analysis: &AnalysisCommand,
) -> Result<Option<&'static str>, CliError> {
    match analysis {
        AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. } => Ok(None),
        AnalysisCommand::Op => Ok(Some("op")),
        AnalysisCommand::Dc { .. } => Ok(Some("dc")),
        AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => Ok(Some("ac")),
        AnalysisCommand::Tran { .. } => Ok(Some("tran")),
        AnalysisCommand::Hb { .. } => Ok(Some("hb")),
        AnalysisCommand::Sp { .. } => Ok(Some("sp")),
        AnalysisCommand::Stb { .. } => Ok(Some("stb")),
        AnalysisCommand::Disto { .. } => Ok(Some("disto")),
        AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => Ok(Some("noise")),
        AnalysisCommand::Sensitivity { .. } => Ok(Some("sens")),
        AnalysisCommand::Tf { .. } => Ok(Some("tf")),
        AnalysisCommand::PoleZero { .. } => Ok(Some("pz")),
        AnalysisCommand::Four { .. } => Ok(Some("four")),
        AnalysisCommand::Pss(_)
        | AnalysisCommand::Pac(_)
        | AnalysisCommand::Pnoise(_)
        | AnalysisCommand::Envelope(_) => Err(unsupported_deck_analysis_error(analysis, None)),
        AnalysisCommand::MonteCarlo(_) => Err(CliError::InvalidArgument {
            message: ".STEP cannot wrap authored Monte Carlo until deterministic nested seed/substream derivation is configured"
                .to_string(),
            suggestion: Some(
                "run the parameter coordinates or Monte Carlo campaign as the outer experiment, but not both in one deck"
                    .to_string(),
            ),
        }),
    }
}

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
        let Some(kind) = physical_step_analysis_kind(analysis)? else {
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
        MaterializedRunError::AlterUnsupported => CliError::InvalidArgument {
            message: "textual .ALTER must be expanded before canonical deck materialization"
                .to_string(),
            suggestion: Some(
                "run the source through the CLI multi-run expander before materializing coordinates"
                .to_string(),
            ),
        },
        MaterializedRunError::AnalysisIdentityMismatch {
            coordinate,
            expected,
            actual,
        } => CliError::InvalidArgument {
            message: format!(
                ".STEP coordinate {coordinate} conditionally changes the child-analysis signature from {expected:?} to {actual:?}"
            ),
            suggestion: Some(
                "keep the authored physical-analysis and post-processing card set unconditional across every coordinate"
                    .to_string(),
            ),
        },
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

fn preflight_step_coordinates(
    engine: &Engine,
    materializer: &DeckPlanMaterializer<'_>,
    base_signature: &[&'static str],
    aggregate_report_values: Option<usize>,
    args: &RunArgs,
) -> Result<Vec<Vec<&'static str>>, CliError> {
    let mut signatures = Vec::with_capacity(materializer.len());
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
        signatures.push(signature);

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
    Ok(signatures)
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
    let ctx = RunContext::new(
        engine, netlist, args, config, verbose, quiet, run_label, None, None,
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
            &result,
            &materialized.netlist().saves,
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
            analysis_id: implicit_analysis.output_namespace().analysis_component(),
            coordinate: canonical_coordinate_description(canonical_coordinate),
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
        // partial batch due to a late schema mismatch.
        for run in &preflight {
            let coordinate_path =
                tag_output_path(&base_output, &sanitize_run_tag(&run.coordinate_tag));
            let path = tag_output_path(&coordinate_path, &run.analysis_id);
            basic::write_dc_op_output(&path, &run.signals, ctx.format)?;
            outputs.push(path);
        }
        let manifest_path = conditional_step_schema_path(&base_output);
        write_conditional_step_schema_manifest(
            &manifest_path,
            &schema_union,
            &preflight,
            &outputs,
        )?;
        outputs.push(manifest_path);
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
    analysis_id: String,
    coordinate: String,
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

fn write_conditional_step_schema_manifest(
    path: &std::path::Path,
    union: &rspice_core::execution::SchemaUnion,
    coordinates: &[ImplicitStepCoordinate],
    artifacts: &[PathBuf],
) -> Result<(), CliError> {
    if artifacts.len() != coordinates.len() {
        return Err(CliError::InternalError {
            message: format!(
                "conditional STEP manifest has {} coordinate(s) but {} artifact path(s)",
                coordinates.len(),
                artifacts.len()
            ),
        });
    }
    let descriptors = union
        .schema()
        .descriptors()
        .iter()
        .map(signal_descriptor_json)
        .collect::<Vec<_>>();
    let coordinate_documents = coordinates
        .iter()
        .zip(artifacts)
        .map(|(coordinate, artifact)| {
            let artifact = artifact
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .ok_or_else(|| CliError::InternalError {
                    message: format!(
                        "conditional STEP artifact '{}' has no portable UTF-8 filename",
                        artifact.display()
                    ),
                })?;
            Ok(serde_json::json!({
                "coordinate_id": coordinate.coordinate_id.to_string(),
                "analysis_id": coordinate.analysis_id,
                "assignment": coordinate.coordinate,
                "topology_fingerprint": coordinate.topology.to_string(),
                "validity": coordinate.validity,
                "artifact": artifact,
            }))
        })
        .collect::<Result<Vec<_>, CliError>>()?;
    let document = serde_json::json!({
        "schema_version": 1,
        "analysis": "implicit_op",
        "aggregation": "coordinate_local",
        "missingness": "union_validity_bitmap",
        "union_schema": descriptors,
        "coordinates": coordinate_documents,
    });
    let text = serde_json::to_string_pretty(&document)
        .map_err(|error| CliError::output_json_error(path, error))?
        + "\n";
    write_atomic(
        path,
        AtomicArtifactOptions::new(Durability::SyncFileAndParent),
        |writer| {
            writer
                .write_all(text.as_bytes())
                .map_err(|error| CliError::output_error(path, error))
        },
    )
    .map_err(|error| map_atomic_output_error(path, error))?;
    Ok(())
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
        let (report, outputs) =
            run_concrete_deck(netlist, args, config, verbose, quiet, run_label, None, None)?;
        return Ok(DeckOutcome {
            reports: vec![report],
            outputs,
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
    let coordinate_signatures = preflight_step_coordinates(
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
    let mut reports = Vec::with_capacity(materializer.len());
    let mut outputs = Vec::new();
    for (run_index, expected_signature) in coordinate_signatures.iter().enumerate() {
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
        if &materialized_signature != expected_signature {
            return Err(CliError::InternalError {
                message: format!(
                    ".STEP coordinate {} changed its preflight physical-analysis signature from {:?} to {:?}",
                    run_index + 1,
                    expected_signature,
                    materialized_signature
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
        let (report, run_outputs) = match run_concrete_deck(
            materialized.netlist(),
            args,
            config,
            verbose,
            quiet,
            Some(&label),
            Some(canonical_coordinate),
            Some(materialized.analyses()),
        ) {
            Ok(outcome) => outcome,
            Err(_) if crate::abort::reason().is_some() => break,
            Err(error) => return Err(error),
        };
        reports.push(report);
        outputs.extend(run_outputs);
        if crate::abort::reason().is_some() {
            break;
        }
    }
    if reports.len() != materializer.len() && crate::abort::reason().is_none() {
        return Err(CliError::InternalError {
            message: format!(
                ".STEP completed {} of {} planned coordinates without a cancellation or error",
                reports.len(),
                materializer.len()
            ),
        });
    }
    Ok(DeckOutcome { reports, outputs })
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
    coordinate: Option<&RunCoordinate>,
    materialized_analyses: Option<&[MaterializedAnalysis]>,
) -> Result<(SimulationReport, Vec<PathBuf>), CliError> {
    if verbose {
        println!("Title: {}", netlist.title);
        println!("Elements: {}", netlist.elements.len());
        println!("Analyses: {}", netlist.analyses.len());
    }

    let sim_config = build_sim_config(args, config, netlist);
    let engine = Engine::try_new(sim_config)?;
    let ctx = RunContext::new(
        &engine,
        netlist,
        args,
        config,
        verbose,
        quiet,
        run_label,
        coordinate,
        materialized_analyses,
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
        ctx.ensure_materialized_namespaces_consumed()?;
        let measurements = ctx.measurements.borrow().clone();
        let passed = measurements.iter().all(|meas| meas.passed);
        return Ok((
            SimulationReport {
                name,
                netlist: args.input.display().to_string(),
                passed,
                duration_secs: start_time.elapsed().as_secs_f64(),
                error: None,
                error_details: None,
                measurements,
            },
            ctx.outputs.into_inner(),
        ));
    }

    let mut ran_analysis = false;
    let mut simulation_error: Option<String> = None;
    let mut simulation_error_details: Option<crate::cli::ErrorDetails> = None;
    let mut transient_postprocessors = Vec::new();

    for (idx, analysis) in netlist.analyses.iter().enumerate() {
        if verbose {
            println!(
                "\nRunning analysis {}/{}: {:?}",
                idx + 1,
                netlist.analyses.len(),
                analysis
            );
        }

        ran_analysis = true;
        if matches!(analysis, AnalysisCommand::Four { .. }) {
            // A Fourier card is source-order independent in SPICE decks. Run
            // all physical analyses first so it consumes the final authored
            // transient even when the card precedes `.TRAN`.
            transient_postprocessors.push(analysis);
            continue;
        }
        if let Err(e) = ctx.run_analysis(analysis) {
            if is_run_setup_or_output_error(&e) {
                return Err(e);
            }
            simulation_error_details = Some(e.details());
            simulation_error = Some(simulation_error_message(&e));
            break;
        }
    }

    if simulation_error.is_none() {
        for analysis in transient_postprocessors {
            if let Err(e) = ctx.run_analysis(analysis) {
                if is_run_setup_or_output_error(&e) {
                    return Err(e);
                }
                simulation_error_details = Some(e.details());
                simulation_error = Some(simulation_error_message(&e));
                break;
            }
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
        ctx.ensure_materialized_namespaces_consumed()?;
    }

    let duration = start_time.elapsed().as_secs_f64();
    let measurements = ctx.measurements.borrow().clone();
    let passed = simulation_error.is_none() && measurements.iter().all(|meas| meas.passed);

    Ok((
        SimulationReport {
            name,
            netlist: args.input.display().to_string(),
            passed,
            duration_secs: duration,
            error: simulation_error,
            error_details: simulation_error_details,
            measurements,
        },
        ctx.outputs.into_inner(),
    ))
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

fn is_run_setup_or_output_error(error: &CliError) -> bool {
    matches!(
        error,
        CliError::InvalidArgument { .. }
            | CliError::OutputError { .. }
            | CliError::OutputSerializationError { .. }
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
        let output_node = resolve_node(ctx, output_node, "--sens-output")?;
        frequency::run_sensitivity(ctx, output_node, param, ctx.args.sens_value.unwrap_or(1.0))?;
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
            Some(("timed_out", "cancellation"))
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
