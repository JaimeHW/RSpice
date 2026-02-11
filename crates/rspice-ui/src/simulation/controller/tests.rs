use super::*;

// -------------------------------------------------------------------------
// Parse SPICE Value Tests
// -------------------------------------------------------------------------

#[test]
fn test_parse_spice_value_plain_number() {
    assert!((parse_spice_value("100") - 100.0).abs() < 1e-10);
    assert!((parse_spice_value("1.5") - 1.5).abs() < 1e-10);
    assert!((parse_spice_value("-10") - (-10.0)).abs() < 1e-10);
}

#[test]
fn test_parse_spice_value_scientific() {
    assert!((parse_spice_value("1e-9") - 1e-9).abs() < 1e-20);
    assert!((parse_spice_value("2.5E6") - 2.5e6).abs() < 1.0);
}

#[test]
fn test_parse_spice_value_kilo() {
    assert!((parse_spice_value("1k") - 1000.0).abs() < 1e-10);
    assert!((parse_spice_value("10K") - 10000.0).abs() < 1e-10);
    assert!((parse_spice_value("4.7k") - 4700.0).abs() < 1e-10);
}

#[test]
fn test_parse_spice_value_mega() {
    assert!((parse_spice_value("1Meg") - 1e6).abs() < 1.0);
    assert!((parse_spice_value("2.2meg") - 2.2e6).abs() < 1.0);
}

#[test]
fn test_parse_spice_value_milli() {
    assert!((parse_spice_value("1m") - 1e-3).abs() < 1e-15);
    assert!((parse_spice_value("100m") - 0.1).abs() < 1e-10);
}

#[test]
fn test_parse_spice_value_micro() {
    assert!((parse_spice_value("1u") - 1e-6).abs() < 1e-18);
    assert!((parse_spice_value("10u") - 10e-6).abs() < 1e-17);
}

#[test]
fn test_parse_spice_value_nano() {
    assert!((parse_spice_value("1n") - 1e-9).abs() < 1e-21);
    assert!((parse_spice_value("100n") - 100e-9).abs() < 1e-18);
}

#[test]
fn test_parse_spice_value_pico() {
    assert!((parse_spice_value("1p") - 1e-12).abs() < 1e-24);
    assert!((parse_spice_value("10p") - 10e-12).abs() < 1e-23);
}

#[test]
fn test_parse_spice_value_femto() {
    assert!((parse_spice_value("1f") - 1e-15).abs() < 1e-27);
}

#[test]
fn test_parse_spice_value_giga() {
    assert!((parse_spice_value("1G") - 1e9).abs() < 1.0);
    assert!((parse_spice_value("2.4G") - 2.4e9).abs() < 1.0);
}

#[test]
fn test_parse_spice_value_empty() {
    assert_eq!(parse_spice_value(""), 0.0);
    assert_eq!(parse_spice_value("   "), 0.0);
}

#[test]
fn test_parse_spice_value_checked_valid_suffix() {
    let value = parse_spice_value_checked("4.7k").expect("4.7k should parse");
    assert!((value - 4700.0).abs() < 1e-10);
}

#[test]
fn test_parse_spice_value_checked_rejects_unknown_suffix() {
    let err = parse_spice_value_checked("10xyz").expect_err("unknown suffix must fail");
    assert!(err.contains("unsupported SPICE suffix"));
}

// -------------------------------------------------------------------------
// Controller Tests
// -------------------------------------------------------------------------

#[test]
fn test_controller_new() {
    let controller = SimulationController::new();
    assert!(!controller.is_running());
}

#[test]
fn test_controller_default() {
    let controller = SimulationController::default();
    assert!(!controller.is_running());
}

#[test]
fn test_controller_status_initial() {
    let controller = SimulationController::new();
    assert!(matches!(controller.status(), SimulationStatus::Idle));
}

#[test]
fn test_finish_simulation_batch_reports_failed_run_status() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state.simulation.start_run().success = false;

    controller.finish_simulation_batch(&mut state);

    assert_eq!(state.simulation.status, "Completed with errors");
}

#[test]
fn test_start_next_analysis_without_cached_netlist_reports_error_instead_of_panicking() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state.simulation.start_run();

    controller.total_analyses = 1;
    controller.pending_analyses.push_back(QueuedAnalysis {
        spec: AnalysisSpec::DcOp,
        config: Some(AnalysisConfig::DcOp),
        spec_options: SpecExecutionOptions::default(),
        analysis_line: ".OP".to_string(),
    });
    controller.cached_netlist = None;

    controller.start_next_analysis(&mut state);

    assert_eq!(state.simulation.status, "Error");
    assert!(controller.pending_analyses.is_empty());
    assert!(
        state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("missing cached netlist")),
        "expected a user-visible missing-netlist error message"
    );
    assert!(
        state
            .simulation
            .active_run()
            .map(|run| !run.success)
            .unwrap_or(false),
        "active run should be marked failed"
    );
}

#[test]
fn test_analysis_name() {
    let controller = SimulationController::new();

    assert_eq!(
        controller.analysis_name(&AnalysisConfig::DcOp),
        "DC Operating Point"
    );

    let tran = AnalysisConfig::Transient(TransientAnalysisConfig {
        stop_time: 1e-6,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: Some(1e-9),
        uic: false,
    });
    assert_eq!(controller.analysis_name(&tran), "Transient");
}

#[test]
fn test_apply_simulation_options_to_netlist_skips_default_options_block() {
    let netlist = "* test\nV1 in 0 dc 1\n.op\n.end\n";
    let opts = crate::simulation::dialog::SimulationOptions::default();
    let merged = SimulationController::apply_simulation_options_to_netlist(netlist, &opts);
    assert_eq!(merged, netlist);
}

#[test]
fn test_apply_simulation_options_to_netlist_inserts_options_before_end() {
    let netlist = "* test\nV1 in 0 dc 1\n.op\n.end\n";
    let mut opts = crate::simulation::dialog::SimulationOptions::default();
    opts.reltol = 2e-4;
    opts.temp = 85.0;

    let merged = SimulationController::apply_simulation_options_to_netlist(netlist, &opts);
    assert!(merged.contains(".OPTIONS"));
    assert!(merged.contains("RELTOL=2.00e-4"));
    assert!(merged.contains("TEMP=85.00"));

    let options_pos = merged
        .find(".OPTIONS")
        .expect("options block should be present");
    let end_pos = merged.rfind(".end").expect(".end should still be present");
    assert!(options_pos < end_pos, "options block must precede .end");
}

#[test]
fn test_build_transient_config_uses_output_step_without_forcing_internal_max_step() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.tran_stop = "5m".to_string();
    state.dialogs.tran_step = "10n".to_string();
    state.dialogs.tran_start = "0".to_string();

    let spec = controller
        .build_analysis_spec_for_index(&state, 1)
        .expect("transient spec should build");
    let config = controller
        .analysis_spec_to_config(&state, &spec)
        .expect("transient config should build");
    let tran = match config {
        AnalysisConfig::Transient(tran) => tran,
        _ => panic!("Expected transient config"),
    };

    assert!((tran.stop_time - 5e-3).abs() < 1e-15);
    assert!((tran.step_time - 10e-9).abs() < 1e-18);
    assert_eq!(tran.max_timestep, None);
}

#[test]
fn test_enabled_analysis_indices_defaults_to_dcop() {
    let state = AppState::default();
    let indices = SimulationController::enabled_analysis_indices(&state);
    assert_eq!(indices, vec![0]);
}

#[test]
fn test_enabled_analysis_indices_defaults_to_active_tab_when_none_enabled() {
    let mut state = AppState::default();
    state.dialogs.sim_active_tab = 2;
    let indices = SimulationController::enabled_analysis_indices(&state);
    assert_eq!(indices, vec![2]);
}

#[test]
fn test_build_analysis_plan_rejects_unimplemented_analysis_tab() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(99);

    let errors = controller
        .build_analysis_plan(&state)
        .expect_err("unsupported analysis should fail planning");
    assert!(errors.iter().any(|e| e.contains("Unknown")));
}

#[test]
fn test_build_analysis_plan_includes_supported_analyses_in_order() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.extend([1, 2, 4]);
    state.dialogs.tran_stop = "5m".to_string();
    state.dialogs.tran_step = "10n".to_string();
    state.dialogs.ac_fstart = "1".to_string();
    state.dialogs.ac_fstop = "1Meg".to_string();
    state.dialogs.ac_points = "20".to_string();
    state.dialogs.noise_output = "out".to_string();
    state.dialogs.noise_fstart = "10".to_string();
    state.dialogs.noise_fstop = "100Meg".to_string();

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    assert_eq!(plan.analyses.len(), 3);
    assert!(matches!(plan.analyses[0], AnalysisSpec::Transient { .. }));
    assert!(matches!(plan.analyses[1], AnalysisSpec::Ac { .. }));
    assert!(matches!(plan.analyses[2], AnalysisSpec::Noise { .. }));
}

