use super::*;

// =========================================================================
// AnalysisRunType Tests
// =========================================================================

#[test]
fn test_run_type_display() {
    assert_eq!(AnalysisRunType::DcOp.display_name(), "DC Operating Point");
    assert_eq!(AnalysisRunType::Transient.display_name(), "Transient");
    assert_eq!(AnalysisRunType::Disto.display_name(), "DISTO");
    assert_eq!(AnalysisRunType::Stb.display_name(), "STB");
    assert_eq!(AnalysisRunType::Pxf.display_name(), "PXF");
    assert_eq!(AnalysisRunType::Pstb.display_name(), "PSTB");
    assert_eq!(AnalysisRunType::Reliability.display_name(), "Reliability");
    assert_eq!(AnalysisRunType::Optimization.display_name(), "Optimization");
    assert_eq!(AnalysisRunType::Soa.display_name(), "Safety (SOA)");
}

#[test]
fn test_run_type_requires_dc_op() {
    assert!(AnalysisRunType::Ac.requires_dc_op());
    assert!(AnalysisRunType::Disto.requires_dc_op());
    assert!(AnalysisRunType::Noise.requires_dc_op());
    assert!(AnalysisRunType::Stb.requires_dc_op());
    assert!(AnalysisRunType::Reliability.requires_dc_op());
    assert!(!AnalysisRunType::Optimization.requires_dc_op());
    assert!(!AnalysisRunType::Soa.requires_dc_op());
    assert!(!AnalysisRunType::Transient.requires_dc_op());
    assert!(!AnalysisRunType::DcOp.requires_dc_op());
}

