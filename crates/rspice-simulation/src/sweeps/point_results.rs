//! PVT family assembly and terminal reduction over exact retained evidence.

use std::collections::HashMap;

use rspice_core::{NoAbort, Value};
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::waveform::RetainedWaveform;

use super::mapping::{map_corner_results, map_temperature_results};
use super::{CornerBaseMode, CornerPoint};
use crate::results::{SimulationResult, WaveformData};

/// One scalar per node at a single swept point, in a node order the caller
/// keeps identical across points: the mappers pair names and values by index.
/// Index zero is ground and is omitted from family traces.
#[derive(Debug, Clone)]
pub struct SweepPointResult {
    pub node_names: Vec<String>,
    pub node_values: Vec<Value>,
}

/// What a temperature family names its axis. The deck-driven parametric runner
/// spells the same target for a `.STEP TEMP`, so a plot cannot tell a
/// temperature step apart by where it came from.
const TEMPERATURE_TARGET: &str = "TEMP";

/// The corner family a set of solved points adds up to.
///
/// `solved` arrives in declaration order and may include points that failed;
/// a point that did not converge contributes nothing to the axis and one to the
/// failure count, which is what the corner executor did when it dropped a
/// point's result.
pub fn corner_family_of_points(
    base_mode: &CornerBaseMode,
    declared_points: usize,
    converged: &[(CornerPoint, SweepPointResult)],
) -> Result<SimulationResult, String> {
    if converged.is_empty() {
        return Err("Corner analysis produced no converged corner points".to_owned());
    }
    let (x_values, x_label, x_unit, temperatures_c, corner_labels, voltages) =
        map_corner_results(converged, base_mode.metric_label(), &NoAbort)
            .map_err(|error| error.to_string())?;

    Ok(SimulationResult::Corner {
        waveforms: waveforms_over(&x_values, voltages),
        x_values,
        x_label,
        x_unit,
        temperatures_c,
        corner_labels,
        num_failures: declared_points.saturating_sub(converged.len()),
        // Deliberately none. Every point of this family is its own retained
        // `AnalysisResult` carrying its own `.MEAS` evaluation and its own PVT
        // attribution, so a limit is already answered over all of them by the
        // ordinary worst-of join. Restating those measurements on the reduction
        // would enter each point into the same verdict twice and report a
        // yield over double the trials the run performed.
        member_measurements: Vec::new(),
    })
}

/// The temperature family a set of solved points adds up to.
///
/// The same reduction as a corner's, laid against the temperatures themselves
/// rather than against a derived corner axis: a temperature step varies one
/// quantity, so the axis is that quantity and needs no derivation.
pub fn temperature_family_of_points(
    base_mode: &CornerBaseMode,
    declared_points: usize,
    converged: &[(Value, SweepPointResult)],
) -> Result<SimulationResult, String> {
    if converged.is_empty() {
        return Err("Parametric analysis produced no converged sweep points".to_owned());
    }
    let (sweep_values, voltages) =
        map_temperature_results(converged, base_mode.metric_label(), &NoAbort)
            .map_err(|error| error.to_string())?;

    Ok(SimulationResult::Parametric {
        target: TEMPERATURE_TARGET.to_owned(),
        waveforms: waveforms_over(&sweep_values, voltages),
        sweep_values,
        num_failures: declared_points.saturating_sub(converged.len()),
        // Deliberately none, for the same reason a corner family reports none:
        // each temperature point is already its own retained result.
        member_measurements: Vec::new(),
    })
}

fn waveforms_over(
    x_values: &[Value],
    voltages: Vec<(String, Vec<Value>)>,
) -> HashMap<String, WaveformData> {
    let mut waveforms = HashMap::with_capacity(voltages.len());
    for (name, values) in voltages {
        waveforms.insert(
            name.clone(),
            WaveformData::new_time_domain(name, x_values.to_vec(), values),
        );
    }
    waveforms
}

/// The scalar each node contributed at one point, named the way the deck names
/// the node, sorted so every point agrees on an order.
///
/// The reduction is the base analysis's own terminal value: the converged
/// operating point, the last swept point, the last transient sample, or the
/// magnitude at the terminal frequency.
pub fn point_node_values<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    base_mode: &CornerBaseMode,
) -> Result<Option<Vec<(String, f64)>>, String> {
    let mut values = match base_mode {
        CornerBaseMode::Op | CornerBaseMode::ConfiguredOp(_) => {
            let Some(values) = operating_point_node_values(analysis) else {
                return Ok(None);
            };
            values
        }
        CornerBaseMode::Ac { .. } => terminal_ac_magnitudes(analysis),
        CornerBaseMode::DcSweep { .. } | CornerBaseMode::DcSweepNested { .. } => {
            terminal_dc_node_samples(analysis, base_mode)?
        }
        CornerBaseMode::Transient { .. } | CornerBaseMode::TransientWindow { .. } => {
            terminal_node_samples(analysis)
        }
    };
    if values.is_empty() {
        return Ok(None);
    }
    values.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(Some(values))
}

