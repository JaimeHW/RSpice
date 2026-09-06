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

use super::RunContext;
use super::basic::run_dc_op;
use crate::cli::{CliError, map_atomic_output_error};
use crate::commands::publish;

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
    frequencies: &[f64],
) -> Result<(), CliError> {
    // The default harmonic order, the multi-tone common basis, and the
    // `.OPTIONS HBINT NUMFREQ` collocation rule all belong to `rspice-core`.
    let config = rspice_core::analysis::HbConfig::from_hb_card(
        frequencies,
        &ctx.netlist.options.hb_num_frequencies,
    )
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
/// node voltage — the same shape as a DC sweep with the stepped quantity
/// as the abscissa.
pub(super) fn export_step_sweep(
    ctx: &RunContext<'_>,
    step_name: &str,
    sweep_results: &[(f64, rspice_core::solver::SimulationResult)],
) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;
    let Some(ref output_path) = ctx.output_path_for("step") else {
        return Ok(());
    };

    let sweep_vals: Vec<f64> = sweep_results.iter().map(|(v, _)| *v).collect();
    let step_error = |source| CliError::CoreSimulationError {
        source,
        analysis: Some("Step output projection".to_string()),
    };
    let inventory = crate::commands::run_signals::dc_sweep_voltage_signals(sweep_results)
        .map_err(step_error)?;
    let signals = crate::commands::run_signals::scalar_export_signals(
        ctx.netlist,
        rspice_core::execution::AnalysisResultKind::DcSweep,
        "Step",
        &sweep_vals,
        &inventory,
        &crate::abort::ProcessAbort,
    )
    .map_err(step_error)?;

    match ctx.format {
        crate::cli::OutputFormat::Hdf5 => {
            let mut data = crate::hdf5::Hdf5SimulationData::new();
            data.title = "Step Sweep".to_string();

            let mut sweep = crate::hdf5::Hdf5WaveformSection::new(step_name, sweep_vals.clone());
            for signal in &signals {
                sweep.add_typed_signal(
                    signal.display_name.clone(),
                    signal.raw_variable_type(),
                    signal.values.clone(),
                );
            }
            data.dc_sweep = Some(sweep);

            crate::hdf5::write_hdf5(output_path, &data)
                .map_err(|err| super::shared::map_hdf5_output_error(output_path, err))?;
        }
        crate::cli::OutputFormat::Raw | crate::cli::OutputFormat::RawAscii => {
            let node_names: Vec<String> = signals
                .iter()
                .map(|signal| signal.raw_name.clone())
                .collect();
            let node_waveforms: Vec<Vec<f64>> =
                signals.iter().map(|signal| signal.values.clone()).collect();
            rspice_core::io::export_dc_sweep(
                output_path,
                &sweep_vals,
                step_name,
                &node_names,
                &node_waveforms,
                match ctx.format {
                    crate::cli::OutputFormat::RawAscii => rspice_core::io::RawFormat::Ascii,
                    _ => rspice_core::io::RawFormat::Binary,
                },
            )
            .map_err(|e| CliError::OutputError {
                path: output_path.clone(),
                source: e,
            })?;
        }
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
                &signals,
            )
            .write(output_path, ctx.format)?;
        }
    }

    if !ctx.quiet {
        println!("  Step results exported to: {}", output_path.display());
    }
    Ok(())
}

