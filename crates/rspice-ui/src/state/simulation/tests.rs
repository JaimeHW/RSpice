use super::*;

// =========================================================================
// AnalysisType Tests
// =========================================================================

#[test]
fn test_analysis_type_display_names() {
    assert_eq!(AnalysisType::DcOp.display_name(), "DC Operating Point");
    assert_eq!(AnalysisType::Ac.display_name(), "AC Analysis");
    assert_eq!(AnalysisType::Disto.display_name(), "DISTO");
    assert_eq!(AnalysisType::Transient.display_name(), "Transient");
    assert_eq!(AnalysisType::Noise.display_name(), "Noise");
    assert_eq!(AnalysisType::PoleZero.display_name(), "Pole-Zero");
    assert_eq!(AnalysisType::Tf.display_name(), "Transfer Function");
    assert_eq!(AnalysisType::Pac.display_name(), "PAC");
    assert_eq!(AnalysisType::Pnoise.display_name(), "PNoise");
    assert_eq!(AnalysisType::Pxf.display_name(), "PXF");
    assert_eq!(AnalysisType::Pstb.display_name(), "PSTB");
    assert_eq!(AnalysisType::Stb.display_name(), "STB");
    assert_eq!(AnalysisType::MonteCarlo.display_name(), "Monte Carlo");
    assert_eq!(AnalysisType::Parametric.display_name(), "Parametric Sweep");
    assert_eq!(AnalysisType::Corner.display_name(), "Corner Sweep");
    assert_eq!(AnalysisType::Reliability.display_name(), "Reliability");
    assert_eq!(AnalysisType::Optimization.display_name(), "Optimization");
    assert_eq!(AnalysisType::Soa.display_name(), "Safety (SOA)");
    assert_eq!(AnalysisType::SParameter.display_name(), "S-Parameter");
    assert_eq!(AnalysisType::Envelope.display_name(), "Envelope");
    assert_eq!(AnalysisType::Fourier.display_name(), "Fourier");
}

#[test]
fn test_analysis_type_spice_commands() {
    assert_eq!(AnalysisType::DcOp.spice_command(), ".op");
    assert_eq!(AnalysisType::DcSweep.spice_command(), ".dc");
    assert_eq!(AnalysisType::Ac.spice_command(), ".ac");
    assert_eq!(AnalysisType::Disto.spice_command(), ".disto");
    assert_eq!(AnalysisType::Transient.spice_command(), ".tran");
    assert_eq!(AnalysisType::Noise.spice_command(), ".noise");
    assert_eq!(AnalysisType::PoleZero.spice_command(), ".pz");
    assert_eq!(AnalysisType::Tf.spice_command(), ".tf");
    assert_eq!(AnalysisType::Sensitivity.spice_command(), ".sens");
    assert_eq!(AnalysisType::Pac.spice_command(), ".pac");
    assert_eq!(AnalysisType::Pnoise.spice_command(), ".pnoise");
    assert_eq!(AnalysisType::Pxf.spice_command(), ".pxf");
    assert_eq!(AnalysisType::Pstb.spice_command(), ".pstb");
    assert_eq!(AnalysisType::Stb.spice_command(), ".stb");
    assert_eq!(AnalysisType::MonteCarlo.spice_command(), ".mc");
    assert_eq!(AnalysisType::Parametric.spice_command(), ".step");
    assert_eq!(AnalysisType::Corner.spice_command(), ".step");
    assert_eq!(AnalysisType::Reliability.spice_command(), ".reliability");
    assert_eq!(AnalysisType::Optimization.spice_command(), ".opt");
    assert_eq!(AnalysisType::Soa.spice_command(), ".soa");
    assert_eq!(AnalysisType::SParameter.spice_command(), ".sp");
    assert_eq!(AnalysisType::Envelope.spice_command(), ".envlp");
    assert_eq!(AnalysisType::Fourier.spice_command(), ".four");
    assert_eq!(AnalysisType::HarmonicBalance.spice_command(), ".hb");
    assert_eq!(AnalysisType::Pss.spice_command(), ".pss");
}