#[test]
fn test_build_analysis_plan_accepts_nested_dc_sweep() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(3);
    state.dialogs.dc_nested = true;
    state.dialogs.dc_source = "V1".to_string();
    state.dialogs.dc_start = "0".to_string();
    state.dialogs.dc_stop = "1".to_string();
    state.dialogs.dc_step = "0.1".to_string();
    state.dialogs.dc_source2 = "V2".to_string();
    state.dialogs.dc_start2 = "0".to_string();
    state.dialogs.dc_stop2 = "2".to_string();
    state.dialogs.dc_step2 = "0.5".to_string();

    let plan = controller
        .build_analysis_plan(&state)
        .expect("nested sweep should build a valid plan");
    assert_eq!(plan.analyses.len(), 1);
    match &plan.analyses[0] {
        AnalysisSpec::DcSweep {
            source_name,
            source2,
            start2,
            stop2,
            step2,
            ..
        } => {
            assert_eq!(source_name, "V1");
            assert_eq!(source2.as_deref(), Some("V2"));
            assert_eq!(*start2, Some(0.0));
            assert_eq!(*stop2, Some(2.0));
            assert_eq!(*step2, Some(0.5));
        }
        other => panic!("expected DC sweep spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_plan_rejects_nested_dc_without_secondary_source() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(3);
    state.dialogs.dc_nested = true;
    state.dialogs.dc_source = "V1".to_string();
    state.dialogs.dc_start = "0".to_string();
    state.dialogs.dc_stop = "1".to_string();
    state.dialogs.dc_step = "0.1".to_string();
    state.dialogs.dc_source2.clear();
    state.dialogs.dc_start2 = "0".to_string();
    state.dialogs.dc_stop2 = "2".to_string();
    state.dialogs.dc_step2 = "0.5".to_string();

    let errors = controller
        .build_analysis_plan(&state)
        .expect_err("nested sweep with missing source2 should fail");
    assert!(errors.iter().any(|e| e.contains("secondary sweep source")));
}

#[test]
fn test_build_queue_from_plan_maps_nested_dc_config() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(3);
    state.dialogs.dc_nested = true;
    state.dialogs.dc_source = "V1".to_string();
    state.dialogs.dc_start = "0".to_string();
    state.dialogs.dc_stop = "1".to_string();
    state.dialogs.dc_step = "0.1".to_string();
    state.dialogs.dc_source2 = "V2".to_string();
    state.dialogs.dc_start2 = "0".to_string();
    state.dialogs.dc_stop2 = "2".to_string();
    state.dialogs.dc_step2 = "0.5".to_string();

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");
    assert_eq!(queue.len(), 1);
    match &queue[0].config {
        Some(AnalysisConfig::DcSweep(dc)) => {
            assert_eq!(dc.source, "V1");
            assert_eq!(dc.source2.as_deref(), Some("V2"));
            assert_eq!(dc.start2, Some(0.0));
            assert_eq!(dc.stop2, Some(2.0));
            assert_eq!(dc.step2, Some(0.5));
        }
        other => panic!("expected nested DC config, got {:?}", other),
    }
}

#[test]
fn test_build_queue_from_plan_maps_pole_zero_config() {
    use crate::simulation::dialog::pz::{PzAnalysisType, PzConfig, PzTransferType};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(5);
    state.dialogs.pz_state = crate::simulation::dialog::pz::PzDialogState::from_config(
        &PzConfig::new("vin", "vout")
            .with_transfer(PzTransferType::Current)
            .with_type(PzAnalysisType::PolesOnly),
    );

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");
    assert_eq!(queue.len(), 1);

    match &queue[0].config {
        Some(AnalysisConfig::PoleZero(pz)) => {
            assert_eq!(pz.input_node, "VIN");
            assert_eq!(pz.output_node, "VOUT");
            assert_eq!(pz.transfer_type, "CUR");
            assert!(matches!(
                pz.analysis_type,
                crate::simulation::config::PzAnalysisType::PolesOnly
            ));
        }
        _ => panic!("Expected pole-zero config"),
    }
}

#[test]
fn test_build_analysis_spec_for_pole_zero_uses_dialog_configuration() {
    use crate::simulation::dialog::pz::{PzAnalysisType, PzConfig, PzTransferType};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.pz_state = crate::simulation::dialog::pz::PzDialogState::from_config(
        &PzConfig::new("in", "out")
            .with_transfer(PzTransferType::Current)
            .with_type(PzAnalysisType::ZerosOnly),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 5)
        .expect("pole-zero spec should build");
    match spec {
        AnalysisSpec::PoleZero {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type,
            analysis_type,
        } => {
            assert_eq!(input_node, "IN");
            assert_eq!(input_ref, "0");
            assert_eq!(output_node, "OUT");
            assert_eq!(output_ref, "0");
            assert_eq!(transfer_type, "CUR");
            assert_eq!(analysis_type, "ZER");
        }
        _ => panic!("Expected pole-zero spec"),
    }
}

#[test]
fn test_build_analysis_spec_for_sensitivity_uses_dialog_configuration() {
    use crate::simulation::dialog::sens::{SensConfig, SensType};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.sens_state = crate::simulation::dialog::sens::SensDialogState::from_config(
        &SensConfig::new("V(out)")
            .with_type(SensType::Ac)
            .with_ac_freq(5e6),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 6)
        .expect("sensitivity spec should build");
    match spec {
        AnalysisSpec::Sensitivity {
            output_var,
            ac_mode,
            frequency,
        } => {
            assert_eq!(output_var, "V(out)");
            assert!(ac_mode);
            assert_eq!(frequency, Some(5e6));
        }
        _ => panic!("Expected sensitivity spec"),
    }
}

#[test]
fn test_build_analysis_spec_for_monte_carlo_uses_dialog_validation() {
    use crate::simulation::dialog::mc::{McBaseAnalysis, McConfig, McDistribution};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.mc_state = crate::simulation::dialog::mc::McDialogState::from_config(
        &McConfig::new(64)
            .with_distribution(McDistribution::Gaussian)
            .with_base(McBaseAnalysis::Dc)
            .with_seed(1234),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 7)
        .expect("Monte Carlo spec should build");
    assert!(matches!(spec, AnalysisSpec::MonteCarlo));
}

