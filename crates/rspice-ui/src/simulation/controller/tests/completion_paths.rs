//! Controller completion-path tests for exports, retained results, and terminal run state.

use super::*;

#[test]
fn touchstone_auto_export_uses_export_workflow_io() {
    let mut state = AppState::default();
    state.simulation.start_run();

    let mut controller = SimulationController::new();
    controller.touchstone_export_policy = TouchstoneExportPolicy::enabled(
        2,
        PathBuf::from("designs"),
        std::ffi::OsString::from("amp"),
    )
    .expect("valid prepared Touchstone policy");
    controller.current_spec = Some(AnalysisSpec::SParameter {
        start_freq: 1.0e6,
        stop_freq: 2.0e6,
        points_per_unit: 2,
        sweep: FrequencySweep::Linear,
        z0: 50.0,
        ports: vec![
            SpPort {
                node_pos: "IN".to_string(),
                node_neg: "0".to_string(),
                z0: None,
            },
            SpPort {
                node_pos: "OUT".to_string(),
                node_neg: "0".to_string(),
                z0: Some(75.0),
            },
        ],
    });
    controller.current_analysis_idx = 1;
    // Live editor mutations after dispatch must not redirect or reformat
    // the prepared automatic export.
    state.schematic.current_file = Some(PathBuf::from("changed").join("redirect.sch"));
    let mut changed = crate::simulation::dialog::SpConfig::default();
    changed.touchstone_export = false;
    changed.touchstone_version = 1;
    state.sim_setup.sp = crate::simulation::dialog::SpDialogState::from_config(&changed);

    let export_io = MockExportWorkflowIo::default();
    let prepared = controller
        .prepare_touchstone_export(&synthetic_sparameter_result(), 1)
        .expect("valid Touchstone export")
        .expect("enabled Touchstone export");
    SimulationController::commit_touchstone_export(&mut state, &export_io, prepared);

    assert!(export_io.writes.borrow().is_empty());
    let writes = export_io.create_only_writes.borrow();
    assert_eq!(writes.len(), 1);
    assert_eq!(
        writes[0].0,
        PathBuf::from("designs").join("amp_run0001_sp01.s2p")
    );
    assert!(writes[0].1.contains("[Version] 2.0"));
    assert!(writes[0].1.contains("[Reference] 5e1 7.5e1"));
}

#[test]
fn touchstone_native_completion_message_confirms_file_export() {
    let message = SimulationController::touchstone_export_completed_message(Path::new("amp.s2p"));
    assert!(message.contains("Exported Touchstone"));
    assert!(message.contains("amp.s2p"));
}

#[test]
fn ac_result_conversion_retains_complex_components_for_export() {
    let controller = SimulationController::new();
    let result = synthetic_sparameter_result();

    let analysis = controller.convert_to_analysis_result_with_metadata_owned(
        result,
        AnalysisType::SParameter,
        "SP",
    );
    let magnitude = analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "|S11|")
        .expect("magnitude trace exists");
    let complex = magnitude
        .complex
        .as_ref()
        .expect("magnitude trace retains complex source data");

    assert_eq!(complex.source_name, "S11");
    assert_eq!(&*complex.real, &[0.1, 0.2]);
    assert_eq!(&*complex.imag, &[0.0, 0.0]);
}

#[test]
fn ac_result_conversion_drops_traces_with_mismatched_frequency_shapes() {
    let controller = SimulationController::new();
    let frequencies = vec![1.0, 10.0, 100.0];
    let mut waveforms = std::collections::HashMap::new();
    waveforms.insert(
        "V(bad_real)".to_string(),
        crate::simulation::results::WaveformData::new_complex(
            "V(bad_real)",
            frequencies.clone(),
            vec![1.0, 2.0],
            vec![0.0, 0.0, 0.0],
        ),
    );
    waveforms.insert(
        "V(bad_imag)".to_string(),
        crate::simulation::results::WaveformData::new_complex(
            "V(bad_imag)",
            frequencies.clone(),
            vec![1.0, 2.0, 3.0],
            vec![0.0, 0.0],
        ),
    );
    waveforms.insert(
        "V(good)".to_string(),
        crate::simulation::results::WaveformData::new_complex(
            "V(good)",
            frequencies.clone(),
            vec![3.0, 4.0, 5.0],
            vec![4.0, 3.0, 0.0],
        ),
    );

    let analysis = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Ac {
            frequencies,
            waveforms,
            measurements: Vec::new(),
        },
        AnalysisType::Ac,
        "AC",
    );

    let names: Vec<_> = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.name.as_str())
        .collect();
    assert_eq!(names, vec!["|V(good)|", "phase(V(good))"]);
    assert!(
        analysis
            .waveforms
            .iter()
            .all(|waveform| waveform.x.len() == waveform.y.len()),
        "converted AC traces must never pair mismatched x/y arrays"
    );
}

#[test]
fn noise_result_conversion_drops_traces_with_mismatched_frequency_shapes() {
    let controller = SimulationController::new();
    let mut contributors = std::collections::HashMap::new();
    contributors.insert("good".to_string(), vec![1.0e-18, 2.0e-18, 3.0e-18]);
    contributors.insert("bad".to_string(), vec![1.0e-18, 2.0e-18]);

    let analysis = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Noise {
            frequencies: vec![1.0, 10.0, 100.0],
            output_noise: vec![2.0e-18, 3.0e-18],
            input_noise: Some(vec![1.0e-18, 1.5e-18, 2.0e-18]),
            contributors,
            summary: None,
            measurements: Vec::new(),
        },
        AnalysisType::Noise,
        "Noise",
    );

    let names: Vec<_> = analysis
        .waveforms
        .iter()
        .map(|waveform| waveform.name.as_str())
        .collect();
    assert_eq!(names, vec!["inoise", "noise(good)"]);
    assert!(
        analysis
            .waveforms
            .iter()
            .all(|waveform| waveform.x.len() == waveform.y.len()),
        "converted noise traces must never pair mismatched x/y arrays"
    );
}

