use super::*;

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
fn test_convert_transient_result_reuses_shared_time_axis_across_waveforms() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;
    use std::sync::Arc;

    let controller = SimulationController::new();
    let time = vec![0.0, 1e-9, 2e-9, 3e-9];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(a)".to_string(),
        EngineWaveformData::new_time_domain("V(a)", time.clone(), vec![0.0, 1.0, 0.0, -1.0]),
    );
    waveforms.insert(
        "V(b)".to_string(),
        EngineWaveformData::new_time_domain("V(b)", time.clone(), vec![1.0, 0.0, -1.0, 0.0]),
    );

    let analysis = controller.convert_to_analysis_result_with_metadata(
        &SimulationResult::Transient { time, waveforms },
        crate::state::AnalysisType::Transient,
        "Transient",
    );

    assert_eq!(analysis.waveforms.len(), 2);
    assert!(Arc::ptr_eq(
        &analysis.waveforms[0].x,
        &analysis.waveforms[1].x
    ));
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
fn test_preferred_viewer_for_analysis_matches_navigation_policy() {
    use crate::state::AnalysisType;

    let all_analysis_types = [
        AnalysisType::DcOp,
        AnalysisType::DcSweep,
        AnalysisType::Ac,
        AnalysisType::Disto,
        AnalysisType::Transient,
        AnalysisType::Noise,
        AnalysisType::PoleZero,
        AnalysisType::Tf,
        AnalysisType::Sensitivity,
        AnalysisType::Pac,
        AnalysisType::Pnoise,
        AnalysisType::Pxf,
        AnalysisType::Pstb,
        AnalysisType::Stb,
        AnalysisType::MonteCarlo,
        AnalysisType::Parametric,
        AnalysisType::Corner,
        AnalysisType::Reliability,
        AnalysisType::Optimization,
        AnalysisType::Soa,
        AnalysisType::SParameter,
        AnalysisType::Envelope,
        AnalysisType::Fourier,
        AnalysisType::HarmonicBalance,
        AnalysisType::Pss,
    ];

    for analysis_type in all_analysis_types {
        assert_eq!(
            SimulationController::preferred_viewer_for_analysis(analysis_type),
            crate::common::analysis_navigation::preferred_viewer(analysis_type),
            "viewer policy mismatch for {:?}",
            analysis_type
        );
    }
}

fn seed_active_transient_analysis(
    state: &mut AppState,
    name: &str,
    time: Vec<f64>,
    values: Vec<f64>,
) {
    let analysis = crate::state::AnalysisResult::new(1, AnalysisType::Transient, "Transient")
        .with_waveforms(vec![crate::state::WaveformData::new(
            name, time, values, "#4aa3ff",
        )]);
    let run = state.simulation.start_run();
    run.add_analysis(analysis);
    state.simulation.complete_run();
}

fn wait_for_derived_view(
    controller: &mut SimulationController,
    state: &mut AppState,
    viewer: crate::viewers::ActiveViewer,
) -> crate::simulation::controller::DerivedViewerLoadState {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    loop {
        let load_state = controller.ensure_transient_viewer_data(state, viewer);
        if !matches!(
            load_state,
            crate::simulation::controller::DerivedViewerLoadState::Loading
        ) {
            return load_state;
        }
        controller.sync_transient_post_views(state);
        assert!(
            std::time::Instant::now() < deadline,
            "timed out waiting for {:?}",
            viewer
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

#[test]
fn test_update_waveforms_transient_populates_waveforms_eye_and_fft() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .simulation
        .waveforms
        .push(crate::state::WaveformData::new(
            "stale",
            vec![0.0],
            vec![0.0],
            "#000000",
        ));

    let sample_count = 512;
    let dt = 1e-10;
    let bit_period = 2e-9;
    let mut time = Vec::with_capacity(sample_count);
    let mut signal = Vec::with_capacity(sample_count);
    for i in 0..sample_count {
        let t = i as f64 * dt;
        time.push(t);
        let cycles = (t / bit_period).floor() as i64;
        signal.push(if cycles % 2 == 0 { 1.0 } else { -1.0 });
    }

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), signal),
    );

    controller.update_waveforms(&mut state, &SimulationResult::Transient { time, waveforms });

    assert_eq!(state.simulation.waveforms.len(), 1);
    assert_eq!(state.simulation.waveforms[0].name, "V(out)");
    assert!(state.simulation.node_to_waveform.contains_key("V(out)"));
    assert!(state.eye_diagram_state.trace_count() > 0);
    assert!(state.fft_state.has_data());
    assert_eq!(
        state.panels.active_bottom_tab,
        crate::common::app::BottomPanelTab::Waveform
    );
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::Waveform
    );
}