#[test]
fn test_build_analysis_spec_for_pss_uses_dialog_configuration() {
    use crate::simulation::dialog::pss::PssConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.pss_state = crate::simulation::dialog::pss::PssDialogState::from_config(
        &PssConfig::new(2.5e6).with_harmonics(15),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 8)
        .expect("PSS spec should build");
    match spec {
        AnalysisSpec::Pss {
            fundamental_freq,
            num_harmonics,
            tolerance,
        } => {
            assert!((fundamental_freq - 2.5e6).abs() < 1e-6);
            assert_eq!(num_harmonics, 15);
            assert!((tolerance - 1e-3).abs() < 1e-15);
        }
        other => panic!("expected PSS spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_stb_uses_dialog_configuration() {
    use crate::simulation::dialog::stb::StbConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.stb_state = crate::simulation::dialog::stb::StbDialogState::from_config(
        &StbConfig::new("L1")
            .with_freq_range(10.0, 1e6)
            .with_points(12),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 9)
        .expect("STB spec should build");
    match spec {
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            points_per_decade,
        } => {
            assert_eq!(probe_node, "L1");
            assert!((start_freq - 10.0).abs() < 1e-12);
            assert!((stop_freq - 1e6).abs() < 1e-3);
            assert_eq!(points_per_decade, 12);
        }
        other => panic!("expected STB spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_harmonic_balance_uses_dialog_configuration() {
    use crate::simulation::dialog::hb::{HbConfig, HbSolverType, HbToneConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.hb_state = crate::simulation::dialog::hb::HbDialogState::from_config(
        &HbConfig::new(1.2e9, 11)
            .add_tone(
                HbToneConfig::new(900e6, 5)
                    .with_name("LO")
                    .with_source("VLO"),
            )
            .add_tone(HbToneConfig::new(2.1e9, 3).with_name("AUX"))
            .with_solver(HbSolverType::Krylov)
            .with_oversample(6)
            .with_tolerance(2e-6)
            .with_source_stepping(true),
    );
    state.dialogs.hb_state.fundamental_name = "RF".to_string();
    state.dialogs.hb_state.fundamental_source = "VRF".to_string();
    state.dialogs.hb_state.maxiter = "175".to_string();
    state.dialogs.hb_state.damping = "0.6".to_string();

    let spec = controller
        .build_analysis_spec_for_index(&state, 11)
        .expect("HB spec should build");
    match spec {
        AnalysisSpec::HarmonicBalance {
            tones,
            reltol,
            abstol,
            max_iterations,
            damping,
            oversample,
            max_mixing_order,
            use_krylov,
            gmres_restart,
            source_stepping,
            verbose,
        } => {
            assert_eq!(tones.len(), 3);
            assert!((tones[0].frequency - 1.2e9).abs() < 1e-3);
            assert_eq!(tones[0].harmonics, 11);
            assert_eq!(tones[0].name.as_deref(), Some("RF"));
            assert_eq!(tones[0].source.as_deref(), Some("VRF"));
            assert!((tones[1].frequency - 900e6).abs() < 1e-3);
            assert_eq!(tones[1].harmonics, 5);
            assert_eq!(tones[1].name.as_deref(), Some("LO"));
            assert_eq!(tones[1].source.as_deref(), Some("VLO"));
            assert!((tones[2].frequency - 2.1e9).abs() < 1e-3);
            assert_eq!(tones[2].harmonics, 3);
            assert_eq!(tones[2].name.as_deref(), Some("AUX"));
            assert!(tones[2].source.is_none());
            assert!((reltol - 2e-6).abs() < 1e-18);
            assert!((abstol - 1e-12).abs() < 1e-24);
            assert_eq!(max_iterations, 175);
            assert!((damping - 0.6).abs() < 1e-15);
            assert_eq!(oversample, 6);
            assert_eq!(max_mixing_order, 5);
            assert!(use_krylov);
            assert_eq!(gmres_restart, 30);
            assert!(source_stepping);
            assert!(!verbose);
        }
        other => panic!("expected harmonic balance spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_sparameter_uses_dialog_configuration() {
    use crate::simulation::dialog::sp::{SpConfig, SpPortConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.sp_state = crate::simulation::dialog::sp::SpDialogState::from_config(
        &SpConfig::decade(1e6, 2e9, 20)
            .with_z0(75.0)
            .with_ports(vec![
                SpPortConfig::single_ended(1, "rf_in"),
                SpPortConfig::single_ended(2, "rf_out"),
            ]),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 12)
        .expect("S-parameter spec should build");
    match spec {
        AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            z0,
            ports,
        } => {
            assert!((start_freq - 1e6).abs() < 1e-6);
            assert!((stop_freq - 2e9).abs() < 1e-3);
            assert_eq!(points_per_unit, 20);
            assert!(matches!(sweep, FrequencySweep::Decade));
            assert!((z0 - 75.0).abs() < 1e-9);
            assert_eq!(ports.len(), 2);
            assert_eq!(ports[0].node_pos, "RF_IN");
            assert_eq!(ports[0].node_neg, "0");
            assert_eq!(ports[1].node_pos, "RF_OUT");
            assert_eq!(ports[1].node_neg, "0");
        }
        other => panic!("expected S-parameter spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_envelope_uses_dialog_configuration() {
    use crate::simulation::dialog::envelope::EnvelopeConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.envelope_state =
        crate::simulation::dialog::envelope::EnvelopeDialogState::from_config(
            &EnvelopeConfig::new(5e9, 2e-6).with_harmonics(13),
        );

    let spec = controller
        .build_analysis_spec_for_index(&state, 19)
        .expect("Envelope spec should build");
    match spec {
        AnalysisSpec::Envelope {
            fundamental_freq,
            stop_time,
            num_harmonics,
            max_step,
        } => {
            assert!((fundamental_freq - 5e9).abs() < 1e-3);
            assert!((stop_time - 2e-6).abs() < 1e-15);
            assert_eq!(num_harmonics, 13);
            assert_eq!(max_step, None);
        }
        other => panic!("expected Envelope spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_fourier_uses_dialog_configuration() {
    use crate::simulation::dialog::fourier::FourierConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.fourier_state =
        crate::simulation::dialog::fourier::FourierDialogState::from_config(
            &FourierConfig::new(2e6, 15)
                .with_output("outp")
                .with_window(1e-6, 11e-6),
        );

    let spec = controller
        .build_analysis_spec_for_index(&state, 20)
        .expect("Fourier spec should build");
    match spec {
        AnalysisSpec::Fourier {
            fundamental_freq,
            num_harmonics,
            output_node,
            output_ref,
            start_time,
            stop_time,
        } => {
            assert!((fundamental_freq - 2e6).abs() < 1e-6);
            assert_eq!(num_harmonics, 15);
            assert_eq!(output_node, "OUTP");
            assert_eq!(output_ref, "");
            assert!((start_time - 1e-6).abs() < 1e-15);
            assert!((stop_time - 11e-6).abs() < 1e-15);
        }
        other => panic!("expected Fourier spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_reliability_uses_dialog_configuration() {
    use crate::simulation::dialog::reliability::ReliabilityConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.reliability_state =
        crate::simulation::dialog::reliability::ReliabilityDialogState::from_config(
            &ReliabilityConfig {
                target_years: vec![2.0, 7.0, 15.0],
                enable_hci: true,
                enable_nbti: false,
                enable_em: true,
                min_stress_voltage: 0.2,
            },
        );

    let spec = controller
        .build_analysis_spec_for_index(&state, 21)
        .expect("Reliability spec should build");
    match spec {
        AnalysisSpec::Reliability {
            target_years,
            enable_hci,
            enable_nbti,
            enable_em,
            min_stress_voltage,
        } => {
            assert_eq!(target_years, vec![2.0, 7.0, 15.0]);
            assert!(enable_hci);
            assert!(!enable_nbti);
            assert!(enable_em);
            assert!((min_stress_voltage - 0.2).abs() < 1e-12);
        }
        other => panic!("expected Reliability spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_optimization_uses_dialog_configuration() {
    use crate::simulation::dialog::optimization::{
        OptimizationAlgorithmMode, OptimizationConfig, OptimizationGoalMode,
        OptimizationVariableConfig,
    };

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.optimization_state =
        crate::simulation::dialog::optimization::OptimizationDialogState::from_config(
            &OptimizationConfig {
                variables: vec![OptimizationVariableConfig {
                    name: "RLOAD".to_string(),
                    min: 500.0,
                    max: 5_000.0,
                    initial: 1_000.0,
                }],
                objective_node: "out".to_string(),
                objective_ref: "0".to_string(),
                goal_mode: OptimizationGoalMode::Target,
                target_value: Some(1.1),
                algorithm: OptimizationAlgorithmMode::PatternSearch,
                max_iterations: 80,
                cost_tolerance: 1e-9,
                fd_step: 1e-4,
                initial_step: 0.2,
                min_step: 1e-8,
            },
        );

    let spec = controller
        .build_analysis_spec_for_index(&state, 22)
        .expect("Optimization spec should build");
    match spec {
        AnalysisSpec::Optimization {
            variables,
            objective_node,
            objective_ref,
            goal,
            target,
            algorithm,
            max_iterations,
            cost_tolerance,
            ..
        } => {
            assert_eq!(variables.len(), 1);
            assert_eq!(variables[0].name, "RLOAD");
            assert_eq!(objective_node, "out");
            assert_eq!(objective_ref, "0");
            assert!(matches!(goal, OptimizationGoal::Target));
            assert_eq!(target, Some(1.1));
            assert!(matches!(algorithm, OptimizationAlgorithm::PatternSearch));
            assert_eq!(max_iterations, 80);
            assert!((cost_tolerance - 1e-9).abs() < 1e-18);
        }
        other => panic!("expected Optimization spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_soa_uses_dialog_configuration() {
    use crate::simulation::dialog::soa::SoaConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.soa_state =
        crate::simulation::dialog::soa::SoaDialogState::from_config(&SoaConfig {
            stop_time: 2e-6,
            step_time: 5e-9,
            check_vgs_max: true,
            max_vgs: 1.6,
            check_vds_max: false,
            max_vds: 3.3,
            check_vbe_max: true,
            max_vbe: 0.8,
            check_vce_max: false,
            max_vce: 5.0,
        });

    let spec = controller
        .build_analysis_spec_for_index(&state, 23)
        .expect("SOA spec should build");
    match spec {
        AnalysisSpec::Soa {
            stop_time,
            step_time,
            check_vgs_max,
            max_vgs,
            check_vds_max,
            check_vbe_max,
            max_vbe,
            check_vce_max,
            ..
        } => {
            assert!((stop_time - 2e-6).abs() < 1e-15);
            assert!((step_time - 5e-9).abs() < 1e-18);
            assert!(check_vgs_max);
            assert!((max_vgs - 1.6).abs() < 1e-12);
            assert!(!check_vds_max);
            assert!(check_vbe_max);
            assert!((max_vbe - 0.8).abs() < 1e-12);
            assert!(!check_vce_max);
        }
        other => panic!("expected SOA spec, got {:?}", other),
    }
}

#[test]
fn test_build_analysis_spec_for_disto_uses_dialog_configuration() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.ac_fstart = "10".to_string();
    state.dialogs.ac_fstop = "10Meg".to_string();
    state.dialogs.ac_points = "12".to_string();
    state.dialogs.ac_sweep_type = 1; // octave
    state.dialogs.disto_f2_over_f1 = "1.75".to_string();

    let spec = controller
        .build_analysis_spec_for_index(&state, 24)
        .expect("DISTO spec should build");
    match spec {
        AnalysisSpec::Disto {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            f2_over_f1,
        } => {
            assert!((start_freq - 10.0).abs() < 1e-12);
            assert!((stop_freq - 10e6).abs() < 1e-6);
            assert_eq!(points_per_unit, 12);
            assert!(matches!(sweep, FrequencySweep::Octave));
            assert_eq!(f2_over_f1, Some(1.75));
        }
        other => panic!("expected DISTO spec, got {:?}", other),
    }
}

#[test]
fn test_build_queue_from_plan_uses_executable_optimization_command() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(22);

    let plan = controller
        .build_analysis_plan(&state)
        .expect("optimization plan should build");
    assert_eq!(plan.analyses.len(), 1);

    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("optimization queue should build");
    assert_eq!(queue.len(), 1);
    assert!(
        queue[0].analysis_line.starts_with(".opt "),
        "optimization command must be emitted as executable SPICE, got: {}",
        queue[0].analysis_line
    );
    assert!(
        !queue[0].analysis_line.trim_start().starts_with('*'),
        "optimization command must not be commented out"
    );
}

#[test]
fn test_build_queue_from_plan_uses_executable_soa_command() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(23);

    let plan = controller
        .build_analysis_plan(&state)
        .expect("soa plan should build");
    assert_eq!(plan.analyses.len(), 1);

    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("soa queue should build");
    assert_eq!(queue.len(), 1);
    assert!(
        queue[0].analysis_line.starts_with(".soa "),
        "soa command must be emitted as executable SPICE, got: {}",
        queue[0].analysis_line
    );
    assert!(
        !queue[0].analysis_line.trim_start().starts_with('*'),
        "soa command must not be commented out"
    );
}

#[test]
fn test_build_queue_from_plan_routes_disto_via_spec_with_disto_command_line() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(24);
    state.dialogs.ac_fstart = "1".to_string();
    state.dialogs.ac_fstop = "1Meg".to_string();
    state.dialogs.ac_points = "10".to_string();
    state.dialogs.disto_f2_over_f1 = "1.5".to_string();

    let plan = controller
        .build_analysis_plan(&state)
        .expect("disto plan should build");
    assert_eq!(plan.analyses.len(), 1);
    assert!(matches!(plan.analyses[0], AnalysisSpec::Disto { .. }));

    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("disto queue should build");
    assert_eq!(queue.len(), 1);
    assert!(
        queue[0].config.is_none(),
        "DISTO should execute via spec path"
    );
    assert!(
        queue[0].analysis_line.starts_with(".disto "),
        "DISTO command should emit native DISTO command, got: {}",
        queue[0].analysis_line
    );
    assert!(
        queue[0].analysis_line.contains(" 1.5"),
        "DISTO command should include optional f2/f1 ratio when set, got: {}",
        queue[0].analysis_line
    );
}

#[test]
fn test_build_queue_from_plan_routes_disto_without_optional_ratio() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(24);
    state.dialogs.ac_fstart = "10".to_string();
    state.dialogs.ac_fstop = "10Meg".to_string();
    state.dialogs.ac_points = "8".to_string();
    state.dialogs.disto_f2_over_f1 = String::new();

    let plan = controller
        .build_analysis_plan(&state)
        .expect("disto plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("disto queue should build");
    assert_eq!(queue.len(), 1);
    assert!(queue[0].analysis_line.starts_with(".disto "));
    assert_eq!(queue[0].analysis_line.split_whitespace().count(), 5);
}

#[test]
fn test_build_analysis_spec_for_pac_accepts_valid_dialog_configuration() {
    use crate::simulation::dialog::pac::PacConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.pac_state = crate::simulation::dialog::pac::PacDialogState::from_config(
        &PacConfig::new(10e3, 5e6, 12)
            .with_input("V1")
            .with_output("OUT")
            .with_sidebands(3),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 13)
        .expect("PAC spec should build");
    assert!(matches!(spec, AnalysisSpec::Pac));
}

#[test]
fn test_build_analysis_spec_for_pnoise_accepts_valid_dialog_configuration() {
    use crate::simulation::dialog::pnoise::{NoiseReferenceType, PnoiseConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.pnoise_state = crate::simulation::dialog::pnoise::PnoiseDialogState::from_config(
        &PnoiseConfig::new(10.0, 10e6, 12)
            .with_output("OUT")
            .with_sidebands(3)
            .with_noise_ref(NoiseReferenceType::Phase),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 14)
        .expect("PNOISE spec should build");
    assert!(matches!(spec, AnalysisSpec::Pnoise));
}

#[test]
fn test_build_analysis_spec_for_pxf_accepts_valid_dialog_configuration() {
    use crate::simulation::dialog::pxf::PxfConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.pxf_state = crate::simulation::dialog::pxf::PxfDialogState::from_config(
        &PxfConfig::new(10.0, 10e6, 12)
            .with_input("V1")
            .with_output("OUT", 1)
            .with_sidebands(3),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 15)
        .expect("PXF spec should build");
    assert!(matches!(spec, AnalysisSpec::Pxf));
}

#[test]
fn test_build_analysis_spec_for_pstb_accepts_valid_dialog_configuration() {
    use crate::simulation::dialog::pstb::PstbConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.pstb_state = crate::simulation::dialog::pstb::PstbDialogState::from_config(
        &PstbConfig::new("lprobe")
            .with_harmonics(12)
            .with_multipliers(6)
            .with_annotate(false),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 16)
        .expect("PSTB spec should build");
    assert!(matches!(spec, AnalysisSpec::Pstb));
}

#[test]
fn test_build_analysis_spec_for_tf_accepts_valid_dialog_configuration() {
    use crate::simulation::dialog::xf::{XfConfig, XfSweepType};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    let mut cfg = XfConfig::new(1e3, 1e9, 20)
        .with_input("V1")
        .with_output("OUT")
        .with_group_delay(true);
    cfg.sweep_type = XfSweepType::Octave;
    cfg.input_impedance = true;
    cfg.output_impedance = true;
    state.dialogs.xf_state = crate::simulation::dialog::xf::XfDialogState::from_config(&cfg);

    let spec = controller
        .build_analysis_spec_for_index(&state, 17)
        .expect("TF spec should build");
    assert!(matches!(spec, AnalysisSpec::Tf));
}

#[test]
fn test_build_queue_from_plan_emits_worst_case_monte_carlo_command() {
    use crate::simulation::dialog::mc::{McConfig, McDistribution};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(7);
    state.dialogs.mc_state = crate::simulation::dialog::mc::McDialogState::from_config(
        &McConfig::new(16)
            .with_distribution(McDistribution::WorstCase)
            .with_seed(9),
    );

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 1);
    assert!(matches!(queue[0].spec, AnalysisSpec::MonteCarlo));
    assert!(
        queue[0].analysis_line.contains("DIST WORSTCASE"),
        "expected WORSTCASE distribution in .MC command"
    );
}

#[test]
fn test_build_queue_from_plan_stores_pss_and_hb_as_spec_executed_runs() {
    use crate::simulation::dialog::hb::HbConfig;
    use crate::simulation::dialog::pac::PacConfig;
    use crate::simulation::dialog::pss::PssConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [8usize, 11usize, 13usize].into_iter().collect();
    state.dialogs.pss_state =
        crate::simulation::dialog::pss::PssDialogState::from_config(&PssConfig::new(10e6));
    state.dialogs.hb_state =
        crate::simulation::dialog::hb::HbDialogState::from_config(&HbConfig::new(2.4e9, 9));
    state.dialogs.pac_state = crate::simulation::dialog::pac::PacDialogState::from_config(
        &PacConfig::new(1e3, 1e6, 8)
            .with_input("V1")
            .with_output("OUT")
            .with_sidebands(2),
    );
    state.dialogs.simulation_options_config.reltol = 2e-4;
    state.dialogs.simulation_options_config.abstol = 3e-11;

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 3);
    assert!(matches!(queue[0].spec, AnalysisSpec::Pss { .. }));
    assert!(queue[0].config.is_none());
    assert!(queue[0].analysis_line.starts_with(".pss "));

    assert!(matches!(
        queue[1].spec,
        AnalysisSpec::HarmonicBalance { .. }
    ));
    assert!(queue[1].config.is_none());
    assert!(queue[1].analysis_line.starts_with(".hb "));

    assert!(matches!(queue[2].spec, AnalysisSpec::Pac));
    assert!(queue[2].config.is_none());
    assert!(queue[2].analysis_line.starts_with(".pac "));
    assert!(queue[2].spec_options.pac.is_some());
    assert!(matches!(
        queue[2]
            .spec_options
            .pac
            .as_ref()
            .expect("PAC options should be present")
            .sweep,
        crate::services::simulation_runner::PacFrequencySweep::Decade
    ));
    let pac_cfg = queue[2]
        .spec_options
        .pac
        .as_ref()
        .expect("PAC options should be present");
    assert!((pac_cfg.reltol - 2e-4).abs() < 1e-18);
    assert!((pac_cfg.abstol - 3e-11).abs() < 1e-22);
}

#[test]
fn test_build_queue_from_plan_stores_stb_as_spec_executed_run() {
    use crate::simulation::dialog::stb::StbConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [9usize].into_iter().collect();
    state.dialogs.stb_state = crate::simulation::dialog::stb::StbDialogState::from_config(
        &StbConfig::new("L1")
            .with_freq_range(1.0, 1e6)
            .with_points(16),
    );

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 1);
    assert!(matches!(queue[0].spec, AnalysisSpec::Stb { .. }));
    assert!(queue[0].config.is_none());
    assert!(queue[0].analysis_line.starts_with(".stb "));
    assert!(queue[0].spec_options.pac.is_none());
    assert!(queue[0].spec_options.pxf.is_none());
    assert!(queue[0].spec_options.tf.is_none());
    assert!(queue[0].spec_options.pnoise.is_none());
}

#[test]
fn test_build_queue_from_plan_stores_pxf_as_spec_executed_run() {
    use crate::simulation::dialog::pss::PssConfig;
    use crate::simulation::dialog::pxf::PxfConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [15usize].into_iter().collect();
    state.dialogs.pss_state =
        crate::simulation::dialog::pss::PssDialogState::from_config(&PssConfig::new(5e6));
    state.dialogs.pxf_state = crate::simulation::dialog::pxf::PxfDialogState::from_config(
        &PxfConfig::new(1e3, 1e6, 10)
            .with_input("V1")
            .with_output("OUT", 1)
            .with_sidebands(3),
    );
    state.dialogs.simulation_options_config.reltol = 7e-4;
    state.dialogs.simulation_options_config.abstol = 4e-12;

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 1);
    assert!(matches!(queue[0].spec, AnalysisSpec::Pxf));
    assert!(queue[0].config.is_none());
    assert!(queue[0].analysis_line.starts_with(".pxf "));
    let pxf_cfg = queue[0]
        .spec_options
        .pxf
        .as_ref()
        .expect("PXF options should be present");
    assert_eq!(pxf_cfg.input_source, "V1");
    assert_eq!(pxf_cfg.output_sideband, 1);
    assert_eq!(pxf_cfg.max_sideband, 3);
    assert!(matches!(
        pxf_cfg.sweep,
        crate::services::simulation_runner::PxfFrequencySweep::Decade
    ));
    assert!((pxf_cfg.reltol - 7e-4).abs() < 1e-18);
    assert!((pxf_cfg.abstol - 4e-12).abs() < 1e-24);
}

#[test]
fn test_build_queue_from_plan_stores_pstb_as_spec_executed_run() {
    use crate::simulation::dialog::pss::PssConfig;
    use crate::simulation::dialog::pstb::PstbConfig;

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [16usize].into_iter().collect();
    state.dialogs.pss_state =
        crate::simulation::dialog::pss::PssDialogState::from_config(&PssConfig::new(6e6));
    state.dialogs.pstb_state = crate::simulation::dialog::pstb::PstbDialogState::from_config(
        &PstbConfig::new("LPROBE")
            .with_harmonics(12)
            .with_multipliers(4),
    );

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 1);
    assert!(matches!(queue[0].spec, AnalysisSpec::Pstb));
    assert!(queue[0].config.is_none());
    assert!(queue[0].analysis_line.starts_with(".pstb "));

    let pstb_cfg = queue[0]
        .spec_options
        .pstb
        .as_ref()
        .expect("PSTB options should be present");
    assert_eq!(pstb_cfg.probe_instance, "LPROBE");
    assert_eq!(pstb_cfg.max_harmonics, 12);
    assert_eq!(pstb_cfg.num_multipliers, 4);
    assert!((pstb_cfg.pss_fundamental_freq - 6e6).abs() < 1e-6);
}

#[test]
fn test_build_queue_from_plan_stores_tf_and_pnoise_as_spec_executed_runs() {
    use crate::simulation::dialog::pnoise::{NoiseReferenceType, PnoiseConfig};
    use crate::simulation::dialog::pss::PssConfig;
    use crate::simulation::dialog::xf::{XfConfig, XfSweepType};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [14usize, 17usize].into_iter().collect();

    state.dialogs.pss_state =
        crate::simulation::dialog::pss::PssDialogState::from_config(&PssConfig::new(5e6));
    state.dialogs.pnoise_state = crate::simulation::dialog::pnoise::PnoiseDialogState::from_config(
        &PnoiseConfig::new(10.0, 10e6, 10)
            .with_output("OUT")
            .with_input("V1")
            .with_sidebands(2)
            .with_noise_ref(NoiseReferenceType::Phase),
    );
    state.dialogs.simulation_options_config.reltol = 9e-4;
    state.dialogs.simulation_options_config.abstol = 6e-13;

    let mut xf_cfg = XfConfig::new(1e3, 1e8, 8)
        .with_input("V1")
        .with_output("OUT")
        .with_group_delay(true);
    xf_cfg.sweep_type = XfSweepType::Linear;
    xf_cfg.input_impedance = true;
    xf_cfg.output_impedance = true;
    state.dialogs.xf_state = crate::simulation::dialog::xf::XfDialogState::from_config(&xf_cfg);

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 2);
    assert!(matches!(queue[0].spec, AnalysisSpec::Pnoise));
    assert!(queue[0].config.is_none());
    assert!(queue[0].analysis_line.starts_with(".pnoise "));
    assert!(queue[0].spec_options.pnoise.is_some());
    let pnoise_cfg = queue[0]
        .spec_options
        .pnoise
        .as_ref()
        .expect("PNOISE options should be present");
    assert!(matches!(
        pnoise_cfg.noise_ref,
        crate::services::simulation_runner::PnoiseReference::Phase
    ));
    assert_eq!(pnoise_cfg.input_source, "V1");
    assert!((pnoise_cfg.reltol - 9e-4).abs() < 1e-18);
    assert!((pnoise_cfg.abstol - 6e-13).abs() < 1e-25);

    assert!(matches!(queue[1].spec, AnalysisSpec::Tf));
    assert!(queue[1].config.is_none());
    assert!(queue[1].analysis_line.starts_with(".xf "));
    assert!(queue[1].spec_options.tf.is_some());
    let tf_cfg = queue[1]
        .spec_options
        .tf
        .as_ref()
        .expect("TF options should be present");
    assert!(tf_cfg.group_delay);
    assert!(tf_cfg.input_impedance);
    assert!(tf_cfg.output_impedance);
    assert!(matches!(
        tf_cfg.sweep,
        crate::services::simulation_runner::TfFrequencySweep::Linear
    ));
}

#[test]
fn test_build_analysis_spec_for_temperature_sweep_accepts_transient_base() {
    use crate::simulation::dialog::temp::{TempBaseAnalysis, TempConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.temp_state = crate::simulation::dialog::temp::TempDialogState::from_config(
        &TempConfig::new(-40.0, 125.0, 25.0).with_base(TempBaseAnalysis::Transient),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 10)
        .expect("Transient base should be accepted for temperature sweeps");
    assert!(matches!(spec, AnalysisSpec::Parametric));
}

#[test]
fn test_build_analysis_spec_for_corner_accepts_process_and_voltage_sweeps() {
    use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.corner_state = crate::simulation::dialog::corner::CornerDialogState::from_config(
        &CornerConfig::default()
            .with_process_corners(vec![ProcessCorner::TT, ProcessCorner::FF])
            .with_voltages(vec![0.9, 1.0, 1.1])
            .with_temperatures(vec![-40.0, 25.0, 125.0])
            .with_base_analysis(CornerBaseAnalysis::Op),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 18)
        .expect("corner spec should build for full PVT sweep");
    assert!(matches!(spec, AnalysisSpec::Corner));
}

#[test]
fn test_build_queue_from_plan_stores_spec_executed_analyses_without_config() {
    use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};
    use crate::simulation::dialog::temp::{TempBaseAnalysis, TempConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [7usize, 10usize, 18usize].into_iter().collect();
    state.dialogs.temp_state = crate::simulation::dialog::temp::TempDialogState::from_config(
        &TempConfig::new(-40.0, 85.0, 25.0).with_base(TempBaseAnalysis::Op),
    );
    state.dialogs.corner_state = crate::simulation::dialog::corner::CornerDialogState::from_config(
        &CornerConfig::default()
            .with_process_corners(vec![ProcessCorner::TT])
            .with_base_analysis(CornerBaseAnalysis::Op),
    );

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 3);
    assert!(matches!(queue[0].spec, AnalysisSpec::MonteCarlo));
    assert!(queue[0].config.is_none());
    assert!(queue[0].spec_options.corner.is_none());
    assert!(queue[0].analysis_line.starts_with(".mc "));

    assert!(matches!(queue[1].spec, AnalysisSpec::Parametric));
    assert!(queue[1].config.is_none());
    assert!(queue[1].spec_options.corner.is_none());
    assert!(queue[1].spec_options.temp.is_some());
    assert!(matches!(
        queue[1]
            .spec_options
            .temp
            .as_ref()
            .expect("temperature options must be present")
            .base_mode,
        crate::services::simulation_runner::CornerBaseMode::Op
    ));
    assert!(queue[1].analysis_line.starts_with(".step temp "));

    assert!(matches!(queue[2].spec, AnalysisSpec::Corner));
    assert!(queue[2].config.is_none());
    assert!(queue[2].spec_options.corner.is_some());
    assert!(matches!(
        queue[2]
            .spec_options
            .corner
            .as_ref()
            .expect("corner options must be present")
            .base_mode,
        crate::services::simulation_runner::CornerBaseMode::Op
    ));
    assert!(queue[2].analysis_line.starts_with(".temp "));
}

#[test]
fn test_build_analysis_spec_for_corner_accepts_transient_base_mode() {
    use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.corner_state = crate::simulation::dialog::corner::CornerDialogState::from_config(
        &CornerConfig::default().with_base_analysis(CornerBaseAnalysis::Transient),
    );

    let spec = controller
        .build_analysis_spec_for_index(&state, 18)
        .expect("corner transient base mode should be accepted");
    assert!(matches!(spec, AnalysisSpec::Corner));
}

#[test]
fn test_build_queue_from_plan_maps_temperature_ac_base_mode() {
    use crate::simulation::dialog::temp::{TempBaseAnalysis, TempConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [10usize].into_iter().collect();
    state.dialogs.temp_state = crate::simulation::dialog::temp::TempDialogState::from_config(
        &TempConfig::new(-40.0, 125.0, 82.5).with_base(TempBaseAnalysis::Ac),
    );
    state.dialogs.ac_fstart = "1k".to_string();
    state.dialogs.ac_fstop = "10Meg".to_string();
    state.dialogs.ac_points = "12".to_string();
    state.dialogs.ac_sweep_type = 1; // octave

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 1);
    let temp = queue[0]
        .spec_options
        .temp
        .as_ref()
        .expect("temperature options must be present");
    match &temp.base_mode {
        crate::services::simulation_runner::CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => {
            assert!((*start_freq - 1e3).abs() < 1e-12);
            assert!((*stop_freq - 1e7).abs() < 1e-4);
            assert_eq!(*points_per_unit, 12);
            assert!(matches!(
                sweep,
                crate::services::simulation_runner::CornerFrequencySweep::Octave
            ));
        }
        other => panic!("expected AC temp base mode, got {:?}", other),
    }
}

#[test]
fn test_build_queue_from_plan_rejects_temperature_dc_without_source() {
    use crate::simulation::dialog::temp::{TempBaseAnalysis, TempConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [10usize].into_iter().collect();
    state.dialogs.temp_state = crate::simulation::dialog::temp::TempDialogState::from_config(
        &TempConfig::new(-40.0, 125.0, 25.0).with_base(TempBaseAnalysis::Dc),
    );
    state.dialogs.dc_source.clear();
    state.dialogs.dc_start = "0".to_string();
    state.dialogs.dc_stop = "1".to_string();
    state.dialogs.dc_step = "0.1".to_string();

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let err = controller
        .build_queue_from_plan(&state, &plan)
        .expect_err("temperature DC base mode should require source");
    assert!(err.iter().any(|msg| msg.contains("non-empty sweep source")));
}

#[test]
fn test_build_queue_from_plan_maps_corner_ac_base_mode() {
    use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [18usize].into_iter().collect();
    state.dialogs.corner_state = crate::simulation::dialog::corner::CornerDialogState::from_config(
        &CornerConfig::default()
            .with_process_corners(vec![ProcessCorner::TT])
            .with_base_analysis(CornerBaseAnalysis::Ac),
    );
    state.dialogs.ac_fstart = "1k".to_string();
    state.dialogs.ac_fstop = "10Meg".to_string();
    state.dialogs.ac_points = "12".to_string();
    state.dialogs.ac_sweep_type = 1; // octave

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 1);
    let corner = queue[0]
        .spec_options
        .corner
        .as_ref()
        .expect("corner options must be present");
    match &corner.base_mode {
        crate::services::simulation_runner::CornerBaseMode::Ac {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
        } => {
            assert!((*start_freq - 1e3).abs() < 1e-12);
            assert!((*stop_freq - 1e7).abs() < 1e-4);
            assert_eq!(*points_per_unit, 12);
            assert!(matches!(
                sweep,
                crate::services::simulation_runner::CornerFrequencySweep::Octave
            ));
        }
        other => panic!("expected AC corner base mode, got {:?}", other),
    }
}

#[test]
fn test_build_queue_from_plan_maps_corner_dc_base_mode() {
    use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig, ProcessCorner};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [18usize].into_iter().collect();
    state.dialogs.corner_state = crate::simulation::dialog::corner::CornerDialogState::from_config(
        &CornerConfig::default()
            .with_process_corners(vec![ProcessCorner::TT])
            .with_base_analysis(CornerBaseAnalysis::Dc),
    );
    state.dialogs.dc_source = "VDD".to_string();
    state.dialogs.dc_start = "0".to_string();
    state.dialogs.dc_stop = "1.2".to_string();
    state.dialogs.dc_step = "0.1".to_string();

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    assert_eq!(queue.len(), 1);
    let corner = queue[0]
        .spec_options
        .corner
        .as_ref()
        .expect("corner options must be present");
    match &corner.base_mode {
        crate::services::simulation_runner::CornerBaseMode::DcSweep {
            source_name,
            start,
            stop,
            step,
        } => {
            assert_eq!(source_name, "VDD");
            assert_eq!(*start, 0.0);
            assert_eq!(*stop, 1.2);
            assert_eq!(*step, 0.1);
        }
        other => panic!("expected DC corner base mode, got {:?}", other),
    }
}

#[test]
fn test_build_queue_from_plan_rejects_corner_dc_without_source() {
    use crate::simulation::dialog::corner::{CornerBaseAnalysis, CornerConfig};

    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses = [18usize].into_iter().collect();
    state.dialogs.corner_state = crate::simulation::dialog::corner::CornerDialogState::from_config(
        &CornerConfig::default().with_base_analysis(CornerBaseAnalysis::Dc),
    );
    state.dialogs.dc_source.clear();
    state.dialogs.dc_start = "0".to_string();
    state.dialogs.dc_stop = "1".to_string();
    state.dialogs.dc_step = "0.1".to_string();

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let err = controller
        .build_queue_from_plan(&state, &plan)
        .expect_err("corner DC base mode should require source");
    assert!(err.iter().any(|msg| msg.contains("non-empty sweep source")));
}

#[test]
fn test_build_queue_from_plan_maps_transient_optional_maxstep_and_uic() {
    let controller = SimulationController::new();
    let mut state = AppState::default();
    state.dialogs.enabled_analyses.insert(1);
    state.dialogs.tran_stop = "10u".to_string();
    state.dialogs.tran_step = "1n".to_string();
    state.dialogs.tran_start = "500n".to_string();
    state.dialogs.tran_maxstep = "2n".to_string();
    state.dialogs.tran_uic = true;

    let plan = controller
        .build_analysis_plan(&state)
        .expect("plan should build");
    let queue = controller
        .build_queue_from_plan(&state, &plan)
        .expect("queue should build");

    match &queue[0].config {
        Some(AnalysisConfig::Transient(tran)) => {
            assert!((tran.start_time - 500e-9).abs() < 1e-18);
            assert_eq!(tran.max_timestep, Some(2e-9));
            assert!(tran.uic);
        }
        _ => panic!("Expected transient config"),
    }
}

// -------------------------------------------------------------------------
// Config to AnalysisType Mapping Tests
// -------------------------------------------------------------------------

#[test]
fn test_config_to_analysis_type_dc_op() {
    let controller = SimulationController::new();
    assert_eq!(
        controller.config_to_analysis_type(&AnalysisConfig::DcOp),
        crate::state::AnalysisType::DcOp
    );
}

#[test]
fn test_config_to_analysis_type_dc_sweep() {
    let controller = SimulationController::new();
    let config = AnalysisConfig::DcSweep(DcSweepConfig {
        source: "V1".to_string(),
        start: 0.0,
        stop: 5.0,
        step: 0.1,
        source2: None,
        start2: None,
        stop2: None,
        step2: None,
    });
    assert_eq!(
        controller.config_to_analysis_type(&config),
        crate::state::AnalysisType::DcSweep
    );
}

#[test]
fn test_config_to_analysis_type_transient() {
    let controller = SimulationController::new();
    let config = AnalysisConfig::Transient(TransientAnalysisConfig {
        stop_time: 1e-6,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    });
    assert_eq!(
        controller.config_to_analysis_type(&config),
        crate::state::AnalysisType::Transient
    );
}

#[test]
fn test_config_to_analysis_type_ac() {
    let controller = SimulationController::new();
    let config = AnalysisConfig::Ac(AcAnalysisConfig {
        start_freq: 1.0,
        stop_freq: 1e9,
        num_points: 101,
        sweep_type: AcSweepType::Decade,
    });
    assert_eq!(
        controller.config_to_analysis_type(&config),
        crate::state::AnalysisType::Ac
    );
}

#[test]
fn test_config_to_analysis_type_all_variants() {
    use crate::simulation::config::{
        AcSweepType, NoiseAnalysisConfig, PoleZeroConfig, PzAnalysisType, SensitivityConfig,
    };
    let controller = SimulationController::new();

    // Noise - uses reference_node: String (not Option), sweep_type, num_points
    let noise_config = AnalysisConfig::Noise(NoiseAnalysisConfig {
        output_node: "out".to_string(),
        reference_node: "0".to_string(),
        input_source: "V1".to_string(),
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1e6,
    });
    assert_eq!(
        controller.config_to_analysis_type(&noise_config),
        crate::state::AnalysisType::Noise
    );

    // PoleZero - uses input_node, input_ref, output_node, output_ref, transfer_type, analysis_type
    let pz_config = AnalysisConfig::PoleZero(PoleZeroConfig {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "VOL".to_string(),
        analysis_type: PzAnalysisType::PoleZero,
    });
    assert_eq!(
        controller.config_to_analysis_type(&pz_config),
        crate::state::AnalysisType::PoleZero
    );

    // Sensitivity - uses output_var, ac_mode, frequency
    let sens_config = AnalysisConfig::Sensitivity(SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: None,
    });
    assert_eq!(
        controller.config_to_analysis_type(&sens_config),
        crate::state::AnalysisType::Sensitivity
    );
}

#[test]
fn test_spec_to_analysis_type_preserves_advanced_categories() {
    let controller = SimulationController::new();
    let cases = [
        (AnalysisSpec::Tf, crate::state::AnalysisType::Tf),
        (
            AnalysisSpec::Disto {
                start_freq: 1e3,
                stop_freq: 1e6,
                points_per_unit: 10,
                sweep: FrequencySweep::Decade,
                f2_over_f1: Some(1.5),
            },
            crate::state::AnalysisType::Disto,
        ),
        (AnalysisSpec::Pac, crate::state::AnalysisType::Pac),
        (AnalysisSpec::Pnoise, crate::state::AnalysisType::Pnoise),
        (AnalysisSpec::Pxf, crate::state::AnalysisType::Pxf),
        (AnalysisSpec::Pstb, crate::state::AnalysisType::Pstb),
        (
            AnalysisSpec::Stb {
                probe_node: "L1".to_string(),
                start_freq: 1.0,
                stop_freq: 1e6,
                points_per_decade: 10,
            },
            crate::state::AnalysisType::Stb,
        ),
        (
            AnalysisSpec::Reliability {
                target_years: vec![1.0, 5.0],
                enable_hci: true,
                enable_nbti: false,
                enable_em: false,
                min_stress_voltage: 0.05,
            },
            crate::state::AnalysisType::Reliability,
        ),
        (
            AnalysisSpec::Optimization {
                variables: vec![OptimizationVariable {
                    name: "X".to_string(),
                    min: 0.0,
                    max: 1.0,
                    initial: 0.5,
                }],
                objective_node: "out".to_string(),
                objective_ref: "0".to_string(),
                goal: OptimizationGoal::Minimize,
                target: None,
                algorithm: OptimizationAlgorithm::PatternSearch,
                max_iterations: 10,
                cost_tolerance: 1e-6,
                fd_step: 1e-3,
                initial_step: 0.1,
                min_step: 1e-5,
            },
            crate::state::AnalysisType::Optimization,
        ),
        (
            AnalysisSpec::Soa {
                stop_time: 1e-6,
                step_time: 1e-9,
                check_vgs_max: true,
                max_vgs: 1.2,
                check_vds_max: true,
                max_vds: 3.3,
                check_vbe_max: false,
                max_vbe: 0.9,
                check_vce_max: false,
                max_vce: 5.0,
            },
            crate::state::AnalysisType::Soa,
        ),
        (
            AnalysisSpec::SParameter {
                start_freq: 1e6,
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
                ],
            },
            crate::state::AnalysisType::SParameter,
        ),
        (
            AnalysisSpec::Envelope {
                fundamental_freq: 1e9,
                stop_time: 1e-6,
                num_harmonics: 9,
                max_step: None,
            },
            crate::state::AnalysisType::Envelope,
        ),
        (
            AnalysisSpec::Fourier {
                fundamental_freq: 1e6,
                num_harmonics: 11,
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                start_time: 0.0,
                stop_time: 10e-6,
            },
            crate::state::AnalysisType::Fourier,
        ),
    ];

    for (spec, expected) in cases {
        assert_eq!(
            controller.spec_to_analysis_type(&spec),
            expected,
            "unexpected analysis type mapping for {:?}",
            spec.run_type()
        );
    }
}

// -------------------------------------------------------------------------
// Convert to Analysis Result Tests
// -------------------------------------------------------------------------

#[test]
fn test_convert_dc_op_result() {
    use crate::simulation::results::DcOpResult as EngineDcOpResult;
    use crate::simulation::SimulationResult;

    let controller = SimulationController::new();
    let config = AnalysisConfig::DcOp;

    // Create engine DC OP result with sample data
    let mut engine_result = EngineDcOpResult::default();
    engine_result.node_voltages.insert("N001".to_string(), 5.0);
    engine_result.node_voltages.insert("N002".to_string(), 2.5);
    engine_result
        .branch_currents
        .insert("V1".to_string(), 0.001);

    let sim_result = SimulationResult::DcOp(engine_result);
    let analysis = controller.convert_to_analysis_result(&sim_result, &config);

    assert_eq!(analysis.analysis_type, crate::state::AnalysisType::DcOp);
    assert!(analysis.success);
    assert!(analysis.dc_op.is_some());

    let dc_op = analysis.dc_op.unwrap();
    assert_eq!(dc_op.node_voltages.len(), 2);
    assert_eq!(dc_op.branch_currents.len(), 1);

    // Verify node voltage conversion
    let v_n001 = dc_op.node_voltages.iter().find(|v| v.name == "V(N001)");
    assert!(v_n001.is_some());
    assert!((v_n001.unwrap().value - 5.0).abs() < 1e-10);
}

#[test]
fn test_convert_transient_result() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let controller = SimulationController::new();
    let config = AnalysisConfig::Transient(TransientAnalysisConfig {
        stop_time: 1e-6,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    });

    // Create engine transient result using proper constructor
    let time = vec![0.0, 1e-9, 2e-9, 3e-9];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), vec![0.0, 1.0, 2.0, 3.0]),
    );

    let sim_result = SimulationResult::Transient { time, waveforms };
    let analysis = controller.convert_to_analysis_result(&sim_result, &config);

    assert_eq!(
        analysis.analysis_type,
        crate::state::AnalysisType::Transient
    );
    assert!(analysis.success);
    assert!(analysis.dc_op.is_none());
    assert_eq!(analysis.waveforms.len(), 1);
    assert_eq!(analysis.waveforms[0].name, "V(out)");
    assert_eq!(analysis.waveforms[0].x.len(), 4);
    assert_eq!(analysis.waveforms[0].y.len(), 4);
}

