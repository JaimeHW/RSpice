use super::RunContext;
use super::basic::{run_dc_op, run_temp};
use super::shared::generate_step_values;
use crate::cli::CliError;

pub(super) fn run_step(
    ctx: &RunContext<'_>,
    step_cmd: &rspice_core::netlist::StepCommand,
) -> Result<(), CliError> {
    use rspice_core::netlist::StepTarget;

    let values = generate_step_values(&step_cmd.sweep)?;
    if values.is_empty() {
        return Err(CliError::SimulationError {
            message: ".STEP produced no sweep values".to_string(),
            analysis: Some("Step".to_string()),
        });
    }

    match step_cmd.target {
        StepTarget::Param | StepTarget::Device | StepTarget::Model => {
            let target_desc = match step_cmd.target {
                StepTarget::Param => format!("PARAM {}", step_cmd.name),
                StepTarget::Device => {
                    if let Some(param) = &step_cmd.param_name {
                        format!("DEVICE {}.{}", step_cmd.name, param)
                    } else {
                        format!("DEVICE {}", step_cmd.name)
                    }
                }
                StepTarget::Model => {
                    if let Some(param) = &step_cmd.param_name {
                        format!("MODEL {}.{}", step_cmd.name, param)
                    } else {
                        format!("MODEL {}", step_cmd.name)
                    }
                }
                StepTarget::Temp => unreachable!("handled separately"),
            };

            if !ctx.quiet {
                println!(
                    "Running .STEP sweep on {}: {} values ({:.3e} to {:.3e})...",
                    target_desc,
                    values.len(),
                    values.first().unwrap_or(&0.0),
                    values.last().unwrap_or(&0.0)
                );
            }

            let sweep_results = ctx
                .engine
                .run_step_command(ctx.netlist, step_cmd, &values)
                .map_err(|e| CliError::simulation_error_in(e.to_string(), "Step"))?;

            for (_, point) in &sweep_results {
                super::shared::ensure_finite_series(
                    ctx.args.allow_nonfinite,
                    "Step",
                    (1..point.node_voltages.len()).map(|node| {
                        let name = point
                            .node_names
                            .get(node)
                            .map(|n| n.as_str())
                            .unwrap_or("node");
                        (name, std::slice::from_ref(&point.node_voltages[node]))
                    }),
                )?;
            }

            for (i, (value, result)) in sweep_results.iter().enumerate() {
                if ctx.verbose && !ctx.quiet {
                    println!(
                        "  Step {}/{}: {} = {:.4e}",
                        i + 1,
                        values.len(),
                        target_desc,
                        value
                    );
                    if result.node_voltages.len() > 1 {
                        let name = result
                            .node_names
                            .get(1)
                            .cloned()
                            .unwrap_or_else(|| "1".to_string());
                        println!("    V({}) = {:.6} V", name, result.voltage(1));
                    }
                }
            }

            if !ctx.quiet {
                println!(
                    ".STEP sweep complete: {} converged / {} requested",
                    sweep_results.len(),
                    values.len()
                );
            }

            export_step_sweep(ctx, &step_cmd.name, &sweep_results)?;
            Ok(())
        }
        StepTarget::Temp => run_temp(ctx, &values),
    }
}

