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

            for (i, (value, result)) in sweep_results.iter().enumerate() {
                if ctx.verbose && !ctx.quiet {
                    println!(
                        "  Step {}/{}: {} = {:.4e}",
                        i + 1,
                        values.len(),
                        target_desc,
                        value
                    );
                    println!("    V(1) = {:.6} V", result.voltage(1));
                }
            }

            if !ctx.quiet {
                println!(
                    ".STEP sweep complete: {} converged / {} requested",
                    sweep_results.len(),
                    values.len()
                );
            }
            Ok(())
        }
        StepTarget::Temp => run_temp(ctx, &values),
    }
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
        let pb = indicatif::ProgressBar::new(num_runs as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message("Monte Carlo");
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

            if !ctx.quiet {
                println!("✓ Monte Carlo complete: {} runs", result.num_runs);
                println!(
                    "  Convergence: {}/{} runs succeeded",
                    result.num_runs, num_runs
                );

                if ctx.verbose && !result.variables.is_empty() {
                    println!("\n  Statistical Summary:");
                    for (name, stats) in &result.variables {
                        println!("    {}:", name);
                        println!("      Mean:   {:.6}", stats.mean);
                        println!("      Std:    {:.6}", stats.std_dev);
                        println!("      Min:    {:.6}", stats.min);
                        println!("      Max:    {:.6}", stats.max);
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            pb.finish_and_clear();
            Err(CliError::simulation_error_in(e.to_string(), "Monte Carlo"))
        }
    }
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
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "PSS")),
    }
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
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "HB")),
    }
}

pub(super) fn run_corner_sweep(ctx: &RunContext<'_>, corners_str: &str) -> Result<(), CliError> {
    use rspice_core::analysis::advanced::ProcessCorner;

    let corner_strs: Vec<&str> = corners_str.split(',').map(|s| s.trim()).collect();
    let corners: Vec<(ProcessCorner, String)> = corner_strs
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let corner = match s.to_lowercase().as_str() {
                "tt" | "typ" | "typical" => ProcessCorner::TT,
                "ss" | "slow" => ProcessCorner::SS,
                "ff" | "fast" => ProcessCorner::FF,
                "sf" | "snfp" => ProcessCorner::SF,
                "fs" | "fnsp" => ProcessCorner::FS,
                _ => ProcessCorner::Custom(i as u8),
            };
            (corner, s.to_string())
        })
        .collect();

    if !ctx.quiet {
        println!("Running PVT corner sweep: {} corners", corners.len());
        if ctx.verbose {
            println!("  Process corners:");
            for (corner, name) in &corners {
                println!(
                    "    {}: NMOS={:.2}x, PMOS={:.2}x",
                    name,
                    corner.nmos_factor(),
                    corner.pmos_factor()
                );
            }
        }
    }

    let mut results: Vec<(String, bool)> = Vec::new();

    for (i, (corner, name)) in corners.iter().enumerate() {
        if !ctx.quiet {
            println!("\n[{}/{}] Corner: {}", i + 1, corners.len(), name);
        }

        if ctx.verbose && !ctx.quiet {
            println!(
                "  Process scaling: NMOS={:.2}x, PMOS={:.2}x",
                corner.nmos_factor(),
                corner.pmos_factor()
            );
        }

        let mut corner_passed = true;
        for analysis in &ctx.netlist.analyses {
            if let Err(e) = ctx.run_analysis(analysis) {
                if !ctx.quiet {
                    eprintln!("  Analysis failed: {}", e);
                }
                corner_passed = false;
            }
        }

        if ctx.netlist.analyses.is_empty()
            && let Err(e) = run_dc_op(ctx)
        {
            if !ctx.quiet {
                eprintln!("  DC OP failed: {}", e);
            }
            corner_passed = false;
        }

        results.push((name.clone(), corner_passed));
    }

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

    Ok(())
}
