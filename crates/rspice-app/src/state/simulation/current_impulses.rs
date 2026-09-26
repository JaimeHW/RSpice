//! Shared charge-history fixture and result-identity integration coverage.

use rspice_core::{CurrentImpulseOwner, CurrentImpulseTrace};
use rspice_results::current_impulses::CurrentImpulseHistoryEvidence;

pub(crate) fn current_impulse_history_fixture() -> CurrentImpulseHistoryEvidence {
    CurrentImpulseHistoryEvidence {
        start_time_s: 0.0,
        stop_time_s: 1.0,
        delivery_complete: true,
        traces: vec![CurrentImpulseTrace {
            owner: CurrentImpulseOwner::Branch {
                branch_name: "V1".into(),
            },
            complete: true,
            points: vec![rspice_core::CurrentImpulsePoint {
                time: 0.3,
                charge_coulombs: -0.002,
            }],
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType};

    fn analysis(history: Option<CurrentImpulseHistoryEvidence>) -> AnalysisResult {
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
        let Some(history) = history else {
            return analysis;
        };
        analysis.with_result_payload(AnalysisResultPayload::TransientEvents {
            digital_traces: vec![],
            real_traces: vec![],
            digital_buses: vec![],
            current_impulses: Some(history),
        })
    }

    #[test]
    fn current_impulse_history_identity_preserves_charge_time_owner_and_coverage() {
        let history = current_impulse_history_fixture();
        history.validate().unwrap();
        let text = serde_json::to_string(&history).unwrap();
        let restored: CurrentImpulseHistoryEvidence = serde_json::from_str(&text).unwrap();
        assert_eq!(history, restored);
        let digest = analysis(Some(history.clone())).result_data_digest();
        for change in 0..5 {
            let mut changed = history.clone();
            match change {
                0 => changed.traces[0].points[0].time = 0.4,
                1 => changed.traces[0].points[0].charge_coulombs = 0.002,
                2 => changed.traces[0].complete = false,
                3 => changed.delivery_complete = false,
                _ => {
                    changed.traces[0].owner = CurrentImpulseOwner::DeviceLead {
                        device_name: "Q1".into(),
                        parameter: "ic".into(),
                    }
                }
            }
            assert_ne!(digest, analysis(Some(changed)).result_data_digest());
        }
        let mut zero = history;
        zero.traces[0].points.clear();
        zero.validate().unwrap();
        assert_ne!(
            analysis(None).result_data_digest(),
            analysis(Some(zero)).result_data_digest()
        );
    }
}