/// Write the .STEP sweep table: one row per step value, one column per
/// node voltage — the same shape as a DC sweep with the stepped quantity
/// as the abscissa.
fn export_step_sweep(
    ctx: &RunContext<'_>,
    step_name: &str,
    sweep_results: &[(f64, rspice_core::solver::SimulationResult)],
) -> Result<(), CliError> {
    let Some(ref output_path) = ctx.output_path_for("step") else {
        return Ok(());
    };

    let sweep_vals: Vec<f64> = sweep_results.iter().map(|(v, _)| *v).collect();
    let signals = crate::commands::run_signals::apply_save_set(
        crate::commands::run_signals::dc_sweep_voltage_signals(sweep_results),
        &ctx.netlist.saves,
    );

    match ctx.format {
        crate::cli::OutputFormat::Hdf5 => {
            let mut data = crate::hdf5::Hdf5SimulationData::new();
            data.title = "Step Sweep".to_string();

            let mut sweep =
                crate::hdf5::Hdf5WaveformSection::new(step_name, sweep_vals.clone());
            for signal in &signals {
                sweep.add_signal(signal.display_name.clone(), signal.values.clone());
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
            rspice_core::analysis::export_dc_sweep(
                output_path,
                &sweep_vals,
                step_name,
                &node_names,
                &node_waveforms,
                match ctx.format {
                    crate::cli::OutputFormat::RawAscii => rspice_core::analysis::RawFormat::Ascii,
                    _ => rspice_core::analysis::RawFormat::Binary,
                },
            )
            .map_err(|e| CliError::OutputError {
                path: output_path.clone(),
                source: e,
            })?;
        }
        crate::cli::OutputFormat::Csv
        | crate::cli::OutputFormat::Tsv
        | crate::cli::OutputFormat::Json => {
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
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        pb.set_message(format!("Monte Carlo: {} runs (seed {})", num_runs, seed));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    };

    match ctx.engine.run_monte_carlo_with_options(
        ctx.netlist,
        num_runs,
        seed,
        distribution,
        parameter_filter,
    ) {
        Ok(result) => {
            pb.finish_and_clear();

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
                println!("✓ Monte Carlo complete: {} runs (seed={})", result.num_runs, seed);
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
            Err(CliError::simulation_error_in(e.to_string(), "Monte Carlo"))
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
    let Some(ref output_path) = ctx.output_path_for("mc") else {
        return Ok(());
    };

    let num_samples = variables
        .iter()
        .map(|stats| stats.samples.len())
        .max()
        .unwrap_or(0);
    let runs: Vec<f64> = (1..=num_samples).map(|i| i as f64).collect();

    if matches!(ctx.format, crate::cli::OutputFormat::Json) {
        use std::io::Write;
        let json = serde_json::json!({
            "analysis": "monte_carlo",
            "runs": result.num_runs,
            "failures": result.num_failures,
            "seed": seed,
            "variables": variables.iter().map(|stats| {
                serde_json::json!({
                    "name": stats.name,
                    "mean": stats.mean,
                    "std_dev": stats.std_dev,
                    "min": stats.min,
                    "max": stats.max,
                    "samples": stats.samples,
                })
            }).collect::<Vec<_>>(),
        });
        let mut file = std::fs::File::create(output_path).map_err(|e| {
            CliError::output_error(output_path, e)
        })?;
        serde_json::to_writer_pretty(&mut file, &json)
            .map_err(|e| CliError::output_json_error(output_path, e))?;
        file.write_all(b"\n")
            .map_err(|e| CliError::output_error(output_path, e))?;
    } else {
        let signals: Vec<crate::commands::run_signals::ScalarSignal> = variables
            .iter()
            .map(|stats| crate::commands::run_signals::ScalarSignal {
                display_name: stats.name.clone(),
                raw_name: stats.name.clone(),
                kind: crate::commands::run_signals::SignalKind::Voltage,
                values: stats.samples.clone(),
            })
            .collect();

        match ctx.format {
            crate::cli::OutputFormat::Hdf5 => {
                let mut data = crate::hdf5::Hdf5SimulationData::new();
                data.title = "Monte Carlo Samples".to_string();
                let mut sweep = crate::hdf5::Hdf5WaveformSection::new("run", runs.clone());
                for signal in &signals {
                    sweep.add_signal(signal.display_name.clone(), signal.values.clone());
                }
                data.dc_sweep = Some(sweep);
                crate::hdf5::write_hdf5(output_path, &data)
                    .map_err(|err| super::shared::map_hdf5_output_error(output_path, err))?;
            }
            _ => {
                super::export::scalar_table(
                    "monte_carlo",
                    "Monte Carlo Samples",
                    "run",
                    "index",
                    runs,
                    &signals,
                )
                .write(output_path, ctx.format)?;
            }
        }
    }

    if !ctx.quiet {
        println!("  Monte Carlo samples exported to: {}", output_path.display());
    }
    Ok(())
}

pub(super) fn run_monte_carlo_from_command(
    ctx: &RunContext<'_>,
    mc_cmd: &rspice_core::netlist::MonteCarloCommand,
) -> Result<(), CliError> {
    let seed = ctx.args.seed.or(mc_cmd.seed).unwrap_or(1);
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

pub(super) fn run_pss(
    ctx: &RunContext<'_>,
    freq: f64,
    harmonics: usize,
    tstab: Option<f64>,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running PSS analysis: f₀ = {:.3e} Hz, {} harmonics",
            freq, harmonics
        );
    }

    let mut config = rspice_core::analysis::PssConfig::new(freq);
    config.num_harmonics = harmonics;
    if let Some(t) = tstab {
        config.tstab = t;
    }

    match ctx.engine.run_pss(ctx.netlist, config) {
        Ok(pss_result) => {
            if !ctx.quiet {
                println!("✓ PSS converged in {} iterations", pss_result.iterations);
                println!("  Period: {:.6e} s", pss_result.period);
                println!("  Nodes: {}", pss_result.result.num_nodes());

                if ctx.verbose && pss_result.result.num_nodes() > 0 {
                    println!("\n  Harmonic content (node 1):");
                    let harm_data = pss_result.result.harmonics(1, 5);
                    for h in &harm_data {
                        println!(
                            "    H{}: mag={:.6e}, phase={:.2}° (f={:.3e} Hz)",
                            h.harmonic_number, h.magnitude, h.phase, h.frequency
                        );
                    }
                }
            }

            export_pss(ctx, &pss_result.result)?;
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "PSS")),
    }
}

/// Write one period of the converged steady-state waveforms (time domain),
/// the same table shape as a transient export.
fn export_pss(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::PssResult,
) -> Result<(), CliError> {
    let Some(ref output_path) = ctx.output_path_for("pss") else {
        return Ok(());
    };

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
    let signals =
        crate::commands::run_signals::apply_save_set(signals, &ctx.netlist.saves);

    match ctx.format {
        crate::cli::OutputFormat::Hdf5 => {
            let mut data = crate::hdf5::Hdf5SimulationData::new();
            data.title = "Periodic Steady State".to_string();
            let mut section =
                crate::hdf5::Hdf5WaveformSection::new("time", result.time.clone());
            for signal in &signals {
                section.add_signal(signal.display_name.clone(), signal.values.clone());
            }
            data.transient = Some(section);
            crate::hdf5::write_hdf5(output_path, &data)
                .map_err(|err| super::shared::map_hdf5_output_error(output_path, err))?;
        }
        crate::cli::OutputFormat::Raw | crate::cli::OutputFormat::RawAscii => {
            let node_names: Vec<String> = signals
                .iter()
                .map(|signal| signal.raw_name.clone())
                .collect();
            let waveforms: Vec<Vec<f64>> =
                signals.iter().map(|signal| signal.values.clone()).collect();
            rspice_core::analysis::export_transient(
                output_path,
                &result.time,
                &node_names,
                &waveforms,
                match ctx.format {
                    crate::cli::OutputFormat::RawAscii => rspice_core::analysis::RawFormat::Ascii,
                    _ => rspice_core::analysis::RawFormat::Binary,
                },
            )
            .map_err(|e| CliError::OutputError {
                path: output_path.clone(),
                source: e,
            })?;
        }
        _ => {
            super::export::scalar_table(
                "pss",
                "Periodic Steady State",
                "time",
                "time",
                result.time.clone(),
                &signals,
            )
            .write(output_path, ctx.format)?;
        }
    }

    if !ctx.quiet {
        println!("  PSS waveforms exported to: {}", output_path.display());
    }
    Ok(())
}

pub(super) fn run_hb(ctx: &RunContext<'_>, freq: f64, harmonics: usize) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running HB analysis: f₀ = {:.3e} Hz, {} harmonics",
            freq, harmonics
        );
    }

    let config = rspice_core::analysis::HbConfig::new(freq).with_harmonics(harmonics);

    match ctx.engine.run_hb(ctx.netlist, config) {
        Ok(hb_result) => {
            if !ctx.quiet {
                println!("✓ HB converged");
                println!("  Nodes: {}", hb_result.result.num_nodes());
                println!("  Harmonics: {}", hb_result.result.num_harmonics);

                if ctx.verbose && !hb_result.result.spectral_voltages.is_empty() {
                    println!("\n  Spectral content (first node):");
                    let sv = &hb_result.result.spectral_voltages[0];
                    for k in 0..5.min(harmonics) {
                        println!(
                            "    H{}: mag={:.6e}, phase={:.2}°",
                            k,
                            sv.magnitude(k),
                            sv.phase(k).to_degrees()
                        );
                    }
                }
            }

            export_hb(ctx, freq, &hb_result.result)?;
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "HB")),
    }
}

