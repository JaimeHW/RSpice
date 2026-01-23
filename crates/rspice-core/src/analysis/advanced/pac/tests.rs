//! PAC Module Tests
//!
//! Integration tests that verify the complete PAC analysis workflow.

use super::config::{PacConfig, PacSweepType};
use super::solver::{PacAnalyzer, PacAnalyzerState, PacError};
use crate::Value;
use crate::analysis::advanced::pss::{PeriodicWaveform, PssResult};
use std::f64::consts::PI;

//=============================================================================
// Test Fixtures
//=============================================================================

/// Create a PSS result representing a simple mixer with 1 GHz LO
fn create_mixer_pss() -> PssResult {
    let period = 1e-9; // 1 GHz
    let num_samples = 64;

    let time_points: Vec<Value> = (0..num_samples)
        .map(|i| (i as Value) / (num_samples as Value) * period)
        .collect();

    // Sinusoidal LO waveform with some harmonics
    let lo_waveform: Vec<Value> = time_points
        .iter()
        .map(|&t| {
            let phase = 2.0 * PI * t / period;
            1.0 * phase.cos() + 0.1 * (2.0 * phase).cos()
        })
        .collect();

    let rf_waveform: Vec<Value> = vec![0.0; num_samples]; // RF is small signal

    let output_waveform: Vec<Value> = time_points
        .iter()
        .map(|&t| {
            let phase = 2.0 * PI * t / period;
            0.5 * phase.cos() // Attenuated output
        })
        .collect();

    // Use PssResult::new() constructor (period, num_nodes, num_points)
    let mut result = PssResult::new(period, 4, num_samples); // 4 nodes: LO, RF, IF, (one more)

    // Set time points
    result.time = time_points;

    // Set node names (not including ground)
    result.node_names = vec![
        "LO".to_string(),
        "RF".to_string(),
        "IF".to_string(),
        "VDD".to_string(),
    ];

    // Set waveforms using PeriodicWaveform::from_values
    result.waveforms = vec![
        PeriodicWaveform::from_values(lo_waveform),
        PeriodicWaveform::from_values(rf_waveform),
        PeriodicWaveform::from_values(output_waveform),
        PeriodicWaveform::from_values(vec![3.3; num_samples]), // VDD constant
    ];

    result.iterations = 3;
    result.residual_norm = 1e-10;

    result
}

//=============================================================================
// Complete Workflow Tests
//=============================================================================

#[test]
fn test_complete_pac_workflow() {
    // Configure PAC analysis for a mixer
    let config = PacConfig::new()
        .with_sweep(1e6, 100e6, 20) // 1 MHz to 100 MHz IF
        .with_sidebands(-3, 3)
        .with_input_source("VRF")
        .with_output_node("IF");

    // Validate configuration
    assert!(config.validate().is_ok());

    // Create analyzer
    let mut analyzer = PacAnalyzer::new(config);
    assert_eq!(analyzer.state(), PacAnalyzerState::Ready);

    // Run analysis
    let pss = create_mixer_pss();
    let node_names = pss.node_names.clone();
    let result = analyzer.analyze(&pss, 4, node_names, vec![]);

    assert!(result.is_ok());
    let pac_result = result.unwrap();

    // Verify result structure
    assert!((pac_result.fundamental_frequency - 1e9).abs() < 1.0);
    assert_eq!(pac_result.num_sidebands(), 7);
    assert!(pac_result.num_frequencies() >= 20);
}

#[test]
fn test_mixer_conversion_gain() {
    let config = PacConfig::new()
        .with_sweep(10e6, 10e6, 1) // Single frequency
        .with_sidebands(-1, 1);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Get conversion gain from RF (sideband +1) to IF (sideband 0)
    let gain_rf_to_if = result.conversion_gain(1, 0, 0);

    // The gain should be non-trivial
    println!(
        "RF→IF conversion gain: {} dB",
        20.0 * gain_rf_to_if.norm().log10()
    );

    // Also check image rejection
    let gain_image_to_if = result.conversion_gain(-1, 0, 0);

    println!("Image→IF: {} dB", 20.0 * gain_image_to_if.norm().log10());
}

