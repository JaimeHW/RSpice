use super::super::convergence::ConvergenceOptions;
use super::super::multi_run::{AnalysisRunType, AnalysisSpec, FrequencySweep, RunStatus};
use super::*;

// =========================================================================
// ExecutionState Tests
// =========================================================================

#[test]
fn test_execution_state_default() {
    let state = ExecutionState::default();
    assert_eq!(state.status, ExecutionStatus::Idle);
    assert_eq!(state.total_runs, 0);
}

#[test]
fn test_execution_state_progress() {
    let mut state = ExecutionState::default();
    state.total_runs = 10;
    state.completed_runs = 5;

    assert!((state.progress_percent() - 50.0).abs() < 0.1);
}

#[test]
fn test_execution_state_is_complete() {
    let mut state = ExecutionState::default();
    assert!(!state.is_complete());

    state.status = ExecutionStatus::Completed;
    assert!(state.is_complete());
}

#[test]
fn test_execution_state_update_eta() {
    let mut state = ExecutionState::default();
    state.total_runs = 10;
    state.completed_runs = 5;
    state.elapsed_seconds = 10.0;

    state.update_eta();
    assert!(state.eta_seconds.is_some());
    assert!((state.eta_seconds.unwrap() - 10.0).abs() < 0.1); // 2s per run * 5 remaining
}

// =========================================================================
// ExecutionResult Tests
// =========================================================================

#[test]
fn test_execution_result_default() {
    let result = ExecutionResult::default();
    assert!(result.results.is_empty());
    assert!(result.errors.is_empty());
}

#[test]
fn test_execution_result_success_count() {
    let mut result = ExecutionResult::default();
    result.results.insert(
        1,
        MappedResult {
            status: ResultStatus::Success,
            ..Default::default()
        },
    );
    result.results.insert(
        2,
        MappedResult {
            status: ResultStatus::Error,
            ..Default::default()
        },
    );

    assert_eq!(result.success_count(), 1);
}

// =========================================================================
// RunExecutor Tests
// =========================================================================

#[test]
fn test_executor_new() {
    let executor = RunExecutor::new();
    assert!(!executor.is_cancelled());
    assert_eq!(executor.current_progress(), 0);
}

#[test]
fn test_executor_cancel() {
    let executor = RunExecutor::new();
    executor.cancel();
    assert!(executor.is_cancelled());

    executor.reset();
    assert!(!executor.is_cancelled());
}

#[test]
fn test_executor_with_parallel() {
    let executor = RunExecutor::new().with_parallel(4);
    assert_eq!(executor.max_parallel, 4);
}

#[test]
fn test_execute_empty_queue() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new();

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.status, ExecutionStatus::Completed);
    assert_eq!(result.state.total_runs, 0);
}

#[test]
fn test_execute_single_run() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new();
    queue.add(AnalysisRunType::DcOp);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(result.state.completed_runs > 0);
}

// =========================================================================
// AsyncRunExecutor Tests
// =========================================================================

#[test]
fn test_async_executor_new() {
    let executor = AsyncRunExecutor::new();
    assert!(executor.callback.is_none());
}

#[test]
fn test_async_executor_cancel() {
    let executor = AsyncRunExecutor::new();
    executor.cancel();
    assert!(executor.executor.is_cancelled());
}

// =========================================================================
// Phase 2: Run Executor Integration Tests
// =========================================================================

#[test]
fn test_queue_netlist_builder() {
    let queue = RunQueue::new().with_netlist("* Test circuit\nR1 in out 1k\n");
    assert_eq!(queue.netlist(), Some("* Test circuit\nR1 in out 1k\n"));
}

#[test]
fn test_queue_set_netlist() {
    let mut queue = RunQueue::new();
    assert!(queue.netlist().is_none());

    queue.set_netlist("V1 vdd 0 1.8");
    assert!(queue.netlist().is_some());
    assert!(queue.netlist().unwrap().contains("V1"));
}