#[test]
fn test_analysis_type_short_labels() {
    assert_eq!(AnalysisType::DcOp.short_label(), "DC");
    assert_eq!(AnalysisType::Ac.short_label(), "AC");
    assert_eq!(AnalysisType::Disto.short_label(), "DIST");
    assert_eq!(AnalysisType::Transient.short_label(), "TR");
    assert_eq!(AnalysisType::Tf.short_label(), "TF");
    assert_eq!(AnalysisType::Pac.short_label(), "PAC");
    assert_eq!(AnalysisType::Pnoise.short_label(), "PN");
    assert_eq!(AnalysisType::Pxf.short_label(), "PXF");
    assert_eq!(AnalysisType::Pstb.short_label(), "PSTB");
    assert_eq!(AnalysisType::Stb.short_label(), "STB");
    assert_eq!(AnalysisType::MonteCarlo.short_label(), "MC");
    assert_eq!(AnalysisType::Parametric.short_label(), "PAR");
    assert_eq!(AnalysisType::Corner.short_label(), "CRN");
    assert_eq!(AnalysisType::Reliability.short_label(), "REL");
    assert_eq!(AnalysisType::Optimization.short_label(), "OPT");
    assert_eq!(AnalysisType::Soa.short_label(), "SOA");
    assert_eq!(AnalysisType::SParameter.short_label(), "SP");
    assert_eq!(AnalysisType::Envelope.short_label(), "ENV");
    assert_eq!(AnalysisType::Fourier.short_label(), "FOU");
    assert_eq!(AnalysisType::HarmonicBalance.short_label(), "HB");
    assert_eq!(AnalysisType::Pss.short_label(), "PSS");
}

#[test]
fn test_analysis_type_display_trait() {
    assert_eq!(format!("{}", AnalysisType::DcOp), "DC Operating Point");
    assert_eq!(format!("{}", AnalysisType::Transient), "Transient");
}

#[test]
fn test_analysis_type_equality() {
    assert_eq!(AnalysisType::Ac, AnalysisType::Ac);
    assert_ne!(AnalysisType::Ac, AnalysisType::Transient);
}

#[test]
fn test_analysis_type_axis_info_for_hb_and_pss() {
    assert_eq!(
        AnalysisType::HarmonicBalance.axis_info(),
        ("Frequency", "Hz", "Magnitude", "V")
    );
    assert_eq!(
        AnalysisType::Disto.axis_info(),
        ("Frequency", "Hz", "Magnitude", "V")
    );
    assert_eq!(AnalysisType::Pss.axis_info(), ("Time", "s", "Voltage", "V"));
    assert_eq!(
        AnalysisType::Pnoise.axis_info(),
        ("Frequency", "Hz", "Noise", "V^2/Hz")
    );
    assert_eq!(
        AnalysisType::Optimization.axis_info(),
        ("Iteration", "iter", "Cost", "cost")
    );
    assert_eq!(
        AnalysisType::Reliability.axis_info(),
        ("Lifetime", "year", "Shift", "")
    );
}

// =========================================================================
// AnalysisResult Tests
// =========================================================================

#[test]
fn test_analysis_result_creation() {
    let result = AnalysisResult::new(1, AnalysisType::Transient, "Transient (0-1µs)");

    assert_eq!(result.id, 1);
    assert_eq!(result.analysis_type, AnalysisType::Transient);
    assert_eq!(result.label, "Transient (0-1µs)");
    assert!(result.success);
    assert!(result.error_message.is_none());
    assert!(result.waveforms.is_empty());
    assert!(result.dc_op.is_none());
}

#[test]
fn test_analysis_result_failed() {
    let result = AnalysisResult::failed(
        2,
        AnalysisType::Ac,
        "AC Analysis",
        "Singular matrix at DC operating point",
    );

    assert_eq!(result.id, 2);
    assert!(!result.success);
    assert_eq!(
        result.error_message.as_deref(),
        Some("Singular matrix at DC operating point")
    );
}

#[test]
fn test_analysis_result_with_waveforms() {
    let wf = WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.5, 1.5], "#ff0000");
    let result =
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![wf]);

    assert!(result.has_data());
    assert_eq!(result.waveforms.len(), 1);
    assert_eq!(result.waveforms[0].name, "V(out)");
}

#[test]
fn test_analysis_result_with_dc_op() {
    let dc_op = DcOpResult {
        node_voltages: vec![OperatingPointValue {
            name: "V(out)".to_string(),
            value: 2.5,
            unit: "V".to_string(),
        }],
        branch_currents: vec![],
        power_dissipation: vec![],
    };

    let result = AnalysisResult::new(1, AnalysisType::DcOp, "DC Op").with_dc_op(dc_op);

    assert!(result.has_data());
    assert!(result.dc_op.is_some());
    assert_eq!(result.dc_op.as_ref().unwrap().node_voltages.len(), 1);
}

// =========================================================================
// SimulationRun Tests
// =========================================================================

