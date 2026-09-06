//! Publication of one authored `.FOUR` card's spectra.
//!
//! `.FOUR` post-processes a completed transient rather than running its own,
//! and the canonical plan names one analysis instance per resolved operand,
//! binding each to the transient it belongs to. This module publishes one
//! artifact per operand under that identity; it neither invents a transient
//! schedule nor decides which transient a card attaches to.

use super::RunContext;
use super::basic::map_output_projection_error;
use crate::cli::{CliError, OutputFormat, map_atomic_output_error};
use crate::commands::publish;
use std::path::Path;

pub(super) fn run_fourier(
    ctx: &RunContext<'_>,
    four_index: usize,
    fundamental: f64,
    num_harmonics: usize,
) -> Result<(), CliError> {
    use rspice_core::analysis::{FourierAnalysis, FourierConfig};

    if !fundamental.is_finite() || fundamental <= 0.0 {
        return Err(CliError::simulation_error_in(
            format!("invalid Fourier fundamental frequency {fundamental}"),
            "Fourier",
        ));
    }
    if num_harmonics == 0 {
        return Err(CliError::simulation_error_in(
            "Fourier harmonic count must be at least one",
            "Fourier",
        ));
    }

    if !ctx.quiet {
        println!(
            "Running Fourier analysis: fundamental = {} Hz, {} harmonics",
            fundamental, num_harmonics
        );
    }

    // The plan names this card's operands and the transient it post-processes;
    // `.FOUR` never invents an independent transient schedule of its own.
    let (parent, planned_operands) = ctx.planned_fourier_operands(four_index)?;
    let retained = ctx.retained_transient(parent)?;
    let retained = &*retained;
    // A compressed parent transient publishes a decimated waveform, so a DFT
    // over it would report different harmonics than the same deck without
    // `--compress`. The core already evaluated this card against the exact
    // accepted trajectory; consume that instead of re-analyzing.
    let analyzed: Vec<(String, &'static str, rspice_core::analysis::FourierResult)> = match retained
        .post_results
        .as_ref()
    {
        Some(post) => {
            let mut analyzed = Vec::new();
            for spectrum in post
                .fourier
                .iter()
                .filter(|spectrum| spectrum.card_index == four_index)
            {
                if spectrum.fundamental != fundamental || spectrum.harmonic_count != num_harmonics {
                    return Err(CliError::InternalError {
                        message: format!(
                            "retained Fourier spectrum for card {} was evaluated at {} Hz with {} harmonics, but the card authors {fundamental} Hz with {num_harmonics}",
                            four_index + 1,
                            spectrum.fundamental,
                            spectrum.harmonic_count
                        ),
                    });
                }
                analyzed.push((
                    spectrum.output.clone(),
                    spectrum.physical_type,
                    spectrum.spectrum.clone(),
                ));
            }
            analyzed
        }
        None => {
            let columns = rspice_core::analysis::evaluate_tran_four_output_requests_with_abort(
                ctx.netlist,
                &retained.result,
                four_index,
                ctx.engine.config().resource_limits,
                &crate::abort::ProcessAbort,
            )
            .map_err(|error| map_output_projection_error(ctx, error, "Fourier"))?;

            let config = FourierConfig::new(fundamental).with_harmonics(num_harmonics);
            let fourier = FourierAnalysis::new(config);

            let mut analyzed = Vec::new();
            analyzed.try_reserve_exact(columns.len()).map_err(|_| {
                CliError::simulation_error_in("cannot allocate Fourier results", "Fourier")
            })?;
            for (output, physical_type, waveform) in columns {
                let result = fourier
                    .analyze_with_abort(
                        &retained.result.time,
                        &waveform,
                        &crate::abort::ProcessAbort,
                    )
                    .map_err(|error| {
                        if matches!(error, rspice_core::analysis::FourierError::Aborted) {
                            super::cancellation_cli_error(ctx.args.timeout)
                        } else {
                            CliError::simulation_error_in(
                                format!("Fourier output `{output}` could not be analyzed: {error}"),
                                "Fourier",
                            )
                        }
                    })?;
                analyzed.push((output, physical_type, result));
            }
            analyzed
        }
    };

    if !ctx.quiet {
        println!("\n┌────────────────────────────────────────────────────────────────┐");
        println!("│                    FOURIER ANALYSIS RESULTS                    │");
        println!("├────────────────────────────────────────────────────────────────┤");

        for (output, physical_type, result) in &analyzed {
            println!("│ Output: {:43} ({physical_type:8}) │", output);
            println!("│ DC component = {:<47.6e} │", result.dc_component);
            println!("├────────────────────────────────────────────────────────────────┤");
            println!("│  Harmonic    Frequency (Hz)    Magnitude    Phase (deg)        │");
            println!("├────────────────────────────────────────────────────────────────┤");

            for harmonic in result.harmonics.iter().filter(|h| h.harmonic_number > 0) {
                println!(
                    "│  {:3}        {:12.4e}      {:10.6}   {:10.2}          │",
                    harmonic.harmonic_number,
                    harmonic.frequency,
                    harmonic.magnitude,
                    harmonic.phase
                );
            }

            println!("├────────────────────────────────────────────────────────────────┤");
            if let Some(thd) = result.thd {
                println!("│  THD:            {thd:10.4} %                                  │");
            } else {
                // Two columns wider than it used to be: `undefined` stands where
                // the value would, but the row carried no filler for the ` %`
                // the numeric row prints, so it fell two short of the frame.
                println!("│  THD:             undefined                                    │");
            }
            println!("└────────────────────────────────────────────────────────────────┘");
        }
    }

    // The core evaluates one spectrum per resolved operand and the shared
    // result document names one spectrum, so each operand is its own analysis
    // instance and publishes its own artifact. The plan already named those
    // instances and bound each to this transient, so the identities are read
    // off it and checked against the operands the resolver produced: a
    // mismatch would publish one operand's spectrum under another's name.
    if planned_operands.len() != analyzed.len() {
        return Err(CliError::InternalError {
            message: format!(
                "the canonical plan names {} operand(s) for .FOUR card {} of {}, but the resolver produced {}",
                planned_operands.len(),
                four_index + 1,
                retained.analysis_id,
                analyzed.len()
            ),
        });
    }
    for ((analysis_id, planned_output), (output, physical_type, result)) in
        planned_operands.into_iter().zip(&analyzed)
    {
        if planned_output != output {
            return Err(CliError::InternalError {
                message: format!(
                    "planned .FOUR operand {analysis_id} names '{planned_output}' but the resolver produced '{output}'"
                ),
            });
        }
        let Some(output_path) = ctx.output_path_for(&analysis_id.tag()) else {
            continue;
        };
        let one = [(output.clone(), *physical_type, result.clone())];
        super::document::publish_analysis_result(
            ctx,
            &output_path,
            analysis_id,
            // A Fourier spectrum is a harmonic table, not a named series set;
            // the typed harmonics live in the document's payload.
            super::document::empty_schema(),
            || {
                rspice_core::execution::AnalysisResultDocument::from_fourier(
                    analysis_id,
                    retained.analysis,
                    output,
                    fourier_output_unit(physical_type),
                    result,
                )
            },
            |path, format| {
                write_fourier_output(
                    path,
                    format,
                    &retained.analysis_id,
                    &analysis_id.tag(),
                    fundamental,
                    num_harmonics,
                    &one,
                )
            },
        )?;
        if !ctx.quiet {
            println!("\nResults written to: {}", output_path.display());
        }
    }

    Ok(())
}

/// The physical unit one Fourier spectrum's magnitudes carry.
fn fourier_output_unit(physical_type: &str) -> rspice_core::execution::SignalUnit {
    use rspice_core::execution::SignalUnit;
    match physical_type {
        "voltage" => SignalUnit::Volt,
        "current" => SignalUnit::Ampere,
        // `.FOUR` also accepts a braced expression, whose value is a bare
        // circuit parameter with no declared physical dimension.
        _ => SignalUnit::Dimensionless,
    }
}

/// Export Fourier results with full harmonic data (JSON or CSV).
fn write_fourier_output(
    path: &Path,
    format: OutputFormat,
    parent_analysis_id: &str,
    fourier_analysis_id: &str,
    fundamental: f64,
    num_harmonics: usize,
    analyzed: &[(String, &'static str, rspice_core::analysis::FourierResult)],
) -> Result<(), CliError> {
    let io_err = |e: std::io::Error| CliError::output_error(path, e);
    publish::artifact(
        path,
        |file| {
        match format {
            OutputFormat::Csv | OutputFormat::Tsv => {
                let sep = if matches!(format, OutputFormat::Tsv) {
                    '\t'
                } else {
                    ','
                };
                writeln!(
                file,
                "parent_analysis_id{0}analysis_id{0}physical_type{0}output{0}harmonic{0}frequency_hz{0}magnitude{0}phase_deg{0}dc_component{0}thd_percent",
                sep
            )
            .map_err(io_err)?;
                for (output, physical_type, result) in analyzed {
                    let thd_percent = result
                        .thd
                        .map(|value| format!("{value:.6}"))
                        .unwrap_or_default();
                    for harmonic in &result.harmonics {
                        writeln!(
                        file,
                        "{1}{0}{2}{0}{3}{0}{4}{0}{5}{0}{6:.17e}{0}{7:.17e}{0}{8:.6}{0}{9:.17e}{0}{10}",
                        sep,
                        parent_analysis_id,
                        fourier_analysis_id,
                        physical_type,
                        output,
                        harmonic.harmonic_number,
                        harmonic.frequency,
                        harmonic.magnitude,
                        harmonic.phase,
                        result.dc_component,
                        thd_percent
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
                    .map(|(output, physical_type, result)| {
                        let thd_ratio = result.thd.map(|value| value / 100.0);
                        serde_json::json!({
                            "output": output,
                            "physical_type": physical_type,
                            "dc_component": result.dc_component,
                            // Core's thd field is already a percentage.
                            "thd": thd_ratio,
                            "thd_percent": result.thd,
                            "harmonics": result
                                .harmonics
                                .iter()
                                .map(|harmonic| {
                                    serde_json::json!({
                                        "n": harmonic.harmonic_number,
                                        "frequency_hz": harmonic.frequency,
                                        "magnitude": harmonic.magnitude,
                                        "phase_deg": harmonic.phase,
                                    })
                                })
                                .collect::<Vec<_>>(),
                        })
                    })
                    .collect();

                let json = serde_json::json!({
                    "analysis": "fourier",
                    "analysis_id": fourier_analysis_id,
                    "parent_analysis_id": parent_analysis_id,
                    "fundamental_hz": fundamental,
                    "num_harmonics": num_harmonics,
                    "results": results,
                });
                let text = serde_json::to_string_pretty(&json)
                    .map_err(|e| CliError::output_json_error(path, e))?;
                writeln!(file, "{}", text).map_err(io_err)?;
            }
        }

            Ok(())
        },
    )
    .map_err(|error| map_atomic_output_error(path, error))
}
