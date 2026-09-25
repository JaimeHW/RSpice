//! SOA evidence crosses the worker boundary without losing duration or derating data.

use super::*;
use crate::results::safety::{SoaDurationEvidence, SoaPowerDerating};
use rspice_results::safety::SoaPowerDeratingEvidence;

#[test]
fn soa_duration_and_derating_evidence_survive_worker_json() {
    let evaluation = SoAEvaluation {
        envelope: None,
        duration: Some(SoaDurationEvidence {
            cumulative: None,
            minimum_duration_s: 2.0,
            total_exceedance_s: 3.0,
            longest_excursion_s: 2.5,
            qualified_excursions: 1,
            rejected_excursions: 1,
            clipped_excursions: 0,
        }),
        thresholds: Default::default(),
        derating: Some(SoaPowerDeratingEvidence {
            rated_power_w: 1.0,
            curve: SoaPowerDerating {
                reference_temperature_kelvin: 300.0,
                watts_per_kelvin: 0.01,
            },
        }),
        device_id: "Q1".into(),
        parameter: SoAParameter::Pdiss,
        limit_value: 0.5,
        worst_actual_value: 0.8,
        worst_time: 2.0,
        sample_count: 3,
        unit: "W".into(),
        description: "Rated power".into(),
        verdict: SoARuleVerdict::Critical,
    };

    let wire = WorkerSoAEvaluation::from(evaluation.clone());
    let bytes = serde_json::to_vec(&wire).expect("serialize worker SOA evidence");
    let restored: WorkerSoAEvaluation =
        serde_json::from_slice(&bytes).expect("deserialize worker SOA evidence");
    assert_eq!(SoAEvaluation::from(restored), evaluation);
}