#[test]
fn test_execute_without_netlist_fails() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new();
    queue.add(AnalysisRunType::DcOp);

    // Execute without netlist - should handle gracefully (fail the run)
    let result = executor.execute(&mut queue);
    // The run completes but with failure since no netlist
    assert_eq!(result.state.total_runs, 1);
}

#[test]
fn test_execute_with_valid_netlist() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new()
        .with_netlist("* Simple RC circuit\nV1 in 0 DC 1\nR1 in out 1k\nC1 out 0 1p\n.op\n");
    queue.add(AnalysisRunType::DcOp);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    // With valid netlist, simulation should attempt to run
    assert!(result.state.completed_runs >= 1 || result.state.failed_runs >= 1);
}

#[test]
fn test_analysis_type_mapping_coverage() {
    use super::super::multi_run::AnalysisRunType;
    use super::super::result_mapper::MappedAnalysisType;

    let executor = RunExecutor::new();

    // Test all AnalysisRunType variants map correctly
    let mappings = [
        (AnalysisRunType::DcOp, MappedAnalysisType::DcOp),
        (AnalysisRunType::DcSweep, MappedAnalysisType::DcSweep),
        (AnalysisRunType::Ac, MappedAnalysisType::Ac),
        (AnalysisRunType::Disto, MappedAnalysisType::Disto),
        (AnalysisRunType::Transient, MappedAnalysisType::Transient),
        (AnalysisRunType::Noise, MappedAnalysisType::Noise),
        (AnalysisRunType::PoleZero, MappedAnalysisType::PoleZero),
        (AnalysisRunType::Pxf, MappedAnalysisType::Pxf),
        (AnalysisRunType::Pstb, MappedAnalysisType::Pstb),
        (AnalysisRunType::Stb, MappedAnalysisType::Stb),
        (
            AnalysisRunType::Reliability,
            MappedAnalysisType::Reliability,
        ),
        (
            AnalysisRunType::Optimization,
            MappedAnalysisType::Optimization,
        ),
        (AnalysisRunType::Soa, MappedAnalysisType::Soa),
        (AnalysisRunType::Envelope, MappedAnalysisType::Envelope),
        (AnalysisRunType::Fourier, MappedAnalysisType::Fourier),
    ];

    for (run_type, expected) in mappings {
        let mapped = executor.map_analysis_type(run_type);
        assert_eq!(mapped, expected, "Mapping failed for {:?}", run_type);
    }
}

#[test]
fn test_execute_multiple_analyses_with_netlist() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new()
        .with_netlist("* RC\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1p\n.op\n.tran 1n 10n\n");

    queue.add(AnalysisRunType::DcOp);
    queue.add(AnalysisRunType::Transient);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 2);
}

#[test]
fn test_inject_options_block_before_end_inserts_before_end_directive() {
    let netlist = "* test\nV1 in 0 1\nR1 in 0 1k\n.END\n";
    let options_block = ".OPTIONS RELTOL=1e-4\n.TEMP 85";
    let injected = RunExecutor::inject_options_block_before_end(netlist, options_block);
    assert!(injected.contains(options_block));
    let end_pos = injected
        .to_ascii_lowercase()
        .rfind(".end")
        .expect("injected netlist should contain .end");
    let opt_pos = injected
        .find(".OPTIONS")
        .expect("injected netlist should contain .OPTIONS block");
    assert!(
        opt_pos < end_pos,
        "options block must appear before .end directive"
    );
}

#[test]
fn test_inject_options_block_before_end_appends_when_end_missing() {
    let netlist = "* test\nV1 in 0 1\nR1 in 0 1k";
    let options_block = ".OPTIONS RELTOL=1e-4\n.TEMP 85";
    let injected = RunExecutor::inject_options_block_before_end(netlist, options_block);
    assert!(injected.starts_with(netlist));
    assert!(injected.contains(options_block));
}