#[test]
fn advanced_result_conversion_retains_exact_family_metadata() {
    use crate::state::{AnalysisResultFamilyMetadata, MonteCarloVariableMetadata};

    let controller = SimulationController::new();
    let empty_waveforms = || std::collections::HashMap::new();

    let monte_carlo = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 0x5eed,
            runs_requested: 4,
            runs_completed: 3,
            num_failures: 1,
            all_converged: false,
            variables: vec![crate::simulation::results::MonteCarloVariableResult {
                name: "V(out)".to_owned(),
                samples: vec![0.9, 1.0, 1.1],
                mean: 1.0,
                std_dev: 0.1,
                min: 0.9,
                max: 1.1,
                histogram: vec![1, 2],
                bin_edges: vec![0.85, 1.0, 1.15],
            }],
        },
        AnalysisType::MonteCarlo,
        "MC",
    );
    assert_eq!(
        monte_carlo.family_metadata,
        Some(AnalysisResultFamilyMetadata::MonteCarlo {
            member_measurements: Vec::new(),
            seed: 0x5eed,
            runs_requested: 4,
            runs_completed: 3,
            failures: 1,
            all_converged: false,
            variables: vec![MonteCarloVariableMetadata {
                name: "V(out)".to_owned(),
                samples: vec![0.9, 1.0, 1.1],
                mean: 1.0,
                std_dev: 0.1,
                min: 0.9,
                max: 1.1,
            }],
        })
    );
    assert_eq!(monte_carlo.waveforms[0].name, "hist(V(out))");

    let parametric = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Parametric {
            member_measurements: Vec::new(),
            target: "PARAM rload".to_owned(),
            sweep_values: vec![1_000.0, 2_000.0],
            waveforms: empty_waveforms(),
            num_failures: 1,
        },
        AnalysisType::Parametric,
        "STEP",
    );
    assert_eq!(
        parametric.family_metadata,
        Some(AnalysisResultFamilyMetadata::Parametric {
            member_measurements: Vec::new(),
            target: "PARAM rload".to_owned(),
            sweep_values: vec![1_000.0, 2_000.0],
            failed_points: 1,
        })
    );

    let corner = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Corner {
            member_measurements: Vec::new(),
            x_values: vec![0.0, 1.0],
            x_label: "Corner Index".to_owned(),
            x_unit: String::new(),
            temperatures_c: vec![-40.0, 125.0],
            corner_labels: vec!["SS_0.9V_-40C".to_owned(), "FF_1.1V_125C".to_owned()],
            waveforms: empty_waveforms(),
            num_failures: 0,
        },
        AnalysisType::Corner,
        "Corner",
    );
    assert_eq!(
        corner.family_metadata,
        Some(AnalysisResultFamilyMetadata::Corner {
            member_measurements: Vec::new(),
            x_values: vec![0.0, 1.0],
            x_label: "Corner Index".to_owned(),
            x_unit: String::new(),
            temperatures_c: vec![-40.0, 125.0],
            corner_labels: vec!["SS_0.9V_-40C".to_owned(), "FF_1.1V_125C".to_owned()],
            failed_corners: 0,
        })
    );

    let reliability = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Reliability {
            years: vec![1.0, 5.0, 10.0],
            waveforms: empty_waveforms(),
            device_results: vec![crate::simulation::ReliabilityResult {
                device_id: "M1".to_owned(),
                stress: crate::simulation::StressMetrics {
                    avg_vgs_stress: 1.2,
                    avg_vds_stress: 1.8,
                    avg_temp: 358.15,
                    duration: 3_600.0,
                },
                shifts: std::collections::HashMap::from([
                    ("1y".to_owned(), crate::simulation::ParamShift::default()),
                    ("5y".to_owned(), crate::simulation::ParamShift::default()),
                    ("10y".to_owned(), crate::simulation::ParamShift::default()),
                ]),
            }],
        },
        AnalysisType::Reliability,
        "Reliability",
    );
    assert_eq!(
        reliability.family_metadata,
        Some(AnalysisResultFamilyMetadata::Reliability {
            years: vec![1.0, 5.0, 10.0],
        })
    );
    assert!(matches!(
        reliability.result_payload,
        Some(AnalysisResultPayload::Reliability { ref devices }) if devices.len() == 1
    ));

    let optimization = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Optimization {
            iterations: vec![0.0, 1.0, 2.0],
            waveforms: empty_waveforms(),
            best_cost: 0.125,
            best_variables: std::collections::HashMap::from([
                ("w".to_owned(), 2.0e-6),
                ("l".to_owned(), 180.0e-9),
            ]),
            converged: true,
        },
        AnalysisType::Optimization,
        "Optimization",
    );
    assert_eq!(
        optimization.family_metadata,
        Some(AnalysisResultFamilyMetadata::Optimization {
            iterations: vec![0.0, 1.0, 2.0],
            best_cost: 0.125,
            best_variables: std::collections::BTreeMap::from([
                ("l".to_owned(), 180.0e-9),
                ("w".to_owned(), 2.0e-6),
            ]),
            converged: true,
        })
    );

    let soa = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Soa {
            time: vec![0.0, 1.0e-9],
            waveforms: empty_waveforms(),
            violations: Vec::new(),
            evaluations: vec![crate::services::safety::SoAEvaluation {
                device_id: "M1".to_owned(),
                parameter: crate::services::safety::SoAParameter::Vgs,
                limit_value: 1.8,
                worst_actual_value: 1.0,
                worst_time: 1.0e-9,
                sample_count: 2,
                unit: "V".to_owned(),
                description: "Maximum gate-source voltage".to_owned(),
                verdict: crate::services::safety::SoARuleVerdict::Pass,
            }],
        },
        AnalysisType::Soa,
        "SOA",
    );
    assert_eq!(
        soa.family_metadata,
        Some(AnalysisResultFamilyMetadata::Soa {
            time: vec![0.0, 1.0e-9],
        })
    );
    assert!(matches!(
        soa.result_payload,
        Some(AnalysisResultPayload::Soa {
            ref evaluations,
            ref violations,
        }) if evaluations.len() == 1 && violations.is_empty()
    ));
}