#[test]
fn test_ensure_transient_fft_viewer_data_loads_in_background() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();

    let fs = 250_000.0;
    let n = 4096usize;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let values: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 18_000.0 * t).sin())
        .collect();
    seed_active_transient_analysis(&mut state, "V(out)", time, values);

    let initial =
        controller.ensure_transient_viewer_data(&mut state, crate::viewers::ActiveViewer::Fft);
    assert_eq!(
        initial,
        crate::simulation::controller::DerivedViewerLoadState::Loading
    );

    let final_state = wait_for_derived_view(
        &mut controller,
        &mut state,
        crate::viewers::ActiveViewer::Fft,
    );
    assert_eq!(
        final_state,
        crate::simulation::controller::DerivedViewerLoadState::Ready
    );
    assert!(state.fft_state.has_data());
    assert_eq!(state.fft_state.selected_source.as_deref(), Some("V(out)"));
}

#[test]
fn test_ensure_transient_eye_viewer_data_loads_in_background() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();

    let sample_count = 2048usize;
    let dt = 1e-10;
    let bit_period = 1.6e-9;
    let mut time = Vec::with_capacity(sample_count);
    let mut signal = Vec::with_capacity(sample_count);
    for i in 0..sample_count {
        let t = i as f64 * dt;
        time.push(t);
        let cycles = (t / bit_period).floor() as i64;
        signal.push(if cycles % 2 == 0 { 1.0 } else { -1.0 });
    }
    seed_active_transient_analysis(&mut state, "V(data)", time, signal);

    let final_state = wait_for_derived_view(
        &mut controller,
        &mut state,
        crate::viewers::ActiveViewer::EyeDiagram,
    );
    assert_eq!(
        final_state,
        crate::simulation::controller::DerivedViewerLoadState::Ready
    );
    assert!(state.eye_diagram_state.trace_count() > 0);
}

#[test]
fn test_sync_transient_post_views_clears_fft_when_active_analysis_changes() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();

    let fs = 200_000.0;
    let n = 2048usize;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let values_a: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 9_000.0 * t).sin())
        .collect();
    let values_b: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 15_000.0 * t).sin())
        .collect();

    let run = state.simulation.start_run();
    run.add_analysis(
        crate::state::AnalysisResult::new(1, AnalysisType::Transient, "TR-A").with_waveforms(vec![
            crate::state::WaveformData::new("V(a)", time.clone(), values_a, "#4aa3ff"),
        ]),
    );
    run.add_analysis(
        crate::state::AnalysisResult::new(2, AnalysisType::Transient, "TR-B").with_waveforms(vec![
            crate::state::WaveformData::new("V(b)", time, values_b, "#ffb347"),
        ]),
    );
    state.simulation.complete_run();

    let final_state = wait_for_derived_view(
        &mut controller,
        &mut state,
        crate::viewers::ActiveViewer::Fft,
    );
    assert_eq!(
        final_state,
        crate::simulation::controller::DerivedViewerLoadState::Ready
    );
    assert!(state.fft_state.has_data());

    assert!(state.simulation.select_analysis(1));
    controller.sync_transient_post_views(&mut state);

    assert!(!state.fft_state.has_data());
    let reload_state = wait_for_derived_view(
        &mut controller,
        &mut state,
        crate::viewers::ActiveViewer::Fft,
    );
    assert_eq!(
        reload_state,
        crate::simulation::controller::DerivedViewerLoadState::Ready
    );
    assert_eq!(state.fft_state.selected_source.as_deref(), Some("V(b)"));
}