#[test]
fn test_with_convergence_options_populates_engine_override() {
    let mut conv = ConvergenceOptions::default();
    conv.temperature = 85.0;
    conv.tnom = 30.0;
    conv.tolerances.reltol = 2e-4;

    let executor = RunExecutor::new().with_convergence_options(&conv);
    let override_opts = executor
        .engine_options_override
        .as_ref()
        .expect("convergence options should produce engine override");
    assert!((override_opts.temp - 85.0).abs() < 1e-12);
    assert!((override_opts.tnom - 30.0).abs() < 1e-12);
    assert!((override_opts.reltol - 2e-4).abs() < 1e-15);
}

#[test]
fn test_execute_with_engine_options_injects_options_into_execution_path() {
    let netlist = "* option injection path\nV1 in 0 1\nR1 in 0 1k\n.end\n";

    let mut baseline_queue = RunQueue::new().with_netlist(netlist);
    baseline_queue.add_analysis(AnalysisSpec::DcOp);
    let baseline_result = RunExecutor::new().execute(&mut baseline_queue);
    assert!(
        baseline_result.errors.is_empty(),
        "baseline run should succeed: {:?}",
        baseline_result.errors
    );

    let mut invalid_opts = EngineOptions::engine_defaults();
    invalid_opts.reltol = f64::NAN;

    let mut override_queue = RunQueue::new().with_netlist(netlist);
    override_queue.add_analysis(AnalysisSpec::DcOp);
    let override_result = RunExecutor::new()
        .with_engine_options(invalid_opts)
        .execute(&mut override_queue);

    assert!(
        !override_result.errors.is_empty(),
        "override run should fail when injected options produce invalid netlist syntax"
    );
    let joined_errors = override_result
        .errors
        .values()
        .cloned()
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        joined_errors
            .to_ascii_lowercase()
            .contains("expected value, found identifier 'nan'"),
        "error should come from injected invalid RELTOL option, got: {joined_errors}"
    );
}

#[test]
fn test_execute_parallel_completes_dependency_graph() {
    let executor = RunExecutor::new().with_parallel(4);
    let mut queue = RunQueue::new().with_netlist("* dep\nV1 in 0 1\nR1 in out 1k\n");

    let root = queue.add(AnalysisRunType::DcOp);
    let child_a = queue.add(AnalysisRunType::DcOp);
    queue
        .get_mut(child_a)
        .expect("child_a run must exist")
        .dependencies
        .push(root);
    let child_b = queue.add(AnalysisRunType::DcOp);
    queue
        .get_mut(child_b)
        .expect("child_b run must exist")
        .dependencies
        .push(root);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 3);
    assert_eq!(result.state.failed_runs, 0);
    assert_eq!(
        queue.get(root).expect("root run must exist").status,
        RunStatus::Completed
    );
    assert_eq!(
        queue.get(child_a).expect("child_a run must exist").status,
        RunStatus::Completed
    );
    assert_eq!(
        queue.get(child_b).expect("child_b run must exist").status,
        RunStatus::Completed
    );
}

#[test]
fn test_execute_parallel_skips_dependents_on_failure() {
    let executor = RunExecutor::new().with_parallel(2);
    let mut queue = RunQueue::new().with_netlist("* fail deps\nV1 in 0 1\nR1 in out 1k\n");

    let failing = queue.add(AnalysisRunType::Ac); // Missing AnalysisSpec by design
    let dependent = queue.add(AnalysisRunType::DcOp);
    queue
        .get_mut(dependent)
        .expect("dependent run must exist")
        .dependencies
        .push(failing);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 2);
    assert_eq!(result.state.failed_runs, 1);
    assert_eq!(
        queue.get(failing).expect("failing run must exist").status,
        RunStatus::Failed
    );
    assert_eq!(
        queue
            .get(dependent)
            .expect("dependent run must exist")
            .status,
        RunStatus::Skipped
    );
    assert!(
        result
            .errors
            .values()
            .any(|err| err.contains("missing AnalysisSpec")),
        "expected an AnalysisSpec validation failure in errors: {:?}",
        result.errors
    );
}

