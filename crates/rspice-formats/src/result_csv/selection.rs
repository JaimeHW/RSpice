//! Exact selected-result text over host-admitted canonical evidence.
//!
//! Source resolution and retained-evidence quarantine belong to the host.
//! Vector shape checks here prevent indexing malformed storage during encoding.

use rspice_app_types::product::DatasetId;
use rspice_results::{
    analysis_result::AnalysisResult, operating_point::OperatingPointValue,
    waveform::RetainedWaveform,
};

/// Encode the final real and optional complex sample at full precision.
pub fn encode_exact_sample(waveform: &RetainedWaveform) -> Result<String, String> {
    use std::fmt::Write as _;

    validate_result_waveform_vectors(waveform)?;
    let index = waveform
        .x
        .len()
        .checked_sub(1)
        .ok_or_else(|| "The selected quantity retains no samples.".to_owned())?;
    let mut exact = format!(
        "{}\tx={:.17e}\ty={:.17e}",
        waveform.name, waveform.x[index], waveform.y[index]
    );
    if let Some(complex) = &waveform.complex {
        write!(
            exact,
            "\treal={:.17e}\timag={:.17e}",
            complex.real[index], complex.imag[index]
        )
        .expect("writing to a String cannot fail");
    }
    Ok(exact)
}

/// Encode an admitted signal as exact sample rows with its source labels.
pub fn encode_signal_tsv(
    dataset: DatasetId,
    analysis_label: &str,
    waveform: &RetainedWaveform,
) -> Result<String, String> {
    use std::fmt::Write as _;

    validate_result_waveform_vectors(waveform)?;
    let mut exact = String::with_capacity(waveform.x.len().saturating_mul(64).min(8_000_000));
    writeln!(exact, "# dataset\t{}", dataset).expect("writing to a String cannot fail");
    writeln!(exact, "# analysis\t{}", analysis_label).expect("writing to a String cannot fail");
    writeln!(exact, "# quantity\t{}", waveform.name).expect("writing to a String cannot fail");
    if waveform.complex.is_some() {
        exact.push_str("sample\tx\ty\treal\timag\n");
    } else {
        exact.push_str("sample\tx\ty\n");
    }
    for index in 0..waveform.x.len() {
        write!(
            exact,
            "{index}\t{:.17e}\t{:.17e}",
            waveform.x[index], waveform.y[index]
        )
        .expect("writing to a String cannot fail");
        if let Some(complex) = &waveform.complex {
            write!(
                exact,
                "\t{:.17e}\t{:.17e}",
                complex.real[index], complex.imag[index]
            )
            .expect("writing to a String cannot fail");
        }
        exact.push('\n');
    }
    Ok(exact)
}