#[test]
fn test_convert_ac_result() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let controller = SimulationController::new();
    let config = AnalysisConfig::Ac(AcAnalysisConfig {
        start_freq: 1.0,
        stop_freq: 1e6,
        num_points: 5,
        sweep_type: AcSweepType::Decade,
    });

    let frequencies = vec![1.0, 10.0, 100.0, 1000.0, 10000.0];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_freq_domain(
            "V(out)",
            frequencies.clone(),
            vec![1.0, 0.9, 0.7, 0.5, 0.3],
        ),
    );

    let sim_result = SimulationResult::Ac {
        frequencies,
        waveforms,
    };
    let analysis = controller.convert_to_analysis_result(&sim_result, &config);

    assert_eq!(analysis.analysis_type, crate::state::AnalysisType::Ac);
    assert_eq!(analysis.waveforms.len(), 1);
    assert_eq!(analysis.waveforms[0].name, "|V(out)|"); // Magnitude notation
}

#[test]
fn test_convert_dc_sweep_result() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let controller = SimulationController::new();
    let config = AnalysisConfig::DcSweep(DcSweepConfig {
        source: "V1".to_string(),
        start: 0.0,
        stop: 5.0,
        step: 1.0,
        source2: None,
        start2: None,
        stop2: None,
        step2: None,
    });

    let sweep_values = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain(
            "V(out)",
            sweep_values.clone(),
            vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5],
        ),
    );

    let sim_result = SimulationResult::DcSweep {
        sweep_var: "V1".to_string(),
        sweep_values,
        waveforms,
    };
    let analysis = controller.convert_to_analysis_result(&sim_result, &config);

    assert_eq!(analysis.analysis_type, crate::state::AnalysisType::DcSweep);
    assert_eq!(analysis.waveforms.len(), 1);
    assert_eq!(analysis.waveforms[0].x.len(), 6);
}

