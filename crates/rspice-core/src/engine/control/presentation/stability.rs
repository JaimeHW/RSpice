//! STB vectors and scalar determinations share the document's physical meaning.
use super::*;
use crate::engine::StbAnalysisResult;
use crate::execution::AnalysisResultDocument;
use crate::netlist::expr::ExprError;

#[derive(Clone, Copy)]
pub(super) enum StabilityColumn {
    LoopGain,
    Magnitude,
    Decibels,
    Phase,
    Metric(Metric),
}

#[derive(Clone, Copy)]
pub(super) enum Metric {
    GainMargin,
    GainFrequency,
    PhaseMargin,
    PhaseFrequency,
    DcGain,
    DcDecibels,
    UnityBandwidth,
    Crossovers,
    MultipleCrossovers,
}

impl StabilityColumn {
    pub(super) fn sample(self, result: &StbAnalysisResult, row: usize) -> Option<ComplexValue> {
        let margins = &result.result.margins;
        match self {
            Self::LoopGain => result.loop_gains.get(row).copied(),
            Self::Magnitude => result
                .result
                .bode_points
                .get(row)?
                .magnitude
                .map(Into::into),
            Self::Decibels => result
                .result
                .bode_points
                .get(row)?
                .magnitude_db
                .map(Into::into),
            Self::Phase => result
                .result
                .bode_points
                .get(row)?
                .phase_deg
                .map(Into::into),
            Self::Metric(metric) => match metric {
                Metric::GainMargin => margins.gain_margin.map(|margin| margin.value.into()),
                Metric::GainFrequency => margins.gain_margin.map(|margin| margin.frequency.into()),
                Metric::PhaseMargin => margins.phase_margin.map(|margin| margin.value.into()),
                Metric::PhaseFrequency | Metric::UnityBandwidth => {
                    margins.phase_margin.map(|margin| margin.frequency.into())
                }
                Metric::DcGain => margins.dc_loop_gain,
                Metric::DcDecibels => margins.dc_gain_db().map(Into::into),
                Metric::Crossovers => Some((margins.num_crossovers as Value).into()),
                Metric::MultipleCrossovers => Some(Value::from(margins.num_crossovers > 1).into()),
            },
        }
    }
}

pub(super) fn select<'a>(dataset: &'a ControlNamedDataset, name: &str) -> Option<Selected<'a>> {
    if !matches!(dataset.result, ControlAnalysisResult::Stability(_)) {
        return None;
    }
    use StabilityColumn::*;
    let db = || SignalUnit::Custom("dB".into());
    let (column, signal, unit) = match name.to_ascii_lowercase().as_str() {
        "loopgain" | "loop_gain" => (LoopGain, "loop_gain", SignalUnit::Dimensionless),
        "loop_gain_magnitude" => (Magnitude, "loop_gain_magnitude", SignalUnit::Dimensionless),
        "loopgain_mag_db" | "loop_gain_db" => (Decibels, "loop_gain_db", db()),
        "loopgain_phase_deg" | "loop_gain_phase" => (Phase, "loop_gain_phase", SignalUnit::Degree),
        "gain_margin" | "gain_margin_db" => {
            (Metric(self::Metric::GainMargin), "gain_margin_db", db())
        }
        "gain_margin_frequency" => (
            Metric(self::Metric::GainFrequency),
            "gain_margin_frequency",
            SignalUnit::Hertz,
        ),
        "phase_margin" | "phase_margin_degrees" => (
            Metric(self::Metric::PhaseMargin),
            "phase_margin_degrees",
            SignalUnit::Degree,
        ),
        "phase_margin_frequency" => (
            Metric(self::Metric::PhaseFrequency),
            "phase_margin_frequency",
            SignalUnit::Hertz,
        ),
        "dc_loop_gain" => (
            Metric(self::Metric::DcGain),
            "dc_loop_gain",
            SignalUnit::Dimensionless,
        ),
        "dc_loop_gain_db" => (Metric(self::Metric::DcDecibels), "dc_loop_gain_db", db()),
        "unity_gain_bandwidth" => (
            Metric(self::Metric::UnityBandwidth),
            "unity_gain_bandwidth",
            SignalUnit::Hertz,
        ),
        "unity_gain_crossovers" => (
            Metric(self::Metric::Crossovers),
            "unity_gain_crossovers",
            SignalUnit::Dimensionless,
        ),
        "multiple_unity_gain_crossovers" => (
            Metric(self::Metric::MultipleCrossovers),
            "multiple_unity_gain_crossovers",
            SignalUnit::Dimensionless,
        ),
        _ => return None,
    };
    Some(Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal: signal.into(),
        },
        column: Column::Stability(column),
        unit,
        noise_probe: None,
    })
}

pub(in crate::engine::control) fn resolve_scalar(
    circuit: &ControlCircuit,
    name: &str,
) -> Result<Option<ComplexValue>, ExprError> {
    let Ok((dataset, raw)) = circuit.qualified(name, 0) else {
        return Ok(None);
    };
    let Some(selected) = select(dataset, raw) else {
        return Ok(None);
    };
    if !matches!(
        selected.column,
        Column::Stability(StabilityColumn::Metric(_))
    ) && dataset.length() != 1
    {
        return Err(ExprError::InvalidArgument(format!(
            "stability vector '{name}' has multiple samples; a scalar requires one sample"
        )));
    }
    selected
        .sample(0)
        .filter(|value| value.re.is_finite() && value.im.is_finite())
        .map(Some)
        .ok_or_else(|| {
            ExprError::InvalidArgument(format!(
                "stability scalar '{name}' has no finite determination"
            ))
        })
}

pub(super) fn printed_scalar(
    selected: &Selected<'_>,
    label: &str,
    position: usize,
    line: usize,
) -> Result<Option<ControlScalar>, ControlError> {
    if !matches!(
        selected.column,
        Column::Stability(StabilityColumn::Metric(_))
    ) {
        return Ok(None);
    }
    let ControlAnalysisResult::Stability(result) = &selected.dataset.result else {
        return Err(command_error(
            line,
            "stability scalar lost its result owner",
        ));
    };
    let scalar = AnalysisResultDocument::stability_scalars(&result.result)
        .map_err(|error| command_error(line, error.to_string()))?
        .into_iter()
        .find(|scalar| scalar.name() == selected.id.signal)
        .ok_or_else(|| command_error(line, "stability scalar lost its document identity"))?;
    let scalar = crate::execution::result_document::ResultScalar::new(
        scalar.name(),
        label,
        Some(selected.unit.clone()),
        scalar.value().clone(),
    )
    .map_err(|error| command_error(line, error.to_string()))?;
    Ok(Some(ControlScalar {
        position,
        dataset: selected.dataset.name.clone(),
        scalar,
    }))
}