#[test]
fn test_execute_parallel_stop_on_error_false_still_skips_blocked_dependents() {
    let executor = RunExecutor::new().with_parallel(3);
    let mut queue = RunQueue::new().with_netlist("* fail deps\nV1 in 0 1\nR1 in out 1k\n");
    queue.stop_on_error = false;

    let failing = queue.add(AnalysisRunType::Ac); // Missing AnalysisSpec by design
    let blocked = queue.add(AnalysisRunType::DcOp);
    queue
        .get_mut(blocked)
        .expect("blocked run must exist")
        .dependencies
        .push(failing);
    let independent = queue.add(AnalysisRunType::DcOp);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 3);
    assert_eq!(result.state.failed_runs, 1);
    assert_eq!(result.state.completed_runs, 3);
    assert_eq!(result.state.status, ExecutionStatus::CompletedWithErrors);
    assert_eq!(
        queue.get(failing).expect("failing run must exist").status,
        RunStatus::Failed
    );
    assert_eq!(
        queue.get(blocked).expect("blocked run must exist").status,
        RunStatus::Skipped
    );
    assert_eq!(
        queue
            .get(independent)
            .expect("independent run must exist")
            .status,
        RunStatus::Completed
    );
    assert!(
        result
            .errors
            .get(&blocked)
            .expect("blocked run should emit an error")
            .contains("Blocked by unresolved dependencies"),
        "blocked dependent should include unresolved-dependency error text"
    );
}

#[test]
fn test_execute_marks_missing_dependency_as_skipped_error() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist("* dep gap\nV1 in 0 1\nR1 in out 1k\n");
    let blocked = queue.add(AnalysisRunType::DcOp);
    queue
        .get_mut(blocked)
        .expect("blocked run must exist")
        .dependencies
        .push(999_999);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert_eq!(result.state.failed_runs, 0);
    assert_eq!(result.state.completed_runs, 1);
    assert_eq!(result.state.status, ExecutionStatus::CompletedWithErrors);
    assert_eq!(
        queue.get(blocked).expect("blocked run must exist").status,
        RunStatus::Skipped
    );
    assert!(
        result
            .errors
            .get(&blocked)
            .expect("missing dependency should be reported")
            .contains("missing"),
        "missing dependency should be part of reported skip reason"
    );
}

#[test]
fn test_execute_parallel_allows_independent_success_after_failure() {
    let executor = RunExecutor::new().with_parallel(2);
    let mut queue = RunQueue::new().with_netlist("* independent\nV1 in 0 1\nR1 in out 1k\n");

    let failing = queue.add(AnalysisRunType::Ac); // Missing AnalysisSpec by design
    let succeeding = queue.add(AnalysisRunType::DcOp);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 2);
    assert_eq!(result.state.failed_runs, 1);
    assert_eq!(result.state.completed_runs, 2);
    assert_eq!(
        queue.get(failing).expect("failing run must exist").status,
        RunStatus::Failed
    );
    assert_eq!(
        queue
            .get(succeeding)
            .expect("succeeding run must exist")
            .status,
        RunStatus::Completed
    );
}

#[test]
fn test_execute_parallel_missing_netlist_fails_all_runs_without_deadlock() {
    let executor = RunExecutor::new().with_parallel(4);
    let mut queue = RunQueue::new();
    queue.add(AnalysisRunType::DcOp);
    queue.add(AnalysisRunType::DcOp);
    queue.add(AnalysisRunType::DcOp);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 3);
    assert_eq!(result.state.failed_runs, 3);
    assert_eq!(result.state.completed_runs, 3);
    assert_eq!(result.errors.len(), 3);
    assert!(result
        .errors
        .values()
        .all(|err| err.contains("No netlist configured for queue")));
}

#[test]
fn test_parameterized_analysis_requires_spec() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist("* test\nV1 in 0 1\nR1 in out 1k\n");
    queue.add(AnalysisRunType::Ac);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.failed_runs, 1);
    assert_eq!(result.errors.len(), 1);
    let err = result
        .errors
        .values()
        .next()
        .expect("missing expected error message");
    assert!(err.contains("missing AnalysisSpec"));
}