#[test]
fn test_convert_pole_zero_result() {
    use crate::simulation::config::{PoleZeroConfig, PzAnalysisType};
    use crate::simulation::SimulationResult;

    let controller = SimulationController::new();
    let config = AnalysisConfig::PoleZero(PoleZeroConfig {
        input_node: "in".to_string(),
        input_ref: "0".to_string(),
        output_node: "out".to_string(),
        output_ref: "0".to_string(),
        transfer_type: "VOL".to_string(),
        analysis_type: PzAnalysisType::PoleZero,
    });

    let sim_result = SimulationResult::PoleZero {
        poles: vec![(-1000.0, 0.0), (-500.0, 1000.0)],
        zeros: vec![(-100.0, 0.0)],
        gain: 10.0,
    };
    let analysis = controller.convert_to_analysis_result(&sim_result, &config);

    assert_eq!(analysis.analysis_type, crate::state::AnalysisType::PoleZero);
    assert!(analysis.waveforms.is_empty()); // PZ results are console-only
}

#[test]
fn test_convert_monte_carlo_result() {
    use crate::simulation::results::MonteCarloVariableResult;
    use crate::simulation::SimulationResult;

    let controller = SimulationController::new();
    let sim_result = SimulationResult::MonteCarlo {
        runs_requested: 16,
        runs_completed: 15,
        num_failures: 1,
        all_converged: false,
        variables: vec![MonteCarloVariableResult {
            name: "V(out)".to_string(),
            mean: 0.99,
            std_dev: 0.02,
            min: 0.9,
            max: 1.05,
            histogram: vec![2, 5, 6, 2],
            bin_edges: vec![0.9, 0.95, 1.0, 1.05, 1.1],
        }],
    };

    let analysis = controller.convert_to_analysis_result_with_metadata(
        &sim_result,
        crate::state::AnalysisType::MonteCarlo,
        "Monte Carlo",
    );
    assert_eq!(
        analysis.analysis_type,
        crate::state::AnalysisType::MonteCarlo
    );
    assert_eq!(analysis.waveforms.len(), 1);
    assert_eq!(analysis.waveforms[0].name, "hist(V(out))");
}