pub(super) fn run_monte_carlo(
    ctx: &RunContext<'_>,
    num_runs: usize,
    seed: u64,
    distribution: rspice_core::analysis::Distribution,
    parameter_filter: Option<&[String]>,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running Monte Carlo analysis: {} iterations (seed={})",
            num_runs, seed
        );
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

    match ctx.engine.run_monte_carlo_with_options_and_abort(
        ctx.netlist,
        num_runs,
        seed,
        distribution,
        parameter_filter,
        &crate::abort::ProcessAbort,
    ) {
        Ok(result) => {
            pb.finish_and_clear();
            ensure_not_cancelled(ctx)?;

            if result.num_failures >= num_runs {
                return Err(CliError::simulation_error_in(
                    format!("all {} Monte Carlo runs failed to converge", num_runs),
                    "Monte Carlo",
                ));
            }
            if result.num_failures > 0 {
                eprintln!(
                    "Warning: {}/{} Monte Carlo runs failed to converge; statistics \
                     cover the surviving runs only",
                    result.num_failures, num_runs
                );
            }

            // Deterministic ordering for display and export.
            let mut variables: Vec<&rspice_core::analysis::VariableStatistics> =
                result.variables.values().collect();
            variables.sort_by(|a, b| a.name.cmp(&b.name));

            if !ctx.quiet {
                println!(
                    "✓ Monte Carlo complete: {} runs (seed={})",
                    result.num_runs, seed
                );
                if !variables.is_empty() {
                    println!(
                        "  {:<24} {:>13} {:>13} {:>13} {:>13}",
                        "VARIABLE", "MEAN", "STD", "MIN", "MAX"
                    );
                    for stats in &variables {
                        println!(
                            "  {:<24} {:>13.6e} {:>13.6e} {:>13.6e} {:>13.6e}",
                            stats.name, stats.mean, stats.std_dev, stats.min, stats.max
                        );
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
    let runs: Vec<f64> = (1..=num_samples).map(|i| i as f64).collect();
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
                let mut sweep = crate::hdf5::Hdf5WaveformSection::new("run", runs.clone());
                for signal in &signals {
                    sweep.add_typed_signal(
                        signal.display_name.clone(),
                        signal.raw_variable_type(),
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
                "run",
                "index",
                runs.clone(),
                &signals,
            )
            .write(path, format),
        },
    )?;

    if !ctx.quiet {
        println!(
            "  Monte Carlo samples exported to: {}",
            output_path.display()
        );
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

    run_monte_carlo(ctx, mc_cmd.runs, seed, distribution, parameter_filter)
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
    announce_pss(ctx, &config);
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
    );
    if let Some(path) = &artifact.path {
        export_pss(ctx, artifact.analysis, path, &pss_result.result)?;
    }
    Ok(())
}

/// Announce one periodic steady state before the shooting solve starts.
pub(super) fn announce_pss(ctx: &RunContext<'_>, config: &rspice_core::analysis::PssConfig) {
    if ctx.quiet {
        return;
    }
    if config.is_autonomous() {
        println!(
            "Running PSS analysis: autonomous, {} harmonics",
            config.num_harmonics
        );
    } else {
        println!(
            "Running PSS analysis: f₀ = {:.3e} Hz, {} harmonics",
            config.fundamental_freq, config.num_harmonics
        );
    }
}

/// Report one converged periodic steady state on the console.
pub(super) fn report_pss(
    ctx: &RunContext<'_>,
    iterations: usize,
    period: f64,
    result: &rspice_core::analysis::PssResult,
) {
    if ctx.quiet {
        return;
    }
    println!("✓ PSS converged in {iterations} iterations");
    println!("  Period: {period:.6e} s");
    println!("  Nodes: {}", result.num_nodes());

    if ctx.verbose && result.num_nodes() > 0 {
        println!("\n  Harmonic content (node 1):");
        for harmonic in &result.harmonics(1, 5) {
            println!(
                "    H{}: mag={:.6e}, phase={:.2}° (f={:.3e} Hz)",
                harmonic.harmonic_number, harmonic.magnitude, harmonic.phase, harmonic.frequency
            );
        }
    }
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
                            signal.values.clone(),
                        );
                    }
                    data.transient = Some(section);
                    crate::hdf5::write_hdf5(path, &data)
                        .map_err(|err| super::shared::map_hdf5_output_error(path, err))?;
                }
                crate::cli::OutputFormat::Raw | crate::cli::OutputFormat::RawAscii => {
                    let node_names: Vec<String> = signals
                        .iter()
                        .map(|signal| signal.raw_name.clone())
                        .collect();
                    let waveforms: Vec<Vec<f64>> =
                        signals.iter().map(|signal| signal.values.clone()).collect();
                    rspice_core::io::export_transient(
                        path,
                        &result.time,
                        &node_names,
                        &waveforms,
                        match format {
                            crate::cli::OutputFormat::RawAscii => rspice_core::io::RawFormat::Ascii,
                            _ => rspice_core::io::RawFormat::Binary,
                        },
                    )
                    .map_err(|e| CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    })?;
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

    ctx.record_output(output_path.to_path_buf());
    if !ctx.quiet {
        println!("  PSS waveforms exported to: {}", output_path.display());
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
        println!(
            "Running HB analysis: f₀ = {:.3e} Hz, {} harmonics",
            config.fundamental_freq, config.num_harmonics
        );
    }

    let harmonics = config.num_harmonics;
    let hb_result = ctx
        .engine
        .run_hb_with_abort(ctx.netlist, config, &crate::abort::ProcessAbort)
        .map_err(|error| map_advanced_simulation_error(ctx, "HB", error))?;
    ensure_not_cancelled(ctx)?;
    if !ctx.quiet {
        println!("✓ HB converged");
        println!("  Nodes: {}", hb_result.result.num_nodes());
        println!("  Harmonics: {}", hb_result.result.num_harmonics);

        if ctx.verbose && !hb_result.result.spectral_voltages.is_empty() {
            println!("\n  Spectral content (first node):");
            let sv = &hb_result.result.spectral_voltages[0];
            for k in 0..=4.min(harmonics) {
                println!(
                    "    H{}: mag={:.6e}, phase={:.2}°",
                    k,
                    sv.magnitude(k),
                    sv.phase(k).to_degrees()
                );
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

    ctx.record_output(output_path.to_path_buf());
    if !ctx.quiet {
        println!("  HB spectrum exported to: {}", output_path.display());
    }
    Ok(())
}

pub(super) fn run_corner_sweep(ctx: &RunContext<'_>, corners_str: &str) -> Result<(), CliError> {
    ensure_not_cancelled(ctx)?;
    let corners: Vec<String> = corners_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if corners.is_empty() {
        return Err(CliError::InvalidArgument {
            message: "--corners requires at least one corner name".to_string(),
            suggestion: Some("e.g. --corners tt,ss,ff".to_string()),
        });
    }

    let corner_lib = match ctx.args.corner_lib.as_ref() {
        Some(lib) => {
            if !lib.exists() {
                return Err(CliError::InputNotFound {
                    path: lib.clone(),
                    source: std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Corner library not found",
                    ),
                });
            }
            // Absolute so the reference resolves regardless of netlist location.
            Some(std::path::absolute(lib).unwrap_or_else(|_| lib.clone()))
        }
        None => None,
    };

    if !ctx.quiet {
        println!("Running process corner sweep: {} corners", corners.len());
        match &corner_lib {
            Some(lib) => println!("  Corner library: {}", lib.display()),
            None => {
                println!("  Note: no --corner-lib given; every corner runs nominal models.");
                println!(
                    "        Provide a library with per-corner .lib sections (.lib ss ... .endl)"
                );
                println!("        to apply real corner models.");
            }
        }
    }

    let jobs = super::effective_jobs(
        ctx.args.jobs,
        corners.len(),
        ctx.engine.config().resource_limits.max_parallel_workers,
    )?;
    let results: Vec<(String, bool, bool)> = if jobs > 1 && corners.len() > 1 {
        run_corners_parallel(ctx, &corners, corner_lib.as_deref(), jobs)?
    } else {
        let mut results = Vec::with_capacity(corners.len());
        for (i, name) in corners.iter().enumerate() {
            ensure_not_cancelled(ctx)?;
            if !ctx.quiet {
                println!("\n[{}/{}] Corner: {}", i + 1, corners.len(), name);
            }

            let (simulation_passed, measurements_passed) = match corner_lib.as_deref() {
                Some(lib) => match run_corner_with_lib(ctx, lib, name) {
                    Ok(status) => status,
                    Err(e) => {
                        ensure_not_cancelled(ctx)?;
                        if !ctx.quiet {
                            eprintln!("  Corner '{}' failed: {}", name, e);
                        }
                        (false, false)
                    }
                },
                None => match run_corner_nominal(ctx, name) {
                    Ok(status) => status,
                    Err(e) => {
                        ensure_not_cancelled(ctx)?;
                        if !ctx.quiet {
                            eprintln!("  Corner '{}' failed: {}", name, e);
                        }
                        (false, false)
                    }
                },
            };

            results.push((name.clone(), simulation_passed, measurements_passed));
        }
        results
    };
    ensure_not_cancelled(ctx)?;

    if !ctx.quiet {
        println!("\n┌─────────────────────────────────────┐");
        println!("│        Corner Sweep Summary         │");
        println!("├─────────────────────────────────────┤");
        for (name, simulation_passed, measurements_passed) in &results {
            let passed = *simulation_passed && *measurements_passed;
            let status = if passed { "✓ PASS" } else { "✗ FAIL" };
            // The status field is one column wider than the frame's interior
            // minus the four padding columns, because the frame is 37 columns
            // wide: a 24-column field left this row one short of its own
            // border, which nobody could see while every rune was mojibake.
            println!("│  {:6}  {:>25}  │", name, status);
        }
        println!("└─────────────────────────────────────┘");

        let passed_count = results
            .iter()
            .filter(|(_, simulation_passed, measurements_passed)| {
                *simulation_passed && *measurements_passed
            })
            .count();
        println!(
            "\n✓ Corner sweep complete: {}/{} corners passed",
            passed_count,
            corners.len()
        );
    }

    let failed: Vec<&str> = results
        .iter()
        .filter(|(_, simulation_passed, _)| !simulation_passed)
        .map(|(name, _, _)| name.as_str())
        .collect();
    if failed.is_empty() {
        Ok(())
    } else {
        Err(CliError::simulation_error_in(
            format!("corner(s) failed: {}", failed.join(", ")),
            "Corners",
        ))
    }
}

/// The result of one corner run, returned from worker threads so the
/// parent context merges everything deterministically in corner order.
struct CornerOutcome {
    simulation_passed: bool,
    measurements_passed: bool,
    error: Option<String>,
    measurements: Vec<crate::report::MeasurementReport>,
    outputs: Vec<std::path::PathBuf>,
}

/// Everything a corner worker needs, free of the parent context's interior
/// mutability so corners can run on threads.
struct CornerSetup<'a> {
    args: &'a crate::cli::RunArgs,
    config: rspice_core::SimulationConfig,
    source: String,
    base: std::path::PathBuf,
    output: Option<std::path::PathBuf>,
    format: crate::cli::OutputFormat,
    compress: bool,
    compress_tol: f64,
}

fn corner_setup<'a>(ctx: &RunContext<'a>) -> Result<CornerSetup<'a>, CliError> {
    let source = ctx
        .netlist
        .source_text
        .clone()
        .ok_or_else(|| CliError::InternalError {
            message: "netlist source unavailable for corner re-elaboration".to_string(),
        })?;
    Ok(CornerSetup {
        args: ctx.args,
        config: ctx.engine.config().clone(),
        source,
        base: ctx
            .netlist
            .source_path
            .clone()
            .unwrap_or_else(|| ctx.args.input.clone()),
        output: ctx.output.clone(),
        format: ctx.format,
        compress: ctx.compress,
        compress_tol: ctx.compress_tol,
    })
}