#[test]
fn test_update_waveforms_transient_prefers_selected_fft_source_trace() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .fft_state
        .set_selected_source(Some("V(sel)".to_string()));

    let sample_count = 4096usize;
    let fs = 100_000.0;
    let time: Vec<f64> = (0..sample_count).map(|i| i as f64 / fs).collect();
    let sig_a: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 2_000.0 * t).sin())
        .collect();
    let sig_b: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 9_000.0 * t).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(a)".to_string(),
        EngineWaveformData::new_time_domain("V(a)", time.clone(), sig_a),
    );
    waveforms.insert(
        "V(sel)".to_string(),
        EngineWaveformData::new_time_domain("V(sel)", time.clone(), sig_b),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.name, "V(sel)");
    let fundamental = state.fft_state.fundamental_freq().expect("fundamental");
    assert!((fundamental - 9_000.0).abs() < 600.0);
}

#[test]
fn test_update_waveforms_transient_fft_source_fallback_is_deterministic_by_name() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();

    let sample_count = 4096usize;
    let fs = 100_000.0;
    let time: Vec<f64> = (0..sample_count).map(|i| i as f64 / fs).collect();

    let weak: Vec<f64> = time
        .iter()
        .map(|t| 0.01 * (2.0 * std::f64::consts::PI * 2_000.0 * t).sin())
        .collect();
    let strong: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 8_000.0 * t).sin())
        .collect();
    let dc: Vec<f64> = vec![2.0; sample_count];

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "A_weak".to_string(),
        EngineWaveformData::new_time_domain("A_weak", time.clone(), weak),
    );
    waveforms.insert(
        "Z_strong".to_string(),
        EngineWaveformData::new_time_domain("Z_strong", time.clone(), strong),
    );
    waveforms.insert(
        "M_dc".to_string(),
        EngineWaveformData::new_time_domain("M_dc", time.clone(), dc),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.name, "A_weak");
    let fundamental = state.fft_state.fundamental_freq().expect("fundamental");
    assert!((fundamental - 2_000.0).abs() < 400.0);
}

#[test]
fn test_update_waveforms_transient_missing_preferred_fft_source_uses_deterministic_fallback() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .fft_state
        .set_selected_source(Some("nonexistent_trace".to_string()));

    let sample_count = 4096usize;
    let fs = 100_000.0;
    let time: Vec<f64> = (0..sample_count).map(|i| i as f64 / fs).collect();
    let low: Vec<f64> = time
        .iter()
        .map(|t| 0.02 * (2.0 * std::f64::consts::PI * 1_000.0 * t).sin())
        .collect();
    let high: Vec<f64> = time
        .iter()
        .map(|t| 0.8 * (2.0 * std::f64::consts::PI * 7_000.0 * t).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "B_low".to_string(),
        EngineWaveformData::new_time_domain("B_low", time.clone(), low),
    );
    waveforms.insert(
        "C_high".to_string(),
        EngineWaveformData::new_time_domain("C_high", time.clone(), high),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.name, "B_low");
    assert_eq!(state.fft_state.selected_source.as_deref(), Some("B_low"));
}

#[test]
fn test_update_waveforms_transient_preferred_fft_source_matches_normalized_name() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state.fft_state.set_selected_source(Some("out".to_string()));

    let sample_count = 4096usize;
    let fs = 100_000.0;
    let time: Vec<f64> = (0..sample_count).map(|i| i as f64 / fs).collect();
    let out: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 9_000.0 * t).sin())
        .collect();
    let other: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 2_000.0 * t).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), out),
    );
    waveforms.insert(
        "V(a)".to_string(),
        EngineWaveformData::new_time_domain("V(a)", time.clone(), other),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.name, "V(out)");
    let fundamental = state.fft_state.fundamental_freq().expect("fundamental");
    assert!((fundamental - 9_000.0).abs() < 600.0);
}