#[test]
fn scalar_and_complex_analysis_conversion_retains_exact_typed_payloads() {
    use crate::state::{
        AnalysisResultPayload, ComplexResultValue, PoleZeroRootSetEvidence,
        PoleZeroSpectrumCertificate, SensitivityResultMode, SensitivityResultRow,
    };

    let controller = SimulationController::new();
    let pole_evidence = PoleZeroRootSetEvidence::Qualified {
        certificate: PoleZeroSpectrumCertificate {
            problem_order: 2,
            infinite_count: 0,
            max_backward_error: 1.0e-14,
            qualification_tolerance:
                PoleZeroSpectrumCertificate::canonical_qualification_tolerance(2).unwrap(),
        },
    };
    let zero_evidence = PoleZeroRootSetEvidence::Qualified {
        certificate: PoleZeroSpectrumCertificate {
            problem_order: 1,
            infinite_count: 0,
            max_backward_error: 2.0e-14,
            qualification_tolerance:
                PoleZeroSpectrumCertificate::canonical_qualification_tolerance(1).unwrap(),
        },
    };
    let pole_zero = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::PoleZero {
            poles: vec![(-1.0, 2.0), (-1.0, -2.0)],
            zeros: vec![(-3.0, 0.0)],
            pole_evidence: pole_evidence.clone(),
            zero_evidence: zero_evidence.clone(),
            gain: Some(4.0),
        },
        AnalysisType::PoleZero,
        "PZ",
    );
    assert_eq!(
        pole_zero.result_payload,
        Some(AnalysisResultPayload::PoleZero {
            poles: vec![
                ComplexResultValue {
                    real: -1.0,
                    imaginary: 2.0,
                },
                ComplexResultValue {
                    real: -1.0,
                    imaginary: -2.0,
                },
            ],
            zeros: vec![ComplexResultValue {
                real: -3.0,
                imaginary: 0.0,
            }],
            pole_evidence,
            zero_evidence,
            gain: Some(4.0),
        })
    );
    assert!(pole_zero.has_data());

    let sensitivity = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Sensitivity {
            output: "V(out)".to_owned(),
            ac_mode: true,
            frequency_hz: Some(10_000.0),
            sensitivities: std::collections::HashMap::from([
                ("width".to_owned(), 2.0),
                ("length".to_owned(), -1.0),
            ]),
            normalized: std::collections::HashMap::from([
                ("width".to_owned(), 0.5),
                ("length".to_owned(), -0.25),
            ]),
        },
        AnalysisType::Sensitivity,
        "SENS",
    );
    assert_eq!(
        sensitivity.result_payload,
        Some(AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Ac {
                frequency_hz: 10_000.0,
            },
            rows: vec![
                SensitivityResultRow {
                    parameter: "length".to_owned(),
                    raw: -1.0,
                    normalized: -0.25,
                },
                SensitivityResultRow {
                    parameter: "width".to_owned(),
                    raw: 2.0,
                    normalized: 0.5,
                },
            ],
        })
    );

    let scalar = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::MeasurementsOnly {
            measurements: std::collections::HashMap::from([
                ("zeta".to_owned(), 0.7),
                ("gain".to_owned(), 10.0),
            ]),
        },
        AnalysisType::DcMismatch,
        "DC mismatch",
    );
    assert_eq!(
        scalar.result_payload,
        Some(AnalysisResultPayload::ScalarMeasurements {
            values: std::collections::BTreeMap::from([
                ("gain".to_owned(), 10.0),
                ("zeta".to_owned(), 0.7),
            ]),
        })
    );
}

#[test]
fn incomplete_reliability_and_soa_results_fail_closed_without_retained_payloads() {
    let controller = SimulationController::new();
    let reliability = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Reliability {
            years: vec![1.0, 10.0],
            waveforms: std::collections::HashMap::new(),
            device_results: Vec::new(),
        },
        AnalysisType::Reliability,
        "Reliability",
    );
    assert!(!reliability.success);
    assert!(reliability.result_payload.is_none());
    assert!(
        reliability
            .error_message
            .as_deref()
            .is_some_and(|message| message.contains("no device evidence"))
    );

    let soa = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Soa {
            time: vec![0.0, 1.0],
            waveforms: std::collections::HashMap::new(),
            violations: Vec::new(),
            evaluations: Vec::new(),
        },
        AnalysisType::Soa,
        "SOA",
    );
    assert!(!soa.success);
    assert!(soa.result_payload.is_none());
    assert!(
        soa.error_message
            .as_deref()
            .is_some_and(|message| message.contains("no evaluated-rule evidence"))
    );
}

#[test]
fn invalid_sensitivity_result_contract_fails_closed() {
    let controller = SimulationController::new();
    let analysis = controller.convert_to_analysis_result_with_metadata_owned(
        crate::simulation::SimulationResult::Sensitivity {
            output: "V(out)".to_owned(),
            ac_mode: false,
            frequency_hz: None,
            sensitivities: std::collections::HashMap::from([("width".to_owned(), 2.0)]),
            normalized: std::collections::HashMap::new(),
        },
        AnalysisType::Sensitivity,
        "SENS",
    );

    assert!(!analysis.success);
    assert!(analysis.result_payload.is_none());
    assert!(
        analysis
            .error_message
            .as_deref()
            .is_some_and(|message| message.contains("misaligned"))
    );
}

#[test]
fn manual_deck_trigger_runs_deck_analysis_without_enabled_run_set() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("the sole run-set analysis disables");
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
    state.simulation.request_manual_deck_run();
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");

    controller.start_simulation(&mut state);
    let total_analyses = controller.total_analyses;
    let current_spec = controller.current_spec.clone();
    let source_domain = controller
        .current_provenance
        .as_ref()
        .map(AnalysisResultProvenance::source_domain);
    let cached_netlist = controller.cached_netlist.clone().unwrap_or_default();
    let run_count = state.simulation.runs.len();
    let status = state.simulation.status.clone();
    controller.abort();

    assert_eq!(total_analyses, 1);
    assert!(matches!(current_spec, Some(AnalysisSpec::DcOp { .. })));
    assert_eq!(source_domain, Some(AnalysisResultSourceDomain::ManualDeck));
    assert!(cached_netlist.contains(".op\n.end"));
    assert_eq!(run_count, 1);
    assert_eq!(status, "DC Operating Point");
}