/// Run one corner in isolation: re-elaborate, simulate every analysis,
/// evaluate measurements. Quiet by construction — workers must not
/// interleave solver chatter — and self-contained so it can run on any
/// thread.
fn run_corner_job(
    setup: &CornerSetup<'_>,
    lib: Option<&std::path::Path>,
    corner: &str,
) -> CornerOutcome {
    if crate::abort::reason().is_some() {
        return CornerOutcome {
            simulation_passed: false,
            measurements_passed: false,
            error: Some(format!("corner '{corner}': cancelled")),
            measurements: Vec::new(),
            outputs: Vec::new(),
        };
    }
    let corner_source = match lib {
        Some(lib) => {
            // Inject the corner's library section right below the title so
            // its models and parameters are defined before first use.
            let mut corner_source = String::with_capacity(setup.source.len() + 64);
            let mut lines = setup.source.lines();
            if let Some(title) = lines.next() {
                corner_source.push_str(title);
                corner_source.push('\n');
            }
            corner_source.push_str(&format!(".lib \"{}\" {}\n", lib.display(), corner));
            for line in lines {
                corner_source.push_str(line);
                corner_source.push('\n');
            }
            corner_source
        }
        None => setup.source.clone(),
    };

    let parse_options = super::parse_options_for_run(setup.args, setup.config.resource_limits);
    let corner_netlist = match rspice_core::Netlist::parse_with_path_and_options_and_abort(
        &corner_source,
        &setup.base,
        parse_options,
        &crate::abort::ProcessAbort,
    ) {
        Ok(netlist) => netlist,
        Err(e) => {
            return CornerOutcome {
                simulation_passed: false,
                measurements_passed: false,
                error: Some(format!("corner '{}': {}", corner, e)),
                measurements: Vec::new(),
                outputs: Vec::new(),
            };
        }
    };

    let corner_engine = rspice_core::Engine::new(setup.config.clone());
    let corner_ctx = match RunContext::for_elaborated_deck(
        &corner_engine,
        &corner_netlist,
        setup.args,
        setup.format,
        super::ElaboratedDeckPaths {
            output: corner_output_path(setup.output.as_deref(), corner),
            checkpoint: corner_output_path(setup.args.checkpoint.as_deref(), corner),
            resume: corner_output_path(setup.args.resume.as_deref(), corner),
        },
        &super::RunContextSettings {
            show_progress: false,
            compress: setup.compress,
            compress_tol: setup.compress_tol,
            coordinate: None,
            verbose: false,
            quiet: true,
        },
    ) {
        Ok(context) => context,
        Err(error) => {
            return CornerOutcome {
                simulation_passed: false,
                measurements_passed: false,
                error: Some(format!("corner '{}': {}", corner, error)),
                measurements: Vec::new(),
                outputs: Vec::new(),
            };
        }
    };

    let mut passed = true;
    let mut error: Option<String> = None;
    if corner_netlist.analyses.is_empty() {
        if let Err(e) = run_dc_op(&corner_ctx) {
            passed = false;
            error.get_or_insert(e.to_string());
        }
    } else {
        for analysis in super::analyses_in_execution_order(&corner_netlist) {
            if crate::abort::reason().is_some() {
                passed = false;
                error.get_or_insert_with(|| "cancelled".to_string());
                break;
            }
            if let Err(e) = corner_ctx.run_analysis(analysis) {
                passed = false;
                error.get_or_insert(e.to_string());
                if crate::abort::reason().is_some() {
                    break;
                }
            }
        }
    }

    if crate::abort::reason().is_some() {
        return CornerOutcome {
            simulation_passed: false,
            measurements_passed: false,
            error: error.or_else(|| Some("cancelled".to_string())),
            measurements: Vec::new(),
            outputs: Vec::new(),
        };
    }

    corner_ctx.record_unevaluated_measurements();
    let measurements: Vec<_> = corner_ctx
        .measurements
        .into_inner()
        .into_iter()
        .map(|mut m| {
            m.name = format!("{}:{}", corner, m.name);
            m
        })
        .collect();
    let measurements_passed = measurements.iter().all(|m| m.passed);

    CornerOutcome {
        simulation_passed: passed,
        measurements_passed,
        error,
        measurements,
        outputs: corner_ctx.outputs.into_inner(),
    }
}

