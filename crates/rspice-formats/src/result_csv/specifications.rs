//! Exact specification report CSV over the retained run's own requirement contract.
use crate::table::escape_csv_field as csv_field;
use rspice_results::{
    analysis_result::AnalysisResult,
    run::SimulationRun,
    specification::{
        SpecEntry,
        report::{resolved_specifications, result_rows},
    },
    waveform::RetainedWaveform,
};

/// Encoded specification rows and the count used by their publisher.
#[derive(Debug)]
pub struct EncodedSpecificationCsv {
    pub contents: String,
    pub row_count: usize,
}

/// Encode the frozen requirement set, using workspace requirements only for legacy runs.
pub fn encode_specification_csv<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
    run: &SimulationRun<A>,
    workspace_specs: &[SpecEntry],
) -> EncodedSpecificationCsv {
    let specs = resolved_specifications(run, workspace_specs);
    let rows = result_rows(run, &specs);
    let mut contents = String::from(
        "measurement,expression,value,minimum,maximum,limit,margin,unit,scope,worst_corner,status,detail\n",
    );
    for row in &rows {
        let spec = specs
            .iter()
            .find(|spec| spec.measurement.eq_ignore_ascii_case(&row.measurement));
        let value = row.value.map(|value| format!("{value:.17e}"));
        let minimum = spec
            .and_then(|spec| spec.min)
            .map(|value| format!("{value:.17e}"));
        let maximum = spec
            .and_then(|spec| spec.max)
            .map(|value| format!("{value:.17e}"));
        let margin = row.margin.map(|value| format!("{value:.17e}"));
        let scope = spec
            .map(|spec| serde_json::to_string(&spec.scope).unwrap_or_else(|_| "null".to_owned()))
            .unwrap_or_default();
        contents.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_field(&row.measurement),
            csv_field(&row.expression),
            csv_field(value.as_deref().unwrap_or_default()),
            csv_field(minimum.as_deref().unwrap_or_default()),
            csv_field(maximum.as_deref().unwrap_or_default()),
            csv_field(&row.limit),
            csv_field(margin.as_deref().unwrap_or_default()),
            csv_field(&row.unit),
            csv_field(&scope),
            csv_field(row.worst_corner.as_deref().unwrap_or_default()),
            row.status.label(),
            csv_field(&row.detail),
        ));
    }
    EncodedSpecificationCsv {
        contents,
        row_count: rows.len(),
    }
}