fn terminal_dc_node_samples<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    base_mode: &CornerBaseMode,
) -> Result<Vec<(String, f64)>, String> {
    use rspice_results::dc_sweep::{DcSweepFamily, DcSweepQuantity, DcTraceView};
    let Some(AnalysisResultPayload::DcSweep { evidence }) = &analysis.result_payload else {
        return Err("DC point is missing exact curve identity and traversal evidence; rerun the point before assembling its family".to_owned());
    };
    match (base_mode, &evidence.family) {
        (CornerBaseMode::DcSweep { source_name, .. }, DcSweepFamily::Single)
            if source_name.eq_ignore_ascii_case(&evidence.source) => {}
        (
            CornerBaseMode::DcSweepNested {
                source_name,
                source2,
                ..
            },
            DcSweepFamily::Nested { source, .. },
        ) if source_name.eq_ignore_ascii_case(&evidence.source)
            && source2.eq_ignore_ascii_case(source) => {}
        _ => return Err("DC point evidence does not match its declared sweep family".to_owned()),
    }
    evidence.validate_retained_traces(analysis.waveforms.iter().map(AsRef::as_ref).map(
        |trace| DcTraceView {
            name: &trace.name,
            unit: trace.unit.as_deref(),
            x: &trace.x,
            sample_count: trace.y.len(),
            complex: trace.complex.is_some(),
        },
    ))?;
    let member = evidence.member_count() - 1;
    let by_name = analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .map(|trace| (trace.name.as_str(), trace))
        .collect::<HashMap<_, _>>();
    let mut values = Vec::new();
    for curve in evidence
        .curve_indices()
        .filter(|curve| curve.member == member)
    {
        let quantity = &evidence.quantities[curve.quantity];
        let DcSweepQuantity::NodeVoltage(node) = quantity else {
            continue;
        };
        let trace_name = evidence.trace_name(quantity, member);
        let trace = by_name[trace_name.as_str()];
        let index = evidence
            .terminal_sample(member, trace.y.len())
            .ok_or_else(|| "DC point has no terminal sample".to_owned())?;
        values.push((node.clone(), trace.y[index]));
    }
    Ok(values)
}

/// The operating point reads its retained MNA ordering rather than its node
/// map: the map is unordered, and two points that enumerated it differently
/// would swap their traces.
fn operating_point_node_values<W>(analysis: &AnalysisResult<W>) -> Option<Vec<(String, f64)>> {
    let Some(AnalysisResultPayload::OperatingPoint {
        mna_node_names,
        mna_solution,
        ..
    }) = analysis.result_payload.as_ref()
    else {
        return None;
    };
    Some(
        mna_node_names
            .iter()
            .zip(mna_solution.iter())
            .map(|(name, value)| (name.clone(), *value))
            .collect(),
    )
}

fn terminal_node_samples<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
) -> Vec<(String, f64)> {
    analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .filter(|waveform| !is_branch_current(&waveform.name))
        .filter_map(|waveform| {
            waveform
                .y
                .last()
                .map(|value| (waveform.name.clone(), *value))
        })
        .collect()
}

/// AC contributes the magnitude at the terminal frequency, recomputed from the
/// retained complex samples so it is the same quantity the collapsing executor
/// took from the solver rather than a re-derivation of the display trace.
fn terminal_ac_magnitudes<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
) -> Vec<(String, f64)> {
    analysis
        .waveforms
        .iter()
        .map(AsRef::as_ref)
        .filter_map(|waveform| {
            let node = waveform
                .name
                .strip_prefix("|V(")
                .and_then(|inner| inner.strip_suffix(")|"))?;
            let complex = waveform.complex.as_ref()?;
            let real = complex.real.last()?;
            let imaginary = complex.imag.last()?;
            Some((node.to_owned(), real.hypot(*imaginary)))
        })
        .collect()
}

/// A family is one scalar per node, so a branch-current trace beside them is
/// not a node and must not become one.
fn is_branch_current(name: &str) -> bool {
    (name.starts_with("I(") || name.starts_with("i(")) && name.ends_with(')')
}