/// Fan corners out over worker threads. Per-corner output files carry the
/// corner tag, so workers never write the same path; solver stdout is
/// suppressed and results merge into the parent in corner order, keeping
/// reports and exports deterministic regardless of completion order.
fn run_corners_parallel(
    ctx: &RunContext<'_>,
    corners: &[String],
    lib: Option<&std::path::Path>,
    jobs: usize,
) -> Result<Vec<(String, bool, bool)>, CliError> {
    use rayon::prelude::*;

    let setup = corner_setup(ctx)?;
    let workers = jobs.min(corners.len());
    if !ctx.quiet {
        println!(
            "  {} corners across {} workers (per-corner solver output suppressed)",
            corners.len(),
            workers
        );
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .thread_name(|index| format!("rspice-corner-{index}"))
        .build()
        .map_err(|error| CliError::InternalError {
            message: format!("failed to create bounded corner worker pool: {error}"),
        })?;
    // Corner artifacts belong to the run that fanned them out, so a worker
    // publishes into the transaction this thread opened rather than
    // replacing its destination on its own.
    let transaction = publish::current();
    let outcomes: Vec<CornerOutcome> = pool.install(|| {
        corners
            .par_iter()
            .map(|corner| {
                let _joined = transaction.clone().map(publish::enter);
                run_corner_job(&setup, lib, corner)
            })
            .collect()
    });
    ensure_not_cancelled(ctx)?;

    let mut results = Vec::with_capacity(corners.len());
    for (name, outcome) in corners.iter().zip(outcomes) {
        if let Some(ref err) = outcome.error
            && !ctx.quiet
        {
            eprintln!("  Corner '{}' failed: {}", name, err);
        }
        ctx.measurements.borrow_mut().extend(outcome.measurements);
        ctx.outputs.borrow_mut().extend(outcome.outputs);
        results.push((
            name.clone(),
            outcome.simulation_passed,
            outcome.measurements_passed,
        ));
    }
    Ok(results)
}

/// Re-elaborate the deck with the corner's `.lib` section applied and run
/// every analysis against the corner models.
fn run_corner_with_lib(
    ctx: &RunContext<'_>,
    lib: &std::path::Path,
    corner: &str,
) -> Result<(bool, bool), CliError> {
    let source = ctx
        .netlist
        .source_text
        .as_deref()
        .ok_or_else(|| CliError::InternalError {
            message: "netlist source unavailable for corner re-elaboration".to_string(),
        })?;

    // Inject the corner's library section right below the title so its
    // models and parameters are defined before first use.
    let mut corner_source = String::with_capacity(source.len() + 64);
    let mut lines = source.lines();
    if let Some(title) = lines.next() {
        corner_source.push_str(title);
        corner_source.push('\n');
    }
    corner_source.push_str(&format!(".lib \"{}\" {}\n", lib.display(), corner));
    for line in lines {
        corner_source.push_str(line);
        corner_source.push('\n');
    }

    let base = ctx
        .netlist
        .source_path
        .clone()
        .unwrap_or_else(|| ctx.args.input.clone());
    run_corner_serial_source(ctx, &corner_source, &base, corner)
}

fn run_corner_serial_source(
    ctx: &RunContext<'_>,
    source: &str,
    base: &std::path::Path,
    corner: &str,
) -> Result<(bool, bool), CliError> {
    ensure_not_cancelled(ctx)?;
    let parse_options = super::parse_options_for_run(ctx.args, ctx.engine.config().resource_limits);
    let corner_netlist = rspice_core::Netlist::parse_with_path_and_options_and_abort(
        source,
        base,
        parse_options,
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| match error {
        rspice_core::netlist::ParseWithAbortError::Aborted => {
            super::cancellation_cli_error(ctx.args.timeout)
        }
        rspice_core::netlist::ParseWithAbortError::Parse(error) => CliError::ParseError {
            message: format!("corner '{}': {}", corner, error),
            line: None,
            suggestion: None,
        },
    })?;

    let corner_engine = rspice_core::Engine::new(ctx.engine.config().clone());
    let corner_ctx = RunContext::for_elaborated_deck(
        &corner_engine,
        &corner_netlist,
        ctx.args,
        ctx.format,
        super::ElaboratedDeckPaths {
            output: corner_output_path(ctx.output.as_deref(), corner),
            checkpoint: corner_output_path(ctx.checkpoint.as_deref(), corner),
            resume: corner_output_path(ctx.resume.as_deref(), corner),
        },
        &super::RunContextSettings {
            show_progress: ctx.show_progress,
            compress: ctx.compress,
            compress_tol: ctx.compress_tol,
            coordinate: ctx.coordinate.clone(),
            verbose: ctx.verbose,
            quiet: ctx.quiet,
        },
    )?;

    let mut passed = true;
    if corner_netlist.analyses.is_empty() {
        if let Err(e) = run_dc_op(&corner_ctx) {
            ensure_not_cancelled(ctx)?;
            if !ctx.quiet {
                eprintln!("  DC OP failed: {}", e);
            }
            passed = false;
        }
    } else {
        for analysis in super::analyses_in_execution_order(&corner_netlist) {
            ensure_not_cancelled(ctx)?;
            if let Err(e) = corner_ctx.run_analysis(analysis) {
                ensure_not_cancelled(ctx)?;
                if !ctx.quiet {
                    eprintln!("  Analysis failed: {}", e);
                }
                passed = false;
            }
        }
    }
    ensure_not_cancelled(ctx)?;

    // Surface this corner's measurements in CI reports under tagged names.
    corner_ctx.record_unevaluated_measurements();
    let corner_measurements = corner_ctx.measurements.into_inner();
    let measurements_passed = corner_measurements.iter().all(|m| m.passed);
    ctx.measurements
        .borrow_mut()
        .extend(corner_measurements.into_iter().map(|mut m| {
            m.name = format!("{}:{}", corner, m.name);
            m
        }));
    ctx.outputs
        .borrow_mut()
        .extend(corner_ctx.outputs.into_inner());

    Ok((passed, measurements_passed))
}

/// Run the deck's analyses unchanged (no corner library available).
fn run_corner_nominal(ctx: &RunContext<'_>, corner: &str) -> Result<(bool, bool), CliError> {
    let source = ctx
        .netlist
        .source_text
        .as_deref()
        .ok_or_else(|| CliError::InternalError {
            message: "netlist source unavailable for nominal corner re-elaboration".to_string(),
        })?;
    let base = ctx
        .netlist
        .source_path
        .clone()
        .unwrap_or_else(|| ctx.args.input.clone());

    run_corner_serial_source(ctx, source, &base, corner)
}

/// `results.csv` -> `results.ss.csv` so corner exports cannot collide.
fn corner_output_path(
    output: Option<&std::path::Path>,
    corner: &str,
) -> Option<std::path::PathBuf> {
    let path = output?;
    let mut file_name = path
        .file_stem()
        .map(|stem| stem.to_os_string())
        .unwrap_or_default();
    file_name.push(format!(".{corner}"));
    if let Some(ext) = path.extension() {
        file_name.push(".");
        file_name.push(ext);
    }
    Some(path.with_file_name(file_name))
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
    ensure_not_cancelled(ctx)?;

    let frequencies = run
        .scattering
        .data
        .iter()
        .map(|matrix| matrix.frequency)
        .collect::<Vec<_>>();
    let scattering = scattering_cube(&run.scattering);
    if !ctx.quiet {
        println!(
            "Running {}-port S-parameter analysis: {} frequency points",
            run.ports.len(),
            frequencies.len()
        );
        if let Some(first) = scattering
            .first()
            .and_then(|row| row.first())
            .and_then(|series| series.first())
        {
            println!(
                "  @ {:e} Hz: |S_1_1|={:.4}",
                frequencies.first().copied().unwrap_or(0.0),
                first.norm()
            );
        }
    }

    let Some(resolved) = ctx.resolve_output("sp") else {
        return Ok(());
    };
    let analysis_id = resolved.analysis("sp")?;
    let output_path = &resolved.path;
    if touchstone_extension_matches(output_path, run.ports.len()) {
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
    } else {
        let signals = sparameter_export_signals(&run, &frequencies, &scattering);
        super::document::publish_analysis_result(
            ctx,
            output_path,
            analysis_id,
            super::document::complex_schema(&signals)?,
            || {
                rspice_core::execution::AnalysisResultDocument::from_s_parameters(
                    analysis_id,
                    &run.scattering,
                )
            },
            |path, format| {
                if matches!(format, crate::cli::OutputFormat::Hdf5) {
                    let mut hdf5 = crate::hdf5::Hdf5SimulationData::new();
                    hdf5.title = "S-Parameters".to_string();
                    hdf5.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                    let mut section = crate::hdf5::Hdf5AcSection::new(frequencies.clone());
                    for signal in &signals {
                        section.add_signal(
                            signal.display_name.clone(),
                            signal.real.clone(),
                            signal.imag.clone(),
                        );
                    }
                    hdf5.ac = Some(section);
                    crate::hdf5::write_hdf5(path, &hdf5)
                        .map_err(|err| super::shared::map_hdf5_output_error(path, err))
                } else {
                    super::export::complex_table(
                        "sp",
                        "S-Parameters",
                        frequencies.clone(),
                        &signals,
                    )
                    .write(path, format)
                }
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
            ctx.record_output(noise_path.clone());
            let builder =
                rspice_core::execution::AnalysisResultDocument::from_port_noise(analysis_id, noise)
                    .map_err(|error| super::document::document_error(ctx, analysis_id, error))?;
            let document = super::document::finish(ctx, analysis_id, builder)?;
            super::document::write_document(ctx, &noise_path, &document)?;
            if !ctx.quiet {
                println!("  Port noise exported to: {}", noise_path.display());
            }
        }
    }

    if !ctx.quiet {
        println!("  S-parameters exported to: {}", output_path.display());
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

fn sparameter_export_signals(
    run: &rspice_core::engine::SParameterRun,
    frequencies: &[f64],
    scattering: &[Vec<Vec<rspice_core::Complex64>>],
) -> Vec<crate::commands::run_signals::ComplexSignal> {
    use crate::commands::run_signals::{ComplexSignal, SignalKind};

    let count = run.ports.len();
    let mut signals = Vec::with_capacity(
        count * count + run.port_noise.as_ref().map_or(0, |_| count * count + 6),
    );
    let mut push = |name: String, values: &[rspice_core::Complex64], kind: SignalKind| {
        signals.push(ComplexSignal {
            display_name: name.clone(),
            raw_name: name,
            kind,
            real: values.iter().map(|value| value.re).collect(),
            imag: values.iter().map(|value| value.im).collect(),
        });
    };

    for (row, columns) in scattering.iter().enumerate() {
        for (column, series) in columns.iter().enumerate() {
            push(
                format!("S_{}_{}", row + 1, column + 1),
                series,
                SignalKind::Voltage,
            );
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
                    &series,
                    SignalKind::Scalar,
                );
            }
        }
        let constant = |value| vec![rspice_core::Complex64::new(value, 0.0); frequencies.len()];
        push(
            "noise_reference_temperature_K".to_string(),
            &constant(noise.reference_temperature_kelvin),
            SignalKind::Scalar,
        );
        push(
            "noise_normalization_4kT_J".to_string(),
            &constant(
                4.0 * rspice_core::constants::K_BOLTZMANN * noise.reference_temperature_kelvin,
            ),
            SignalKind::Scalar,
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
                &real_values(|parameter| parameter.noise_resistance),
                SignalKind::Scalar,
            );
            push(
                "noise_factor_linear".to_string(),
                &real_values(|parameter| parameter.noise_factor),
                SignalKind::Scalar,
            );
            push(
                "minimum_noise_factor_linear".to_string(),
                &real_values(|parameter| parameter.minimum_noise_factor),
                SignalKind::Scalar,
            );
            let optimum = parameters
                .iter()
                .map(|parameter| parameter.optimum_source_reflection)
                .collect::<Vec<_>>();
            push(
                "optimum_source_reflection".to_string(),
                &optimum,
                SignalKind::Scalar,
            );
        }
    }
    signals
}

fn touchstone_extension_matches(path: &std::path::Path, num_ports: usize) -> bool {
    path.extension().is_some_and(|ext| {
        ext.eq_ignore_ascii_case("snp") || ext.eq_ignore_ascii_case(format!("s{}p", num_ports))
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
/// Standard matched-termination wave method: for each drive port, a source
/// of 2 V AC behind Z0 excites the port (incident wave of 1 V) while the
/// other port is terminated in Z0. The port voltages then read off the
/// S-parameters directly — `Sjj = Vj − 1`, `Sij = Vi` — with no matrix
/// inversion and no floating-port hazard. The deck supplies the bias
/// network and sweep; its own sources must not carry AC specifications.
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

    let source = ctx
        .netlist
        .source_text
        .as_deref()
        .ok_or_else(|| CliError::InternalError {
            message: "netlist source unavailable for S-parameter excitation".to_string(),
        })?;
    let base = ctx
        .netlist
        .source_path
        .clone()
        .unwrap_or_else(|| ctx.args.input.clone());

    if !ctx.quiet {
        println!(
            "Running 2-port S-parameter extraction: Z0={}Ω, {} frequency points",
            z0,
            frequencies.len()
        );
    }

    // One AC sweep per driven port, with the excitation network appended.
    let drive = |drive_port: usize| -> Result<Vec<rspice_core::analysis::AcResult>, CliError> {
        let (dp, dm) = (&port_nodes[2 * drive_port], &port_nodes[2 * drive_port + 1]);
        let (lp, lm) = (
            &port_nodes[2 * (1 - drive_port)],
            &port_nodes[2 * (1 - drive_port) + 1],
        );
        let mut excited = String::with_capacity(source.len() + 128);
        for line in source.lines() {
            if line.trim().eq_ignore_ascii_case(".end") {
                excited.push_str(&format!(
                    "VSPDRV spdrv_node {dm} AC 2\nRSPSRC spdrv_node {dp} {z0}\nRSPLOAD {lp} {lm} {z0}\n"
                ));
            }
            excited.push_str(line);
            excited.push('\n');
        }
        let parse_options =
            super::parse_options_for_run(ctx.args, ctx.engine.config().resource_limits);
        let netlist = rspice_core::Netlist::parse_with_path_and_options_and_abort(
            &excited,
            &base,
            parse_options,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| match error {
            rspice_core::netlist::ParseWithAbortError::Aborted => {
                super::cancellation_cli_error(ctx.args.timeout)
            }
            rspice_core::netlist::ParseWithAbortError::Parse(error) => CliError::ParseError {
                message: format!("S-parameter excitation: {error}"),
                line: None,
                suggestion: None,
            },
        })?;
        ctx.engine
            .run_ac_with_abort(&netlist, &frequencies, &crate::abort::ProcessAbort)
            .map_err(|error| map_advanced_simulation_error(ctx, "S-Parameters", error))
    };

    let drive1 = drive(0)?;
    let drive2 = drive(1)?;
    ensure_not_cancelled(ctx)?;

    // Differential port voltage at one sweep point.
    let ground_policy = ctx.netlist.ground_policy();
    let port_v = |result: &rspice_core::analysis::AcResult,
                  plus: &str,
                  minus: &str|
     -> Result<rspice_core::Complex64, CliError> {
        let lookup = |node: &str| -> Result<rspice_core::Complex64, CliError> {
            if ground_policy.is_ground(node) {
                return Ok(rspice_core::Complex64::new(0.0, 0.0));
            }
            result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(node))
                .and_then(|index| result.voltages.get(index).copied())
                .ok_or_else(|| CliError::SimulationError {
                    message: format!("S-parameter port node '{node}' not found in the circuit"),
                    analysis: Some("S-Parameters".to_string()),
                })
        };
        Ok(lookup(plus)? - lookup(minus)?)
    };

    // With Vs = 2 V behind Z0, the incident wave at the driven port is 1 V:
    // Sjj = Vj - 1, Sij = Vi.
    let one = rspice_core::Complex64::new(1.0, 0.0);
    let mut s11 = Vec::with_capacity(frequencies.len());
    let mut s21 = Vec::with_capacity(frequencies.len());
    let mut s12 = Vec::with_capacity(frequencies.len());
    let mut s22 = Vec::with_capacity(frequencies.len());
    for (point1, point2) in drive1.iter().zip(&drive2) {
        ensure_not_cancelled(ctx)?;
        s11.push(port_v(point1, &port_nodes[0], &port_nodes[1])? - one);
        s21.push(port_v(point1, &port_nodes[2], &port_nodes[3])?);
        s22.push(port_v(point2, &port_nodes[2], &port_nodes[3])? - one);
        s12.push(port_v(point2, &port_nodes[0], &port_nodes[1])?);
    }

    if !ctx.quiet
        && let (Some(first_s11), Some(first_s21)) = (s11.first(), s21.first())
    {
        println!(
            "  @ {:e} Hz: |S11|={:.4} |S21|={:.4}",
            frequencies.first().copied().unwrap_or(0.0),
            first_s11.norm(),
            first_s21.norm()
        );
    }

    ensure_not_cancelled(ctx)?;
    if let Some(resolved) = ctx.resolve_output("sparam") {
        let analysis_id = resolved.analysis("sparam")?;
        let output_path = &resolved.path;
        if output_path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("s2p") || ext.eq_ignore_ascii_case("snp"))
        {
            write_touchstone_2port(output_path, z0, &frequencies, [&s11, &s21, &s12, &s22])?;
        } else {
            let signal = |name: &str, values: &[rspice_core::Complex64]| {
                crate::commands::run_signals::ComplexSignal {
                    display_name: name.to_string(),
                    raw_name: name.to_string(),
                    kind: crate::commands::run_signals::SignalKind::Voltage,
                    real: values.iter().map(|c| c.re).collect(),
                    imag: values.iter().map(|c| c.im).collect(),
                }
            };
            let signals = vec![
                signal("S11", &s11),
                signal("S21", &s21),
                signal("S12", &s12),
                signal("S22", &s22),
            ];
            let core_result = two_port_core_result(z0, &frequencies, [&s11, &s21, &s12, &s22]);
            super::document::publish_analysis_result(
                ctx,
                output_path,
                analysis_id,
                super::document::complex_schema(&signals)?,
                || {
                    rspice_core::execution::AnalysisResultDocument::from_s_parameters(
                        analysis_id,
                        &core_result,
                    )
                },
                |path, format| {
                    if matches!(format, crate::cli::OutputFormat::Hdf5) {
                        let mut data = crate::hdf5::Hdf5SimulationData::new();
                        data.title = "S-Parameters".to_string();
                        data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                        let mut section = crate::hdf5::Hdf5AcSection::new(frequencies.clone());
                        for s in &signals {
                            section.add_signal(
                                s.display_name.clone(),
                                s.real.clone(),
                                s.imag.clone(),
                            );
                        }
                        data.ac = Some(section);
                        crate::hdf5::write_hdf5(path, &data)
                            .map_err(|err| super::shared::map_hdf5_output_error(path, err))
                    } else {
                        super::export::complex_table(
                            "sparam",
                            "S-Parameters",
                            frequencies.clone(),
                            &signals,
                        )
                        .write(path, format)
                    }
                },
            )?;
        }
        if !ctx.quiet {
            println!("  S-parameters exported to: {}", output_path.display());
        }
    }

    Ok(())
}

/// Re-assemble the `--sparam` two-port sweep into the core result type the
/// shared document is built from.
fn two_port_core_result(
    z0: f64,
    frequencies: &[f64],
    s: [&[rspice_core::Complex64]; 4],
) -> rspice_core::analysis::s_param::SParameterResult {
    use rspice_core::analysis::s_param::{Port, SMatrix, SParameterResult};

    // `--sparam` drives the deck's two named ports; their reference planes are
    // not carried through this path, so the ports are identified by number.
    let ports = (1..=2)
        .map(|number| Port {
            number,
            node_pos: format!("port{number}"),
            node_neg: "0".to_string(),
            z0,
        })
        .collect::<Vec<_>>();
    let mut result = SParameterResult::new(z0, ports);
    // `s` is ordered S11, S21, S12, S22, matching Touchstone two-port order.
    let placement = [(0, 0), (1, 0), (0, 1), (1, 1)];
    for (index, frequency) in frequencies.iter().enumerate() {
        let mut matrix = SMatrix::new(*frequency, 2);
        for (series, (row, column)) in s.iter().zip(placement) {
            if let Some(value) = series.get(index) {
                matrix.set(row, column, *value);
            }
        }
        result.data.push(matrix);
    }
    result
}

/// Touchstone v1 two-port file (`# HZ S RI R <z0>`, S11 S21 S12 S22 order).
fn write_touchstone_2port(
    path: &std::path::Path,
    z0: f64,
    frequencies: &[f64],
    s: [&[rspice_core::Complex64]; 4],
) -> Result<(), CliError> {
    publish::artifact(path, |file| {
        writeln!(file, "! 2-port S-parameters").map_err(|e| CliError::output_error(path, e))?;
        writeln!(file, "# HZ S RI R {z0}").map_err(|e| CliError::output_error(path, e))?;
        let [s11, s21, s12, s22] = s;
        for (index, freq) in frequencies.iter().enumerate() {
            let entry = |values: &[rspice_core::Complex64]| {
                values
                    .get(index)
                    .copied()
                    .unwrap_or_else(|| rspice_core::Complex64::new(0.0, 0.0))
            };
            let (a, b, c, d) = (entry(s11), entry(s21), entry(s12), entry(s22));
            writeln!(
                file,
                "{freq:.9e} {:.9e} {:.9e} {:.9e} {:.9e} {:.9e} {:.9e} {:.9e} {:.9e}",
                a.re, a.im, b.re, b.im, c.re, c.im, d.re, d.im
            )
            .map_err(|e| CliError::output_error(path, e))?;
        }
        Ok(())
    })
    .map_err(|error| map_atomic_output_error(path, error))
}
