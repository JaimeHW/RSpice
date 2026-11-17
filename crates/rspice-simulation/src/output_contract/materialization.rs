//! Exact saved-output calculation and retention, independent of host presentation.

use super::{
    PreparedSavedOutput, TransientSelectionGrid, parse_probe, parse_rf_port, probe_identity,
    validate_selection_grid,
};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::calculator;
use rspice_results::calculator::evaluator::CalcValue;
use rspice_results::saved_output::{
    SavedOutputKind, SavedOutputMaterializationStatus, SavedOutputPolicy, SavedOutputStreaming,
};
use rspice_results::waveform::{RetainedWaveform, WaveformValues};
use std::borrow::Cow;
use std::collections::HashSet;
use std::ops::Range;
use std::sync::Arc;

mod bindings;
mod dc_family;
mod materialize;
mod probe;
#[cfg(test)]
mod resample_tests;
pub use materialize::{
    apply_saved_output_policy, materialize_deferred_saved_output, materialize_live_saved_outputs,
    materialize_saved_outputs, retain_plan_saved_outputs,
};
pub use probe::resolve_raw_probe;
pub use rspice_results::waveform::OutputWaveform;

fn resolve_contract_waveform<W: OutputWaveform>(
    contract: &PreparedSavedOutput,
    analysis: &AnalysisResult<W>,
    waveforms: &[W],
) -> Result<W, String> {
    match contract.kind() {
        SavedOutputKind::RawVoltageOrCurrent => resolve_raw_probe(
            contract.source_expression(),
            waveforms,
            contract.name(),
            analysis.analysis_type.uses_complex_bode_projection(),
        ),
        SavedOutputKind::DerivedExpression => resolve_derived_expression(
            contract.source_expression(),
            waveforms,
            contract.name(),
            contract.complex_policy(),
        ),
        SavedOutputKind::DeviceOperatingPointQuantity => {
            resolve_device_quantity(contract.source_expression(), analysis, contract.name())
        }
        SavedOutputKind::NoiseContributor => {
            // Typed noise names include output tuples and mechanism labels. A
            // quoted reference preserves that complete identity and its units.
            if let Ok(calculator::ast::CalculatorExpr::WaveformRef {
                signal,
                dataset: None,
            }) = calculator::spice_parser::try_parse(contract.source_expression())
                && contract.source_expression().trim_start().starts_with('"')
            {
                return clone_named_waveform(waveforms, &signal, contract.name());
            }
            let source = format!("noise({})", contract.source_expression().trim());
            clone_named_waveform(waveforms, &source, contract.name()).or_else(|_| {
                clone_named_waveform(waveforms, contract.source_expression(), contract.name())
            })
        }
        SavedOutputKind::RfPortQuantity => {
            let (output, input) = parse_rf_port(contract.source_expression())?;
            let separated = format!("S{output}_{input}");
            clone_named_waveform(waveforms, &separated, contract.name())
                .or_else(|error| {
                    if output <= 9 && input <= 9 {
                        clone_named_waveform(
                            waveforms,
                            &format!("S{output}{input}"),
                            contract.name(),
                        )
                    } else {
                        Err(error)
                    }
                })
                .or_else(|_| {
                    clone_named_waveform(waveforms, contract.source_expression(), contract.name())
                })
        }
    }
}

fn resolve_derived_expression<W: OutputWaveform>(
    expression: &str,
    waveforms: &[W],
    output_name: &str,
    complex_policy: rspice_results::saved_output::ComplexExpressionPolicy,
) -> Result<W, String> {
    resolve_derived_with(
        expression,
        output_name,
        &calculator::retained::WaveformsContext::with_policy(waveforms, complex_policy),
        waveforms.first(),
    )
}