#[test]
fn test_update_waveforms_transient_ambiguous_normalized_source_prefers_voltage_trace() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state.fft_state.set_selected_source(Some("out".to_string()));

    let sample_count = 4096usize;
    let fs = 100_000.0;
    let time: Vec<f64> = (0..sample_count).map(|i| i as f64 / fs).collect();
    let v_out: Vec<f64> = time
        .iter()
        .map(|t| 0.7 * (2.0 * std::f64::consts::PI * 9_000.0 * t).sin())
        .collect();
    let i_out: Vec<f64> = time
        .iter()
        .map(|t| 1.5 * (2.0 * std::f64::consts::PI * 2_000.0 * t).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "I(out)".to_string(),
        EngineWaveformData::new_time_domain("I(out)", time.clone(), i_out),
    );
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), v_out),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.name, "V(out)");
    let fundamental = state.fft_state.fundamental_freq().expect("fundamental");
    assert!((fundamental - 9_000.0).abs() < 600.0);
}

#[test]
fn test_update_waveforms_transient_typed_current_source_selects_current_trace() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .fft_state
        .set_selected_source(Some("I(out)".to_string()));

    let sample_count = 4096usize;
    let fs = 100_000.0;
    let time: Vec<f64> = (0..sample_count).map(|i| i as f64 / fs).collect();
    let v_out: Vec<f64> = time
        .iter()
        .map(|t| 0.7 * (2.0 * std::f64::consts::PI * 9_000.0 * t).sin())
        .collect();
    let i_out: Vec<f64> = time
        .iter()
        .map(|t| 1.5 * (2.0 * std::f64::consts::PI * 2_000.0 * t).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "I(out)".to_string(),
        EngineWaveformData::new_time_domain("I(out)", time.clone(), i_out),
    );
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), v_out),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.name, "I(out)");
    let fundamental = state.fft_state.fundamental_freq().expect("fundamental");
    assert!((fundamental - 2_000.0).abs() < 400.0);
}

#[test]
fn test_update_waveforms_transient_duplicate_labels_use_deterministic_key() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .fft_state
        .set_selected_source(Some("NET6".to_string()));

    let sample_count = 4096usize;
    let fs = 100_000.0;
    let time: Vec<f64> = (0..sample_count).map(|i| i as f64 / fs).collect();
    let a_signal: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 1_500.0 * t).sin())
        .collect();
    let b_signal: Vec<f64> = time
        .iter()
        .map(|t| (2.0 * std::f64::consts::PI * 9_000.0 * t).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "B_path.NET6".to_string(),
        EngineWaveformData::new_time_domain("NET6", time.clone(), b_signal),
    );
    waveforms.insert(
        "A_path.NET6".to_string(),
        EngineWaveformData::new_time_domain("NET6", time.clone(), a_signal),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.name, "A_path.NET6");
    assert_eq!(
        state.fft_state.selected_source.as_deref(),
        Some("A_path.NET6")
    );
    let fundamental = state.fft_state.fundamental_freq().expect("fundamental");
    assert!((fundamental - 1_500.0).abs() < 250.0);
}

#[test]
fn test_update_waveforms_transient_fft_reference_mode_preserves_large_uniform_input() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .fft_state
        .set_input_fidelity(crate::analysis::fft::state::InputFidelity::Reference);
    state
        .fft_state
        .set_selected_source(Some("V(out)".to_string()));

    let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
    let fs = 2_000_000.0;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let signal: Vec<f64> = (0..n)
        .map(|i| (2.0 * std::f64::consts::PI * 250_000.0 * i as f64 / fs).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), signal),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.decimation_factor, 1);
    assert_eq!(source.samples.len(), n);
}

