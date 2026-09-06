//! Frequency-domain and small-signal analyses: `.AC` (including `DATA=`
//! table sweeps), `.DISTO`, `.STB`, `.NOISE` (including `DATA=`), `.PZ`,
//! `.TF`, and `.SENS`.
//!
//! `.DISTO` exports the actual sinusoidal peak phasors produced by the core's
//! third-order Volterra solver, together with each product's physical
//! frequency and explicit magnitude normalization to the F1 response. The
//! report-shaped analyses (`.TF`, `.PZ`, `.SENS`) have no natural HDF5 section
//! and reject `-f hdf5` rather than write a misleading file.

use super::RunContext;
use super::shared::{generate_frequency_sweep, map_hdf5_output_error};
use crate::cli::{CliError, OutputFormat};
use crate::commands::run_signals::{ComplexSignal, ac_signals, voltage_display_name};
use crate::hdf5::{
    Hdf5AcSection, Hdf5DistortionSection, Hdf5DistortionSeries, Hdf5DistortionSignal,
    Hdf5SimulationData, Hdf5WaveformSection, write_hdf5,
};
use crate::report::format_spice_exponent;

fn map_frequency_error(
    ctx: &RunContext<'_>,
    analysis: &str,
    source: rspice_core::SimulationError,
) -> CliError {
    if matches!(source, rspice_core::SimulationError::Aborted) {
        super::cancellation_cli_error(ctx.args.timeout)
    } else {
        CliError::CoreSimulationError {
            source,
            analysis: Some(analysis.to_string()),
        }
    }
}

/// Node lookup for analyses whose core entrypoint still accepts numerical
/// ports. Unlike the older shared resolver, construction is cancellable, so a
/// large hierarchy cannot make a frequency-analysis timeout wait for
/// elaboration to finish.
struct FrequencyNodeResolver {
    node_name_to_index: std::collections::HashMap<String, usize>,
    ground_policy: rspice_core::netlist::GroundPolicy,
}

impl FrequencyNodeResolver {
    fn from_context(ctx: &RunContext<'_>) -> Result<Self, CliError> {
        let circuit = ctx
            .engine
            .build_circuit_with_abort(ctx.netlist, &crate::abort::ProcessAbort)
            .map_err(|source| map_frequency_error(ctx, "Node Resolution", source))?;
        let node_name_to_index = circuit
            .node_names_sorted()
            .iter()
            .enumerate()
            .map(|(index, name)| (name.to_ascii_uppercase(), index + 1))
            .collect();
        Ok(Self {
            node_name_to_index,
            ground_policy: ctx.netlist.ground_policy(),
        })
    }

