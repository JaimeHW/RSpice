//! Retaining one `.SENS` study as a result.
//!
//! Layer: controller, result retention. A study produces no waveform: what it
//! produces is one column of derivatives per variable per solved point, and
//! every one of those numbers is the engine's. So the whole answer is the
//! payload, and this file only decides whether the run's evidence is evidence.
//!
//! The validation is the engine's own rules restated by calling the
//! evidence's validator rather than re-deriving them here. A study that fails
//! it is a failed result with the sentence that refused it, never a sheet
//! ranking numbers that answer no question.

use std::sync::Arc;

use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType, SensitivityStudyEvidence};

/// The retained result one sensitivity study becomes.
pub(super) fn analysis_result(
    analysis_type: AnalysisType,
    label: &str,
    evidence: Arc<SensitivityStudyEvidence>,
) -> AnalysisResult {
    let payload = AnalysisResultPayload::SensitivityStudy { evidence };
    match payload.validate_for(analysis_type) {
        Ok(()) => {
            AnalysisResult::new(1, analysis_type, label.to_string()).with_result_payload(payload)
        }
        Err(error) => AnalysisResult::failed(
            1,
            analysis_type,
            label.to_string(),
            format!("Invalid retained analysis payload: {error}"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{SensitivityBasisEvidence, SensitivityStudyRow};
    use rspice_core::analysis::sensitivity::SensitivityValue;

    fn evidence() -> SensitivityStudyEvidence {
        SensitivityStudyEvidence {
            output: "I(V1)".to_owned(),
            filter: "PARAM:*".to_owned(),
            basis: SensitivityBasisEvidence::Dc { output: -2.0 },
            rows: vec![SensitivityStudyRow {
                parameter: "PARAM:DRIVE".to_owned(),
                nominal_value: 2.0,
                raw: vec![SensitivityValue::Available(-0.5)],
                normalized: vec![SensitivityValue::Available(0.5)],
                phase: Vec::new(),
            }],
        }
    }

    #[test]
    fn a_valid_study_is_retained_whole_and_an_invalid_one_fails_with_its_reason() {
        let retained = analysis_result(AnalysisType::Sensitivity, "SENS", Arc::new(evidence()));
        assert!(retained.success);
        let Some(AnalysisResultPayload::SensitivityStudy { evidence: kept }) =
            retained.result_payload.as_ref()
        else {
            panic!("the study is the whole result");
        };
        assert_eq!(kept.filter, "PARAM:*");
        assert_eq!(kept.rows.len(), 1);

        // A column that does not span the grid is not the engine's answer.
        let mut broken = evidence();
        broken.rows[0].normalized.clear();
        let failed = analysis_result(AnalysisType::Sensitivity, "SENS", Arc::new(broken));
        assert!(!failed.success);
        assert!(
            failed
                .error_message
                .as_deref()
                .is_some_and(|error: &str| error.contains("Invalid retained analysis payload")),
            "{:?}",
            failed.error_message
        );
    }

    /// A study attached to another analysis is refused rather than rendered
    /// under a sheet that would read it as something else.
    #[test]
    fn a_study_under_the_wrong_analysis_type_is_refused() {
        let wrong = analysis_result(AnalysisType::Ac, "AC", Arc::new(evidence()));
        assert!(!wrong.success);
    }
}