#[test]
fn test_parameterized_analysis_with_spec_runs() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist("* test\nV1 in 0 1\nR1 in out 1k\n");
    queue.add_analysis(AnalysisSpec::Ac {
        start_freq: 1.0,
        stop_freq: 1e3,
        points_per_unit: 5,
        sweep: FrequencySweep::Decade,
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    if let Some(err) = result.errors.values().next() {
        assert!(
            !err.contains("missing AnalysisSpec"),
            "run should fail for circuit/solver reasons, not missing spec"
        );
    }
}

#[test]
fn test_disto_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new()
        .with_netlist("* disto\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
    queue.add_analysis(AnalysisSpec::Disto {
        start_freq: 1e3,
        stop_freq: 1e6,
        points_per_unit: 8,
        sweep: FrequencySweep::Decade,
        f2_over_f1: Some(1.5),
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected DISTO run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped DISTO result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Disto);
    assert!(mapped.waveforms.iter().any(|wf| wf.name.contains("THD(%)")));
    assert!(!mapped.measurements.is_empty());
    assert_eq!(mapped.status, ResultStatus::Success);
}

#[test]
fn test_pole_zero_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist("* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n");
    queue.add_analysis(AnalysisSpec::PoleZero {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "VOL".to_string(),
        analysis_type: "PZ".to_string(),
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    if let Some(err) = result.errors.values().next() {
        assert!(
            !err.contains("not implemented in RunExecutor yet"),
            "pole-zero should execute via service runner"
        );
    }
}

#[test]
fn test_pole_zero_cur_transfer_uses_transimpedance_units() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist("* pz\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n");
    queue.add_analysis(AnalysisSpec::PoleZero {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "CUR".to_string(),
        analysis_type: "PZ".to_string(),
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected successful CUR pole-zero run, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped result");
    let gain = mapped
        .measurements
        .iter()
        .find(|m| m.name == "dc_gain")
        .expect("dc_gain measurement should exist");
    assert_eq!(gain.unit, "V/A");
}

#[test]
fn test_sensitivity_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new()
        .with_netlist("* sens\n.param RVAL=1k\nV1 in 0 1\nR1 in out RVAL\nR2 out 0 1k\n");
    queue.add_analysis(AnalysisSpec::Sensitivity {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    if let Some(err) = result.errors.values().next() {
        assert!(
            !err.contains("not implemented in RunExecutor yet"),
            "sensitivity should execute via service runner"
        );
    }
}

#[test]
fn test_sensitivity_current_output_uses_current_units() {
    let executor = RunExecutor::new();
    let mut queue =
        RunQueue::new().with_netlist("* sens i\n.param RVAL=1k\nV1 in 0 1\nR1 in 0 {RVAL}\n");
    queue.add_analysis(AnalysisSpec::Sensitivity {
        output_var: "I(V1)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected successful current-output sensitivity run, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped sensitivity result");
    let raw_measurement = mapped
        .measurements
        .iter()
        .find(|m| m.name.starts_with("d(I(V1))/d("))
        .expect("expected raw current sensitivity measurement");
    assert_eq!(raw_measurement.unit, "A/unit");
}

#[test]
fn test_sensitivity_voltage_output_uses_voltage_units() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new()
        .with_netlist("* sens v\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n");
    queue.add_analysis(AnalysisSpec::Sensitivity {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: None,
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected successful voltage-output sensitivity run, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped sensitivity result");
    let raw_measurement = mapped
        .measurements
        .iter()
        .find(|m| m.name.starts_with("d(V(out))/d("))
        .expect("expected raw voltage sensitivity measurement");
    assert_eq!(raw_measurement.unit, "V/unit");
}

#[test]
fn test_sensitivity_ac_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new()
        .with_netlist("* sens ac\n.param RVAL=1k\nV1 in 0 AC 1\nR1 in out {RVAL}\nC1 out 0 1n\n");
    queue.add_analysis(AnalysisSpec::Sensitivity {
        output_var: "V(out)".to_string(),
        ac_mode: true,
        frequency: Some(1e6),
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    if let Some(err) = result.errors.values().next() {
        assert!(
            !err.contains("not supported yet"),
            "ac sensitivity should execute via service runner"
        );
    }
}

#[test]
fn test_monte_carlo_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist(
            "* mc\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.MC 8 SEED 7 DIST GAUSS SPREAD 0.02 PARAMS RVAL\n.end\n",
        );
    queue.add_analysis(AnalysisSpec::MonteCarlo);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected Monte Carlo run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped Monte Carlo result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::MonteCarlo);
    assert!(
        mapped
            .measurements
            .iter()
            .any(|m| m.name == "runs_requested" && (m.value - 8.0).abs() < 1e-12),
        "expected runs_requested measurement"
    );
}

#[test]
fn test_monte_carlo_analysis_requires_mc_command() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist("* no mc\nV1 in 0 1\nR1 in 0 1k\n");
    queue.add_analysis(AnalysisSpec::MonteCarlo);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.failed_runs, 1);
    let err = result
        .errors
        .values()
        .next()
        .expect("expected Monte Carlo configuration error");
    assert!(err.contains(".MC command"));
}

#[test]
fn test_parametric_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist(
            "* step\n.param RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.STEP PARAM RVAL 1k 4k 1k\n.end\n",
        );
    queue.add_analysis(AnalysisSpec::Parametric);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected parametric run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped parametric result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Parametric);
    assert!(!mapped.waveforms.is_empty(), "expected stepped waveforms");
    assert_eq!(mapped.waveforms[0].x.len(), 4);
}

#[test]
fn test_parametric_analysis_requires_step_command() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist("* no step\nV1 in 0 1\nR1 in 0 1k\n");
    queue.add_analysis(AnalysisSpec::Parametric);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.failed_runs, 1);
    let err = result
        .errors
        .values()
        .next()
        .expect("expected parametric configuration error");
    assert!(err.contains(".STEP command"));
}

