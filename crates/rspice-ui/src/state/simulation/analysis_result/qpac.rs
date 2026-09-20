//! Exact QPAC display traces derived from retained complex MNA responses.
use super::*;
use rspice_core::engine::{QpacAnalysisResult, QpacInputQuantity};

type Trace = (String, &'static str, Vec<rspice_core::Complex64>);

impl AnalysisResultPayload {
    pub(crate) fn qpac_display_traces(result: &QpacAnalysisResult) -> Result<Vec<Trace>, String> {
        let grid = result
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        let metadata = &result.metadata;
        let tuple = &metadata.request.output_lattice;
        let index = grid.index_of(tuple).expect("validated QPAC tuple");
        let drive = metadata.drive();
        let suffix = format!(" [k={tuple:?}]");
        let mut traces = Vec::new();
        for row in 0..metadata.node_names.len() + metadata.branch_names.len() {
            let (name, unit) = if row < metadata.node_names.len() {
                (format!("V({}){suffix}", metadata.node_names[row]), "V")
            } else {
                (
                    format!(
                        "I({}){suffix}",
                        metadata.branch_names[row - metadata.node_names.len()]
                    ),
                    "A",
                )
            };
            let values: Vec<_> = result
                .unit_solutions
                .iter()
                .map(|s| s.spectra[row][index] * drive)
                .collect();
            if values
                .iter()
                .any(|v| !v.re.is_finite() || !v.im.is_finite() || !v.norm().is_finite())
            {
                return Err("QPAC displayed response overflowed at the authored drive".into());
            }
            traces.push((name, unit, values));
        }
        let output = format!(
            "V({},{})",
            metadata.request.output_node, metadata.request.output_ref
        );
        let (input_unit, gain_unit) = match metadata.input_quantity {
            QpacInputQuantity::Voltage => ("V", "1"),
            QpacInputQuantity::Current => ("I", "Ω"),
        };
        traces.push((
            format!("{output}{suffix}"),
            "V",
            result.output_response.clone(),
        ));
        traces.push((
            format!(
                "H({output}/{input_unit}({})) [in={:?}; out={tuple:?}]",
                metadata.request.input_source, metadata.request.input_lattice
            ),
            gain_unit,
            result.output_transfer.clone(),
        ));
        if traces
            .iter()
            .flat_map(|(_, _, values)| values)
            .any(|v| !v.norm().is_finite())
        {
            return Err("QPAC displayed magnitude overflowed".into());
        }
        Ok(traces)
    }
}