#[test]
fn controller_manual_run_receipt_remains_authoritative_if_result_provenance_is_stripped() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("run-set analysis disables");
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    let project_revision = state.workspace.project.revision();
    state.simulation.request_manual_deck_run();
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");

    controller.start_simulation(&mut state);

    let task_provenance = controller
        .current_provenance
        .clone()
        .expect("manual task owns exact provenance");
    let expanded = controller
        .cached_netlist
        .as_deref()
        .expect("sealed manual deck");
    let expected_source_digest = crate::simulation::execution::content_digest(
        "rspice.manual-executable-source/v1",
        expanded.as_bytes(),
    );
    {
        let run = state.simulation.active_run().expect("manual run starts");
        let receipt = run.prepared_receipt().expect("manual run is sealed");
        assert_eq!(
            receipt.source_domain(),
            AnalysisResultSourceDomain::ManualDeck
        );
        assert_eq!(receipt.simulation_plan_id(), None);
        assert_eq!(receipt.project_revision(), project_revision);
        assert_eq!(
            receipt.prepared_snapshot_digest(),
            task_provenance.prepared_snapshot_digest()
        );
        assert_eq!(receipt.source_content_digest(), expected_source_digest);
        assert!(receipt.source_check_receipt().is_manual_source_check());
        assert_eq!(receipt.tasks().len(), 1);
        let task = &receipt.tasks()[0];
        assert_eq!(task.instance_id(), task_provenance.source_instance_id());
        assert_eq!(task.source_revision(), project_revision);
        assert_eq!(task.analysis_kind_tag(), 0);
        assert!(task.dependencies().is_empty());
    }

    let run = state
        .simulation
        .active_run_mut()
        .expect("manual run remains active");
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_provenance(task_provenance),
    );
    assert!(run.validate_provenance().is_ok());
    run.analyses[0].provenance = None;
    assert!(matches!(
        run.provenance(),
        Some(SimulationRunProvenance::Prepared(_))
    ));
    assert!(run.validate_provenance().is_err());

    controller.abort();
}

#[test]
fn controller_manual_run_receipt_survives_production_project_round_trip() {
    let mut state = AppState::default();
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned());
    state.simulation.request_manual_deck_run();
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");
    controller.start_simulation(&mut state);
    let task_provenance = controller
        .current_provenance
        .clone()
        .expect("controller owns the prepared manual task");
    let expected_source_id = task_provenance.source_instance_id();
    state
        .simulation
        .active_run_mut()
        .expect("prepared manual run")
        .add_analysis(
            AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_provenance(task_provenance),
        );
    controller.abort();

    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state)
        .expect("production snapshot accepts controller manual run");
    let json = crate::io::project_io::serialize_project_file(&project)
        .expect("controller manual run serializes");
    let loaded = crate::io::project_io::load_project_text(&json, None)
        .expect("controller manual run reloads");
    assert!(
        loaded.execution_context.is_some(),
        "manual receipt must remain independent of the unrelated retained plan"
    );
    let restored = loaded
        .simulation_results
        .into_simulation_state()
        .expect("controller manual history restores");
    let run = &restored.runs[0];
    let receipt = run.prepared_receipt().expect("manual receipt retained");
    let result_provenance = run.analyses[0]
        .provenance
        .as_ref()
        .expect("manual result provenance retained");

    assert_eq!(
        receipt.source_domain(),
        AnalysisResultSourceDomain::ManualDeck
    );
    assert_eq!(receipt.simulation_plan_id(), None);
    assert!(receipt.source_check_receipt().is_manual_source_check());
    assert_eq!(receipt.tasks()[0].instance_id(), expected_source_id);
    assert_eq!(result_provenance.source_instance_id(), expected_source_id);
    assert_eq!(
        result_provenance.prepared_snapshot_digest(),
        receipt.prepared_snapshot_digest()
    );
}

#[test]
fn manual_deck_run_preserves_editor_source_without_ui_option_injection() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("the sole run-set analysis disables");
    state.sim_setup.options.reltol = 1.0e-4;
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
    state.simulation.request_manual_deck_run();
    let mut controller = SimulationController::new();
    controller
        .validate_manual_deck_document(&state)
        .expect("explicit validation authorizes the exact manual deck");

    controller.start_simulation(&mut state);
    let cached_netlist = controller.cached_netlist.clone().unwrap_or_default();
    controller.abort();

    assert_eq!(cached_netlist, "deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n");
    assert!(!cached_netlist.contains(".OPTIONS"));
    assert!(!cached_netlist.contains("RELTOL"));
}

#[test]
fn manual_deck_runs_use_imported_netlist_origin_for_relative_includes() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.schematic.current_file = Some(PathBuf::from("schematics").join("amp.rsch"));
    state.workspace.netlist_source =
        Some("deck\n.include models.lib\nV1 out 0 1\n.op\n.end\n".to_string());
    state.workspace.netlist_source_path = Some(PathBuf::from("decks").join("bias.cir"));

    assert_eq!(
        SimulationController::analysis_source_path(&state).as_deref(),
        Some(std::path::Path::new("decks").join("bias.cir").as_path())
    );
}

#[test]
fn manual_deck_runs_do_not_fall_back_to_schematic_path() {
    let mut state = AppState::default();
    state.simulation.run_intent = SimulationRunIntent::ManualDeck;
    state.schematic.current_file = Some(PathBuf::from("schematics").join("amp.rsch"));
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\n.op\n.end\n".to_string());

    assert!(
        SimulationController::analysis_source_path(&state).is_none(),
        "manual netlist text without an origin must not resolve includes from the schematic file"
    );
}