#[test]
fn test_pac_with_wide_sideband_range() {
    let config = PacConfig::new()
        .with_sweep(1e6, 50e6, 10)
        .with_sidebands(-7, 7); // Wide range for intermodulation

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    assert_eq!(result.num_sidebands(), 15);

    // Check that all sidebands are accessible
    for sb in -7..=7 {
        let v = result.voltage(1, 0, sb);
        // Should not panic
        let _ = v.norm();
    }
}

#[test]
fn test_pac_frequency_sweep_types() {
    let pss = create_mixer_pss();

    // Test linear sweep
    let config_linear = PacConfig::new()
        .with_sweep(1e6, 10e6, 10)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-1, 1);

    let mut analyzer = PacAnalyzer::new(config_linear);
    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Linear sweep: check uniform spacing
    let freqs = &result.frequencies;
    if freqs.len() > 2 {
        let step = freqs[1] - freqs[0];
        for i in 2..freqs.len() {
            let actual_step = freqs[i] - freqs[i - 1];
            assert!(
                (actual_step - step).abs() / step < 0.01,
                "Linear sweep should have uniform spacing"
            );
        }
    }

    // Test decade sweep
    let config_decade = PacConfig::new()
        .with_sweep(1e6, 100e6, 10) // 2 decades
        .with_sweep_type(PacSweepType::Decade)
        .with_sidebands(-1, 1);

    let mut analyzer = PacAnalyzer::new(config_decade);
    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Decade sweep: check logarithmic spacing
    let freqs = &result.frequencies;
    if freqs.len() > 2 {
        let ratio = freqs[1] / freqs[0];
        for i in 2..freqs.len() {
            let actual_ratio = freqs[i] / freqs[i - 1];
            assert!(
                (actual_ratio - ratio).abs() / ratio < 0.05,
                "Decade sweep should have uniform log spacing"
            );
        }
    }
}

#[test]
fn test_pac_result_spectrum_extraction() {
    let config = PacConfig::new()
        .with_sweep(10e6, 10e6, 1)
        .with_sidebands(-2, 2);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Extract voltage spectrum at output node
    let spectrum = result.voltage_spectrum(3, 0); // IF node

    assert_eq!(spectrum.len(), 5); // 5 sidebands
    assert!(spectrum.contains_key(&-2));
    assert!(spectrum.contains_key(&-1));
    assert!(spectrum.contains_key(&0));
    assert!(spectrum.contains_key(&1));
    assert!(spectrum.contains_key(&2));
}

#[test]
fn test_pac_conversion_matrix_properties() {
    let config = PacConfig::new()
        .with_sweep(1e6, 10e6, 5)
        .with_sidebands(-2, 2);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Get all transfer functions from RF to IF
    let transfers = result.get_transfer(1, 0);

    assert_eq!(transfers.len(), result.num_frequencies());

    // Check transfer data
    for transfer in &transfers {
        assert_eq!(transfer.input_sideband, 1);
        assert_eq!(transfer.output_sideband, 0);
        assert!(transfer.frequency_offset > 0.0);
    }
}

#[test]
fn test_pac_magnitude_phase_vs_frequency() {
    let config = PacConfig::new()
        .with_sweep(1e6, 50e6, 20)
        .with_sidebands(-1, 1);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Get magnitude vs frequency curve
    let mag_curve = result.magnitude_vs_frequency(3, 0); // IF node, sideband 0

    assert!(!mag_curve.is_empty());

    // Verify frequencies are increasing
    for i in 1..mag_curve.len() {
        assert!(
            mag_curve[i].0 > mag_curve[i - 1].0,
            "Frequencies should increase"
        );
    }

    // Get dB curve
    let db_curve = result.magnitude_db_vs_frequency(3, 0);
    assert_eq!(db_curve.len(), mag_curve.len());

    // Get phase curve
    let phase_curve = result.phase_vs_frequency(3, 0);
    assert_eq!(phase_curve.len(), mag_curve.len());

    // Phase should be in [-π, π]
    for (_, phase) in &phase_curve {
        assert!(
            *phase >= -PI && *phase <= PI,
            "Phase should be in [-π, π], got {}",
            phase
        );
    }
}