/// Write the harmonic-balance spectrum: harmonic frequencies as the scale,
/// one complex column per node.
fn export_hb(
    ctx: &RunContext<'_>,
    fundamental: f64,
    result: &rspice_core::analysis::HbResult,
) -> Result<(), CliError> {
    let Some(ref output_path) = ctx.output_path_for("hb") else {
        return Ok(());
    };

    let num_coeffs = result
        .spectral_voltages
        .iter()
        .map(|sv| sv.coefficients.len())
        .max()
        .unwrap_or(0);
    let frequencies: Vec<f64> = result
        .spectral_voltages
        .first()
        .filter(|sv| sv.frequencies.len() == num_coeffs)
        .map(|sv| sv.frequencies.clone())
        .unwrap_or_else(|| (0..num_coeffs).map(|k| fundamental * k as f64).collect());

    let signals: Vec<crate::commands::run_signals::ComplexSignal> = result
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
    let signals =
        crate::commands::run_signals::apply_save_set_complex(signals, &ctx.netlist.saves);

    if matches!(ctx.format, crate::cli::OutputFormat::Hdf5) {
        let mut data = crate::hdf5::Hdf5SimulationData::new();
        data.title = "Harmonic Balance Spectrum".to_string();
        let mut section = crate::hdf5::Hdf5AcSection::new(frequencies);
        for signal in &signals {
            section.add_signal(
                signal.display_name.clone(),
                signal.real.clone(),
                signal.imag.clone(),
            );
        }
        data.ac = Some(section);
        crate::hdf5::write_hdf5(output_path, &data)
            .map_err(|err| super::shared::map_hdf5_output_error(output_path, err))?;
    } else {
        super::export::complex_table("hb", "Harmonic Balance Spectrum", frequencies, &signals)
            .write(output_path, ctx.format)?;
    }

    if !ctx.quiet {
        println!("  HB spectrum exported to: {}", output_path.display());
    }
    Ok(())
}

