use super::*;
use crate::state::AnalysisType;

#[test]
fn test_analysis_type_spice_command() {
    assert_eq!(AnalysisType::DcOp.spice_command(), ".op");
    assert_eq!(AnalysisType::Transient.spice_command(), ".tran");
    assert_eq!(AnalysisType::Ac.spice_command(), ".ac");
}

#[test]
fn test_dc_sweep_to_spice() {
    let cfg = DcSweepConfig {
        source: "Vin".to_string(),
        start: 0.0,
        stop: 5.0,
        step: 0.1,
        ..Default::default()
    };
    assert_eq!(cfg.to_spice(), ".dc Vin 0 5 0.1");
}

#[test]
fn test_dc_sweep_nested() {
    let cfg = DcSweepConfig {
        source: "Vin".to_string(),
        start: 0.0,
        stop: 5.0,
        step: 0.1,
        source2: Some("Vbias".to_string()),
        start2: Some(0.0),
        stop2: Some(1.0),
        step2: Some(0.25),
    };
    assert_eq!(cfg.to_spice(), ".dc Vin 0 5 0.1 Vbias 0 1 0.25");
}

#[test]
fn test_dc_sweep_validate() {
    let cfg = DcSweepConfig::default();
    assert!(cfg.validate().is_ok());

    let bad = DcSweepConfig {
        step: 0.0,
        ..Default::default()
    };
    assert!(bad.validate().is_err());
}

#[test]
fn test_dc_sweep_validate_rejects_partial_secondary_config() {
    let cfg = DcSweepConfig {
        source2: Some("Vbias".to_string()),
        start2: Some(0.0),
        stop2: Some(1.0),
        step2: None,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn test_dc_sweep_validate_rejects_duplicate_secondary_source() {
    let cfg = DcSweepConfig {
        source: "Vin".to_string(),
        source2: Some("vin".to_string()),
        start2: Some(0.0),
        stop2: Some(1.0),
        step2: Some(0.1),
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn test_dc_sweep_num_points() {
    let cfg = DcSweepConfig {
        start: 0.0,
        stop: 1.0,
        step: 0.1,
        ..Default::default()
    };
    assert_eq!(cfg.num_points(), 11);
}

#[test]
fn test_transient_to_spice_basic() {
    let cfg = TransientAnalysisConfig {
        stop_time: 1e-6,
        step_time: 1e-9,
        start_time: 0.0,
        max_timestep: None,
        uic: false,
    };
    assert_eq!(cfg.to_spice(), ".tran 0.000000001 0.000001");
}

#[test]
fn test_transient_to_spice_with_uic() {
    let cfg = TransientAnalysisConfig {
        uic: true,
        ..Default::default()
    };
    assert!(cfg.to_spice().contains("UIC"));
}

#[test]
fn test_transient_validate() {
    let cfg = TransientAnalysisConfig::default();
    assert!(cfg.validate().is_ok());

    let bad = TransientAnalysisConfig {
        stop_time: -1.0,
        ..Default::default()
    };
    assert!(bad.validate().is_err());
}

#[test]
fn test_ac_to_spice() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1e9,
    };
    assert_eq!(cfg.to_spice(), ".ac dec 10 1 1000000000");
}

#[test]
fn test_ac_total_points_decade() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1e3, // 3 decades
    };
    assert_eq!(cfg.total_points(), 31);
}

#[test]
fn test_ac_validate() {
    let cfg = AcAnalysisConfig::default();
    assert!(cfg.validate().is_ok());

    let bad = AcAnalysisConfig {
        start_freq: 1e9,
        stop_freq: 1.0, // reversed
        ..Default::default()
    };
    assert!(bad.validate().is_err());
}

#[test]
fn test_noise_to_spice() {
    let cfg = NoiseAnalysisConfig {
        output_node: "out".to_string(),
        reference_node: "0".to_string(),
        input_source: "Vin".to_string(),
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1e6,
    };
    assert_eq!(cfg.to_spice(), ".noise V(out,0) Vin dec 10 1 1000000");
}

