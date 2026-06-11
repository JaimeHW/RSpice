use super::RunContext;
use super::shared::{NodeResolver, map_hdf5_output_error};
use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::{
    dc_operating_point_current_signals, dc_operating_point_signals,
    dc_operating_point_voltage_signals, insert_transient_measurement_aliases,
    transient_voltage_signals,
};
use crate::hdf5::{Hdf5SimulationData, Hdf5WaveformSection, write_hdf5};
use rspice_core::Netlist;
use std::path::Path;

pub(super) fn run_dc_op(ctx: &RunContext<'_>) -> Result<(), CliError> {
    if !ctx.quiet {
        println!("Running DC operating point...");
    }

    match ctx.engine.run_dc_op(ctx.netlist) {
        Ok(result) => {
            if !ctx.quiet {
                let voltage_signals = dc_operating_point_voltage_signals(&result);
                let current_signals = dc_operating_point_current_signals(&result);
                println!("DC Operating Point:");
                for signal in voltage_signals.iter().take(10) {
                    println!("  {} = {:.6} V", signal.display_name, signal.values[0]);
                }
                if voltage_signals.len() > 10 {
                    println!("  ... ({} more node voltages)", voltage_signals.len() - 10);
                }
                for signal in current_signals.iter().take(5) {
                    println!("  {} = {:.6} A", signal.display_name, signal.values[0]);
                }
                if current_signals.len() > 5 {
                    println!("  ... ({} more branch currents)", current_signals.len() - 5);
                }
            }

            if let Some(ref output_path) = ctx.output_path_for("op") {
                write_dc_op_output(output_path, &result, ctx.format, &ctx.netlist.saves)?;
                if !ctx.quiet {
                    println!("Results exported to: {}", output_path.display());
                }
            }

            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "DC OP")),
    }
}

fn write_dc_op_output(
    path: &Path,
    result: &rspice_core::solver::SimulationResult,
    format: OutputFormat,
    saves: &rspice_core::netlist::SaveSet,
) -> Result<(), CliError> {
    use std::io::Write;

    let signals =
        crate::commands::run_signals::apply_save_set(dc_operating_point_signals(result), saves);

    if matches!(format, OutputFormat::Hdf5) {
        let mut data = Hdf5SimulationData::new();
        data.title = "DC Operating Point".to_string();

        let mut operating_point = Hdf5WaveformSection::new("point", vec![0.0]);
        for signal in &signals {
            operating_point.add_signal(signal.display_name.clone(), signal.values.clone());
        }
        data.operating_point = Some(operating_point);

        write_hdf5(path, &data).map_err(|err| map_hdf5_output_error(path, err))?;
        return Ok(());
    }

    let mut file = std::fs::File::create(path).map_err(|e| CliError::OutputError {
        path: path.to_path_buf(),
        source: e,
    })?;

    match format {
        OutputFormat::Json => {
            let mut vars = serde_json::Map::new();
            for signal in &signals {
                vars.insert(
                    signal.display_name.clone(),
                    serde_json::json!(signal.values[0]),
                );
            }
            let json = serde_json::json!({
                "analysis": "dc_op",
                "variables": vars,
            });
            writeln!(file, "{}", serde_json::to_string_pretty(&json).unwrap()).map_err(|e| {
                CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                }
            })?;
        }
        OutputFormat::Csv => {
            writeln!(file, "signal,value").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for signal in &signals {
                writeln!(file, "{},{:.9e}", signal.display_name, signal.values[0]).map_err(
                    |e| CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    },
                )?;
            }
        }
        OutputFormat::Tsv => {
            writeln!(file, "signal\tvalue").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for signal in &signals {
                writeln!(file, "{}\t{:.9e}", signal.display_name, signal.values[0]).map_err(
                    |e| CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    },
                )?;
            }
        }
        OutputFormat::Raw | OutputFormat::RawAscii => {
            writeln!(file, "Title: DC Operating Point").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "Plotname: DC OP").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "Flags: real").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "No. Variables: {}", signals.len()).map_err(|e| {
                CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                }
            })?;
            writeln!(file, "No. Points: 1").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "Variables:").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for (index, signal) in signals.iter().enumerate() {
                writeln!(
                    file,
                    "\t{}\t{}\t{}",
                    index,
                    signal.display_name,
                    signal.kind.raw_variable_type()
                )
                .map_err(|e| CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                })?;
            }
            writeln!(file, "Values:").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "0").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for signal in &signals {
                writeln!(file, "\t{:.9e}", signal.values[0]).map_err(|e| {
                    CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    }
                })?;
            }
        }
        OutputFormat::Hdf5 => unreachable!("HDF5 handled before text writers"),
    }

    Ok(())
}