#[test]
fn test_update_waveforms_transient_fft_interactive_mode_caps_large_uniform_input() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .fft_state
        .set_input_fidelity(crate::analysis::fft::state::InputFidelity::Interactive);
    state
        .fft_state
        .set_selected_source(Some("V(out)".to_string()));

    let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
    let fs = 2_000_000.0;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let signal: Vec<f64> = (0..n)
        .map(|i| (2.0 * std::f64::consts::PI * 250_000.0 * i as f64 / fs).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), signal),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert!(source.samples.len() <= crate::analysis::fft::DEFAULT_MAX_FFT_POINTS);
    assert!(source.decimation_factor > 1);
}

#[test]
fn test_update_waveforms_transient_fft_auto_n_tracks_effective_sample_count() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .fft_state
        .set_input_fidelity(crate::analysis::fft::state::InputFidelity::Interactive);
    state
        .fft_state
        .set_selected_source(Some("V(out)".to_string()));
    state.fft_state.sample_count_auto = true;
    state.fft_state.sample_count = 2048;

    let n = crate::analysis::fft::DEFAULT_MAX_FFT_POINTS * 3;
    let fs = 2_000_000.0;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let signal: Vec<f64> = (0..n)
        .map(|i| (2.0 * std::f64::consts::PI * 250_000.0 * i as f64 / fs).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), signal),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(state.fft_state.sample_count, source.samples.len());
}

#[test]
fn test_update_waveforms_transient_fft_manual_window_and_sample_count_are_applied() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state
        .fft_state
        .set_input_fidelity(crate::analysis::fft::state::InputFidelity::Reference);
    state
        .fft_state
        .set_selected_source(Some("V(out)".to_string()));
    state.fft_state.time_window_auto = false;
    state.fft_state.time_window_start = 0.2;
    state.fft_state.time_window_end = 0.4;
    state.fft_state.sample_count_auto = false;
    state.fft_state.sample_count = 2048;

    let n = 100_000usize;
    let fs = 100_000.0;
    let time: Vec<f64> = (0..n).map(|i| i as f64 / fs).collect();
    let signal: Vec<f64> = (0..n)
        .map(|i| (2.0 * std::f64::consts::PI * 5_000.0 * i as f64 / fs).sin())
        .collect();

    let mut waveforms = HashMap::new();
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_time_domain("V(out)", time.clone(), signal),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Transient {
            time: time.clone(),
            waveforms,
        },
    );

    let source = state.fft_state.source_cache.as_ref().expect("source cache");
    assert_eq!(source.decimation_factor, 1);
    assert_eq!(source.samples.len(), 2048);
    assert!(source.original_count > 15_000);
    assert!(source.original_count < 25_000);
}

#[test]
fn test_update_waveforms_ac_populates_bode_nyquist_and_smith_data() {
    use crate::simulation::results::WaveformData as EngineWaveformData;
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();

    let frequencies = vec![1e3, 1e4, 1e5, 1e6];
    let mut waveforms = HashMap::new();
    waveforms.insert(
        "S11".to_string(),
        EngineWaveformData::new_complex(
            "S11",
            frequencies.clone(),
            vec![0.1, 0.15, 0.2, 0.25],
            vec![0.0, -0.02, -0.04, -0.08],
        ),
    );
    waveforms.insert(
        "V(out)".to_string(),
        EngineWaveformData::new_complex(
            "V(out)",
            frequencies.clone(),
            vec![10.0, 3.0, 1.2, 0.7],
            vec![0.0, -1.0, -0.8, -0.2],
        ),
    );

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Ac {
            frequencies,
            waveforms,
        },
    );

    assert_eq!(state.simulation.waveforms.len(), 2);
    assert!(state
        .simulation
        .waveforms
        .iter()
        .any(|wf| wf.name.as_str() == "|S11|"));
    assert!(state
        .simulation
        .waveforms
        .iter()
        .any(|wf| wf.name.as_str() == "|V(out)|"));
    assert_eq!(state.bode_plot_state.trace_count(), 2);
    assert_eq!(state.nyquist_state.curve_count(), 2);
    assert_eq!(state.smith_chart_state.traces.len(), 1);
    assert_eq!(state.smith_chart_state.traces[0].name, "S11");
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::BodePlot
    );
}

