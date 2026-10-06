use super::*;
use clap::Parser;

#[test]
fn gp_workloads_preserve_results_and_measure_both_storage_contracts() {
    for model in [
        GpTransientPhaseModel::ExactDelay,
        GpTransientPhaseModel::NgspiceWeil,
    ] {
        let mut peaks = Vec::new();
        for checkpoints in [false, true] {
            let workload = Workload::new(2, 256, model, checkpoints).unwrap();
            let engine = Engine::new(workload.config.clone());
            let (_, waveform) = workload.measure(&engine).unwrap();
            assert!(waveform.points >= 257);
            let cancelled = workload.cancel(&engine).unwrap();
            assert!(cancelled.accepted_samples > 0);
            assert!(cancelled.request_simulation_time_seconds >= workload.cancel_at);
            assert!(cancelled.request_simulation_time_seconds < workload.stop);
            assert_eq!(cancelled.accepted_after_request, 0);
            assert!(cancelled.polls_after_request > 0);
            assert_eq!(workload.measure(&engine).unwrap().1, waveform);
            peaks.push(workload.peak_transport_bytes().unwrap());
        }
        if model == GpTransientPhaseModel::ExactDelay {
            assert!(peaks[0] > 0);
            assert!(peaks[1] > peaks[0], "retained checkpoints must be charged");
        } else {
            assert_eq!(peaks, [0, 0]);
        }
    }
}

#[test]
fn gp_benchmark_rejects_empty_or_inapplicable_measurement_sizes() {
    for args in [
        vec!["--samples", "0"],
        vec!["--samples", "1"],
        vec!["--devices", "0"],
        vec!["--steps", "255"],
        vec!["--devices", ""],
    ] {
        let mut command = vec!["rspice-bench", "gp-transient"];
        command.extend(args);
        assert!(crate::Cli::try_parse_from(command).is_err());
    }
}

#[test]
fn gp_budget_failure_is_retained_in_an_immutable_report() {
    let directory = tempfile::tempdir().unwrap();
    let args = GpTransientArgs {
        samples: 2,
        devices: vec![1],
        steps: 256,
        out: directory.path().join("report.json"),
        exploratory: true,
        max_run_ms: None,
        max_cancel_ms: None,
        max_transport_bytes: Some(0),
    };
    assert_eq!(run(&args).unwrap(), ExitCode::FAILURE);
    let report: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&args.out).unwrap()).unwrap();
    assert_eq!(report["payload"]["passed"], false);
    let cases = report["payload"]["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 4);
    assert!(cases.iter().any(|case| case["passed"] == true));
    assert!(cases.iter().any(|case| case["passed"] == false));
    assert!(run(&args).is_err());
}
