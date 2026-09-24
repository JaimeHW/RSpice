//! QPXF unit-transfer traces and finite group-delay segments from retained evidence.
use super::*;
use rspice_core::engine::{QpxfAnalysisResult, QpxfGroupDelay, QpxfOutput, QpxfQuantity};

pub(crate) struct QpxfDisplayTrace {
    pub name: String,
    pub unit: &'static str,
    pub frequencies: Vec<f64>,
    pub real: Vec<f64>,
    pub imaginary: Option<Vec<f64>>,
}
impl AnalysisResultPayload {
    pub(crate) fn qpxf_display_traces(
        response: &QpxfAnalysisResult,
    ) -> Result<Vec<QpxfDisplayTrace>, String> {
        response
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        let metadata = &response.metadata;
        let output = match &metadata.request.output {
            QpxfOutput::Voltage { positive, negative } => format!("V({positive},{negative})"),
            QpxfOutput::BranchCurrent { branch } => format!("I({branch})"),
        };
        let output_quantity = metadata.request.output.quantity();
        let mut traces = Vec::new();
        for transfer in &response.transfers {
            let source = &metadata.input_sources[transfer.input_source];
            let name = format!(
                "H({output}/{}({})) [in={:?}; out={:?}]",
                if source.quantity == QpxfQuantity::Voltage {
                    "V"
                } else {
                    "I"
                },
                source.name,
                transfer.input_lattice,
                metadata.request.output_lattice
            );
            let unit = match (output_quantity, source.quantity) {
                (QpxfQuantity::Voltage, QpxfQuantity::Voltage)
                | (QpxfQuantity::Current, QpxfQuantity::Current) => "1",
                (QpxfQuantity::Voltage, QpxfQuantity::Current) => "Ω",
                (QpxfQuantity::Current, QpxfQuantity::Voltage) => "S",
            };
            if transfer
                .values
                .iter()
                .any(|v| !v.re.hypot(v.im).is_finite())
            {
                return Err("QPXF displayed transfer magnitude overflowed".into());
            }
            let (real, imaginary) = transfer.values.iter().map(|v| (v.re, v.im)).unzip();
            traces.push(QpxfDisplayTrace {
                name: name.clone(),
                unit,
                frequencies: metadata.output_frequencies_hz.clone(),
                real,
                imaginary: Some(imaginary),
            });
            if let Some(delay) = &transfer.group_delay {
                // Separate contiguous runs prevent a plot from drawing through
                // undefined phase, a magnitude-floor gap or a numerical limit.
                let mut start = 0;
                let mut segment = 0;
                while start < delay.len() {
                    if !matches!(delay[start], QpxfGroupDelay::Finite(_)) {
                        start += 1;
                        continue;
                    }
                    let mut end = start + 1;
                    while end < delay.len() && matches!(delay[end], QpxfGroupDelay::Finite(_)) {
                        end += 1;
                    }
                    segment += 1;
                    let suffix = if start == 0 && end == delay.len() {
                        String::new()
                    } else {
                        format!(" [segment {segment}]")
                    };
                    let real = delay[start..end]
                        .iter()
                        .map(|value| match value {
                            QpxfGroupDelay::Finite(seconds) => *seconds,
                            _ => unreachable!("finite segment"),
                        })
                        .collect();
                    traces.push(QpxfDisplayTrace {
                        name: format!("group_delay({name}){suffix}"),
                        unit: "s",
                        frequencies: metadata.output_frequencies_hz[start..end].to_vec(),
                        real,
                        imaginary: None,
                    });
                    start = end;
                }
            }
        }
        Ok(traces)
    }
}