#[test]
fn test_pz_to_spice() {
    let cfg = PoleZeroConfig::default();
    assert!(cfg.to_spice().starts_with(".pz"));
    assert!(cfg.to_spice().contains("VOL"));
    assert!(cfg.to_spice().contains("PZ"));
}

#[test]
fn test_pz_poles_only() {
    let cfg = PoleZeroConfig {
        analysis_type: PzAnalysisType::PolesOnly,
        ..Default::default()
    };
    assert!(cfg.to_spice().contains("POL"));
}

#[test]
fn test_sensitivity_dc() {
    let cfg = SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: false,
        frequency: None,
    };
    assert_eq!(cfg.to_spice(), ".sens V(out)");
}

#[test]
fn test_sensitivity_ac() {
    let cfg = SensitivityConfig {
        output_var: "V(out)".to_string(),
        ac_mode: true,
        frequency: Some(1e6),
    };
    assert!(cfg.to_spice().contains("AC"));
}

#[test]
fn test_analysis_config_type() {
    let cfg = AnalysisConfig::Transient(TransientAnalysisConfig::default());
    assert_eq!(cfg.analysis_type(), AnalysisType::Transient);
}

#[test]
fn test_analysis_config_validate() {
    let cfg = AnalysisConfig::DcOp;
    assert!(cfg.validate().is_ok());

    let cfg = AnalysisConfig::Transient(TransientAnalysisConfig::default());
    assert!(cfg.validate().is_ok());
}

//=========================================================================
// AC Frequency Generation Tests - Commercial Grade Coverage
//=========================================================================

#[test]
fn test_ac_generate_frequencies_decade_basic() {
    // 1Hz to 1kHz = 3 decades, 10 points per decade = 31 points
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1000.0,
    };

    let freqs = cfg.generate_frequencies();

    // Should have approximately 30 + 1 = 31 points for 3 decades
    assert!(!freqs.is_empty());
    assert!(
        freqs.len() >= 20,
        "Expected at least 20 points, got {}",
        freqs.len()
    );

    // First and last should match start/stop
    assert!((freqs[0] - 1.0).abs() < 1e-10);
    assert!((freqs.last().unwrap() - 1000.0).abs() < 1e-6);

    // Should be monotonically increasing
    for i in 1..freqs.len() {
        assert!(
            freqs[i] > freqs[i - 1],
            "Not monotonically increasing at index {}",
            i
        );
    }
}

#[test]
fn test_ac_generate_frequencies_decade_one_decade() {
    // 1Hz to 10Hz = 1 decade, 20 points per decade = 21 points
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 20,
        start_freq: 1.0,
        stop_freq: 10.0,
    };

    let freqs = cfg.generate_frequencies();

    assert_eq!(
        freqs.len(),
        21,
        "Expected 21 points for 1 decade with 20 pts/decade"
    );
    assert!((freqs[0] - 1.0).abs() < 1e-10);
    assert!((freqs.last().unwrap() - 10.0).abs() < 1e-10);
}

#[test]
fn test_ac_generate_frequencies_octave_basic() {
    // 1Hz to 8Hz = 3 octaves, 10 points per octave
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Octave,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 8.0,
    };

    let freqs = cfg.generate_frequencies();

    assert!(!freqs.is_empty());
    assert!(
        freqs.len() >= 25,
        "Expected at least 25 points, got {}",
        freqs.len()
    );

    // First and last should match
    assert!((freqs[0] - 1.0).abs() < 1e-10);
    assert!((freqs.last().unwrap() - 8.0).abs() < 1e-6);
}

#[test]
fn test_ac_generate_frequencies_linear_basic() {
    // Linear sweep: 100 to 200 Hz, 11 points
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Linear,
        num_points: 11,
        start_freq: 100.0,
        stop_freq: 200.0,
    };

    let freqs = cfg.generate_frequencies();

    assert_eq!(freqs.len(), 11);
    assert!((freqs[0] - 100.0).abs() < 1e-10);
    assert!((freqs.last().unwrap() - 200.0).abs() < 1e-10);

    // Linear sweep should have uniform spacing
    let step = (200.0 - 100.0) / 10.0;
    for i in 0..freqs.len() {
        let expected = 100.0 + i as f64 * step;
        assert!(
            (freqs[i] - expected).abs() < 1e-10,
            "Point {} expected {}, got {}",
            i,
            expected,
            freqs[i]
        );
    }
}