#[test]
fn test_corner_analysis_with_temp_command_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new()
        .with_netlist("* corner\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.TEMP -40 27 125\n.end\n");
    queue.add_analysis(AnalysisSpec::Corner);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected corner run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped corner result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Corner);
    assert!(!mapped.waveforms.is_empty(), "expected corner waveforms");
    assert_eq!(mapped.waveforms[0].x.len(), 3);
}

#[test]
fn test_corner_analysis_requires_temp_command() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist("* no temp\nV1 in 0 1\nR1 in 0 1k\n");
    queue.add_analysis(AnalysisSpec::Corner);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.failed_runs, 1);
    let err = result
        .errors
        .values()
        .next()
        .expect("expected corner configuration error");
    assert!(err.contains(".TEMP"));
}

#[test]
fn test_corner_execution_preserves_netlist() {
    let executor = RunExecutor::new();
    let base_queue = RunQueue::new().with_netlist("* Test\nR1 a b 1k\n");

    let corners = vec![PvtCorner {
        process: "tt".to_string(),
        ..Default::default()
    }];

    let results = executor.execute_corners(&base_queue, &corners);
    assert_eq!(results.len(), 1);
    assert!(results.contains_key("tt"));
}

#[test]
fn test_reliability_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist(
            "* reliability\nVDD vdd 0 1.8\nVG g 0 1.2\nR1 vdd d 1k\nM1 d g 0 0 NM W=10u L=1u\n.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02\n.end\n",
        );
    queue.add_analysis(AnalysisSpec::Reliability {
        target_years: vec![1.0, 5.0, 10.0],
        enable_hci: true,
        enable_nbti: true,
        enable_em: false,
        min_stress_voltage: 0.05,
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected reliability run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped reliability result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Reliability);
    assert!(
        !mapped.waveforms.is_empty(),
        "expected reliability waveforms"
    );
    assert!(mapped
        .waveforms
        .iter()
        .any(|wf| wf.name.starts_with("DVTH(") || wf.name.starts_with("DRDS(")));
    assert!(mapped
        .measurements
        .iter()
        .any(|m| m.name == "devices_analyzed"));
}

