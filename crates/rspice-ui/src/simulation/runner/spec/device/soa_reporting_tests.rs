//! Device-reporting tests retain between-sample SOA violations across worker transport.

use super::*;
use crate::results::safety::SoAParameter;
use crate::state::{AnalysisResultPayload, AnalysisType};
use rspice_core::abort_signal::NoAbort;

#[test]
fn soa_reporting_preserves_between_sample_violations_through_worker_and_retention() {
    let run = |options: &str| {
        let deck = format!(
            "SOA reporting\nVg g 0 PWL(0 .1 4n .1 5n 3 6n .1 10n .1)\nVd d 0 1\nM1 d g 0 0 nm W=1u L=1u\n.model nm NMOS LEVEL=1 VTO=.5 KP=.001\n{options}\n.end\n"
        );
        run_soa(
            &deck,
            false,
            Default::default(),
            vec![svc_runner::SoaRuleConfig {
                parameter: SoAParameter::Vgs,
                max_value: 1.8,
                devices: vec!["M1".into()],
                models: vec![],
                minimum_duration_s: Some(0.25e-9),
                duration_mode: Default::default(),
                voltage_basis: Default::default(),
                current_envelope: None,
                power_derating: None,
            }],
            10e-9,
            0.1e-9,
            false,
            1.8,
            false,
            3.3,
            false,
            0.9,
            false,
            5.0,
            None,
            &NoAbort,
        )
        .unwrap()
    };
    let controller = crate::simulation::SimulationController::new();
    let retain = |result| {
        controller.convert_to_analysis_result_with_metadata_owned(
            result,
            AnalysisType::Soa,
            "SOA reporting",
        )
    };
    let full = retain(run(""));
    assert!(full.success, "{:?}", full.error_message);
    let Some(AnalysisResultPayload::Soa {
        evaluations: baseline_evaluations,
        violations: baseline_violations,
        source_history: None,
    }) = &full.result_payload
    else {
        panic!("full SOA");
    };
    assert!(!baseline_violations.is_empty());
    // Absence remains byte-compatible with projects that predate reporting views.
    assert!(
        !serde_json::to_string(full.result_payload.as_ref().unwrap())
            .unwrap()
            .contains("source_history")
    );

    for (options, expected) in [
        (
            ".options OUTPUT INITIAL_INTERVAL=4n",
            vec![0.0, 4e-9, 8e-9, 10e-9],
        ),
        (
            ".options OUTPUT OUTPUTTIMEPOINTS=2.73n,7.35n",
            vec![2.73e-9, 7.35e-9, 10e-9],
        ),
    ] {
        let result = run(options);
        let restored =
            crate::simulation::runner::worker_contract::round_trip_response_for_test(result);
        let mut retained = retain(restored);
        assert!(retained.success, "{:?}", retained.error_message);
        retained.validate_retained_evidence().unwrap();
        let Some(AnalysisResultPayload::Soa {
            evaluations,
            violations,
            source_history: Some(source),
        }) = &retained.result_payload
        else {
            panic!("reported SOA");
        };
        assert_eq!(evaluations, baseline_evaluations);
        assert_eq!(violations, baseline_violations);
        assert_eq!(retained.family_metadata, full.family_metadata);
        for wave in &source.waveforms {
            let baseline = full.waveforms.iter().find(|w| w.name == wave.name).unwrap();
            assert_eq!(source.time, *baseline.x);
            assert_eq!(wave.values, *baseline.y);
        }
        for wave in &retained.waveforms {
            assert_eq!(wave.x.len(), expected.len());
            for (actual, expected) in wave.x.iter().zip(&expected) {
                assert!((actual - expected).abs() < 1e-22);
            }
        }
        let stress = retained
            .waveforms
            .iter()
            .find(|w| w.name == "SOA_VGS(M1)")
            .unwrap();
        assert!(stress.y.iter().all(|v| *v < 1.8));
        let counts = retained
            .waveforms
            .iter()
            .find(|w| w.name == "SOA_VIOLATION_COUNT")
            .unwrap();
        assert!(counts.y.iter().all(|v| v.fract() == 0.0));
        assert_eq!(counts.y.last().copied(), Some(violations.len() as f64));
        let json = serde_json::to_string(&retained.result_payload).unwrap();
        retained.result_payload = serde_json::from_str(&json).unwrap();
        retained.validate_retained_evidence().unwrap();
        let before = retained.result_payload.clone();
        let Some(AnalysisResultPayload::Soa {
            source_history: Some(source),
            ..
        }) = &mut retained.result_payload
        else {
            unreachable!()
        };
        let source = std::sync::Arc::make_mut(source);
        let stress = source
            .waveforms
            .iter_mut()
            .find(|w| w.name == "SOA_VGS(M1)")
            .unwrap();
        let worst = stress
            .values
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .unwrap()
            .0;
        stress.values[worst] = 0.1;
        assert!(
            retained.validate_retained_evidence().is_err(),
            "altered peak must not authenticate"
        );
        retained.result_payload = before;
        std::sync::Arc::make_mut(&mut retained.waveforms[0].y)[0] += 0.25;
        assert!(
            retained.validate_retained_evidence().is_err(),
            "altered report must not authenticate"
        );
    }
}
