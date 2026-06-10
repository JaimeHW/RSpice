use super::RunContext;
use super::shared::{NodeResolver, generate_frequency_sweep, map_hdf5_output_error};
use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::{ac_signals, voltage_display_name};
use crate::hdf5::{Hdf5AcSection, Hdf5SimulationData, Hdf5WaveformSection, write_hdf5};

/// Run `.TF`: DC small-signal transfer function, input impedance, and
/// output impedance, reported in ngspice's format.
pub(super) fn run_tf_from_command(
    ctx: &RunContext<'_>,
    output_node: &str,
    reference_node: Option<&str>,
    output_is_current: bool,
    input_source: &str,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!("Running DC transfer function analysis...");
    }

    let result = ctx
        .engine
        .run_transfer_function(
            ctx.netlist,
            output_node,
            reference_node,
            output_is_current,
            input_source,
        )
        .map_err(|e| CliError::simulation_error_in(e.to_string(), "Transfer Function"))?;

    // ngspice's exact labels, per-form ordering, and C-style %e exponent
    // formatting, so scripts written against ngspice's .TF output parse
    // RSpice's unchanged.
    let probe = result.output.to_lowercase();
    let source = result.input.to_lowercase();
    let gain = format_spice_exponent(result.gain);
    let zin = format_spice_exponent(result.input_impedance);
    let zout = format_spice_exponent(result.output_impedance);
    println!("Transfer function information:");
    println!("transfer_function = {gain}");
    if output_is_current {
        println!("{source}#input_impedance = {zin}");
        println!("{}#output_impedance = {zout}", output_node.to_lowercase());
    } else {
        println!("output_impedance_at_{probe} = {zout}");
        println!("{source}#input_impedance = {zin}");
    }

    Ok(())
}

/// Format like C's `%e` (ngspice's style): six fractional digits and a
/// signed, zero-padded, at-least-two-digit exponent.
fn format_spice_exponent(value: f64) -> String {
    let formatted = format!("{value:.6e}");
    match formatted.split_once('e') {
        Some((mantissa, exponent)) => {
            let (sign, digits) = match exponent.strip_prefix('-') {
                Some(rest) => ('-', rest),
                None => ('+', exponent),
            };
            format!("{mantissa}e{sign}{digits:0>2}")
        }
        None => formatted,
    }
}

pub(super) fn run_disto(
    ctx: &RunContext<'_>,
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
    f2_over_f1: Option<f64>,
) -> Result<(), CliError> {
    if let Some(ratio) = f2_over_f1
        && (!ratio.is_finite() || ratio <= 1.0)
    {
        return Err(CliError::simulation_error_in(
            format!(
                "Invalid .DISTO f2_over_f1 ratio '{}': expected a finite value > 1",
                ratio
            ),
            "DISTO",
        ));
    }

    if ctx.verbose && !ctx.quiet {
        match f2_over_f1 {
            Some(ratio) => println!(
                "DISTO note: using linearized AC sweep in CLI (f2/f1={:.6}); full IMD metrics are available in rspice-ui",
                ratio
            ),
            None => println!(
                "DISTO note: using linearized AC sweep in CLI; full harmonic/IMD metrics are available in rspice-ui"
            ),
        }
    }

    run_ac(ctx, variation, points, start_freq, stop_freq)
}