#[test]
fn test_ac_generate_frequencies_linear_single_point() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Linear,
        num_points: 1,
        start_freq: 100.0,
        stop_freq: 200.0,
    };

    let freqs = cfg.generate_frequencies();

    assert_eq!(freqs.len(), 1);
    // Single point should be at start, adjusted to stop for final value
    assert!((freqs[0] - 200.0).abs() < 1e-10);
}

#[test]
fn test_ac_generate_frequencies_linear_two_points() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Linear,
        num_points: 2,
        start_freq: 100.0,
        stop_freq: 200.0,
    };

    let freqs = cfg.generate_frequencies();

    assert_eq!(freqs.len(), 2);
    assert!((freqs[0] - 100.0).abs() < 1e-10);
    assert!((freqs[1] - 200.0).abs() < 1e-10);
}

#[test]
fn test_ac_generate_frequencies_invalid_zero_start() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 0.0,
        stop_freq: 100.0,
    };

    let freqs = cfg.generate_frequencies();
    assert!(
        freqs.is_empty(),
        "Should return empty for zero start frequency"
    );
}

#[test]
fn test_ac_generate_frequencies_invalid_negative_start() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: -1.0,
        stop_freq: 100.0,
    };

    let freqs = cfg.generate_frequencies();
    assert!(
        freqs.is_empty(),
        "Should return empty for negative start frequency"
    );
}

#[test]
fn test_ac_generate_frequencies_invalid_start_greater_than_stop() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1000.0,
        stop_freq: 100.0,
    };

    let freqs = cfg.generate_frequencies();
    assert!(freqs.is_empty(), "Should return empty when start > stop");
}

#[test]
fn test_ac_generate_frequencies_invalid_start_equals_stop() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 100.0,
        stop_freq: 100.0,
    };

    let freqs = cfg.generate_frequencies();
    assert!(freqs.is_empty(), "Should return empty when start == stop");
}

#[test]
fn test_ac_generate_frequencies_linear_zero_points() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Linear,
        num_points: 0,
        start_freq: 100.0,
        stop_freq: 1000.0,
    };

    let freqs = cfg.generate_frequencies();
    assert!(freqs.is_empty(), "Should return empty for zero points");
}

#[test]
fn test_ac_generate_frequencies_high_frequency_range() {
    // Typical RF frequency sweep: 1MHz to 10GHz
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1e6,
        stop_freq: 10e9,
    };

    let freqs = cfg.generate_frequencies();

    // 4 decades = 41 points
    assert!(freqs.len() >= 35);
    assert!((freqs[0] - 1e6).abs() < 1e-3);
    assert!((freqs.last().unwrap() - 10e9).abs() / 10e9 < 1e-6);
}

#[test]
fn test_ac_generate_frequencies_sub_hz() {
    // Very low frequency sweep for power supply applications
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 0.01,
        stop_freq: 100.0,
    };

    let freqs = cfg.generate_frequencies();

    // 4 decades = 41 points
    assert!(freqs.len() >= 35);
    assert!((freqs[0] - 0.01).abs() < 1e-12);
}

#[test]
fn test_ac_generate_frequencies_decade_logarithmic_spacing() {
    // Verify truly logarithmic spacing
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 100.0,
    };

    let freqs = cfg.generate_frequencies();

    // The ratio between consecutive points should be approximately constant
    // for logarithmic spacing
    if freqs.len() >= 3 {
        let ratio1 = freqs[1] / freqs[0];
        let ratio2 = freqs[2] / freqs[1];
        // Ratios should be very close for log spacing
        assert!(
            (ratio1 - ratio2).abs() / ratio1 < 0.01,
            "Logarithmic spacing violated: ratios {} and {}",
            ratio1,
            ratio2
        );
    }
}