#[test]
fn test_optimization_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist(
            "* optimization\n.param RTOP=1k\n.param RBOT=1k\nV1 in 0 2\nR1 in out {RTOP}\nR2 out 0 {RBOT}\n.end\n",
        );
    queue.add_analysis(AnalysisSpec::Optimization {
        variables: vec![super::super::multi_run::OptimizationVariable {
            name: "RBOT".to_string(),
            min: 500.0,
            max: 3000.0,
            initial: 1000.0,
        }],
        objective_node: "out".to_string(),
        objective_ref: "0".to_string(),
        goal: super::super::multi_run::OptimizationGoal::Target,
        target: Some(1.2),
        algorithm: super::super::multi_run::OptimizationAlgorithm::PatternSearch,
        max_iterations: 48,
        cost_tolerance: 1e-8,
        fd_step: 1e-4,
        initial_step: 0.2,
        min_step: 1e-8,
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected optimization run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped optimization result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Optimization);
    assert!(mapped
        .waveforms
        .iter()
        .any(|wf| wf.name == "Optimization Cost"));
    assert!(mapped.measurements.iter().any(|m| m.name == "best_cost"));
}

#[test]
fn test_soa_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new().with_netlist(
            "* soa\nVDD d 0 3.3\nVG g 0 PULSE(0 2.5 0 1n 1n 8n 16n)\nM1 d g 0 0 NM W=10u L=1u\n.model NM NMOS VTO=0.7 KP=200u LAMBDA=0.02\n.end\n",
        );
    queue.add_analysis(AnalysisSpec::Soa {
        stop_time: 32e-9,
        step_time: 1e-9,
        check_vgs_max: true,
        max_vgs: 1.2,
        check_vds_max: true,
        max_vds: 3.0,
        check_vbe_max: false,
        max_vbe: 0.9,
        check_vce_max: false,
        max_vce: 5.0,
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected SOA run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped SOA result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Soa);
    assert!(mapped
        .waveforms
        .iter()
        .any(|wf| wf.name == "SOA Violation Count"));
    assert!(mapped
        .measurements
        .iter()
        .any(|m| m.name == "num_violations"));
}

#[test]
fn test_tf_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue =
        RunQueue::new().with_netlist("* tf\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
    queue.add_analysis(AnalysisSpec::Tf);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected TF run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped TF result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Tf);
    assert!(
        mapped.waveforms.iter().any(|wf| wf.name.starts_with("H(")),
        "expected transfer-function waveform"
    );
}

#[test]
fn test_pac_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue =
        RunQueue::new().with_netlist("* pac\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
    queue.add_analysis(AnalysisSpec::Pac);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected PAC run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped PAC result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Pac);
    assert!(!mapped.waveforms.is_empty(), "expected PAC spectra");
}

#[test]
fn test_pxf_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue =
        RunQueue::new().with_netlist("* pxf\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
    queue.add_analysis(AnalysisSpec::Pxf);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected PXF run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped PXF result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Pxf);
    assert!(
        mapped
            .waveforms
            .iter()
            .any(|wf| wf.name.starts_with("H(sb")),
        "expected transfer waveform in mapped PXF result"
    );
    assert!(
        mapped
            .measurements
            .iter()
            .any(|m| m.name == "input_sideband"),
        "expected sideband measurement metadata"
    );
}

#[test]
fn test_pnoise_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue =
        RunQueue::new().with_netlist("* pnoise\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
    queue.add_analysis(AnalysisSpec::Pnoise);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected PNOISE run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped PNOISE result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Pnoise);
    assert_eq!(mapped.waveforms.len(), 1);
    assert_eq!(mapped.waveforms[0].x.len(), mapped.waveforms[0].y.len());
}