#[test]
fn test_simulation_run_creation() {
    let run = SimulationRun::new(1);

    assert_eq!(run.id, 1);
    assert!(run.label.starts_with("Run 1"));
    assert!(run.analyses.is_empty());
    assert!(run.success);
    assert_eq!(run.elapsed_time, 0.0);
}

#[test]
fn test_simulation_run_add_analysis() {
    let mut run = SimulationRun::new(1);

    run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC Op"));
    run.add_analysis(AnalysisResult::new(2, AnalysisType::Transient, "Transient"));

    assert_eq!(run.analyses.len(), 2);
    assert_eq!(run.successful_analyses(), 2);
    assert!(run.success);
}

#[test]
fn test_simulation_run_failed_analysis_marks_run_failed() {
    let mut run = SimulationRun::new(1);

    run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC Op"));
    run.add_analysis(AnalysisResult::failed(
        2,
        AnalysisType::Ac,
        "AC",
        "Matrix singular",
    ));

    assert!(!run.success);
    assert_eq!(run.successful_analyses(), 1);
}

#[test]
fn test_simulation_run_find_analysis() {
    let mut run = SimulationRun::new(1);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC Op"));
    run.add_analysis(AnalysisResult::new(2, AnalysisType::Transient, "Transient"));

    let dc = run.find_analysis(AnalysisType::DcOp);
    assert!(dc.is_some());
    assert_eq!(dc.unwrap().id, 1);

    let ac = run.find_analysis(AnalysisType::Ac);
    assert!(ac.is_none());
}

// =========================================================================
// SimulationState Run Management Tests
// =========================================================================

#[test]
fn test_simulation_state_start_run() {
    let mut state = SimulationState::default();

    assert!(!state.has_results());
    assert_eq!(state.run_count(), 0);

    let run = state.start_run();
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "Transient"));

    assert!(state.has_results());
    assert_eq!(state.run_count(), 1);
    assert_eq!(state.active_run_idx, Some(0));
}

#[test]
fn test_simulation_state_multiple_runs() {
    let mut state = SimulationState::default();

    // Start first run
    state.start_run();
    assert_eq!(state.next_run_id, 1);

    // Start second run
    state.start_run();
    assert_eq!(state.next_run_id, 2);
    assert_eq!(state.run_count(), 2);

    // Newest run is at index 0
    assert_eq!(state.runs[0].id, 2);
    assert_eq!(state.runs[1].id, 1);
}

#[test]
fn test_simulation_state_select_run() {
    let mut state = SimulationState::default();

    state.start_run();
    state.start_run();

    // Active is newest (index 0)
    assert_eq!(state.active_run_idx, Some(0));

    // Select older run
    assert!(state.select_run(1));
    assert_eq!(state.active_run_idx, Some(1));

    // Invalid index
    assert!(!state.select_run(10));
}

#[test]
fn test_simulation_state_select_analysis() {
    let mut state = SimulationState::default();

    let run = state.start_run();
    run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC"));
    run.add_analysis(AnalysisResult::new(2, AnalysisType::Transient, "TR"));

    assert!(state.select_analysis(0));
    assert_eq!(state.active_analysis_idx, Some(0));

    assert!(state.select_analysis(1));
    assert_eq!(state.active_analysis_idx, Some(1));

    assert!(!state.select_analysis(10));
}

#[test]
fn test_simulation_state_complete_run_syncs_waveforms() {
    let mut state = SimulationState::default();

    let run = state.start_run();
    let wf1 = WaveformData::new("V(1)", vec![0.0, 1.0], vec![0.0, 1.0], "#ff0000");
    let wf2 = WaveformData::new("V(2)", vec![0.0, 1.0], vec![0.5, 1.5], "#00ff00");

    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![wf1, wf2]),
    );

    state.complete_run();

    // Legacy waveforms should be synced
    assert_eq!(state.waveforms.len(), 2);
    assert_eq!(state.waveforms[0].name, "V(1)");
    assert_eq!(state.waveforms[1].name, "V(2)");
    assert_eq!(state.node_to_waveform.get("V(1)"), Some(&0));
    assert_eq!(state.node_to_waveform.get("V(2)"), Some(&1));
}

#[test]
fn test_simulation_state_delete_run() {
    let mut state = SimulationState::default();

    state.start_run();
    state.start_run();
    state.start_run();

    assert_eq!(state.run_count(), 3);

    // Delete middle run
    assert!(state.delete_run(1));
    assert_eq!(state.run_count(), 2);

    // Active was 0, should still be 0
    assert_eq!(state.active_run_idx, Some(0));

    // Delete invalid
    assert!(!state.delete_run(10));
}