pub(super) fn run_dc_sweep(
    ctx: &RunContext<'_>,
    source: &str,
    start: f64,
    stop: f64,
    step: f64,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running DC sweep on {} from {} to {} by {}...",
            source, start, stop, step
        );
    }

    match ctx
        .engine
        .run_dc_sweep(ctx.netlist, source, start, stop, step)
    {
        Ok(results) => {
            if !ctx.quiet {
                println!("DC Sweep: {} points computed", results.len());
            }

            if let Some(ref output_path) = ctx.output_path_for("dc") {
                let sweep_vals: Vec<f64> = results.iter().map(|(v, _)| *v).collect();
                let signals = crate::commands::run_signals::apply_save_set(
                    crate::commands::run_signals::dc_sweep_voltage_signals(&results),
                    &ctx.netlist.saves,
                );
                match ctx.format {
                    OutputFormat::Hdf5 => {
                        let mut data = Hdf5SimulationData::new();
                        data.title = "DC Sweep".to_string();

                        let mut dc_sweep = Hdf5WaveformSection::new(source, sweep_vals.clone());
                        for signal in &signals {
                            dc_sweep
                                .add_signal(signal.display_name.clone(), signal.values.clone());
                        }
                        data.dc_sweep = Some(dc_sweep);

                        write_hdf5(output_path, &data)
                            .map_err(|err| map_hdf5_output_error(output_path, err))?;
                    }
                    OutputFormat::Raw | OutputFormat::RawAscii => {
                        let node_names: Vec<String> = signals
                            .iter()
                            .map(|signal| signal.raw_name.clone())
                            .collect();
                        let node_waveforms: Vec<Vec<f64>> =
                            signals.iter().map(|signal| signal.values.clone()).collect();
                        rspice_core::analysis::export_dc_sweep(
                            output_path,
                            &sweep_vals,
                            source,
                            &node_names,
                            &node_waveforms,
                            raw_export_format(ctx.format),
                        )
                        .map_err(|e| CliError::OutputError {
                            path: output_path.clone(),
                            source: e,
                        })?;
                    }
                    OutputFormat::Csv | OutputFormat::Tsv | OutputFormat::Json => {
                        super::export::scalar_table(
                            "dc_sweep",
                            "DC transfer characteristic",
                            source,
                            "voltage",
                            sweep_vals,
                            &signals,
                        )
                        .write(output_path, ctx.format)?;
                    }
                }

                if !ctx.quiet {
                    println!("Results exported to: {}", output_path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "DC Sweep")),
    }
}

pub(super) fn run_transient(
    ctx: &RunContext<'_>,
    tstop: f64,
    tstep: f64,
    tstart: f64,
    max_step: Option<f64>,
) -> Result<(), CliError> {
    let internal_max_step = resolve_transient_max_step(tstep, tstop, tstart, max_step);

    let pb = if ctx.quiet {
        indicatif::ProgressBar::hidden()
    } else if ctx.show_progress {
        // The engine does not report per-step progress to the CLI yet, so
        // show honest elapsed time rather than a bar that never advances.
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Transient: {} to {} s (step {})",
            tstart, tstop, tstep
        ));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    } else {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Running transient: {} to {} (step {})...",
            tstart, tstop, tstep
        ));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    };

    let result = if ctx.compress {
        let compression_tol = ctx.compress_tol;
        let compression = rspice_core::engine::CompressionConfig {
            enabled: true,
            abs_tol: compression_tol,
            rel_tol: compression_tol,
            min_interval: tstep / 10.0,
        };
        let result =
            ctx.engine
                .run_tran_compressed(ctx.netlist, tstop, internal_max_step, compression);
        pb.finish_and_clear();
        match result {
            Ok(compressed) => {
                if !ctx.quiet {
                    println!(
                        "✓ Transient complete (compressed): {} points (compression ratio: {:.1}x)",
                        compressed.time.len(),
                        (tstop / tstep) / compressed.time.len() as f64
                    );
                }
                Ok(compressed.into())
            }
            Err(e) => Err(e),
        }
    } else {
        let result = ctx.engine.run_tran(ctx.netlist, tstop, internal_max_step);
        pb.finish_and_clear();
        result
    };

    match result {
        Ok(result) => {
            if !ctx.quiet && !ctx.compress {
                println!(
                    "✓ Transient complete: {} time points computed",
                    result.time.len()
                );
            }

            let collect_reports =
                ctx.args.meas_file.is_some() || ctx.args.report_file.is_some();
            if ctx.args.meas || collect_reports {
                // Only print measurement results to stdout when --meas was given.
                let print_quiet = ctx.quiet || !ctx.args.meas;
                let reports = run_measurements(ctx.netlist, &result, print_quiet);
                if collect_reports {
                    ctx.measurements.borrow_mut().extend(reports);
                }
            }

            if let Some(ref output_path) = ctx.output_path_for("tran") {
                let signals = crate::commands::run_signals::apply_save_set(
                    transient_voltage_signals(&result),
                    &ctx.netlist.saves,
                );
                match ctx.format {
                    OutputFormat::Hdf5 => {
                        let mut data = Hdf5SimulationData::new();
                        data.title = "Transient Analysis".to_string();

                        let mut transient = Hdf5WaveformSection::new("time", result.time.clone());
                        for signal in &signals {
                            transient
                                .add_signal(signal.display_name.clone(), signal.values.clone());
                        }
                        data.transient = Some(transient);

                        write_hdf5(output_path, &data)
                            .map_err(|err| map_hdf5_output_error(output_path, err))?;
                    }
                    OutputFormat::Raw | OutputFormat::RawAscii => {
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
                            raw_export_format(ctx.format),
                        )
                        .map_err(|e| CliError::OutputError {
                            path: output_path.clone(),
                            source: e,
                        })?;
                    }
                    OutputFormat::Csv | OutputFormat::Tsv | OutputFormat::Json => {
                        super::export::scalar_table(
                            "transient",
                            "Transient Analysis",
                            "time",
                            "time",
                            result.time.clone(),
                            &signals,
                        )
                        .write(output_path, ctx.format)?;
                    }
                }

                if !ctx.quiet {
                    println!("  Results exported to: {}", output_path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Transient")),
    }
}

pub(super) fn run_measurements(
    netlist: &Netlist,
    result: &rspice_core::engine::TransientResult,
    quiet: bool,
) -> Vec<crate::report::MeasurementReport> {
    use crate::report::MeasurementReport;

    let transient_measurements: Vec<_> = netlist
        .measurements
        .iter()
        .filter(|measurement| measurement.analysis.eq_ignore_ascii_case("TRAN"))
        .cloned()
        .collect();

    let mut signals: std::collections::HashMap<String, &[f64]> = std::collections::HashMap::new();
    insert_transient_measurement_aliases(&mut signals, result);

    let mut meas_engine = rspice_core::MeasureEngine::new();
    for measurement in &transient_measurements {
        meas_engine.add(measurement.clone());
    }
    let meas_results = meas_engine.evaluate(&result.time, &signals);

    let mut reports = Vec::new();

    if !quiet {
        if meas_results.is_empty() && transient_measurements.is_empty() {
            if netlist.measurements.is_empty() {
                println!("  No .MEAS statements found in netlist");
            } else {
                println!("  No transient .MEAS statements found in netlist");
            }
        } else {
            println!("  Measurement Results ({}):", meas_results.len());
            for mr in &meas_results {
                if let Some(value) = mr.value {
                    println!("    {} = {:.6}", mr.name, value);
                } else {
                    println!(
                        "    {} = FAILED ({})",
                        mr.name,
                        mr.error.as_ref().unwrap_or(&String::new())
                    );
                }
            }
        }
    }

    for mr in meas_results {
        reports.push(MeasurementReport {
            name: mr.name,
            value: mr.value,
            expected: None,
            tolerance: None,
            passed: mr.value.is_some(),
            error: mr.error,
        });
    }

    reports
}

pub(super) fn run_fourier(
    ctx: &RunContext<'_>,
    fundamental: f64,
    outputs: &[String],
    num_harmonics: usize,
) -> Result<(), CliError> {
    use rspice_core::analysis::{FourierAnalysis, FourierConfig};

    if !ctx.quiet {
        println!(
            "Running Fourier analysis: fundamental = {} Hz, {} harmonics",
            fundamental, num_harmonics
        );
        if ctx.verbose {
            println!("  Output nodes: {:?}", outputs);
        }
    }

    let period = 1.0 / fundamental;
    let analysis_time = period * (num_harmonics + 2) as f64;
    let tstep = period / 100.0;

    let tran_result = ctx
        .engine
        .run_tran(ctx.netlist, analysis_time, tstep)
        .map_err(|e| CliError::simulation_error_in(e.to_string(), "Fourier (transient)"))?;

    let config = FourierConfig::new(fundamental).with_harmonics(num_harmonics);
    let fourier = FourierAnalysis::new(config);
    let resolver = NodeResolver::from_netlist(ctx.engine, ctx.netlist)?;

    let mut analyzed: Vec<(String, rspice_core::analysis::FourierResult)> = Vec::new();
    for output in outputs {
        if let Some((node_idx, reference_idx)) = resolver.parse_voltage_probe(output) {
            let result = if reference_idx == 0 {
                let waveform = tran_result.voltage_waveform(node_idx);
                fourier.analyze(&tran_result.time, waveform)
            } else {
                let pos_waveform = tran_result.voltage_waveform(node_idx);
                let neg_waveform = tran_result.voltage_waveform(reference_idx);
                let diff_waveform: Vec<f64> = pos_waveform
                    .iter()
                    .zip(neg_waveform.iter())
                    .map(|(vp, vn)| vp - vn)
                    .collect();
                fourier.analyze(&tran_result.time, &diff_waveform)
            };
            analyzed.push((output.clone(), result));
        } else if !ctx.quiet {
            println!("Warning: Could not find node for output '{}'", output);
        }
    }

    if !ctx.quiet {
        println!("\n┌────────────────────────────────────────────────────────────────┐");
        println!("│                    FOURIER ANALYSIS RESULTS                    │");
        println!("├────────────────────────────────────────────────────────────────┤");

        for (output, result) in &analyzed {
            println!("│ Output: {:54} │", output);
            println!("├────────────────────────────────────────────────────────────────┤");
            println!("│  Harmonic    Frequency (Hz)    Magnitude    Phase (deg)       │");
            println!("├────────────────────────────────────────────────────────────────┤");

            for (i, harmonic) in result.harmonics.iter().enumerate() {
                let freq = fundamental * (i + 1) as f64;
                println!(
                    "│  {:3}        {:12.4e}      {:10.6}   {:10.2}         │",
                    i + 1,
                    freq,
                    harmonic.magnitude,
                    harmonic.phase.to_degrees()
                );
            }

            println!("├────────────────────────────────────────────────────────────────┤");
            println!(
                "│  DC Component:   {:10.6}                                    │",
                result.dc_component
            );
            println!(
                "│  THD:            {:10.4} %                                   │",
                result.thd * 100.0
            );
            println!("└────────────────────────────────────────────────────────────────┘");
        }
    }

    if let Some(ref output_path) = ctx.output_path_for("four") {
        write_fourier_output(
            output_path,
            ctx.format,
            fundamental,
            num_harmonics,
            &analyzed,
        )?;
        if !ctx.quiet {
            println!("\nResults written to: {}", output_path.display());
        }
    }

    Ok(())
}

/// Export Fourier results with full harmonic data (JSON or CSV).
fn write_fourier_output(
    path: &Path,
    format: OutputFormat,
    fundamental: f64,
    num_harmonics: usize,
    analyzed: &[(String, rspice_core::analysis::FourierResult)],
) -> Result<(), CliError> {
    use std::io::Write;

    let io_err = |e: std::io::Error| CliError::output_error(path, e);
    let mut file = std::fs::File::create(path).map_err(io_err)?;

    match format {
        OutputFormat::Csv | OutputFormat::Tsv => {
            let sep = if matches!(format, OutputFormat::Tsv) { '\t' } else { ',' };
            writeln!(
                file,
                "output{0}harmonic{0}frequency_hz{0}magnitude{0}phase_deg{0}dc_component{0}thd_percent",
                sep
            )
            .map_err(io_err)?;
            for (output, result) in analyzed {
                for (i, harmonic) in result.harmonics.iter().enumerate() {
                    writeln!(
                        file,
                        "{1}{0}{2}{0}{3:.9e}{0}{4:.9e}{0}{5:.6}{0}{6:.9e}{0}{7:.6}",
                        sep,
                        output,
                        i + 1,
                        fundamental * (i + 1) as f64,
                        harmonic.magnitude,
                        harmonic.phase.to_degrees(),
                        result.dc_component,
                        result.thd * 100.0,
                    )
                    .map_err(io_err)?;
                }
            }
        }
        _ => {
            // Fourier results are tables of harmonics, not waveforms; JSON is
            // the structured default for every other requested format.
            let results: Vec<serde_json::Value> = analyzed
                .iter()
                .map(|(output, result)| {
                    serde_json::json!({
                        "output": output,
                        "dc_component": result.dc_component,
                        "thd": result.thd,
                        "thd_percent": result.thd * 100.0,
                        "harmonics": result
                            .harmonics
                            .iter()
                            .enumerate()
                            .map(|(i, harmonic)| {
                                serde_json::json!({
                                    "n": i + 1,
                                    "frequency_hz": fundamental * (i + 1) as f64,
                                    "magnitude": harmonic.magnitude,
                                    "phase_deg": harmonic.phase.to_degrees(),
                                })
                            })
                            .collect::<Vec<_>>(),
                    })
                })
                .collect();

            let json = serde_json::json!({
                "analysis": "fourier",
                "fundamental_hz": fundamental,
                "num_harmonics": num_harmonics,
                "results": results,
            });
            writeln!(file, "{}", serde_json::to_string_pretty(&json).unwrap()).map_err(io_err)?;
        }
    }

    Ok(())
}

pub(super) fn run_temp(ctx: &RunContext<'_>, temperatures: &[f64]) -> Result<(), CliError> {
    if !ctx.quiet {
        println!("Running temperature sweep: {} points", temperatures.len());
        if ctx.verbose {
            println!("  Temperatures: {:?} °C", temperatures);
        }
    }

    let mut results: Vec<(f64, Vec<f64>)> = Vec::new();
    let mut node_names: Vec<String> = Vec::new();

    for (i, temp_c) in temperatures.iter().enumerate() {
        let temp_k = temp_c + 273.15;

        if !ctx.quiet {
            println!(
                "\n[{}/{}] Temperature: {:.1} °C ({:.1} K)",
                i + 1,
                temperatures.len(),
                temp_c,
                temp_k
            );
        }

        let mut temp_config = ctx.engine.config().clone();
        temp_config.temperature = temp_k;
        let temp_engine = rspice_core::Engine::new(temp_config);

        match temp_engine.run_dc_op(ctx.netlist) {
            Ok(result) => {
                if ctx.verbose && !ctx.quiet {
                    for j in 1..=result.node_voltages.len().min(5) {
                        println!("  V({}) = {:.6} V", j, result.voltage(j));
                    }
                }
                if node_names.is_empty() {
                    node_names = result.node_names.clone();
                }
                results.push((*temp_c, result.node_voltages.clone()));
            }
            Err(e) => {
                if !ctx.quiet {
                    eprintln!("  DC OP failed at {:.1} °C: {}", temp_c, e);
                }
            }
        }
    }

    if !ctx.quiet {
        println!("\n┌─────────────────────────────────────┐");
        println!("│     Temperature Sweep Summary       │");
        println!("├─────────────────────────────────────┤");
        println!(
            "│  Points:  {:3}/{:3} converged         │",
            results.len(),
            temperatures.len()
        );
        println!(
            "│  Range:   {:6.1} °C to {:6.1} °C    │",
            temperatures.first().unwrap_or(&0.0),
            temperatures.last().unwrap_or(&0.0)
        );
        println!("└─────────────────────────────────────┘");
    }

    if let Some(ref output_path) = ctx.output_path_for("temp") {
        use std::io::Write;

        let mut file = std::fs::File::create(output_path).map_err(|e| CliError::OutputError {
            path: output_path.clone(),
            source: e,
        })?;

        match ctx.format {
            OutputFormat::Csv => {
                let num_nodes = results.first().map(|(_, v)| v.len()).unwrap_or(0);
                let header: String = (1..num_nodes)
                    .map(|i| {
                        let name = node_names
                            .get(i)
                            .cloned()
                            .unwrap_or_else(|| i.to_string());
                        format!(",V({})", name)
                    })
                    .collect();
                writeln!(file, "Temperature_C{}", header).map_err(|e| CliError::OutputError {
                    path: output_path.clone(),
                    source: e,
                })?;

                for (temp, voltages) in &results {
                    // Skip index 0: ground is not a column
                    let values: String = voltages
                        .iter()
                        .skip(1)
                        .map(|v| format!(",{:.9e}", v))
                        .collect();
                    writeln!(file, "{:.2}{}", temp, values).map_err(|e| CliError::OutputError {
                        path: output_path.clone(),
                        source: e,
                    })?;
                }
            }
            OutputFormat::Json => {
                let json = serde_json::json!({
                    "analysis": "temperature_sweep",
                    "temperatures_c": temperatures,
                    "results": results.iter().map(|(t, v)| {
                        serde_json::json!({
                            "temperature_c": t,
                            "voltages": v,
                        })
                    }).collect::<Vec<_>>(),
                });
                writeln!(file, "{}", serde_json::to_string_pretty(&json).unwrap()).map_err(
                    |e| CliError::OutputError {
                        path: output_path.clone(),
                        source: e,
                    },
                )?;
            }
            _ => {
                for (temp, voltages) in &results {
                    writeln!(file, "T={:.2}C: {:?}", temp, voltages).map_err(|e| {
                        CliError::OutputError {
                            path: output_path.clone(),
                            source: e,
                        }
                    })?;
                }
            }
        }

        if !ctx.quiet {
            println!("\nResults written to: {}", output_path.display());
        }
    }

    Ok(())
}

fn raw_export_format(format: OutputFormat) -> rspice_core::analysis::RawFormat {
    match format {
        OutputFormat::RawAscii => rspice_core::analysis::RawFormat::Ascii,
        _ => rspice_core::analysis::RawFormat::Binary,
    }
}

fn resolve_transient_max_step(
    tstep: f64,
    tstop: f64,
    tstart: f64,
    explicit_max_step: Option<f64>,
) -> f64 {
    explicit_max_step
        .filter(|step| step.is_finite() && *step > 0.0)
        .unwrap_or_else(|| default_transient_max_step(tstep, tstop, tstart))
        .max(1e-18)
}

fn default_transient_max_step(tstep: f64, tstop: f64, tstart: f64) -> f64 {
    let analysis_window = tstop - tstart;
    let fallback_window = if analysis_window.is_finite() && analysis_window > 0.0 {
        analysis_window
    } else {
        tstop.abs().max(tstep.abs())
    };
    let window_limit = if fallback_window.is_finite() && fallback_window > 0.0 {
        fallback_window / 50.0
    } else {
        1e-18
    };

    if tstep.is_finite() && tstep > 0.0 && tstep < window_limit {
        tstep
    } else {
        window_limit
    }
}
