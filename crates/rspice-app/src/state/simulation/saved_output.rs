//! Validate application saved-output receipts through the canonical retained-data rules.

use crate::state::{AnalysisResult, WaveformData};
use rspice_results::dc_sweep::DcTraceView;
use rspice_results::saved_output::SavedOutputValidationRef;

fn trace_view(waveform: &WaveformData) -> DcTraceView<'_> {
    DcTraceView {
        name: &waveform.name,
        unit: waveform.unit.as_deref(),
        x: &waveform.x,
        sample_count: waveform.y.len(),
        complex: waveform.complex.is_some(),
    }
}

impl AnalysisResult {
    pub(in crate::state::simulation) fn validate_saved_output_receipts(
        &self,
        quasi_periodic_basis: Option<&[WaveformData]>,
    ) -> Result<(), String> {
        SavedOutputValidationRef {
            analysis_type: self.analysis_type,
            waveforms: self.waveforms.iter().map(trace_view),
            dc_op: self.dc_op.as_ref(),
            result_payload: self.result_payload.as_ref(),
            saved_output_receipts: &self.saved_output_receipts,
            success: self.success,
            provenance: self.provenance.as_ref(),
        }
        .validate(quasi_periodic_basis.map(|waves| waves.iter().map(trace_view)))
    }
}
