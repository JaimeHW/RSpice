//! Regenerate a solved display basis without trusting authored saved-output labels.
//! Full signed numerical evidence survives any plot selection. Present engine
//! traces must still be exact projections; additional plots need saved receipts.
use super::*;
use std::sync::Arc;

impl AnalysisResultPayload {
    pub fn retained_waveform_basis(&self) -> Result<Option<Vec<RetainedWaveform>>, String> {
        if let Self::Stb { response } = self {
            return Self::stb_waveforms(response).map(Some);
        }
        let mut traces = match self {
            AnalysisResultPayload::Qpss { operating_point } => {
                AnalysisResultPayload::qpss_display_traces(operating_point)?
            }
            AnalysisResultPayload::Qpac { response } => {
                AnalysisResultPayload::qpac_display_traces(response)?
                    .into_iter()
                    .map(|(name, unit, values)| {
                        let (real, imaginary) = values.iter().map(|v| (v.re, v.im)).unzip();
                        crate::analysis_payload::QpxfDisplayTrace {
                            name,
                            unit,
                            frequencies: response.metadata.request.offsets_hz.clone(),
                            real,
                            imaginary: Some(imaginary),
                        }
                    })
                    .collect()
            }
            AnalysisResultPayload::Qpxf { response } => {
                AnalysisResultPayload::qpxf_display_traces(response)?
            }
            AnalysisResultPayload::Qpnoise { response } => {
                AnalysisResultPayload::qpnoise_display_traces(response)?
            }
            _ => return Ok(None),
        };
        traces.sort_by(|a, b| a.name.cmp(&b.name));
        let mut waveforms = Vec::new();
        for trace in traces {
            let x = Arc::new(trace.frequencies);
            if let Some(imaginary) = trace.imaginary {
                let magnitude: Vec<_> = trace
                    .real
                    .iter()
                    .zip(&imaginary)
                    .map(|(r, i)| r.hypot(*i))
                    .collect();
                let phase: Vec<_> = trace
                    .real
                    .iter()
                    .zip(&imaginary)
                    .map(|(r, i)| i.atan2(*r).to_degrees())
                    .collect();
                waveforms.push(
                    RetainedWaveform::new(format!("|{}|", trace.name), x.clone(), magnitude)
                        .with_unit(trace.unit)
                        .with_complex_components(trace.name.clone(), trace.real, imaginary),
                );
                waveforms.push(
                    RetainedWaveform::new(format!("phase({})", trace.name), x, phase)
                        .with_unit("°"),
                );
            } else {
                waveforms
                    .push(RetainedWaveform::new(trace.name, x, trace.real).with_unit(trace.unit));
            }
        }
        Ok(Some(waveforms))
    }
}
impl<W: AsRef<RetainedWaveform>> AnalysisResult<W> {
    pub(super) fn validate_retained_display_basis(
        &self,
        expected: Option<&[RetainedWaveform]>,
    ) -> Result<(), String> {
        let Some(expected) = expected else {
            return Ok(());
        };
        let canonical: std::collections::HashMap<_, _> =
            expected.iter().map(|w| (w.name.as_str(), w)).collect();
        let authored: std::collections::HashSet<_> = self
            .saved_output_receipts
            .iter()
            .flat_map(|r| r.status.materialized_waveforms())
            .map(|(name, _)| name)
            .collect();
        for actual in self.waveforms.iter().map(AsRef::as_ref) {
            if let Some(expected) = canonical.get(actual.name.as_str()) {
                if actual.x != expected.x
                    || actual.y != expected.y
                    || actual.unit != expected.unit
                    || actual.complex != expected.complex
                {
                    return Err(format!(
                        "retained trace '{}' differs from its retained numerical evidence",
                        actual.name
                    ));
                }
            } else if !authored.contains(actual.name.as_str()) {
                return Err(format!(
                    "retained trace '{}' is absent from the solved basis and saved-output receipts",
                    actual.name
                ));
            }
        }
        Ok(())
    }
}