pub(super) fn run_ac(
    ctx: &RunContext<'_>,
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running AC analysis: {} to {} Hz ({} points)...",
            start_freq, stop_freq, points
        );
    }

    let frequencies = generate_frequency_sweep(variation, points, start_freq, stop_freq);

    match ctx.engine.run_ac(ctx.netlist, &frequencies) {
        Ok(results) => {
            if !ctx.quiet {
                println!("AC Analysis: {} frequency points", results.len());
                if ctx.verbose && !results.is_empty() {
                    let first = &results[0];
                    let last = results.last().unwrap();
                    let first_label = first
                        .node_names
                        .first()
                        .map_or_else(|| "V(1)".to_string(), |name| voltage_display_name(name, 1));
                    println!(
                        "  @ {:e} Hz: |{}| = {:.4}",
                        first.frequency,
                        first_label,
                        first.voltage_magnitude(1)
                    );
                    println!(
                        "  @ {:e} Hz: |{}| = {:.4}",
                        last.frequency,
                        first_label,
                        last.voltage_magnitude(1)
                    );
                }
            }

            if let Some(ref output_path) = ctx.output_path_for("ac") {
                let signals = ac_signals(&results);
                let frequencies: Vec<f64> =
                    results.iter().map(|result| result.frequency).collect();
                if matches!(ctx.format, OutputFormat::Hdf5) {
                    let mut data = Hdf5SimulationData::new();
                    data.title = "AC Analysis".to_string();

                    let mut ac = Hdf5AcSection::new(frequencies);
                    for signal in &signals {
                        ac.add_signal(
                            signal.display_name.clone(),
                            signal.real.clone(),
                            signal.imag.clone(),
                        );
                    }
                    data.ac = Some(ac);

                    write_hdf5(output_path, &data)
                        .map_err(|err| map_hdf5_output_error(output_path, err))?;
                } else {
                    super::export::complex_table("ac", "AC Analysis", frequencies, &signals)
                        .write(output_path, ctx.format)?;
                }
                if !ctx.quiet {
                    println!("  AC response exported to: {}", output_path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "AC")),
    }
}

pub(super) fn run_noise(
    ctx: &RunContext<'_>,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running Noise analysis on {} from {} to {} Hz ({} points)...",
            output_node, start_freq, stop_freq, points
        );
    }

    let resolver = NodeResolver::from_netlist(ctx.engine, ctx.netlist)?;
    let output = resolver
        .resolve_node(output_node)
        .ok_or_else(|| CliError::SimulationError {
            message: format!("Invalid .NOISE output node '{}'", output_node),
            analysis: Some("Noise".to_string()),
        })?;
    let output_neg = match reference_node {
        Some(reference) => {
            Some(
                resolver
                    .resolve_node(reference)
                    .ok_or_else(|| CliError::SimulationError {
                        message: format!("Invalid .NOISE reference node '{}'", reference),
                        analysis: Some("Noise".to_string()),
                    })?,
            )
        }
        None => None,
    };

    let input_source_exists = ctx.netlist.elements.iter().any(|element| {
        element.name.eq_ignore_ascii_case(input_source)
            && matches!(
                element.kind,
                rspice_core::netlist::ElementKind::VoltageSource(_)
                    | rspice_core::netlist::ElementKind::CurrentSource(_)
            )
    });
    if !input_source_exists {
        return Err(CliError::SimulationError {
            message: format!(
                "Invalid .NOISE input source '{}': expected an independent V or I source name",
                input_source
            ),
            analysis: Some("Noise".to_string()),
        });
    }

    let frequencies = generate_frequency_sweep(variation, points, start_freq, stop_freq);
    match ctx.engine.run_noise_with_input_source(
        ctx.netlist,
        output,
        output_neg,
        input_source,
        &frequencies,
        ctx.engine.config().temperature,
    ) {
        Ok(results) => {
            if !ctx.quiet {
                println!("Noise Analysis: {} frequency points", results.len());
                if let Some(reference) = reference_node {
                    println!("  Output node: V({},{})", output_node, reference);
                } else {
                    println!("  Output node: V({})", output_node);
                }
                println!("  Input source: {}", input_source);
                if let (Some(first), Some(last)) = (results.first(), results.last()) {
                    println!(
                        "  @ {:e} Hz: output_noise={:.6e} V^2/Hz",
                        first.frequency, first.output_noise_density
                    );
                    println!(
                        "  @ {:e} Hz: input_referred={:.6e}",
                        first.frequency, first.input_referred_density
                    );
                    println!(
                        "  @ {:e} Hz: output_noise={:.6e} V^2/Hz",
                        last.frequency, last.output_noise_density
                    );
                    println!(
                        "  @ {:e} Hz: input_referred={:.6e}",
                        last.frequency, last.input_referred_density
                    );
                }
            }

            if let Some(ref output_path) = ctx.output_path_for("noise") {
                let noise_frequencies: Vec<f64> =
                    results.iter().map(|result| result.frequency).collect();
                let onoise: Vec<f64> = results
                    .iter()
                    .map(|result| result.output_noise_density)
                    .collect();
                let inoise: Vec<f64> = results
                    .iter()
                    .map(|result| result.input_referred_density)
                    .collect();

                if matches!(ctx.format, OutputFormat::Hdf5) {
                    let mut data = Hdf5SimulationData::new();
                    data.title = "Noise Analysis".to_string();

                    let mut noise = Hdf5WaveformSection::new("frequency", noise_frequencies);
                    noise.add_signal("onoise_spectrum", onoise);
                    noise.add_signal("inoise_spectrum", inoise);
                    data.noise = Some(noise);

                    write_hdf5(output_path, &data)
                        .map_err(|err| map_hdf5_output_error(output_path, err))?;
                } else {
                    use super::export::{ColumnData, ExportColumn, ExportTable};

                    ExportTable {
                        analysis: "noise".to_string(),
                        plot_name: "Noise Spectral Density Curves".to_string(),
                        scale_name: "frequency".to_string(),
                        scale_type: "frequency".to_string(),
                        scale: noise_frequencies,
                        columns: vec![
                            ExportColumn {
                                name: "onoise_spectrum".to_string(),
                                var_type: "voltage".to_string(),
                                data: ColumnData::Real(onoise),
                            },
                            ExportColumn {
                                name: "inoise_spectrum".to_string(),
                                var_type: "voltage".to_string(),
                                data: ColumnData::Real(inoise),
                            },
                        ],
                    }
                    .write(output_path, ctx.format)?;
                }

                if !ctx.quiet {
                    println!("  Noise spectra exported to: {}", output_path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Noise")),
    }
}

pub(super) fn run_pz(
    ctx: &RunContext<'_>,
    input_node: usize,
    output_node: usize,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running Pole-Zero analysis: input=node {}, output=node {}",
            input_node, output_node
        );
    }

    match ctx.engine.run_pz(ctx.netlist, input_node, output_node) {
        Ok(result) => {
            if !ctx.quiet {
                println!("✓ Pole-Zero analysis complete");
                println!("  Poles: {}", result.poles.len());
                println!("  Zeros: {}", result.zeros.len());

                if ctx.verbose {
                    println!("\n  Poles:");
                    for (i, pole) in result.poles.iter().enumerate() {
                        let freq = pole.im / (2.0 * std::f64::consts::PI);
                        let q = if pole.re.abs() > 1e-15 {
                            -pole.im / (2.0 * pole.re)
                        } else {
                            f64::INFINITY
                        };
                        println!(
                            "    P{}: {:.3e} + j{:.3e}  (f={:.3e} Hz, Q={:.2})",
                            i,
                            pole.re,
                            pole.im,
                            freq.abs(),
                            q
                        );
                    }
                    println!("\n  Zeros:");
                    for (i, zero) in result.zeros.iter().enumerate() {
                        println!("    Z{}: {:.3e} + j{:.3e}", i, zero.re, zero.im);
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Pole-Zero")),
    }
}

pub(super) fn run_pz_from_command(
    ctx: &RunContext<'_>,
    input_pos: &str,
    input_neg: &str,
    output_pos: &str,
    output_neg: &str,
    transfer_type: rspice_core::netlist::PoleZeroTransferType,
    analysis_type: rspice_core::netlist::PoleZeroAnalysisType,
) -> Result<(), CliError> {
    let resolver = NodeResolver::from_netlist(ctx.engine, ctx.netlist)?;

    let resolve = |node: &str| {
        resolver
            .resolve_node(node)
            .ok_or_else(|| CliError::SimulationError {
                message: format!("Invalid .PZ node reference '{}'", node),
                analysis: Some("Pole-Zero".to_string()),
            })
    };

    let in_pos = resolve(input_pos)?;
    let in_neg = resolve(input_neg)?;
    let out_pos = resolve(output_pos)?;
    let out_neg = resolve(output_neg)?;

    let input_is_current = matches!(
        transfer_type,
        rspice_core::netlist::PoleZeroTransferType::Current
    );
    let (compute_poles, compute_zeros) = match analysis_type {
        rspice_core::netlist::PoleZeroAnalysisType::PoleZero => (true, true),
        rspice_core::netlist::PoleZeroAnalysisType::PolesOnly => (true, false),
        rspice_core::netlist::PoleZeroAnalysisType::ZerosOnly => (false, true),
    };

    if !ctx.quiet {
        println!(
            "Running Pole-Zero analysis from netlist command: in=({},{}) out=({},{}) transfer={:?} mode={:?}",
            in_pos, in_neg, out_pos, out_neg, transfer_type, analysis_type
        );
    }

    match ctx.engine.run_pz_ports(
        ctx.netlist,
        in_pos,
        Some(in_neg),
        out_pos,
        Some(out_neg),
        input_is_current,
        compute_poles,
        compute_zeros,
    ) {
        Ok(result) => {
            if !ctx.quiet {
                println!("✓ Pole-Zero analysis complete");
                println!("  Poles: {}", result.poles.len());
                println!("  Zeros: {}", result.zeros.len());

                if ctx.verbose {
                    println!("\n  Poles:");
                    for (i, pole) in result.poles.iter().enumerate() {
                        let freq = pole.im / (2.0 * std::f64::consts::PI);
                        let q = if pole.re.abs() > 1e-15 {
                            -pole.im / (2.0 * pole.re)
                        } else {
                            f64::INFINITY
                        };
                        println!(
                            "    P{}: {:.3e} + j{:.3e}  (f={:.3e} Hz, Q={:.2})",
                            i,
                            pole.re,
                            pole.im,
                            freq.abs(),
                            q
                        );
                    }
                    println!("\n  Zeros:");
                    for (i, zero) in result.zeros.iter().enumerate() {
                        println!("    Z{}: {:.3e} + j{:.3e}", i, zero.re, zero.im);
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Pole-Zero")),
    }
}

pub(super) fn run_sensitivity(
    ctx: &RunContext<'_>,
    output_node: usize,
    param_name: &str,
    param_value: f64,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running Sensitivity analysis: ∂V({})/∂{} at {}={:.6e}",
            output_node, param_name, param_name, param_value
        );
    }

    match ctx
        .engine
        .run_sensitivity(ctx.netlist, output_node, param_name, param_value, None)
    {
        Ok(sensitivity) => {
            if !ctx.quiet {
                println!("✓ Sensitivity analysis complete");
                println!(
                    "  ∂V({})/∂{} = {:.6e} V/unit",
                    output_node, param_name, sensitivity
                );

                if ctx.verbose {
                    let nominal_sens = sensitivity * param_value;
                    println!(
                        "  Normalized: {:.2}% change per 1% parameter variation",
                        nominal_sens * 100.0
                    );
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Sensitivity")),
    }
}

pub(super) fn run_sensitivity_from_command(
    ctx: &RunContext<'_>,
    output_node: &str,
    reference_node: Option<&str>,
    ac_sweep: Option<rspice_core::netlist::SensitivityAcSweep>,
) -> Result<(), CliError> {
    let resolver = NodeResolver::from_netlist(ctx.engine, ctx.netlist)?;
    let out_pos = resolver
        .resolve_node(output_node)
        .ok_or_else(|| CliError::SimulationError {
            message: format!("Invalid .SENS output node '{}'", output_node),
            analysis: Some("Sensitivity".to_string()),
        })?;
    let out_neg = match reference_node {
        Some(node) => resolver
            .resolve_node(node)
            .ok_or_else(|| CliError::SimulationError {
                message: format!("Invalid .SENS reference node '{}'", node),
                analysis: Some("Sensitivity".to_string()),
            })?,
        None => 0,
    };

    let mut params: Vec<(String, f64)> = ctx
        .netlist
        .params
        .all_params()
        .into_iter()
        .filter(|(name, value)| {
            !name.starts_with("IC_")
                && !name.starts_with("NODESET_")
                && value.is_finite()
                && value.abs() > 0.0
        })
        .collect();
    params.sort_by(|a, b| a.0.cmp(&b.0));

    if params.is_empty() {
        return Err(CliError::SimulationError {
            message: ".SENS requires at least one non-zero top-level .PARAM".to_string(),
            analysis: Some("Sensitivity".to_string()),
        });
    }

    if let Some(ac) = ac_sweep {
        let freqs = generate_frequency_sweep(ac.variation, ac.points, ac.start_freq, ac.stop_freq);
        if freqs.is_empty() {
            return Err(CliError::SimulationError {
                message: "Invalid .SENS AC frequency sweep configuration".to_string(),
                analysis: Some("Sensitivity".to_string()),
            });
        }

        if !ctx.quiet {
            println!(
                "Running AC Sensitivity analysis: V({},{}) over {} frequencies",
                output_node,
                reference_node.unwrap_or("0"),
                freqs.len()
            );
        }

        for (param_name, param_value) in &params {
            let pos = ctx
                .engine
                .run_sensitivity_ac(ctx.netlist, out_pos, param_name, *param_value, &freqs, None)
                .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity AC"))?;
            let combined = if out_neg == 0 {
                pos
            } else {
                let neg = ctx
                    .engine
                    .run_sensitivity_ac(
                        ctx.netlist,
                        out_neg,
                        param_name,
                        *param_value,
                        &freqs,
                        None,
                    )
                    .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity AC"))?;
                pos.iter()
                    .zip(neg.iter())
                    .map(|(sp, sn)| sp - sn)
                    .collect::<Vec<_>>()
            };

            if !ctx.quiet {
                let first = combined.first().copied().unwrap_or(0.0);
                let last = combined.last().copied().unwrap_or(0.0);
                println!(
                    "  d|V|/d{}: {:.6e} @ {:e} Hz, {:.6e} @ {:e} Hz",
                    param_name,
                    first,
                    freqs.first().copied().unwrap_or(0.0),
                    last,
                    freqs.last().copied().unwrap_or(0.0)
                );
            }

            if ctx.verbose && !ctx.quiet {
                let peak = combined
                    .iter()
                    .map(|v| v.abs())
                    .fold(0.0_f64, |acc, v| acc.max(v));
                println!("    peak |d|V|/d{}| = {:.6e}", param_name, peak);
            }
        }

        return Ok(());
    }

    if !ctx.quiet {
        println!(
            "Running DC Sensitivity analysis: V({},{})",
            output_node,
            reference_node.unwrap_or("0")
        );
    }

    let mut results: Vec<(String, f64)> = Vec::with_capacity(params.len());
    for (param_name, param_value) in &params {
        let pos = ctx
            .engine
            .run_sensitivity(ctx.netlist, out_pos, param_name, *param_value, None)
            .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity"))?;
        let combined = if out_neg == 0 {
            pos
        } else {
            let neg = ctx
                .engine
                .run_sensitivity(ctx.netlist, out_neg, param_name, *param_value, None)
                .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity"))?;
            pos - neg
        };
        results.push((param_name.clone(), combined));
    }

    results.sort_by(|a, b| {
        b.1.abs()
            .partial_cmp(&a.1.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if !ctx.quiet {
        println!("✓ Sensitivity analysis complete");
        for (name, value) in &results {
            println!("  ∂V/∂{} = {:.6e}", name, value);
        }
    }

    Ok(())
}