#[test]
fn test_convert_parametric_result() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let controller = SimulationController::new();
    let sweep_values = vec![-40.0, 25.0, 85.0];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", sweep_values.clone(), vec![1.1, 1.0, 0.9]),
    );
    let sim_result = SimulationResult::Parametric {
        target: "TEMP".to_string(),
        sweep_values,
        waveforms,
        num_failures: 0,
    };

    let analysis = controller.convert_to_analysis_result_with_metadata(
        &sim_result,
        crate::state::AnalysisType::Parametric,
        "Parametric",
    );
    assert_eq!(
        analysis.analysis_type,
        crate::state::AnalysisType::Parametric
    );
    assert_eq!(analysis.waveforms.len(), 1);
    assert_eq!(analysis.waveforms[0].x.len(), 3);
}

#[test]
fn test_convert_corner_result() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let controller = SimulationController::new();
    let temperatures = vec![-40.0, 25.0, 125.0];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", temperatures.clone(), vec![1.2, 1.0, 0.8]),
    );
    let sim_result = SimulationResult::Corner {
        x_values: temperatures.clone(),
        x_label: "Temperature".to_string(),
        x_unit: "C".to_string(),
        temperatures_c: temperatures,
        corner_labels: vec![
            "TT_1.000000V_-40.000000C".to_string(),
            "TT_1.000000V_25.000000C".to_string(),
            "TT_1.000000V_125.000000C".to_string(),
        ],
        waveforms,
        num_failures: 0,
    };

    let analysis = controller.convert_to_analysis_result_with_metadata(
        &sim_result,
        crate::state::AnalysisType::Corner,
        "Corner",
    );
    assert_eq!(analysis.analysis_type, crate::state::AnalysisType::Corner);
    assert_eq!(analysis.waveforms.len(), 1);
    assert_eq!(analysis.waveforms[0].x.len(), 3);
}