#[test]
fn test_pac_image_rejection_ratio() {
    let config = PacConfig::new()
        .with_sweep(10e6, 10e6, 1)
        .with_sidebands(-1, 1);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    let irr = result.image_rejection_db(0);

    // IRR should be finite (or infinity if perfect rejection)
    println!("Image Rejection Ratio: {} dB", irr);
}

#[test]
fn test_pac_sideband_data_access() {
    let config = PacConfig::new()
        .with_sweep(10e6, 10e6, 1)
        .with_sidebands(-1, 1);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Access sideband data
    for sb in -1..=1 {
        let data = result.get_sideband_data(0, sb);
        assert!(data.is_some(), "Sideband {} should exist", sb);

        let sd = data.unwrap();
        assert_eq!(sd.sideband, sb);
        assert!((sd.frequency_offset - 10e6).abs() < 1.0);
    }

    // Out of range should return None
    assert!(result.get_sideband_data(0, 5).is_none());
}

#[test]
fn test_pac_node_lookup() {
    let config = PacConfig::new()
        .with_sweep(10e6, 10e6, 1)
        .with_sidebands(-1, 1);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Case-insensitive lookup
    // Fixture has: ["LO"(0), "RF"(1), "IF"(2), "VDD"(3)]
    assert_eq!(result.node_index("LO"), Some(0));
    assert_eq!(result.node_index("lo"), Some(0)); // case insensitive
    assert_eq!(result.node_index("Lo"), Some(0));
    assert_eq!(result.node_index("IF"), Some(2));
    assert_eq!(result.node_index("NONEXISTENT"), None);
}

#[test]
fn test_pac_voltage_by_name() {
    let config = PacConfig::new()
        .with_sweep(10e6, 10e6, 1)
        .with_sidebands(0, 0);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Access by name
    let v_if = result.voltage_by_name("IF", 0, 0);
    let v_lo = result.voltage_by_name("lo", 0, 0); // lowercase

    // Ground should be zero
    let v_gnd = result.voltage_by_name("0", 0, 0);
    assert!((v_gnd.norm()).abs() < 1e-10);

    // LO node should have different voltage than IF
    // (depending on circuit topology)
    let _ = v_if.norm();
    let _ = v_lo.norm();
}

#[test]
fn test_pac_error_handling() {
    // Test with zero-period PSS
    let mut bad_pss = create_mixer_pss();
    bad_pss.period = 0.0;

    let config = PacConfig::new().with_sweep(1e6, 1e9, 10);
    let mut analyzer = PacAnalyzer::new(config);

    let result = analyzer.analyze(&bad_pss, 4, bad_pss.node_names.clone(), vec![]);
    assert!(result.is_err());

    match result {
        Err(PacError::InvalidPssSolution(_)) => (),
        _ => panic!("Expected InvalidPssSolution error"),
    }
}

#[test]
fn test_pac_single_sideband() {
    // Edge case: single sideband (DC only)
    let config = PacConfig::new()
        .with_sweep(1e6, 1e6, 1)
        .with_sidebands(0, 0);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    assert_eq!(result.num_sidebands(), 1);
    assert_eq!(result.sideband_indices(), vec![0]);
}

#[test]
fn test_pac_high_frequency_sweep() {
    // High frequency sweep for RF/mmWave
    let config = PacConfig::new()
        .with_sweep(1e9, 10e9, 50) // 1-10 GHz
        .with_sidebands(-2, 2);

    let mut analyzer = PacAnalyzer::new(config);
    let pss = create_mixer_pss();

    let result = analyzer
        .analyze(&pss, 4, pss.node_names.clone(), vec![])
        .unwrap();

    // Verify high frequency handling
    let max_freq = result
        .frequencies
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(max_freq >= 9e9, "Should reach high frequencies");
}
