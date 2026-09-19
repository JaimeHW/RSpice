//! What the Contribution sheet prints, for both families it serves.
//!
//! The sheet ranks contributions to one number, and two analyses produce
//! that shape: a sensitivity report and a DC mismatch spread. A printed page
//! cannot be hovered for the rest of the story, so each has to carry the
//! total its ranked list divides.

use super::*;

/// A DC mismatch sheet prints the spread and its contributor table.
///
/// Two tables rather than one: a ranked list read without the total it
/// divides says nothing, and a printed page cannot be hovered for the rest.
#[test]
fn a_dc_mismatch_sheet_prints_its_contributor_table() {
    use crate::state::{
        DcMismatchContributorEvidence, DcMismatchEvidence, DcMismatchScopeEvidence,
    };

    let payload = AnalysisResultPayload::DcMismatch {
        evidence: std::sync::Arc::new(DcMismatchEvidence {
            output: "V(OUT)".to_owned(),
            output_unit: "V".to_owned(),
            nominal_value: 0.5,
            sigma_multiplier: 3.0,
            sigma_total: 2.0e-3,
            sigma_mismatch: 2.0e-3,
            sigma_process: 0.0,
            include_mismatch: true,
            include_process: false,
            contributor_limit: 2,
            threshold: 0.0,
            normalized_contributions: true,
            applied_correlations_mismatch: 0,
            applied_correlations_process: 0,
            evaluated_contributors: 6,
            contributors: vec![
                DcMismatchContributorEvidence {
                    instance: "R1".to_owned(),
                    parameter: "R1V".to_owned(),
                    scope: DcMismatchScopeEvidence::Mismatch,
                    sigma_parameter: 10.0,
                    sensitivity: 2.0e-4,
                    contribution: 2.0e-3,
                    share: 0.75,
                },
                DcMismatchContributorEvidence {
                    instance: "R2".to_owned(),
                    parameter: "R2V".to_owned(),
                    scope: DcMismatchScopeEvidence::Mismatch,
                    sigma_parameter: 10.0,
                    sensitivity: -1.0e-4,
                    contribution: -1.0e-3,
                    share: -0.25,
                },
            ],
        }),
    };
    let analysis = AnalysisResult::new(1, AnalysisType::DcMismatch, "DCMATCH")
        .with_result_payload(payload.clone());
    let summary = semantic_result_summary(ResultViewer::Contribution, &analysis)
        .expect("a DC mismatch payload has a semantic table");
    assert_eq!(summary.payload, Some(payload));
    assert_eq!(summary.tables.len(), 2);
    assert_eq!(summary.tables[0].title, "DC mismatch of V(OUT)");
    assert_eq!(
        summary.tables[0].rows[1],
        vec!["Sigma total".to_owned(), exact_number(2.0e-3)]
    );
    assert_eq!(
        summary.tables[0].rows[5],
        vec!["Retained".to_owned(), "2 of 6 evaluated".to_owned()]
    );
    assert_eq!(summary.tables[1].title, "Contributors");
    assert_eq!(
        summary.tables[1].rows[0],
        vec![
            "1".to_owned(),
            "R1".to_owned(),
            "R1V".to_owned(),
            "mismatch".to_owned(),
            exact_number(2.0e-3),
            exact_number(0.75),
            exact_number(0.75),
        ]
    );
    // The signed cumulative sum, never renormalized.
    assert_eq!(summary.tables[1].rows[1][5], exact_number(-0.25));
    assert_eq!(summary.tables[1].rows[1][6], exact_number(0.5));

    // And the refusal names the family that is actually missing.
    let bare = AnalysisResult::new(2, AnalysisType::DcMismatch, "DCMATCH");
    assert!(semantic_result_summary(ResultViewer::Contribution, &bare).is_err());
}
