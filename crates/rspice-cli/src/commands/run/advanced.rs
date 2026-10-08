//! Sweeps, RF, and statistical analyses: `.STEP`, `.HB`, `.SP`, Monte Carlo,
//! plus the `--pss-freq`, `--sparam`, and `--corners` command-line modes.
//!
//! The `.HB` and PSS writers live here because both routes — an authored card
//! and a command-line mode — publish the same artifact. The authored periodic
//! family that consumes an `.HB`/`.PSS` carrier is in [`super::periodic`].
//!
//! Two S-parameter paths live here and write different tags. The `.SP` card
//! solves the N ports the deck declares with `PORT` voltage sources (`sp`
//! tag); `--sparam` drives four explicitly named nodes as a two-port over the
//! deck's `.AC` sweep (`sparam` tag). Both write Touchstone when the `-o`
//! extension matches the port count, and the standard complex tables
//! otherwise.
//!
//! Corner sweeps re-elaborate the deck per corner on `--jobs` workers,
//! tagging each corner's output so workers never collide.

use rspice_core::analysis::s_param;
use rspice_core::execution::SignalSchema;

use super::RunContext;
use crate::cli::{CliError, map_atomic_output_error};
use crate::commands::export_table::ExportTable;
use crate::commands::{publish, truncate};

fn ensure_not_cancelled(ctx: &RunContext<'_>) -> Result<(), CliError> {
    if crate::abort::reason().is_some() {
        Err(super::cancellation_cli_error(ctx.args.timeout))
    } else {
        Ok(())
    }
}

fn map_advanced_simulation_error(
    ctx: &RunContext<'_>,
    analysis: &str,
    error: rspice_core::SimulationError,
) -> CliError {
    if matches!(error, rspice_core::SimulationError::Aborted) {
        super::cancellation_cli_error(ctx.args.timeout)
    } else {
        // Carry the engine's typed failure rather than its text. Every
        // analysis routed through here - HB, PSS, Monte Carlo, S-parameters -
        // stringified its failure into the simulation category, so a device
        // the analysis refuses and one that failed to converge left this
        // process with the same status.
        CliError::CoreSimulationError {
            source: error,
            analysis: Some(analysis.to_string()),
        }
    }
}

/// Run one authored `.HB` card and retain the carrier it converged on.
///
/// The retained operating point is what an authored `.PAC`, `.PNOISE` or
/// `.ENVELOPE` bound to this instance linearizes around, so the large-signal
/// problem is solved once per card rather than once per dependent analysis.
pub(super) fn run_hb_from_command(
    ctx: &RunContext<'_>,
    card: &rspice_core::netlist::HbCard,
) -> Result<(), CliError> {
    // Every control the solve reads is on the card, and the resolution of the
    // card against the deck's options — the default harmonic order, the
    // multi-tone common basis, the `.OPTIONS HBINT NUMFREQ` collocation rule
    // — belongs to `rspice-core`.
    let config = rspice_core::analysis::HbConfig::from_hb_card(card, &ctx.netlist.options)
        .map_err(|error| CliError::simulation_error_in(error.to_string(), "HB"))?;

    let artifact = ctx.resolve_periodic_analysis("hb")?;
    let hb_result = solve_hb(ctx, config.clone())?;
    if let Some(path) = &artifact.path {
        export_hb(
            ctx,
            artifact.analysis,
            path,
            config.fundamental_freq,
            &hb_result.result,
        )?;
    }
    ctx.retain_hb(artifact.analysis, hb_result.operating_point, config);
    Ok(())
}

/// Write the .STEP sweep table: one row per step value, one column per
/// projected operating-point signal, with the stepped quantity
/// as the abscissa.
pub(super) fn export_step_sweep(
    ctx: &RunContext<'_>,
    step_name: &str,
    sweep_vals: Vec<f64>,
    signals: &[crate::commands::run_signals::ScalarSignal],
) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;
    let Some(ref output_path) = ctx.output_path_for("step") else {
        return Ok(());
    };

    match ctx.format {
        crate::cli::OutputFormat::Hdf5 => {
            let mut data = crate::hdf5::Hdf5SimulationData::new();
            data.title = "Step Sweep".to_string();

            let mut sweep = crate::hdf5::Hdf5WaveformSection::new(step_name, sweep_vals.clone());
            for signal in signals {
                sweep.add_typed_signal(
                    signal.display_name.clone(),
                    signal.raw_variable_type(),
                    signal.unit_symbol(),
                    signal.values.clone(),
                );
            }
            data.dc_sweep = Some(sweep);

            crate::hdf5::write_hdf5(output_path, &data)
                .map_err(|err| super::shared::map_hdf5_output_error(output_path, err))?;
        }
        crate::cli::OutputFormat::Raw | crate::cli::OutputFormat::RawAscii |
        crate::cli::OutputFormat::Csv
        | crate::cli::OutputFormat::Tsv
        | crate::cli::OutputFormat::Json
        // A step sweep has no event timeline, so the table writer refuses VCD
        // by name rather than this arm deciding it a second time.
        | crate::cli::OutputFormat::Vcd => {
            super::export::scalar_table(
                "step_sweep",
                "Step Sweep",
                step_name,
                "value",
                sweep_vals,
                signals,
            )
            .write(output_path, ctx.format)?;
        }
    }

    ctx.record_output(output_path.clone());
    if !ctx.quiet {
        crate::console::line(format_args!(
            "  Step results exported to: {}",
            output_path.display()
        ))?;
    }
    Ok(())
}

