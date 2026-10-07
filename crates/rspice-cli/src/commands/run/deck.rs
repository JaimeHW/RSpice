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
    crate::commands::input::parse_options(
        &args.netlist_options,
        resource_limits,
        requested_mode_name(args).is_none(),
    )
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
    require_positive_arg("--event-flux-abstol", args.event_flux_abstol)?;
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
    let transient_option =
        args.checkpoint.is_some() || args.resume.is_some() || args.tran_stop.is_some();
    let runs_authored_transient = netlist
        .analyses
        .iter()
        .any(|analysis| matches!(analysis, AnalysisCommand::Tran { .. }))
        && requested_mode_name(args).is_none_or(|mode| mode == "--corners");
    if transient_option && !runs_authored_transient && netlist.control_script.is_none() {
        return Err(CliError::InvalidArgument {
            message: "--checkpoint, --resume and --tran-stop require an executed .TRAN analysis".into(),
            suggestion: Some("add a .TRAN card and remove overriding analysis modes, or remove the transient options".into()),
        });
    }
    let steps = step_commands(netlist);
    let has_temperature_axis = netlist
        .analyses
        .iter()
        .any(|analysis| matches!(analysis, AnalysisCommand::Temp { .. }))
        || steps
            .iter()
            .any(|step| step.target == rspice_core::netlist::StepTarget::Temp);
    if has_temperature_axis && args.temp.is_some() {
        return Err(CliError::InvalidArgument {
            message: "--temp conflicts with an authored .TEMP or .STEP TEMP axis".into(),
            suggestion: Some("remove --temp to execute the authored temperatures, or remove the temperature axis to use one override".into()),
        });
    }
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
    if (args.checkpoint.is_some() || args.resume.is_some())
        && !signature.contains(&"tran")
        && netlist.control_script.is_none()
    {
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
    run_label: Option<&str>,
) -> Result<usize, CliError> {
    validate_step_frontend_compatibility(netlist, args)?;

    let resource_limits = config.resources.limits();
    let canonical_plan =
        DeckPlan::from_netlist_with_abort(netlist, &resource_limits, &crate::abort::ProcessAbort)
            .map_err(|error| map_deck_plan_error(error, args))?;
    if canonical_plan.axes().is_empty() {
        super::restart::protect_planned_inputs(
            netlist,
            args,
            run_label,
            false,
            canonical_plan
                .analyses()
                .iter()
                .map(|analysis| analysis.id())
                .filter(|id| id.kind() == rspice_core::execution::AnalysisKind::Tran),
            resource_limits,
        )?;
        if requested_mode_name(args).is_none() {
            crate::commands::preflight::netlist(
                netlist,
                &args.input,
                resource_limits,
                args.tran_stop,
            )?;
        }
        return Ok(1);
    }

    let base_signature = step_analysis_signature(netlist);
    let engine = build_engine(args, config, netlist)?;
    let materializer = engine
        .prepare_deck_plan_materializer_with_abort(
            netlist,
            &canonical_plan,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_materialized_run_error(error, args, "Step planning preflight"))?;
    let aggregate_report_values = (base_signature.is_empty() && netlist.control_script.is_none())
        .then(|| 1usize.saturating_add(netlist.measurements.len().saturating_mul(3)));
    preflight_step_coordinates(
        &engine,
        &materializer,
        &base_signature,
        aggregate_report_values,
        args,
        run_label,
    )?;
    Ok(materializer.len())
}