#[test]
fn simulate_run_set_does_not_run_manual_deck_source() {
    let mut state = AppState::default();
    let plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = plan.instances()[0].id();
    plan.set_enabled(transient_id, false)
        .expect("default transient disables");
    let (op_id, _) = plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts as the sole enabled analysis");
    assert_eq!(plan.instances()[0].id(), op_id);
    state.workspace.netlist_source = Some("deck\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_string());
    state.simulation.request_simulate_run_set();
    let mut controller = SimulationController::new();

    controller.start_simulation(&mut state);
    let status = state.simulation.status.clone();
    let run_count = state.simulation.runs.len();
    let total_analyses = controller.total_analyses;
    controller.abort();

    assert_eq!(status, "Run blocked");
    assert_eq!(run_count, 0);
    assert_eq!(total_analyses, 0);
}

#[test]
fn successful_manual_deck_run_promotes_pending_baseline() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    state.simulation.netlist_content =
        "deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n".to_string();
    let run_id = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_id);
    state.ui.netlist.pending_manual_run_id = Some(run_id);
    state.ui.netlist.pending_run_buffer =
        Some("deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n".to_string());
    state.ui.netlist.edited_lines.insert(0);

    controller.finish_simulation_batch(&mut state);

    assert_eq!(
        state.ui.netlist.last_run_buffer.as_deref(),
        Some("deck\n.param r=1k cl = 2p expr={x}\n.op\n.end\n")
    );
    assert!((state.ui.netlist.last_run_params["r"] - 1e3).abs() < 1e-9);
    assert!((state.ui.netlist.last_run_params["cl"] - 2e-12).abs() < 1e-21);
    assert!(!state.ui.netlist.last_run_params.contains_key("expr"));
    assert!(state.ui.netlist.pending_manual_run_id.is_none());
    assert!(state.ui.netlist.pending_run_buffer.is_none());
    assert!(state.ui.netlist.edited_lines.is_empty());
}

#[test]
fn failed_manual_deck_run_keeps_previous_baseline() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    state.ui.netlist.last_run_buffer = Some("old\n.op\n.end\n".to_string());
    let run = state.simulation.start_run();
    let run_id = run.id;
    run.success = false;
    bind_test_run_running(&mut state, &mut controller, run_id);
    state.ui.netlist.pending_manual_run_id = Some(run_id);
    state.ui.netlist.pending_run_buffer = Some("new\n.op\n.end\n".to_string());

    controller.finish_simulation_batch(&mut state);

    assert_eq!(
        state.ui.netlist.last_run_buffer.as_deref(),
        Some("old\n.op\n.end\n")
    );
    assert!(state.ui.netlist.pending_manual_run_id.is_none());
    assert!(state.ui.netlist.pending_run_buffer.is_none());
}

#[test]
fn batch_without_exact_run_ownership_does_not_modify_selected_history() {
    let mut state = AppState::default();
    let historical_id = state.simulation.start_run().id;
    let historical_lifecycle = state
        .simulation
        .run_by_sequence(historical_id)
        .expect("historical run")
        .lifecycle;
    let mut controller = SimulationController::new();
    controller.total_analyses = 1;

    controller.finish_simulation_batch(&mut state);

    let historical = state
        .simulation
        .run_by_sequence(historical_id)
        .expect("historical run remains");
    assert!(historical.success);
    assert_eq!(historical.lifecycle, historical_lifecycle);
    assert_eq!(state.simulation.status, "Completed with errors");
}

#[test]
fn disappeared_batch_target_does_not_reselect_the_active_historical_analysis() {
    let mut state = AppState::default();
    let historical_id = state.simulation.start_run().id;
    let historical = state
        .simulation
        .run_by_sequence_mut(historical_id)
        .expect("historical run");
    historical.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "first"));
    historical.add_analysis(AnalysisResult::new(2, AnalysisType::Ac, "second"));
    assert!(state.simulation.select_latest_analysis());
    assert_eq!(
        state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.label.as_str()),
        Some("second")
    );

    let mut controller = SimulationController::new();
    controller.current_run_id = Some(historical_id + 10_000);
    controller.total_analyses = 1;

    controller.finish_simulation_batch(&mut state);

    assert_eq!(
        state
            .simulation
            .active_analysis()
            .map(|analysis| analysis.label.as_str()),
        Some("second"),
        "a missing completion target must not call complete_run on the selected history row"
    );
}

#[test]
fn live_transient_accumulator_rejects_partial_or_schema_changing_points() {
    let sample = |time, waveforms: &[(&str, f64)]| TransientSampleDelta {
        time,
        waveforms: waveforms
            .iter()
            .map(
                |(name, value)| crate::simulation::runner::TransientWaveformSample {
                    name: (*name).to_owned(),
                    value: *value,
                    y_unit: "V".to_owned(),
                },
            )
            .collect(),
        events: Vec::new(),
        real_events: Vec::new(),
    };
    let mut accumulator = LiveTransientAccumulator::default();

    accumulator.ingest(vec![
        sample(0.0, &[("out", 0.0), ("ref", 1.0)]),
        sample(1.0, &[("out", 1.0)]),
        sample(2.0, &[("out", 2.0), ("other", 2.0)]),
        sample(3.0, &[("out", 3.0), ("ref", 4.0)]),
    ]);

    assert_eq!(accumulator.waveforms.len(), 2);
    for waveform in &accumulator.waveforms {
        assert_eq!(waveform.x, vec![0.0, 3.0]);
        assert_eq!(waveform.x.len(), waveform.y.len());
    }
}

#[test]
fn live_transient_accumulator_keeps_a_change_compressed_event_history() {
    use crate::simulation::runner::{TransientDigitalEventSample, TransientRealEventSample};

    let delta =
        |time: f64, events: &[(&str, u8)], real_events: &[(&str, f64)]| TransientSampleDelta {
            time,
            waveforms: vec![crate::simulation::runner::TransientWaveformSample {
                name: "out".to_owned(),
                value: time,
                y_unit: "V".to_owned(),
            }],
            events: events
                .iter()
                .map(|(name, value_code)| TransientDigitalEventSample {
                    name: (*name).to_owned(),
                    value_code: *value_code,
                })
                .collect(),
            real_events: real_events
                .iter()
                .map(|(name, value)| TransientRealEventSample {
                    name: (*name).to_owned(),
                    value: *value,
                })
                .collect(),
        };

    let mut accumulator = LiveTransientAccumulator::default();
    accumulator.ingest(vec![
        delta(0.0, &[("clk", 0), ("d", 12)], &[("vsense", 1.5)]),
        // A repeated value, a time that does not advance, a code outside the
        // 12-state encoding, and a non-finite real are each dropped for their
        // own node without disturbing the nodes beside them.
        delta(1.0e-9, &[("clk", 0), ("bad", 13)], &[("vsense", f64::NAN)]),
        delta(0.0, &[("d", 0)], &[]),
        delta(2.0e-9, &[("clk", 1)], &[("vsense", 2.5)]),
    ]);

    let payload = accumulator
        .event_payload(AnalysisType::Transient)
        .expect("a live event history is retained");
    let AnalysisResultPayload::TransientEvents {
        digital_traces,
        real_traces,
    } = payload
    else {
        panic!("live events are retained as a transient event payload");
    };

    assert_eq!(
        digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .collect::<Vec<_>>(),
        vec!["clk", "d"],
        "an unnamed code outside the encoding must not open a trace"
    );
    assert_eq!(
        digital_traces[0]
            .points
            .iter()
            .map(|point| (point.time_s, point.value_code))
            .collect::<Vec<_>>(),
        vec![(0.0, 0), (2.0e-9, 1)]
    );
    assert_eq!(
        digital_traces[1]
            .points
            .iter()
            .map(|point| (point.time_s, point.value_code))
            .collect::<Vec<_>>(),
        vec![(0.0, 12)],
        "a stale message replaying a time this node already has is rejected"
    );
    assert_eq!(
        real_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .collect::<Vec<_>>(),
        vec!["vsense"]
    );
    assert_eq!(
        real_traces[0]
            .points
            .iter()
            .map(|point| (point.time_s, point.value))
            .collect::<Vec<_>>(),
        vec![(0.0, 1.5), (2.0e-9, 2.5)]
    );

    accumulator.clear();
    assert!(accumulator.is_empty());
    assert!(accumulator.event_payload(AnalysisType::Transient).is_none());
}

