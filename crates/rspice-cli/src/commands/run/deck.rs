//! Turning command-line arguments and a deck source into something runnable.
//!
//! Everything here happens before any solver work: reading and preprocessing
//! the source, refusing a command line that contradicts the deck, refusing an
//! argument that is not a number the analysis can use, resolving the
//! simulation configuration, and computing the physical-analysis signature a
//! run axis must preserve at every coordinate.

// This module was split out of `run.rs` and still works against the run
// command's own context, errors, and helpers, so it takes the parent's
// imports rather than restating them.
use super::axis::preflight_step_coordinates;
use super::naming::analysis_output_tag;
use super::*;

pub(super) fn parse_options_for_run(
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

/// `--pss-freq` and an authored `.PSS` both name a periodic steady state.
/// Executing one and dropping the other would silently discard what the deck
/// or the command line asked for.
pub(super) fn validate_pss_flag_conflict(
    netlist: &Netlist,
    args: &RunArgs,
) -> Result<(), CliError> {
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

pub(super) fn materialize_addresistors_artifact(
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

pub(super) fn validate_run_numeric_args(args: &RunArgs) -> Result<(), CliError> {
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
fn step_analysis_signature_kind(analysis: &AnalysisCommand) -> Option<&'static str> {
    match analysis {
        AnalysisCommand::Four { .. } => Some(POST_PROCESS_FOURIER_SIGNATURE),
        other => analysis_output_tag(other),
    }
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

pub(super) fn step_analysis_signature(netlist: &Netlist) -> Vec<&'static str> {
    netlist
        .analyses
        .iter()
        .filter_map(step_analysis_signature_kind)
        .collect()
}

pub(super) fn validate_step_frontend_compatibility(
    netlist: &Netlist,
    args: &RunArgs,
) -> Result<(), CliError> {
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
    let signature = step_analysis_signature(netlist);
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

/// Preflight one already-expanded `.ALTER`/textual-`.DATA` variant without
/// solving it or publishing output. The returned count is the number of
/// concrete Cartesian coordinates this outer variant will execute.
pub(super) fn preflight_deck_run_count(
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

    let base_signature = step_analysis_signature(netlist);
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

pub(super) fn load_netlist_from_source(
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
pub(super) fn parse_format_name(name: &str) -> Result<OutputFormat, CliError> {
    use clap::ValueEnum;
    OutputFormat::from_str(name, true).map_err(|_| CliError::ConfigError {
        message: format!(
            "invalid output.format '{}'; expected one of: raw, ascii, csv, json, tsv, hdf5, vcd",
            name
        ),
    })
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

pub(super) fn build_sim_config(
    args: &RunArgs,
    config: &Config,
    netlist: &Netlist,
) -> SimulationConfig {
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