pub(super) fn load_netlist_from_source(
    source: &str,
    args: &RunArgs,
    config: &Config,
    emit_diagnostics: bool,
) -> Result<Netlist, CliError> {
    let parse_options = parse_options_for_run(args, config.resources.limits());
    let mut netlist = crate::commands::input::parse_source(
        source,
        &args.input,
        &args.netlist_options,
        config,
        parse_options,
        args.timeout,
    )?;
    super::sources::protect(
        &netlist,
        &build_engine(args, config, &netlist)?,
        args.timeout,
    )?;

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
        netlist.override_output_selection(saves, override_requests);
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

/// Construct every CLI run engine from the authoritative configuration once.
/// Analyses must not apply deck options over explicit CLI overrides again.
pub(super) fn build_engine(
    args: &RunArgs,
    config: &Config,
    netlist: &Netlist,
) -> Result<Engine, CliError> {
    Ok(Engine::try_new_with_resolved_config(build_sim_config(
        args, config, netlist,
    ))?)
}

pub(super) fn build_observed_engine(
    args: &RunArgs,
    config: &Config,
    netlist: &Netlist,
    quiet: bool,
) -> Result<Engine, CliError> {
    Ok(
        build_engine(args, config, netlist)?.with_compiler_diagnostic_handler(move |diagnostic| {
            if !quiet {
                crate::observability::compiler_diagnostic(diagnostic);
            }
        }),
    )
}

fn build_sim_config(args: &RunArgs, config: &Config, netlist: &Netlist) -> SimulationConfig {
    let base = config.core_simulation_config();

    // Configured presets belong to the base layer. Only an explicit CLI
    // selection may replace convergence controls authored in the deck.
    let convergence_preset = args
        .convergence
        .as_deref()
        .and_then(ConvergencePreset::from_mode_name);

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
        transient_event_flux_abstol: args.event_flux_abstol,
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
            .netlist_options
            .spice_dialect
            .map(crate::cli::SpiceDialectArg::simulation_dialect),
        jfet_level2_model: None,
    };

    resolve_simulation_config(&base, Some(&netlist.options), &overrides)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{Cli, Commands};
    use clap::Parser;
    use rspice_core::config::DampingStrategy;

    fn run_args(extra: &[&str]) -> RunArgs {
        let cli = Cli::try_parse_from(
            ["rspice", "run", "policy.cir"]
                .into_iter()
                .chain(extra.iter().copied()),
        )
        .expect("valid run arguments");
        let Commands::Run(args) = cli.command else {
            panic!("expected run command");
        };
        args
    }

    #[test]
    fn convergence_policy_preserves_authored_deck_options_without_a_cli_preset() {
        let args = run_args(&[]);
        let netlist = Netlist::parse(
            "Convergence policy\nV1 in 0 1\nR1 in 0 1k\n\
             .options gminstepping=0 sourcestepping=1 pseudotransient=0 arclength=1 damping=bankrose\n\
             .op\n.end\n",
        ).expect("valid deck");
        for mode in ["default", "fast", "robust"] {
            let mut config = Config::default();
            config.simulation.convergence_mode = mode.into();
            let engine = build_engine(&args, &config, &netlist).expect("valid run engine");
            let policy = &engine.config().convergence_config;
            assert!(!policy.gmin_stepping, "{mode}");
            assert!(policy.source_stepping, "{mode}");
            assert!(!policy.pseudo_transient, "{mode}");
            assert!(policy.arc_length, "{mode}");
            assert_eq!(policy.damping_strategy, DampingStrategy::BankRose, "{mode}");
        }
    }

    #[test]
    fn convergence_policy_explicit_cli_presets_outrank_the_deck_and_config() {
        let netlist = Netlist::parse(
            "Convergence policy\nV1 in 0 1\nR1 in 0 1k\n\
             .options gminstepping=0 sourcestepping=1 pseudotransient=0 arclength=1 damping=bankrose\n\
             + reltol=6e-5 abstol=7e-14 vntol=8e-7 chgtol=9e-15\n.op\n.end\n",
        ).expect("valid deck");
        let mut config = Config::default();
        config.simulation.convergence_mode = "fast".into();
        config.simulation.residual_reltol = 4e-5;
        for (mode, stepping, arc_length, damping) in [
            ("fast", false, false, DampingStrategy::None),
            ("default", true, false, DampingStrategy::VoltageLimiting),
            ("robust", true, true, DampingStrategy::Combined),
        ] {
            let args = run_args(&["--convergence", mode, "--reltol", "2e-4"]);
            let engine = build_engine(&args, &config, &netlist).expect("valid run engine");
            let policy = &engine.config().convergence_config;
            assert_eq!(policy.gmin_stepping, stepping, "{mode}");
            assert_eq!(policy.source_stepping, stepping, "{mode}");
            assert_eq!(policy.pseudo_transient, stepping, "{mode}");
            assert_eq!(policy.arc_length, arc_length, "{mode}");
            assert_eq!(policy.damping_strategy, damping, "{mode}");
            assert_eq!(policy.voltage_reltol, 2e-4);
            assert_eq!(policy.voltage_abstol, 8e-7);
            assert_eq!(policy.current_abstol, 7e-14);
            assert_eq!(policy.charge_abstol, 9e-15);
            assert_eq!(policy.residual_reltol, 6e-5);
        }
    }

    #[test]
    fn convergence_policy_deck_changes_preserve_other_configured_preset_fields() {
        let args = run_args(&[]);
        let netlist = Netlist::parse(
            "Convergence policy\nV1 in 0 1\nR1 in 0 1k\n\
             .options gminstepping=1\n.op\n.end\n",
        )
        .expect("valid deck");
        let mut config = Config::default();
        config.simulation.convergence_mode = "fast".into();
        let engine = build_engine(&args, &config, &netlist).expect("valid run engine");
        let policy = &engine.config().convergence_config;
        assert!(policy.gmin_stepping);
        assert!(!policy.source_stepping);
        assert!(!policy.pseudo_transient);
        assert!(!policy.arc_length);
        assert_eq!(policy.damping_strategy, DampingStrategy::None);
    }
}