#[test]
fn test_color_for_index_cycles() {
    // Test that colors cycle properly
    let color0 = SimulationController::color_for_index(0);
    let color1 = SimulationController::color_for_index(1);
    let color8 = SimulationController::color_for_index(8); // Should wrap to 0

    assert_ne!(color0, color1);
    assert_eq!(color0, color8); // Wraps around after 8 colors
}

#[test]
fn test_color_for_index_valid_hex() {
    for i in 0..8 {
        let color = SimulationController::color_for_index(i);
        assert!(color.starts_with('#'));
        assert_eq!(color.len(), 7); // #RRGGBB format
    }
}

#[test]
fn test_current_config_none_initially() {
    let controller = SimulationController::new();
    assert!(controller.current_config.is_none());
    assert!(controller.current_spec.is_none());
}

#[test]
fn test_build_touchstone_dataset_from_sparameter_ac_result() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let freqs = vec![1e6, 2e6];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "S11".to_string(),
        EngineWaveformData::new_complex("S11", freqs.clone(), vec![0.1, 0.2], vec![0.01, 0.02]),
    );
    waveforms.insert(
        "S21".to_string(),
        EngineWaveformData::new_complex("S21", freqs.clone(), vec![0.9, 0.8], vec![0.0, -0.1]),
    );
    waveforms.insert(
        "S12".to_string(),
        EngineWaveformData::new_complex("S12", freqs.clone(), vec![0.02, 0.03], vec![0.0, 0.0]),
    );
    waveforms.insert(
        "S22".to_string(),
        EngineWaveformData::new_complex("S22", freqs.clone(), vec![0.2, 0.3], vec![-0.01, -0.02]),
    );

    let result = SimulationResult::Ac {
        frequencies: freqs.clone(),
        waveforms,
    };
    let dataset = SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 50.0], 2)
        .expect("touchstone dataset should build");

    assert_eq!(dataset.point_count(), 2);
    assert_eq!(dataset.signal_count(), 8);
    assert_eq!(
        dataset
            .metadata
            .get("num_ports")
            .cloned()
            .unwrap_or_default(),
        "2"
    );
    assert_eq!(
        dataset
            .metadata
            .get("touchstone_version")
            .cloned()
            .unwrap_or_default(),
        "2"
    );
}

