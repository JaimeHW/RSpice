use super::RunContext;
use super::shared::{NodeResolver, generate_frequency_sweep, map_hdf5_output_error};
use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::{ac_signals, voltage_display_name};
use crate::hdf5::{Hdf5AcSection, Hdf5SimulationData, Hdf5WaveformSection, write_hdf5};
use crate::report::format_spice_exponent;

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

    if let Some(ref output_path) = ctx.output_path_for("tf") {
        reject_hdf5(ctx.format, "transfer function")?;
        use super::export::{ColumnData, ExportColumn, ExportTable};

        let scalar = |name: &str, var_type: &str, value: f64| ExportColumn {
            name: name.to_string(),
            var_type: var_type.to_string(),
            data: ColumnData::Real(vec![value]),
        };
        ExportTable {
            analysis: "tf".to_string(),
            plot_name: "DC Transfer Function".to_string(),
            scale_name: "point".to_string(),
            scale_type: "index".to_string(),
            scale: vec![0.0],
            columns: vec![
                scalar("transfer_function", "gain", result.gain),
                scalar(
                    &format!("{source}#input_impedance"),
                    "impedance",
                    result.input_impedance,
                ),
                scalar(
                    &format!("output_impedance_at_{probe}"),
                    "impedance",
                    result.output_impedance,
                ),
            ],
        }
        .write(output_path, ctx.format)?;

        if !ctx.quiet {
            println!("  Transfer function exported to: {}", output_path.display());
        }
    }

    Ok(())
}

