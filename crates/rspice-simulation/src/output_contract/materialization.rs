//! Exact saved-output calculation and retention, independent of host presentation.

use super::{
    PreparedSavedOutput, TransientSelectionGrid, parse_probe, parse_rf_port, probe_identity,
    validate_selection_grid,
};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::calculator;
use rspice_results::calculator::evaluator::CalcValue;
use rspice_results::saved_output::{
    SavedOutputKind, SavedOutputMaterializationStatus, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming,
};
use rspice_results::waveform::RetainedWaveform;
use std::collections::HashSet;
use std::sync::Arc;

mod bindings;
mod dc_family;
mod materialize;
mod probe;
mod waveform;
pub use materialize::{
    apply_saved_output_policy, materialize_deferred_saved_output, materialize_live_saved_outputs,
    materialize_saved_outputs, retain_plan_saved_outputs,
};
pub use probe::resolve_raw_probe;
pub use waveform::OutputWaveform;

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
            let engine = rspice_app_types::hierarchy_path::ProbeTarget::engine_alias(device)?;
            return find_literal_waveform(waveforms, &format!("@{engine}[{quantity}]"));
        }
        let (current, node) = probe_identity(requested);
        let engine = rspice_app_types::hierarchy_path::ProbeTarget::engine_alias(node)?;
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
    waveform.reset_display_cache();
    waveform
}

pub fn resample_selected_and_final<W: OutputWaveform>(
    waveform: &W,
    grid: TransientSelectionGrid,
) -> Result<W, String> {
    validate_selection_grid(grid)?;
    if waveform.as_ref().x.is_empty() || waveform.as_ref().x.len() != waveform.as_ref().y.len() {
        return Err("source waveform has no aligned samples".to_owned());
    }
    if waveform
        .as_ref()
        .x
        .windows(2)
        .any(|window| !window[0].is_finite() || window[1] <= window[0])
    {
        return Err("source waveform axis is not strictly increasing".to_owned());
    }
    let first = waveform.as_ref().x[0];
    let last = *waveform.as_ref().x.last().expect("non-empty checked");
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
    let resample = |values: &[f64]| {
        rspice_results::interpolation::WaveformInterpolator::new(&waveform.as_ref().x, values)
            .and_then(|source| source.resample(&x))
            .map_err(|error| error.to_string())
    };
    let y = resample(&waveform.as_ref().y)?;
    let mut result = RetainedWaveform::new(&waveform.as_ref().name, x.clone(), y);
    result.as_mut().unit = waveform.as_ref().unit.clone();
    if let Some(complex) = &waveform.as_ref().complex {
        let real = resample(&complex.real)?;
        let imag = resample(&complex.imag)?;
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
    output.reset_display_cache();
    Ok(output)
}