pub(super) fn run_monte_carlo(
    ctx: &RunContext<'_>,
    num_runs: usize,
    first_trial: usize,
    seed: u64,
    distribution: rspice_core::analysis::Distribution,
    parameter_filter: Option<&[String]>,
    mean_confidence: Option<(
        f64,
        rspice_core::analysis::monte_carlo::MeanConfidenceMethod,
    )>,
) -> Result<(), CliError> {
    if !ctx.quiet {
        crate::console::line(format_args!(
            "Running Monte Carlo analysis: {} iterations starting at trial {} (seed={})",
            num_runs, first_trial, seed
        ))?;
    }

    let pb = if ctx.quiet {
        indicatif::ProgressBar::hidden()
    } else {
        // The engine runs all iterations in one call without progress
        // callbacks; show honest elapsed time instead of a frozen bar.
        let pb = indicatif::ProgressBar::new_spinner();
        let style = indicatif::ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .map_err(|error| CliError::InternalError {
                message: format!("invalid built-in Monte Carlo progress template: {error}"),
            })?;
        pb.set_style(style);
        pb.set_message(format!("Monte Carlo: {} runs (seed {})", num_runs, seed));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    };

    match ctx.engine.run_monte_carlo_voltages_with_abort(
        ctx.netlist,
        &rspice_core::engine::MonteCarloRunConfig {
            first_trial,
            num_runs,
            seed,
            distribution,
            variation_source: rspice_core::engine::MonteCarloVariationSource::ParameterTolerance,
            parameter_filter,
            environment: None,
        },
        &crate::abort::ProcessAbort,
    ) {
        Ok(mut result) => {
            if let Some((level, method)) = mean_confidence {
                result
                    .compute_mean_confidence(
                        level,
                        method,
                        ctx.engine.config().resource_limits,
                        &crate::abort::ProcessAbort,
                    )
                    .map_err(|error| {
                        map_advanced_simulation_error(ctx, "Monte Carlo confidence", error)
                    })?;
            }
            pb.finish_and_clear();
            ensure_not_cancelled(ctx)?;

            if result.num_failures >= num_runs {
                return Err(CliError::simulation_error_in(
                    format!("all {} Monte Carlo runs failed to converge", num_runs),
                    "Monte Carlo",
                ));
            }
            if result.num_failures > 0 {
                crate::observability::diagnostic(
                    "monte_carlo_partial",
                    None,
                    format_args!(
                        "Warning: {}/{} Monte Carlo runs failed to converge; statistics \
                     cover the surviving runs only",
                        result.num_failures, num_runs
                    ),
                );
            }

            // Deterministic ordering for display and export.
            let mut variables: Vec<&rspice_core::analysis::VariableStatistics> =
                result.variables.values().collect();
            variables.sort_by(|a, b| a.name.cmp(&b.name));

            if !ctx.quiet {
                crate::console::line(format_args!(
                    "✓ Monte Carlo complete: {} runs (seed={})",
                    result.num_runs, seed
                ))?;
                if !variables.is_empty() {
                    crate::console::line(format_args!(
                        "  {:<24} {:>13} {:>13} {:>13} {:>13}",
                        "VARIABLE", "MEAN", "STD", "MIN", "MAX"
                    ))?;
                    for stats in &variables {
                        crate::console::line(format_args!(
                            "  {:<24} {:>13.6e} {:>13.6e} {:>13.6e} {:>13.6e}",
                            stats.name, stats.mean, stats.std_dev, stats.min, stats.max
                        ))?;
                    }
                }
            }

            export_monte_carlo(ctx, seed, &result, &variables)?;
            Ok(())
        }
        Err(e) => {
            pb.finish_and_clear();
            Err(map_advanced_simulation_error(ctx, "Monte Carlo", e))
        }
    }
}

/// Write Monte Carlo results: per-run samples as the table body (one row
/// per run, one column per tracked variable). The JSON format additionally
/// carries the summary statistics and run metadata.
fn export_monte_carlo(
    ctx: &RunContext<'_>,
    seed: u64,
    result: &rspice_core::analysis::MonteCarloResult,
    variables: &[&rspice_core::analysis::VariableStatistics],
) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;
    let Some(resolved) = ctx.resolve_output("mc") else {
        return Ok(());
    };
    let analysis_id = resolved.analysis("mc")?;
    let output_path = resolved.path;

    let num_samples = variables
        .iter()
        .map(|stats| stats.samples.len())
        .max()
        .unwrap_or(0);
    let indices =
        result
            .successful_trial_indices
            .as_ref()
            .ok_or_else(|| CliError::InternalError {
                message: "Monte Carlo export requires the original trial identities".into(),
            })?;
    if indices.len() != num_samples
        || indices
            .iter()
            .any(|&index| (index as u128) > (1_u128 << 53) - 1)
    {
        return Err(CliError::InternalError {
            message:
                "Monte Carlo trial identities cannot be represented exactly in the exported table"
                    .into(),
        });
    }
    let runs: Vec<f64> = indices.iter().map(|&index| index as f64).collect();
    let signals: Vec<crate::commands::run_signals::ScalarSignal> = variables
        .iter()
        .map(|stats| crate::commands::run_signals::ScalarSignal {
            display_name: stats.name.clone(),
            raw_name: stats.name.clone(),
            kind: crate::commands::run_signals::SignalKind::Voltage,
            values: stats.samples.clone(),
        })
        .collect();
    // The campaign seed is run configuration rather than a result field. It is
    // reported on the console and in the `--summary` manifest; the typed
    // document carries the statistics the core computed.
    let _ = seed;

    super::document::publish_analysis_result(
        ctx,
        &output_path,
        analysis_id,
        super::document::scalar_schema(&signals)?,
        || rspice_core::execution::AnalysisResultDocument::from_monte_carlo(analysis_id, result),
        |path, format| match format {
            crate::cli::OutputFormat::Hdf5 => {
                let mut data = crate::hdf5::Hdf5SimulationData::new();
                data.title = "Monte Carlo Samples".to_string();
                data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                let mut sweep = crate::hdf5::Hdf5WaveformSection::new("trial_index", runs.clone());
                for signal in &signals {
                    sweep.add_typed_signal(
                        signal.display_name.clone(),
                        signal.raw_variable_type(),
                        signal.unit_symbol(),
                        signal.values.clone(),
                    );
                }
                data.dc_sweep = Some(sweep);
                crate::hdf5::write_hdf5(path, &data)
                    .map_err(|err| super::shared::map_hdf5_output_error(path, err))
            }
            format => super::export::scalar_table(
                "monte_carlo",
                "Monte Carlo Samples",
                "trial_index",
                "index",
                runs.clone(),
                &signals,
            )
            .write(path, format),
        },
    )?;

    if !ctx.quiet {
        crate::console::line(format_args!(
            "  Monte Carlo samples exported to: {}",
            output_path.display()
        ))?;
    }
    Ok(())
}