pub(super) fn run_corner_sweep(ctx: &RunContext<'_>, corners_str: &str) -> Result<(), CliError> {
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

    let jobs = ctx.args.jobs.unwrap_or(1).max(1);
    let results: Vec<(String, bool)> = if jobs > 1 && corners.len() > 1 {
        run_corners_parallel(ctx, &corners, corner_lib.as_deref(), jobs)?
    } else {
        let mut results = Vec::with_capacity(corners.len());
        for (i, name) in corners.iter().enumerate() {
            if !ctx.quiet {
                println!("\n[{}/{}] Corner: {}", i + 1, corners.len(), name);
            }

            let corner_passed = match corner_lib.as_deref() {
                Some(lib) => match run_corner_with_lib(ctx, lib, name) {
                    Ok(passed) => passed,
                    Err(e) => {
                        if !ctx.quiet {
                            eprintln!("  Corner '{}' failed: {}", name, e);
                        }
                        false
                    }
                },
                None => run_corner_nominal(ctx),
            };

            results.push((name.clone(), corner_passed));
        }
        results
    };

    if !ctx.quiet {
        println!("\n┌─────────────────────────────────────┐");
        println!("│        Corner Sweep Summary         │");
        println!("├─────────────────────────────────────┤");
        for (name, passed) in &results {
            let status = if *passed { "✓ PASS" } else { "✗ FAIL" };
            println!("│  {:6}  {:>24}  │", name, status);
        }
        println!("└─────────────────────────────────────┘");

        let passed_count = results.iter().filter(|(_, passed)| *passed).count();
        println!(
            "\n✓ Corner sweep complete: {}/{} corners passed",
            passed_count,
            corners.len()
        );
    }

    let failed: Vec<&str> = results
        .iter()
        .filter(|(_, passed)| !passed)
        .map(|(name, _)| name.as_str())
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
    passed: bool,
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

    let corner_netlist = match rspice_core::Netlist::parse_with_path(&corner_source, &setup.base)
    {
        Ok(netlist) => netlist,
        Err(e) => {
            return CornerOutcome {
                passed: false,
                error: Some(format!("corner '{}': {}", corner, e)),
                measurements: Vec::new(),
                outputs: Vec::new(),
            };
        }
    };

    let corner_engine = rspice_core::Engine::new(setup.config.clone());
    let corner_ctx = RunContext {
        engine: &corner_engine,
        netlist: &corner_netlist,
        args: setup.args,
        format: setup.format,
        output: corner_output_path(setup.output.as_deref(), corner),
        show_progress: false,
        compress: setup.compress,
        compress_tol: setup.compress_tol,
        multi_analysis: corner_netlist.analyses.len() > 1,
        verbose: false,
        quiet: true,
        measurements: std::cell::RefCell::new(Vec::new()),
        evaluated_meas: std::cell::RefCell::new(std::collections::HashSet::new()),
        outputs: std::cell::RefCell::new(Vec::new()),
    };

    let mut passed = true;
    let mut error: Option<String> = None;
    if corner_netlist.analyses.is_empty() {
        if let Err(e) = run_dc_op(&corner_ctx) {
            passed = false;
            error.get_or_insert(e.to_string());
        }
    } else {
        for analysis in &corner_netlist.analyses {
            if let Err(e) = corner_ctx.run_analysis(analysis) {
                passed = false;
                error.get_or_insert(e.to_string());
            }
        }
    }

    corner_ctx.record_unevaluated_measurements();
    let measurements = corner_ctx
        .measurements
        .into_inner()
        .into_iter()
        .map(|mut m| {
            m.name = format!("{}:{}", corner, m.name);
            m
        })
        .collect();

    CornerOutcome {
        passed,
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
) -> Result<Vec<(String, bool)>, CliError> {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let setup = corner_setup(ctx)?;
    let workers = jobs.min(corners.len());
    if !ctx.quiet {
        println!(
            "  {} corners across {} workers (per-corner solver output suppressed)",
            corners.len(),
            workers
        );
    }

    let next = AtomicUsize::new(0);
    let slots: Vec<Mutex<Option<CornerOutcome>>> =
        (0..corners.len()).map(|_| Mutex::new(None)).collect();

    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let index = next.fetch_add(1, Ordering::SeqCst);
                    if index >= corners.len() {
                        break;
                    }
                    let outcome = run_corner_job(&setup, lib, &corners[index]);
                    *slots[index].lock().expect("corner slot") = Some(outcome);
                }
            });
        }
    });

    let mut results = Vec::with_capacity(corners.len());
    for (name, slot) in corners.iter().zip(slots) {
        let outcome = slot
            .into_inner()
            .expect("corner slot lock")
            .expect("worker filled its slot");
        if let Some(ref err) = outcome.error {
            if !ctx.quiet {
                eprintln!("  Corner '{}' failed: {}", name, err);
            }
        }
        ctx.measurements.borrow_mut().extend(outcome.measurements);
        ctx.outputs.borrow_mut().extend(outcome.outputs);
        results.push((name.clone(), outcome.passed));
    }
    Ok(results)
}