    fn resolve_node(&self, node: &str) -> Option<usize> {
        let node = node.trim();
        if node.is_empty() {
            return None;
        }
        if self.ground_policy.is_ground(node) {
            return Some(0);
        }
        if let Ok(index) = node.parse::<usize>() {
            return Some(index);
        }
        self.node_name_to_index
            .get(&node.to_ascii_uppercase())
            .copied()
    }
}

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
        .run_transfer_function_with_abort(
            ctx.netlist,
            output_node,
            reference_node,
            output_is_current,
            input_source,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| map_frequency_error(ctx, "Transfer Function", source))?;

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

    if let Some(output) = ctx.resolve_output("tf") {
        reject_hdf5(ctx.format, "transfer function")?;
        let analysis_id = output.analysis("tf")?;
        use super::export::{ColumnData, ExportColumn, ExportTable};

        let scalar = |name: &str, var_type: &str, value: f64| ExportColumn {
            name: name.to_string(),
            var_type: var_type.to_string(),
            data: ColumnData::Real(vec![value]),
        };
        let table = ExportTable {
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
        };
        super::document::publish_table_result(
            ctx,
            &output.path,
            analysis_id,
            // The transfer function exports three named scalars rather than a
            // series, so its typed values live in the document's payload.
            super::document::empty_schema(),
            &table,
            || {
                rspice_core::execution::AnalysisResultDocument::from_transfer_function(
                    analysis_id,
                    &result,
                )
            },
        )?;

        if !ctx.quiet {
            println!("  Transfer function exported to: {}", output.path.display());
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
    let mode = if f2_over_f1.is_some() {
        "two-tone intermodulation"
    } else {
        "single-tone harmonic"
    };
    if !ctx.quiet {
        println!(
            "Running DISTO {mode} analysis: {start_freq} to {stop_freq} Hz ({points} points)..."
        );
    }

    let frequencies = generate_frequency_sweep(variation, points, start_freq, stop_freq)?;
    let result = ctx
        .engine
        .run_distortion_with_abort(
            ctx.netlist,
            &frequencies,
            f2_over_f1,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| map_frequency_error(ctx, "DISTO", source))?;

    let projection = distortion_projection(ctx, &result)?;
    if !ctx.quiet {
        println!(
            "DISTO Analysis: {} F1 points, products: {}",
            result.points.len(),
            projection
                .series
                .iter()
                .filter(|series| series.is_product)
                .map(|series| series.label.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        if ctx.verbose {
            println!(
                "  Values are actual sinusoidal peak phasors; every non-F1 ratio is |response|/|F1| for the same signal"
            );
        }
    }

    if let Some(output) = ctx.resolve_output("disto") {
        let analysis_id = output.analysis("disto")?;
        let table = distortion_export_table(projection.clone())?;
        let schema = table_schema(&table)?;
        super::document::publish_analysis_result(
            ctx,
            &output.path,
            analysis_id,
            schema,
            || {
                rspice_core::execution::AnalysisResultDocument::from_distortion(
                    analysis_id,
                    &result,
                )
            },
            |path, format| {
                if matches!(format, OutputFormat::Hdf5) {
                    let mut data = Hdf5SimulationData::new();
                    data.title = "Volterra Distortion Analysis".to_string();
                    data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);
                    data.distortion = Some(distortion_hdf5_section(&projection)?);
                    write_hdf5(path, &data).map_err(|error| map_hdf5_output_error(path, error))
                } else {
                    table.write(path, format)
                }
            },
        )?;
        if !ctx.quiet {
            println!(
                "  Volterra distortion products exported to: {}",
                output.path.display()
            );
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct DistortionSeries {
    label: String,
    is_product: bool,
    physical_frequencies: Vec<f64>,
    signals: Vec<ComplexSignal>,
}

#[derive(Debug, Clone)]
struct DistortionProjection {
    f2_over_f1: Option<f64>,
    f1_frequencies: Vec<f64>,
    series: Vec<DistortionSeries>,
}

fn distortion_projection(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::DistortionAnalysisResult,
) -> Result<DistortionProjection, CliError> {
    use rspice_core::analysis::{AcResult, DistortionProduct};

    if result.points.is_empty() {
        return Err(distortion_schema_error(
            "the core returned no F1 result points".to_string(),
        ));
    }

    let f1_rows: Vec<AcResult> = result
        .points
        .iter()
        .map(|point| point.fundamental_f1.clone())
        .collect();
    let f1_frequencies: Vec<f64> = f1_rows.iter().map(|row| row.frequency).collect();
    let mut series = vec![distortion_series(ctx, "f1", false, f1_rows)?];

    if result.is_two_tone() {
        let f2_rows = result
            .points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                point.fundamental_f2.clone().ok_or_else(|| {
                    distortion_schema_error(format!(
                        "missing F2 fundamental response at F1 point {index}"
                    ))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        series.push(distortion_series(ctx, "f2", false, f2_rows)?);
    }

    let product_kinds: &[DistortionProduct] = if result.is_two_tone() {
        &[
            DistortionProduct::Sum,
            DistortionProduct::Difference,
            DistortionProduct::ThirdOrderDifference,
        ]
    } else {
        &[
            DistortionProduct::SecondHarmonic,
            DistortionProduct::ThirdHarmonic,
        ]
    };
    for &product in product_kinds {
        let rows = result
            .points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                point
                    .product(product)
                    .map(|value| value.response.clone())
                    .ok_or_else(|| {
                        distortion_schema_error(format!(
                            "missing '{}' response at F1 point {index}",
                            product.label()
                        ))
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        series.push(distortion_series(ctx, product.label(), true, rows)?);
    }

    let sample_count = f1_frequencies.len();
    if series.iter().any(|value| {
        value.physical_frequencies.len() != sample_count
            || value.signals.iter().any(|signal| {
                signal.real.len() != sample_count || signal.imag.len() != sample_count
            })
    }) {
        return Err(distortion_schema_error(
            "a projected series length does not match the F1 sweep".to_string(),
        ));
    }

    Ok(DistortionProjection {
        f2_over_f1: result.f2_over_f1,
        f1_frequencies,
        series,
    })
}

fn distortion_series(
    ctx: &RunContext<'_>,
    label: &str,
    is_product: bool,
    rows: Vec<rspice_core::analysis::AcResult>,
) -> Result<DistortionSeries, CliError> {
    let physical_frequencies: Vec<f64> = rows.iter().map(|row| row.frequency).collect();
    let signals = ac_signals(&rows).map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some(format!("DISTO {label} output projection")),
    })?;
    let signals = crate::commands::run_signals::complex_export_signals(
        ctx.netlist,
        rspice_core::execution::AnalysisResultKind::Distortion,
        "DISTO",
        &physical_frequencies,
        &signals,
        &crate::abort::ProcessAbort,
    )
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some(format!("DISTO {label} output projection")),
    })?;

    if !ctx.args.allow_nonfinite {
        for signal in &signals {
            for (index, (&real, &imag)) in signal.real.iter().zip(&signal.imag).enumerate() {
                if !real.is_finite() || !imag.is_finite() {
                    return Err(CliError::SimulationError {
                        message: format!(
                            "{label}:{} is non-finite at F1 point {index}; the distortion response is not physical. Use --allow-nonfinite to export anyway.",
                            signal.display_name
                        ),
                        analysis: Some("DISTO".to_string()),
                    });
                }
            }
        }
    }

    Ok(DistortionSeries {
        label: label.to_string(),
        is_product,
        physical_frequencies,
        signals,
    })
}

fn distortion_schema_error(message: String) -> CliError {
    CliError::CoreSimulationError {
        source: rspice_core::SimulationError::Circuit(format!(
            "malformed distortion result: {message}"
        )),
        analysis: Some("DISTO output projection".to_string()),
    }
}

fn distortion_export_table(
    projection: DistortionProjection,
) -> Result<super::export::ExportTable, CliError> {
    use super::export::{ColumnData, ExportColumn, ExportTable};

    let f1_signals = projection
        .series
        .first()
        .map(|series| series.signals.clone())
        .unwrap_or_default();
    let mut columns = Vec::new();
    if let Some(ratio) = projection.f2_over_f1 {
        columns.push(ExportColumn {
            name: "f2_over_f1".to_string(),
            var_type: "ratio".to_string(),
            data: ColumnData::Real(vec![ratio; projection.f1_frequencies.len()]),
        });
    }

    for series in projection.series {
        if series.label != "f1" {
            columns.push(ExportColumn {
                name: format!("frequency({})", series.label),
                var_type: "frequency".to_string(),
                data: ColumnData::Real(series.physical_frequencies),
            });
        }

        for signal in series.signals {
            let phasor_name = format!("peak({}:{})", series.label, signal.display_name);
            let magnitudes: Vec<f64> = signal
                .real
                .iter()
                .zip(&signal.imag)
                .map(|(&real, &imag)| real.hypot(imag))
                .collect();
            let phases: Vec<f64> = signal
                .real
                .iter()
                .zip(&signal.imag)
                .map(|(&real, &imag)| imag.atan2(real).to_degrees())
                .collect();
            let ratios = if series.label == "f1" {
                None
            } else {
                Some(distortion_magnitude_ratio(
                    &f1_signals,
                    &series.label,
                    &signal,
                    &magnitudes,
                )?)
            };
            let var_type = signal.raw_variable_type().to_string();
            columns.push(ExportColumn {
                name: phasor_name,
                var_type: var_type.clone(),
                data: ColumnData::Complex {
                    real: signal.real,
                    imag: signal.imag,
                },
            });
            columns.push(ExportColumn {
                name: format!("magnitude({}:{})", series.label, signal.display_name),
                var_type,
                data: ColumnData::Real(magnitudes.clone()),
            });
            columns.push(ExportColumn {
                name: format!("phase_deg({}:{})", series.label, signal.display_name),
                var_type: "phase".to_string(),
                data: ColumnData::Real(phases),
            });

            if let Some(ratios) = ratios {
                columns.push(ExportColumn {
                    name: format!(
                        "magnitude_ratio_to_f1({}:{})",
                        series.label, signal.display_name
                    ),
                    var_type: "ratio".to_string(),
                    data: ColumnData::Real(ratios),
                });
            }
        }
    }

    Ok(ExportTable {
        analysis: "disto".to_string(),
        plot_name: "Volterra Distortion Analysis".to_string(),
        scale_name: "frequency(f1)".to_string(),
        scale_type: "frequency".to_string(),
        scale: projection.f1_frequencies,
        columns,
    })
}

fn distortion_hdf5_section(
    projection: &DistortionProjection,
) -> Result<Hdf5DistortionSection, CliError> {
    let f1_signals = projection
        .series
        .first()
        .map(|series| series.signals.as_slice())
        .unwrap_or_default();
    let mut hdf5_series = Vec::with_capacity(projection.series.len());

    for series in &projection.series {
        let mut signals = Vec::with_capacity(series.signals.len());
        for signal in &series.signals {
            let magnitude: Vec<f64> = signal
                .real
                .iter()
                .zip(&signal.imag)
                .map(|(&real, &imag)| real.hypot(imag))
                .collect();
            let phase_degrees = signal
                .real
                .iter()
                .zip(&signal.imag)
                .map(|(&real, &imag)| imag.atan2(real).to_degrees())
                .collect();
            let magnitude_ratio_to_f1 = if series.label == "f1" {
                None
            } else {
                Some(distortion_magnitude_ratio(
                    f1_signals,
                    &series.label,
                    signal,
                    &magnitude,
                )?)
            };
            signals.push(Hdf5DistortionSignal {
                name: signal.display_name.clone(),
                var_type: signal.raw_variable_type().to_string(),
                real: signal.real.clone(),
                imag: signal.imag.clone(),
                magnitude,
                phase_degrees,
                magnitude_ratio_to_f1,
            });
        }
        hdf5_series.push(Hdf5DistortionSeries {
            label: series.label.clone(),
            is_product: series.is_product,
            physical_frequency: series.physical_frequencies.clone(),
            signals,
        });
    }

    Ok(Hdf5DistortionSection {
        mode: if projection.f2_over_f1.is_some() {
            "two_tone".to_string()
        } else {
            "harmonic".to_string()
        },
        f2_over_f1: projection.f2_over_f1,
        phasor_convention: "actual_sinusoidal_peak".to_string(),
        ratio_normalization: "magnitude_over_same_signal_f1_magnitude".to_string(),
        f1_frequency: projection.f1_frequencies.clone(),
        series: hdf5_series,
    })
}

fn distortion_magnitude_ratio(
    f1_signals: &[ComplexSignal],
    series_label: &str,
    signal: &ComplexSignal,
    numerator_magnitudes: &[f64],
) -> Result<Vec<f64>, CliError> {
    let fundamental = f1_signals
        .iter()
        .find(|fundamental| {
            fundamental.kind == signal.kind
                && fundamental.raw_name.eq_ignore_ascii_case(&signal.raw_name)
        })
        .ok_or_else(|| {
            distortion_schema_error(format!(
                "{series_label} response has signal '{}' but the F1 response does not",
                signal.display_name
            ))
        })?;
    if fundamental.real.len() != numerator_magnitudes.len()
        || fundamental.imag.len() != numerator_magnitudes.len()
    {
        return Err(distortion_schema_error(format!(
            "{series_label} response signal '{}' does not align with its F1 normalization series",
            signal.display_name
        )));
    }

    numerator_magnitudes
        .iter()
        .zip(fundamental.real.iter().zip(&fundamental.imag))
        .enumerate()
        .map(|(index, (&numerator, (&real, &imag)))| {
            let denominator = real.hypot(imag);
            if denominator == 0.0 {
                Err(distortion_schema_error(format!(
                    "cannot normalize {series_label} signal '{}' at F1 point {index}: the F1 magnitude is zero (response magnitude {numerator:.17e}); no finite magnitude ratio exists",
                    signal.display_name
                )))
            } else {
                Ok(numerator / denominator)
            }
        })
        .collect()
}

pub(super) fn run_ac_data(ctx: &RunContext<'_>, table_name: &str) -> Result<(), CliError> {
    let points = ctx
        .netlist
        .frequency_data_table_points(table_name)
        .map_err(|error| invalid_ac_data(format!(".AC DATA {error}")))?;

    if !ctx.quiet {
        println!(
            "Running AC DATA analysis from table {} ({} points)...",
            table_name,
            points.len()
        );
    }

    let (_row_netlists, results) = ctx
        .engine
        .run_ac_data_with_abort(ctx.netlist, table_name, &crate::abort::ProcessAbort)
        .map_err(|source| map_frequency_error(ctx, "AC DATA", source))?;
    finish_ac_results(ctx, results)
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

    let frequencies = generate_frequency_sweep(variation, points, start_freq, stop_freq)?;
    run_ac_frequencies(ctx, frequencies)
}

fn run_ac_frequencies(ctx: &RunContext<'_>, frequencies: Vec<f64>) -> Result<(), CliError> {
    let results = ctx
        .engine
        .run_ac_with_abort(ctx.netlist, &frequencies, &crate::abort::ProcessAbort)
        .map_err(|source| map_frequency_error(ctx, "AC", source))?;
    finish_ac_results(ctx, results)
}

fn finish_ac_results(
    ctx: &RunContext<'_>,
    results: Vec<rspice_core::analysis::AcResult>,
) -> Result<(), CliError> {
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

    let measurements = rspice_core::analysis::evaluate_ac_measurements_with_abort(
        ctx.netlist,
        &results,
        &crate::abort::ProcessAbort,
    )
    .map_err(|source| CliError::CoreSimulationError {
        source,
        analysis: Some("AC measurement projection".to_string()),
    })?;
    ctx.record_measurements("AC", measurements);
    let continuous_measurements =
        rspice_core::analysis::evaluate_ac_continuous_measurements(ctx.netlist, &results);
    super::shared::record_continuous_measurements(ctx, "AC_CONT", continuous_measurements);

    if !ctx.quiet {
        println!("AC Analysis: {} frequency points", results.len());
        if ctx.verbose
            && let (Some(first), Some(last)) = (results.first(), results.last())
        {
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

    // Projection is part of executing an authored output contract, not merely
    // serialization. Validate it even when the caller did not request a file
    // so `.SAVE`/`.PRINT` can never be reported as successful while skipped.
    let frequencies: Vec<f64> = results.iter().map(|result| result.frequency).collect();
    let ac_projection_error = |source| CliError::CoreSimulationError {
        source,
        analysis: Some("AC output projection".to_string()),
    };
    let inventory = ac_signals(&results).map_err(ac_projection_error)?;
    let signals = crate::commands::run_signals::complex_export_signals(
        ctx.netlist,
        rspice_core::execution::AnalysisResultKind::Ac,
        "AC",
        &frequencies,
        &inventory,
        &crate::abort::ProcessAbort,
    )
    .map_err(ac_projection_error)?;

    if let Some(output) = ctx.resolve_output("ac") {
        let analysis_id = output.analysis("ac")?;
        super::document::publish_analysis_result(
            ctx,
            &output.path,
            analysis_id,
            super::document::complex_schema(&signals)?,
            || rspice_core::execution::AnalysisResultDocument::from_ac(analysis_id, &results),
            |path, format| {
                if matches!(format, OutputFormat::Hdf5) {
                    let mut data = Hdf5SimulationData::new();
                    data.title = "AC Analysis".to_string();
                    data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);

                    let mut ac = Hdf5AcSection::new(frequencies.clone());
                    for signal in &signals {
                        ac.add_signal(
                            signal.display_name.clone(),
                            signal.real.clone(),
                            signal.imag.clone(),
                        );
                    }
                    data.ac = Some(ac);

                    write_hdf5(path, &data).map_err(|err| map_hdf5_output_error(path, err))
                } else {
                    super::export::complex_table("ac", "AC Analysis", frequencies.clone(), &signals)
                        .write(path, format)
                }
            },
        )?;
        if !ctx.quiet {
            println!("  AC response exported to: {}", output.path.display());
        }
    }
    Ok(())
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
    use rspice_core::analysis::{StbConfig, StbSweepType};
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
        .run_stb_with_abort(ctx.netlist, config, &crate::abort::ProcessAbort)
        .map_err(|source| map_frequency_error(ctx, "STB", source))?;

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

    if let Some(output) = ctx.resolve_output("stb") {
        let analysis_id = output.analysis("stb")?;
        use super::export::{ColumnData, ExportColumn, ExportTable};

        let table = ExportTable {
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
        };
        let schema = table_schema(&table)?;
        super::document::publish_analysis_result(
            ctx,
            &output.path,
            analysis_id,
            schema,
            || {
                rspice_core::execution::AnalysisResultDocument::from_stability(
                    analysis_id,
                    &stb.result,
                )
            },
            |path, format| {
                if matches!(format, OutputFormat::Hdf5) {
                    let mut data = Hdf5SimulationData::new();
                    data.title = "STB Loop Gain".to_string();
                    data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);

                    let mut ac = Hdf5AcSection::new(stb.frequencies.clone());
                    ac.add_signal(
                        "loopgain".to_string(),
                        stb.loop_gains.iter().map(|g| g.re).collect(),
                        stb.loop_gains.iter().map(|g| g.im).collect(),
                    );
                    data.ac = Some(ac);

                    write_hdf5(path, &data).map_err(|err| map_hdf5_output_error(path, err))
                } else {
                    table.write(path, format)
                }
            },
        )?;
        if !ctx.quiet {
            println!("  Loop gain exported to: {}", output.path.display());
        }
    }

    Ok(())
}

/// The schema of a flat artifact assembled column by column rather than from
/// a projected signal list: each column keeps its exported name and value type.
fn table_schema(
    table: &super::export::ExportTable,
) -> Result<rspice_core::execution::SignalSchema, CliError> {
    super::document::distinct_schema(table.columns.iter().map(|column| {
        rspice_core::execution::signal_descriptor(
            &column.name,
            &column.name,
            rspice_core::execution::SignalKind::Scalar,
            if matches!(column.data, super::export::ColumnData::Complex { .. }) {
                rspice_core::execution::SignalValueType::Complex
            } else {
                rspice_core::execution::SignalValueType::Real
            },
        )
    }))
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

    let frequencies = generate_frequency_sweep(variation, points, start_freq, stop_freq)?;
    let execution = ctx.engine.run_noise_named_with_input_source_and_abort(
        ctx.netlist,
        output_node,
        reference_node,
        input_source,
        &frequencies,
        ctx.engine.config().temperature,
        &crate::abort::ProcessAbort,
    );
    finish_noise(
        ctx,
        output_node,
        reference_node,
        input_source,
        execution,
        true,
    )
}

pub(super) fn run_noise_data(
    ctx: &RunContext<'_>,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    table_name: &str,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!("Running Noise DATA analysis from table {table_name}...");
    }
    let execution = ctx
        .engine
        .run_noise_data_named_with_input_source_and_abort(
            ctx.netlist,
            output_node,
            reference_node,
            input_source,
            table_name,
            ctx.engine.config().temperature,
            &crate::abort::ProcessAbort,
        )
        .map(|(_, results)| results);
    let integrate = execution.as_ref().is_ok_and(|results| {
        results
            .windows(2)
            .all(|pair| pair[1].frequency > pair[0].frequency)
    });
    finish_noise(
        ctx,
        output_node,
        reference_node,
        input_source,
        execution,
        integrate,
    )
}

fn finish_noise(
    ctx: &RunContext<'_>,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    execution: Result<Vec<rspice_core::analysis::NoiseResult>, rspice_core::SimulationError>,
    integrate: bool,
) -> Result<(), CliError> {
    match execution {
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

            let measurements = rspice_core::analysis::evaluate_noise_measurements_with_abort(
                ctx.netlist,
                &results,
                &crate::abort::ProcessAbort,
            )
            .map_err(|source| CliError::CoreSimulationError {
                source,
                analysis: Some("Noise measurement projection".to_string()),
            })?;
            ctx.record_measurements("NOISE", measurements);

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

                if integrate {
                    print_noise_contribution_summary(&results, ctx.verbose);
                } else {
                    println!(
                        "  Total-noise integration disabled: DATA frequencies are not strictly increasing"
                    );
                }
            }

            if let Some(output) = ctx.resolve_output("noise") {
                let analysis_id = output.analysis("noise")?;
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

                use super::export::{ColumnData, ExportColumn, ExportTable};

                let table = ExportTable {
                    analysis: "noise".to_string(),
                    plot_name: "Noise Spectral Density Curves".to_string(),
                    scale_name: "frequency".to_string(),
                    scale_type: "frequency".to_string(),
                    scale: noise_frequencies.clone(),
                    columns: vec![
                        ExportColumn {
                            name: "onoise_spectrum".to_string(),
                            var_type: "voltage".to_string(),
                            data: ColumnData::Real(onoise.clone()),
                        },
                        ExportColumn {
                            name: "inoise_spectrum".to_string(),
                            var_type: "voltage".to_string(),
                            data: ColumnData::Real(inoise.clone()),
                        },
                    ],
                };
                let schema = table_schema(&table)?;
                super::document::publish_analysis_result(
                    ctx,
                    &output.path,
                    analysis_id,
                    schema,
                    || {
                        rspice_core::execution::AnalysisResultDocument::from_noise(
                            analysis_id,
                            &results,
                        )
                    },
                    |path, format| {
                        if matches!(format, OutputFormat::Hdf5) {
                            let mut data = Hdf5SimulationData::new();
                            data.title = "Noise Analysis".to_string();
                            data.identity = Some(super::document::hdf5_identity(ctx, analysis_id)?);

                            let mut noise =
                                Hdf5WaveformSection::new("frequency", noise_frequencies.clone());
                            noise.add_signal("onoise_spectrum", onoise.clone());
                            noise.add_signal("inoise_spectrum", inoise.clone());
                            data.noise = Some(noise);

                            write_hdf5(path, &data).map_err(|err| map_hdf5_output_error(path, err))
                        } else {
                            table.write(path, format)
                        }
                    },
                )?;

                if !ctx.quiet {
                    println!("  Noise spectra exported to: {}", output.path.display());
                }
            }
            Ok(())
        }
        Err(source) => Err(map_frequency_error(ctx, "Noise", source)),
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
    input_is_current: bool,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running Pole-Zero analysis: input=node {} ({}), output=node {}",
            input_node,
            if input_is_current {
                "current"
            } else {
                "voltage"
            },
            output_node
        );
    }

    match ctx.engine.run_pz_ports_with_abort(
        ctx.netlist,
        input_node,
        None,
        output_node,
        None,
        input_is_current,
        true,
        true,
        &crate::abort::ProcessAbort,
    ) {
        Ok(result) => {
            report_pz(ctx, &result)?;
            Ok(())
        }
        Err(source) => Err(map_frequency_error(ctx, "Pole-Zero", source)),
    }
}

/// Print the pole/zero summary and export the singularities.
///
/// The export follows the rawfile convention for .PZ results: a single
/// point with one complex variable per pole/zero (`pole(1)`, `zero(1)`, ...).
fn report_pz(
    ctx: &RunContext<'_>,
    result: &rspice_core::analysis::PoleZeroResult,
) -> Result<(), CliError> {
    let poles = result.poles.as_slice();
    let zeros = result.zeros.as_slice();
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

    if let Some(output) = ctx.resolve_output("pz") {
        reject_hdf5(ctx.format, "pole-zero")?;
        let analysis_id = output.analysis("pz")?;
        use super::export::{ColumnData, ExportColumn, ExportTable};

        let singularity =
            |label: &str, index: usize, value: &rspice_core::Complex64| ExportColumn {
                name: format!("{label}({})", index + 1),
                var_type: "frequency".to_string(),
                data: ColumnData::Complex {
                    real: vec![value.re],
                    imag: vec![value.im],
                },
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

        let table = ExportTable {
            analysis: "pz".to_string(),
            plot_name: "Pole-Zero Analysis".to_string(),
            scale_name: "point".to_string(),
            scale_type: "index".to_string(),
            scale: vec![0.0],
            columns,
        };
        let schema = table_schema(&table)?;
        super::document::publish_table_result(
            ctx,
            &output.path,
            analysis_id,
            schema,
            &table,
            || rspice_core::execution::AnalysisResultDocument::from_pole_zero(analysis_id, result),
        )?;

        if !ctx.quiet {
            println!("  Poles/zeros exported to: {}", output.path.display());
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
    let resolver = FrequencyNodeResolver::from_context(ctx)?;

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

    match ctx.engine.run_pz_ports_with_abort(
        ctx.netlist,
        in_pos,
        Some(in_neg),
        out_pos,
        Some(out_neg),
        input_is_current,
        compute_poles,
        compute_zeros,
        &crate::abort::ProcessAbort,
    ) {
        Ok(result) => {
            report_pz(ctx, &result)?;
            Ok(())
        }
        Err(source) => Err(map_frequency_error(ctx, "Pole-Zero", source)),
    }
}

pub(super) fn run_sensitivity(
    ctx: &RunContext<'_>,
    output_node: usize,
    output_name: &str,
    param_name: &str,
    param_value: f64,
) -> Result<(), CliError> {
    if !ctx.quiet {
        println!(
            "Running Sensitivity analysis: ∂V({})/∂{} at {}={:.6e}",
            output_name, param_name, param_name, param_value
        );
    }

    match ctx.engine.run_sensitivity_with_abort(
        ctx.netlist,
        output_node,
        param_name,
        param_value,
        None,
        &crate::abort::ProcessAbort,
    ) {
        Ok(sensitivity) => {
            if !ctx.quiet {
                println!("✓ Sensitivity analysis complete");
                println!(
                    "  ∂V({})/∂{} = {:.6e} V/unit",
                    output_name, param_name, sensitivity
                );

                if ctx.verbose {
                    let nominal_sens = sensitivity * param_value;
                    println!(
                        "  Normalized: {:.2}% change per 1% parameter variation",
                        nominal_sens * 100.0
                    );
                }
            }

            export_parameter_sensitivity(
                ctx,
                output_node,
                output_name,
                param_name,
                param_value,
                sensitivity,
            )?;
            Ok(())
        }
        Err(source) => Err(map_frequency_error(ctx, "Sensitivity", source)),
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
    let resolver = FrequencyNodeResolver::from_context(ctx)?;
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
    let output = if output_is_current {
        rspice_core::analysis::AcSensitivityOutput::BranchCurrent(output_node.to_string())
    } else {
        rspice_core::analysis::AcSensitivityOutput::Voltage {
            positive: out_pos,
            negative: (out_neg != 0).then_some(out_neg),
        }
    };
    let output_label = if output_is_current {
        format!("I({output_node})")
    } else {
        reference_node.map_or_else(
            || format!("V({output_node})"),
            |reference| format!("V({output_node},{reference})"),
        )
    };
    let output_unit = if output_is_current { "A" } else { "V" };

    if let Some(ac) = ac_sweep {
        let freqs = generate_frequency_sweep(ac.variation, ac.points, ac.start_freq, ac.stop_freq)?;

        if !ctx.quiet {
            println!(
                "Running AC Sensitivity analysis: {} over {} frequencies",
                output_label,
                freqs.len()
            );
        }

        let result = ctx
            .engine
            .run_sensitivity_ac_complete_with_abort(
                ctx.netlist,
                output,
                &freqs,
                filters,
                &crate::abort::ProcessAbort,
            )
            .map_err(|source| map_frequency_error(ctx, "Sensitivity AC", source))?;

        for trace in &result.sensitivities {
            let combined = &trace.magnitude;

            if !ctx.quiet {
                let first = combined.first().copied().unwrap_or(0.0);
                let last = combined.last().copied().unwrap_or(0.0);
                println!(
                    "  d|{}|/d{}: {:.6e} {} per native parameter unit @ {:e} Hz, {:.6e} @ {:e} Hz",
                    output_label,
                    trace.vector_name,
                    first,
                    output_unit,
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
                println!(
                    "    peak |d|{}|/d{}| = {:.6e} {} per native parameter unit",
                    output_label, trace.vector_name, peak, output_unit
                );
            }
        }

        if let Some(resolved) = ctx.resolve_output("sens") {
            reject_hdf5(ctx.format, "sensitivity")?;
            let analysis_id = resolved.analysis("sens")?;
            use super::export::{ColumnData, ExportColumn, ExportTable};

            let table = ExportTable {
                analysis: "sens_ac".to_string(),
                plot_name: "AC Sensitivity".to_string(),
                scale_name: "frequency".to_string(),
                scale_type: "frequency".to_string(),
                scale: freqs.clone(),
                columns: result
                    .sensitivities
                    .iter()
                    .map(|trace| ExportColumn {
                        name: format!("d{}/d({})", output_label, trace.vector_name),
                        var_type: "sensitivity".to_string(),
                        data: ColumnData::Complex {
                            real: trace.absolute.iter().map(|value| value.re).collect(),
                            imag: trace.absolute.iter().map(|value| value.im).collect(),
                        },
                    })
                    .collect(),
            };
            let schema = table_schema(&table)?;
            // The shared sensitivity payload carries a complex derivative
            // trace per parameter beside the operating-point derivatives, so
            // an AC sweep publishes the same typed document a DC card does;
            // the flat table remains the projection for the other formats.
            super::document::publish_table_result(
                ctx,
                &resolved.path,
                analysis_id,
                schema,
                &table,
                || {
                    rspice_core::execution::AnalysisResultDocument::from_ac_sensitivity(
                        analysis_id,
                        &result,
                    )
                },
            )?;

            if !ctx.quiet {
                println!("  Sensitivities exported to: {}", resolved.path.display());
            }
        }

        return Ok(());
    }

    if !ctx.quiet {
        println!("Running DC Sensitivity analysis: {output_label}");
    }

    let result = ctx
        .engine
        .run_sensitivity_dc_complete_with_abort(
            ctx.netlist,
            output,
            filters,
            &crate::abort::ProcessAbort,
        )
        .map_err(|source| map_frequency_error(ctx, "Sensitivity", source))?;
    let complete = result.clone();
    let mut sensitivities = result.sensitivities;

    sensitivities.sort_by(|a, b| {
        b.absolute
            .abs()
            .partial_cmp(&a.absolute.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if !ctx.quiet {
        println!("✓ Sensitivity analysis complete");
        for sensitivity in &sensitivities {
            println!(
                "  ∂{}/∂{} = {:.6e} {} per native parameter unit (normalized: {:.6e})",
                output_label,
                sensitivity.vector_name,
                sensitivity.absolute,
                output_unit,
                sensitivity.normalized
            );
        }
    }

    let results = sensitivities
        .iter()
        .map(|sensitivity| (sensitivity.vector_name.clone(), sensitivity.absolute))
        .collect::<Vec<_>>();
    export_dc_sensitivity_result(ctx, &output_label, &complete, &results)?;
    Ok(())
}

/// Write the single derivative the `--sens-*` command-line probe computed.
///
/// The probe differentiates the output with respect to one netlist parameter
/// rather than one device instance parameter, which is what the shared payload
/// declares as a parameter sensitivity: the parameter's own nominal value, the
/// absolute derivative, and the normalized derivative against the operating
/// point the same deck settles at. The operating point is solved here because
/// the probe's own two perturbed solves deliberately do not report it.
fn export_parameter_sensitivity(
    ctx: &RunContext<'_>,
    output_node: usize,
    output_name: &str,
    param_name: &str,
    param_value: f64,
    absolute: f64,
) -> Result<(), CliError> {
    let output_label = format!("V({output_name})");
    let results = [(param_name.to_string(), absolute)];
    let Some(resolved) = ctx.resolve_output("sens") else {
        return Ok(());
    };
    reject_hdf5(ctx.format, "sensitivity")?;
    let analysis_id = resolved.analysis("sens")?;
    let table = dc_sensitivity_table(&output_label, &results);
    let schema = table_schema(&table)?;
    super::document::publish_table_result(
        ctx,
        &resolved.path,
        analysis_id,
        schema,
        &table,
        || {
            // Only invoked for the typed representation, so the nominal
            // operating point is solved only when a consumer asked for a
            // document that declares it.
            let nominal = ctx
                .engine
                .run_dc_op_with_abort(ctx.netlist, &crate::abort::ProcessAbort)
                .map_err(
                    |error| rspice_core::execution::ResultDocumentError::SourceResult {
                        location: "parameter sensitivity result",
                        detail: format!(
                            "the nominal operating point the normalized sensitivity is taken \
                             against could not be solved: {error}"
                        ),
                    },
                )?;
            let output_value = nominal.try_voltage(output_node).ok_or(
                rspice_core::execution::ResultDocumentError::SourceResult {
                    location: "parameter sensitivity result",
                    detail: format!("node {output_node} carries no operating-point voltage"),
                },
            )?;
            rspice_core::execution::AnalysisResultDocument::from_parameter_sensitivity(
                analysis_id,
                &output_label,
                output_value,
                param_name,
                param_value,
                absolute,
            )
        },
    )?;

    if !ctx.quiet {
        println!("  Sensitivities exported to: {}", resolved.path.display());
    }
    Ok(())
}

/// The flat DC sensitivity table: one row, one column per derivative, each
/// named by the selected probe identity (`dV(out)/d(R1)`, `dI(V1)/d(R1)`).
fn dc_sensitivity_table(output: &str, results: &[(String, f64)]) -> super::export::ExportTable {
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
                name: format!("d{output}/d({name})"),
                var_type: "sensitivity".to_string(),
                data: ColumnData::Real(vec![*value]),
            })
            .collect(),
    }
}

/// Write a complete `.SENS` result.
fn export_dc_sensitivity_result(
    ctx: &RunContext<'_>,
    output: &str,
    result: &rspice_core::analysis::SensitivityResult,
    results: &[(String, f64)],
) -> Result<(), CliError> {
    let Some(resolved) = ctx.resolve_output("sens") else {
        return Ok(());
    };
    reject_hdf5(ctx.format, "sensitivity")?;
    let analysis_id = resolved.analysis("sens")?;
    let table = dc_sensitivity_table(output, results);
    let schema = table_schema(&table)?;
    super::document::publish_table_result(
        ctx,
        &resolved.path,
        analysis_id,
        schema,
        &table,
        || rspice_core::execution::AnalysisResultDocument::from_sensitivity(analysis_id, result),
    )?;

    if !ctx.quiet {
        println!("  Sensitivities exported to: {}", resolved.path.display());
    }
    Ok(())
}