/// Run one authored Monte Carlo card.
///
/// Inside a `.STEP` or `.TEMP` sweep the card runs once per coordinate. Giving
/// every coordinate the authored seed would repeat one sample across the
/// sweep, and drawing from a shared stream would make a coordinate's answer
/// depend on how many coordinates ran before it, so the stream is derived from
/// the authored seed and the coordinate's own stable identity by the core rule
/// every surface uses. A deck with no run axis has no coordinate and keeps the
/// authored seed unchanged.
pub(super) fn run_monte_carlo_from_command(
    ctx: &RunContext<'_>,
    mc_cmd: &rspice_core::netlist::MonteCarloCommand,
) -> Result<(), CliError> {
    let authored_seed = ctx.args.seed.or(mc_cmd.seed).unwrap_or(1);
    let seed = ctx.run_coordinate().map_or(authored_seed, |coordinate| {
        rspice_core::execution::monte_carlo_seed_at_coordinate(
            authored_seed,
            coordinate.stable_id(),
        )
    });
    let distribution = match mc_cmd.distribution {
        rspice_core::netlist::MonteCarloDistribution::Gaussian => {
            rspice_core::analysis::Distribution::Gaussian {
                sigma: mc_cmd.relative_spread,
            }
        }
        rspice_core::netlist::MonteCarloDistribution::Uniform => {
            rspice_core::analysis::Distribution::Uniform {
                tolerance: mc_cmd.relative_spread,
            }
        }
        rspice_core::netlist::MonteCarloDistribution::WorstCase => {
            rspice_core::analysis::Distribution::WorstCase {
                tolerance: mc_cmd.relative_spread,
            }
        }
    };
    let parameter_filter = if mc_cmd.params.is_empty() {
        None
    } else {
        Some(mc_cmd.params.as_slice())
    };

    run_monte_carlo(
        ctx,
        mc_cmd.runs,
        mc_cmd.first_trial,
        seed,
        distribution,
        parameter_filter,
        Some((mc_cmd.confidence_pct, mc_cmd.confidence_method.into())),
    )
}

/// The `--pss-freq` route. It supersedes the deck's authored cards outright,
/// so no authored `.PAC`/`.PNOISE` can consume its carrier and the operating
/// point is not retained.
pub(super) fn run_pss(
    ctx: &RunContext<'_>,
    freq: f64,
    harmonics: usize,
    tstab: Option<f64>,
) -> Result<(), CliError> {
    let mut config = rspice_core::analysis::PssConfig::new(freq);
    config.num_harmonics = harmonics;
    if let Some(t) = tstab {
        config.tstab = t;
    }

    let artifact = ctx.resolve_periodic_analysis("pss")?;
    announce_pss(ctx, &config)?;
    let pss_result = ctx
        .engine
        .run_pss_with_abort(ctx.netlist, config, &crate::abort::ProcessAbort)
        .map_err(|error| map_advanced_simulation_error(ctx, "PSS", error))?;
    ensure_not_cancelled(ctx)?;
    report_pss(
        ctx,
        pss_result.iterations,
        pss_result.period,
        &pss_result.result,
    )?;
    if let Some(path) = &artifact.path {
        export_pss(ctx, artifact.analysis, path, &pss_result.result)?;
    }
    Ok(())
}

/// Announce one periodic steady state before the shooting solve starts.
pub(super) fn announce_pss(
    ctx: &RunContext<'_>,
    config: &rspice_core::analysis::PssConfig,
) -> Result<(), CliError> {
    if ctx.quiet {
        return Ok(());
    }
    if config.is_autonomous() {
        crate::console::line(format_args!(
            "Running PSS analysis: autonomous, {} harmonics",
            config.num_harmonics
        ))?;
    } else {
        crate::console::line(format_args!(
            "Running PSS analysis: f₀ = {:.3e} Hz, {} harmonics",
            config.fundamental_freq, config.num_harmonics
        ))?;
    }
    Ok(())
}

/// Report one converged periodic steady state on the console.
pub(super) fn report_pss(
    ctx: &RunContext<'_>,
    iterations: usize,
    period: f64,
    result: &rspice_core::analysis::PssResult,
) -> Result<(), CliError> {
    if ctx.quiet {
        return Ok(());
    }
    crate::console::line(format_args!("✓ PSS converged in {iterations} iterations"))?;
    crate::console::line(format_args!("  Period: {period:.6e} s"))?;
    crate::console::line(format_args!("  Nodes: {}", result.num_nodes()))?;

    if ctx.verbose && result.num_nodes() > 0 {
        crate::console::line(format_args!("\n  Harmonic content (node 1):"))?;
        for harmonic in &result.harmonics(1, 5) {
            crate::console::line(format_args!(
                "    H{}: mag={:.6e}, phase={:.2}° (f={:.3e} Hz)",
                harmonic.harmonic_number, harmonic.magnitude, harmonic.phase, harmonic.frequency
            ))?;
        }
    }
    Ok(())
}