#[test]
fn test_update_waveforms_noise_populates_output_input_and_contributors() {
    use crate::simulation::SimulationResult;
    use std::collections::HashMap;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();

    let frequencies = vec![1.0, 10.0, 100.0];
    let output_noise = vec![1e-12, 2e-12, 3e-12];
    let input_noise = Some(vec![5e-13, 6e-13, 7e-13]);
    let mut contributors = HashMap::new();
    contributors.insert("R1".to_string(), vec![1e-13, 2e-13, 3e-13]);
    contributors.insert("M1".to_string(), vec![4e-13, 5e-13, 6e-13]);

    controller.update_waveforms(
        &mut state,
        &SimulationResult::Noise {
            frequencies,
            output_noise,
            input_noise,
            contributors,
        },
    );

    assert_eq!(state.simulation.waveforms.len(), 4);
    assert!(state.simulation.node_to_waveform.contains_key("onoise"));
    assert!(state.simulation.node_to_waveform.contains_key("inoise"));
    assert!(state
        .simulation
        .waveforms
        .iter()
        .any(|wf| wf.name.starts_with("noise(")));
    assert!(
        state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Noise:")),
        "expected noise summary console output"
    );
}

#[test]
fn test_update_waveforms_monte_carlo_loads_histograms_and_skips_invalid_bins() {
    use crate::simulation::results::MonteCarloVariableResult;
    use crate::simulation::SimulationResult;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();

    let variables = vec![
        MonteCarloVariableResult {
            name: "V(out)".to_string(),
            mean: 1.0,
            std_dev: 0.01,
            min: 0.97,
            max: 1.03,
            histogram: vec![2, 6, 8, 4],
            bin_edges: vec![0.96, 0.98, 1.0, 1.02, 1.04],
        },
        MonteCarloVariableResult {
            name: "I(vdd)".to_string(),
            mean: 2e-3,
            std_dev: 1e-4,
            min: 1.8e-3,
            max: 2.3e-3,
            histogram: vec![1, 2, 3],
            bin_edges: vec![1.8e-3, 2.0e-3], // Invalid, should be skipped
        },
    ];

    controller.update_waveforms(
        &mut state,
        &SimulationResult::MonteCarlo {
            runs_requested: 32,
            runs_completed: 32,
            num_failures: 0,
            all_converged: true,
            variables,
        },
    );

    assert_eq!(state.histogram_state.histogram_count(), 1);
    assert_eq!(state.simulation.waveforms.len(), 2);
    assert!(state
        .simulation
        .waveforms
        .iter()
        .any(|wf| wf.name.as_str() == "hist(V(out))"));
    assert_eq!(
        state.panels.active_bottom_tab,
        crate::common::app::BottomPanelTab::Waveform
    );
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::Histogram
    );
}

#[test]
fn test_update_waveforms_monte_carlo_without_valid_histograms_selects_log_tab() {
    use crate::simulation::results::MonteCarloVariableResult;
    use crate::simulation::SimulationResult;

    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;

    controller.update_waveforms(
        &mut state,
        &SimulationResult::MonteCarlo {
            runs_requested: 8,
            runs_completed: 7,
            num_failures: 1,
            all_converged: false,
            variables: vec![MonteCarloVariableResult {
                name: "invalid".to_string(),
                mean: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
                histogram: vec![],
                bin_edges: vec![0.0],
            }],
        },
    );

    assert!(state.simulation.waveforms.is_empty());
    assert_eq!(state.histogram_state.histogram_count(), 0);
    assert_eq!(
        state.panels.active_bottom_tab,
        crate::common::app::BottomPanelTab::Log
    );
    assert_eq!(
        state.active_viewer(),
        crate::viewers::ActiveViewer::Waveform
    );
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