/// Re-elaborate the deck with the corner's `.lib` section applied and run
/// every analysis against the corner models.
fn run_corner_with_lib(
    ctx: &RunContext<'_>,
    lib: &std::path::Path,
    corner: &str,
) -> Result<bool, CliError> {
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
    let corner_netlist = rspice_core::Netlist::parse_with_path(&corner_source, &base).map_err(
        |e| CliError::ParseError {
            message: format!("corner '{}': {}", corner, e),
            line: None,
            suggestion: None,
        },
    )?;

    let corner_engine = rspice_core::Engine::new(ctx.engine.config().clone());
    let corner_ctx = RunContext {
        engine: &corner_engine,
        netlist: &corner_netlist,
        args: ctx.args,
        format: ctx.format,
        output: corner_output_path(ctx.output.as_deref(), corner),
        show_progress: ctx.show_progress,
        compress: ctx.compress,
        compress_tol: ctx.compress_tol,
        multi_analysis: corner_netlist.analyses.len() > 1,
        verbose: ctx.verbose,
        quiet: ctx.quiet,
        measurements: std::cell::RefCell::new(Vec::new()),
        evaluated_meas: std::cell::RefCell::new(std::collections::HashSet::new()),
        outputs: std::cell::RefCell::new(Vec::new()),
    };

    let mut passed = true;
    if corner_netlist.analyses.is_empty() {
        if let Err(e) = run_dc_op(&corner_ctx) {
            if !ctx.quiet {
                eprintln!("  DC OP failed: {}", e);
            }
            passed = false;
        }
    } else {
        for analysis in &corner_netlist.analyses {
            if let Err(e) = corner_ctx.run_analysis(analysis) {
                if !ctx.quiet {
                    eprintln!("  Analysis failed: {}", e);
                }
                passed = false;
            }
        }
    }

    // Surface this corner's measurements in CI reports under tagged names.
    corner_ctx.record_unevaluated_measurements();
    let corner_measurements = corner_ctx.measurements.into_inner();
    ctx.measurements
        .borrow_mut()
        .extend(corner_measurements.into_iter().map(|mut m| {
            m.name = format!("{}:{}", corner, m.name);
            m
        }));
    ctx.outputs
        .borrow_mut()
        .extend(corner_ctx.outputs.into_inner());

    Ok(passed)
}