/// Write one period of the converged steady-state waveforms (time domain),
/// the same table shape as a transient export.
pub(super) fn export_pss(
    ctx: &RunContext<'_>,
    analysis_id: rspice_core::execution::AnalysisInstanceId,
    output_path: &std::path::Path,
    result: &rspice_core::analysis::PssResult,
) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;

    let signals: Vec<crate::commands::run_signals::ScalarSignal> = result
        .waveforms
        .iter()
        .enumerate()
        .map(|(index, waveform)| {
            let raw_name = result
                .node_names
                .get(index)
                .cloned()
                .unwrap_or_else(|| (index + 1).to_string());
            crate::commands::run_signals::ScalarSignal {
                display_name: format!("V({raw_name})"),
                raw_name,
                kind: crate::commands::run_signals::SignalKind::Voltage,
                values: waveform.values.clone(),
            }
        })
        .collect();
    let signals = crate::commands::run_signals::scalar_export_signals(
        ctx.netlist,
        rspice_core::execution::AnalysisResultKind::Pss,
        "PSS",
        &result.time,
        &signals,
        &crate::abort::ProcessAbort,
    )
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some("PSS output projection".to_string()),
    })?;

    super::document::publish_analysis_result(
        ctx,
        output_path,
        analysis_id,
        super::document::scalar_schema(&signals)?,
        || rspice_core::execution::AnalysisResultDocument::from_pss(analysis_id, result),
        |path, format| {
            match format {
                crate::cli::OutputFormat::Hdf5 => {
                    let mut data = crate::hdf5::Hdf5SimulationData::new();
                    data.title = "Periodic Steady State".to_string();
                    data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                    let mut section =
                        crate::hdf5::Hdf5WaveformSection::new("time", result.time.clone());
                    for signal in &signals {
                        section.add_typed_signal(
                            signal.display_name.clone(),
                            signal.raw_variable_type(),
                            signal.unit_symbol(),
                            signal.values.clone(),
                        );
                    }
                    data.transient = Some(section);
                    crate::hdf5::write_hdf5(path, &data)
                        .map_err(|err| super::shared::map_hdf5_output_error(path, err))?;
                }
                format => {
                    super::export::scalar_table(
                        "pss",
                        "Periodic Steady State",
                        "time",
                        "time",
                        result.time.clone(),
                        &signals,
                    )
                    .write(path, format)?;
                }
            }
            Ok(())
        },
    )?;

    if !ctx.quiet {
        crate::console::line(format_args!(
            "  PSS waveforms exported to: {}",
            output_path.display()
        ))?;
    }
    Ok(())
}

/// The `--hb-freq` route. It supersedes the deck's authored cards outright, so
/// no authored `.PAC`/`.PNOISE`/`.ENVELOPE` can consume its carrier and the
/// operating point is not retained.
pub(super) fn run_hb(ctx: &RunContext<'_>, freq: f64, harmonics: usize) -> Result<(), CliError> {
    let config = rspice_core::analysis::HbConfig::new(freq).with_harmonics(harmonics);
    let fundamental = config.fundamental_freq;
    let artifact = ctx.resolve_periodic_analysis("hb")?;
    let hb_result = solve_hb(ctx, config)?;
    if let Some(path) = &artifact.path {
        export_hb(ctx, artifact.analysis, path, fundamental, &hb_result.result)?;
    }
    Ok(())
}

/// Solve one harmonic-balance configuration and report it on the console.
fn solve_hb(
    ctx: &RunContext<'_>,
    config: rspice_core::analysis::HbConfig,
) -> Result<rspice_core::engine::HbAnalysisResult, CliError> {
    if !ctx.quiet {
        crate::console::line(format_args!(
            "Running HB analysis: f₀ = {:.3e} Hz, {} harmonics",
            config.fundamental_freq, config.num_harmonics
        ))?;
    }

    let harmonics = config.num_harmonics;
    let hb_result = ctx
        .engine
        .run_hb_with_abort(ctx.netlist, config, &crate::abort::ProcessAbort)
        .map_err(|error| map_advanced_simulation_error(ctx, "HB", error))?;
    ensure_not_cancelled(ctx)?;
    if !ctx.quiet {
        crate::console::line(format_args!("✓ HB converged"))?;
        crate::console::line(format_args!("  Nodes: {}", hb_result.result.num_nodes()))?;
        crate::console::line(format_args!(
            "  Harmonics: {}",
            hb_result.result.num_harmonics
        ))?;

        if ctx.verbose && !hb_result.result.spectral_voltages.is_empty() {
            crate::console::line(format_args!("\n  Spectral content (first node):"))?;
            let sv = &hb_result.result.spectral_voltages[0];
            for k in 0..=4.min(harmonics) {
                crate::console::line(format_args!(
                    "    H{}: mag={:.6e}, phase={:.2}°",
                    k,
                    sv.magnitude(k),
                    sv.phase(k).to_degrees()
                ))?;
            }
        }
    }
    Ok(hb_result)
}

