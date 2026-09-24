//! Persistence of the objects a failed run named.
//!
//! An attribution is evidence about one run at the topology it ran against,
//! so it is written with the result rather than rebuilt on load. What this
//! covers is the only way that can go wrong quietly: a project written
//! before attributions existed must still open, naming none.

use super::*;

/// Every project written before the engine could name a failure's objects
/// omits the field entirely, and must read back as "it named none" rather
/// than failing to load.
#[test]
fn a_result_written_without_an_attribution_still_loads() {
    let without: ProjectAnalysisResult = serde_json::from_str(
        r#"{"id":1,"analysis_type":"Transient","label":"TRAN","timestamp":0.0,"success":false,
            "error_message":"Analysis failed"}"#,
    )
    .expect("a document predating attributions must load");
    assert!(without.failure_attribution.is_none());

    let with = ProjectAnalysisResult {
        failure_attribution: Some(crate::state::ConvergenceAttribution::from(
            &rspice_core::diagnostics::ConvergenceDiagnostic {
                class: rspice_core::diagnostics::ConvergenceFailureClass::SingularSystem,
                sites: vec![rspice_core::diagnostics::ConvergenceSite {
                    name: "OUT".to_owned(),
                    kind: rspice_core::diagnostics::ConvergenceSiteKind::Node,
                    residual: None,
                }],
                elided_sites: 3,
                failure_message: "matrix is singular".to_owned(),
            },
        )),
        ..without
    };
    let round_tripped: ProjectAnalysisResult =
        serde_json::from_str(&serde_json::to_string(&with).expect("serializes"))
            .expect("round trips");
    assert_eq!(round_tripped, with);
}