fn resolve_derived_with<W: OutputWaveform>(
    expression: &str,
    output_name: &str,
    context: &impl calculator::evaluator::EvaluationContext,
    axis_source: Option<&W>,
) -> Result<W, String> {
    let parsed = calculator::spice_parser::Parser::new(expression)
        .try_parse()
        .map_err(|error| format!("expression parse failed: {error}"))?;
    let value = calculator::evaluator::evaluate(&parsed, context)
        .map_err(|error| format!("expression evaluation failed: {error}"))?;
    calculator::retained::evaluated_waveform(
        value,
        output_name,
        axis_source.map(|source| source.as_ref().x.as_slice()),
    )
    .map(W::from_retained)
}
fn resolve_device_quantity<W: OutputWaveform>(
    expression: &str,
    analysis: &AnalysisResult<W>,
    output_name: &str,
) -> Result<W, String> {
    let body = expression
        .trim()
        .strip_prefix('@')
        .ok_or_else(|| "device quantity must begin with '@'".to_owned())?;
    let open = body
        .find('[')
        .ok_or_else(|| "device quantity is missing '['".to_owned())?;
    let device = &body[..open];
    let quantity = body[open + 1..]
        .strip_suffix(']')
        .ok_or_else(|| "device quantity is missing ']'".to_owned())?;
    let report = analysis
        .device_op
        .as_ref()
        .ok_or_else(|| "analysis retained no device operating-point report".to_owned())?;
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case(device))
        .ok_or_else(|| format!("device '{device}' is absent from the operating-point report"))?;
    let value = entry
        .params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(quantity))
        .map(|(_, value)| *value)
        .ok_or_else(|| format!("device '{device}' has no '{quantity}' quantity"))?;
    Ok(W::from_retained(RetainedWaveform::new(
        output_name,
        vec![0.0],
        vec![value],
    )))
}

fn clone_named_waveform<W: OutputWaveform>(
    waveforms: &[W],
    source: &str,
    output_name: &str,
) -> Result<W, String> {
    find_waveform(waveforms, source)
        .map(|waveform| clone_with_name(waveform, output_name))
        .ok_or_else(|| format!("source waveform '{source}' is absent"))
}

fn find_waveform<'a, W: OutputWaveform>(waveforms: &'a [W], requested: &str) -> Option<&'a W> {
    let requested = requested.trim();
    find_literal_waveform(waveforms, requested).or_else(|| {
        if let Some((device, quantity)) =
            rspice_results::saved_output::device_current_probe(requested)
        {
            let engine = rspice_vocabulary::hierarchy_path::ProbeTarget::engine_alias(device)?;
            return find_literal_waveform(waveforms, &format!("@{engine}[{quantity}]"));
        }
        let (current, node) = probe_identity(requested);
        let engine = rspice_vocabulary::hierarchy_path::ProbeTarget::engine_alias(node)?;
        find_literal_waveform(
            waveforms,
            &format!("{}({engine})", if current { "I" } else { "V" }),
        )
    })
}

fn find_literal_waveform<'a, W: OutputWaveform>(
    waveforms: &'a [W],
    requested: &str,
) -> Option<&'a W> {
    // Exact authored names win before compatibility with bare engine nodes.
    // A node named V1 and branch I(V1) are different physical quantities.
    waveforms
        .iter()
        .find(|waveform| waveform.as_ref().name.eq_ignore_ascii_case(requested))
        .or_else(|| {
            waveforms.iter().find(|waveform| {
                let source = waveform
                    .as_ref()
                    .complex
                    .as_ref()
                    .map_or(waveform.as_ref().name.as_str(), |complex| {
                        complex.source_name.as_str()
                    });
                let (current, node) = probe_identity(source);
                let (requested_current, requested_node) = probe_identity(requested);
                current == requested_current && node.eq_ignore_ascii_case(requested_node)
            })
        })
}

fn waveform_matches_requested<W: OutputWaveform>(waveform: &W, requested: &str) -> bool {
    find_waveform(std::slice::from_ref(waveform), requested).is_some()
}

fn clone_with_name<W: OutputWaveform>(source: &W, name: &str) -> W {
    let mut waveform = source.clone();
    waveform.as_mut().name = name.to_owned();
    waveform
}

/// Rows of each column a resample holds at once: a stored run's columns are read a block at a
/// time, never whole.
const RESAMPLE_BLOCK_ROWS: usize = 1 << 16;

