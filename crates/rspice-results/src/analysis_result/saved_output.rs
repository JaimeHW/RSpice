//! Validate analysis saved-output receipts through the canonical retained-data rules.

use super::{AnalysisResult, RetainedWaveform};
use crate::dc_sweep::DcTraceView;
use crate::saved_output::SavedOutputValidationRef;

fn trace_view(waveform: &RetainedWaveform) -> DcTraceView<'_> {
    DcTraceView {
        name: &waveform.name,
        unit: waveform.unit.as_deref(),
        x: &waveform.x,
        sample_count: waveform.y.len(),
        complex: waveform.complex.is_some(),
    }
}

impl<W: AsRef<RetainedWaveform>> AnalysisResult<W> {
    pub(super) fn validate_saved_output_receipts(
        &self,
        quasi_periodic_basis: Option<&[RetainedWaveform]>,
    ) -> Result<(), String> {
        SavedOutputValidationRef {
            analysis_type: self.analysis_type,
            waveforms: self.waveforms.iter().map(AsRef::as_ref).map(trace_view),
            dc_op: self.dc_op.as_ref(),
            result_payload: self.result_payload.as_ref(),
            saved_output_receipts: &self.saved_output_receipts,
            success: self.success,
            provenance: self.provenance.as_ref(),
        }
        .validate(quasi_periodic_basis.map(|waves| waves.iter().map(trace_view)))
    }
}