/// The report-shaped analyses (TF, PZ, sensitivity) have no natural HDF5
/// section; fail with a clear message instead of writing a misleading file.
fn reject_hdf5(format: OutputFormat, what: &str) -> Result<(), CliError> {
    if matches!(format, OutputFormat::Hdf5) {
        return Err(CliError::InvalidArgument {
            message: format!("HDF5 output is not supported for {what} results"),
            suggestion: Some("use --format csv, json, or raw".to_string()),
        });
    }
    Ok(())
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

pub(super) fn run_ac_data(ctx: &RunContext<'_>, table_name: &str) -> Result<(), CliError> {
    let table = ctx
        .netlist
        .data_tables
        .iter()
        .find(|table| table.name.eq_ignore_ascii_case(table_name))
        .ok_or_else(|| invalid_ac_data(format!(".AC DATA table '{table_name}' not found")))?;
    let freq_col = table
        .params
        .iter()
        .position(|param| param.eq_ignore_ascii_case("FREQ"))
        .ok_or_else(|| {
            invalid_ac_data(format!(
                ".AC DATA table '{}' must contain a FREQ column",
                table.name
            ))
        })?;

    if table.rows.is_empty() {
        return Err(invalid_ac_data(format!(
            ".AC DATA table '{}' has no rows",
            table.name
        )));
    }

    let mut frequencies = Vec::with_capacity(table.rows.len());
    for (row_idx, row) in table.rows.iter().enumerate() {
        if row.len() != table.params.len() {
            return Err(invalid_ac_data(format!(
                ".AC DATA table '{}' row {} has {} value(s), expected {}",
                table.name,
                row_idx + 1,
                row.len(),
                table.params.len()
            )));
        }
        let frequency = row[freq_col];
        if !frequency.is_finite() || frequency < 0.0 {
            return Err(invalid_ac_data(format!(
                ".AC DATA table '{}' row {} has invalid frequency {}",
                table.name,
                row_idx + 1,
                frequency
            )));
        }
        frequencies.push(frequency);
    }

    if !ctx.quiet {
        println!(
            "Running AC DATA analysis from table {} ({} points)...",
            table.name,
            frequencies.len()
        );
    }

    run_ac_frequencies(ctx, frequencies)
}

fn invalid_ac_data(message: String) -> CliError {
    CliError::InvalidArgument {
        message,
        suggestion: Some("fix the .DATA table referenced by .AC DATA=<name>".to_string()),
    }
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
    run_ac_frequencies(ctx, frequencies)
}

fn run_ac_frequencies(ctx: &RunContext<'_>, frequencies: Vec<f64>) -> Result<(), CliError> {
    match ctx.engine.run_ac(ctx.netlist, &frequencies) {
        Ok(results) => {
            if !ctx.args.allow_nonfinite {
                for result in &results {
                    for (node, voltage) in result.voltages.iter().enumerate() {
                        if !voltage.re.is_finite() || !voltage.im.is_finite() {
                            let name = result
                                .node_names
                                .get(node)
                                .map(|n| n.as_str())
                                .unwrap_or("node");
                            return Err(CliError::SimulationError {
                                message: format!(
                                    "{name} is non-finite at {:e} Hz; the solution is \
                                     not physical. Use --allow-nonfinite to export anyway.",
                                    result.frequency
                                ),
                                analysis: Some("AC".to_string()),
                            });
                        }
                    }
                }
            }

            ctx.record_measurements(
                "AC",
                rspice_core::analysis::advanced::evaluate_ac_measurements(ctx.netlist, &results),
            );

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
                let signals = crate::commands::run_signals::apply_save_set_complex(
                    ac_signals(&results),
                    &ctx.netlist.saves,
                );
                let frequencies: Vec<f64> = results.iter().map(|result| result.frequency).collect();
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

/// Run `.STB`: Tian double-injection loop gain at a designated 0 V probe
/// source, with gain/phase margins extracted from the sweep.
pub(super) fn run_stb(
    ctx: &RunContext<'_>,
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
    probe: &str,
) -> Result<(), CliError> {
    use rspice_core::analysis::advanced::{StbConfig, StbSweepType};
    use rspice_core::netlist::FreqVariation;

    if !ctx.quiet {
        println!(
            "Running STB (loop stability) analysis: {} to {} Hz, probe {}...",
            start_freq, stop_freq, probe
        );
    }

    let sweep_type = match variation {
        FreqVariation::Lin => StbSweepType::Linear,
        FreqVariation::Dec => StbSweepType::Decade,
        FreqVariation::Oct => StbSweepType::Octave,
    };
    let config = StbConfig::new()
        .with_sweep(start_freq, stop_freq, points)
        .with_sweep_type(sweep_type)
        .with_probe(probe);

    let stb = ctx
        .engine
        .run_stb(ctx.netlist, config)
        .map_err(|e| CliError::simulation_error_in(e.to_string(), "STB"))?;

    if !ctx.args.allow_nonfinite {
        for (freq, gain) in stb.frequencies.iter().zip(stb.loop_gains.iter()) {
            if !gain.re.is_finite() || !gain.im.is_finite() {
                return Err(CliError::SimulationError {
                    message: format!(
                        "loop gain is non-finite at {freq:e} Hz; the solution is not \
                         physical. Use --allow-nonfinite to export anyway."
                    ),
                    analysis: Some("STB".to_string()),
                });
            }
        }
    }

    let margins = &stb.result.margins;
    if !ctx.quiet {
        println!(
            "STB Analysis: {} frequency points, probe {}",
            stb.frequencies.len(),
            stb.probe_name
        );
        if margins.num_crossovers == 0 {
            println!(
                "  Loop gain never crosses unity ({:.1} dB at DC); no phase margin to report",
                margins.dc_gain_db
            );
        } else {
            println!(
                "  Phase margin: {:.2} deg at {:.4e} Hz (unity-gain crossover)",
                margins.phase_margin_deg, margins.phase_margin_freq
            );
            println!(
                "  Gain margin: {:.2} dB at {:.4e} Hz",
                margins.gain_margin_db, margins.gain_margin_freq
            );
            if margins.conditionally_stable {
                println!(
                    "  Conditionally stable: {} unity-gain crossovers",
                    margins.num_crossovers
                );
            }
        }
        for warning in &stb.result.warnings {
            println!("  Warning: {warning}");
        }
    }

    if let Some(ref output_path) = ctx.output_path_for("stb") {
        if matches!(ctx.format, OutputFormat::Hdf5) {
            let mut data = Hdf5SimulationData::new();
            data.title = "STB Loop Gain".to_string();

            let mut ac = Hdf5AcSection::new(stb.frequencies.clone());
            ac.add_signal(
                "loopgain".to_string(),
                stb.loop_gains.iter().map(|g| g.re).collect(),
                stb.loop_gains.iter().map(|g| g.im).collect(),
            );
            data.ac = Some(ac);

            write_hdf5(output_path, &data)
                .map_err(|err| map_hdf5_output_error(output_path, err))?;
        } else {
            use super::export::{ColumnData, ExportColumn, ExportTable};

            ExportTable {
                analysis: "stb".to_string(),
                plot_name: "STB Loop Gain".to_string(),
                scale_name: "frequency".to_string(),
                scale_type: "frequency".to_string(),
                scale: stb.frequencies.clone(),
                columns: vec![
                    ExportColumn {
                        name: "loopgain".to_string(),
                        var_type: "gain".to_string(),
                        data: ColumnData::Complex {
                            real: stb.loop_gains.iter().map(|g| g.re).collect(),
                            imag: stb.loop_gains.iter().map(|g| g.im).collect(),
                        },
                    },
                    ExportColumn {
                        name: "loopgain_mag_db".to_string(),
                        var_type: "gain".to_string(),
                        data: ColumnData::Real(
                            stb.loop_gains
                                .iter()
                                .map(|g| 20.0 * g.norm().max(1e-300).log10())
                                .collect(),
                        ),
                    },
                    ExportColumn {
                        name: "loopgain_phase_deg".to_string(),
                        var_type: "phase".to_string(),
                        data: ColumnData::Real(
                            stb.loop_gains
                                .iter()
                                .map(|g| g.im.atan2(g.re).to_degrees())
                                .collect(),
                        ),
                    },
                ],
            }
            .write(output_path, ctx.format)?;
        }
        if !ctx.quiet {
            println!("  Loop gain exported to: {}", output_path.display());
        }
    }

    Ok(())
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
            if !ctx.args.allow_nonfinite {
                for result in &results {
                    if !result.output_noise_rms().is_finite()
                        || !result.input_referred_rms().is_finite()
                    {
                        return Err(CliError::SimulationError {
                            message: format!(
                                "noise spectrum is non-finite at {:e} Hz; the solution \
                                 is not physical. Use --allow-nonfinite to export anyway.",
                                result.frequency
                            ),
                            analysis: Some("Noise".to_string()),
                        });
                    }
                }
            }

            ctx.record_measurements(
                "NOISE",
                rspice_core::analysis::advanced::evaluate_noise_measurements(ctx.netlist, &results),
            );

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
                        "  @ {:e} Hz: output_noise={:.6e} V/sqrt(Hz)",
                        first.frequency,
                        first.output_noise_rms()
                    );
                    println!(
                        "  @ {:e} Hz: input_referred={:.6e} /sqrt(Hz)",
                        first.frequency,
                        first.input_referred_rms()
                    );
                    println!(
                        "  @ {:e} Hz: output_noise={:.6e} V/sqrt(Hz)",
                        last.frequency,
                        last.output_noise_rms()
                    );
                    println!(
                        "  @ {:e} Hz: input_referred={:.6e} /sqrt(Hz)",
                        last.frequency,
                        last.input_referred_rms()
                    );
                }

                print_noise_contribution_summary(&results, ctx.verbose);
            }

            if let Some(ref output_path) = ctx.output_path_for("noise") {
                let noise_frequencies: Vec<f64> =
                    results.iter().map(|result| result.frequency).collect();
                // ngspice-46 emits the onoise/inoise vectors in
                // root-spectral-density units (V/sqrt(Hz)) unless the legacy
                // `sqrnoise` control variable is set; exported tables carry
                // the modern convention so they diff cleanly against it.
                let onoise: Vec<f64> = results
                    .iter()
                    .map(|result| result.output_noise_rms())
                    .collect();
                let inoise: Vec<f64> = results
                    .iter()
                    .map(|result| result.input_referred_rms())
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

/// Print the ranked, band-integrated noise-contributor table.
///
/// Compact (top 10) by default; `verbose` lifts the cap. Rows carry the
/// device, mechanism, integrated output power, and share of the total so
/// the dominant contributor is visible at a glance.
fn print_noise_contribution_summary(results: &[rspice_core::analysis::NoiseResult], verbose: bool) {
    let integrated = rspice_core::analysis::IntegratedNoise::new(results.to_vec());
    let summary = integrated.contribution_summary();
    if summary.is_empty() {
        return;
    }

    const COMPACT_ROW_CAP: usize = 10;
    let cap = if verbose { usize::MAX } else { COMPACT_ROW_CAP };

    println!("  Noise Contributors (band-integrated, ranked):");
    println!(
        "    {:<20} {:<9} {:>14} {:>8}",
        "DEVICE", "TYPE", "POWER (V^2)", "SHARE"
    );
    for contribution in summary.iter().take(cap) {
        println!(
            "    {:<20} {:<9} {:>14.4e} {:>7.2}%",
            contribution.device_name,
            contribution.noise_type.label(),
            contribution.integrated_power,
            contribution.percentage
        );
    }
    if summary.len() > cap {
        println!(
            "    ... ({} more contributors; rerun with --verbose for all)",
            summary.len() - cap
        );
    }
    println!(
        "  Total integrated output noise: {:.6e} V rms",
        integrated.total_output_noise()
    );
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
            report_pz(ctx, &result.poles, &result.zeros)?;
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Pole-Zero")),
    }
}

/// Print the pole/zero summary and export the singularities.
///
/// The export follows the rawfile convention for .PZ results: a single
/// point with one complex variable per pole/zero (`pole(1)`, `zero(1)`, ...).
fn report_pz(
    ctx: &RunContext<'_>,
    poles: &[rspice_core::analysis::advanced::pole_zero::Complex],
    zeros: &[rspice_core::analysis::advanced::pole_zero::Complex],
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!("✓ Pole-Zero analysis complete");
        println!("  Poles: {}", poles.len());
        println!("  Zeros: {}", zeros.len());

        if ctx.verbose {
            println!("\n  Poles:");
            for (i, pole) in poles.iter().enumerate() {
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
            for (i, zero) in zeros.iter().enumerate() {
                println!("    Z{}: {:.3e} + j{:.3e}", i, zero.re, zero.im);
            }
        }
    }

    if let Some(ref output_path) = ctx.output_path_for("pz") {
        reject_hdf5(ctx.format, "pole-zero")?;
        use super::export::{ColumnData, ExportColumn, ExportTable};

        let singularity =
            |label: &str,
             index: usize,
             value: &rspice_core::analysis::advanced::pole_zero::Complex| {
                ExportColumn {
                    name: format!("{label}({})", index + 1),
                    var_type: "frequency".to_string(),
                    data: ColumnData::Complex {
                        real: vec![value.re],
                        imag: vec![value.im],
                    },
                }
            };
        let columns: Vec<ExportColumn> = poles
            .iter()
            .enumerate()
            .map(|(i, p)| singularity("pole", i, p))
            .chain(
                zeros
                    .iter()
                    .enumerate()
                    .map(|(i, z)| singularity("zero", i, z)),
            )
            .collect();

        ExportTable {
            analysis: "pz".to_string(),
            plot_name: "Pole-Zero Analysis".to_string(),
            scale_name: "point".to_string(),
            scale_type: "index".to_string(),
            scale: vec![0.0],
            columns,
        }
        .write(output_path, ctx.format)?;

        if !ctx.quiet {
            println!("  Poles/zeros exported to: {}", output_path.display());
        }
    }

    Ok(())
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
            report_pz(ctx, &result.poles, &result.zeros)?;
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

            export_dc_sensitivities(ctx, &[(param_name.to_string(), sensitivity)])?;
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Sensitivity")),
    }
}

pub(super) fn run_sensitivity_from_command(
    ctx: &RunContext<'_>,
    output_node: &str,
    reference_node: Option<&str>,
    output_is_current: bool,
    filters: &[String],
    ac_sweep: Option<rspice_core::netlist::SensitivityAcSweep>,
) -> Result<(), CliError> {
    let resolver = NodeResolver::from_netlist(ctx.engine, ctx.netlist)?;
    let out_pos = if output_is_current {
        0
    } else {
        resolver
            .resolve_node(output_node)
            .ok_or_else(|| CliError::SimulationError {
                message: format!("Invalid .SENS output node '{}'", output_node),
                analysis: Some("Sensitivity".to_string()),
            })?
    };
    let out_neg = if output_is_current {
        0
    } else {
        match reference_node {
            Some(node) => resolver
                .resolve_node(node)
                .ok_or_else(|| CliError::SimulationError {
                    message: format!("Invalid .SENS reference node '{}'", node),
                    analysis: Some("Sensitivity".to_string()),
                })?,
            None => 0,
        }
    };

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

        let output = if output_is_current {
            rspice_core::analysis::AcSensitivityOutput::BranchCurrent(output_node.to_string())
        } else {
            rspice_core::analysis::AcSensitivityOutput::Voltage {
                positive: out_pos,
                negative: (out_neg != 0).then_some(out_neg),
            }
        };
        let result = ctx
            .engine
            .run_sensitivity_ac_complete(ctx.netlist, output, &freqs, filters)
            .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity AC"))?;

        for trace in &result.sensitivities {
            let combined = &trace.magnitude;

            if !ctx.quiet {
                let first = combined.first().copied().unwrap_or(0.0);
                let last = combined.last().copied().unwrap_or(0.0);
                println!(
                    "  d|V|/d{}: {:.6e} @ {:e} Hz, {:.6e} @ {:e} Hz",
                    trace.vector_name,
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
                println!("    peak |d|V|/d{}| = {:.6e}", trace.vector_name, peak);
            }
        }

        if let Some(ref output_path) = ctx.output_path_for("sens") {
            reject_hdf5(ctx.format, "sensitivity")?;
            use super::export::{ColumnData, ExportColumn, ExportTable};

            ExportTable {
                analysis: "sens_ac".to_string(),
                plot_name: "AC Sensitivity".to_string(),
                scale_name: "frequency".to_string(),
                scale_type: "frequency".to_string(),
                scale: freqs.clone(),
                columns: result
                    .sensitivities
                    .iter()
                    .map(|trace| ExportColumn {
                        name: format!("dV/d({})", trace.vector_name),
                        var_type: "sensitivity".to_string(),
                        data: ColumnData::Complex {
                            real: trace.absolute.iter().map(|value| value.re).collect(),
                            imag: trace.absolute.iter().map(|value| value.im).collect(),
                        },
                    })
                    .collect(),
            }
            .write(output_path, ctx.format)?;

            if !ctx.quiet {
                println!("  Sensitivities exported to: {}", output_path.display());
            }
        }

        return Ok(());
    }

    if output_is_current {
        return Err(CliError::SimulationError {
            message: "DC .SENS I(element) is not yet supported by the DC adjoint path".to_string(),
            analysis: Some("Sensitivity".to_string()),
        });
    }

    if !filters.is_empty() {
        return Err(CliError::SimulationError {
            message: "DC .SENS device filters are not yet supported by the DC adjoint path"
                .to_string(),
            analysis: Some("Sensitivity".to_string()),
        });
    }

    if !ctx.quiet {
        println!(
            "Running DC Sensitivity analysis: V({},{})",
            output_node,
            reference_node.unwrap_or("0")
        );
    }

    let linearized = ctx
        .engine
        .run_sensitivity_linearized(ctx.netlist, out_pos, (out_neg != 0).then_some(out_neg))
        .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity"))?;
    let mut results = linearized
        .sensitivities
        .into_iter()
        .map(|trace| (trace.element, trace.absolute))
        .collect::<Vec<_>>();

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

    export_dc_sensitivities(ctx, &results)?;
    Ok(())
}

/// Write DC sensitivities: a single-point table with one `dV/d(param)`
/// column per parameter.
fn export_dc_sensitivities(
    ctx: &RunContext<'_>,
    results: &[(String, f64)],
) -> Result<(), CliError> {
    let Some(ref output_path) = ctx.output_path_for("sens") else {
        return Ok(());
    };
    reject_hdf5(ctx.format, "sensitivity")?;
    use super::export::{ColumnData, ExportColumn, ExportTable};

    ExportTable {
        analysis: "sens".to_string(),
        plot_name: "DC Sensitivity".to_string(),
        scale_name: "point".to_string(),
        scale_type: "index".to_string(),
        scale: vec![0.0],
        columns: results
            .iter()
            .map(|(name, value)| ExportColumn {
                name: format!("dV/d({name})"),
                var_type: "sensitivity".to_string(),
                data: ColumnData::Real(vec![*value]),
            })
            .collect(),
    }
    .write(output_path, ctx.format)?;

    if !ctx.quiet {
        println!("  Sensitivities exported to: {}", output_path.display());
    }
    Ok(())
}