#[test]
fn test_simulation_state_delete_active_run() {
    let mut state = SimulationState::default();

    let run_old = state.start_run();
    run_old.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TR-old").with_waveforms(vec![
            WaveformData::new("V(old)", vec![0.0, 1.0], vec![0.0, 1.0], "#ff0000"),
        ]),
    );
    state.complete_run();

    let run_new = state.start_run();
    run_new.add_analysis(
        AnalysisResult::new(2, AnalysisType::Transient, "TR-new").with_waveforms(vec![
            WaveformData::new("V(new)", vec![0.0, 1.0], vec![1.0, 2.0], "#00ff00"),
        ]),
    );
    state.complete_run();

    // Active is newest run at index 0.
    assert_eq!(state.active_run_idx, Some(0));
    assert_eq!(state.waveforms.len(), 1);
    assert_eq!(state.waveforms[0].name, "V(new)");
    assert_eq!(state.node_to_waveform.get("V(new)"), Some(&0));

    // Delete active run.
    state.delete_run(0);

    // Should select newest remaining (index 0) and sync displayed waveforms.
    assert_eq!(state.active_run_idx, Some(0));
    assert_eq!(state.active_analysis_idx, Some(0));
    assert_eq!(state.run_count(), 1);
    assert_eq!(state.waveforms.len(), 1);
    assert_eq!(state.waveforms[0].name, "V(old)");
    assert_eq!(state.node_to_waveform.get("V(old)"), Some(&0));
    assert!(!state.node_to_waveform.contains_key("V(new)"));
}

#[test]
fn test_simulation_state_prune_history() {
    let mut state = SimulationState::default();

    // Create MAX_RUN_HISTORY + 5 runs
    for _ in 0..(MAX_RUN_HISTORY + 5) {
        state.start_run();
    }

    // Should be pruned to MAX_RUN_HISTORY
    assert_eq!(state.run_count(), MAX_RUN_HISTORY);

    // Newest run should still be at index 0
    assert_eq!(state.runs[0].id, (MAX_RUN_HISTORY + 5) as u64);
}

#[test]
fn test_simulation_state_clear_runs() {
    let mut state = SimulationState::default();

    let run = state.start_run();
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TR").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#00aaff"),
        ]),
    );
    state.complete_run();

    state.start_run();
    state.select_run(1);
    let first_id = state.next_run_id;
    let version_before_clear = state.data_version;

    assert!(!state.waveforms.is_empty());
    assert!(!state.node_to_waveform.is_empty());

    state.clear_runs();

    assert!(!state.has_results());
    assert_eq!(state.active_run_idx, None);
    assert_eq!(state.active_analysis_idx, None);
    assert!(state.waveforms.is_empty());
    assert!(state.node_to_waveform.is_empty());
    assert_ne!(state.data_version, version_before_clear);

    // next_run_id should be preserved
    assert_eq!(state.next_run_id, first_id);
}

#[test]
fn test_simulation_state_select_run_without_analyses_clears_displayed_waveforms() {
    let mut state = SimulationState::default();

    let run_with_data = state.start_run();
    run_with_data.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TR").with_waveforms(vec![
            WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.2, 0.4], "#ffaa00"),
        ]),
    );
    state.complete_run();
    assert_eq!(state.waveforms.len(), 1);

    // Newest run has no analyses.
    state.start_run();
    assert!(state.select_run(0));
    assert_eq!(state.active_run_idx, Some(0));
    assert_eq!(state.active_analysis_idx, None);
    assert!(state.waveforms.is_empty());
    assert!(state.node_to_waveform.is_empty());
}

#[test]
fn test_simulation_state_active_getters() {
    let mut state = SimulationState::default();

    // No active run initially
    assert!(state.active_run().is_none());
    assert!(state.active_analysis().is_none());

    let run = state.start_run();
    run.add_analysis(AnalysisResult::new(1, AnalysisType::DcOp, "DC"));

    // Now we have an active run
    assert!(state.active_run().is_some());
    assert_eq!(state.active_run().unwrap().id, 1);

    // No active analysis yet
    assert!(state.active_analysis().is_none());

    state.select_analysis(0);
    assert!(state.active_analysis().is_some());
    assert_eq!(
        state.active_analysis().unwrap().analysis_type,
        AnalysisType::DcOp
    );
}