#[test]
fn live_transient_accumulator_bounds_the_provisional_event_history() {
    use crate::simulation::runner::TransientDigitalEventSample;

    let deltas = (0..LiveTransientAccumulator::MAX_LIVE_EVENT_POINTS + 64)
        .map(|index| TransientSampleDelta {
            time: index as f64,
            waveforms: Vec::new(),
            events: vec![TransientDigitalEventSample {
                name: "clk".to_owned(),
                value_code: (index % 2) as u8,
            }],
            real_events: Vec::new(),
        })
        .collect();

    let mut accumulator = LiveTransientAccumulator::default();
    accumulator.ingest(deltas);

    let AnalysisResultPayload::TransientEvents { digital_traces, .. } = accumulator
        .event_payload(AnalysisType::Transient)
        .expect("a live event history is retained")
    else {
        panic!("live events are retained as a transient event payload");
    };
    assert_eq!(
        digital_traces[0].points.len(),
        LiveTransientAccumulator::MAX_LIVE_EVENT_POINTS
    );
    assert_eq!(
        digital_traces[0]
            .points
            .last()
            .expect("the ceiling retains points")
            .time_s,
        (LiveTransientAccumulator::MAX_LIVE_EVENT_POINTS - 1) as f64,
        "the provisional history stops at the ceiling rather than dropping its start"
    );
}

#[test]
fn live_transient_accumulator_compacts_aligned_source_traces() {
    let mut accumulator = LiveTransientAccumulator::default();
    let deltas = (0..LiveTransientAccumulator::MAX_SOURCE_SAMPLES + 1)
        .map(|index| TransientSampleDelta {
            time: index as f64,
            waveforms: vec![
                crate::simulation::runner::TransientWaveformSample {
                    name: "out".to_owned(),
                    value: (index as f64 / 10.0).sin(),
                    y_unit: "V".to_owned(),
                },
                crate::simulation::runner::TransientWaveformSample {
                    name: "ref".to_owned(),
                    value: (index as f64 / 17.0).cos(),
                    y_unit: "V".to_owned(),
                },
            ],
            events: Vec::new(),
            real_events: Vec::new(),
        })
        .collect();

    accumulator.ingest(deltas);

    assert_eq!(accumulator.waveforms.len(), 2);
    assert!(accumulator.waveforms[0].x.len() <= LiveTransientAccumulator::COMPACTED_SOURCE_SAMPLES);
    assert_eq!(
        accumulator.waveforms[0].x, accumulator.waveforms[1].x,
        "derived expressions require one aligned provisional axis"
    );
    assert_eq!(accumulator.waveforms[0].x.first(), Some(&0.0));
    assert_eq!(
        accumulator.waveforms[0].x.last(),
        Some(&(LiveTransientAccumulator::MAX_SOURCE_SAMPLES as f64))
    );
}

#[test]
fn successful_manual_deck_run_preserves_post_launch_diff_pips() {
    let mut state = AppState::default();
    let mut controller = SimulationController::new();
    state.simulation.netlist_content = "deck\n.op\nR1 out 0 2k\n.end\n".to_string();
    let run_id = state.simulation.start_run().id;
    bind_test_run_running(&mut state, &mut controller, run_id);
    state.ui.netlist.pending_manual_run_id = Some(run_id);
    state.ui.netlist.pending_run_buffer = Some("deck\n.op\nR1 out 0 1k\n.end\n".to_string());

    controller.finish_simulation_batch(&mut state);

    assert_eq!(
        state.ui.netlist.last_run_buffer.as_deref(),
        Some("deck\n.op\nR1 out 0 1k\n.end\n")
    );
    assert!(state.ui.netlist.edited_lines.contains(&2));
    assert_eq!(state.ui.netlist.edited_lines.len(), 1);
}

#[test]
fn ui_progress_fraction_uses_runner_fraction_or_running_floor() {
    assert!((SimulationController::ui_progress_fraction(Some(0.42), true) - 0.42).abs() < 1e-6);
    assert_eq!(SimulationController::ui_progress_fraction(None, true), 0.08);
    assert_eq!(SimulationController::ui_progress_fraction(None, false), 0.0);
    assert_eq!(
        SimulationController::ui_progress_fraction(Some(1.2), true),
        1.0
    );
}