#[test]
fn test_map_pnoise_data_prefers_input_noise_for_input_reference() {
    let data = crate::services::simulation_runner::PnoiseData {
        frequencies: vec![1e3, 1e4],
        output_noise: vec![10.0, 20.0],
        input_noise: Some(vec![1.0, 2.0]),
        total_output_noise: Some(3e-6),
        contributors: vec![],
        carrier_frequency: 1e6,
        sideband_factor: 1,
        reference: crate::services::simulation_runner::PnoiseReference::Input,
        warnings: vec![],
    };

    let mapped = RunExecutor::map_pnoise_data(data);
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Pnoise);
    assert_eq!(mapped.waveforms.len(), 1);
    assert_eq!(mapped.waveforms[0].y, vec![1.0, 2.0]);
    assert_eq!(mapped.waveforms[0].y_label, "Input-Referred Noise");
    assert_eq!(mapped.waveforms[0].y_unit, "V^2/Hz");
}

#[test]
fn test_map_pnoise_data_input_reference_falls_back_to_output_noise() {
    let data = crate::services::simulation_runner::PnoiseData {
        frequencies: vec![1e3, 1e4],
        output_noise: vec![7.0, 8.0],
        input_noise: None,
        total_output_noise: None,
        contributors: vec![],
        carrier_frequency: 1e6,
        sideband_factor: 1,
        reference: crate::services::simulation_runner::PnoiseReference::Input,
        warnings: vec![],
    };

    let mapped = RunExecutor::map_pnoise_data(data);
    assert_eq!(mapped.waveforms.len(), 1);
    assert_eq!(mapped.waveforms[0].y, vec![7.0, 8.0]);
}

#[test]
fn test_stb_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue =
        RunQueue::new().with_netlist("* stb\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n");
    queue.add_analysis(AnalysisSpec::Stb {
        probe_node: "1".to_string(),
        start_freq: 1.0,
        stop_freq: 1e6,
        points_per_decade: 8,
    });

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected STB run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped STB result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Stb);
    assert_eq!(mapped.waveforms.len(), 2);
    assert!(mapped
        .waveforms
        .iter()
        .any(|wf| wf.name == "Loop Gain (dB)"));
    assert!(mapped
        .waveforms
        .iter()
        .any(|wf| wf.name == "Loop Phase (deg)"));
    assert!(mapped.measurements.iter().any(|m| m.name == "phase_margin"));
}

#[test]
fn test_pstb_analysis_with_spec_is_executed() {
    let executor = RunExecutor::new();
    let mut queue = RunQueue::new()
        .with_netlist("* pstb\nV1 in 0 1\nR1 in mid 1k\nLPROBE mid out 1u\nC1 out 0 1n\n.end\n");
    queue.add_analysis(AnalysisSpec::Pstb);

    let result = executor.execute(&mut queue);
    assert_eq!(result.state.total_runs, 1);
    assert!(
        result.errors.is_empty(),
        "expected PSTB run to succeed, got errors: {:?}",
        result.errors
    );

    let mapped = result
        .results
        .values()
        .next()
        .expect("expected mapped PSTB result");
    assert_eq!(mapped.analysis_type, MappedAnalysisType::Pstb);
    assert_eq!(mapped.waveforms.len(), 4);
    assert!(mapped
        .waveforms
        .iter()
        .any(|wf| wf.name == "Floquet |lambda|"));
    assert!(mapped
        .waveforms
        .iter()
        .any(|wf| wf.name == "Stability Margin (dB)"));
    assert!(mapped
        .waveforms
        .iter()
        .any(|wf| wf.name == "Probe Mode Participation"));
    assert!(mapped
        .measurements
        .iter()
        .any(|m| m.name == "dominant_multiplier"));
    assert!(mapped
        .measurements
        .iter()
        .any(|m| m.name == "probe_state_index"));
    assert!(mapped
        .measurements
        .iter()
        .any(|m| m.name == "probe_state_persistence_db"));
    assert!(mapped
        .measurements
        .iter()
        .any(|m| m.name == "dominant_probe_mode_participation"));
}