/// Write the harmonic-balance spectrum: harmonic frequencies as the scale,
/// one complex column per retained node voltage or MNA branch current.
fn export_hb(
    ctx: &RunContext<'_>,
    analysis_id: rspice_core::execution::AnalysisInstanceId,
    output_path: &std::path::Path,
    fundamental: f64,
    result: &rspice_core::analysis::HbResult,
) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;

    let num_coeffs = result
        .spectral_voltages
        .iter()
        .map(|sv| sv.coefficients.len())
        .chain(
            result
                .mna_branch_currents
                .iter()
                .map(|branch| branch.coefficients.len()),
        )
        .max()
        .unwrap_or(0);
    let frequencies: Vec<f64> = result
        .spectral_voltages
        .iter()
        .map(|sv| &sv.frequencies)
        .chain(
            result
                .mna_branch_currents
                .iter()
                .map(|branch| &branch.frequencies),
        )
        .find(|frequencies| frequencies.len() == num_coeffs)
        .cloned()
        .unwrap_or_else(|| (0..num_coeffs).map(|k| fundamental * k as f64).collect());

    let mut signals: Vec<crate::commands::run_signals::ComplexSignal> = result
        .spectral_voltages
        .iter()
        .map(|sv| {
            let mut real = Vec::with_capacity(num_coeffs);
            let mut imag = Vec::with_capacity(num_coeffs);
            for k in 0..num_coeffs {
                let c = sv
                    .coefficients
                    .get(k)
                    .copied()
                    .unwrap_or_else(|| rspice_core::Complex64::new(0.0, 0.0));
                real.push(c.re);
                imag.push(c.im);
            }
            crate::commands::run_signals::ComplexSignal {
                display_name: format!("V({})", sv.node_name),
                raw_name: sv.node_name.clone(),
                kind: crate::commands::run_signals::SignalKind::Voltage,
                real,
                imag,
            }
        })
        .collect();
    signals.extend(result.mna_branch_currents.iter().map(|branch| {
        let mut real = Vec::with_capacity(num_coeffs);
        let mut imag = Vec::with_capacity(num_coeffs);
        for harmonic in 0..num_coeffs {
            let coefficient = branch
                .coefficients
                .get(harmonic)
                .copied()
                .unwrap_or_else(|| rspice_core::Complex64::new(0.0, 0.0));
            real.push(coefficient.re);
            imag.push(coefficient.im);
        }
        crate::commands::run_signals::ComplexSignal {
            display_name: format!("I({})", branch.device_name),
            raw_name: branch.device_name.clone(),
            kind: crate::commands::run_signals::SignalKind::Current,
            real,
            imag,
        }
    }));
    let signals = crate::commands::run_signals::complex_export_signals(
        ctx.netlist,
        rspice_core::execution::AnalysisResultKind::HarmonicBalance,
        "HB",
        &frequencies,
        &signals,
        &crate::abort::ProcessAbort,
    )
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some("HB output projection".to_string()),
    })?;

    super::document::publish_analysis_result(
        ctx,
        output_path,
        analysis_id,
        super::document::complex_schema(&signals)?,
        || {
            rspice_core::execution::AnalysisResultDocument::from_harmonic_balance(
                analysis_id,
                result,
            )
        },
        |path, format| {
            if matches!(format, crate::cli::OutputFormat::Hdf5) {
                let mut data = crate::hdf5::Hdf5SimulationData::new();
                data.title = "Harmonic Balance Spectrum".to_string();
                data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                let mut section = crate::hdf5::Hdf5AcSection::new(frequencies.clone());
                for signal in &signals {
                    section.add_signal(
                        signal.display_name.clone(),
                        signal.unit_symbol(),
                        signal.real.clone(),
                        signal.imag.clone(),
                    );
                }
                data.ac = Some(section);
                crate::hdf5::write_hdf5(path, &data)
                    .map_err(|err| super::shared::map_hdf5_output_error(path, err))
            } else {
                super::export::complex_table(
                    "hb",
                    "Harmonic Balance Spectrum",
                    frequencies.clone(),
                    &signals,
                )
                .write(path, format)
            }
        },
    )?;

    if !ctx.quiet {
        crate::console::line(format_args!(
            "  HB spectrum exported to: {}",
            output_path.display()
        ))?;
    }
    Ok(())
}

/// Run one authored `.SP` card and publish its scattering sweep.
///
/// Port collection, the excitation sweep, the wave-to-scattering conversion
/// and — under `DONOISE` — the port-noise covariance solve and its two-port
/// derivation are all one core operation with one validity policy. This
/// command supplies the artifact destination and the output representation
/// and nothing else; it does not decide what a `.SP` card means.
pub(super) fn run_sparam_from_command(
    ctx: &RunContext<'_>,
    card: &rspice_core::netlist::AnalysisCommand,
) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;
    let run = ctx
        .engine
        .run_sp_with_abort(ctx.netlist, card, &crate::abort::ProcessAbort)
        .map_err(|error| map_advanced_simulation_error(ctx, "S-Parameters", error))?;
    publish_sparam_run(ctx, &run, "sp")
}

fn publish_sparam_run(
    ctx: &RunContext<'_>,
    run: &rspice_core::engine::SParameterRun,
    kind: &str,
) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;

    let frequencies = run
        .scattering
        .data
        .iter()
        .map(|matrix| matrix.frequency)
        .collect::<Vec<_>>();
    let scattering = scattering_cube(&run.scattering);
    if !ctx.quiet {
        crate::console::line(format_args!(
            "Running {}-port S-parameter analysis: {} frequency points",
            run.ports.len(),
            frequencies.len()
        ))?;
        if let Some(first) = scattering
            .first()
            .and_then(|row| row.first())
            .and_then(|series| series.first())
        {
            crate::console::line(format_args!(
                "  @ {:e} Hz: |S_1_1|={:.4}",
                frequencies.first().copied().unwrap_or(0.0),
                first.norm()
            ))?;
        }
    }

    let Some(resolved) = ctx.resolve_output(kind) else {
        return Ok(());
    };
    let analysis_id = resolved.analysis(kind)?;
    let output_path = &resolved.path;
    if touchstone_extension_matches(output_path, run.ports.len())? {
        if run.port_noise.is_some() {
            return Err(CliError::InvalidArgument {
                message: format!(
                    "{} cannot retain the full .SP DONOISE covariance and normalization provenance",
                    output_path.display()
                ),
                suggestion: Some("use CSV, TSV, raw, or HDF5 output for .SP DONOISE".to_string()),
            });
        }
        write_touchstone_nport(output_path, &run.ports, &frequencies, &scattering)?;
        ctx.record_output(output_path.clone());
    } else {
        let (table, schema) = sparameter_export_table(run, frequencies, &scattering, kind)?;
        super::document::publish_table_result(
            ctx,
            output_path,
            analysis_id,
            schema,
            &table,
            || {
                rspice_core::execution::AnalysisResultDocument::from_s_parameters(
                    analysis_id,
                    &run.scattering,
                )
            },
        )?;

        // Port noise is the `.SP` card's second result. It shares the card's
        // analysis identity, exactly as the shared document declares, and is
        // published as its own typed artifact beside the scattering one: the
        // S-parameter payload has no room for a covariance sweep, and folding
        // one into the other would make each document describe two studies.
        // The flat formats keep both in one table, because they have no
        // per-family payload to separate.
        if let Some(noise) = &run.port_noise
            && matches!(ctx.format, crate::cli::OutputFormat::Json)
        {
            // The noise document is a sibling of the scattering one, so it
            // takes that artifact's own path with `port-noise` composed into
            // it. Resolving a second output namespace would give the two
            // documents the same path whenever the deck authors only this
            // card, and the second would overwrite the first.
            let noise_path = super::sibling_output_path(output_path, "port-noise");
            let builder =
                rspice_core::execution::AnalysisResultDocument::from_port_noise(analysis_id, noise)
                    .map_err(|error| super::document::document_error(ctx, analysis_id, error))?;
            let document = super::document::finish(ctx, analysis_id, builder)?;
            let publication = super::document::typed_publication(&noise_path, &document)?;
            super::document::write_document(ctx, &noise_path, &document)?;
            ctx.record_published(publication);
            if !ctx.quiet {
                crate::console::line(format_args!(
                    "  Port noise exported to: {}",
                    noise_path.display()
                ))?;
            }
        }
    }

    if !ctx.quiet {
        crate::console::line(format_args!(
            "  S-parameters exported to: {}",
            output_path.display()
        ))?;
    }
    Ok(())
}