/// A corner declaration keeps a task so the run's receipt has an entry for it,
/// but its turn must not reach the runner: the points are the solve. If it were
/// dispatched, the declared space would be solved a second time.
#[test]
fn a_corner_declarations_turn_assembles_its_family_without_reaching_the_runner() {
    use crate::product::ProcessCorner;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::services::simulation_runner::{CornerBaseMode, CornerProcess, CornerRunConfig};
    use crate::simulation::execution::{
        ExecutionPermitIssuer, ExecutionTargetCapabilities, PreparedRunSnapshot, PreparedTask,
        RunSourceReceipt, SavePolicy, SnapshotParts,
    };

    let corner = QueuedAnalysis {
        numeric_override: None,
        spec: AnalysisSpec::Corner,
        config: None,
        spec_options: SpecExecutionOptions {
            corner: Some(CornerRunConfig {
                process_corners: vec![CornerProcess::TT],
                voltages: vec![1.8, 1.62],
                supply_source_names: vec!["VDD".to_owned()],
                temperatures_c: vec![27.0],
                full_matrix: true,
                nominal_voltage: Some(1.8),
                base_mode: CornerBaseMode::Op,
                model_bindings: Vec::new(),
                points: Vec::new(),
            }),
            ..SpecExecutionOptions::default()
        },
        analysis_line: ".corner".to_owned(),
    };
    let snapshot = PreparedRunSnapshot::new(SnapshotParts {
        intent: SimulationRunIntent::SimulateRunSet,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision: 3,
        topology_revision: 4,
        source_digest: ContentDigest::from_bytes([0x81; 32]),
        reference_process: ProcessCorner::TT,
        reference_temperature_celsius: 27.0,
        run_set: None,
        tasks: vec![PreparedTask::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            Vec::new(),
            "Corner",
            corner,
        )],
        executable_netlist: "corner\nVDD vdd 0 DC 1.8\nR1 vdd out 1k\nR2 out 0 1k\n.op\n.end\n"
            .to_owned(),
        save_policy: SavePolicy::RetainEngineProducedResults,
        model_identities: Vec::new(),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: crate::state::PreparedSpecificationPolicy::default(),
        project_veriloga_runtimes: Default::default(),
        target: ExecutionTargetCapabilities::current(),
        receipt: RunSourceReceipt::SchematicDrc(ContentDigest::from_bytes([0x82; 32])),
        advisories: Vec::new(),
        manual_source: None,
        cross_probe: None,
        touchstone_export: TouchstoneExportPolicy::disabled(),
        sealed_source_dependencies: Vec::new(),
    })
    .expect("corner snapshot validates");
    let digest = snapshot.digest();
    let proof = ExecutionPermitIssuer::default()
        .issue(digest)
        .expect("permit issues")
        .consume(digest, digest)
        .expect("permit consumes");
    let dispatch = snapshot
        .authorize_dispatch(proof)
        .expect("snapshot authorizes");

    let mut controller = SimulationController::new();
    for task in dispatch.tasks() {
        controller.point_families.register(task);
    }
    let mut tasks = dispatch.into_tasks();
    assert_eq!(tasks.len(), 3, "two declared points and the assembly");

    // Both points ran and neither converged. That is still evidence about those
    // corners, and the declaration must answer for it in its own position
    // rather than being dispatched to find out.
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    for _ in 0..2 {
        let point = tasks.pop_front().expect("a declared point");
        let provenance = AnalysisResultProvenance::new(
            point.instance_id(),
            point.source_revision(),
            point.snapshot_digest(),
            point.dependencies().to_vec(),
        )
        .expect("point provenance")
        .with_pvt_point(point.pvt_point().cloned());
        state
            .simulation
            .run_by_sequence_mut(run_sequence)
            .expect("active run")
            .add_analysis(
                AnalysisResult::failed(1, AnalysisType::DcOp, point.label(), "solver failed")
                    .with_provenance(provenance),
            );
    }
    let declaration = tasks.front().expect("the assembly is last").instance_id();

    controller.current_run_id = Some(run_sequence);
    controller.current_analysis_idx = 2;
    controller.total_analyses = 3;
    controller.pending_analyses = tasks;
    controller.start_next_analysis(&mut state);

    assert!(
        !controller.is_running(),
        "the assembly must never be handed to the runner"
    );
    assert_eq!(
        controller.total_analyses, 0,
        "the batch finished in the same call, so nothing is awaiting an engine"
    );
    let run = state
        .simulation
        .run_by_sequence(run_sequence)
        .expect("completed run remains");
    assert_eq!(run.analyses.len(), 3);
    let family = run
        .find_analysis_by_source_instance(declaration)
        .expect("the declaration retained a result in its own position");
    assert_eq!(family.analysis_type, AnalysisType::Corner);
    assert!(!family.success, "no corner converged, so the family failed");
    assert_eq!(run.analyses[2].id, family.id, "and it is retained last");
}

#[test]
fn a_wholesale_catalogue_replacement_cannot_reuse_an_inspection_key_it_did_not_earn() {
    // The inspection digest is a repaint cache key, so the tempting cheap
    // version of it is a mutation counter on the model catalogue. That is
    // unsound here: `ModelLibraryManager` is replaced whole when a project is
    // opened, when a recovery comparison is accepted, when design history
    // restores a candidate, and when a model-hub operation publishes a rebuilt
    // one — and a replacement arrives carrying whatever counter it was
    // serialized with, which may be one this session has already cached an
    // answer against. This builds that collision deliberately: every other
    // input to the digest is held identical, including the two counters it does
    // fold in, and the catalogue is replaced by a *deserialized* one rather
    // than mutated in place, which is how a project open does it.
    use crate::state::model_library::ModelLibraryManager;

    let mut state = AppState::default();
    let epoch = state.design_execution_epoch;
    let project_revision = state.workspace.project.revision();

    let mut first = ModelLibraryManager::new();
    first
        .load_library_bytes(
            "opened-project.lib",
            b".model nch NMOS (LEVEL=1 VTO=0.4)\n".to_vec(),
            None,
        )
        .expect("the first project's catalogue imports");
    let mut second = ModelLibraryManager::new();
    second
        .load_library_bytes(
            "opened-project.lib",
            b".model nch NMOS (LEVEL=1 VTO=0.9)\n.model pch PMOS (LEVEL=1)\n".to_vec(),
            None,
        )
        .expect("the second project's catalogue imports");

    state.model_library_manager = first;
    let before = prepared_run::design_inspection_input_digest(&state);

    state.model_library_manager = serde_json::from_str(
        &serde_json::to_string(&second).expect("a catalogue serializes into a project file"),
    )
    .expect("a catalogue restores out of a project file");

    assert_eq!(
        state.design_execution_epoch, epoch,
        "the collision is only real while every counter the digest folds in is unmoved"
    );
    assert_eq!(state.workspace.project.revision(), project_revision);
    assert_ne!(
        prepared_run::design_inspection_input_digest(&state),
        before,
        "a catalogue replaced wholesale presented a key this session had already \
         cached an answer against; the Bins & geometry page would go on stating the \
         previous project's model cards"
    );
}

