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
use rspice_core::Value;
use rspice_core::analysis::HarmonicComponent;
use std::path::Path;

/// One analyzed operand of a `.FOUR` card: the authored output name, the
/// physical type it resolved to, and the spectrum evaluated for it.
type AnalyzedSpectrum = (String, &'static str, rspice_core::analysis::FourierResult);

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
    let analyzed: Vec<AnalyzedSpectrum> = match retained.post_results.as_ref() {
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
        println!();
        render_fourier_frame(&analyzed, &mut |line| println!("{line}"));
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

/// Columns inside the Fourier results frame, between its two borders, when the
/// harmonic column is at its narrowest.
const FOURIER_FRAME_INTERIOR: usize = 64;

/// Columns the harmonic index is printed in unless the document needs more.
///
/// Three digits carries every harmonic count a `.FOUR` card usually authors,
/// and it is the width the frame was drawn around.
const FOURIER_HARMONIC_WIDTH: usize = 3;

/// Columns the analyzed output's authored name is printed in at that width.
const FOURIER_OUTPUT_WIDTH: usize = 42;

/// Columns the DC component is printed in at that width.
const FOURIER_DC_WIDTH: usize = 47;

/// Columns the physical type is printed in.
///
/// `parameter` — what a braced `.FOUR` expression is — is nine characters, and
/// the eight this field used to be is what pushed that row's border out.
const FOURIER_PHYSICAL_TYPE_WIDTH: usize = 9;

/// Columns this document's harmonic index needs.
///
/// A `.FOUR` card authors its own harmonic count and nothing in the card bounds
/// it below a thousand, so the index column is sized to the largest index the
/// document actually carries. Cutting an index would misname a harmonic, and
/// letting one run past its column would carry that row's border out of the
/// frame, so the frame is widened instead — by this much on every row at once.
/// The width never falls below [`FOURIER_HARMONIC_WIDTH`], so a document whose
/// harmonics stop below a thousand renders exactly as it always has.
fn fourier_harmonic_width(analyzed: &[AnalyzedSpectrum]) -> usize {
    let largest = analyzed
        .iter()
        .flat_map(|(_, _, result)| result.harmonics.iter())
        .map(|harmonic| harmonic.harmonic_number)
        .max()
        .unwrap_or(0);
    let digits = largest.checked_ilog10().map_or(1, |log| log as usize + 1);
    digits.max(FOURIER_HARMONIC_WIDTH)
}

/// The columns every row gains over the frame's drawn width.
fn fourier_frame_overhang(harmonic_width: usize) -> usize {
    harmonic_width.saturating_sub(FOURIER_HARMONIC_WIDTH)
}

/// Columns inside the frame once the harmonic column has been sized.
fn fourier_frame_interior(harmonic_width: usize) -> usize {
    FOURIER_FRAME_INTERIOR + fourier_frame_overhang(harmonic_width)
}

/// Emit every line of the results frame for one analyzed document.
///
/// The lines are handed to `emit` one at a time rather than collected: a card
/// authoring many harmonics for many operands prints a frame far larger than
/// the spectra it was rendered from.
fn render_fourier_frame(analyzed: &[AnalyzedSpectrum], emit: &mut impl FnMut(&str)) {
    let harmonic_width = fourier_harmonic_width(analyzed);
    emit(&fourier_frame_rule('┌', '┐', harmonic_width));
    emit(&fourier_title_row(harmonic_width));
    emit(&fourier_frame_rule('├', '┤', harmonic_width));

    for (output, physical_type, result) in analyzed {
        emit(&fourier_output_row(output, physical_type, harmonic_width));
        emit(&fourier_dc_row(result.dc_component, harmonic_width));
        emit(&fourier_frame_rule('├', '┤', harmonic_width));
        emit(&fourier_harmonic_header_row(harmonic_width));
        emit(&fourier_frame_rule('├', '┤', harmonic_width));

        for harmonic in result.harmonics.iter().filter(|h| h.harmonic_number > 0) {
            emit(&fourier_harmonic_row(harmonic, harmonic_width));
        }

        emit(&fourier_frame_rule('├', '┤', harmonic_width));
        emit(&fourier_thd_row(result.thd, harmonic_width));
        emit(&fourier_frame_rule('└', '┘', harmonic_width));
    }
}

/// One horizontal rule of the frame, with the corners the caller needs.
fn fourier_frame_rule(left: char, right: char, harmonic_width: usize) -> String {
    let interior = fourier_frame_interior(harmonic_width);
    format!("{left}{}{right}", "─".repeat(interior))
}

/// The frame's caption row.
fn fourier_title_row(harmonic_width: usize) -> String {
    let interior = fourier_frame_interior(harmonic_width);
    format!("│{:^interior$}│", "FOURIER ANALYSIS RESULTS")
}

/// The row naming one analyzed output and its physical type.
///
/// Output names are authored — a hierarchical node name is routinely longer
/// than any column — and a format width pads but never truncates, so the name
/// is cut to its column rather than allowed to carry this row's border past
/// the frame. The name's column is where this row spends the frame's overhang.
fn fourier_output_row(output: &str, physical_type: &str, harmonic_width: usize) -> String {
    let width = FOURIER_OUTPUT_WIDTH + fourier_frame_overhang(harmonic_width);
    format!(
        "│ Output: {:width$} ({:type_width$}) │",
        crate::commands::truncate(output, width),
        physical_type,
        width = width,
        type_width = FOURIER_PHYSICAL_TYPE_WIDTH
    )
}

/// The row carrying one spectrum's DC component.
fn fourier_dc_row(dc_component: Value, harmonic_width: usize) -> String {
    let width = FOURIER_DC_WIDTH + fourier_frame_overhang(harmonic_width);
    format!("│ DC component = {dc_component:<width$.6e} │")
}

/// The headings above one spectrum's harmonics.
///
/// The overhang is spent immediately after the index heading, exactly where the
/// index column gains it, so every heading to its right stays over its field.
fn fourier_harmonic_header_row(harmonic_width: usize) -> String {
    let overhang = " ".repeat(fourier_frame_overhang(harmonic_width));
    format!("│  Harmonic{overhang}    Frequency (Hz)    Magnitude    Phase (deg)        │")
}

/// One harmonic's row.
fn fourier_harmonic_row(harmonic: &HarmonicComponent, harmonic_width: usize) -> String {
    format!(
        "│  {:harmonic_width$}        {:12.4e}      {:10.6}   {:10.2}          │",
        harmonic.harmonic_number, harmonic.frequency, harmonic.magnitude, harmonic.phase
    )
}

/// The row carrying one spectrum's total harmonic distortion.
fn fourier_thd_row(thd: Option<Value>, harmonic_width: usize) -> String {
    let interior = fourier_frame_interior(harmonic_width);
    // `undefined` stands where the value would, but carries no ` %` of its own,
    // so both spellings are padded to the frame rather than to a fixed filler.
    let body = match thd {
        Some(thd) => format!("  THD:            {thd:10.4} %"),
        None => "  THD:             undefined".to_string(),
    };
    format!("│{body:<interior$}│")
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
    analyzed: &[AnalyzedSpectrum],
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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::analysis::FourierResult;

    /// Every line the frame prints for one document, in order.
    fn frame_lines(analyzed: &[AnalyzedSpectrum]) -> Vec<String> {
        let mut lines = Vec::new();
        render_fourier_frame(analyzed, &mut |line| lines.push(line.to_string()));
        lines
    }

    /// One analyzed operand carrying `count` harmonics above its DC term.
    fn spectrum(output: &str, count: usize, thd: Option<Value>) -> AnalyzedSpectrum {
        let harmonics = (0..=count)
            .map(|n| HarmonicComponent {
                harmonic_number: n,
                frequency: 1.0e3 * n as Value,
                magnitude: 1.0 / (n as Value + 1.0),
                phase: -90.0 + n as Value,
            })
            .collect();
        (
            output.to_string(),
            "voltage",
            FourierResult {
                fundamental_freq: 1.0e3,
                dc_component: 1.2345e-3,
                harmonics,
                thd,
            },
        )
    }

    #[test]
    fn every_fourier_output_row_is_the_frame_wide() {
        let width = fourier_frame_rule('┌', '┐', FOURIER_HARMONIC_WIDTH)
            .chars()
            .count();
        assert_eq!(width, FOURIER_FRAME_INTERIOR + 2);
        for rule in [('├', '┤'), ('└', '┘')] {
            assert_eq!(
                fourier_frame_rule(rule.0, rule.1, FOURIER_HARMONIC_WIDTH)
                    .chars()
                    .count(),
                width
            );
        }

        // The three types `evaluate_tran_four_output_requests` can report,
        // against both a short name and a hierarchical one no column fits.
        let long = "x_top.x_bias.x_mirror.m_cascode_tail:drain_current_probe";
        for physical_type in ["voltage", "current", "parameter"] {
            for output in ["vout", long] {
                let row = fourier_output_row(output, physical_type, FOURIER_HARMONIC_WIDTH);
                assert_eq!(
                    row.chars().count(),
                    width,
                    "this row leaves the frame: {row}"
                );
            }
        }
        assert!(
            fourier_output_row(long, "parameter", FOURIER_HARMONIC_WIDTH)
                .contains("x_top.x_bias.x_mirror.m_cascode_tail:dr..."),
            "a name wider than its column is cut to it"
        );
        assert!(
            fourier_output_row("vout", "voltage", FOURIER_HARMONIC_WIDTH)
                .contains("Output: vout   "),
            "a name inside its column is printed whole and padded"
        );
    }

    /// A document whose harmonics stop below a thousand prints exactly the
    /// bytes this module printed before the index column could grow.
    ///
    /// The expectation is a frozen copy of that rendering — the same literal
    /// widths and filler, spelled out here rather than derived from the
    /// constants the printer now sizes itself from.
    #[test]
    fn a_document_below_a_thousand_harmonics_renders_byte_for_byte_as_before() {
        let analyzed = vec![
            spectrum("vout", 3, Some(2.5)),
            spectrum("x_top.x_bias:branch_current", 999, None),
        ];

        let interior = 64;
        let mut expected = vec![
            format!("┌{}┐", "─".repeat(interior)),
            format!("│{:^interior$}│", "FOURIER ANALYSIS RESULTS"),
            format!("├{}┤", "─".repeat(interior)),
        ];
        for (output, physical_type, result) in &analyzed {
            expected.push(format!(
                "│ Output: {:42} ({:9}) │",
                crate::commands::truncate(output, 42),
                physical_type
            ));
            expected.push(format!("│ DC component = {:<47.6e} │", result.dc_component));
            expected.push(format!("├{}┤", "─".repeat(interior)));
            expected.push(
                "│  Harmonic    Frequency (Hz)    Magnitude    Phase (deg)        │".to_string(),
            );
            expected.push(format!("├{}┤", "─".repeat(interior)));
            for harmonic in result.harmonics.iter().filter(|h| h.harmonic_number > 0) {
                expected.push(format!(
                    "│  {:3}        {:12.4e}      {:10.6}   {:10.2}          │",
                    harmonic.harmonic_number,
                    harmonic.frequency,
                    harmonic.magnitude,
                    harmonic.phase
                ));
            }
            expected.push(format!("├{}┤", "─".repeat(interior)));
            expected.push(match result.thd {
                Some(thd) => {
                    format!("│  THD:            {thd:10.4} %                                  │")
                }
                None => {
                    "│  THD:             undefined                                    │".to_string()
                }
            });
            expected.push(format!("└{}┘", "─".repeat(interior)));
        }

        assert_eq!(
            frame_lines(&analyzed),
            expected,
            "a document under a thousand harmonics must render unchanged"
        );
    }

    /// `.FOUR` bounds nothing at three digits, and a four-digit index that
    /// overran its column would carry its row's border out of the frame.
    #[test]
    fn a_card_authoring_more_than_a_thousand_harmonics_keeps_the_frame_square() {
        let lines = frame_lines(&[spectrum("vout", 1200, Some(2.5))]);
        let width = lines[0].chars().count();
        assert_eq!(
            width,
            FOURIER_FRAME_INTERIOR + 2 + 1,
            "a four-digit index widens every row by exactly one column"
        );
        for line in &lines {
            assert_eq!(
                line.chars().count(),
                width,
                "this line leaves the frame: {line}"
            );
        }
        assert!(
            lines.iter().any(|line| line.starts_with("│  1200 ")),
            "the largest index is printed whole, not cut to three digits"
        );
        assert!(
            lines.iter().any(|line| line.starts_with("│   999 ")),
            "a shorter index is padded into the widened column"
        );

        // The width the frame grows at, from the last index that fits.
        assert_eq!(
            frame_lines(&[spectrum("vout", 999, None)])[0]
                .chars()
                .count(),
            FOURIER_FRAME_INTERIOR + 2
        );
    }
}