/// Blocks of `count` rows, each sharing its last row with the next, so every pair of
/// neighbouring rows sits in one block.
fn overlapping_blocks(count: usize) -> impl Iterator<Item = Range<usize>> {
    (0..count.saturating_sub(1).max(1))
        .step_by(RESAMPLE_BLOCK_ROWS)
        .map(move |start| start..(start + RESAMPLE_BLOCK_ROWS + 1).min(count))
}

fn read_rows(values: &WaveformValues, rows: Range<usize>) -> Result<Cow<'_, [f64]>, String> {
    values
        .read_range(rows)
        .map_err(|failure| failure.to_string())
}

/// `values` over `axis` at the ascending `targets`, all inside the axis, interpolated block by
/// block. A target falls in the first block that spans it, and the interpolator's answer there
/// is the one it gives over the whole axis: the interval around a target and the knots it lands
/// on are the same rows.
fn resample_blocks(
    axis: &WaveformValues,
    values: &WaveformValues,
    targets: &[f64],
) -> Result<Vec<f64>, String> {
    if values.len() != axis.len() {
        return Err(rspice_results::interpolation::InterpolationError::LengthMismatch.to_string());
    }
    let mut resampled = Vec::with_capacity(targets.len());
    for rows in overlapping_blocks(axis.len()) {
        if resampled.len() == targets.len() {
            break;
        }
        let x = read_rows(axis, rows.clone())?;
        let last = *x.last().expect("a block holds a row");
        let taken = targets[resampled.len()..].partition_point(|target| *target <= last);
        if taken == 0 {
            continue;
        }
        let y = read_rows(values, rows)?;
        let interpolator = rspice_results::interpolation::WaveformInterpolator::new(&x, &y)
            .map_err(|error| error.to_string())?;
        let next = resampled.len();
        for &target in &targets[next..next + taken] {
            resampled.push(
                interpolator
                    .interpolate_at(target)
                    .map_err(|error| error.to_string())?,
            );
        }
    }
    if resampled.len() != targets.len() {
        return Err(rspice_results::interpolation::InterpolationError::OutsideDomain.to_string());
    }
    Ok(resampled)
}

/// Resample onto the selection grid and its exact final point. A stored run's columns are read
/// a block at a time, so the output costs its own samples and a block of each source column.
pub fn resample_selected_and_final<W: OutputWaveform>(
    waveform: &W,
    grid: TransientSelectionGrid,
) -> Result<W, String> {
    validate_selection_grid(grid)?;
    let source = waveform.as_ref();
    let count = source.x.len();
    if count == 0 || count != source.y.len() {
        return Err("source waveform has no aligned samples".to_owned());
    }
    // A stored run's time axis was checked to ascend strictly as the run ended.
    if !source.x.is_ascending_axis() {
        for rows in overlapping_blocks(count) {
            if read_rows(&source.x, rows)?
                .windows(2)
                .any(|window| !window[0].is_finite() || window[1] <= window[0])
            {
                return Err("source waveform axis is not strictly increasing".to_owned());
            }
        }
    }
    let Some((first, last)) = source.x.ends() else {
        return Err("source waveform has no aligned samples".to_owned());
    };
    let start = grid.start.max(first);
    let stop = grid.stop.min(last);
    if stop < start {
        return Err("selected-point grid does not overlap the source axis".to_owned());
    }
    let mut x = Vec::new();
    let mut cursor = start;
    while cursor < stop {
        x.push(cursor);
        cursor = start + grid.step * x.len() as f64;
    }
    if x.last()
        .is_none_or(|value| value.to_bits() != stop.to_bits())
    {
        x.push(stop);
    }
    let y = resample_blocks(&source.x, &source.y, &x)?;
    let mut result = RetainedWaveform::new(&source.name, x.clone(), y);
    result.as_mut().unit = source.unit.clone();
    if let Some(complex) = &source.complex {
        let real = resample_blocks(&source.x, &complex.real, &x)?;
        let imag = resample_blocks(&source.x, &complex.imag, &x)?;
        result.as_mut().y = real
            .iter()
            .zip(&imag)
            .map(|(real, imag)| real.hypot(*imag))
            .collect::<Vec<_>>()
            .into();
        result = result.with_complex_components(&complex.source_name, real, imag);
    }
    let mut output = waveform.clone();
    *output.as_mut() = result;
    Ok(output)
}