#[test]
fn test_build_touchstone_dataset_records_per_port_reference_metadata() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let freqs = vec![1e6];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "S11".to_string(),
        EngineWaveformData::new_complex("S11", freqs.clone(), vec![0.1], vec![0.0]),
    );
    waveforms.insert(
        "S21".to_string(),
        EngineWaveformData::new_complex("S21", freqs.clone(), vec![0.9], vec![0.0]),
    );
    waveforms.insert(
        "S12".to_string(),
        EngineWaveformData::new_complex("S12", freqs.clone(), vec![0.02], vec![0.0]),
    );
    waveforms.insert(
        "S22".to_string(),
        EngineWaveformData::new_complex("S22", freqs.clone(), vec![0.2], vec![0.0]),
    );

    let result = SimulationResult::Ac {
        frequencies: freqs,
        waveforms,
    };
    let dataset = SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 75.0], 2)
        .expect("touchstone dataset should include per-port z0");
    assert_eq!(
        dataset.metadata.get("z0_ports").map(String::as_str),
        Some("50,75")
    );
}

#[test]
fn test_build_touchstone_dataset_rejects_non_uniform_reference_for_v1() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let freqs = vec![1e6];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "S11".to_string(),
        EngineWaveformData::new_complex("S11", freqs.clone(), vec![0.1], vec![0.0]),
    );
    waveforms.insert(
        "S21".to_string(),
        EngineWaveformData::new_complex("S21", freqs.clone(), vec![0.9], vec![0.0]),
    );
    waveforms.insert(
        "S12".to_string(),
        EngineWaveformData::new_complex("S12", freqs.clone(), vec![0.02], vec![0.0]),
    );
    waveforms.insert(
        "S22".to_string(),
        EngineWaveformData::new_complex("S22", freqs.clone(), vec![0.2], vec![0.0]),
    );

    let result = SimulationResult::Ac {
        frequencies: freqs,
        waveforms,
    };
    let err = SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 75.0], 1)
        .expect_err("touchstone v1 must reject non-uniform z0");
    assert!(err.contains("v1 export does not support per-port reference impedance"));
}

#[test]
fn test_build_touchstone_dataset_requires_complex_components() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let freqs = vec![1e6, 2e6];
    let mut waveforms = HashMap::new();
    // Real-only S11 should fail conversion.
    waveforms.insert(
        "S11".to_string(),
        EngineWaveformData::new_freq_domain("S11", freqs.clone(), vec![0.1, 0.2]),
    );
    waveforms.insert(
        "S21".to_string(),
        EngineWaveformData::new_complex("S21", freqs.clone(), vec![0.9, 0.8], vec![0.0, 0.0]),
    );
    waveforms.insert(
        "S12".to_string(),
        EngineWaveformData::new_complex("S12", freqs.clone(), vec![0.02, 0.03], vec![0.0, 0.0]),
    );
    waveforms.insert(
        "S22".to_string(),
        EngineWaveformData::new_complex("S22", freqs.clone(), vec![0.2, 0.3], vec![-0.01, -0.02]),
    );

    let result = SimulationResult::Ac {
        frequencies: freqs,
        waveforms,
    };
    let err = SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 50.0], 1)
        .expect_err("missing imag should fail");
    assert!(err.contains("missing imaginary component"));
}

#[test]
fn test_build_touchstone_dataset_from_three_port_result() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let freqs = vec![1e6, 2e6];
    let mut waveforms = HashMap::new();
    for row in 1..=3 {
        for col in 1..=3 {
            let name = format!("S{}_{}", row, col);
            waveforms.insert(
                name.clone(),
                EngineWaveformData::new_complex(
                    name.clone(),
                    freqs.clone(),
                    vec![0.1 * row as f64, 0.2 * col as f64],
                    vec![0.01 * col as f64, -0.02 * row as f64],
                ),
            );
        }
    }

    let result = SimulationResult::Ac {
        frequencies: freqs.clone(),
        waveforms,
    };
    let dataset =
        SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 60.0, 50.0], 2)
            .expect("touchstone dataset should build for three ports");

    assert_eq!(dataset.point_count(), 2);
    assert_eq!(dataset.signal_count(), 18);
    assert_eq!(
        dataset
            .metadata
            .get("num_ports")
            .cloned()
            .unwrap_or_default(),
        "3"
    );
}

#[test]
fn test_touchstone_export_path_uses_schematic_file_directory() {
    let mut state = AppState::default();
    state.schematic.current_file = Some(PathBuf::from("C:\\proj\\rf\\amp_top.rsch"));

    let path = SimulationController::touchstone_export_path(&state, 7, 2, 2);
    let normalized = path.to_string_lossy().replace('\\', "/");
    assert!(
        normalized.ends_with("C:/proj/rf/amp_top_run0007_sp02.s2p"),
        "unexpected export path: {}",
        normalized
    );
}

#[test]
fn test_touchstone_export_path_uses_port_count_extension() {
    let mut state = AppState::default();
    state.schematic.current_file = Some(PathBuf::from("C:\\proj\\rf\\amp_top.rsch"));

    let path = SimulationController::touchstone_export_path(&state, 7, 2, 3);
    let normalized = path.to_string_lossy().replace('\\', "/");
    assert!(
        normalized.ends_with("C:/proj/rf/amp_top_run0007_sp02.s3p"),
        "unexpected export path: {}",
        normalized
    );
}