#[test]
fn test_analysis_spec_stb_validation() {
    let valid = AnalysisSpec::Stb {
        probe_node: "LSTB".to_string(),
        start_freq: 1.0,
        stop_freq: 1e9,
        points_per_decade: 10,
    };
    assert!(valid.validate().is_ok());

    let invalid = AnalysisSpec::Stb {
        probe_node: "".to_string(),
        start_freq: 0.0,
        stop_freq: 1.0,
        points_per_decade: 0,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_analysis_spec_disto_validation() {
    let valid = AnalysisSpec::Disto {
        start_freq: 1.0,
        stop_freq: 1e9,
        points_per_unit: 12,
        sweep: FrequencySweep::Decade,
        f2_over_f1: Some(1.5),
    };
    assert!(valid.validate().is_ok());
    assert_eq!(valid.run_type(), AnalysisRunType::Disto);

    let invalid = AnalysisSpec::Disto {
        start_freq: 0.0,
        stop_freq: 1e9,
        points_per_unit: 0,
        sweep: FrequencySweep::Decade,
        f2_over_f1: Some(1.0),
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_hb_tone_spec_builders() {
    let tone = HbToneSpec::new(2.4e9, 7).with_source("VLO").with_name("LO");
    assert!((tone.frequency - 2.4e9).abs() < 1e-6);
    assert_eq!(tone.harmonics, 7);
    assert_eq!(tone.source.as_deref(), Some("VLO"));
    assert_eq!(tone.name.as_deref(), Some("LO"));
}

#[test]
fn test_analysis_spec_hb_validation() {
    let valid = AnalysisSpec::HarmonicBalance {
        tones: vec![HbToneSpec::new(1e9, 9), HbToneSpec::new(900e6, 5)],
        reltol: 1e-6,
        abstol: 1e-12,
        max_iterations: 100,
        damping: 0.8,
        oversample: 2,
        max_mixing_order: 5,
        use_krylov: false,
        gmres_restart: 30,
        source_stepping: false,
        verbose: false,
    };
    assert!(valid.validate().is_ok());

    let invalid = AnalysisSpec::HarmonicBalance {
        tones: vec![HbToneSpec::new(1e9, 0)],
        reltol: 1e-6,
        abstol: 1e-12,
        max_iterations: 100,
        damping: 0.8,
        oversample: 2,
        max_mixing_order: 5,
        use_krylov: false,
        gmres_restart: 30,
        source_stepping: false,
        verbose: false,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_analysis_spec_pstb_validation() {
    let spec = AnalysisSpec::Pstb;
    assert!(spec.validate().is_ok());
    assert_eq!(spec.run_type(), AnalysisRunType::Pstb);
}

#[test]
fn test_analysis_spec_reliability_validation() {
    let valid = AnalysisSpec::Reliability {
        target_years: vec![1.0, 5.0, 10.0],
        enable_hci: true,
        enable_nbti: false,
        enable_em: true,
        min_stress_voltage: 0.05,
    };
    assert!(valid.validate().is_ok());
    assert_eq!(valid.run_type(), AnalysisRunType::Reliability);

    let invalid = AnalysisSpec::Reliability {
        target_years: vec![0.0],
        enable_hci: false,
        enable_nbti: false,
        enable_em: false,
        min_stress_voltage: -1.0,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_analysis_spec_optimization_validation() {
    let valid = AnalysisSpec::Optimization {
        variables: vec![OptimizationVariable {
            name: "RLOAD".to_string(),
            min: 1e3,
            max: 10e3,
            initial: 2e3,
        }],
        objective_node: "out".to_string(),
        objective_ref: "0".to_string(),
        goal: OptimizationGoal::Target,
        target: Some(1.2),
        algorithm: OptimizationAlgorithm::PatternSearch,
        max_iterations: 64,
        cost_tolerance: 1e-8,
        fd_step: 1e-4,
        initial_step: 0.1,
        min_step: 1e-8,
    };
    assert!(valid.validate().is_ok());
    assert_eq!(valid.run_type(), AnalysisRunType::Optimization);

    let invalid = AnalysisSpec::Optimization {
        variables: vec![OptimizationVariable {
            name: "RLOAD".to_string(),
            min: 10e3,
            max: 1e3,
            initial: 2e3,
        }],
        objective_node: "out".to_string(),
        objective_ref: "out".to_string(),
        goal: OptimizationGoal::Target,
        target: None,
        algorithm: OptimizationAlgorithm::PatternSearch,
        max_iterations: 0,
        cost_tolerance: -1.0,
        fd_step: 0.0,
        initial_step: 0.0,
        min_step: 1.0,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_analysis_spec_soa_validation() {
    let valid = AnalysisSpec::Soa {
        stop_time: 1e-6,
        step_time: 1e-9,
        check_vgs_max: true,
        max_vgs: 1.8,
        check_vds_max: true,
        max_vds: 3.3,
        check_vbe_max: false,
        max_vbe: 0.9,
        check_vce_max: false,
        max_vce: 5.0,
    };
    assert!(valid.validate().is_ok());
    assert_eq!(valid.run_type(), AnalysisRunType::Soa);

    let invalid = AnalysisSpec::Soa {
        stop_time: 1e-9,
        step_time: 1e-6,
        check_vgs_max: false,
        max_vgs: 0.0,
        check_vds_max: false,
        max_vds: 0.0,
        check_vbe_max: false,
        max_vbe: 0.0,
        check_vce_max: false,
        max_vce: 0.0,
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_analysis_spec_sparameter_validation_supports_multiport() {
    let valid = AnalysisSpec::SParameter {
        start_freq: 1e3,
        stop_freq: 1e9,
        points_per_unit: 10,
        sweep: FrequencySweep::Decade,
        z0: 50.0,
        ports: vec![
            SpPort {
                node_pos: "in".to_string(),
                node_neg: "0".to_string(),
                z0: None,
            },
            SpPort {
                node_pos: "out".to_string(),
                node_neg: "0".to_string(),
                z0: Some(60.0),
            },
            SpPort {
                node_pos: "aux".to_string(),
                node_neg: "0".to_string(),
                z0: None,
            },
        ],
    };
    assert!(valid.validate().is_ok());
    assert_eq!(valid.run_type(), AnalysisRunType::SParameter);
}

#[test]
fn test_analysis_spec_sparameter_validation_rejects_missing_ports() {
    let invalid = AnalysisSpec::SParameter {
        start_freq: 1e3,
        stop_freq: 1e9,
        points_per_unit: 10,
        sweep: FrequencySweep::Decade,
        z0: 50.0,
        ports: vec![SpPort {
            node_pos: "in".to_string(),
            node_neg: "0".to_string(),
            z0: None,
        }],
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_analysis_spec_sparameter_validation_rejects_invalid_port_z0() {
    let invalid = AnalysisSpec::SParameter {
        start_freq: 1e3,
        stop_freq: 1e9,
        points_per_unit: 10,
        sweep: FrequencySweep::Decade,
        z0: 50.0,
        ports: vec![
            SpPort {
                node_pos: "in".to_string(),
                node_neg: "0".to_string(),
                z0: Some(0.0),
            },
            SpPort {
                node_pos: "out".to_string(),
                node_neg: "0".to_string(),
                z0: None,
            },
        ],
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_run_type_requires_pss() {
    assert!(AnalysisRunType::Pac.requires_pss());
    assert!(AnalysisRunType::Pnoise.requires_pss());
    assert!(AnalysisRunType::Pxf.requires_pss());
    assert!(AnalysisRunType::Pstb.requires_pss());
    assert!(!AnalysisRunType::Ac.requires_pss());
}

#[test]
fn test_analysis_spec_sensitivity_validation() {
    let valid = AnalysisSpec::Sensitivity {
        output_var: "V(out)".to_string(),
        ac_mode: true,
        frequency: Some(1e6),
    };
    assert!(valid.validate().is_ok());

    let invalid = AnalysisSpec::Sensitivity {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: Some(1e6),
    };
    assert!(invalid.validate().is_err());
}

#[test]
fn test_analysis_spec_pole_zero_validation() {
    let valid = AnalysisSpec::PoleZero {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "VOL".to_string(),
        analysis_type: "PZ".to_string(),
    };
    assert!(valid.validate().is_ok());

    let invalid_transfer = AnalysisSpec::PoleZero {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "BAD".to_string(),
        analysis_type: "PZ".to_string(),
    };
    assert!(invalid_transfer.validate().is_err());

    let invalid_type = AnalysisSpec::PoleZero {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "VOL".to_string(),
        analysis_type: "UNKNOWN".to_string(),
    };
    assert!(invalid_type.validate().is_err());
}

#[test]
fn test_analysis_spec_dc_sweep_nested_validation() {
    let valid = AnalysisSpec::DcSweep {
        source_name: "V1".to_string(),
        start: 0.0,
        stop: 1.0,
        step: 0.1,
        source2: Some("V2".to_string()),
        start2: Some(0.0),
        stop2: Some(2.0),
        step2: Some(0.5),
    };
    assert!(valid.validate().is_ok());
}

#[test]
fn test_analysis_spec_dc_sweep_nested_requires_complete_secondary_fields() {
    let invalid = AnalysisSpec::DcSweep {
        source_name: "V1".to_string(),
        start: 0.0,
        stop: 1.0,
        step: 0.1,
        source2: Some("V2".to_string()),
        start2: Some(0.0),
        stop2: Some(2.0),
        step2: None,
    };
    assert!(invalid.validate().is_err());
}

// =========================================================================
// AnalysisRun Tests
// =========================================================================

#[test]
fn test_run_creation() {
    let run = AnalysisRun::new(1, AnalysisRunType::Ac);
    assert_eq!(run.id, 1);
    assert_eq!(run.run_type, AnalysisRunType::Ac);
    assert_eq!(run.status, RunStatus::Pending);
}

#[test]
fn test_run_with_spec_sets_type() {
    let run = AnalysisRun::new(1, AnalysisRunType::DcOp).with_spec(AnalysisSpec::Ac {
        start_freq: 1.0,
        stop_freq: 1e6,
        points_per_unit: 20,
        sweep: FrequencySweep::Decade,
    });
    assert_eq!(run.run_type, AnalysisRunType::Ac);
    assert!(run.spec.is_some());
}

#[test]
fn test_run_validate_requires_spec_for_parameterized_run() {
    let run = AnalysisRun::new(1, AnalysisRunType::Ac);
    assert!(run.validate().is_err());
}

#[test]
fn test_run_lifecycle() {
    let mut run = AnalysisRun::new(1, AnalysisRunType::DcOp);

    run.start(1000);
    assert_eq!(run.status, RunStatus::Running);
    assert_eq!(run.start_time, Some(1000));

    run.update_progress(50);
    assert_eq!(run.progress, 50);

    run.complete(1010);
    assert_eq!(run.status, RunStatus::Completed);
    assert_eq!(run.elapsed(), Some(10));
}

#[test]
fn test_run_failure() {
    let mut run = AnalysisRun::new(1, AnalysisRunType::DcOp);
    run.start(1000);
    run.fail("Convergence failure", 1005);

    assert_eq!(run.status, RunStatus::Failed);
    assert_eq!(run.error, Some("Convergence failure".to_string()));
}

#[test]
fn test_run_dependencies() {
    let run = AnalysisRun::new(3, AnalysisRunType::Ac)
        .with_dependency(1)
        .with_dependency(2);

    assert!(!run.dependencies_met(&[1]));
    assert!(run.dependencies_met(&[1, 2]));
    assert!(run.dependencies_met(&[1, 2, 3]));
}

// =========================================================================
// RunQueue Tests
// =========================================================================

#[test]
fn test_queue_creation() {
    let queue = RunQueue::new();
    assert!(queue.is_empty());
    assert!(queue.stop_on_error);
}

#[test]
fn test_queue_add() {
    let mut queue = RunQueue::new();
    let id = queue.add(AnalysisRunType::DcOp);

    assert_eq!(queue.len(), 1);
    assert!(queue.get(id).is_some());
}

#[test]
fn test_queue_add_analysis_spec() {
    let mut queue = RunQueue::new();
    let id = queue.add_analysis(AnalysisSpec::DcSweep {
        source_name: "V1".to_string(),
        start: 0.0,
        stop: 1.0,
        step: 0.1,
        source2: None,
        start2: None,
        stop2: None,
        step2: None,
    });

    let run = queue.get(id).expect("run should exist");
    assert_eq!(run.run_type, AnalysisRunType::DcSweep);
    assert!(run.validate().is_ok());
}

#[test]
fn test_analysis_plan_into_queue() {
    let plan = AnalysisPlan::new()
        .add(AnalysisSpec::DcOp)
        .add(AnalysisSpec::Ac {
            start_freq: 1.0,
            stop_freq: 1e6,
            points_per_unit: 10,
            sweep: FrequencySweep::Decade,
        });

    let queue = plan.into_queue().expect("valid plan should produce queue");
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.runs()[0].run_type, AnalysisRunType::DcOp);
    assert_eq!(queue.runs()[1].run_type, AnalysisRunType::Ac);
}

#[test]
fn test_queue_auto_deps() {
    let mut queue = RunQueue::new();
    let ac_id = queue.add_with_deps(AnalysisRunType::Ac);

    // Should auto-add DC OP
    assert_eq!(queue.len(), 2);

    let ac_run = queue.get(ac_id).unwrap();
    assert!(!ac_run.dependencies.is_empty());
}

#[test]
fn test_queue_execution() {
    let mut queue = RunQueue::new();
    queue.add(AnalysisRunType::DcOp);
    queue.add(AnalysisRunType::Transient);

    // Start first
    let id = queue.start_next(1000).unwrap();
    assert_eq!(queue.current_run, Some(id));

    // Complete it
    queue.complete_current(1010);
    assert!(queue.current_run.is_none());
    assert_eq!(queue.count_by_status(RunStatus::Completed), 1);

    // Start second
    let id2 = queue.start_next(1010).unwrap();
    assert_ne!(id, id2);
}

#[test]
fn test_queue_dependency_ordering() {
    let mut queue = RunQueue::new();
    let dc_id = queue.add(AnalysisRunType::DcOp);
    let ac_id = queue.add(AnalysisRunType::Ac);

    // Add dependency manually
    if let Some(ac) = queue.get_mut(ac_id) {
        ac.dependencies.push(dc_id);
    }

    // Only DC should be runnable
    assert_eq!(queue.next_runnable(), Some(dc_id));

    // Complete DC
    queue.start_next(1000);
    queue.complete_current(1010);

    // Now AC is runnable
    assert_eq!(queue.next_runnable(), Some(ac_id));
}

#[test]
fn test_queue_failure_skip() {
    let mut queue = RunQueue::new();
    let id1 = queue.add(AnalysisRunType::DcOp);
    let id2 = queue.add(AnalysisRunType::Ac);

    // Make AC depend on DC
    if let Some(ac) = queue.get_mut(id2) {
        ac.dependencies.push(id1);
    }

    // Start and fail DC
    queue.start_next(1000);
    queue.fail_current("Error", 1005);

    // AC should be skipped
    assert_eq!(queue.get(id2).unwrap().status, RunStatus::Skipped);
}

#[test]
fn test_queue_progress() {
    let mut queue = RunQueue::new();
    queue.add(AnalysisRunType::DcOp);
    queue.add(AnalysisRunType::DcOp);

    assert_eq!(queue.overall_progress(), 0);

    queue.start_next(1000);
    queue.complete_current(1010);

    assert_eq!(queue.overall_progress(), 50);

    queue.start_next(1010);
    queue.complete_current(1020);

    assert_eq!(queue.overall_progress(), 100);
}

#[test]
fn test_queue_pause_resume() {
    let mut queue = RunQueue::new();
    queue.add(AnalysisRunType::DcOp);

    queue.pause();
    assert!(queue.paused);
    assert!(queue.next_runnable().is_none());

    queue.resume();
    assert!(!queue.paused);
    assert!(queue.next_runnable().is_some());
}

#[test]
fn test_queue_cancel_all() {
    let mut queue = RunQueue::new();
    queue.add(AnalysisRunType::DcOp);
    queue.add(AnalysisRunType::Ac);
    queue.start_next(1000);

    queue.cancel_all(1005);

    assert_eq!(queue.count_by_status(RunStatus::Cancelled), 2);
}

#[test]
fn test_queue_is_done() {
    let mut queue = RunQueue::new();
    queue.add(AnalysisRunType::DcOp);

    assert!(!queue.is_done());

    queue.start_next(1000);
    queue.complete_current(1010);

    assert!(queue.is_done());
}