#[test]
fn a_failed_run_anchors_its_console_row_to_the_objects_the_engine_named() {
    let mut state = AppState::default();
    let attribution = crate::state::ConvergenceAttribution::from(
        &rspice_core::diagnostics::ConvergenceDiagnostic {
            class: rspice_core::diagnostics::ConvergenceFailureClass::NewtonNonConvergence,
            sites: vec![
                rspice_core::diagnostics::ConvergenceSite {
                    name: "OUT".to_owned(),
                    kind: rspice_core::diagnostics::ConvergenceSiteKind::Node,
                    residual: Some(4.5),
                },
                rspice_core::diagnostics::ConvergenceSite {
                    name: "V1".to_owned(),
                    kind: rspice_core::diagnostics::ConvergenceSiteKind::Branch,
                    residual: None,
                },
            ],
            elided_sites: 0,
            failure_message: "Convergence failed after 100 iterations".to_owned(),
        },
    );

    SimulationController::report_failed_analysis(
        &mut state,
        "Analysis failed: Convergence failed after 100 iterations",
        &Some(attribution),
    );

    let entries: Vec<&crate::diagnostics::LogEntry> = state.log_buffer.entries().collect();
    assert_eq!(
        entries[0].message, "Analysis failed: Convergence failed after 100 iterations",
        "the engine's own prose must reach the console unchanged"
    );
    assert_eq!(
        entries[0].anchor,
        Some(crate::diagnostics::LogAnchor::Simulation {
            nets: vec!["OUT".to_owned()],
            devices: vec!["V1".to_owned()],
        }),
        "the failure row must carry the objects it named"
    );
    assert!(
        entries[1].message.contains("Did not converge")
            && entries[1].message.contains("OUT")
            && entries[1].message.contains("V1"),
        "the named objects are their own statement: {}",
        entries[1].message
    );
}

#[test]
fn a_real_operating_point_refusal_anchors_the_conductor_the_engine_named() {
    // Nothing here is injected: the deck is refused by the solver, the
    // attribution is the one `rspice-core` recorded (`engine/core.rs:1309`),
    // and the console row is written by the same call the run loop makes.
    let bridge = crate::simulation::EngineBridge::new();
    let error = bridge
        .run(
            &crate::simulation::config::AnalysisConfig::DcOp(
                crate::simulation::dialog::OpConfig::default(),
            ),
            "current-driven floating node\n\
             i1 0 out dc 1m\n\
             c1 out 0 1u\n\
             .op\n\
             .end\n",
        )
        .expect_err("a current-driven floating node has no operating point");
    let attribution = error
        .attribution()
        .cloned()
        .expect("the operating point is the analysis that names its refusals");

    let mut state = AppState::default();
    SimulationController::report_failed_analysis(
        &mut state,
        &format!("Analysis failed: {error}"),
        &Some(attribution),
    );

    let entries: Vec<&crate::diagnostics::LogEntry> = state.log_buffer.entries().collect();
    let Some(crate::diagnostics::LogAnchor::Simulation { nets, .. }) = entries[0].anchor.clone()
    else {
        panic!(
            "the refusal row must be anchored to what the engine named, got {:?}",
            entries[0].anchor
        );
    };
    assert!(
        nets.iter().any(|net| net.eq_ignore_ascii_case("out")),
        "the floating node must reach the console anchor by name: {nets:?}"
    );
    assert_eq!(
        entries.len(),
        2,
        "an attributed failure writes the engine's prose and, separately, what it named"
    );
}

#[test]
fn a_failure_that_named_nothing_writes_the_row_it_always_did() {
    let mut state = AppState::default();

    SimulationController::report_failed_analysis(
        &mut state,
        "Analysis failed: Parse error: unterminated .subckt",
        &None,
    );

    let entries: Vec<&crate::diagnostics::LogEntry> = state.log_buffer.entries().collect();
    assert_eq!(entries.len(), 1, "no attribution, no second row");
    assert!(entries[0].anchor.is_none());
}

/// A hand-written deck's `.param` values are read with the engineering
/// notation parser, and a parameter authored with its unit used to be dropped
/// outright: the deck said `tr=1ns`, the parser refused the `s`, and the
/// controller carried no value for `tr` at all.
#[test]
fn manual_deck_parameters_authored_with_units_are_read() {
    let values = SimulationController::manual_deck_param_values(
        "deck\n.param tr=1ns vsupply=5V rload=1k\n.end\n",
    );

    assert_eq!(values.get("tr").copied(), Some(1e-9));
    assert_eq!(values.get("vsupply").copied(), Some(5.0));
    assert_eq!(values.get("rload").copied(), Some(1e3));
}

/// Sealing an aborted run is a new generation of the retained evidence.
///
/// The abort path appends a partial analysis, flips the run's verdict and
/// seals its lifecycle, and it did all of that at a constant data version.
/// Every workspace memo over that evidence is keyed on the version — the
/// manifest's dataset digest among them — so the reader was left holding a
/// projection of the run as it stood before the abort.
#[test]
fn sealing_an_aborted_run_declares_a_new_dataset_generation() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    let run_sequence = state.simulation.start_run().id;
    controller.current_run_id = Some(run_sequence);
    assert!(state.simulation.select_run(0));

    let before = state.simulation.data_version;
    let digest_before = state.simulation.runs[0]
        .dataset_content_digest()
        .to_string();

    controller.seal_aborted_run(
        &mut state,
        Some(AnalysisResult::new(
            1,
            AnalysisType::Transient,
            "aborted TRAN",
        )),
    );

    assert!(!state.simulation.runs[0].success);
    assert_eq!(
        state.simulation.runs[0].lifecycle,
        SimulationRunLifecycle::Aborted
    );
    assert_ne!(
        state.simulation.runs[0]
            .dataset_content_digest()
            .to_string(),
        digest_before,
        "the fixture must actually change the retained evidence"
    );
    assert_ne!(
        state.simulation.data_version, before,
        "the aborted run was sealed at the generation the memos already describe"
    );
}