#[test]
fn test_ac_total_points_decade_comprehensive() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1000.0, // 3 decades
    };

    let total = cfg.total_points();
    // 3 decades * 10 points/decade + 1 = 31
    assert_eq!(total, 31);
}

#[test]
fn test_ac_total_points_octave_comprehensive() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Octave,
        num_points: 5,
        start_freq: 1.0,
        stop_freq: 8.0, // 3 octaves
    };

    let total = cfg.total_points();
    // 3 octaves * 5 points/octave + 1 = 16
    assert_eq!(total, 16);
}

#[test]
fn test_ac_total_points_linear_comprehensive() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Linear,
        num_points: 50,
        start_freq: 100.0,
        stop_freq: 1000.0,
    };

    let total = cfg.total_points();
    assert_eq!(total, 50);
}

#[test]
fn test_ac_validate_valid_config() {
    let cfg = AcAnalysisConfig::default();
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_ac_validate_zero_start() {
    let cfg = AcAnalysisConfig {
        start_freq: 0.0,
        ..Default::default()
    };
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().iter().any(|e| e.contains("positive")));
}

#[test]
fn test_ac_validate_zero_stop() {
    let cfg = AcAnalysisConfig {
        stop_freq: 0.0,
        ..Default::default()
    };
    let result = cfg.validate();
    assert!(result.is_err());
}

#[test]
fn test_ac_validate_start_greater_than_stop() {
    let cfg = AcAnalysisConfig {
        start_freq: 1000.0,
        stop_freq: 100.0,
        ..Default::default()
    };
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().iter().any(|e| e.contains("less than")));
}

#[test]
fn test_ac_validate_zero_points() {
    let cfg = AcAnalysisConfig {
        num_points: 0,
        ..Default::default()
    };
    let result = cfg.validate();
    assert!(result.is_err());
}

#[test]
fn test_ac_to_spice_decade() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1e9,
    };
    let spice = cfg.to_spice();
    assert!(spice.starts_with(".ac"));
    assert!(spice.contains("dec"));
    assert!(spice.contains("10"));
}

#[test]
fn test_ac_to_spice_octave() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Octave,
        num_points: 5,
        start_freq: 100.0,
        stop_freq: 10000.0,
    };
    let spice = cfg.to_spice();
    assert!(spice.contains("oct"));
}

#[test]
fn test_ac_to_spice_linear() {
    let cfg = AcAnalysisConfig {
        sweep_type: AcSweepType::Linear,
        num_points: 100,
        start_freq: 1000.0,
        stop_freq: 2000.0,
    };
    let spice = cfg.to_spice();
    assert!(spice.contains("lin"));
}

//=========================================================================
// Noise Analysis Tests - Commercial Grade Coverage
//=========================================================================

#[test]
fn test_noise_generate_frequencies_decade() {
    let cfg = NoiseAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1000.0,
        ..Default::default()
    };

    let freqs = cfg.generate_frequencies();

    // 3 decades * 10 points/decade + 1
    assert!(freqs.len() >= 20);
    assert!((freqs[0] - 1.0).abs() < 1e-10);
    assert!((freqs.last().unwrap() - 1000.0).abs() < 1e-6);

    // Monotonically increasing
    for i in 1..freqs.len() {
        assert!(freqs[i] > freqs[i - 1]);
    }
}

#[test]
fn test_noise_generate_frequencies_octave() {
    let cfg = NoiseAnalysisConfig {
        sweep_type: AcSweepType::Octave,
        num_points: 5,
        start_freq: 100.0,
        stop_freq: 1600.0, // 4 octaves
        ..Default::default()
    };

    let freqs = cfg.generate_frequencies();

    assert!(!freqs.is_empty());
    assert!((freqs[0] - 100.0).abs() < 1e-10);
    assert!((freqs.last().unwrap() - 1600.0).abs() < 1e-6);
}