/// Encode the selected artifact; the host supplies its resolved analysis and name.
pub fn encode_artifact_text<W>(
    dataset: DatasetId,
    analysis: &AnalysisResult<W>,
    canonical_name: &str,
) -> Result<String, String> {
    use std::fmt::Write as _;

    let mut exact = format!(
        "# rspice-result-artifact-v1\n# dataset\t{}\n# analysis\t{}\n# canonical-name\t{}\n",
        dataset, analysis.label, canonical_name
    );
    match canonical_name {
        "dc-op/node-voltages" => append_operating_point_values(
            &mut exact,
            analysis
                .dc_op
                .as_ref()
                .map(|op| op.node_voltages.as_slice()),
        )?,
        "dc-op/branch-currents" => append_operating_point_values(
            &mut exact,
            analysis
                .dc_op
                .as_ref()
                .map(|op| op.branch_currents.as_slice()),
        )?,
        "dc-op/power-dissipation" => append_operating_point_values(
            &mut exact,
            analysis
                .dc_op
                .as_ref()
                .map(|op| op.power_dissipation.as_slice()),
        )?,
        "dc-op/device-report" => {
            let report = analysis.device_op.as_ref().ok_or_else(|| {
                "The selected device operating-point report is unavailable.".to_owned()
            })?;
            exact.push_str("device\tkind\tregion\tparameter\tvalue\n");
            for entry in &report.entries {
                for (parameter, value) in &entry.params {
                    writeln!(
                        exact,
                        "{}\t{}\t{}\t{}\t{:.17e}",
                        entry.name,
                        entry.device_kind,
                        entry.region.unwrap_or(""),
                        parameter,
                        value
                    )
                    .expect("writing to a String cannot fail");
                }
            }
        }
        "noise/contributions" => {
            let noise = analysis.noise_summary.as_ref().ok_or_else(|| {
                "The selected noise contribution table is unavailable.".to_owned()
            })?;
            exact.push_str("device\tmechanism\tpower_v2\tshare_percent\n");
            for row in &noise.rows {
                writeln!(
                    exact,
                    "{}\t{}\t{:.17e}\t{:.17e}",
                    row.device, row.mechanism, row.power, row.share_pct
                )
                .expect("writing to a String cannot fail");
            }
        }
        "noise/output-rms" | "noise/input-rms" => {
            let noise = analysis
                .noise_summary
                .as_ref()
                .ok_or_else(|| "The selected noise scalar is unavailable.".to_owned())?;
            let value = if canonical_name == "noise/output-rms" {
                noise.total_rms
            } else {
                noise.input_rms
            }
            .ok_or_else(|| "The selected noise scalar was not retained.".to_owned())?;
            writeln!(exact, "value\tunit\n{value:.17e}\tV")
                .expect("writing to a String cannot fail");
        }
        canonical if canonical.starts_with("payload/") => {
            let payload = analysis.result_payload.as_ref().ok_or_else(|| {
                "The selected analysis-native result payload is unavailable.".to_owned()
            })?;
            exact.push_str("schema\tanalysis-result-payload-v1\njson\n");
            exact.push_str(&serde_json::to_string_pretty(payload).map_err(|error| {
                format!("Could not serialize the retained typed payload: {error}")
            })?);
            exact.push('\n');
        }
        "family/metadata" => {
            let metadata = analysis
                .family_metadata
                .as_ref()
                .ok_or_else(|| "The selected family metadata is unavailable.".to_owned())?;
            exact.push_str("schema\tanalysis-family-metadata-v1\njson\n");
            exact.push_str(&serde_json::to_string_pretty(metadata).map_err(|error| {
                format!("Could not serialize the retained family metadata: {error}")
            })?);
            exact.push('\n');
        }
        canonical if canonical.starts_with("measurement/") => {
            let name = canonical.trim_start_matches("measurement/");
            let mut matches = analysis
                .measurements
                .iter()
                .filter(|measurement| measurement.name == name);
            let measurement = matches
                .next()
                .filter(|_| matches.next().is_none())
                .ok_or_else(|| {
                    "The selected measurement no longer resolves uniquely.".to_owned()
                })?;
            exact.push_str("name\tvalue\tpassed\texpected\ttolerance\tevent-axis\terror\n");
            writeln!(
                exact,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                measurement.name,
                optional_exact_float(measurement.value),
                measurement.passed,
                optional_exact_float(measurement.expected),
                optional_exact_float(measurement.tolerance),
                optional_exact_float(measurement.event_axis),
                measurement.error.as_deref().unwrap_or("")
            )
            .expect("writing to a String cannot fail");
        }
        _ => return Err("The selected typed result has no exact-value adapter.".to_owned()),
    }
    Ok(exact)
}

/// Frame selected items in order, resolving one item at a time and stopping on error.
pub fn encode_selection_text(
    items: impl ExactSizeIterator<Item = Result<(String, String), String>>,
) -> Result<String, String> {
    use std::fmt::Write as _;

    if items.len() == 0 {
        return Err("No Data Browser quantities are selected.".to_owned());
    }
    let mut contents = format!(
        "# rspice-result-selection-v1\n# item-count\t{}\n",
        items.len()
    );
    for (index, item) in items.enumerate() {
        let (path, exact) = item?;
        writeln!(contents, "\n## item\t{}\n## stable-path\t{path}", index + 1)
            .expect("writing to a String cannot fail");
        contents.push_str(&exact);
        if !contents.ends_with('\n') {
            contents.push('\n');
        }
    }
    Ok(contents)
}

fn validate_result_waveform_vectors(waveform: &RetainedWaveform) -> Result<(), String> {
    if waveform.x.len() != waveform.y.len() {
        return Err(format!(
            "{} has {} x coordinates and {} y values; exact copy is fail-closed.",
            waveform.name,
            waveform.x.len(),
            waveform.y.len()
        ));
    }
    if let Some(complex) = &waveform.complex
        && (complex.real.len() != waveform.x.len() || complex.imag.len() != waveform.x.len())
    {
        return Err(format!(
            "{} has complex components whose lengths do not match its {} coordinates; exact copy is fail-closed.",
            waveform.name,
            waveform.x.len()
        ));
    }
    Ok(())
}

fn append_operating_point_values(
    target: &mut String,
    values: Option<&[OperatingPointValue]>,
) -> Result<(), String> {
    use std::fmt::Write as _;

    let values =
        values.ok_or_else(|| "The selected operating-point array is unavailable.".to_owned())?;
    target.push_str("canonical-name\tvalue\tunit\n");
    for value in values {
        writeln!(
            target,
            "{}\t{:.17e}\t{}",
            value.name, value.value, value.unit
        )
        .expect("writing to a String cannot fail");
    }
    Ok(())
}

fn optional_exact_float(value: Option<f64>) -> String {
    value.map_or_else(String::new, |value| format!("{value:.17e}"))
}