/// The swept scattering matrix in the `[row][column][frequency]` shape the
/// shared Touchstone writer and the flat column projection both take.
fn scattering_cube(result: &s_param::SParameterResult) -> Vec<Vec<Vec<rspice_core::Complex64>>> {
    let count = result.ports.len();
    (0..count)
        .map(|row| {
            (0..count)
                .map(|column| {
                    result
                        .data
                        .iter()
                        .map(|matrix| matrix.get(row + 1, column + 1))
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn sparameter_export_table(
    run: &rspice_core::engine::SParameterRun,
    frequencies: Vec<f64>,
    scattering: &[Vec<Vec<rspice_core::Complex64>>],
    kind: &str,
) -> Result<(ExportTable, SignalSchema), CliError> {
    use crate::commands::export_table::{ColumnData, ExportColumn, stated_unit, unit_type};
    use rspice_core::execution::{
        SignalDescriptor, SignalKind, SignalOwner, SignalShape, SignalUnit, SignalValueType,
    };

    let count = run.ports.len();
    let mut columns = Vec::with_capacity(
        count * count + count + run.port_noise.as_ref().map_or(0, |_| count * count + 6),
    );
    let mut descriptors = Vec::with_capacity(columns.capacity());
    let mut push = |name: String, data: ColumnData, unit: SignalUnit| {
        let value_type = if matches!(data, ColumnData::Complex { .. }) {
            SignalValueType::Complex
        } else {
            SignalValueType::Real
        };
        columns.push(ExportColumn {
            name: name.clone(),
            var_type: unit_type(&unit, "parameter").to_string(),
            unit: stated_unit(&unit),
            data,
        });
        descriptors.push(SignalDescriptor::new(
            &name,
            &name,
            SignalKind::Scalar,
            unit,
            value_type,
            SignalShape::Scalar,
            SignalOwner::Analysis,
        ));
    };
    let complex = |values: &[rspice_core::Complex64]| ColumnData::Complex {
        real: values.iter().map(|value| value.re).collect(),
        imag: values.iter().map(|value| value.im).collect(),
    };
    let legacy_names = kind == "sparam";

    for (row, columns) in scattering.iter().enumerate() {
        for (column, series) in columns.iter().enumerate() {
            let (row, column, series) = if legacy_names {
                (column, row, &scattering[column][row])
            } else {
                (row, column, series)
            };
            let name = if legacy_names {
                format!("S{}{}", row + 1, column + 1)
            } else {
                format!("S_{}_{}", row + 1, column + 1)
            };
            push(name, complex(series), SignalUnit::Dimensionless);
        }
    }

    if let Some(noise) = &run.port_noise {
        for row in 0..count {
            for column in 0..count {
                let series = noise
                    .points
                    .iter()
                    .map(|point| {
                        point
                            .current_correlation
                            .get(row)
                            .and_then(|entries| entries.get(column))
                            .copied()
                            .unwrap_or(rspice_core::Complex64::ZERO)
                    })
                    .collect::<Vec<_>>();
                push(
                    format!("CY_A2_per_Hz_{}_{}", row + 1, column + 1),
                    complex(&series),
                    SignalUnit::Custom("A^2/Hz".to_string()),
                );
            }
        }
        let constant = |value| vec![rspice_core::Complex64::new(value, 0.0); frequencies.len()];
        push(
            "noise_reference_temperature_K".to_string(),
            complex(&constant(noise.reference_temperature_kelvin)),
            SignalUnit::Custom("K".to_string()),
        );
        push(
            "noise_normalization_4kT_J".to_string(),
            complex(&constant(
                4.0 * rspice_core::constants::K_BOLTZMANN * noise.reference_temperature_kelvin,
            )),
            SignalUnit::Custom("J".to_string()),
        );
        if let Some(parameters) = &noise.two_port {
            let real_values = |project: fn(&s_param::TwoPortNoise) -> f64| {
                parameters
                    .iter()
                    .map(|parameter| rspice_core::Complex64::new(project(parameter), 0.0))
                    .collect::<Vec<_>>()
            };
            push(
                "noise_resistance_ohm".to_string(),
                complex(&real_values(|parameter| parameter.noise_resistance)),
                SignalUnit::Ohm,
            );
            push(
                "noise_factor_linear".to_string(),
                complex(&real_values(|parameter| parameter.noise_factor)),
                SignalUnit::Dimensionless,
            );
            push(
                "minimum_noise_factor_linear".to_string(),
                complex(&real_values(|parameter| parameter.minimum_noise_factor)),
                SignalUnit::Dimensionless,
            );
            let optimum = parameters
                .iter()
                .map(|parameter| parameter.optimum_source_reflection)
                .collect::<Vec<_>>();
            push(
                "optimum_source_reflection".to_string(),
                complex(&optimum),
                SignalUnit::Dimensionless,
            );
        }
    }
    // Append references so established matrix/noise column ordering stays
    // stable. Each coefficient is meaningful only with its port normalization.
    for port in &run.ports {
        push(
            format!("Z0({})", port.number),
            ColumnData::Real(vec![port.z0; frequencies.len()]),
            SignalUnit::Ohm,
        );
    }
    let schema = super::document::distinct_schema(descriptors)?;
    Ok((
        ExportTable {
            scale_unit: Some("Hz".to_string()),
            analysis: kind.to_string(),
            plot_name: "S-Parameters".to_string(),
            scale_name: "frequency".to_string(),
            scale_type: "frequency".to_string(),
            scale: frequencies,
            columns,
        },
        schema,
    ))
}

fn touchstone_extension_matches(
    path: &std::path::Path,
    num_ports: usize,
) -> Result<bool, CliError> {
    let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
        return Ok(false);
    };
    if extension.eq_ignore_ascii_case("snp") {
        return Ok(true);
    }
    let Some(ports) = extension
        .strip_prefix(['s', 'S'])
        .and_then(|value| value.strip_suffix(['p', 'P']))
        .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
    else {
        return Ok(false);
    };
    if ports.parse::<usize>().ok() == Some(num_ports) && num_ports > 0 {
        return Ok(true);
    }
    Err(CliError::InvalidArgument {
        message: format!(
            "Touchstone output '{}' declares {ports} ports, but the result has {num_ports}",
            path.display()
        ),
        suggestion: Some(format!("use an .s{num_ports}p extension for this network")),
    })
}

/// Write an N-port Touchstone v1 file through the shared core writer.
///
/// Formatting, the option line, and the mixed-reference-impedance refusal all
/// live in `rspice_core`, so a deck exported here and through the Python
/// bindings produces the same bytes.
fn write_touchstone_nport(
    path: &std::path::Path,
    ports: &[s_param::SParameterPort],
    frequencies: &[f64],
    s: &[Vec<Vec<rspice_core::Complex64>>],
) -> Result<(), CliError> {
    if ports.is_empty() {
        return Ok(());
    }
    let reference_impedances: Vec<f64> = ports.iter().map(|port| port.z0).collect();
    let comments = vec![format!("{}-port S-parameters", ports.len())];
    let document = s_param::touchstone(
        &s_param::TouchstoneInput {
            frequencies,
            parameters: s,
            reference_impedances: &reference_impedances,
            comments: &comments,
        },
        s_param::TouchstoneFormat::RealImaginary,
        s_param::TouchstoneFrequencyUnit::Hz,
    )
    .map_err(|message| CliError::InvalidArgument {
        message,
        suggestion: Some("use CSV, JSON, or HDF5 output for per-port z0 values".to_string()),
    })?;
    publish::artifact(path, |writer| {
        writer
            .write_all(document.as_bytes())
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))
}

/// Two-port S-parameter extraction over the deck's `.AC` sweep.
///
/// A source behind Z0 terminates each configured reference plane. The shared SP
/// runner supplies the bias, independent port excitations, completed grid,
/// and scattering matrix; all exports use that same result.
pub(super) fn run_sparam(ctx: &RunContext<'_>, ports_spec: &str, z0: f64) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;
    if !z0.is_finite() || z0 <= 0.0 {
        return Err(CliError::InvalidArgument {
            message: format!("--sparam-z0 must be a positive impedance, got {z0}"),
            suggestion: Some("e.g. --sparam-z0 50".to_string()),
        });
    }
    let port_nodes: Vec<String> = ports_spec
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if port_nodes.len() != 4 {
        return Err(CliError::InvalidArgument {
            message: format!(
                "--sparam needs four comma-separated port nodes (P1+,P1-,P2+,P2-), got {}",
                port_nodes.len()
            ),
            suggestion: Some("e.g. --sparam \"in,0,out,0\"".to_string()),
        });
    }

    // The deck's .AC card defines the sweep.
    let Some(rspice_core::netlist::AnalysisCommand::Ac {
        variation,
        points,
        start_freq,
        stop_freq,
    }) = ctx
        .netlist
        .analyses
        .iter()
        .find(|a| matches!(a, rspice_core::netlist::AnalysisCommand::Ac { .. }))
        .cloned()
    else {
        return Err(CliError::SimulationError {
            message: "--sparam requires a .AC card in the deck to define the sweep".to_string(),
            analysis: Some("S-Parameters".to_string()),
        });
    };
    let frequencies =
        super::shared::generate_frequency_sweep(variation, points, start_freq, stop_freq)?;

    let mut netlist = ctx.netlist.clone();
    let ports = (0..2)
        .map(|index| s_param::Port {
            number: index + 1,
            node_pos: port_nodes[2 * index].clone(),
            node_neg: port_nodes[2 * index + 1].clone(),
            z0,
        })
        .collect::<Vec<_>>();
    s_param::declare_ports_with_abort(&mut netlist, &ports, &crate::abort::ProcessAbort).map_err(
        |error| match error {
            s_param::PortError::Aborted => super::cancellation_cli_error(ctx.args.timeout),
            other => CliError::InvalidArgument {
                message: other.to_string(),
                suggestion: None,
            },
        },
    )?;
    let run = ctx
        .engine
        .run_sp_over_grid_with_abort(&netlist, &frequencies, false, &crate::abort::ProcessAbort)
        .map_err(|error| map_advanced_simulation_error(ctx, "S-Parameters", error))?;
    publish_sparam_run(ctx, &run, "sparam")
}

/// Run one authored `.DCMATCH` card.
///
/// Like `.TF`, this is a single-point analysis: one nominal operating point,
/// one variance sum, no sweep and no axis of its own. The summary quotes the
/// probe, its nominal value, the three sigmas, the total at the multiple the
/// card asked for, and then the ranked contributors — because which instance
/// owns the spread is the question the card was authored to ask.
pub(super) fn run_dc_match_from_command(
    ctx: &RunContext<'_>,
    card: &rspice_core::netlist::DcMatchCard,
) -> Result<(), CliError> {
    if !ctx.quiet {
        crate::console::line(format_args!("Running DC mismatch analysis..."))?;
    }
    let result = ctx
        .engine
        .run_dc_match_with_abort(ctx.netlist, card, &crate::abort::ProcessAbort)
        .map_err(|error| map_advanced_simulation_error(ctx, "DC Mismatch", error))?;
    ensure_not_cancelled(ctx)?;
    report_dc_match(ctx, &result)?;
    export_dc_match(ctx, &result)
}

/// Unit symbol of a `.DCMATCH` probe, read off the probe the result names.
///
/// The shared document decides the same way, so the printed unit and the
/// document's declared unit cannot disagree.
fn dc_match_unit(result: &rspice_core::analysis::dcmatch::DcMatchResult) -> &'static str {
    if result.output.starts_with('I') {
        "A"
    } else {
        "V"
    }
}

/// Print one mismatch result.
///
/// Every quoted key is a scalar the typed document publishes under the same
/// name, so a reader who moves from the terminal to the artifact does not
/// have to translate.
fn report_dc_match(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::dcmatch::DcMatchResult,
) -> Result<(), CliError> {
    if ctx.quiet {
        return Ok(());
    }
    let unit = dc_match_unit(result);
    crate::console::line(format_args!("DC mismatch information:"))?;
    crate::console::line(format_args!("output = {}", result.output))?;
    crate::console::line(format_args!(
        "nominal_value = {:.6e} {unit}",
        result.nominal_value
    ))?;
    crate::console::line(format_args!(
        "sigma_total = {:.6e} {unit}",
        result.sigma_total
    ))?;
    crate::console::line(format_args!(
        "sigma_mismatch = {:.6e} {unit}",
        result.sigma_mismatch
    ))?;
    crate::console::line(format_args!(
        "sigma_process = {:.6e} {unit}",
        result.sigma_process
    ))?;
    crate::console::line(format_args!(
        "quoted_sigma = {:.6e} {unit} ({:.6e} sigma)",
        result.quoted_sigma(),
        result.sigma_multiplier
    ))?;
    // Printed only when the design declared one: a line reading "0, 0" on
    // every uncorrelated deck would say nothing and hide the case that matters.
    if result.applied_correlations_mismatch + result.applied_correlations_process > 0 {
        crate::console::line(format_args!(
            "correlations applied: process {}, mismatch {}",
            result.applied_correlations_process, result.applied_correlations_mismatch
        ))?;
    }
    if result.contributors.is_empty() {
        crate::console::line(format_args!(
            "contributors: none of the {} evaluated pass the card's limits",
            result.evaluated_contributors
        ))?;
        return Ok(());
    }
    crate::console::line(format_args!(
        "contributors ({} listed of {} evaluated, largest variance share first, by magnitude):",
        result.contributors.len(),
        result.evaluated_contributors
    ))?;
    crate::console::line(format_args!(
        "  {:<24} {:<10} {:<10} {:>13} {:>13} {:>13}",
        "INSTANCE", "PARAMETER", "SCOPE", "SHARE", "SENSITIVITY", "CONTRIBUTION"
    ))?;
    for contributor in &result.contributors {
        crate::console::line(format_args!(
            "  {:<24} {:<10} {:<10} {:>13.6e} {:>13.6e} {:>13.6e}",
            truncate(&contributor.instance, 24),
            truncate(&contributor.parameter, 10),
            contributor.scope.tag(),
            contributor.share,
            contributor.sensitivity,
            contributor.contribution
        ))?;
    }
    Ok(())
}

/// Write one mismatch result.
///
/// The flat table is a single row of named scalars, the way `.TF`'s is: the
/// three sigmas and the nominal value, then each retained contributor's
/// displacement and variance share. The ranked table with each contributor's
/// own sigma and derivative stays in the typed document, which is the only
/// representation that can carry a table of rows.
fn export_dc_match(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::dcmatch::DcMatchResult,
) -> Result<(), CliError> {
    let Some(resolved) = ctx.resolve_output("dcmatch") else {
        return Ok(());
    };
    let analysis_id = resolved.analysis("dcmatch")?;
    use super::export::{ColumnData, ExportColumn, ExportTable};

    let unit = dc_match_unit(result);
    let scalar = |name: String, var_type: &str, value: f64| ExportColumn {
        unit: None,
        name,
        var_type: var_type.to_string(),
        data: ColumnData::Real(vec![value]),
    };
    let quantity = if unit == "A" { "current" } else { "voltage" };
    let mut columns = vec![
        scalar("nominal_value".to_owned(), quantity, result.nominal_value),
        scalar("sigma_total".to_owned(), quantity, result.sigma_total),
        scalar("sigma_mismatch".to_owned(), quantity, result.sigma_mismatch),
        scalar("sigma_process".to_owned(), quantity, result.sigma_process),
        scalar("quoted_sigma".to_owned(), quantity, result.quoted_sigma()),
    ];
    for contributor in &result.contributors {
        let owner = format!("{}/{}", contributor.instance, contributor.parameter);
        columns.push(scalar(
            format!("contribution({owner})"),
            quantity,
            contributor.contribution,
        ));
        columns.push(scalar(
            format!("share({owner})"),
            "share",
            contributor.share,
        ));
    }
    let table = ExportTable {
        scale_unit: None,
        analysis: "dcmatch".to_string(),
        plot_name: "DC Mismatch".to_string(),
        scale_name: "point".to_string(),
        scale_type: "index".to_string(),
        scale: vec![0.0],
        columns,
    };
    super::document::publish_table_result(
        ctx,
        &resolved.path,
        analysis_id,
        // The mismatch result publishes named scalars and a ranked
        // contributor table rather than a series, so its typed values live in
        // the document's payload.
        super::document::empty_schema(),
        &table,
        || rspice_core::execution::AnalysisResultDocument::from_dc_match(analysis_id, result),
    )?;

    if !ctx.quiet {
        crate::console::line(format_args!(
            "  DC mismatch exported to: {}",
            resolved.path.display()
        ))?;
    }
    Ok(())
}