#[test]
fn test_noise_generate_frequencies_linear() {
    let cfg = NoiseAnalysisConfig {
        sweep_type: AcSweepType::Linear,
        num_points: 21,
        start_freq: 100.0,
        stop_freq: 1100.0,
        ..Default::default()
    };

    let freqs = cfg.generate_frequencies();

    assert_eq!(freqs.len(), 21);
    assert!((freqs[0] - 100.0).abs() < 1e-10);
    assert!((freqs.last().unwrap() - 1100.0).abs() < 1e-10);

    // Uniform spacing
    let step = (1100.0 - 100.0) / 20.0; // 50 Hz
    for i in 0..freqs.len() {
        let expected = 100.0 + i as f64 * step;
        assert!((freqs[i] - expected).abs() < 1e-10);
    }
}

#[test]
fn test_noise_generate_frequencies_invalid() {
    // Zero start
    let cfg = NoiseAnalysisConfig {
        start_freq: 0.0,
        stop_freq: 1000.0,
        ..Default::default()
    };
    assert!(cfg.generate_frequencies().is_empty());

    // Negative start
    let cfg = NoiseAnalysisConfig {
        start_freq: -1.0,
        stop_freq: 1000.0,
        ..Default::default()
    };
    assert!(cfg.generate_frequencies().is_empty());

    // Start >= stop
    let cfg = NoiseAnalysisConfig {
        start_freq: 1000.0,
        stop_freq: 100.0,
        ..Default::default()
    };
    assert!(cfg.generate_frequencies().is_empty());
}

#[test]
fn test_noise_validate_valid() {
    let cfg = NoiseAnalysisConfig::default();
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_noise_validate_missing_output_node() {
    let cfg = NoiseAnalysisConfig {
        output_node: "".to_string(),
        ..Default::default()
    };
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .iter()
        .any(|e| e.contains("Output node")));
}

#[test]
fn test_noise_validate_missing_input_source() {
    let cfg = NoiseAnalysisConfig {
        input_source: "".to_string(),
        ..Default::default()
    };
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .iter()
        .any(|e| e.contains("Input source")));
}

#[test]
fn test_noise_validate_frequency_range() {
    // Zero frequency
    let cfg = NoiseAnalysisConfig {
        start_freq: 0.0,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());

    // Negative frequency
    let cfg = NoiseAnalysisConfig {
        stop_freq: -100.0,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());

    // Start >= stop
    let cfg = NoiseAnalysisConfig {
        start_freq: 1e9,
        stop_freq: 1e6,
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[test]
fn test_noise_to_spice_comprehensive() {
    let cfg = NoiseAnalysisConfig {
        output_node: "out".to_string(),
        reference_node: "0".to_string(),
        input_source: "Vin".to_string(),
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1.0,
        stop_freq: 1e6,
    };
    let spice = cfg.to_spice();
    assert!(spice.starts_with(".noise"));
    assert!(spice.contains("V(out,0)"));
    assert!(spice.contains("Vin"));
    assert!(spice.contains("dec"));
}

#[test]
fn test_noise_default_temperature() {
    let cfg = NoiseAnalysisConfig::default();
    assert_eq!(cfg.default_temperature(), 300.0); // 27°C
}

#[test]
fn test_noise_audio_band() {
    // Standard audio band: 20Hz to 20kHz
    let cfg = NoiseAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 20,
        start_freq: 20.0,
        stop_freq: 20000.0,
        ..Default::default()
    };

    let freqs = cfg.generate_frequencies();

    // 3 decades * 20 points + 1
    assert!(freqs.len() >= 55);
    assert!((freqs[0] - 20.0).abs() < 1e-10);
    assert!((freqs.last().unwrap() - 20000.0).abs() < 1.0);
}

#[test]
fn test_noise_rf_band() {
    // RF noise analysis: 1MHz to 1GHz
    let cfg = NoiseAnalysisConfig {
        sweep_type: AcSweepType::Decade,
        num_points: 10,
        start_freq: 1e6,
        stop_freq: 1e9,
        ..Default::default()
    };

    let freqs = cfg.generate_frequencies();

    // 3 decades * 10 points + 1
    assert!(freqs.len() >= 25);
    // Verify precision at high frequencies
    assert!((freqs[0] / 1e6 - 1.0).abs() < 1e-6);
}