/// Run the deck's analyses unchanged (no corner library available).
fn run_corner_nominal(ctx: &RunContext<'_>) -> bool {
    let mut passed = true;
    if ctx.netlist.analyses.is_empty() {
        if let Err(e) = run_dc_op(ctx) {
            if !ctx.quiet {
                eprintln!("  DC OP failed: {}", e);
            }
            passed = false;
        }
    } else {
        for analysis in &ctx.netlist.analyses {
            if let Err(e) = ctx.run_analysis(analysis) {
                if !ctx.quiet {
                    eprintln!("  Analysis failed: {}", e);
                }
                passed = false;
            }
        }
    }
    passed
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

/// Two-port S-parameter extraction over the deck's `.AC` sweep.
///
/// Standard matched-termination wave method: for each drive port, a source
/// of 2 V AC behind Z0 excites the port (incident wave of 1 V) while the
/// other port is terminated in Z0. The port voltages then read off the
/// S-parameters directly — `Sjj = Vj − 1`, `Sij = Vi` — with no matrix
/// inversion and no floating-port hazard. The deck supplies the bias
/// network and sweep; its own sources must not carry AC specifications.
pub(super) fn run_sparam(
    ctx: &RunContext<'_>,
    ports_spec: &str,
    z0: f64,
) -> Result<(), CliError> {
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
        super::shared::generate_frequency_sweep(variation, points, start_freq, stop_freq);

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
        let (dp, dm) = (
            &port_nodes[2 * drive_port],
            &port_nodes[2 * drive_port + 1],
        );
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
        let netlist = rspice_core::Netlist::parse_with_path(&excited, &base).map_err(|e| {
            CliError::ParseError {
                message: format!("S-parameter excitation: {e}"),
                line: None,
                suggestion: None,
            }
        })?;
        ctx.engine
            .run_ac(&netlist, &frequencies)
            .map_err(|e| CliError::simulation_error_in(e.to_string(), "S-Parameters"))
    };

    let drive1 = drive(0)?;
    let drive2 = drive(1)?;

    // Differential port voltage at one sweep point.
    let port_v = |result: &rspice_core::analysis::AcResult,
                  plus: &str,
                  minus: &str|
     -> Result<rspice_core::Complex64, CliError> {
        let lookup = |node: &str| -> Result<rspice_core::Complex64, CliError> {
            if node == "0" || node.eq_ignore_ascii_case("gnd") {
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
        s11.push(port_v(point1, &port_nodes[0], &port_nodes[1])? - one);
        s21.push(port_v(point1, &port_nodes[2], &port_nodes[3])?);
        s22.push(port_v(point2, &port_nodes[2], &port_nodes[3])? - one);
        s12.push(port_v(point2, &port_nodes[0], &port_nodes[1])?);
    }

    if !ctx.quiet {
        if let (Some(first_s11), Some(first_s21)) = (s11.first(), s21.first()) {
            println!(
                "  @ {:e} Hz: |S11|={:.4} |S21|={:.4}",
                frequencies.first().copied().unwrap_or(0.0),
                first_s11.norm(),
                first_s21.norm()
            );
        }
    }

    if let Some(ref output_path) = ctx.output_path_for("sparam") {
        if output_path.extension().is_some_and(|ext| {
            ext.eq_ignore_ascii_case("s2p") || ext.eq_ignore_ascii_case("snp")
        }) {
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
            if matches!(ctx.format, crate::cli::OutputFormat::Hdf5) {
                let mut data = crate::hdf5::Hdf5SimulationData::new();
                data.title = "S-Parameters".to_string();
                let mut section = crate::hdf5::Hdf5AcSection::new(frequencies.clone());
                for s in &signals {
                    section.add_signal(s.display_name.clone(), s.real.clone(), s.imag.clone());
                }
                data.ac = Some(section);
                crate::hdf5::write_hdf5(output_path, &data)
                    .map_err(|err| super::shared::map_hdf5_output_error(output_path, err))?;
            } else {
                super::export::complex_table(
                    "sparam",
                    "S-Parameters",
                    frequencies.clone(),
                    &signals,
                )
                .write(output_path, ctx.format)?;
            }
        }
        if !ctx.quiet {
            println!("  S-parameters exported to: {}", output_path.display());
        }
    }

    Ok(())
}

/// Touchstone v1 two-port file (`# HZ S RI R <z0>`, S11 S21 S12 S22 order).
fn write_touchstone_2port(
    path: &std::path::Path,
    z0: f64,
    frequencies: &[f64],
    s: [&[rspice_core::Complex64]; 4],
) -> Result<(), CliError> {
    use std::io::Write;
    let mut file = std::fs::File::create(path).map_err(|e| CliError::output_error(path, e))?;
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
}
